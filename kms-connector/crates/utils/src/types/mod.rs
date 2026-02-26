pub mod db;
pub mod extra_data;
mod grpc;
pub mod gw_event;
pub mod handle;
pub mod kms_response;

pub use grpc::{KmsGrpcRequest, KmsGrpcResponse};
pub use gw_event::{GatewayEvent, GatewayEventKind};
pub use kms_response::{
    CrsgenResponse, KeygenResponse, KmsResponse, KmsResponseKind, PrepKeygenResponse,
    PublicDecryptionResponse, UserDecryptionResponse,
};

use alloy::{
    hex::{self, FromHexError},
    primitives::U256,
};
use anyhow::anyhow;
use kms_grpc::kms::v1::RequestId;

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
