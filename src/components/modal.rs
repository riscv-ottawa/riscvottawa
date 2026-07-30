use leptos::prelude::*;

// Shared modal shell used by ProjectCard, EventCard, and ProjectRail. Renders
// nothing while `open` is false; when true, shows a centered overlay that
// closes on click-outside, the "x" button, or Escape. The body is provided via
// children, so callers control the inner markup.
#[component]
pub fn Modal(open: RwSignal<bool>, children: ChildrenFn) -> impl IntoView {
    register_escape(open);

    view! {
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
                    {children()}
                </div>
            </div>
        })}
    }
}

// Escape closes the modal. Registered for the component's lifetime and gated to
// the client build so it never touches `window()` during SSR.
#[cfg(feature = "hydrate")]
fn register_escape(open: RwSignal<bool>) {
    let handle = window_event_listener(leptos::ev::keydown, move |ev| {
        if ev.key() == "Escape" && open.get() {
            open.set(false);
        }
    });
    on_cleanup(move || handle.remove());
}

#[cfg(not(feature = "hydrate"))]
fn register_escape(_open: RwSignal<bool>) {}
