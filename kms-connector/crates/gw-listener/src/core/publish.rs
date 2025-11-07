use crate::monitoring::metrics::EVENT_STORED_COUNTER;
use alloy::primitives::U256;
use anyhow::anyhow;
use connector_utils::{
    monitoring::otlp::PropagationContext,
    types::{
        GatewayEvent, GatewayEventKind,
        db::{EventType, ParamsTypeDb, SnsCiphertextMaterialDbItem},
    },
};
use fhevm_gateway_bindings::{
    decryption::Decryption::{PublicDecryptionRequest, UserDecryptionRequest},
    kms_generation::KMSGeneration::{
        CrsgenRequest, KeyReshareSameSet, KeygenRequest, PrepKeygenRequest,
    },
};
use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use tracing::{debug, info, warn};

#[tracing::instrument(skip_all)]
pub async fn publish_event(
    db_pool: &Pool<Postgres>,
    event: GatewayEvent,
    block_number: Option<u64>,
) -> anyhow::Result<()> {
    info!(block_number, "Storing {:?} in DB...", event.kind);

    let event_type = (&event.kind).into();
    let otlp_ctx = event.otlp_context;
    let query_result = match event.kind {
        GatewayEventKind::PublicDecryption(e) => {
            publish_public_decryption(db_pool, e, otlp_ctx).await
        }
        GatewayEventKind::UserDecryption(e) => publish_user_decryption(db_pool, e, otlp_ctx).await,
        GatewayEventKind::PrepKeygen(e) => publish_prep_keygen_request(db_pool, e, otlp_ctx).await,
        GatewayEventKind::Keygen(e) => publish_keygen_request(db_pool, e, otlp_ctx).await,
        GatewayEventKind::Crsgen(e) => publish_crsgen_request(db_pool, e, otlp_ctx).await,
        GatewayEventKind::PrssInit(id) => publish_prss_init(db_pool, id, otlp_ctx).await,
        GatewayEventKind::KeyReshareSameSet(e) => {
            publish_key_reshare_same_set(db_pool, e, otlp_ctx).await
        }
    }
    .map_err(|err| anyhow!("Failed to publish event: {err}"))?;

    if query_result.rows_affected() == 1 {
        info!("Event successfully stored in DB!");
        EVENT_STORED_COUNTER.inc();
    } else {
        warn!("Unexpected query result while publishing event: {query_result:?}");
    }

    update_last_block_polled(db_pool, event_type, block_number).await?;
    Ok(())
}

async fn publish_public_decryption(
    db_pool: &Pool<Postgres>,
    request: PublicDecryptionRequest,
    otlp_ctx: PropagationContext,
) -> anyhow::Result<PgQueryResult> {
    let sns_ciphertexts_db = request
        .snsCtMaterials
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();

    sqlx::query!(
            "INSERT INTO public_decryption_requests(decryption_id, sns_ct_materials, extra_data, otlp_context) \
            VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
            request.decryptionId.as_le_slice(),
            sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
            request.extraData.as_ref(),
            bc2wrap::serialize(&otlp_ctx)?,
        )
        .execute(db_pool)
        .await
        .map_err(anyhow::Error::from)
}

async fn publish_user_decryption(
    db_pool: &Pool<Postgres>,
    request: UserDecryptionRequest,
    otlp_ctx: PropagationContext,
) -> anyhow::Result<PgQueryResult> {
    let sns_ciphertexts_db = request
        .snsCtMaterials
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();

    sqlx::query!(
        "INSERT INTO user_decryption_requests(\
                decryption_id, sns_ct_materials, user_address, public_key, extra_data, otlp_context\
            ) \
            VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
        request.decryptionId.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
        request.userAddress.as_slice(),
        request.publicKey.as_ref(),
        request.extraData.as_ref(),
        bc2wrap::serialize(&otlp_ctx)?,
    )
    .execute(db_pool)
    .await
    .map_err(anyhow::Error::from)
}

async fn publish_prep_keygen_request(
    db_pool: &Pool<Postgres>,
    request: PrepKeygenRequest,
    otlp_ctx: PropagationContext,
) -> anyhow::Result<PgQueryResult> {
    let params_type: ParamsTypeDb = request.paramsType.try_into()?;
    sqlx::query!(
        "INSERT INTO prep_keygen_requests(prep_keygen_id, epoch_id, params_type, otlp_context) \
            VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        request.prepKeygenId.as_le_slice(),
        request.epochId.as_le_slice(),
        params_type as ParamsTypeDb,
        bc2wrap::serialize(&otlp_ctx)?,
    )
    .execute(db_pool)
    .await
    .map_err(anyhow::Error::from)
}

async fn publish_keygen_request(
    db_pool: &Pool<Postgres>,
    request: KeygenRequest,
    otlp_ctx: PropagationContext,
) -> anyhow::Result<PgQueryResult> {
    sqlx::query!(
        "INSERT INTO keygen_requests(prep_keygen_id, key_id, otlp_context) \
            VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        request.prepKeygenId.as_le_slice(),
        request.keyId.as_le_slice(),
        bc2wrap::serialize(&otlp_ctx)?,
    )
    .execute(db_pool)
    .await
    .map_err(anyhow::Error::from)
}

async fn publish_crsgen_request(
    db_pool: &Pool<Postgres>,
    request: CrsgenRequest,
    otlp_ctx: PropagationContext,
) -> anyhow::Result<PgQueryResult> {
    let params_type: ParamsTypeDb = request.paramsType.try_into()?;
    sqlx::query!(
        "INSERT INTO crsgen_requests(crs_id, max_bit_length, params_type, otlp_context) \
            VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        request.crsId.as_le_slice(),
        request.maxBitLength.as_le_slice(),
        params_type as ParamsTypeDb,
        bc2wrap::serialize(&otlp_ctx)?,
    )
    .execute(db_pool)
    .await
    .map_err(anyhow::Error::from)
}

async fn publish_prss_init(
    db_pool: &Pool<Postgres>,
    id: U256,
    otlp_ctx: PropagationContext,
) -> anyhow::Result<PgQueryResult> {
    sqlx::query!(
        "INSERT INTO prss_init(id, otlp_context) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        id.as_le_slice(),
        bc2wrap::serialize(&otlp_ctx)?,
    )
    .execute(db_pool)
    .await
    .map_err(anyhow::Error::from)
}

async fn publish_key_reshare_same_set(
    db_pool: &Pool<Postgres>,
    request: KeyReshareSameSet,
    otlp_ctx: PropagationContext,
) -> anyhow::Result<PgQueryResult> {
    let params_type: ParamsTypeDb = request.paramsType.try_into()?;
    sqlx::query!(
            "INSERT INTO key_reshare_same_set(prep_keygen_id, key_id, key_reshare_id, params_type, otlp_context) \
            VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
            request.prepKeygenId.as_le_slice(),
            request.keyId.as_le_slice(),
            request.keyReshareId.as_le_slice(),
            params_type as ParamsTypeDb,
            bc2wrap::serialize(&otlp_ctx)?,
        )
        .execute(db_pool)
        .await
        .map_err(anyhow::Error::from)
}

/// Updates the registered last block polled in DB.
#[tracing::instrument(skip_all)]
pub async fn update_last_block_polled(
    db_pool: &Pool<Postgres>,
    event_type: EventType,
    last_block_polled: Option<u64>,
) -> anyhow::Result<()> {
    info!(
        last_block_polled,
        "Updating last block polled in DB for {event_type}"
    );
    let query_result = sqlx::query!(
        "UPDATE last_block_polled SET block_number = $2 \
        WHERE event_type = $1 AND (block_number IS NULL OR block_number < $2)",
        event_type as EventType,
        last_block_polled.map(|n| n.to_le_bytes().to_vec()),
    )
    .execute(db_pool)
    .await?;

    if query_result.rows_affected() == 1 {
        info!(
            last_block_polled,
            "Last block polled for {event_type} was successfully updated!"
        );
    } else {
        debug!(
            last_block_polled,
            "Last block polled for {event_type} was not updated: {query_result:?}"
        );
    }

    Ok(())
}
