use tracing::{debug, error, info};

use crate::core::event::{ApiCategory, ApiVersion, RelayerEvent, RelayerEventData};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use alloy::rpc::types::Log;
use futures_util::StreamExt;
use std::sync::Arc;

pub async fn event_listener(
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

                    let id = orchestrator.new_request_id();
                    let event = RelayerEvent::new(
                        id,
                        ApiVersion {
                            category: ApiCategory::PRODUCTION,
                            number: 1,
                        },
                        RelayerEventData::EventLogFromHostL1  {
                            event_log,
                        },
                    );
                    debug!(
                        file = file!(),
                        line = line!(),
                        event_id = ?id,
                        event_type = ?std::any::type_name_of_val(&event.data),
                        "Dispatching event"
                    );

                    // Dispatch with error logging
                    if let Err(e) = orchestrator.dispatch_event(event).await {
                        error!(
                            file = file!(),
                            line = line!(),
                            error = %e,
                            "Failed to dispatch event"
                        );
                    }
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
