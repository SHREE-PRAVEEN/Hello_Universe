-- -------------------------------------------------------
-- REPORTS (abuse / content reports)
-- -------------------------------------------------------
CREATE TABLE reports (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    reporter_id     UUID NOT NULL REFERENCES users(id),
    entity_type     VARCHAR(50) NOT NULL,
    entity_id       UUID NOT NULL,
    reason          VARCHAR(100) NOT NULL,   -- 'spam', 'inappropriate', 'copyright', etc.
    description     TEXT,
    status          moderation_status NOT NULL DEFAULT 'pending',
    resolved_by     UUID REFERENCES users(id),
    resolved_at     TIMESTAMPTZ,
    resolution_note TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

