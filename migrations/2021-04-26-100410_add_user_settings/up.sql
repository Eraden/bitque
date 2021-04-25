CREATE TYPE "TextEditorModeType" AS ENUM (
    'md_only',
    'rte_only',
    'mixed'
    );

CREATE TABLE IF NOT EXISTS user_settings
(
    id               serial                                 not null unique primary key,
    user_id          int references users (id)              not null,
    text_editor_mode "TextEditorModeType" DEFAULT 'md_only' NOT NULL
);
