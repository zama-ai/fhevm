use crate::keyset::fetch_keyset;
use crate::squash_noise::SquashNoiseCiphertext;
use crate::HandleItem;
use crate::KeySet;
use crate::UploadJob;
use crate::{Config, DBConfig, ExecutionError};
use fhevm_engine_common::telemetry;
use fhevm_engine_common::utils::compact_hex;
use sqlx::pool::PoolConnection;
use sqlx::postgres::PgListener;
use sqlx::{Acquire, PgPool, Postgres, Transaction};
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use tfhe::set_server_key;
use tokio::select;
use tokio::sync::mpsc::Sender;
use tokio::time::{interval, sleep};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use fhevm_engine_common::types::{get_ct_type, SupportedFheCiphertexts};

const RETRY_DB_CONN_INTERVAL: Duration = Duration::from_secs(5);

enum ConnStatus {
    Established(sqlx::pool::PoolConnection<sqlx::Postgres>),
    Failed,
    Cancelled,
}

/// Executes the worker logic for the SnS task.
pub(crate) async fn run_loop(
    conf: &Config,
    tx: &Sender<UploadJob>,
    token: CancellationToken,
) -> Result<(), ExecutionError> {
    let tenant_api_key = &conf.tenant_api_key;
    let conf = &conf.db;

    let t = telemetry::tracer("init_service");
    let s = t.child_span("pg_connect");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(conf.max_connections)
        .connect(&conf.url)
        .await?;
    telemetry::end_span(s);

    let mut listener = PgListener::connect_with(&pool).await?;

    info!(target: "worker", "Connected to PostgresDB");

    listener
        .listen_all(conf.listen_channels.iter().map(|v| v.as_str()))
        .await?;

    let s = t.child_span("fetch_keyset");
    let keys: KeySet = fetch_keyset(&pool, tenant_api_key).await?;
    telemetry::end_span(s);

    t.end();

    loop {
        let mut conn: PoolConnection<Postgres> =
            match acquire_connection(&pool, token.clone()).await {
                ConnStatus::Established(conn) => conn,
                ConnStatus::Failed => {
                    sleep(RETRY_DB_CONN_INTERVAL).await;
                    continue; // Retry to reacquire a connection
                }
                ConnStatus::Cancelled => break,
            };

        let mut gc_ticker = interval(conf.cleanup_interval);
        let mut polling_ticker = interval(Duration::from_secs(conf.polling_interval.into()));

        loop {
            match fetch_and_execute_sns_tasks(&mut conn, tx, &keys, conf).await {
                Ok(_) => {
                    // Check if more tasks are available
                    let count = get_remaining_tasks(&mut conn).await?;
                    if count > 0 {
                        if token.is_cancelled() {
                            return Ok(());
                        }
                        info!(target: "worker", {count}, "SnS tasks available");
                        continue;
                    }
                }
                Err(ExecutionError::DbError(err)) => {
                    error!(target: "worker", "Failed to proceed due to DB error: {err}");
                    break; // Break to reacquire a connection
                }
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
                    garbage_collect(&mut conn).await?;
                }
            }
        }
    }

    Ok(())
}

// Clean up the database by removing old ciphertexts128 already uploaded to S3.
async fn garbage_collect(conn: &mut PoolConnection<Postgres>) -> Result<(), ExecutionError> {
    let start = SystemTime::now();
    let rows = sqlx::query!(
        "UPDATE ciphertexts c
        SET ciphertext128 = NULL
        WHERE ciphertext128 is NOT NULL -- needed for getting an accurate rows_affected
        AND EXISTS (
            SELECT 1
            FROM ciphertext_digest d
            WHERE d.tenant_id = c.tenant_id
            AND d.handle = c.handle
            AND d.ciphertext128 IS NOT NULL
        );"
    )
    .execute(conn.as_mut())
    .await?.rows_affected();
 
    if rows > 0 {
        let _s = telemetry::tracer_with_start_time("cleanup_ct128", start);
        info!(target: "worker", "Cleaning up old ciphertexts128, rows_affected: {}", rows);
    }

    Ok(())
}

/// Fetch and process SnS tasks from the database.
async fn fetch_and_execute_sns_tasks(
    conn: &mut PoolConnection<Postgres>,
    tx: &Sender<UploadJob>,
    keys: &KeySet,
    conf: &DBConfig,
) -> Result<(), ExecutionError> {
    let mut db_txn = match conn.begin().await {
        Ok(txn) => txn,
        Err(err) => {
            error!(target: "worker", "Failed to begin transaction: {err}");
            return Err(err.into());
        }
    };

    let trx = &mut db_txn;

    if let Some(mut tasks) = query_sns_tasks(trx, conf.batch_limit).await? {
        let t = telemetry::tracer("batch_execution");
        t.set_attribute("count", tasks.len().to_string());

        process_tasks(&mut tasks, keys, tx)?;
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

    Ok(())
}

async fn acquire_connection(pool: &PgPool, token: CancellationToken) -> ConnStatus {
    select! {
        conn = pool.acquire() => match conn {
            Ok(conn) =>   ConnStatus::Established(conn),
            Err(err) => {
                error!(target: "worker", "Failed to acquire connection: {err}");
                ConnStatus::Failed
            }
        },
        _ = token.cancelled() => {
            info!(target: "worker", "Cancellation received while acquiring connection");
            ConnStatus::Cancelled
        }
    }
}

/// Queries the database for a fixed number of tasks.
async fn query_sns_tasks(
    db_txn: &mut Transaction<'_, Postgres>,
    limit: u32,
) -> Result<Option<Vec<HandleItem>>, ExecutionError> {
    let start_time = SystemTime::now();
    let records = sqlx::query!(
        " 
        SELECT a.*, c.ciphertext
        FROM pbs_computations a
        JOIN ciphertexts c 
        ON a.handle = c.handle          -- fetch handles inserted into the ciphertexts table
        WHERE c.ciphertext IS NOT NULL  -- filter out tasks with no computed ciphertext64
        AND a.is_completed = FALSE      -- filter out completed tasks
        ORDER BY a.created_at           -- quickly find uncompleted tasks
        FOR UPDATE SKIP LOCKED
        LIMIT $1;
        ",
        limit as i64
    )
    .fetch_all(db_txn.as_mut())
    .await?;

    info!(target: "sns", { count = records.len()}, "Fetched SnS tasks");

    if records.is_empty() {
        return Ok(None);
    }

    let t = telemetry::tracer_with_start_time("db_fetch_tasks", start_time);
    t.set_attribute("count", records.len().to_string());
    t.end();

    let tasks = records
        .into_iter()
        .map(|record| HandleItem {
            tenant_id: record.tenant_id,
            handle: record.handle.clone(),
            ct64_compressed: Arc::new(record.ciphertext),
            ct128_uncompressed: Arc::new(Vec::new()), // to be computed
            otel: telemetry::tracer_with_handle("task", record.handle),
        })
        .collect();

    Ok(Some(tasks))
}

/// Returns the number of remaining tasks in the database.
async fn get_remaining_tasks(
    conn: &mut sqlx::pool::PoolConnection<sqlx::Postgres>,
) -> Result<i64, ExecutionError> {
    let mut db_txn = match conn.begin().await {
        Ok(txn) => txn,
        Err(err) => {
            error!(target: "worker", "Failed to begin transaction: {err}");
            return Err(err.into());
        }
    };

    let records_count = sqlx::query_scalar!(
        "
        SELECT COUNT(*)
        FROM (
            SELECT 1
            FROM pbs_computations a
            JOIN ciphertexts c 
            ON a.handle = c.handle
            WHERE c.ciphertext IS NOT NULL
            AND a.is_completed = FALSE -- filter out completed tasks
            FOR UPDATE OF a SKIP LOCKED -- don't count locked rows
        ) AS unlocked_rows;
        ",
    )
    .fetch_one(db_txn.as_mut())
    .await?;

    Ok(records_count.unwrap_or(0))
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

        match ct.squash_noise_and_serialize() {
            Ok(squashed_noise_serialized) => {
                telemetry::end_span(span);
                info!(target: "sns", { handle }, "Ciphertext converted, length: {}", squashed_noise_serialized.len());

                // Optional: Decrypt and log for debugging
                #[cfg(feature = "test_decrypt_128")]
                {
                    if let Some(client_key) = &keys.client_key {
                        let ct = ct
                            .decrypt_squash_noise(client_key, &squashed_noise_serialized)
                            .expect("Failed to decrypt");

                        info!(target: "sns", { handle }, "Decrypted plaintext: {:?}", ct);
                    }
                }

                task.ct128_uncompressed = Arc::new(squashed_noise_serialized);
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
        if !task.ct128_uncompressed.is_empty() {
            let ciphertext128 = &task.ct128_uncompressed;
            let s = task.otel.child_span("ct128_db_insert");

            // Insert the ciphertext128 into the database for reliability
            // Later on, we clean up all uploaded ct128
            let res = sqlx::query!(
                    "
                    UPDATE ciphertexts
                    SET ciphertext128 = $1
                    WHERE handle = $2;",
                    ciphertext128.as_ref(),
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
        if !task.ct128_uncompressed.is_empty() {
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
