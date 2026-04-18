use crate::{
    config::AppState,
    repositories::ModerationRepo,
    utils::errors::AppResult,
};
use uuid::Uuid;

pub struct NotificationService;

impl NotificationService {
    pub async fn notify(
        state: &AppState,
        user_id: Uuid,
        notification_type: &str,
        title: &str,
        body: Option<&str>,
        entity_type: Option<&str>,
        entity_id: Option<Uuid>,
        action_url: Option<&str>,
    ) -> AppResult<()> {
        ModerationRepo::create_notification(
            &state.db,
            user_id,
            notification_type,
            title,
            body,
            entity_type,
            entity_id,
            action_url,
        )
        .await?;
        Ok(())
    }

    pub async fn notify_project_approved(state: &AppState, project_id: Uuid, owner_id: Uuid) -> AppResult<()> {
        Self::notify(
            state,
            owner_id,
            "system",
            "Your project has been approved",
            Some("Your project is now live on the platform."),
            Some("project"),
            Some(project_id),
            None,
        )
        .await
    }

    pub async fn notify_project_rejected(
        state: &AppState,
        project_id: Uuid,
        owner_id: Uuid,
        reason: &str,
    ) -> AppResult<()> {
        Self::notify(
            state,
            owner_id,
            "moderation",
            "Your project was not approved",
            Some(reason),
            Some("project"),
            Some(project_id),
            None,
        )
        .await
    }

    pub async fn notify_new_comment(
        state: &AppState,
        content_owner_id: Uuid,
        entity_type: &str,
        entity_id: Uuid,
        commenter_username: &str,
    ) -> AppResult<()> {
        Self::notify(
            state,
            content_owner_id,
            "comment",
            &format!("{} commented on your content", commenter_username),
            None,
            Some(entity_type),
            Some(entity_id),
            None,
        )
        .await
    }

    pub async fn notify_new_follower(state: &AppState, followed_id: Uuid, follower_username: &str) -> AppResult<()> {
        Self::notify(
            state,
            followed_id,
            "follow",
            &format!("{} started following you", follower_username),
            None,
            None,
            None,
            None,
        )
        .await
    }

    pub async fn notify_purchase_complete(
        state: &AppState,
        user_id: Uuid,
        entity_type: &str,
        entity_id: Uuid,
    ) -> AppResult<()> {
        Self::notify(
            state,
            user_id,
            "purchase",
            "Purchase successful",
            Some("You now have access to this content."),
            Some(entity_type),
            Some(entity_id),
            None,
        )
        .await
    }
}
