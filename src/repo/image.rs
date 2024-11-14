use crate::dto::{Direction, PageQueryParam};
use crate::entity;
use crate::entity::image::{ImageLevel1, ImageLevel2};
use chrono::Utc;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, PaginatorTrait, QueryOrder, Set
};
use uuid::Uuid;

use crate::error::AppResult;

#[tracing::instrument]
pub async fn save(
    tx: &DatabaseTransaction,
    filename: &str,
    url: &str,
    user_id: Uuid,
    level1: ImageLevel1,
    level2: ImageLevel2,
) -> AppResult<i64> {
    let image = crate::entity::image::ActiveModel {
        id: NotSet,
        filename: Set(filename.to_string()),
        url: Set(url.to_string()),
        user_id: Set(user_id),
        level1: Set(level1),
        level2: Set(level2),
        create_at: Set(Utc::now()),
        update_at: Set(Utc::now()),
    }
    .insert(tx)
    .await?;
    Ok(image.id)
}

#[tracing::instrument(skip_all)]
pub async fn find_by_id<C>(
    conn: &C,
    id: i64,
) -> AppResult<Option<entity::image::Model>>
where
  C: ConnectionTrait,
   {
    let model = entity::image::Entity::find_by_id(id).one(conn).await?;
    Ok(model)
}


#[tracing::instrument(skip_all)]
pub async fn find_page(
    conn: &DatabaseConnection,
    param: PageQueryParam,
) -> AppResult<Vec<entity::image::Model>> {
    let mut select = entity::image::Entity::find();
    match param.sort_direction {
        Some(Direction::DESC) => {
            select = select.order_by_desc(entity::image::Column::CreateAt);
        }
        _ => {
            select = select.order_by_asc(entity::image::Column::CreateAt);
        }
    }
    let models = select
        .paginate(conn, param.page_size)
        .fetch_page(param.page_num)
        .await?;
    Ok(models)
}

//
// TODO 删除图片
