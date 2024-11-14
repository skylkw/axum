use crate::dto::{GetImageListResponse, PageQueryParam};
use crate::error::AppResult;
use crate::server::state::AppState;
use crate::service;
use crate::util::claim::UserClaims;
use axum::body::Body;
use axum::extract::{Multipart, Path, Query, State};
use axum::response::Response;
use axum::Json;
use tracing::info;

use crate::dto::UploadImageResponse;

/// Upload.
pub async fn upload_image(
    State(state): State<AppState>,
    user: UserClaims,
    multipart: Multipart,
) -> AppResult<Json<Vec<UploadImageResponse>>> {
    info!("Get token info by user_id: {}", user.uid);
    match service::image::upload_image(&state, user.uid, multipart).await {
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

/// show image.
pub async fn show_image(Path(filename): Path<String>) -> AppResult<Response> {
    match service::image::show_image(&filename).await {
        Ok(resp) => {
            info!("Success show image.");
            let body = Body::from(resp.body);
            Ok(Response::builder()
                .header("Content-Type", resp.content_type)
                .body(body)
                .unwrap())
        }
        Err(e) => {
            info!("Unsuccessfully show image error: {e:?}.");
            Err(e)
        }
    }
}

pub async fn list(
    State(state): State<AppState>,
    user: UserClaims,
    Query(param): Query<PageQueryParam>,
) -> AppResult<Json<GetImageListResponse>> {
    info!("Get list of image by: {} parameter: {:?}.", user.uid, param);
    match service::image::list(&state, user.uid, param).await {
        Ok(resp) => {
            info!(
                "Success get list of images by user_id: {} response: {resp:?}.",
                user.uid
            );
            Ok(Json(resp))
        }
        Err(e) => {
            info!("Unsuccessful get image list: {e:?}");
            Err(e)
        }
    }
}
