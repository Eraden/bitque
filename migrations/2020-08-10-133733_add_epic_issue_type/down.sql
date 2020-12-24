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
