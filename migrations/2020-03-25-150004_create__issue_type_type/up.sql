DROP TYPE IF EXISTS "IssueTypeType" CASCADE;
CREATE TYPE "IssueTypeType" AS ENUM (
    'task',
    'bug',
    'story'
);
