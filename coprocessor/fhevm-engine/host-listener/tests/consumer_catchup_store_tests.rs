use host_listener::consumer::catchup::{store, ManualCatchupArgs};
use sqlx::PgPool;
use test_harness::instance::{setup_test_db, ImportMode};

fn args(from: i64, to: Option<i64>, keep: bool) -> ManualCatchupArgs {
    ManualCatchupArgs {
        catchup_from_block: Some(from),
        catchup_up_to_block: to,
        catchup_unfinalized: false,
        no_override_catchup: keep,
    }
}

#[tokio::test]
async fn flow_is_part_of_identity_and_delivery_admission() {
    let instance = setup_test_db(ImportMode::None).await.unwrap();
    let pool = PgPool::connect(instance.db_url.as_str()).await.unwrap();
    let finalized = args(-10, None, true);
    let mut unfinalized = finalized.clone();
    unfinalized.catchup_unfinalized = true;
    let a = store::ensure_requested(
        &pool,
        1,
        "host.1",
        &finalized,
        Some(1000),
        None,
        false,
    )
    .await
    .unwrap()
    .unwrap();
    let b = store::ensure_requested(
        &pool,
        1,
        "host.1",
        &unfinalized,
        Some(1000),
        None,
        false,
    )
    .await
    .unwrap()
    .unwrap();
    assert_ne!(
        a, b,
        "identical bounds in different flows require different IDs"
    );
    for (flags, id, is_finalized) in
        [(&finalized, a, true), (&unfinalized, b, false)]
    {
        assert_eq!(
            store::ensure_requested(
                &pool, 1, "host.1", flags, None, None, false
            )
            .await
            .unwrap(),
            Some(id)
        );
        assert!(store::accepts(&pool, 1, "host.1", Some(id), is_finalized)
            .await
            .unwrap());
        assert!(!store::accepts(&pool, 1, "host.1", Some(id), !is_finalized)
            .await
            .unwrap());
    }
    let claimed = store::take_request_ownership(&pool, 1, "host.1", false)
        .await
        .unwrap();
    assert_eq!(claimed.len(), 2);
    for request in &claimed {
        assert_eq!(request.finalized, request.id == a);
    }
    let renewed = store::renew(
        &pool,
        &claimed.iter().map(|r| r.id).collect::<Vec<_>>(),
        &claimed.iter().map(|r| r.owner_id).collect::<Vec<_>>(),
        false,
    )
    .await
    .unwrap();
    assert_eq!(renewed.len(), 2);
    for request in &renewed {
        assert_eq!(request.finalized, request.id == a);
    }
    unfinalized.no_override_catchup = false;
    assert_eq!(
        store::ensure_requested(
            &pool,
            1,
            "host.1",
            &unfinalized,
            None,
            None,
            false
        )
        .await
        .unwrap(),
        Some(b)
    );
    assert!(!store::accepts(&pool, 1, "host.1", Some(a), true)
        .await
        .unwrap());
    assert!(store::accepts(&pool, 1, "host.1", Some(b), false)
        .await
        .unwrap());
    let pending = store::renew(
        &pool,
        &claimed.iter().map(|r| r.id).collect::<Vec<_>>(),
        &claimed.iter().map(|r| r.owner_id).collect::<Vec<_>>(),
        false,
    )
    .await
    .unwrap();
    assert!(pending
        .iter()
        .any(|r| r.id == a && r.finalized && r.cancel_pending));
    assert!(pending
        .iter()
        .any(|r| r.id == b && !r.finalized && !r.cancel_pending));
}

#[tokio::test]
async fn creation_leases_atomically_and_matching_flags_do_not_steal() {
    let instance = setup_test_db(ImportMode::None).await.unwrap();
    let pool = PgPool::connect(instance.db_url.as_str()).await.unwrap();
    let flags = args(-10, None, false);
    let left_owner = uuid::Uuid::new_v4();
    let right_owner = uuid::Uuid::new_v4();
    let (left, right) = tokio::join!(
        store::ensure_requested(
            &pool,
            1,
            "host.1",
            &flags,
            Some(1000),
            Some(left_owner),
            false
        ),
        store::ensure_requested(
            &pool,
            1,
            "host.1",
            &flags,
            Some(2000),
            Some(right_owner),
            false
        ),
    );
    let id = left.unwrap().unwrap();
    assert_eq!(right.unwrap(), Some(id));
    let (owner, start, end, valid): (uuid::Uuid, i64, i64, bool) = sqlx::query_as(
        "SELECT owner_id, block_start, block_end, lease_until > clock_timestamp()
         FROM host_consumer_catchups WHERE id = $1")
        .bind(id).fetch_one(&pool).await.unwrap();
    assert!(valid);
    let loser = if owner == left_owner {
        assert_eq!((start, end), (990, 999));
        right_owner
    } else {
        assert_eq!(owner, right_owner);
        assert_eq!((start, end), (1990, 1999));
        left_owner
    };
    assert!(
        store::take_request_ownership(&pool, 1, "host.1", false)
            .await
            .unwrap()
            .is_empty(),
        "the new request is never visible without its creator lease"
    );
    assert!(store::renew(&pool, &[id], &[loser], false)
        .await
        .unwrap()
        .is_empty());
    let adopted = store::renew(&pool, &[id], &[owner], false).await.unwrap();
    assert_eq!(adopted.len(), 1);
    assert_eq!(adopted[0].id, id);
    assert!(adopted[0].request_pending);
}

#[tokio::test]
async fn expired_owner_cannot_renew_or_acknowledge_after_takeover() {
    let instance = setup_test_db(ImportMode::None).await.unwrap();
    let pool = PgPool::connect(instance.db_url.as_str()).await.unwrap();
    let mut ids = Vec::new();
    for start in [10, 30] {
        ids.push(
            store::ensure_requested(
                &pool,
                1,
                "host.1",
                &args(start, Some(start + 10), true),
                Some(100),
                None,
                false,
            )
            .await
            .unwrap()
            .unwrap(),
        );
    }
    assert!(store::take_request_ownership(&pool, 2, "host.1", false)
        .await
        .unwrap()
        .is_empty());
    assert!(store::take_request_ownership(&pool, 1, "other", false)
        .await
        .unwrap()
        .is_empty());
    let (left, right) = tokio::join!(
        store::take_request_ownership(&pool, 1, "host.1", false),
        store::take_request_ownership(&pool, 1, "host.1", false),
    );
    let mut original = left.unwrap();
    original.extend(right.unwrap());
    assert_eq!(
        original.len(),
        2,
        "concurrent claimers must not share ownership"
    );
    original.sort_by_key(|r| r.id);
    assert_ne!(original[0].id, original[1].id);
    assert!(store::take_request_ownership(&pool, 1, "host.1", false)
        .await
        .unwrap()
        .is_empty());
    let a = original.iter().find(|r| r.id == ids[0]).unwrap();
    let b = original.iter().find(|r| r.id == ids[1]).unwrap();
    sqlx::query("UPDATE host_consumer_catchups SET lease_until = clock_timestamp() - interval '1 second' WHERE id = $1")
        .bind(a.id).execute(&pool).await.unwrap();
    assert!(!store::owns_command(&pool, a).await.unwrap());
    assert!(!store::requested(&pool, a.id, a.owner_id, false)
        .await
        .unwrap());
    let renewed =
        store::renew(&pool, &[a.id, b.id], &[a.owner_id, b.owner_id], false)
            .await
            .unwrap();
    assert_eq!(renewed.len(), 1, "an expired token must never be renewed");
    assert_eq!(renewed[0].id, b.id);
    let claimed = store::take_request_ownership(&pool, 1, "host.1", false)
        .await
        .unwrap();
    assert_eq!(claimed.len(), 1);
    let next = &claimed[0];
    assert_eq!(
        (next.id, next.block_start, next.block_end),
        (a.id, a.block_start, a.block_end)
    );
    assert_ne!(next.owner_id, a.owner_id);
    assert!(store::owns_command(&pool, next).await.unwrap());
    assert!(!store::requested(&pool, a.id, a.owner_id, false)
        .await
        .unwrap());
    assert!(store::requested(&pool, next.id, next.owner_id, false)
        .await
        .unwrap());
    // Replacement is operator intent, independent of which replica owns a row.
    store::ensure_requested(
        &pool,
        1,
        "host.1",
        &args(50, Some(60), false),
        Some(100),
        None,
        false,
    )
    .await
    .unwrap();
    assert!(!store::cancelled(&pool, a.id, a.owner_id, false)
        .await
        .unwrap());
    let pending: bool = sqlx::query_scalar(
        "SELECT cancel_pending FROM host_consumer_catchups WHERE id = $1",
    )
    .bind(a.id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(
        pending,
        "stale callbacks cannot acknowledge the new owner's cancellation"
    );
    assert!(store::cancelled(&pool, next.id, next.owner_id, false)
        .await
        .unwrap());
    assert!(store::renew(&pool, &[a.id], &[next.owner_id], false)
        .await
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn ownership_renewal_extends_the_database_deadline_without_reissuing() {
    let instance = setup_test_db(ImportMode::None).await.unwrap();
    let pool = PgPool::connect(instance.db_url.as_str()).await.unwrap();
    let id = store::ensure_requested(
        &pool,
        1,
        "host.1",
        &args(10, Some(20), false),
        Some(100),
        None,
        false,
    )
    .await
    .unwrap()
    .unwrap();
    let old = store::take_request_ownership(&pool, 1, "host.1", false)
        .await
        .unwrap()
        .remove(0);
    assert!(store::requested(&pool, id, old.owner_id, false)
        .await
        .unwrap());
    let deadline_sql = "SELECT (EXTRACT(EPOCH FROM lease_until) * 1000000)::bigint FROM host_consumer_catchups WHERE id = $1";
    let before: i64 = sqlx::query_scalar(deadline_sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();
    let refreshed = store::renew(&pool, &[id], &[old.owner_id], false)
        .await
        .unwrap();
    let after: i64 = sqlx::query_scalar(deadline_sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(before, after, "an early poll must not extend the lease");
    assert_eq!(
        refreshed.len(),
        1,
        "valid ownership is still returned between renewals"
    );
    assert!(!refreshed[0].request_pending);
    sqlx::query("UPDATE host_consumer_catchups SET lease_until = clock_timestamp() + interval '10 seconds' WHERE id = $1")
        .bind(id).execute(&pool).await.unwrap();
    let renewed = store::renew(&pool, &[id], &[old.owner_id], false)
        .await
        .unwrap();
    assert_eq!(renewed.len(), 1);
    assert_eq!(renewed[0].owner_id, old.owner_id);
    assert!(
        !renewed[0].request_pending,
        "heartbeat must not republish work"
    );
    let extended: bool = sqlx::query_scalar("SELECT lease_until > clock_timestamp() + interval '50 seconds' FROM host_consumer_catchups WHERE id = $1")
        .bind(id).fetch_one(&pool).await.unwrap();
    assert!(extended);
    assert!(store::completed(&pool, 1, "host.1", id, false)
        .await
        .unwrap());
    assert!(store::take_request_ownership(&pool, 1, "host.1", false)
        .await
        .unwrap()
        .is_empty());
}

#[derive(sqlx::FromRow)]
struct PendingCommand {
    id: uuid::Uuid,
    block_start: i64,
    block_end: i64,
    cancel_pending: bool,
}

async fn pending_commands(
    pool: &PgPool,
    chain: i64,
    consumer: &str,
    resend: bool,
) -> Result<Vec<PendingCommand>, sqlx::Error> {
    sqlx::query_as("SELECT id, block_start, block_end, cancel_pending FROM host_consumer_catchups
        WHERE chain_id = $1 AND consumer_id = $2
          AND ((status = 'ONGOING' AND ($3 OR request_pending)) OR cancel_pending)
        ORDER BY cancel_pending DESC, created_at, id")
        .bind(chain).bind(consumer).bind(resend).fetch_all(pool).await
}

async fn owner_for(pool: &PgPool, id: uuid::Uuid) -> uuid::Uuid {
    let (chain, consumer, existing): (i64, String, Option<uuid::Uuid>) = sqlx::query_as(
        "SELECT chain_id, consumer_id, owner_id FROM host_consumer_catchups WHERE id = $1")
        .bind(id).fetch_one(pool).await.unwrap();
    if let Some(owner) = existing {
        return owner;
    }
    store::take_request_ownership(pool, chain, &consumer, false)
        .await
        .unwrap()
        .into_iter()
        .find(|r| r.id == id)
        .unwrap()
        .owner_id
}

#[tokio::test]
async fn completion_is_scoped_idempotent_and_does_not_override_suppression() {
    let instance = setup_test_db(ImportMode::None).await.unwrap();
    let pool = PgPool::connect(instance.db_url.as_str()).await.unwrap();
    let mut ids = Vec::new();
    for start in [10, 30] {
        ids.push(
            store::ensure_requested(
                &pool,
                1,
                "host.1",
                &args(start, Some(start + 10), true),
                Some(100),
                None,
                false,
            )
            .await
            .unwrap()
            .unwrap(),
        );
    }
    let (a, b) = (ids[0], ids[1]);
    let owner = store::take_request_ownership(&pool, 1, "host.1", false)
        .await
        .unwrap()[0]
        .owner_id;
    assert_eq!(
        store::block_end(&pool, 1, "host.1", a).await.unwrap(),
        Some(20)
    );
    assert_eq!(store::block_end(&pool, 2, "host.1", a).await.unwrap(), None);
    assert_eq!(store::block_end(&pool, 1, "other", a).await.unwrap(), None);
    // Ingestion can finish on a replica which does not hold a live command lease.
    sqlx::query("UPDATE host_consumer_catchups SET lease_until = clock_timestamp() - interval '1 second' WHERE owner_id = $1")
        .bind(owner).execute(&pool).await.unwrap();
    store::completed(&pool, 2, "host.1", a, false)
        .await
        .unwrap();
    store::completed(&pool, 1, "other", a, false).await.unwrap();
    assert_eq!(
        pending_commands(&pool, 1, "host.1", true)
            .await
            .unwrap()
            .len(),
        2
    );
    for _ in 0..2 {
        store::completed(&pool, 1, "host.1", a, false)
            .await
            .unwrap();
    }
    let (status, pending): (String, bool) = sqlx::query_as(
        "SELECT status, request_pending FROM host_consumer_catchups WHERE id = $1",
    )
    .bind(a)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!((status.as_str(), pending), ("DONE", false));
    for resend in [false, true] {
        let commands =
            pending_commands(&pool, 1, "host.1", resend).await.unwrap();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].id, b, "only ongoing requests are resumed");
    }
    assert!(
        store::accepts(&pool, 1, "host.1", Some(a), true)
            .await
            .unwrap(),
        "completion must allow earlier deliveries to drain"
    );
    store::ensure_requested(
        &pool,
        1,
        "host.1",
        &args(50, Some(60), false),
        Some(100),
        None,
        false,
    )
    .await
    .unwrap()
    .unwrap();
    store::completed(&pool, 1, "host.1", b, false)
        .await
        .unwrap();
    let (status, pending): (String, bool) = sqlx::query_as(
        "SELECT status, cancel_pending FROM host_consumer_catchups WHERE id = $1",
    )
    .bind(b)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!((status.as_str(), pending), ("SUPPRESSED", true));
    assert!(!store::accepts(&pool, 1, "host.1", Some(b), true)
        .await
        .unwrap());
}

#[tokio::test]
async fn suppression_waits_for_ingestion_and_prechecked_old_blocks_cannot_commit(
) {
    use alloy::primitives::{Address, B256};
    use fhevm_engine_common::chain_id::ChainId;
    use host_listener::{
        cmd::block_history::BlockSummary,
        database::{
            ingest::{
                ingest_block_logs_in_transaction, BlockLogs, IngestOptions,
            },
            tfhe_event_propagate::Database,
        },
    };

    let instance = setup_test_db(ImportMode::None).await.unwrap();
    let pool = PgPool::connect(instance.db_url.as_str()).await.unwrap();
    let id = store::ensure_requested(
        &pool,
        1,
        "host.1",
        &args(10, Some(20), false),
        Some(100),
        None,
        false,
    )
    .await
    .unwrap()
    .unwrap();
    let mut in_flight = pool.begin().await.unwrap();
    sqlx::query(
        "SELECT status FROM host_consumer_catchups WHERE id = $1 FOR SHARE",
    )
    .bind(id)
    .execute(&mut *in_flight)
    .await
    .unwrap();
    let other_pool = pool.clone();
    let mut replacement = tokio::spawn(async move {
        store::ensure_requested(
            &other_pool,
            1,
            "host.1",
            &args(30, Some(40), false),
            Some(100),
            None,
            false,
        )
        .await
    });
    assert!(
        tokio::time::timeout(
            std::time::Duration::from_millis(100),
            &mut replacement
        )
        .await
        .is_err(),
        "suppression must wait for admitted ingestion"
    );
    in_flight.commit().await.unwrap();
    let new_id = replacement.await.unwrap().unwrap().unwrap();
    store::completed(&pool, 1, "host.1", new_id, false)
        .await
        .unwrap();
    let chain = ChainId::try_from(1u64).unwrap();
    let db = Database::new(&instance.db_url, chain, 16).await.unwrap();
    let options = IngestOptions {
        dependence_by_connexity: false,
        dependence_cross_block: false,
        dependent_ops_max_per_chain: 0,
        is_protocol_config_listener: false,
        disable_synthetic_ops: true,
    };
    for (number, request) in [(31, new_id), (11, id)] {
        let block = BlockLogs {
            logs: vec![],
            summary: BlockSummary {
                number,
                hash: B256::repeat_byte(number as u8),
                parent_hash: B256::ZERO,
                timestamp: 1_700_000_000,
            },
            catchup: false,
            finalized: true,
        };
        let mut tx = db.new_transaction().await.unwrap().unwrap();
        if !store::lock_for_ingest(&mut tx, chain.as_i64(), request)
            .await
            .unwrap()
        {
            continue;
        }
        ingest_block_logs_in_transaction(
            chain,
            &db,
            tx,
            &block,
            &Some(Address::ZERO),
            &Some(Address::repeat_byte(1)),
            &None,
            &None,
            &None,
            options.clone(),
        )
        .await
        .unwrap();
    }
    let blocks: Vec<i64> = sqlx::query_scalar("SELECT block_number FROM host_chain_blocks_valid WHERE chain_id = 1 ORDER BY block_number").fetch_all(&pool).await.unwrap();
    assert_eq!(
        blocks,
        vec![31],
        "the old request is rechecked inside the block transaction"
    );
}

#[tokio::test]
async fn replicas_reuse_the_first_id_and_relative_bounds() {
    let instance = setup_test_db(ImportMode::None).await.unwrap();
    let pool = PgPool::connect(instance.db_url.as_str()).await.unwrap();
    let flags = args(-100, None, false);
    let (a, b) = tokio::join!(
        store::ensure_requested(
            &pool,
            1,
            "host.1",
            &flags,
            Some(1000),
            None,
            false
        ),
        store::ensure_requested(
            &pool,
            1,
            "host.1",
            &flags,
            Some(2000),
            None,
            false
        ),
    );
    let id = a.unwrap().unwrap();
    assert_eq!(Some(id), b.unwrap());
    let existing =
        store::ensure_requested(&pool, 1, "host.1", &flags, None, None, false)
            .await
            .unwrap();
    assert_eq!(existing, Some(id), "resumption needs no live reference");
    let rows = pending_commands(&pool, 1, "host.1", true).await.unwrap();
    assert_eq!(rows.len(), 1);
    assert!(matches!(
        (rows[0].block_start, rows[0].block_end),
        (900, 999) | (1900, 1999)
    ));
    store::requested(&pool, id, owner_for(&pool, id).await, false)
        .await
        .unwrap();
    assert!(pending_commands(&pool, 1, "host.1", false)
        .await
        .unwrap()
        .is_empty());
    let resumed = pending_commands(&pool, 1, "host.1", true).await.unwrap();
    assert_eq!(
        (resumed[0].id, resumed[0].block_start, resumed[0].block_end),
        (id, rows[0].block_start, rows[0].block_end)
    );
    assert!(store::accepts(&pool, 1, "host.1", Some(id), true)
        .await
        .unwrap());
    assert!(!store::accepts(&pool, 2, "host.1", Some(id), true)
        .await
        .unwrap());
    assert!(!store::accepts(&pool, 1, "other", Some(id), true)
        .await
        .unwrap());
    assert!(!store::accepts(&pool, 1, "host.1", None, true)
        .await
        .unwrap());
}

#[tokio::test]
async fn replacement_persists_cancellation_and_no_override_preserves_other_ids()
{
    let instance = setup_test_db(ImportMode::None).await.unwrap();
    let pool = PgPool::connect(instance.db_url.as_str()).await.unwrap();
    let a = store::ensure_requested(
        &pool,
        1,
        "host.1",
        &args(10, Some(20), false),
        Some(100),
        None,
        false,
    )
    .await
    .unwrap()
    .unwrap();
    store::requested(&pool, a, owner_for(&pool, a).await, false)
        .await
        .unwrap();
    let b = store::ensure_requested(
        &pool,
        1,
        "host.1",
        &args(30, Some(40), true),
        Some(100),
        None,
        false,
    )
    .await
    .unwrap()
    .unwrap();
    assert_ne!(a, b);
    assert!(store::accepts(&pool, 1, "host.1", Some(a), true)
        .await
        .unwrap());
    let c = store::ensure_requested(
        &pool,
        1,
        "host.1",
        &args(50, Some(60), false),
        Some(100),
        None,
        false,
    )
    .await
    .unwrap()
    .unwrap();
    let pending = pending_commands(&pool, 1, "host.1", false).await.unwrap();
    assert_eq!(pending.len(), 3);
    assert!(pending[..2].iter().all(|r| r.cancel_pending));
    assert_eq!(pending[2].id, c, "cancel commands precede new replay");
    assert!(!store::accepts(&pool, 1, "host.1", Some(a), true)
        .await
        .unwrap());
    assert!(!store::accepts(&pool, 1, "host.1", Some(b), true)
        .await
        .unwrap());
    // Simulate a crash after publishing a cancel but before recording its ACK.
    assert_eq!(
        pending_commands(&pool, 1, "host.1", false)
            .await
            .unwrap()
            .len(),
        3
    );
    store::cancelled(&pool, a, owner_for(&pool, a).await, false)
        .await
        .unwrap();
    store::cancelled(&pool, b, owner_for(&pool, b).await, false)
        .await
        .unwrap();
    store::requested(&pool, c, owner_for(&pool, c).await, false)
        .await
        .unwrap();
    assert!(pending_commands(&pool, 1, "host.1", false)
        .await
        .unwrap()
        .is_empty());
    assert_eq!(
        pending_commands(&pool, 1, "host.1", true).await.unwrap()[0].id,
        c
    );
    // The same flags after suppression make a new identity; suppression is final.
    let next = store::ensure_requested(
        &pool,
        1,
        "host.1",
        &args(10, Some(20), false),
        Some(200),
        None,
        false,
    )
    .await
    .unwrap()
    .unwrap();
    assert_ne!(a, next);
    assert!(!store::accepts(&pool, 1, "host.1", Some(c), true)
        .await
        .unwrap());
}

#[tokio::test]
async fn waiting_for_a_reference_and_empty_startup_leave_intent_untouched() {
    let instance = setup_test_db(ImportMode::None).await.unwrap();
    let pool = PgPool::connect(instance.db_url.as_str()).await.unwrap();
    let first = store::ensure_requested(
        &pool,
        1,
        "host.1",
        &args(10, Some(20), false),
        Some(100),
        None,
        false,
    )
    .await
    .unwrap()
    .unwrap();
    assert!(store::ensure_requested(
        &pool,
        1,
        "host.1",
        &args(-100, None, false),
        None,
        None,
        false
    )
    .await
    .unwrap()
    .is_none());
    assert!(store::accepts(&pool, 1, "host.1", Some(first), true)
        .await
        .unwrap());
    // A no-flags startup reads commands and never calls ensure_requested.
    assert_eq!(
        pending_commands(&pool, 1, "host.1", true)
            .await
            .unwrap()
            .len(),
        1
    );
    assert!(pending_commands(&pool, 1, "other", true)
        .await
        .unwrap()
        .is_empty());
    let other = store::ensure_requested(
        &pool,
        1,
        "other",
        &args(10, Some(20), false),
        Some(100),
        None,
        false,
    )
    .await
    .unwrap()
    .unwrap();
    assert_ne!(first, other);
    assert!(store::accepts(&pool, 1, "host.1", Some(first), true)
        .await
        .unwrap());
}
