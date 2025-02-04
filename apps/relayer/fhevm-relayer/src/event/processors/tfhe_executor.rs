use crate::{
    errors::EventProcessingError, ethereum::provider::TFHEExecutor, event::types::ContractEvent,
};
use alloy::primitives::B256;
use alloy::rpc::types::Log as RpcLog;
use alloy_sol_types::SolEvent;
use std::sync::Arc;
use tracing::{debug, info, instrument};

/// Register FHE operations.
///
/// This processor handles events related to FHE operations such as addition
/// and subtraction of encrypted values.
///
/// # Events Handled
///
/// - `FheAdd(address,uint256,uint256,bytes1,uint256)`
/// - `FheSub(address,uint256,uint256,bytes1,uint256)`
///
#[derive(Debug, Clone)]
pub struct TfheExecutor;

#[derive(Debug, Clone)]
pub enum EventType {
    FheAdd(TFHEExecutor::FheAdd),
    FheSub(TFHEExecutor::FheSub),
}

impl Default for TfheExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl TfheExecutor {
    pub fn new() -> Self {
        TfheExecutor
    }

    #[instrument(skip_all)]
    fn handle_event(&self, event: EventType) -> Result<(), EventProcessingError> {
        match event {
            EventType::FheAdd(add) => {
                info!(?add, "Handling FheAdd operation");
                Ok(())
            }
            EventType::FheSub(sub) => {
                info!(?sub, "Handling FheSub operation");
                Ok(())
            }
        }
    }
}

impl ContractEvent for TfheExecutor {
    fn topics(&self) -> Vec<B256> {
        vec![
            TFHEExecutor::FheAdd::SIGNATURE_HASH,
            TFHEExecutor::FheSub::SIGNATURE_HASH,
        ]
    }

    fn process_event(&self, log: &RpcLog) -> Result<(), EventProcessingError> {
        debug!(?log.inner.address, "Processing TFHE event");

        let event_signature = log
            .inner
            .data
            .topics()
            .first()
            .ok_or(EventProcessingError::MissingTopic)?;

        let event = match event_signature {
            &TFHEExecutor::FheAdd::SIGNATURE_HASH => {
                TFHEExecutor::FheAdd::decode_log_data(log.data(), true)
                    .map(EventType::FheAdd)
                    .map_err(EventProcessingError::DecodingError)?
            }
            &TFHEExecutor::FheSub::SIGNATURE_HASH => {
                TFHEExecutor::FheSub::decode_log_data(log.data(), true)
                    .map(EventType::FheSub)
                    .map_err(EventProcessingError::DecodingError)?
            }
            _ => return Err(EventProcessingError::UnknownEvent(log.inner.address)),
        };

        self.handle_event(event)
    }
}

impl ContractEvent for Arc<TfheExecutor> {
    fn topics(&self) -> Vec<B256> {
        (**self).topics()
    }

    fn process_event(&self, log: &RpcLog) -> Result<(), EventProcessingError> {
        (**self).process_event(log)
    }
}
