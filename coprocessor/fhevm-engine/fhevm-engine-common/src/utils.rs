use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use serde::{de::DeserializeOwned, Serialize};
use tfhe::{named::Named, prelude::ParameterSetConformant, Unversionize, Versionize};

use crate::types::FhevmError;

pub const SAFE_SER_DESER_LIMIT: u64 = 1024 * 1024 * 16;
pub const SAFE_SER_DESER_KEY_LIMIT: u64 = 1024 * 1024 * 1024 * 2;
pub const SAFE_SER_DESER_SNS_KEY_LIMIT: u64 = 1024 * 1024 * 1024 * 2;

pub fn safe_serialize<T: Serialize + Named + Versionize>(object: &T) -> Vec<u8> {
    let mut out = vec![];
    tfhe::safe_serialization::safe_serialize(object, &mut out, SAFE_SER_DESER_LIMIT)
        .expect("safe serialize succeeds");
    out
}

pub fn safe_deserialize<T: DeserializeOwned + Named + Unversionize>(
    input: &[u8],
) -> Result<T, FhevmError> {
    tfhe::safe_serialization::safe_deserialize(input, SAFE_SER_DESER_LIMIT)
        .map_err(|e| FhevmError::DeserializationError(e.into()))
}

pub fn safe_deserialize_conformant<
    T: DeserializeOwned + Named + Unversionize + ParameterSetConformant,
>(
    input: &[u8],
    parameter_set: &T::ParameterSet,
) -> Result<T, FhevmError> {
    tfhe::safe_serialization::safe_deserialize_conformant(
        input,
        SAFE_SER_DESER_LIMIT,
        parameter_set,
    )
    .map_err(|e| FhevmError::DeserializationError(e.into()))
}

pub fn safe_serialize_key<T: Serialize + Named + Versionize>(object: &T) -> Vec<u8> {
    let mut out = vec![];
    tfhe::safe_serialization::safe_serialize(object, &mut out, SAFE_SER_DESER_KEY_LIMIT)
        .expect("safe serialize succeeds");
    out
}

pub fn safe_deserialize_key<T: DeserializeOwned + Named + Unversionize>(
    input: &[u8],
) -> Result<T, FhevmError> {
    tfhe::safe_serialization::safe_deserialize(input, SAFE_SER_DESER_KEY_LIMIT)
        .map_err(|e| FhevmError::DeserializationError(e.into()))
}

pub fn safe_deserialize_sns_key<T: DeserializeOwned + Named + Unversionize>(
    input: &[u8],
) -> Result<T, FhevmError> {
    tfhe::safe_serialization::safe_deserialize(input, SAFE_SER_DESER_SNS_KEY_LIMIT)
        .map_err(|e| FhevmError::DeserializationError(e.into()))
}

// Print first 4 and last 4 bytes of a blob as hex
pub fn compact_hex(blob: &[u8]) -> String {
    const OFFSET: usize = 8;
    match blob.len() {
        0 => String::from("0x"),
        len if len <= 2 * OFFSET => format!("0x{}", hex::encode(blob)),
        _ => {
            let hex_str = hex::encode(blob);
            format!(
                "0x{}...{}",
                &hex_str[..OFFSET],
                &hex_str[hex_str.len() - OFFSET..]
            )
        }
    }
}

#[derive(Clone, Debug)]
pub struct HeartBeat {
    timestamp_origin: std::time::Instant,
    timestamp: Arc<AtomicU64>,
}
impl HeartBeat {
    pub fn new() -> Self {
        Self {
            timestamp_origin: std::time::Instant::now(),
            timestamp: Arc::new(AtomicU64::new(0)),
        }
    }

    fn now_timestamp(&self) -> u64 {
        self.timestamp_origin.elapsed().as_secs()
    }

    pub fn update(&self) {
        let now = self.now_timestamp();
        self.timestamp.store(now, Ordering::Relaxed);
    }

    pub fn is_recent(&self, freshness: &Duration) -> bool {
        let elapsed = self.now_timestamp() - self.timestamp.load(Ordering::Relaxed);
        elapsed <= freshness.as_secs()
    }
}

impl Default for HeartBeat {
    fn default() -> Self {
        Self::new()
    }
}
