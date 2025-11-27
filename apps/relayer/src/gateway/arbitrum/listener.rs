use tracing::{error, info};

use crate::core::event::{
    ApiCategory, ApiVersion, GatewayChainEventData, RelayerEvent, RelayerEventData,
};
use crate::core::job_id::JobId;
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use crate::store::sql::repositories::block_number_repo::BlockNumberRepository;
use alloy::rpc::types::Log;
use futures::StreamExt;
use std::sync::Arc;

pub async fn arbitrum_listener(
    mut subscription: alloy::pubsub::SubscriptionStream<Log>,
    orchestrator: Arc<
        Orchestrator<
            impl EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
            RelayerEvent,
        >,
    >,
    block_number_repo: Arc<BlockNumberRepository>,
) {
    loop {
        tokio::select! {
            event = subscription.next() => match event {
                Some(event_log) => {
                    let tx_hash = event_log.transaction_hash.expect("Event log must have transaction hash");
                    let event = RelayerEvent::new(
                        JobId::from_uuid_v7(orchestrator.new_internal_request_id()),
                        ApiVersion {
                            category: ApiCategory::PRODUCTION,
                            number: 1,
                        },
                        RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd {
                            log: event_log.clone(),
                            tx_hash
                        }),
                    );
                    orchestrator.dispatch_event(event).await.unwrap_or_else(|e| {
                        error!(
                            error = %e,
                            "dispatching event"
                        );
                    });

                    if let Some(block_number) = event_log.block_number {
                        let block_hash = event_log.block_hash
                            .map(|h| format!("{:#x}", h))
                            .unwrap_or_else(|| "0x0".to_string());

                        // Try to update first, if that fails (no row exists), insert
                        if block_number_repo.update_block_info(block_number, block_hash.clone()).await.is_err() {
                            block_number_repo.insert_initial_block_info(block_number, block_hash).await.unwrap_or_else(|e| {
                                error!(
                                    error = %e,
                                    "inserting initial block info"
                                );
                            });
                        }
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
