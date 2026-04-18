use crate::models::media::*;
use sqlx::PgPool;
use uuid::Uuid;
use crate::utils::errors::{AppError, AppResult};

pub struct MediaRepo;

impl MediaRepo {
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<MediaFile> {
        sqlx::query_as!(
            MediaFile,
            r#"SELECT id, project_id, project_version_id, uploaded_by, organization_id,
               original_filename, display_name, description, media_type, mime_type,
               file_size_bytes, checksum_sha256, checksum_md5,
               width_px, height_px, duration_seconds, page_count, bitrate_kbps,
               attributes, version_number, parent_id, visibility, access_type,
               is_primary, is_processed, processing_error,
               created_at, updated_at, deleted_at
               FROM media_files WHERE id = $1 AND deleted_at IS NULL"#,
            id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::not_found("Media file"))
    }

    pub async fn find_by_project(pool: &PgPool, project_id: Uuid) -> AppResult<Vec<MediaFileWithStorage>> {
        Ok(sqlx::query_as!(
            MediaFileWithStorage,
            r#"SELECT mf.id, mf.project_id, mf.uploaded_by, mf.original_filename,
               mf.display_name, mf.description, mf.media_type, mf.mime_type,
               mf.file_size_bytes, mf.visibility, mf.access_type,
               mf.is_primary, mf.is_processed, mf.version_number, mf.created_at,
               msl.cdn_url, msl.storage_path, msl.backend
               FROM media_files mf
               LEFT JOIN media_storage_locations msl ON msl.media_file_id = mf.id AND msl.is_primary = TRUE
               WHERE mf.project_id = $1 AND mf.deleted_at IS NULL
               ORDER BY mf.is_primary DESC, mf.created_at DESC"#,
            project_id
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn create(
        pool: &PgPool,
        project_id: Option<Uuid>,
        uploaded_by: Uuid,
        organization_id: Option<Uuid>,
        filename: &str,
        media_type: &str,
        mime_type: &str,
        file_size: i64,
        checksum: &str,
        req: &UploadMediaRequest,
    ) -> AppResult<MediaFile> {
        Ok(sqlx::query_as!(
            MediaFile,
            r#"INSERT INTO media_files
               (project_id, uploaded_by, organization_id, original_filename, display_name,
                description, media_type, mime_type, file_size_bytes, checksum_sha256,
                visibility, access_type, is_primary, attributes)
               VALUES ($1,$2,$3,$4, COALESCE($5,$4),$6,$7,$8,$9,$10,
                       COALESCE($11,'private'), COALESCE($12,'free'),
                       COALESCE($13, FALSE), COALESCE($14,'{}'))
               RETURNING id, project_id, project_version_id, uploaded_by, organization_id,
               original_filename, display_name, description, media_type, mime_type,
               file_size_bytes, checksum_sha256, checksum_md5,
               width_px, height_px, duration_seconds, page_count, bitrate_kbps,
               attributes, version_number, parent_id, visibility, access_type,
               is_primary, is_processed, processing_error,
               created_at, updated_at, deleted_at"#,
            project_id as _,
            uploaded_by,
            organization_id as _,
            filename,
            req.display_name.as_deref(),
            req.description.as_deref(),
            media_type,
            mime_type,
            file_size,
            checksum,
            req.visibility.as_deref(),
            req.access_type.as_deref(),
            req.is_primary,
            req.attributes as _
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn create_storage_location(
        pool: &PgPool,
        media_file_id: Uuid,
        backend: &str,
        bucket: Option<&str>,
        storage_path: &str,
        cdn_url: Option<&str>,
        region: Option<&str>,
    ) -> AppResult<MediaStorageLocation> {
        Ok(sqlx::query_as!(
            MediaStorageLocation,
            r#"INSERT INTO media_storage_locations
               (media_file_id, backend, bucket, storage_path, cdn_url, region)
               VALUES ($1,$2,$3,$4,$5,$6)
               RETURNING id, media_file_id, backend, bucket, storage_path,
               cdn_url, region, is_primary, is_replica, created_at"#,
            media_file_id, backend, bucket, storage_path, cdn_url, region
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn mark_processed(pool: &PgPool, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            "UPDATE media_files SET is_processed = TRUE, updated_at = NOW() WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn mark_processing_failed(pool: &PgPool, id: Uuid, error: &str) -> AppResult<()> {
        sqlx::query!(
            "UPDATE media_files SET processing_error = $2, updated_at = NOW() WHERE id = $1",
            id, error
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn soft_delete(pool: &PgPool, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            "UPDATE media_files SET deleted_at = NOW() WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn search(
        pool: &PgPool,
        q: &str,
        media_type: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<MediaFileWithStorage>> {
        Ok(sqlx::query_as!(
            MediaFileWithStorage,
            r#"SELECT mf.id, mf.project_id, mf.uploaded_by, mf.original_filename,
               mf.display_name, mf.description, mf.media_type, mf.mime_type,
               mf.file_size_bytes, mf.visibility, mf.access_type,
               mf.is_primary, mf.is_processed, mf.version_number, mf.created_at,
               msl.cdn_url, msl.storage_path, msl.backend
               FROM media_files mf
               LEFT JOIN media_storage_locations msl ON msl.media_file_id = mf.id AND msl.is_primary = TRUE
               WHERE mf.search_vector @@ plainto_tsquery('english', $1)
                 AND mf.deleted_at IS NULL
                 AND ($2::TEXT IS NULL OR mf.media_type = $2)
               ORDER BY ts_rank(mf.search_vector, plainto_tsquery('english', $1)) DESC
               LIMIT $3 OFFSET $4"#,
            q, media_type, limit, offset
        )
        .fetch_all(pool)
        .await?)
    }
}
