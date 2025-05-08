use crate::entities::{ContentKind, Metadata, RawPost};
use chrono::NaiveDate;
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

pub fn parse_content(og_path: &str, content: &str) -> Result<RawPost, String> {
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
        og_path: og_path.to_string(),
        kind: ContentKind::Markdown, // TODO: Currently only markdown is supported
    })
}

pub fn parse_metadata(front_matter: HashMap<String, String>) -> Result<Metadata, String> {
    let mut front_matter = front_matter.clone();
    let title = front_matter.remove("title").ok_or("Title is required")?;
    let description = front_matter
        .remove("description")
        .ok_or("Description is required")?;
    let date_str = front_matter.remove("date").ok_or("Date is required")?;
    let tags_str = front_matter.remove("tags").ok_or("Tags are required")?;

    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date format: {}. Expected YYYY-MM-DD", e))?;

    let tags = tags_str
        .split(',')
        .map(|tag| tag.trim().to_string())
        .collect::<Vec<String>>();

    Ok(Metadata {
        title,
        description,
        date,
        tags,
        extras: front_matter,
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

        let post = parse_content("test.md", &content).unwrap();
        assert_eq!(post.front_matter, expected_front_matter);
        assert_eq!(post.body, expected_body);
        assert_eq!(post.og_path, "test.md");
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

        let post = parse_content("test.md", &content).unwrap();
        assert_eq!(post.front_matter, expected_front_matter);
        assert_eq!(post.body, expected_body);
        assert_eq!(post.og_path, "test.md");
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

        let post = parse_content("test.md", &content).unwrap();
        assert_eq!(post.front_matter, expected_front_matter);
        assert_eq!(post.body, expected_body);
        assert_eq!(post.og_path, "test.md");
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

    #[test]
    fn test_parse_metadata_ok() {
        let front_matter = HashMap::from([
            ("title".to_string(), "My First Post".to_string()),
            ("date".to_string(), "2021-01-01".to_string()),
            (
                "description".to_string(),
                "This is my first post".to_string(),
            ),
            ("tags".to_string(), "tag1,tag2".to_string()),
        ]);

        let metadata = parse_metadata(front_matter).unwrap();
        assert_eq!(metadata.title, "My First Post");
        assert_eq!(metadata.date, NaiveDate::from_ymd_opt(2021, 1, 1).unwrap());
        assert_eq!(metadata.description, "This is my first post");
        assert_eq!(metadata.tags, vec!["tag1", "tag2"]);
        assert_eq!(metadata.extras.len(), 0);
    }

    #[test]
    fn test_parse_metadata_ok_extras() {
        let front_matter = HashMap::from([
            ("title".to_string(), "My First Post".to_string()),
            ("date".to_string(), "2021-01-01".to_string()),
            (
                "description".to_string(),
                "This is my first post".to_string(),
            ),
            ("tags".to_string(), "tag1,tag2".to_string()),
            ("hello".to_string(), "world".to_string()),
            ("foo".to_string(), "bar".to_string()),
        ]);

        let metadata = parse_metadata(front_matter).unwrap();
        assert_eq!(metadata.title, "My First Post");
        assert_eq!(metadata.date, NaiveDate::from_ymd_opt(2021, 1, 1).unwrap());
        assert_eq!(metadata.description, "This is my first post");
        assert_eq!(metadata.tags, vec!["tag1", "tag2"]);
        assert_eq!(metadata.extras.len(), 2);
        assert_eq!(metadata.extras.get("hello"), Some(&"world".to_string()));
        assert_eq!(metadata.extras.get("foo"), Some(&"bar".to_string()));
    }

    #[test]
    fn test_parse_metadata_invalid_date() {
        let front_matter = HashMap::from([
            ("title".to_string(), "My First Post".to_string()),
            ("date".to_string(), "invalid-date".to_string()),
            (
                "description".to_string(),
                "This is my first post".to_string(),
            ),
            ("tags".to_string(), "tag1,tag2".to_string()),
        ]);

        let result = parse_metadata(front_matter);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Invalid date format"));
    }

    #[test]
    fn test_parse_metadata_missing_required_fields() {
        let front_matter = HashMap::from([("title".to_string(), "My First Post".to_string())]);

        let result = parse_metadata(front_matter);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Description is required");
    }
}
