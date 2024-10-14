use axum::extract::State;
use axum::Json;
use garde::Validate;
use tracing::{info, warn};

use crate::error::AppResult;
use crate::server::state::AppState;
use crate::util::claim::UserClaims;
use crate::{dto::*, service};

/// Refresh token.

pub async fn refresh(
  State(state): State<AppState>,
  Json(req): Json<RefreshTokenRequest>,
) -> AppResult<Json<TokenResponse>> {
  info!("Refresh token with request: {req:?}.");
  match service::token::refresh(&state, req).await {
    Ok(resp) => {
      info!("Success refresh token user response: {resp:?}.");
      Ok(Json(resp))
    }
    Err(e) => {
      warn!("Unsuccessfully refresh token error: {e:?}.");
      Err(e)
    }
  }
}

/// Get token information.
pub async fn info(
  State(state): State<AppState>,
  user: UserClaims,
  Json(req): Json<TokenInfoRequest>,
) -> AppResult<Json<UserClaims>> {
  req.validate()?;
  info!("Get token information by user_id: {}.", user.uid);
  match service::token::info(&state, user, req).await {
    Ok(resp) => {
      info!("Success get token information response: {resp:?}.");
      Ok(Json(resp))
    }
    Err(e) => {
      warn!("Unsuccessfully get token information error: {e:?}.");
      Err(e)
    }
  }
}
