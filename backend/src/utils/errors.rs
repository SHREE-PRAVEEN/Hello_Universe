use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
 
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),
 
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
 
    #[error("Forbidden: {0}")]
    Forbidden(String),
 
    #[error("Bad request: {0}")]
    BadRequest(String),
 
    #[error("Conflict: {0}")]
    Conflict(String),
 
    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(String),
 
    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
 
    #[error("Database error")]
    Database(#[from] sqlx::Error),
 
    #[error("Validation error: {0}")]
    Validation(String),
 
    #[error("Payment error: {0}")]
    Payment(String),
 
    #[error("Rate limit exceeded")]
    RateLimited,
 
    #[error("File too large")]
    FileTooLarge,
 
    #[error("Unsupported media type: {0}")]
    UnsupportedMediaType(String),
}
 
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, "FORBIDDEN", msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg.clone()),
            AppError::UnprocessableEntity(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "UNPROCESSABLE_ENTITY",
                msg.clone(),
            ),
            AppError::Validation(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "VALIDATION_ERROR",
                msg.clone(),
            ),
            AppError::RateLimited => (
                StatusCode::TOO_MANY_REQUESTS,
                "RATE_LIMITED",
                "Too many requests. Please try again later.".to_string(),
            ),
            AppError::FileTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "FILE_TOO_LARGE",
                "Uploaded file exceeds the maximum allowed size.".to_string(),
            ),
            AppError::UnsupportedMediaType(msg) => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "UNSUPPORTED_MEDIA_TYPE",
                msg.clone(),
            ),
            AppError::Payment(msg) => (
                StatusCode::PAYMENT_REQUIRED,
                "PAYMENT_ERROR",
                msg.clone(),
            ),
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                // Check for unique constraint violation
                if let sqlx::Error::Database(db_err) = e {
                    if db_err.code().map(|c| c == "23505").unwrap_or(false) {
                        return (
                            StatusCode::CONFLICT,
                            Json(json!({
                                "error": { "code": "DUPLICATE_ENTRY", "message": "A record with these details already exists." }
                            })),
                        )
                            .into_response();
                    }
                }
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    "A database error occurred.".to_string(),
                )
            }
            AppError::Internal(e) => {
                tracing::error!("Internal error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "An internal error occurred.".to_string(),
                )
            }
        };
 
        (
            status,
            Json(json!({
                "error": {
                    "code": code,
                    "message": message
                }
            })),
        )
            .into_response()
    }
}
 
pub type AppResult<T> = Result<T, AppError>;
 
// Helpers
impl AppError {
    pub fn not_found(resource: &str) -> Self {
        Self::NotFound(format!("{} not found", resource))
    }
 
    pub fn unauthorized() -> Self {
        Self::Unauthorized("Authentication required".to_string())
    }
 
    pub fn forbidden() -> Self {
        Self::Forbidden("You do not have permission to perform this action".to_string())
    }
}
 