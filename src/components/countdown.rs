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

// The interval can freeze for two reasons. Chrome (since v88) throttles
// background-tab timers to roughly once a minute after the tab has been hidden
// a few minutes, and energy-saver modes can stop them entirely; this surfaces
// as a stale countdown that snaps back once the tab is shown (visibilitychange).
// The back/forward cache is worse: the browser freezes the page with timers
// paused and the old time still painted, then restores it later. That restore
// fires `pageshow` (with `persisted = true`), and unlike visibilitychange it is
// reliable across browsers, so without it some users see a stuck timer until a
// hard refresh. Re-read the clock on both so the display resyncs immediately.
// Only meaningful on the client.
#[cfg(feature = "hydrate")]
fn refresh_when_visible(now: RwSignal<f64>) {
    use send_wrapper::SendWrapper;
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    let doc = document();
    let win = window();

    let on_visible = Closure::<dyn FnMut()>::new(move || now.set(now_ms()));
    doc.add_event_listener_with_callback("visibilitychange", on_visible.as_ref().unchecked_ref())
        .expect("failed to register visibilitychange listener");

    let on_show = Closure::<dyn FnMut()>::new(move || now.set(now_ms()));
    win.add_event_listener_with_callback("pageshow", on_show.as_ref().unchecked_ref())
        .expect("failed to register pageshow listener");

    // The wasm `Closure` is not `Send + Sync`, but `on_cleanup` requires it.
    // We only ever touch it on the (single) browser thread, so wrapping it is
    // sound; this mirrors how leptos itself manages event listeners.
    let guard = SendWrapper::new((doc, win, on_visible, on_show));
    on_cleanup(move || {
        let (doc, win, on_visible, on_show) = guard.take();
        let _ = doc.remove_event_listener_with_callback(
            "visibilitychange",
            on_visible.as_ref().unchecked_ref(),
        );
        let _ =
            win.remove_event_listener_with_callback("pageshow", on_show.as_ref().unchecked_ref());
    });
}

#[cfg(not(feature = "hydrate"))]
fn refresh_when_visible(_now: RwSignal<f64>) {}

const HOUR_MS: f64 = 3_600_000.0;
// Once an event starts, keep showing "happening now" for this long, then
// "just finished" until the retire window elapses and we advance to the next.
const HAPPENING_WINDOW_MS: f64 = 4.0 * HOUR_MS;
const RETIRE_WINDOW_MS: f64 = 12.0 * HOUR_MS;

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    // Before the event starts: show the ticking countdown.
    Counting,
    // Within HAPPENING_WINDOW_MS after the start.
    Happening,
    // Between HAPPENING_WINDOW_MS and RETIRE_WINDOW_MS after the start.
    Finished,
}

// Title plus start time in milliseconds since the epoch: everything the
// countdown needs. Events that only have a month can't be counted down to, so
// they never make it into this list.
#[derive(Clone)]
struct Scheduled {
    title: String,
    start_ms: f64,
}

#[component]
pub fn Countdown(events: Vec<Event>) -> impl IntoView {
    let events: Vec<Scheduled> = events
        .into_iter()
        .filter_map(|e| {
            Some(Scheduled {
                start_ms: e.date.instant()?.unix_timestamp() as f64 * 1000.0,
                title: e.title,
            })
        })
        .collect();
    let events = StoredValue::new(events);
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

    // The active event is the soonest one whose retire window hasn't elapsed.
    // Events arrive sorted ascending, so this advances to the next event as
    // each one ages past RETIRE_WINDOW_MS. `None` once nothing is left to show.
    let active = Memo::new(move |_| {
        let now = now.get();
        events.with_value(|evs| evs.iter().position(|e| now < e.start_ms + RETIRE_WINDOW_MS))
    });

    move || {
        active.get().map(|idx| {
            let event = events.with_value(|evs| evs[idx].clone());
            let target_ms = event.start_ms;
            let title = event.title;

            let remaining = move || ((target_ms - now.get()) / 1000.0).max(0.0) as i64;
            let days = move || remaining() / 86_400;
            let hours = move || (remaining() % 86_400) / 3_600;
            let minutes = move || (remaining() % 3_600) / 60;
            let seconds = move || remaining() % 60;

            let phase = Memo::new(move |_| {
                let elapsed = now.get() - target_ms;
                if elapsed < 0.0 {
                    Phase::Counting
                } else if elapsed < HAPPENING_WINDOW_MS {
                    Phase::Happening
                } else {
                    Phase::Finished
                }
            });

            view! {
                <section class="border-b border-line bg-surface">
                    <div class="container-page flex flex-col items-start gap-4 py-5 md:flex-row md:items-center md:justify-between">
                        <div>
                            <p class="font-mono text-xs uppercase tracking-[0.4em] text-accent">"// t-minus"</p>
                            <p class="mt-1 font-mono text-sm text-ink">
                                "Count down to: "
                                <span class="text-warm uppercase">{title}</span>
                            </p>
                        </div>
                        {move || match phase.get() {
                            Phase::Counting => view! {
                                <div class="flex items-end gap-3 font-mono md:gap-4">
                                    <Unit value=Signal::derive(days) label="days"/>
                                    <Colon/>
                                    <Unit value=Signal::derive(hours) label="hrs"/>
                                    <Colon/>
                                    <Unit value=Signal::derive(minutes) label="min"/>
                                    <Colon/>
                                    <Unit value=Signal::derive(seconds) label="sec"/>
                                </div>
                            }.into_any(),
                            Phase::Happening => view! {
                                <a
                                    href="/events"
                                    class="font-mono text-sm uppercase tracking-[0.2em] text-accent hover:text-accent-soft"
                                >
                                    "It's happening now, see details!"
                                </a>
                            }.into_any(),
                            Phase::Finished => view! {
                                <p class="font-mono text-sm uppercase tracking-[0.2em] text-accent">
                                    "Just finished, hope you enjoyed it!"
                                </p>
                            }.into_any(),
                        }}
                    </div>
                </section>
            }
        })
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
