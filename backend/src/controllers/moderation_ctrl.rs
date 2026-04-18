use axum::{extract::{Path, Query, State}, Json};
use uuid::Uuid;
use crate::{
    config::AppState,
    middleware::{rbac, AuthUser},
    models::moderation::*,
    services::ModerationService,
    utils::errors::AppResult,
};

pub async fn list_queue(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> AppResult<Json<Vec<ModerationQueueItem>>> {
    rbac::require_moderator(&user)?;
    let page = params.get("page").and_then(|v| v.parse().ok()).unwrap_or(1);
    let per_page = params.get("per_page").and_then(|v| v.parse().ok()).unwrap_or(20);
    let items = ModerationService::list_queue(&state, page, per_page).await?;
    Ok(Json(items))
}

pub async fn decide(
    State(state): State<AppState>,
    user: AuthUser,
    Path(queue_id): Path<Uuid>,
    Json(req): Json<ModerationDecisionRequest>,
) -> AppResult<Json<ModerationAction>> {
    rbac::require_moderator(&user)?;
    let action = ModerationService::decide(&state, queue_id, user.id(), &req).await?;
    Ok(Json(action))
}

pub async fn my_notifications(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> AppResult<Json<Vec<Notification>>> {
    let unread_only = params.get("unread_only").map(|v| v == "true").unwrap_or(false);
    let notifications = ModerationService::get_notifications(&state, user.id(), unread_only).await?;
    Ok(Json(notifications))
}

pub async fn mark_notification_read(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    ModerationService::mark_read(&state, id, user.id()).await?;
    Ok(Json(serde_json::json!({ "message": "Marked as read" })))
}
