use std::collections::HashMap;
use std::path::Path;

pub fn read_content<P: AsRef<Path>>(source_directory: &P) -> HashMap<String, String> {
    let mut posts = HashMap::new();

    let files = std::fs::read_dir(source_directory).unwrap();
    for file in files {
        let file_path = file.unwrap().path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let file_content = std::fs::read_to_string(&file_path).unwrap();
        posts.insert(file_name.to_string(), file_content);
    }

    posts
}
