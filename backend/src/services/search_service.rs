use crate::{
    config::AppState,
    models::ai::{SearchRequest, SearchResponse},
    repositories::{AnalyticsRepo, SearchRepo},
    utils::errors::AppResult,
};
use std::time::Instant;
use uuid::Uuid;

pub struct SearchService;

impl SearchService {
    pub async fn search(
        state: &AppState,
        req: &SearchRequest,
        user_id: Option<Uuid>,
    ) -> AppResult<SearchResponse> {
        let start = Instant::now();
        let limit = req.per_page.unwrap_or(20).min(100);
        let offset = (req.page.unwrap_or(1) - 1) * limit;

        let entity_types = req.entity_types.as_deref();

        let (results, total) =
            SearchRepo::full_text_search(&state.db, &req.q, entity_types, limit, offset).await?;

        let latency_ms = start.elapsed().as_millis() as u64;

        // Log query asynchronously
        let _ = AnalyticsRepo::log_search_query(
            &state.db,
            user_id,
            &req.q,
            Some(total as i32),
            Some(latency_ms as i32),
        )
        .await;

        Ok(SearchResponse {
            results,
            total,
            query: req.q.clone(),
            latency_ms,
        })
    }

    pub async fn autocomplete(state: &AppState, prefix: &str) -> AppResult<Vec<String>> {
        SearchRepo::autocomplete_tags(&state.db, prefix, 10).await
    }

    pub async fn popular_searches(state: &AppState) -> AppResult<Vec<String>> {
        SearchRepo::popular_searches(&state.db, 10).await
    }

    pub async fn reindex_project(state: &AppState, project_id: Uuid) -> AppResult<()> {
        let project = sqlx::query!(
            "SELECT title, short_description, description FROM projects WHERE id=$1",
            project_id
        )
        .fetch_optional(&state.db)
        .await?;

        if let Some(p) = project {
            let tags: Vec<String> = sqlx::query_scalar!(
                "SELECT t.name FROM tags t JOIN project_tags pt ON pt.tag_id=t.id WHERE pt.project_id=$1",
                project_id
            )
            .fetch_all(&state.db)
            .await?;

            let body = format!(
                "{} {}",
                p.short_description.as_deref().unwrap_or(""),
                p.description.as_deref().unwrap_or("")
            );

            SearchRepo::upsert_search_index(
                &state.db,
                "project",
                project_id,
                &p.title,
                Some(&body),
                Some(&tags),
                &serde_json::json!({"type": "project"}),
            )
            .await?;
        }
        Ok(())
    }
}
