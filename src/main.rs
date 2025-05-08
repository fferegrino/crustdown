mod content_parser;
mod content_reader;
mod content_render;
mod content_router;
mod content_writer;
mod entities;

use content_parser::{parse_content, parse_metadata};
use content_reader::read_content;
use content_render::Rendererer;
use content_router::ContentRouter;
use content_writer::ContentWriter;
use entities::{Metadata, PostOutput};

fn main() {
    let default_posts_path = "_posts";
    let posts = read_content(default_posts_path);

    let posts = posts.iter().map(|(file_name, content)| {
        let post = parse_content(file_name, content).unwrap();
        post
    });

    let router = ContentRouter::new(default_posts_path.to_string());
    let content_renderer = Rendererer::new();

    let posts = posts.map(|post| {
        let route = router.route_post(&post);
        let rendered_post = content_renderer.render_post(&post);
        let metadata = parse_metadata(post.front_matter).unwrap();
        PostOutput {
            route,
            metadata,
            rendered_content: rendered_post,
        }
    });

    let content_writer = ContentWriter::new("_site");
    content_writer.clean_output_dir();

    let posts = posts.collect::<Vec<_>>();

    let index_content = content_renderer.render_index(&posts);
    content_writer.write_content("index.html", &index_content);
    for post in posts {
        content_writer.write_content(&post.route, &post.rendered_content);
    }
}
