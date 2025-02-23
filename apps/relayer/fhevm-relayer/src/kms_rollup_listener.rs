use alloy_sol_types::SolEvent;
use tracing::{error, info};

use crate::ethereum::bindings::ZKPoKManager;
use crate::kms_connector_relayer_event::{
    self, KmsInputEventData, KmsRelayerEvent, KmsRelayerEventData,
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
    ZKPoKManager::VerifyProofRequest::SIGNATURE_HASH;

pub async fn event_listener_rollup(
    mut subscription: alloy::pubsub::SubscriptionStream<Log>,
    orchestrator: Arc<
        Orchestrator<
            impl EventDispatcher<KmsRelayerEvent> + HandlerRegistry<KmsRelayerEvent>,
            KmsRelayerEvent,
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
                                KmsRelayerEventData::KmsInput(
                                KmsInputEventData::EventLogRequestFromGwL2  {
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

                    let event = KmsRelayerEvent::new(
                        id,
                        kms_connector_relayer_event::ApiVersion {
                            category: kms_connector_relayer_event::ApiCategory::PRODUCTION,
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
