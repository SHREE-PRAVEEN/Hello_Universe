use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use crate::{
    config::AppState,
    middleware::{rbac, AuthUser, OptionalAuthUser},
    models::project::*,
    services::{AnalyticsService, ProjectService},
    utils::{
        errors::{AppError, AppResult},
        pagination::{PaginatedResponse, PaginationParams},
    },
};

pub async fn list(
    State(state): State<AppState>,
    user: OptionalAuthUser,
    Query(filter): Query<ProjectFilterParams>,
) -> AppResult<Json<PaginatedResponse<ProjectWithAuthor>>> {
    let (projects, total) = ProjectService::list(&state, &filter).await?;
    let params = PaginationParams {
        page: filter.page,
        per_page: filter.per_page,
        sort: filter.sort.clone(),
        order: None,
    };
    Ok(Json(PaginatedResponse::new(projects, total, &params)))
}

pub async fn get(
    State(state): State<AppState>,
    user: OptionalAuthUser,
    Path(slug): Path<String>,
) -> AppResult<Json<ProjectWithAuthor>> {
    let project = ProjectService::get_by_slug(&state, &slug).await?;

    // Track view
    let _ = AnalyticsService::track_project_view(
        &state,
        project.id,
        user.user_id(),
        None, None, None, None,
    ).await;

    Ok(Json(project))
}

pub async fn create(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<CreateProjectRequest>,
) -> AppResult<Json<Project>> {
    rbac::require_developer(&user)?;
    let project = ProjectService::create(&state, user.id(), &req).await?;
    Ok(Json(project))
}

pub async fn update(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateProjectRequest>,
) -> AppResult<Json<Project>> {
    let project = ProjectService::update(&state, id, user.id(), &req).await?;
    Ok(Json(project))
}

pub async fn submit_for_review(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    ProjectService::submit_for_review(&state, id, user.id()).await?;
    Ok(Json(serde_json::json!({"message": "Project submitted for review"})))
}

pub async fn publish(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    rbac::require_moderator(&user)?;
    ProjectService::publish(&state, id, user.id()).await?;
    Ok(Json(serde_json::json!({"message": "Project published"})))
}

pub async fn archive(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    ProjectService::archive(&state, id, user.id()).await?;
    Ok(Json(serde_json::json!({"message": "Project archived"})))
}

pub async fn create_version(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateVersionRequest>,
) -> AppResult<Json<ProjectVersion>> {
    let version = ProjectService::create_version(&state, id, user.id(), &req).await?;
    Ok(Json(version))
}

pub async fn get_versions(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Vec<ProjectVersion>>> {
    let versions = ProjectService::get_versions(&state, id).await?;
    Ok(Json(versions))
}

pub async fn add_collaborator(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<AddCollaboratorRequest>,
) -> AppResult<Json<ProjectCollaborator>> {
    let collab = ProjectService::add_collaborator(&state, id, user.id(), &req).await?;
    Ok(Json(collab))
}

pub async fn list_categories(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<ProjectCategory>>> {
    let categories = ProjectService::list_categories(&state).await?;
    Ok(Json(categories))
}

pub async fn top_downloads(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> AppResult<Json<Vec<crate::models::analytics::TopProject>>> {
    let from = params.get("from").map(|s| s.as_str()).unwrap_or("2024-01-01");
    let to = params.get("to").map(|s| s.as_str()).unwrap_or("2099-12-31");
    let top = ProjectService::top_downloads(&state, from, to).await?;
    Ok(Json(top))
}
