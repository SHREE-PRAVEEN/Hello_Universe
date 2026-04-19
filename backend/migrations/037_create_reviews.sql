-- -------------------------------------------------------
-- REVIEWS (star ratings + text)
-- -------------------------------------------------------
CREATE TABLE reviews (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_type     VARCHAR(50) NOT NULL,   -- 'project', 'media_file'
    entity_id       UUID NOT NULL,
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    rating          SMALLINT NOT NULL CHECK (rating BETWEEN 1 AND 5),
    title           VARCHAR(200),
    body            TEXT,
    is_verified     BOOLEAN NOT NULL DEFAULT FALSE,   -- verified purchase
    helpful_count   INTEGER NOT NULL DEFAULT 0,
    status          moderation_status NOT NULL DEFAULT 'pending',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ,

    UNIQUE (entity_type, entity_id, user_id)
);
