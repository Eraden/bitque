BEGIN;

ALTER TABLE users ADD COLUMN role "UserRoleType" DEFAULT 'user' NOT NULL;
ALTER TABLE users ADD COLUMN project_id int;

UPDATE users
SET project_id = user_projects.project_id,
    role = user_projects.role
FROM user_projects
INNER JOIN user_projects
ON user_projects.user_id = users.id;

DROP TABLE user_projects;

ALTER TABLE users
ALTER COLUMN project_id
ADD CONSTRAINT users_project_id_fkey
FOREIGN KEY (project_id)
REFERENCES projects (id)
MATCH FULL;

COMMIT;
