-- -------------------------------------------------------
-- PROJECT VIEWS
-- -------------------------------------------------------
CREATE TABLE project_views (
    id          UUID NOT NULL DEFAULT uuid_generate_v4(),
    project_id  UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    user_id     UUID REFERENCES users(id),
    session_id  VARCHAR(100),
    ip_address  INET,
    referrer    TEXT,
    user_agent  TEXT,
    country     CHAR(2),
    viewed_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id, viewed_at)
) PARTITION BY RANGE (viewed_at);

CREATE TABLE project_views_2024 PARTITION OF project_views
    FOR VALUES FROM ('2024-01-01') TO ('2025-01-01');
CREATE TABLE project_views_2025 PARTITION OF project_views
    FOR VALUES FROM ('2025-01-01') TO ('2026-01-01');
CREATE TABLE project_views_2026 PARTITION OF project_views
    FOR VALUES FROM ('2026-01-01') TO ('2027-01-01');
