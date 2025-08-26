use crate::orchestrator::traits::Event;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MockEvent {
    pub request_id: Uuid,
    pub event_id: u8,
    pub name: String,
    pub timestamp: u64,
}

impl MockEvent {
    pub fn new(request_id: Uuid, event_id: u8, name: &str) -> Self {
        let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => 0,
        };
        MockEvent {
            request_id,
            event_id,
            name: name.to_string(),
            timestamp,
        }
    }
}

impl Event for MockEvent {
    fn event_name(&self) -> &str {
        &self.name
    }
    fn event_id(&self) -> u8 {
        self.event_id
    }
    fn request_id(&self) -> Uuid {
        self.request_id
    }
    fn timestamp(&self) -> u64 {
        self.timestamp
    }
}
