use crate::entities::RawPost;
use minijinja::{Environment, context};
pub enum Content<T> {
    Markdown(T),
}

fn render_markdown_post(post_body: &str) -> String {
    markdown::to_html(post_body)
}

pub struct Rendererer<'a> {
    env: Environment<'a>,
}

impl<'a> Rendererer<'a> {
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.add_template("layout.html", include_str!("_templates/layout.html"))
            .unwrap();
        env.add_template("post.html", include_str!("_templates/post.html"))
            .unwrap();

        Rendererer { env }
    }

    pub fn render(&self, content: &Content<RawPost>) -> String {
        let body = match content {
            Content::Markdown(content) => render_markdown_post(&content.body),
        };

        let metadata = match content {
            Content::Markdown(content) => content.front_matter.clone(),
        };

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
        let post = Content::Markdown(RawPost {
            front_matter: metadata,
            body: "Hello, world!".to_string(),
        });
        let rendered_post = renderer.render(&post);
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
