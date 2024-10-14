use axum::routing::post;
use axum::Router;

use crate::handler::upload;
use crate::server::state::AppState;

pub fn add_routers(router: Router<AppState>, prefix: &str) -> Router<AppState> {
    router.nest(prefix, upload_routes())
}

fn upload_routes() -> Router<AppState> {
    Router::new().route("/image", post(upload::upload_image))
}
