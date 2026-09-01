use crate::components::calendar::Calendar;
use crate::components::event_card::{format_event_date, EventCard};
use crate::components::prose::Prose;
use crate::components::rsvp::RsvpButton;
use crate::content::{get_events_page, Event, EventsPageData};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn Events() -> impl IntoView {
    let data = Resource::new_blocking(|| (), |_| async move { get_events_page().await });

    view! {
        <Title text="Events - RISC-V Ottawa"/>
        <section class="container-page py-16">
            <p class="font-mono text-xs uppercase tracking-[0.4em] text-accent">"/ events"</p>
            <h1 class="mt-4 font-mono text-4xl font-bold text-ink md:text-5xl">
                "Upcoming events"
            </h1>
            <p class="mt-4 max-w-2xl text-mute">
                "Talks, project nights, and hands-on sessions. Click a highlighted upcoming day for details, or jump to the calendar below."
            </p>

            <Suspense fallback=|| view! { <PageSkeleton/> }>
                {move || data.get().map(|result| match result {
                    Ok(d) => view! { <Body data=d/> }.into_any(),
                    Err(e) => view! {
                        <p class="mt-12 text-warm">
                            {format!("Could not load events: {e}")}
                        </p>
                    }.into_any(),
                })}
            </Suspense>
        </section>
    }
}

#[component]
fn Body(data: EventsPageData) -> impl IntoView {
    // A spotlight event gets pulled out and given the top of the page; leaving
    // it in the grid below as well would just show the same night twice.
    let featured = data
        .upcoming
        .iter()
        .find(|ev| ev.spotlight.is_some())
        .cloned();
    let featured_slug = featured.as_ref().map(|ev| ev.slug.clone());
    let rest: Vec<Event> = data
        .upcoming
        .into_iter()
        .filter(|ev| Some(&ev.slug) != featured_slug.as_ref())
        .collect();

    // Show events in the current month. If the current month has at most one
    // event, fall back to the next two upcoming events instead.
    let this_month: Vec<_> = rest
        .iter()
        .filter(|ev| ev.date.in_month(data.today.year(), data.today.month()))
        .cloned()
        .collect();
    let agenda = if this_month.len() > 1 {
        this_month
    } else {
        rest.into_iter().take(2).collect()
    };

    view! {
        {featured.map(|event| view! { <FeaturedEvent event/> })}

        <div class="mt-20">
            <p class="font-mono text-xs uppercase tracking-[0.3em] text-accent">"// agenda"</p>
            <h2 class="mt-2 font-mono text-2xl font-bold text-ink md:text-3xl">"Next up"</h2>
            {if agenda.is_empty() {
                view! {
                    <div class="mt-6 rounded-sm border border-line bg-surface p-6 text-mute">
                        <p class="text-sm">"No upcoming events on the calendar yet."</p>
                    </div>
                }.into_any()
            } else {
                view! {
                    <ul class="mt-6 grid gap-6 md:grid-cols-2">
                        {agenda.into_iter().map(|ev| view! { <EventCard event=ev/> }).collect::<Vec<_>>()}
                    </ul>
                }.into_any()
            }}
        </div>

        <div class="mt-12">
            <Calendar events=data.events today=data.today/>
        </div>
    }
}

#[component]
fn PageSkeleton() -> impl IntoView {
    view! {
        <div class="mt-12 h-96 animate-pulse rounded-sm border border-line bg-surface"></div>
    }
}

/// The spotlight event, pinned above the ordinary agenda. Enough to make the
/// click worth taking; the page itself carries the lineup.
#[component]
fn FeaturedEvent(event: Event) -> impl IntoView {
    let spotlight = event
        .spotlight
        .clone()
        .expect("featured events are spotlights");
    let when = format_event_date(&event.date);
    let href = event.href();
    let names = spotlight.marquee_names();

    view! {
        <div class="mt-12 rounded-sm border border-accent/60 bg-surface p-8 shadow-soft md:p-10">
            <p class="font-mono text-xs uppercase tracking-[0.4em] text-warm">
                "// " {spotlight.kicker}
            </p>
            <h2 class="mt-4 max-w-3xl font-mono text-2xl font-bold leading-tight text-ink md:text-3xl">
                <A href=href.clone() attr:class="transition hover:text-accent-soft">
                    {spotlight.headline}
                </A>
            </h2>
            <p class="mt-3 font-mono text-sm uppercase tracking-[0.2em] text-warm">
                {when} " \u{00b7} " {event.location.clone()}
            </p>
            <Prose text=spotlight.tagline class="mt-5 max-w-2xl text-ink/90"/>
            {(!names.is_empty()).then(|| view! {
                <p class="mt-4 max-w-2xl text-sm text-mute">"With " {names.join(" \u{00b7} ")}</p>
            })}
            <div class="mt-8 flex flex-wrap items-start gap-x-6 gap-y-3">
                <RsvpButton luma_url=event.luma_url.clone()/>
                <A href=href attr:class="inline-flex items-center justify-center rounded-sm border border-line px-5 py-3 font-mono text-sm text-ink transition hover:border-accent hover:text-accent-soft">
                    "Full agenda \u{2192}"
                </A>
            </div>
        </div>
    }
}
