-- -------------------------------------------------------
-- SESSIONS / REFRESH TOKENS
-- -------------------------------------------------------
CREATE TABLE sessions (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    refresh_token   TEXT NOT NULL UNIQUE,           -- hashed refresh token
    user_agent      TEXT,
    ip_address      INET,
    device_id       VARCHAR(255),
    is_revoked      BOOLEAN NOT NULL DEFAULT FALSE,
    last_used_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at      TIMESTAMPTZ NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- -------------------------------------------------------
-- EMAIL VERIFICATION / PASSWORD RESET TOKENS
-- -------------------------------------------------------
CREATE TABLE auth_tokens (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash  TEXT NOT NULL UNIQUE,
    token_type  VARCHAR(50) NOT NULL,   -- 'email_verify', 'password_reset', 'invite'
    expires_at  TIMESTAMPTZ NOT NULL,
    used_at     TIMESTAMPTZ,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);