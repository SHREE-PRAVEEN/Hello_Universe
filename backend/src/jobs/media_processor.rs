use crate::{
    config::AppState,
    repositories::MediaRepo,
    utils::errors::AppResult,
};
use tracing::info;

/// Pick up unprocessed media files and extract metadata
pub async fn process_pending(state: &AppState) -> AppResult<()> {
    let pending = sqlx::query!(
        r#"SELECT id, media_type, mime_type, original_filename
           FROM media_files
           WHERE is_processed = FALSE
             AND processing_error IS NULL
             AND deleted_at IS NULL
           LIMIT 20"#
    )
    .fetch_all(&state.db)
    .await?;

    if pending.is_empty() {
        return Ok(());
    }

    info!("Processing {} media files", pending.len());

    for row in pending {
        let result = process_one(state, row.id, &row.media_type, &row.mime_type).await;
        match result {
            Ok(_) => {
                MediaRepo::mark_processed(&state.db, row.id).await?;
                info!(media_id = %row.id, "Media processed");
            }
            Err(e) => {
                let err_str = e.to_string();
                MediaRepo::mark_processing_failed(&state.db, row.id, &err_str).await?;
                tracing::warn!(media_id = %row.id, error = %err_str, "Media processing failed");
            }
        }
    }

    Ok(())
}

async fn process_one(
    state: &AppState,
    media_id: uuid::Uuid,
    media_type: &str,
    mime_type: &str,
) -> AppResult<()> {
    match media_type {
        "image" => {
            // In production: download from S3, extract dimensions via `image` crate,
            // generate thumbnail, re-upload thumbnail, update width_px/height_px
            sqlx::query!(
                "UPDATE media_files SET is_processed = TRUE WHERE id = $1",
                media_id
            )
            .execute(&state.db)
            .await?;
        }
        "video" => {
            // In production: trigger AWS MediaConvert job, store job ID in metadata,
            // poll status and update when complete
            sqlx::query!(
                "UPDATE media_files SET is_processed = TRUE WHERE id = $1",
                media_id
            )
            .execute(&state.db)
            .await?;
        }
        "pdf" | "document" => {
            // In production: extract page count via `lopdf` crate
            sqlx::query!(
                "UPDATE media_files SET is_processed = TRUE WHERE id = $1",
                media_id
            )
            .execute(&state.db)
            .await?;
        }
        _ => {
            // Mark as processed without further action
            sqlx::query!(
                "UPDATE media_files SET is_processed = TRUE WHERE id = $1",
                media_id
            )
            .execute(&state.db)
            .await?;
        }
    }
    Ok(())
}
