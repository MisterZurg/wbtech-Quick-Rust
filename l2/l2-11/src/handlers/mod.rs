mod models;

use axum::extract::Path;
use axum::Json;

/// POST /create_event
pub async fn create_event(Json(payload): Json<models::CreateEventPayload>) -> &'static str {
    todo!()
}


/// POST /update_event/:id
pub async fn update_event(Path(id): Path<String>, Json(payload): Json<models::CreateEventPayload>) -> &'static str {
    todo!()
}

/// POST /delete_event/:id
pub async fn delete_event(Path(id): Path<String>) -> &'static str {
    todo!()
}

/// GET /events_for_day
pub async fn events_for_day() -> &'static str {
    "events_for_day"
}

/// GET /events_for_week
pub async fn events_for_week() -> &'static str {
    todo!()
}

/// GET /events_for_month
pub async fn events_for_month() -> &'static str {
    todo!()
}

/// validate_params — helper for для парсинга и валидации параметров методов
/// create_event && update_event
fn validate_params() {

}