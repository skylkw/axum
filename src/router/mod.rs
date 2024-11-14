use crate::server::state::AppState;
use axum::extract::DefaultBodyLimit;
use axum::Router;

pub mod admin;
pub mod image;
pub mod server;
pub mod token;
pub mod user;
pub mod annotation;

pub fn create_router_app(state: AppState) -> Router {
    let router = Router::new();
    let router = server::add_routers(router, "/api/v1/server");
    let router = user::add_routers(router, "/api/v1/user");
    let router = token::add_routers(router, "/api/v1/token");
    let router = admin::user::add_routers(router, "/api/v1/admin/user");
    let router = image::add_routers(router, "/api/v1/image");
    let router = annotation::add_routers(router, "/api/v1/annotation");
    router.with_state(state).layer(DefaultBodyLimit::max(1024 * 1024 * 100))
}
