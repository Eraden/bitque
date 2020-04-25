CREATE TYPE "TimeTrackingType" AS ENUM (
    'untracked',
    'fibonacci',
    'hourly'
);

ALTER TABLE projects ADD COLUMN time_tracking "TimeTrackingType" NOT NULL DEFAULT 'untracked';
