use crate::{
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        TokioEventDispatcher,
    },
    relayer_event::{DecryptedValue, RelayerEvent, RelayerEventData},
};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

struct DecryptionResultData {
    gateway_l2_request_id: String,
}

pub struct ArbitrumGatewayL2Handler {
    dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
    context_data: dashmap::DashMap<Uuid, DecryptionResultData>,
}

impl ArbitrumGatewayL2Handler {
    pub fn new(dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>) -> Self {
        Self {
            dispatcher,
            context_data: dashmap::DashMap::new(),
        }
    }

    async fn mock_handle_decrypt_request_received(&self, event: RelayerEvent) {
        let next_event_data = RelayerEventData::DecryptionResponseRcvdFromGwL2 {
            decrypted_value: DecryptedValue::PublicDecrypt {
                plaintext: vec![1, 2, 3],
                signatures: vec![vec![1, 2, 3]],
            },
        };
        info!(
            "Decryption request received. Responding with mock data: request_id: {:?}",
            event.request_id,
        );
        let _ = self
            .dispatcher
            .dispatch_event(event.derive_next_event(next_event_data))
            .await;
    }
    async fn noop_handle_decrypt_reponse_event_log(&self, _event: RelayerEvent) {}
}

#[async_trait]
impl EventHandler<RelayerEvent> for ArbitrumGatewayL2Handler {
    async fn handle_event(&self, event: RelayerEvent) {
        match event.clone().data {
            RelayerEventData::DecryptRequestRcvd {
                ct_handles,
                operation,
            } => {
                self.mock_handle_decrypt_request_received(event).await;
            }
            RelayerEventData::DecryptResponseEventLogRcvdFromGwL2 { log } => {}
            _ => {
                self.noop_handle_decrypt_reponse_event_log(event).await;
            }
        }
    }
}
