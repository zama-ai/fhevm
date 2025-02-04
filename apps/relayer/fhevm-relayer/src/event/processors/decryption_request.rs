use crate::{
    errors::EventProcessingError,
    ethereum::provider::{DecryptionOracle, GatewayContract},
    event::types::ContractEvent,
};
use alloy::primitives::B256;
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
        vec![DecryptionOracle::DecryptionRequest::SIGNATURE_HASH]
    }

    fn process_event(&self, log: &RpcLog) -> Result<(), EventProcessingError> {
        debug!(?log.inner.address, "Processing event");

        // Extract event signature
        let event_signature = log
            .inner
            .data
            .topics()
            .first()
            .ok_or(EventProcessingError::MissingTopic)?;

        // Match to one of the options and decode it. Very unlikely to have a decoding error.
        let event = match event_signature {
            &GatewayContract::EventDecryption::SIGNATURE_HASH => {
                GatewayContract::EventDecryption::decode_log_data(log.data(), true)
                    .map(EventType::EventDecryption)
                    .map_err(EventProcessingError::DecodingError)?
            }
            &DecryptionOracle::DecryptionRequest::SIGNATURE_HASH => {
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
