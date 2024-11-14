use axum::routing::get;
use axum::Router;

use crate::handler::image;
use crate::server::state::AppState;

use axum::routing::post;






pub fn add_routers(router: Router<AppState>, prefix: &str) -> Router<AppState> {
    router.nest(prefix, image_routes())
}

fn image_routes() -> Router<AppState> {
    Router::new()
        .route("/upload", post(image::upload_image))
        .route("/:filename", get(image::show_image))
        .route("/list", get(image::list))
}
