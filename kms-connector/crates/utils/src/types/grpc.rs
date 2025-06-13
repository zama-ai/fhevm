use alloy::primitives::U256;
use kms_grpc::kms::v1::{
    PublicDecryptionRequest, PublicDecryptionResponse, UserDecryptionRequest,
    UserDecryptionResponse,
};
use tonic::Response;

/// The different GRPC requests used by the KMS Connector for communication the with KMS Core.
#[derive(Clone, Debug)]
pub enum KmsGrpcRequest {
    PublicDecryption(PublicDecryptionRequest),
    UserDecryption(UserDecryptionRequest),
}

impl From<PublicDecryptionRequest> for KmsGrpcRequest {
    fn from(value: PublicDecryptionRequest) -> Self {
        Self::PublicDecryption(value)
    }
}

impl From<UserDecryptionRequest> for KmsGrpcRequest {
    fn from(value: UserDecryptionRequest) -> Self {
        Self::UserDecryption(value)
    }
}

/// The different KMS Core GRPC responses used by the KMS Connector.
pub enum KmsGrpcResponse {
    PublicDecryption {
        decryption_id: U256,
        grpc_response: PublicDecryptionResponse,
    },
    UserDecryption {
        decryption_id: U256,
        grpc_response: UserDecryptionResponse,
    },
}

impl From<(U256, Response<PublicDecryptionResponse>)> for KmsGrpcResponse {
    fn from(value: (U256, Response<PublicDecryptionResponse>)) -> Self {
        Self::PublicDecryption {
            decryption_id: value.0,
            grpc_response: value.1.into_inner(),
        }
    }
}

impl From<(U256, Response<UserDecryptionResponse>)> for KmsGrpcResponse {
    fn from(value: (U256, Response<UserDecryptionResponse>)) -> Self {
        Self::UserDecryption {
            decryption_id: value.0,
            grpc_response: value.1.into_inner(),
        }
    }
}
