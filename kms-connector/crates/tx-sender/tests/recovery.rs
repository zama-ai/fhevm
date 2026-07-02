use alloy::primitives::ruint::aliases::U256;
use connector_utils::{
    tests::{
        db::{
            requests::{
                InsertRequestOptions, insert_rand_crsgen_request, insert_rand_keygen_request,
                insert_rand_new_kms_context, insert_rand_new_kms_epoch,
                insert_rand_prep_keygen_request, insert_rand_public_decryption_request,
                insert_rand_user_decryption_request,
            },
            responses::{
                insert_rand_crsgen_response, insert_rand_epoch_result_response,
                insert_rand_keygen_response, insert_rand_new_kms_context_response,
                insert_rand_prep_keygen_response, insert_rand_public_decrypt_response,
                insert_rand_user_decrypt_response,
            },
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
use tx_sender::monitoring::operation_recovery::spawn_operation_recovery_routine;

/// Interval short enough that an inserted operation is "expired" after a brief sleep.
const SHORT_INTERVAL: Duration = Duration::from_secs(1);
/// Interval long enough that a freshly inserted operation is never affected during a test.
const LONG_INTERVAL: Duration = Duration::from_secs(120);

#[rstest]
#[case::public_decryption_request("PublicDecryptionRequest")]
#[case::user_decryption_request("UserDecryptionRequest")]
#[case::public_decryption_response("PublicDecryptionResponse")]
#[case::user_decryption_response("UserDecryptionResponse")]
#[case::prep_keygen_request("PrepKeygenRequest")]
#[case::keygen_request("KeygenRequest")]
#[case::crsgen_request("CrsgenRequest")]
#[case::prep_keygen_response("PrepKeygenResponse")]
#[case::keygen_response("KeygenResponse")]
#[case::crsgen_response("CrsgenResponse")]
#[case::new_kms_context("NewKmsContext")]
#[case::new_kms_epoch("NewKmsEpoch")]
#[case::new_kms_context_response("NewKmsContextResponse")]
#[case::epoch_result_response("EpochResultResponse")]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn unlocks_old_under_process(#[case] operation: &str) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let id = insert_operation_in_db_and_get_id(
        test_instance.db(),
        operation,
        OperationStatus::UnderProcess,
    )
    .await?;

    // Sleep to reach the (short) `operation_under_process_timeout`.
    tokio::time::sleep(Duration::from_secs(2)).await;
    // Use a long expiry so the operation is not failed before being unlocked.
    run_operation_recovery(test_instance.db(), LONG_INTERVAL, SHORT_INTERVAL).await;

    let status = get_operation_status(test_instance.db(), operation, id).await?;
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
async fn does_not_unlock_recent_under_process_decryption(
    #[case] operation: &str,
) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let id = insert_operation_in_db_and_get_id(
        test_instance.db(),
        operation,
        OperationStatus::UnderProcess,
    )
    .await?;

    // Long intervals: the operation is too recent to be unlocked or failed.
    run_operation_recovery(test_instance.db(), LONG_INTERVAL, LONG_INTERVAL).await;

    let status = get_operation_status(test_instance.db(), operation, id).await?;
    assert_eq!(status, OperationStatus::UnderProcess);

    Ok(())
}

#[rstest]
#[case::public_decryption_request("PublicDecryptionRequest")]
#[case::user_decryption_request("UserDecryptionRequest")]
#[case::public_decryption_response("PublicDecryptionResponse")]
#[case::user_decryption_response("UserDecryptionResponse")]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn fails_expired_decryption(#[case] operation: &str) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let pending_id =
        insert_operation_in_db_and_get_id(test_instance.db(), operation, OperationStatus::Pending)
            .await?;
    let under_process_id = insert_operation_in_db_and_get_id(
        test_instance.db(),
        operation,
        OperationStatus::UnderProcess,
    )
    .await?;
    let completed_id = insert_operation_in_db_and_get_id(
        test_instance.db(),
        operation,
        OperationStatus::Completed,
    )
    .await?;

    // Sleep so inserted operations are older than the (short) expiry.
    tokio::time::sleep(Duration::from_secs(2)).await;
    // Short expiry, long `operation_under_process_timeout`: expired operations are failed, not
    // unlocked.
    run_operation_recovery(test_instance.db(), SHORT_INTERVAL, LONG_INTERVAL).await;

    // Expired pending/under_process decryptions are failed...
    assert_eq!(
        get_operation_status(test_instance.db(), operation, pending_id).await?,
        OperationStatus::Failed
    );
    assert_eq!(
        get_operation_status(test_instance.db(), operation, under_process_id).await?,
        OperationStatus::Failed
    );
    // ...while terminal statuses are left untouched.
    assert_eq!(
        get_operation_status(test_instance.db(), operation, completed_id).await?,
        OperationStatus::Completed
    );

    Ok(())
}

#[rstest]
#[case::public_decryption_request("PublicDecryptionRequest")]
#[case::user_decryption_request("UserDecryptionRequest")]
#[case::public_decryption_response("PublicDecryptionResponse")]
#[case::user_decryption_response("UserDecryptionResponse")]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn does_not_fail_recent_decryption(#[case] operation: &str) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let pending_id =
        insert_operation_in_db_and_get_id(test_instance.db(), operation, OperationStatus::Pending)
            .await?;

    // Long expiry: the operation is too recent to be failed.
    run_operation_recovery(test_instance.db(), LONG_INTERVAL, LONG_INTERVAL).await;

    assert_eq!(
        get_operation_status(test_instance.db(), operation, pending_id).await?,
        OperationStatus::Pending
    );

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////

/// Runs the operation recovery routine against `db` for a short while, then stops it.
///
/// The routine performs both failing and unlocking on every iteration, so callers pick a long
/// interval for the dimension they don't want to trigger.
async fn run_operation_recovery(
    db: &Pool<Postgres>,
    decryption_expiry: Duration,
    operation_under_process_timeout: Duration,
) {
    info!("Triggering the operation recovery routine...");
    let cancel_token = CancellationToken::new();
    let handle = spawn_operation_recovery_routine(
        Duration::from_millis(200),
        PgInterval::try_from(decryption_expiry).unwrap(),
        PgInterval::try_from(operation_under_process_timeout).unwrap(),
        db.clone(),
        cancel_token.clone(),
    );

    // Let the routine run a few (idempotent) iterations, then stop it.
    tokio::time::sleep(Duration::from_secs(1)).await;
    cancel_token.cancel();
    handle.await.expect("operation recovery routine panicked");
    info!("Done!");
}

async fn insert_operation_in_db_and_get_id(
    db: &Pool<Postgres>,
    operation: &str,
    status: OperationStatus,
) -> anyhow::Result<U256> {
    let insert_option = InsertRequestOptions::new().with_status(status);
    let id = match operation {
        "PublicDecryptionRequest" => {
            insert_rand_public_decryption_request(db, insert_option)
                .await?
                .decryptionId
        }
        "UserDecryptionRequest" => {
            insert_rand_user_decryption_request(db, insert_option)
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
        "PrepKeygenRequest" => {
            insert_rand_prep_keygen_request(db, insert_option)
                .await?
                .prepKeygenId
        }
        "KeygenRequest" => insert_rand_keygen_request(db, insert_option).await?.keyId,
        "CrsgenRequest" => insert_rand_crsgen_request(db, insert_option).await?.crsId,
        "PrepKeygenResponse" => {
            insert_rand_prep_keygen_response(db, None, Some(status))
                .await?
                .prep_keygen_id
        }
        "KeygenResponse" => {
            insert_rand_keygen_response(db, None, Some(status))
                .await?
                .key_id
        }
        "CrsgenResponse" => {
            insert_rand_crsgen_response(db, None, Some(status))
                .await?
                .crs_id
        }
        "NewKmsContext" => {
            insert_rand_new_kms_context(db, insert_option)
                .await?
                .contextId
        }
        "NewKmsEpoch" => insert_rand_new_kms_epoch(db, insert_option).await?.epochId,
        "NewKmsContextResponse" => {
            insert_rand_new_kms_context_response(db, None, Some(status))
                .await?
                .context_id
        }
        "EpochResultResponse" => {
            insert_rand_epoch_result_response(db, None, Some(status))
                .await?
                .epoch_id
        }
        _ => panic!("Unexpected operation type"),
    };
    Ok(id)
}

async fn get_operation_status(
    db: &Pool<Postgres>,
    operation: &str,
    id: U256,
) -> anyhow::Result<OperationStatus> {
    info!("Getting {operation} #{id} status in DB...");

    let query = match operation {
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
        "PrepKeygenRequest" => "SELECT status FROM prep_keygen_requests WHERE prep_keygen_id = $1",
        "KeygenRequest" => "SELECT status FROM keygen_requests WHERE key_id = $1",
        "CrsgenRequest" => "SELECT status FROM crsgen_requests WHERE crs_id = $1",
        "PrepKeygenResponse" => {
            "SELECT status FROM prep_keygen_responses WHERE prep_keygen_id = $1"
        }
        "KeygenResponse" => "SELECT status FROM keygen_responses WHERE key_id = $1",
        "CrsgenResponse" => "SELECT status FROM crsgen_responses WHERE crs_id = $1",
        "NewKmsContext" => "SELECT status FROM new_kms_context WHERE context_id = $1",
        "NewKmsEpoch" => "SELECT status FROM new_kms_epoch WHERE epoch_id = $1",
        "NewKmsContextResponse" => {
            "SELECT status FROM new_kms_context_responses WHERE context_id = $1"
        }
        "EpochResultResponse" => "SELECT status FROM epoch_result_responses WHERE epoch_id = $1",
        _ => panic!("Unexpected operation type"),
    };
    let status: OperationStatus = sqlx::query(query)
        .bind(id.as_le_slice())
        .fetch_one(db)
        .await?
        .try_get("status")?;
    info!("OK!");

    Ok(status)
}
