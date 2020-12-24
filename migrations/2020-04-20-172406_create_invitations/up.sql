create type "InvitationStateType" AS ENUM (
    'sent',
    'accepted',
    'revoked'
);

create table invitations (
    id serial primary key not null,
    name text not null,
    email text not null,
    state "InvitationStateType" not null default 'sent',
    project_id integer not null references projects (id),
    invited_by_id integer not null references users (id),
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);
