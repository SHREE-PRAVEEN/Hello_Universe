use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AiTag {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub tag_name: String,
    pub confidence: bigdecimal::BigDecimal,
    pub model_name: Option<String>,
    pub model_version: Option<String>,
    pub raw_response: Option<serde_json::Value>,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AiEmbedding {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub model_name: String,
    pub embedding: serde_json::Value,
    pub dimensions: i32,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SearchIndex {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub title: String,
    pub body: Option<String>,
    pub tags: Option<Vec<String>>,
    pub rank_score: f32,
    pub metadata: serde_json::Value,
    pub updated_at: OffsetDateTime,
}

// ---- DTOs ----

#[derive(Debug, Serialize)]
pub struct AiTagResult {
    pub entity_id: Uuid,
    pub entity_type: String,
    pub tags: Vec<AiTagItem>,
    pub model: String,
}

#[derive(Debug, Serialize)]
pub struct AiTagItem {
    pub tag: String,
    pub confidence: f64,
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub q: String,
    pub entity_types: Option<Vec<String>>,
    pub filters: Option<serde_json::Value>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SearchResult {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub title: String,
    pub rank_score: f32,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total: i64,
    pub query: String,
    pub latency_ms: u64,
}
