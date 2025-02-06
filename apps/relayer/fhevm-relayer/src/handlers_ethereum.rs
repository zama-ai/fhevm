use async_trait::async_trait;
use tracing::info;

use crate::{
    ethereum::{
        bindings::{DecryptionOracle, GatewayContract, TFHEExecutor, Transfer},
        extract_event_signature,
    },
    orchestrator::{traits::EventHandler, TokioEventDispatcher},
    relayer_event::{self, RelayerEvent},
};
use alloy::rpc::types::Log;
use std::sync::Arc;

use alloy_sol_types::SolEvent;

pub struct EthereumHostL1EventLogHandler {
    dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
}

impl EthereumHostL1EventLogHandler {
    pub fn new(dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>) -> Self {
        Self { dispatcher }
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for EthereumHostL1EventLogHandler {
    async fn handle_event(&self, event: relayer_event::RelayerEvent) {
        let eth_event_log: Log;
        match event.data {
            relayer_event::RelayerEventData::HostL1EventLogReceived { log: l } => eth_event_log = l,
            _ => {
                return;
            }
        }

        match extract_event_signature(&eth_event_log).unwrap() {
            &GatewayContract::EventDecryption::SIGNATURE_HASH => {
                info!(
                    "{:?} {:?}",
                    GatewayContract::EventDecryption::SIGNATURE,
                    eth_event_log.block_number
                )
            }
            &DecryptionOracle::DecryptionRequest::SIGNATURE_HASH => {
                info!(
                    "{:?} {:?}",
                    DecryptionOracle::DecryptionRequest::SIGNATURE,
                    eth_event_log.block_number
                )
            }
            &TFHEExecutor::FheAdd::SIGNATURE_HASH => {
                info!(
                    "{:?} {:?}",
                    TFHEExecutor::FheAdd::SIGNATURE,
                    eth_event_log.block_number
                )
            }
            &TFHEExecutor::FheSub::SIGNATURE_HASH => {
                info!(
                    "{:?} {:?}",
                    TFHEExecutor::FheSub::SIGNATURE,
                    eth_event_log.block_number
                )
            }
            &Transfer::SIGNATURE_HASH => {
                info!("{:?} {:?}", Transfer::SIGNATURE, eth_event_log.block_number)
            }
            _ => {
                // Ignore the event
            }
        }
    }
}
