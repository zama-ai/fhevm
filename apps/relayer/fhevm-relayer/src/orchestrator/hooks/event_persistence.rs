use crate::orchestrator::traits::{Event, PreDispatchHook};
use crate::store::EventStore;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use tracing::{error, info};

/// A hook that persists events using the EventStore
pub struct EventPersistenceHook<E: Event + serde::Serialize + for<'de> serde::Deserialize<'de>> {
    name: String,
    event_store: Arc<EventStore<E>>,
}

impl<E> EventPersistenceHook<E>
where
    E: Event + serde::Serialize + for<'de> serde::Deserialize<'de> + 'static,
{
    pub fn new(event_store: Arc<EventStore<E>>) -> Arc<Self> {
        Arc::new(Self {
            name: "event_persistence".to_string(),
            event_store,
        })
    }
}

impl<E> Display for EventPersistenceHook<E>
where
    E: Event + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[async_trait::async_trait]
impl<E> PreDispatchHook<E> for EventPersistenceHook<E>
where
    E: Event + serde::Serialize + for<'de> serde::Deserialize<'de> + Send + Sync + 'static,
{
    async fn run(&self, event: E) {
        match self.event_store.persist_event(event.clone()).await {
            Ok(_) => {
                info!(
                    request_id = %event.request_id(),
                    event_name = %event.event_name(),
                    "Event persisted successfully"
                );
            }
            Err(e) => {
                error!("Failed to persist event: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_utils::MockEvent;
    use crate::store::key_value_db::InMemoryKVStore;
    use crate::store::EventStore;
    use std::sync::Arc;
    use uuid::Uuid;

    mod test_utils {
        use super::*;
        pub fn setup() -> (
            Arc<EventStore<MockEvent>>,
            Arc<EventPersistenceHook<MockEvent>>,
        ) {
            let kv_store = Arc::new(InMemoryKVStore::default());
            let event_store = Arc::new(EventStore::<MockEvent>::new(kv_store));
            let hook = EventPersistenceHook::new(event_store.clone());
            (event_store, hook)
        }

        pub async fn add_and_assert(
            hook: &Arc<EventPersistenceHook<MockEvent>>,
            store: &Arc<EventStore<MockEvent>>,
            request_id: Uuid,
            events: &[(u8, String)],
        ) {
            for (event_id, event_name) in events {
                let event = MockEvent::new(request_id, *event_id, event_name);
                hook.run(event.clone()).await;

                // Assert latest event is updated
                let latest = store.get_latest_event(request_id).await.unwrap();
                assert!(latest.is_some());
                assert_eq!(latest.as_ref().unwrap().event_id(), *event_id);
                assert_eq!(latest.as_ref().unwrap().event_name(), event_name);

                // Assert get_all_events returns correct number and order
                let all = store.get_all_events(request_id).await.unwrap();
                assert_eq!(all.len(), *event_id as usize);
                for (i, ev) in all.iter().enumerate() {
                    assert_eq!(ev.event_id(), (i as u8) + 1);
                    assert_eq!(ev.event_name(), &events[i].1);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_event_persistence_hook_single_request_id_multiple_events() {
        let (event_store, hook) = test_utils::setup();
        let request_id = Uuid::new_v4();
        let events: Vec<(u8, String)> = (1..=5)
            .map(|event_id| (event_id, format!("Event{}", event_id)))
            .collect();
        test_utils::add_and_assert(&hook, &event_store, request_id, &events).await;
    }

    #[tokio::test]
    async fn test_event_persistence_hook_multiple_request_ids() {
        let (event_store, hook) = test_utils::setup();
        let request_ids: Vec<Uuid> = (0..3).map(|_| Uuid::new_v4()).collect();

        for (idx, &rid) in request_ids.iter().enumerate() {
            let events: Vec<(u8, String)> = (1..=4)
                .map(|eid| (eid, format!("Set{}_Event{}", idx + 1, eid)))
                .collect();
            test_utils::add_and_assert(&hook, &event_store, rid, &events).await;
        }
    }
}
