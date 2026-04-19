-- -------------------------------------------------------
-- SUBSCRIPTIONS
-- -------------------------------------------------------
CREATE TABLE subscriptions (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id             UUID REFERENCES users(id),
    organization_id     UUID REFERENCES organizations(id),
    plan_id             UUID NOT NULL REFERENCES plans(id),
    status              subscription_status NOT NULL DEFAULT 'trialing',
    billing_cycle       VARCHAR(20) NOT NULL DEFAULT 'monthly',  -- 'monthly', 'yearly'
    current_period_start TIMESTAMPTZ NOT NULL,
    current_period_end   TIMESTAMPTZ NOT NULL,
    trial_end_at        TIMESTAMPTZ,
    cancelled_at        TIMESTAMPTZ,
    cancel_reason       TEXT,
    external_id         VARCHAR(255) UNIQUE,   -- Stripe subscription ID
    metadata            JSONB NOT NULL DEFAULT '{}',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT subscription_has_subject CHECK (
        user_id IS NOT NULL OR organization_id IS NOT NULL
    )
);
