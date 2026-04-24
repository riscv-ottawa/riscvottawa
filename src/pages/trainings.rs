use crate::components::training_card::TrainingCard;
use crate::content::{get_trainings, Training};
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Trainings() -> impl IntoView {
    let trainings = Resource::new_blocking(|| (), |_| async move { get_trainings().await });

    view! {
        <Title text="Trainings - RISC-V Ottawa"/>
        <section class="container-page py-16">
            <p class="font-mono text-xs uppercase tracking-[0.4em] text-accent">"/ trainings"</p>
            <h1 class="mt-4 font-mono text-4xl font-bold text-ink md:text-5xl">
                "Public trainings"
            </h1>
            <p class="mt-4 max-w-2xl text-mute">
                "A catalog of trainings we run for the community. Each is open to the public and free unless marked otherwise."
            </p>

            <Suspense fallback=|| view! { <ListSkeleton/> }>
                {move || trainings.get().map(|result| match result {
                    Ok(items) => view! { <TrainingList items=items/> }.into_any(),
                    Err(e) => view! {
                        <p class="mt-12 text-warm">
                            {format!("Could not load trainings: {e}")}
                        </p>
                    }.into_any(),
                })}
            </Suspense>
        </section>
    }
}

#[component]
fn ListSkeleton() -> impl IntoView {
    view! {
        <div class="mt-12 h-24 animate-pulse rounded-sm border border-line bg-surface"></div>
    }
}

#[component]
fn TrainingList(items: Vec<Training>) -> impl IntoView {
    if items.is_empty() {
        return view! {
            <div class="mt-16 rounded-sm border border-line bg-surface p-10 text-mute">
                <p class="font-mono text-xs uppercase tracking-[0.3em] text-accent">"empty"</p>
                <p class="mt-4">"No trainings listed yet."</p>
            </div>
        }
        .into_any();
    }
    view! {
        <ul class="mt-12 grid gap-6 md:grid-cols-2">
            {items
                .into_iter()
                .map(|t| view! { <TrainingCard training=t/> })
                .collect::<Vec<_>>()}
        </ul>
    }
    .into_any()
}
