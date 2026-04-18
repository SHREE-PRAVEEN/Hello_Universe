use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use crate::{
    config::AppState,
    middleware::AuthUser,
    models::organization::*,
    services::OrgService,
    utils::errors::AppResult,
};

pub async fn create(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<CreateOrganizationRequest>,
) -> AppResult<Json<Organization>> {
    let org = OrgService::create(&state, user.id(), &req).await?;
    Ok(Json(org))
}

pub async fn get(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> AppResult<Json<Organization>> {
    let org = OrgService::get(&state, &slug).await?;
    Ok(Json(org))
}

pub async fn update(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateOrganizationRequest>,
) -> AppResult<Json<Organization>> {
    let org = OrgService::update(&state, id, user.id(), &req).await?;
    Ok(Json(org))
}

pub async fn add_member(
    State(state): State<AppState>,
    user: AuthUser,
    Path(org_id): Path<Uuid>,
    Json(body): Json<serde_json::Value>,
) -> AppResult<Json<OrganizationMember>> {
    let user_id: Uuid = serde_json::from_value(body["user_id"].clone())
        .map_err(|_| crate::utils::errors::AppError::BadRequest("Invalid user_id".into()))?;
    let member = OrgService::add_member(&state, org_id, user.id(), user_id).await?;
    Ok(Json(member))
}

pub async fn remove_member(
    State(state): State<AppState>,
    user: AuthUser,
    Path((org_id, user_id)): Path<(Uuid, Uuid)>,
) -> AppResult<Json<serde_json::Value>> {
    OrgService::remove_member(&state, org_id, user.id(), user_id).await?;
    Ok(Json(serde_json::json!({ "message": "Member removed" })))
}

pub async fn list_members(
    State(state): State<AppState>,
    Path(org_id): Path<Uuid>,
) -> AppResult<Json<Vec<OrganizationMember>>> {
    let members = OrgService::list_members(&state, org_id).await?;
    Ok(Json(members))
}

pub async fn my_organizations(
    State(state): State<AppState>,
    user: AuthUser,
) -> AppResult<Json<Vec<OrganizationWithStats>>> {
    let orgs = OrgService::my_organizations(&state, user.id()).await?;
    Ok(Json(orgs))
}
