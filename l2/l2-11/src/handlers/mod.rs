mod models;
mod mw_validate;

use axum::response::{Html, IntoResponse, Response};
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

use crate::error::{Error, Result};
use crate::handlers::models::CalendarEvent;
use crate::repository;

use serde_json::json;
use log::debug;

/// POST /create_event
pub async fn create_event(
    _mc: State<repository::ModelController>,
    Json(payload): Json<models::CreateCalendarEventPayload>,
) -> Result<Json<CalendarEvent>> {
    let calendar_event = _mc.create_event(
        payload.user_id,
        crate::repository::models::CalendarEventForCreate {
            date: payload.date,
            description: payload.description,
        }).await?;


    let resp = CalendarEvent{
        event_id: calendar_event.event_id,
        date: calendar_event.date,
        description: calendar_event.description,
    };

    println!("🚧DEBUG: create_event {:?}", resp);

    Ok(Json(resp))
}


/// POST /update_event
pub async fn update_event(
    _mc: State<repository::ModelController>,
    Json(payload): Json<models::UpdateCalendarEventPayload>
) -> Result<Json<CalendarEvent>> {
    let calendar_event = _mc.update_event(
        payload.user_id,
        payload.event_id,
        payload.date, repository::models::CalendarEventForUpdate {
            description: payload.description,
        }).await?;


    let resp = CalendarEvent {
        event_id: calendar_event.event_id,
        date: calendar_event.date,
        description: calendar_event.description,
    };

    println!("🚧DEBUG: update_event {:?}", resp);

    Ok(Json(resp))
}

/// POST /delete_event
pub async fn delete_event(
    _mc: State<repository::ModelController>,
    Json(payload): Json<models::UpdateCalendarEventPayload>
) -> Result<Json<CalendarEvent>> {

}


#[derive(Debug, Deserialize)]
pub struct eventsForDayParams {
    user_id: i64,
    date: String,
}

/// GET /events_for_day
pub async fn events_for_day(
    _mc: State<repository::ModelController>,
    Query(params): Query<eventsForDayParams>,
) -> Result<Json<Vec<CalendarEvent>>> {
    let calendar_events = _mc.get_events_for_day(params.user_id, params.date).await?;

    let resp: Vec<CalendarEvent> = calendar_events
        .into_iter() // Use into_iter if you want to consume the original vector
        .map(|event| CalendarEvent {
            event_id: event.event_id,
            date: event.date,
            description: event.description,
        })
        .collect::<Vec<_>>();

    println!("events_for_day {:?}", resp);

    Ok(Json(resp))
}
//
// /// GET /events_for_week
// pub async fn events_for_week() -> &'static str {
//     todo!()
// }
//
// /// GET /events_for_month
// pub async fn events_for_month() -> &'static str {
//     todo!()
// }
//
// /// validate_params — helper for для парсинга и валидации параметров методов
// /// create_event && update_event
// fn validate_params() {
//
// }


// pub fn routes(mc: ModelController) -> Router {
//     Router::new()
//         .route("/tickets", post(create_ticket).get(list_tickets))
//         .route("/tickets/:id", delete(delete_ticket))
//         .with_state(mc)
// }


/// GET /heartbeat
pub async fn heartbeat() -> impl IntoResponse {
    const MESSAGE: &str = "Simple Calendar CRUD build with Tokio & Axum";

    let json_response = json!({
        "status": "👌success 🗿",
        "message": MESSAGE
    });

    Json(json_response)
}