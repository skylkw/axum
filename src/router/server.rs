use axum::routing::get;
use axum::Router;

use crate::{handler::server, server::state::AppState};

pub fn add_routers(router: Router<AppState>, prefix: &str) -> Router<AppState> {
    router.nest(prefix, server_routes())
}

fn server_routes() -> Router<AppState> {
    Router::new()
        .route("/health_check", get(server::health_check))
        .route("/state", get(server::server_state))
}
