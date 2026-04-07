use alloy::primitives::{FixedBytes, U256};
use anyhow::anyhow;
use connector_utils::{
    monitoring::otlp::PropagationContext,
    types::{
        ProtocolEvent, ProtocolEventKind,
        db::{EventType, ParamsTypeDb, SnsCiphertextMaterialDbItem},
    },
};
use fhevm_gateway_bindings::{
    decryption::Decryption::{PublicDecryptionRequest, UserDecryptionRequest},
    kms_generation::KMSGeneration::{CrsgenRequest, KeygenRequest, PrepKeygenRequest},
};
use sqlx::{
    PgExecutor, Pool, Postgres,
    postgres::PgQueryResult,
    types::chrono::{DateTime, Utc},
};
use tracing::{debug, info, warn};

/// Inserts all events and updates the last block polled in a single transaction.
/// On failure, the transaction is rolled back automatically.
#[tracing::instrument(skip_all)]
pub async fn publish_batch(
    db_pool: &Pool<Postgres>,
    events: Vec<ProtocolEvent>,
    event_types: &[EventType],
    block_number: u64,
) -> anyhow::Result<()> {
    let mut tx = db_pool.begin().await?;
    for event in events {
        publish_event_inner(&mut *tx, event).await?;
    }
    update_last_block_polled(&mut *tx, event_types, Some(block_number)).await?;
    tx.commit().await?;
    Ok(())
}

async fn publish_event_inner<'e>(
    executor: impl PgExecutor<'e>,
    event: ProtocolEvent,
) -> anyhow::Result<()> {
    info!("Storing {:?} in DB...", event.kind);

    let otlp_ctx = event.otlp_context;
    let tx_hash = event.tx_hash;
    let created_at = event.created_at;
    let query_result = match event.kind {
        ProtocolEventKind::PublicDecryption(e) => {
            publish_public_decryption(executor, e, tx_hash, created_at, otlp_ctx).await
        }
        ProtocolEventKind::UserDecryption(e) => {
            publish_user_decryption(executor, e, tx_hash, created_at, otlp_ctx).await
        }
        ProtocolEventKind::PrepKeygen(e) => {
            let params_type: ParamsTypeDb = e.paramsType.try_into()?;
            publish_prep_keygen_request(executor, e, params_type, tx_hash, created_at, otlp_ctx)
                .await
        }
        ProtocolEventKind::Keygen(e) => {
            publish_keygen_request(executor, e, tx_hash, created_at, otlp_ctx).await
        }
        ProtocolEventKind::Crsgen(e) => {
            let params_type: ParamsTypeDb = e.paramsType.try_into()?;
            publish_crsgen_request(executor, e, params_type, tx_hash, created_at, otlp_ctx).await
        }
    }
    .map_err(|err| anyhow!("Failed to publish event: {err}"))?;

    if query_result.rows_affected() == 1 {
        info!("Event successfully stored in DB!");
    } else {
        warn!("Unexpected query result while publishing event: {query_result:?}");
    }

    Ok(())
}

async fn publish_public_decryption<'e>(
    executor: impl PgExecutor<'e>,
    request: PublicDecryptionRequest,
    tx_hash: Option<FixedBytes<32>>,
    created_at: DateTime<Utc>,
    otlp_ctx: PropagationContext,
) -> anyhow::Result<PgQueryResult> {
    let sns_ciphertexts_db = request
        .snsCtMaterials
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();

    sqlx::query!(
        "INSERT INTO public_decryption_requests(\
            decryption_id, sns_ct_materials, extra_data, tx_hash, created_at, otlp_context\
        ) \
        VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
        request.decryptionId.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
        request.extraData.as_ref(),
        tx_hash.map(|h| h.to_vec()),
        created_at,
        bc2wrap::serialize(&otlp_ctx)?,
    )
    .execute(executor)
    .await
    .map_err(anyhow::Error::from)
}

async fn publish_user_decryption<'e>(
    executor: impl PgExecutor<'e>,
    request: UserDecryptionRequest,
    tx_hash: Option<FixedBytes<32>>,
    created_at: DateTime<Utc>,
    otlp_ctx: PropagationContext,
) -> anyhow::Result<PgQueryResult> {
    let sns_ciphertexts_db = request
        .snsCtMaterials
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();

    sqlx::query!(
        "INSERT INTO user_decryption_requests(\
            decryption_id, sns_ct_materials, user_address, public_key, extra_data, tx_hash,\
            created_at, otlp_context\
        ) \
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT DO NOTHING",
        request.decryptionId.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
        request.userAddress.as_slice(),
        request.publicKey.as_ref(),
        request.extraData.as_ref(),
        tx_hash.map(|h| h.to_vec()),
        created_at,
        bc2wrap::serialize(&otlp_ctx)?,
    )
    .execute(executor)
    .await
    .map_err(anyhow::Error::from)
}

async fn publish_prep_keygen_request<'e>(
    executor: impl PgExecutor<'e>,
    request: PrepKeygenRequest,
    params_type: ParamsTypeDb,
    tx_hash: Option<FixedBytes<32>>,
    created_at: DateTime<Utc>,
    otlp_ctx: PropagationContext,
) -> anyhow::Result<PgQueryResult> {
    sqlx::query!(
        "INSERT INTO prep_keygen_requests(\
            prep_keygen_id, epoch_id, params_type, tx_hash, created_at, otlp_context\
        ) \
        VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
        request.prepKeygenId.as_le_slice(),
        request.epochId.as_le_slice(),
        params_type as ParamsTypeDb,
        tx_hash.map(|h| h.to_vec()),
        created_at,
        bc2wrap::serialize(&otlp_ctx)?,
    )
    .execute(executor)
    .await
    .map_err(anyhow::Error::from)
}

async fn publish_keygen_request<'e>(
    executor: impl PgExecutor<'e>,
    request: KeygenRequest,
    tx_hash: Option<FixedBytes<32>>,
    created_at: DateTime<Utc>,
    otlp_ctx: PropagationContext,
) -> anyhow::Result<PgQueryResult> {
    sqlx::query!(
        "INSERT INTO keygen_requests(prep_keygen_id, key_id, tx_hash, created_at, otlp_context) \
            VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
        request.prepKeygenId.as_le_slice(),
        request.keyId.as_le_slice(),
        tx_hash.map(|h| h.to_vec()),
        created_at,
        bc2wrap::serialize(&otlp_ctx)?,
    )
    .execute(executor)
    .await
    .map_err(anyhow::Error::from)
}

async fn publish_crsgen_request<'e>(
    executor: impl PgExecutor<'e>,
    request: CrsgenRequest,
    params_type: ParamsTypeDb,
    tx_hash: Option<FixedBytes<32>>,
    created_at: DateTime<Utc>,
    otlp_ctx: PropagationContext,
) -> anyhow::Result<PgQueryResult> {
    sqlx::query!(
        "INSERT INTO crsgen_requests(\
            crs_id, max_bit_length, params_type, tx_hash, created_at, otlp_context\
        ) \
        VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
        request.crsId.as_le_slice(),
        request.maxBitLength.as_le_slice(),
        params_type as ParamsTypeDb,
        tx_hash.map(|h| h.to_vec()),
        created_at,
        bc2wrap::serialize(&otlp_ctx)?,
    )
    .execute(executor)
    .await
    .map_err(anyhow::Error::from)
}

/// Updates the registered last block polled in DB for the given event types.
#[tracing::instrument(skip_all)]
pub async fn update_last_block_polled<'e>(
    executor: impl PgExecutor<'e>,
    event_types: &[EventType],
    last_block_polled: Option<u64>,
) -> anyhow::Result<()> {
    info!(
        last_block_polled,
        "Updating last block polled in DB for {event_types:?}"
    );
    let query_result = sqlx::query!(
        "UPDATE last_block_polled SET block_number = $2, updated_at = $3 \
        WHERE event_type = ANY($1::event_type[]) AND (block_number IS NULL OR block_number < $2)",
        event_types as &[EventType],
        last_block_polled.map(|n| n as i64),
        Utc::now(),
    )
    .execute(executor)
    .await?;

    let rows_affected = query_result.rows_affected();
    if rows_affected > 0 {
        info!(
            last_block_polled,
            "Last block polled updated for {}/{} event types in {event_types:?}",
            rows_affected,
            event_types.len()
        );
    } else {
        debug!(
            last_block_polled,
            "Last block polled for {event_types:?} was not updated: {query_result:?}"
        );
    }

    Ok(())
}

pub async fn publish_context_id(db_pool: &Pool<Postgres>, context_id: U256) -> anyhow::Result<()> {
    info!("Publishing KMS context #{context_id} in DB...");
    let now = Utc::now();
    let query_result = sqlx::query!(
        "INSERT INTO kms_context(id, is_valid, created_at, updated_at) \
        VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        context_id.as_le_slice(),
        true,
        now,
        now,
    )
    .execute(db_pool)
    .await?;

    if query_result.rows_affected() == 1 {
        info!("KMS context #{context_id} was successfully published!");
    } else {
        debug!("KMS context #{context_id} was not published: {query_result:?}");
    }
    Ok(())
}
