CREATE TABLE issue_assignees (
    id serial primary key not null,
    issue_id integer not null references issues (id),
    user_id integer not null references users (id),
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);
