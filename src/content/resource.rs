use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceSection {
    #[serde(default, skip_deserializing)]
    pub slug: String,
    pub title: String,
    #[serde(default)]
    pub links: Vec<ResourceLink>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceLink {
    pub name: String,
    pub href: String,
    #[serde(default)]
    pub description: Option<String>,
}
