#![allow(unused)] // For beginning only

use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use chrono::{DateTime, Utc};

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json,
    Router,
    extract::{Path, Query, State},
};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres, FromRow, PgPool, Row, Executor};

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
    payment: Payment,
    items: Vec<Item>,
    locale: String,
    internal_signature: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: i64,
    date_created: DateTime<Utc>,
    oof_shard: String,
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
    amount: i64,
    payment_dt: i64,
    bank: String,
    delivery_cost: i64,
    goods_total: i64,
    custom_fee: i64,
}

// Item ...
#[derive(Debug, FromRow, Deserialize, Serialize)]
struct Item {
    chrt_id: i64,
    track_number: String,
    price: i64,
    rid: String,
    name: String,
    sale: i64,
    size: String,
    total_price: i64,
    nm_id: i64,
    brand: String,
    status: i64,
}


// ------------------------
// -- App configuration
// ------------------------
#[derive(Deserialize, Serialize)]
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

    // let database_url = "postgresql://MikhailGrachev:mgrachev@localhost:5432/wbtech_school";
    fn get_connection_url(&self) -> String {
        format!("{}://{}:{}@{}:{}/{}", self.db_type, self.postgres_user, self.postgres_password, self.postgres_host, self.postgres_port, self.postgres_db)
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
    "postgresql".to_string()
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
    // TODO : ask how to make a transaction .. the trait `Executor<'_>` is not implemented
    // let mut tnx = data.db.begin().await.inspect_err(|err|{
    //     let error_response = json!({
    //         "status": "fail",
    //         "message": format!("🚨️🗿 You should ⚡️🧑🏿‍🦱⚡️ now! Failed starting a transaction:{}", err),
    //     });
    //     Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    // });
    sqlx::query("
        INSERT INTO orders
            (order_uid, track_number, entry, locale, internal_signature, customer_id, delivery_service, shardkey, sm_id, date_created, oof_shard)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)") // RETURNING *")
        .bind(&body.order_uid)
        .bind(&body.track_number)
        .bind(&body.entry)
        .bind(&body.locale)
        .bind(&body.internal_signature)
        .bind(&body.customer_id)
        .bind(&body.delivery_service)
        .bind(&body.shardkey)
        .bind(&body.sm_id)
        .bind(&body.date_created)
        .bind(&body.oof_shard)
        .fetch_one(&data.db)
        .await;

    sqlx::query("
        INSERT INTO deliveries (order_uid, name, phone, zip, city, address, region, email)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
        .bind(&body.order_uid)
        .bind(&body.delivery.name)
        .bind(&body.delivery.phone)
        .bind(&body.delivery.zip)
        .bind(&body.delivery.city)
        .bind(&body.delivery.address)
        .bind(&body.delivery.region)
        .bind(&body.delivery.email)
        .fetch_one(&data.db)
        .await;

    // let order_uid: &str =  &body.order_uid.clone().as_str();

    sqlx::QueryBuilder::<Postgres>::new("
            INSERT INTO items (order_uid, chrt_id, track_number, price, rid, name, sale, i_size, total_price, nm_id, brand, status)")
    .push_values(body.items, |mut build, item| {
        build
            .push_bind(&body.order_uid)
            .push_bind(item.chrt_id)
            .push_bind(item.track_number)
            .push_bind(item.price)
            .push_bind(item.rid)
            .push_bind(item.name)
            .push_bind(item.sale)
            .push_bind(item.size)
            .push_bind(item.total_price)
            .push_bind(item.nm_id)
            .push_bind(item.brand)
            .push_bind(item.status);
    })
    .build()
    .fetch_one(&data.db)
    .await;

    sqlx::query("
        INSERT INTO payments (transaction_id, request_id, currency, provider, amount, payment_dt, bank, delivery_cost, goods_total, custom_fee)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
        .bind(&body.order_uid)
        .bind(&body.payment.request_id)
        .bind(&body.payment.currency)
        .bind(&body.payment.provider)
        .bind(&body.payment.amount)
        .bind(&body.payment.payment_dt)
        .bind(&body.payment.bank)
        .bind(&body.payment.delivery_cost)
        .bind(&body.payment.goods_total)
        .bind(&body.payment.custom_fee)
        .fetch_one(&data.db)
        .await;


    // tnx.unwrap().commit().await.inspect_err(|err| {
    //     println!("🚨🗿 Error commiting transaction in database :{}", err);
    // });



    let note_response = json!({"status": "success","data": json!({
        "order": "created_order"
    })});

    return Ok((StatusCode::CREATED, Json(note_response)));
}

async fn get_order_handler(
    Path(order_uid): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut order = Order{
        order_uid: order_uid.clone(),
        track_number: "".to_string(),
        entry: "".to_string(),
        delivery: Delivery {
            name: "".to_string(),
            phone: "".to_string(),
            zip: "".to_string(),
            city: "".to_string(),
            address: "".to_string(),
            region: "".to_string(),
            email: "".to_string(),
        },
        payment: Payment {
            transaction: "".to_string(),
            request_id: "".to_string(),
            currency: "".to_string(),
            provider: "".to_string(),
            amount: 0,
            payment_dt: 0,
            bank: "".to_string(),
            delivery_cost: 0,
            goods_total: 0,
            custom_fee: 0,
        },
        items: vec![],
        locale: "".to_string(),
        internal_signature: "".to_string(),
        customer_id: "".to_string(),
        delivery_service: "".to_string(),
        shardkey: "".to_string(),
        sm_id: 0,
        date_created: Default::default(),
        oof_shard: "".to_string(),
    };

    let query_order_result = sqlx::query( "SELECT * FROM orders WHERE order_uid = $1")
        .bind(&order_uid)
        .fetch_one(&data.db)
        .await;

    match query_order_result {
        Ok(row) => {
            order.order_uid = row.get("order_uid");
            order.track_number = row.get("track_number");
            order.entry = row.get("entry");
            order.locale = row.get("locale");
            order.internal_signature= row.get("internal_signature");
            order.customer_id= row.get("customer_id");
            order.delivery_service= row.get("delivery_service");
            order.shardkey= row.get("shardkey");
            order.sm_id= row.get("sm_id");
            order.date_created = row.get("date_created");
            order.oof_shard= row.get("oof_shard");
        } Err(_) => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Order with ID: {} not found", order_uid)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }


    let query_delivery_result = sqlx::query("SELECT * FROM deliveries WHERE order_uid = $1")
        .bind(&order_uid)
        .fetch_one(&data.db)
        .await;


    match query_delivery_result {
        Ok(row) => {
            let delivery = Delivery{
                name: row.get("name"),
                phone: row.get("phone"),
                zip: row.get("zip"),
                city: row.get("city"),
                address: row.get("address"),
                region: row.get("region"),
                email: row.get("email"),
            };
            order.delivery = delivery;
        } Err(_) => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Delivery with order_uid: {} not found", order_uid)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }


    let query_payment_result = sqlx::query("SELECT * FROM payments WHERE transaction_id = $1")
        .bind(&order_uid)
        .fetch_one(&data.db)
        .await;

    match query_payment_result {
        Ok(row) => {
            let payment = Payment{
                transaction: row.get("transaction_id"),
                request_id: row.get("request_id"),
                currency: row.get("currency"),
                provider: row.get("provider"),
                amount: row.get("amount"),
                payment_dt: row.get("payment_dt"),
                bank: row.get("bank"),
                delivery_cost: row.get("delivery_cost"),
                goods_total: row.get("goods_total"),
                custom_fee: row.get("custom_fee"),
            };

            order.payment = payment;
        } Err(_) => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Payment with transaction_id: {} not found", order_uid) // transaction_id == order_uid
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }


    let query_items_result = sqlx::query("SELECT * FROM items WHERE order_uid = $1")
        .bind(&order_uid)
        .fetch_all(&data.db)
        .await;

    match query_items_result {
        Ok(rows) => {
            let mut items: Vec<Item> = Vec::with_capacity(rows.len());
            for row in rows {
                items.push(Item {
                    chrt_id: row.get("chrt_id"),
                    track_number: row.get("track_number"),
                    price: row.get("price"),
                    rid: row.get("rid"),
                    name: row.get("name"),
                    sale: row.get("sale"),
                    size: row.get("i_size"),
                    total_price: row.get("total_price"),
                    nm_id: row.get("nm_id"),
                    brand: row.get("brand"),
                    status: row.get("status"),
                });
            }
            order.items = items;
        } Err(_) => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Items with order_uid: {} not found", order_uid)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }


    let order_response = json!({
        "status": "success",
        "data": json!({ "order": order})
    });

    Ok(Json(order_response))
}


pub struct AppState {
    db: Pool<Postgres>,
}


#[tokio::main]
async fn main() {
    // Parse config
    dotenvy::dotenv().expect("Unable to access/read .env file");

    let cfg = envy::from_env::<Config>().unwrap();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&cfg.get_connection_url())
        .await
    {
        Ok(pool) => {
            println!("🧃🗿 Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🚨🗿️ You should ⚡️🧑🏿‍🦱⚡️ now! Failed to connect to the database: {:?}", err);
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