CREATE TABLE tokens (
    id serial primary key not null,
    user_id integer not null references users (id),
    access_token uuid not null,
    refresh_token uuid not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);
