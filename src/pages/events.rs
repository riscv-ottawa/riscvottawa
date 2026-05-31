use crate::components::calendar::Calendar;
use crate::components::event_card::EventCard;
use crate::content::{get_events_page, EventsPageData};
use leptos::prelude::*;
use leptos_meta::Title;

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
                "Training sessions, talks, and project nights. Click a highlighted upcoming day for details, or jump to the calendar below."
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
    // Show events in the current month. If the current month has at most one
    // event, fall back to the next two upcoming events instead.
    let this_month: Vec<_> = data
        .upcoming
        .iter()
        .filter(|ev| ev.date.year() == data.today.year() && ev.date.month() == data.today.month())
        .cloned()
        .collect();
    let agenda = if this_month.len() > 1 {
        this_month
    } else {
        data.upcoming.into_iter().take(2).collect()
    };

    view! {
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
