use crate::constant::UPLOAD_IMAGE_PATH;
use crate::dto::response::ImageResponse;
use crate::dto::UploadImageResponse;
use crate::dto::{GetImageListResponse, Image, PageQueryParam};
use crate::error::{AppError, AppResult};
use crate::server::state::AppState;
use crate::{entity, repo};
use axum::extract::Multipart;
use once_cell::sync::Lazy;
use sea_orm::TransactionTrait;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tracing::info;
use uuid::Uuid;

pub async fn upload_image(
    state: &AppState,
    user_id: Uuid,
    mut multipart: Multipart,
) -> AppResult<Vec<UploadImageResponse>> {
    // 获取目标文件夹路径
    let upload_dir = Lazy::force(&UPLOAD_IMAGE_PATH);
    // 确保目标文件夹存在
    if !upload_dir.exists() {
        fs::create_dir_all(upload_dir)
            .await
            .map_err(AppError::IoError)?;
    }
    let tx = state.db.begin().await?;
    let mut uploaded_files = Vec::new();
    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::MultipartError(e))?
    {
        let mut filename = field
            .file_name()
            .expect("No file name provided")
            .to_string();
        info!("Uploading file: {}", filename);
        
        // 分离文件名和扩展名
        let mut file_stem = String::from(&filename);
        let mut extension = String::new();
        if let Some(pos) = filename.rfind('.') {
            file_stem = filename[..pos].to_string();
            extension = filename[pos..].to_string();
        }

        // 生成文件路径
        let mut file_path = upload_dir.join(&filename);
        let mut counter = 1;
        while file_path.exists() {
            filename = format!("{}({}){}", file_stem, counter, extension);
            file_path = upload_dir.join(&filename);
            counter += 1;
        }

        let mut o_file = fs::File::create(&file_path)
            .await
            .map_err(|e| AppError::IoError(e))?;

        while let Ok(chunk_data) = field.chunk().await {
            if let Some(bytes_data) = chunk_data {
                o_file
                    .write_all(&bytes_data)
                    .await
                    .map_err(|e| AppError::IoError(e))?;
            } else {
                break;
            }
        }

        let url = format!(
            "http://{}:{}/api/v1/image/{}",
            state.config.server.addr, state.config.server.port, filename
        );
        let res = repo::image::save(
            &tx,
            &filename,
            &url,
            user_id,
            entity::image::ImageLevel1::Sar,
            entity::image::ImageLevel2::Vehicle,
        )
        .await?;
        info!("Image saved with id: {},{}", res, filename);
        uploaded_files.push(UploadImageResponse::new(filename, url));
    }
    tx.commit().await?;
    Ok(uploaded_files)
}

// 图片服务处理函数
pub async fn show_image(filename: &str) -> AppResult<ImageResponse> {
    let upload_dir = Lazy::force(&UPLOAD_IMAGE_PATH);
    let path = upload_dir.join(&filename);
    let data = fs::read(path).await.map_err(|e| AppError::IoError(e))?;
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

pub async fn list(
    state: &AppState,
    user_id: Uuid,
    param: PageQueryParam,
) -> AppResult<GetImageListResponse> {
    info!("Get image list by user_id: {}", user_id);
    let list = repo::image::find_page(&state.db, param)
        .await?
        .into_iter()
        .map(Image::from)
        .collect::<Vec<_>>();
    Ok(GetImageListResponse::new(list))
}
