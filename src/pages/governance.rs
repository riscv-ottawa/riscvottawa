use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

const GOVERNANCE_URL: &str = "https://github.com/riscv-ottawa/governance";

#[component]
pub fn GovernanceOverview() -> impl IntoView {
    view! {
        <Title text="Governance - RISC-V Ottawa"/>
        <section class="container-page py-16">
            <p class="font-mono text-xs uppercase tracking-[0.4em] text-accent">"/ governance"</p>
            <h1 class="mt-4 font-mono text-4xl font-bold text-ink md:text-5xl">
                "Governance"
            </h1>
            <p class="mt-4 max-w-2xl text-mute">
                "RISC-V Ottawa is a volunteer-run, vendor-neutral community. How we are "
                "organized, how decisions get made, and the roles that keep things running "
                "are documented in the open."
            </p>
            <p class="mt-4 max-w-2xl text-mute">
                "The full governance model lives in our public "
                <a
                    href=GOVERNANCE_URL
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-accent hover:text-accent-soft"
                >
                    "governance repository"
                </a>
                " on GitHub. Read it, open an issue, or send a pull request to take part."
            </p>

            <div class="mt-8 flex flex-wrap gap-4 font-mono text-sm">
                <a
                    href=GOVERNANCE_URL
                    target="_blank"
                    rel="noopener noreferrer"
                    class="rounded-sm border border-accent px-4 py-2 font-semibold text-accent transition-colors hover:bg-accent hover:text-paper"
                >
                    "Read the governance repo"
                </a>
                <A
                    href="/values"
                    attr:class="rounded-sm border border-accent px-4 py-2 font-semibold text-accent transition-colors hover:bg-accent hover:text-paper"
                >
                    "Our values"
                </A>
                <A
                    href="/code-of-conduct"
                    attr:class="rounded-sm border border-accent px-4 py-2 font-semibold text-accent transition-colors hover:bg-accent hover:text-paper"
                >
                    "Code of conduct"
                </A>
            </div>
        </section>
    }
}
