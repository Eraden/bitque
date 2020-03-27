CREATE EXTENSION "uuid-ossp";

CREATE TYPE ProjectCategory as ENUM (
    'software',
    'marketing',
    'business'
);

CREATE TABLE projects (
    id serial primary key not null,
    name text not null,
    url text not null default '',
    description text not null default '',
    category text not null default 'software',
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);

CREATE TABLE users (
    id serial primary key not null,
    name text not null,
    email text not null,
    avatar_url text,
    project_id integer not null references projects (id),
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);

CREATE TABLE issues (
    id serial primary key not null,
    title text not null,
    issue_type text not null,
    status text not null,
    priority text not null,
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

CREATE TABLE comments (
    id serial primary key not null,
    body text not null,
    user_id integer not null references users (id),
    issue_id integer not null references issues (id),
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);

CREATE TABLE tokens (
    id serial primary key not null,
    user_id integer not null references users (id),
    access_token uuid not null,
    refresh_token uuid not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);

