use axum::Router;

use crate::handler::annotation;
use crate::server::state::AppState;

use axum::routing::{get, post};

pub fn add_routers(router: Router<AppState>, prefix: &str) -> Router<AppState> {
    router.nest(prefix, annotation_routes())
}

fn annotation_routes() -> Router<AppState> {
    Router::new()
        .route("/save_bulk", post(annotation::save_annotation_bulk))
        .route(
            "/image",
            get(annotation::get_annotations_by_image),
        )
}
