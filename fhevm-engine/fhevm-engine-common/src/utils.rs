use serde::{de::DeserializeOwned, Serialize};
use tfhe::{named::Named, prelude::ParameterSetConformant, Unversionize, Versionize};

use crate::types::FhevmError;

pub const SAFE_SER_DESER_LIMIT: u64 = 1024 * 1024 * 16;
pub const SAFE_SER_DESER_KEY_LIMIT: u64 = 1024 * 1024 * 512;

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
