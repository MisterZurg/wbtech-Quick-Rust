mod handlers;

use axum::Router;
use axum::routing::{get, post};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // build our application with a route
    let app = Router::new()
        // POST /create_event
        .route("/create_event", post(handlers::create_event))
        // POST /update_event
        .route("/update_event/:id", post(handlers::update_event))
        // POST /delete_event
        .route("/delete_event/:id", post(handlers::delete_event))
        // GET /events_for_day
        .route("/events_for_day", get(handlers::events_for_day))
        // GET /events_for_week
        .route("/events_for_week", get(handlers::events_for_week))
        // GET /events_for_month
        .route("/events_for_month", get(handlers::events_for_month));


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
