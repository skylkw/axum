use axum::routing::post;
use axum::Router;

use crate::handler::token;
use crate::server::state::AppState;

pub fn add_routers(router: Router<AppState>, prefix: &str) -> Router<AppState> {
    router.nest(prefix, token_routes())
}

fn token_routes() -> Router<AppState> {
    Router::new()
        .route("/refresh", post(token::refresh))
        .route("/info", post(token::info))
}
