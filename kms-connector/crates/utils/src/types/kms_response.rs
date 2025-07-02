use std::fmt::Display;

use crate::types::{
    GatewayEvent, KmsGrpcResponse,
    fhe::{abi_encode_plaintexts, fhe_type_to_string},
};
use alloy::primitives::U256;
use anyhow::anyhow;
use kms_grpc::kms::v1::{PublicDecryptionResponse, UserDecryptionResponse};
use sqlx::{Pool, Postgres, Row, postgres::PgRow};
use tracing::{debug, info};

#[derive(Clone, Debug, PartialEq)]
pub enum KmsResponse {
    PublicDecryption {
        decryption_id: U256,
        decrypted_result: Vec<u8>,
        signature: Vec<u8>,
    },
    UserDecryption {
        decryption_id: U256,
        user_decrypted_shares: Vec<u8>,
        signature: Vec<u8>,
    },
}

impl KmsResponse {
    /// Processes a KMS GRPC response into a `KmsResponse` enum.
    pub fn process(response: KmsGrpcResponse) -> anyhow::Result<Self> {
        match response {
            KmsGrpcResponse::PublicDecryption {
                decryption_id,
                grpc_response,
            } => Self::process_public_decryption(decryption_id, grpc_response),
            KmsGrpcResponse::UserDecryption {
                decryption_id,
                grpc_response,
            } => Self::process_user_decryption(decryption_id, grpc_response),
        }
    }

    fn process_public_decryption(
        decryption_id: U256,
        grpc_response: PublicDecryptionResponse,
    ) -> anyhow::Result<Self> {
        let payload = grpc_response.payload.ok_or_else(|| {
            anyhow!("Received empty payload for public decryption {decryption_id}")
        })?;

        for pt in &payload.plaintexts {
            debug!(
                "Public decryption result type: {} for request {}",
                fhe_type_to_string(pt.fhe_type),
                decryption_id
            );
        }

        // Encode all plaintexts using ABI encoding
        let result = abi_encode_plaintexts(&payload.plaintexts);

        // Get the external signature
        let signature = payload
            .external_signature
            .ok_or_else(|| anyhow!("KMS Core did not provide required EIP-712 signature"))?;

        info!(
            "Storing public decryption response for request {} with {} plaintexts",
            decryption_id,
            payload.plaintexts.len()
        );
        Ok(KmsResponse::PublicDecryption {
            decryption_id,
            decrypted_result: result.into(),
            signature,
        })
    }

    fn process_user_decryption(
        decryption_id: U256,
        grpc_response: UserDecryptionResponse,
    ) -> anyhow::Result<KmsResponse> {
        let payload = grpc_response
            .payload
            .ok_or_else(|| anyhow!("Received empty payload for user decryption {decryption_id}"))?;

        // Serialize all signcrypted ciphertexts
        let serialized_response_payload = bincode::serialize(&payload)
            .map_err(|e| anyhow!("Failed to serialize UserDecryption payload: {e}"))?;

        for ct in &payload.signcrypted_ciphertexts {
            debug!(
                "User decryption result type: {} for request {}",
                fhe_type_to_string(ct.fhe_type),
                decryption_id
            );
        }

        info!(
            "Storing user decryption response for request {} with {} ciphertexts",
            decryption_id,
            payload.signcrypted_ciphertexts.len()
        );
        Ok(KmsResponse::UserDecryption {
            decryption_id,
            user_decrypted_shares: serialized_response_payload,
            signature: grpc_response.external_signature,
        })
    }

    /// Create a new `KmsResponse::PublicDecryption` from a `PgRow`.
    pub fn from_public_decryption_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(KmsResponse::PublicDecryption {
            decryption_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?),
            decrypted_result: row.try_get("decrypted_result")?,
            signature: row.try_get("signature")?,
        })
    }

    /// Create a new `KmsResponse::UserDecryption` from a `PgRow`.
    pub fn from_user_decryption_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(KmsResponse::UserDecryption {
            decryption_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?),
            user_decrypted_shares: row.try_get("user_decrypted_shares")?,
            signature: row.try_get("signature")?,
        })
    }

    /// Sets the `under_process` field of the event associated to this response as `FALSE` in the
    /// database.
    pub async fn free_associated_event(&self, db: &Pool<Postgres>) {
        match self {
            KmsResponse::PublicDecryption { decryption_id, .. } => {
                GatewayEvent::mark_public_decryption_as_free(db, *decryption_id).await
            }
            KmsResponse::UserDecryption { decryption_id, .. } => {
                GatewayEvent::mark_user_decryption_as_free(db, *decryption_id).await
            }
        }
    }
}

impl Display for KmsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KmsResponse::PublicDecryption { decryption_id, .. } => {
                write!(f, "PublicDecryptionResponse #{decryption_id}")
            }
            KmsResponse::UserDecryption { decryption_id, .. } => {
                write!(f, "UserDecryptionResponse #{decryption_id}")
            }
        }
    }
}
