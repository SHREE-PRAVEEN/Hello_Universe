-- -------------------------------------------------------
-- CONTENT HASHES (integrity record per asset)
-- -------------------------------------------------------
CREATE TABLE content_hashes (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_type     VARCHAR(50) NOT NULL,
    entity_id       UUID NOT NULL,
    hash_algorithm  VARCHAR(20) NOT NULL DEFAULT 'sha256',
    hash_value      VARCHAR(128) NOT NULL,
    computed_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    computed_by     UUID REFERENCES users(id),
    UNIQUE (entity_type, entity_id, hash_algorithm)
);