use axum::routing::{get, post, put};
use axum::Router;

use crate::handler::user;
use crate::server::state::AppState;

pub fn add_routers(router: Router<AppState>, prefix: &str) -> Router<AppState> {
    router.nest(prefix, user_routes())
}
fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(user::register))
        .route("/active", put(user::active))
        .route("/login", post(user::login))
        .route("/login2fa", post(user::login2fa))
        .route("/logout", post(user::logout))
        .route("/password", get(user::forget_password))
        .route("/password", put(user::reset_password))
        .route("/profile", get(user::get_profile))
        .route("/profile", put(user::update_profile))
        .route("/codes", get(user::get_access_codes))
}
