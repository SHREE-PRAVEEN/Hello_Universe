use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::{ApiError, ApiResponse};
use crate::middleware::AuthenticatedUser;
use crate::services::crypto_services::{BlockchainService, WalletVerification, SignatureVerifyRequest};
use crate::models::transaction::{Transaction, CreatePaymentRequest, PaymentResponse};

/// Request to get a nonce for wallet signature
#[derive(Debug, Deserialize)]
pub struct NonceRequest {
    pub address: String,
}

/// Request to link wallet to account
#[derive(Debug, Deserialize)]
pub struct LinkWalletRequest {
    pub address: String,
    pub signature: String,
    pub message: String,
}

/// Transaction query parameters
#[derive(Debug, Deserialize)]
pub struct TransactionQuery {
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Get nonce for wallet authentication
pub async fn get_nonce(
    body: web::Json<NonceRequest>,
) -> Result<HttpResponse, ApiError> {
    let blockchain_service = BlockchainService::new();
    
    if !BlockchainService::is_valid_eth_address(&body.address) {
        return Err(ApiError::ValidationError("Invalid Ethereum address".to_string()));
    }

    let nonce = BlockchainService::generate_nonce();
    let message = BlockchainService::generate_sign_message(&nonce);

    Ok(ApiResponse::success(WalletVerification {
        address: body.address.clone(),
        message,
        nonce,
    }))
}

/// Verify wallet signature
pub async fn verify_signature(
    body: web::Json<SignatureVerifyRequest>,
) -> Result<HttpResponse, ApiError> {
    let blockchain_service = BlockchainService::new();
    
    let is_valid = blockchain_service.verify_signature(
        &body.message,
        &body.signature,
        &body.address,
    )?;

    Ok(ApiResponse::success(serde_json::json!({
        "valid": is_valid,
        "address": body.address
    })))
}

/// Link wallet to user account
pub async fn link_wallet(
    pool: web::Data<Arc<PgPool>>,
    user: AuthenticatedUser,
    body: web::Json<LinkWalletRequest>,
) -> Result<HttpResponse, ApiError> {
    let blockchain_service = BlockchainService::new();
    
    // Verify the signature
    let is_valid = blockchain_service.verify_signature(
        &body.message,
        &body.signature,
        &body.address,
    )?;

    if !is_valid {
        return Err(ApiError::Unauthorized("Invalid wallet signature".to_string()));
    }

    // Check if wallet is already linked to another account
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM users WHERE wallet_address = $1 AND id != $2"
    )
    .bind(&body.address)
    .bind(user.user_id)
    .fetch_one(pool.get_ref().as_ref())
    .await?;

    if existing > 0 {
        return Err(ApiError::Conflict("Wallet already linked to another account".to_string()));
    }

    // Update user's wallet address
    sqlx::query(
        "UPDATE users SET wallet_address = $1, updated_at = NOW() WHERE id = $2"
    )
    .bind(&body.address)
    .bind(user.user_id)
    .execute(pool.get_ref().as_ref())
    .await?;

    log::info!("Wallet {} linked to user {}", body.address, user.user_id);

    Ok(ApiResponse::success(serde_json::json!({
        "message": "Wallet linked successfully",
        "address": body.address
    })))
}

/// Get user's transactions
pub async fn get_transactions(
    pool: web::Data<Arc<PgPool>>,
    user: AuthenticatedUser,
    query: web::Query<TransactionQuery>,
) -> Result<HttpResponse, ApiError> {
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);

    let transactions = match &query.status {
        Some(status) => {
            sqlx::query_as::<_, Transaction>(
                "SELECT * FROM transactions WHERE user_id = $1 AND status = $2 ORDER BY created_at DESC LIMIT $3 OFFSET $4"
            )
            .bind(user.user_id)
            .bind(status)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool.get_ref().as_ref())
            .await?
        }
        None => {
            sqlx::query_as::<_, Transaction>(
                "SELECT * FROM transactions WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
            )
            .bind(user.user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool.get_ref().as_ref())
            .await?
        }
    };

    Ok(ApiResponse::success(serde_json::json!({
        "transactions": transactions,
        "count": transactions.len(),
        "limit": limit,
        "offset": offset
    })))
}

/// Create a new payment
pub async fn create_payment(
    pool: web::Data<Arc<PgPool>>,
    user: AuthenticatedUser,
    body: web::Json<CreatePaymentRequest>,
) -> Result<HttpResponse, ApiError> {
    // Validate payment method
    let valid_methods = ["stripe", "razorpay", "crypto"];
    if !valid_methods.contains(&body.payment_method.as_str()) {
        return Err(ApiError::ValidationError("Invalid payment method".to_string()));
    }

    // Get product price (in production, fetch from database or config)
    let amount = 1.60; // $1.60 USD
    let currency = "USD";
    
    // Generate payment ID
    let payment_id = format!("pay_{}", Uuid::new_v4().to_string().replace("-", ""));

    // Create transaction record
    let transaction = sqlx::query_as::<_, Transaction>(
        r#"
        INSERT INTO transactions (user_id, amount, currency, payment_method, payment_id, status, product_type)
        VALUES ($1, $2, $3, $4, $5, 'pending', $6)
        RETURNING *
        "#
    )
    .bind(user.user_id)
    .bind(amount)
    .bind(currency)
    .bind(&body.payment_method)
    .bind(&payment_id)
    .bind(&body.product_type)
    .fetch_one(pool.get_ref().as_ref())
    .await?;

    log::info!("Payment created: {} for user {}", payment_id, user.user_id);

    Ok(ApiResponse::created(PaymentResponse {
        payment_id,
        client_secret: Some(format!("cs_{}", Uuid::new_v4())), // In production, get from payment provider
        amount,
        currency: currency.to_string(),
    }))
}

/// Verify transaction status
pub async fn verify_transaction(
    pool: web::Data<Arc<PgPool>>,
    user: AuthenticatedUser,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let tx_hash = path.into_inner();
    let blockchain_service = BlockchainService::new();

    let status = blockchain_service.verify_transaction(&tx_hash).await?;

    // Update transaction in database if found
    sqlx::query(
        "UPDATE transactions SET blockchain_tx_hash = $1 WHERE user_id = $2 AND blockchain_tx_hash IS NULL"
    )
    .bind(&tx_hash)
    .bind(user.user_id)
    .execute(pool.get_ref().as_ref())
    .await
    .ok(); // Ignore errors

    Ok(ApiResponse::success(status))
}

/// Get token balance
pub async fn get_balance(
    user: AuthenticatedUser,
    pool: web::Data<Arc<PgPool>>,
) -> Result<HttpResponse, ApiError> {
    // Get user's wallet address
    let wallet_address: Option<String> = sqlx::query_scalar(
        "SELECT wallet_address FROM users WHERE id = $1"
    )
    .bind(user.user_id)
    .fetch_optional(pool.get_ref().as_ref())
    .await?
    .flatten();

    match wallet_address {
        Some(address) => {
            let blockchain_service = BlockchainService::new();
            let balance = blockchain_service.get_token_balance(&address).await?;
            Ok(ApiResponse::success(balance))
        }
        None => {
            Ok(ApiResponse::success(serde_json::json!({
                "message": "No wallet linked",
                "balance": null
            })))
        }
    }
}

/// Blockchain service health check
pub async fn health_check() -> HttpResponse {
    let blockchain_service = BlockchainService::new();
    
    HttpResponse::Ok().json(serde_json::json!({
        "service": "blockchain",
        "status": if blockchain_service.is_configured() { "available" } else { "not_configured" },
        "features": {
            "wallet_verification": true,
            "transactions": blockchain_service.is_configured(),
            "token_balance": blockchain_service.is_configured()
        }
    }))
}
