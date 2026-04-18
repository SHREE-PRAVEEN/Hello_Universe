use crate::{
    config::AppState,
    models::analytics::*,
    repositories::AnalyticsRepo,
    utils::errors::AppResult,
};
use uuid::Uuid;

pub struct AnalyticsService;

impl AnalyticsService {
    pub async fn track_project_view(
        state: &AppState,
        project_id: Uuid,
        user_id: Option<Uuid>,
        session_id: Option<&str>,
        ip: Option<&str>,
        referrer: Option<&str>,
        country: Option<&str>,
    ) -> AppResult<()> {
        AnalyticsRepo::record_project_view(
            &state.db,
            project_id,
            user_id,
            session_id,
            ip,
            referrer,
            country,
        )
        .await
    }

    pub async fn track_media_view(
        state: &AppState,
        media_id: Uuid,
        user_id: Option<Uuid>,
        session_id: Option<&str>,
        watch_seconds: Option<i32>,
        completion_pct: Option<f64>,
        country: Option<&str>,
    ) -> AppResult<()> {
        AnalyticsRepo::record_media_view(
            &state.db,
            media_id,
            user_id,
            session_id,
            watch_seconds,
            completion_pct,
            country,
        )
        .await
    }

    pub async fn track_download(
        state: &AppState,
        req: &TrackDownloadRequest,
        user_id: Option<Uuid>,
        ip: Option<&str>,
        country: Option<&str>,
    ) -> AppResult<()> {
        AnalyticsRepo::record_download(
            &state.db,
            &req.entity_type,
            req.entity_id,
            user_id,
            ip,
            country,
            req.version_label.as_deref(),
            None,
        )
        .await
    }

    pub async fn project_view_stats(
        state: &AppState,
        project_id: Uuid,
        from: &str,
        to: &str,
    ) -> AppResult<Vec<ProjectDailyStats>> {
        AnalyticsRepo::project_view_stats(&state.db, project_id, from, to).await
    }

    pub async fn engagement_summary(
        state: &AppState,
        entity_id: Uuid,
    ) -> AppResult<EngagementSummary> {
        AnalyticsRepo::engagement_summary(&state.db, entity_id).await
    }

    pub async fn top_projects(
        state: &AppState,
        from: &str,
        to: &str,
    ) -> AppResult<Vec<TopProject>> {
        crate::repositories::ProjectRepo::top_downloads(&state.db, from, to, 20).await
    }
}
