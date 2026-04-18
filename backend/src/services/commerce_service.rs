use crate::{
    config::AppState,
    models::commerce::*,
    repositories::CommerceRepo,
    utils::errors::{AppError, AppResult},
};
use uuid::Uuid;

pub struct CommerceService;

impl CommerceService {
    pub async fn list_plans(state: &AppState) -> AppResult<Vec<Plan>> {
        CommerceRepo::list_plans(&state.db).await
    }

    pub async fn get_active_subscription(
        state: &AppState,
        user_id: Uuid,
    ) -> AppResult<Option<Subscription>> {
        CommerceRepo::get_active_subscription(&state.db, user_id).await
    }

    pub async fn subscribe(
        state: &AppState,
        user_id: Uuid,
        req: &CreateSubscriptionRequest,
    ) -> AppResult<Subscription> {
        // Check plan exists
        let plan = sqlx::query_as!(
            Plan,
            r#"SELECT id, name, slug, description, price_monthly, price_yearly,
               currency, max_projects, max_storage_gb, max_collaborators,
               features, is_active, is_public, sort_order, created_at, updated_at
               FROM plans WHERE id=$1 AND is_active=TRUE"#,
            req.plan_id
        )
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::not_found("Plan"))?;

        // Cancel existing subscription if any
        if let Some(existing) = CommerceRepo::get_active_subscription(&state.db, user_id).await? {
            CommerceRepo::cancel_subscription(&state.db, existing.id, Some("Upgraded/changed plan"))
                .await?;
        }

        let now = time::OffsetDateTime::now_utc();
        let period_end = if req.billing_cycle == "yearly" {
            now + time::Duration::days(365)
        } else {
            now + time::Duration::days(30)
        };

        // TODO: integrate Stripe to create real payment subscription
        // For now, create subscription record directly
        CommerceRepo::create_subscription(
            &state.db,
            Some(user_id),
            req.organization_id,
            req.plan_id,
            &req.billing_cycle,
            now,
            period_end,
            None, // stripe subscription ID would go here
        )
        .await
    }

    pub async fn cancel_subscription(state: &AppState, user_id: Uuid) -> AppResult<()> {
        let sub = CommerceRepo::get_active_subscription(&state.db, user_id)
            .await?
            .ok_or_else(|| AppError::not_found("Active subscription"))?;
        CommerceRepo::cancel_subscription(&state.db, sub.id, Some("User requested")).await
    }

    pub async fn initiate_purchase(
        state: &AppState,
        user_id: Uuid,
        req: &CreatePurchaseRequest,
    ) -> AppResult<CheckoutSession> {
        // Check not already purchased
        let already = CommerceRepo::has_entitlement(
            &state.db,
            user_id,
            &req.entity_type,
            req.entity_id,
        )
        .await?;
        if already {
            return Err(AppError::Conflict("You already have access to this content".into()));
        }

        // Get price from entity
        let price = match req.entity_type.as_str() {
            "project" => {
                sqlx::query_scalar!(
                    "SELECT price FROM projects WHERE id=$1 AND access_type='paid'",
                    req.entity_id
                )
                .fetch_optional(&state.db)
                .await?
                .flatten()
                .ok_or_else(|| AppError::BadRequest("Content is not available for purchase".into()))?
            }
            _ => return Err(AppError::BadRequest("Unsupported entity type for purchase".into())),
        };

        let amount: f64 = price.to_string().parse().unwrap_or(0.0);

        // TODO: create Stripe PaymentIntent and return client_secret
        let purchase = CommerceRepo::create_purchase(
            &state.db,
            user_id,
            &req.entity_type,
            req.entity_id,
            amount,
            "USD",
            &req.payment_method,
            None, // payment_intent_id from Stripe
        )
        .await?;

        Ok(CheckoutSession {
            purchase_id: purchase.id,
            client_secret: "pi_mock_secret".to_string(), // replace with Stripe secret
            amount,
            currency: "USD".to_string(),
        })
    }

    pub async fn complete_purchase(state: &AppState, purchase_id: Uuid) -> AppResult<()> {
        let purchase = CommerceRepo::find_purchase(&state.db, purchase_id).await?;
        CommerceRepo::complete_purchase(&state.db, purchase_id).await?;

        // Grant entitlement
        CommerceRepo::grant_entitlement(
            &state.db,
            purchase.user_id,
            &purchase.entity_type,
            purchase.entity_id,
            "purchase",
            Some(purchase_id),
            None,
            None,
        )
        .await?;

        Ok(())
    }

    pub async fn check_access(
        state: &AppState,
        user_id: Uuid,
        entity_type: &str,
        entity_id: Uuid,
    ) -> AppResult<bool> {
        CommerceRepo::has_entitlement(&state.db, user_id, entity_type, entity_id).await
    }

    pub async fn user_purchases(
        state: &AppState,
        user_id: Uuid,
        page: i64,
        per_page: i64,
    ) -> AppResult<Vec<Purchase>> {
        let limit = per_page.min(100);
        let offset = (page - 1) * limit;
        CommerceRepo::user_purchases(&state.db, user_id, limit, offset).await
    }

    pub async fn revenue_report(state: &AppState, from: &str, to: &str) -> AppResult<Vec<RevenueReport>> {
        CommerceRepo::revenue_report(&state.db, from, to).await
    }
}
