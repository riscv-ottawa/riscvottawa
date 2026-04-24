use leptos::prelude::*;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let on_click = move |_| {
        #[cfg(target_arch = "wasm32")]
        flip_theme();
    };

    view! {
        <button
            type="button"
            on:click=on_click
            aria-label="Toggle light and dark theme"
            title="Toggle theme"
            class="inline-flex h-8 w-8 items-center justify-center rounded-sm border border-line text-mute transition hover:border-accent hover:text-accent"
        >
            // Sun: shown in dark mode (target = switch to light).
            <svg
                viewBox="0 0 24 24"
                class="hidden h-4 w-4 dark:block"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
            >
                <circle cx="12" cy="12" r="4"/>
                <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41"/>
            </svg>
            // Moon: shown in light mode (target = switch to dark).
            <svg
                viewBox="0 0 24 24"
                class="block h-4 w-4 dark:hidden"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
            >
                <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
            </svg>
        </button>
    }
}

#[cfg(target_arch = "wasm32")]
fn flip_theme() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(html) = document.document_element() else {
        return;
    };
    let next = match html.get_attribute("data-theme").as_deref() {
        Some("light") => "dark",
        _ => "light",
    };
    let _ = html.set_attribute("data-theme", next);
    if let Ok(Some(storage)) = window.local_storage() {
        let _ = storage.set_item("theme", next);
    }
}
