use crate::errors::EventProcessingError;
use alloy::primitives::FixedBytes;
use alloy::rpc::types::Log;

pub fn extract_event_signature(log: &Log) -> Result<&FixedBytes<32>, EventProcessingError> {
    log.inner
        .data
        .topics()
        .first()
        .ok_or(EventProcessingError::MissingTopic)
}
