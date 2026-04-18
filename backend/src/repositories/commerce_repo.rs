use crate::models::commerce::*;
use sqlx::PgPool;
use uuid::Uuid;
use crate::utils::errors::{AppError, AppResult};

pub struct CommerceRepo;

impl CommerceRepo {
    // ---- Plans ----
    pub async fn list_plans(pool: &PgPool) -> AppResult<Vec<Plan>> {
        Ok(sqlx::query_as!(
            Plan,
            r#"SELECT id, name, slug, description, price_monthly, price_yearly,
               currency, max_projects, max_storage_gb, max_collaborators,
               features, is_active, is_public, sort_order, created_at, updated_at
               FROM plans WHERE is_active = TRUE AND is_public = TRUE
               ORDER BY sort_order ASC"#
        )
        .fetch_all(pool)
        .await?)
    }

    // ---- Subscriptions ----
    pub async fn get_active_subscription(pool: &PgPool, user_id: Uuid) -> AppResult<Option<Subscription>> {
        Ok(sqlx::query_as!(
            Subscription,
            r#"SELECT id, user_id, organization_id, plan_id, status, billing_cycle,
               current_period_start, current_period_end, trial_end_at,
               cancelled_at, cancel_reason, external_id, metadata, created_at, updated_at
               FROM subscriptions
               WHERE user_id = $1 AND status = 'active'
                 AND NOW() BETWEEN current_period_start AND current_period_end
               ORDER BY created_at DESC LIMIT 1"#,
            user_id
        )
        .fetch_optional(pool)
        .await?)
    }

    pub async fn create_subscription(
        pool: &PgPool,
        user_id: Option<Uuid>,
        org_id: Option<Uuid>,
        plan_id: Uuid,
        billing_cycle: &str,
        period_start: time::OffsetDateTime,
        period_end: time::OffsetDateTime,
        external_id: Option<&str>,
    ) -> AppResult<Subscription> {
        Ok(sqlx::query_as!(
            Subscription,
            r#"INSERT INTO subscriptions
               (user_id, organization_id, plan_id, billing_cycle, status,
                current_period_start, current_period_end, external_id)
               VALUES ($1,$2,$3,$4,'active',$5,$6,$7)
               RETURNING id, user_id, organization_id, plan_id, status, billing_cycle,
               current_period_start, current_period_end, trial_end_at,
               cancelled_at, cancel_reason, external_id, metadata, created_at, updated_at"#,
            user_id as _, org_id as _, plan_id, billing_cycle,
            period_start, period_end, external_id
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn cancel_subscription(pool: &PgPool, id: Uuid, reason: Option<&str>) -> AppResult<()> {
        sqlx::query!(
            "UPDATE subscriptions SET status = 'cancelled', cancelled_at = NOW(), cancel_reason = $2
             WHERE id = $1",
            id, reason
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    // ---- Purchases ----
    pub async fn create_purchase(
        pool: &PgPool,
        user_id: Uuid,
        entity_type: &str,
        entity_id: Uuid,
        amount: f64,
        currency: &str,
        payment_method: &str,
        payment_intent_id: Option<&str>,
    ) -> AppResult<Purchase> {
        Ok(sqlx::query_as!(
            Purchase,
            r#"INSERT INTO purchases
               (user_id, entity_type, entity_id, amount, currency, payment_method, payment_intent_id)
               VALUES ($1,$2,$3,$4,$5,$6,$7)
               RETURNING id, user_id, entity_type, entity_id, amount, currency, status,
               payment_method, payment_intent_id, invoice_id, discount_code,
               discount_amount, tax_amount, metadata, completed_at, created_at, updated_at"#,
            user_id, entity_type, entity_id, bigdecimal::BigDecimal::try_from(amount).unwrap(),
            currency, payment_method, payment_intent_id
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn complete_purchase(pool: &PgPool, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            "UPDATE purchases SET status = 'completed', completed_at = NOW() WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn find_purchase(pool: &PgPool, id: Uuid) -> AppResult<Purchase> {
        sqlx::query_as!(
            Purchase,
            r#"SELECT id, user_id, entity_type, entity_id, amount, currency, status,
               payment_method, payment_intent_id, invoice_id, discount_code,
               discount_amount, tax_amount, metadata, completed_at, created_at, updated_at
               FROM purchases WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::not_found("Purchase"))
    }

    pub async fn user_purchases(pool: &PgPool, user_id: Uuid, limit: i64, offset: i64) -> AppResult<Vec<Purchase>> {
        Ok(sqlx::query_as!(
            Purchase,
            r#"SELECT id, user_id, entity_type, entity_id, amount, currency, status,
               payment_method, payment_intent_id, invoice_id, discount_code,
               discount_amount, tax_amount, metadata, completed_at, created_at, updated_at
               FROM purchases WHERE user_id = $1
               ORDER BY created_at DESC LIMIT $2 OFFSET $3"#,
            user_id, limit, offset
        )
        .fetch_all(pool)
        .await?)
    }

    // ---- Entitlements ----
    pub async fn has_entitlement(
        pool: &PgPool,
        user_id: Uuid,
        entity_type: &str,
        entity_id: Uuid,
    ) -> AppResult<bool> {
        let result = sqlx::query_scalar!(
            r#"SELECT EXISTS (
               -- Free public content
               SELECT 1 FROM projects p WHERE p.id = $3 AND p.access_type = 'free' AND p.visibility = 'public'
               UNION ALL
               -- Direct entitlement
               SELECT 1 FROM content_entitlements ce
               WHERE ce.user_id = $1 AND ce.entity_type = $2 AND ce.entity_id = $3
                 AND (ce.expires_at IS NULL OR ce.expires_at > NOW()) AND ce.revoked_at IS NULL
               UNION ALL
               -- Active subscription
               SELECT 1 FROM subscriptions s
               WHERE s.user_id = $1 AND s.status = 'active'
                 AND NOW() BETWEEN s.current_period_start AND s.current_period_end
               UNION ALL
               -- Admin/moderator override
               SELECT 1 FROM user_roles ur
               JOIN roles r ON r.id = ur.role_id
               WHERE ur.user_id = $1 AND r.slug IN ('admin','moderator')
                 AND (ur.expires_at IS NULL OR ur.expires_at > NOW())
               UNION ALL
               -- Org member for org-only content
               SELECT 1 FROM projects p
               JOIN organization_members om ON om.organization_id = p.organization_id
               WHERE p.id = $3 AND om.user_id = $1 AND om.is_active = TRUE AND p.visibility = 'organization'
            ) AS has_access"#,
            user_id, entity_type, entity_id
        )
        .fetch_one(pool)
        .await?;
        Ok(result.unwrap_or(false))
    }

    pub async fn grant_entitlement(
        pool: &PgPool,
        user_id: Uuid,
        entity_type: &str,
        entity_id: Uuid,
        source: &str,
        source_id: Option<Uuid>,
        granted_by: Option<Uuid>,
        expires_at: Option<time::OffsetDateTime>,
    ) -> AppResult<ContentEntitlement> {
        Ok(sqlx::query_as!(
            ContentEntitlement,
            r#"INSERT INTO content_entitlements
               (user_id, entity_type, entity_id, entitlement_source, source_id, granted_by, expires_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7)
               ON CONFLICT (user_id, entity_type, entity_id) DO UPDATE
               SET expires_at = EXCLUDED.expires_at, revoked_at = NULL
               RETURNING id, user_id, entity_type, entity_id, entitlement_source,
               source_id, granted_by, granted_at, expires_at, revoked_at, revoke_reason"#,
            user_id, entity_type, entity_id, source,
            source_id as _, granted_by as _, expires_at
        )
        .fetch_one(pool)
        .await?)
    }

    // ---- Revenue ----
    pub async fn revenue_report(pool: &PgPool, from: &str, to: &str) -> AppResult<Vec<RevenueReport>> {
        Ok(sqlx::query_as!(
            RevenueReport,
            r#"SELECT u.id AS developer_id, u.username, u.display_name,
               o.name AS organization,
               SUM(rs.amount) AS "earned_amount: bigdecimal::BigDecimal",
               rs.currency,
               COUNT(DISTINCT rs.purchase_id) AS "sale_count: i64",
               SUM(CASE WHEN r.id IS NOT NULL THEN r.amount ELSE 0 END) AS "refunded_amount: bigdecimal::BigDecimal"
               FROM revenue_splits rs
               JOIN users u ON u.id = rs.recipient_id
               LEFT JOIN organizations o ON o.id = rs.organization_id
               LEFT JOIN refunds r ON r.purchase_id = rs.purchase_id AND r.status = 'completed'
               WHERE rs.created_at BETWEEN $1::TIMESTAMPTZ AND $2::TIMESTAMPTZ
                 AND rs.status = 'completed'
               GROUP BY u.id, u.username, u.display_name, o.name, rs.currency
               ORDER BY SUM(rs.amount) DESC"#,
            from, to
        )
        .fetch_all(pool)
        .await?)
    }
}
