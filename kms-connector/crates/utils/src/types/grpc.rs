use alloy::{hex, primitives::U256};
use anyhow::anyhow;
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

/// Struct representing a KMS Core GRPC responses used by the KMS Connector.
#[derive(Clone, Debug)]
pub struct KmsGrpcResponse {
    /// The ID of the request associated to the response.
    pub id: U256,

    /// The kind of GRPC response.
    pub kind: KmsGrpcResponseKind,
}

/// The different KMS Core GRPC responses used by the KMS Connector.
#[derive(Clone, Debug)]
pub enum KmsGrpcResponseKind {
    PublicDecryption(PublicDecryptionResponse),
    UserDecryption(UserDecryptionResponse),
}

impl KmsGrpcResponse {
    /// Parses a `KmsGrpcResponse`.
    pub fn parse<T: Into<KmsGrpcResponseKind>>(id: &str, kind: T) -> anyhow::Result<Self> {
        Ok(Self {
            id: U256::try_from_be_slice(&hex::decode(id)?)
                .ok_or_else(|| anyhow!("Failed to parse decryption_id"))?,
            kind: kind.into(),
        })
    }
}

impl From<Response<PublicDecryptionResponse>> for KmsGrpcResponseKind {
    fn from(value: Response<PublicDecryptionResponse>) -> Self {
        Self::PublicDecryption(value.into_inner())
    }
}

impl From<Response<UserDecryptionResponse>> for KmsGrpcResponseKind {
    fn from(value: Response<UserDecryptionResponse>) -> Self {
        Self::UserDecryption(value.into_inner())
    }
}
