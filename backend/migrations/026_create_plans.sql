-- -------------------------------------------------------
-- PLANS (subscription tiers)
-- -------------------------------------------------------
CREATE TABLE plans (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name                VARCHAR(150) NOT NULL,
    slug                VARCHAR(100) NOT NULL UNIQUE,
    description         TEXT,
    price_monthly       NUMERIC(12,2),
    price_yearly        NUMERIC(12,2),
    currency            CHAR(3) NOT NULL DEFAULT 'USD',
    max_projects        INTEGER,           -- NULL = unlimited
    max_storage_gb      INTEGER,
    max_collaborators   INTEGER,
    features            JSONB NOT NULL DEFAULT '{}',
    is_active           BOOLEAN NOT NULL DEFAULT TRUE,
    is_public           BOOLEAN NOT NULL DEFAULT TRUE,
    sort_order          INTEGER NOT NULL DEFAULT 0,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
