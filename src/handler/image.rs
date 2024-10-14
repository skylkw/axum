use crate::error::{AppError, AppResult};
use crate::service;
use axum::body::Body;
use axum::extract::Path;
use axum::response::Response;
use tracing::info;




/// show image.
pub async fn show_image(Path(filename): Path<String>) -> AppResult<Response> {
    match service::image::show_image(&filename).await {
        Ok(resp) => {
            info!("Success show image.");
            let body =  Body::from(resp.body);
            Ok(Response::builder()
                .header("Content-Type", resp.content_type)
                .body(body).unwrap())
        }
        Err(e) => {
            info!("Unsuccessfully show image error: {e:?}.");
            Err(e)
        }
    }
}
