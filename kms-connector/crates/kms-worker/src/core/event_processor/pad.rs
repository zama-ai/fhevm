//! Pad decryption processing. Not implemented yet.

use crate::core::event_processor::ProcessingError;
use anyhow::anyhow;
use kms_connector_api::ErrorCode;

/// The rejection of every pad decryption, until they can be processed.
pub fn unsupported() -> ProcessingError {
    ProcessingError::irrecoverable(
        ErrorCode::Unprocessable,
        anyhow!("pad decryption is not supported yet"),
    )
}
