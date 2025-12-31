//! Database configuration and connection pooling utilities

use sqlx::{postgres::PgPoolOptions, PgPool, Error};
use std::time::Duration;

/// Database connection configuration
#[derive(Debug, Clone)]
pub struct DbConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            url: std::env::var("DATABASE_URL").unwrap_or_default(),
            max_connections: 10,
            min_connections: 2,
            connect_timeout: Duration::from_secs(10),
            idle_timeout: Duration::from_secs(600),
            max_lifetime: Duration::from_secs(1800),
        }
    }
}

impl DbConfig {
    /// Create config from environment variables
    pub fn from_env() -> Self {
        Self {
            url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            max_connections: std::env::var("DB_MAX_CONNECTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            min_connections: std::env::var("DB_MIN_CONNECTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(2),
            connect_timeout: Duration::from_secs(
                std::env::var("DB_CONNECT_TIMEOUT")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(10),
            ),
            idle_timeout: Duration::from_secs(
                std::env::var("DB_IDLE_TIMEOUT")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(600),
            ),
            max_lifetime: Duration::from_secs(
                std::env::var("DB_MAX_LIFETIME")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(1800),
            ),
        }
    }

    /// Create a connection pool with these settings
    pub async fn create_pool(&self) -> Result<PgPool, Error> {
        PgPoolOptions::new()
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .acquire_timeout(self.connect_timeout)
            .idle_timeout(self.idle_timeout)
            .max_lifetime(self.max_lifetime)
            .connect(&self.url)
            .await
    }
}

/// Create a database connection pool with default settings
pub async fn create_pool(database_url: &str) -> Result<PgPool, Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(database_url)
        .await
}

/// Check database health
pub async fn health_check(pool: &PgPool) -> Result<(), Error> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await?;
    Ok(())
}

/// Get database statistics
pub async fn get_stats(pool: &PgPool) -> DbStats {
    DbStats {
        size: pool.size(),
        idle: pool.num_idle(),
        is_closed: pool.is_closed(),
    }
}

/// Database statistics
#[derive(Debug, serde::Serialize)]
pub struct DbStats {
    pub size: u32,
    pub idle: usize,
    pub is_closed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = DbConfig::default();
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.min_connections, 2);
    }
}