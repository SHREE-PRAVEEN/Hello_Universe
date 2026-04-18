use crate::models::project::*;
use sqlx::PgPool;
use uuid::Uuid;
use crate::utils::errors::{AppError, AppResult};

pub struct ProjectRepo;

impl ProjectRepo {
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Project> {
        sqlx::query_as!(
            Project,
            r#"SELECT id, owner_id, organization_id, category_id, title, slug,
               short_description, description, status, visibility, access_type,
               price, currency, license, license_details, version_label,
               thumbnail_url, demo_url, repository_url,
               download_count, view_count, like_count, featured, metadata,
               published_at, created_at, updated_at, deleted_at
               FROM projects WHERE id = $1 AND deleted_at IS NULL"#,
            id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::not_found("Project"))
    }

    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> AppResult<ProjectWithAuthor> {
        sqlx::query_as!(
            ProjectWithAuthor,
            r#"SELECT
               p.id, p.title, p.slug, p.short_description, p.status, p.visibility,
               p.access_type, p.price, p.currency, p.license, p.version_label,
               p.thumbnail_url, p.download_count, p.view_count, p.like_count,
               p.featured, p.published_at, p.created_at, p.updated_at,
               p.owner_id, u.username AS owner_username, u.display_name AS owner_display_name,
               u.avatar_url AS owner_avatar_url,
               p.organization_id,
               o.name AS organization_name, o.slug AS organization_slug,
               pc.name AS category_name,
               ROUND(AVG(rv.rating)::NUMERIC, 2) AS "avg_rating: f64",
               COUNT(DISTINCT rv.id) AS "review_count: i64",
               COALESCE(json_agg(DISTINCT t.name) FILTER (WHERE t.id IS NOT NULL), '[]') AS "tags: serde_json::Value"
               FROM projects p
               JOIN users u ON u.id = p.owner_id
               LEFT JOIN organizations o ON o.id = p.organization_id
               LEFT JOIN project_categories pc ON pc.id = p.category_id
               LEFT JOIN project_tags pt ON pt.project_id = p.id
               LEFT JOIN tags t ON t.id = pt.tag_id
               LEFT JOIN reviews rv ON rv.entity_type = 'project' AND rv.entity_id = p.id AND rv.status = 'approved' AND rv.deleted_at IS NULL
               WHERE p.slug = $1 AND p.deleted_at IS NULL
               GROUP BY p.id, u.id, o.id, pc.id"#,
            slug
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::not_found("Project"))
    }

    pub async fn list_published(
        pool: &PgPool,
        filter: &ProjectFilterParams,
        limit: i64,
        offset: i64,
    ) -> AppResult<(Vec<ProjectWithAuthor>, i64)> {
        let tags: Option<Vec<&str>> = filter.tags.as_ref().map(|t| t.split(',').collect());

        let rows = sqlx::query_as!(
            ProjectWithAuthor,
            r#"SELECT
               p.id, p.title, p.slug, p.short_description, p.status, p.visibility,
               p.access_type, p.price, p.currency, p.license, p.version_label,
               p.thumbnail_url, p.download_count, p.view_count, p.like_count,
               p.featured, p.published_at, p.created_at, p.updated_at,
               p.owner_id, u.username AS owner_username, u.display_name AS owner_display_name,
               u.avatar_url AS owner_avatar_url,
               p.organization_id, o.name AS organization_name, o.slug AS organization_slug,
               pc.name AS category_name,
               ROUND(AVG(rv.rating)::NUMERIC, 2) AS "avg_rating: f64",
               COUNT(DISTINCT rv.id) AS "review_count: i64",
               COALESCE(json_agg(DISTINCT t.name) FILTER (WHERE t.id IS NOT NULL), '[]') AS "tags: serde_json::Value"
               FROM projects p
               JOIN users u ON u.id = p.owner_id
               LEFT JOIN organizations o ON o.id = p.organization_id
               LEFT JOIN project_categories pc ON pc.id = p.category_id
               LEFT JOIN project_tags pt ON pt.project_id = p.id
               LEFT JOIN tags t ON t.id = pt.tag_id
               LEFT JOIN reviews rv ON rv.entity_type = 'project' AND rv.entity_id = p.id AND rv.status = 'approved' AND rv.deleted_at IS NULL
               WHERE p.status = 'published'
                 AND p.deleted_at IS NULL
                 AND ($1::TEXT IS NULL OR p.search_vector @@ plainto_tsquery('english', $1))
                 AND ($2::UUID IS NULL OR p.category_id = $2)
                 AND ($3::TEXT IS NULL OR p.access_type = $3)
                 AND ($4::UUID IS NULL OR p.owner_id = $4)
                 AND ($5::UUID IS NULL OR p.organization_id = $5)
                 AND ($6::BOOL IS NULL OR p.featured = $6)
                 AND ($7::TEXT[] IS NULL OR t.slug = ANY($7))
               GROUP BY p.id, u.id, o.id, pc.id
               ORDER BY
                 CASE WHEN $8 = 'popular'  THEN p.download_count END DESC NULLS LAST,
                 CASE WHEN $8 = 'trending' THEN p.view_count      END DESC NULLS LAST,
                 CASE WHEN $8 = 'rated'    THEN AVG(rv.rating)    END DESC NULLS LAST,
                 p.published_at DESC NULLS LAST
               LIMIT $9 OFFSET $10"#,
            filter.q.as_deref(),
            filter.owner_id as _,
            filter.access_type.as_deref(),
            filter.owner_id as _,
            filter.organization_id as _,
            filter.featured as _,
            tags.as_deref() as _,
            filter.sort.as_deref(),
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        let total = sqlx::query_scalar!(
            r#"SELECT COUNT(DISTINCT p.id)
               FROM projects p
               LEFT JOIN project_tags pt ON pt.project_id = p.id
               LEFT JOIN tags t ON t.id = pt.tag_id
               WHERE p.status = 'published' AND p.deleted_at IS NULL
                 AND ($1::TEXT IS NULL OR p.search_vector @@ plainto_tsquery('english', $1))
                 AND ($2::UUID IS NULL OR p.category_id = $2)
                 AND ($3::TEXT IS NULL OR p.access_type = $3)"#,
            filter.q.as_deref(),
            filter.category_id as _,
            filter.access_type.as_deref()
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        Ok((rows, total))
    }

    pub async fn create(pool: &PgPool, owner_id: Uuid, req: &CreateProjectRequest) -> AppResult<Project> {
        let slug = slugify(&req.title);
        Ok(sqlx::query_as!(
            Project,
            r#"INSERT INTO projects
               (owner_id, organization_id, category_id, title, slug, short_description,
                description, visibility, access_type, price, currency, license, license_details,
                thumbnail_url, demo_url, repository_url, metadata)
               VALUES ($1,$2,$3,$4,$5,$6,$7,
                       COALESCE($8,'private'), COALESCE($9,'free'),
                       $10, COALESCE($11,'USD'), COALESCE($12,'proprietary'),$13,
                       $14,$15,$16, COALESCE($17,'{}'))
               RETURNING id, owner_id, organization_id, category_id, title, slug,
               short_description, description, status, visibility, access_type,
               price, currency, license, license_details, version_label,
               thumbnail_url, demo_url, repository_url,
               download_count, view_count, like_count, featured, metadata,
               published_at, created_at, updated_at, deleted_at"#,
            owner_id,
            req.organization_id as _,
            req.category_id as _,
            req.title,
            slug,
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
        .fetch_one(pool)
        .await?)
    }

    pub async fn update_status(pool: &PgPool, id: Uuid, status: &str) -> AppResult<()> {
        let published_at = if status == "published" {
            Some(time::OffsetDateTime::now_utc())
        } else {
            None
        };
        sqlx::query!(
            "UPDATE projects SET status = $2, published_at = COALESCE($3, published_at), updated_at = NOW()
             WHERE id = $1 AND deleted_at IS NULL",
            id, status, published_at
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn soft_delete(pool: &PgPool, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            "UPDATE projects SET deleted_at = NOW(), status = 'archived' WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn set_tags(pool: &PgPool, project_id: Uuid, tags: &[String]) -> AppResult<()> {
        // Remove old tags
        sqlx::query!("DELETE FROM project_tags WHERE project_id = $1", project_id)
            .execute(pool)
            .await?;
        // Upsert new tags and link
        for tag_name in tags {
            let slug = slugify(tag_name);
            let tag = sqlx::query!(
                "INSERT INTO tags (name, slug) VALUES ($1, $2)
                 ON CONFLICT (slug) DO UPDATE SET use_count = tags.use_count
                 RETURNING id",
                tag_name, slug
            )
            .fetch_one(pool)
            .await?;
            sqlx::query!(
                "INSERT INTO project_tags (project_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
                project_id, tag.id
            )
            .execute(pool)
            .await?;
        }
        Ok(())
    }

    pub async fn create_version(
        pool: &PgPool,
        project_id: Uuid,
        created_by: Uuid,
        req: &CreateVersionRequest,
    ) -> AppResult<ProjectVersion> {
        // Unset previous current version
        sqlx::query!(
            "UPDATE project_versions SET is_current = FALSE WHERE project_id = $1 AND is_current = TRUE",
            project_id
        )
        .execute(pool)
        .await?;

        Ok(sqlx::query_as!(
            ProjectVersion,
            r#"INSERT INTO project_versions (project_id, version_label, changelog, commit_hash, created_by, is_current)
               VALUES ($1, $2, $3, $4, $5, TRUE)
               RETURNING id, project_id, version_label, commit_hash, changelog,
               snapshot_meta, created_by, is_current, created_at"#,
            project_id, req.version_label, req.changelog.as_deref(),
            req.commit_hash.as_deref(), created_by
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_versions(pool: &PgPool, project_id: Uuid) -> AppResult<Vec<ProjectVersion>> {
        Ok(sqlx::query_as!(
            ProjectVersion,
            r#"SELECT id, project_id, version_label, commit_hash, changelog,
               snapshot_meta, created_by, is_current, created_at
               FROM project_versions WHERE project_id = $1
               ORDER BY created_at DESC"#,
            project_id
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_collaborators(pool: &PgPool, project_id: Uuid) -> AppResult<Vec<ProjectCollaborator>> {
        Ok(sqlx::query_as!(
            ProjectCollaborator,
            "SELECT id, project_id, user_id, role, added_by, added_at
             FROM project_collaborators WHERE project_id = $1",
            project_id
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn top_downloads(pool: &PgPool, from: &str, to: &str, limit: i64) -> AppResult<Vec<crate::models::analytics::TopProject>> {
        Ok(sqlx::query_as!(
            crate::models::analytics::TopProject,
            r#"SELECT p.id, p.title, p.slug,
               u.username AS author, o.name AS organization,
               COUNT(d.id) AS "period_downloads: i64",
               p.download_count AS total_downloads,
               SUM(pu.amount) AS total_revenue
               FROM projects p
               JOIN downloads d ON d.entity_type = 'project' AND d.entity_id = p.id
                 AND d.downloaded_at BETWEEN $1::TIMESTAMPTZ AND $2::TIMESTAMPTZ
               JOIN users u ON u.id = p.owner_id
               LEFT JOIN organizations o ON o.id = p.organization_id
               LEFT JOIN purchases pu ON pu.entity_type = 'project' AND pu.entity_id = p.id
                 AND pu.status = 'completed'
                 AND pu.created_at BETWEEN $1::TIMESTAMPTZ AND $2::TIMESTAMPTZ
               WHERE p.deleted_at IS NULL
               GROUP BY p.id, u.id, o.id
               ORDER BY COUNT(d.id) DESC
               LIMIT $3"#,
            from, to, limit
        )
        .fetch_all(pool)
        .await?)
    }
}

fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
