CREATE TYPE "TimeTrackingType" AS ENUM (
    'untracked',
    'fibonacci',
    'hourly'
);

ALTER TABLE issues ALTER COLUMN estimate SET DATA TYPE double precision;
ALTER TABLE issues ALTER COLUMN time_spent SET DATA TYPE double precision;
ALTER TABLE issues ALTER COLUMN time_remaining SET DATA TYPE double precision;

ALTER TABLE projects ADD COLUMN time_tracking "TimeTrackingType" NOT NULL DEFAULT 'untracked';
