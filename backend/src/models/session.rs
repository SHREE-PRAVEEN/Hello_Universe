use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token: String, // stored as SHA-256 hash
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub device_id: Option<String>,
    pub is_revoked: bool,
    pub last_used_at: OffsetDateTime,
    pub expires_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

impl TokenPair {
    pub fn new(access_token: String, refresh_token: String, expires_in: i64) -> Self {
        Self {
            access_token,
            refresh_token,
            expires_in,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AuthToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub token_type: String,
    pub expires_at: OffsetDateTime,
    pub used_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}
