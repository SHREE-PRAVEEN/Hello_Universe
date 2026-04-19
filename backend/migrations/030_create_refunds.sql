-- -------------------------------------------------------
-- REFUNDS
-- -------------------------------------------------------
CREATE TABLE refunds (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    purchase_id     UUID NOT NULL REFERENCES purchases(id),
    user_id         UUID NOT NULL REFERENCES users(id),
    amount          NUMERIC(12,2) NOT NULL,
    currency        CHAR(3) NOT NULL DEFAULT 'USD',
    reason          TEXT,
    status          transaction_status NOT NULL DEFAULT 'pending',
    processed_by    UUID REFERENCES users(id),
    external_id     VARCHAR(255),   -- refund ID from payment gateway
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);