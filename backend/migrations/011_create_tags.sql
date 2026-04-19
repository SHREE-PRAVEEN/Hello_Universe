-- -------------------------------------------------------
-- TAGS (global reusable tags)
-- -------------------------------------------------------
CREATE TABLE tags (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name        VARCHAR(100) NOT NULL UNIQUE,
    slug        VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    use_count   INTEGER NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);




-- -------------------------------------------------------
-- PROJECT <-> TAG (many-to-many)
-- -------------------------------------------------------
CREATE TABLE project_tags (
    project_id  UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    tag_id      UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    tagged_by   UUID REFERENCES users(id),
    tagged_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (project_id, tag_id)
);