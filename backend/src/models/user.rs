use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub email_verified_at: Option<OffsetDateTime>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub website_url: Option<String>,
    pub location: Option<String>,
    pub status: String,
    pub is_superadmin: bool,
    pub metadata: serde_json::Value,
    pub last_login_at: Option<OffsetDateTime>,
    pub last_login_ip: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

/// Public user profile (safe to return in API responses)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PublicUser {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub website_url: Option<String>,
    pub location: Option<String>,
    pub created_at: OffsetDateTime,
}

/// Full user with roles (used for auth)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuthUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub status: String,
    pub is_superadmin: bool,
    pub roles: Option<serde_json::Value>,
    pub permissions: Option<serde_json::Value>,
}

/// Create user request
#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 80))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
    pub display_name: Option<String>,
}

/// Update user request
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub website_url: Option<String>,
    pub location: Option<String>,
    pub avatar_url: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserWithStats {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub created_at: OffsetDateTime,
    pub project_count: Option<i64>,
    pub follower_count: Option<i64>,
    pub following_count: Option<i64>,
}
