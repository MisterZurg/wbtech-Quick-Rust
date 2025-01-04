pub mod models;
//
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use models::{CalendarEvent, CalendarEventForCreate};
use crate::repository::models::CalendarEventForUpdate;

#[derive(Clone, Debug, Serialize)]
struct Calendar {
    pub user_id: i64,
    // Date -> Description
    events: HashMap<String, String>,
}

/// ModelController — struct wraps AppState for working with business logic.
#[derive(Clone)]
pub struct ModelController {
    // users[user_id] -> user_calendar
    // user_calendar[date] -> Vector<events>
    // calendar_storage: Arc<Mutex<Option<HashMap<i64, HashMap<String, Vec<CalendarEvent>>>>>>,
    calendar_storage: Arc<Mutex<HashMap<i64, HashMap<String, Vec<CalendarEvent>>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            calendar_storage: Arc::default(),
        })
    }
}

// CRUD Implementation
impl ModelController {
    // region:    --- POST Handlers
    pub async fn create_event(
        &self,
        user_id: i64,
        event_fc: CalendarEventForCreate,
    ) -> Result<CalendarEvent> {
        let mut calendar_storage_storage_guard = self.calendar_storage.lock().unwrap();

        match calendar_storage_storage_guard.get_mut(&user_id) {
            Some(user_calendar) => {
                match user_calendar.get_mut(&event_fc.date) {
                    Some(events) => {
                        let event_id = events.len();

                        let created_event = CalendarEvent::new(
                            event_id as i64,
                            event_fc.date,
                            event_fc.description
                        );

                        events.push(created_event.clone());
                        Ok(created_event)
                    }
                    // TODO: Add ERROR not found
                    None => panic!("🚨 NO CALENDAR 🚨"),
                }
            }
            // Case There's no such user
            None => {
                // Create calendar for user
                // let new_user_calendar: HashMap<i64, HashMap<String, Vec<CalendarEvent>>> = HashMap::new();
                let event = CalendarEvent::new(
                    0,
                    event_fc.date.clone(),
                    event_fc.description,
                );

                let mut new_user_calendar:HashMap<String, Vec<CalendarEvent>> = HashMap::new();
                new_user_calendar.insert(event_fc.date, vec![event.clone()]);

                calendar_storage_storage_guard.insert(user_id, new_user_calendar);

                Ok(event)
            },
        }
    }
    pub async fn update_event(
        &self,
        user_id: i64,
        event_id: i64,
        event_date: String,
        event_upd: CalendarEventForUpdate,
    ) -> Result<CalendarEvent> {
        let mut calendar_storage_storage_guard = self.calendar_storage.lock().unwrap();

        match calendar_storage_storage_guard.get_mut(&user_id) {
            Some(user_calendar) => {
                match user_calendar.get_mut(&event_date) {
                    Some(mut events) => {
                        let event = CalendarEvent::new(event_id.clone(), event_date.clone(), event_upd.description.clone());

                        for i in 0..events.len() {
                            if events[i].event_id.clone() == event_id {
                                events[i] = event.clone();
                                break;
                            }
                        }

                        Ok(event)
                    }
                    None => { panic!("NO user_calendar") },
                }
            }
            // Case There's no such user
            None => { panic!("NO user found") },
        }
    }

    pub async fn delete_event(
        &self,
        user_id: i64,
        remove_event_id: i64,
        event_date: String,
    ) -> Result<()> {
        let mut calendar_storage_storage_guard = self.calendar_storage.lock().unwrap();


        match calendar_storage_storage_guard.get_mut(&user_id) {
            Some(user_calendar) => {
                match user_calendar.get_mut(&event_date) {
                    Some(mut events) => {

                        if let Some(index) = events.iter().position(|event| event.event_id == remove_event_id) {
                            events.swap_remove(index);
                        }

                        Ok(())
                    }
                    None => { panic!("NO user_calendar") },
                }
            }
            // Case There's no such user
            None => { panic!("NO user found") },
        }
    }


    // endregion: --- POST Handlers
    // region:    --- GET Handlers
    pub async fn get_events_for_day(
        &self,
        user_id: i64,
        date: String,
    ) -> Result<Vec<CalendarEvent>> {
        let mut calendar_storage_storage_guard = self.calendar_storage.lock().unwrap();

        match calendar_storage_storage_guard.get(&user_id) {
            Some(user_calendar) => {
                match user_calendar.get(&date) {
                    Some(events) => {
                        Ok(events.clone())
                    }
                    // TODO
                    None => panic!("Error."),
                }
            }
            // TODO
            None => panic!(),
        }
    }
    // endregion: --- GET Handlers
}


// struct Event {
//     date: String,
//     description: String,
// }

// event_id: i64, // user_id#date definitely no blind sql injection
// struct CalendarStorage {
//     storage: HashMap<String, Event>,
// }

// impl CalendarStorage {
//     fn new() -> CalendarStorage {
//         CalendarStorage{ storage: HashMap::new() }
//     }
//
//     fn create_event(&mut self, record: Event) {
//         self.storage.insert(
//             self.make_key(),
//             record,
//         )
//     }
//
//     fn get_event(&self, event_id: String) -> Event {
//         let event = self.storage.get(&event_id);
//         match event {
//             Some(event) => event,
//             None => None,
//         }
//     }
//
//     fn make_key(record: &Event) -> String {
//         format!("{}#{}", record.user_id, record.date)
//     }
// }
