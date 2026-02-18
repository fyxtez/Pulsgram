CREATE TABLE IF NOT EXISTS chats (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name         TEXT NOT NULL UNIQUE,
    chat_id      TEXT NOT NULL UNIQUE
);

CREATE INDEX IF NOT EXISTS idx_chats_name ON chats (name);