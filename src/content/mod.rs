pub mod event;
pub mod project;
pub mod resource;

pub use event::Event;
pub use project::{Level, Project};
pub use resource::{ResourceLink, ResourceSection};

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

#[server(GetProjects, "/api")]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    let store = expect_context::<ContentStore>();
    Ok(store.projects.iter().cloned().collect())
}

#[server(GetResourceSections, "/api")]
pub async fn get_resource_sections() -> Result<Vec<ResourceSection>, ServerFnError> {
    let store = expect_context::<ContentStore>();
    Ok(store.resources.iter().cloned().collect())
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

#[server(GetFeaturedProjects, "/api")]
pub async fn get_featured_projects() -> Result<Vec<Project>, ServerFnError> {
    let store = expect_context::<ContentStore>();
    Ok(store.projects.iter().take(3).cloned().collect())
}

#[server(GetInauguralEvent, "/api")]
pub async fn get_inaugural_event() -> Result<Option<Event>, ServerFnError> {
    let store = expect_context::<ContentStore>();
    Ok(store
        .events
        .iter()
        .find(|e| e.slug.ends_with("inaugural-meeting"))
        .cloned())
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
