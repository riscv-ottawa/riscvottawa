use crate::content::Event;
use leptos::prelude::*;

#[component]
pub fn EventCard(event: Event) -> impl IntoView {
    let when = format_event_date(&event.date);
    let tags = event.tags.clone();

    let open = RwSignal::new(false);

    let m_when = when.clone();
    let m_title = event.title.clone();
    let m_location = event.location.clone();
    let m_description = event.description.clone();
    let m_tags = event.tags.clone();
    let m_luma = event.luma_url.clone();

    view! {
        <li class="flex h-full flex-col rounded-sm border border-line bg-surface p-6 transition hover:border-accent/60">
            <p class="font-mono text-xs uppercase tracking-[0.3em] text-warm">{when}</p>
            <h3 class="mt-3 font-mono text-xl font-semibold text-ink">{event.title}</h3>
            <p class="mt-1 text-sm text-mute">{event.location}</p>
            <p class="mt-4 text-sm text-ink/90">{event.summary}</p>
            <div class="mt-auto flex flex-col gap-4 pt-5">
                <div class="flex flex-wrap gap-2">
                    {tags
                        .into_iter()
                        .map(|t| view! {
                            <span class="rounded-sm border border-line px-2 py-0.5 font-mono text-xs text-mute">
                                {t}
                            </span>
                        })
                        .collect::<Vec<_>>()}
                </div>
                <div class="flex flex-wrap items-center gap-x-5 gap-y-2 font-mono text-xs uppercase tracking-[0.2em]">
                    <button
                        type="button"
                        on:click=move |_| open.set(true)
                        class="text-accent hover:text-accent-soft"
                    >
                        "Details"
                    </button>
                    <a
                        href=event.luma_url
                        target="_blank"
                        rel="noopener"
                        class="text-accent hover:text-accent-soft"
                    >
                        "RSVP on Luma"
                    </a>
                </div>
            </div>
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
                        <p class="font-mono text-xs uppercase tracking-[0.3em] text-warm">
                            {m_when.clone()}
                        </p>
                        <h2 class="mt-3 font-mono text-2xl font-bold text-ink md:text-3xl">
                            {m_title.clone()}
                        </h2>
                        <p class="mt-1 text-sm text-mute">{m_location.clone()}</p>
                        <p class="mt-6 text-ink/90">{m_description.clone()}</p>
                        {(!m_tags.is_empty()).then(|| {
                            let tags = m_tags.clone();
                            view! {
                                <div class="mt-6 flex flex-wrap gap-2">
                                    {tags
                                        .into_iter()
                                        .map(|t| view! {
                                            <span class="rounded-sm border border-line px-2 py-0.5 font-mono text-xs text-mute">
                                                {t}
                                            </span>
                                        })
                                        .collect::<Vec<_>>()}
                                </div>
                            }
                        })}
                        <div class="mt-8 flex flex-wrap gap-x-5 gap-y-2 font-mono text-xs uppercase tracking-[0.2em]">
                            <a
                                href=m_luma.clone()
                                target="_blank"
                                rel="noopener"
                                class="text-accent hover:text-accent-soft"
                            >
                                "RSVP on Luma"
                            </a>
                        </div>
                    </div>
                </div>
            })}
        </li>
    }
}

pub fn format_event_date(dt: &time::OffsetDateTime) -> String {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    const WEEKDAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let month_idx = (u8::from(dt.month()) - 1) as usize;
    let weekday_idx = (dt.weekday().number_from_monday() - 1) as usize;
    format!(
        "{} {} {:02} {} \u{2022} {:02}:{:02}",
        WEEKDAYS[weekday_idx],
        MONTHS[month_idx],
        dt.day(),
        dt.year(),
        dt.hour(),
        dt.minute(),
    )
}
