use crate::{
    monitoring::otlp::PropagationContext,
    tests::{
        rand::{rand_address, rand_public_key, rand_sns_ct, rand_u256},
        setup::{S3_CT_DIGEST, S3_CT_HANDLE},
    },
    types::{
        GatewayEventKind,
        db::{EventType, OperationStatus, ParamsTypeDb, SnsCiphertextMaterialDbItem},
        gw_event::PRSS_INIT_ID,
    },
};
use alloy::{
    hex,
    primitives::{FixedBytes, U256},
};
use anyhow::anyhow;
use fhevm_gateway_bindings::{
    decryption::Decryption::{
        PublicDecryptionRequest, SnsCiphertextMaterial, UserDecryptionRequest,
    },
    kms_generation::KMSGeneration::{
        CrsgenRequest, KeyReshareSameSet, KeygenRequest, PRSSInit, PrepKeygenRequest,
    },
};
use sqlx::{Pool, Postgres, types::chrono::Utc};
use tracing::info;

pub async fn insert_rand_request(
    db: &Pool<Postgres>,
    event_type: EventType,
    options: InsertRequestOptions,
) -> anyhow::Result<GatewayEventKind> {
    let inserted_response = match event_type {
        EventType::PublicDecryptionRequest => insert_rand_public_decryption_request(db, options)
            .await?
            .into(),
        EventType::UserDecryptionRequest => insert_rand_user_decryption_request(db, options)
            .await?
            .into(),
        EventType::PrepKeygenRequest => insert_rand_prep_keygen_request(db, options).await?.into(),
        EventType::KeygenRequest => insert_rand_keygen_request(db, options).await?.into(),
        EventType::CrsgenRequest => insert_rand_crsgen_request(db, options).await?.into(),
        EventType::PrssInit => insert_rand_prss_init(db, options).await?.into(),
        EventType::KeyReshareSameSet => insert_rand_key_reshare_same_set(db, options).await?.into(),
    };
    Ok(inserted_response)
}

pub async fn insert_rand_public_decryption_request(
    db: &Pool<Postgres>,
    options: InsertRequestOptions,
) -> anyhow::Result<PublicDecryptionRequest> {
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
    let extra_data = vec![];
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
    let extra_data = vec![];
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

pub async fn insert_rand_prep_keygen_request(
    db: &Pool<Postgres>,
    options: InsertRequestOptions,
) -> anyhow::Result<PrepKeygenRequest> {
    let prep_keygen_request_id = options.id.unwrap_or_else(rand_u256);
    let epoch_id = rand_u256();
    let params_type = ParamsTypeDb::Test;
    let status = options.status.unwrap_or(OperationStatus::Pending);

    sqlx::query!(
        "INSERT INTO prep_keygen_requests(\
            prep_keygen_id, epoch_id, params_type, otlp_context, created_at, already_sent, status\
        ) \
        VALUES ($1, $2, $3, $4, $5, $6, $7)",
        prep_keygen_request_id.as_le_slice(),
        epoch_id.as_le_slice(),
        params_type as ParamsTypeDb,
        bc2wrap::serialize(&PropagationContext::empty())?,
        Utc::now(),
        options.already_sent,
        status as OperationStatus,
    )
    .execute(db)
    .await?;

    Ok(PrepKeygenRequest {
        prepKeygenId: prep_keygen_request_id,
        epochId: epoch_id,
        paramsType: params_type as u8,
    })
}

pub async fn insert_rand_keygen_request(
    db: &Pool<Postgres>,
    options: InsertRequestOptions,
) -> anyhow::Result<KeygenRequest> {
    let key_id = options.id.unwrap_or_else(rand_u256);
    let prep_key_id = rand_u256();
    let status = options.status.unwrap_or(OperationStatus::Pending);

    sqlx::query!(
        "INSERT INTO keygen_requests(\
            prep_keygen_id, key_id, created_at, otlp_context, already_sent, status\
        ) \
        VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
        prep_key_id.as_le_slice(),
        key_id.as_le_slice(),
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

    sqlx::query!(
        "INSERT INTO crsgen_requests(\
            crs_id, max_bit_length, params_type, created_at, otlp_context, already_sent, status\
        ) \
        VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT DO NOTHING",
        crs_id.as_le_slice(),
        max_bit_length.as_le_slice(),
        params_type as ParamsTypeDb,
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
    })
}

pub async fn insert_rand_prss_init(
    db: &Pool<Postgres>,
    options: InsertRequestOptions,
) -> anyhow::Result<PRSSInit> {
    let prss_init_id = options.id.unwrap_or(PRSS_INIT_ID);
    let status = options.status.unwrap_or(OperationStatus::Pending);

    sqlx::query!(
        "INSERT INTO prss_init(id, created_at, otlp_context, status)
        VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        prss_init_id.as_le_slice(),
        Utc::now(),
        bc2wrap::serialize(&PropagationContext::empty())?,
        status as OperationStatus,
    )
    .execute(db)
    .await?;

    Ok(PRSSInit)
}

pub async fn insert_rand_key_reshare_same_set(
    db: &Pool<Postgres>,
    options: InsertRequestOptions,
) -> anyhow::Result<KeyReshareSameSet> {
    let key_id = options.id.unwrap_or_else(rand_u256);
    let prep_keygen_id = rand_u256();
    let key_reshare_id = rand_u256();
    let params_type = ParamsTypeDb::Test;
    let status = options.status.unwrap_or(OperationStatus::Pending);

    sqlx::query!(
        "INSERT INTO key_reshare_same_set(\
            prep_keygen_id, key_id, key_reshare_id, params_type, created_at, otlp_context, status\
        ) \
        VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT DO NOTHING",
        prep_keygen_id.as_le_slice(),
        key_id.as_le_slice(),
        key_reshare_id.as_le_slice(),
        params_type as ParamsTypeDb,
        Utc::now(),
        bc2wrap::serialize(&PropagationContext::empty())?,
        status as OperationStatus,
    )
    .execute(db)
    .await?;

    Ok(KeyReshareSameSet {
        prepKeygenId: prep_keygen_id,
        keyId: key_id,
        keyReshareId: key_reshare_id,
        paramsType: params_type as u8,
    })
}

pub async fn check_no_uncompleted_request_in_db(
    db: &Pool<Postgres>,
    event_type: EventType,
) -> anyhow::Result<()> {
    info!("Checking no pending requests are remaining in DB...");
    let query = match event_type {
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
        EventType::PrssInit => {
            "SELECT COUNT(id) FROM prss_init WHERE status NOT IN ('completed', 'failed')"
        }
        EventType::KeyReshareSameSet => {
            "SELECT COUNT(key_id) FROM key_reshare_same_set \
            WHERE status NOT IN ('completed', 'failed')"
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
    event_type: EventType,
) -> anyhow::Result<()> {
    info!("Checking request is marked as failed in DB...");
    let query = match event_type {
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
        EventType::PrssInit => "SELECT COUNT(id) FROM prss_init WHERE status = 'failed'",
        EventType::KeyReshareSameSet => {
            "SELECT COUNT(key_id) FROM key_reshare_same_set WHERE status = 'failed'"
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
}
