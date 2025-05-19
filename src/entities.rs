use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub enum ContentKind {
    Markdown,
}

pub struct RawPost {
    pub front_matter: HashMap<String, String>,
    pub body: String,
    pub og_path: String,
    pub kind: ContentKind,
}

#[derive(Debug, Clone, Serialize)]
pub struct Metadata {
    pub title: String,
    pub description: String,
    pub date: NaiveDate,
    pub tags: Vec<String>,
    pub extras: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostOutput {
    pub route: String,
    pub metadata: Metadata,
    pub rendered_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub title: String,
    pub description: String,
}
