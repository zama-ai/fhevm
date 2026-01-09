use crate::{
    monitoring::otlp::PropagationContext,
    tests::rand::{rand_address, rand_contract_addresses, rand_handles, rand_public_key, rand_signature, rand_u256},
    types::{
        GatewayEventKind, PublicDecryptionRequestV2, UserDecryptionRequestV2,
        db::{EventType, OperationStatus, ParamsTypeDb},
        gw_event::PRSS_INIT_ID,
    },
};
use alloy::primitives::U256;
use anyhow::anyhow;
use fhevm_gateway_bindings::kms_generation::KMSGeneration::{
    CrsgenRequest, KeyReshareSameSet, KeygenRequest, PRSSInit, PrepKeygenRequest,
};
use sqlx::{Pool, Postgres};
use tracing::info;

pub async fn insert_rand_request(
    db: &Pool<Postgres>,
    request_str: EventType,
    id: Option<U256>,
    already_sent: bool,
    status: Option<OperationStatus>,
) -> anyhow::Result<GatewayEventKind> {
    let inserted_response = match request_str {
        EventType::PublicDecryptionRequest => {
            insert_rand_public_decryption_request(db, id, already_sent, status)
                .await?
                .into()
        }
        EventType::UserDecryptionRequest => {
            insert_rand_user_decryption_request(db, id, already_sent, status)
                .await?
                .into()
        }
        EventType::PrepKeygenRequest => {
            insert_rand_prep_keygen_request(db, id, already_sent, status)
                .await?
                .into()
        }
        EventType::KeygenRequest => insert_rand_keygen_request(db, id, already_sent, status)
            .await?
            .into(),
        EventType::CrsgenRequest => insert_rand_crsgen_request(db, id, already_sent, status)
            .await?
            .into(),
        EventType::PrssInit => insert_rand_prss_init(db, id, status).await?.into(),
        EventType::KeyReshareSameSet => insert_rand_key_reshare_same_set(db, id, status)
            .await?
            .into(),
    };
    Ok(inserted_response)
}

pub async fn insert_rand_public_decryption_request(
    db: &Pool<Postgres>,
    id: Option<U256>,
    already_sent: bool,
    status: Option<OperationStatus>,
) -> anyhow::Result<PublicDecryptionRequestV2> {
    let request_id = id.unwrap_or_else(rand_u256);
    let handles = rand_handles(2);
    let contract_addresses = rand_contract_addresses(2);
    let chain_id = rand_u256();
    let timestamp = rand_u256();
    let status = status.unwrap_or(OperationStatus::Pending);

    let handles_bytes: Vec<[u8; 32]> = handles.iter().map(|h| h.0).collect();
    let contract_addresses_bytes: Vec<[u8; 20]> = contract_addresses.iter().map(|a| a.0 .0).collect();

    sqlx::query(
        "INSERT INTO public_decryption_requests(decryption_id, handles, contract_addresses, chain_id, timestamp, otlp_context, already_sent, status) \
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT DO NOTHING",
    )
    .bind(request_id.as_le_slice())
    .bind(handles_bytes)
    .bind(contract_addresses_bytes)
    .bind(chain_id.as_le_slice())
    .bind(timestamp.as_le_slice())
    .bind(bc2wrap::serialize(&PropagationContext::empty())?)
    .bind(already_sent)
    .bind(status as OperationStatus)
    .execute(db)
    .await?;

    Ok(PublicDecryptionRequestV2 {
        request_id,
        handles,
        contract_addresses,
        chain_id,
        timestamp,
    })
}

pub async fn insert_rand_user_decryption_request(
    db: &Pool<Postgres>,
    id: Option<U256>,
    already_sent: bool,
    status: Option<OperationStatus>,
) -> anyhow::Result<UserDecryptionRequestV2> {
    let request_id = id.unwrap_or_else(rand_u256);
    let handles = rand_handles(2);
    let contract_addresses = rand_contract_addresses(2);
    let user_address = rand_address();
    let public_key = rand_public_key();
    let signature = rand_signature();
    let chain_id = rand_u256();
    let timestamp = rand_u256();
    let status = status.unwrap_or(OperationStatus::Pending);

    let handles_bytes: Vec<[u8; 32]> = handles.iter().map(|h| h.0).collect();
    let contract_addresses_bytes: Vec<[u8; 20]> = contract_addresses.iter().map(|a| a.0 .0).collect();

    sqlx::query(
        "INSERT INTO user_decryption_requests(\
            decryption_id, handles, contract_addresses, user_address, public_key, signature, chain_id, timestamp, otlp_context, already_sent, status\
        ) \
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) ON CONFLICT DO NOTHING",
    )
    .bind(request_id.as_le_slice())
    .bind(handles_bytes)
    .bind(contract_addresses_bytes)
    .bind(user_address.as_slice())
    .bind(&public_key)
    .bind(&signature)
    .bind(chain_id.as_le_slice())
    .bind(timestamp.as_le_slice())
    .bind(bc2wrap::serialize(&PropagationContext::empty())?)
    .bind(already_sent)
    .bind(status as OperationStatus)
    .execute(db)
    .await?;

    Ok(UserDecryptionRequestV2 {
        request_id,
        handles,
        contract_addresses,
        user_address,
        public_key,
        signature,
        chain_id,
        timestamp,
    })
}

pub async fn insert_rand_prep_keygen_request(
    db: &Pool<Postgres>,
    id: Option<U256>,
    already_sent: bool,
    status: Option<OperationStatus>,
) -> anyhow::Result<PrepKeygenRequest> {
    let prep_keygen_request_id = id.unwrap_or_else(rand_u256);
    let epoch_id = rand_u256();
    let params_type = ParamsTypeDb::Test;
    let status = status.unwrap_or(OperationStatus::Pending);

    sqlx::query!(
        "INSERT INTO prep_keygen_requests(prep_keygen_id, epoch_id, params_type, otlp_context, already_sent, status) \
        VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
        prep_keygen_request_id.as_le_slice(),
        epoch_id.as_le_slice(),
        params_type as ParamsTypeDb,
        bc2wrap::serialize(&PropagationContext::empty())?,
        already_sent,
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
    id: Option<U256>,
    already_sent: bool,
    status: Option<OperationStatus>,
) -> anyhow::Result<KeygenRequest> {
    let key_id = id.unwrap_or_else(rand_u256);
    let prep_key_id = rand_u256();
    let status = status.unwrap_or(OperationStatus::Pending);

    sqlx::query!(
        "INSERT INTO keygen_requests(prep_keygen_id, key_id, otlp_context, already_sent, status) \
        VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
        prep_key_id.as_le_slice(),
        key_id.as_le_slice(),
        bc2wrap::serialize(&PropagationContext::empty())?,
        already_sent,
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
    id: Option<U256>,
    already_sent: bool,
    status: Option<OperationStatus>,
) -> anyhow::Result<CrsgenRequest> {
    let crs_id = id.unwrap_or_else(rand_u256);
    let max_bit_length = rand_u256();
    let params_type = ParamsTypeDb::Test;
    let status = status.unwrap_or(OperationStatus::Pending);

    sqlx::query!(
        "INSERT INTO crsgen_requests(crs_id, max_bit_length, params_type, otlp_context, already_sent, status) \
        VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
        crs_id.as_le_slice(),
        max_bit_length.as_le_slice(),
        params_type as ParamsTypeDb,
        bc2wrap::serialize(&PropagationContext::empty())?,
        already_sent,
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
    id: Option<U256>,
    status: Option<OperationStatus>,
) -> anyhow::Result<PRSSInit> {
    let prss_init_id = id.unwrap_or(PRSS_INIT_ID);
    let status = status.unwrap_or(OperationStatus::Pending);

    sqlx::query!(
        "INSERT INTO prss_init(id, otlp_context, status) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        prss_init_id.as_le_slice(),
        bc2wrap::serialize(&PropagationContext::empty())?,
        status as OperationStatus,
    )
    .execute(db)
    .await?;

    Ok(PRSSInit)
}

pub async fn insert_rand_key_reshare_same_set(
    db: &Pool<Postgres>,
    id: Option<U256>,
    status: Option<OperationStatus>,
) -> anyhow::Result<KeyReshareSameSet> {
    let key_id = id.unwrap_or_else(rand_u256);
    let prep_keygen_id = rand_u256();
    let key_reshare_id = rand_u256();
    let params_type = ParamsTypeDb::Test;
    let status = status.unwrap_or(OperationStatus::Pending);

    sqlx::query!(
        "INSERT INTO key_reshare_same_set(prep_keygen_id, key_id, key_reshare_id, params_type, otlp_context, status) \
        VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
        prep_keygen_id.as_le_slice(),
        key_id.as_le_slice(),
        key_reshare_id.as_le_slice(),
        params_type as ParamsTypeDb,
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
            "SELECT COUNT(request_id) FROM public_decryption_requests WHERE status = 'pending'"
        }
        EventType::UserDecryptionRequest => {
            "SELECT COUNT(request_id) FROM user_decryption_requests WHERE status = 'pending'"
        }
        EventType::PrepKeygenRequest => {
            "SELECT COUNT(prep_keygen_id) FROM prep_keygen_requests WHERE status = 'pending'"
        }
        EventType::KeygenRequest => {
            "SELECT COUNT(key_id) FROM keygen_requests WHERE status = 'pending'"
        }
        EventType::CrsgenRequest => {
            "SELECT COUNT(crs_id) FROM crsgen_requests WHERE status = 'pending'"
        }
        EventType::PrssInit => "SELECT COUNT(id) FROM prss_init WHERE status = 'pending'",
        EventType::KeyReshareSameSet => {
            "SELECT COUNT(key_id) FROM key_reshare_same_set WHERE status = 'pending'"
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
