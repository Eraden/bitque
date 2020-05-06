DROP TYPE IF EXISTS "IssueStatusType" CASCADE;
CREATE TYPE "IssueStatusType" AS ENUM (
    'backlog',
    'selected',
    'in_progress',
    'done'
);
ALTER TABLE issues ADD COLUMN status "IssueStatusType";
UPDATE issues
SET status = issue_statuses.name :: "IssueStatusType"
FROM issue_statuses
WHERE issue_statuses.id = issues.issue_status_id;

ALTER TABLE issues DROP COLUMN issue_status_id;
ALTER TABLE issues ALTER COLUMN status SET NOT NULL;
DROP TABLE issue_statuses;
