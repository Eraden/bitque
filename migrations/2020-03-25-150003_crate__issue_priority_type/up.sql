DROP TYPE IF EXISTS "IssuePriorityType" CASCADE;
CREATE TYPE "IssuePriorityType" as ENUM (
    'highest',
    'high',
    'medium',
    'low',
    'lowest'
);
