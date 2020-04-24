alter TABLE issues alter COLUMN estimate SET DATA TYPE integer;
alter TABLE issues alter COLUMN time_spent SET DATA TYPE integer;
alter TABLE issues alter COLUMN time_remaining SET DATA TYPE integer;

alter TABLE projects drop COLUMN time_tracking;

drop TYPE IF EXISTS "TimeTrackingType";
