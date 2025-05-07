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
