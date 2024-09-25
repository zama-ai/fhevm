use serde::{de::DeserializeOwned, Serialize};
use tfhe::{named::Named, Unversionize, Versionize};

use crate::types::FhevmError;

pub const SAFE_SER_DESER_LIMIT: u64 = 1024 * 1024 * 16;
pub const SAFE_SER_DESER_SKS_LIMIT: u64 = 1024 * 1024 * 512;

pub fn safe_serialize<T: Serialize + Named>(object: &T) -> Vec<u8> {
    let mut out = vec![];
    tfhe::safe_serialize(object, &mut out, SAFE_SER_DESER_LIMIT).expect("safe serialize succeeds");
    out
}

pub fn safe_deserialize<T: DeserializeOwned + Named>(input: &[u8]) -> Result<T, FhevmError> {
    tfhe::safe_deserialization::safe_deserialize(input, SAFE_SER_DESER_LIMIT)
        .map_err(|e| FhevmError::DeserializationError(e.into()))
}

pub fn safe_serialize_versioned<T: Named + Versionize>(object: &T) -> Vec<u8> {
    let mut out = vec![];
    tfhe::safe_serialize_versioned(object, &mut out, SAFE_SER_DESER_LIMIT)
        .expect("safe serialize versioned succeeds");
    out
}

pub fn safe_deserialize_versioned<T: Named + Unversionize>(input: &[u8]) -> Result<T, FhevmError> {
    tfhe::safe_deserialization::safe_deserialize_versioned(input, SAFE_SER_DESER_LIMIT)
        .map_err(|e| FhevmError::DeserializationError(e.into()))
}

pub fn safe_serialize_versioned_sks(object: &tfhe::ServerKey) -> Vec<u8> {
    let mut out = vec![];
    tfhe::safe_serialize_versioned(object, &mut out, SAFE_SER_DESER_SKS_LIMIT)
        .expect("safe serialize versioned of SKS succeeds");
    out
}

pub fn safe_deserialize_versioned_sks(input: &[u8]) -> Result<tfhe::ServerKey, FhevmError> {
    tfhe::safe_deserialization::safe_deserialize_versioned(input, SAFE_SER_DESER_SKS_LIMIT)
        .map_err(|e| FhevmError::DeserializationError(e.into()))
}
