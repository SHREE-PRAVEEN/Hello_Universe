use crate::{
    config::AppState,
    models::organization::*,
    repositories::OrgRepo,
    utils::errors::{AppError, AppResult},
};
use uuid::Uuid;

pub struct OrgService;

impl OrgService {
    pub async fn create(state: &AppState, owner_id: Uuid, req: &CreateOrganizationRequest) -> AppResult<Organization> {
        OrgRepo::create(&state.db, owner_id, req).await
    }

    pub async fn get(state: &AppState, slug: &str) -> AppResult<Organization> {
        OrgRepo::find_by_slug(&state.db, slug).await
    }

    pub async fn update(state: &AppState, id: Uuid, requester_id: Uuid, req: &UpdateOrganizationRequest) -> AppResult<Organization> {
        let org = OrgRepo::find_by_id(&state.db, id).await?;
        if org.owner_id != requester_id {
            return Err(AppError::Forbidden("Only the org owner can update organization details".into()));
        }
        Ok(sqlx::query_as!(
            Organization,
            r#"UPDATE organizations SET
               name = COALESCE($2, name),
               description = COALESCE($3, description),
               logo_url = COALESCE($4, logo_url),
               website_url = COALESCE($5, website_url),
               email = COALESCE($6, email),
               country = COALESCE($7, country),
               metadata = CASE WHEN $8::jsonb IS NOT NULL THEN $8 ELSE metadata END,
               updated_at = NOW()
               WHERE id = $1 AND deleted_at IS NULL
               RETURNING id, name, slug, org_type, description, logo_url, website_url,
               email, country, verified, owner_id, metadata, created_at, updated_at, deleted_at"#,
            id,
            req.name.as_deref(),
            req.description.as_deref(),
            req.logo_url.as_deref(),
            req.website_url.as_deref(),
            req.email.as_deref(),
            req.country.as_deref(),
            req.metadata as _
        )
        .fetch_one(&state.db)
        .await?)
    }

    pub async fn add_member(state: &AppState, org_id: Uuid, requester_id: Uuid, user_id: Uuid) -> AppResult<OrganizationMember> {
        let org = OrgRepo::find_by_id(&state.db, org_id).await?;
        let requester_is_owner = org.owner_id == requester_id;
        let requester_is_member = OrgRepo::is_member(&state.db, org_id, requester_id).await?;
        if !requester_is_owner && !requester_is_member {
            return Err(AppError::Forbidden("Only org members can invite others".into()));
        }
        OrgRepo::add_member(&state.db, org_id, user_id, Some(requester_id), None).await
    }

    pub async fn remove_member(state: &AppState, org_id: Uuid, requester_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let org = OrgRepo::find_by_id(&state.db, org_id).await?;
        if org.owner_id != requester_id && requester_id != user_id {
            return Err(AppError::Forbidden("Cannot remove other members".into()));
        }
        OrgRepo::remove_member(&state.db, org_id, user_id).await
    }

    pub async fn list_members(state: &AppState, org_id: Uuid) -> AppResult<Vec<OrganizationMember>> {
        OrgRepo::list_members(&state.db, org_id).await
    }

    pub async fn my_organizations(state: &AppState, user_id: Uuid) -> AppResult<Vec<OrganizationWithStats>> {
        OrgRepo::user_organizations(&state.db, user_id).await
    }
}
