use crate::{
    config::AppState,
    integrations::openai::OpenAiClient,
    models::ai::*,
    repositories::SearchRepo,
    utils::errors::AppResult,
};
use uuid::Uuid;

pub struct AiService;

impl AiService {
    pub async fn auto_tag_project(state: &AppState, project_id: Uuid) -> AppResult<AiTagResult> {
        let project = sqlx::query!(
            "SELECT title, short_description, description FROM projects WHERE id=$1",
            project_id
        )
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| crate::utils::errors::AppError::not_found("Project"))?;

        let content = format!(
            "Title: {}\nDescription: {}\n{}",
            project.title,
            project.short_description.as_deref().unwrap_or(""),
            project.description.as_deref().unwrap_or("")
        );

        Self::generate_and_store_tags(state, "project", project_id, &content).await
    }

    pub async fn auto_tag_media(state: &AppState, media_id: Uuid) -> AppResult<AiTagResult> {
        let media = sqlx::query!(
            "SELECT original_filename, display_name, description, mime_type FROM media_files WHERE id=$1",
            media_id
        )
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| crate::utils::errors::AppError::not_found("Media file"))?;

        let content = format!(
            "Filename: {}\nDisplay name: {}\nType: {}\nDescription: {}",
            media.original_filename,
            media.display_name.as_deref().unwrap_or(""),
            media.mime_type,
            media.description.as_deref().unwrap_or("")
        );

        Self::generate_and_store_tags(state, "media_file", media_id, &content).await
    }

    async fn generate_and_store_tags(
        state: &AppState,
        entity_type: &str,
        entity_id: Uuid,
        content: &str,
    ) -> AppResult<AiTagResult> {
        let client = OpenAiClient::new(&state.config.openai_api_key, &state.config.openai_model);
        let tags = client.extract_tags(content).await?;

        SearchRepo::upsert_ai_tags(&state.db, entity_type, entity_id, &tags, &state.config.openai_model)
            .await?;

        Ok(AiTagResult {
            entity_id,
            entity_type: entity_type.to_string(),
            tags,
            model: state.config.openai_model.clone(),
        })
    }

    pub async fn get_tags(state: &AppState, entity_type: &str, entity_id: Uuid) -> AppResult<Vec<AiTag>> {
        SearchRepo::get_ai_tags(&state.db, entity_type, entity_id).await
    }
}
