use crate::dto::UploadImageResponse;
use crate::error::AppResult;
use crate::server::state::AppState;
use crate::service;
use crate::util::claim::UserClaims;
use axum::extract::{Multipart, State};
use axum::Json;
use tracing::info;

/// Upload.
pub async fn upload_image(State(state): State<AppState>,user: UserClaims, multipart: Multipart) -> AppResult<Json<Vec<UploadImageResponse>>> {
    info!("Get token info by user_id: {}", user.uid);
    match service::upload::upload_image(&state,user, multipart).await {
        Ok(resp) => {
            info!("Success upload image.");
            Ok(Json(resp))
        }
        Err(e) => {
            info!("Unsuccessfully upload image error: {e:?}.");
            Err(e)
        }
    }
}
