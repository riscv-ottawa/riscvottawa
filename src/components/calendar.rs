use crate::content::Event;
use leptos::prelude::*;
use std::collections::BTreeMap;
use time::{Date, Duration, Month};

#[derive(Clone, PartialEq, Eq)]
struct CalendarCell {
    date: Date,
    in_view_month: bool,
    is_today: bool,
    event_titles: Vec<String>,
}

#[component]
pub fn Calendar(events: Vec<Event>, today: Date) -> impl IntoView {
    let view_state = RwSignal::new((today.year(), today.month()));
    let selected = RwSignal::new(None::<Date>);

    let mut map: BTreeMap<Date, Vec<Event>> = BTreeMap::new();
    for e in events {
        map.entry(e.date.date()).or_default().push(e);
    }
    let events_by_date = StoredValue::new(map);

    let go_prev = move |_| {
        view_state.update(|(y, m)| {
            let (ny, nm) = prev_year_month(*y, *m);
            *y = ny;
            *m = nm;
        });
        selected.set(None);
    };
    let go_next = move |_| {
        view_state.update(|(y, m)| {
            let (ny, nm) = next_year_month(*y, *m);
            *y = ny;
            *m = nm;
        });
        selected.set(None);
    };

    let go_today = move |_| {
        view_state.set((today.year(), today.month()));
        selected.set(None);
    };

    view! {
        <section class="overflow-hidden rounded-sm border border-line bg-surface">
            <header class="flex items-center justify-between gap-3 border-b border-line px-4 py-2.5">
                <button
                    type="button"
                    on:click=go_prev
                    aria-label="Previous month"
                    class="rounded-sm border border-line px-2.5 py-1 font-mono text-xs uppercase tracking-[0.2em] text-mute transition hover:border-accent hover:text-accent-soft"
                >
                    "< prev"
                </button>
                <div class="flex flex-col items-center gap-0.5">
                    <h2 class="font-mono text-base font-semibold text-ink">
                        {move || {
                            let (y, m) = view_state.get();
                            format!("{} {y}", month_name(m))
                        }}
                    </h2>
                    {move || {
                        let (y, m) = view_state.get();
                        if y != today.year() || m != today.month() {
                            Some(view! {
                                <button
                                    type="button"
                                    on:click=go_today
                                    class="font-mono text-[10px] uppercase tracking-[0.2em] text-accent hover:text-accent-soft transition"
                                >
                                    "Back to current month"
                                </button>
                            })
                        } else {
                            None
                        }
                    }}
                </div>
                <button
                    type="button"
                    on:click=go_next
                    aria-label="Next month"
                    class="rounded-sm border border-line px-2.5 py-1 font-mono text-xs uppercase tracking-[0.2em] text-mute transition hover:border-accent hover:text-accent-soft"
                >
                    "next >"
                </button>
            </header>

            <div class="grid grid-cols-7 border-b border-line text-center font-mono text-xs uppercase tracking-[0.2em] text-mute">
                <div class="py-1.5">"Mon"</div>
                <div class="py-1.5">"Tue"</div>
                <div class="py-1.5">"Wed"</div>
                <div class="py-1.5">"Thu"</div>
                <div class="py-1.5">"Fri"</div>
                <div class="py-1.5">"Sat"</div>
                <div class="py-1.5">"Sun"</div>
            </div>

            <div class="grid grid-cols-7">
                {move || {
                    let (y, m) = view_state.get();
                    build_cells(y, m, today, &events_by_date)
                        .into_iter()
                        .map(|cell| view! { <CalendarDay cell=cell selected=selected/> })
                        .collect::<Vec<_>>()
                }}
            </div>

            {move || {
                selected.get().and_then(|d| {
                    let day_events: Vec<Event> = events_by_date
                        .with_value(|m| m.get(&d).cloned().unwrap_or_default());
                    if day_events.is_empty() {
                        None
                    } else {
                        Some(view! { <SelectedDayPanel date=d events=day_events/> }.into_any())
                    }
                })
            }}
        </section>
    }
}

#[component]
fn CalendarDay(cell: CalendarCell, selected: RwSignal<Option<Date>>) -> impl IntoView {
    let CalendarCell {
        date,
        in_view_month,
        is_today,
        event_titles,
    } = cell;
    let day = date.day();
    let total = event_titles.len();
    let has_events = total > 0;
    let visible_titles: Vec<String> = event_titles.into_iter().take(2).collect();
    let extra = total.saturating_sub(visible_titles.len());

    let on_click = move |_| {
        if !has_events {
            selected.set(None);
            return;
        }
        selected.update(|s| {
            *s = if *s == Some(date) { None } else { Some(date) };
        });
    };

    view! {
        <button
            type="button"
            on:click=on_click
            aria-pressed=move || selected.get() == Some(date)
            class=move || {
                let mut c = String::from(
                    "flex min-h-[3.5rem] flex-col items-stretch border-b border-r border-line p-1.5 text-left font-mono text-xs transition",
                );
                if in_view_month {
                    c.push_str(" text-ink");
                } else {
                    c.push_str(" text-mute/40");
                }
                if has_events {
                    c.push_str(" cursor-pointer hover:bg-accent/5");
                } else {
                    c.push_str(" cursor-default");
                }
                if selected.get() == Some(date) {
                    c.push_str(" bg-accent/10 outline outline-1 outline-accent/60");
                }
                c
            }
        >
            <span class=move || {
                if is_today {
                    "inline-flex h-6 w-6 items-center justify-center rounded-full bg-accent text-paper"
                } else {
                    ""
                }
            }>
                {day}
            </span>
            <div class="mt-1 flex flex-col gap-0.5 overflow-hidden">
                {visible_titles.into_iter().map(|t| view! {
                    <div class="truncate rounded-sm bg-warm/15 px-1.5 py-0.5 font-mono text-[11px] leading-tight text-warm-soft">
                        {t}
                    </div>
                }).collect::<Vec<_>>()}
                {if extra > 0 {
                    Some(view! {
                        <div class="px-1.5 font-mono text-[10px] text-mute">
                            "+ "{extra}" more"
                        </div>
                    })
                } else {
                    None
                }}
            </div>
        </button>
    }
}

#[component]
fn SelectedDayPanel(date: Date, events: Vec<Event>) -> impl IntoView {
    let header = format_long_date(&date);
    view! {
        <div class="border-t border-line bg-paper px-4 py-4">
            <p class="font-mono text-xs uppercase tracking-[0.3em] text-accent">{header}</p>
            <ul class="mt-3 divide-y divide-line">
                {events.into_iter().map(|e| {
                    let time_label = format!("{:02}:{:02}", e.date.hour(), e.date.minute());
                    view! {
                        <li class="flex flex-col gap-2 py-3 md:flex-row md:items-baseline md:justify-between">
                            <div>
                                <p class="font-mono text-sm text-ink">
                                    <span class="mr-2 text-warm">{time_label}</span>
                                    {e.title}
                                </p>
                                <p class="text-xs text-mute">{e.location}</p>
                            </div>
                            <a
                                href=e.luma_url
                                target="_blank"
                                rel="noopener"
                                class="shrink-0 font-mono text-xs uppercase tracking-[0.2em] text-accent hover:text-accent-soft"
                            >
                                "RSVP on Luma"
                            </a>
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

fn build_cells(
    year: i32,
    month: Month,
    today: Date,
    events_by_date: &StoredValue<BTreeMap<Date, Vec<Event>>>,
) -> Vec<CalendarCell> {
    let first = Date::from_calendar_date(year, month, 1).expect("valid first-of-month");
    let offset = first.weekday().number_from_monday() as i64 - 1;
    let start = first - Duration::days(offset);
    (0..42i64)
        .map(|i| {
            let date = start + Duration::days(i);
            let event_titles = events_by_date.with_value(|m| {
                m.get(&date)
                    .map(|evs| evs.iter().map(|e| e.title.clone()).collect::<Vec<_>>())
                    .unwrap_or_default()
            });
            CalendarCell {
                date,
                in_view_month: date.month() == month && date.year() == year,
                is_today: date == today,
                event_titles,
            }
        })
        .collect()
}

fn prev_year_month(year: i32, month: Month) -> (i32, Month) {
    match month {
        Month::January => (year - 1, Month::December),
        other => (year, other.previous()),
    }
}

fn next_year_month(year: i32, month: Month) -> (i32, Month) {
    match month {
        Month::December => (year + 1, Month::January),
        other => (year, other.next()),
    }
}

fn month_name(m: Month) -> &'static str {
    match m {
        Month::January => "January",
        Month::February => "February",
        Month::March => "March",
        Month::April => "April",
        Month::May => "May",
        Month::June => "June",
        Month::July => "July",
        Month::August => "August",
        Month::September => "September",
        Month::October => "October",
        Month::November => "November",
        Month::December => "December",
    }
}

fn format_long_date(d: &Date) -> String {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    const WEEKDAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let mi = (u8::from(d.month()) - 1) as usize;
    let wi = (d.weekday().number_from_monday() - 1) as usize;
    format!(
        "{} {} {:02} {}",
        WEEKDAYS[wi],
        MONTHS[mi],
        d.day(),
        d.year()
    )
}
