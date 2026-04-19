
-- -------------------------------------------------------
-- PURCHASES (one-time content purchases)
-- -------------------------------------------------------
CREATE TABLE purchases (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id             UUID NOT NULL REFERENCES users(id),
    entity_type         VARCHAR(50) NOT NULL,   -- 'project', 'media_file', 'plan'
    entity_id           UUID NOT NULL,
    amount              NUMERIC(12,2) NOT NULL,
    currency            CHAR(3) NOT NULL DEFAULT 'USD',
    status              transaction_status NOT NULL DEFAULT 'pending',
    payment_method      VARCHAR(50),            -- 'stripe', 'paypal', 'crypto'
    payment_intent_id   VARCHAR(255) UNIQUE,    -- external payment ID
    invoice_id          UUID,                   -- filled after invoice generation
    discount_code       VARCHAR(50),
    discount_amount     NUMERIC(12,2) NOT NULL DEFAULT 0,
    tax_amount          NUMERIC(12,2) NOT NULL DEFAULT 0,
    metadata            JSONB NOT NULL DEFAULT '{}',
    completed_at        TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);