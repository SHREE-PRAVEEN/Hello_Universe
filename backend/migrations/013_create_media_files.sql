-- -------------------------------------------------------
-- MEDIA FILES (master record per file asset)
-- -------------------------------------------------------
CREATE TABLE media_files (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id          UUID REFERENCES projects(id) ON DELETE CASCADE,
    project_version_id  UUID REFERENCES project_versions(id),
    uploaded_by         UUID NOT NULL REFERENCES users(id),
    organization_id     UUID REFERENCES organizations(id),

    -- Identification
    original_filename   VARCHAR(500) NOT NULL,
    display_name        VARCHAR(500),
    description         TEXT,
    media_type          media_type NOT NULL,
    mime_type           VARCHAR(200) NOT NULL,

    -- File properties
    file_size_bytes     BIGINT NOT NULL,
    checksum_sha256     VARCHAR(64) NOT NULL,   -- integrity check
    checksum_md5        VARCHAR(32),

    -- Media-specific metadata
    width_px            INTEGER,    -- images/video
    height_px           INTEGER,    -- images/video
    duration_seconds    NUMERIC(12,3), -- video/audio
    page_count          INTEGER,    -- PDFs/documents
    bitrate_kbps        INTEGER,    -- video/audio

    -- Flexible extra metadata
    attributes          JSONB NOT NULL DEFAULT '{}',

    -- Versioning
    version_number      INTEGER NOT NULL DEFAULT 1,
    parent_id           UUID REFERENCES media_files(id),  -- previous version

    -- Visibility
    visibility          visibility NOT NULL DEFAULT 'private',
    access_type         content_access NOT NULL DEFAULT 'free',

    -- State
    is_primary          BOOLEAN NOT NULL DEFAULT FALSE,   -- primary file for a project
    is_processed        BOOLEAN NOT NULL DEFAULT FALSE,   -- transcoding/thumbnail done
    processing_error    TEXT,

    search_vector       TSVECTOR,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ
);


