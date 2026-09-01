use crate::content::{LUMA_CALENDAR_URL, LUMA_PENDING};
use leptos::prelude::*;

/// The one action we want from a visitor. Until an event's own Luma page exists
/// the button subscribes them to the group calendar rather than telling them to
/// come back later: it is the only step taken now that still pays off when RSVP
/// opens.
#[component]
pub fn RsvpButton(#[prop(into)] luma_url: String) -> impl IntoView {
    let live = !luma_url.trim().is_empty();
    let (href, label) = if live {
        (luma_url, "Save your spot")
    } else {
        (
            LUMA_CALENDAR_URL.to_string(),
            "Get notified when RSVP opens",
        )
    };
    view! {
        <div>
            <a
                href=href
                target="_blank"
                rel="noopener"
                class="inline-flex items-center rounded-sm bg-accent px-6 py-3 font-mono text-sm text-paper transition hover:bg-accent-soft"
            >
                {label}
            </a>
            {(!live).then(|| view! {
                <p class="mt-2 font-mono text-xs text-mute">{LUMA_PENDING}</p>
            })}
        </div>
    }
}
