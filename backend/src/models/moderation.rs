use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ModerationQueue {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub queue_reason: String,
    pub priority: i16,
    pub assigned_to: Option<Uuid>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ModerationAction {
    pub id: Uuid,
    pub queue_id: Option<Uuid>,
    pub moderator_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub action: String,
    pub reason: Option<String>,
    pub rejection_reason: Option<String>,
    pub previous_status: Option<String>,
    pub new_status: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditLog {
    pub id: Uuid,
    pub actor_id: Option<Uuid>,
    pub actor_role: Option<String>,
    pub organization_id: Option<Uuid>,
    pub action: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub old_state: Option<serde_json::Value>,
    pub new_state: Option<serde_json::Value>,
    pub diff: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub request_id: Option<String>,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: String,
    pub title: String,
    pub body: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub action_url: Option<String>,
    pub is_read: bool,
    pub read_at: Option<OffsetDateTime>,
    pub sent_via: Option<Vec<String>>,
    pub created_at: OffsetDateTime,
}

// ---- DTOs ----

#[derive(Debug, Deserialize)]
pub struct ModerationDecisionRequest {
    pub action: String, // approve | reject | flag | escalate | ban
    pub reason: Option<String>,
    pub rejection_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAuditLogRequest {
    pub action: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub old_state: Option<serde_json::Value>,
    pub new_state: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ModerationQueueItem {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub queue_reason: String,
    pub priority: i16,
    pub status: String,
    pub assigned_to: Option<Uuid>,
    pub assignee_username: Option<String>,
    pub created_at: OffsetDateTime,
}
