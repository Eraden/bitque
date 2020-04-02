DROP TYPE IF EXISTS "IssueStatusType" CASCADE;
CREATE TYPE "IssueStatusType" AS ENUM (
    'backlog',
    'selected',
    'in_progress',
    'done'
);
