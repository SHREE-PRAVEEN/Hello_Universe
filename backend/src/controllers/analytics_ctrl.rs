use axum::{extract::{Path, Query, State}, Json};
use uuid::Uuid;
use crate::{
    config::AppState,
    middleware::{rbac, AuthUser, OptionalAuthUser},
    models::analytics::*,
    services::AnalyticsService,
    utils::errors::AppResult,
};

pub async fn track_view(
    State(state): State<AppState>,
    user: OptionalAuthUser,
    Json(req): Json<TrackViewRequest>,
) -> AppResult<Json<serde_json::Value>> {
    match req.entity_type.as_str() {
        "project" => {
            AnalyticsService::track_project_view(
                &state,
                req.entity_id,
                user.user_id(),
                req.session_id.as_deref(),
                None,
                req.referrer.as_deref(),
                None,
            ).await?;
        }
        "media_file" => {
            AnalyticsService::track_media_view(
                &state,
                req.entity_id,
                user.user_id(),
                req.session_id.as_deref(),
                None, None, None,
            ).await?;
        }
        _ => {}
    }
    Ok(Json(serde_json::json!({ "tracked": true })))
}

pub async fn track_download(
    State(state): State<AppState>,
    user: OptionalAuthUser,
    Json(req): Json<TrackDownloadRequest>,
) -> AppResult<Json<serde_json::Value>> {
    AnalyticsService::track_download(&state, &req, user.user_id(), None, None).await?;
    Ok(Json(serde_json::json!({ "tracked": true })))
}

pub async fn project_stats(
    State(state): State<AppState>,
    user: AuthUser,
    Path(project_id): Path<Uuid>,
    Query(params): Query<AnalyticsRangeParams>,
) -> AppResult<Json<Vec<ProjectDailyStats>>> {
    let from = params.from.as_deref().unwrap_or("2024-01-01");
    let to = params.to.as_deref().unwrap_or("2099-12-31");
    let stats = AnalyticsService::project_view_stats(&state, project_id, from, to).await?;
    Ok(Json(stats))
}

pub async fn engagement(
    State(state): State<AppState>,
    Path(entity_id): Path<Uuid>,
) -> AppResult<Json<EngagementSummary>> {
    let summary = AnalyticsService::engagement_summary(&state, entity_id).await?;
    Ok(Json(summary))
}

pub async fn top_projects(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<AnalyticsRangeParams>,
) -> AppResult<Json<Vec<TopProject>>> {
    rbac::require_admin(&user)?;
    let from = params.from.as_deref().unwrap_or("2024-01-01");
    let to = params.to.as_deref().unwrap_or("2099-12-31");
    let top = AnalyticsService::top_projects(&state, from, to).await?;
    Ok(Json(top))
}
