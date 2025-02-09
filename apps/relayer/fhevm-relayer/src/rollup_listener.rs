use tracing::info;

use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use crate::relayer_event::{self, RelayerEvent};
use alloy::rpc::types::Log;
use futures_util::StreamExt;
use std::sync::Arc;

pub async fn event_listener_rollup(
    mut subscription: alloy::pubsub::SubscriptionStream<Log>,
    orchestrator: Arc<
        Orchestrator<
            impl EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
            RelayerEvent,
        >,
    >,
) {
    loop {
        tokio::select! {
            event = subscription.next() => match event {
                Some(event_log) => {

                    info!("rollup listener catches one event");

                    let id = orchestrator.new_request_id();
                    let event = RelayerEvent::new(
                        id,
                        relayer_event::ApiVersion {
                            category: relayer_event::ApiCategory::PRODUCTION,
                            number: 1,
                        },
                        relayer_event::RelayerEventData::DecryptResponseEventLogRcvdFromGwL2  {
                            log:event_log,
                        },
                    );
                    _ = orchestrator.dispatch_event(event).await; // TODO: Proper error handling and make it aync.
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
    }
}
