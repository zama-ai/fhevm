use super::containment_test_support::*;
use super::*;
use clap::Parser;
use std::collections::HashSet;
use time::{Date, Month, PrimitiveDateTime, Time};

fn schedule(secs: i64) -> PrimitiveDateTime {
    PrimitiveDateTime::new(
        Date::from_calendar_date(2024, Month::January, 1).unwrap(),
        Time::from_hms(0, 0, 0).unwrap(),
    ) + time::Duration::seconds(secs)
}

fn item(tx: u8, out: u8, dep: u8, secs: i64) -> WorkItem {
    WorkItem {
        output_handle: handle(out),
        dependencies: vec![handle(dep)],
        fhe_operation: SupportedFheOperations::FheNot as i16,
        is_scalar: false,
        is_allowed: true,
        is_error: false,
        error_message: None,
        transaction_id: handle(tx),
        schedule_order: schedule(secs),
        operand_boundary_mask: None,
        dependence_chain_id: None,
        group_id: None,
        output_index: 0,
        output_count: 1,
    }
}

#[test]
fn filter_work_keeps_healthy_siblings_and_lists_empty_transactions() {
    let drifted = HashSet::from([handle(1)]);
    let filtered = frozen_computations::filter_work(
        vec![
            item(10, 2, 1, 1),
            item(10, 3, 99, 1),
            item(11, 4, 1, 2),
            item(11, 5, 4, 2),
        ],
        &drifted,
    );
    assert_eq!(filtered.kept.len(), 1);
    assert_eq!(filtered.kept[0].output_handle, handle(3));
    assert_eq!(filtered.empty_transactions, vec![handle(11)]);
    assert!(filtered.freeze.frozen.contains(&(handle(10), handle(2))));
    assert!(filtered.freeze.frozen.contains(&(handle(11), handle(4))));
    assert!(filtered.freeze.frozen.contains(&(handle(11), handle(5))));
    assert!(!filtered.freeze.frozen.contains(&(handle(10), handle(3))));
    // Two txs, each blocked only by handle 1 (k=1). Internal row 5 does not add a tx.
    assert_eq!(filtered.freeze.weights.get(&handle(1)).copied(), Some(2.0));
    assert_eq!(
        frozen_computations::schedule_filter_counts(4, &filtered),
        frozen_computations::ScheduleFilterCounts {
            dropped_scheduled: 3,
            dropped_transactions: 1,
            dropped_batches: 0,
            transactions_found: 1,
            batches_found: 1,
        }
    );
}

#[test]
fn schedule_filter_counts_empty_pick_is_a_noop() {
    let filtered = frozen_computations::filter_work(vec![], &HashSet::new());
    assert_eq!(
        frozen_computations::schedule_filter_counts(0, &filtered),
        frozen_computations::ScheduleFilterCounts::default()
    );
}

#[test]
fn schedule_filter_counts_fully_frozen_pick_drops_the_batch() {
    let drifted = HashSet::from([handle(1)]);
    let filtered =
        frozen_computations::filter_work(vec![item(10, 2, 1, 1), item(11, 3, 1, 2)], &drifted);
    assert!(filtered.kept.is_empty());
    assert_eq!(
        frozen_computations::schedule_filter_counts(2, &filtered),
        frozen_computations::ScheduleFilterCounts {
            dropped_scheduled: 2,
            dropped_transactions: 2,
            dropped_batches: 1,
            transactions_found: 0,
            batches_found: 1,
        }
    );
}

#[test]
fn filter_work_freezes_across_selected_transactions() {
    let drifted = HashSet::from([handle(1)]);
    let filtered = frozen_computations::filter_work(
        vec![
            item(10, 2, 1, 1),
            item(11, 3, 2, 2),
            item(12, 4, 3, 3),
            item(13, 5, 99, 4),
        ],
        &drifted,
    );
    let kept: HashSet<_> = filtered
        .kept
        .iter()
        .map(|row| row.output_handle.clone())
        .collect();
    assert_eq!(kept, HashSet::from([handle(5)]));
    assert!(filtered.freeze.frozen.contains(&(handle(10), handle(2))));
    assert!(filtered.freeze.frozen.contains(&(handle(11), handle(3))));
    assert!(filtered.freeze.frozen.contains(&(handle(12), handle(4))));
    // Transitive cone: txs 10, 11, 12 are all blocked by handle 1 (k=1).
    assert_eq!(filtered.freeze.weights.get(&handle(1)).copied(), Some(3.0));
}

#[test]
fn unlock_share_splits_a_tx_blocked_by_two_drifted_handles() {
    let drifted = HashSet::from([handle(1), handle(7)]);
    let mut both = item(10, 2, 1, 1);
    both.dependencies = vec![handle(1), handle(7)];
    let filtered = frozen_computations::filter_work(vec![both, item(11, 3, 1, 2)], &drifted);
    assert_eq!(filtered.freeze.weights.get(&handle(1)).copied(), Some(1.5));
    assert_eq!(filtered.freeze.weights.get(&handle(7)).copied(), Some(0.5));
}

#[test]
fn late_blocker_on_the_same_batch_grows_k_and_adds_the_new_handle() {
    let first = frozen_computations::filter_work(
        vec![item(10, 2, 1, 1), item(11, 3, 2, 2)],
        &HashSet::from([handle(1)]),
    );
    assert_eq!(first.freeze.weights.get(&handle(1)).copied(), Some(2.0));
    let mut extra = item(11, 3, 2, 2);
    extra.dependencies = vec![handle(2), handle(8)];
    let second = frozen_computations::filter_work(
        vec![item(10, 2, 1, 1), extra],
        &HashSet::from([handle(1), handle(8)]),
    );
    // tx 10: {1} → 1; tx 11: {1,8} → 0.5 each.
    assert_eq!(second.freeze.weights.get(&handle(1)).copied(), Some(1.5));
    assert_eq!(second.freeze.weights.get(&handle(8)).copied(), Some(0.5));
}

#[tokio::test]
async fn no_unhealed_ct64_skips_filtering() {
    let (_db, pool) = setup().await;
    computation(&pool, 2, 1, 10, true, 90).await;
    let filtered = frozen_computations::containment_filter(&pool, vec![item(10, 2, 1, 1)])
        .await
        .unwrap();
    assert!(filtered.freeze.frozen.is_empty());
    assert_eq!(filtered.kept.len(), 1);
    assert!(filtered.empty_transactions.is_empty());
}

#[tokio::test]
async fn empty_transactions_are_pushed_behind_the_window() {
    let (_db, pool) = setup().await;
    drift(&pool, 1).await;
    computation(&pool, 2, 1, 10, true, 90).await;
    computation(&pool, 3, 99, 11, true, 90).await;
    sqlx::query("UPDATE computations SET schedule_order = $1 WHERE transaction_id = $2")
        .bind(schedule(1))
        .bind(handle(10))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("UPDATE computations SET schedule_order = $1 WHERE transaction_id = $2")
        .bind(schedule(5))
        .bind(handle(11))
        .execute(&pool)
        .await
        .unwrap();
    let mut trx = pool.begin().await.unwrap();
    let filtered =
        frozen_computations::containment_filter(&pool, vec![item(10, 2, 1, 1), item(11, 3, 99, 5)])
            .await
            .unwrap();
    assert_eq!(filtered.empty_transactions, vec![handle(10)]);
    assert!(filtered.freeze.frozen.contains(&(handle(10), handle(2))));
    frozen_computations::penalize_frozen_transactions(
        &mut trx,
        &filtered.empty_transactions,
        schedule(5),
    )
    .await
    .unwrap();
    trx.commit().await.unwrap();
    let bumped: PrimitiveDateTime =
        sqlx::query_scalar("SELECT schedule_order FROM computations WHERE transaction_id = $1")
            .bind(handle(10))
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(bumped, schedule(6));
}

#[tokio::test]
async fn tx_unlock_potential_is_an_ema_visible_before_the_batch_commits() {
    let (_db, pool) = setup().await;
    drift(&pool, 1).await;
    computation(&pool, 2, 1, 10, true, 90).await;
    computation(&pool, 4, 1, 11, true, 90).await;
    computation(&pool, 6, 1, 42, true, 90).await;
    let work = || vec![item(10, 2, 1, 1), item(11, 4, 1, 1), item(42, 6, 1, 1)];
    frozen_computations::containment_filter(&pool, work())
        .await
        .unwrap();
    // Three txs, k=1 each: (0 + 3) / 2 = 1.5, committed on the pool.
    let first: f64 = sqlx::query_scalar(
        "SELECT tx_unlock_potential FROM drifted_handle_demand WHERE handle = $1",
    )
    .bind(handle(1))
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(first, 1.5);
    frozen_computations::containment_filter(&pool, work())
        .await
        .unwrap();
    let second: f64 = sqlx::query_scalar(
        "SELECT tx_unlock_potential FROM drifted_handle_demand WHERE handle = $1",
    )
    .bind(handle(1))
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(second, 2.25);
}

#[tokio::test]
async fn tx_unlock_potential_does_not_lock_drifted_handle() {
    let (_db, pool) = setup().await;
    drift(&pool, 1).await;
    let mut held = pool.begin().await.unwrap();
    sqlx::query("SELECT id FROM drifted_handle WHERE handle = $1 FOR UPDATE")
        .bind(handle(1))
        .fetch_one(held.as_mut())
        .await
        .unwrap();
    tokio::time::timeout(
        std::time::Duration::from_secs(1),
        frozen_computations::containment_filter(&pool, vec![item(10, 2, 1, 1)]),
    )
    .await
    .expect("demand writes must not wait on drifted_handle row locks")
    .unwrap();
    let written: f64 = sqlx::query_scalar(
        "SELECT tx_unlock_potential FROM drifted_handle_demand WHERE handle = $1",
    )
    .bind(handle(1))
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(written, 0.5);
    held.commit().await.unwrap();
}

async fn check_healthy_work_still_runs(disable_locking: bool, adaptive: bool) {
    let (db, pool) = setup().await;
    drift(&pool, 1).await;
    for chain in [90, 91] {
        sqlx::query(
            "INSERT INTO dependence_chain (dependence_chain_id, status) VALUES ($1, 'updated')",
        )
        .bind(handle(chain))
        .execute(&pool)
        .await
        .unwrap();
    }
    for out in 2..8 {
        computation(&pool, out, if out == 3 { 2 } else { 1 }, out, true, 90).await;
    }
    for out in 20..24 {
        computation(&pool, out, 99, out, true, if out < 22 { 90 } else { 91 }).await;
    }
    let mut args = crate::daemon_cli::Args::parse_from([
        "tfhe-worker",
        "--work-items-batch-size",
        "4",
        "--dependence-chains-per-batch",
        "2",
    ]);
    args.database_url = Some(db.db_url().to_owned().into());
    args.disable_dcid_locking = disable_locking;
    args.dcid_adaptive_batch_execution = adaptive;
    let health = crate::health_check::HealthCheck::new(
        db.db_url().to_owned().into(),
        Duration::from_secs(300),
    );
    let mut locks = dependence_chain::LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        30,
        disable_locking,
        None,
        None,
        None,
        3,
    );
    let mut trx = pool.begin().await.unwrap();
    let (nodes, _, more, _) = query_for_work(
        &args,
        &health,
        &mut trx,
        &mut locks,
        &mut 0,
        &mut DeferredTransactionCooldown::new(),
        false,
    )
    .await
    .unwrap();
    assert!(more || !nodes.is_empty());
    let frozen_direct: HashSet<_> = [2, 4, 5, 6, 7].into_iter().map(handle).collect();
    let transactions: HashSet<_> = nodes.iter().map(|n| n.transaction_id.clone()).collect();
    assert!(
        transactions.is_disjoint(&frozen_direct),
        "direct frozen consumers must not be scheduled: {transactions:?}"
    );
    trx.rollback().await.unwrap();
}

#[tokio::test]
#[serial_test::serial(frozen_metrics)]
async fn lockless_keeps_non_frozen_work() {
    check_healthy_work_still_runs(true, false).await;
}

#[tokio::test]
#[serial_test::serial(frozen_metrics)]
async fn locked_keeps_non_frozen_work() {
    check_healthy_work_still_runs(false, false).await;
}

#[tokio::test]
#[serial_test::serial(frozen_metrics)]
async fn adaptive_keeps_non_frozen_work() {
    check_healthy_work_still_runs(false, true).await;
}

#[tokio::test]
async fn frozen_only_chain_rotates_without_completing_pending_work() {
    let (db, pool) = setup().await;
    drift(&pool, 1).await;
    for chain in [90, 91] {
        sqlx::query(
            "INSERT INTO dependence_chain (dependence_chain_id, status, last_updated_at)
            VALUES ($1, 'updated', NOW() + make_interval(secs => $2))",
        )
        .bind(handle(chain))
        .bind(f64::from(chain - 90))
        .execute(&pool)
        .await
        .unwrap();
    }
    computation(&pool, 2, 1, 2, true, 90).await;
    computation(&pool, 3, 99, 3, true, 91).await;
    sqlx::query(
        "UPDATE dependence_chain SET last_updated_at = last_updated_at - INTERVAL '1 minute'",
    )
    .execute(&pool)
    .await
    .unwrap();
    let args = crate::daemon_cli::Args::parse_from([
        "tfhe-worker",
        "--work-items-batch-size",
        "1",
        "--dependence-chains-per-batch",
        "1",
    ]);
    let health = crate::health_check::HealthCheck::new(
        db.db_url().to_owned().into(),
        Duration::from_secs(300),
    );
    let mut locks = dependence_chain::LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        30,
        false,
        None,
        None,
        None,
        3,
    );
    let mut cooldown = DeferredTransactionCooldown::new();
    let mut cycles = 0;
    let mut trx = pool.begin().await.unwrap();
    let (nodes, _, _, _) = query_for_work(
        &args,
        &health,
        &mut trx,
        &mut locks,
        &mut cycles,
        &mut cooldown,
        false,
    )
    .await
    .unwrap();
    assert!(nodes.is_empty() || nodes[0].transaction_id == handle(3));
    trx.commit().await.unwrap();
    let pending: bool = sqlx::query_scalar(
        "SELECT NOT is_completed AND NOT is_error FROM computations WHERE output_handle = $1",
    )
    .bind(handle(2))
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(pending);
}

#[tokio::test]
async fn healthy_sibling_in_a_partly_frozen_transaction_is_selected() {
    let (db, pool) = setup().await;
    drift(&pool, 1).await;
    computation(&pool, 2, 1, 10, true, 90).await;
    computation(&pool, 3, 99, 10, true, 90).await;
    let mut args =
        crate::daemon_cli::Args::parse_from(["tfhe-worker", "--work-items-batch-size", "1"]);
    args.disable_dcid_locking = true;
    let health = crate::health_check::HealthCheck::new(
        db.db_url().to_owned().into(),
        Duration::from_secs(300),
    );
    let mut locks = dependence_chain::LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        30,
        true,
        None,
        None,
        None,
        3,
    );
    let mut trx = pool.begin().await.unwrap();
    let (nodes, _, _, _) = query_for_work(
        &args,
        &health,
        &mut trx,
        &mut locks,
        &mut 0,
        &mut DeferredTransactionCooldown::new(),
        false,
    )
    .await
    .unwrap();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].results, vec![handle(3)]);
}
