use crate::components::modal::Modal;
use crate::components::prose::Prose;
use crate::content::{Level, Project};
use leptos::prelude::*;

// Compact, horizontally-scrollable overview of every project. Each tile shows
// the level, title, and a short summary; clicking opens the shared detail
// modal inline so visitors can browse the whole catalog without leaving the
// home page. Tiles snap when scrolled and rise into place on load.
#[component]
pub fn ProjectRail(items: Vec<Project>) -> impl IntoView {
    if items.is_empty() {
        return view! { <EmptyNote/> }.into_any();
    }

    let open = RwSignal::new(false);
    let selected: RwSignal<Option<Project>> = RwSignal::new(None);

    view! {
        <ul class="mt-8 flex snap-x snap-mandatory gap-4 overflow-x-auto py-2">
            {items
                .into_iter()
                .enumerate()
                .map(|(i, project)| {
                    let level_class = match project.level {
                        Level::Beginner => "text-accent",
                        Level::Intermediate => "text-warm",
                        Level::Advanced => "text-rust",
                    };
                    let level_label = project.level.label();
                    let title = project.title.clone();
                    let summary = project.summary.clone();
                    view! {
                        <li class="snap-start shrink-0">
                            <button
                                type="button"
                                on:click=move |_| {
                                    selected.set(Some(project.clone()));
                                    open.set(true);
                                }
                                style=format!("animation-delay: {}ms", i * 60)
                                class="animate-rail-rise flex h-full w-64 flex-col rounded-sm border border-line bg-surface p-5 text-left transition hover:-translate-y-1 hover:border-accent/60"
                            >
                                <div class=format!(
                                    "flex items-center gap-2 font-mono text-[0.65rem] uppercase tracking-[0.3em] {}",
                                    level_class
                                )>
                                    <span class="inline-block h-1.5 w-1.5 rounded-full bg-current"></span>
                                    {level_label}
                                </div>
                                <h3 class="mt-3 line-clamp-1 font-mono text-base font-semibold text-ink">
                                    {title}
                                </h3>
                                <p class="mt-2 line-clamp-2 text-sm text-mute">
                                    {summary}
                                </p>
                                <p class="mt-auto pt-4 font-mono text-[0.65rem] uppercase tracking-[0.2em] text-accent">
                                    "Details \u{2192}"
                                </p>
                            </button>
                        </li>
                    }
                })
                .collect::<Vec<_>>()}
        </ul>
        <Modal open>
            {move || selected.get().map(|p| {
                let prereqs = if p.prerequisites.is_empty() {
                    None
                } else {
                    Some(p.prerequisites.join(", "))
                };
                view! {
                    <h2 class="mt-3 font-mono text-2xl font-bold text-ink md:text-3xl">
                        {p.title}
                    </h2>
                    <Prose text=p.description class="mt-6 text-ink/90"/>
                    {prereqs.map(|pr| view! {
                        <p class="mt-6 font-mono text-xs uppercase tracking-[0.3em] text-mute">
                            "prereqs // "{pr}
                        </p>
                    })}
                    <div class="mt-8 flex flex-wrap gap-x-5 gap-y-2 font-mono text-xs uppercase tracking-[0.2em]">
                        {p.website_url.map(|url| view! {
                            <a
                                href=url
                                target="_blank"
                                rel="noopener"
                                class="text-accent hover:text-accent-soft"
                            >
                                "Project website"
                            </a>
                        })}
                        {p.contact_url.map(|url| view! {
                            <a href=url class="text-accent hover:text-accent-soft">
                                "Contact organizers"
                            </a>
                        })}
                    </div>
                }
                .into_any()
            })}
        </Modal>
    }
    .into_any()
}

#[component]
fn EmptyNote() -> impl IntoView {
    view! {
        <div class="mt-8 rounded-sm border border-line bg-surface p-6 text-mute">
            <p class="text-sm">"Nothing to show yet. Check back soon."</p>
        </div>
    }
}
