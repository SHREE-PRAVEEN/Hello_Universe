use crate::{
    config::AppState,
    models::session::TokenPair,
    repositories::{AuthRepo, UserRepo},
    utils::{
        crypto::{generate_secure_token, hash_password, hash_token, verify_password},
        errors::{AppError, AppResult},
        jwt::JwtManager,
    },
};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

pub struct AuthService;

impl AuthService {
    pub async fn register(
        state: &AppState,
        username: &str,
        email: &str,
        password: &str,
        display_name: Option<&str>,
        ip: Option<&str>,
    ) -> AppResult<TokenPair> {
        // Check duplicates
        if UserRepo::find_by_email(&state.db, email).await?.is_some() {
            return Err(AppError::Conflict("Email already registered".into()));
        }
        if UserRepo::find_by_username(&state.db, username).await?.is_some() {
            return Err(AppError::Conflict("Username already taken".into()));
        }

        let hash = hash_password(password)?;
        let user = UserRepo::create(&state.db, username, email, &hash, display_name).await?;

        // Issue email verification token
        let raw_token = generate_secure_token();
        let token_hash = hash_token(&raw_token);
        let expires = OffsetDateTime::now_utc() + Duration::hours(24);
        AuthRepo::create_auth_token(&state.db, user.id, &token_hash, "email_verify", expires).await?;

        // TODO: dispatch email via notification service
        tracing::info!(user_id = %user.id, "Verification email token created");

        // Build session + JWT
        Self::build_token_pair(state, user.id, email, username, ip).await
    }

    pub async fn login(
        state: &AppState,
        email: &str,
        password: &str,
        ip: Option<&str>,
        user_agent: Option<&str>,
    ) -> AppResult<TokenPair> {
        let auth_user = UserRepo::find_auth_user(&state.db, email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid email or password".into()))?;

        // Load password hash
        let user = UserRepo::find_by_email(&state.db, email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid email or password".into()))?;

        if !verify_password(password, &user.password_hash)? {
            return Err(AppError::Unauthorized("Invalid email or password".into()));
        }

        if auth_user.status == "suspended" || auth_user.status == "banned" {
            return Err(AppError::Forbidden(format!("Account is {}", auth_user.status)));
        }

        UserRepo::update_last_login(&state.db, user.id, ip.unwrap_or("unknown")).await?;

        // Extract roles and permissions from JSON
        let roles = Self::extract_string_array(&auth_user.roles);
        let permissions = Self::extract_string_array(&auth_user.permissions);

        let jwt = JwtManager::new(
            &state.config.jwt_secret,
            state.config.jwt_access_token_expiry_seconds,
            state.config.jwt_refresh_token_expiry_days,
        );

        let access_token = jwt.issue_access_token(user.id, email, &user.username, roles, permissions)?;
        let raw_refresh = generate_secure_token();
        let refresh_hash = hash_token(&raw_refresh);

        let session_expires = OffsetDateTime::now_utc()
            + Duration::days(state.config.jwt_refresh_token_expiry_days);

        let session = AuthRepo::create_session(
            &state.db,
            user.id,
            &refresh_hash,
            user_agent,
            ip,
            session_expires,
        )
        .await?;

        let refresh_token = jwt.issue_refresh_token(user.id, session.id)?;

        Ok(TokenPair::new(
            access_token,
            refresh_token,
            state.config.jwt_access_token_expiry_seconds,
        ))
    }

    pub async fn refresh(state: &AppState, refresh_token: &str) -> AppResult<TokenPair> {
        let jwt = JwtManager::new(
            &state.config.jwt_secret,
            state.config.jwt_access_token_expiry_seconds,
            state.config.jwt_refresh_token_expiry_days,
        );

        let claims = jwt.verify_refresh_token(refresh_token)?;
        let session_id = Uuid::parse_str(&claims.session_id)
            .map_err(|_| AppError::Unauthorized("Invalid session".into()))?;
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("Invalid token subject".into()))?;

        // Check session is still valid
        let raw_refresh_hash = hash_token(refresh_token);
        let session = AuthRepo::find_session_by_token_hash(&state.db, &raw_refresh_hash)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Session expired or revoked".into()))?;

        if session.user_id != user_id {
            return Err(AppError::Unauthorized("Token mismatch".into()));
        }

        AuthRepo::touch_session(&state.db, session_id).await?;

        let auth_user = UserRepo::find_auth_user(&state.db, "")
            .await?;

        let user = UserRepo::find_by_id(&state.db, user_id).await?;
        let auth = UserRepo::find_auth_user(&state.db, &user.email).await?
            .ok_or_else(|| AppError::Unauthorized("User not found".into()))?;

        let roles = Self::extract_string_array(&auth.roles);
        let permissions = Self::extract_string_array(&auth.permissions);

        let access_token = jwt.issue_access_token(
            user_id,
            &user.email,
            &user.username,
            roles,
            permissions,
        )?;

        let new_raw_refresh = generate_secure_token();
        let new_refresh_hash = hash_token(&new_raw_refresh);
        let session_expires = OffsetDateTime::now_utc()
            + Duration::days(state.config.jwt_refresh_token_expiry_days);

        // Revoke old session, create new one (refresh token rotation)
        AuthRepo::revoke_session(&state.db, session_id).await?;
        let new_session = AuthRepo::create_session(
            &state.db,
            user_id,
            &new_refresh_hash,
            session.user_agent.as_deref(),
            session.ip_address.as_deref(),
            session_expires,
        )
        .await?;

        let new_refresh_token = jwt.issue_refresh_token(user_id, new_session.id)?;

        Ok(TokenPair::new(
            access_token,
            new_refresh_token,
            state.config.jwt_access_token_expiry_seconds,
        ))
    }

    pub async fn logout(state: &AppState, refresh_token: &str) -> AppResult<()> {
        let raw_hash = hash_token(refresh_token);
        if let Some(session) = AuthRepo::find_session_by_token_hash(&state.db, &raw_hash).await? {
            AuthRepo::revoke_session(&state.db, session.id).await?;
        }
        Ok(())
    }

    pub async fn logout_all(state: &AppState, user_id: Uuid) -> AppResult<()> {
        AuthRepo::revoke_all_user_sessions(&state.db, user_id).await
    }

    pub async fn verify_email(state: &AppState, token: &str) -> AppResult<()> {
        let token_hash = hash_token(token);
        let record = AuthRepo::consume_auth_token(&state.db, &token_hash, "email_verify")
            .await?
            .ok_or_else(|| AppError::BadRequest("Invalid or expired verification token".into()))?;
        UserRepo::mark_email_verified(&state.db, record.user_id).await
    }

    pub async fn request_password_reset(state: &AppState, email: &str) -> AppResult<()> {
        // Always return OK to prevent email enumeration
        if let Some(user) = UserRepo::find_by_email(&state.db, email).await? {
            let raw_token = generate_secure_token();
            let token_hash = hash_token(&raw_token);
            let expires = OffsetDateTime::now_utc() + Duration::hours(2);
            AuthRepo::create_auth_token(&state.db, user.id, &token_hash, "password_reset", expires)
                .await?;
            tracing::info!(user_id = %user.id, "Password reset token created");
            // TODO: dispatch email
        }
        Ok(())
    }

    pub async fn reset_password(state: &AppState, token: &str, new_password: &str) -> AppResult<()> {
        let token_hash = hash_token(token);
        let record = AuthRepo::consume_auth_token(&state.db, &token_hash, "password_reset")
            .await?
            .ok_or_else(|| AppError::BadRequest("Invalid or expired reset token".into()))?;
        let new_hash = hash_password(new_password)?;
        UserRepo::update_password(&state.db, record.user_id, &new_hash).await?;
        AuthRepo::revoke_all_user_sessions(&state.db, record.user_id).await
    }

    // ---- Helpers ----
    async fn build_token_pair(
        state: &AppState,
        user_id: Uuid,
        email: &str,
        username: &str,
        ip: Option<&str>,
    ) -> AppResult<TokenPair> {
        let jwt = JwtManager::new(
            &state.config.jwt_secret,
            state.config.jwt_access_token_expiry_seconds,
            state.config.jwt_refresh_token_expiry_days,
        );
        let access_token =
            jwt.issue_access_token(user_id, email, username, vec![], vec![])?;
        let raw_refresh = generate_secure_token();
        let refresh_hash = hash_token(&raw_refresh);
        let expires = OffsetDateTime::now_utc()
            + Duration::days(state.config.jwt_refresh_token_expiry_days);
        let session =
            AuthRepo::create_session(&state.db, user_id, &refresh_hash, None, ip, expires).await?;
        let refresh_token = jwt.issue_refresh_token(user_id, session.id)?;
        Ok(TokenPair::new(
            access_token,
            refresh_token,
            state.config.jwt_access_token_expiry_seconds,
        ))
    }

    fn extract_string_array(val: &Option<serde_json::Value>) -> Vec<String> {
        val.as_ref()
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    }
}
