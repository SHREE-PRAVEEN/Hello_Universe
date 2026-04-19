-- -------------------------------------------------------
-- NOTIFICATIONS
-- -------------------------------------------------------
CREATE TABLE notifications (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    type            notification_type NOT NULL,
    title           VARCHAR(300) NOT NULL,
    body            TEXT,
    entity_type     VARCHAR(50),
    entity_id       UUID,
    action_url      TEXT,
    is_read         BOOLEAN NOT NULL DEFAULT FALSE,
    read_at         TIMESTAMPTZ,
    sent_via        TEXT[],    -- ['email', 'push', 'in_app']
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);