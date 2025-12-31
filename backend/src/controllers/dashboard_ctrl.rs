use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::errors::{ApiError, ApiResponse};
use crate::middleware::{AuthenticatedUser, OptionalUser};

/// Dashboard overview statistics
#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub devices: DeviceStats,
    pub transactions: TransactionStats,
    pub account: AccountStats,
}

#[derive(Debug, Serialize)]
pub struct DeviceStats {
    pub total: i64,
    pub online: i64,
    pub offline: i64,
    pub maintenance: i64,
    pub by_type: Vec<DeviceTypeCount>,
}

#[derive(Debug, Serialize)]
pub struct DeviceTypeCount {
    pub device_type: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct TransactionStats {
    pub total: i64,
    pub total_amount: f64,
    pub pending: i64,
    pub completed: i64,
    pub failed: i64,
}

#[derive(Debug, Serialize)]
pub struct AccountStats {
    pub is_verified: bool,
    pub is_premium: bool,
    pub has_wallet: bool,
    pub member_since: String,
}

/// Activity timeline entry
#[derive(Debug, Serialize)]
pub struct ActivityEntry {
    pub activity_type: String,
    pub description: String,
    pub timestamp: String,
    pub metadata: serde_json::Value,
}

/// Get dashboard overview
pub async fn get_overview(
    pool: web::Data<Arc<PgPool>>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, ApiError> {
    // Get device statistics
    let device_total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM devices WHERE user_id = $1"
    )
    .bind(user.user_id)
    .fetch_one(pool.get_ref().as_ref())
    .await?;

    let device_online: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM devices WHERE user_id = $1 AND status = 'online'"
    )
    .bind(user.user_id)
    .fetch_one(pool.get_ref().as_ref())
    .await?;

    let device_maintenance: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM devices WHERE user_id = $1 AND status = 'maintenance'"
    )
    .bind(user.user_id)
    .fetch_one(pool.get_ref().as_ref())
    .await?;

    // Get devices by type
    let devices_by_type: Vec<(String, i64)> = sqlx::query_as(
        "SELECT device_type, COUNT(*) FROM devices WHERE user_id = $1 GROUP BY device_type"
    )
    .bind(user.user_id)
    .fetch_all(pool.get_ref().as_ref())
    .await?;

    // Get transaction statistics
    let tx_total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM transactions WHERE user_id = $1"
    )
    .bind(user.user_id)
    .fetch_one(pool.get_ref().as_ref())
    .await?;

    let tx_amount: Option<f64> = sqlx::query_scalar(
        "SELECT COALESCE(SUM(amount), 0) FROM transactions WHERE user_id = $1 AND status = 'completed'"
    )
    .bind(user.user_id)
    .fetch_one(pool.get_ref().as_ref())
    .await?;

    let tx_pending: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM transactions WHERE user_id = $1 AND status = 'pending'"
    )
    .bind(user.user_id)
    .fetch_one(pool.get_ref().as_ref())
    .await?;

    let tx_completed: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM transactions WHERE user_id = $1 AND status = 'completed'"
    )
    .bind(user.user_id)
    .fetch_one(pool.get_ref().as_ref())
    .await?;

    // Get user account info
    let user_info: (bool, bool, Option<String>, chrono::DateTime<chrono::Utc>) = sqlx::query_as(
        "SELECT is_verified, is_premium, wallet_address, created_at FROM users WHERE id = $1"
    )
    .bind(user.user_id)
    .fetch_one(pool.get_ref().as_ref())
    .await?;

    let stats = DashboardStats {
        devices: DeviceStats {
            total: device_total,
            online: device_online,
            offline: device_total - device_online - device_maintenance,
            maintenance: device_maintenance,
            by_type: devices_by_type
                .into_iter()
                .map(|(device_type, count)| DeviceTypeCount { device_type, count })
                .collect(),
        },
        transactions: TransactionStats {
            total: tx_total,
            total_amount: tx_amount.unwrap_or(0.0),
            pending: tx_pending,
            completed: tx_completed,
            failed: tx_total - tx_pending - tx_completed,
        },
        account: AccountStats {
            is_verified: user_info.0,
            is_premium: user_info.1,
            has_wallet: user_info.2.is_some(),
            member_since: user_info.3.format("%Y-%m-%d").to_string(),
        },
    };

    Ok(ApiResponse::success(stats))
}

/// Get recent activity
pub async fn get_activity(
    pool: web::Data<Arc<PgPool>>,
    user: AuthenticatedUser,
    query: web::Query<ActivityQuery>,
) -> Result<HttpResponse, ApiError> {
    let limit = query.limit.unwrap_or(20).min(100);

    // Combine device and transaction activities
    let mut activities: Vec<ActivityEntry> = Vec::new();

    // Get recent device activities
    let devices: Vec<(String, String, String, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
        "SELECT device_name, device_type, status, created_at FROM devices WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2"
    )
    .bind(user.user_id)
    .bind(limit / 2)
    .fetch_all(pool.get_ref().as_ref())
    .await?;

    for (name, dtype, status, created) in devices {
        activities.push(ActivityEntry {
            activity_type: "device".to_string(),
            description: format!("{} ({}) registered", name, dtype),
            timestamp: created.to_rfc3339(),
            metadata: serde_json::json!({
                "device_name": name,
                "device_type": dtype,
                "status": status
            }),
        });
    }

    // Get recent transactions
    let transactions: Vec<(f64, String, String, String, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
        "SELECT amount, currency, status, product_type, created_at FROM transactions WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2"
    )
    .bind(user.user_id)
    .bind(limit / 2)
    .fetch_all(pool.get_ref().as_ref())
    .await?;

    for (amount, currency, status, product, created) in transactions {
        activities.push(ActivityEntry {
            activity_type: "transaction".to_string(),
            description: format!("{} {} {} - {}", amount, currency, product, status),
            timestamp: created.to_rfc3339(),
            metadata: serde_json::json!({
                "amount": amount,
                "currency": currency,
                "status": status,
                "product_type": product
            }),
        });
    }

    // Sort by timestamp descending
    activities.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    activities.truncate(limit as usize);

    Ok(ApiResponse::success(serde_json::json!({
        "activities": activities,
        "count": activities.len()
    })))
}

/// Get quick stats (lightweight)
pub async fn get_quick_stats(
    pool: web::Data<Arc<PgPool>>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, ApiError> {
    let device_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM devices WHERE user_id = $1"
    )
    .bind(user.user_id)
    .fetch_one(pool.get_ref().as_ref())
    .await?;

    let online_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM devices WHERE user_id = $1 AND status = 'online'"
    )
    .bind(user.user_id)
    .fetch_one(pool.get_ref().as_ref())
    .await?;

    let tx_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM transactions WHERE user_id = $1"
    )
    .bind(user.user_id)
    .fetch_one(pool.get_ref().as_ref())
    .await?;

    Ok(ApiResponse::success(serde_json::json!({
        "devices": device_count,
        "online_devices": online_count,
        "transactions": tx_count
    })))
}

/// Get system-wide public stats (no auth required)
pub async fn get_public_stats(
    pool: Option<web::Data<Arc<PgPool>>>,
) -> HttpResponse {
    match pool {
        Some(p) => {
            let total_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
                .fetch_one(p.get_ref().as_ref())
                .await
                .unwrap_or(0);

            let total_devices: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM devices")
                .fetch_one(p.get_ref().as_ref())
                .await
                .unwrap_or(0);

            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "data": {
                    "total_users": total_users,
                    "total_devices": total_devices,
                    "platform": "RoboVeda",
                    "version": "1.0.0"
                }
            }))
        }
        None => {
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "data": {
                    "platform": "RoboVeda",
                    "version": "1.0.0",
                    "database": "unavailable"
                }
            }))
        }
    }
}

// Query parameters
#[derive(Debug, Deserialize)]
pub struct ActivityQuery {
    pub limit: Option<i64>,
}
