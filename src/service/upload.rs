use crate::constant::UPLOAD_IMAGE_PATH;
use crate::dto::UploadImageResponse;
use crate::error::{AppError, AppResult};
use crate::server::state::AppState;
use crate::util::claim::UserClaims;
use axum::extract::Multipart;
use once_cell::sync::Lazy;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tracing::info;
pub async fn upload_image(
    state: &AppState,
    user: UserClaims,
    mut multipart: Multipart,
) -> AppResult<Vec<UploadImageResponse>> {
    info!("Get token info by user_id: {}", user.uid);

    // 获取目标文件夹路径
    let upload_dir = Lazy::force(&UPLOAD_IMAGE_PATH);
    // 确保目标文件夹存在
    if !upload_dir.exists() {
        fs::create_dir_all(upload_dir)
            .await
            .map_err(AppError::IoError)?;
    }
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
        uploaded_files.push(UploadImageResponse::new(filename, url));
    }
    Ok(uploaded_files)
}
