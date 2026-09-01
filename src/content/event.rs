use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use time::{Date, Duration, Month, OffsetDateTime};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    /// The URL this event lives at, `/events/<slug>`. Defaults to the file name
    /// without its extension, but set it explicitly for anything with a public
    /// link: file names carry dates, dates move, and a URL that moves with them
    /// breaks every link anyone has already shared.
    #[serde(default)]
    pub slug: String,
    pub title: String,
    pub date: EventDate,
    pub location: String,
    pub summary: String,
    #[serde(default)]
    pub description: String,
    pub luma_url: String,
    #[serde(default)]
    pub tags: Vec<String>,
    /// Present only on the occasional meetup we turn into a bigger night. Its
    /// absence is what keeps every other event on the card-and-modal path.
    #[serde(default)]
    pub spotlight: Option<Spotlight>,
}

impl Event {
    /// An empty `luma_url` means the Luma page hasn't been published yet. We
    /// release them about two weeks before each event.
    pub fn has_luma(&self) -> bool {
        !self.luma_url.trim().is_empty()
    }

    /// The dedicated page for this event, which only exists for spotlights.
    pub fn href(&self) -> String {
        format!("/events/{}", self.slug)
    }
}

/// Where to send people before an event's own Luma page exists. Subscribing to
/// the calendar is the only action that still pays off later: they get the
/// notification the moment RSVP opens, instead of being told to come back.
pub const LUMA_CALENDAR_URL: &str = "https://luma.com/riscv-ottawa";

/// What to say in place of an RSVP link while the Luma page is still pending.
pub const LUMA_PENDING: &str = "RSVP opens ~2 weeks before";

// A key that lands in the wrong table (easy with nested TOML) should be a
// startup error, not a section that quietly renders empty.
/// The extra material a spotlight event carries: marquee copy, who is speaking,
/// and the shape of the evening. Everything below the headline is optional, so
/// the page can go up as soon as there is something worth announcing and fill
/// in as the lineup locks.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spotlight {
    pub kicker: String,
    pub headline: String,
    pub tagline: String,
    /// Three or so reasons to give up an evening, in the visitor's terms.
    #[serde(default)]
    pub draws: Vec<String>,
    #[serde(default)]
    pub speakers: Vec<Speaker>,
    #[serde(default)]
    pub panel: Option<Panel>,
    /// The practical asides: what we're feeding people, what we're giving away,
    /// what we still need someone to cover.
    #[serde(default)]
    pub extras: Vec<Extra>,
    #[serde(default)]
    pub schedule: Vec<Slot>,
    #[serde(default)]
    pub teaser: Option<Teaser>,
    /// Skills and contributors we're looking for, one line each.
    #[serde(default)]
    pub call_to_build: Vec<String>,
    /// Absolute path to a share card under `public/`, e.g. `/og-september.png`.
    /// Omitted until someone makes one: an `og:image` pointing at a missing file
    /// unfurls worse than no tag at all.
    #[serde(default)]
    pub og_image: Option<String>,
}

impl Spotlight {
    /// Names for the home page band. Confirmed only: a one-line roster has no
    /// room to carry the "invited, not confirmed" caveat honestly.
    pub fn marquee_names(&self) -> Vec<String> {
        self.speakers
            .iter()
            .chain(self.panel.iter().flat_map(|p| p.panelists.iter()))
            .filter(|s| s.status == Status::Confirmed && !s.name.trim().is_empty())
            .map(|s| s.name.clone())
            .collect()
    }
}

/// How settled someone's participation is. `Pending` has expressed interest but
/// not confirmed; `Tba` is a slot we know we want to fill and have not. Both are
/// shown, because a lineup that visibly fills up gives people a reason to come
/// back, but only `Confirmed` names go on the home page.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    #[default]
    Confirmed,
    Pending,
    Tba,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Speaker {
    /// Empty for a slot that is still `Tba`.
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub affiliation: String,
    /// Turns the affiliation into a link to that organisation. Separate from
    /// `link`, which is the person's own page: an employer's site is not a
    /// stand-in for it, and conflating them sends people to the wrong place.
    #[serde(default)]
    pub affiliation_url: Option<String>,
    /// What they are here as, e.g. "Invited talk" or "Moderator".
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub topic: String,
    #[serde(default)]
    pub status: Status,
    #[serde(default)]
    pub link: Option<String>,
    /// Site-root path to a headshot under `public/`, e.g.
    /// `/speakers/megan-lehn.jpg`. Square images; anything else is cropped to
    /// one. Left out until we actually have the picture, and the card falls
    /// back to initials so a half-filled lineup still looks deliberate.
    #[serde(default)]
    pub photo: Option<String>,
}

impl Speaker {
    /// Up to two letters standing in for a missing headshot.
    pub fn initials(&self) -> String {
        self.name
            .split_whitespace()
            .filter_map(|word| word.chars().next())
            .take(2)
            .collect::<String>()
            .to_uppercase()
    }

    /// What to print. An unfilled slot has no name yet, and saying so is part of
    /// the point.
    pub fn display_name(&self) -> &str {
        if self.name.trim().is_empty() {
            "To be announced"
        } else {
            &self.name
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Panel {
    pub title: String,
    #[serde(default)]
    pub subtitle: String,
    pub moderator: Speaker,
    #[serde(default)]
    pub panelists: Vec<Speaker>,
}

/// One block in the agenda. Blocks carry a length rather than a clock
/// time; the page derives the times by accumulating from the event's own start,
/// so moving the doors time moves the whole evening with it.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Slot {
    pub minutes: u32,
    pub title: String,
    #[serde(default)]
    pub presenter: String,
    #[serde(default)]
    pub detail: String,
    /// Give this block the accent treatment. For the one or two moments the
    /// evening is actually built around.
    #[serde(default)]
    pub highlight: bool,
}

impl Slot {
    /// What to print when there is no start instant to hang clock times off.
    pub fn length_label(&self) -> String {
        format!("{} min", self.minutes)
    }
}

/// Clock ranges for a agenda, one label per slot and in order, accumulated
/// from the event's start. Blocks run back to back, so each one begins where the
/// previous ended.
pub fn schedule_clock(start: OffsetDateTime, slots: &[Slot]) -> Vec<String> {
    let mut cursor = start;
    slots
        .iter()
        .map(|slot| {
            let end = cursor + Duration::minutes(i64::from(slot.minutes));
            let label = format!(
                "{:02}:{:02} \u{2013} {:02}:{:02}",
                cursor.hour(),
                cursor.minute(),
                end.hour(),
                end.minute()
            );
            cursor = end;
            label
        })
        .collect()
}

/// One aside about the night itself: pizza, a giveaway, a slot we still need a
/// sponsor for. `Status::Tba` marks the ones that are asks rather than
/// promises, and the card is drawn unfilled to match.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Extra {
    /// Eyebrow above the title, e.g. "pizza" or "raffle".
    pub label: String,
    pub title: String,
    pub body: String,
    /// Site-root path to a picture under `public/`, e.g. `/pizza.svg`. Same rule
    /// as a speaker photo: left out until the file exists, and the card lays
    /// itself out as text alone when it is missing.
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub status: Status,
    /// Both halves of the link, or neither. A label with no URL would render as
    /// dead text where people expect somewhere to click.
    #[serde(default)]
    pub cta_label: String,
    #[serde(default)]
    pub cta_url: String,
}

/// Something we're announcing at the event and not before. Withholding is the
/// point, so it carries copy but no link.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Teaser {
    pub label: String,
    pub headline: String,
    pub body: String,
}

/// When an event happens, at whatever precision we've settled on. Content files
/// write either an RFC3339 timestamp (`2026-09-16T18:30:00-04:00`) or a bare
/// month (`2026-10`) for an event we've committed to but not yet scheduled.
/// Month-only events still appear in listings; they just have no cell to sit in
/// on the calendar and nothing precise enough to count down to.
#[derive(Clone, Copy, Debug)]
pub enum EventDate {
    At(OffsetDateTime),
    Month { year: i32, month: Month },
}

impl EventDate {
    /// The exact start, or `None` if only the month is known.
    pub fn instant(self) -> Option<OffsetDateTime> {
        match self {
            EventDate::At(dt) => Some(dt),
            EventDate::Month { .. } => None,
        }
    }

    /// The calendar day this event lands on, or `None` if only the month is known.
    pub fn date(self) -> Option<Date> {
        self.instant().map(OffsetDateTime::date)
    }

    pub fn in_month(self, year: i32, month: Month) -> bool {
        match self {
            EventDate::At(dt) => dt.year() == year && dt.month() == month,
            EventDate::Month { year: y, month: m } => y == year && m == month,
        }
    }

    /// Whether the event is still ahead of us. A month-only event counts as
    /// upcoming for the whole of its month.
    pub fn is_upcoming(self, now: OffsetDateTime) -> bool {
        match self {
            EventDate::At(dt) => dt >= now,
            EventDate::Month { year, month } => {
                (year, u8::from(month)) >= (now.year(), u8::from(now.month()))
            }
        }
    }

    // Month-only events sort after every scheduled event in the same month.
    fn sort_key(self) -> (i32, u8, u8, u8, u8) {
        match self {
            EventDate::At(dt) => (
                dt.year(),
                dt.month().into(),
                dt.day(),
                dt.hour(),
                dt.minute(),
            ),
            EventDate::Month { year, month } => (year, month.into(), u8::MAX, 0, 0),
        }
    }
}

impl Ord for EventDate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_key().cmp(&other.sort_key())
    }
}

impl PartialOrd for EventDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EventDate {
    fn eq(&self, other: &Self) -> bool {
        self.sort_key() == other.sort_key()
    }
}

impl Eq for EventDate {}

impl Serialize for EventDate {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            EventDate::At(dt) => time::serde::rfc3339::serialize(dt, s),
            EventDate::Month { year, month } => {
                s.serialize_str(&format!("{year:04}-{:02}", u8::from(*month)))
            }
        }
    }
}

impl<'de> Deserialize<'de> for EventDate {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        let raw = String::deserialize(d)?;
        parse_event_date(raw.trim()).map_err(D::Error::custom)
    }
}

fn parse_event_date(raw: &str) -> Result<EventDate, String> {
    // `YYYY-MM`, the month-only form. Anything longer is a full timestamp.
    if raw.len() == 7 && raw.as_bytes()[4] == b'-' {
        let year: i32 = raw[..4]
            .parse()
            .map_err(|_| format!("`{raw}` has an invalid year"))?;
        let month = raw[5..]
            .parse::<u8>()
            .ok()
            .and_then(|m| Month::try_from(m).ok())
            .ok_or_else(|| format!("`{raw}` has an invalid month"))?;
        return Ok(EventDate::Month { year, month });
    }

    OffsetDateTime::parse(raw, &time::format_description::well_known::Rfc3339)
        .map(EventDate::At)
        .map_err(|e| format!("`{raw}` is neither an RFC3339 timestamp nor a `YYYY-MM` month: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(raw: &str) -> EventDate {
        parse_event_date(raw).expect("valid timestamp")
    }

    fn slot(minutes: u32) -> Slot {
        Slot {
            minutes,
            title: "t".into(),
            presenter: String::new(),
            detail: String::new(),
            highlight: false,
        }
    }

    // The one thing that can actually go wrong here is a block whose length
    // carries the cursor past the hour.
    #[test]
    fn schedule_clock_carries_across_the_hour() {
        let start = match at("2026-09-30T18:45:00-04:00") {
            EventDate::At(dt) => dt,
            EventDate::Month { .. } => unreachable!(),
        };
        let labels = schedule_clock(start, &[slot(15), slot(45)]);
        assert_eq!(labels, ["18:45 \u{2013} 19:00", "19:00 \u{2013} 19:45"]);
    }

    #[test]
    fn parses_both_forms() {
        assert!(matches!(at("2026-09-16T18:30:00-04:00"), EventDate::At(_)));
        assert!(matches!(
            at("2026-10"),
            EventDate::Month {
                year: 2026,
                month: Month::October
            }
        ));
        assert!(parse_event_date("2026-13").is_err());
        assert!(parse_event_date("October").is_err());
    }

    #[test]
    fn month_only_sorts_after_scheduled_events_in_the_same_month() {
        let mut dates = vec![
            at("2026-10"),
            at("2026-11-18T18:30:00-04:00"),
            at("2026-10-21T18:30:00-04:00"),
        ];
        dates.sort();
        assert_eq!(
            dates,
            vec![
                at("2026-10-21T18:30:00-04:00"),
                at("2026-10"),
                at("2026-11-18T18:30:00-04:00"),
            ]
        );
    }

    // `toml` is only pulled in under `ssr`, so these parse against the real
    // deserializer there and are skipped elsewhere.
    #[cfg(feature = "ssr")]
    #[test]
    fn ordinary_events_parse_without_a_spotlight() {
        let ev: Event = toml::from_str(
            r#"
            title = "Monthly meetup"
            date = "2026-10"
            location = "TBD"
            summary = "s"
            luma_url = ""
            "#,
        )
        .expect("an event file needs no spotlight");
        assert!(ev.spotlight.is_none());
        assert!(!ev.has_luma());
    }

    // `toml` is only pulled in under `ssr`, so these parse against the real
    // deserializer there and are skipped elsewhere.
    #[cfg(feature = "ssr")]
    #[test]
    fn spotlight_parses_and_only_confirmed_names_reach_the_marquee() {
        let ev: Event = toml::from_str(
            r#"
            title = "Monthly meetup"
            date = "2026-09-16T18:30:00-04:00"
            location = "Ottawa"
            summary = "s"
            luma_url = ""

            [spotlight]
            kicker = "A bigger night"
            headline = "RISC-V in 2026"
            tagline = "Where Canada fits."
            draws = ["A panel", "A reveal"]

            [[spotlight.speakers]]
            name = "Megan Lehn"
            affiliation = "RISC-V International"
            affiliation_url = "https://riscv.org"

            [[spotlight.speakers]]
            status = "tba"
            role = "Intro talk"

            [spotlight.panel]
            title = "RISC-V in 2026"
            moderator = { name = "Yusef", role = "Moderator" }

            [[spotlight.panel.panelists]]
            name = "Mike Thompson"

            [[spotlight.panel.panelists]]
            name = "Mike Borza"
            status = "pending"

            [[spotlight.schedule]]
            minutes = 30
            title = "Panel"
            highlight = true

            [[spotlight.extras]]
            label = "pizza"
            title = "We are looking for a pizza sponsor"
            body = "Message an organizer."
            status = "tba"
            "#,
        )
        .expect("valid spotlight");

        let s = ev.spotlight.expect("spotlight present");
        // Speakers default to confirmed, so the marquee picks up Megan and Mike
        // Thompson; the `tba` slot has no name and Mike Borza is only invited.
        assert_eq!(s.marquee_names(), vec!["Megan Lehn", "Mike Thompson"]);
        assert_eq!(s.speakers[0].status, Status::Confirmed);
        assert_eq!(
            s.speakers[0].affiliation_url.as_deref(),
            Some("https://riscv.org")
        );
        // An affiliation with no URL stays plain text rather than becoming a
        // dead link.
        assert!(s.speakers[1].affiliation_url.is_none());
        assert_eq!(s.speakers[1].display_name(), "To be announced");
        assert!(s.schedule[0].highlight);
        // An unfilled extra is an ask, and the page draws it differently.
        assert_eq!(s.extras[0].status, Status::Tba);
        assert_eq!(s.panel.expect("panel").moderator.name, "Yusef");
    }

    #[test]
    fn month_only_stays_upcoming_for_its_whole_month() {
        let late_october = OffsetDateTime::parse(
            "2026-10-28T09:00:00-04:00",
            &time::format_description::well_known::Rfc3339,
        )
        .expect("valid timestamp");
        assert!(at("2026-10").is_upcoming(late_october));
        assert!(!at("2026-09").is_upcoming(late_october));
    }
}
