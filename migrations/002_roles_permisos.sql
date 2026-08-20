-- CREATE TABLE roles (
--     id UUID PRIMARY KEY,
--     name TEXT UNIQUE NOT NULL,
--     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
-- );

-- CREATE TABLE permissions (
--     id UUID PRIMARY KEY,
--     name TEXT UNIQUE NOT NULL
-- );

-- CREATE TABLE role_permissions (
--     role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
--     permission_id UUID NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
--     PRIMARY KEY (role_id, permission_id)
-- );

