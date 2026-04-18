use axum::{extract::{Path, Query, State}, Json};
use uuid::Uuid;
use crate::{
    config::AppState,
    middleware::{AuthUser, OptionalAuthUser},
    models::engagement::*,
    repositories::EngagementRepo,
    utils::errors::AppResult,
};

// ---- Reviews ----

pub async fn create_review(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<CreateReviewRequest>,
) -> AppResult<Json<Review>> {
    let review = EngagementRepo::create_review(&state.db, user.id(), &req).await?;
    Ok(Json(review))
}

pub async fn get_reviews(
    State(state): State<AppState>,
    Path((entity_type, entity_id)): Path<(String, Uuid)>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> AppResult<Json<serde_json::Value>> {
    let page = params.get("page").and_then(|v| v.parse().ok()).unwrap_or(1i64);
    let per_page = params.get("per_page").and_then(|v| v.parse().ok()).unwrap_or(20i64);
    let reviews = EngagementRepo::get_reviews(&state.db, &entity_type, entity_id, per_page, (page-1)*per_page).await?;
    let summary = EngagementRepo::rating_summary(&state.db, entity_id).await?;
    Ok(Json(serde_json::json!({ "reviews": reviews, "summary": summary })))
}

// ---- Comments ----

pub async fn create_comment(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<CreateCommentRequest>,
) -> AppResult<Json<Comment>> {
    let comment = EngagementRepo::create_comment(&state.db, user.id(), &req).await?;
    Ok(Json(comment))
}

pub async fn get_comments(
    State(state): State<AppState>,
    Path((entity_type, entity_id)): Path<(String, Uuid)>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> AppResult<Json<Vec<CommentWithAuthor>>> {
    let page = params.get("page").and_then(|v| v.parse().ok()).unwrap_or(1i64);
    let per_page = params.get("per_page").and_then(|v| v.parse().ok()).unwrap_or(20i64);
    let comments = EngagementRepo::get_comments(&state.db, &entity_type, entity_id, per_page, (page-1)*per_page).await?;
    Ok(Json(comments))
}

pub async fn delete_comment(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    EngagementRepo::delete_comment(&state.db, id, user.id()).await?;
    Ok(Json(serde_json::json!({ "message": "Comment deleted" })))
}

// ---- Favorites ----

pub async fn toggle_favorite(
    State(state): State<AppState>,
    user: AuthUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<Json<serde_json::Value>> {
    let entity_type = body["entity_type"].as_str().unwrap_or("project");
    let entity_id: Uuid = serde_json::from_value(body["entity_id"].clone())
        .map_err(|_| crate::utils::errors::AppError::BadRequest("entity_id required".into()))?;
    let added = EngagementRepo::toggle_favorite(&state.db, user.id(), entity_type, entity_id).await?;
    Ok(Json(serde_json::json!({ "favorited": added })))
}

pub async fn is_favorited(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> AppResult<Json<serde_json::Value>> {
    let entity_type = params.get("entity_type").map(|s| s.as_str()).unwrap_or("project");
    let entity_id: Uuid = params.get("entity_id")
        .and_then(|v| Uuid::parse_str(v).ok())
        .ok_or_else(|| crate::utils::errors::AppError::BadRequest("entity_id required".into()))?;
    let fav = EngagementRepo::is_favorited(&state.db, user.id(), entity_type, entity_id).await?;
    Ok(Json(serde_json::json!({ "favorited": fav })))
}

// ---- Follows ----

pub async fn toggle_follow(
    State(state): State<AppState>,
    user: AuthUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<Json<serde_json::Value>> {
    let entity_type = body["entity_type"].as_str().unwrap_or("user");
    let entity_id: Uuid = serde_json::from_value(body["entity_id"].clone())
        .map_err(|_| crate::utils::errors::AppError::BadRequest("entity_id required".into()))?;
    let following = EngagementRepo::toggle_follow(&state.db, user.id(), entity_type, entity_id).await?;
    Ok(Json(serde_json::json!({ "following": following })))
}

// ---- Reports ----

pub async fn create_report(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<CreateReportRequest>,
) -> AppResult<Json<Report>> {
    let report = EngagementRepo::create_report(&state.db, user.id(), &req).await?;
    Ok(Json(report))
}
