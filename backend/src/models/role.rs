use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub scope: String,
    pub is_system: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub module: String,
    pub action: String,
    pub scope: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserRole {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub granted_by: Option<Uuid>,
    pub granted_at: OffsetDateTime,
    pub expires_at: Option<OffsetDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct AssignRoleRequest {
    pub role_slug: String,
    pub expires_at: Option<OffsetDateTime>,
}
