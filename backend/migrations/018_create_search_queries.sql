-- -------------------------------------------------------
-- SEARCH QUERIES (logging for analytics and suggestions)
-- -------------------------------------------------------
CREATE TABLE search_queries (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID REFERENCES users(id),
    session_id      UUID REFERENCES sessions(id),
    query_text      TEXT NOT NULL,
    filters         JSONB NOT NULL DEFAULT '{}',
    result_count    INTEGER,
    clicked_id      UUID,      -- entity_id the user clicked
    clicked_type    VARCHAR(50),
    search_latency_ms INTEGER,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
