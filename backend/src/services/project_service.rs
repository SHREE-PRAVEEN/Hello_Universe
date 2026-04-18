use crate::{
    config::AppState,
    models::project::*,
    repositories::{AnalyticsRepo, ModerationRepo, ProjectRepo},
    utils::errors::{AppError, AppResult},
};
use uuid::Uuid;

pub struct ProjectService;

impl ProjectService {
    pub async fn list(
        state: &AppState,
        filter: &ProjectFilterParams,
    ) -> AppResult<(Vec<ProjectWithAuthor>, i64)> {
        let limit = filter.per_page.unwrap_or(20).min(100);
        let offset = (filter.page.unwrap_or(1) - 1) * limit;
        ProjectRepo::list_published(&state.db, filter, limit, offset).await
    }

    pub async fn get_by_slug(state: &AppState, slug: &str) -> AppResult<ProjectWithAuthor> {
        ProjectRepo::find_by_slug(&state.db, slug).await
    }

    pub async fn create(
        state: &AppState,
        owner_id: Uuid,
        req: &CreateProjectRequest,
    ) -> AppResult<Project> {
        // Validate org membership if org_id provided
        if let Some(org_id) = req.organization_id {
            let is_member = crate::repositories::OrgRepo::is_member(&state.db, org_id, owner_id).await?;
            if !is_member {
                return Err(AppError::Forbidden(
                    "You are not a member of this organization".into(),
                ));
            }
        }

        let project = ProjectRepo::create(&state.db, owner_id, req).await?;

        // Set tags
        if let Some(tags) = &req.tags {
            ProjectRepo::set_tags(&state.db, project.id, tags).await?;
        }

        // Log activity
        AnalyticsRepo::log_activity(
            &state.db,
            Some(owner_id),
            req.organization_id,
            "project.create",
            Some("project"),
            Some(project.id),
            Some(&format!("Created project: {}", project.title)),
            None,
        )
        .await?;

        Ok(project)
    }

    pub async fn update(
        state: &AppState,
        id: Uuid,
        requester_id: Uuid,
        req: &UpdateProjectRequest,
    ) -> AppResult<Project> {
        let project = ProjectRepo::find_by_id(&state.db, id).await?;
        Self::assert_can_edit(&project, requester_id)?;

        let updated = sqlx::query_as!(
            Project,
            r#"UPDATE projects SET
               title = COALESCE($2, title),
               category_id = COALESCE($3, category_id),
               short_description = COALESCE($4, short_description),
               description = COALESCE($5, description),
               visibility = COALESCE($6, visibility),
               access_type = COALESCE($7, access_type),
               price = COALESCE($8, price),
               currency = COALESCE($9, currency),
               license = COALESCE($10, license),
               license_details = COALESCE($11, license_details),
               thumbnail_url = COALESCE($12, thumbnail_url),
               demo_url = COALESCE($13, demo_url),
               repository_url = COALESCE($14, repository_url),
               metadata = CASE WHEN $15::jsonb IS NOT NULL THEN $15 ELSE metadata END,
               updated_at = NOW()
               WHERE id = $1 AND deleted_at IS NULL
               RETURNING id, owner_id, organization_id, category_id, title, slug,
               short_description, description, status, visibility, access_type,
               price, currency, license, license_details, version_label,
               thumbnail_url, demo_url, repository_url,
               download_count, view_count, like_count, featured, metadata,
               published_at, created_at, updated_at, deleted_at"#,
            id,
            req.title.as_deref(),
            req.category_id as _,
            req.short_description.as_deref(),
            req.description.as_deref(),
            req.visibility.as_deref(),
            req.access_type.as_deref(),
            req.price as _,
            req.currency.as_deref(),
            req.license.as_deref(),
            req.license_details.as_deref(),
            req.thumbnail_url.as_deref(),
            req.demo_url.as_deref(),
            req.repository_url.as_deref(),
            req.metadata as _
        )
        .fetch_one(&state.db)
        .await?;

        if let Some(tags) = &req.tags {
            ProjectRepo::set_tags(&state.db, id, tags).await?;
        }

        Ok(updated)
    }

    pub async fn submit_for_review(
        state: &AppState,
        id: Uuid,
        requester_id: Uuid,
    ) -> AppResult<()> {
        let project = ProjectRepo::find_by_id(&state.db, id).await?;
        Self::assert_can_edit(&project, requester_id)?;

        if project.status != "draft" && project.status != "rejected" {
            return Err(AppError::BadRequest(
                "Only draft or rejected projects can be submitted for review".into(),
            ));
        }

        ProjectRepo::update_status(&state.db, id, "pending_review").await?;

        // Enqueue in moderation
        ModerationRepo::enqueue(&state.db, "project", id, "new_submission", 3).await?;

        Ok(())
    }

    pub async fn publish(state: &AppState, id: Uuid, moderator_id: Uuid) -> AppResult<()> {
        let project = ProjectRepo::find_by_id(&state.db, id).await?;
        if project.status != "pending_review" && project.status != "approved" {
            return Err(AppError::BadRequest("Project must be approved before publishing".into()));
        }
        ProjectRepo::update_status(&state.db, id, "published").await?;

        ModerationRepo::insert_audit_log(
            &state.db,
            Some(moderator_id),
            Some("moderator"),
            project.organization_id,
            "project.publish",
            Some("project"),
            Some(id),
            Some(&serde_json::json!({"status": project.status})),
            Some(&serde_json::json!({"status": "published"})),
            None,
            None,
        )
        .await?;

        Ok(())
    }

    pub async fn archive(state: &AppState, id: Uuid, requester_id: Uuid) -> AppResult<()> {
        let project = ProjectRepo::find_by_id(&state.db, id).await?;
        Self::assert_can_edit(&project, requester_id)?;
        ProjectRepo::soft_delete(&state.db, id).await
    }

    pub async fn create_version(
        state: &AppState,
        project_id: Uuid,
        requester_id: Uuid,
        req: &CreateVersionRequest,
    ) -> AppResult<ProjectVersion> {
        let project = ProjectRepo::find_by_id(&state.db, project_id).await?;
        Self::assert_can_edit(&project, requester_id)?;
        ProjectRepo::create_version(&state.db, project_id, requester_id, req).await
    }

    pub async fn get_versions(state: &AppState, project_id: Uuid) -> AppResult<Vec<ProjectVersion>> {
        ProjectRepo::get_versions(&state.db, project_id).await
    }

    pub async fn add_collaborator(
        state: &AppState,
        project_id: Uuid,
        requester_id: Uuid,
        req: &AddCollaboratorRequest,
    ) -> AppResult<ProjectCollaborator> {
        let project = ProjectRepo::find_by_id(&state.db, project_id).await?;
        if project.owner_id != requester_id {
            return Err(AppError::Forbidden("Only the project owner can add collaborators".into()));
        }

        Ok(sqlx::query_as!(
            ProjectCollaborator,
            r#"INSERT INTO project_collaborators (project_id, user_id, role, added_by)
               VALUES ($1, $2, COALESCE($3, 'contributor'), $4)
               ON CONFLICT (project_id, user_id) DO UPDATE SET role = EXCLUDED.role
               RETURNING id, project_id, user_id, role, added_by, added_at"#,
            project_id,
            req.user_id,
            req.role.as_deref(),
            requester_id
        )
        .fetch_one(&state.db)
        .await?)
    }

    pub async fn top_downloads(
        state: &AppState,
        from: &str,
        to: &str,
    ) -> AppResult<Vec<crate::models::analytics::TopProject>> {
        ProjectRepo::top_downloads(&state.db, from, to, 20).await
    }

    pub async fn list_categories(state: &AppState) -> AppResult<Vec<ProjectCategory>> {
        Ok(sqlx::query_as!(
            ProjectCategory,
            "SELECT id, name, slug, parent_id, description, icon_url, sort_order, created_at
             FROM project_categories ORDER BY sort_order ASC, name ASC"
        )
        .fetch_all(&state.db)
        .await?)
    }

    // ---- Access guards ----
    fn assert_can_edit(project: &Project, requester_id: Uuid) -> AppResult<()> {
        if project.owner_id != requester_id {
            return Err(AppError::Forbidden(
                "You do not have permission to edit this project".into(),
            ));
        }
        Ok(())
    }
}
