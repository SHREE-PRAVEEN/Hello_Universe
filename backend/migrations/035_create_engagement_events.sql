-- -------------------------------------------------------
-- ENGAGEMENT EVENTS (generic event stream)
-- -------------------------------------------------------
CREATE TABLE engagement_events (
    id              UUID NOT NULL DEFAULT uuid_generate_v4(),
    user_id         UUID REFERENCES users(id),
    session_id      VARCHAR(100),
    entity_type     VARCHAR(50),
    entity_id       UUID,
    event_type      VARCHAR(80) NOT NULL,
    properties      JSONB NOT NULL DEFAULT '{}',
    ip_address      INET,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id, created_at)
) PARTITION BY RANGE (created_at);

CREATE TABLE engagement_events_2024 PARTITION OF engagement_events
    FOR VALUES FROM ('2024-01-01') TO ('2025-01-01');
CREATE TABLE engagement_events_2025 PARTITION OF engagement_events
    FOR VALUES FROM ('2025-01-01') TO ('2026-01-01');
CREATE TABLE engagement_events_2026 PARTITION OF engagement_events
    FOR VALUES FROM ('2026-01-01') TO ('2027-01-01');
