use axum::routing::get;
use axum::Router;

use crate::handler::admin;
use crate::server::state::AppState;

pub fn add_routers(router: Router<AppState>, prefix: &str) -> Router<AppState> {
    router.nest(prefix, admin_routes())
}

fn admin_routes() -> Router<AppState> {
    Router::new().route("/user/list", get(admin::user::list))
}
