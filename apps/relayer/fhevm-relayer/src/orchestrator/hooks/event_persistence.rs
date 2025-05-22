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

impl<E> PreDispatchHook<E> for EventPersistenceHook<E>
where
    E: Event + serde::Serialize + for<'de> serde::Deserialize<'de> + Send + Sync + 'static,
{
    fn run(&self, event: E) {
        let event_store = self.event_store.clone();

        tokio::spawn(async move {
            match event_store.persist_event(event.clone()).await {
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
        });
    }
}
