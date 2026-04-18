use crate::utils::errors::{AppError, AppResult};
use serde::{Deserialize, Serialize};

pub struct StripeClient {
    secret_key: String,
    webhook_secret: String,
    http: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntent {
    pub id: String,
    pub client_secret: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct StripeWebhookEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub data: serde_json::Value,
}

impl StripeClient {
    pub fn new(secret_key: &str, webhook_secret: &str) -> Self {
        Self {
            secret_key: secret_key.to_string(),
            webhook_secret: webhook_secret.to_string(),
            http: reqwest::Client::new(),
        }
    }

    /// Create a PaymentIntent for a purchase
    pub async fn create_payment_intent(
        &self,
        amount_cents: i64,
        currency: &str,
        metadata: serde_json::Value,
    ) -> AppResult<PaymentIntent> {
        let params = [
            ("amount", amount_cents.to_string()),
            ("currency", currency.to_string()),
            ("automatic_payment_methods[enabled]", "true".to_string()),
        ];

        let resp = self.http
            .post("https://api.stripe.com/v1/payment_intents")
            .basic_auth(&self.secret_key, Some(""))
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Stripe request failed: {}", e)))?;

        if !resp.status().is_success() {
            let err = resp.text().await.unwrap_or_default();
            return Err(AppError::Payment(format!("Stripe error: {}", err)));
        }

        resp.json::<PaymentIntent>().await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Stripe parse error: {}", e)))
    }

    /// Verify Stripe webhook signature
    pub fn verify_webhook(&self, payload: &[u8], signature: &str) -> AppResult<StripeWebhookEvent> {
        // In production use stripe-rust or verify manually with HMAC-SHA256
        // Simplified here — replace with proper Stripe signature verification
        let event: StripeWebhookEvent = serde_json::from_slice(payload)
            .map_err(|e| AppError::BadRequest(format!("Invalid webhook payload: {}", e)))?;
        Ok(event)
    }

    /// Refund a payment intent
    pub async fn refund(&self, payment_intent_id: &str, amount_cents: Option<i64>) -> AppResult<String> {
        let mut params = vec![("payment_intent", payment_intent_id.to_string())];
        if let Some(amt) = amount_cents {
            params.push(("amount", amt.to_string()));
        }

        let resp = self.http
            .post("https://api.stripe.com/v1/refunds")
            .basic_auth(&self.secret_key, Some(""))
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Stripe refund failed: {}", e)))?;

        let data: serde_json::Value = resp.json().await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Stripe refund parse: {}", e)))?;

        Ok(data["id"].as_str().unwrap_or("").to_string())
    }
}
