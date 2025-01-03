mod handlers;
mod repository;
mod error;
mod models;

pub use self::error::{Error, Result};


use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::Router;
use axum::routing::{get, post};
use crate::repository::ModelController;

#[derive(Default)]
pub struct AppState {
    calendar: Mutex<HashMap<String, String>>,
}

#[tokio::main]
async fn main()-> Result<()> {
    let mc = ModelController::new().await?;

    // build our application with a route
    let app = Router::new()
        // Get HeatBeat
        .route("/heartbeat", get(handlers::heartbeat))
        // POST /create_event
        .route("/create_event", post(handlers::create_event))
        // POST /update_event
        .route("/update_event", post(handlers::update_event))
        // POST /delete_event
        .route("/delete_event", post(handlers::delete_event))
        // GET /events_for_day
        .route("/events_for_day", get(handlers::events_for_day))
        // GET /events_for_week
        // .route("/events_for_week", get(handlers::events_for_week))
        // GET /events_for_month
        // .route("/events_for_month", get(handlers::events_for_month))
        .with_state(mc.clone());


    println!("🍷🗿 Server started successfully");
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}