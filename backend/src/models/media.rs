use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MediaFile {
    pub id: Uuid,
    pub project_id: Option<Uuid>,
    pub project_version_id: Option<Uuid>,
    pub uploaded_by: Uuid,
    pub organization_id: Option<Uuid>,
    pub original_filename: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub media_type: String,
    pub mime_type: String,
    pub file_size_bytes: i64,
    pub checksum_sha256: String,
    pub checksum_md5: Option<String>,
    pub width_px: Option<i32>,
    pub height_px: Option<i32>,
    pub duration_seconds: Option<bigdecimal::BigDecimal>,
    pub page_count: Option<i32>,
    pub bitrate_kbps: Option<i32>,
    pub attributes: serde_json::Value,
    pub version_number: i32,
    pub parent_id: Option<Uuid>,
    pub visibility: String,
    pub access_type: String,
    pub is_primary: bool,
    pub is_processed: bool,
    pub processing_error: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MediaStorageLocation {
    pub id: Uuid,
    pub media_file_id: Uuid,
    pub backend: String,
    pub bucket: Option<String>,
    pub storage_path: String,
    pub cdn_url: Option<String>,
    pub region: Option<String>,
    pub is_primary: bool,
    pub is_replica: bool,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MediaFileWithStorage {
    pub id: Uuid,
    pub project_id: Option<Uuid>,
    pub uploaded_by: Uuid,
    pub original_filename: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub media_type: String,
    pub mime_type: String,
    pub file_size_bytes: i64,
    pub visibility: String,
    pub access_type: String,
    pub is_primary: bool,
    pub is_processed: bool,
    pub version_number: i32,
    pub created_at: OffsetDateTime,
    // From storage join
    pub cdn_url: Option<String>,
    pub storage_path: Option<String>,
    pub backend: Option<String>,
}

// ---- DTOs ----

#[derive(Debug, Deserialize)]
pub struct UploadMediaRequest {
    pub project_id: Option<Uuid>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub access_type: Option<String>,
    pub is_primary: Option<bool>,
    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct MediaFilterParams {
    pub q: Option<String>,
    pub media_type: Option<String>,
    pub project_id: Option<Uuid>,
    pub tags: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Response after a successful upload
#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub media_file: MediaFile,
    pub storage: MediaStorageLocation,
    pub upload_url: Option<String>, // presigned URL if client-side upload
}
