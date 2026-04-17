use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::utils::errors::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn issue_token(subject: &str, secret: &str, ttl_minutes: i64) -> AppResult<String> {
    let exp = (Utc::now() + Duration::minutes(ttl_minutes)).timestamp() as usize;
    let claims = Claims {
        sub: subject.to_string(),
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("failed to issue token: {e}")))
}

pub fn verify_token(token: &str, secret: &str) -> AppResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized)
}
