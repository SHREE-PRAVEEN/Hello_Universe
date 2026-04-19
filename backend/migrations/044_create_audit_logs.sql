-- -------------------------------------------------------
-- AUDIT LOGS (immutable compliance trail)
-- -------------------------------------------------------
CREATE TABLE audit_logs (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    actor_id        UUID REFERENCES users(id),          -- who performed the action
    actor_role      VARCHAR(80),                         -- their role at time of action
    organization_id UUID REFERENCES organizations(id),
    action          VARCHAR(150) NOT NULL,               -- 'user.delete', 'project.status_change'
    entity_type     VARCHAR(50),
    entity_id       UUID,
    old_state       JSONB,                               -- snapshot before change
    new_state       JSONB,                               -- snapshot after change
    diff            JSONB,                               -- computed diff
    ip_address      INET,
    user_agent      TEXT,
    request_id      VARCHAR(100),                        -- tracing ID
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
    -- NOTE: No updated_at — audit logs are immutable
);