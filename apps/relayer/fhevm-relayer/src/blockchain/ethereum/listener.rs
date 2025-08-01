use tracing::{error, info};

use crate::core::event::{
    ApiCategory, ApiVersion, GenericEventData, RelayerEvent, RelayerEventData,
};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use crate::store::BlockNumberStore;
use alloy::rpc::types::Log;
use futures_util::StreamExt;
use std::sync::Arc;

pub type EventLogConverter = fn(Log) -> RelayerEventData;

pub fn gateway_event_log_converter(log: Log) -> RelayerEventData {
    RelayerEventData::Generic(GenericEventData::EventLogFromGw { log })
}

pub fn fhevm_event_log_converter(log: Log) -> RelayerEventData {
    RelayerEventData::Generic(GenericEventData::EventLogFromFhevm { log })
}

pub async fn ethereum_listener(
    mut subscription: alloy::pubsub::SubscriptionStream<Log>,
    log_converter: EventLogConverter,
    orchestrator: Arc<
        Orchestrator<
            impl EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
            RelayerEvent,
        >,
    >,
    block_number_store: Arc<BlockNumberStore>,
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
                        log_converter(event_log.clone()),
                    );
                    orchestrator.dispatch_event(event).await.unwrap_or_else(|e| {
                        error!(
                            error = %e,
                            "dispatching event"
                        );
                    });

                    if let Some(block_number) = event_log.block_number {
                        block_number_store.persist_last_block_number(block_number).await.unwrap_or_else(|e| {
                            error!(
                                error = %e,
                                "persisting last block number"
                            );
                        });
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
