use crate::{
    monitoring::otlp::PropagationContext,
    types::{
        KmsGrpcResponse,
        db::{KeyDigestDbItem, KeyType, OperationStatus},
        request_id_to_u256,
    },
};
use alloy::{hex, primitives::U256, sol_types::SolValue};
use anyhow::anyhow;
use fhevm_host_bindings::protocol_config::{
    IKMSGeneration::KeyDigest,
    IProtocolConfig::{EpochCrsResult, EpochKeyResult},
};
use kms_grpc::{
    kms::v1::{
        CrsGenResult, EpochResultResponse as GrpcEpochResultResponse, KeyGenPreprocResult,
        KeyGenResult, PublicDecryptionResponse as GrpcPublicDecryptionResponse,
        UserDecryptionResponse as GrpcUserDecryptionResponse,
    },
    rpc_types::abi_encode_plaintexts,
};
use sqlx::{
    Pool, Postgres, Row,
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
};
use std::fmt::Display;
use tracing::{debug, info, warn};

#[derive(Clone, Debug, PartialEq)]
pub struct KmsResponse {
    pub kind: KmsResponseKind,
    pub created_at: DateTime<Utc>,
    pub otlp_context: PropagationContext,
}

#[derive(Clone, Debug, PartialEq)]
pub enum KmsResponseKind {
    PublicDecryption(PublicDecryptionResponse),
    UserDecryption(UserDecryptionResponse),
    PrepKeygen(PrepKeygenResponse),
    Keygen(KeygenResponse),
    Crsgen(CrsgenResponse),
    NewKmsContext(NewKmsContextResponse),
    EpochResult(EpochResultResponse),
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

#[derive(Clone, Debug, PartialEq)]
pub struct NewKmsContextResponse {
    pub context_id: U256,
}

/// Result of an MPC epoch creation.
///
/// `keys` and `crs_list` are the ABI-encoded `EpochKeyResult[]` / `EpochCrsResult[]` arrays
/// expected by `confirmEpochActivation`.
#[derive(Clone, Debug, PartialEq)]
pub struct EpochResultResponse {
    pub context_id: U256,
    pub epoch_id: U256,
    pub keys: Vec<u8>,
    pub crs_list: Vec<u8>,
}

impl KmsResponse {
    pub fn new(kind: KmsResponseKind, otlp_context: PropagationContext) -> Self {
        Self {
            kind,
            created_at: Utc::now(),
            otlp_context,
        }
    }

    /// Sets the response's `status` field to `pending` in the database.
    pub async fn mark_as_pending(&self, db: &Pool<Postgres>) -> anyhow::Result<()> {
        warn!("Failed to process response. Restoring `status` field to `pending` in DB...");
        self.update_status(db, OperationStatus::Pending).await
    }

    /// Sets the response's `status` field to `completed` in the database.
    pub async fn mark_as_completed(&self, db: &Pool<Postgres>) -> anyhow::Result<()> {
        info!(
            "Response successfully processed. Setting its `status` field to `completed` in DB..."
        );
        self.update_status(db, OperationStatus::Completed).await
    }

    /// Sets the response's `status` field to `failed` in the database.
    pub async fn mark_as_failed(&self, db: &Pool<Postgres>) -> anyhow::Result<()> {
        warn!("Failed to process response. Restoring `status` field to `failed` in DB...");
        self.update_status(db, OperationStatus::Failed).await
    }

    async fn update_status(
        &self,
        db: &Pool<Postgres>,
        status: OperationStatus,
    ) -> anyhow::Result<()> {
        let query = match &self.kind {
            KmsResponseKind::PublicDecryption(r) => sqlx::query!(
                "UPDATE public_decryption_responses SET status = $1 WHERE decryption_id = $2",
                status as OperationStatus,
                r.decryption_id.as_le_slice()
            ),
            KmsResponseKind::UserDecryption(r) => sqlx::query!(
                "UPDATE user_decryption_responses SET status = $1 WHERE decryption_id = $2",
                status as OperationStatus,
                r.decryption_id.as_le_slice()
            ),
            KmsResponseKind::PrepKeygen(r) => sqlx::query!(
                "UPDATE prep_keygen_responses SET status = $1 WHERE prep_keygen_id = $2",
                status as OperationStatus,
                r.prep_keygen_id.as_le_slice()
            ),
            KmsResponseKind::Keygen(r) => sqlx::query!(
                "UPDATE keygen_responses SET status = $1 WHERE key_id = $2",
                status as OperationStatus,
                r.key_id.as_le_slice()
            ),
            KmsResponseKind::Crsgen(r) => sqlx::query!(
                "UPDATE crsgen_responses SET status = $1 WHERE crs_id = $2",
                status as OperationStatus,
                r.crs_id.as_le_slice()
            ),
            KmsResponseKind::NewKmsContext(r) => sqlx::query!(
                "UPDATE new_kms_context_responses SET status = $1 WHERE context_id = $2",
                status as OperationStatus,
                r.context_id.as_le_slice()
            ),
            KmsResponseKind::EpochResult(r) => sqlx::query!(
                "UPDATE epoch_result_responses SET status = $1 WHERE epoch_id = $2",
                status as OperationStatus,
                r.epoch_id.as_le_slice()
            ),
        };

        match query.execute(db).await {
            Ok(r) if r.rows_affected() == 1 => {
                info!("Successfully updated response in DB!");
                Ok(())
            }
            Ok(r) => Err(anyhow!(
                "Failed to set {} status to `{status}` in DB: unexpected query result: {r:?}",
                self.kind
            )),
            Err(e) => Err(anyhow!(
                "Failed to set {} status to `{status}` in DB: {e}",
                self.kind
            )),
        }
    }
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
            KmsGrpcResponse::NewKmsContext { context_id } => {
                Ok(Self::NewKmsContext(NewKmsContextResponse { context_id }))
            }
            KmsGrpcResponse::EpochResult {
                context_id,
                epoch_id,
                grpc_response,
            } => {
                let (keys, crs_list) = encode_epoch_result(grpc_response)?;
                Ok(Self::EpochResult(EpochResultResponse {
                    context_id,
                    epoch_id,
                    keys,
                    crs_list,
                }))
            }
            KmsGrpcResponse::NoResponseExpected => {
                Err(anyhow!("No response expected from KMS. Nothing to process"))
            }
        }
    }
}

/// Converts the Core's `EpochResultResponse` into the ABI-encoded `(EpochKeyResult[],
/// EpochCrsResult[])` payload expected by `confirmEpochActivation`.
fn encode_epoch_result(response: GrpcEpochResultResponse) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let (mut keys, mut crs_list) = (vec![], vec![]);
    for key in response.reshare_responses {
        let Some(key_id) = key.request_id.map(request_id_to_u256).transpose()? else {
            return Err(anyhow!("KeyGenResult missing key_id"));
        };
        let Some(preproc_id) = key.preprocessing_id.map(request_id_to_u256).transpose()? else {
            return Err(anyhow!("KeyGenResult missing preprocessing_id"));
        };
        let mut key_digests = vec![];
        for kd in key.key_digests {
            key_digests.push(KeyDigest {
                keyType: kd.key_type.parse::<KeyType>()? as u8,
                digest: kd.digest.into(),
            });
        }
        keys.push(EpochKeyResult {
            prepKeygenId: preproc_id,
            keyId: key_id,
            keyDigests: key_digests,
            signature: key.external_signature.into(),
        });
    }

    for crs in response.crs_responses {
        let Some(crs_id) = crs.request_id.map(request_id_to_u256).transpose()? else {
            return Err(anyhow!("CrsGenResult missing request_id"));
        };
        crs_list.push(EpochCrsResult {
            crsId: crs_id,
            maxBitLength: U256::from(crs.max_num_bits),
            crsDigest: crs.crs_digest.into(),
            signature: crs.external_signature.into(),
        });
    }

    Ok((keys.abi_encode(), crs_list.abi_encode()))
}

pub fn from_public_decryption_row(row: &PgRow) -> anyhow::Result<KmsResponse> {
    Ok(KmsResponse {
        otlp_context: bc2wrap::deserialize_slice(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
        kind: KmsResponseKind::PublicDecryption(PublicDecryptionResponse {
            decryption_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?),
            decrypted_result: row.try_get("decrypted_result")?,
            signature: row.try_get("signature")?,
            extra_data: row.try_get("extra_data")?,
        }),
        created_at: row.try_get("created_at")?,
    })
}

pub fn from_user_decryption_row(row: &PgRow) -> anyhow::Result<KmsResponse> {
    Ok(KmsResponse {
        otlp_context: bc2wrap::deserialize_slice(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
        kind: KmsResponseKind::UserDecryption(UserDecryptionResponse {
            decryption_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?),
            user_decrypted_shares: row.try_get("user_decrypted_shares")?,
            signature: row.try_get("signature")?,
            extra_data: row.try_get("extra_data")?,
        }),
        created_at: row.try_get("created_at")?,
    })
}

pub fn from_prep_keygen_row(row: &PgRow) -> anyhow::Result<KmsResponse> {
    Ok(KmsResponse {
        otlp_context: bc2wrap::deserialize_slice(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
        kind: KmsResponseKind::PrepKeygen(PrepKeygenResponse {
            prep_keygen_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("prep_keygen_id")?),
            signature: row.try_get("signature")?,
        }),
        created_at: row.try_get("created_at")?,
    })
}

pub fn from_keygen_row(row: &PgRow) -> anyhow::Result<KmsResponse> {
    Ok(KmsResponse {
        otlp_context: bc2wrap::deserialize_slice(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
        kind: KmsResponseKind::Keygen(KeygenResponse {
            key_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("key_id")?),
            key_digests: row.try_get("key_digests")?,
            signature: row.try_get("signature")?,
        }),
        created_at: row.try_get("created_at")?,
    })
}

pub fn from_crsgen_row(row: &PgRow) -> anyhow::Result<KmsResponse> {
    Ok(KmsResponse {
        otlp_context: bc2wrap::deserialize_slice(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
        kind: KmsResponseKind::Crsgen(CrsgenResponse {
            crs_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("crs_id")?),
            crs_digest: row.try_get("crs_digest")?,
            signature: row.try_get("signature")?,
        }),
        created_at: row.try_get("created_at")?,
    })
}

pub fn from_new_kms_context_response_row(row: &PgRow) -> anyhow::Result<KmsResponse> {
    Ok(KmsResponse {
        otlp_context: bc2wrap::deserialize_slice(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
        kind: KmsResponseKind::NewKmsContext(NewKmsContextResponse {
            context_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("context_id")?),
        }),
        created_at: row.try_get("created_at")?,
    })
}

pub fn from_epoch_result_row(row: &PgRow) -> anyhow::Result<KmsResponse> {
    Ok(KmsResponse {
        otlp_context: bc2wrap::deserialize_slice(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
        kind: KmsResponseKind::EpochResult(EpochResultResponse {
            context_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("context_id")?),
            epoch_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("epoch_id")?),
            keys: row.try_get("keys")?,
            crs_list: row.try_get("crs_list")?,
        }),
        created_at: row.try_get("created_at")?,
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
            KmsResponseKind::NewKmsContext(r) => {
                write!(f, "NewKmsContextResponse #{}", r.context_id)
            }
            KmsResponseKind::EpochResult(r) => write!(
                f,
                "EpochResultResponse context #{} epoch #{}",
                r.context_id, r.epoch_id
            ),
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
            KmsResponseKind::NewKmsContext(_) => NEW_KMS_CONTEXT_RESPONSE_STR,
            KmsResponseKind::EpochResult(_) => EPOCH_RESULT_RESPONSE_STR,
        }
    }
}

pub const PUBLIC_DECRYPTION_RESPONSE_STR: &str = "public_decryption_response";
pub const USER_DECRYPTION_RESPONSE_STR: &str = "user_decryption_response";
pub const PREP_KEYGEN_RESPONSE_STR: &str = "prep_keygen_response";
pub const KEYGEN_RESPONSE_STR: &str = "keygen_response";
pub const CRSGEN_RESPONSE_STR: &str = "crsgen_response";
pub const NEW_KMS_CONTEXT_RESPONSE_STR: &str = "new_kms_context_response";
pub const EPOCH_RESULT_RESPONSE_STR: &str = "epoch_result_response";
