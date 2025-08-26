mod tests {

    use super::*;
    use crate::orchestrator::event::traits::Event;
    use crate::orchestrator::event_dispatcher::tokio_dispatcher::TokioDispatcher;
    use crate::orchestrator::event_dispatcher::traits::EventHandler;
    use crate::relayer_event::{self, RelayerEvent};

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
        _ = orchestrator.event_dispatcher.dispatch(event).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}
