use crate::utils::errors::{AppError, AppResult};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;
 
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,       // user_id as string
    pub email: String,
    pub username: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub iat: i64,
    pub exp: i64,
    pub jti: String,       // unique token ID for revocation
}
 
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub session_id: String,
    pub iat: i64,
    pub exp: i64,
}
 
pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_expiry_seconds: i64,
    refresh_expiry_days: i64,
}
 
impl JwtManager {
    pub fn new(secret: &str, access_expiry_seconds: i64, refresh_expiry_days: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            access_expiry_seconds,
            refresh_expiry_days,
        }
    }
 
    pub fn issue_access_token(
        &self,
        user_id: Uuid,
        email: &str,
        username: &str,
        roles: Vec<String>,
        permissions: Vec<String>,
    ) -> AppResult<String> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            username: username.to_string(),
            roles,
            permissions,
            iat: now,
            exp: now + self.access_expiry_seconds,
            jti: Uuid::new_v4().to_string(),
        };
 
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("JWT encode error: {}", e)))
    }
 
    pub fn issue_refresh_token(&self, user_id: Uuid, session_id: Uuid) -> AppResult<String> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let claims = RefreshClaims {
            sub: user_id.to_string(),
            session_id: session_id.to_string(),
            iat: now,
            exp: now + (self.refresh_expiry_days * 86400),
        };
 
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("JWT encode error: {}", e)))
    }
 
    pub fn verify_access_token(&self, token: &str) -> AppResult<Claims> {
        let mut validation = Validation::default();
        validation.validate_exp = true;
 
        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|d| d.claims)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    AppError::Unauthorized("Token has expired".to_string())
                }
                _ => AppError::Unauthorized(format!("Invalid token: {}", e)),
            })
    }
 
    pub fn verify_refresh_token(&self, token: &str) -> AppResult<RefreshClaims> {
        let mut validation = Validation::default();
        validation.validate_exp = true;
 
        decode::<RefreshClaims>(token, &self.decoding_key, &validation)
            .map(|d| d.claims)
            .map_err(|e| AppError::Unauthorized(format!("Invalid refresh token: {}", e)))
    }
}
 