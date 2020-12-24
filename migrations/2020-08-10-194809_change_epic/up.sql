ALTER TABLE "issues"
    ALTER COLUMN "issue_type"
        SET DATA TYPE TEXT
        USING "issue_type"::TEXT;

UPDATE "issues"
SET "issue_type" = 'task'
WHERE "issue_type" = 'epic';

DROP TYPE IF EXISTS "IssueTypeType" CASCADE;
CREATE TYPE "IssueTypeType" AS ENUM (
    'task',
    'bug',
    'story'
    );

ALTER TABLE "issues"
    ALTER COLUMN "issue_type"
        SET DATA TYPE "IssueTypeType"
        USING "issue_type"::"IssueTypeType";

CREATE TABLE epics (
    id serial primary key not null,
    name text not null,
    user_id integer not null references users (id),
    project_id integer not null references projects (id),
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);

ALTER TABLE "issues"
ADD COLUMN "epic_id" integer
REFERENCES "epics" ( "id" ) NULL;
