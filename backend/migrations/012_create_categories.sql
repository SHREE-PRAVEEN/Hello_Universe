-- -------------------------------------------------------
-- PROJECT CATEGORIES
-- -------------------------------------------------------
CREATE TABLE project_categories (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name        VARCHAR(150) NOT NULL,
    slug        VARCHAR(150) NOT NULL UNIQUE,
    parent_id   UUID REFERENCES project_categories(id),  -- hierarchical categories
    description TEXT,
    icon_url    TEXT,
    sort_order  INTEGER NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

