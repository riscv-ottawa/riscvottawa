use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use time::{Date, Month, OffsetDateTime};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(default, skip_deserializing)]
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
