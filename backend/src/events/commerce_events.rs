use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;
use super::DomainEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseCompleted {
    pub purchase_id: Uuid,
    pub user_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub occurred_at: OffsetDateTime,
}

impl DomainEvent for PurchaseCompleted {
    fn event_type(&self) -> &'static str { "commerce.purchase_completed" }
    fn aggregate_id(&self) -> Uuid { self.purchase_id }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionStarted {
    pub subscription_id: Uuid,
    pub user_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub plan_id: Uuid,
    pub billing_cycle: String,
    pub occurred_at: OffsetDateTime,
}

impl DomainEvent for SubscriptionStarted {
    fn event_type(&self) -> &'static str { "commerce.subscription_started" }
    fn aggregate_id(&self) -> Uuid { self.subscription_id }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionCancelled {
    pub subscription_id: Uuid,
    pub user_id: Option<Uuid>,
    pub reason: Option<String>,
    pub occurred_at: OffsetDateTime,
}

impl DomainEvent for SubscriptionCancelled {
    fn event_type(&self) -> &'static str { "commerce.subscription_cancelled" }
    fn aggregate_id(&self) -> Uuid { self.subscription_id }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundIssued {
    pub refund_id: Uuid,
    pub purchase_id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub occurred_at: OffsetDateTime,
}

impl DomainEvent for RefundIssued {
    fn event_type(&self) -> &'static str { "commerce.refund_issued" }
    fn aggregate_id(&self) -> Uuid { self.refund_id }
}
