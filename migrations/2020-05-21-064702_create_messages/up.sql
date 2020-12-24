CREATE TABLE messages (
    id serial primary key not null,
    receiver_id int not null references users (id),
    sender_id int not null references users (id),
    summary text not null,
    description text not null,
    message_type text not null,
    hyper_link text not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);
