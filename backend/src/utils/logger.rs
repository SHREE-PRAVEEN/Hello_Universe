/// Structured logging helpers
use tracing::{error, info, warn};
use uuid::Uuid;
 
pub fn log_request(method: &str, path: &str, status: u16, latency_ms: u64, user_id: Option<Uuid>) {
    info!(
        method = method,
        path = path,
        status = status,
        latency_ms = latency_ms,
        user_id = user_id.map(|id| id.to_string()).as_deref(),
        "HTTP request"
    );
}
 
pub fn log_error(context: &str, err: &dyn std::error::Error, user_id: Option<Uuid>) {
    error!(
        context = context,
        error = %err,
        user_id = user_id.map(|id| id.to_string()).as_deref(),
        "Application error"
    );
}
 
pub fn log_security_event(event: &str, user_id: Option<Uuid>, ip: Option<&str>) {
    warn!(
        event = event,
        user_id = user_id.map(|id| id.to_string()).as_deref(),
        ip = ip,
        "Security event"
    );
}
 