use axum::{
    extract::{ConnectInfo, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use redis::AsyncCommands;
use std::net::SocketAddr;
use crate::config::AppState;

pub async fn rate_limit_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    let ip = req
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ci| ci.0.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let key = format!("rate_limit:{}", ip);
    let limit = state.config.rate_limit_requests_per_minute as i64;

    let mut redis = state.redis.clone();

    let count: i64 = redis.incr(&key, 1).await.unwrap_or(1);
    if count == 1 {
        let _: () = redis.expire(&key, 60).await.unwrap_or(());
    }

    if count > limit {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            axum::Json(serde_json::json!({
                "error": {
                    "code": "RATE_LIMITED",
                    "message": "Too many requests. Please try again later."
                }
            })),
        )
            .into_response();
    }

    next.run(req).await
}
