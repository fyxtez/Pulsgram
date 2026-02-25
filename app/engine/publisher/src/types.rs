use telegram_types::Message;

#[derive(Debug, Clone)]
pub enum EventTag {
    Token,
    Other,
}

// enum size = size of largest variant + discriminant
// ----------------- the largest variant contains at least 7480 bytes (Message)
// Heap memory simplified = dynamically allocated memory, accessed via pointer.
// Tradeoff of heap memory: + smaller enum size, - extra memory lookup and slightly slower access.
#[derive(Debug, Clone)]
pub struct TgEvent {
    // Box is required because `Message` is very large (~7.5 KB).
    // Without boxing, every PulsgramEvent becomes 7.5 KB in size,
    // since enum size equals the largest variant.
    //
    // Boxing moves the large data to the heap and keeps only
    // an 8-byte pointer inside the enum.
    pub message: Box<Message>,
    //Visual Comparison
    //  Without Box

    // Publishing one event:

    // Clone 7.5 KB
    // Clone 7.5 KB
    // Clone 7.5 KB
    // Clone 7.5 KB
    // (for each subscriber)

    // Enum size = 7.5 KB.

    //  With Box

    // Publishing one event:

    // Clone pointer (8 bytes)
    // Clone pointer (8 bytes)
    // Clone pointer (8 bytes)
    // Clone pointer (8 bytes)
    // (for each subscriber)
}

#[derive(Debug, Clone)]
pub struct ErrorEvent {
    pub source: &'static str,
    pub message_text: String,
}

#[derive(Debug, Clone)]
pub struct TradeEvent {
    pub symbol: String,
    pub entry: f64,
}

#[derive(Debug, Clone)]
pub enum PulsgramEvent {
    Telegram(TgEvent),
    Error(ErrorEvent),
    Trade(TradeEvent),
}
