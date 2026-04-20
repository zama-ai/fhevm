use std::collections::HashMap;

/// Unified message envelope for all queue backends.
///
/// Replaces `&[u8]` (RMQ) and `&RedisMessage` (Redis) with a single type that
/// both backends construct before calling the handler.
#[derive(Debug, Clone)]
pub struct Message {
    /// Raw payload bytes — the serialized application data.
    pub payload: Vec<u8>,
    /// Backend-agnostic delivery metadata.
    pub metadata: MessageMetadata,
}

/// Delivery metadata attached to every [`Message`].
///
/// Each backend populates these fields from its native concepts:
/// - RMQ: `id` = delivery tag, `topic` = queue name, `delivery_count` = x-death count + 1
/// - Redis: `id` = stream entry ID, `topic` = stream name, `delivery_count` = XPENDING delivery count
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageMetadata {
    /// Unique message identifier within the backend.
    pub id: String,
    /// The topic/queue/stream this message was consumed from.
    pub topic: String,
    /// How many times this message has been delivered (1 = first delivery).
    pub delivery_count: u64,
    /// Arbitrary key-value headers (RMQ properties or Redis entry fields).
    pub headers: HashMap<String, String>,
}

impl MessageMetadata {
    /// Create metadata with no headers.
    pub fn new(id: impl Into<String>, topic: impl Into<String>, delivery_count: u64) -> Self {
        Self {
            id: id.into(),
            topic: topic.into(),
            delivery_count,
            headers: HashMap::new(),
        }
    }
}
