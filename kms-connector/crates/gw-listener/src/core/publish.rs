use alloy::{
    primitives::{FixedBytes, U256},
    sol_types::SolValue,
};
use anyhow::anyhow;
use connector_utils::{
    monitoring::otlp::PropagationContext,
    types::{
        ProtocolEvent, ProtocolEventKind,
        db::{ParamsTypeDb, SnsCiphertextMaterialDbItem},
    },
};
use fhevm_gateway_bindings::decryption::Decryption::{
    PublicDecryptionRequest, UserDecryptionRequest_0 as UserDecryptionRequest,
    UserDecryptionRequest_1 as UserDecryptionRequestV2,
};
use fhevm_host_bindings::{
    kms_generation::KMSGeneration::{CrsgenRequest, KeygenRequest, PrepKeygenRequest},
    protocol_config::ProtocolConfig::{NewKmsContext, NewKmsEpoch},
};
use sqlx::{
    PgExecutor, Pool, Postgres,
    postgres::PgQueryResult,
    types::chrono::{DateTime, Utc},
};
use tracing::{debug, info, warn};

/// Chain identifier used as the primary key in `last_block_polled_by_chain`.
#[derive(Debug, Clone, Copy)]
pub enum ChainName {
    Ethereum,
    Gateway,
}

impl ChainName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ethereum => "ethereum",
            Self::Gateway => "gateway",
        }
    }
}

/// Inserts all events and updates the last block polled in a single transaction.
/// On failure, the transaction is rolled back automatically.
#[tracing::instrument(skip_all)]
pub async fn publish_batch(
    db_pool: &Pool<Postgres>,
    events: Vec<ProtocolEvent>,
    chain: ChainName,
    block_number: u64,
) -> anyhow::Result<()> {
    let mut tx = db_pool.begin().await?;
    for event in events {
        publish_event_inner(&mut *tx, event).await?;
    }
    update_last_block_polled(&mut *tx, chain, Some(block_number)).await?;
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
        ProtocolEventKind::UserDecryptionV2(e) => {
            publish_user_decryption_v2(executor, e, tx_hash, created_at, otlp_ctx).await
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
        ProtocolEventKind::NewKmsContext(e) => {
            publish_new_kms_context(executor, e, tx_hash, created_at, otlp_ctx).await
        }
        ProtocolEventKind::NewKmsEpoch(e) => {
            publish_new_kms_epoch(executor, e, tx_hash, created_at, otlp_ctx).await
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
        "INSERT INTO public_decryption_requests(
            decryption_id, sns_ct_materials, extra_data, tx_hash, created_at, otlp_context
        )
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

    // RFC016-specific columns (`handle_owner_addresses`, `handle_contract_addresses`,
    // `allowed_contracts`, `start_timestamp`, `duration_seconds`, `signature`) are left unset —
    // they default to NULL for legacy rows, which is what the reader uses to identify the variant.
    sqlx::query!(
        "INSERT INTO user_decryption_requests(
            decryption_id, sns_ct_materials, user_address, public_key, extra_data, tx_hash,
            created_at, otlp_context
        )
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

async fn publish_user_decryption_v2<'e>(
    executor: impl PgExecutor<'e>,
    request: UserDecryptionRequestV2,
    tx_hash: Option<FixedBytes<32>>,
    created_at: DateTime<Utc>,
    otlp_ctx: PropagationContext,
) -> anyhow::Result<PgQueryResult> {
    let sns_ciphertexts_db = request
        .snsCtMaterials
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();

    let handle_owner_addresses: Vec<Vec<u8>> = request
        .handles
        .iter()
        .map(|h| h.ownerAddress.to_vec())
        .collect();
    let handle_contract_addresses: Vec<Vec<u8>> = request
        .handles
        .iter()
        .map(|h| h.contractAddress.to_vec())
        .collect();
    let payload = &request.payload;
    let allowed_contracts: Vec<Vec<u8>> = payload
        .allowedContracts
        .iter()
        .map(|a| a.to_vec())
        .collect();

    // `startTimestamp` and `durationSeconds` are `uint256` on-chain but Unix-epoch seconds in
    // practice, so they fit easily in `BIGINT`. A Gateway emitting values past i64::MAX would be
    // broken; we surface that as an error rather than silently truncating.
    let start_timestamp: i64 = payload
        .requestValidity
        .startTimestamp
        .try_into()
        .map_err(|_| anyhow!("RFC016 startTimestamp does not fit in i64"))?;
    let duration_seconds: i64 = payload
        .requestValidity
        .durationSeconds
        .try_into()
        .map_err(|_| anyhow!("RFC016 durationSeconds does not fit in i64"))?;

    sqlx::query!(
        "INSERT INTO user_decryption_requests(
            decryption_id, sns_ct_materials, user_address, public_key, extra_data, tx_hash,
            created_at, otlp_context, handle_owner_addresses, handle_contract_addresses,
            allowed_contracts, start_timestamp, duration_seconds, signature
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        ON CONFLICT DO NOTHING",
        request.decryptionId.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
        payload.userAddress.as_slice(),
        payload.publicKey.as_ref(),
        payload.extraData.as_ref(),
        tx_hash.map(|h| h.to_vec()),
        created_at,
        bc2wrap::serialize(&otlp_ctx)?,
        &handle_owner_addresses,
        &handle_contract_addresses,
        &allowed_contracts,
        start_timestamp,
        duration_seconds,
        payload.signature.as_ref(),
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
        "INSERT INTO prep_keygen_requests(
            prep_keygen_id, params_type, extra_data, tx_hash, created_at, otlp_context
        )
        VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
        request.prepKeygenId.as_le_slice(),
        params_type as ParamsTypeDb,
        request.extraData.as_ref(),
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
        "INSERT INTO keygen_requests(prep_keygen_id, key_id, extra_data, tx_hash, created_at, otlp_context)
            VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
        request.prepKeygenId.as_le_slice(),
        request.keyId.as_le_slice(),
        request.extraData.as_ref(),
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
        "INSERT INTO crsgen_requests(
            crs_id, max_bit_length, params_type, extra_data, tx_hash, created_at, otlp_context
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT DO NOTHING",
        request.crsId.as_le_slice(),
        request.maxBitLength.as_le_slice(),
        params_type as ParamsTypeDb,
        request.extraData.as_ref(),
        tx_hash.map(|h| h.to_vec()),
        created_at,
        bc2wrap::serialize(&otlp_ctx)?,
    )
    .execute(executor)
    .await
    .map_err(anyhow::Error::from)
}

/// Updates the registered last block polled in DB for the given chain.
#[tracing::instrument(skip_all)]
pub async fn update_last_block_polled<'e>(
    executor: impl PgExecutor<'e>,
    chain: ChainName,
    last_block_polled: Option<u64>,
) -> anyhow::Result<()> {
    let chain_name = chain.as_str();
    info!(
        last_block_polled,
        "Updating last block polled in DB for chain {chain_name}"
    );
    let query_result = sqlx::query!(
        "UPDATE last_block_polled_by_chain SET block_number = $2, updated_at = $3
        WHERE chain_name = $1 AND (block_number IS NULL OR block_number < $2)",
        chain_name,
        last_block_polled.map(|n| n as i64),
        Utc::now(),
    )
    .execute(executor)
    .await?;

    if query_result.rows_affected() > 0 {
        info!(
            last_block_polled,
            "Last block polled updated for chain {chain_name}"
        );
    } else {
        debug!(
            last_block_polled,
            "Last block polled for chain {chain_name} was not updated: {query_result:?}"
        );
    }

    Ok(())
}

/// Persists the `(context_id, epoch_id)` pair fetched at startup via
/// `ProtocolConfig::getActiveKmsContextAndEpoch()`.
pub async fn publish_context_and_epoch(
    db_pool: &Pool<Postgres>,
    context_id: U256,
    epoch_id: U256,
) -> anyhow::Result<()> {
    info!("Publishing KMS context #{context_id} (epoch #{epoch_id}) in DB...");
    let now = Utc::now();
    let query_result = sqlx::query!(
        "INSERT INTO kms_context(id, epoch_id, is_valid, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
        context_id.as_le_slice(),
        epoch_id.as_le_slice(),
        true,
        now,
        now,
    )
    .execute(db_pool)
    .await?;

    if query_result.rows_affected() == 1 {
        info!("KMS context #{context_id} (epoch #{epoch_id}) was successfully published!");
    } else {
        debug!("KMS context #{context_id} (epoch #{epoch_id}) was not published: {query_result:?}");
    }
    Ok(())
}

async fn publish_new_kms_context<'e>(
    executor: impl PgExecutor<'e>,
    event: NewKmsContext,
    tx_hash: Option<FixedBytes<32>>,
    created_at: DateTime<Utc>,
    otlp_ctx: PropagationContext,
) -> anyhow::Result<PgQueryResult> {
    // Nested arrays + the thresholds tuple are ABI-encoded to keep the schema flat. Decoding is
    // delegated to consumers; the on-chain `getKmsContextAnchor()` hash check in the follow-up
    // issue will validate this payload against the canonical encoding.
    let kms_node_params = event.kmsNodeParams.abi_encode();
    let thresholds = event.thresholds.abi_encode();
    let pcr_values = event.pcrValues.abi_encode();

    sqlx::query!(
        "INSERT INTO new_kms_context(
            context_id, previous_context_id, kms_node_params, thresholds, software_version,
            pcr_values, tx_hash, created_at, otlp_context
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) ON CONFLICT DO NOTHING",
        event.contextId.as_le_slice(),
        event.previousContextId.as_le_slice(),
        kms_node_params,
        thresholds,
        event.softwareVersion,
        pcr_values,
        tx_hash.map(|h| h.to_vec()),
        created_at,
        bc2wrap::serialize(&otlp_ctx)?,
    )
    .execute(executor)
    .await
    .map_err(anyhow::Error::from)
}

async fn publish_new_kms_epoch<'e>(
    executor: impl PgExecutor<'e>,
    event: NewKmsEpoch,
    tx_hash: Option<FixedBytes<32>>,
    created_at: DateTime<Utc>,
    otlp_ctx: PropagationContext,
) -> anyhow::Result<PgQueryResult> {
    let keys = event.keys.abi_encode();
    let crs_list = event.crsList.abi_encode();

    sqlx::query!(
        "INSERT INTO new_kms_epoch(
            context_id, previous_context_id, epoch_id, previous_epoch_id, keys, crs_list,
            tx_hash, created_at, otlp_context
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) ON CONFLICT DO NOTHING",
        event.kmsContextId.as_le_slice(),
        event.previousContextId.as_le_slice(),
        event.epochId.as_le_slice(),
        event.previousEpochId.as_le_slice(),
        keys,
        crs_list,
        tx_hash.map(|h| h.to_vec()),
        created_at,
        bc2wrap::serialize(&otlp_ctx)?,
    )
    .execute(executor)
    .await
    .map_err(anyhow::Error::from)
}

/// Action derived from a decoded Ethereum log.
#[derive(Clone, Debug, PartialEq)]
pub enum EthereumEventAction {
    /// Store the event in the DB, for the kms-worker to forward it to the KMS Core.
    // Boxed to keep the enum small: `ProtocolEvent` is an order of magnitude bigger than
    // the other variant (`clippy::large_enum_variant`).
    StoreEvent(Box<ProtocolEvent>),
    /// Mark a destroyed KMS context as invalid in the `kms_context` table; this implies that
    /// all its associated epoch IDs also become invalid.
    ///
    /// Context destruction is the only invalidation that can ever happen: once active, an
    /// epoch stays active on-chain until its context is destroyed (RFC-005 leaves epoch
    /// expiration/cleanup out of scope).
    InvalidateContext(U256),
}

/// Applies all the Ethereum log actions and updates the last block polled in a single
/// transaction. On failure, the transaction is rolled back automatically.
#[tracing::instrument(skip_all)]
pub async fn publish_ethereum_batch(
    db_pool: &Pool<Postgres>,
    actions: Vec<EthereumEventAction>,
    block_number: u64,
) -> anyhow::Result<()> {
    let mut tx = db_pool.begin().await?;
    for action in actions {
        match action {
            EthereumEventAction::StoreEvent(event) => publish_event_inner(&mut *tx, *event).await?,
            EthereumEventAction::InvalidateContext(context_id) => {
                invalidate_kms_context(&mut *tx, context_id).await?
            }
        }
    }
    update_last_block_polled(&mut *tx, ChainName::Ethereum, Some(block_number)).await?;
    tx.commit().await?;
    Ok(())
}

/// Marks a destroyed KMS context as invalid; this implies that all its associated epoch IDs
/// also become invalid.
async fn invalidate_kms_context<'e>(
    executor: impl PgExecutor<'e>,
    context_id: U256,
) -> anyhow::Result<()> {
    let query_result = sqlx::query!(
        "UPDATE kms_context SET is_valid = FALSE, updated_at = $2 WHERE id = $1",
        context_id.as_le_slice(),
        Utc::now(),
    )
    .execute(executor)
    .await?;

    info!(
        "KMS context #{context_id} destroyed: {} epoch(s) invalidated in DB",
        query_result.rows_affected()
    );
    Ok(())
}
