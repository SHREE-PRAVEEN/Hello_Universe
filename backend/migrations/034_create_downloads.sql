-- -------------------------------------------------------
-- DOWNLOADS
-- -------------------------------------------------------
CREATE TABLE downloads (
    id              UUID NOT NULL DEFAULT uuid_generate_v4(),
    entity_type     VARCHAR(50) NOT NULL,
    entity_id       UUID NOT NULL,
    user_id         UUID REFERENCES users(id),
    ip_address      INET,
    country         CHAR(2),
    version_label   VARCHAR(50),
    file_size_bytes BIGINT,
    downloaded_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id, downloaded_at)
) PARTITION BY RANGE (downloaded_at);

CREATE TABLE downloads_2024 PARTITION OF downloads
    FOR VALUES FROM ('2024-01-01') TO ('2025-01-01');
CREATE TABLE downloads_2025 PARTITION OF downloads
    FOR VALUES FROM ('2025-01-01') TO ('2026-01-01');
CREATE TABLE downloads_2026 PARTITION OF downloads
    FOR VALUES FROM ('2026-01-01') TO ('2027-01-01');
