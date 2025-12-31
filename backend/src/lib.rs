//! RoboVeda Backend Library
//! 
//! This library provides the core functionality for the RoboVeda platform,
//! including authentication, AI services, robotics management, and blockchain integration.

pub mod config;
pub mod controllers;
pub mod errors;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;

// Re-export commonly used types
pub use config::AppConfig;
pub use errors::{ApiError, ApiResponse, ApiResult};
pub use middleware::{AuthenticatedUser, OptionalUser, AdminUser};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name  
pub const NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_exists() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_name_exists() {
        assert_eq!(NAME, "backend");
    }
}