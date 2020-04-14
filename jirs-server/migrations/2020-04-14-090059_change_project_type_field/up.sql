ALTER TABLE projects
ALTER COLUMN category
DROP DEFAULT;

ALTER TABLE projects
ALTER COLUMN category
SET DATA TYPE "ProjectCategoryType"
USING category::text::"ProjectCategoryType";

ALTER TABLE projects
ALTER COLUMN category
SET DEFAULT 'software';
