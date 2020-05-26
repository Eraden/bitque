CREATE TYPE "MessageTypeType" AS ENUM (
    'received_invitation',
    'assigned_to_issue',
    'mention'
);

ALTER TABLE messages
ALTER COLUMN message_type
SET DATA TYPE "MessageTypeType"
USING message_type::text::"MessageTypeType";
