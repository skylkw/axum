use crate::dto::{
    AnnotationsQueryParams, GetAnnotationsResponse, MessageResponse, SaveAnnotationBulkRequest,
};
use crate::error::AppResult;
use crate::server::state::AppState;
use crate::service;
use crate::util::claim::UserClaims;
use axum::extract::{Query, State};
use axum::Json;
use tracing::info;

// pub async fn add_annotation(
//     State(state): State<AppState>,
//     user: UserClaims,
//     annotation: AddAnnotationRequest,
// ) -> AppResult<Json<()>> {
//     info!("Get token info by user_id: {}", user.uid);
//     match service::annotation::add_annotation(&state, user, annotation).await {
//         Ok(_) => {
//             info!("Success add annotation.");
//             Ok(Json(()))
//         }
//         Err(e) => {
//             info!("Unsuccessfully add annotation error: {e:?}.");
//             Err(e)
//         }
//     }
// }

pub async fn save_annotation_bulk(
    State(state): State<AppState>,
    user: UserClaims,
    Json(req): Json<SaveAnnotationBulkRequest>,
) -> AppResult<Json<MessageResponse>> {
    info!("Get token info by user_id: {:?}", req);
    match service::annotation::save_annotation_bulk(
        &state,
        req.image_id,
        user.uid,
        req.annotations.clone(),
    )
    .await
    {
        Ok(_) => {
            info!("Success add annotation.");
            Ok(Json(MessageResponse::new("Success add annotation.")))
        }
        Err(e) => {
            info!("Unsuccessfully add annotation error: {e:?}.");
            Err(e)
        }
    }
}

pub async fn get_annotations_by_image(
    State(state): State<AppState>,
    user: UserClaims,
    Query(req): Query<AnnotationsQueryParams>,
) -> AppResult<Json<GetAnnotationsResponse>> {
    info!("Get annotations by user_id: {:?}", user);
    match service::annotation::get_annotations_by_image(&state, req).await {
        Ok(annotations) => {
            info!("Success get annotations by image.");
            Ok(Json(annotations))
        }
        Err(e) => {
            info!("Unsuccessfully get annotations by image error: {e:?}.");
            Err(e)
        }
    }
}
