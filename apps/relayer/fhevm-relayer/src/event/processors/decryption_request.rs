use crate::{
    errors::EventProcessingError,
    ethereum::provider::{
        DecryptionOracle, GatewayContract, DECRYPTION_EVENT_SIGNATURE,
        DECRYPTION_ORACLE_EVENT_SIGNATURE,
    },
    event::types::ContractEvent,
};
use alloy::primitives::{keccak256, B256};
use alloy::rpc::types::Log as RpcLog;
use alloy_sol_types::SolEvent;
use std::sync::Arc;
use tracing::{debug, info, instrument};

#[derive(Debug, Clone)]
pub enum EventType {
    EventDecryption(GatewayContract::EventDecryption),
    DecryptionRequest(DecryptionOracle::DecryptionRequest),
}

#[derive(Debug, Clone)]
pub struct DecryptionOracleExecutor;

impl Default for DecryptionOracleExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl DecryptionOracleExecutor {
    pub fn new() -> Self {
        DecryptionOracleExecutor
    }

    #[instrument(skip_all)]
    fn handle_event(&self, event: EventType) -> Result<(), EventProcessingError> {
        match event {
            EventType::DecryptionRequest(req) => {
                info!(?req, "Handling DecryptionRequest");
                Ok(())
            }
            EventType::EventDecryption(dec) => {
                info!(?dec, "Handling EventDecryption");
                Ok(())
            }
        }
    }
}

impl ContractEvent for DecryptionOracleExecutor {
    fn topics(&self) -> Vec<B256> {
        vec![keccak256(DECRYPTION_ORACLE_EVENT_SIGNATURE)]
    }

    fn process_event(&self, log: &RpcLog) -> Result<(), EventProcessingError> {
        debug!(?log.inner.address, "Processing event");

        let event_signature = log
            .inner
            .data
            .topics()
            .first()
            .ok_or(EventProcessingError::MissingTopic)?;

        let event = match event_signature {
            sig if sig == &keccak256(DECRYPTION_EVENT_SIGNATURE) => {
                GatewayContract::EventDecryption::decode_log_data(log.data(), true)
                    .map(EventType::EventDecryption)
                    .map_err(EventProcessingError::DecodingError)?
            }
            sig if sig == &keccak256(DECRYPTION_ORACLE_EVENT_SIGNATURE) => {
                DecryptionOracle::DecryptionRequest::decode_log_data(log.data(), true)
                    .map(EventType::DecryptionRequest)
                    .map_err(EventProcessingError::DecodingError)?
            }
            _ => return Err(EventProcessingError::UnknownEvent(log.inner.address)),
        };

        self.handle_event(event)
    }
}

impl ContractEvent for Arc<DecryptionOracleExecutor> {
    fn topics(&self) -> Vec<B256> {
        (**self).topics()
    }

    fn process_event(&self, log: &RpcLog) -> Result<(), EventProcessingError> {
        (**self).process_event(log)
    }
}
