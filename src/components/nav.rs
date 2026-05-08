use crate::components::theme_toggle::ThemeToggle;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Nav() -> impl IntoView {
    let open = RwSignal::new(false);
    let toggle = move |_| open.update(|v| *v = !*v);
    let close = move |_| open.set(false);

    view! {
        <header class="sticky top-0 z-40 bg-paper/80 backdrop-blur">
            <div class="hairline">
                <div class="container-page flex h-16 items-center justify-between gap-3">
                    <A href="/" attr:class="group flex items-center gap-3">
                        <span class="font-mono text-sm text-accent">"rvo:"</span>
                        <span class="font-mono text-base font-semibold tracking-tight text-ink group-hover:text-accent-soft">
                            "riscvottawa"
                        </span>
                    </A>
                    <nav class="flex items-center gap-4 font-mono text-sm md:gap-8">
                        <A
                            href="/trainings"
                            attr:class="hidden text-mute transition-colors hover:text-ink md:inline"
                        >
                            "trainings"
                        </A>
                        <A
                            href="/events"
                            attr:class="hidden text-mute transition-colors hover:text-ink md:inline"
                        >
                            "events"
                        </A>
                        <A
                            href="/resources"
                            attr:class="hidden text-mute transition-colors hover:text-ink md:inline"
                        >
                            "resources"
                        </A>
                        <ThemeToggle/>
                        <a
                            href="https://github.com/riscv-ottawa"
                            target="_blank"
                            rel="noopener noreferrer"
                            aria-label="riscv-ottawa on GitHub"
                            class="inline-flex items-center gap-2 rounded-sm border border-line px-3 py-1.5 text-mute transition hover:border-accent hover:text-accent-soft"
                        >
                            <svg
                                viewBox="0 0 24 24"
                                class="h-4 w-4 fill-current"
                                aria-hidden="true"
                            >
                                <path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/>
                            </svg>
                            <span class="hidden text-xs uppercase tracking-[0.2em] sm:inline">
                                "github"
                            </span>
                        </a>
                        <button
                            type="button"
                            class="inline-flex h-8 w-8 items-center justify-center rounded-sm border border-line text-mute transition hover:border-accent hover:text-accent md:hidden"
                            on:click=toggle
                            aria-label="Toggle navigation menu"
                            aria-controls="mobile-nav"
                            aria-expanded=move || if open.get() { "true" } else { "false" }
                        >
                            <svg
                                viewBox="0 0 24 24"
                                class="h-4 w-4"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                aria-hidden="true"
                            >
                                <path d=move || {
                                    if open.get() {
                                        "M6 6l12 12M18 6L6 18"
                                    } else {
                                        "M4 7h16M4 12h16M4 17h16"
                                    }
                                }/>
                            </svg>
                        </button>
                    </nav>
                </div>
            </div>
            <div
                id="mobile-nav"
                class=move || {
                    let base = "overflow-hidden transition-[max-height] duration-200 ease-out md:hidden";
                    if open.get() {
                        format!("{base} max-h-40 hairline")
                    } else {
                        format!("{base} max-h-0")
                    }
                }
            >
                <nav class="container-page flex flex-col font-mono text-sm">
                    <A
                        href="/trainings"
                        attr:class="border-b border-line py-3 text-mute transition-colors hover:text-ink"
                        on:click=close
                    >
                        "trainings"
                    </A>
                    <A
                        href="/events"
                        attr:class="border-b border-line py-3 text-mute transition-colors hover:text-ink"
                        on:click=close
                    >
                        "events"
                    </A>
                    <A
                        href="/resources"
                        attr:class="py-3 text-mute transition-colors hover:text-ink"
                        on:click=close
                    >
                        "resources"
                    </A>
                </nav>
            </div>
        </header>
    }
}
