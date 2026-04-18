use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Review {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub user_id: Uuid,
    pub rating: i16,
    pub title: Option<String>,
    pub body: Option<String>,
    pub is_verified: bool,
    pub helpful_count: i32,
    pub status: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub user_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub body: String,
    pub like_count: i32,
    pub status: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CommentWithAuthor {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub parent_id: Option<Uuid>,
    pub body: String,
    pub like_count: i32,
    pub status: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Favorite {
    pub id: Uuid,
    pub user_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Follow {
    pub id: Uuid,
    pub follower_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Report {
    pub id: Uuid,
    pub reporter_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub reason: String,
    pub description: Option<String>,
    pub status: String,
    pub resolved_by: Option<Uuid>,
    pub resolved_at: Option<OffsetDateTime>,
    pub resolution_note: Option<String>,
    pub created_at: OffsetDateTime,
}

// ---- DTOs ----

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateReviewRequest {
    pub entity_type: String,
    pub entity_id: Uuid,
    #[validate(range(min = 1, max = 5))]
    pub rating: i16,
    pub title: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateCommentRequest {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub parent_id: Option<Uuid>,
    #[validate(length(min = 1, max = 5000))]
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateReportRequest {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub reason: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RatingSummary {
    pub entity_id: Uuid,
    pub avg_rating: f64,
    pub total_reviews: i64,
    pub five_star: i64,
    pub four_star: i64,
    pub three_star: i64,
    pub two_star: i64,
    pub one_star: i64,
}
