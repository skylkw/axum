use once_cell::sync::Lazy;
use crate::constant::UPLOAD_IMAGE_PATH;
use crate::error::{AppError, AppResult};
use tokio::fs;
use crate::dto::response::ImageResponse;



// 图片服务处理函数
pub async fn show_image(
    filename: &str,
) -> AppResult<ImageResponse> {
    let upload_dir = Lazy::force(&UPLOAD_IMAGE_PATH); 
    let path = upload_dir.join(&filename);
    let data  = fs::read(path).await.map_err(|e| AppError::IoError(e))?;
    // 根据后缀生成不同的 Content-Type
    let content_type = match filename.split('.').last() {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("bmp") => "image/bmp",
        Some("tiff") | Some("tif") => "image/tiff",
        Some("webp") => "image/webp",
        Some("ico") => "image/x-icon",
        Some("svg") => "image/svg+xml",
        Some("heif") | Some("heic") => "image/heif",
        Some("avif") => "image/avif",
        _ => "application/octet-stream",
    };
    Ok(ImageResponse::new(content_type.to_string(), data))
}