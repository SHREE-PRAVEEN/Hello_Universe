-- -------------------------------------------------------
-- ORGANIZATION MEMBERS
-- -------------------------------------------------------
CREATE TABLE organization_members (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    org_role_id     UUID REFERENCES organization_roles(id) ON DELETE SET NULL,
    invited_by      UUID REFERENCES users(id),
    joined_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_active       BOOLEAN NOT NULL DEFAULT TRUE,
    UNIQUE (organization_id, user_id)
);