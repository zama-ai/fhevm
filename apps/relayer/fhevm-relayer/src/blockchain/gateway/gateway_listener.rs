use tracing::{error, info};

use crate::core::event::{
    ApiCategory, ApiVersion, GenericEventData, RelayerEvent, RelayerEventData,
};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use alloy::rpc::types::Log;
use futures_util::StreamExt;
use std::sync::Arc;

pub async fn event_listener_gateway(
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
                    let event = RelayerEvent::new(
                        orchestrator.new_request_id(),
                        ApiVersion {
                            category: ApiCategory::PRODUCTION,
                            number: 1,
                        },
                        RelayerEventData::Generic(GenericEventData::EventLogFromGw   {
                                                log: event_log
                                            }),
                    );

                    orchestrator.dispatch_event(event).await.unwrap_or_else(|e| {
                        error!("Failed to dispatch event: {e}");
                    });
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
