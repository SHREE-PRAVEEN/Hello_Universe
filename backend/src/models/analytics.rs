use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProjectView {
    pub id: Uuid,
    pub project_id: Uuid,
    pub user_id: Option<Uuid>,
    pub session_id: Option<String>,
    pub ip_address: Option<String>,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
    pub country: Option<String>,
    pub viewed_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MediaView {
    pub id: Uuid,
    pub media_file_id: Uuid,
    pub user_id: Option<Uuid>,
    pub session_id: Option<String>,
    pub watch_seconds: Option<i32>,
    pub completion_pct: Option<bigdecimal::BigDecimal>,
    pub ip_address: Option<String>,
    pub country: Option<String>,
    pub viewed_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Download {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub user_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub country: Option<String>,
    pub version_label: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub downloaded_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SearchQuery {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub session_id: Option<Uuid>,
    pub query_text: String,
    pub filters: serde_json::Value,
    pub result_count: Option<i32>,
    pub clicked_id: Option<Uuid>,
    pub clicked_type: Option<String>,
    pub search_latency_ms: Option<i32>,
    pub created_at: OffsetDateTime,
}

// ---- Aggregate / Report Models ----

#[derive(Debug, Serialize, FromRow)]
pub struct TopProject {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub author: String,
    pub organization: Option<String>,
    pub period_downloads: Option<i64>,
    pub total_downloads: i64,
    pub total_revenue: Option<bigdecimal::BigDecimal>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ProjectDailyStats {
    pub project_id: Uuid,
    pub day: OffsetDateTime,
    pub views: Option<i64>,
    pub unique_visitors: Option<i64>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct EngagementSummary {
    pub entity_id: Uuid,
    pub total_views: Option<i64>,
    pub unique_views: Option<i64>,
    pub total_downloads: Option<i64>,
    pub total_likes: Option<i64>,
    pub avg_watch_pct: Option<f64>,
}

// ---- DTOs ----

#[derive(Debug, Deserialize)]
pub struct AnalyticsRangeParams {
    pub from: Option<String>, // ISO date string
    pub to: Option<String>,
    pub granularity: Option<String>, // day | week | month
}

#[derive(Debug, Deserialize)]
pub struct TrackViewRequest {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub session_id: Option<String>,
    pub referrer: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TrackDownloadRequest {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub version_label: Option<String>,
}
