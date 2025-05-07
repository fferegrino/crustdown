mod content_parser;
mod content_reader;
mod content_render;
mod content_router;
mod content_writer;
mod entities;

use content_parser::parse_content;
use content_reader::read_content;
use content_render::Rendererer;
use content_router::ContentRouter;
fn main() {
    let default_posts_path = "_posts";
    let posts = read_content(default_posts_path);

    let posts = posts.iter().map(|(file_name, content)| {
        let post = parse_content(file_name, content).unwrap();
        post
    });

    let router = ContentRouter::new(default_posts_path.to_string());
    let content_renderer = Rendererer::new();

    let mut rendered_posts = Vec::new();
    let mut rendered_routes = Vec::new();

    for post in posts {
        let route = router.route_post(&post);
        let rendered_post = content_renderer.render_post(&post);
        rendered_posts.push(rendered_post);
        rendered_routes.push(route);
    }

    
}
