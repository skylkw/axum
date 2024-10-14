use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde::Deserialize;
use serde::Serialize;
use strum::EnumString;

use crate::entity;

pub type AppResult<T = ()> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
  // 资源未找到错误
  #[error("{0} not found")]
  NotFoundError(Resource),
  // 资源不可用错误
  #[error("{0} not available")]
  NotAvailableError(Resource),
  // 资源已存在错误
  #[error("{0} already exists")]
  ResourceExistsError(Resource),
  // 权限拒绝错误
  #[error("{0}")]
  PermissionDeniedError(String),
  // 用户未激活错误
  #[error("{0}")]
  UserNotActiveError(String),
  // 无效会话错误
  #[error("{0}")]
  InvalidSessionError(String),
  // 冲突错误
  #[error("{0}")]
  ConflictError(String),
  // 未授权错误
  #[error("{0}")]
  UnauthorizedError(String),
  // 错误请求错误
  #[error("bad request {0}")]
  BadRequestError(String),
  // 无效负载错误
  #[error("{0}")]
  InvalidPayloadError(String),
  // 哈希错误
  #[error("{0}")]
  HashError(String),
  // 无效输入错误
  #[error(transparent)]
  InvalidInputError(#[from] garde::Report),
  // 数据库错误
  #[error(transparent)]
  DatabaseError(#[from] sea_orm::error::DbErr),
  // WebSocket 错误
  #[error(transparent)]
  WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),
  // IO 错误
  #[error(transparent)]
  IoError(#[from] std::io::Error),
  // UUID 错误
  #[error(transparent)]
  UuidError(#[from] uuid::Error),
  // JWT 错误
  #[error(transparent)]
  JwtError(#[from] jsonwebtoken::errors::Error),
  // HTTP 客户端错误
  #[error(transparent)]
  HttpClientError(#[from] reqwest::Error),
  // Redis 错误
  #[error(transparent)]
  RedisError(#[from] redis::RedisError),
  // 配置错误
  #[error(transparent)]
  ConfigError(#[from] config::ConfigError),
  // SMTP 错误
  #[error(transparent)]
  SmtpError(#[from] lettre::transport::smtp::Error),
  // Lettre 错误
  #[error(transparent)]
  LetterError(#[from] lettre::error::Error),
  // JSON 解析错误
  #[error(transparent)]
  ParseJsonError(#[from] serde_json::Error),
  // 浮点数解析错误
  #[error(transparent)]
  ParseFloatError(#[from] std::num::ParseFloatError),
  // 地址解析错误
  #[error(transparent)]
  AddrParseError(#[from] std::net::AddrParseError),
  // 任务生成错误
  #[error(transparent)]
  SpawnTaskError(#[from] tokio::task::JoinError),
  // Tera 模板引擎错误
  #[error(transparent)]
  TeraError(#[from] tera::Error),
  // Base64 解码错误
  #[error(transparent)]
  Base64Error(#[from] base64::DecodeError),
  // Strum 解析错误
  #[error(transparent)]
  StrumParseError(#[from] strum::ParseError),
  // 系统时间错误
  #[error(transparent)]
  SystemTimeError(#[from] std::time::SystemTimeError),
  // Axum 框架错误
  #[error(transparent)]
  AxumError(#[from] axum::Error),
  // 未知错误
  #[error(transparent)]
  UnknownError(#[from] anyhow::Error),
  // 不可到达错误
  #[error(transparent)]
  Infallible(#[from] std::convert::Infallible),
  // 类型头错误
  #[error(transparent)]
  TypeHeaderError(#[from] axum_extra::typed_header::TypedHeaderRejection),
  // 多部分表单错误
  #[error(transparent)]
  MultipartError(#[from] axum::extract::multipart::MultipartError),
}

impl From<argon2::password_hash::Error> for AppError {
  fn from(value: argon2::password_hash::Error) -> Self {
    AppError::HashError(value.to_string())
  }
}

impl AppError {
  pub fn response(self) -> (StatusCode, AppResponseError) {
    use AppError::*;
    let message = self.to_string();
    let (kind, code, details, status_code) = match self {
      InvalidPayloadError(_err) => (
        "INVALID_PAYLOAD_ERROR".to_string(),
        None,
        vec![],
        StatusCode::BAD_REQUEST,
      ),
      BadRequestError(_err) => (
        "BAD_REQUEST_ERROR".to_string(),
        None,
        vec![],
        StatusCode::BAD_REQUEST,
      ),
      NotAvailableError(resource) => (
        format!("{resource}_NOT_AVAILABLE_ERROR"),
        None,
        vec![],
        StatusCode::NOT_FOUND,
      ),
      NotFoundError(resource) => (
        format!("{resource}_NOT_FOUND_ERROR"),
        Some(resource.resource_type as i32),
        resource.details.clone(),
        StatusCode::NOT_FOUND,
      ),
      ResourceExistsError(resource) => (
        format!("{resource}_ALREADY_EXISTS_ERROR"),
        Some(resource.resource_type as i32),
        resource.details.clone(),
        StatusCode::CONFLICT,
      ),
      AxumError(_err) => (
        "AXUM_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      ConfigError(_err) => (
        "CONFIG_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      AddrParseError(_err) => (
        "ADDR_PARSE_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      IoError(err) => {
        let (status, kind, code) = match err.kind() {
          std::io::ErrorKind::NotFound => (
            StatusCode::NOT_FOUND,
            format!("{}_NOT_FOUND_ERROR", ResourceType::File),
            Some(ResourceType::File as i32),
          ),
          std::io::ErrorKind::PermissionDenied => {
            (StatusCode::FORBIDDEN, "FORBIDDEN_ERROR".to_string(), None)
          }
          _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "IO_ERROR".to_string(),
            None,
          ),
        };
        (kind, code, vec![], status)
      }
      WebSocketError(_err) => (
        "WEBSOCKET_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      ParseJsonError(_err) => (
        "PARSE_JSON_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      StrumParseError(_err) => (
        "STRUM_PARSE_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      HttpClientError(_err) => (
        "HTTP_CLIENT_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      SystemTimeError(_err) => (
        "SYSTEM_TIME_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      SpawnTaskError(_err) => (
        "SPAWN_TASK_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      UnknownError(_err) => (
        "UNKNOWN_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      PermissionDeniedError(_err) => (
        "PERMISSION_DENIED_ERROR".to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      InvalidSessionError(_err) => (
        "INVALID_SESSION_ERROR".to_string(),
        None,
        vec![],
        StatusCode::BAD_REQUEST,
      ),
      ConflictError(_err) => (
        "CONFLICT_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      UserNotActiveError(_err) => (
        "USER_NOT_ACTIVE_ERROR".to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      UnauthorizedError(_err) => (
        "UNAUTHORIZED_ERROR".to_string(),
        None,
        vec![],
        StatusCode::UNAUTHORIZED,
      ),
      UuidError(_err) => (
        "UUID_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      JwtError(_err) => (
        "UNAUTHORIZED_ERROR".to_string(),
        None,
        vec![],
        StatusCode::UNAUTHORIZED,
      ),
      RedisError(_err) => (
        "REDIS_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      SmtpError(_err) => (
        "SMTP_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      LetterError(_err) => (
        "LETTER_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      HashError(_err) => (
        "HASH_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      ParseFloatError(_err) => (
        "PARSE_FLOAT_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      TeraError(_err) => (
        "TERA_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      Base64Error(_err) => (
        "BASE64_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      InvalidInputError(err) => (
        "INVALID_INPUT_ERROR".to_string(),
        None,
        err
          .iter()
          .map(|(p, e)| (p.to_string(), e.to_string()))
          .collect(),
        StatusCode::BAD_REQUEST,
      ),
      DatabaseError(_err) => (
        "DATABASE_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      Infallible(_err) => (
        "INFALLIBLE".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      TypeHeaderError(_err) => (
        "TYPE_HEADER_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      MultipartError(_err) => (
        "MULTIPART_ERROR".to_string(),
        None,
        vec![],
        StatusCode::BAD_REQUEST,
      ),
    };

    (
      status_code,
      AppResponseError::new(kind, message, code, details),
    )
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    let (status_code, body) = self.response();
    (status_code, Json(body)).into_response()
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AppResponseError {
  pub kind: String,
  pub error_message: String,
  pub code: Option<i32>,
  pub details: Vec<(String, String)>,
}

impl AppResponseError {
  pub fn new(
    kind: impl Into<String>,
    message: impl Into<String>,
    code: Option<i32>,
    details: Vec<(String, String)>,
  ) -> Self {
    Self {
      kind: kind.into(),
      error_message: message.into(),
      code,
      details,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Resource {
  pub details: Vec<(String, String)>,
  pub resource_type: ResourceType,
}

impl std::fmt::Display for Resource {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // TODO
    self.resource_type.fmt(f)
  }
}

#[derive(Debug, EnumString, strum::Display, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceType {
  #[strum(serialize = "USER")]
  User,
  #[strum(serialize = "FILE")]
  File,
  #[strum(serialize = "SESSION")]
  Session,
  #[strum(serialize = "MESSAGE")]
  Message,
}

pub fn invalid_input_error(field: &'static str, message: &'static str) -> AppError {
  let mut report = garde::Report::new();
  report.append(garde::Path::new(field), garde::Error::new(message));
  AppError::InvalidInputError(report)
}

pub trait ToAppResult {
  type Output: entity::AppEntity;
  fn to_result(self) -> AppResult<Self::Output>;
  fn check_absent(self) -> AppResult;
  fn check_absent_details(self, details: Vec<(String, String)>) -> AppResult;
  fn to_result_details(self, details: Vec<(String, String)>) -> AppResult<Self::Output>;
}

impl<T> ToAppResult for Option<T>
where
  T: entity::AppEntity,
{
  type Output = T;
  fn to_result(self) -> AppResult<Self::Output> {
    self.ok_or_else(|| {
      AppError::NotFoundError(Resource {
        details: vec![],
        resource_type: Self::Output::RESOURCE,
      })
    })
  }

  fn to_result_details(self, details: Vec<(String, String)>) -> AppResult<Self::Output> {
    self.ok_or_else(|| {
      AppError::NotFoundError(Resource {
        details,
        resource_type: Self::Output::RESOURCE,
      })
    })
  }

  fn check_absent(self) -> AppResult {
    if self.is_some() {
      Err(AppError::ResourceExistsError(Resource {
        details: vec![],
        resource_type: Self::Output::RESOURCE,
      }))
    } else {
      Ok(())
    }
  }

  fn check_absent_details(self, details: Vec<(String, String)>) -> AppResult {
    if self.is_some() {
      Err(AppError::ResourceExistsError(Resource {
        details,
        resource_type: Self::Output::RESOURCE,
      }))
    } else {
      Ok(())
    }
  }
}