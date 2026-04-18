use crate::{config::AppState, utils::errors::AppResult};
use tracing::info;

/// Refresh materialized views and update denormalized counters
pub async fn run(state: &AppState) -> AppResult<()> {
    info!("Starting analytics rollup");

    // Update project view_count from partitioned table (last 24 hours only for efficiency)
    sqlx::query!(
        r#"UPDATE projects p SET view_count = (
               SELECT COUNT(*) FROM project_views pv WHERE pv.project_id = p.id
           )
           WHERE p.deleted_at IS NULL
             AND p.id IN (
               SELECT DISTINCT project_id FROM project_views
               WHERE viewed_at > NOW() - INTERVAL '25 hours'
             )"#
    )
    .execute(&state.db)
    .await?;

    // Update project like_count from engagement_events
    sqlx::query!(
        r#"UPDATE projects p SET like_count = (
               SELECT COUNT(*) FROM engagement_events e
               WHERE e.entity_type = 'project' AND e.entity_id = p.id AND e.event_type = 'like'
           )
           WHERE p.deleted_at IS NULL
             AND p.id IN (
               SELECT DISTINCT entity_id FROM engagement_events
               WHERE entity_type = 'project' AND created_at > NOW() - INTERVAL '25 hours'
             )"#
    )
    .execute(&state.db)
    .await?;

    // Update tag use_count
    sqlx::query!(
        r#"UPDATE tags t SET use_count = (
               SELECT COUNT(*) FROM project_tags pt WHERE pt.tag_id = t.id
           ) + (
               SELECT COUNT(*) FROM media_tags mt WHERE mt.tag_id = t.id
           )"#
    )
    .execute(&state.db)
    .await?;

    // Expire subscriptions past their end date
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'expired'
           WHERE status = 'active' AND current_period_end < NOW()"#
    )
    .execute(&state.db)
    .await?;

    // Revoke expired entitlements
    sqlx::query!(
        r#"UPDATE content_entitlements SET revoked_at = NOW(), revoke_reason = 'expired'
           WHERE expires_at < NOW() AND revoked_at IS NULL"#
    )
    .execute(&state.db)
    .await?;

    // Expire stale session tokens
    sqlx::query!(
        r#"UPDATE sessions SET is_revoked = TRUE
           WHERE expires_at < NOW() AND is_revoked = FALSE"#
    )
    .execute(&state.db)
    .await?;

    info!("Analytics rollup complete");
    Ok(())
}
