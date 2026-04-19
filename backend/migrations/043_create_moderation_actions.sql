-- -------------------------------------------------------
-- MODERATION ACTIONS (decisions made)
-- -------------------------------------------------------
CREATE TABLE moderation_actions (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    queue_id            UUID REFERENCES moderation_queue(id),
    moderator_id        UUID NOT NULL REFERENCES users(id),
    entity_type         VARCHAR(50) NOT NULL,
    entity_id           UUID NOT NULL,
    action              VARCHAR(80) NOT NULL,  -- 'approve', 'reject', 'flag', 'escalate', 'ban'
    reason              TEXT,
    rejection_reason    VARCHAR(200),
    previous_status     VARCHAR(50),
    new_status          VARCHAR(50),
    metadata            JSONB NOT NULL DEFAULT '{}',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);