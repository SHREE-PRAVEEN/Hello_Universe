use crate::models::organization::*;
use sqlx::PgPool;
use uuid::Uuid;
use crate::utils::errors::{AppError, AppResult};

pub struct OrgRepo;

impl OrgRepo {
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Organization> {
        sqlx::query_as!(
            Organization,
            "SELECT id, name, slug, org_type, description, logo_url, website_url,
             email, country, verified, owner_id, metadata, created_at, updated_at, deleted_at
             FROM organizations WHERE id = $1 AND deleted_at IS NULL",
            id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::not_found("Organization"))
    }

    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> AppResult<Organization> {
        sqlx::query_as!(
            Organization,
            "SELECT id, name, slug, org_type, description, logo_url, website_url,
             email, country, verified, owner_id, metadata, created_at, updated_at, deleted_at
             FROM organizations WHERE slug = $1 AND deleted_at IS NULL",
            slug
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::not_found("Organization"))
    }

    pub async fn create(pool: &PgPool, owner_id: Uuid, req: &CreateOrganizationRequest) -> AppResult<Organization> {
        Ok(sqlx::query_as!(
            Organization,
            r#"INSERT INTO organizations (name, slug, org_type, description, website_url, email, country, owner_id)
               VALUES ($1,$2,COALESCE($3,'company')::org_type,$4,$5,$6,$7,$8)
               RETURNING id, name, slug, org_type, description, logo_url, website_url,
               email, country, verified, owner_id, metadata, created_at, updated_at, deleted_at"#,
            req.name, req.slug, req.org_type.as_deref(), req.description.as_deref(),
            req.website_url.as_deref(), req.email.as_deref(), req.country.as_deref(), owner_id
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn is_member(pool: &PgPool, org_id: Uuid, user_id: Uuid) -> AppResult<bool> {
        Ok(sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM organization_members WHERE organization_id=$1 AND user_id=$2 AND is_active=TRUE)",
            org_id, user_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(false))
    }

    pub async fn add_member(
        pool: &PgPool,
        org_id: Uuid,
        user_id: Uuid,
        invited_by: Option<Uuid>,
        org_role_id: Option<Uuid>,
    ) -> AppResult<OrganizationMember> {
        Ok(sqlx::query_as!(
            OrganizationMember,
            r#"INSERT INTO organization_members (organization_id, user_id, invited_by, org_role_id)
               VALUES ($1,$2,$3,$4)
               ON CONFLICT (organization_id, user_id) DO UPDATE SET is_active = TRUE
               RETURNING id, organization_id, user_id, org_role_id, invited_by, joined_at, is_active"#,
            org_id, user_id, invited_by as _, org_role_id as _
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn remove_member(pool: &PgPool, org_id: Uuid, user_id: Uuid) -> AppResult<()> {
        sqlx::query!(
            "UPDATE organization_members SET is_active = FALSE WHERE organization_id=$1 AND user_id=$2",
            org_id, user_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn list_members(pool: &PgPool, org_id: Uuid) -> AppResult<Vec<OrganizationMember>> {
        Ok(sqlx::query_as!(
            OrganizationMember,
            "SELECT id, organization_id, user_id, org_role_id, invited_by, joined_at, is_active
             FROM organization_members WHERE organization_id=$1 AND is_active=TRUE",
            org_id
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn user_organizations(pool: &PgPool, user_id: Uuid) -> AppResult<Vec<OrganizationWithStats>> {
        Ok(sqlx::query_as!(
            OrganizationWithStats,
            r#"SELECT o.id, o.name, o.slug, o.org_type, o.description, o.logo_url, o.verified, o.created_at,
               (SELECT COUNT(*) FROM projects p WHERE p.organization_id = o.id AND p.deleted_at IS NULL) AS "project_count: i64",
               (SELECT COUNT(*) FROM organization_members m WHERE m.organization_id = o.id AND m.is_active = TRUE) AS "member_count: i64"
               FROM organizations o
               JOIN organization_members om ON om.organization_id = o.id
               WHERE om.user_id = $1 AND om.is_active = TRUE AND o.deleted_at IS NULL"#,
            user_id
        )
        .fetch_all(pool)
        .await?)
    }
}
