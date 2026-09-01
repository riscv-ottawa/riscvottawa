pub mod event;
pub mod project;
pub mod resource;

pub use event::{
    schedule_clock, Event, EventDate, Extra, Panel, Slot, Speaker, Spotlight, Status, Teaser,
    LUMA_CALENDAR_URL, LUMA_PENDING,
};
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
        .filter(|e| e.date.is_upcoming(now))
        .take(3)
        .cloned()
        .collect())
}

// Events the countdown may display: anything whose start is still in the future
// or finished within the last 12 hours. The client picks the active one and
// advances to the next as each event's post-event window elapses. Sorted
// ascending by date (the content loader guarantees this). Events that only have
// a month are left out; there is no instant to count down to.
#[server(GetCountdownEvents, "/api")]
pub async fn get_countdown_events() -> Result<Vec<Event>, ServerFnError> {
    let store = expect_context::<ContentStore>();
    let cutoff = time::OffsetDateTime::now_utc() - RETIRE_AFTER;
    Ok(store
        .events
        .iter()
        .filter(|e| e.date.instant().is_some_and(|dt| dt >= cutoff))
        .cloned()
        .collect())
}

// How long an event stays on the home page after it starts. Matches
// `RETIRE_WINDOW_MS` in the countdown component: the timer and the spotlight
// band should disappear together, not one and then the other.
#[cfg(feature = "ssr")]
const RETIRE_AFTER: time::Duration = time::Duration::hours(12);

/// The event behind a `/events/<slug>` URL, but only if it is a spotlight.
/// Ordinary meetups return `None` and the page renders a not-found, so we never
/// ship a thin page for an event that has nothing extra to say.
#[server(GetSpotlightEvent, "/api")]
pub async fn get_spotlight_event(slug: String) -> Result<Option<Event>, ServerFnError> {
    let store = expect_context::<ContentStore>();
    Ok(store
        .events
        .iter()
        .find(|e| e.slug == slug && e.spotlight.is_some())
        .cloned())
}

/// The spotlight event to feature on the home page: the soonest one that hasn't
/// aged out yet. `None` most months, which is the point.
#[server(GetFeaturedSpotlight, "/api")]
pub async fn get_featured_spotlight() -> Result<Option<Event>, ServerFnError> {
    let store = expect_context::<ContentStore>();
    let now = time::OffsetDateTime::now_utc();
    // Events are sorted ascending, so the first match is the soonest. A
    // month-only spotlight counts for the whole of its month; a scheduled one
    // stays up until the retire window elapses.
    Ok(store
        .events
        .iter()
        .find(|e| {
            e.spotlight.is_some()
                && match e.date.instant() {
                    Some(dt) => dt >= now - RETIRE_AFTER,
                    None => e.date.is_upcoming(now),
                }
        })
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
        .filter(|e| e.date.is_upcoming(now))
        .cloned()
        .collect();
    Ok(EventsPageData {
        events,
        upcoming,
        today: now.date(),
    })
}
