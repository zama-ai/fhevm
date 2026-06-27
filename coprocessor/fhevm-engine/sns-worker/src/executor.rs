use crate::aws_upload::check_is_ready;
use crate::keyset::fetch_latest_keyset;
use crate::metrics::SNS_LATENCY_OP_HISTOGRAM;
use crate::metrics::TASK_EXECUTE_FAILURE_COUNTER;
use crate::metrics::TASK_EXECUTE_SUCCESS_COUNTER;
use crate::squash_noise::SquashNoiseCiphertext;
use crate::BigCiphertext;
use crate::Ciphertext128Format;
use crate::HandleItem;
use crate::InternalEvents;
use crate::KeySet;
use crate::SchedulePolicy;
use crate::UploadJob;
use crate::{Config, ExecutionError};
use aws_sdk_s3::Client;
use fhevm_engine_common::branch::{
    is_branchless_producer, read_settled_height, select_producer_candidate, ProducerBlockHashed,
};
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::db_keys::DbKeyId;
use fhevm_engine_common::healthz_server::{HealthCheckService, HealthStatus, Version};
use fhevm_engine_common::pg_pool::PostgresPoolManager;
use fhevm_engine_common::pg_pool::ServiceError;
use fhevm_engine_common::telemetry;
use fhevm_engine_common::tfhe_ops::current_ciphertext_version;
use fhevm_engine_common::types::{get_ct_type, SupportedFheCiphertexts};
use fhevm_engine_common::utils::to_hex;
use opentelemetry::trace::{Status, TraceContextExt};
use rayon::prelude::*;
use sqlx::postgres::PgListener;
use sqlx::Pool;
use sqlx::{PgPool, Postgres, Transaction};
use std::fmt;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use tfhe::set_server_key;
use tfhe::ClientKey;
use tokio::select;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tokio::time::interval;
use tokio_util::sync::CancellationToken;
use tracing::error_span;
use tracing::warn;
use tracing::{debug, error, info, Instrument};
use tracing_opentelemetry::OpenTelemetrySpanExt;

const S3_HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, Copy)]
pub enum Order {
    Asc,
    Desc,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Order::Asc => write!(f, "ASC"),
            Order::Desc => write!(f, "DESC"),
        }
    }
}

#[derive(Clone, Debug)]
struct SnsTaskRow {
    handle: Vec<u8>,
    transaction_id: Option<Vec<u8>>,
    host_chain_id: i64,
    producer_block_hash: Vec<u8>,
    block_hash: Vec<u8>,
    block_number: Option<i64>,
}

#[derive(Clone, Debug)]
struct CiphertextBytesCandidate {
    producer_block_hash: Vec<u8>,
    ciphertext: Vec<u8>,
}

impl ProducerBlockHashed for CiphertextBytesCandidate {
    fn producer_block_hash(&self) -> &[u8] {
        &self.producer_block_hash
    }
}

pub struct SwitchNSquashService {
    pool: PgPool,
    conf: Config,
    // Timestamp of the last moment the service was active
    last_active_at: Arc<RwLock<SystemTime>>,
    s3_client: Arc<Client>,
    _token: CancellationToken,
    tx: Sender<UploadJob>,

    /// Channel to emit internal events, e.g. keys-loaded event
    events_tx: InternalEvents,
}
impl HealthCheckService for SwitchNSquashService {
    async fn health_check(&self) -> HealthStatus {
        let mut status = HealthStatus::default();
        status.set_db_connected(&self.pool).await;

        let mut is_s3_ready: bool = false;
        let mut is_s3_connected: bool = false;

        // Timeout for S3 readiness check as the S3 client has its internal retry logic
        match tokio::time::timeout(
            S3_HEALTH_CHECK_TIMEOUT,
            check_is_ready(&self.s3_client, &self.conf),
        )
        .await
        {
            Ok((is_ready, is_connected)) => {
                is_s3_connected = is_connected;
                is_s3_ready = is_ready;
            }
            Err(_) => {
                status.add_error_details(
                    "S3 readiness check timed out. Ensure S3 is reachable and configured correctly.".to_owned(),
                );
            }
        }

        status.set_custom_check("s3_buckets", is_s3_ready, true);
        status.set_custom_check("s3_connection", is_s3_connected, true);

        status
    }

    async fn is_alive(&self) -> bool {
        let last_active_at = *self.last_active_at.read().await;
        let threshold = self.conf.health_checks.liveness_threshold;

        (SystemTime::now()
            .duration_since(last_active_at)
            .map(|d| d.as_secs())
            .unwrap_or(u64::MAX) as u32)
            < threshold.as_secs() as u32
    }

    fn get_version(&self) -> Version {
        // Later, the unknowns will be initialized from build.rs
        Version {
            name: "sns-worker",
            version: "unknown",
            build: "unknown",
        }
    }
}

impl SwitchNSquashService {
    pub async fn create(
        pool_mngr: &PostgresPoolManager,
        conf: Config,
        tx: Sender<UploadJob>,
        token: CancellationToken,
        s3_client: Arc<Client>,
        events_tx: InternalEvents,
    ) -> Result<SwitchNSquashService, ExecutionError> {
        Ok(SwitchNSquashService {
            pool: pool_mngr.pool(),
            conf,
            last_active_at: Arc::new(RwLock::new(SystemTime::now())),
            _token: token,
            s3_client,
            tx,
            events_tx,
        })
    }

    pub async fn run(&self, pool_mngr: &PostgresPoolManager) -> Result<(), ServiceError> {
        let keys_cache: Arc<RwLock<lru::LruCache<DbKeyId, KeySet>>> = Arc::new(RwLock::new(
            lru::LruCache::new(NonZeroUsize::new(10).unwrap()),
        ));

        let op = |pool: Pool<Postgres>, token: CancellationToken| {
            let conf = self.conf.clone();
            let tx = self.tx.clone();
            let last_active_at = self.last_active_at.clone();
            let keys_cache = keys_cache.clone();
            let events_tx = self.events_tx.clone();

            async move {
                run_loop(
                    conf,
                    tx,
                    pool,
                    token,
                    last_active_at.clone(),
                    keys_cache,
                    events_tx,
                )
                .await
                .map_err(ServiceError::from)
            }
        };

        pool_mngr.blocking_with_db_retry(op, "sns").await
    }
}

#[tracing::instrument(name = "fetch_keyset", skip_all)]
async fn get_keyset(
    pool: PgPool,
    keys_cache: Arc<RwLock<lru::LruCache<DbKeyId, KeySet>>>,
) -> Result<Option<(DbKeyId, KeySet)>, ExecutionError> {
    fetch_latest_keyset(&keys_cache, &pool).await
}

/// Executes the worker logic for the SnS task.
pub(crate) async fn run_loop(
    conf: Config,
    tx: Sender<UploadJob>,
    pool: PgPool,
    token: CancellationToken,
    last_active_at: Arc<RwLock<SystemTime>>,
    keys_cache: Arc<RwLock<lru::LruCache<DbKeyId, KeySet>>>,
    events_tx: InternalEvents,
) -> Result<(), ExecutionError> {
    update_last_active(last_active_at.clone()).await;

    let mut listener = PgListener::connect_with(&pool).await?;
    info!("Connected to PostgresDB");

    listener
        .listen_all(conf.db.listen_channels.iter().map(|v| v.as_str()))
        .await?;

    let mut keys: Option<(DbKeyId, KeySet)> = None;
    let mut gc_ticker = interval(conf.db.cleanup_interval);
    let mut gc_timestamp = SystemTime::now();
    let mut polling_ticker = interval(Duration::from_secs(conf.db.polling_interval.into()));

    loop {
        // Continue looping until the service is cancelled or a critical error occurs
        update_last_active(last_active_at.clone()).await;

        let latest_keys = get_keyset(pool.clone(), keys_cache.clone()).await?;
        if let Some((key_id_gw, keyset)) = latest_keys {
            let key_changed = keys
                .as_ref()
                .map(|(current_key_id_gw, _)| current_key_id_gw != &key_id_gw)
                .unwrap_or(true);
            if key_changed {
                info!(key_id_gw = hex::encode(&key_id_gw), "Fetched keyset");
                // Notify that the keys are loaded
                if let Some(events_tx) = &events_tx {
                    let _ = events_tx.try_send("event_keys_loaded");
                }
            }
            keys = Some((key_id_gw, keyset));
        } else {
            warn!("No keys available, retrying in 5 seconds");
            tokio::time::sleep(Duration::from_secs(5)).await;
            if token.is_cancelled() {
                return Ok(());
            }
            continue;
        }

        // keys is guaranteed by the branch above; panic here if that invariant ever regresses.
        let (_, keys) = keys.as_ref().expect("keyset should be available");

        let (maybe_remaining, _tasks_processed) =
            fetch_and_execute_sns_tasks(&pool, &tx, keys, &conf, &token)
                .await
                .inspect(|(_, tasks_processed)| {
                    TASK_EXECUTE_SUCCESS_COUNTER.inc_by(*tasks_processed as u64);
                })
                .inspect_err(|_| {
                    TASK_EXECUTE_FAILURE_COUNTER.inc();
                })?;
        if maybe_remaining {
            if token.is_cancelled() {
                return Ok(());
            }

            info!("more tasks to process, continuing");
            if let Ok(elapsed) = gc_timestamp.elapsed() {
                if elapsed >= conf.db.cleanup_interval {
                    info!("gc interval, cleaning up");
                    gc_ticker.reset();
                    gc_timestamp = SystemTime::now();
                    garbage_collect(&pool, conf.db.gc_batch_limit).await?;
                }
            }

            continue;
        }

        select! {
            _ = token.cancelled() => return Ok(()),
            notification = listener.try_recv() => {
                match notification? {
                    Some(notification) => {
                        info!(notification = ?notification, "Received notification");
                    }
                    None => {
                        // sqlx already reconnected the LISTEN connection; keep going.
                        warn!("postgres LISTEN connection reset; reconnected");
                        continue;
                    }
                }
            },
            _ = polling_ticker.tick() => {
                debug!( "Polling timeout, rechecking for tasks");
            },
            // Garbage collecting
            _ = gc_ticker.tick() => {
                info!("gc tick, on_idle");
                gc_timestamp = SystemTime::now();
                garbage_collect(&pool, conf.db.gc_batch_limit).await?;
            }
        }
    }
}

/// Clean up the database by removing old ciphertexts128 already uploaded to S3.
/// Ideally, the table will be cleaned up by txn-sender if it's working properly
pub async fn garbage_collect(pool: &PgPool, limit: u32) -> Result<(), ExecutionError> {
    if limit == 0 {
        // GC disabled
        return Ok(());
    }

    let count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)::BIGINT AS "count!"
        FROM ciphertexts128_branch
        "#
    )
    .fetch_one(pool)
    .await?;

    if count <= limit as i64 {
        // Avoid unnecessary cleanup when there are not too many rows
        return Ok(());
    }

    info!(
        count,
        "Starting garbage collection of ciphertexts128_branch"
    );

    // Limit the number of rows to update in case of a large backlog due to catchup or burst.
    // Skip locked to prevent concurrent updates.
    let cleanup_span = tracing::info_span!("cleanup_ct128", rows_affected = tracing::field::Empty);
    let rows_affected: u64 = async {
        Ok::<u64, sqlx::Error>(
            sqlx::query!(
                r#"
                WITH uploaded_ct128 AS (
                    SELECT c.handle, c.producer_block_hash
                    FROM ciphertexts128_branch c
                    WHERE NOT EXISTS (
                        SELECT 1
                        FROM ciphertext_digest_branch d
                        WHERE d.handle = c.handle
                          AND d.producer_block_hash = c.producer_block_hash
                          AND d.ciphertext128 IS NULL
                    )
                    FOR UPDATE OF c SKIP LOCKED
                    LIMIT $1
                )
                DELETE FROM ciphertexts128_branch c
                USING uploaded_ct128 r
                WHERE c.handle = r.handle
                AND c.producer_block_hash = r.producer_block_hash
                "#,
                limit as i32,
            )
            .execute(pool)
            .await?
            .rows_affected(),
        )
    }
    .instrument(cleanup_span.clone())
    .await?;
    cleanup_span.record("rows_affected", rows_affected as i64);

    if rows_affected > 0 {
        info!(parent: &cleanup_span,
            rows_affected = rows_affected,
            "Cleaning up old ciphertexts128_branch"
        );
    }

    Ok(())
}

/// Fetch and process SnS tasks from the database.
/// Returns (maybe_remaining, number_of_tasks_processed) on success.
async fn fetch_and_execute_sns_tasks(
    pool: &PgPool,
    tx: &Sender<UploadJob>,
    keys: &KeySet,
    conf: &Config,
    token: &CancellationToken,
) -> Result<(bool, usize), ExecutionError> {
    let mut db_txn = match pool.begin().await {
        Ok(txn) => txn,
        Err(err) => {
            error!(error = %err, "Failed to begin transaction");
            return Err(err.into());
        }
    };

    let order = if conf.db.lifo {
        Order::Desc
    } else {
        Order::Asc
    };

    let trx = &mut db_txn;

    let mut maybe_remaining = false;
    let tasks_processed;
    if let Some(mut tasks) = query_sns_tasks(
        trx,
        conf.db.batch_limit,
        conf.db.branch_cutover_block,
        order,
        &keys.key_id_gw,
    )
    .await?
    {
        maybe_remaining = conf.db.batch_limit as usize == tasks.len();
        tasks_processed = tasks.len();

        let batch_exec_span = tracing::info_span!("batch_execution", count = tasks.len());

        batch_exec_span.in_scope(|| {
            process_tasks(
                &mut tasks,
                keys,
                tx,
                conf.enable_compression,
                conf.schedule_policy,
                token.clone(),
            )
        })?;

        update_computations_status(trx, &tasks)
            .instrument(batch_exec_span.clone())
            .await?;

        let batch_store_span = tracing::info_span!(
            parent: &batch_exec_span,
            "batch_store_ciphertext128"
        );
        let batch_store = async {
            update_ciphertext128(trx, &tasks).await?;
            notify_ciphertext128_ready(trx, &conf.db.notify_channel).await?;

            // Try to enqueue the tasks for upload in the DB
            // This is a best-effort attempt, as the upload worker might not be available
            enqueue_upload_tasks(trx, &tasks).await?;
            Ok::<(), ExecutionError>(())
        };
        if let Err(err) = batch_store.instrument(batch_store_span.clone()).await {
            batch_store_span
                .context()
                .span()
                .set_status(Status::error(err.to_string()));
            return Err(err);
        }
        drop(batch_store_span);

        db_txn.commit().await?;

        for task in tasks.iter() {
            if let Some(transaction_id) = &task.transaction_id {
                telemetry::try_end_l1_transaction(pool, transaction_id).await?;
            }
        }
    } else {
        tasks_processed = 0;
        db_txn.rollback().await?;
    }

    Ok((maybe_remaining, tasks_processed))
}

/// Queries the database for a fixed number of tasks.
#[tracing::instrument(name = "db_fetch_tasks", skip_all, fields(count = tracing::field::Empty))]
pub async fn query_sns_tasks(
    db_txn: &mut Transaction<'_, Postgres>,
    limit: u32,
    branch_cutover_block: i64,
    order: Order,
    key_id_gw: &DbKeyId,
) -> Result<Option<Vec<HandleItem>>, ExecutionError> {
    let build_task = |handle: Vec<u8>,
                      transaction_id: Option<Vec<u8>>,
                      host_chain_id_raw: i64,
                      producer_block_hash: Vec<u8>,
                      block_hash: Vec<u8>,
                      block_number: Option<i64>,
                      ciphertext: Vec<u8>|
     -> Result<HandleItem, ExecutionError> {
        let host_chain_id = ChainId::try_from(host_chain_id_raw)
            .map_err(|e| ExecutionError::ConversionError(e.into()))?;
        let task_span = tracing::info_span!(
            "task",
            txn_id = tracing::field::Empty,
            handle = tracing::field::Empty
        );
        telemetry::record_short_hex(&task_span, "handle", &handle);
        telemetry::record_short_hex_if_some(&task_span, "txn_id", transaction_id.as_deref());

        Ok(HandleItem {
            // TODO: During key rotation, ensure all coprocessors pin the same key_id_gw for a batch
            // (e.g., via gateway coordination) to keep ciphertext_digest consistent.
            key_id_gw: key_id_gw.clone(),
            host_chain_id,
            handle,
            producer_block_hash,
            block_hash,
            block_number,
            ct64_compressed: Arc::new(ciphertext),
            ct128: Arc::new(BigCiphertext::default()), // to be computed
            ct64_digest: None,
            ct128_digest: None,
            s3_format_version: None,
            span: task_span,
            transaction_id,
        })
    };
    let task_rows = match order {
        Order::Asc => sqlx::query!(
            r#"
            SELECT
                a.handle AS "handle!",
                a.transaction_id,
                a.host_chain_id AS "host_chain_id!",
                a.producer_block_hash AS "producer_block_hash!",
                a.block_hash AS "block_hash!",
                a.block_number
            FROM pbs_computations_branch a
            WHERE a.is_completed = FALSE
              AND (
                (a.producer_block_hash = ''::bytea AND a.block_hash = ''::bytea)
                OR a.block_number >= $2
              )
              AND EXISTS (
                  SELECT 1 FROM ciphertexts_branch c
                   WHERE c.handle = a.handle
                     AND c.producer_block_hash = a.producer_block_hash
                     AND c.ciphertext IS NOT NULL
                     AND c.ciphertext_version = $3
              )
            ORDER BY a.created_at ASC
            FOR UPDATE OF a SKIP LOCKED
            LIMIT $1
            "#,
            limit as i64,
            branch_cutover_block,
            current_ciphertext_version(),
        )
        .fetch_all(db_txn.as_mut())
        .await?
        .into_iter()
        .map(|row| SnsTaskRow {
            handle: row.handle,
            transaction_id: row.transaction_id,
            host_chain_id: row.host_chain_id,
            producer_block_hash: row.producer_block_hash,
            block_hash: row.block_hash,
            block_number: row.block_number,
        })
        .collect::<Vec<_>>(),
        Order::Desc => sqlx::query!(
            r#"
            SELECT
                a.handle AS "handle!",
                a.transaction_id,
                a.host_chain_id AS "host_chain_id!",
                a.producer_block_hash AS "producer_block_hash!",
                a.block_hash AS "block_hash!",
                a.block_number
            FROM pbs_computations_branch a
            WHERE a.is_completed = FALSE
              AND (
                (a.producer_block_hash = ''::bytea AND a.block_hash = ''::bytea)
                OR a.block_number >= $2
              )
              AND EXISTS (
                  SELECT 1 FROM ciphertexts_branch c
                   WHERE c.handle = a.handle
                     AND c.producer_block_hash = a.producer_block_hash
                     AND c.ciphertext IS NOT NULL
                     AND c.ciphertext_version = $3
              )
            ORDER BY a.created_at DESC
            FOR UPDATE OF a SKIP LOCKED
            LIMIT $1
            "#,
            limit as i64,
            branch_cutover_block,
            current_ciphertext_version(),
        )
        .fetch_all(db_txn.as_mut())
        .await?
        .into_iter()
        .map(|row| SnsTaskRow {
            handle: row.handle,
            transaction_id: row.transaction_id,
            host_chain_id: row.host_chain_id,
            producer_block_hash: row.producer_block_hash,
            block_hash: row.block_hash,
            block_number: row.block_number,
        })
        .collect::<Vec<_>>(),
    };

    if task_rows.is_empty() {
        info!(target: "worker", { count = 0, order = order.to_string() }, "Fetched SnS tasks");
        tracing::Span::current().record("count", 0);
        return Ok(None);
    }

    let handles = task_rows
        .iter()
        .map(|row| row.handle.clone())
        .collect::<Vec<_>>();
    let mut ciphertext_candidates: std::collections::HashMap<
        Vec<u8>,
        Vec<CiphertextBytesCandidate>,
    > = std::collections::HashMap::new();
    let rows = sqlx::query!(
        r#"
        SELECT handle, producer_block_hash, ciphertext
        FROM ciphertexts_branch
        WHERE handle = ANY($1::BYTEA[])
          AND ciphertext IS NOT NULL
          AND ciphertext_version = $2
        "#,
        &handles as _,
        current_ciphertext_version(),
    )
    .fetch_all(db_txn.as_mut())
    .await?;
    for row in rows {
        let handle = row.handle;
        ciphertext_candidates
            .entry(handle)
            .or_default()
            .push(CiphertextBytesCandidate {
                producer_block_hash: row.producer_block_hash,
                ciphertext: row.ciphertext,
            });
    }

    let tasks = task_rows
        .into_iter()
        .map(|row| {
            let candidates = ciphertext_candidates.get(&row.handle).ok_or_else(|| {
                ExecutionError::MissingCiphertext64(format!(
                    "missing ciphertext candidates for handle {}",
                    hex::encode(&row.handle)
                ))
            })?;
            let ciphertext = select_producer_candidate(candidates, &row.producer_block_hash)
                .ok_or_else(|| {
                    ExecutionError::MissingCiphertext64(format!(
                        "missing matching ciphertext for handle {} and producer {}",
                        hex::encode(&row.handle),
                        hex::encode(&row.producer_block_hash),
                    ))
                })?
                .ciphertext
                .clone();

            build_task(
                row.handle,
                row.transaction_id,
                row.host_chain_id,
                row.producer_block_hash,
                row.block_hash,
                row.block_number,
                ciphertext,
            )
        })
        .collect::<Result<Vec<_>, ExecutionError>>()?;

    info!(target: "worker", { count = tasks.len(), order = order.to_string() }, "Fetched SnS tasks");
    tracing::Span::current().record("count", tasks.len());

    Ok(Some(tasks))
}

async fn enqueue_upload_tasks(
    db_txn: &mut Transaction<'_, Postgres>,
    tasks: &[HandleItem],
) -> Result<(), ExecutionError> {
    for task in tasks.iter() {
        task.enqueue_upload_task(db_txn).await?;
    }
    Ok(())
}

/// Processes the tasks by decompressing and converting the ciphertexts.
///
/// This uses the `rayon` to parallelize the squash_noise_and_serialize.
///
/// The computed ciphertexts are sent to the upload worker via the provided channel.
fn process_tasks(
    batch: &mut [HandleItem],
    keys: &KeySet,
    tx: &Sender<UploadJob>,
    enable_compression: bool,
    policy: SchedulePolicy,
    token: CancellationToken,
) -> Result<(), ExecutionError> {
    set_server_key(keys.server_key.clone());

    match policy {
        SchedulePolicy::Sequential => {
            for task in batch.iter_mut() {
                compute_task(
                    task,
                    tx,
                    enable_compression,
                    token.clone(),
                    &keys.client_key,
                );
            }
        }
        SchedulePolicy::RayonParallel => {
            rayon::broadcast(|_| {
                tfhe::set_server_key(keys.server_key.clone());
            });

            batch.par_iter_mut().for_each(|task| {
                compute_task(
                    task,
                    tx,
                    enable_compression,
                    token.clone(),
                    &keys.client_key,
                );
            });
        }
    }

    Ok(())
}

fn compute_task(
    task: &mut HandleItem,
    tx: &Sender<UploadJob>,
    enable_compression: bool,
    token: CancellationToken,
    _client_key: &Option<ClientKey>,
) {
    let started_at = SystemTime::now();
    let thread_id = format!("{:?}", std::thread::current().id());
    // Cross-boundary: compute_task runs on a thread-pool worker;
    // restore the OTel context that was captured when the task was enqueued.
    let span = error_span!("compute", thread_id = %thread_id);
    span.set_parent(task.span.context());
    let _enter = span.enter();

    let handle = to_hex(&task.handle);

    // Check if the task is cancelled
    if token.is_cancelled() {
        warn!({ handle }, "Task processing cancelled");
        return;
    }

    let ct64_compressed = task.ct64_compressed.as_ref();
    if ct64_compressed.is_empty() {
        error!({ handle }, "Empty ciphertext64, skipping task");
        return; // Skip empty ciphertexts
    }

    let decompress_span = tracing::info_span!("decompress_ct64");
    let ct = match decompress_span.in_scope(|| decompress_ct(&task.handle, ct64_compressed)) {
        Ok(ct) => ct,
        Err(err) => {
            decompress_span
                .context()
                .span()
                .set_status(Status::error(err.to_string()));
            error!({ handle = handle, error = %err }, "Failed to decompress ct64");
            return;
        }
    };

    let ct_type = ct.type_name().to_owned();
    info!( { handle, ct_type }, "Converting ciphertext");

    let squash_span = tracing::info_span!(
        "squash_noise",
        ct_type = %ct_type
    );
    let _squash_enter = squash_span.enter();

    match ct.squash_noise_and_serialize(enable_compression) {
        Ok(bytes) => {
            info!(
                handle = handle,
                length = bytes.len(),
                compressed = enable_compression,
                "Ciphertext converted"
            );

            #[cfg(feature = "test_decrypt_128")]
            decrypt_big_ct(_client_key, &bytes, &ct, &task.handle, enable_compression);

            let format = if enable_compression {
                Ciphertext128Format::CompressedOnCpu
            } else {
                Ciphertext128Format::UncompressedOnCpu
            };

            task.ct128 = Arc::new(BigCiphertext::new(bytes, format));

            // Start uploading the ciphertexts as soon as the ct128 is computed
            //
            // The service must continue running the squashed noise algorithm,
            // regardless of the availability of the upload worker.
            if let Err(err) = tx
                .try_send(UploadJob::Normal(task.clone()))
                .map_err(|err| ExecutionError::InternalSendError(err.to_string()))
            {
                let send_task_span = tracing::error_span!("send_task");
                let _send_task_enter = send_task_span.enter();
                send_task_span
                    .context()
                    .span()
                    .set_status(Status::error(err.to_string()));
                // This could happen if either we are experiencing a burst of tasks
                // or the upload worker cannot recover the connection to AWS S3
                //
                // In this case, we should log the error and rely on the retry mechanism.
                //
                // There are three levels of task buffering:
                // 1. The spawned uploading tasks (size: conf.max_concurrent_uploads)
                // 2. The input channel of the upload worker (size: conf.max_concurrent_uploads * 10)
                // 3. The PostgresDB (size: unlimited)
                error!({ action = "review", error = %err }, "Failed to send task to upload worker");
            }

            let elapsed = started_at.elapsed().map(|d| d.as_secs_f64()).unwrap_or(0.0);
            if elapsed > 0.0 {
                SNS_LATENCY_OP_HISTOGRAM.observe(elapsed);
            }
        }
        Err(err) => {
            squash_span
                .context()
                .span()
                .set_status(Status::error(err.to_string()));
            error!({ handle = handle, error = %err }, "Failed to convert ct");
        }
    };
}

fn ensure_ct128_write_above_settled(
    task: &HandleItem,
    settled_height: i64,
) -> Result<(), ExecutionError> {
    if is_branchless_producer(&task.producer_block_hash) {
        return Ok(());
    }
    let Some(block_number) = task.block_number else {
        return Err(ExecutionError::DbError(sqlx::Error::Protocol(format!(
            "refusing branch ct128 write without block_number for handle {} producer_block_hash {}",
            hex::encode(&task.handle),
            hex::encode(&task.producer_block_hash),
        ))));
    };
    if block_number <= settled_height {
        return Err(ExecutionError::DbError(sqlx::Error::Protocol(
            format!(
                "refusing settled branch ct128 write for handle {} producer_block_hash {} block {} <= settled_height {}",
                hex::encode(&task.handle),
                hex::encode(&task.producer_block_hash),
                block_number,
                settled_height,
            ),
        )));
    }
    Ok(())
}

/// Updates the database with the computed large ciphertexts.
///
/// The ct128 is temporarily stored in PostgresDB to ensure reliability.
/// After the AWS uploader successfully uploads the ct128 to S3, the ct128 blob
/// is deleted from Postgres.
async fn update_ciphertext128(
    db_txn: &mut Transaction<'_, Postgres>,
    tasks: &[HandleItem],
) -> Result<(), ExecutionError> {
    let mut settled_heights = std::collections::HashMap::<i64, i64>::new();
    for task in tasks {
        if !task.ct128.is_empty() {
            let host_chain_id = task.host_chain_id.as_i64();
            let settled_height = if let Some(settled_height) = settled_heights.get(&host_chain_id) {
                *settled_height
            } else {
                let settled_height = read_settled_height(db_txn, host_chain_id).await?;
                settled_heights.insert(host_chain_id, settled_height);
                settled_height
            };
            ensure_ct128_write_above_settled(task, settled_height)?;

            let ciphertext128 = task.ct128.bytes();
            let persist_span = tracing::info_span!("ciphertexts128_insert");
            let res = sqlx::query!(
                r#"
                INSERT INTO ciphertexts128_branch (
                    handle,
                    producer_block_hash,
                    block_number,
                    ciphertext
                )
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (handle, producer_block_hash)
                DO UPDATE SET
                    ciphertext = EXCLUDED.ciphertext,
                    block_number = COALESCE(ciphertexts128_branch.block_number, EXCLUDED.block_number)
                "#,
                &task.handle,
                &task.producer_block_hash,
                task.block_number,
                &ciphertext128,
            )
            .execute(db_txn.as_mut())
            .instrument(persist_span.clone())
            .await;

            match res {
                Ok(val) => {
                    drop(persist_span);
                    info!(
                        handle = to_hex(&task.handle),
                        query_res = format!("{:?}", val),
                        size = ciphertext128.len(),
                        "Persisted ct128 successfully"
                    );
                }
                Err(err) => {
                    persist_span
                        .context()
                        .span()
                        .set_status(Status::error(err.to_string()));
                    drop(persist_span);
                    error!( handle = to_hex(&task.handle), error = %err, "Failed to persist ct128");
                    // Although this is a single error, we drop the entire batch to be on the safe side
                    // This will ensure we will not mark a task as completed falsely
                    return Err(err.into());
                }
            }
        } else {
            error!(handle = to_hex(&task.handle), "ct128 not computed");
        }
    }

    Ok(())
}

async fn update_computations_status(
    db_txn: &mut Transaction<'_, Postgres>,
    tasks: &[HandleItem],
) -> Result<(), ExecutionError> {
    for task in tasks {
        if !task.ct128.is_empty() {
            sqlx::query!(
                r#"
                UPDATE pbs_computations_branch
                SET is_completed = TRUE, completed_at = NOW()
                WHERE host_chain_id = $1
                  AND handle = $2
                  AND producer_block_hash = $3
                  AND block_hash = $4
                "#,
                task.host_chain_id.as_i64(),
                &task.handle,
                &task.producer_block_hash,
                &task.block_hash,
            )
            .execute(db_txn.as_mut())
            .await?;
        } else {
            error!( handle = ?task.handle, "Large ciphertext not computed for task");
        }
    }
    Ok(())
}

/// Notifies the database that large ciphertexts are ready.
async fn notify_ciphertext128_ready(
    db_txn: &mut Transaction<'_, Postgres>,
    db_channel: &str,
) -> Result<(), ExecutionError> {
    sqlx::query!("SELECT pg_notify($1, '')", db_channel)
        .execute(db_txn.as_mut())
        .await?;
    Ok(())
}

/// Decompresses a ciphertext based on its type.
fn decompress_ct(
    handle: &[u8],
    compressed_ct: &[u8],
) -> Result<SupportedFheCiphertexts, ExecutionError> {
    let ct_type = get_ct_type(handle)?;

    let result = SupportedFheCiphertexts::decompress_no_memcheck(ct_type, compressed_ct)?;
    Ok(result)
}
#[cfg(feature = "test_decrypt_128")]
/// Decrypts a squashed noise ciphertext and returns the decrypted value.
/// This function is used for testing purposes only.
fn decrypt_big_ct(
    client_key: &Option<ClientKey>,
    bytes: &[u8],
    ct: &SupportedFheCiphertexts,
    handle: &[u8],
    is_compressed: bool,
) {
    {
        if let Some(client_key) = &client_key {
            let pt = if is_compressed {
                ct.decrypt_squash_noise_compressed(client_key, bytes)
            } else {
                ct.decrypt_squash_noise(client_key, bytes)
            }
            .expect("Failed to decrypt");

            info!(plaintext = pt, handle = to_hex(handle), "Decrypted");
        }
    }
}

async fn update_last_active(last_active_at: Arc<RwLock<SystemTime>>) {
    let mut value = last_active_at.write().await;
    *value = SystemTime::now();
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use test_harness::instance::{setup_test_db, DBInstance, ImportMode};

    fn handle_item(producer_block_hash: Vec<u8>, block_number: Option<i64>) -> HandleItem {
        handle_item_with_ct128(producer_block_hash, block_number, BigCiphertext::default())
    }

    fn handle_item_with_ct128(
        producer_block_hash: Vec<u8>,
        block_number: Option<i64>,
        ct128: BigCiphertext,
    ) -> HandleItem {
        HandleItem {
            host_chain_id: ChainId::try_from(12345_i64).unwrap(),
            key_id_gw: vec![0x11; 32],
            handle: vec![0x42; 32],
            producer_block_hash,
            block_hash: vec![0x24; 32],
            block_number,
            ct64_compressed: Arc::new(b"ct64".to_vec()),
            ct128: Arc::new(ct128),
            ct64_digest: None,
            ct128_digest: None,
            s3_format_version: None,
            span: tracing::Span::none(),
            transaction_id: None,
        }
    }

    #[test]
    fn write_guard_rejects_branch_ct128_at_or_below_settlement() {
        let err = ensure_ct128_write_above_settled(&handle_item(vec![0xAA; 32], Some(10)), 10)
            .expect_err("settled branch ct128 write should be rejected");

        assert!(err
            .to_string()
            .contains("refusing settled branch ct128 write"));
    }

    #[test]
    fn write_guard_rejects_branch_ct128_without_block_number() {
        let err = ensure_ct128_write_above_settled(&handle_item(vec![0xAA; 32], None), 10)
            .expect_err("branch ct128 write without block number should be rejected");

        assert!(err
            .to_string()
            .contains("refusing branch ct128 write without block_number"));
    }

    #[test]
    fn write_guard_allows_branchless_and_future_branch_ct128() {
        ensure_ct128_write_above_settled(&handle_item(vec![], None), 10)
            .expect("branchless rows are outside settlement");
        ensure_ct128_write_above_settled(&handle_item(vec![0xAA; 32], Some(11)), 10)
            .expect("future branch row should be accepted");
    }

    async fn setup_pool() -> (DBInstance, sqlx::PgPool) {
        let test_instance = setup_test_db(ImportMode::None)
            .await
            .expect("valid db instance");

        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(3)
            .connect(test_instance.db_url())
            .await
            .expect("connect test db");

        (test_instance, pool)
    }

    async fn set_settled_height(pool: &sqlx::PgPool, settled_height: i64) {
        sqlx::query(
            "INSERT INTO coprocessor_settlement (chain_id, settled_height)
             VALUES ($1, $2)
             ON CONFLICT (chain_id)
             DO UPDATE SET settled_height = EXCLUDED.settled_height",
        )
        .bind(12345_i64)
        .bind(settled_height)
        .execute(pool)
        .await
        .expect("set settled height");
    }

    async fn count_ct128_rows(pool: &sqlx::PgPool) -> i64 {
        sqlx::query_scalar("SELECT COUNT(*) FROM ciphertexts128_branch WHERE handle = $1")
            .bind(vec![0x42_u8; 32])
            .fetch_one(pool)
            .await
            .expect("count ciphertexts128_branch rows")
    }

    fn computed_handle_item(producer_block_hash: Vec<u8>, block_number: Option<i64>) -> HandleItem {
        handle_item_with_ct128(
            producer_block_hash,
            block_number,
            BigCiphertext::new(vec![0xCA, 0xFE], Ciphertext128Format::CompressedOnCpu),
        )
    }

    #[tokio::test]
    #[serial(db)]
    async fn update_ciphertext128_rejects_settled_branch_write_path() {
        let (_test_instance, pool) = setup_pool().await;
        set_settled_height(&pool, 10).await;
        let task = computed_handle_item(vec![0xAA; 32], Some(10));

        let mut tx = pool.begin().await.expect("begin transaction");
        let err = update_ciphertext128(&mut tx, &[task])
            .await
            .expect_err("settled branch ct128 write should be rejected");
        tx.rollback().await.expect("rollback transaction");

        assert!(err
            .to_string()
            .contains("refusing settled branch ct128 write"));
        assert_eq!(count_ct128_rows(&pool).await, 0);
    }

    #[tokio::test]
    #[serial(db)]
    async fn update_ciphertext128_allows_future_branch_write_path() {
        let (_test_instance, pool) = setup_pool().await;
        set_settled_height(&pool, 10).await;
        let task = computed_handle_item(vec![0xAA; 32], Some(11));

        let mut tx = pool.begin().await.expect("begin transaction");
        update_ciphertext128(&mut tx, &[task])
            .await
            .expect("future branch ct128 write should be accepted");
        tx.commit().await.expect("commit transaction");

        assert_eq!(count_ct128_rows(&pool).await, 1);
    }
}
