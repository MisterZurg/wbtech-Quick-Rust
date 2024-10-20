use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateEventPayload {
    // event_id: i64, // user_id#date definitely no blind sql injection
    // user_id: i64,
    date: String,
    description: String,
}

#[derive(Deserialize)]
pub struct UpdateEventPayload {
    // event_id: i64, // user_id#date definitely no blind sql injection
    // user_id: i64,
    date: String,
    description: String,
}