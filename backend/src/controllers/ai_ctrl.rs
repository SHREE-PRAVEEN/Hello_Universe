use axum::{extract::{Path, State}, Json};
use uuid::Uuid;
use crate::{
    config::AppState,
    middleware::{rbac, AuthUser},
    models::ai::*,
    services::AiService,
    utils::errors::AppResult,
};

pub async fn tag_project(
    State(state): State<AppState>,
    user: AuthUser,
    Path(project_id): Path<Uuid>,
) -> AppResult<Json<AiTagResult>> {
    rbac::require_moderator(&user)?;
    let result = AiService::auto_tag_project(&state, project_id).await?;
    Ok(Json(result))
}

pub async fn tag_media(
    State(state): State<AppState>,
    user: AuthUser,
    Path(media_id): Path<Uuid>,
) -> AppResult<Json<AiTagResult>> {
    rbac::require_moderator(&user)?;
    let result = AiService::auto_tag_media(&state, media_id).await?;
    Ok(Json(result))
}

pub async fn get_tags(
    State(state): State<AppState>,
    Path((entity_type, entity_id)): Path<(String, Uuid)>,
) -> AppResult<Json<Vec<AiTag>>> {
    let tags = AiService::get_tags(&state, &entity_type, entity_id).await?;
    Ok(Json(tags))
}
