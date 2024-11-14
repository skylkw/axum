use crate::entity;
use chrono::Utc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::dto::Annotation;
use crate::error::{AppError, AppResult};

#[tracing::instrument]
pub async fn save(
    tx: &DatabaseTransaction,
    image_id: i64,
    user_id: Uuid,
    annotation: Annotation,
) -> AppResult<Uuid> {
    let annotation = entity::annotation::ActiveModel {
        id: Set(annotation.id),
        image_id: Set(image_id),
        user_id: Set(user_id),
        label: Set(annotation.label),
        annotation_type: Set(annotation.annotation_type),
        tag_level1: Set(annotation.tag_level1),
        tag_level2: Set(annotation.tag_level2),
        tag_level3: Set(annotation.tag_level3),
        tag_level4: Set(annotation.tag_level4),
        tag_level5: Set(annotation.tag_level5.unwrap_or(0)),
        content: Set(annotation.content),
        create_at: Set(Utc::now()),
        update_at: Set(Utc::now()),
    };
    let annotation = annotation.insert(tx).await?;
    Ok(annotation.id)
}

pub async fn save_bulk(
    tx: &DatabaseTransaction,
    image_id: i64,
    user_id: Uuid,
    annotations: Vec<Annotation>,
) -> AppResult<()> {
    let annotations: Vec<entity::annotation::ActiveModel> = annotations
        .into_iter()
        .map(|annotation| entity::annotation::ActiveModel {
            id: Set(annotation.id),
            image_id: Set(image_id),
            user_id: Set(user_id),
            label: Set(annotation.label),
            annotation_type: Set(annotation.annotation_type),
            tag_level1: Set(annotation.tag_level1),
            tag_level2: Set(annotation.tag_level2),
            tag_level3: Set(annotation.tag_level3),
            tag_level4: Set(annotation.tag_level4),
            tag_level5: Set(annotation.tag_level5.unwrap_or(0)),
            content: Set(annotation.content),
            create_at: Set(Utc::now()),
            update_at: Set(Utc::now()),
        })
        .collect();

    entity::annotation::Entity::insert_many(annotations)
        .on_conflict(
            OnConflict::column(entity::annotation::Column::Id)
                .update_columns([
                    entity::annotation::Column::ImageId,
                    entity::annotation::Column::UserId,
                    entity::annotation::Column::AnnotationType,
                    entity::annotation::Column::TagLevel1,
                    entity::annotation::Column::TagLevel2,
                    entity::annotation::Column::TagLevel3,
                    entity::annotation::Column::TagLevel4,
                    entity::annotation::Column::TagLevel5,
                    entity::annotation::Column::Content,
                    entity::annotation::Column::UpdateAt,
                ])
                .to_owned(),
        )
        .exec(tx)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;
    Ok(())
}

// 根据图片id获取标注信息
pub async fn get_annotations_by_image(
    db: &DatabaseConnection,
    image_id: i64,
) -> AppResult<Vec<entity::annotation::Model>> {
    let annotations = entity::annotation::Entity::find()
        .filter(entity::annotation::Column::ImageId.eq(image_id))
        .all(db)
        .await?;
    Ok(annotations)
}