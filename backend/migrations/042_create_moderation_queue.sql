-- -------------------------------------------------------
-- MODERATION QUEUE
-- -------------------------------------------------------
CREATE TABLE moderation_queue (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_type     VARCHAR(50) NOT NULL,
    entity_id       UUID NOT NULL,
    queue_reason    VARCHAR(100) NOT NULL,  -- 'new_upload', 'reported', 'edited', 'appeal'
    priority        SMALLINT NOT NULL DEFAULT 3 CHECK (priority BETWEEN 1 AND 5),
    assigned_to     UUID REFERENCES users(id),
    status          moderation_status NOT NULL DEFAULT 'pending',
    notes           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);