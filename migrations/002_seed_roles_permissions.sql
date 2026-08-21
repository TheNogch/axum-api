-- Roles base
INSERT INTO roles (id, name) VALUES
    ('00000000-0000-0000-0000-000000000001', 'user'),
    ('00000000-0000-0000-0000-000000000002', 'admin');

-- Catálogo de permisos
INSERT INTO permissions (id, name) VALUES
    ('00000000-0000-0000-0000-000000000101', 'users:read'),
    ('00000000-0000-0000-0000-000000000102', 'users:write'),
    ('00000000-0000-0000-0000-000000000103', 'users:delete'),
    ('00000000-0000-0000-0000-000000000104', 'roles:manage');

-- El rol admin tiene todos los permisos
INSERT INTO role_permissions (role_id, permission_id)
SELECT '00000000-0000-0000-0000-000000000002', id FROM permissions;

-- El rol "user" no tiene permisos explícitos:
-- el acceso a sus propios datos (/users/me) no depende de este sistema de permisos,
-- se resuelve simplemente comparando el id del token con el recurso solicitado.