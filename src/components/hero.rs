use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="relative overflow-hidden">
            <div class="pointer-events-none absolute inset-0 hero-grid"></div>
            <div class="container-page relative py-24 md:py-32">
                <p class="font-mono text-xs uppercase tracking-[0.4em] text-accent">
                    "RISC-V Ottawa // est. 2026"
                </p>
                <h1 class="mt-6 max-w-4xl font-mono text-4xl font-bold leading-[1.05] text-ink md:text-5xl">
                   "A community in Ottawa for everyone curious about "
                   <a
                       href="https://riscv.org/"
                       target="_blank"
                       rel="noopener"
                       class="text-warm no-underline hover:underline"
                   >
                       "RISC-V"
                   </a>
                </h1>
                <p class="mt-6 max-w-2xl text-lg text-mute">
                    "Calling all engineers, researchers, and students in Ottawa to build, break, and learn on RISC-V. From look-up tables to operating systems, let's explore it all."
                </p>
                <div class="mt-10 flex flex-wrap items-center gap-4 font-mono text-sm">
                    <A
                        href="/events"
                        attr:class="inline-flex items-center gap-2 rounded-sm bg-accent px-5 py-3 text-paper transition hover:bg-accent-soft"
                    >
                        "See upcoming events"
                    </A>
                    <A
                        href="/projects"
                        attr:class="inline-flex items-center gap-2 rounded-sm border border-line px-5 py-3 text-ink transition hover:border-accent hover:text-accent-soft"
                    >
                        "Browse projects"
                    </A>
                </div>
            </div>
        </section>
    }
}
