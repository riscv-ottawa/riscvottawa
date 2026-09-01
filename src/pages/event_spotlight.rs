use crate::components::countdown::{start_ms, use_now, EventClock};
use crate::components::event_card::format_event_date;
use crate::components::prose::{single_line, Prose};
use crate::components::rsvp::RsvpButton;
use crate::content::{
    get_spotlight_event, schedule_clock, Event, Extra, Panel, Slot, Speaker, Spotlight, Status,
    Teaser,
};
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use time::OffsetDateTime;

const SITE_ORIGIN: &str = "https://riscvottawa.ca";
const DISCORD_URL: &str = "https://discord.gg/EfryE4wfk4";

/// A page for the occasional meetup we turn into something bigger. Ordinary
/// events have nothing extra to say and keep the card-and-modal path, so a slug
/// without a `[spotlight]` block lands on the not-found below.
#[component]
pub fn EventSpotlight() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();
    let data = Resource::new_blocking(slug, |slug| async move { get_spotlight_event(slug).await });

    view! {
        <Suspense fallback=|| view! { <PageSkeleton/> }>
            {move || match data.get() {
                Some(Ok(Some(event))) => view! { <Body event/> }.into_any(),
                Some(Ok(None)) => view! { <NoSuchEvent/> }.into_any(),
                Some(Err(e)) => view! {
                    <section class="container-page py-24">
                        <p class="text-warm">{format!("Could not load this event: {e}")}</p>
                    </section>
                }.into_any(),
                None => view! { <PageSkeleton/> }.into_any(),
            }}
        </Suspense>
    }
}

#[component]
fn Body(event: Event) -> impl IntoView {
    let s: Spotlight = event.spotlight.clone().expect("spotlight events only");
    let when = format_event_date(&event.date);
    let now = use_now();

    let page_title = format!("{} - RISC-V Ottawa", s.headline);
    let share_url = format!("{SITE_ORIGIN}{}", event.href());
    // Flattened: an unfurl is a single run of text, and a raw newline in a meta
    // tag is at best ignored and at worst truncates the description.
    let share_blurb = format!(
        "{when} \u{00b7} {}. {}",
        event.location,
        single_line(&s.tagline)
    );

    view! {
        <Title text=page_title/>
        // This page exists to be pasted into LinkedIn and Discord, where a bare
        // link currently unfurls to nothing.
        <Meta property="og:title" content=s.headline.clone()/>
        <Meta property="og:description" content=share_blurb/>
        <Meta property="og:url" content=share_url/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:site_name" content="RISC-V Ottawa"/>
        // No `name="description"` here: the shell already emits the site-wide
        // one and crawlers take the first, so a second would just be noise.
        {s.og_image.clone().map(|src| view! {
            <Meta property="og:image" content=format!("{SITE_ORIGIN}{src}")/>
            <Meta name="twitter:card" content="summary_large_image"/>
        })}

        <section class="relative overflow-hidden border-b border-line">
            <div class="pointer-events-none absolute inset-0 hero-grid"></div>
            <div class="container-page relative py-16 md:py-24">
                <p class="font-mono text-xs uppercase tracking-[0.4em] text-accent">
                    "// " {s.kicker.clone()}
                </p>
                <h1 class="mt-5 max-w-4xl font-mono text-3xl font-bold leading-[1.1] text-ink md:text-5xl">
                    {s.headline.clone()}
                </h1>
                <Prose text=s.tagline.clone() class="mt-6 max-w-2xl text-lg text-mute"/>

                <dl class="mt-10 flex flex-wrap gap-x-12 gap-y-5">
                    <Fact label="when" value=when.clone()/>
                    <Fact label="where" value=event.location.clone()/>
                    <Fact label="cost" value="Free \u{00b7} all welcome".to_string()/>
                </dl>

                <div class="mt-10 flex flex-col gap-8 sm:flex-row sm:items-end sm:justify-between">
                    <RsvpButton luma_url=event.luma_url.clone()/>
                    {start_ms(&event).map(|target_ms| view! { <EventClock target_ms now/> })}
                </div>
            </div>
        </section>

        {(!s.draws.is_empty()).then(|| view! { <Draws items=s.draws.clone()/> })}
        {(!s.speakers.is_empty()).then(|| view! { <Speakers items=s.speakers.clone()/> })}
        {s.panel.clone().map(|panel| view! { <PanelBlock panel/> })}
        {(!s.extras.is_empty()).then(|| view! { <Extras items=s.extras.clone()/> })}
        {(!s.schedule.is_empty()).then(|| view! {
            <RunOfShow items=s.schedule.clone() start=event.date.instant()/>
        })}
        {s.teaser.clone().map(|teaser| view! { <TeaserBlock teaser/> })}
        {(!s.call_to_build.is_empty()).then(|| view! { <GetInvolved items=s.call_to_build.clone()/> })}

        <Closing event=event.clone() when=when/>
    }
}

#[component]
fn Fact(label: &'static str, value: String) -> impl IntoView {
    view! {
        <div>
            <dt class="font-mono text-[0.6rem] uppercase tracking-[0.3em] text-accent">{label}</dt>
            <dd class="mt-2 max-w-xs font-mono text-sm text-ink">{value}</dd>
        </div>
    }
}

#[component]
fn SectionHead(eyebrow: &'static str, title: &'static str) -> impl IntoView {
    view! {
        <p class="font-mono text-xs uppercase tracking-[0.4em] text-accent">{eyebrow}</p>
        <h2 class="mt-3 font-mono text-2xl font-bold text-ink md:text-3xl">{title}</h2>
    }
}

#[component]
fn Draws(items: Vec<String>) -> impl IntoView {
    view! {
        <section class="container-page py-16">
            <SectionHead eyebrow="/ why come" title="What you get out of the evening"/>
            <div class="mt-8 grid gap-6 md:grid-cols-3">
                {items
                    .into_iter()
                    .enumerate()
                    .map(|(i, draw)| view! {
                        <div class="rounded-sm border border-line bg-surface p-6">
                            <p class="font-mono text-xs text-accent">{format!("{:02}", i + 1)}</p>
                            <Prose text=draw class="mt-3 text-ink/90"/>
                        </div>
                    })
                    .collect::<Vec<_>>()}
            </div>
        </section>
    }
}

#[component]
fn Speakers(items: Vec<Speaker>) -> impl IntoView {
    view! {
        <section id="speakers" class="container-page hairline py-16">
            <SectionHead eyebrow="/ speakers" title="Who you'll hear from"/>
            <ul class="mt-8 grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                {items
                    .into_iter()
                    .map(|speaker| view! { <li><SpeakerCard speaker/></li> })
                    .collect::<Vec<_>>()}
            </ul>
        </section>
    }
}

/// One person, or one slot we haven't filled. Unfilled slots are shown rather
/// than hidden: a lineup that is visibly still filling gives people a reason to
/// come back, and hiding them makes the evening look smaller than it is.
#[component]
fn SpeakerCard(speaker: Speaker) -> impl IntoView {
    let tba = speaker.status == Status::Tba;
    let frame = if tba {
        "h-full rounded-sm border border-dashed border-line bg-surface/50 p-6"
    } else {
        "h-full rounded-sm border border-line bg-surface p-6 transition hover:border-accent/60"
    };
    let name_class = if tba {
        "mt-1 font-mono text-lg font-semibold text-mute"
    } else {
        "mt-1 font-mono text-lg font-semibold text-ink"
    };
    let name = speaker.display_name().to_string();

    view! {
        <div class=frame>
            <div class="flex items-center gap-4">
                <Avatar speaker=speaker.clone()/>
                <div class="min-w-0">
                    {(!speaker.role.is_empty()).then(|| view! {
                        <p class="font-mono text-[0.6rem] uppercase tracking-[0.3em] text-warm">
                            {speaker.role.clone()}
                        </p>
                    })}
                    <p class=name_class>
                        {match speaker.link.clone() {
                            Some(href) if !tba => view! {
                                <a href=href target="_blank" rel="noopener" class="hover:text-accent-soft">
                                    {name.clone()}
                                </a>
                            }.into_any(),
                            _ => view! { {name.clone()} }.into_any(),
                        }}
                    </p>
                    {(!speaker.affiliation.is_empty()).then(|| view! {
                        <p class="mt-1 text-sm text-mute">
                            <Affiliation speaker=speaker.clone()/>
                        </p>
                    })}
                </div>
            </div>
            {(!speaker.topic.is_empty()).then(|| view! {
                <Prose text=speaker.topic.clone() class="mt-4 text-sm text-ink/90"/>
            })}
            {(speaker.status == Status::Pending).then(|| view! {
                <p class="mt-4 inline-block rounded-sm border border-line px-2 py-0.5 font-mono text-[0.6rem] uppercase tracking-[0.2em] text-mute">
                    "invited"
                </p>
            })}
        </div>
    }
}

/// Where someone is from, linked to that organisation when the content file
/// gives a URL. Kept subtle: it sits under the name and must not compete with
/// it, so it stays muted and picks up the accent only on hover.
#[component]
fn Affiliation(speaker: Speaker) -> impl IntoView {
    let text = speaker.affiliation.clone();
    match speaker.affiliation_url.as_deref().map(str::trim) {
        Some(href) if !href.is_empty() => view! {
            <a
                href=href.to_string()
                target="_blank"
                rel="noopener"
                class="underline transition-colors hover:text-accent-soft"
            >
                {text}
            </a>
        }
        .into_any(),
        _ => view! { {text} }.into_any(),
    }
}

/// A speaker's headshot, their initials while we're waiting on one, or an empty
/// dashed circle for a slot nobody is filling yet. Always the same square, so
/// the grid holds its shape as pictures arrive one at a time.
#[component]
fn Avatar(speaker: Speaker, #[prop(optional)] small: bool) -> impl IntoView {
    let (box_size, px, text) = if small {
        ("h-10 w-10", "40", "text-xs")
    } else {
        ("h-14 w-14", "56", "text-sm")
    };

    match (speaker.photo.clone(), speaker.status) {
        (Some(src), _) => view! {
            <img
                src=src
                alt=speaker.display_name().to_string()
                width=px
                height=px
                loading="lazy"
                decoding="async"
                class=format!("{box_size} shrink-0 rounded-full border border-line object-cover")
            />
        }
        .into_any(),
        (None, Status::Tba) => view! {
            <div
                aria-hidden="true"
                class=format!("{box_size} shrink-0 rounded-full border border-dashed border-line")
            ></div>
        }
        .into_any(),
        (None, _) => view! {
            <div
                aria-hidden="true"
                class=format!(
                    "{box_size} shrink-0 rounded-full border border-line bg-paper \
                     flex items-center justify-center font-mono {text} text-mute",
                )
            >
                {speaker.initials()}
            </div>
        }
        .into_any(),
    }
}

/// The panel gets its own framed block. It is the piece the evening is built
/// around, and burying it in the speaker grid would read as one talk among many.
#[component]
fn PanelBlock(panel: Panel) -> impl IntoView {
    // The label was a lowercase literal, which threw away whatever the content
    // file wrote. Print the role as given, and only supply a default when the
    // file leaves it out.
    let role = match panel.moderator.role.trim() {
        "" => "Moderator",
        given => given,
    }
    .to_string();

    view! {
        <section id="panel" class="container-page py-16">
            <div class="rounded-sm border border-accent/60 bg-surface p-8 shadow-soft md:p-12">
                <p class="font-mono text-xs uppercase tracking-[0.4em] text-warm">"// the panel"</p>
                <h2 class="mt-4 max-w-3xl font-mono text-2xl font-bold leading-tight text-ink md:text-4xl">
                    {panel.title}
                </h2>
                {(!panel.subtitle.is_empty()).then(|| view! {
                    <Prose text=panel.subtitle.clone() class="mt-4 max-w-2xl text-mute"/>
                })}
                <div class="mt-6 flex items-center gap-3">
                    <Avatar speaker=panel.moderator.clone() small=true/>
                    <p class="font-mono text-sm text-ink">
                        <span class="text-accent">{format!("{role}: ")}</span>
                        {panel.moderator.display_name().to_string()}
                        {(!panel.moderator.affiliation.is_empty()).then(|| view! {
                            " \u{00b7} "
                            <Affiliation speaker=panel.moderator.clone()/>
                        })}
                    </p>
                </div>
                <ul class="mt-8 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                    {panel.panelists
                        .into_iter()
                        .map(|speaker| view! { <li><SpeakerCard speaker/></li> })
                        .collect::<Vec<_>>()}
                </ul>
            </div>
        </section>
    }
}

/// Pizza, giveaways, and the slots we still need someone to cover. It sits
/// under the panel because it is an aside to the lineup, not part of it, and
/// people who have already decided to come are the ones who read this far.
#[component]
fn Extras(items: Vec<Extra>) -> impl IntoView {
    view! {
        <section class="container-page py-16">
            <SectionHead eyebrow="/ on the night" title="Pizza, and something to take home"/>
            <ul class="mt-8 grid gap-6 md:grid-cols-2">
                {items
                    .into_iter()
                    .map(|extra| view! { <li><ExtraCard extra/></li> })
                    .collect::<Vec<_>>()}
            </ul>
        </section>
    }
}

/// One aside. An unfilled one takes the same dashed frame as an unfilled
/// speaker slot: the page already uses that to mean "we want this and do not
/// have it yet", and saying so plainly is what gets it filled.
#[component]
fn ExtraCard(extra: Extra) -> impl IntoView {
    let wanted = extra.status == Status::Tba;
    let frame = if wanted {
        "h-full rounded-sm border border-dashed border-warm/60 bg-surface/50 p-6"
    } else {
        "h-full rounded-sm border border-line bg-surface p-6"
    };
    let cta_url = extra.cta_url.trim().to_string();
    let cta_label = extra.cta_label.trim().to_string();
    // A `mailto:` hands off to the mail client, so sending it to a new tab
    // leaves an empty one behind. Only real pages get `_blank`.
    let new_tab = cta_url.starts_with("http");

    view! {
        <div class=frame>
            <div class="flex flex-col gap-5 sm:flex-row sm:items-start">
                {extra.image.clone().map(|src| view! {
                    <img
                        src=src
                        alt=extra.title.clone()
                        width="96"
                        loading="lazy"
                        decoding="async"
                        class="w-24 shrink-0 self-start rounded-sm object-contain"
                    />
                })}
                <div class="min-w-0">
                    <p class="font-mono text-[0.6rem] uppercase tracking-[0.3em] text-warm">
                        {extra.label}
                    </p>
                    <p class="mt-2 font-mono text-lg font-semibold text-ink">{extra.title}</p>
                    <Prose text=extra.body class="mt-3 text-sm text-ink/90"/>
                    {(!cta_url.is_empty() && !cta_label.is_empty()).then(|| view! {
                        <p class="mt-5">
                            <a
                                href=cta_url
                                target=new_tab.then_some("_blank")
                                rel=new_tab.then_some("noopener")
                                class="font-mono text-xs uppercase tracking-[0.2em] text-accent transition hover:text-accent-soft"
                            >
                                {cta_label}
                            </a>
                        </p>
                    })}
                </div>
            </div>
        </div>
    }
}

#[component]
fn RunOfShow(items: Vec<Slot>, start: Option<OffsetDateTime>) -> impl IntoView {
    // An event we've committed to but not yet scheduled has no instant to hang
    // the first block off, so it falls back to showing lengths.
    let labels = match start {
        Some(start) => schedule_clock(start, &items),
        None => items.iter().map(Slot::length_label).collect(),
    };
    let note = if start.is_some() {
        "Our plan for the night. Blocks can shift by a few minutes on the day."
    } else {
        "Block lengths are settled; exact start times land with the venue."
    };

    view! {
        <section id="schedule" class="container-page hairline py-16">
            <SectionHead eyebrow="/ agenda" title="How the evening goes"/>
            <p class="mt-4 max-w-2xl text-sm text-mute">{note}</p>
            <ol class="mt-8 border-l border-line pl-6">
                {items
                    .into_iter()
                    .zip(labels)
                    .map(|(slot, label)| {
                        // Muted dots on the rule, accent for the moments the
                        // evening is built around.
                        let dot = if slot.highlight {
                            "absolute -left-[1.8125rem] top-2 h-2.5 w-2.5 rounded-full bg-accent"
                        } else {
                            "absolute -left-[1.8125rem] top-2 h-2.5 w-2.5 rounded-full bg-mute/50"
                        };
                        let title_class = if slot.highlight {
                            "font-mono text-lg font-semibold text-accent"
                        } else {
                            "font-mono text-lg font-semibold text-ink"
                        };
                        view! {
                            <li class="relative pb-8 last:pb-0">
                                <span class=dot></span>
                                <p class="font-mono text-[0.6rem] uppercase tracking-[0.3em] text-mute">
                                    {label}
                                </p>
                                <p class=title_class>{slot.title}</p>
                                {(!slot.presenter.is_empty()).then(|| view! {
                                    <p class="mt-1 font-mono text-sm text-warm">{slot.presenter.clone()}</p>
                                })}
                                {(!slot.detail.is_empty()).then(|| view! {
                                    <Prose text=slot.detail.clone() class="mt-2 max-w-2xl text-sm text-ink/90"/>
                                })}
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}
            </ol>
        </section>
    }
}

/// Deliberately withholding. The block carries copy and no link, because the
/// point of announcing it in the room is that you have to be in the room.
#[component]
fn TeaserBlock(teaser: Teaser) -> impl IntoView {
    view! {
        <section class="container-page py-16">
            <div class="relative overflow-hidden rounded-sm border border-warm/50 bg-surface p-8 md:p-12">
                <div class="pointer-events-none absolute inset-0 hero-grid opacity-60"></div>
                <div class="relative">
                    <p class="font-mono text-xs uppercase tracking-[0.4em] text-warm">
                        {teaser.label}
                    </p>
                    <h2 class="mt-4 max-w-3xl font-mono text-2xl font-bold leading-tight text-ink md:text-4xl">
                        {teaser.headline}
                    </h2>
                    <Prose text=teaser.body class="mt-6 max-w-2xl text-mute"/>
                </div>
            </div>
        </section>
    }
}

#[component]
fn GetInvolved(items: Vec<String>) -> impl IntoView {
    view! {
        <section class="container-page hairline py-16">
            <SectionHead eyebrow="/ get involved" title="Who we're looking for"/>
            <ul class="mt-8 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {items
                    .into_iter()
                    .map(|item| view! {
                        <li class="rounded-sm border border-line bg-surface p-5">
                            <Prose text=item class="mt-3 text-sm text-ink/90 first:mt-0"/>
                        </li>
                    })
                    .collect::<Vec<_>>()}
            </ul>
            <div class="mt-8 flex flex-wrap gap-x-10 gap-y-3 font-mono text-xs uppercase tracking-[0.2em]">
                <a
                    href=DISCORD_URL
                    target="_blank"
                    rel="noopener"
                    class="text-accent transition hover:text-accent-soft"
                >
                    "Join the Discord \u{2192}"
                </a>
                <A href="/projects" attr:class="text-accent transition hover:text-accent-soft">
                    "Browse member projects \u{2192}"
                </A>
            </div>
        </section>
    }
}

#[component]
fn Closing(event: Event, when: String) -> impl IntoView {
    view! {
        <section class="container-page py-16">
            <div class="rounded-sm border border-line bg-surface p-8 md:p-12">
                <div class="flex flex-col gap-8 md:flex-row md:items-end md:justify-between">
                    <div>
                        <h2 class="font-mono text-2xl font-bold text-ink md:text-3xl">
                            "Come along"
                        </h2>
                        <p class="mt-4 max-w-xl text-mute">
                            "No RISC-V experience assumed. Students, working engineers, researchers, and the merely curious are all welcome, and it costs nothing."
                        </p>
                        <dl class="mt-8 flex flex-wrap gap-x-12 gap-y-5">
                            <Fact label="when" value=when/>
                            <Fact label="where" value=event.location.clone()/>
                        </dl>
                        <p class="mt-8 font-mono text-xs uppercase tracking-[0.2em]">
                            <A
                                href="/code-of-conduct"
                                attr:class="text-accent transition hover:text-accent-soft"
                            >
                                "Read the code of conduct \u{2192}"
                            </A>
                        </p>
                    </div>
                    <div class="shrink-0 md:text-right">
                        <RsvpButton luma_url=event.luma_url.clone()/>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn NoSuchEvent() -> impl IntoView {
    view! {
        <section class="container-page py-24 text-center">
            <p class="font-mono text-sm uppercase tracking-[0.3em] text-accent">"404"</p>
            <h1 class="mt-4 font-mono text-3xl font-bold text-ink md:text-4xl">
                "No such event"
            </h1>
            <p class="mt-4 text-mute">
                "This event either doesn't exist or doesn't have a page of its own."
            </p>
            <A href="/events" attr:class="mt-8 inline-block text-accent hover:text-accent-soft">
                "See all events"
            </A>
        </section>
    }
}

#[component]
fn PageSkeleton() -> impl IntoView {
    view! {
        <div class="container-page py-16">
            <div class="h-64 animate-pulse rounded-sm border border-line bg-surface"></div>
        </div>
    }
}
