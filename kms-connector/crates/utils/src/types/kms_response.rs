use crate::types::{
    GatewayEvent, KmsGrpcResponse, db::KeyDigestDbItem, fhe::abi_encode_plaintexts,
};
use alloy::{hex, primitives::U256};
use anyhow::anyhow;
use kms_grpc::kms::v1::{
    KeyGenPreprocResult, KeyGenResult, PublicDecryptionResponse as GrpcPublicDecryptionResponse,
    UserDecryptionResponse as GrpcUserDecryptionResponse,
};
use sqlx::{Pool, Postgres, Row, postgres::PgRow};
use std::fmt::Display;
use tracing::debug;

#[derive(Clone, Debug, PartialEq)]
pub enum KmsResponse {
    PublicDecryption(PublicDecryptionResponse),
    UserDecryption(UserDecryptionResponse),
    PrepKeygen(PrepKeygenResponse),
    Keygen(KeygenResponse),
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

#[derive(Clone, Debug, PartialEq)]
pub struct PrepKeygenResponse {
    pub prep_keygen_id: U256,
    pub signature: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeygenResponse {
    pub key_id: U256,
    pub key_digests: Vec<KeyDigestDbItem>,
    pub signature: Vec<u8>,
}

impl KmsResponse {
    pub fn id(&self) -> U256 {
        match self {
            KmsResponse::PublicDecryption(r) => r.decryption_id,
            KmsResponse::UserDecryption(r) => r.decryption_id,
            KmsResponse::PrepKeygen(r) => r.prep_keygen_id,
            KmsResponse::Keygen(r) => r.key_id,
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
            KmsGrpcResponse::PrepKeygen(grpc_response) => {
                PrepKeygenResponse::process(grpc_response).map(Self::PrepKeygen)
            }
            KmsGrpcResponse::Keygen(grpc_response) => {
                KeygenResponse::process(grpc_response).map(Self::Keygen)
            }
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

    pub fn from_prep_keygen_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(KmsResponse::PrepKeygen(PrepKeygenResponse {
            prep_keygen_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("prep_keygen_id")?),
            signature: row.try_get("signature")?,
        }))
    }

    pub fn from_keygen_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(KmsResponse::Keygen(KeygenResponse {
            key_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("key_id")?),
            key_digests: row.try_get("key_digests")?,
            signature: row.try_get("signature")?,
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
            KmsResponse::PrepKeygen(r) => {
                GatewayEvent::mark_prep_keygen_as_pending(db, r.prep_keygen_id).await
            }
            KmsResponse::Keygen(r) => GatewayEvent::mark_keygen_as_pending(db, r.key_id).await,
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

        Ok(PublicDecryptionResponse {
            decryption_id,
            decrypted_result: result.into(),
            signature: grpc_response.external_signature,
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

        Ok(UserDecryptionResponse {
            decryption_id,
            user_decrypted_shares: serialized_response_payload,
            signature: grpc_response.external_signature,
            extra_data: grpc_response.extra_data,
        })
    }
}

impl PrepKeygenResponse {
    fn process(grpc_response: KeyGenPreprocResult) -> anyhow::Result<Self> {
        let prep_keygen_id = U256::try_from_be_slice(&hex::decode(
            &grpc_response
                .preprocessing_id
                .as_ref()
                .ok_or_else(|| anyhow!("No preprocessing id in `KeyGenPreprocResult`"))?
                .request_id,
        )?)
        .ok_or_else(|| {
            anyhow!(
                "Failed to parse preprocessing_id: {:?}",
                grpc_response.preprocessing_id
            )
        })?;

        Ok(PrepKeygenResponse {
            prep_keygen_id,
            signature: grpc_response.external_signature,
        })
    }
}

impl KeygenResponse {
    fn process(grpc_response: KeyGenResult) -> anyhow::Result<Self> {
        let key_id = U256::try_from_be_slice(&hex::decode(
            &grpc_response
                .request_id
                .as_ref()
                .ok_or_else(|| anyhow!("No preprocessing id in `KeyGenPreprocResult`"))?
                .request_id,
        )?)
        .ok_or_else(|| anyhow!("Failed to parse request_id: {:?}", grpc_response.request_id))?;

        let key_digests = grpc_response
            .key_digests
            .into_iter()
            .map(|(k, d)| {
                Ok(KeyDigestDbItem {
                    key_type: k.parse()?,
                    digest: d,
                })
            })
            .collect::<anyhow::Result<Vec<KeyDigestDbItem>>>()?;

        Ok(KeygenResponse {
            key_id,
            key_digests,
            signature: grpc_response.external_signature,
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
            KmsResponse::PrepKeygen(r) => {
                write!(f, "PrepKeygenResponse #{}", r.prep_keygen_id)
            }
            KmsResponse::Keygen(r) => {
                write!(f, "KeygenResponse #{}", r.key_id)
            }
        }
    }
}
