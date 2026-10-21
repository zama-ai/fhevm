use alloy::primitives::{Address, B256};
use ciphertext_attestation::{prf::Version, PrfOutputAttestation};
use prf_service::{
    pads::{read, Admission, Admissions, PadOutput, PadState},
    PrfPath,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{sync::Arc, time::Duration};
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};

const LIMIT: i64 = 100;
const MAX_ADDS: usize = 2;
const CONNECTOR: Address = Address::repeat_byte(0x11);
const OTHER_CONNECTOR: Address = Address::repeat_byte(0x22);

async fn test_db() -> (DBInstance, PgPool, Arc<Admissions>) {
    let instance = setup_test_db(ImportMode::None).await.expect("test db");
    let pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(instance.db_url())
        .await
        .expect("pool");
    let admissions = Arc::new(Admissions::new(pool.clone(), LIMIT, MAX_ADDS));
    (instance, pool, admissions)
}

async fn lock_connector(
    pool: &PgPool,
    connector: Address,
) -> sqlx::Transaction<'static, sqlx::Postgres> {
    let mut lock = pool.begin().await.unwrap();
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended('pads:' || encode($1, 'hex'), 0))")
        .bind(connector.as_slice())
        .execute(lock.as_mut())
        .await
        .unwrap();
    lock
}

async fn lock_waiters(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM pg_locks WHERE locktype = 'advisory' AND NOT granted")
        .fetch_one(pool)
        .await
        .unwrap()
}

fn path(label: u8, pad_length: u16) -> PrfPath {
    PrfPath {
        prf: 0,
        label: B256::repeat_byte(label),
        pad_length,
    }
}

async fn set_output(pool: &PgPool, path: &PrfPath, output: &PadOutput) {
    let attestation = &output.attestation;
    sqlx::query(
        "UPDATE pads SET ciphertext128 = $4, ciphertext_digest = $5, sns_ciphertext_digest = $6,
                signer = $7, signature = $8
          WHERE prf_instance = $1 AND label = $2 AND pad_length = $3",
    )
    .bind(i32::from(path.prf))
    .bind(path.label.as_slice())
    .bind(i32::from(path.pad_length))
    .bind(&output.ciphertext128)
    .bind(attestation.ciphertext_digest.as_slice())
    .bind(attestation.sns_ciphertext_digest.as_slice())
    .bind(attestation.signer.as_slice())
    .bind(&attestation.signature)
    .execute(pool)
    .await
    .unwrap();
}

async fn set_error(pool: &PgPool, path: &PrfPath) {
    sqlx::query(
        "UPDATE pads SET is_error = true
          WHERE prf_instance = $1 AND label = $2 AND pad_length = $3",
    )
    .bind(i32::from(path.prf))
    .bind(path.label.as_slice())
    .bind(i32::from(path.pad_length))
    .execute(pool)
    .await
    .unwrap();
}

#[tokio::test]
async fn concurrent_requests_make_one_row() {
    let (_instance, _pool, admissions) = test_db().await;
    let requests = (0..13).map(|i| {
        let admissions = admissions.clone();
        let connector = Address::repeat_byte(i + 1);
        tokio::spawn(async move { admissions.admit(&path(1, 64), connector).await })
    });
    let requests: Vec<_> = requests.collect();
    let mut created = 0;
    for request in requests {
        match request.await.unwrap().unwrap() {
            Admission::Created => created += 1,
            admission => assert_eq!(admission, Admission::Existing(PadState::Pending)),
        }
    }
    assert_eq!(created, 1);
}

#[tokio::test]
async fn concurrent_requests_of_a_connector_stay_in_its_budget() {
    let (_instance, _pool, admissions) = test_db().await;
    let requests = (0..10).map(|label| {
        let admissions = admissions.clone();
        tokio::spawn(async move { admissions.admit(&path(label, 64), CONNECTOR).await })
    });
    let requests: Vec<_> = requests.collect();
    let mut created = 0;
    for request in requests {
        match request.await.unwrap().unwrap() {
            Admission::Created => created += 1,
            admission => assert_eq!(admission, Admission::OverBudget),
        }
    }
    assert_eq!(created, 1);
}

#[tokio::test]
async fn requests_on_two_pods_stay_in_the_budget() {
    let (_instance, pool, pod_a) = test_db().await;
    let pod_b = Arc::new(Admissions::new(pool.clone(), LIMIT, MAX_ADDS));
    let requests: Vec<_> = (0..10)
        .map(|label| {
            let pod = if label % 2 == 0 {
                pod_a.clone()
            } else {
                pod_b.clone()
            };
            tokio::spawn(async move { pod.admit(&path(label, 64), CONNECTOR).await })
        })
        .collect();
    let mut created = 0;
    for request in requests {
        match request.await.unwrap().unwrap() {
            Admission::Created => created += 1,
            admission => assert_eq!(admission, Admission::OverBudget),
        }
    }
    assert_eq!(created, 1);
}

#[tokio::test]
async fn connector_over_budget_joins_a_row_added_during_its_admission() {
    let (_instance, pool, admissions) = test_db().await;
    assert_eq!(
        admissions.admit(&path(1, 64), CONNECTOR).await.unwrap(),
        Admission::Created
    );

    // Hold the lock of CONNECTOR. Its `admit` then stops after the first read.
    let lock = lock_connector(&pool, CONNECTOR).await;
    let blocked = tokio::spawn({
        let admissions = admissions.clone();
        async move { admissions.admit(&path(2, 64), CONNECTOR).await }
    });
    while lock_waiters(&pool).await == 0 {
        tokio::task::yield_now().await;
    }

    // Another connector adds the row now.
    assert_eq!(
        admissions
            .admit(&path(2, 64), OTHER_CONNECTOR)
            .await
            .unwrap(),
        Admission::Created
    );
    lock.rollback().await.unwrap();
    assert_eq!(
        blocked.await.unwrap().unwrap(),
        Admission::Existing(PadState::Pending)
    );
}

#[tokio::test]
async fn requests_of_a_connector_wait_in_memory_not_on_the_lock() {
    let (_instance, pool, admissions) = test_db().await;
    // Hold the lock of CONNECTOR, so its requests stop before they can add a row.
    let lock = lock_connector(&pool, CONNECTOR).await;
    let requests: Vec<_> = (0..10)
        .map(|label| {
            let admissions = admissions.clone();
            tokio::spawn(async move { admissions.admit(&path(label, 64), CONNECTOR).await })
        })
        .collect();
    while lock_waiters(&pool).await < MAX_ADDS as i64 {
        tokio::task::yield_now().await;
    }
    // Only MAX_ADDS requests wait for the lock with a connection. The others wait in memory.
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_eq!(lock_waiters(&pool).await, MAX_ADDS as i64);

    // Another connector is not blocked.
    assert_eq!(
        admissions
            .admit(&path(20, 64), OTHER_CONNECTOR)
            .await
            .unwrap(),
        Admission::Created
    );

    lock.rollback().await.unwrap();
    let mut created = 0;
    for request in requests {
        match request.await.unwrap().unwrap() {
            Admission::Created => created += 1,
            admission => assert_eq!(admission, Admission::OverBudget),
        }
    }
    assert_eq!(created, 1);
}

#[tokio::test]
async fn budget_counts_the_pending_pad_bits_of_each_connector() {
    let (_instance, pool, admissions) = test_db().await;
    let admit = |path, connector| {
        let admissions = admissions.clone();
        async move { admissions.admit(&path, connector).await.unwrap() }
    };

    assert_eq!(admit(path(1, 64), CONNECTOR).await, Admission::Created);
    assert_eq!(admit(path(2, 64), CONNECTOR).await, Admission::OverBudget);
    // Another length of the same label is another output.
    assert_eq!(admit(path(1, 36), CONNECTOR).await, Admission::Created);
    // Joining a row takes no slot, and the other connector has its own budget.
    assert_eq!(
        admit(path(1, 64), OTHER_CONNECTOR).await,
        Admission::Existing(PadState::Pending)
    );
    assert_eq!(
        admit(path(2, 64), OTHER_CONNECTOR).await,
        Admission::Created
    );

    // A failed row frees its slots.
    set_error(&pool, &path(1, 64)).await;
    assert_eq!(
        admit(path(1, 64), CONNECTOR).await,
        Admission::Existing(PadState::Failed)
    );
    assert_eq!(admit(path(3, 64), CONNECTOR).await, Admission::Created);
}

#[tokio::test]
async fn complete_row_gives_the_output() {
    let (_instance, pool, admissions) = test_db().await;
    let path = path(1, 64);
    assert_eq!(read(&pool, &path).await.unwrap(), None);
    assert_eq!(
        admissions.admit(&path, CONNECTOR).await.unwrap(),
        Admission::Created
    );

    let output = PadOutput {
        ciphertext128: vec![7; 32],
        attestation: PrfOutputAttestation {
            version: Version::V1,
            ciphertext_digest: B256::repeat_byte(1),
            sns_ciphertext_digest: B256::repeat_byte(2),
            signer: Address::repeat_byte(3),
            signature: vec![4; 65],
        },
    };
    set_output(&pool, &path, &output).await;

    let complete = PadState::Complete(output);
    assert_eq!(read(&pool, &path).await.unwrap(), Some(complete.clone()));
    assert_eq!(
        admissions.admit(&path, OTHER_CONNECTOR).await.unwrap(),
        Admission::Existing(complete)
    );
}

#[tokio::test]
async fn table_refuses_a_pad_length_above_uint16() {
    let (_instance, pool, _admissions) = test_db().await;
    let label = B256::repeat_byte(1);
    let insert = |pad_length: i32| {
        sqlx::query(
            "INSERT INTO pads (prf_instance, label, pad_length, triggered_by)
             VALUES (0, $1, $2, $3)",
        )
        .bind(label.as_slice())
        .bind(pad_length)
        .bind(CONNECTOR.as_slice())
        .execute(&pool)
    };
    insert(65535).await.unwrap();
    let err = insert(65536).await.unwrap_err();
    let code = err.as_database_error().and_then(|e| e.code());
    // 23514 is check_violation.
    assert_eq!(code.as_deref(), Some("23514"), "{err}");
}
