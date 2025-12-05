use crate::{
    monitoring::otlp::PropagationContext,
    types::{KmsGrpcResponse, db::KeyDigestDbItem},
};
use alloy::{hex, primitives::U256};
use anyhow::anyhow;
use kms_grpc::{
    kms::v1::{
        CrsGenResult, KeyGenPreprocResult, KeyGenResult,
        PublicDecryptionResponse as GrpcPublicDecryptionResponse,
        UserDecryptionResponse as GrpcUserDecryptionResponse,
    },
    rpc_types::abi_encode_plaintexts,
};
use sqlx::{Row, postgres::PgRow};
use std::fmt::Display;
use tracing::debug;

#[derive(Clone, Debug, PartialEq)]
pub struct KmsResponse {
    pub kind: KmsResponseKind,
    pub otlp_context: PropagationContext,
}

impl KmsResponse {
    pub fn new(kind: KmsResponseKind, otlp_context: PropagationContext) -> Self {
        Self { kind, otlp_context }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum KmsResponseKind {
    PublicDecryption(PublicDecryptionResponse),
    UserDecryption(UserDecryptionResponse),
    PrepKeygen(PrepKeygenResponse),
    Keygen(KeygenResponse),
    Crsgen(CrsgenResponse),
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

#[derive(Clone, Debug, PartialEq)]
pub struct CrsgenResponse {
    pub crs_id: U256,
    pub crs_digest: Vec<u8>,
    pub signature: Vec<u8>,
}

impl KmsResponseKind {
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
            KmsGrpcResponse::Crsgen(grpc_response) => {
                CrsgenResponse::process(grpc_response).map(Self::Crsgen)
            }
            KmsGrpcResponse::NoResponseExpected => {
                Err(anyhow!("No response expected from KMS. Nothing to process"))
            }
        }
    }
}

pub fn from_public_decryption_row(row: &PgRow) -> anyhow::Result<KmsResponse> {
    Ok(KmsResponse {
        otlp_context: bc2wrap::deserialize(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
        kind: KmsResponseKind::PublicDecryption(PublicDecryptionResponse {
            decryption_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?),
            decrypted_result: row.try_get("decrypted_result")?,
            signature: row.try_get("signature")?,
            extra_data: row.try_get("extra_data")?,
        }),
    })
}

pub fn from_user_decryption_row(row: &PgRow) -> anyhow::Result<KmsResponse> {
    Ok(KmsResponse {
        otlp_context: bc2wrap::deserialize(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
        kind: KmsResponseKind::UserDecryption(UserDecryptionResponse {
            decryption_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?),
            user_decrypted_shares: row.try_get("user_decrypted_shares")?,
            signature: row.try_get("signature")?,
            extra_data: row.try_get("extra_data")?,
        }),
    })
}

pub fn from_prep_keygen_row(row: &PgRow) -> anyhow::Result<KmsResponse> {
    Ok(KmsResponse {
        otlp_context: bc2wrap::deserialize(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
        kind: KmsResponseKind::PrepKeygen(PrepKeygenResponse {
            prep_keygen_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("prep_keygen_id")?),
            signature: row.try_get("signature")?,
        }),
    })
}

pub fn from_keygen_row(row: &PgRow) -> anyhow::Result<KmsResponse> {
    Ok(KmsResponse {
        otlp_context: bc2wrap::deserialize(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
        kind: KmsResponseKind::Keygen(KeygenResponse {
            key_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("key_id")?),
            key_digests: row.try_get("key_digests")?,
            signature: row.try_get("signature")?,
        }),
    })
}

pub fn from_crsgen_row(row: &PgRow) -> anyhow::Result<KmsResponse> {
    Ok(KmsResponse {
        otlp_context: bc2wrap::deserialize(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
        kind: KmsResponseKind::Crsgen(CrsgenResponse {
            crs_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("crs_id")?),
            crs_digest: row.try_get("crs_digest")?,
            signature: row.try_get("signature")?,
        }),
    })
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
        let result = abi_encode_plaintexts(&payload.plaintexts)?;

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
        let serialized_response_payload = bc2wrap::serialize(&payload)
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
                .ok_or_else(|| anyhow!("No request_id in `KeyGenResult`"))?
                .request_id,
        )?)
        .ok_or_else(|| anyhow!("Failed to parse request_id: {:?}", grpc_response.request_id))?;

        let key_digests = grpc_response
            .key_digests
            .into_iter()
            .map(|kd| {
                Ok(KeyDigestDbItem {
                    key_type: kd.key_type.parse()?,
                    digest: kd.digest,
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

impl CrsgenResponse {
    fn process(grpc_response: CrsGenResult) -> anyhow::Result<Self> {
        let crs_id = U256::try_from_be_slice(&hex::decode(
            &grpc_response
                .request_id
                .as_ref()
                .ok_or_else(|| anyhow!("No request_id in `CrsGenResult`"))?
                .request_id,
        )?)
        .ok_or_else(|| anyhow!("Failed to parse request_id: {:?}", grpc_response.request_id))?;

        Ok(CrsgenResponse {
            crs_id,
            crs_digest: grpc_response.crs_digest,
            signature: grpc_response.external_signature,
        })
    }
}

impl Display for KmsResponseKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KmsResponseKind::PublicDecryption(r) => {
                write!(f, "PublicDecryptionResponse #{}", r.decryption_id)
            }
            KmsResponseKind::UserDecryption(r) => {
                write!(f, "UserDecryptionResponse #{}", r.decryption_id)
            }
            KmsResponseKind::PrepKeygen(r) => {
                write!(f, "PrepKeygenResponse #{}", r.prep_keygen_id)
            }
            KmsResponseKind::Keygen(r) => {
                write!(f, "KeygenResponse #{}", r.key_id)
            }
            KmsResponseKind::Crsgen(r) => {
                write!(f, "CrsgenResponse #{}", r.crs_id)
            }
        }
    }
}

impl KmsResponseKind {
    /// Converts the `KmsResponseKind` in a `&str` format.
    pub fn as_str(&self) -> &'static str {
        match self {
            KmsResponseKind::PublicDecryption(_) => PUBLIC_DECRYPTION_RESPONSE_STR,
            KmsResponseKind::UserDecryption(_) => USER_DECRYPTION_RESPONSE_STR,
            KmsResponseKind::PrepKeygen(_) => PREP_KEYGEN_RESPONSE_STR,
            KmsResponseKind::Keygen(_) => KEYGEN_RESPONSE_STR,
            KmsResponseKind::Crsgen(_) => CRSGEN_RESPONSE_STR,
        }
    }
}

pub const PUBLIC_DECRYPTION_RESPONSE_STR: &str = "public_decryption_response";
pub const USER_DECRYPTION_RESPONSE_STR: &str = "user_decryption_response";
pub const PREP_KEYGEN_RESPONSE_STR: &str = "prep_keygen_response";
pub const KEYGEN_RESPONSE_STR: &str = "keygen_response";
pub const CRSGEN_RESPONSE_STR: &str = "crsgen_response";
