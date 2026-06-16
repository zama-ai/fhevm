use alloy::primitives::ruint::aliases::U256;
use connector_utils::{
    tests::{
        db::{
            requests::{
                InsertRequestOptions, insert_rand_public_decryption_request,
                insert_rand_user_decryption_request,
            },
            responses::{insert_rand_public_decrypt_response, insert_rand_user_decrypt_response},
        },
        setup::TestInstanceBuilder,
    },
    types::db::OperationStatus,
};
use rstest::rstest;
use sqlx::{Pool, Postgres, Row, postgres::types::PgInterval};
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::info;
use tx_sender::monitoring::garbage_collection::spawn_garbage_collection_routine;

/// Interval short enough that an inserted element is "expired" after a brief sleep.
const SHORT_INTERVAL: Duration = Duration::from_secs(1);
/// Interval long enough that a freshly inserted element is never collected during a test.
const LONG_INTERVAL: Duration = Duration::from_secs(120);

#[rstest]
#[case::public_decryption_request("PublicDecryptionRequest")]
#[case::user_decryption_request("UserDecryptionRequest")]
#[case::public_decryption_response("PublicDecryptionResponse")]
#[case::user_decryption_response("UserDecryptionResponse")]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn deletes_old_completed_and_failed_decryption(#[case] elem: &str) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let mut inserted_ids = vec![];
    let statuses = [
        OperationStatus::Completed,
        OperationStatus::Failed,
        OperationStatus::Pending,
        OperationStatus::UnderProcess,
    ];
    for status in statuses {
        inserted_ids.push(insert_elem_in_db_and_get_id(test_instance.db(), elem, status).await?);
    }

    // Sleep to let elements "expires" in DB
    tokio::time::sleep(Duration::from_secs(2)).await;
    run_garbage_collection(test_instance.db(), SHORT_INTERVAL, LONG_INTERVAL).await;

    // Verify old completed/failed elem are removed from DB, and others are still in DB
    let remaining_ids = fetch_elem_ids_in_db(test_instance.db(), elem).await?;
    assert_eq!(remaining_ids, inserted_ids[2..]);

    Ok(())
}

#[rstest]
#[case::public_decryption_request("PublicDecryptionRequest")]
#[case::user_decryption_request("UserDecryptionRequest")]
#[case::public_decryption_response("PublicDecryptionResponse")]
#[case::user_decryption_response("UserDecryptionResponse")]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn does_not_delete_recent_decryption(#[case] elem: &str) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let mut inserted_ids = vec![];
    let statuses = [
        OperationStatus::Completed,
        OperationStatus::Failed,
        OperationStatus::Pending,
        OperationStatus::UnderProcess,
    ];
    for status in statuses {
        inserted_ids.push(insert_elem_in_db_and_get_id(test_instance.db(), elem, status).await?);
    }

    // Long intervals: nothing is old enough to be collected.
    run_garbage_collection(test_instance.db(), LONG_INTERVAL, LONG_INTERVAL).await;

    // Verify all elems are still in DB
    let remaining_ids = fetch_elem_ids_in_db(test_instance.db(), elem).await?;
    assert_eq!(remaining_ids, inserted_ids);

    Ok(())
}

#[rstest]
#[case::public_decryption_request("PublicDecryptionRequest")]
#[case::user_decryption_request("UserDecryptionRequest")]
#[case::public_decryption_response("PublicDecryptionResponse")]
#[case::user_decryption_response("UserDecryptionResponse")]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn unlocks_old_under_process_decryption(#[case] elem: &str) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let id = insert_elem_in_db_and_get_id(test_instance.db(), elem, OperationStatus::UnderProcess)
        .await?;

    // Sleep to reach the (short) `under_process_limit`.
    tokio::time::sleep(Duration::from_secs(2)).await;
    // Use a long expiry so the GC does not delete the element before unlocking it.
    run_garbage_collection(test_instance.db(), LONG_INTERVAL, SHORT_INTERVAL).await;

    let status = get_elem_status(test_instance.db(), elem, id).await?;
    assert_eq!(status, OperationStatus::Pending);

    Ok(())
}

#[rstest]
#[case::public_decryption_request("PublicDecryptionRequest")]
#[case::user_decryption_request("UserDecryptionRequest")]
#[case::public_decryption_response("PublicDecryptionResponse")]
#[case::user_decryption_response("UserDecryptionResponse")]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn does_not_unlock_recent_under_process_decryption(#[case] elem: &str) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let id = insert_elem_in_db_and_get_id(test_instance.db(), elem, OperationStatus::UnderProcess)
        .await?;

    // Long `under_process_limit`: the element is too recent to be unlocked.
    run_garbage_collection(test_instance.db(), LONG_INTERVAL, LONG_INTERVAL).await;

    let status = get_elem_status(test_instance.db(), elem, id).await?;
    assert_eq!(status, OperationStatus::UnderProcess);

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////

/// Runs the garbage collection routine against `db` for a short while, then stops it.
///
/// The routine performs both deletion and unlocking on every iteration, so callers pick a long
/// interval for the dimension they don't want to trigger.
async fn run_garbage_collection(
    db: &Pool<Postgres>,
    decryption_expiry: Duration,
    under_process_limit: Duration,
) {
    info!("Triggering the garbage collection routine...");
    let cancel_token = CancellationToken::new();
    let handle = spawn_garbage_collection_routine(
        Duration::from_millis(200),
        PgInterval::try_from(decryption_expiry).unwrap(),
        PgInterval::try_from(under_process_limit).unwrap(),
        db.clone(),
        cancel_token.clone(),
    );

    // Let the routine run a few (idempotent) iterations, then stop it.
    tokio::time::sleep(Duration::from_secs(1)).await;
    cancel_token.cancel();
    handle.await.expect("garbage collection routine panicked");
    info!("Done!");
}

async fn insert_elem_in_db_and_get_id(
    db: &Pool<Postgres>,
    elem: &str,
    status: OperationStatus,
) -> anyhow::Result<U256> {
    let id = match elem {
        "PublicDecryptionRequest" => {
            insert_rand_public_decryption_request(
                db,
                InsertRequestOptions::new().with_status(status),
            )
            .await?
            .decryptionId
        }
        "UserDecryptionRequest" => {
            insert_rand_user_decryption_request(db, InsertRequestOptions::new().with_status(status))
                .await?
                .decryptionId
        }
        "PublicDecryptionResponse" => {
            insert_rand_public_decrypt_response(db, None, Some(status))
                .await?
                .decryption_id
        }
        "UserDecryptionResponse" => {
            insert_rand_user_decrypt_response(db, None, Some(status))
                .await?
                .decryption_id
        }
        _ => panic!("Unexpected element type"),
    };
    Ok(id)
}

async fn fetch_elem_ids_in_db(db: &Pool<Postgres>, elem: &str) -> anyhow::Result<Vec<U256>> {
    info!("Fetching remaining {elem} ids in DB...");
    let query = match elem {
        "PublicDecryptionRequest" => {
            "SELECT decryption_id FROM public_decryption_requests ORDER BY created_at"
        }
        "UserDecryptionRequest" => {
            "SELECT decryption_id FROM user_decryption_requests ORDER BY created_at"
        }
        "PublicDecryptionResponse" => {
            "SELECT decryption_id FROM public_decryption_responses ORDER BY created_at"
        }
        "UserDecryptionResponse" => {
            "SELECT decryption_id FROM user_decryption_responses ORDER BY created_at"
        }
        _ => panic!("Unexpected element type"),
    };
    let remaining_ids = sqlx::query(query)
        .fetch_all(db)
        .await?
        .into_iter()
        .map(|row| U256::from_le_slice(&row.try_get::<Vec<u8>, _>("decryption_id").unwrap()))
        .collect();
    info!("Done!");
    Ok(remaining_ids)
}

async fn get_elem_status(
    db: &Pool<Postgres>,
    elem: &str,
    id: U256,
) -> anyhow::Result<OperationStatus> {
    info!("Getting {elem} #{id} status in DB...");

    let query = match elem {
        "PublicDecryptionRequest" => {
            "SELECT status FROM public_decryption_requests WHERE decryption_id = $1"
        }
        "UserDecryptionRequest" => {
            "SELECT status FROM user_decryption_requests WHERE decryption_id = $1"
        }
        "PublicDecryptionResponse" => {
            "SELECT status FROM public_decryption_responses WHERE decryption_id = $1"
        }
        "UserDecryptionResponse" => {
            "SELECT status FROM user_decryption_responses WHERE decryption_id = $1"
        }
        _ => panic!("Unexpected element type"),
    };
    let status: OperationStatus = sqlx::query(query)
        .bind(id.as_le_slice())
        .fetch_one(db)
        .await?
        .try_get("status")?;
    info!("OK!");

    Ok(status)
}
