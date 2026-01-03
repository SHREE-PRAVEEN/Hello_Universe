//! Logging utilities for the RoboVeda backend
//!
//! Provides structured logging with context and helper functions.

use tracing::{info, warn, error, debug, instrument};
use std::time::Instant;

/// Log an API request with timing information
pub struct RequestTimer {
    method: String,
    path: String,
    start: Instant,
}

impl RequestTimer {
    pub fn new(method: &str, path: &str) -> Self {
        debug!(method = %method, path = %path, "Request started");
        Self {
            method: method.to_string(),
            path: path.to_string(),
            start: Instant::now(),
        }
    }

    pub fn finish(self, status: u16) {
        let duration = self.start.elapsed();
        info!(
            method = %self.method,
            path = %self.path,
            status = status,
            duration_ms = ?duration.as_millis(),
            "Request completed"
        );
    }

    pub fn finish_with_error(self, error: &str) {
        let duration = self.start.elapsed();
        error!(
            method = %self.method,
            path = %self.path,
            error = %error,
            duration_ms = ?duration.as_millis(),
            "Request failed"
        );
    }
}

/// Log user authentication events
pub fn log_auth_event(event: &str, user_id: Option<&str>, success: bool, details: Option<&str>) {
    if success {
        info!(
            event = %event,
            user_id = ?user_id,
            success = success,
            details = ?details,
            "Auth event"
        );
    } else {
        warn!(
            event = %event,
            user_id = ?user_id,
            success = success,
            details = ?details,
            "Auth event failed"
        );
    }
}

/// Log database operations
pub fn log_db_operation(operation: &str, table: &str, rows_affected: Option<u64>, duration_ms: u64) {
    debug!(
        operation = %operation,
        table = %table,
        rows_affected = ?rows_affected,
        duration_ms = duration_ms,
        "Database operation"
    );
}

/// Log external API calls
pub fn log_external_api(service: &str, endpoint: &str, status: u16, duration_ms: u64) {
    if status >= 400 {
        warn!(
            service = %service,
            endpoint = %endpoint,
            status = status,
            duration_ms = duration_ms,
            "External API error"
        );
    } else {
        debug!(
            service = %service,
            endpoint = %endpoint,
            status = status,
            duration_ms = duration_ms,
            "External API call"
        );
    }
}

/// Log security events (rate limiting, blocked requests, etc.)
pub fn log_security_event(event_type: &str, ip: Option<&str>, details: &str) {
    warn!(
        event_type = %event_type,
        ip = ?ip,
        details = %details,
        "Security event"
    );
}

/// Log device/robotics events
pub fn log_device_event(device_id: &str, event: &str, details: Option<&str>) {
    info!(
        device_id = %device_id,
        event = %event,
        details = ?details,
        "Device event"
    );
}

/// Log blockchain/payment events
pub fn log_blockchain_event(event: &str, tx_hash: Option<&str>, amount: Option<f64>, status: &str) {
    info!(
        event = %event,
        tx_hash = ?tx_hash,
        amount = ?amount,
        status = %status,
        "Blockchain event"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_timer() {
        let timer = RequestTimer::new("GET", "/api/health");
        std::thread::sleep(std::time::Duration::from_millis(10));
        timer.finish(200);
    }

    #[test]
    fn test_log_auth_event() {
        log_auth_event("login", Some("user-123"), true, Some("password"));
        log_auth_event("login", Some("user-456"), false, Some("invalid password"));
    }
}