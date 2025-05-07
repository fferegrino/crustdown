use std::collections::HashMap;

pub struct RawPost {
    pub front_matter: HashMap<String, String>,
    pub body: String,
    pub og_path: String,
}
