// use serde::{Deserialize, Serialize};
//
//
// #[derive(Clone, Debug, Serialize)]
// pub struct Event {
//     pub id: i64,
//     pub date: String,
//     pub description: String,
// }
//
// #[derive(Deserialize)]
// pub struct CreateEventPayload {
//     pub id: i64,
//     pub date: String,
//     pub description: String,
// }
//
// #[derive(Serialize)]
// pub struct GetEventPayload {
//     pub id: i64,
//     pub date: String,
//     pub description: String,
// }
//
// //
// // #[derive(Deserialize)]
// // pub struct UpdateEventPayload {
// //     // event_id: i64, // user_id#date definitely no blind sql injection
// //     // user_id: i64,
// //     date: String,
// //     description: String,
// // }
//
//
// #[derive(Clone)]
// pub struct CreateCalendarEvent {
//     pub date: String,
//     pub description: String,
// }
//
// #[derive(Clone)]
// pub struct CalendarEvent {
//     pub user_id: i64,
//     pub date: String,
//     pub description: String,
// }
//
// impl CalendarEvent {
//     pub fn new(user_id: i64, date: String, description: String) -> Self {
//         CalendarEvent {
//             user_id,
//             date,
//             description,
//         }
//     }
// }