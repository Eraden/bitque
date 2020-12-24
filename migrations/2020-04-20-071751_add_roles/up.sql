DROP TYPE IF EXISTS "UserRoleType" CASCADE;
CREATE TYPE "UserRoleType" AS ENUM (
    'user',
    'manager',
    'owner'
);

ALTER TABLE users ADD COLUMN role "UserRoleType" DEFAULT 'user' NOT NULL;
