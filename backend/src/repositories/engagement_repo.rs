use crate::models::engagement::*;
use sqlx::PgPool;
use uuid::Uuid;
use crate::utils::errors::{AppError, AppResult};

pub struct EngagementRepo;

impl EngagementRepo {
    // ---- Reviews ----
    pub async fn create_review(pool: &PgPool, user_id: Uuid, req: &CreateReviewRequest) -> AppResult<Review> {
        Ok(sqlx::query_as!(
            Review,
            r#"INSERT INTO reviews (entity_type, entity_id, user_id, rating, title, body)
               VALUES ($1,$2,$3,$4,$5,$6)
               RETURNING id, entity_type, entity_id, user_id, rating, title, body,
               is_verified, helpful_count, status, created_at, updated_at, deleted_at"#,
            req.entity_type, req.entity_id, user_id, req.rating,
            req.title.as_deref(), req.body.as_deref()
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_reviews(
        pool: &PgPool,
        entity_type: &str,
        entity_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<Review>> {
        Ok(sqlx::query_as!(
            Review,
            r#"SELECT id, entity_type, entity_id, user_id, rating, title, body,
               is_verified, helpful_count, status, created_at, updated_at, deleted_at
               FROM reviews
               WHERE entity_type = $1 AND entity_id = $2
                 AND status = 'approved' AND deleted_at IS NULL
               ORDER BY helpful_count DESC, created_at DESC
               LIMIT $3 OFFSET $4"#,
            entity_type, entity_id, limit, offset
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn rating_summary(pool: &PgPool, entity_id: Uuid) -> AppResult<RatingSummary> {
        sqlx::query_as!(
            RatingSummary,
            r#"SELECT $1::UUID AS "entity_id: Uuid",
               COALESCE(AVG(rating), 0)::FLOAT8 AS "avg_rating!: f64",
               COUNT(*) AS "total_reviews!: i64",
               COUNT(*) FILTER (WHERE rating = 5) AS "five_star!: i64",
               COUNT(*) FILTER (WHERE rating = 4) AS "four_star!: i64",
               COUNT(*) FILTER (WHERE rating = 3) AS "three_star!: i64",
               COUNT(*) FILTER (WHERE rating = 2) AS "two_star!: i64",
               COUNT(*) FILTER (WHERE rating = 1) AS "one_star!: i64"
               FROM reviews WHERE entity_id = $1 AND status = 'approved' AND deleted_at IS NULL"#,
            entity_id
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    // ---- Comments ----
    pub async fn create_comment(pool: &PgPool, user_id: Uuid, req: &CreateCommentRequest) -> AppResult<Comment> {
        Ok(sqlx::query_as!(
            Comment,
            r#"INSERT INTO comments (entity_type, entity_id, user_id, parent_id, body)
               VALUES ($1,$2,$3,$4,$5)
               RETURNING id, entity_type, entity_id, user_id, parent_id, body,
               like_count, status, created_at, updated_at, deleted_at"#,
            req.entity_type, req.entity_id, user_id, req.parent_id as _, req.body
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_comments(
        pool: &PgPool,
        entity_type: &str,
        entity_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<CommentWithAuthor>> {
        Ok(sqlx::query_as!(
            CommentWithAuthor,
            r#"SELECT c.id, c.entity_type, c.entity_id, c.user_id,
               u.username, u.display_name, u.avatar_url,
               c.parent_id, c.body, c.like_count, c.status,
               c.created_at, c.updated_at
               FROM comments c
               JOIN users u ON u.id = c.user_id
               WHERE c.entity_type = $1 AND c.entity_id = $2
                 AND c.parent_id IS NULL AND c.deleted_at IS NULL
               ORDER BY c.created_at ASC
               LIMIT $3 OFFSET $4"#,
            entity_type, entity_id, limit, offset
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn delete_comment(pool: &PgPool, id: Uuid, user_id: Uuid) -> AppResult<()> {
        sqlx::query!(
            "UPDATE comments SET deleted_at = NOW() WHERE id = $1 AND user_id = $2",
            id, user_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    // ---- Favorites ----
    pub async fn toggle_favorite(pool: &PgPool, user_id: Uuid, entity_type: &str, entity_id: Uuid) -> AppResult<bool> {
        let existing = sqlx::query_scalar!(
            "SELECT id FROM favorites WHERE user_id=$1 AND entity_type=$2 AND entity_id=$3",
            user_id, entity_type, entity_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(fav_id) = existing {
            sqlx::query!("DELETE FROM favorites WHERE id = $1", fav_id)
                .execute(pool)
                .await?;
            Ok(false) // removed
        } else {
            sqlx::query!(
                "INSERT INTO favorites (user_id, entity_type, entity_id) VALUES ($1,$2,$3)",
                user_id, entity_type, entity_id
            )
            .execute(pool)
            .await?;
            Ok(true) // added
        }
    }

    pub async fn is_favorited(pool: &PgPool, user_id: Uuid, entity_type: &str, entity_id: Uuid) -> AppResult<bool> {
        Ok(sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM favorites WHERE user_id=$1 AND entity_type=$2 AND entity_id=$3)",
            user_id, entity_type, entity_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(false))
    }

    // ---- Follows ----
    pub async fn toggle_follow(pool: &PgPool, follower_id: Uuid, entity_type: &str, entity_id: Uuid) -> AppResult<bool> {
        let existing = sqlx::query_scalar!(
            "SELECT id FROM follows WHERE follower_id=$1 AND entity_type=$2 AND entity_id=$3",
            follower_id, entity_type, entity_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(fid) = existing {
            sqlx::query!("DELETE FROM follows WHERE id = $1", fid).execute(pool).await?;
            Ok(false)
        } else {
            sqlx::query!(
                "INSERT INTO follows (follower_id, entity_type, entity_id) VALUES ($1,$2,$3)",
                follower_id, entity_type, entity_id
            )
            .execute(pool)
            .await?;
            Ok(true)
        }
    }

    // ---- Reports ----
    pub async fn create_report(pool: &PgPool, reporter_id: Uuid, req: &CreateReportRequest) -> AppResult<Report> {
        Ok(sqlx::query_as!(
            Report,
            r#"INSERT INTO reports (reporter_id, entity_type, entity_id, reason, description)
               VALUES ($1,$2,$3,$4,$5)
               RETURNING id, reporter_id, entity_type, entity_id, reason, description,
               status, resolved_by, resolved_at, resolution_note, created_at"#,
            reporter_id, req.entity_type, req.entity_id, req.reason,
            req.description.as_deref()
        )
        .fetch_one(pool)
        .await?)
    }
}
