use crate::keyset::fetch_keyset;
use crate::HandleItem;
use crate::KeySet;
use crate::{Config, DBConfig, ExecutionError};
use sqlx::pool::PoolConnection;
use sqlx::postgres::PgListener;
use sqlx::{Acquire, PgPool, Postgres, Transaction};
use std::time::Duration;
use tfhe::integer::IntegerCiphertext;
use tfhe::set_server_key;
use tokio::select;
use tokio::sync::mpsc::Sender;
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
    tx: &Sender<HandleItem>,
    token: CancellationToken,
) -> Result<(), ExecutionError> {
    let tenant_api_key = &conf.tenant_api_key;
    let conf = &conf.db;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(conf.max_connections)
        .connect(&conf.url)
        .await?;

    let mut listener = PgListener::connect_with(&pool).await?;

    listener
        .listen_all(conf.listen_channels.iter().map(|v| v.as_str()))
        .await?;

    let keys: KeySet = fetch_keyset(&pool, tenant_api_key).await?;

    loop {
        let mut conn: PoolConnection<Postgres> =
            match acquire_connection(&pool, token.clone()).await {
                ConnStatus::Established(conn) => conn,
                ConnStatus::Failed => {
                    tokio::time::sleep(RETRY_DB_CONN_INTERVAL).await;
                    continue; // Retry to reacquire a connection
                }
                ConnStatus::Cancelled => break,
            };

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
                _ = listener.try_recv() => {
                    debug!(target: "worker", "Received notification");
                },
                _ = tokio::time::sleep(Duration::from_secs(conf.polling_interval.into())) => {
                    debug!(target: "worker", "Polling timeout, rechecking for tasks");
                }
            }
        }
    }

    Ok(())
}

/// Fetch and process SnS tasks from the database.
async fn fetch_and_execute_sns_tasks(
    conn: &mut PoolConnection<Postgres>,
    tx: &Sender<HandleItem>,
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

    if let Some(mut tasks) = query_sns_tasks(&mut db_txn, conf.batch_limit).await? {
        process_tasks(&mut tasks, keys)?;
        update_computations_status(&mut db_txn, &tasks).await?;
        update_ciphertext128(&mut db_txn, &tasks).await?;
        notify_ciphertext128_ready(&mut db_txn, &conf.notify_channel).await?;
        db_txn.commit().await?;

        // Submits ciphertexts to the upload worker for processing.
        for task in tasks {
            tx.send(task)
                .await
                .map_err(|_| ExecutionError::RecvFailure)?;
        }
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

    let tasks = records
        .into_iter()
        .map(|record| HandleItem {
            tenant_id: record.tenant_id,
            handle: record.handle,
            ct64_compressed: record.ciphertext,
            ct128_uncompressed: None,
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

/// Processes the tasks by decompressing and transforming ciphertexts.
fn process_tasks(tasks: &mut [HandleItem], keys: &KeySet) -> Result<(), ExecutionError> {
    set_server_key(keys.server_key.clone());

    for task in tasks.iter_mut() {
        let ct = decompress_ct(&task.handle, &task.ct64_compressed)?;
        let raw_ct = ct.to_ciphertext64();
        let handle = to_hex(&task.handle);

        let blocks = raw_ct.blocks().len();
        info!(target: "sns",  { handle, blocks }, "Converting ciphertext");

        let ciphertext128 = keys.sns_key.to_large_ciphertext(&raw_ct)?;

        info!(target: "sns",  { handle }, "Ciphertext converted");

        // Optional: Decrypt and log for debugging
        #[cfg(feature = "test_decrypt_128")]
        {
            if let Some(sns_secret_key) = &keys.sns_secret_key {
                let decrypted = sns_secret_key.decrypt_128(&ciphertext128);
                info!(target: "sns", { handle, decrypted }, "Decrypted plaintext");
            }
        }

        let ciphertext128 = bincode::serialize(&ciphertext128)?;
        task.ct128_uncompressed = Some(ciphertext128);
    }

    Ok(())
}

/// Updates the database with the computed large ciphertexts.
async fn update_ciphertext128(
    db_txn: &mut Transaction<'_, Postgres>,
    tasks: &[HandleItem],
) -> Result<(), ExecutionError> {
    for task in tasks {
        if let Some(ciphertext128) = &task.ct128_uncompressed {
            sqlx::query!(
                "
                UPDATE ciphertexts
                SET ciphertext128 = $1
                WHERE handle = $2;",
                ciphertext128,
                task.handle
            )
            .execute(db_txn.as_mut())
            .await?;

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
        if task.ct128_uncompressed.is_some() {
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

// Print first 4 and last 4 bytes of a blob as hex
fn to_hex(handle: &[u8]) -> String {
    const OFFSET: usize = 8;
    match handle.len() {
        0 => String::from("0x"),
        len if len <= 2 * OFFSET => format!("0x{}", hex::encode(handle)),
        _ => {
            let hex_str = hex::encode(handle);
            format!(
                "0x{}...{}",
                &hex_str[..OFFSET],
                &hex_str[hex_str.len() - OFFSET..]
            )
        }
    }
}
