CREATE TABLE users (
    id serial primary key not null,
    name text not null,
    email text not null,
    avatar_url text,
    project_id integer not null references projects (id),
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);
