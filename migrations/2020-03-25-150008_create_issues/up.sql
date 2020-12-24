CREATE TABLE issues (
    id serial primary key not null,
    title text not null,
    issue_type "IssueTypeType" NOT NULL DEFAULT 'task',
    status "IssueStatusType" NOT NULL DEFAULT 'backlog',
    priority "IssuePriorityType" NOT NULL DEFAULT 'low',
    list_position double precision not null default 0,
    description text,
    description_text text,
    estimate integer,
    time_spent integer,
    time_remaining integer,
    reporter_id integer not null references users (id),
    project_id integer not null references projects (id),
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);
