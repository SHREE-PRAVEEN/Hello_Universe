-- -------------------------------------------------------
-- BLOCKCHAIN TRANSACTIONS (on-chain proof anchors)
-- -------------------------------------------------------
CREATE TABLE blockchain_transactions (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_type     VARCHAR(50) NOT NULL,
    entity_id       UUID NOT NULL,
    network         VARCHAR(50) NOT NULL,    -- 'ethereum', 'polygon', 'solana', etc.
    tx_hash         VARCHAR(128) NOT NULL UNIQUE,
    block_number    BIGINT,
    block_hash      VARCHAR(128),
    contract_address VARCHAR(60),
    payload_hash    VARCHAR(128),           -- hash of the committed payload
    status          VARCHAR(20) NOT NULL DEFAULT 'pending',
    confirmed_at    TIMESTAMPTZ,
    created_by      UUID REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
