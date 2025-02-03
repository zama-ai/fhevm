use crate::orchestrator::event::traits::Event;
use crate::orchestrator::event_dispatcher::traits::{Dispatcher, HandleRegistry};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;

pub struct Orchestrator<D: Dispatcher<E> + HandleRegistry<E>, E: Event> {
    /// Used for generation UUIDs to uniquely identify incoming requests.
    /// TODO: Document more details.
    pub uuid_generator: UuidGenerator,

    pub event_dispatcher: Arc<D>,

    _marker: std::marker::PhantomData<E>,
}

impl<D: Dispatcher<E> + HandleRegistry<E>, E: Event> Orchestrator<D, E> {
    pub fn new(event_dispatcher: Arc<D>, node_id: &[u8]) -> Self {
        Self {
            uuid_generator: UuidGenerator::new(node_id, DEFAULT_UUID_CONTEXT_INITIAL_VALUE),
            event_dispatcher,
            _marker: std::marker::PhantomData,
        }
    }
}

pub struct UuidGenerator {
    /// Context holds a thread-safe counter that will be used to ensure
    /// uniqueness of generated UUIDs across threads, even if timestamps match
    /// by chance.
    /// Its initial value can be randomly chosen, say even to 0.
    /// It need not be same across different instances or processes of the application.
    context: Context,

    // Node ID uniquely identifies this node for UUID generation and should be
    // unique for each instance or process of the application.
    node_id: Vec<u8>,
}

/// Randomly chosen value. See `UuidGenerator::context` for details.
pub const DEFAULT_UUID_CONTEXT_INITIAL_VALUE: u16 = 0;

impl UuidGenerator {
    pub fn new(node_id: &[u8], context_initial_value: u16) -> Self {
        Self {
            context: Context::new(context_initial_value),
            node_id: node_id.to_vec(),
        }
    }

    pub fn generate_id(&self) -> Uuid {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let ts = Timestamp::from_unix(&self.context, now.as_secs(), now.subsec_nanos());
        Uuid::new_v1(ts, &self.node_id).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::orchestrator::event::relayer_event::{self, RelayerEvent};
    use crate::orchestrator::event::traits::Event;
    use crate::orchestrator::event_dispatcher::tokio_dispatcher::TokioDispatcher;
    use crate::orchestrator::event_dispatcher::traits::EventHandler;

    #[test]
    fn test_uuid_generator() {
        let node_id = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab];
        let generator = UuidGenerator::new(&node_id, DEFAULT_UUID_CONTEXT_INITIAL_VALUE);

        let id1 = generator.generate_id();
        let id2 = generator.generate_id();

        assert_ne!(id1, id2);
    }

    struct SimpleEventHandler;

    impl EventHandler<RelayerEvent> for SimpleEventHandler {
        fn handle(&self, event: RelayerEvent) {
            println!("Handling event: {:?}", event.event_name());
        }
    }

    #[tokio::test]
    async fn test_orchestrator() {
        let node_id = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab];

        let pubsub = Arc::new(TokioDispatcher::<RelayerEvent>::new());
        let orchestrator = Orchestrator::new(pubsub, &node_id);

        let id = orchestrator.uuid_generator.generate_id();

        let event = relayer_event::RelayerEvent::new(
            id,
            relayer_event::ApiVersion {
                category: relayer_event::ApiCategory::PRODUCTION,
                number: 1,
            },
            relayer_event::RelayerEventData::DecryptionFailed {
                error: "dummy error".to_string(),
            },
        );

        let handler = Arc::new(SimpleEventHandler);
        orchestrator
            .event_dispatcher
            .register_handler(event.event_id(), handler);
        orchestrator.event_dispatcher.dispatch(event).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}
