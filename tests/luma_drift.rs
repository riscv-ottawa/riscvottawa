//! Drift check between `content/events/*.toml` and the Luma calendar's public ICS feed.
//!
//! Luma is authoritative for logistics: when an event page goes live, what time
//! it starts, and where. This test catches the places where the hand-written YAML
//! events in this repo may mistakenly disagree; most often an event that went live on
//! Luma while its `luma_url` here is still empty.
//!
//! It talks to the network, so it is ignored by default:
//!
//! ```text
//! cargo test --features ssr --no-default-features --test luma_drift -- --ignored --nocapture
//! ```
#![cfg(feature = "ssr")]

use riscvottawa::content::{ContentStore, Event, EventDate};
use std::process::Command;
use time::format_description::well_known::Rfc3339;
use time::{Month, OffsetDateTime, UtcOffset};

const CALENDAR_ID: &str = "cal-18SVzTkmrwRASgg";

/// Luma writes every `DTSTART` in UTC, so an evening meetup lands late in the
/// UTC day and, at a month boundary, can roll into the next UTC month. Shifting
/// both sides to a fixed Ottawa-ish offset before bucketing by month keeps such
/// an event in the month a reader would name. Only the month is taken from the
/// result, so the DST hour does not matter.
const MONTH_BUCKET_OFFSET: UtcOffset = time::macros::offset!(-5);

#[derive(Debug, Clone)]
struct LumaEvent {
    uid: String,
    start: OffsetDateTime,
    title: String,
    location: String,
    url: Option<String>,
}

#[test]
#[ignore = "fetches the Luma ICS feed; run with --ignored"]
fn content_events_match_the_luma_feed() {
    let store = ContentStore::load_from_dir("content").expect("content/ loads");
    let feed = fetch_feed().unwrap_or_else(|e| panic!("{e}"));
    let luma =
        parse_calendar(&feed).unwrap_or_else(|e| panic!("could not parse the Luma feed: {e}"));

    // An empty parse means the fetch or the format changed, not that the
    // calendar is empty. Treating it as "no drift" would make this test pass
    // for the wrong reason.
    assert!(
        !luma.is_empty(),
        "the Luma feed parsed to zero events; check the feed URL and format before trusting this"
    );

    let problems = compare(&store.events, &luma);

    println!(
        "checked {} content events against {} Luma events",
        store.events.len(),
        luma.len()
    );

    assert!(
        problems.is_empty(),
        "content/events is out of sync with Luma:\n\n{}\n",
        problems.join("\n\n")
    );
}

/// Compares both sides and returns one block of text per event that disagrees.
/// Title and location are deliberately not compared.
fn compare(events: &[Event], luma: &[LumaEvent]) -> Vec<String> {
    let luma_slugs: Vec<Option<String>> = luma
        .iter()
        .map(|l| l.url.as_deref().and_then(luma_slug))
        .collect();

    let mut problems = Vec::new();
    let mut claimed = vec![false; luma.len()];

    for event in events {
        let mut notes: Vec<String> = Vec::new();
        let want = luma_slug(&event.luma_url);

        let by_slug = want
            .as_deref()
            .and_then(|w| luma_slugs.iter().position(|s| s.as_deref() == Some(w)));

        if want.is_some() && by_slug.is_none() {
            notes.push(format!(
                "luma_url is {} but no event in the feed has that link",
                event.luma_url
            ));
        }

        // Falling back to the month lets us match an event whose Luma page
        // exists but whose `luma_url` here is still empty, which is the whole
        // point of the check. One meetup a month makes this unambiguous.
        let matched = by_slug.or_else(|| {
            let bucket = toml_month(event);
            luma.iter()
                .enumerate()
                .find_map(|(i, l)| (!claimed[i] && luma_month(l) == bucket).then_some(i))
        });

        let Some(i) = matched else {
            // Nothing on Luma for this event. Expected for the TBD entries we
            // publish ahead of their Luma page, so it is not a problem on its
            // own; only a broken luma_url is, and that is already noted.
            if !notes.is_empty() {
                problems.push(block(event, notes));
            }
            continue;
        };
        claimed[i] = true;
        let l = &luma[i];

        if want.is_none() {
            match l.url.as_deref() {
                Some(url) => notes.push(format!(
                    "published on Luma; set luma_url = \"{url}\"\n    Luma venue: {}",
                    l.location
                )),
                None => notes.push(format!(
                    "matched Luma event \"{}\" but its description carries no event link",
                    l.title
                )),
            }
        } else if let (Some(a), Some(b)) = (want.as_deref(), luma_slugs[i].as_deref()) {
            if a != b {
                notes.push(format!(
                    "luma_url is {} but the feed has this month's event at https://luma.com/{b}",
                    event.luma_url
                ));
            }
        }

        match event.date {
            EventDate::At(dt) if dt != l.start => notes.push(format!(
                "date is {} but Luma starts it at {}",
                stamp(dt),
                stamp(l.start)
            )),
            EventDate::Month { year, month } => notes.push(format!(
                "date is month-only ({year}-{:02}) but Luma has scheduled it: date = \"{}\"",
                u8::from(month),
                stamp(l.start)
            )),
            EventDate::At(_) => {}
        }

        if !notes.is_empty() {
            problems.push(block(event, notes));
        }
    }

    for (i, l) in luma.iter().enumerate() {
        if !claimed[i] {
            problems.push(format!(
                "on Luma but not in content/events/: \"{}\"\n  - starts {}\n  - {}\n  - {}",
                l.title,
                stamp(l.start),
                l.url.as_deref().unwrap_or("(no link in description)"),
                l.location
            ));
        }
    }

    problems
}

fn block(event: &Event, notes: Vec<String>) -> String {
    let mut out = format!("content/events/{}.toml", event.slug);
    for note in notes {
        out.push_str("\n  - ");
        out.push_str(&note);
    }
    out
}

fn stamp(dt: OffsetDateTime) -> String {
    dt.format(&Rfc3339).unwrap_or_else(|_| dt.to_string())
}

fn toml_month(event: &Event) -> (i32, Month) {
    match event.date {
        EventDate::At(dt) => {
            let local = dt.to_offset(MONTH_BUCKET_OFFSET);
            (local.year(), local.month())
        }
        EventDate::Month { year, month } => (year, month),
    }
}

fn luma_month(l: &LumaEvent) -> (i32, Month) {
    let local = l.start.to_offset(MONTH_BUCKET_OFFSET);
    (local.year(), local.month())
}

/// The trailing path segment of a Luma event link, which is the only part that
/// identifies the event. `luma.com` and `lu.ma` are the same host to Luma.
fn luma_slug(url: &str) -> Option<String> {
    let trimmed = url.trim().trim_end_matches('/');
    for prefix in [
        "https://luma.com/",
        "https://lu.ma/",
        "http://luma.com/",
        "http://lu.ma/",
    ] {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            if !rest.is_empty() {
                return Some(rest.to_ascii_lowercase());
            }
        }
    }
    None
}

fn fetch_feed() -> Result<String, String> {
    let url = format!("https://api.lu.ma/ics/get?entity=calendar&id={CALENDAR_ID}");
    let out = Command::new("curl")
        .args(["--silent", "--show-error", "--fail", "--location"])
        .args(["--max-time", "20"])
        .arg(&url)
        .output()
        .map_err(|e| format!("could not run curl to fetch {url}: {e}"))?;

    if !out.status.success() {
        return Err(format!(
            "fetching {url} failed ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    String::from_utf8(out.stdout).map_err(|e| format!("{url} returned invalid UTF-8: {e}"))
}

fn parse_calendar(raw: &str) -> Result<Vec<LumaEvent>, String> {
    let unfolded = unfold(raw);
    let mut out = Vec::new();
    let mut current: Option<Vec<(String, String)>> = None;

    for line in unfolded.lines() {
        match line.trim_end_matches('\r') {
            "BEGIN:VEVENT" => current = Some(Vec::new()),
            "END:VEVENT" => {
                let props = current
                    .take()
                    .ok_or_else(|| "END:VEVENT without a matching BEGIN".to_string())?;
                out.push(build_event(&props)?);
            }
            other => {
                if let Some(props) = current.as_mut() {
                    if let Some(prop) = split_property(other) {
                        props.push(prop);
                    }
                }
            }
        }
    }
    Ok(out)
}

/// RFC 5545 folds long lines by inserting a line break and a single space or
/// tab. Undo that before anything else reads a property.
fn unfold(raw: &str) -> String {
    raw.replace("\r\n ", "")
        .replace("\r\n\t", "")
        .replace("\n ", "")
        .replace("\n\t", "")
}

/// Splits `NAME;PARAM="a:b":value` into its uppercased name and its value. The
/// separating colon is the first one outside a quoted parameter, which matters
/// for lines like `ORGANIZER;CN="Fatima":MAILTO:...`.
fn split_property(line: &str) -> Option<(String, String)> {
    let mut in_quotes = false;
    for (i, c) in line.char_indices() {
        match c {
            '"' => in_quotes = !in_quotes,
            ':' if !in_quotes => {
                let head = &line[..i];
                let name = head.split(';').next().unwrap_or(head);
                return Some((name.to_ascii_uppercase(), line[i + 1..].to_string()));
            }
            _ => {}
        }
    }
    None
}

fn build_event(props: &[(String, String)]) -> Result<LumaEvent, String> {
    let get = |key: &str| {
        props
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.as_str())
    };

    let uid = get("UID").unwrap_or("(no UID)").to_string();
    let raw_start = get("DTSTART").ok_or_else(|| format!("VEVENT {uid} has no DTSTART"))?;
    let start = parse_utc_stamp(raw_start).ok_or_else(|| {
        format!("VEVENT {uid} has a DTSTART this check cannot read: `{raw_start}`")
    })?;
    let description = unescape(get("DESCRIPTION").unwrap_or_default());

    Ok(LumaEvent {
        uid,
        start,
        title: unescape(get("SUMMARY").unwrap_or_default()),
        location: unescape(get("LOCATION").unwrap_or_default()),
        url: extract_luma_url(&description),
    })
}

/// Luma emits UTC stamps (`20260819T223000Z`). Local or date-only forms would
/// need timezone handling we do not have, so they are rejected loudly rather
/// than guessed at.
fn parse_utc_stamp(raw: &str) -> Option<OffsetDateTime> {
    let format = time::macros::format_description!("[year][month][day]T[hour][minute][second]Z");
    time::PrimitiveDateTime::parse(raw.trim(), &format)
        .ok()
        .map(time::PrimitiveDateTime::assume_utc)
}

fn unescape(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') | Some('N') => out.push('\n'),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

/// Luma has no URL property; it opens the description with
/// "Get up-to-date information at: <link>".
fn extract_luma_url(description: &str) -> Option<String> {
    for prefix in ["https://luma.com/", "https://lu.ma/"] {
        let Some(start) = description.find(prefix) else {
            continue;
        };
        let rest = &description[start..];
        let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
        let url = rest[..end].trim_end_matches(['.', ',', ')']);
        if url.len() > prefix.len() {
            return Some(url.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // A folded DESCRIPTION, a quoted parameter on ORGANIZER, and escaped commas
    // in LOCATION: the three shapes that broke naive parsing of the real feed.
    const FIXTURE: &str = "BEGIN:VCALENDAR\r\n\
        VERSION:2.0\r\n\
        BEGIN:VEVENT\r\n\
        DTSTART:20260819T223000Z\r\n\
        DTEND:20260820T003000Z\r\n\
        ORGANIZER;CN=\"Fatima\":MAILTO:calendar-invite@lu.ma\r\n\
        UID:evt-1j8nckxA60kEVWq@events.lu.ma\r\n\
        SUMMARY:RISC-V Ottawa monthly meeting (August)\r\n\
        DESCRIPTION:Get up-to-date information at: https://luma.com/exynnx7d\\n\\nAd\r\n\
        \x20dress:\\nIncubator13\r\n\
        LOCATION:Incubator13\\, 815 St. Laurent Blvd\\, Ottawa\\, ON\r\n\
        END:VEVENT\r\n\
        END:VCALENDAR\r\n";

    fn parsed() -> LumaEvent {
        let events = parse_calendar(FIXTURE).expect("fixture parses");
        assert_eq!(events.len(), 1);
        events.into_iter().next().unwrap()
    }

    #[test]
    fn reads_a_vevent_the_way_luma_writes_one() {
        let e = parsed();
        assert_eq!(e.uid, "evt-1j8nckxA60kEVWq@events.lu.ma");
        assert_eq!(e.title, "RISC-V Ottawa monthly meeting (August)");
        assert_eq!(stamp(e.start), "2026-08-19T22:30:00Z");
        assert_eq!(e.url.as_deref(), Some("https://luma.com/exynnx7d"));
    }

    #[test]
    fn unfolds_continuations_and_unescapes_text() {
        let e = parsed();
        // "Ad" + folded "dress:" rejoins, and the \n escapes become newlines.
        assert!(e.location.contains("815 St. Laurent Blvd, Ottawa, ON"));
        assert_eq!(
            e.url.as_deref(),
            Some("https://luma.com/exynnx7d"),
            "the link must not absorb the escaped newline that follows it"
        );
    }

    #[test]
    fn organizer_quoted_colon_does_not_split_early() {
        let (name, value) =
            split_property("ORGANIZER;CN=\"Fatima:Joe\":MAILTO:a@lu.ma").expect("splits");
        assert_eq!(name, "ORGANIZER");
        assert_eq!(value, "MAILTO:a@lu.ma");
    }

    #[test]
    fn slugs_ignore_host_and_trailing_slash() {
        assert_eq!(
            luma_slug("https://luma.com/exynnx7d").as_deref(),
            Some("exynnx7d")
        );
        assert_eq!(
            luma_slug("https://lu.ma/exynnx7d/").as_deref(),
            Some("exynnx7d")
        );
        assert_eq!(luma_slug(""), None);
        assert_eq!(luma_slug("https://luma.com/"), None);
    }

    #[test]
    fn rejects_a_dtstart_it_cannot_read() {
        // Local and date-only forms need timezone handling this check does not
        // have; guessing would silently misreport drift.
        assert!(parse_utc_stamp("20260819T183000").is_none());
        assert!(parse_utc_stamp("20260819").is_none());
        assert!(parse_utc_stamp("20260819T223000Z").is_some());
    }

    fn event(slug: &str, date: &str, luma_url: &str) -> Event {
        let toml = format!(
            "title = \"t\"\ndate = \"{date}\"\nlocation = \"l\"\nsummary = \"s\"\nluma_url = \"{luma_url}\"\n"
        );
        let mut e: Event = toml::from_str(&toml).expect("valid event toml");
        e.slug = slug.to_string();
        e
    }

    #[test]
    fn flags_an_event_luma_published_while_luma_url_is_empty() {
        let problems = compare(&[event("2026-08-monthly", "2026-08", "")], &[parsed()]);
        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("content/events/2026-08-monthly.toml"));
        assert!(problems[0].contains("set luma_url = \"https://luma.com/exynnx7d\""));
        assert!(problems[0].contains("date = \"2026-08-19T22:30:00Z\""));
    }

    #[test]
    fn flags_a_start_time_that_moved_on_luma() {
        let problems = compare(
            &[event(
                "2026-08-monthly",
                "2026-08-19T18:00:00-04:00",
                "https://luma.com/exynnx7d",
            )],
            &[parsed()],
        );
        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("Luma starts it at 2026-08-19T22:30:00Z"));
    }

    #[test]
    fn flags_a_luma_event_with_no_toml_at_all() {
        let problems = compare(&[], &[parsed()]);
        assert_eq!(problems.len(), 1);
        assert!(problems[0].starts_with("on Luma but not in content/events/"));
    }

    #[test]
    fn flags_a_luma_url_that_is_no_longer_in_the_feed() {
        let problems = compare(
            &[event(
                "2026-12-monthly",
                "2026-12-16T18:30:00-05:00",
                "https://luma.com/deleted99",
            )],
            &[parsed()],
        );
        assert_eq!(
            problems.len(),
            2,
            "the stale link, plus August unaccounted for"
        );
        assert!(problems
            .iter()
            .any(|p| p.contains("no event in the feed has that link")));
    }

    #[test]
    fn stays_quiet_when_both_sides_agree() {
        let matching = event(
            "2026-08-monthly",
            "2026-08-19T18:30:00-04:00",
            "https://luma.com/exynnx7d",
        );
        assert!(compare(&[matching], &[parsed()]).is_empty());
    }

    #[test]
    fn a_tbd_month_entry_with_no_luma_page_is_not_drift() {
        // We publish month-only entries before their Luma page exists. That is
        // the normal state of the next two months, not a problem to report.
        let tbd = event("2026-11-monthly", "2026-11", "");
        assert!(compare(&[tbd], &[]).is_empty());
    }
}
