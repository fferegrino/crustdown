use crate::entities::{ContentKind, PostOutput, RawPost};
use minijinja::{Environment, context};

fn render_markdown_post(post_body: &str) -> String {
    markdown::to_html(post_body)
}

pub struct Rendererer<'a> {
    env: Environment<'a>,
}

impl Rendererer<'_> {
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.add_template("layout.html", include_str!("_templates/layout.html"))
            .unwrap();
        env.add_template("post.html", include_str!("_templates/post.html"))
            .unwrap();
        env.add_template("index.html", include_str!("_templates/index.html"))
            .unwrap();

        Rendererer { env }
    }

    pub fn render_post(&self, content: &RawPost) -> String {
        let body = match content.kind {
            ContentKind::Markdown => render_markdown_post(&content.body),
        };

        let metadata = content.front_matter.clone();

        let template = self.env.get_template("post.html").unwrap();
        template
            .render(context! {
                post => context! {
                    content => body,
                    metadata => metadata
                },
            })
            .unwrap()
    }

    pub fn render_index(&self, posts: &Vec<PostOutput>) -> String {
        let template = self.env.get_template("index.html").unwrap();
        template
            .render(context! {
                posts => posts,
                site => context! {
                    metadata => context! {
                        title => "My Blog"
                    }
                },
            })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;
    use textwrap::dedent;

    #[test]
    fn test_render_markdown_post() {
        let post_body = "Hello, world!";
        let rendered_post = render_markdown_post(post_body);
        assert_eq!(rendered_post, "<p>Hello, world!</p>");
    }

    #[test]
    fn test_render_markdown_post_with_title() {
        let post_body = "# Test\nHello, world!";
        let rendered_post = render_markdown_post(post_body);
        assert_eq!(rendered_post, "<h1>Test</h1>\n<p>Hello, world!</p>");
    }

    #[test]
    fn test_render_markdown_post_with_liquid() {
        let post_body = dedent(
            "
        # Liquid! {{num | minus: 2}}

        ```ruby
        puts 'Hello, world!'
        ```
        ",
        );
        let post_body = post_body.trim();

        let expected_post_body = dedent(
            "
        <h1>Liquid! {{num | minus: 2}}</h1>
        <pre><code class=\"language-ruby\">puts 'Hello, world!'
        </code></pre>
        ",
        );
        let expected_post_body = expected_post_body.trim();

        let rendered_post = render_markdown_post(post_body);
        assert_eq!(rendered_post, expected_post_body);
    }

    #[test]
    fn test_render_post() {
        let renderer = Rendererer::new();
        let mut metadata = HashMap::new();
        metadata.insert("title".to_string(), "¡Hola mundo!".to_string());
        let post = RawPost {
            og_path: "test.md".to_string(),
            front_matter: metadata,
            body: "Hello, world!".to_string(),
            kind: ContentKind::Markdown,
        };
        let rendered_post = renderer.render_post(&post);
        assert_eq!(
            rendered_post,
            "<!DOCTYPE html>
<html>
    <head>
        <title>¡Hola mundo!</title>
    </head>
    <body>

    <div class=\"post-content\">
        <p>Hello, world!</p>
    </div>

    </body>
</html>"
        );
    }
}
