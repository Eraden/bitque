ALTER TABLE invitations ADD COLUMN bind_token UUID NOT NULL DEFAULT uuid_generate_v4();
