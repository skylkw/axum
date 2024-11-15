use fake::faker::internet::en::{Password, SafeEmail, Username};
use fake::Dummy;
use garde::Validate;
use serde::{Deserialize, Serialize};
use strum::Display;
use uuid::Uuid;

use super::Annotation;

#[derive(Debug, Deserialize, Serialize, Dummy, Validate)]
pub struct RegisterRequest {
    #[dummy(faker = "Username()")]
    #[garde(ascii, length(min = 3, max = 25))]
    pub username: String,
    #[dummy(faker = "SafeEmail()")]
    #[garde(email)]
    pub email: String,
    #[dummy(faker = "Password(8..100)")]
    #[garde(length(min = 8))]
    pub password: String,
}

impl RegisterRequest {
    pub fn new(username: &str, email: &str, password: &str) -> Self {
        Self {
            password: password.to_string(),
            username: username.to_string(),
            email: email.to_string(),
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

#[derive(Debug, Deserialize, Serialize, Dummy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageQueryParam {
    pub page_num: u64,
    pub page_size: u64,
    pub sort_by: Option<String>,
    pub sort_direction: Option<Direction>,
}

#[derive(
    Serialize, Deserialize, Debug, Display, Dummy, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum Direction {
    DESC,
    ASC,
}

// TODO #![feature(unboxed_closures)] unstable
impl Direction {
    pub fn as_closure<T>(&self) -> impl Fn((T, T)) -> bool
    where
        T: Ord,
    {
        match self {
            Direction::ASC => |(a, b)| a <= b,
            Direction::DESC => |(a, b)| a >= b,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, Dummy)]
pub struct ActiveRequest {
    #[garde(length(min = 5))]
    pub code: String,
    #[garde(skip)]
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Dummy, Validate)]
#[serde(tag = "type")]
pub struct LoginRequest {
    #[dummy(faker = "SafeEmail()")]
    #[garde(email)]
    pub email: String,
    #[dummy(faker = "Password(8..100)")]
    #[garde(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate, Dummy)]
pub struct Login2faRequest {
    #[garde(skip)]
    pub user_id: Uuid,
    #[garde(length(min = 5))]
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Dummy)]
pub struct RefreshTokenRequest {
    #[garde(length(min = 30))]
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Dummy)]
pub struct TokenInfoRequest {
    #[garde(length(min = 30))]
    pub token: String,
}
#[derive(Debug, Deserialize, Validate, Dummy)]
pub struct ForgetPasswordQueryParam {
    #[dummy(faker = "SafeEmail()")]
    #[garde(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Validate, Dummy)]
pub struct SetPasswordRequest {
    #[dummy(faker = "Password(8..100)")]
    #[garde(length(min = 8))]
    pub new_password: String,
    #[garde(length(min = 5))]
    pub code: String,
    #[garde(skip)]
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Validate, Dummy, Default)]
pub struct UpdateProfileRequest {
    #[dummy(faker = "Username()")]
    #[garde(skip)]
    pub username: Option<String>,
    #[dummy(faker = "Password(8..100)")]
    #[garde(length(min = 8))]
    pub password: Option<String>,
    #[garde(skip)]
    pub is_2fa: Option<bool>,
    #[garde(skip)]
    pub is_private: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveAnnotationBulkRequest {
    pub image_id: i64,
    pub annotations: Vec<Annotation>,
}

// 获取annotation的请求参数
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnotationsQueryParams {
    pub image_id: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_invalid_email_register_request() {
        let req = RegisterRequest::new("username", "email", "password");
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_invalid_pass_register_request() {
        let req = RegisterRequest::new("username", "email@test.com", "pass");
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_valid_user_register_request() {
        let req = RegisterRequest::new("foo", "foo@bar.com", "password");
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_valid_register_request() {
        let req = RegisterRequest::new("username", "email@test.com", "password");
        assert!(req.validate().is_ok());
    }
}
