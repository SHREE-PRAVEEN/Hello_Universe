use axum::{extract::{Path, State}, Json};
use uuid::Uuid;
use crate::{
    config::AppState,
    middleware::AuthUser,
    models::blockchain::*,
    services::BlockchainService,
    utils::errors::AppResult,
};

pub async fn verify(
    State(state): State<AppState>,
    Path((entity_type, entity_id)): Path<(String, Uuid)>,
    Json(body): Json<serde_json::Value>,
) -> AppResult<Json<VerificationResult>> {
    // Client provides file bytes as base64 for server-side verification
    let data_b64 = body["data_base64"].as_str()
        .ok_or_else(|| crate::utils::errors::AppError::BadRequest("data_base64 required".into()))?;
    let data = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        data_b64,
    ).map_err(|_| crate::utils::errors::AppError::BadRequest("Invalid base64".into()))?;

    let result = BlockchainService::verify(&state, &entity_type, entity_id, &data).await?;
    Ok(Json(result))
}

pub async fn record_ownership(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<TransferOwnershipRequest>,
) -> AppResult<Json<OwnershipRecord>> {
    let record = BlockchainService::record_ownership(
        &state,
        &req.entity_type,
        req.entity_id,
        req.to_user_id,
        req.to_org_id,
    )
    .await?;
    Ok(Json(record))
}

pub async fn anchor_transaction(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<AnchorToBlockchainRequest>,
) -> AppResult<Json<BlockchainTransaction>> {
    // Generate payload hash for what we're anchoring
    let payload = format!("{}:{}", req.entity_type, req.entity_id);
    let payload_hash = crate::utils::crypto::sha256_hex(payload.as_bytes());
    // In production: call blockchain integration to submit TX
    let tx_hash = format!("0x{}", crate::utils::crypto::generate_secure_token());
    let tx = BlockchainService::record_transaction(
        &state,
        &req.entity_type,
        req.entity_id,
        &req.network,
        &tx_hash,
        &payload_hash,
        user.id(),
    )
    .await?;
    Ok(Json(tx))
}
