#![allow(unused)] // For beginning only

use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json,
    Router,
    extract::{Path, Query, State},
};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres, FromRow, PgPool, Row};

use serde::{Deserialize, Serialize};
use serde_json::json;

// ------------------------
// -- Model Logic
// ------------------------
/// Order represents the database model that we got from backend.
#[derive(Debug, FromRow, Deserialize, Serialize)]
struct Order {
    order_uid: String,
    track_number: String,
    entry: String,
    delivery: Delivery,
    // items: Vec<Item>,
    // locale: String,
    // internal_signature: String,
    // customer_id: String,
    // delivery_service: String,
    // shardkey: String,
    // sm_id: u64,
    // date_created: String,
    // oof_shard: String,
}

/// Delivery ...
#[derive(Debug, FromRow, Deserialize, Serialize)]
struct Delivery {
    name: String,
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    email: String,
}

/// Delivery ...
#[derive(Debug, FromRow, Deserialize, Serialize)]
struct Payment {
    transaction: String,
    request_id: String,
    currency: String,
    provider: String,
    amount: u64,
    payment_dt: u64,
    bank: String,
    delivery_cost: u64,
    goods_total: u64,
    custom_fee: u64,
}

// Item ...
#[derive(Debug, FromRow, Deserialize, Serialize)]
struct Item {
    chrt_id: u64,
    track_number: String,
    price: u64,
    rid: String,
    name: String,
    sale: u64,
    size: String,
    total_price: u64,
    nm_id: u64,
    brand: String,
    status: u64,
}


// ------------------------
// -- App configuration
// ------------------------
struct Config {
    #[serde(default = "default_db_type")]
    db_type: String,
    #[serde(default = "default_db_user")]
    postgres_user: String,
    #[serde(default = "default_db_password")]
    postgres_password: String,
    #[serde(default = "default_db_name")]
    postgres_db: String,
    #[serde(default = "default_db_host")]
    postgres_host: String,
    #[serde(default = "default_db_port")]
    postgres_port: String,
}

impl Config {
    fn show_cfg(&self) {
        println!("DB_TYPE {:?}", self.db_type);
        println!("DB_NAME {:?}", self.postgres_db);
        println!("DB_USER {:?}", self.postgres_user);
        println!("DB_PASSWORD {:?}", self.postgres_password);
        println!("DB_HOST {:?}", self.postgres_host);
        println!("DB_PORT {:?}", self.postgres_port);
    }

    // TODO fix connection string
    fn get_connection_url(&self) -> String {
        format!("{}://{}:{}@{}:{}/{}", self.db_type, self.postgres_user, self.postgres_password, self.postgres_host, self.postgres_port, self.postgres_port)
    }
}

/// default_db_user() provides default value for ... if not set
fn default_db_user() -> String {
    "postgres".to_string()
}

/// default_db_() provides default value for ... if not set
fn default_db_password() -> String {
    "password".to_string()
}

/// default_db_() provides default value for ... if not set
fn default_db_type() -> String {
    "postgres".to_string()
}

/// default_db_() provides default value for ... if not set
fn default_db_name() -> String {
    "wb-db".to_string()
}

/// default_db_() provides default value for ... if not set
fn default_db_host() -> String {
    "localhost".to_string()
}

/// default_db_() provides default value for ... if not set
fn default_db_port() -> String {
    "5432".to_string()
}




async fn health_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

// ------------------------
// -- Business Logic
// ------------------------
pub async fn create_order_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<Order>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("{:?}", body);
    let query_result = sqlx::query(
        "INSERT INTO orders (order_uid, track_number, entry) VALUES ($1, $2, $3) RETURNING *")
        .bind(body.order_uid.to_string())
        .bind(body.track_number.to_string())
        .bind(body.entry.to_string())
        .fetch_one(&data.db)
        .await;


    match query_result {
        Ok(row) => {
            let order = Order{
                order_uid: row.get("order_uid"),
                track_number: row.get("track_number"),
                entry: row.get("entry"),
            };

            let note_response = json!({"status": "success","data": json!({
                "note": order
            })});

            return Ok((StatusCode::CREATED, Json(note_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = json!({
                    "status": "fail",
                    "message": "Note with that title already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

async fn get_order_handler(
    Path(order_uid): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("Got {}", order_uid);

    let query_result = sqlx::query( "SELECT * FROM orders WHERE order_uid = $1")
        .bind(&order_uid)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(row) => {
            let order = Order{
                order_uid: row.get("order_uid"),
                track_number: row.get("track_number"),
                entry: row.get("entry"),
                delivery: Delivery {
                    name: "".to_string(),
                    phone: "".to_string(),
                    zip: "".to_string(),
                    city: "".to_string(),
                    address: "".to_string(),
                    region: "".to_string(),
                    email: "".to_string(),
                },
            };

            let order_response = json!({"status": "success","data": json!({
                "note": order
            })});

            return Ok(Json(order_response));
        }
        Err(_) => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Note with ID: {} not found", order_uid)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}


pub struct AppState {
    db: Pool<Postgres>,
}


#[tokio::main]
async fn main() {
    // Parse config
    dotenvy::dotenv().expect("Unable to access/read .env file");

    // TODO: CFG
    // let cfg = envy::from_env::<Config>().unwrap();
    // cfg.show_cfg();
    // println!("{}", cfg.get_connection_url());

    // postgresql://[user[:password]@][netloc][:port][/dbname][?param1=value1&...]
    let database_url = "postgresql://MikhailGrachev:mgrachev@localhost:5432/wbtech_school";

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app_state = Arc::new(AppState { db: pool.clone() });
    let app = Router::new()
        .route("/orders/health", get(health_handler))
        .route("/orders/:order_uid", get(get_order_handler))
        .route("/orders", post(create_order_handler))
        .with_state(app_state);;

    println!("🍷🗿 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}