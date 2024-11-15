use chrono::{DateTime, Utc};
use fake::Dummy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    constant::BEARER,
    entity::{self, role::RoleUser},
    error::AppResponseError,
};

use super::{Annotation, Image};

#[derive(Debug, Serialize, Deserialize, Dummy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetUserListResponse {
    pub list: Vec<GetUserResponse>,
}

#[derive(Debug, Serialize, Deserialize, Dummy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetUserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role_name: RoleUser,
    pub is_active: bool,
    pub is_2fa: bool,
    pub create_at: DateTime<Utc>,
}

impl From<entity::user::Model> for GetUserResponse {
    fn from(user: entity::user::Model) -> Self {
        GetUserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            role_name: user.role,
            is_active: user.is_active,
            is_2fa: user.is_2fa,
            create_at: user.create_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Dummy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServiceStatusResponse {
    pub db: bool,
    pub redis: bool,
    pub email: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse<T> {
    pub data: Vec<T>,
    pub page_num: i64,
    pub page_size: i64,
    pub total: i64,
}

impl<T> PageResponse<T> {
    pub fn new(data: Vec<T>, page_num: i64, page_size: i64, total: i64) -> PageResponse<T> {
        PageResponse {
            data,
            page_num,
            page_size,
            total,
        }
    }

    pub fn map<F, B>(&self, f: F) -> PageResponse<B>
    where
        F: FnMut(&T) -> B,
    {
        let data: Vec<B> = self.data.iter().map(f).collect();
        PageResponse {
            data,
            page_num: self.page_num,
            page_size: self.page_size,
            total: self.total,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Dummy)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResponse {
    pub id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Dummy)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum LoginResponse {
    Token(TokenResponse),
    Code { message: String, expire_in: u64 },
}

impl From<TokenResponse> for LoginResponse {
    fn from(value: TokenResponse) -> Self {
        LoginResponse::Token(value)
    }
}

#[derive(Debug, Serialize, Deserialize, Dummy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expire_in: u64,
}

impl TokenResponse {
    pub fn new(access_token: String, refresh_token: String, expire_in: u64) -> Self {
        Self {
            token_type: BEARER.to_string(),
            access_token,
            refresh_token,
            expire_in,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Dummy)]
#[serde(rename_all = "camelCase")]
pub struct ForgetPasswordResponse {
    pub expire_in: u64,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponse {
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub is_2fa: bool,
    pub create_at: DateTime<Utc>,
}

impl From<entity::user::Model> for ProfileResponse {
    fn from(user: entity::user::Model) -> Self {
        ProfileResponse {
            username: user.username,
            email: user.email,
            is_active: user.is_active,
            is_2fa: user.is_2fa,
            create_at: user.create_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum AppResultResponse<R> {
    Err(AppResponseError),
    Ok(R),
}

impl<R> AppResultResponse<R> {
    #[allow(dead_code)]
    pub const fn is_ok(&self) -> bool {
        matches!(*self, AppResultResponse::Ok(_))
    }
    #[allow(dead_code)]
    pub const fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageResponse {
    pub content_type: String,
    pub body: Vec<u8>,
}

impl ImageResponse {
    pub fn new(content_type: String, body: Vec<u8>) -> Self {
        Self { content_type, body }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadImageResponse {
    pub filename: String,
    pub url: String,
}

impl UploadImageResponse {
    pub fn new(filename: String, url: String) -> Self {
        Self { filename, url }
    }
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetImageListResponse {
    pub list: Vec<Image>,
}

impl GetImageListResponse {
    pub fn new(list: Vec<Image>) -> Self {
        Self { list }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAnnotationsResponse {
    pub annotations: Vec<Annotation>,
}

impl GetAnnotationsResponse {
    pub fn new(annotations: Vec<Annotation>) -> Self {
        Self { annotations }
    }
}
