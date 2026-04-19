-- -------------------------------------------------------
-- SEARCH INDEX (pre-computed FTS documents)
-- -------------------------------------------------------
CREATE TABLE search_index (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_type     VARCHAR(50) NOT NULL,   -- 'project', 'media_file', 'user', 'organization'
    entity_id       UUID NOT NULL UNIQUE,
    title           TEXT NOT NULL,
    body            TEXT,
    tags            TEXT[],
    search_vector   TSVECTOR NOT NULL,
    rank_score      REAL NOT NULL DEFAULT 0,   -- pre-computed relevance rank
    metadata        JSONB NOT NULL DEFAULT '{}',
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (entity_type, entity_id)
);