pub mod event;
pub mod training;

pub use event::Event;
pub use training::{Level, Training};

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use time::Date;

#[cfg(feature = "ssr")]
mod loader;

#[cfg(feature = "ssr")]
pub use loader::{ContentStore, LoadError};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventsPageData {
    pub events: Vec<Event>,
    pub upcoming: Vec<Event>,
    pub today: Date,
}

#[server(GetTrainings, "/api")]
pub async fn get_trainings() -> Result<Vec<Training>, ServerFnError> {
    let store = expect_context::<ContentStore>();
    Ok(store.trainings.iter().cloned().collect())
}

#[server(GetUpcomingEvents, "/api")]
pub async fn get_upcoming_events() -> Result<Vec<Event>, ServerFnError> {
    let store = expect_context::<ContentStore>();
    let now = time::OffsetDateTime::now_utc();
    Ok(store
        .events
        .iter()
        .filter(|e| e.date >= now)
        .take(3)
        .cloned()
        .collect())
}

#[server(GetFeaturedTrainings, "/api")]
pub async fn get_featured_trainings() -> Result<Vec<Training>, ServerFnError> {
    let store = expect_context::<ContentStore>();
    Ok(store.trainings.iter().take(3).cloned().collect())
}

#[server(GetEventsPage, "/api")]
pub async fn get_events_page() -> Result<EventsPageData, ServerFnError> {
    let store = expect_context::<ContentStore>();
    let now = time::OffsetDateTime::now_utc();
    let events: Vec<Event> = store.events.iter().cloned().collect();
    let upcoming: Vec<Event> = store
        .events
        .iter()
        .filter(|e| e.date >= now)
        .cloned()
        .collect();
    Ok(EventsPageData {
        events,
        upcoming,
        today: now.date(),
    })
}
