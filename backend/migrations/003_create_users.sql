-- -------------------------------------------------------
-- USERS
-- -------------------------------------------------------
CREATE TABLE users (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username            VARCHAR(80)  NOT NULL UNIQUE,
    email               VARCHAR(255) NOT NULL UNIQUE,
    email_verified_at   TIMESTAMPTZ,
    password_hash       TEXT         NOT NULL,
    display_name        VARCHAR(150),
    avatar_url          TEXT,
    bio                 TEXT,
    website_url         TEXT,
    location            VARCHAR(150),
    status              user_status  NOT NULL DEFAULT 'pending_verification',
    is_superadmin       BOOLEAN      NOT NULL DEFAULT FALSE,
    metadata            JSONB        NOT NULL DEFAULT '{}',   -- extra profile KVs
    search_vector       TSVECTOR,                             -- full-text index
    last_login_at       TIMESTAMPTZ,
    last_login_ip       INET,
    created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ
);

