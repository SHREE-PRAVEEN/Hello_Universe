-- -------------------------------------------------------
-- LICENSES (license templates)
-- -------------------------------------------------------
CREATE TABLE licenses (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name            VARCHAR(200) NOT NULL UNIQUE,
    slug            VARCHAR(150) NOT NULL UNIQUE,
    license_type    license_type NOT NULL,
    description     TEXT,
    terms_url       TEXT,
    allows_commercial    BOOLEAN NOT NULL DEFAULT FALSE,
    allows_modification  BOOLEAN NOT NULL DEFAULT FALSE,
    allows_distribution  BOOLEAN NOT NULL DEFAULT FALSE,
    requires_attribution BOOLEAN NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);