//TODO: Never use select * in real code. This is good.
pub const SELECT_ALL: &str = "SELECT id, name, chat_id FROM chats";

pub const SELECT_BY_ID: &str = "SELECT id, name, chat_id FROM chats WHERE id = $1";

pub const INSERT: &str = "INSERT INTO chats (name, chat_id)
         VALUES ($1, $2)
         RETURNING id, name, chat_id";
pub const DELETE: &str = "DELETE FROM chats WHERE id = $1";
