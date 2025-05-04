use std::collections::HashMap;
use std::error::Error;

struct Post {
    front_matter: String,
    body: String,
}

fn main() {
    let default_posts_path = "_posts";
    let posts = read_content(default_posts_path);

    for (file_name, content) in posts {
        let post = parse_content(&content).unwrap();
    }
}

fn read_content(source_directory: &str) -> HashMap<String, String> {
    let mut posts = HashMap::new();

    let files = std::fs::read_dir(source_directory).unwrap();
    for file in files {
        let file_path = file.unwrap().path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let file_content = std::fs::read_to_string(&file_path).unwrap();
        posts.insert(file_name.to_string(), file_content);
    }

    for (file_name, content) in posts.clone() {
        let post = parse_content(&content).unwrap();
        println!("{}", file_name);
        println!("{}", post.front_matter);
        println!("{}", post.body);
    }

    posts
}

fn parse_content(content: &str) -> Result<Post, String> {
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

    Ok(Post {
        front_matter: front_matter.join("\n"),
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

        let expected_front_matter = "title: My First Post\ndate: 2021-01-01";
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

        let expected_front_matter = "";
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

        let expected_front_matter = "title: My First Post\ndate: 2021-01-01";
        let expected_body = "";

        let post = parse_content(&content).unwrap();
        assert_eq!(post.front_matter, expected_front_matter);
        assert_eq!(post.body, expected_body);
    }
}
