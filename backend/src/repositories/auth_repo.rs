use crate::models::session::*;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::utils::errors::AppResult;

pub struct AuthRepo;

impl AuthRepo {
    pub async fn create_session(
        pool: &PgPool,
        user_id: Uuid,
        refresh_token_hash: &str,
        user_agent: Option<&str>,
        ip_address: Option<&str>,
        expires_at: OffsetDateTime,
    ) -> AppResult<Session> {
        Ok(sqlx::query_as!(
            Session,
            r#"INSERT INTO sessions (user_id, refresh_token, user_agent, ip_address, expires_at)
               VALUES ($1, $2, $3, $4, $5)
               RETURNING id, user_id, refresh_token, user_agent, ip_address, device_id,
               is_revoked, last_used_at, expires_at, created_at"#,
            user_id,
            refresh_token_hash,
            user_agent,
            ip_address,
            expires_at
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn find_session_by_token_hash(
        pool: &PgPool,
        token_hash: &str,
    ) -> AppResult<Option<Session>> {
        Ok(sqlx::query_as!(
            Session,
            r#"SELECT id, user_id, refresh_token, user_agent, ip_address, device_id,
               is_revoked, last_used_at, expires_at, created_at
               FROM sessions
               WHERE refresh_token = $1 AND is_revoked = FALSE AND expires_at > NOW()"#,
            token_hash
        )
        .fetch_optional(pool)
        .await?)
    }

    pub async fn revoke_session(pool: &PgPool, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            "UPDATE sessions SET is_revoked = TRUE WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn revoke_all_user_sessions(pool: &PgPool, user_id: Uuid) -> AppResult<()> {
        sqlx::query!(
            "UPDATE sessions SET is_revoked = TRUE WHERE user_id = $1",
            user_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn touch_session(pool: &PgPool, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            "UPDATE sessions SET last_used_at = NOW() WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn create_auth_token(
        pool: &PgPool,
        user_id: Uuid,
        token_hash: &str,
        token_type: &str,
        expires_at: OffsetDateTime,
    ) -> AppResult<AuthToken> {
        Ok(sqlx::query_as!(
            AuthToken,
            r#"INSERT INTO auth_tokens (user_id, token_hash, token_type, expires_at)
               VALUES ($1, $2, $3, $4)
               RETURNING id, user_id, token_hash, token_type, expires_at, used_at, created_at"#,
            user_id, token_hash, token_type, expires_at
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn consume_auth_token(pool: &PgPool, token_hash: &str, token_type: &str) -> AppResult<Option<AuthToken>> {
        Ok(sqlx::query_as!(
            AuthToken,
            r#"UPDATE auth_tokens SET used_at = NOW()
               WHERE token_hash = $1 AND token_type = $2
                 AND used_at IS NULL AND expires_at > NOW()
               RETURNING id, user_id, token_hash, token_type, expires_at, used_at, created_at"#,
            token_hash, token_type
        )
        .fetch_optional(pool)
        .await?)
    }
}
