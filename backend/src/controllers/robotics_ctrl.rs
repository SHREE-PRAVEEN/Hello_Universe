use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::errors::{ApiError, ApiResponse};
use crate::middleware::AuthenticatedUser;
use crate::models::device::{Device, RegisterDeviceRequest, DeviceCommand};
use crate::services::robotics_services::{RoboticsService, CommandResult};

/// Device query parameters
#[derive(Debug, Deserialize)]
pub struct DeviceQuery {
    pub status: Option<String>,
    pub device_type: Option<String>,
}

/// Register a new device
pub async fn register_device(
    pool: web::Data<Arc<PgPool>>,
    user: AuthenticatedUser,
    body: web::Json<RegisterDeviceRequest>,
) -> Result<HttpResponse, ApiError> {
    // Validate device type
    let valid_types = ["drone", "robot", "rover"];
    if !valid_types.contains(&body.device_type.as_str()) {
        return Err(ApiError::ValidationError(format!(
            "Invalid device type. Must be one of: {:?}",
            valid_types
        )));
    }

    // Check if user already has max devices (limit to 10)
    let device_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM devices WHERE user_id = $1"
    )
    .bind(user.user_id)
    .fetch_one(pool.get_ref().as_ref())
    .await?;

    if device_count >= 10 {
        return Err(ApiError::BadRequest("Maximum device limit reached (10)".to_string()));
    }

    // Create device
    let device = sqlx::query_as::<_, Device>(
        r#"
        INSERT INTO devices (user_id, device_name, device_type, firmware_version, status, metadata)
        VALUES ($1, $2, $3, $4, 'offline', '{}')
        RETURNING *
        "#
    )
    .bind(user.user_id)
    .bind(&body.device_name)
    .bind(&body.device_type)
    .bind(&body.firmware_version)
    .fetch_one(pool.get_ref().as_ref())
    .await?;

    log::info!("Device registered: {} for user {}", device.id, user.user_id);

    Ok(ApiResponse::created(device))
}

/// Get user's devices
pub async fn get_devices(
    pool: web::Data<Arc<PgPool>>,
    user: AuthenticatedUser,
    query: web::Query<DeviceQuery>,
) -> Result<HttpResponse, ApiError> {
    let devices = match (&query.status, &query.device_type) {
        (Some(status), Some(device_type)) => {
            sqlx::query_as::<_, Device>(
                "SELECT * FROM devices WHERE user_id = $1 AND status = $2 AND device_type = $3 ORDER BY created_at DESC"
            )
            .bind(user.user_id)
            .bind(status)
            .bind(device_type)
            .fetch_all(pool.get_ref().as_ref())
            .await?
        }
        (Some(status), None) => {
            sqlx::query_as::<_, Device>(
                "SELECT * FROM devices WHERE user_id = $1 AND status = $2 ORDER BY created_at DESC"
            )
            .bind(user.user_id)
            .bind(status)
            .fetch_all(pool.get_ref().as_ref())
            .await?
        }
        (None, Some(device_type)) => {
            sqlx::query_as::<_, Device>(
                "SELECT * FROM devices WHERE user_id = $1 AND device_type = $2 ORDER BY created_at DESC"
            )
            .bind(user.user_id)
            .bind(device_type)
            .fetch_all(pool.get_ref().as_ref())
            .await?
        }
        (None, None) => {
            sqlx::query_as::<_, Device>(
                "SELECT * FROM devices WHERE user_id = $1 ORDER BY created_at DESC"
            )
            .bind(user.user_id)
            .fetch_all(pool.get_ref().as_ref())
            .await?
        }
    };

    Ok(ApiResponse::success(serde_json::json!({
        "devices": devices,
        "count": devices.len()
    })))
}

/// Get single device by ID
pub async fn get_device(
    pool: web::Data<Arc<PgPool>>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let device_id = path.into_inner();

    let device = sqlx::query_as::<_, Device>(
        "SELECT * FROM devices WHERE id = $1 AND user_id = $2"
    )
    .bind(device_id)
    .bind(user.user_id)
    .fetch_optional(pool.get_ref().as_ref())
    .await?;

    match device {
        Some(d) => Ok(ApiResponse::success(d)),
        None => Err(ApiError::NotFound("Device not found".to_string())),
    }
}

/// Send command to device
pub async fn send_command(
    pool: web::Data<Arc<PgPool>>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    body: web::Json<DeviceCommand>,
) -> Result<HttpResponse, ApiError> {
    let device_id = path.into_inner();
    let robotics_service = RoboticsService::new();

    // Get device
    let device = sqlx::query_as::<_, Device>(
        "SELECT * FROM devices WHERE id = $1 AND user_id = $2"
    )
    .bind(device_id)
    .bind(user.user_id)
    .fetch_optional(pool.get_ref().as_ref())
    .await?;

    let device = device.ok_or_else(|| ApiError::NotFound("Device not found".to_string()))?;

    // Check device is online
    if device.status != "online" {
        return Err(ApiError::BadRequest(format!(
            "Device is not online. Current status: {}",
            device.status
        )));
    }

    // Validate command for device type
    robotics_service.validate_command(&device.device_type, &body.command)?;

    // Parse command parameters
    let params = robotics_service.parse_command_params(&body.command, &body.parameters)?;
    let battery_drain = robotics_service.estimate_battery_drain(&body.command, &params);

    // In production, send command to actual device via MQTT/WebSocket
    let command_id = Uuid::new_v4();
    
    log::info!(
        "Command {} sent to device {}: {} (estimated battery drain: {:.2}%)",
        command_id, device_id, body.command, battery_drain
    );

    Ok(ApiResponse::success(CommandResult {
        command_id,
        status: "sent".to_string(),
        executed_at: Utc::now(),
        estimated_duration_ms: 1000,
        estimated_battery_drain: battery_drain,
    }))
}

/// Update device status
pub async fn update_status(
    pool: web::Data<Arc<PgPool>>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    body: web::Json<StatusUpdate>,
) -> Result<HttpResponse, ApiError> {
    let device_id = path.into_inner();

    // Validate status
    let valid_statuses = ["online", "offline", "maintenance"];
    if !valid_statuses.contains(&body.status.as_str()) {
        return Err(ApiError::ValidationError(format!(
            "Invalid status. Must be one of: {:?}",
            valid_statuses
        )));
    }

    let result = sqlx::query(
        "UPDATE devices SET status = $1, last_seen = NOW() WHERE id = $2 AND user_id = $3"
    )
    .bind(&body.status)
    .bind(device_id)
    .bind(user.user_id)
    .execute(pool.get_ref().as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Device not found".to_string()));
    }

    Ok(ApiResponse::success(serde_json::json!({
        "device_id": device_id,
        "status": body.status,
        "updated_at": Utc::now()
    })))
}

/// Get device telemetry
pub async fn get_telemetry(
    pool: web::Data<Arc<PgPool>>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let device_id = path.into_inner();
    let robotics_service = RoboticsService::new();

    // Verify device belongs to user
    let device = sqlx::query_as::<_, Device>(
        "SELECT * FROM devices WHERE id = $1 AND user_id = $2"
    )
    .bind(device_id)
    .bind(user.user_id)
    .fetch_optional(pool.get_ref().as_ref())
    .await?;

    let device = device.ok_or_else(|| ApiError::NotFound("Device not found".to_string()))?;

    // Generate simulated telemetry (in production, fetch from device)
    let telemetry = robotics_service.generate_telemetry(&device.device_type);

    Ok(ApiResponse::success(serde_json::json!({
        "device_id": device_id,
        "device_name": device.device_name,
        "telemetry": telemetry
    })))
}

/// Delete device
pub async fn delete_device(
    pool: web::Data<Arc<PgPool>>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let device_id = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM devices WHERE id = $1 AND user_id = $2"
    )
    .bind(device_id)
    .bind(user.user_id)
    .execute(pool.get_ref().as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Device not found".to_string()));
    }

    log::info!("Device {} deleted by user {}", device_id, user.user_id);

    Ok(crate::errors::success_message("Device deleted successfully"))
}

/// Robotics service health check
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "service": "robotics",
        "status": "available",
        "supported_devices": ["drone", "robot", "rover"],
        "features": {
            "device_registration": true,
            "command_execution": true,
            "telemetry": true,
            "real_time_control": false
        }
    }))
}

// Request types
#[derive(Debug, Deserialize)]
pub struct StatusUpdate {
    pub status: String,
}
