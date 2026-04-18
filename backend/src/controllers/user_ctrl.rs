use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use crate::{
    config::AppState,
    middleware::{rbac, AuthUser},
    models::user::*,
    services::UserService,
    utils::errors::{AppError, AppResult},
};

pub async fn get_me(
    State(state): State<AppState>,
    user: AuthUser,
) -> AppResult<Json<UserWithStats>> {
    let profile = UserService::get_profile(&state, user.id()).await?;
    Ok(Json(profile))
}

pub async fn update_me(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<UpdateUserRequest>,
) -> AppResult<Json<User>> {
    let updated = UserService::update_profile(&state, user.id(), &req).await?;
    Ok(Json(updated))
}

pub async fn get_profile(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> AppResult<Json<PublicUser>> {
    let profile = UserService::get_public(&state, &username).await?;
    Ok(Json(profile))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<UserWithStats>> {
    let profile = UserService::get_profile(&state, id).await?;
    Ok(Json(profile))
}

pub async fn search_users(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> AppResult<Json<Vec<PublicUser>>> {
    let q = params.get("q").ok_or_else(|| AppError::BadRequest("q required".into()))?;
    let page = params.get("page").and_then(|v| v.parse().ok()).unwrap_or(1);
    let per_page = params.get("per_page").and_then(|v| v.parse().ok()).unwrap_or(20);
    let users = UserService::search(&state, q, page, per_page).await?;
    Ok(Json(users))
}

pub async fn delete_account(
    State(state): State<AppState>,
    user: AuthUser,
) -> AppResult<Json<serde_json::Value>> {
    UserService::delete(&state, user.id()).await?;
    Ok(Json(serde_json::json!({ "message": "Account deleted" })))
}

// Admin: delete any user
pub async fn admin_delete_user(
    State(state): State<AppState>,
    caller: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    rbac::require_admin(&caller)?;
    UserService::delete(&state, id).await?;
    Ok(Json(serde_json::json!({ "message": "User deleted" })))
}
