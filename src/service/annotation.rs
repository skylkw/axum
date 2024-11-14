use crate::dto::{Annotation, AnnotationsQueryParams, GetAnnotationsResponse};
use crate::error::{AppError, AppResult};
use crate::repo;
use crate::server::state::AppState;

use sea_orm::TransactionTrait;

use uuid::Uuid;

pub async fn save_annotation_bulk(
    state: &AppState,
    image_id: i64,
    user_id: Uuid,
    annotations: Vec<Annotation>,
) -> AppResult<()> {
    let tx = state.db.begin().await?;
    // 判断image_id是否存在
    match repo::image::find_by_id(&tx, image_id).await? {
        Some(_) => {
            repo::annotation::save_bulk(&tx, image_id, user_id, annotations).await?;
            tx.commit().await?;
            Ok(())
        }
        None => Err(AppError::BadRequestError("Image not found".to_string())),
    }
}

pub async fn get_annotations_by_image(
    state: &AppState,
    req: AnnotationsQueryParams,
) -> AppResult<GetAnnotationsResponse> {
    let annotations = repo::annotation::get_annotations_by_image(&state.db, req.image_id)
        .await?
        .into_iter()
        .map(Annotation::from)
        .collect();
    Ok(GetAnnotationsResponse::new(annotations))
}
