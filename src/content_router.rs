use crate::entities::{ContentKind, RawPost};
use std::collections::HashMap;

pub struct ContentRouter {
    posts_path: String,
}

impl ContentRouter {
    pub fn new(posts_path: String) -> Self {
        Self { posts_path }
    }

    fn get_path_with_no_extension(&self, og_path: &str) -> String {
        std::path::Path::new(&og_path)
            .with_extension("")
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn route_post(&self, raw_post: &RawPost) -> String {
        match raw_post.kind {
            ContentKind::Markdown => {
                format!(
                    "{}/{}/index.html",
                    self.posts_path,
                    self.get_path_with_no_extension(&raw_post.og_path)
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_path_with_no_extension_test() {
        let router = ContentRouter::new("posts".to_string());
        let path = router.get_path_with_no_extension("test.md");
        assert_eq!(path, "test");
    }

    #[test]
    fn test_route() {
        let router = ContentRouter::new("posts".to_string());
        let content = RawPost {
            og_path: "test.md".to_string(),
            front_matter: HashMap::new(),
            body: "test".to_string(),
            kind: ContentKind::Markdown,
        };
        let path = router.route_post(&content);
        assert_eq!(path, "posts/test/index.html");
    }

    #[test]
    fn test_route_with_nested_path() {
        let router = ContentRouter::new("hello-world".to_string());
        let content = RawPost {
            og_path: "test/nested/post.md".to_string(),
            front_matter: HashMap::new(),
            body: "test".to_string(),
            kind: ContentKind::Markdown,
        };
        let path = router.route_post(&content);
        assert_eq!(path, "hello-world/test/nested/post/index.html");
    }
}
