-- IPFS RECORDS
-- -------------------------------------------------------
CREATE TABLE ipfs_records (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    media_file_id   UUID REFERENCES media_files(id),
    entity_type     VARCHAR(50),
    entity_id       UUID,
    cid             VARCHAR(100) NOT NULL UNIQUE,   -- IPFS Content Identifier
    pin_status      VARCHAR(20) NOT NULL DEFAULT 'unpinned',  -- 'pinned', 'unpinned', 'pending'
    gateway_url     TEXT,
    pinned_at       TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
