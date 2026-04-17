use serde::{Deserialize, Serialize};

use crate::utils::errors::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub storage_backend: String,
    pub cors_allow_origin: String,
}

impl AppConfig {
    pub fn from_env() -> AppResult<Self> {
        Ok(Self {
            host: std::env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("APP_PORT")
                .ok()
                .and_then(|v| v.parse::<u16>().ok())
                .unwrap_or(8080),
            database_url: required("DATABASE_URL")?,
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "replace_me_in_production".to_string()),
            storage_backend: std::env::var("STORAGE_BACKEND").unwrap_or_else(|_| "s3".to_string()),
            cors_allow_origin: std::env::var("CORS_ALLOW_ORIGIN")
                .unwrap_or_else(|_| "*".to_string()),
        })
    }
}

fn required(key: &str) -> AppResult<String> {
    std::env::var(key).map_err(|_| AppError::Config(format!("missing required env var {key}")))
}
