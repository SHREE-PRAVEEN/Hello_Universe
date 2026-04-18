use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{
    config::AppState,
    middleware::AuthUser,
    models::session::TokenPair,
    services::AuthService,
    utils::errors::{AppError, AppResult},
};

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 80))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Deserialize, Validate)]
pub struct ResetPasswordRequest {
    pub token: String,
    #[validate(length(min = 8))]
    pub new_password: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<TokenPair>> {
    req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let tokens = AuthService::register(
        &state,
        &req.username,
        &req.email,
        &req.password,
        req.display_name.as_deref(),
        None,
    )
    .await?;
    Ok(Json(tokens))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<TokenPair>> {
    let tokens = AuthService::login(&state, &req.email, &req.password, None, None).await?;
    Ok(Json(tokens))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(req): Json<RefreshRequest>,
) -> AppResult<Json<TokenPair>> {
    let tokens = AuthService::refresh(&state, &req.refresh_token).await?;
    Ok(Json(tokens))
}

pub async fn logout(
    State(state): State<AppState>,
    Json(req): Json<LogoutRequest>,
) -> AppResult<Json<MessageResponse>> {
    AuthService::logout(&state, &req.refresh_token).await?;
    Ok(Json(MessageResponse { message: "Logged out successfully".into() }))
}

pub async fn logout_all(
    State(state): State<AppState>,
    user: AuthUser,
) -> AppResult<Json<MessageResponse>> {
    AuthService::logout_all(&state, user.id()).await?;
    Ok(Json(MessageResponse { message: "All sessions revoked".into() }))
}

pub async fn verify_email(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> AppResult<Json<MessageResponse>> {
    let token = params.get("token").ok_or_else(|| AppError::BadRequest("Missing token".into()))?;
    AuthService::verify_email(&state, token).await?;
    Ok(Json(MessageResponse { message: "Email verified successfully".into() }))
}

pub async fn request_password_reset(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> AppResult<Json<MessageResponse>> {
    let email = body["email"].as_str().ok_or_else(|| AppError::BadRequest("Missing email".into()))?;
    AuthService::request_password_reset(&state, email).await?;
    Ok(Json(MessageResponse { message: "If that email exists, a reset link has been sent.".into() }))
}

pub async fn reset_password(
    State(state): State<AppState>,
    Json(req): Json<ResetPasswordRequest>,
) -> AppResult<Json<MessageResponse>> {
    req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    AuthService::reset_password(&state, &req.token, &req.new_password).await?;
    Ok(Json(MessageResponse { message: "Password reset successfully".into() }))
}

pub async fn me(user: AuthUser) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "id": user.id(),
        "email": user.email(),
        "username": user.username(),
        "roles": user.roles(),
        "permissions": user.permissions(),
    }))
}
