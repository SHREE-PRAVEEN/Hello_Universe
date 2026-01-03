use actix_web::{HttpResponse, ResponseError};
use std::fmt;

/// Centralized API error types for consistent error handling
#[derive(Debug)]
pub enum ApiError {
    // Authentication errors
    Unauthorized(String),
    Forbidden(String),
    InvalidToken(String),
    TokenExpired,
    
    // Validation errors
    ValidationError(String),
    BadRequest(String),
    
    // Resource errors
    NotFound(String),
    Conflict(String),
    
    // Database errors
    DatabaseError(String),
    ConnectionError(String),
    
    // External service errors
    ExternalServiceError(String),
    PaymentError(String),
    BlockchainError(String),
    AIServiceError(String),
    
    // General errors
    InternalError(String),
    RateLimited,
    ServiceUnavailable(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            ApiError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            ApiError::TokenExpired => write!(f, "Token has expired"),
            ApiError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            ApiError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            ApiError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            ApiError::ExternalServiceError(msg) => write!(f, "External service error: {}", msg),
            ApiError::PaymentError(msg) => write!(f, "Payment error: {}", msg),
            ApiError::BlockchainError(msg) => write!(f, "Blockchain error: {}", msg),
            ApiError::AIServiceError(msg) => write!(f, "AI service error: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            ApiError::RateLimited => write!(f, "Rate limit exceeded"),
            ApiError::ServiceUnavailable(msg) => write!(f, "Service unavailable: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_type) = match self {
            ApiError::Unauthorized(_) => (actix_web::http::StatusCode::UNAUTHORIZED, "unauthorized"),
            ApiError::Forbidden(_) => (actix_web::http::StatusCode::FORBIDDEN, "forbidden"),
            ApiError::InvalidToken(_) => (actix_web::http::StatusCode::UNAUTHORIZED, "invalid_token"),
            ApiError::TokenExpired => (actix_web::http::StatusCode::UNAUTHORIZED, "token_expired"),
            ApiError::ValidationError(_) => (actix_web::http::StatusCode::BAD_REQUEST, "validation_error"),
            ApiError::BadRequest(_) => (actix_web::http::StatusCode::BAD_REQUEST, "bad_request"),
            ApiError::NotFound(_) => (actix_web::http::StatusCode::NOT_FOUND, "not_found"),
            ApiError::Conflict(_) => (actix_web::http::StatusCode::CONFLICT, "conflict"),
            ApiError::DatabaseError(_) => (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, "database_error"),
            ApiError::ConnectionError(_) => (actix_web::http::StatusCode::SERVICE_UNAVAILABLE, "connection_error"),
            ApiError::ExternalServiceError(_) => (actix_web::http::StatusCode::BAD_GATEWAY, "external_service_error"),
            ApiError::PaymentError(_) => (actix_web::http::StatusCode::PAYMENT_REQUIRED, "payment_error"),
            ApiError::BlockchainError(_) => (actix_web::http::StatusCode::BAD_GATEWAY, "blockchain_error"),
            ApiError::AIServiceError(_) => (actix_web::http::StatusCode::BAD_GATEWAY, "ai_service_error"),
            ApiError::InternalError(_) => (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, "internal_error"),
            ApiError::RateLimited => (actix_web::http::StatusCode::TOO_MANY_REQUESTS, "rate_limited"),
            ApiError::ServiceUnavailable(_) => (actix_web::http::StatusCode::SERVICE_UNAVAILABLE, "service_unavailable"),
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": {
                "type": error_type,
                "message": self.to_string()
            },
            "success": false
        }))
    }
}

// Conversions from common error types
impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        log::error!("Database error: {:?}", err);
        match err {
            sqlx::Error::RowNotFound => ApiError::NotFound("Resource not found".to_string()),
            sqlx::Error::Database(db_err) => {
                if let Some(code) = db_err.code() {
                    if code == "23505" {
                        return ApiError::Conflict("Resource already exists".to_string());
                    }
                }
                ApiError::DatabaseError(db_err.to_string())
            }
            _ => ApiError::DatabaseError(err.to_string()),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        log::warn!("JWT error: {:?}", err);
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => ApiError::TokenExpired,
            _ => ApiError::InvalidToken(err.to_string()),
        }
    }
}

impl From<bcrypt::BcryptError> for ApiError {
    fn from(err: bcrypt::BcryptError) -> Self {
        log::error!("Bcrypt error: {:?}", err);
        ApiError::InternalError("Password processing failed".to_string())
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(err: validator::ValidationErrors) -> Self {
        ApiError::ValidationError(err.to_string())
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        log::error!("HTTP client error: {:?}", err);
        ApiError::ExternalServiceError(err.to_string())
    }
}

/// Standardized API response wrapper
#[derive(serde::Serialize)]
pub struct ApiResponse<T: serde::Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T: serde::Serialize> ApiResponse<T> {
    pub fn success(data: T) -> HttpResponse {
        HttpResponse::Ok().json(Self {
            success: true,
            data: Some(data),
            message: None,
        })
    }
    
    pub fn success_with_message(data: T, message: &str) -> HttpResponse {
        HttpResponse::Ok().json(Self {
            success: true,
            data: Some(data),
            message: Some(message.to_string()),
        })
    }
    
    pub fn created(data: T) -> HttpResponse {
        HttpResponse::Created().json(Self {
            success: true,
            data: Some(data),
            message: Some("Resource created successfully".to_string()),
        })
    }
}

/// Empty response for operations without data
pub fn success_message(message: &str) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": message
    }))
}

/// Result type alias for API handlers
pub type ApiResult<T> = Result<T, ApiError>;
