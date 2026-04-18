use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Plan {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub price_monthly: Option<bigdecimal::BigDecimal>,
    pub price_yearly: Option<bigdecimal::BigDecimal>,
    pub currency: String,
    pub max_projects: Option<i32>,
    pub max_storage_gb: Option<i32>,
    pub max_collaborators: Option<i32>,
    pub features: serde_json::Value,
    pub is_active: bool,
    pub is_public: bool,
    pub sort_order: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Subscription {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub plan_id: Uuid,
    pub status: String,
    pub billing_cycle: String,
    pub current_period_start: OffsetDateTime,
    pub current_period_end: OffsetDateTime,
    pub trial_end_at: Option<OffsetDateTime>,
    pub cancelled_at: Option<OffsetDateTime>,
    pub cancel_reason: Option<String>,
    pub external_id: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Purchase {
    pub id: Uuid,
    pub user_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub amount: bigdecimal::BigDecimal,
    pub currency: String,
    pub status: String,
    pub payment_method: Option<String>,
    pub payment_intent_id: Option<String>,
    pub invoice_id: Option<Uuid>,
    pub discount_code: Option<String>,
    pub discount_amount: bigdecimal::BigDecimal,
    pub tax_amount: bigdecimal::BigDecimal,
    pub metadata: serde_json::Value,
    pub completed_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Invoice {
    pub id: Uuid,
    pub user_id: Uuid,
    pub invoice_number: String,
    pub subtotal: bigdecimal::BigDecimal,
    pub tax_amount: bigdecimal::BigDecimal,
    pub discount_amount: bigdecimal::BigDecimal,
    pub total: bigdecimal::BigDecimal,
    pub currency: String,
    pub status: String,
    pub issued_at: Option<OffsetDateTime>,
    pub due_at: Option<OffsetDateTime>,
    pub paid_at: Option<OffsetDateTime>,
    pub billing_address: serde_json::Value,
    pub line_items: serde_json::Value,
    pub pdf_url: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Refund {
    pub id: Uuid,
    pub purchase_id: Uuid,
    pub user_id: Uuid,
    pub amount: bigdecimal::BigDecimal,
    pub currency: String,
    pub reason: Option<String>,
    pub status: String,
    pub processed_by: Option<Uuid>,
    pub external_id: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ContentEntitlement {
    pub id: Uuid,
    pub user_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub entitlement_source: String,
    pub source_id: Option<Uuid>,
    pub granted_by: Option<Uuid>,
    pub granted_at: OffsetDateTime,
    pub expires_at: Option<OffsetDateTime>,
    pub revoked_at: Option<OffsetDateTime>,
    pub revoke_reason: Option<String>,
}

// ---- DTOs ----

#[derive(Debug, Deserialize)]
pub struct CreatePurchaseRequest {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub payment_method: String,
    pub discount_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub plan_id: Uuid,
    pub billing_cycle: String,
    pub organization_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct RefundRequest {
    pub purchase_id: Uuid,
    pub reason: Option<String>,
    pub amount: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct CheckoutSession {
    pub purchase_id: Uuid,
    pub client_secret: String,
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Serialize)]
pub struct RevenueReport {
    pub developer_id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub organization: Option<String>,
    pub earned_amount: Option<bigdecimal::BigDecimal>,
    pub currency: String,
    pub sale_count: Option<i64>,
    pub refunded_amount: Option<bigdecimal::BigDecimal>,
}
