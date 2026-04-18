use crate::models::moderation::*;
use sqlx::PgPool;
use uuid::Uuid;
use crate::utils::errors::{AppError, AppResult};

pub struct ModerationRepo;

impl ModerationRepo {
    pub async fn enqueue(
        pool: &PgPool,
        entity_type: &str,
        entity_id: Uuid,
        reason: &str,
        priority: i16,
    ) -> AppResult<ModerationQueue> {
        Ok(sqlx::query_as!(
            ModerationQueue,
            r#"INSERT INTO moderation_queue (entity_type, entity_id, queue_reason, priority)
               VALUES ($1, $2, $3, $4)
               RETURNING id, entity_type, entity_id, queue_reason, priority,
               assigned_to, status, notes, created_at, updated_at"#,
            entity_type, entity_id, reason, priority
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn list_pending(pool: &PgPool, limit: i64, offset: i64) -> AppResult<Vec<ModerationQueueItem>> {
        Ok(sqlx::query_as!(
            ModerationQueueItem,
            r#"SELECT mq.id, mq.entity_type, mq.entity_id, mq.queue_reason,
               mq.priority, mq.status, mq.assigned_to,
               u.username AS assignee_username, mq.created_at
               FROM moderation_queue mq
               LEFT JOIN users u ON u.id = mq.assigned_to
               WHERE mq.status = 'pending'
               ORDER BY mq.priority DESC, mq.created_at ASC
               LIMIT $1 OFFSET $2"#,
            limit, offset
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn take_action(
        pool: &PgPool,
        queue_id: Uuid,
        moderator_id: Uuid,
        entity_type: &str,
        entity_id: Uuid,
        action: &str,
        reason: Option<&str>,
        rejection_reason: Option<&str>,
        prev_status: Option<&str>,
        new_status: Option<&str>,
    ) -> AppResult<ModerationAction> {
        // Update queue item
        sqlx::query!(
            "UPDATE moderation_queue SET status = $2, updated_at = NOW() WHERE id = $1",
            queue_id,
            if action == "escalate" { "escalated" } else { "approved" }
        )
        .execute(pool)
        .await?;

        Ok(sqlx::query_as!(
            ModerationAction,
            r#"INSERT INTO moderation_actions
               (queue_id, moderator_id, entity_type, entity_id, action, reason,
                rejection_reason, previous_status, new_status)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
               RETURNING id, queue_id, moderator_id, entity_type, entity_id, action,
               reason, rejection_reason, previous_status, new_status, metadata, created_at"#,
            queue_id, moderator_id, entity_type, entity_id, action,
            reason, rejection_reason, prev_status, new_status
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn insert_audit_log(
        pool: &PgPool,
        actor_id: Option<Uuid>,
        actor_role: Option<&str>,
        org_id: Option<Uuid>,
        action: &str,
        entity_type: Option<&str>,
        entity_id: Option<Uuid>,
        old_state: Option<&serde_json::Value>,
        new_state: Option<&serde_json::Value>,
        ip: Option<&str>,
        request_id: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"INSERT INTO audit_logs
               (actor_id, actor_role, organization_id, action, entity_type, entity_id,
                old_state, new_state, ip_address, request_id)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9::inet,$10)"#,
            actor_id as _, actor_role, org_id as _, action, entity_type, entity_id as _,
            old_state as _, new_state as _, ip, request_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn create_notification(
        pool: &PgPool,
        user_id: Uuid,
        notification_type: &str,
        title: &str,
        body: Option<&str>,
        entity_type: Option<&str>,
        entity_id: Option<Uuid>,
        action_url: Option<&str>,
    ) -> AppResult<Notification> {
        Ok(sqlx::query_as!(
            Notification,
            r#"INSERT INTO notifications
               (user_id, type, title, body, entity_type, entity_id, action_url)
               VALUES ($1,$2,$3,$4,$5,$6,$7)
               RETURNING id, user_id, type AS notification_type, title, body,
               entity_type, entity_id, action_url, is_read, read_at, sent_via, created_at"#,
            user_id, notification_type, title, body,
            entity_type, entity_id as _, action_url
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_user_notifications(
        pool: &PgPool,
        user_id: Uuid,
        unread_only: bool,
        limit: i64,
    ) -> AppResult<Vec<Notification>> {
        Ok(sqlx::query_as!(
            Notification,
            r#"SELECT id, user_id, type AS notification_type, title, body,
               entity_type, entity_id, action_url, is_read, read_at, sent_via, created_at
               FROM notifications
               WHERE user_id = $1 AND ($2 = FALSE OR is_read = FALSE)
               ORDER BY created_at DESC LIMIT $3"#,
            user_id, unread_only, limit
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn mark_notification_read(pool: &PgPool, id: Uuid, user_id: Uuid) -> AppResult<()> {
        sqlx::query!(
            "UPDATE notifications SET is_read = TRUE, read_at = NOW()
             WHERE id = $1 AND user_id = $2",
            id, user_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
