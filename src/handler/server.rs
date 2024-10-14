use crate::client::redis::RedisClientExt;
use crate::dto::{MessageResponse, ServiceStatusResponse};
use crate::error::AppResult;
use crate::server::state::AppState;
use axum::extract::State;
use axum::Json;
use tracing::error;

// Health check.
pub async fn health_check() -> AppResult<Json<MessageResponse>> {
  Ok(Json(MessageResponse::new("Ok")))
}

// Sever connection state.
pub async fn server_state(State(state): State<AppState>) -> AppResult<Json<ServiceStatusResponse>> {
  let db = state.db.ping().await;
  if let Err(e) = db.as_ref() {
    error!("Database connection failed error: {e}.");
  }
  let email = state.email.test_connection().await;
  if let Err(e) = email.as_ref() {
    error!("Email service connection failed error: {e}.");
  }
  let redis = state.redis.ping().await;
  if let Err(e) = redis.as_ref() {
    error!("Redis connection failed error: {e}.");
  }
  let resp = ServiceStatusResponse {
    db: db.is_ok(),
    redis: redis.is_ok(),
    email: email.is_ok(),
  };
  Ok(Json(resp))
}

#[cfg(test)]
pub mod tests {

  use super::*;

  #[tokio::test]
  async fn test_health_check_handler() {
    assert_eq!(health_check().await.unwrap().0.message, "Ok");
  }
}
