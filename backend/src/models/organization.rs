use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub org_type: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub website_url: Option<String>,
    pub email: Option<String>,
    pub country: Option<String>,
    pub verified: bool,
    pub owner_id: Uuid,
    pub metadata: serde_json::Value,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrganizationMember {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub org_role_id: Option<Uuid>,
    pub invited_by: Option<Uuid>,
    pub joined_at: OffsetDateTime,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrganizationRole {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub permissions: serde_json::Value,
    pub is_default: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateOrganizationRequest {
    #[validate(length(min = 2, max = 200))]
    pub name: String,
    #[validate(length(min = 2, max = 100))]
    pub slug: String,
    pub org_type: Option<String>,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub email: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrganizationRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub website_url: Option<String>,
    pub email: Option<String>,
    pub country: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct OrganizationWithStats {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub org_type: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub verified: bool,
    pub created_at: OffsetDateTime,
    pub project_count: Option<i64>,
    pub member_count: Option<i64>,
}
