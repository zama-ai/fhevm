use crate::aws_upload::check_is_ready;
use crate::keyset::fetch_keyset;
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
use fhevm_engine_common::healthz_server::{HealthCheckService, HealthStatus, Version};
use fhevm_engine_common::pg_pool::PostgresPoolManager;
use fhevm_engine_common::pg_pool::ServiceError;
use fhevm_engine_common::telemetry;
use fhevm_engine_common::types::{get_ct_type, SupportedFheCiphertexts};
use fhevm_engine_common::utils::to_hex;
use opentelemetry::trace::{Status, TraceContextExt};
use rayon::prelude::*;
use sqlx::postgres::PgListener;
use sqlx::Pool;
use sqlx::{PgPool, Postgres, Row, Transaction};
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

fn set_span_error(span: &tracing::Span, err: &impl std::fmt::Display) {
    span.context()
        .span()
        .set_status(Status::error(err.to_string()));
}

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

    pub async fn run(&self, pool_mngr: &PostgresPoolManager) {
        let keys_cache: Arc<RwLock<lru::LruCache<String, KeySet>>> = Arc::new(RwLock::new(
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

        let _ = pool_mngr.blocking_with_db_retry(op, "sns").await;
    }
}

async fn get_keyset(
    pool: PgPool,
    keys_cache: Arc<RwLock<lru::LruCache<String, KeySet>>>,
    tenant_api_key: &String,
) -> Result<Option<KeySet>, ExecutionError> {
    let _t = telemetry::tracer("fetch_keyset", &None);
    {
        let mut cache = keys_cache.write().await;
        if let Some(keys) = cache.get(tenant_api_key) {
            info!(tenant_api_key = tenant_api_key, "Keyset found in cache");
            return Ok(Some(keys.clone()));
        }
    }
    let keys: Option<KeySet> = fetch_keyset(&keys_cache, &pool, tenant_api_key).await?;
    Ok(keys)
}

/// Executes the worker logic for the SnS task.
pub(crate) async fn run_loop(
    conf: Config,
    tx: Sender<UploadJob>,
    pool: PgPool,
    token: CancellationToken,
    last_active_at: Arc<RwLock<SystemTime>>,
    keys_cache: Arc<RwLock<lru::LruCache<String, KeySet>>>,
    events_tx: InternalEvents,
) -> Result<(), ExecutionError> {
    update_last_active(last_active_at.clone()).await;

    let tenant_api_key = &conf.tenant_api_key;
    let mut listener = PgListener::connect_with(&pool).await?;
    info!("Connected to PostgresDB");

    listener
        .listen_all(conf.db.listen_channels.iter().map(|v| v.as_str()))
        .await?;

    let mut keys = None;
    let mut gc_ticker = interval(conf.db.cleanup_interval);
    let mut gc_timestamp = SystemTime::now();
    let mut polling_ticker = interval(Duration::from_secs(conf.db.polling_interval.into()));

    loop {
        // Continue looping until the service is cancelled or a critical error occurs
        update_last_active(last_active_at.clone()).await;

        let Some(keys) = keys.as_ref() else {
            keys = get_keyset(pool.clone(), keys_cache.clone(), tenant_api_key).await?;
            if keys.is_some() {
                info!(tenant_api_key = tenant_api_key, "Fetched keyset");
                // Notify that the keys are loaded
                if let Some(events_tx) = &events_tx {
                    let _ = events_tx.try_send("event_keys_loaded");
                }
            } else {
                warn!(
                    tenant_api_key = tenant_api_key,
                    "No keys available, retrying in 5 seconds"
                );
                tokio::time::sleep(Duration::from_secs(5)).await;
            }

            if token.is_cancelled() {
                return Ok(());
            }
            continue;
        };

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
            n = listener.try_recv() => {
                info!( notification = ?n, "Received notification");
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

    let count: Option<i64> = sqlx::query_scalar!(
        "
        SELECT COUNT(*)::BIGINT
        FROM ciphertexts128
        "
    )
    .fetch_one(pool)
    .await?;

    let count = count.unwrap_or(0);
    if count <= limit as i64 {
        // Avoid unnecessary cleanup when there are not too many rows
        return Ok(());
    }

    info!(count, "Starting garbage collection of ciphertexts128");

    // Limit the number of rows to update in case of a large backlog due to catchup or burst
    // Skip Locked to prevent concurrent updates
    let start = SystemTime::now();
    let rows_affected: u64 = sqlx::query!(
        "
        WITH uploaded_ct128 AS (
            SELECT c.tenant_id, c.handle
            FROM ciphertexts128 c
            JOIN ciphertext_digest d
            ON d.tenant_id = c.tenant_id
            AND d.handle = c.handle
            WHERE d.ciphertext128 IS NOT NULL
            FOR UPDATE OF c SKIP LOCKED
            LIMIT $1
        )

        DELETE FROM ciphertexts128 c
        USING uploaded_ct128 r
        WHERE c.tenant_id = r.tenant_id
        AND c.handle = r.handle;
        ",
        limit as i32
    )
    .execute(pool)
    .await?
    .rows_affected();

    if rows_affected > 0 {
        let _s = telemetry::tracer_with_start_time("cleanup_ct128", start);
        info!(
            rows_affected = rows_affected,
            "Cleaning up old ciphertexts128"
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
    if let Some(mut tasks) = query_sns_tasks(trx, conf.db.batch_limit, order).await? {
        maybe_remaining = conf.db.batch_limit as usize == tasks.len();
        tasks_processed = tasks.len();

        let t = telemetry::tracer("batch_execution", &None);
        t.set_attribute("count", tasks.len().to_string());

        process_tasks(
            &mut tasks,
            keys,
            tx,
            conf.enable_compression,
            conf.schedule_policy,
            token.clone(),
        )?;

        update_computations_status(trx, &tasks).await?;

        let batch_store_span = tracing::info_span!(
            "batch_store_ciphertext128",
            operation = "batch_store_ciphertext128"
        );
        batch_store_span.set_parent(t.context().clone());
        let batch_store = async {
            update_ciphertext128(trx, &tasks).await?;
            notify_ciphertext128_ready(trx, &conf.db.notify_channel).await?;

            // Try to enqueue the tasks for upload in the DB
            // This is a best-effort attempt, as the upload worker might not be available
            enqueue_upload_tasks(trx, &tasks).await?;
            Ok::<(), ExecutionError>(())
        };
        if let Err(err) = batch_store.instrument(batch_store_span.clone()).await {
            set_span_error(&batch_store_span, &err);
            return Err(err);
        }

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
pub async fn query_sns_tasks(
    db_txn: &mut Transaction<'_, Postgres>,
    limit: u32,
    order: Order,
) -> Result<Option<Vec<HandleItem>>, ExecutionError> {
    let start_time = SystemTime::now();

    let query = format!(
        "
        SELECT a.*, c.ciphertext
        FROM pbs_computations a
        JOIN ciphertexts c 
        ON a.handle = c.handle
        WHERE c.ciphertext IS NOT NULL
        AND a.is_completed = FALSE
        ORDER BY a.created_at {}
        FOR UPDATE SKIP LOCKED
        LIMIT $1;
        ",
        order
    );

    let records = sqlx::query(&query)
        .bind(limit as i64)
        .fetch_all(db_txn.as_mut())
        .await?;

    info!(target: "worker", { count = records.len(), order = order.to_string() }, "Fetched SnS tasks");

    if records.is_empty() {
        return Ok(None);
    }

    let t = telemetry::tracer_with_start_time("db_fetch_tasks", start_time);
    t.set_attribute("count", records.len().to_string());
    t.end();

    // Convert the records into HandleItem structs
    let tasks = records
        .into_iter()
        .map(|record| {
            let tenant_id: i32 = record.try_get("tenant_id")?;
            let handle: Vec<u8> = record.try_get("handle")?;
            let ciphertext: Vec<u8> = record.try_get("ciphertext")?;
            let transaction_id: Option<Vec<u8>> = record.try_get("transaction_id")?;

            Ok(HandleItem {
                tenant_id,
                handle: handle.clone(),
                ct64_compressed: Arc::new(ciphertext),
                ct128: Arc::new(BigCiphertext::default()), // to be computed
                otel: telemetry::tracer_with_handle("task", handle, &transaction_id),
                transaction_id,
            })
        })
        .collect::<Result<Vec<_>, ExecutionError>>()?;

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
    let span = error_span!("compute", thread_id = %thread_id);
    span.set_parent(task.otel.context().clone());
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

    let decompress_span = tracing::info_span!("decompress_ct64", operation = "decompress_ct64");
    decompress_span.set_parent(task.otel.context().clone());
    let ct = decompress_span.in_scope(|| decompress_ct(&task.handle, ct64_compressed).unwrap()); // TODO handle error properly

    let ct_type = ct.type_name().to_owned();
    info!( { handle, ct_type }, "Converting ciphertext");

    let squash_span = tracing::info_span!(
        "squash_noise",
        ct_type = %ct_type,
        operation = "squash_noise"
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
                let send_task_span = tracing::error_span!("send_task", operation = "send_task");
                send_task_span.set_parent(task.otel.context().clone());
                set_span_error(&send_task_span, &err);
            }

            let elapsed = started_at.elapsed().map(|d| d.as_secs_f64()).unwrap_or(0.0);
            if elapsed > 0.0 {
                SNS_LATENCY_OP_HISTOGRAM.observe(elapsed);
            }
        }
        Err(err) => {
            error!({ handle = handle, error = %err }, "Failed to convert ct");
        }
    };
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
    for task in tasks {
        if !task.ct128.is_empty() {
            let ciphertext128 = task.ct128.bytes();
            let persist_span =
                tracing::info_span!("ciphertexts128_insert", operation = "ciphertexts128_insert");
            persist_span.set_parent(task.otel.context().clone());
            let res = sqlx::query!(
                "
                INSERT INTO ciphertexts128 (
                        tenant_id,
                        handle,
                        ciphertext
                )
                VALUES ($1, $2, $3)",
                task.tenant_id,
                task.handle,
                ciphertext128,
            )
            .execute(db_txn.as_mut())
            .instrument(persist_span.clone())
            .await;

            match res {
                Ok(val) => {
                    info!(
                        handle = to_hex(&task.handle),
                        query_res = format!("{:?}", val),
                        size = ciphertext128.len(),
                        "Persisted ct128 successfully"
                    );
                }
                Err(err) => {
                    error!( handle = to_hex(&task.handle), error = %err, "Failed to persist ct128");
                    set_span_error(&persist_span, &err);
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
                "
                UPDATE pbs_computations
                SET is_completed = TRUE, completed_at = NOW()
                WHERE handle = $1;",
                task.handle
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
    sqlx::query("SELECT pg_notify($1, '')")
        .bind(db_channel)
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
