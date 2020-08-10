ALTER TABLE "issues"
    ALTER COLUMN "issue_type"
        SET DATA TYPE TEXT
        USING "issue_type"::TEXT;

DROP TYPE IF EXISTS "IssueTypeType" CASCADE;
CREATE TYPE "IssueTypeType" AS ENUM (
    'task',
    'bug',
    'story',
    'epic'
    );

ALTER TABLE "issues"
    ALTER COLUMN "issue_type"
        SET DATA TYPE "IssueTypeType"
        USING "issue_type"::"IssueTypeType";
