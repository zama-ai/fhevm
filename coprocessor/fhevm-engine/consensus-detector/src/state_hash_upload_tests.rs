//! Component regressions for hash uploads racing real ingestion containment.
use super::*;
use crate::ingestion_recovery_tests::{
    object_store_with_response, partial_block, repair_is_contained,
};
use fhevm_engine_common::chain_recovery::{lock_history_tx, ListenerSource};
use fhevm_engine_common::versioning::CUTOVER_LOCK_ID;
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::oneshot;
use tokio::time::{sleep, timeout};

#[derive(Clone, Copy, Debug)]
enum Track {
    Host,
    Gateway,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Completion {
    Success,
    Timeout,
    Cancel,
}

async fn upload(
    track: Track,
    pool: &Pool<Postgres>,
    s3: &aws_sdk_s3::Client,
) -> anyhow::Result<()> {
    let cutoff = Duration::from_secs(3600);
    match track {
        Track::Host => upload_pending_state_hashes(pool, s3, "test", 10, cutoff).await,
        Track::Gateway => upload_pending_gw_state_hashes(pool, s3, "test", 54321, 10, cutoff).await,
    }
}

async fn uploaded(track: Track, pool: &Pool<Postgres>) -> anyhow::Result<bool> {
    let (schema, chain) = match track {
        Track::Host => ("public", 42_i64),
        Track::Gateway => (GCS_SCHEMA_QUOTED, 54321),
    };
    Ok(sqlx::query_scalar(&format!(
        "SELECT s3_uploaded_at IS NOT NULL FROM {schema}.state_hash
         WHERE chain_id = $1 AND block_number = 100"
    ))
    .bind(chain)
    .fetch_one(pool)
    .await?)
}

async fn containment_during_upload(track: Track, completion: Completion) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = partial_block().await?;
    let mut tx = pool.begin().await?;
    compute_and_insert_gcs(&mut tx, 42, 100, 100, 10, "public").await?;
    tx.commit().await?;
    if matches!(track, Track::Gateway) {
        sqlx::raw_sql(&format!(
            "CREATE SCHEMA {GCS_SCHEMA_QUOTED};
             CREATE TABLE {GCS_SCHEMA_QUOTED}.state_hash (LIKE public.state_hash INCLUDING ALL);
             INSERT INTO {GCS_SCHEMA_QUOTED}.state_hash (chain_id, block_number, state_hash)
               SELECT 54321, block_number, state_hash FROM public.state_hash"
        ))
        .execute(&pool)
        .await?;
    }
    // The guarded window check and bookkeeping must reuse the held connection.
    let upload_pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(1))
        .connect_with((*pool.connect_options()).clone())
        .await?;
    let (received_tx, received_rx) = oneshot::channel();
    let (finish_tx, finish_rx) = oneshot::channel();
    let (s3, server) = object_store_with_response(async move {
        received_tx.send(()).unwrap();
        let _ = finish_rx.await;
    })
    .await?;
    let task_pool = upload_pool.clone();
    let upload = tokio::spawn(async move { upload(track, &task_pool, &s3).await });
    timeout(Duration::from_secs(10), received_rx).await??;

    // Uploads retain the shared cutover fence, never history admission: normal
    // ingestion must still be able to acquire its history lock during S3 I/O.
    let mut history_tx = begin_write_guarded(&pool, false, GcsRollbackPolicy::Skip)
        .await?
        .into_tx()
        .expect("fixture must admit ordinary ingestion");
    timeout(
        Duration::from_secs(2),
        lock_history_tx(&mut history_tx, ListenerSource::Host, false, 42),
    )
    .await??;
    history_tx.rollback().await?;

    let halt_pool = pool.clone();
    let halt = tokio::spawn(async move {
        // A real late ACL changes committed inventory. Its halt must wait for
        // the upload's bookkeeping, or for failure/cancellation to release it.
        repair_is_contained(&mut db, &halt_pool).await?;
        uploaded(track, &halt_pool).await
    });
    timeout(Duration::from_secs(10), async {
        loop {
            assert!(
                !halt.is_finished(),
                "{track:?}: halt passed an in-flight upload"
            );
            let waiting: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM pg_locks
                 WHERE locktype = 'advisory' AND NOT granted AND mode = 'ExclusiveLock'
                   AND database = (SELECT oid FROM pg_database WHERE datname = current_database())
                   AND classid = $1::bigint::oid AND objid = $2::bigint::oid)",
            )
            .bind(CUTOVER_LOCK_ID >> 32)
            .bind(CUTOVER_LOCK_ID & 0xffff_ffff)
            .fetch_one(&pool)
            .await?;
            if waiting {
                return Ok::<_, anyhow::Error>(());
            }
            sleep(Duration::from_millis(10)).await;
        }
    })
    .await??;
    assert!(!uploaded(track, &pool).await?);

    match completion {
        Completion::Success => {
            finish_tx.send(()).unwrap();
            timeout(Duration::from_secs(10), upload).await???;
            timeout(Duration::from_secs(10), server).await??;
        }
        Completion::Timeout => {
            // The server never responds. The real production deadline must
            // release the fence and leave the hash eligible for a later retry.
            timeout(STATE_HASH_UPLOAD_TIMEOUT + Duration::from_secs(5), upload).await???;
            server.abort();
        }
        Completion::Cancel => {
            upload.abort();
            assert!(upload.await.unwrap_err().is_cancelled());
            server.abort();
        }
    }
    let uploaded_at_halt = timeout(Duration::from_secs(10), halt).await???;
    assert_eq!(
        uploaded_at_halt,
        completion == Completion::Success,
        "{track:?}"
    );
    // Cancellation must return the connection to a pool that stays open.
    assert_eq!(uploaded(track, &upload_pool).await?, uploaded_at_halt);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn host_and_gateway_hash_uploads_finish_before_containment() -> anyhow::Result<()> {
    for track in [Track::Host, Track::Gateway] {
        containment_during_upload(track, Completion::Success).await?;
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn hung_hash_uploads_release_the_fence_and_remain_pending() -> anyhow::Result<()> {
    for track in [Track::Host, Track::Gateway] {
        containment_during_upload(track, Completion::Timeout).await?;
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn cancelled_hash_uploads_release_the_fence_and_remain_pending() -> anyhow::Result<()> {
    for track in [Track::Host, Track::Gateway] {
        containment_during_upload(track, Completion::Cancel).await?;
    }
    Ok(())
}
