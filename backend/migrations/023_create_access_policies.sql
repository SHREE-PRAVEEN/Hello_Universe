-- ACCESS POLICIES (rules governing who can access what)
-- -------------------------------------------------------
CREATE TABLE access_policies (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_type     VARCHAR(50) NOT NULL,   -- 'project', 'media_file', 'organization'
    entity_id       UUID NOT NULL,
    policy_name     VARCHAR(150) NOT NULL,
    access_type     content_access NOT NULL,
    allowed_roles   TEXT[],                 -- role slugs that bypass checks
    require_org     BOOLEAN NOT NULL DEFAULT FALSE,
    require_login   BOOLEAN NOT NULL DEFAULT TRUE,
    geo_allowlist   CHAR(2)[],              -- ISO country codes allowed
    geo_blocklist   CHAR(2)[],
    ip_allowlist    INET[],
    custom_rules    JSONB NOT NULL DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
