use crate::{
    config::AppState,
    integrations::s3::S3Client,
    models::media::*,
    repositories::{AnalyticsRepo, MediaRepo, ModerationRepo},
    utils::{
        crypto::sha256_hex,
        errors::{AppError, AppResult},
    },
};
use mime_guess::MimeGuess;
use uuid::Uuid;

pub struct MediaService;

impl MediaService {
    /// Upload a file: compute checksum, upload to S3, persist metadata
    pub async fn upload(
        state: &AppState,
        uploader_id: Uuid,
        filename: &str,
        data: Vec<u8>,
        req: &UploadMediaRequest,
    ) -> AppResult<UploadResponse> {
        // Enforce size limit
        let max_bytes = state.config.max_file_size_bytes();
        if data.len() as u64 > max_bytes {
            return Err(AppError::FileTooLarge);
        }

        let checksum = sha256_hex(&data);
        let mime = MimeGuess::from_path(filename)
            .first_or_octet_stream()
            .to_string();

        let media_type = Self::classify_mime(&mime);
        let file_size = data.len() as i64;

        // S3 upload
        let s3 = S3Client::new(state);
        let storage_path = format!(
            "uploads/{}/{}/{}",
            uploader_id,
            uuid::Uuid::new_v4(),
            filename
        );
        let cdn_url = s3
            .upload(&storage_path, data, &mime)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("S3 upload failed: {}", e)))?;

        let org_id = None; // TODO: derive from project if needed

        let media_file = MediaRepo::create(
            &state.db,
            req.project_id,
            uploader_id,
            org_id,
            filename,
            &media_type,
            &mime,
            file_size,
            &checksum,
            req,
        )
        .await?;

        let storage = MediaRepo::create_storage_location(
            &state.db,
            media_file.id,
            "s3",
            Some(&state.config.aws_s3_bucket),
            &storage_path,
            Some(&cdn_url),
            Some(&state.config.aws_region),
        )
        .await?;

        // Enqueue for processing (thumbnails, metadata extraction)
        if media_type == "video" || media_type == "image" {
            tracing::info!(
                media_id = %media_file.id,
                "Queued for media processing"
            );
            // jobs::media_processor will pick this up
        }

        // Enqueue for moderation
        ModerationRepo::enqueue(&state.db, "media_file", media_file.id, "new_upload", 3).await?;

        AnalyticsRepo::log_activity(
            &state.db,
            Some(uploader_id),
            None,
            "media.upload",
            Some("media_file"),
            Some(media_file.id),
            Some(filename),
            None,
        )
        .await?;

        Ok(UploadResponse {
            upload_url: Some(cdn_url),
            media_file,
            storage,
        })
    }

    /// Generate a presigned URL for client-side direct upload
    pub async fn presign_upload(
        state: &AppState,
        uploader_id: Uuid,
        filename: &str,
        content_type: &str,
    ) -> AppResult<String> {
        let s3 = S3Client::new(state);
        let key = format!(
            "uploads/{}/{}/{}",
            uploader_id,
            uuid::Uuid::new_v4(),
            filename
        );
        s3.presign_put(&key, content_type, 900)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Presign failed: {}", e)))
    }

    /// Generate a presigned GET URL for private/premium content
    pub async fn presign_download(
        state: &AppState,
        media_id: Uuid,
        requester_id: Uuid,
    ) -> AppResult<String> {
        let media = MediaRepo::find_by_id(&state.db, media_id).await?;

        // Access check
        if media.visibility != "public" || media.access_type != "free" {
            if media.uploaded_by != requester_id {
                let entitled = crate::repositories::CommerceRepo::has_entitlement(
                    &state.db,
                    requester_id,
                    "media_file",
                    media_id,
                )
                .await?;
                if !entitled {
                    return Err(AppError::Forbidden("Access denied to this media file".into()));
                }
            }
        }

        let storage = sqlx::query!(
            "SELECT storage_path FROM media_storage_locations WHERE media_file_id=$1 AND is_primary=TRUE",
            media_id
        )
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::not_found("Media storage location"))?;

        let s3 = S3Client::new(state);
        s3.presign_get(&storage.storage_path, 3600)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Presign failed: {}", e)))
    }

    pub async fn get_by_id(state: &AppState, id: Uuid) -> AppResult<MediaFile> {
        MediaRepo::find_by_id(&state.db, id).await
    }

    pub async fn list_for_project(
        state: &AppState,
        project_id: Uuid,
    ) -> AppResult<Vec<MediaFileWithStorage>> {
        MediaRepo::find_by_project(&state.db, project_id).await
    }

    pub async fn delete(state: &AppState, id: Uuid, requester_id: Uuid) -> AppResult<()> {
        let media = MediaRepo::find_by_id(&state.db, id).await?;
        if media.uploaded_by != requester_id {
            return Err(AppError::Forbidden("Cannot delete another user's file".into()));
        }
        MediaRepo::soft_delete(&state.db, id).await?;
        // Note: actual S3 deletion is handled by a background cleanup job
        Ok(())
    }

    fn classify_mime(mime: &str) -> String {
        match mime.split('/').next().unwrap_or("") {
            "image" => "image".into(),
            "video" => "video".into(),
            "audio" => "document".into(),
            "text" => "code".into(),
            _ => match mime {
                "application/pdf" => "pdf".into(),
                m if m.contains("cad") || m.contains("step") || m.contains("stl") => "cad".into(),
                m if m.contains("zip") || m.contains("tar") => "archive".into(),
                _ => "other".into(),
            },
        }
    }
}
