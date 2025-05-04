use crate::entities::RawPost;
use std::collections::HashMap;

fn parse_front_matter(lines: Vec<String>) -> Result<HashMap<String, String>, String> {
    let mut front_matter = HashMap::new();

    for line in lines {
        let parts = line.splitn(2, ':').collect::<Vec<&str>>();
        let key = parts[0].trim();
        let value = parts[1].trim();
        if !value.is_empty() {
            front_matter.insert(key.to_string(), value.to_string());
        }
    }

    Ok(front_matter)
}

pub fn parse_content(content: &str) -> Result<RawPost, String> {
    let mut front_matter = Vec::<String>::new();
    let mut body = Vec::<String>::new();

    let mut lines = content.lines();

    let Some(first_line) = lines.next() else {
        return Err("The content is empt".to_string());
    };

    if !first_line.starts_with("---") {
        return Err("No front matter found".to_string());
    }

    let mut is_in_front_matter = true;

    for line in lines {
        if line.starts_with("---") {
            is_in_front_matter = false;
            continue;
        }

        if is_in_front_matter {
            front_matter.push(line.to_string());
        } else {
            body.push(line.to_string());
        }
    }

    Ok(RawPost {
        front_matter: parse_front_matter(front_matter).unwrap(),
        body: body.join("\n"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use textwrap::dedent;

    #[test]
    fn test_parse_content_ok() {
        let content = dedent(
            "
        ---
        title: My First Post
        date: 2021-01-01
        ---
        This is my first post.
        ",
        );
        let content = content.trim();

        let expected_front_matter = HashMap::from([
            ("title".to_string(), "My First Post".to_string()),
            ("date".to_string(), "2021-01-01".to_string()),
        ]);
        let expected_body = "This is my first post.";

        let post = parse_content(&content).unwrap();
        assert_eq!(post.front_matter, expected_front_matter);
        assert_eq!(post.body, expected_body);
    }

    #[test]
    fn test_parse_content_with_blank_front_matter() {
        let content = dedent(
            "
        ---
        ---
        This is my first post.
        ",
        );
        let content = content.trim();

        let expected_front_matter = HashMap::new();
        let expected_body = "This is my first post.";

        let post = parse_content(&content).unwrap();
        assert_eq!(post.front_matter, expected_front_matter);
        assert_eq!(post.body, expected_body);
    }

    #[test]
    fn test_parse_content_with_blank_body() {
        let content = dedent(
            "
        ---
        title: My First Post
        date: 2021-01-01
        ---
        ",
        );
        let content = content.trim();

        let expected_front_matter = HashMap::from([
            ("title".to_string(), "My First Post".to_string()),
            ("date".to_string(), "2021-01-01".to_string()),
        ]);
        let expected_body = "";

        let post = parse_content(&content).unwrap();
        assert_eq!(post.front_matter, expected_front_matter);
        assert_eq!(post.body, expected_body);
    }

    #[test]
    fn test_parse_front_matter_ok() {
        let front_matter = vec![
            "title: My First Post".to_string(),
            "date: 2021-01-01".to_string(),
        ];
        let front_matter = parse_front_matter(front_matter).unwrap();
        assert_eq!(
            front_matter.get("title"),
            Some(&"My First Post".to_string())
        );
        assert_eq!(front_matter.get("date"), Some(&"2021-01-01".to_string()));
    }

    #[test]
    fn test_parse_front_matter_empty_field() {
        let front_matter = vec!["title: My First Post".to_string(), "date:".to_string()];
        let front_matter = parse_front_matter(front_matter).unwrap();
        assert_eq!(
            front_matter.get("title"),
            Some(&"My First Post".to_string())
        );
        assert_eq!(front_matter.get("date"), None);
    }

    #[test]
    fn test_parse_front_matter_empty() {
        let front_matter = vec![];
        let front_matter = parse_front_matter(front_matter).unwrap();
        assert_eq!(front_matter.len(), 0);
    }
}
