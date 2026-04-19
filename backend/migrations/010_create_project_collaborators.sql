-- -------------------------------------------------------
-- PROJECT COLLABORATORS
-- -------------------------------------------------------
CREATE TABLE project_collaborators (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id  UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role        VARCHAR(50) NOT NULL DEFAULT 'contributor',  -- 'owner', 'contributor', 'viewer'
    added_by    UUID REFERENCES users(id),
    added_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (project_id, user_id)
);

