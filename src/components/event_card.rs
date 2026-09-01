use crate::components::modal::Modal;
use crate::components::prose::Prose;
use crate::content::{Event, EventDate, LUMA_PENDING};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn EventCard(event: Event) -> impl IntoView {
    let when = format_event_date(&event.date);
    let tags = event.tags.clone();

    let has_luma = event.has_luma();
    let spotlight_href = event.spotlight.as_ref().map(|_| event.href());

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
            <Prose text=event.summary class="mt-4 text-sm text-ink/90"/>
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
                    // A spotlight event has a page worth landing on, so the
                    // card links there instead of opening the modal.
                    {match spotlight_href.clone() {
                        Some(href) => view! {
                            <A
                                href=href
                                attr:class="text-accent hover:text-accent-soft underline"
                            >
                                "Full details \u{2192}"
                            </A>
                        }.into_any(),
                        None => view! {
                            <button
                                type="button"
                                on:click=move |_| open.set(true)
                                class="text-accent hover:text-accent-soft underline"
                            >
                                "Details"
                            </button>
                        }.into_any(),
                    }}
                    {if has_luma {
                        view! {
                            <a
                                href=event.luma_url
                                target="_blank"
                                rel="noopener"
                                class="text-accent hover:text-accent-soft"
                            >
                                "RSVP on Luma"
                            </a>
                        }.into_any()
                    } else {
                        view! { <span class="text-mute">{LUMA_PENDING}</span> }.into_any()
                    }}
                </div>
            </div>
            <Modal open>
                <p class="font-mono text-xs uppercase tracking-[0.3em] text-warm">
                    {m_when.clone()}
                </p>
                <h2 class="mt-3 font-mono text-2xl font-bold text-ink md:text-3xl">
                    {m_title.clone()}
                </h2>
                <p class="mt-1 text-sm text-mute">{m_location.clone()}</p>
                <Prose text=m_description.clone() class="mt-6 text-ink/90"/>
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
                    {if has_luma {
                        view! {
                            <a
                                href=m_luma.clone()
                                target="_blank"
                                rel="noopener"
                                class="text-accent hover:text-accent-soft"
                            >
                                "RSVP on Luma"
                            </a>
                        }.into_any()
                    } else {
                        view! { <span class="text-mute">{LUMA_PENDING}</span> }.into_any()
                    }}
                </div>
            </Modal>
        </li>
    }
}

const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];
const WEEKDAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

pub fn format_event_date(date: &EventDate) -> String {
    match date {
        EventDate::At(dt) => {
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
        EventDate::Month { year, month } => {
            let month_idx = (u8::from(*month) - 1) as usize;
            format!("{} {year} \u{2022} Day TBD", MONTHS[month_idx])
        }
    }
}

/// Month and year alone, for places that show the day separately or don't have
/// one settled yet.
pub fn format_event_month(date: &EventDate) -> String {
    let (year, month) = match date {
        EventDate::At(dt) => (dt.year(), u8::from(dt.month())),
        EventDate::Month { year, month } => (*year, u8::from(*month)),
    };
    format!("{} {year}", MONTHS[(month - 1) as usize])
}
