use anyhow::Result;
use clap::Parser;

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

/// A static site generator for markdown content
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the posts directory
    #[arg(short, long, default_value = "_posts")]
    posts_dir: String,

    /// Path to the output directory
    #[arg(short, long, default_value = "_site")]
    output_dir: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let posts = read_content(&args.posts_dir);

    let posts = posts
        .iter()
        .map(|(file_name, content)| parse_content(file_name, content).unwrap());

    let router = ContentRouter::new(args.posts_dir.clone());
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

    let content_writer = ContentWriter::new(&args.output_dir);
    content_writer.clean_output_dir();

    let posts = posts.collect::<Vec<_>>();

    let index_content = content_renderer.render_index(&posts);
    content_writer.write_content("index.html", &index_content);
    for post in posts {
        content_writer.write_content(&post.route, &post.rendered_content);
    }

    Ok(())
}
