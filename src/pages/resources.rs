use crate::content::{get_resource_sections, ResourceSection};
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Resources() -> impl IntoView {
    let sections = Resource::new_blocking(|| (), |_| async move { get_resource_sections().await });

    view! {
        <Title text="Resources - RISC-V Ottawa"/>
        <section class="container-page py-16">
            <p class="font-mono text-xs uppercase tracking-[0.4em] text-accent">"/ resources"</p>
            <h1 class="mt-4 font-mono text-4xl font-bold text-ink md:text-5xl">
                "Resources"
            </h1>
            <p class="mt-4 max-w-2xl text-mute">
                "A curated directory of RISC-V related links: specifications, books, tools, communities, and more."
            </p>

            <Suspense fallback=|| view! { <ListSkeleton/> }>
                {move || sections.get().map(|result| match result {
                    Ok(items) => view! { <ResourceLayout sections=items/> }.into_any(),
                    Err(e) => view! {
                        <p class="mt-12 text-warm">
                            {format!("Could not load resources: {e}")}
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
fn ResourceLayout(sections: Vec<ResourceSection>) -> impl IntoView {
    if sections.is_empty() {
        return view! {
            <div class="mt-16 rounded-sm border border-line bg-surface p-10 text-mute">
                <p class="font-mono text-xs uppercase tracking-[0.3em] text-accent">"empty"</p>
                <p class="mt-4">"No resources listed yet."</p>
            </div>
        }
        .into_any();
    }

    let toc_entries: Vec<(String, String)> = sections
        .iter()
        .map(|s| (s.slug.clone(), s.title.clone()))
        .collect();

    view! {
        <div class="mt-12 flex flex-col gap-10 md:flex-row md:items-start md:gap-12">
            <Toc entries=toc_entries/>
            <div class="flex-1 space-y-12">
                {sections
                    .into_iter()
                    .map(|s| view! { <SectionBlock section=s/> })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
    .into_any()
}

#[component]
fn Toc(entries: Vec<(String, String)>) -> impl IntoView {
    view! {
        <aside class="md:order-last md:w-56 md:shrink-0 md:sticky md:top-20 md:self-start">
            <p class="font-mono text-xs uppercase tracking-[0.3em] text-accent">"on this page"</p>
            <nav class="mt-4 hairline">
                <ol class="flex flex-col font-mono text-sm">
                    {entries
                        .into_iter()
                        .map(|(slug, title)| view! {
                            <li>
                                <a
                                    href=format!("#{slug}")
                                    class="block border-b border-line py-2 text-mute transition-colors hover:text-ink"
                                >
                                    {title}
                                </a>
                            </li>
                        })
                        .collect::<Vec<_>>()}
                </ol>
            </nav>
        </aside>
    }
}

#[component]
fn SectionBlock(section: ResourceSection) -> impl IntoView {
    let ResourceSection { slug, title, links } = section;
    view! {
        <section id=slug class="scroll-mt-20">
            <h2 class="font-mono text-2xl font-semibold text-ink md:text-3xl">{title}</h2>
            {if links.is_empty() {
                view! {
                    <p class="mt-4 text-mute">"No links in this section yet."</p>
                }.into_any()
            } else {
                view! {
                    <ul class="mt-4 hairline">
                        {links
                            .into_iter()
                            .map(|link| view! {
                                <li class="border-b border-line">
                                    <a
                                        href=link.href
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="group block py-3 text-mute transition-colors hover:text-accent-soft"
                                    >
                                        <span class="block text-ink transition-colors group-hover:text-accent-soft">
                                            {link.name}
                                        </span>
                                        {link.description.map(|d| view! {
                                            <span class="mt-1 block text-sm text-mute">
                                                {d}
                                            </span>
                                        })}
                                    </a>
                                </li>
                            })
                            .collect::<Vec<_>>()}
                    </ul>
                }.into_any()
            }}
        </section>
    }
}
