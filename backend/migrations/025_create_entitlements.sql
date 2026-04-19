-- -------------------------------------------------------
-- CONTENT ENTITLEMENTS (what a user is authorized to access)
-- -------------------------------------------------------
CREATE TABLE content_entitlements (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    entity_type     VARCHAR(50) NOT NULL,
    entity_id       UUID NOT NULL,
    entitlement_source VARCHAR(50) NOT NULL,  -- 'purchase', 'subscription', 'grant', 'org_membership'
    source_id       UUID,                      -- FK to the granting record
    granted_by      UUID REFERENCES users(id),
    granted_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at      TIMESTAMPTZ,
    revoked_at      TIMESTAMPTZ,
    revoke_reason   TEXT,

    UNIQUE (user_id, entity_type, entity_id)
);