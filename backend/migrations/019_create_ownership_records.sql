-- -------------------------------------------------------
-- OWNERSHIP RECORDS (chain of custody)
-- -------------------------------------------------------
CREATE TABLE ownership_records (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_type         VARCHAR(50) NOT NULL,   -- 'project', 'media_file'
    entity_id           UUID NOT NULL,
    owner_user_id       UUID REFERENCES users(id),
    owner_org_id        UUID REFERENCES organizations(id),
    transferred_from_id UUID REFERENCES ownership_records(id),  -- previous owner record
    transfer_reason     TEXT,
    metadata            JSONB NOT NULL DEFAULT '{}',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT ownership_has_owner CHECK (
        owner_user_id IS NOT NULL OR owner_org_id IS NOT NULL
    )
);
