use alloy_sol_types::SolEvent;
use tracing::{error, info};

use crate::blockchain::ethereum::bindings::{DecyptionManager, ZKPoKManager};
use crate::core::event::{ApiCategory, ApiVersion, InputEventData, RelayerEvent, RelayerEventData};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use alloy::hex;
use alloy::primitives::FixedBytes;
use alloy::rpc::types::Log;
use futures_util::StreamExt;
use std::sync::Arc;
use tokio::time::sleep;

// Define event topics as constants

const PROOF_VERIFICATION_RESPONSE_TOPIC: alloy::primitives::FixedBytes<32> =
    ZKPoKManager::VerifyProofResponse::SIGNATURE_HASH;

const DECRYPTION_RESPONSE_TOPIC: alloy::primitives::FixedBytes<32> =
    DecyptionManager::PublicDecryptionResponse::SIGNATURE_HASH;

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
                    info!("rollup listener catches one event with topic {:?}", event_log.topic0());

                    let id = orchestrator.new_request_id();

                    // Determine event type based on topic
                    let event_data = if let Some(topic0) = event_log.topic0() {
                        // Convert B256 to FixedBytes<32>
                        let topic_bytes = FixedBytes::<32>::from_slice(topic0.as_slice());

                        match topic_bytes {
                        PROOF_VERIFICATION_RESPONSE_TOPIC => {
                            let sleep_time = 1;
                            info!("Received Proof Verification response event");
                            info!("Artificial sleep in anvil dev mode {}s", sleep_time);
                            tokio::time::sleep(tokio::time::Duration::from_secs(sleep_time)).await;
                            RelayerEventData::Input(
                                    InputEventData::EventLogResponseFromGwL2   {
                                        log: event_log
                                    }
                                )
                            },
                            DECRYPTION_RESPONSE_TOPIC => {
                                info!("Received Decryption response event");
                                RelayerEventData::EventLogResponseFromGwL2   {
                                            log: event_log
                                        }

                                },

                            _ => {
                                info!("Unknown event topic: 0x{}", hex::encode(topic0));
                                continue; // Skip unknown events
                            }
                        }
                    } else {
                        error!("Event log missing topic0");
                        continue;
                    };

                    let event = RelayerEvent::new(
                        id,
                        ApiVersion {
                            category: ApiCategory::PRODUCTION,
                            number: 1,
                        },
                        event_data,
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
