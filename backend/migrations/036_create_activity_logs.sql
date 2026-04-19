-- -------------------------------------------------------
-- ACTIVITY LOGS (user action trail)
-- -------------------------------------------------------
CREATE TABLE activity_logs (
    id              UUID NOT NULL DEFAULT uuid_generate_v4(),
    user_id         UUID REFERENCES users(id),
    organization_id UUID REFERENCES organizations(id),
    action          VARCHAR(100) NOT NULL,
    entity_type     VARCHAR(50),
    entity_id       UUID,
    description     TEXT,
    metadata        JSONB NOT NULL DEFAULT '{}',
    ip_address      INET,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id, created_at)
) PARTITION BY RANGE (created_at);

CREATE TABLE activity_logs_2024 PARTITION OF activity_logs
    FOR VALUES FROM ('2024-01-01') TO ('2025-01-01');
CREATE TABLE activity_logs_2025 PARTITION OF activity_logs
    FOR VALUES FROM ('2025-01-01') TO ('2026-01-01');
CREATE TABLE activity_logs_2026 PARTITION OF activity_logs
    FOR VALUES FROM ('2026-01-01') TO ('2027-01-01');


