use crate::components::{footer::Footer, nav::Nav};
use crate::pages::{events::Events, home::Home, resources::Resources, trainings::Trainings};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, HashedStylesheet, Meta, MetaTags, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::use_location;
use leptos_router::StaticSegment;

const THEME_INIT_SCRIPT: &str = "(function(){try{var s=localStorage.getItem('theme');var t=(s==='light'||s==='dark')?s:(window.matchMedia('(prefers-color-scheme: light)').matches?'light':'dark');document.documentElement.setAttribute('data-theme',t)}catch(e){document.documentElement.setAttribute('data-theme','dark')}})();";

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <script inner_html=THEME_INIT_SCRIPT></script>
                <meta name="theme-color" content="#11141a" media="(prefers-color-scheme: dark)"/>
                <meta name="theme-color" content="#f7f5f0" media="(prefers-color-scheme: light)"/>
                <link rel="icon" type="image/svg+xml" href="/favicon.svg"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options.clone()/>
                <HashedStylesheet id="leptos" options=options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="RISC-V Ottawa"/>
        <Meta name="description" content="RISC-V Ottawa is a community of engineers, researchers, and students exploring the open RISC-V instruction set architecture."/>

        <Router>
            <HashScroll/>
            <div class="min-h-screen flex flex-col">
                <Nav/>
                <main class="flex-1">
                    <Routes fallback=NotFound>
                        <Route path=StaticSegment("") view=Home/>
                        <Route path=StaticSegment("trainings") view=Trainings/>
                        <Route path=StaticSegment("events") view=Events/>
                        <Route path=StaticSegment("resources") view=Resources/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}

// `leptos_router` intercepts `<a>` clicks for client-side navigation, which
// updates `location.hash` but skips the browser's native scroll-to-anchor and
// may reset scroll position on navigation. This effect reacts to hash changes
// and scrolls to the target element, after the router settles.
// This restores in-page anchor behavior for every `<a href="#...">`
// in the app after hydration.
#[component]
fn HashScroll() -> impl IntoView {
    let location = use_location();
    Effect::new(move |_| {
        let hash = location.hash.get();
        let id = hash.trim_start_matches('#').to_string();
        if id.is_empty() {
            return;
        }
        request_animation_frame(move || {
            if let Some(el) = document().get_element_by_id(&id) {
                el.scroll_into_view();
            }
        });
    });
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <section class="container-page py-24 text-center">
            <p class="font-mono text-sm uppercase tracking-[0.3em] text-accent">"404"</p>
            <h1 class="mt-4 font-mono text-4xl font-bold text-ink">"Page not found"</h1>
            <p class="mt-4 text-mute">"The page you are looking for does not exist."</p>
            <a href="/" class="mt-8 inline-block text-accent hover:text-accent-soft">"Return home"</a>
        </section>
    }
}
