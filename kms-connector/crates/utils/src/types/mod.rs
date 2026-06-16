pub mod db;
pub mod event;
pub mod extra_data;
mod grpc;
pub mod handle;
pub mod kms_response;

pub use event::{ProtocolEvent, ProtocolEventKind};
pub use grpc::{KmsGrpcRequest, KmsGrpcResponse};
pub use kms_response::{
    CrsgenResponse, EpochResultResponse, KeygenResponse, KmsResponse, KmsResponseKind,
    NewKmsContextResponse, PrepKeygenResponse, PublicDecryptionResponse, UserDecryptionResponse,
};

use alloy::{
    hex::{self, FromHexError},
    primitives::U256,
};
use anyhow::anyhow;
use kms_grpc::kms::v1::RequestId;

/// Mirrors `KMS_CONTEXT_COUNTER_BASE` from `host-contracts/contracts/shared/Constants.sol`.
pub const KMS_CONTEXT_COUNTER_BASE: U256 = U256::from_be_bytes([
    7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

/// Mirrors `DEFAULT_EPOCH_ID` from `host-contracts/contracts/shared/Constants.sol`: the KMS
/// fallback epoch ID when no epoch is specified. Format: [0x08 type tag | 31 counter bytes].
pub const DEFAULT_EPOCH_ID: U256 = U256::from_be_bytes([
    8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
]);

/// KMS context ID seeded into the database for integration tests.
#[cfg(feature = "tests")]
pub const TESTING_KMS_CONTEXT: U256 = U256::ONE;

pub fn u256_to_u32(integer: U256) -> anyhow::Result<u32> {
    // Get integer's least significant bits
    let Some(integer_lsb) = integer.as_le_slice().get(0..4) else {
        return Err(anyhow!(
            "Failed to get least significant bits of U256. Should be unreachable"
        ));
    };

    Ok(u32::from_le_bytes(integer_lsb.try_into()?))
}

/// Converts a U256 request ID to a valid hex format that KMS Core expects.
///
/// The KMS Core expects a hex string that decodes to exactly 32 bytes (big-endian).
pub fn u256_to_request_id(request_id: U256) -> RequestId {
    let bytes = request_id.to_be_bytes::<32>();
    RequestId {
        request_id: hex::encode(bytes),
    }
}

/// Converts a RequestId to a U256 (big-endian).
pub fn request_id_to_u256(request_id: RequestId) -> Result<U256, FromHexError> {
    hex::decode_to_array::<_, 32>(request_id.request_id).map(U256::from_be_bytes)
}
