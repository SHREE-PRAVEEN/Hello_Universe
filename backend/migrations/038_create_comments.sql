-- -------------------------------------------------------
-- COMMENTS (threaded comments)
-- -------------------------------------------------------
CREATE TABLE comments (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_type VARCHAR(50) NOT NULL,
    entity_id   UUID NOT NULL,
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_id   UUID REFERENCES comments(id) ON DELETE CASCADE,  -- threading
    body        TEXT NOT NULL,
    like_count  INTEGER NOT NULL DEFAULT 0,
    status      moderation_status NOT NULL DEFAULT 'approved',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at  TIMESTAMPTZ
);