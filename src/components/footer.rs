use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="mt-24 border-t border-line bg-surface">
            <div class="container-page flex flex-col gap-4 py-10 text-sm text-mute md:flex-row md:items-center md:justify-between">
                <div class="flex items-center gap-3 font-mono">
                    <span class="text-accent">"rvo:"</span>
                    <span>"riscvottawa"</span>
                </div>
                <p class="font-mono text-xs uppercase tracking-[0.3em]">
                    "open ISA, open community"
                </p>
            </div>
        </footer>
    }
}
