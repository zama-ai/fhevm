use crate::{
    monitoring::otlp::PropagationContext,
    tests::{
        rand::{rand_address, rand_public_key, rand_signature, rand_sns_ct, rand_u256},
        setup::{S3_CT_DIGEST, S3_CT_HANDLE, TESTING_KMS_CONTEXT, TESTING_KMS_EPOCH},
    },
    types::{
        ProtocolEventKind,
        db::{EventType, OperationStatus, ParamsTypeDb, SnsCiphertextMaterialDbItem},
        extra_data::EXTRA_DATA_V2_VERSION,
    },
};
use alloy::{
    hex,
    primitives::{FixedBytes, U256},
};
use anyhow::anyhow;
use fhevm_gateway_bindings::decryption::{
    Decryption::{
        HandleEntry, PublicDecryptionRequest, SnsCiphertextMaterial,
        UserDecryptionRequest_0 as UserDecryptionRequest,
        UserDecryptionRequest_1 as UserDecryptionRequestV2,
    },
    IDecryption::{RequestValiditySeconds, UserDecryptionRequestPayload},
};
use fhevm_host_bindings::kms_generation::KMSGeneration::{
    CrsgenRequest, KeygenRequest, PrepKeygenRequest,
};
use sqlx::{Pool, Postgres, types::chrono::Utc};
use std::fmt::Display;
use tracing::info;

/// Test-only event discriminator. Unlike `EventType` — which mirrors the Postgres `event_type` enum
/// and collapses legacy + RFC016 user decryptions into a single `UserDecryptionRequest` variant —
/// `TestEventType` has distinct `UserDecryption` and `UserDecryptionV2` cases. This lets
/// parametrized tests exercise both events.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TestEventType {
    PublicDecryption,
    UserDecryption,
    UserDecryptionV2,
    PrepKeygen,
    Keygen,
    Crsgen,
}

impl TestEventType {
    pub fn event_type(self) -> EventType {
        match self {
            Self::PublicDecryption => EventType::PublicDecryptionRequest,
            Self::UserDecryption | Self::UserDecryptionV2 => EventType::UserDecryptionRequest,
            Self::PrepKeygen => EventType::PrepKeygenRequest,
            Self::Keygen => EventType::KeygenRequest,
            Self::Crsgen => EventType::CrsgenRequest,
        }
    }
}

impl Display for TestEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserDecryptionV2 => write!(f, "UserDecryptionRequestV2"),
            _ => Display::fmt(&self.event_type(), f),
        }
    }
}

pub async fn insert_rand_request(
    db: &Pool<Postgres>,
    event_type: TestEventType,
    options: InsertRequestOptions,
) -> anyhow::Result<ProtocolEventKind> {
    let inserted_response = match event_type {
        TestEventType::PublicDecryption => insert_rand_public_decryption_request(db, options)
            .await?
            .into(),
        TestEventType::UserDecryption => insert_rand_user_decryption_request(db, options)
            .await?
            .into(),
        TestEventType::UserDecryptionV2 => insert_rand_user_decryption_request_v2(db, options)
            .await?
            .into(),
        TestEventType::PrepKeygen => insert_rand_prep_keygen_request(db, options).await?.into(),
        TestEventType::Keygen => insert_rand_keygen_request(db, options).await?.into(),
        TestEventType::Crsgen => insert_rand_crsgen_request(db, options).await?.into(),
    };
    Ok(inserted_response)
}

pub async fn insert_rand_public_decryption_request(
    db: &Pool<Postgres>,
    options: InsertRequestOptions,
) -> anyhow::Result<PublicDecryptionRequest> {
    let decryption_id = options.id.unwrap_or_else(rand_u256);
    let extra_data = options.build_extra_data();
    let sns_cts = match options.sns_ct_materials {
        Some(materials) => materials,
        None => {
            let mut sns_ct = rand_sns_ct();
            sns_ct.ctHandle = FixedBytes::from_slice(&hex::decode(S3_CT_HANDLE)?);
            sns_ct.snsCiphertextDigest = FixedBytes::from_slice(&hex::decode(S3_CT_DIGEST)?);
            vec![sns_ct]
        }
    };
    let status = options.status.unwrap_or(OperationStatus::Pending);

    let sns_ciphertexts_db = sns_cts
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();

    sqlx::query!(
        "INSERT INTO public_decryption_requests(\
            decryption_id, sns_ct_materials, extra_data, tx_hash, created_at, otlp_context,
            already_sent, status\
        ) \
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
        extra_data.clone(),
        options.tx_hash.map(|h| h.to_vec()),
        Utc::now(),
        bc2wrap::serialize(&PropagationContext::empty())?,
        options.already_sent,
        status as OperationStatus,
    )
    .execute(db)
    .await?;

    Ok(PublicDecryptionRequest {
        decryptionId: decryption_id,
        snsCtMaterials: sns_cts,
        extraData: extra_data.into(),
    })
}

pub async fn insert_rand_user_decryption_request(
    db: &Pool<Postgres>,
    options: InsertRequestOptions,
) -> anyhow::Result<UserDecryptionRequest> {
    let decryption_id = options.id.unwrap_or_else(rand_u256);
    let extra_data = options.build_extra_data();
    let sns_cts = match options.sns_ct_materials {
        Some(materials) => materials,
        None => {
            let mut sns_ct = rand_sns_ct();
            sns_ct.ctHandle = FixedBytes::from_slice(&hex::decode(S3_CT_HANDLE)?);
            sns_ct.snsCiphertextDigest = FixedBytes::from_slice(&hex::decode(S3_CT_DIGEST)?);
            vec![sns_ct]
        }
    };
    let user_address = rand_address();
    let public_key = rand_public_key();

    let status = options.status.unwrap_or(OperationStatus::Pending);
    let sns_ciphertexts_db = sns_cts
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();

    sqlx::query!(
        "INSERT INTO user_decryption_requests(\
            decryption_id, sns_ct_materials, user_address, public_key, extra_data, tx_hash,\
            created_at, otlp_context, already_sent, status\
        ) \
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
        user_address.as_slice(),
        &public_key,
        extra_data.clone(),
        options.tx_hash.map(|h| h.to_vec()),
        Utc::now(),
        bc2wrap::serialize(&PropagationContext::empty())?,
        options.already_sent,
        status as OperationStatus,
    )
    .execute(db)
    .await?;

    Ok(UserDecryptionRequest {
        decryptionId: decryption_id,
        snsCtMaterials: sns_cts,
        userAddress: user_address,
        publicKey: public_key.into(),
        extraData: extra_data.into(),
    })
}

pub async fn insert_rand_user_decryption_request_v2(
    db: &Pool<Postgres>,
    options: InsertRequestOptions,
) -> anyhow::Result<UserDecryptionRequestV2> {
    let decryption_id = options.id.unwrap_or_else(rand_u256);
    let sns_cts = match options.sns_ct_materials {
        Some(materials) => materials,
        None => {
            let mut sns_ct = rand_sns_ct();
            sns_ct.ctHandle = FixedBytes::from_slice(&hex::decode(S3_CT_HANDLE)?);
            sns_ct.snsCiphertextDigest = FixedBytes::from_slice(&hex::decode(S3_CT_DIGEST)?);
            vec![sns_ct]
        }
    };
    let user_address = rand_address();
    let public_key = rand_public_key();
    let signature = rand_signature();

    let context_id = options.context_id.unwrap_or(TESTING_KMS_CONTEXT);
    let mut extra_data = vec![0x01];
    extra_data.extend(context_id.to_be_bytes_vec());

    let status = options.status.unwrap_or(OperationStatus::Pending);

    // Mock-friendly shape: `ownerAddress == userAddress` on every handle (direct ownership path) and
    // empty `allowedContracts` (permissive mode). The worker's `check_user_decryption_request_v2`
    // therefore issues exactly one `isAllowed(handle, userAddress)` call per handle and skips the
    // per-`allowedContracts` loop entirely — so tests can mock ACL responses as `vec![true; n_handles]`.
    let handles: Vec<HandleEntry> = sns_cts
        .iter()
        .map(|m| HandleEntry {
            handle: m.ctHandle,
            contractAddress: rand_address(),
            ownerAddress: user_address,
        })
        .collect();
    let handle_owner_addresses: Vec<Vec<u8>> =
        handles.iter().map(|h| h.ownerAddress.to_vec()).collect();
    let handle_contract_addresses: Vec<Vec<u8>> =
        handles.iter().map(|h| h.contractAddress.to_vec()).collect();

    let allowed_contracts: Vec<Vec<u8>> = vec![];

    // Validity window that always covers `now`: started 1h ago, runs for 1 day. Going through `i64`
    // here (rather than persisting the `U256` fields as bytes) matches what `publish_user_decryption_v2`
    // does and what the row reader expects.
    let now_secs = Utc::now().timestamp();
    let start_timestamp: i64 = now_secs - 3600;
    let duration_seconds: i64 = 24 * 3600;

    let sns_ciphertexts_db = sns_cts
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();

    sqlx::query!(
        "INSERT INTO user_decryption_requests(\
            decryption_id, sns_ct_materials, user_address, public_key, extra_data, tx_hash,\
            created_at, otlp_context, already_sent, status, handle_owner_addresses,\
            handle_contract_addresses, allowed_contracts, start_timestamp, duration_seconds,\
            signature\
        ) \
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) \
        ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
        user_address.as_slice(),
        &public_key,
        extra_data.clone(),
        options.tx_hash.map(|h| h.to_vec()),
        Utc::now(),
        bc2wrap::serialize(&PropagationContext::empty())?,
        options.already_sent,
        status as OperationStatus,
        &handle_owner_addresses,
        &handle_contract_addresses,
        &allowed_contracts,
        start_timestamp,
        duration_seconds,
        &signature,
    )
    .execute(db)
    .await?;

    Ok(UserDecryptionRequestV2 {
        decryptionId: decryption_id,
        snsCtMaterials: sns_cts,
        handles,
        payload: UserDecryptionRequestPayload {
            userAddress: user_address,
            publicKey: public_key.into(),
            allowedContracts: vec![],
            requestValidity: RequestValiditySeconds {
                startTimestamp: U256::from(start_timestamp as u64),
                durationSeconds: U256::from(duration_seconds as u64),
            },
            extraData: extra_data.into(),
            signature: signature.into(),
        },
    })
}

pub async fn insert_rand_prep_keygen_request(
    db: &Pool<Postgres>,
    options: InsertRequestOptions,
) -> anyhow::Result<PrepKeygenRequest> {
    let prep_keygen_request_id = options.id.unwrap_or_else(rand_u256);
    let params_type = ParamsTypeDb::Test;
    let status = options.status.unwrap_or(OperationStatus::Pending);
    let extra_data = options.build_extra_data();

    sqlx::query!(
        "INSERT INTO prep_keygen_requests(\
            prep_keygen_id, params_type, extra_data, otlp_context, created_at, already_sent, status\
        ) \
        VALUES ($1, $2, $3, $4, $5, $6, $7)",
        prep_keygen_request_id.as_le_slice(),
        params_type as ParamsTypeDb,
        extra_data.to_vec() as Vec<u8>,
        bc2wrap::serialize(&PropagationContext::empty())?,
        Utc::now(),
        options.already_sent,
        status as OperationStatus,
    )
    .execute(db)
    .await?;

    Ok(PrepKeygenRequest {
        prepKeygenId: prep_keygen_request_id,
        paramsType: params_type as u8,
        extraData: extra_data.into(),
    })
}

pub async fn insert_rand_keygen_request(
    db: &Pool<Postgres>,
    options: InsertRequestOptions,
) -> anyhow::Result<KeygenRequest> {
    let key_id = options.id.unwrap_or_else(rand_u256);
    let prep_key_id = rand_u256();
    let status = options.status.unwrap_or(OperationStatus::Pending);
    let extra_data = options.build_extra_data();

    sqlx::query!(
        "INSERT INTO keygen_requests(\
            prep_keygen_id, key_id, extra_data, created_at, otlp_context, already_sent, status\
        ) \
        VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT DO NOTHING",
        prep_key_id.as_le_slice(),
        key_id.as_le_slice(),
        extra_data.to_vec() as Vec<u8>,
        Utc::now(),
        bc2wrap::serialize(&PropagationContext::empty())?,
        options.already_sent,
        status as OperationStatus,
    )
    .execute(db)
    .await?;

    Ok(KeygenRequest {
        prepKeygenId: prep_key_id,
        keyId: key_id,
        extraData: extra_data.into(),
    })
}

pub async fn insert_rand_crsgen_request(
    db: &Pool<Postgres>,
    options: InsertRequestOptions,
) -> anyhow::Result<CrsgenRequest> {
    let crs_id = options.id.unwrap_or_else(rand_u256);
    let max_bit_length = rand_u256();
    let params_type = ParamsTypeDb::Test;
    let status = options.status.unwrap_or(OperationStatus::Pending);
    let extra_data = options.build_extra_data();

    sqlx::query!(
        "INSERT INTO crsgen_requests(\
            crs_id, max_bit_length, params_type, extra_data, created_at, otlp_context, \
            already_sent, status\
        ) \
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT DO NOTHING",
        crs_id.as_le_slice(),
        max_bit_length.as_le_slice(),
        params_type as ParamsTypeDb,
        extra_data.to_vec() as Vec<u8>,
        Utc::now(),
        bc2wrap::serialize(&PropagationContext::empty())?,
        options.already_sent,
        status as OperationStatus,
    )
    .execute(db)
    .await?;

    Ok(CrsgenRequest {
        crsId: crs_id,
        maxBitLength: max_bit_length,
        paramsType: params_type as u8,
        extraData: extra_data.into(),
    })
}

pub async fn check_no_uncompleted_request_in_db(
    db: &Pool<Postgres>,
    kind: TestEventType,
) -> anyhow::Result<()> {
    info!("Checking no pending requests are remaining in DB...");
    let query = match kind.event_type() {
        EventType::PublicDecryptionRequest => {
            "SELECT COUNT(decryption_id) FROM public_decryption_requests \
            WHERE status NOT IN ('completed', 'failed')"
        }
        EventType::UserDecryptionRequest => {
            "SELECT COUNT(decryption_id) FROM user_decryption_requests \
            WHERE status NOT IN ('completed', 'failed')"
        }
        EventType::PrepKeygenRequest => {
            "SELECT COUNT(prep_keygen_id) FROM prep_keygen_requests \
            WHERE status NOT IN ('completed', 'failed')"
        }
        EventType::KeygenRequest => {
            "SELECT COUNT(key_id) FROM keygen_requests WHERE status NOT IN ('completed', 'failed')"
        }
        EventType::CrsgenRequest => {
            "SELECT COUNT(crs_id) FROM crsgen_requests WHERE status NOT IN ('completed', 'failed')"
        }
    };
    let count: i64 = sqlx::query_scalar(query).fetch_one(db).await?;
    if count == 0 {
        info!("OK!");
        Ok(())
    } else {
        Err(anyhow!("Unexpected number of rows in DB: {count}"))
    }
}

pub async fn check_request_failed_in_db(
    db: &Pool<Postgres>,
    kind: TestEventType,
) -> anyhow::Result<()> {
    info!("Checking request is marked as failed in DB...");
    let query = match kind.event_type() {
        EventType::PublicDecryptionRequest => {
            "SELECT COUNT(decryption_id) FROM public_decryption_requests WHERE status = 'failed'"
        }
        EventType::UserDecryptionRequest => {
            "SELECT COUNT(decryption_id) FROM user_decryption_requests WHERE status = 'failed'"
        }
        EventType::PrepKeygenRequest => {
            "SELECT COUNT(prep_keygen_id) FROM prep_keygen_requests WHERE status = 'failed'"
        }
        EventType::KeygenRequest => {
            "SELECT COUNT(key_id) FROM keygen_requests WHERE status = 'failed'"
        }
        EventType::CrsgenRequest => {
            "SELECT COUNT(crs_id) FROM crsgen_requests WHERE status = 'failed'"
        }
    };
    let count: i64 = sqlx::query_scalar(query).fetch_one(db).await?;
    if count > 0 {
        info!("OK! Found {count} failed request(s).");
        Ok(())
    } else {
        Err(anyhow!("No failed requests found in DB"))
    }
}

#[derive(Default)]
pub struct InsertRequestOptions {
    pub id: Option<U256>,
    pub already_sent: bool,
    pub status: Option<OperationStatus>,
    pub tx_hash: Option<FixedBytes<32>>,
    pub sns_ct_materials: Option<Vec<SnsCiphertextMaterial>>,
    pub context_id: Option<U256>,
    pub epoch_id: Option<U256>,
}

impl InsertRequestOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_id(mut self, id: U256) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_already_sent(mut self, already_sent: bool) -> Self {
        self.already_sent = already_sent;
        self
    }

    pub fn with_status(mut self, status: OperationStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_tx_hash(mut self, tx_hash: FixedBytes<32>) -> Self {
        self.tx_hash = Some(tx_hash);
        self
    }

    pub fn with_sns_ct_materials(mut self, materials: Vec<SnsCiphertextMaterial>) -> Self {
        self.sns_ct_materials = Some(materials);
        self
    }

    pub fn with_context_id(mut self, context_id: U256) -> Self {
        self.context_id = Some(context_id);
        self
    }

    pub fn with_epoch_id(mut self, epoch_id: U256) -> Self {
        self.epoch_id = Some(epoch_id);
        self
    }

    pub fn build_extra_data(&self) -> Vec<u8> {
        let context_id = self.context_id.unwrap_or(TESTING_KMS_CONTEXT);
        let epoch_id = self.epoch_id.unwrap_or(TESTING_KMS_EPOCH);
        let mut extra_data = vec![EXTRA_DATA_V2_VERSION];
        extra_data.extend(context_id.to_be_bytes_vec());
        extra_data.extend(epoch_id.to_be_bytes_vec());
        extra_data
    }
}
