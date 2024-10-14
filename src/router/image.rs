use axum::routing::get;
use axum::Router;

use crate::handler::image;
use crate::server::state::AppState;

pub fn add_routers(router: Router<AppState>, prefix: &str) -> Router<AppState> {
    router.nest(prefix, upload_routes())
}

fn upload_routes() -> Router<AppState> {
    Router::new().route("/:filename", get(image::show_image))
}
