pub mod db;
pub mod fhe;
mod grpc;
mod gw_event;
mod kms_response;

use anyhow::anyhow;
pub use grpc::{KmsGrpcRequest, KmsGrpcResponse};
pub use gw_event::GatewayEvent;
use kms_grpc::kms::v1::RequestId;
pub use kms_response::{KmsResponse, PublicDecryptionResponse, UserDecryptionResponse};

use alloy::{
    hex::{self, FromHexError},
    primitives::U256,
};

pub fn u256_to_u32(integer: U256) -> anyhow::Result<u32> {
    // Get integer's least significant bits
    let Some(integer_lsb) = integer.as_le_slice().get(0..4) else {
        return Err(anyhow!(
            "Failed to get least significant bits of U256. Should be unreachable"
        ));
    };

    Ok(u32::from_le_bytes(integer_lsb.try_into()?))
}

pub fn decode_request_id(request_id: RequestId) -> Result<U256, FromHexError> {
    hex::decode_to_array::<_, 32>(request_id.request_id).map(U256::from_be_bytes)
}
