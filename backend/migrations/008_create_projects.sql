



-- -------------------------------------------------------
-- PROJECTS
-- -------------------------------------------------------
CREATE TABLE projects (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    owner_id            UUID NOT NULL REFERENCES users(id),
    organization_id     UUID REFERENCES organizations(id),
    category_id         UUID REFERENCES project_categories(id),
    title               VARCHAR(300) NOT NULL,
    slug                VARCHAR(300) NOT NULL UNIQUE,
    short_description   VARCHAR(500),
    description         TEXT,
    status              project_status  NOT NULL DEFAULT 'draft',
    visibility          visibility      NOT NULL DEFAULT 'private',
    access_type         content_access  NOT NULL DEFAULT 'free',
    price               NUMERIC(12, 2)  CHECK (price >= 0),
    currency            CHAR(3)         NOT NULL DEFAULT 'USD',
    license             license_type    NOT NULL DEFAULT 'proprietary',
    license_details     TEXT,
    version_label       VARCHAR(50)     NOT NULL DEFAULT '1.0.0',  -- current published version label
    thumbnail_url       TEXT,
    demo_url            TEXT,
    repository_url      TEXT,
    download_count      BIGINT          NOT NULL DEFAULT 0,
    view_count          BIGINT          NOT NULL DEFAULT 0,
    like_count          BIGINT          NOT NULL DEFAULT 0,
    featured            BOOLEAN         NOT NULL DEFAULT FALSE,
    metadata            JSONB           NOT NULL DEFAULT '{}',
    search_vector       TSVECTOR,
    published_at        TIMESTAMPTZ,
    created_at          TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ,

    CONSTRAINT price_required_for_paid CHECK (
        access_type != 'paid' OR (price IS NOT NULL AND price > 0)
    )
);

