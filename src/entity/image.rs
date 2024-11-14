use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use fake::Dummy;
use crate::error::ResourceType;
use strum::Display;
use super::AppEntity;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "image")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub filename: String,
    pub url: String,
    pub user_id: Uuid,
    pub level1: ImageLevel1,
    pub level2: ImageLevel2,
    pub create_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}

impl AppEntity for Model {
    const RESOURCE: ResourceType = ResourceType::Image;
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(
  Debug,
  PartialEq,
  Eq,
  strum::EnumString,
  PartialOrd,
  Ord,
  Deserialize,
  Serialize,
  Dummy,
  Clone,
  Copy,
  EnumIter,
  Display,
  Hash,
  DeriveActiveEnum,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "IMAGE_LEVEL1")]
pub enum ImageLevel1 {
    #[sea_orm(string_value = "Optical")]
    Optical,
    #[sea_orm(string_value = "Infrared")]
    Infrared,
    #[sea_orm(string_value = "SAR")]
    Sar,
}

#[derive(
  Debug,
  PartialEq,
  Eq,
  strum::EnumString,
  PartialOrd,
  Ord,
  Deserialize,
  Serialize,
  Dummy,
  Clone,
  Copy,
  EnumIter,
  Display,
  Hash,
  DeriveActiveEnum,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "IMAGE_LEVEL2")]
pub enum ImageLevel2 {
    #[sea_orm(string_value = "Airplane")]
    Airplane,
    #[sea_orm(string_value = "Vehicle")]
    Vehicle,
}

#[cfg(test)]
pub mod tests {
    use fake::{Fake, Faker};
    use sea_orm::Set;
    use test_context::test_context;

    use crate::entity::TransactionTestContext;

    use super::*;

    #[test_context(TransactionTestContext)]
    #[tokio::test]
    async fn test_insert_and_find_image_entity(ctx: &mut TransactionTestContext) {
        let id = Faker.fake();
        let filename: String = Faker.fake();
        let url: String = Faker.fake();
        let user_id: Uuid = Faker.fake();
        let level1: ImageLevel1 = Faker.fake();
        let level2: ImageLevel2 = Faker.fake();
        let create_at: DateTime<Utc> = Faker.fake();
        let update_at: DateTime<Utc> = Faker.fake();

        ActiveModel {
            id: Set(id),
            filename: Set(filename.clone()),
            url: Set(url.clone()),
            user_id: Set(user_id),
            level1: Set(level1),
            level2: Set(level2),
            create_at: Set(create_at),
            update_at: Set(update_at),
        }
        .insert(&**ctx)
        .await
        .unwrap();

        let image = super::Entity::find_by_id(id)
            .one(&**ctx)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(image.filename, filename);
        assert_eq!(image.url, url);
        assert_eq!(image.user_id, user_id);
        assert_eq!(image.level1, level1);
        assert_eq!(image.level2, level2);
    }
}