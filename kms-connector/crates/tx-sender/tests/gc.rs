use alloy::primitives::ruint::aliases::U256;
use connector_utils::{
    tests::{
        db::{
            requests::{
                insert_rand_public_decryption_request, insert_rand_user_decryption_request,
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
use tracing::info;
use tx_sender::monitoring::gc::{
    delete_completed_and_failed_public_decryption_requests,
    delete_completed_and_failed_public_decryption_responses,
    delete_completed_and_failed_user_decryption_requests,
    delete_completed_and_failed_user_decryption_responses, unlock_public_decryption_requests,
    unlock_public_decryption_responses, unlock_user_decryption_requests,
    unlock_user_decryption_responses,
};

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn deletes_old_public_decryption_requests() -> anyhow::Result<()> {
    test_delete_old_elements_from_db("PublicDecryptionRequest").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn deletes_old_user_decryption_requests() -> anyhow::Result<()> {
    test_delete_old_elements_from_db("UserDecryptionRequest").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn deletes_old_public_decryption_responses() -> anyhow::Result<()> {
    test_delete_old_elements_from_db("PublicDecryptionResponse").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn deletes_old_user_decryption_responses() -> anyhow::Result<()> {
    test_delete_old_elements_from_db("UserDecryptionResponse").await
}

async fn test_delete_old_elements_from_db(elem: &str) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    // Use small expiry
    let expiry = PgInterval::try_from(Duration::from_secs(1)).unwrap();

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
    trigger_gc_delete_elem(test_instance.db(), elem, expiry).await;

    // Verify old completed/failed elem are removed from DB, and others are still in DB
    let remaining_ids = fetch_elem_ids_in_db(test_instance.db(), elem).await?;
    assert_eq!(remaining_ids, inserted_ids[2..]);

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn do_not_delete_recent_public_decryption_requests() -> anyhow::Result<()> {
    test_do_not_delete_recent_elements_from_db("PublicDecryptionRequest").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn do_not_delete_recent_user_decryption_requests() -> anyhow::Result<()> {
    test_do_not_delete_recent_elements_from_db("UserDecryptionRequest").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn do_not_delete_recent_public_decryption_responses() -> anyhow::Result<()> {
    test_do_not_delete_recent_elements_from_db("PublicDecryptionResponse").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn do_not_delete_recent_user_decryption_responses() -> anyhow::Result<()> {
    test_do_not_delete_recent_elements_from_db("UserDecryptionResponse").await
}

async fn test_do_not_delete_recent_elements_from_db(elem: &str) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    // Use "long" expiry
    let expiry = PgInterval::try_from(Duration::from_secs(120)).unwrap();

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
    trigger_gc_delete_elem(test_instance.db(), elem, expiry).await;

    // Verify all elems are still in DB
    let remaining_ids = fetch_elem_ids_in_db(test_instance.db(), elem).await?;
    assert_eq!(remaining_ids, inserted_ids);

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn unlock_old_elems_under_process() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    // Use a small `under_process_limit`
    let under_process_limit = PgInterval::try_from(Duration::from_secs(1)).unwrap();

    let mut inserted_ids = vec![];
    let elem_type = [
        "PublicDecryptionRequest",
        "UserDecryptionRequest",
        "PublicDecryptionResponse",
        "UserDecryptionResponse",
    ];
    for elem in elem_type {
        inserted_ids.push(
            insert_elem_in_db_and_get_id(test_instance.db(), elem, OperationStatus::UnderProcess)
                .await?,
        );
    }

    // Sleep to reach the `under_process_limit`
    tokio::time::sleep(Duration::from_secs(2)).await;
    for (i, elem) in elem_type.into_iter().enumerate() {
        trigger_gc_unlock_elem(test_instance.db(), elem, under_process_limit).await;
        let status = get_elem_status(test_instance.db(), elem, inserted_ids[i]).await?;
        assert_eq!(status, OperationStatus::Pending);
    }

    Ok(())
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn do_not_unlock_recent_elems_under_process() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    // Use a long `under_process_limit`
    let under_process_limit = PgInterval::try_from(Duration::from_secs(120)).unwrap();

    let mut inserted_ids = vec![];
    let elem_type = [
        "PublicDecryptionRequest",
        "UserDecryptionRequest",
        "PublicDecryptionResponse",
        "UserDecryptionResponse",
    ];
    for elem in elem_type {
        inserted_ids.push(
            insert_elem_in_db_and_get_id(test_instance.db(), elem, OperationStatus::UnderProcess)
                .await?,
        );
    }

    for (i, elem) in elem_type.into_iter().enumerate() {
        trigger_gc_unlock_elem(test_instance.db(), elem, under_process_limit).await;
        let status = get_elem_status(test_instance.db(), elem, inserted_ids[i]).await?;
        assert_eq!(status, OperationStatus::UnderProcess);
    }

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////

async fn insert_elem_in_db_and_get_id(
    db: &Pool<Postgres>,
    elem: &str,
    status: OperationStatus,
) -> anyhow::Result<U256> {
    let id = match elem {
        "PublicDecryptionRequest" => {
            insert_rand_public_decryption_request(db, None, false, Some(status))
                .await?
                .decryptionId
        }
        "UserDecryptionRequest" => {
            insert_rand_user_decryption_request(db, None, false, Some(status))
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

async fn trigger_gc_delete_elem(db: &Pool<Postgres>, elem: &str, expiry: PgInterval) {
    info!("Trigger deletion of {elem} by the GC...");
    match elem {
        "PublicDecryptionRequest" => {
            delete_completed_and_failed_public_decryption_requests(db, expiry).await;
        }
        "UserDecryptionRequest" => {
            delete_completed_and_failed_user_decryption_requests(db, expiry).await;
        }
        "PublicDecryptionResponse" => {
            delete_completed_and_failed_public_decryption_responses(db, expiry).await;
        }
        "UserDecryptionResponse" => {
            delete_completed_and_failed_user_decryption_responses(db, expiry).await;
        }
        _ => panic!("Unexpected element type"),
    }
    info!("Done!");
}

async fn trigger_gc_unlock_elem(db: &Pool<Postgres>, elem: &str, under_process_limit: PgInterval) {
    info!("Trigger unlocking of {elem} by the GC...");
    match elem {
        "PublicDecryptionRequest" => {
            unlock_public_decryption_requests(db, under_process_limit).await;
        }
        "UserDecryptionRequest" => {
            unlock_user_decryption_requests(db, under_process_limit).await;
        }
        "PublicDecryptionResponse" => {
            unlock_public_decryption_responses(db, under_process_limit).await;
        }
        "UserDecryptionResponse" => {
            unlock_user_decryption_responses(db, under_process_limit).await;
        }
        _ => panic!("Unexpected element type"),
    }
    info!("Done!");
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
