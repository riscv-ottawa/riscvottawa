use crate::content::Event;
use leptos::prelude::*;

// Current wall-clock time in milliseconds since the Unix epoch. On the client
// we read the browser clock; during SSR we read the system clock. The interval
// that drives the countdown only ever runs on the client (effects don't run on
// the server), so the `js_sys` path is wasm-only.
#[cfg(feature = "hydrate")]
fn now_ms() -> f64 {
    js_sys::Date::now()
}

#[cfg(not(feature = "hydrate"))]
fn now_ms() -> f64 {
    time::OffsetDateTime::now_utc().unix_timestamp_nanos() as f64 / 1_000_000.0
}

// Chrome (since v88) throttles background-tab timers to roughly once a minute
// after the tab has been hidden a few minutes, and energy-saver modes can stop
// them entirely. That makes the interval-driven countdown look frozen even
// though the wall clock is fine. Re-read the clock the moment the tab becomes
// visible again so the display snaps to the correct time without waiting for
// the throttled timer to fire. Only meaningful on the client.
#[cfg(feature = "hydrate")]
fn refresh_when_visible(now: RwSignal<f64>) {
    use send_wrapper::SendWrapper;
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    let doc = document();
    let listener = Closure::<dyn FnMut()>::new(move || now.set(now_ms()));
    doc.add_event_listener_with_callback("visibilitychange", listener.as_ref().unchecked_ref())
        .expect("failed to register visibilitychange listener");

    // The wasm `Closure` is not `Send + Sync`, but `on_cleanup` requires it.
    // We only ever touch it on the (single) browser thread, so wrapping it is
    // sound; this mirrors how leptos itself manages event listeners.
    let guard = SendWrapper::new((doc, listener));
    on_cleanup(move || {
        let (doc, listener) = guard.take();
        let _ = doc.remove_event_listener_with_callback(
            "visibilitychange",
            listener.as_ref().unchecked_ref(),
        );
    });
}

#[cfg(not(feature = "hydrate"))]
fn refresh_when_visible(_now: RwSignal<f64>) {}

#[component]
pub fn Countdown(event: Event) -> impl IntoView {
    let target_ms = event.date.unix_timestamp() as f64 * 1000.0;
    let title = event.title.clone();

    let now = RwSignal::new(now_ms());

    // Tick once a second on the client. The effect (and therefore the interval)
    // never runs during SSR, and we clear it when the component unmounts.
    Effect::new(move |_| {
        let handle =
            set_interval_with_handle(move || now.set(now_ms()), std::time::Duration::from_secs(1))
                .expect("failed to start countdown interval");
        on_cleanup(move || handle.clear());

        // The interval alone is not enough: browsers throttle it in the
        // background, so also resync whenever the tab is shown again.
        refresh_when_visible(now);
    });

    let remaining = move || ((target_ms - now.get()) / 1000.0).max(0.0) as i64;
    let is_done = move || target_ms - now.get() <= 0.0;
    let days = move || remaining() / 86_400;
    let hours = move || (remaining() % 86_400) / 3_600;
    let minutes = move || (remaining() % 3_600) / 60;
    let seconds = move || remaining() % 60;

    view! {
        <section class="border-b border-line bg-surface">
            <div class="container-page flex flex-col items-start gap-4 py-5 md:flex-row md:items-center md:justify-between">
                <div>
                    <p class="font-mono text-xs uppercase tracking-[0.4em] text-accent">"// t-minus"</p>
                    <p class="mt-1 font-mono text-sm text-ink">
                        "Counting down to the "
                        <span class="text-warm uppercase">{title}</span>
                    </p>
                </div>
                {move || if is_done() {
                    view! {
                        <a
                            href="/events"
                            class="font-mono text-sm uppercase tracking-[0.2em] text-accent hover:text-accent-soft"
                        >
                            "It's happening now -- see details"
                        </a>
                    }.into_any()
                } else {
                    view! {
                        <div class="flex items-end gap-3 font-mono md:gap-4">
                            <Unit value=Signal::derive(days) label="days"/>
                            <Colon/>
                            <Unit value=Signal::derive(hours) label="hrs"/>
                            <Colon/>
                            <Unit value=Signal::derive(minutes) label="min"/>
                            <Colon/>
                            <Unit value=Signal::derive(seconds) label="sec"/>
                        </div>
                    }.into_any()
                }}
            </div>
        </section>
    }
}

#[component]
fn Unit(value: Signal<i64>, label: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center">
            <span class="text-2xl font-bold tabular-nums text-ink md:text-3xl">
                {move || format!("{:02}", value.get())}
            </span>
            <span class="mt-1 text-[0.6rem] uppercase tracking-[0.25em] text-mute">{label}</span>
        </div>
    }
}

#[component]
fn Colon() -> impl IntoView {
    view! { <span class="pb-4 text-xl font-bold text-line md:text-2xl">":"</span> }
}
