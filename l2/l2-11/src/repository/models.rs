#[derive(Clone)]
pub struct CalendarEvent {
    pub event_id: i64,
    pub date: String,
    pub description: String,
}
impl CalendarEvent {
    pub fn new(event_id: i64, date: String, description: String) -> Self {
        CalendarEvent {
            event_id,
            date,
            description,
        }
    }

    pub fn default() -> Self {
        CalendarEvent{
            event_id: 0,
            date: "".to_string(),
            description: "".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct CalendarEventForCreate {
    pub date: String,
    pub description: String,
}

#[derive(Clone)]
pub struct CalendarEventForUpdate {
    pub description: String,
}