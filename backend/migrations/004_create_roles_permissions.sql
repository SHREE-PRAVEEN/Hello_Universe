-- -------------------------------------------------------
-- ROLES (platform-level role definitions)
-- -------------------------------------------------------
CREATE TABLE roles (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name        VARCHAR(80)  NOT NULL UNIQUE,  -- 'admin', 'developer', 'client', etc.
    slug        VARCHAR(80)  NOT NULL UNIQUE,
    description TEXT,
    scope       permission_scope NOT NULL DEFAULT 'platform',
    is_system   BOOLEAN NOT NULL DEFAULT FALSE,  -- system roles cannot be deleted
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- -------------------------------------------------------
-- PERMISSIONS
-- -------------------------------------------------------
CREATE TABLE permissions (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name        VARCHAR(150) NOT NULL UNIQUE,  -- e.g. 'projects.publish'
    description TEXT,
    module      VARCHAR(80)  NOT NULL,          -- e.g. 'projects', 'media', 'commerce'
    action      VARCHAR(80)  NOT NULL,          -- e.g. 'read', 'write', 'delete', 'publish'
    scope       permission_scope NOT NULL DEFAULT 'platform',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- -------------------------------------------------------
-- ROLE <-> PERMISSION (many-to-many)
-- -------------------------------------------------------
CREATE TABLE role_permissions (
    role_id       UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    granted_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    granted_by    UUID REFERENCES users(id),
    PRIMARY KEY (role_id, permission_id)
);

-- -------------------------------------------------------
-- USER <-> ROLE (many-to-many, platform-level)
-- -------------------------------------------------------
CREATE TABLE user_roles (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id     UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    granted_by  UUID REFERENCES users(id),
    granted_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at  TIMESTAMPTZ,
    UNIQUE (user_id, role_id)
);

