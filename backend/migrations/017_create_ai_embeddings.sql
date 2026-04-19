-- -------------------------------------------------------
-- AI EMBEDDINGS (for semantic / vector search)
-- -------------------------------------------------------
CREATE TABLE ai_embeddings (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_type     VARCHAR(50) NOT NULL,
    entity_id       UUID NOT NULL,
    model_name      VARCHAR(200) NOT NULL,  -- e.g. 'text-embedding-3-large'
    embedding       JSONB NOT NULL,         -- store as JSON array; replace with pgvector column when ready
    dimensions      INTEGER NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (entity_type, entity_id, model_name)
);