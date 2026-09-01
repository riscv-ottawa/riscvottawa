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

/// A signal holding the current time in milliseconds, ticking once a second on
/// the client and resyncing whenever the tab comes back. Call it once per view
/// that needs a clock and share the signal, so two countdowns on the same page
/// cost one interval and stay in step.
pub fn use_now() -> RwSignal<f64> {
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

    now
}

const HOUR_MS: f64 = 3_600_000.0;
// Once an event starts, keep showing "happening now" for this long, then
// "just finished" until the retire window elapses and we advance to the next.
const HAPPENING_WINDOW_MS: f64 = 4.0 * HOUR_MS;
/// How long an event lingers after its start before we stop showing it. Mirrored
/// server-side by `RETIRE_AFTER` in `content::mod`.
pub const RETIRE_WINDOW_MS: f64 = 12.0 * HOUR_MS;

#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    /// Before the event starts: show the ticking countdown.
    Counting,
    /// Within HAPPENING_WINDOW_MS after the start.
    Happening,
    /// Between HAPPENING_WINDOW_MS and RETIRE_WINDOW_MS after the start.
    Finished,
}

pub fn phase_of(now_ms: f64, target_ms: f64) -> Phase {
    let elapsed = now_ms - target_ms;
    if elapsed < 0.0 {
        Phase::Counting
    } else if elapsed < HAPPENING_WINDOW_MS {
        Phase::Happening
    } else {
        Phase::Finished
    }
}

/// Start of an event in milliseconds since the epoch, or `None` for an event
/// that only has a month settled and so has nothing to count down to.
pub fn start_ms(event: &Event) -> Option<f64> {
    event
        .date
        .instant()
        .map(|dt| dt.unix_timestamp() as f64 * 1000.0)
}

/// The four ticking units. `large` is for the spotlight band, where the clock is
/// the thing drawing the eye rather than a detail in a strip.
#[component]
pub fn CountdownClock(
    target_ms: f64,
    now: RwSignal<f64>,
    #[prop(optional)] large: bool,
) -> impl IntoView {
    let remaining = move || ((target_ms - now.get()) / 1000.0).max(0.0) as i64;
    let days = move || remaining() / 86_400;
    let hours = move || (remaining() % 86_400) / 3_600;
    let minutes = move || (remaining() % 3_600) / 60;
    let seconds = move || remaining() % 60;

    let digit = if large {
        "text-4xl font-bold tabular-nums text-ink sm:text-5xl md:text-6xl"
    } else {
        "text-2xl font-bold tabular-nums text-ink md:text-3xl"
    };
    let colon = if large {
        "pb-6 text-2xl font-bold text-line sm:text-3xl md:text-4xl"
    } else {
        "pb-4 text-xl font-bold text-line md:text-2xl"
    };
    let gap = if large {
        "gap-4 md:gap-6"
    } else {
        "gap-3 md:gap-4"
    };

    view! {
        <div class=format!("flex items-end font-mono {gap}")>
            <Unit value=Signal::derive(days) label="days" class=digit/>
            <span class=colon>":"</span>
            <Unit value=Signal::derive(hours) label="hrs" class=digit/>
            <span class=colon>":"</span>
            <Unit value=Signal::derive(minutes) label="min" class=digit/>
            <span class=colon>":"</span>
            <Unit value=Signal::derive(seconds) label="sec" class=digit/>
        </div>
    }
}

/// One event's clock through its whole life: ticking digits until it starts,
/// then a short status line while it runs and just after. Callers that may not
/// have an exact start use `start_ms` first and decide for themselves what goes
/// in the slot. `large` is for the home page band, where the clock is the thing
/// drawing the eye rather than a detail beside a button.
#[component]
pub fn EventClock(
    target_ms: f64,
    now: RwSignal<f64>,
    #[prop(optional)] large: bool,
) -> impl IntoView {
    let status = if large {
        "font-mono text-xl font-bold uppercase tracking-[0.2em] text-accent md:text-2xl"
    } else {
        "font-mono text-lg font-bold uppercase tracking-[0.2em] text-accent"
    };

    // Via a memo, so the view is rebuilt only when the phase actually changes.
    // Reading `now` straight from the view closure instead re-runs it on every
    // tick, tearing down and remounting the whole clock once a second.
    let phase = Memo::new(move |_| phase_of(now.get(), target_ms));

    move || match phase.get() {
        Phase::Counting => view! { <CountdownClock target_ms now large/> }.into_any(),
        Phase::Happening => view! { <p class=status>"Happening now"</p> }.into_any(),
        Phase::Finished => view! {
            <p class="font-mono text-sm uppercase tracking-[0.2em] text-mute">
                "That's a wrap, thanks for coming"
            </p>
        }
        .into_any(),
    }
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
                start_ms: start_ms(&e)?,
                title: e.title,
            })
        })
        .collect();
    let events = StoredValue::new(events);
    let now = use_now();

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
            let phase = Memo::new(move |_| phase_of(now.get(), target_ms));

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
                                <CountdownClock target_ms now/>
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
fn Unit(value: Signal<i64>, label: &'static str, class: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center">
            <span class=class>{move || format!("{:02}", value.get())}</span>
            <span class="mt-1 text-[0.6rem] uppercase tracking-[0.25em] text-mute">{label}</span>
        </div>
    }
}
