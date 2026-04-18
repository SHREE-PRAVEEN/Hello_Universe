use axum::{extract::{Path, Query, State}, Json};
use uuid::Uuid;
use crate::{
    config::AppState,
    middleware::{rbac, AuthUser},
    models::commerce::*,
    services::CommerceService,
    utils::errors::AppResult,
};

pub async fn list_plans(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<Plan>>> {
    let plans = CommerceService::list_plans(&state).await?;
    Ok(Json(plans))
}

pub async fn get_my_subscription(
    State(state): State<AppState>,
    user: AuthUser,
) -> AppResult<Json<Option<Subscription>>> {
    let sub = CommerceService::get_active_subscription(&state, user.id()).await?;
    Ok(Json(sub))
}

pub async fn subscribe(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<CreateSubscriptionRequest>,
) -> AppResult<Json<Subscription>> {
    let sub = CommerceService::subscribe(&state, user.id(), &req).await?;
    Ok(Json(sub))
}

pub async fn cancel_subscription(
    State(state): State<AppState>,
    user: AuthUser,
) -> AppResult<Json<serde_json::Value>> {
    CommerceService::cancel_subscription(&state, user.id()).await?;
    Ok(Json(serde_json::json!({ "message": "Subscription cancelled" })))
}

pub async fn initiate_purchase(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<CreatePurchaseRequest>,
) -> AppResult<Json<CheckoutSession>> {
    let session = CommerceService::initiate_purchase(&state, user.id(), &req).await?;
    Ok(Json(session))
}

pub async fn complete_purchase(
    State(state): State<AppState>,
    Path(purchase_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    CommerceService::complete_purchase(&state, purchase_id).await?;
    Ok(Json(serde_json::json!({ "message": "Purchase completed" })))
}

pub async fn my_purchases(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> AppResult<Json<Vec<Purchase>>> {
    let page = params.get("page").and_then(|v| v.parse().ok()).unwrap_or(1);
    let per_page = params.get("per_page").and_then(|v| v.parse().ok()).unwrap_or(20);
    let purchases = CommerceService::user_purchases(&state, user.id(), page, per_page).await?;
    Ok(Json(purchases))
}

pub async fn check_access(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> AppResult<Json<serde_json::Value>> {
    let entity_type = params.get("entity_type").map(|s| s.as_str()).unwrap_or("project");
    let entity_id: Uuid = params.get("entity_id")
        .and_then(|v| Uuid::parse_str(v).ok())
        .ok_or_else(|| crate::utils::errors::AppError::BadRequest("entity_id required".into()))?;
    let has_access = CommerceService::check_access(&state, user.id(), entity_type, entity_id).await?;
    Ok(Json(serde_json::json!({ "has_access": has_access })))
}

pub async fn revenue_report(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> AppResult<Json<Vec<RevenueReport>>> {
    rbac::require_admin(&user)?;
    let from = params.get("from").map(|s| s.as_str()).unwrap_or("2024-01-01");
    let to = params.get("to").map(|s| s.as_str()).unwrap_or("2099-12-31");
    let report = CommerceService::revenue_report(&state, from, to).await?;
    Ok(Json(report))
}
