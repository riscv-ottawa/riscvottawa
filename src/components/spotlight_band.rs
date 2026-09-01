use crate::components::countdown::{start_ms, use_now, EventClock};
use crate::components::event_card::{format_event_date, format_event_month};
use crate::components::rsvp::RsvpButton;
use crate::content::Event;
use leptos::prelude::*;
use leptos_router::components::A;

/// The home page's headline draw for a spotlight event: the countdown the plain
/// t-minus strip used to carry, plus enough of the lineup to make the click
/// worth it. Sits above the hero and replaces the strip while it is up, so the
/// page never shows two clocks.
#[component]
pub fn SpotlightBand(event: Event) -> impl IntoView {
    let spotlight = event.spotlight.clone().expect("band needs a spotlight");
    let href = event.href();
    let names = spotlight.marquee_names();
    let now = use_now();

    // The right-hand slot already carries the month for an event we haven't
    // scheduled yet, so repeating it here would print the same thing twice.
    let when_line = match event.date.instant() {
        Some(_) => format!(
            "{} \u{00b7} {}",
            format_event_date(&event.date),
            event.location
        ),
        None => event.location.clone(),
    };

    view! {
        <section class="relative overflow-hidden border-b border-line bg-surface">
            <div class="pointer-events-none absolute inset-0 hero-grid"></div>
            <div class="container-page relative grid gap-10 py-10 md:grid-cols-[1fr_auto] md:items-end md:gap-16 md:py-14">
                <div>
                    <p class="font-mono text-xs uppercase tracking-[0.4em] text-accent">
                        "// " {spotlight.kicker}
                    </p>
                    <h2 class="mt-3 max-w-2xl font-mono text-2xl font-bold leading-tight text-ink md:text-4xl">
                        <A href=href.clone() attr:class="transition hover:text-accent-soft">
                            {spotlight.headline}
                        </A>
                    </h2>
                    <p class="mt-3 font-mono text-sm uppercase tracking-[0.2em] text-warm">
                        {when_line}
                    </p>
                    {(!names.is_empty()).then(|| view! {
                        <p class="mt-4 max-w-2xl text-sm text-mute">
                            "With " {names.join(" \u{00b7} ")}
                            <span class="text-mute/70">" and more to be announced"</span>
                        </p>
                    })}
                </div>

                // One right-aligned column: the clock on top as the thing that
                // draws the eye, then the action, then the way in. `text-right`
                // is inherited by the button's pending note.
                <div class="flex flex-col items-start gap-5 md:items-end md:text-right">
                    <Clock event=event.clone() now/>
                    <A href=href attr:class="inline-flex items-center justify-center rounded-sm border border-line px-5 py-3 font-mono text-sm text-ink transition hover:border-accent hover:text-accent-soft">
                        "Full agenda \u{2192}"
                    </A>
                    <RsvpButton luma_url=event.luma_url.clone()/>
                </div>
            </div>
        </section>
    }
}

/// A month-only event has nothing to count down to, so the slot shows the month
/// at the same weight instead. The band keeps its shape either way, and filling
/// in a date later swaps one for the other without the layout moving.
#[component]
fn Clock(event: Event, now: RwSignal<f64>) -> impl IntoView {
    match start_ms(&event) {
        Some(target_ms) => view! { <EventClock target_ms now large=true/> }.into_any(),
        None => view! {
            <div>
                <p class="font-mono text-3xl font-bold text-ink md:text-4xl">
                    {format_event_month(&event.date)}
                </p>
                <p class="mt-1 font-mono text-[0.6rem] uppercase tracking-[0.25em] text-mute">
                    "exact date to be confirmed"
                </p>
            </div>
        }
        .into_any(),
    }
}
