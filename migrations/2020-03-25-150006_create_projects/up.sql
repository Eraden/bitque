DROP TABLE IF EXISTS projects CASCADE;
CREATE TABLE projects (
    id serial primary key not null,
    name text not null,
    url text not null default '',
    description text not null default '',
    category text not null default 'software',
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);
