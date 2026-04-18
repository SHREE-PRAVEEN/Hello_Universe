use axum::{
    extract::{Multipart, Path, Query, State},
    Json,
};
use uuid::Uuid;
use crate::{
    config::AppState,
    middleware::{AuthUser, OptionalAuthUser},
    models::media::*,
    services::MediaService,
    utils::errors::{AppError, AppResult},
};

pub async fn upload(
    State(state): State<AppState>,
    user: AuthUser,
    mut multipart: Multipart,
) -> AppResult<Json<UploadResponse>> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut filename = String::from("upload");
    let mut req = UploadMediaRequest {
        project_id: None,
        display_name: None,
        description: None,
        visibility: None,
        access_type: None,
        is_primary: None,
        attributes: None,
    };

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        AppError::BadRequest(format!("Multipart error: {}", e))
    })? {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "file" => {
                filename = field
                    .file_name()
                    .unwrap_or("upload")
                    .to_string();
                file_data = Some(
                    field.bytes().await
                        .map_err(|e| AppError::BadRequest(format!("File read error: {}", e)))?
                        .to_vec(),
                );
            }
            "project_id" => {
                let val = field.text().await.unwrap_or_default();
                req.project_id = Uuid::parse_str(&val).ok();
            }
            "display_name" => {
                req.display_name = Some(field.text().await.unwrap_or_default());
            }
            "description" => {
                req.description = Some(field.text().await.unwrap_or_default());
            }
            "visibility" => {
                req.visibility = Some(field.text().await.unwrap_or_default());
            }
            "access_type" => {
                req.access_type = Some(field.text().await.unwrap_or_default());
            }
            "is_primary" => {
                let val = field.text().await.unwrap_or_default();
                req.is_primary = Some(val == "true");
            }
            _ => {}
        }
    }

    let data = file_data.ok_or_else(|| AppError::BadRequest("No file provided".into()))?;
    let response = MediaService::upload(&state, user.id(), &filename, data, &req).await?;
    Ok(Json(response))
}

pub async fn get(
    State(state): State<AppState>,
    user: OptionalAuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<MediaFile>> {
    let media = MediaService::get_by_id(&state, id).await?;
    Ok(Json(media))
}

pub async fn list_for_project(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> AppResult<Json<Vec<MediaFileWithStorage>>> {
    let files = MediaService::list_for_project(&state, project_id).await?;
    Ok(Json(files))
}

pub async fn download_url(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let url = MediaService::presign_download(&state, id, user.id()).await?;
    Ok(Json(serde_json::json!({ "url": url, "expires_in": 3600 })))
}

pub async fn presign(
    State(state): State<AppState>,
    user: AuthUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<Json<serde_json::Value>> {
    let filename = body["filename"].as_str()
        .ok_or_else(|| AppError::BadRequest("filename required".into()))?;
    let content_type = body["content_type"].as_str()
        .ok_or_else(|| AppError::BadRequest("content_type required".into()))?;
    let url = MediaService::presign_upload(&state, user.id(), filename, content_type).await?;
    Ok(Json(serde_json::json!({ "upload_url": url })))
}

pub async fn delete(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    MediaService::delete(&state, id, user.id()).await?;
    Ok(Json(serde_json::json!({ "message": "File deleted" })))
}
