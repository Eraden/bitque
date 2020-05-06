CREATE TABLE issue_statuses (
    id serial PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    position int NOT NULL,
    project_id int NOT NULL REFERENCES projects (id),
    created_at timestamp NOT NULL DEFAULT now(),
    updated_at timestamp NOT NULL DEFAULT now()
);

INSERT INTO issue_statuses (name, project_id, position)
SELECT 'backlog', id, 1
FROM projects;

INSERT INTO issue_statuses (name, project_id, position)
SELECT 'selected', id, 2
FROM projects;

INSERT INTO issue_statuses (name, project_id, position)
SELECT 'in_progress', id, 3
FROM projects;

INSERT INTO issue_statuses (name, project_id, position)
SELECT 'done', id, 4
FROM projects;

ALTER TABLE issues
ADD COLUMN issue_status_id INT REFERENCES issue_statuses ( id );

UPDATE issues
SET issue_status_id = issue_statuses.id
FROM issue_statuses
WHERE issue_statuses.name = issues.status :: text;

ALTER TABLE issues DROP COLUMN status;
ALTER TABLE issues ALTER COLUMN issue_status_id SET NOT NULL;

DROP TYPE IF EXISTS "IssueStatusType";
