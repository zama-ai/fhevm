use std::{borrow::Cow, time::Duration};

use alloy_primitives::{Address, B256, U256};
use block_manifest::ManifestVersion;

use crate::manifest_consensus::ExecutionError;

pub(super) fn downloader_worker_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    format!("{}-{nanos}", std::process::id())
}

/// Error text often echoes peer bytes, e.g. serde's unknown-variant message,
/// and PostgreSQL TEXT rejects NUL (SQLSTATE 22021). Replace it before storing.
pub(super) fn storable_text(text: &str) -> Cow<'_, str> {
    if text.contains('\0') {
        Cow::Owned(text.replace('\0', "\u{FFFD}"))
    } else {
        Cow::Borrowed(text)
    }
}

pub(super) fn duration_micros(field: &str, duration: Duration) -> Result<i64, ExecutionError> {
    i64::try_from(duration.as_micros())
        .map_err(|_| internal(format!("{field} exceeds PostgreSQL interval precision")))
}

pub(super) fn manifest_version(value: i16) -> Result<ManifestVersion, ExecutionError> {
    let value = u8::try_from(value).map_err(|_| internal("manifest version is outside uint8"))?;
    ManifestVersion::try_from(value)
        .map_err(|err| internal(format!("stored manifest version is invalid: {err}")))
}

pub(super) fn u256(field: &str, value: &[u8]) -> Result<U256, ExecutionError> {
    let bytes: [u8; 32] = value
        .try_into()
        .map_err(|_| internal(format!("{field} must be 32 bytes, got {}", value.len())))?;
    Ok(U256::from_be_bytes(bytes))
}

pub(super) fn b256(field: &str, value: &[u8]) -> Result<B256, ExecutionError> {
    let bytes: [u8; 32] = value
        .try_into()
        .map_err(|_| internal(format!("{field} must be 32 bytes, got {}", value.len())))?;
    Ok(B256::from(bytes))
}

pub(super) fn address(field: &str, value: &[u8]) -> Result<Address, ExecutionError> {
    let bytes: [u8; 20] = value
        .try_into()
        .map_err(|_| internal(format!("{field} must be 20 bytes, got {}", value.len())))?;
    Ok(Address::from(bytes))
}

pub(super) fn internal(message: impl Into<String>) -> ExecutionError {
    ExecutionError::InternalError(message.into())
}
