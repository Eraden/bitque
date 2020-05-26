ALTER TABLE messages
ALTER COLUMN message_type
SET DATA TYPE text;
DROP TYPE "MessageTypeType";
