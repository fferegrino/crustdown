use chrono::NaiveDate;
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

pub struct Metadata {
    pub title: String,
    pub description: String,
    pub date: NaiveDate,
    pub tags: Vec<String>,
    pub extras: HashMap<String, String>,
}

pub struct PostOutput {
    pub route: String,
    pub metadata: Metadata,
    pub rendered_content: String,
}
