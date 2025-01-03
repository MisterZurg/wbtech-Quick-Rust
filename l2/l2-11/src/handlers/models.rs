use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize)]
pub struct CalendarEvent {
    pub event_id: i64,
    pub date: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct CreateCalendarEventPayload {
    pub user_id: i64,
    pub date: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct UpdateCalendarEventPayload {
    pub user_id: i64,
    pub date: String,
    pub event_id: i64,
    pub description: String,
}

#[derive(Deserialize)]
pub struct DeleteCalendarEventPayload {
    pub user_id: i64,
    pub date: String,
    pub event_id: i64,
}

#[derive(Serialize)]
pub struct GetCalendarEventPayload {
    pub event_id: i64,
    pub date: String,
    pub description: String,
}