use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(default, skip_deserializing)]
    pub slug: String,
    pub title: String,
    #[serde(with = "time::serde::rfc3339")]
    pub date: OffsetDateTime,
    pub location: String,
    pub summary: String,
    #[serde(default)]
    pub description: String,
    pub luma_url: String,
    #[serde(default)]
    pub tags: Vec<String>,
}
