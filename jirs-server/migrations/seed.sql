INSERT INTO projects (name) values ('initial');
INSERT INTO users (project_id, email, name) values (1, 'foo', 'bar');
INSERT INTO tokens (user_id, access_token, refresh_token) values (1, uuid_generate_v4(), uuid_generate_v4() );

