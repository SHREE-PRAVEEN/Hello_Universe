use crate::{
    config::AppState,
    models::moderation::*,
    repositories::ModerationRepo,
    utils::errors::AppResult,
};
use uuid::Uuid;

pub struct ModerationService;

impl ModerationService {
    pub async fn list_queue(state: &AppState, page: i64, per_page: i64) -> AppResult<Vec<ModerationQueueItem>> {
        let limit = per_page.min(50);
        let offset = (page - 1) * limit;
        ModerationRepo::list_pending(&state.db, limit, offset).await
    }

    pub async fn decide(
        state: &AppState,
        queue_id: Uuid,
        moderator_id: Uuid,
        req: &ModerationDecisionRequest,
    ) -> AppResult<ModerationAction> {
        let queue_item = sqlx::query!(
            "SELECT entity_type, entity_id, status FROM moderation_queue WHERE id=$1",
            queue_id
        )
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| crate::utils::errors::AppError::not_found("Queue item"))?;

        let new_status = match req.action.as_str() {
            "approve" => "approved",
            "reject" => "rejected",
            "flag" => "flagged",
            "escalate" => "escalated",
            _ => return Err(crate::utils::errors::AppError::BadRequest("Invalid action".into())),
        };

        // Apply status to the underlying entity (project / media_file)
        match queue_item.entity_type.as_str() {
            "project" => {
                sqlx::query!(
                    "UPDATE projects SET status=$2 WHERE id=$1",
                    queue_item.entity_id, new_status
                )
                .execute(&state.db)
                .await?;
            }
            "media_file" => {
                // Media files don't have a status column but we note it in action
            }
            _ => {}
        }

        ModerationRepo::take_action(
            &state.db,
            queue_id,
            moderator_id,
            &queue_item.entity_type,
            queue_item.entity_id,
            &req.action,
            req.reason.as_deref(),
            req.rejection_reason.as_deref(),
            Some(&queue_item.status),
            Some(new_status),
        )
        .await
    }

    pub async fn get_notifications(
        state: &AppState,
        user_id: Uuid,
        unread_only: bool,
    ) -> AppResult<Vec<Notification>> {
        ModerationRepo::get_user_notifications(&state.db, user_id, unread_only, 50).await
    }

    pub async fn mark_read(state: &AppState, notification_id: Uuid, user_id: Uuid) -> AppResult<()> {
        ModerationRepo::mark_notification_read(&state.db, notification_id, user_id).await
    }
}
