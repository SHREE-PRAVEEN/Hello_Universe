-- -------------------------------------------------------
-- FOLLOWS
-- -------------------------------------------------------
CREATE TABLE follows (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    follower_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    entity_type     VARCHAR(50) NOT NULL,   -- 'user', 'organization'
    entity_id       UUID NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (follower_id, entity_type, entity_id)
);