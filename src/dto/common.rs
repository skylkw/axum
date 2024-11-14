use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use serde_json::Value;
use uuid::Uuid;

use crate::entity;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub id: i64,
    pub filename: String,
    pub url: String,
    pub user_id: Uuid,
    pub level1: String,
    pub level2: String,
    pub create_at: DateTime<Utc>,
}
impl From<entity::image::Model> for Image {
    fn from(image: entity::image::Model) -> Self {
        Image {
            id: image.id,
            filename: image.filename,
            url: image.url,
            user_id: image.user_id,
            level1: image.level1.to_string(),
            level2: image.level2.to_string(),
            create_at: image.create_at,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Annotation {
    #[serde(rename = "type")]
    pub annotation_type: i64,
    #[serde(rename = "uuid")]
    pub id: Uuid,
    pub label: String,
    #[serde(rename = "tagLevel1")]
    pub tag_level1: i64,
    #[serde(rename = "tagLevel2")]
    pub tag_level2: i64,
    #[serde(rename = "tagLevel3")]
    pub tag_level3: i64,
    #[serde(rename = "tagLevel4")]
    pub tag_level4: i64,
    #[serde(rename = "tagLevel5")]
    pub tag_level5: Option<i64>,
    #[serde(rename = "coor")]
    pub content: Value,
}

impl From<entity::annotation::Model> for Annotation {
    fn from(annotation: crate::entity::annotation::Model) -> Self {
        Self {
            id: annotation.id,
            annotation_type: annotation.annotation_type,
            label: annotation.label,
            tag_level1: annotation.tag_level1,
            tag_level2: annotation.tag_level2,
            tag_level3: annotation.tag_level3,
            tag_level4: annotation.tag_level4,
            tag_level5: Some(annotation.tag_level5),
            content: annotation.content,
        }
    }
}
