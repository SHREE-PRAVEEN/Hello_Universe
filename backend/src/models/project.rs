use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub organization_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub title: String,
    pub slug: String,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub visibility: String,
    pub access_type: String,
    pub price: Option<bigdecimal::BigDecimal>,
    pub currency: String,
    pub license: String,
    pub license_details: Option<String>,
    pub version_label: String,
    pub thumbnail_url: Option<String>,
    pub demo_url: Option<String>,
    pub repository_url: Option<String>,
    pub download_count: i64,
    pub view_count: i64,
    pub like_count: i64,
    pub featured: bool,
    pub metadata: serde_json::Value,
    pub published_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProjectWithAuthor {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub short_description: Option<String>,
    pub status: String,
    pub visibility: String,
    pub access_type: String,
    pub price: Option<bigdecimal::BigDecimal>,
    pub currency: String,
    pub license: String,
    pub version_label: String,
    pub thumbnail_url: Option<String>,
    pub download_count: i64,
    pub view_count: i64,
    pub like_count: i64,
    pub featured: bool,
    pub published_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    // Author
    pub owner_id: Uuid,
    pub owner_username: String,
    pub owner_display_name: Option<String>,
    pub owner_avatar_url: Option<String>,
    // Org
    pub organization_id: Option<Uuid>,
    pub organization_name: Option<String>,
    pub organization_slug: Option<String>,
    // Category
    pub category_name: Option<String>,
    // Aggregates
    pub avg_rating: Option<f64>,
    pub review_count: Option<i64>,
    pub tags: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProjectVersion {
    pub id: Uuid,
    pub project_id: Uuid,
    pub version_label: String,
    pub commit_hash: Option<String>,
    pub changelog: Option<String>,
    pub snapshot_meta: serde_json::Value,
    pub created_by: Uuid,
    pub is_current: bool,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProjectCollaborator {
    pub id: Uuid,
    pub project_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub added_by: Option<Uuid>,
    pub added_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProjectCategory {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub sort_order: i32,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub use_count: i32,
    pub created_at: OffsetDateTime,
}

// ---- Request/Response DTOs ----

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateProjectRequest {
    #[validate(length(min = 3, max = 300))]
    pub title: String,
    pub organization_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    #[validate(length(max = 500))]
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub access_type: Option<String>,
    pub price: Option<f64>,
    pub currency: Option<String>,
    pub license: Option<String>,
    pub license_details: Option<String>,
    pub thumbnail_url: Option<String>,
    pub demo_url: Option<String>,
    pub repository_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub title: Option<String>,
    pub category_id: Option<Uuid>,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub access_type: Option<String>,
    pub price: Option<f64>,
    pub currency: Option<String>,
    pub license: Option<String>,
    pub license_details: Option<String>,
    pub thumbnail_url: Option<String>,
    pub demo_url: Option<String>,
    pub repository_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectFilterParams {
    pub q: Option<String>,
    pub category: Option<String>,
    pub status: Option<String>,
    pub visibility: Option<String>,
    pub access_type: Option<String>,
    pub owner_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub tags: Option<String>, // comma-separated
    pub sort: Option<String>, // popular | newest | trending | rated
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub featured: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateVersionRequest {
    pub version_label: String,
    pub changelog: Option<String>,
    pub commit_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddCollaboratorRequest {
    pub user_id: Uuid,
    pub role: Option<String>,
}
