use crate::models::analytics::*;
use sqlx::PgPool;
use uuid::Uuid;
use crate::utils::errors::AppResult;

pub struct AnalyticsRepo;

impl AnalyticsRepo {
    pub async fn record_project_view(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Option<Uuid>,
        session_id: Option<&str>,
        ip: Option<&str>,
        referrer: Option<&str>,
        country: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"INSERT INTO project_views (project_id, user_id, session_id, ip_address, referrer, country)
               VALUES ($1, $2, $3, $4::inet, $5, $6)"#,
            project_id, user_id as _, session_id, ip, referrer, country
        )
        .execute(pool)
        .await?;
        // bump denormalized counter
        sqlx::query!(
            "UPDATE projects SET view_count = view_count + 1 WHERE id = $1",
            project_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn record_media_view(
        pool: &PgPool,
        media_file_id: Uuid,
        user_id: Option<Uuid>,
        session_id: Option<&str>,
        watch_seconds: Option<i32>,
        completion_pct: Option<f64>,
        country: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"INSERT INTO media_views (media_file_id, user_id, session_id, watch_seconds, completion_pct, country)
               VALUES ($1, $2, $3, $4, $5, $6)"#,
            media_file_id, user_id as _, session_id, watch_seconds,
            completion_pct.map(|v| bigdecimal::BigDecimal::try_from(v).unwrap()) as _,
            country
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn record_download(
        pool: &PgPool,
        entity_type: &str,
        entity_id: Uuid,
        user_id: Option<Uuid>,
        ip: Option<&str>,
        country: Option<&str>,
        version_label: Option<&str>,
        file_size: Option<i64>,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"INSERT INTO downloads (entity_type, entity_id, user_id, ip_address, country, version_label, file_size_bytes)
               VALUES ($1, $2, $3, $4::inet, $5, $6, $7)"#,
            entity_type, entity_id, user_id as _, ip, country, version_label, file_size
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn log_search_query(
        pool: &PgPool,
        user_id: Option<Uuid>,
        query_text: &str,
        result_count: Option<i32>,
        latency_ms: Option<i32>,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"INSERT INTO search_queries (user_id, query_text, result_count, search_latency_ms)
               VALUES ($1, $2, $3, $4)"#,
            user_id as _, query_text, result_count, latency_ms
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn project_view_stats(
        pool: &PgPool,
        project_id: Uuid,
        from: &str,
        to: &str,
    ) -> AppResult<Vec<ProjectDailyStats>> {
        Ok(sqlx::query_as!(
            ProjectDailyStats,
            r#"SELECT project_id, DATE_TRUNC('day', viewed_at) AS "day!: time::OffsetDateTime",
               COUNT(*) AS "views: i64",
               COUNT(DISTINCT user_id) AS "unique_visitors: i64"
               FROM project_views
               WHERE project_id = $1
                 AND viewed_at BETWEEN $2::TIMESTAMPTZ AND $3::TIMESTAMPTZ
               GROUP BY 1, 2 ORDER BY 2 ASC"#,
            project_id, from, to
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn engagement_summary(pool: &PgPool, entity_id: Uuid) -> AppResult<EngagementSummary> {
        sqlx::query_as!(
            EngagementSummary,
            r#"SELECT $1::UUID AS "entity_id: Uuid",
               (SELECT COUNT(*) FROM project_views WHERE project_id = $1) AS "total_views: i64",
               (SELECT COUNT(DISTINCT user_id) FROM project_views WHERE project_id = $1) AS "unique_views: i64",
               (SELECT COUNT(*) FROM downloads WHERE entity_id = $1) AS "total_downloads: i64",
               (SELECT COUNT(*) FROM engagement_events WHERE entity_id = $1 AND event_type = 'like') AS "total_likes: i64",
               NULL::FLOAT8 AS "avg_watch_pct: f64""#,
            entity_id
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    pub async fn log_activity(
        pool: &PgPool,
        user_id: Option<Uuid>,
        org_id: Option<Uuid>,
        action: &str,
        entity_type: Option<&str>,
        entity_id: Option<Uuid>,
        description: Option<&str>,
        ip: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"INSERT INTO activity_logs (user_id, organization_id, action, entity_type, entity_id, description, ip_address)
               VALUES ($1, $2, $3, $4, $5, $6, $7::inet)"#,
            user_id as _, org_id as _, action, entity_type, entity_id as _, description, ip
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
