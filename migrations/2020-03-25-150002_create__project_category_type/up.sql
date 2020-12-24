DROP TYPE IF EXISTS "ProjectCategoryType" CASCADE;
CREATE TYPE "ProjectCategoryType" as ENUM (
    'software',
    'marketing',
    'business'
);
