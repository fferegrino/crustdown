pub struct ContentWriter<'a> {
    output_dir: &'a str,
}

impl<'a> ContentWriter<'a> {
    fn new(output_dir: &'a str) -> Self {
        Self { output_dir }
    }

    fn clean_output_dir(&self) {
        std::fs::remove_dir_all(self.output_dir).unwrap();
        std::fs::create_dir_all(self.output_dir).unwrap();
    }

    fn write_content(&self, path_from_root: &str, content: &str) {
        let path = format!("{}/{}", self.output_dir, path_from_root);
        // Create parent directories if they don't exist
        if let Some(parent) = std::path::Path::new(&path).parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, content).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_clean_output_dir() {
        let temp_dir = TempDir::new().unwrap();
        let writer = ContentWriter::new(temp_dir.path().to_str().unwrap());

        // Create some test files
        fs::write(temp_dir.path().join("test1.txt"), "content1").unwrap();
        fs::write(temp_dir.path().join("test2.txt"), "content2").unwrap();

        // Clean the directory
        writer.clean_output_dir();

        // Verify the directory is empty
        assert!(fs::read_dir(temp_dir.path()).unwrap().next().is_none());
    }

    #[test]
    fn test_write_content() {
        let temp_dir = TempDir::new().unwrap();
        let writer = ContentWriter::new(temp_dir.path().to_str().unwrap());

        // Write a test post
        let content = "Test post content";
        writer.write_content("test-post.md", content);

        // Verify the file was created with correct content
        let file_path = temp_dir.path().join("test-post.md");
        assert!(file_path.exists());
        assert_eq!(fs::read_to_string(file_path).unwrap(), content);
    }

    #[test]
    fn test_write_content_with_nested_path() {
        let temp_dir = TempDir::new().unwrap();
        let writer = ContentWriter::new(temp_dir.path().to_str().unwrap());

        // Write a test post in a nested directory
        let content = "Nested test post content";
        writer.write_content("nested/path/test-post.md", content);

        // Verify the file was created with correct content
        let file_path = temp_dir.path().join("nested/path/test-post.md");
        assert!(file_path.exists());
        assert_eq!(fs::read_to_string(file_path).unwrap(), content);
    }

    #[test]
    fn test_write_content_creates_parent_directories() {
        let temp_dir = TempDir::new().unwrap();
        let writer = ContentWriter::new(temp_dir.path().to_str().unwrap());

        // Write to a deeply nested path
        let nested_path = "very/deeply/nested/path/file.txt";
        let content = "Test content";
        writer.write_content(nested_path, content);

        // Verify all parent directories were created
        let mut current_path = temp_dir.path().to_path_buf();
        for component in ["very", "deeply", "nested", "path"] {
            current_path = current_path.join(component);
            assert!(
                current_path.exists(),
                "Directory {} was not created",
                current_path.display()
            );
            assert!(
                current_path.is_dir(),
                "{} is not a directory",
                current_path.display()
            );
        }

        // Verify the file was created with correct content
        let file_path = temp_dir.path().join(nested_path);
        assert!(file_path.exists());
        assert_eq!(fs::read_to_string(file_path).unwrap(), content);
    }
}
