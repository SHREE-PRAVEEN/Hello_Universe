-- -------------------------------------------------------
-- ORGANIZATIONS
-- -------------------------------------------------------
CREATE TABLE organizations (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name            VARCHAR(200) NOT NULL,
    slug            VARCHAR(100) NOT NULL UNIQUE,
    org_type        org_type     NOT NULL DEFAULT 'company',
    description     TEXT,
    logo_url        TEXT,
    website_url     TEXT,
    email           VARCHAR(255),
    country         CHAR(2),                -- ISO 3166-1 alpha-2
    verified        BOOLEAN NOT NULL DEFAULT FALSE,
    owner_id        UUID NOT NULL REFERENCES users(id),
    metadata        JSONB NOT NULL DEFAULT '{}',
    search_vector   TSVECTOR,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ
);

-- -------------------------------------------------------
-- ORGANIZATION ROLES (org-scoped role definitions)
-- -------------------------------------------------------
CREATE TABLE organization_roles (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name            VARCHAR(80) NOT NULL,
    slug            VARCHAR(80) NOT NULL,
    description     TEXT,
    permissions     JSONB NOT NULL DEFAULT '[]',  -- array of permission slugs
    is_default      BOOLEAN NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (organization_id, slug)
);

