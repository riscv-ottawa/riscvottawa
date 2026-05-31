use crate::components::countdown::Countdown;
use crate::components::cpu_widget::CpuWidget;
use crate::components::event_card::EventCard;
use crate::components::hero::Hero;
use crate::components::training_card::TrainingCard;
use crate::content::{
    get_featured_trainings, get_inaugural_event, get_upcoming_events, Event, Training,
};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn Home() -> impl IntoView {
    let events = Resource::new_blocking(|| (), |_| async move { get_upcoming_events().await });
    let trainings =
        Resource::new_blocking(|| (), |_| async move { get_featured_trainings().await });
    let inaugural = Resource::new_blocking(|| (), |_| async move { get_inaugural_event().await });

    view! {
        <Title text="RISC-V Ottawa"/>

        <Suspense fallback=|| ()>
            {move || inaugural.get().and_then(Result::ok).flatten().map(|ev| {
                view! { <Countdown event=ev/> }
            })}
        </Suspense>

        <Hero/>

        <CpuWidget/>

        <section class="container-page py-16">
            <StripHeader
                eyebrow="/ events"
                title="Next up"
                link_href="/events"
                link_text="See all events"
            />
            <Suspense fallback=|| view! { <StripSkeleton/> }>
                {move || events.get().map(|result| match result {
                    Ok(items) => view! { <EventStrip items=items/> }.into_any(),
                    Err(_) => view! { <EmptyNote/> }.into_any(),
                })}
            </Suspense>
        </section>

        <section class="container-page hairline py-16">
            <StripHeader
                eyebrow="/ trainings"
                title="Featured trainings"
                link_href="/trainings"
                link_text="Browse the catalog"
            />
            <Suspense fallback=|| view! { <StripSkeleton/> }>
                {move || trainings.get().map(|result| match result {
                    Ok(items) => view! { <TrainingStrip items=items/> }.into_any(),
                    Err(_) => view! { <EmptyNote/> }.into_any(),
                })}
            </Suspense>
        </section>
    }
}

#[component]
fn StripHeader(
    eyebrow: &'static str,
    title: &'static str,
    link_href: &'static str,
    link_text: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex items-baseline justify-between gap-4">
            <div>
                <p class="font-mono text-xs uppercase tracking-[0.4em] text-accent">{eyebrow}</p>
                <h2 class="mt-2 font-mono text-3xl font-bold text-ink md:text-4xl">{title}</h2>
            </div>
            <A
                href=link_href
                attr:class="hidden shrink-0 font-mono text-xs uppercase tracking-[0.2em] text-accent hover:text-accent-soft md:inline"
            >
                {link_text}""
            </A>
        </div>
    }
}

#[component]
fn EventStrip(items: Vec<Event>) -> impl IntoView {
    if items.is_empty() {
        return view! { <EmptyNote/> }.into_any();
    }
    view! {
        <ul class="mt-8 grid gap-6 md:grid-cols-3">
            {items
                .into_iter()
                .map(|ev| view! { <EventCard event=ev/> })
                .collect::<Vec<_>>()}
        </ul>
    }
    .into_any()
}

#[component]
fn TrainingStrip(items: Vec<Training>) -> impl IntoView {
    if items.is_empty() {
        return view! { <EmptyNote/> }.into_any();
    }
    view! {
        <ul class="mt-8 grid gap-6 md:grid-cols-3">
            {items
                .into_iter()
                .map(|t| view! { <TrainingCard training=t/> })
                .collect::<Vec<_>>()}
        </ul>
    }
    .into_any()
}

#[component]
fn StripSkeleton() -> impl IntoView {
    view! {
        <div class="mt-8 grid gap-6 md:grid-cols-3">
            <div class="h-40 animate-pulse rounded-sm border border-line bg-surface"></div>
            <div class="h-40 animate-pulse rounded-sm border border-line bg-surface"></div>
            <div class="h-40 animate-pulse rounded-sm border border-line bg-surface"></div>
        </div>
    }
}

#[component]
fn EmptyNote() -> impl IntoView {
    view! {
        <div class="mt-8 rounded-sm border border-line bg-surface p-6 text-mute">
            <p class="text-sm">"Nothing to show yet. Check back soon."</p>
        </div>
    }
}
