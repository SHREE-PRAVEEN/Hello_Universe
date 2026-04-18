use crate::{
    config::AppState,
    integrations::openai::OpenAiClient,
    repositories::SearchRepo,
    utils::errors::AppResult,
};
use tracing::info;

/// Find recently published projects with no AI tags and tag them
pub async fn tag_pending(state: &AppState) -> AppResult<()> {
    // Projects published in last 7 days without AI tags
    let projects = sqlx::query!(
        r#"SELECT p.id, p.title,
           COALESCE(p.short_description,'') AS short_description,
           COALESCE(p.description,'') AS description
           FROM projects p
           WHERE p.status = 'published'
             AND p.deleted_at IS NULL
             AND NOT EXISTS (
               SELECT 1 FROM ai_tags a
               WHERE a.entity_type = 'project' AND a.entity_id = p.id
             )
           ORDER BY p.published_at DESC
           LIMIT 10"#
    )
    .fetch_all(&state.db)
    .await?;

    if projects.is_empty() {
        return Ok(());
    }

    info!("AI tagging {} projects", projects.len());

    let client = OpenAiClient::new(&state.config.openai_api_key, &state.config.openai_model);

    for p in &projects {
        let content = format!(
            "Title: {}\nSummary: {}\nDescription: {}",
            p.title,
            p.short_description,
            &p.description[..p.description.len().min(1500)]
        );

        match client.extract_tags(&content).await {
            Ok(tags) => {
                if let Err(e) = SearchRepo::upsert_ai_tags(
                    &state.db,
                    "project",
                    p.id,
                    &tags,
                    &state.config.openai_model,
                )
                .await
                {
                    tracing::warn!(project_id = %p.id, error = %e, "Failed to store AI tags");
                } else {
                    info!(project_id = %p.id, tag_count = tags.len(), "AI tags stored");
                }
            }
            Err(e) => {
                tracing::warn!(project_id = %p.id, error = %e, "AI tagging failed");
            }
        }

        // Respect OpenAI rate limits
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}
