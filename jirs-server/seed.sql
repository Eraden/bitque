insert into projects (name) values ('initial'), ('second'), ('third');
insert into users (project_id, email, name, avatar_url) values (1, 'john@example.com', 'John Doe', 'http://cdn.onlinewebfonts.com/svg/img_553934.png'), (1, 'kate@exampe.com', 'Kate Snow', 'http://www.asthmamd.org/images/icon_user_6.png');
insert into tokens (user_id, access_token, refresh_token) values (1, uuid_generate_v4(), uuid_generate_v4() );
insert into issues(
    title,
    issue_type,
    status,
    priority,
    list_position,
    description,
    description_text,
    reporter_id,
    project_id
) values (
    'Foo',
    'task',
    'backlog',
    'low',
    1,
    'hello world',
    'foz baz',
    1,
    1
), (
    'Foo2',
    'story',
    'selected',
    'low',
    2,
    'hello world 2',
    'foz baz 2',
    1,
    1
), (
    'Foo3',
    'bug',
    'inprogress',
    'low',
    3,
    'hello world 3',
    'foz baz 3',
    2,
    1
);
select * from tokens;
