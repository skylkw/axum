use super::AppEntity;
use crate::error::ResourceType;
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "annotation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub image_id: i64,
    pub user_id: Uuid,
    pub label: String,
    pub annotation_type: i64,
    pub tag_level1: i64,
    pub tag_level2: i64,
    pub tag_level3: i64,
    pub tag_level4: i64,
    pub tag_level5: i64,
    pub content: serde_json::Value,
    pub create_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}

impl AppEntity for Model {
    const RESOURCE: ResourceType = ResourceType::Annotation;
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::image::Entity",
        from = "Column::ImageId",
        to = "super::image::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    Image,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::tags::Entity",
        from = "Column::TagLevel1",
        to = "super::tags::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    TagLevel1,
    #[sea_orm(
        belongs_to = "super::tags::Entity",
        from = "Column::TagLevel2",
        to = "super::tags::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    TagLevel2,
    #[sea_orm(
        belongs_to = "super::tags::Entity",
        from = "Column::TagLevel3",
        to = "super::tags::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    TagLevel3,
    #[sea_orm(
        belongs_to = "super::tags::Entity",
        from = "Column::TagLevel4",
        to = "super::tags::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    TagLevel4,
    #[sea_orm(
        belongs_to = "super::tags::Entity",
        from = "Column::TagLevel5",
        to = "super::tags::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    TagLevel5,
}

impl Related<super::image::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Image.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TagLevel5.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

