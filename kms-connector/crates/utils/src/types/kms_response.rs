use crate::types::{GatewayEvent, KmsGrpcResponse, fhe::abi_encode_plaintexts};
use alloy::primitives::U256;
use anyhow::anyhow;
use kms_grpc::kms::v1::{
    PublicDecryptionResponse as GrpcPublicDecryptionResponse,
    UserDecryptionResponse as GrpcUserDecryptionResponse,
};
use sqlx::{Pool, Postgres, Row, postgres::PgRow};
use std::fmt::Display;
use tracing::{debug, info};

#[derive(Clone, Debug, PartialEq)]
pub enum KmsResponse {
    PublicDecryption(PublicDecryptionResponse),
    UserDecryption(UserDecryptionResponse),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PublicDecryptionResponse {
    pub decryption_id: U256,
    pub decrypted_result: Vec<u8>,
    pub signature: Vec<u8>,
    pub extra_data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UserDecryptionResponse {
    pub decryption_id: U256,
    pub user_decrypted_shares: Vec<u8>,
    pub signature: Vec<u8>,
    pub extra_data: Vec<u8>,
}

impl KmsResponse {
    pub fn id(&self) -> U256 {
        match self {
            KmsResponse::PublicDecryption(r) => r.decryption_id,
            KmsResponse::UserDecryption(r) => r.decryption_id,
        }
    }

    /// Processes a KMS GRPC response into a `KmsResponse` enum.
    pub fn process(response: KmsGrpcResponse) -> anyhow::Result<Self> {
        match response {
            KmsGrpcResponse::PublicDecryption {
                decryption_id,
                grpc_response,
            } => PublicDecryptionResponse::process(decryption_id, grpc_response)
                .map(Self::PublicDecryption),
            KmsGrpcResponse::UserDecryption {
                decryption_id,
                grpc_response,
            } => UserDecryptionResponse::process(decryption_id, grpc_response)
                .map(Self::UserDecryption),
        }
    }

    pub fn from_public_decryption_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(KmsResponse::PublicDecryption(PublicDecryptionResponse {
            decryption_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?),
            decrypted_result: row.try_get("decrypted_result")?,
            signature: row.try_get("signature")?,
            extra_data: row.try_get("extra_data")?,
        }))
    }

    pub fn from_user_decryption_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(KmsResponse::UserDecryption(UserDecryptionResponse {
            decryption_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?),
            user_decrypted_shares: row.try_get("user_decrypted_shares")?,
            signature: row.try_get("signature")?,
            extra_data: row.try_get("extra_data")?,
        }))
    }

    /// Sets the `under_process` field of the event associated to this response as `FALSE` in the
    /// database.
    pub async fn mark_associated_event_as_pending(&self, db: &Pool<Postgres>) {
        match self {
            KmsResponse::PublicDecryption(r) => {
                GatewayEvent::mark_public_decryption_as_pending(db, r.decryption_id).await
            }
            KmsResponse::UserDecryption(r) => {
                GatewayEvent::mark_user_decryption_as_pending(db, r.decryption_id).await
            }
        }
    }
}

impl PublicDecryptionResponse {
    fn process(
        decryption_id: U256,
        grpc_response: GrpcPublicDecryptionResponse,
    ) -> anyhow::Result<Self> {
        let payload = grpc_response.payload.ok_or_else(|| {
            anyhow!("Received empty payload for public decryption {decryption_id}")
        })?;

        for pt in &payload.plaintexts {
            debug!(
                "Public decryption result type: {:?} for request {}",
                pt.fhe_type, decryption_id
            );
        }

        // Encode all plaintexts using ABI encoding
        let result = abi_encode_plaintexts(&payload.plaintexts);

        // Get the external signature
        let signature = grpc_response
            .external_signature
            .ok_or_else(|| anyhow!("KMS Core did not provide required EIP-712 signature"))?;

        info!(
            "Storing public decryption response for request {} with {} plaintexts",
            decryption_id,
            payload.plaintexts.len()
        );
        Ok(PublicDecryptionResponse {
            decryption_id,
            decrypted_result: result.into(),
            signature,
            extra_data: grpc_response.extra_data,
        })
    }
}

impl UserDecryptionResponse {
    fn process(
        decryption_id: U256,
        grpc_response: GrpcUserDecryptionResponse,
    ) -> anyhow::Result<Self> {
        let payload = grpc_response
            .payload
            .ok_or_else(|| anyhow!("Received empty payload for user decryption {decryption_id}"))?;

        // Serialize all signcrypted ciphertexts
        let serialized_response_payload = bincode::serialize(&payload)
            .map_err(|e| anyhow!("Failed to serialize UserDecryption payload: {e}"))?;

        for ct in &payload.signcrypted_ciphertexts {
            debug!(
                "User decryption result type: {:?} for request {}",
                ct.fhe_type, decryption_id
            );
        }

        info!(
            "Storing user decryption response for request {} with {} ciphertexts",
            decryption_id,
            payload.signcrypted_ciphertexts.len()
        );
        Ok(UserDecryptionResponse {
            decryption_id,
            user_decrypted_shares: serialized_response_payload,
            signature: grpc_response.external_signature,
            extra_data: grpc_response.extra_data,
        })
    }
}

impl Display for KmsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KmsResponse::PublicDecryption(r) => {
                write!(f, "PublicDecryptionResponse #{}", r.decryption_id)
            }
            KmsResponse::UserDecryption(r) => {
                write!(f, "UserDecryptionResponse #{}", r.decryption_id)
            }
        }
    }
}
