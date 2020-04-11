insert into projects (name) values ('initial'), ('second'), ('third');
insert into users (project_id, email, name, avatar_url) values (
    1, 'john@example.com', 'John Doe', 'http://cdn.onlinewebfonts.com/svg/img_553934.png
), (
    1, 'kate@exampe.com', 'Kate Snow', 'http://www.asthmamd.org/images/icon_user_6.png
), (
    1, 'mike@example.com', 'Mike Keningham', 'https://cdn0.iconfinder.com/data/icons/user-pictures/100/matureman1-512.png'
);
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
    'in_progress',
    'low',
    3,
    'hello world 3',
    'foz baz 3',
    2,
    1
);

insert into comments (user_id, issue_id, body) values (
    1, 1, 'Vestibulum non neque at dui maximus porttitor fermentum consectetur eros.'
    ),
(
    1, 2, 'Fusce varius ligula ut nisl porttitor, in gravida dolor rhoncus.'
    ),
(
    1, 3, 'Cras viverra urna at urna convallis maximus.'
    ),
(
    2, 1, 'Phasellus sollicitudin nisi eget arcu sollicitudin aliquam.'
    ),
(
    2, 2, 'Duis sodales felis in maximus tincidunt.'
    ),
(
    2, 3, 'Aenean sit amet sem sit amet dolor pellentesque rutrum.'
    ),
(
    3, 1, 'Phasellus placerat dui vitae odio mattis convallis.'
    ),
(
    3, 2, 'Suspendisse quis est eu neque vehicula sagittis.'
    ),
(
    3, 3, 'Duis rutrum quam eget maximus laoreet.'
    ),
(
    1, 1, 'Vestibulum eu ipsum a dui fringilla tristique.'
    ),
(
    1, 2, 'Phasellus porttitor dolor vitae urna aliquam porta.'
    ),
(
    2, 1, 'Curabitur volutpat mauris pretium urna laoreet, eget scelerisque neque fringilla.'
    ),
(
    1, 3, 'Curabitur ac arcu eu eros auctor elementum.'
    ),
(
    3, 1, 'Duis facilisis ipsum nec mi porta ultricies.'
    ),
(
    1, 1, 'In elementum orci nec mi porta imperdiet ut ac ante.'
    ),
(
    2, 3, 'Praesent et orci ut metus interdum sollicitudin.'
);


select * from tokens;
