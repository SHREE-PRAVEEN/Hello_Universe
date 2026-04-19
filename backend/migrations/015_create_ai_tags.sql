-- -------------------------------------------------------
-- AI TAGS (auto-generated tags with confidence)
-- -------------------------------------------------------
CREATE TABLE ai_tags (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_type     VARCHAR(50) NOT NULL,   -- 'project', 'media_file'
    entity_id       UUID NOT NULL,
    tag_name        VARCHAR(200) NOT NULL,
    confidence      NUMERIC(5,4) NOT NULL CHECK (confidence BETWEEN 0 AND 1),
    model_name      VARCHAR(200),
    model_version   VARCHAR(50),
    raw_response    JSONB,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (entity_type, entity_id, tag_name)
);