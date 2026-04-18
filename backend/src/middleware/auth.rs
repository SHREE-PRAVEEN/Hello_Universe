use crate::utils::{
    errors::{AppError, AppResult},
    jwt::{Claims, JwtManager},
};
use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, HeaderMap},
};
use std::sync::Arc;
use crate::config::AppState;

/// Authenticated user extracted from JWT — required on protected routes
#[derive(Debug, Clone)]
pub struct AuthUser(pub Claims);

impl AuthUser {
    pub fn id(&self) -> uuid::Uuid {
        uuid::Uuid::parse_str(&self.0.sub).unwrap()
    }
    pub fn email(&self) -> &str { &self.0.email }
    pub fn username(&self) -> &str { &self.0.username }
    pub fn roles(&self) -> &[String] { &self.0.roles }
    pub fn permissions(&self) -> &[String] { &self.0.permissions }
    pub fn has_role(&self, role: &str) -> bool { self.0.roles.iter().any(|r| r == role) }
    pub fn has_permission(&self, perm: &str) -> bool { self.0.permissions.iter().any(|p| p == perm) }
    pub fn is_admin(&self) -> bool { self.has_role("admin") }
    pub fn is_moderator(&self) -> bool { self.has_role("moderator") || self.is_admin() }
}

#[axum::async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, AppError> {
        let token = extract_bearer_token(&parts.headers)
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".into()))?;

        let jwt = JwtManager::new(
            &state.config.jwt_secret,
            state.config.jwt_access_token_expiry_seconds,
            state.config.jwt_refresh_token_expiry_days,
        );

        let claims = jwt.verify_access_token(token)?;
        Ok(AuthUser(claims))
    }
}

/// Optional authenticated user — None on unauthenticated requests
#[derive(Debug, Clone)]
pub struct OptionalAuthUser(pub Option<Claims>);

impl OptionalAuthUser {
    pub fn user_id(&self) -> Option<uuid::Uuid> {
        self.0.as_ref().and_then(|c| uuid::Uuid::parse_str(&c.sub).ok())
    }
}

#[axum::async_trait]
impl FromRequestParts<AppState> for OptionalAuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, AppError> {
        if let Some(token) = extract_bearer_token(&parts.headers) {
            let jwt = JwtManager::new(
                &state.config.jwt_secret,
                state.config.jwt_access_token_expiry_seconds,
                state.config.jwt_refresh_token_expiry_days,
            );
            match jwt.verify_access_token(token) {
                Ok(claims) => return Ok(OptionalAuthUser(Some(claims))),
                Err(_) => {}
            }
        }
        Ok(OptionalAuthUser(None))
    }
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
}
