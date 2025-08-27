use alloy::{hex, primitives::U256};
use anyhow::anyhow;
use kms_grpc::kms::v1::{
    KeyGenPreprocRequest, KeyGenPreprocResult, PublicDecryptionRequest, PublicDecryptionResponse,
    RequestId, UserDecryptionRequest, UserDecryptionResponse,
};
use tonic::Response;

/// The different GRPC requests used by the KMS Connector for communication the with KMS Core.
#[derive(Clone, Debug)]
pub enum KmsGrpcRequest {
    PublicDecryption(PublicDecryptionRequest),
    UserDecryption(UserDecryptionRequest),
    PrepKeygen(KeyGenPreprocRequest),
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
#[derive(Clone, Debug)]
pub enum KmsGrpcResponse {
    PublicDecryption {
        decryption_id: U256,
        grpc_response: PublicDecryptionResponse,
    },
    UserDecryption {
        decryption_id: U256,
        grpc_response: UserDecryptionResponse,
    },
    PrepKeygen {
        prep_keygen_id: U256,
        grpc_response: KeyGenPreprocResult,
    },
}

impl TryFrom<(RequestId, Response<PublicDecryptionResponse>)> for KmsGrpcResponse {
    type Error = anyhow::Error;

    fn try_from(
        value: (RequestId, Response<PublicDecryptionResponse>),
    ) -> Result<Self, Self::Error> {
        let decryption_id = U256::try_from_be_slice(&hex::decode(value.0.request_id)?)
            .ok_or_else(|| anyhow!("Failed to parse decryption_id"))?;

        Ok(Self::PublicDecryption {
            decryption_id,
            grpc_response: value.1.into_inner(),
        })
    }
}

impl TryFrom<(RequestId, Response<UserDecryptionResponse>)> for KmsGrpcResponse {
    type Error = anyhow::Error;

    fn try_from(value: (RequestId, Response<UserDecryptionResponse>)) -> Result<Self, Self::Error> {
        let decryption_id = U256::try_from_be_slice(&hex::decode(value.0.request_id)?)
            .ok_or_else(|| anyhow!("Failed to parse decryption_id"))?;

        Ok(Self::UserDecryption {
            decryption_id,
            grpc_response: value.1.into_inner(),
        })
    }
}

impl TryFrom<(RequestId, Response<KeyGenPreprocResult>)> for KmsGrpcResponse {
    type Error = anyhow::Error;

    fn try_from(value: (RequestId, Response<KeyGenPreprocResult>)) -> Result<Self, Self::Error> {
        let prep_keygen_id = U256::try_from_be_slice(&hex::decode(value.0.request_id)?)
            .ok_or_else(|| anyhow!("Failed to parse prep_keygen_id"))?;

        Ok(Self::PrepKeygen {
            prep_keygen_id,
            grpc_response: value.1.into_inner(),
        })
    }
}
