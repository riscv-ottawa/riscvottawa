use crate::content::{Level, Project};
use leptos::prelude::*;

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    let level_class = match project.level {
        Level::Beginner => "text-accent",
        Level::Intermediate => "text-warm",
        Level::Advanced => "text-rust",
    };
    let prereqs = if project.prerequisites.is_empty() {
        None
    } else {
        Some(project.prerequisites.join(", "))
    };
    let level_label = project.level.label();

    let open = RwSignal::new(false);

    let m_title = project.title.clone();
    let m_description = project.description.clone();
    let m_prereqs = prereqs.clone();
    let m_website = project.website_url.clone();
    let m_contact = project.contact_url.clone();

    view! {
        <li class="flex h-full flex-col rounded-sm border border-line bg-surface p-6 transition hover:border-accent/60">
            <div class="flex items-center gap-3 font-mono text-xs uppercase tracking-[0.3em]">
                <span class=level_class>{level_label}</span>
            </div>
            <h3 class="mt-3 font-mono text-xl font-semibold text-ink">{project.title}</h3>
            <p class="mt-2 text-sm text-ink/90">{project.summary}</p>
            {prereqs.map(|p| view! {
                <p class="mt-4 font-mono text-xs uppercase tracking-[0.3em] text-mute">
                    "prereqs // "{p}
                </p>
            })}
            <div class="mt-auto flex flex-wrap items-center gap-x-5 gap-y-2 pt-5 font-mono text-xs uppercase tracking-[0.2em]">
                <button
                    type="button"
                    on:click=move |_| open.set(true)
                    class="text-accent hover:text-accent-soft underline"
                >
                    "DETAILS"
                </button>
                {project.website_url.clone().map(|url| view! {
                    <a
                        href=url
                        target="_blank"
                        rel="noopener"
                        class="text-accent hover:text-accent-soft"
                    >
                        "Website"
                    </a>
                })}
                {project.contact_url.map(|url| view! {
                    <a
                        href=url
                        class="text-accent hover:text-accent-soft"
                    >
                        "Contact"
                    </a>
                })}
            </div>
            {move || open.get().then(|| view! {
                <div
                    role="dialog"
                    aria-modal="true"
                    on:click=move |_| open.set(false)
                    class="fixed inset-0 z-50 flex items-center justify-center bg-paper/80 p-4"
                >
                    <div
                        on:click=|ev: leptos::ev::MouseEvent| ev.stop_propagation()
                        class="relative max-h-[85vh] w-full max-w-2xl overflow-y-auto rounded-sm border border-line bg-surface p-8 shadow-soft"
                    >
                        <button
                            type="button"
                            aria-label="Close details"
                            on:click=move |_| open.set(false)
                            class="absolute right-4 top-4 rounded-sm border border-line px-2 py-0.5 font-mono text-sm text-mute hover:border-accent/60 hover:text-accent"
                        >
                            "x"
                        </button>
                        <h2 class="mt-3 font-mono text-2xl font-bold text-ink md:text-3xl">
                            {m_title.clone()}
                        </h2>
                        <p class="mt-6 text-ink/90">{m_description.clone()}</p>
                        {m_prereqs.clone().map(|p| view! {
                            <p class="mt-6 font-mono text-xs uppercase tracking-[0.3em] text-mute">
                                "prereqs // "{p}
                            </p>
                        })}
                        <div class="mt-8 flex flex-wrap gap-x-5 gap-y-2 font-mono text-xs uppercase tracking-[0.2em]">
                            {m_website.clone().map(|url| view! {
                                <a
                                    href=url
                                    target="_blank"
                                    rel="noopener"
                                    class="text-accent hover:text-accent-soft"
                                >
                                    "Project website"
                                </a>
                            })}
                            {m_contact.clone().map(|url| view! {
                                <a
                                    href=url
                                    class="text-accent hover:text-accent-soft"
                                >
                                    "Contact organizers"
                                </a>
                            })}
                        </div>
                    </div>
                </div>
            })}
        </li>
    }
}
