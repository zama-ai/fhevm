use alloy_sol_types::SolEvent;
use tracing::{error, info};

use crate::blockchain::ethereum::bindings::{Decryption, InputVerification};
use crate::gateway_processors_mock::event::{
    self, GatewayProcessorsEvent, GatewayProcessorsEventData, GatewayProcessorsInputEventData,
    PublicDecryptionEventData, UserDecryptionEventData,
};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use alloy::hex;
use alloy::primitives::FixedBytes;
use alloy::rpc::types::Log;
use futures_util::StreamExt;
use std::sync::Arc;

// Define event topics as constants
const PROOF_VERIFICATION_REQUEST_TOPIC: alloy::primitives::FixedBytes<32> =
    InputVerification::VerifyProofRequest::SIGNATURE_HASH;

const DECRYPTION_REQUEST_TOPIC: alloy::primitives::FixedBytes<32> =
    Decryption::PublicDecryptionRequest::SIGNATURE_HASH;

const USER_DECRYPTION_REQUEST_TOPIC: alloy::primitives::FixedBytes<32> =
    Decryption::UserDecryptionRequest::SIGNATURE_HASH;

pub async fn event_listener_gateway(
    mut subscription: alloy::pubsub::SubscriptionStream<Log>,
    orchestrator: Arc<
        Orchestrator<
            impl EventDispatcher<GatewayProcessorsEvent> + HandlerRegistry<GatewayProcessorsEvent>,
            GatewayProcessorsEvent,
        >,
    >,
) {
    loop {
        tokio::select! {
            event = subscription.next() => match event {
                Some(event_log) => {
                    info!("Kms connector rollup listener catches one event with topic {:?}", event_log.topic0());

                    let id = orchestrator.new_request_id();

                    // Determine event type based on topic
                    let event_data = if let Some(topic0) = event_log.topic0() {
                        // Convert B256 to FixedBytes<32>
                        let topic_bytes = FixedBytes::<32>::from_slice(topic0.as_slice());

                        match topic_bytes {
                            PROOF_VERIFICATION_REQUEST_TOPIC => {
                                info!("Received Proof Verification request event");
                                GatewayProcessorsEventData::KmsInput(
                                GatewayProcessorsInputEventData::EventLogRequestFromGw  {
                                    log: event_log
                                }
                            )
                        },
                            DECRYPTION_REQUEST_TOPIC => {
                            info!("Received Public Decryption Request event");
                            GatewayProcessorsEventData::PublicDecrypt(
                                PublicDecryptionEventData::EventLogRequestFromGw  {
                                    log: event_log
                                }
                            )
                        },

                        USER_DECRYPTION_REQUEST_TOPIC => {
                            info!("Received User Decryption Request event");
                            GatewayProcessorsEventData::UserDecrypt(
                                UserDecryptionEventData::EventLogRequestFromGw  {
                                    log: event_log
                                }
                            )
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

                    let event = GatewayProcessorsEvent::new(
                        id,
                        event::ApiVersion {
                            category: event::ApiCategory::PRODUCTION,
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
