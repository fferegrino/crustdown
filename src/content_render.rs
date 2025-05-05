fn render_markdown_post(post_body: &str) -> String {
    markdown::to_html(post_body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
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
}
