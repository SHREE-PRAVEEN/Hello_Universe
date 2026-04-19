-- -------------------------------------------------------
-- REVENUE SPLITS (developer/platform revenue sharing)
-- -------------------------------------------------------
CREATE TABLE revenue_splits (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    purchase_id     UUID NOT NULL REFERENCES purchases(id),
    recipient_id    UUID NOT NULL REFERENCES users(id),
    organization_id UUID REFERENCES organizations(id),
    entity_type     VARCHAR(50) NOT NULL,
    entity_id       UUID NOT NULL,
    split_percent   NUMERIC(5,2) NOT NULL,
    amount          NUMERIC(12,2) NOT NULL,
    currency        CHAR(3) NOT NULL DEFAULT 'USD',
    status          transaction_status NOT NULL DEFAULT 'pending',
    paid_out_at     TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
