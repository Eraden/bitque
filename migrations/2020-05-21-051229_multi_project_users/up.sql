BEGIN;

DROP TABLE IF EXISTS user_projects CASCADE;
CREATE TABLE user_projects (
    id serial primary key not null,
    user_id int not null references users (id),
    project_id int not null references projects (id),
    is_default bool  not null default false,
    is_current bool  not null default false,
    role "UserRoleType" not null default 'user',
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);

INSERT INTO user_projects (user_id, project_id, role, is_default, is_current)
SELECT id, project_id, role, true, true
FROM users;

ALTER TABLE users DROP COLUMN role;
ALTER TABLE users DROP COLUMN project_id;

COMMIT;
