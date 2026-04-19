-- -------------------------------------------------------
-- MEDIA STORAGE LOCATIONS (where the file actually lives)
-- -------------------------------------------------------
CREATE TABLE media_storage_locations (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    media_file_id   UUID NOT NULL REFERENCES media_files(id) ON DELETE CASCADE,
    backend         storage_backend NOT NULL,
    bucket          VARCHAR(255),      -- S3 bucket, GCS bucket, etc.
    storage_path    TEXT NOT NULL,     -- object key / file path
    cdn_url         TEXT,              -- CDN-fronted URL
    region          VARCHAR(50),
    is_primary      BOOLEAN NOT NULL DEFAULT TRUE,  -- primary delivery location
    is_replica      BOOLEAN NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


-- -------------------------------------------------------
-- MEDIA TAGS (media-level tag associations)
-- -------------------------------------------------------
CREATE TABLE media_tags (
    media_file_id   UUID NOT NULL REFERENCES media_files(id) ON DELETE CASCADE,
    tag_id          UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    tagged_by       UUID REFERENCES users(id),
    tagged_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (media_file_id, tag_id)
);