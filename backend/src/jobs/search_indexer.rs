use crate::{config::AppState, repositories::SearchRepo, utils::errors::AppResult};
use tracing::info;

pub async fn rebuild_all(state: &AppState) -> AppResult<()> {
    // Re-index all published projects
    let projects = sqlx::query!(
        r#"SELECT p.id, p.title,
           COALESCE(p.short_description,'') || ' ' || COALESCE(p.description,'') AS body
           FROM projects p
           WHERE p.status = 'published' AND p.deleted_at IS NULL"#
    )
    .fetch_all(&state.db)
    .await?;

    info!("Reindexing {} projects", projects.len());

    for p in projects {
        let tags: Vec<String> = sqlx::query_scalar!(
            "SELECT t.name FROM tags t JOIN project_tags pt ON pt.tag_id=t.id WHERE pt.project_id=$1",
            p.id
        )
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

        SearchRepo::upsert_search_index(
            &state.db,
            "project",
            p.id,
            &p.title,
            p.body.as_deref(),
            Some(&tags),
            &serde_json::json!({"type": "project"}),
        )
        .await?;
    }

    // Re-index users
    let users = sqlx::query!(
        "SELECT id, username, COALESCE(display_name,'') AS display_name, COALESCE(bio,'') AS bio
         FROM users WHERE status='active' AND deleted_at IS NULL"
    )
    .fetch_all(&state.db)
    .await?;

    info!("Reindexing {} users", users.len());

    for u in users {
        let body = format!("{} {}", u.display_name, u.bio);
        SearchRepo::upsert_search_index(
            &state.db,
            "user",
            u.id,
            &u.username,
            Some(&body),
            None,
            &serde_json::json!({"type": "user"}),
        )
        .await?;
    }

    info!("Search index rebuild complete");
    Ok(())
}
