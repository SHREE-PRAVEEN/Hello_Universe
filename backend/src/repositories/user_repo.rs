use crate::models::user::*;
use sqlx::PgPool;
use uuid::Uuid;
use crate::utils::errors::{AppError, AppResult};

pub struct UserRepo;

impl UserRepo {
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<User> {
        sqlx::query_as!(
            User,
            r#"SELECT id, username, email, password_hash,
               email_verified_at, display_name, avatar_url, bio,
               website_url, location, status, is_superadmin, metadata,
               last_login_at, last_login_ip::TEXT, created_at, updated_at, deleted_at
               FROM users WHERE id = $1 AND deleted_at IS NULL"#,
            id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::not_found("User"))
    }

    pub async fn find_by_email(pool: &PgPool, email: &str) -> AppResult<Option<User>> {
        Ok(sqlx::query_as!(
            User,
            r#"SELECT id, username, email, password_hash,
               email_verified_at, display_name, avatar_url, bio,
               website_url, location, status, is_superadmin, metadata,
               last_login_at, last_login_ip::TEXT, created_at, updated_at, deleted_at
               FROM users WHERE email = $1 AND deleted_at IS NULL"#,
            email
        )
        .fetch_optional(pool)
        .await?)
    }

    pub async fn find_by_username(pool: &PgPool, username: &str) -> AppResult<Option<User>> {
        Ok(sqlx::query_as!(
            User,
            r#"SELECT id, username, email, password_hash,
               email_verified_at, display_name, avatar_url, bio,
               website_url, location, status, is_superadmin, metadata,
               last_login_at, last_login_ip::TEXT, created_at, updated_at, deleted_at
               FROM users WHERE username = $1 AND deleted_at IS NULL"#,
            username
        )
        .fetch_optional(pool)
        .await?)
    }

    pub async fn find_auth_user(pool: &PgPool, email: &str) -> AppResult<Option<AuthUser>> {
        Ok(sqlx::query_as!(
            AuthUser,
            r#"SELECT u.id, u.username, u.email, u.status, u.is_superadmin,
               COALESCE(json_agg(DISTINCT r.slug) FILTER (WHERE r.id IS NOT NULL), '[]') AS "roles: serde_json::Value",
               COALESCE(json_agg(DISTINCT p.name)  FILTER (WHERE p.id IS NOT NULL), '[]') AS "permissions: serde_json::Value"
               FROM users u
               LEFT JOIN user_roles ur ON ur.user_id = u.id
                 AND (ur.expires_at IS NULL OR ur.expires_at > NOW())
               LEFT JOIN roles r ON r.id = ur.role_id
               LEFT JOIN role_permissions rp ON rp.role_id = r.id
               LEFT JOIN permissions p ON p.id = rp.permission_id
               WHERE u.email = $1 AND u.deleted_at IS NULL
               GROUP BY u.id"#,
            email
        )
        .fetch_optional(pool)
        .await?)
    }

    pub async fn create(
        pool: &PgPool,
        username: &str,
        email: &str,
        password_hash: &str,
        display_name: Option<&str>,
    ) -> AppResult<User> {
        Ok(sqlx::query_as!(
            User,
            r#"INSERT INTO users (username, email, password_hash, display_name)
               VALUES ($1, $2, $3, $4)
               RETURNING id, username, email, password_hash,
               email_verified_at, display_name, avatar_url, bio,
               website_url, location, status, is_superadmin, metadata,
               last_login_at, last_login_ip::TEXT, created_at, updated_at, deleted_at"#,
            username, email, password_hash, display_name
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn update(pool: &PgPool, id: Uuid, req: &UpdateUserRequest) -> AppResult<User> {
        Ok(sqlx::query_as!(
            User,
            r#"UPDATE users SET
               display_name = COALESCE($2, display_name),
               bio = COALESCE($3, bio),
               website_url = COALESCE($4, website_url),
               location = COALESCE($5, location),
               avatar_url = COALESCE($6, avatar_url),
               metadata = CASE WHEN $7::jsonb IS NOT NULL THEN $7 ELSE metadata END,
               updated_at = NOW()
               WHERE id = $1 AND deleted_at IS NULL
               RETURNING id, username, email, password_hash,
               email_verified_at, display_name, avatar_url, bio,
               website_url, location, status, is_superadmin, metadata,
               last_login_at, last_login_ip::TEXT, created_at, updated_at, deleted_at"#,
            id,
            req.display_name.as_deref(),
            req.bio.as_deref(),
            req.website_url.as_deref(),
            req.location.as_deref(),
            req.avatar_url.as_deref(),
            req.metadata as _
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn update_last_login(pool: &PgPool, id: Uuid, ip: &str) -> AppResult<()> {
        sqlx::query!(
            "UPDATE users SET last_login_at = NOW(), last_login_ip = $2::inet
             WHERE id = $1",
            id, ip
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn mark_email_verified(pool: &PgPool, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            "UPDATE users SET email_verified_at = NOW(), status = 'active'
             WHERE id = $1 AND email_verified_at IS NULL",
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn update_password(pool: &PgPool, id: Uuid, password_hash: &str) -> AppResult<()> {
        sqlx::query!(
            "UPDATE users SET password_hash = $2, updated_at = NOW() WHERE id = $1",
            id, password_hash
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn soft_delete(pool: &PgPool, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            "UPDATE users SET deleted_at = NOW(), status = 'deleted' WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_profile_with_stats(pool: &PgPool, id: Uuid) -> AppResult<UserWithStats> {
        sqlx::query_as!(
            UserWithStats,
            r#"SELECT u.id, u.username, u.display_name, u.avatar_url, u.bio, u.location,
               u.created_at,
               (SELECT COUNT(*) FROM projects p WHERE p.owner_id = u.id AND p.deleted_at IS NULL AND p.status = 'published') AS project_count,
               (SELECT COUNT(*) FROM follows f WHERE f.entity_type = 'user' AND f.entity_id = u.id) AS follower_count,
               (SELECT COUNT(*) FROM follows f WHERE f.follower_id = u.id AND f.entity_type = 'user') AS following_count
               FROM users u WHERE u.id = $1 AND u.deleted_at IS NULL"#,
            id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::not_found("User"))
    }

    pub async fn search(pool: &PgPool, q: &str, limit: i64, offset: i64) -> AppResult<Vec<PublicUser>> {
        Ok(sqlx::query_as!(
            PublicUser,
            r#"SELECT id, username, display_name, avatar_url, bio, website_url, location, created_at
               FROM users
               WHERE search_vector @@ plainto_tsquery('english', $1)
                 AND deleted_at IS NULL AND status = 'active'
               ORDER BY ts_rank(search_vector, plainto_tsquery('english', $1)) DESC
               LIMIT $2 OFFSET $3"#,
            q, limit, offset
        )
        .fetch_all(pool)
        .await?)
    }
}
