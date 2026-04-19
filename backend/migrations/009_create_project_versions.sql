-- -------------------------------------------------------
-- PROJECT VERSIONS (Git-like version history)
-- -------------------------------------------------------
CREATE TABLE project_versions (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id      UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    version_label   VARCHAR(50)  NOT NULL,   -- e.g. '1.2.0', 'v2-beta'
    commit_hash     VARCHAR(64),             -- optional git hash
    changelog       TEXT,
    snapshot_meta   JSONB NOT NULL DEFAULT '{}',  -- snapshot of project state at this version
    created_by      UUID NOT NULL REFERENCES users(id),
    is_current      BOOLEAN NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (project_id, version_label)
);

