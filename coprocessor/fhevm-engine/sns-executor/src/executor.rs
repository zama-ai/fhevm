use crate::aws_upload::check_is_ready;
use crate::keyset::fetch_keyset;
use crate::squash_noise::SquashNoiseCiphertext;
use crate::BigCiphertext;
use crate::Ciphertext128Format;
use crate::HandleItem;
use crate::KeySet;
use crate::UploadJob;
use crate::{Config, DBConfig, ExecutionError};
use aws_sdk_s3::Client;
use fhevm_engine_common::healthz_server::{HealthCheckService, HealthStatus, Version};
use fhevm_engine_common::telemetry;
use fhevm_engine_common::types::{get_ct_type, SupportedFheCiphertexts};
use fhevm_engine_common::utils::compact_hex;
use sqlx::postgres::PgListener;
use sqlx::{PgPool, Postgres, Row, Transaction};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use tfhe::set_server_key;
use tokio::select;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tokio::time::interval;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

const RETRY_DB_CONN_INTERVAL: Duration = Duration::from_secs(5);
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

pub struct SwitchNSquashService {
    pool: PgPool,
    conf: Config,
    // Timestamp of the last moment the service was active
    last_active_at: Arc<RwLock<SystemTime>>,
    s3_client: Arc<Client>,
    token: CancellationToken,
    tx: Sender<UploadJob>,
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

        status.set_custom_check("s3_buckets", is_s3_ready);
        status.set_custom_check("s3_connection", is_s3_connected);

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
        conf: Config,
        tx: Sender<UploadJob>,
        token: CancellationToken,
        s3_client: Arc<Client>,
    ) -> Result<SwitchNSquashService, ExecutionError> {
        let t = telemetry::tracer("init_service");
        let s = t.child_span("pg_connect");

        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(conf.db.max_connections)
            .acquire_timeout(conf.db.timeout)
            .connect(&conf.db.url)
            .await?;

        telemetry::end_span(s);

        Ok(SwitchNSquashService {
            pool,
            conf,
            last_active_at: Arc::new(RwLock::new(SystemTime::now())),
            token,
            s3_client,
            tx,
        })
    }

    pub async fn run(&self) -> Result<(), ExecutionError> {
        run_loop(
            &self.conf,
            &self.tx,
            &self.pool,
            self.token.clone(),
            self.last_active_at.clone(),
        )
        .await
    }
}

/// Executes the worker logic for the SnS task.
pub(crate) async fn run_loop(
    conf: &Config,
    tx: &Sender<UploadJob>,
    pool: &PgPool,
    token: CancellationToken,
    last_active_at: Arc<RwLock<SystemTime>>,
) -> Result<(), ExecutionError> {
    let tenant_api_key = &conf.tenant_api_key;
    let enable_compression = conf.enable_compression;
    let conf = &conf.db;

    let mut listener = PgListener::connect_with(pool).await?;
    info!(target: "worker", "Connected to PostgresDB");

    listener
        .listen_all(conf.listen_channels.iter().map(|v| v.as_str()))
        .await?;

    let t = telemetry::tracer("worker_loop_init");
    let s = t.child_span("fetch_keyset");
    let keys: KeySet = fetch_keyset(pool, tenant_api_key).await?;
    telemetry::end_span(s);
    t.end();

    info!(target: "worker", "Fetched keyset for tenant {}", tenant_api_key);

    let mut gc_ticker = interval(conf.cleanup_interval);
    let mut gc_timestamp = SystemTime::now();
    let mut polling_ticker = interval(Duration::from_secs(conf.polling_interval.into()));

    loop {
        // Continue looping until the service is cancelled or a critical error occurs
        // If a transient db error is encountered, keep retrying until it recovers

        {
            let mut value = last_active_at.write().await;
            *value = SystemTime::now();
        }

        match fetch_and_execute_sns_tasks(pool, tx, &keys, conf, enable_compression).await {
            Ok(maybe_remaining) => {
                if maybe_remaining {
                    if token.is_cancelled() {
                        return Ok(());
                    }

                    info!(target: "worker", "more tasks to process, continuing");
                    if let Ok(elapsed) = gc_timestamp.elapsed() {
                        if elapsed >= conf.cleanup_interval {
                            info!(target: "worker", "gc interval, cleaning up");
                            gc_ticker.reset();
                            gc_timestamp = SystemTime::now();
                            if let Err(err) = garbage_collect(pool, conf.gc_batch_limit).await {
                                error!(target: "worker", "Failed to garbage collect: {}", err);
                            }
                        }
                    }

                    continue;
                }
            }
            Err(ExecutionError::DbError(err)) => match err {
                sqlx::Error::PoolTimedOut | sqlx::Error::Io(_) | sqlx::Error::Tls(_) => {
                    error!(target: "worker", "Transient DB error occurred: {err}");
                }
                _ => {
                    tokio::time::sleep(RETRY_DB_CONN_INTERVAL).await;
                }
            },
            Err(err) => {
                error!(target: "worker", "Failed to process SnS tasks: {err}");
            }
        }

        select! {
            _ = token.cancelled() => return Ok(()),
            n = listener.try_recv() => {
                info!(target: "worker", "Received notification {:?}", n);
            },
            _ = polling_ticker.tick() => {
                debug!(target: "worker", "Polling timeout, rechecking for tasks");
            },
            // Garbage collecting
            _ = gc_ticker.tick() => {
                info!(target: "worker", "gc tick, on_idle");
                gc_timestamp = SystemTime::now();
                if let Err(err) = garbage_collect(pool, conf.gc_batch_limit).await {
                    error!(target: "worker", "Failed to garbage collect: {}", err);
                }
            }
        }
    }
}

// Clean up the database by removing old ciphertexts128 already uploaded to S3.
pub async fn garbage_collect(pool: &PgPool, limit: u32) -> Result<(), ExecutionError> {
    // Limit the number of rows to update in case of a large backlog due to catchup or burst
    // Skip Locked to prevent concurrent updates
    let start = SystemTime::now();
    let rows_affected: u64 = sqlx::query!(
        "
        WITH to_update AS (
            SELECT c.ctid
            FROM ciphertexts c
            JOIN ciphertext_digest d
            ON d.tenant_id = c.tenant_id
            AND d.handle = c.handle
            WHERE c.ciphertext128 IS NOT NULL
            AND d.ciphertext128 IS NOT NULL
            ORDER BY c.created_at
            FOR UPDATE SKIP LOCKED
            LIMIT $1::INT
        )

        UPDATE ciphertexts
            SET ciphertext128 = NULL
            WHERE ctid IN (SELECT ctid FROM to_update);
        ",
        limit as i32
    )
    .execute(pool)
    .await?
    .rows_affected();

    if rows_affected > 0 {
        let _s = telemetry::tracer_with_start_time("cleanup_ct128", start);
        info!(target: "worker", "Cleaning up old ciphertexts128, rows_affected: {}", rows_affected);
    }

    Ok(())
}

/// Fetch and process SnS tasks from the database.
async fn fetch_and_execute_sns_tasks(
    pool: &PgPool,
    tx: &Sender<UploadJob>,
    keys: &KeySet,
    conf: &DBConfig,
    enable_compression: bool,
) -> Result<bool, ExecutionError> {
    let mut db_txn = match pool.begin().await {
        Ok(txn) => txn,
        Err(err) => {
            error!(target: "worker", "Failed to begin transaction: {err}");
            return Err(err.into());
        }
    };

    let order = if conf.lifo { Order::Desc } else { Order::Asc };

    let trx = &mut db_txn;

    let mut maybe_remaining = false;
    if let Some(mut tasks) = query_sns_tasks(trx, conf.batch_limit, order).await? {
        maybe_remaining = conf.batch_limit as usize == tasks.len();

        let t = telemetry::tracer("batch_execution");
        t.set_attribute("count", tasks.len().to_string());

        process_tasks(&mut tasks, keys, tx, enable_compression)?;
        update_computations_status(trx, &tasks).await?;

        let s = t.child_span("batch_store_ciphertext128");
        update_ciphertext128(trx, &tasks).await?;
        notify_ciphertext128_ready(trx, &conf.notify_channel).await?;

        // Try to enqueue the tasks for upload in the DB
        // This is a best-effort attempt, as the upload worker might not be available
        enqueue_upload_tasks(trx, &tasks).await?;
        telemetry::end_span(s);

        db_txn.commit().await?;
    } else {
        db_txn.rollback().await?;
    }

    Ok(maybe_remaining)
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

    info!(target: "sns", { count = records.len(), order = order.to_string() }, "Fetched SnS tasks");

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

            Ok(HandleItem {
                tenant_id,
                handle: handle.clone(),
                ct64_compressed: Arc::new(ciphertext),
                ct128: Arc::new(BigCiphertext::default()), // to be computed
                otel: telemetry::tracer_with_handle("task", handle),
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

/// Processes the tasks by decompressing and transforming ciphertexts.
fn process_tasks(
    tasks: &mut [HandleItem],
    keys: &KeySet,
    tx: &Sender<UploadJob>,
    enable_compression: bool,
) -> Result<(), ExecutionError> {
    for task in tasks.iter_mut() {
        let ct64_compressed = task.ct64_compressed.as_ref();
        if ct64_compressed.is_empty() {
            error!(target: "sns", { handle = ?task.handle }, "Empty ciphertext64, skipping task");
            continue; // Skip empty ciphertexts
        }

        let s = task.otel.child_span("decompress_ct64");
        set_server_key(keys.server_key.clone());
        let ct = decompress_ct(&task.handle, ct64_compressed)?;
        telemetry::end_span(s);

        let handle = compact_hex(&task.handle);
        let ct_type = ct.type_name().to_owned();
        info!(target: "sns",  { handle, ct_type }, "Converting ciphertext");

        let mut span = task.otel.child_span("squash_noise");
        telemetry::attribute(&mut span, "ct_type", ct_type);

        match ct.squash_noise_and_serialize(enable_compression) {
            Ok(bytes) => {
                telemetry::end_span(span);
                info!(target: "sns", { handle }, "Ciphertext converted, length: {}, compressed: {}", bytes.len(), enable_compression);

                #[cfg(feature = "test_decrypt_128")]
                decrypt_big_ct(keys, &bytes, &ct, &task.handle, enable_compression);

                let format = if enable_compression {
                    Ciphertext128Format::CompressedOnCpu
                } else {
                    Ciphertext128Format::UncompressedOnCpu
                };

                task.ct128 = Arc::new(BigCiphertext::new(bytes, format));
            }
            Err(err) => {
                telemetry::end_span_with_err(span, err.to_string());
                error!(target: "sns", { handle }, "Failed to convert ct: {err}");
            }
        };

        // Start uploading the ciphertexts as soon as the ct128 is computed
        //
        // The service must continue running the squashed noise algorithm,
        // regardless of the availability of the upload worker.
        if let Err(err) = tx.try_send(UploadJob::Normal(task.clone())) {
            // This could happen if either we are experiencing a burst of tasks
            // or the upload worker cannot recover the connection to AWS S3
            //
            // In this case, we should log the error and rely on the retry mechanism.
            //
            // There are three levels of task buffering:
            // 1. The spawned uploading tasks (size: conf.max_concurrent_uploads)
            // 2. The input channel of the upload worker (size: conf.max_concurrent_uploads * 10)
            // 3. The PostgresDB (size: unlimited)

            error!({target = "worker", action = "review"},  "Failed to send task to upload worker: {err}");
            telemetry::end_span_with_err(task.otel.child_span("send_task"), err.to_string());
        }
    }

    Ok(())
}

/// Updates the database with the computed large ciphertexts.
///
/// The ct128 is temporarily stored in PostgresDB to ensure reliability.
/// After the AWS uploader successfully uploads the ct128 to S3, the ct128 blob
/// is deleted from Postgres.
///
/// The assumption for now is that the DB insertion is faster and more reliable
/// than the S3 upload. Later on, the DB insertion of ct128 might be removed
/// completely.
async fn update_ciphertext128(
    db_txn: &mut Transaction<'_, Postgres>,
    tasks: &[HandleItem],
) -> Result<(), ExecutionError> {
    for task in tasks {
        if !task.ct128.is_empty() {
            let ciphertext128 = task.ct128.bytes();
            let s = task.otel.child_span("ct128_db_insert");

            // Insert the ciphertext128 into the database for reliability
            // Later on, we clean up all uploaded ct128
            let res = sqlx::query!(
                "
                    UPDATE ciphertexts
                    SET ciphertext128 = $1
                    WHERE handle = $2;",
                ciphertext128,
                task.handle
            )
            .execute(db_txn.as_mut())
            .await;

            match res {
                Ok(val) => {
                    info!(target: "worker", handle = compact_hex(&task.handle),
                        query_res = format!("{:?}", val),  "Inserted ct128 in DB");
                    telemetry::end_span(s);
                }
                Err(err) => {
                    error!(target: "worker", handle = ?task.handle, "Failed to insert ct128 in DB: {err}");
                    telemetry::end_span_with_err(s, err.to_string());
                }
            }

            // Notify add_ciphertexts
        } else {
            error!(target: "worker", handle = ?task.handle, "Large ciphertext not computed for task");
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
            error!(target: "worker", handle = ?task.handle, "Large ciphertext not computed for task");
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

    let result = SupportedFheCiphertexts::decompress(ct_type, compressed_ct)?;
    Ok(result)
}
#[cfg(feature = "test_decrypt_128")]
/// Decrypts a squashed noise ciphertext and returns the decrypted value.
/// This function is used for testing purposes only.
fn decrypt_big_ct(
    keys: &KeySet,
    squashed_noise_serialized: &[u8],
    ct: &SupportedFheCiphertexts,
    handle: &[u8],
    is_compressed: bool,
) {
    {
        if let Some(client_key) = &keys.client_key {
            let ct = if is_compressed {
                ct.decrypt_squash_noise_compressed(client_key, squashed_noise_serialized)
            } else {
                ct.decrypt_squash_noise(client_key, squashed_noise_serialized)
            }
            .expect("Failed to decrypt");

            info!(target: "sns", "Decrypted plaintext: {:?}, handle: {}", ct, compact_hex(handle));
        }
    }
}
