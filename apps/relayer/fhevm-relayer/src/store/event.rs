use anyhow::{anyhow, Result};
use serde_json;
use std::sync::Arc;
use tracing::{debug, error};
use uuid::Uuid;

use crate::orchestrator::traits::Event;
use crate::store::key_value_db::KVStore;

/// EventStore provides a data translation layer for storing and retrieving
/// events using a key value store.
pub struct EventStore<E: Event + serde::Serialize + for<'de> serde::Deserialize<'de>> {
    kv_store: Arc<dyn KVStore>,
    _phantom: std::marker::PhantomData<E>,
}

impl<E> EventStore<E>
where
    E: Event + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    /// Create a new EventStore with the given key-value store backend.
    pub fn new(kv_store: Arc<dyn KVStore>) -> Self {
        Self {
            kv_store,
            _phantom: std::marker::PhantomData,
        }
    }

    // Helper to build key from request_id and event_id with padding
    fn build_key(request_id: &Uuid, event_id: u8) -> String {
        format!("{}-{:04}", request_id, event_id)
    }

    // Helper to build a key prefix for a specific request ID
    fn build_request_prefix(request_id: &Uuid) -> String {
        format!("{}-", request_id)
    }

    // Helper to build latest event ID key for a request ID
    fn build_latest_event_id_key(request_id: &Uuid) -> String {
        format!("latest:{}", request_id)
    }

    /// Persist an event.
    pub async fn persist_event(&self, event: E) -> Result<()> {
        let request_id = event.request_id();
        let event_id = event.event_id();

        // Serialize the event to JSON
        let value = serde_json::to_string(&event)?;

        // Store the event with a key that includes the request ID and event ID
        let key = Self::build_key(&request_id, event_id);
        self.kv_store.put(&key, &value).await?;

        // Update the latest event ID for this request
        let latest_key = Self::build_latest_event_id_key(&request_id);
        self.kv_store
            .put(&latest_key, &event_id.to_string())
            .await?;
        Ok(())
    }

    /// Retrieve the latest event for a given request ID.
    pub async fn get_latest_event(&self, request_id: Uuid) -> Result<Option<E>> {
        // Get the latest event ID for this request
        let latest_key = Self::build_latest_event_id_key(&request_id);

        if let Some(event_id_str) = self.kv_store.get(&latest_key).await? {
            // Parse the event ID
            let event_id = event_id_str
                .parse::<u8>()
                .map_err(|e| anyhow!("Invalid event ID format: {}", e))?;

            // Get the event using the request ID and event ID
            let key = Self::build_key(&request_id, event_id);
            if let Some(value) = self.kv_store.get(&key).await? {
                let event = serde_json::from_str(&value)?;
                return Ok(Some(event));
            }
        }

        Ok(None)
    }

    /// Retrieve all events for a given request ID.
    pub async fn get_all_events(&self, request_id: Uuid) -> Result<Vec<E>> {
        let prefix = Self::build_request_prefix(&request_id);
        let pairs = self.kv_store.get_by_prefix(&prefix).await?;

        // Convert all values to Event objects
        let mut events = Vec::new();
        for (_, value) in pairs {
            match serde_json::from_str::<E>(&value) {
                Ok(event) => events.push(event),
                Err(e) => error!("Failed to deserialize event: {}", e),
            }
        }

        // Sort events by event ID to ensure consistent ordering
        events.sort_by_key(|a| a.event_id());

        debug!(
            "Retrieved {} events for request_id={}",
            events.len(),
            request_id
        );
        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::key_value_db::InMemoryKVStore;

    #[cfg(test)]
    use crate::core::test_utils::MockEvent;

    #[tokio::test]
    async fn test_event_store_inmemorykvstore() {
        let kv_store = InMemoryKVStore::new();
        let event_store = EventStore::<MockEvent>::new(kv_store);

        let request_id = uuid::Uuid::new_v4();

        // Create and persist some test events
        let event1 = MockEvent::new(request_id, 1, "Event1");
        let event2 = MockEvent::new(request_id, 2, "Event2");

        // Test persist_event
        event_store.persist_event(event1.clone()).await.unwrap();
        event_store.persist_event(event2.clone()).await.unwrap();

        // Test get_latest_event
        let latest = event_store.get_latest_event(request_id).await.unwrap();
        assert!(latest.is_some());
        assert_eq!(latest.as_ref().unwrap().event_id(), 2);
        assert_eq!(latest.as_ref().unwrap().event_name(), "Event2");

        // Test get_all_events
        let events = event_store.get_all_events(request_id).await.unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_id(), 1);
        assert_eq!(events[0].event_name(), "Event1");
        assert_eq!(events[1].event_id(), 2);
        assert_eq!(events[1].event_name(), "Event2");
    }
}
