use crate::core::job_id::JobId;
use crate::orchestrator::traits::Event;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MockEvent {
    pub job_id: JobId,
    pub event_id: u8,
    pub name: String,
    pub timestamp: u64,
}

impl MockEvent {
    pub fn new(job_id: JobId, event_id: u8, name: &str) -> Self {
        let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => 0,
        };
        MockEvent {
            job_id,
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
    fn job_id(&self) -> JobId {
        self.job_id
    }
    fn timestamp(&self) -> u64 {
        self.timestamp
    }
}
