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
                    <nav class="flex items-center gap-4 font-mono text-sm lg:gap-8">
                        <A
                            href="/projects"
                            attr:class="hidden text-mute transition-colors hover:text-ink lg:inline"
                        >
                            "projects"
                        </A>
                        <A
                            href="/events"
                            attr:class="hidden text-mute transition-colors hover:text-ink lg:inline"
                        >
                            "events"
                        </A>
                        <A
                            href="/resources"
                            attr:class="hidden text-mute transition-colors hover:text-ink lg:inline"
                        >
                            "resources"
                        </A>
                        <div class="group relative hidden lg:inline-block">
                            <button
                                type="button"
                                class="text-mute underline transition-colors hover:text-ink group-hover:text-ink"
                                aria-haspopup="true"
                            >
                                "governance"
                            </button>
                            <div class="invisible absolute left-0 top-full z-50 flex min-w-44 flex-col rounded-sm border border-line bg-paper pt-2 opacity-0 transition group-hover:visible group-hover:opacity-100 group-focus-within:visible group-focus-within:opacity-100">
                                <A
                                    href="/governance"
                                    attr:class="px-4 py-2 text-mute transition-colors hover:text-ink"
                                >
                                    "overview"
                                </A>
                                <A
                                    href="/values"
                                    attr:class="px-4 py-2 text-mute transition-colors hover:text-ink"
                                >
                                    "values"
                                </A>
                                <A
                                    href="/code-of-conduct"
                                    attr:class="px-4 py-2 text-mute transition-colors hover:text-ink"
                                >
                                    "code of conduct"
                                </A>
                            </div>
                        </div>
                        <div class="hidden items-center gap-2 sm:flex">
                            <SocialLinks/>
                        </div>
                        <div class="flex items-center border-line pl-1 sm:border-l sm:pl-4">
                            <ThemeToggle/>
                        </div>
                        <button
                            type="button"
                            class="inline-flex h-8 w-8 items-center justify-center rounded-sm border border-line text-mute transition hover:border-accent hover:text-accent lg:hidden"
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
                    let base = "overflow-hidden transition-[max-height] duration-200 ease-out lg:hidden";
                    if open.get() {
                        format!("{base} max-h-[40rem] hairline")
                    } else {
                        format!("{base} max-h-0")
                    }
                }
            >
                <nav class="container-page flex flex-col font-mono text-sm">
                    <A
                        href="/projects"
                        attr:class="border-b border-line py-3 text-mute transition-colors hover:text-ink"
                        on:click=close
                    >
                        "projects"
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
                        attr:class="border-b border-line py-3 text-mute transition-colors hover:text-ink"
                        on:click=close
                    >
                        "resources"
                    </A>
                    <p class="border-b border-line py-3 font-mono text-xs uppercase tracking-[0.3em] text-accent">
                        "governance"
                    </p>
                    <A
                        href="/governance"
                        attr:class="border-b border-line py-3 pl-3 text-mute transition-colors hover:text-ink"
                        on:click=close
                    >
                        "overview"
                    </A>
                    <A
                        href="/values"
                        attr:class="border-b border-line py-3 pl-3 text-mute transition-colors hover:text-ink"
                        on:click=close
                    >
                        "values"
                    </A>
                    <A
                        href="/code-of-conduct"
                        attr:class="py-3 pl-3 text-mute transition-colors hover:text-ink"
                        on:click=close
                    >
                        "code of conduct"
                    </A>
                    <div class="flex flex-wrap gap-3 border-t border-line py-4 sm:hidden">
                        <SocialLinks/>
                    </div>
                </nav>
            </div>
        </header>
    }
}

#[component]
fn SocialLinks() -> impl IntoView {
    let tooltip_class = "pointer-events-none absolute left-1/2 top-full z-50 mt-2 -translate-x-1/2 whitespace-nowrap rounded-sm border border-line bg-paper px-2 py-1 text-xs uppercase tracking-[0.2em] text-mute opacity-0 transition group-hover:opacity-100";
    view! {
        <a
            href="https://discord.gg/EfryE4wfk4"
            target="_blank"
            rel="noopener noreferrer"
            aria-label="riscv-ottawa on Discord"
            class="group relative inline-flex items-center justify-center rounded-sm border border-line px-3 py-1.5 text-mute transition hover:border-accent hover:text-accent-soft"
        >
            <svg viewBox="0 0 24 24" class="h-4 w-4 fill-current" aria-hidden="true">
                <path d="M20.317 4.369a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.369a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028 14.09 14.09 0 0 0 1.226-1.994.076.076 0 0 0-.041-.106 13.107 13.107 0 0 1-1.872-.892.077.077 0 0 1-.008-.128c.126-.094.252-.192.372-.291a.074.074 0 0 1 .077-.01c3.927 1.793 8.18 1.793 12.061 0a.074.074 0 0 1 .078.009c.12.099.246.198.373.292a.077.077 0 0 1-.006.127 12.299 12.299 0 0 1-1.873.891.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.029 19.84 19.84 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03ZM8.02 15.331c-1.182 0-2.157-1.085-2.157-2.419 0-1.333.956-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.956 2.418-2.157 2.418Zm7.974 0c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.955-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.946 2.418-2.157 2.418Z"/>
            </svg>
            <span class=tooltip_class>"discord"</span>
        </a>
        <a
            href="https://github.com/riscv-ottawa"
            target="_blank"
            rel="noopener noreferrer"
            aria-label="riscv-ottawa on GitHub"
            class="group relative inline-flex items-center justify-center rounded-sm border border-line px-3 py-1.5 text-mute transition hover:border-accent hover:text-accent-soft"
        >
            <svg viewBox="0 0 24 24" class="h-4 w-4 fill-current" aria-hidden="true">
                <path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/>
            </svg>
            <span class=tooltip_class>"github"</span>
        </a>
        <a
            href="https://www.linkedin.com/company/riscv-ottawa/"
            target="_blank"
            rel="noopener noreferrer"
            aria-label="riscv-ottawa on LinkedIn"
            class="group relative inline-flex items-center justify-center rounded-sm border border-line px-3 py-1.5 text-mute transition hover:border-accent hover:text-accent-soft"
        >
            <svg viewBox="0 0 24 24" class="h-4 w-4 fill-current" aria-hidden="true">
                <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 0 1-2.063-2.065 2.064 2.064 0 1 1 2.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>
            </svg>
            <span class=tooltip_class>"linkedin"</span>
        </a>
    }
}
