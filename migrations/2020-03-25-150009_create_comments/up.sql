CREATE TABLE comments (
    id serial primary key not null,
    body text not null,
    user_id integer not null references users (id),
    issue_id integer not null references issues (id),
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);
