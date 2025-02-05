use tracing::info;

use crate::orchestrator::orchestrator::UuidGenerator;
use crate::orchestrator::traits::Dispatcher;
use crate::orchestrator::TokioEventDispatcher;
use crate::relayer_event::{self, RelayerEvent};
use alloy::rpc::types::Log;
use futures_util::StreamExt;
use std::sync::Arc;

pub async fn event_listener(
    mut subscription: alloy::pubsub::SubscriptionStream<Log>,
    dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
    uuid_generator: Arc<UuidGenerator>,
) {
    loop {
        let ethereum_events_listener = tokio::select! {
            event = subscription.next() => match event {
                Some(event_log) => {

                    let id = uuid_generator.generate_id();
                    let event = RelayerEvent::new(
                        id,
                        relayer_event::ApiVersion {
                            category: relayer_event::ApiCategory::PRODUCTION,
                            number: 1,
                        },
                        relayer_event::RelayerEventData::HostL1EventLogReceived {
                            log: event_log,
                        },
                    );
                    _ = dispatcher.dispatch(event).await; // TODO: Proper error handling and make it aync.
                }
                None => {
                    info!("Subscription stream ended");
                    break;
                }
            },
            _ = tokio::signal::ctrl_c() => {
                info!("Received ctrl + c signal, stopping...");
                break;
            }
        };
        ethereum_events_listener
    }
}
