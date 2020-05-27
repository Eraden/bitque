insert into projects (name) values ('initial'), ('second'), ('third');

insert into issue_statuses (name, project_id, position)
values ('backlog', 1, 1), ('selected', 1, 2), ('in_progress', 1, 3), ('done', 1, 4);

insert into users (email, name, avatar_url) values (
    'john@example.com',
    'John Doe',
    'http://cdn.onlinewebfonts.com/svg/img_553934.png'
), (
    'kate@exampe.com',
    'Kate Snow',
    'http://www.asthmamd.org/images/icon_user_6.png'
), (
    'mike@example.com',
    'Mike Keningham',
    'https://cdn0.iconfinder.com/data/icons/user-pictures/100/matureman1-512.png'
);
insert into user_projects (user_id, project_id, role, is_current, is_default) values (
    1, 1, 'owner', true, true
), (
    2, 1, 'owner', true, true
), (
    3, 1, 'owner', true, true
);
insert into invitations (email, name, state, project_id, invited_by_id) values (
    'foo1@example.com',
    'Foo1',
    'sent',
    1,
    1
), (
    'foo1+revoked@example.com',
    'Foo1 Revoked',
    'revoked',
    1,
    1
), (
    'foo1+accepted@example.com',
    'Foo1 Accepted',
    'accepted',
    1,
    1
), (
    'foo2@example.com',
    'Foo2',
    'sent',
    2,
    2
), (
    'foo2+accepted@example.com',
    'Foo2 Accepted',
    'accepted',
    2,
    2
), (
    'foo2+revoked@example.com',
    'Foo2 Revoked',
    'revoked',
    2,
    2
);
insert into users (project_id, email, name) values (
    2,
    'foo2+accepted@example.com',
    'Foo2 Accepted'
), (
    1,
    'foo1+accepted@example.com',
    'Foo1 Accepted'
);
insert into tokens (user_id, access_token, refresh_token) values (1, uuid_generate_v4(), uuid_generate_v4() );
insert into issues(
    title,
    issue_type,
    priority,
    list_position,
    description,
    description_text,
    reporter_id,
    project_id,
    issue_status_id
) values (
    'Foo',
    'backlog',
    'low',
    1,
    'hello world',
    'foz baz',
    1,
    1,
    1
), (
    'Foo2',
    'selected',
    'low',
    2,
    'hello world 2',
    'foz baz 2',
    1,
    1,
    2
), (
    'Foo3',
    'in_progress',
    'low',
    3,
    'hello world 3',
    'foz baz 3',
    2,
    1,
    3
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

/*
'received_invitation',
'assigned_to_issue',
'mention'
*/
INSERT INTO messages (receiver_id, sender_id, summary, description, message_type, hyper_link, created_at, updated_at)
VALUES (
    1, 1,
    'Foo',
    'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec elit ligula, tempor non nunc eget, maximus elementum enim. Nam ut elit nibh. Nunc aliquet lectus mi, venenatis porttitor turpis rhoncus a. Aliquam tempus, eros lobortis fermentum dapibus, felis sapien tristique mi, ut porta odio lectus quis nisi. Etiam laoreet quis quam vitae iaculis. Vivamus sagittis luctus urna, porttitor fermentum ligula aliquam in. Suspendisse scelerisque nunc id congue elementum. Aliquam erat volutpat. Integer pretium mi in quam varius lacinia. Nullam vitae justo eu mauris congue posuere. Morbi leo mi, varius eu nisl nec, laoreet scelerisque nisl. Fusce in nisi in felis varius fermentum et ac odio. Curabitur sit amet suscipit quam.',
    'received_invitation',
    '',
    now(), now()
), (
    1, 2,
    'Bar',
    'Suspendisse tincidunt euismod justo, at porttitor dolor fermentum ut. Interdum et malesuada fames ac ante ipsum primis in faucibus. Suspendisse maximus sed ex ut sollicitudin. Etiam volutpat ultricies vehicula. Sed at est in mauris cursus fermentum. Duis et lacus metus. Sed ut egestas ipsum, ac consectetur metus. In felis diam, cursus eu felis non, tincidunt elementum lacus. Etiam et massa odio. Vestibulum ornare felis maximus facilisis semper.',
    'assigned_to_issue',
    '/issue/1',
    now(), now()
), (
    2, 1,
    'Foz Baz',
    'Suspendisse quam ligula, @<John Doe> auctor vel diam sit amet, tincidunt venenatis justo. Vestibulum tincidunt mauris et est iaculis, vel consequat turpis porta. Integer eu urna quis diam pharetra lobortis vel nec lacus. Donec ac mollis risus. Morbi pellentesque pulvinar libero, sit amet finibus risus fermentum ac. Vivamus imperdiet mi congue ligula luctus condimentum. Duis arcu turpis, dignissim quis purus eget, dignissim elementum risus. Donec mattis rhoncus lorem quis blandit. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec dignissim tellus eu cursus finibus. Ut pellentesque mi at eros maximus, eu tempor est sodales. Mauris vel feugiat ligula. Integer quis interdum velit, at iaculis arcu. Duis leo sapien, egestas eget erat id, fringilla pulvinar nulla. Nam sollicitudin ullamcorper finibus.',
    'mention',
    '',
    now(), now()
);

select * from tokens;
