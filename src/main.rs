mod content_parser;
mod content_reader;
mod content_render;
mod content_router;
mod content_writer;
mod entities;

use content_parser::parse_content;
use content_reader::read_content;
fn main() {
    let default_posts_path = "_posts";
    let posts = read_content(default_posts_path);

    for (file_name, content) in posts.iter() {
        let post = parse_content(file_name, content).unwrap();
        println!("{}", file_name);
        println!("{:?}", post.front_matter);
        println!("{}", post.body);
    }
}
