use telegram_types::Message;

#[derive(Debug, Clone)]
pub enum EventTag {
    Token,
    Other,
}

#[derive(Debug, Clone)]
pub struct TgEvent {
    pub message: Message,
    pub tag: EventTag,
}