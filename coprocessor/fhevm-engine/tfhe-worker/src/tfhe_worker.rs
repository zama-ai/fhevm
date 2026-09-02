use crate::dependence_chain::{self};
use crate::types::CoprocessorError;
use fhevm_engine_common::database::{
    apply_gcs_mode_search_path, connect_pool_with_options,
    connect_pool_with_options_and_connect_options, resolve_database_url_from_option,
};
use fhevm_engine_common::db_keys::DbKeyCache;
use fhevm_engine_common::gcs_activation::{
    run_gcs_activation_watcher, GCS_GATE_RECHECK, GCS_NOT_ACTIVATED, WORK_AVAILABLE_CHANNEL,
};
use fhevm_engine_common::telemetry;
use fhevm_engine_common::tfhe_ops::check_fhe_operand_types;
use fhevm_engine_common::types::{FhevmError, Handle, SupportedFheCiphertexts};
use fhevm_engine_common::versioning::{GcsRollbackPolicy, WriteGuard};
use fhevm_engine_common::{tfhe_ops::current_ciphertext_version, types::SupportedFheOperations};
use itertools::Itertools;
use lazy_static::lazy_static;
use prometheus::{
    register_histogram, register_int_counter, register_int_gauge, Histogram, IntCounter, IntGauge,
};
#[cfg(feature = "gpu")]
use scheduler::dfg::scheduler::GpuExecutionLimiter;
use scheduler::dfg::types::{CompressedCiphertext, DFGTxInput, SchedulerError};
use scheduler::dfg::{build_component_nodes, ComponentNode, DFComponentGraph, DFGOp};
use scheduler::dfg::{scheduler::Scheduler, types::DFGTaskInput};
use sqlx::types::Uuid;
use sqlx::{postgres::PgListener, query, query_scalar, Postgres};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use time::PrimitiveDateTime;
use tracing::{debug, error, info, warn, Instrument};

const EVENT_CIPHERTEXT_COMPUTED: &str = "event_ciphertext_computed";

#[cfg(feature = "gpu")]
/// Every GPU memory reservation failure is retryable, without limit.
///
/// None of the four variants is a statement about the operands: TimedOut and
/// Cancelled are pressure and shutdown; UnknownDevice and AccountingOverflow
/// are deployment and bookkeeping faults. Stamping any of them would make a
/// computation permanently unexecutable — and, through the dead-producer
/// drain, condemn everything downstream of it — for a condition the inputs
/// did not cause and that a restart or a quieter device would clear.
///
/// These leave no stamp at all, so they also do not consume the panic retry
/// budget: an operator can starve a device indefinitely without losing work.
fn is_retryable_gpu_reservation_error(error: &FhevmError) -> bool {
    matches!(error, FhevmError::GpuMemoryReservationError(_))
}

struct WorkItem {
    output_handle: Vec<u8>,
    dependencies: Vec<Vec<u8>>,
    fhe_operation: i16,
    is_scalar: bool,
    is_allowed: bool,
    /// Terminal error stamp. Errored rows are loaded (the transaction-id
    /// expansion needs the full transaction) but — unless the stamp is
    /// retryable, see [`stamp_is_retryable`] — never re-executed: their
    /// bytes can never exist, and their transaction-local consumers drain
    /// with them (see `prepare_transaction_ops`).
    is_error: bool,
    /// Set alongside `is_error`; consulted only to classify the stamp
    /// (retryable panic vs deterministic error).
    error_message: Option<String>,
    transaction_id: Vec<u8>,
    schedule_order: PrimitiveDateTime,
    /// Authoritative, listener-derived executor `boundaryBits`. Nullable at
    /// the schema level for rows written by a pre-mask listener; for those
    /// legacy rows the worker falls back to the pre-mask inference (an
    /// operand is transaction-local iff this transaction produced it).
    operand_boundary_mask: Option<Vec<u8>>,
    /// The chain this row is filed under. May differ from the locked chain
    /// for rows retained from a fork sibling (`ON CONFLICT .. DO NOTHING`);
    /// such rows are loaded as recompute-only producers, never as work.
    dependence_chain_id: Option<Vec<u8>>,
}

const OPERAND_BOUNDARY_MASK_BYTES: usize = 32;

/// Share of the DCID lease a GPU memory reservation may spend waiting.
///
/// Below 1.0 so a worker that exhausts the wait still gets back to its loop
/// and extends the lease before it expires, rather than losing it by the
/// width of the timeout itself.
pub(crate) const GPU_RESERVATION_LEASE_FRACTION: f32 = 0.8;

/// Marker that makes a stamp RETRYABLE rather than terminal.
///
/// A terminal stamp is a permanent verdict: the work window never re-selects
/// the row, its chain retires without it, and the dead-producer drain
/// condemns everything downstream. That is only correct when the failure is a
/// deterministic property of the operands. Two classes are not:
///
///   * `ExecutionPanic` — a panic caught around an FHE op, its output
///     compression, or a boundary decompression. Can be device or allocation
///     pressure rather than anything about the inputs.
///   * `DecompressionError` — persisted bytes that would not expand. Corrupt
///     bytes are deterministic and must eventually drain, but a transient
///     device fault looks identical at this point, so the first failure must
///     not be the last word.
///
/// Both are stamped visibly, re-selected by the work window, and healed by a
/// later success. Repeated failures DEMOTE the row to the slow lane at
/// `--computation-retry-demote-threshold` — they never promote it to
/// terminal. A retryable stamp is a statement about the failure, not about
/// the operands, so no amount of patience running out turns it into a
/// verdict.
///
/// Substring matching is admittedly coarse, and it is now load-bearing in
/// five places (this constant, the two work-window queries,
/// `release_completed_locks` and [`DEAD_PRODUCER_PREDICATE`]). A dedicated
/// error-class column is the follow-up; until then, every one of those five
/// sites must be changed together.
pub(crate) const RETRYABLE_STAMP_MARKER: &str = "RETRYABLE";

fn stamp_is_retryable(error_message: Option<&str>) -> bool {
    error_message.is_some_and(|m| m.contains(RETRYABLE_STAMP_MARKER))
}

fn dcid_transaction_share(
    work_items_batch_size: i32,
    acquired_dcid_count: usize,
    adaptive_batch_execution: bool,
) -> i32 {
    if !adaptive_batch_execution || acquired_dcid_count <= 1 {
        return work_items_batch_size;
    }
    let dcid_count = i32::try_from(acquired_dcid_count).unwrap_or(i32::MAX);
    work_items_batch_size.saturating_add(dcid_count - 1) / dcid_count
}

fn operand_is_boundary(
    mask: Option<&[u8]>,
    operand_index: usize,
) -> Result<bool, CoprocessorError> {
    let mask = mask.ok_or_else(|| {
        CoprocessorError::Other(
            std::io::Error::other(
                "refusing computation without authoritative operand boundary mask",
            )
            .into(),
        )
    })?;
    if mask.len() != OPERAND_BOUNDARY_MASK_BYTES {
        return Err(CoprocessorError::Other(
            std::io::Error::other(format!(
                "invalid operand boundary mask length {}; expected {OPERAND_BOUNDARY_MASK_BYTES}",
                mask.len()
            ))
            .into(),
        ));
    }
    if operand_index >= OPERAND_BOUNDARY_MASK_BYTES * 8 {
        return Err(CoprocessorError::Other(
            std::io::Error::other(format!(
                "operand index {operand_index} exceeds executor boundary mask width"
            ))
            .into(),
        ));
    }
    let byte_index = OPERAND_BOUNDARY_MASK_BYTES - 1 - operand_index / 8;
    Ok(mask[byte_index] & (1 << (operand_index % 8)) != 0)
}

#[cfg(test)]
mod operand_boundary_mask_tests {
    use super::*;
    use clap::Parser;
    use sqlx::postgres::PgPoolOptions;
    use test_harness::instance::{setup_test_db, ImportMode};

    #[test]
    fn boundary_mask_is_big_endian_and_null_is_rejected_by_the_low_level_check() {
        // NOTE: only this low-level accessor rejects a missing mask; the
        // worker itself never calls it with None — legacy NULL-mask rows
        // take the derive-on-read fallback in prepare_transaction_ops
        // (operand is transaction-local iff this transaction produced it).
        let mut mask = [0_u8; OPERAND_BOUNDARY_MASK_BYTES];
        mask[OPERAND_BOUNDARY_MASK_BYTES - 1] = 0b10;
        assert!(!operand_is_boundary(Some(&mask), 0).expect("valid mask"));
        assert!(operand_is_boundary(Some(&mask), 1).expect("valid mask"));
        assert!(operand_is_boundary(None, 0).is_err());
        assert!(operand_is_boundary(Some(&mask[..31]), 0).is_err());
    }

    /// 32-byte handle with the FheType byte (index 30) set to euint64 and a
    /// distinguishing id in the last byte.
    fn handle(id: u8) -> Vec<u8> {
        let mut h = vec![0_u8; 32];
        h[30] = 5; // FheUint64
        h[31] = id;
        h
    }

    fn mask_with_bits(bits: &[usize]) -> Vec<u8> {
        let mut mask = vec![0_u8; OPERAND_BOUNDARY_MASK_BYTES];
        for bit in bits {
            mask[OPERAND_BOUNDARY_MASK_BYTES - 1 - bit / 8] |= 1 << (bit % 8);
        }
        mask
    }

    fn work_item(
        output: Vec<u8>,
        deps: Vec<Vec<u8>>,
        mask: Option<Vec<u8>>,
        dcid: Option<Vec<u8>>,
        is_allowed: bool,
    ) -> WorkItem {
        WorkItem {
            output_handle: output,
            dependencies: deps,
            fhe_operation: SupportedFheOperations::FheAdd as i16,
            is_scalar: false,
            is_allowed,
            is_error: false,
            error_message: None,
            transaction_id: vec![0xAA; 32],
            schedule_order: PrimitiveDateTime::MIN,
            operand_boundary_mask: mask,
            dependence_chain_id: dcid,
        }
    }

    fn input_kinds(op: &DFGOp) -> Vec<&'static str> {
        op.inputs
            .iter()
            .map(|i| match i {
                DFGTaskInput::LocalDependence(_) => "local",
                DFGTaskInput::BoundaryDependence(_) => "boundary",
                DFGTaskInput::Value(_) => "value",
                DFGTaskInput::Compressed(..) => "compressed",
            })
            .collect()
    }

    #[test]
    fn errored_local_producer_drains_consumer_and_is_not_reexecuted() {
        let dcid = Some(vec![0xD1; 32]);
        // Producer already stamped is_error in the database: it must never
        // re-enter the ops (re-execution fails identically), and its
        // transaction-local consumer can never obtain the obligated raw
        // bytes, so it drains terminally instead of deferring forever.
        let producer = WorkItem {
            is_error: true,
            ..work_item(
                handle(1),
                vec![],
                Some(mask_with_bits(&[])),
                dcid.clone(),
                true,
            )
        };
        // operand 0 = handle(1): transaction-local (bit clear);
        // operand 1 = handle(9): boundary (bit set), sourced from DB.
        let consumer = work_item(
            handle(2),
            vec![handle(1), handle(9)],
            Some(mask_with_bits(&[1])),
            dcid.clone(),
            true,
        );
        let txwork = vec![producer, consumer];
        let prepared =
            prepare_transaction_ops(&txwork, dcid.map(|d| vec![d]).as_deref(), &HashSet::new())
                .unwrap();
        assert!(
            prepared.ops.is_empty(),
            "neither the errored producer nor its drained consumer may execute"
        );
        assert_eq!(prepared.invalid_rows.len(), 1);
        assert_eq!(prepared.invalid_rows[0].0, handle(2));
        assert!(prepared.invalid_rows[0].1.contains("terminally errored"));
    }

    #[test]
    fn dead_boundary_input_drains_owned_consumer_at_op_granularity() {
        let dcid = Some(vec![0xD1; 32]);
        // consumer of a dead boundary handle drains terminally; an
        // independent op of the same transaction keeps computing.
        let consumer = work_item(
            handle(3),
            vec![handle(9), handle(8)],
            Some(mask_with_bits(&[0, 1])),
            dcid.clone(),
            true,
        );
        let independent = work_item(
            handle(4),
            vec![handle(7), handle(8)],
            Some(mask_with_bits(&[0, 1])),
            dcid.clone(),
            true,
        );
        let dead: HashSet<Vec<u8>> = [handle(9)].into_iter().collect();
        let txwork = vec![consumer, independent];
        let prepared = prepare_transaction_ops(&txwork, dcid.map(|d| vec![d]).as_deref(), &dead)
            .expect("prepared");
        assert_eq!(prepared.ops.len(), 1, "independent op still executes");
        assert_eq!(prepared.ops[0].output_handle, handle(4));
        assert_eq!(prepared.invalid_rows.len(), 1);
        assert_eq!(prepared.invalid_rows[0].0, handle(3));
        assert!(prepared.invalid_rows[0].1.contains("dead boundary input"));
    }

    #[test]
    fn retryable_panic_stamp_is_reexecuted_not_drained() {
        let dcid = Some(vec![0xD1; 32]);
        // A panic-stamped producer is the one nondeterministic stamp: it
        // re-executes (success would heal it) and must not drain its
        // transaction-local consumer.
        let producer = WorkItem {
            is_error: true,
            error_message: Some(
                "RETRYABLE Coprocessor scheduler error: ExecutionPanic(\"oom\")".to_string(),
            ),
            ..work_item(
                handle(1),
                vec![handle(7), handle(8)],
                Some(mask_with_bits(&[0, 1])),
                dcid.clone(),
                true,
            )
        };
        let consumer = work_item(
            handle(2),
            vec![handle(1), handle(9)],
            Some(mask_with_bits(&[1])),
            dcid.clone(),
            true,
        );
        let txwork = vec![producer, consumer];
        let prepared =
            prepare_transaction_ops(&txwork, dcid.map(|d| vec![d]).as_deref(), &HashSet::new())
                .expect("prepared");
        assert_eq!(
            prepared.ops.len(),
            2,
            "producer retries, consumer schedules"
        );
        assert!(prepared.invalid_rows.is_empty());
    }

    #[test]
    fn null_mask_derives_boundary_bits_from_transaction_membership() {
        let dcid = Some(vec![0xD1; 32]);
        // producer of handle(1) is in the transaction; handle(9) is not.
        let txwork = vec![
            work_item(
                handle(1),
                vec![handle(8), handle(9)],
                None,
                dcid.clone(),
                true,
            ),
            work_item(
                handle(2),
                vec![handle(1), handle(9)],
                None,
                dcid.clone(),
                true,
            ),
        ];
        let prepared =
            prepare_transaction_ops(&txwork, dcid.map(|d| vec![d]).as_deref(), &HashSet::new())
                .expect("prepared");
        assert_eq!(prepared.ops.len(), 2);
        let consumer = prepared
            .ops
            .iter()
            .find(|op| op.output_handle == handle(2))
            .expect("consumer op");
        // handle(1) is produced by this transaction -> local; handle(9) is
        // not -> canonical persisted form.
        assert_eq!(input_kinds(consumer), ["local", "boundary"]);
    }

    #[test]
    fn invalid_content_is_terminal_not_deferred() {
        let dcid = Some(vec![0xD1; 32]);
        let txwork = vec![work_item(
            handle(1),
            vec![handle(8), handle(9)],
            Some(mask_with_bits(&[0, 1])),
            dcid.clone(),
            true,
        )];
        let mut bad_op = work_item(
            handle(2),
            vec![],
            Some(mask_with_bits(&[])),
            dcid.clone(),
            true,
        );
        bad_op.fhe_operation = 127; // unknown opcode
        let mut txwork = txwork;
        txwork.push(bad_op);
        let prepared =
            prepare_transaction_ops(&txwork, dcid.map(|d| vec![d]).as_deref(), &HashSet::new())
                .expect("prepared");
        // The valid row still schedules; the invalid one is reported for a
        // terminal error, exactly like the executor's own validation.
        assert_eq!(prepared.ops.len(), 1);
        assert_eq!(prepared.invalid_rows.len(), 1);
        assert_eq!(prepared.invalid_rows[0].0, handle(2));
    }

    #[test]
    fn invalid_producer_drains_its_local_consumers() {
        let dcid = Some(vec![0xD1; 32]);
        let mut bad_producer = work_item(
            handle(1),
            vec![],
            Some(mask_with_bits(&[])),
            dcid.clone(),
            true,
        );
        bad_producer.fhe_operation = 127; // unknown opcode
        let txwork = vec![
            bad_producer,
            // bit-0 consumer of the invalid producer: can never obtain the
            // obligated raw bytes, so it must drain with it.
            work_item(
                handle(2),
                vec![handle(1), handle(9)],
                Some(mask_with_bits(&[1])),
                dcid.clone(),
                true,
            ),
            // transitively blocked second-level consumer
            work_item(
                handle(3),
                vec![handle(2), handle(9)],
                Some(mask_with_bits(&[1])),
                dcid.clone(),
                true,
            ),
            // independent op: still schedulable
            work_item(
                handle(4),
                vec![handle(8), handle(9)],
                Some(mask_with_bits(&[0, 1])),
                dcid.clone(),
                true,
            ),
        ];
        let prepared =
            prepare_transaction_ops(&txwork, dcid.map(|d| vec![d]).as_deref(), &HashSet::new())
                .expect("prepared");
        assert_eq!(prepared.ops.len(), 1);
        assert_eq!(prepared.ops[0].output_handle, handle(4));
        let mut errored: Vec<Vec<u8>> = prepared
            .invalid_rows
            .iter()
            .map(|(h, _)| h.clone())
            .collect();
        errored.sort();
        let mut expected = [handle(1), handle(2), handle(3)];
        expected.sort();
        assert_eq!(errored, expected, "producer and both consumers drain");
    }

    #[test]
    fn foreign_invalid_content_defers_instead_of_erroring() {
        let ours = vec![0xD1; 32];
        let theirs = vec![0xD2; 32];
        let mut foreign_bad = work_item(
            handle(1),
            vec![],
            Some(mask_with_bits(&[])),
            Some(theirs),
            true,
        );
        // Content judgments are binary-relative during rolling upgrades: a
        // foreign row we cannot parse must never be stamped is_error.
        foreign_bad.fhe_operation = 127;
        let txwork = vec![
            foreign_bad,
            work_item(
                handle(2),
                vec![handle(1), handle(9)],
                Some(mask_with_bits(&[1])),
                Some(ours.clone()),
                true,
            ),
        ];
        assert!(prepare_transaction_ops(
            &txwork,
            Some(std::slice::from_ref(&ours)),
            &HashSet::new()
        )
        .is_err());
    }

    #[test]
    fn malformed_mask_defers_the_transaction() {
        let dcid = Some(vec![0xD1; 32]);
        let txwork = vec![work_item(
            handle(1),
            vec![handle(8), handle(9)],
            Some(vec![0_u8; 31]), // wrong length
            dcid.clone(),
            true,
        )];
        assert!(prepare_transaction_ops(
            &txwork,
            dcid.map(|d| vec![d]).as_deref(),
            &HashSet::new()
        )
        .is_err());
    }

    #[test]
    fn foreign_local_producer_joins_as_recompute_only() {
        let ours = vec![0xD1; 32];
        let theirs = vec![0xD2; 32];
        // handle(1): retained fork-sibling row (other chain), itself
        // consuming two persisted operands. handle(2): our consumer, taking
        // handle(1) as a transaction-local (bit 0) operand.
        let txwork = vec![
            work_item(
                handle(1),
                vec![handle(8), handle(9)],
                Some(mask_with_bits(&[0, 1])),
                Some(theirs),
                true,
            ),
            work_item(
                handle(2),
                vec![handle(1), handle(9)],
                Some(mask_with_bits(&[1])),
                Some(ours.clone()),
                true,
            ),
        ];
        let prepared =
            prepare_transaction_ops(&txwork, Some(std::slice::from_ref(&ours)), &HashSet::new())
                .expect("prepared");
        assert_eq!(prepared.ops.len(), 2);
        let foreign = prepared
            .ops
            .iter()
            .find(|op| op.output_handle == handle(1))
            .expect("foreign producer joined the graph");
        // Recompute-only: never eligible for results or persistence, even
        // though its own row is allowed.
        assert!(!foreign.is_allowed);
        assert_eq!(input_kinds(foreign), ["boundary", "boundary"]);
        let ours_op = prepared
            .ops
            .iter()
            .find(|op| op.output_handle == handle(2))
            .expect("owned consumer");
        assert!(ours_op.is_allowed);
        assert_eq!(input_kinds(ours_op), ["local", "boundary"]);
        // Foreign rows never anchor the batch's schedule order.
        assert_eq!(
            prepared.earliest_owned_allowed,
            Some(PrimitiveDateTime::MIN)
        );
    }

    #[test]
    fn unneeded_foreign_rows_stay_out_of_the_graph() {
        let ours = vec![0xD1; 32];
        let theirs = vec![0xD2; 32];
        let txwork = vec![
            // Fork-sibling row nothing owned depends on.
            work_item(
                handle(1),
                vec![handle(8), handle(9)],
                Some(mask_with_bits(&[0, 1])),
                Some(theirs),
                true,
            ),
            work_item(
                handle(2),
                vec![handle(8), handle(9)],
                Some(mask_with_bits(&[0, 1])),
                Some(ours.clone()),
                true,
            ),
        ];
        let prepared =
            prepare_transaction_ops(&txwork, Some(std::slice::from_ref(&ours)), &HashSet::new())
                .expect("prepared");
        assert_eq!(prepared.ops.len(), 1);
        assert_eq!(prepared.ops[0].output_handle, handle(2));
    }

    async fn seed_computation(
        pool: &sqlx::PgPool,
        output_handle: &[u8],
        dependencies: Vec<Vec<u8>>,
        mask: Vec<u8>,
        dcid: Option<&[u8]>,
        transaction_id: &[u8],
        schedule_age_secs: i64,
    ) {
        sqlx::query(
            r#"
            INSERT INTO computations (
                output_handle, dependencies, fhe_operation, is_scalar,
                dependence_chain_id, transaction_id, is_allowed,
                schedule_order, host_chain_id, operand_boundary_mask
            ) VALUES ($1, $2, $3, false, $4, $5, true,
                      NOW() - make_interval(secs => $6), 1, $7)
            "#,
        )
        .bind(output_handle)
        .bind(dependencies)
        .bind(SupportedFheOperations::FheNot as i16)
        .bind(dcid)
        .bind(transaction_id)
        .bind(schedule_age_secs as f64)
        .bind(mask)
        .execute(pool)
        .await
        .expect("seed computation");
    }

    fn test_args(batch_size: &str, db_url: &str) -> crate::daemon_cli::Args {
        use clap::Parser;
        let mut args = crate::daemon_cli::Args::parse_from([
            "tfhe-worker",
            "--work-items-batch-size",
            batch_size,
            // One chain per batch: these tests assert per-chain fairness,
            // which multi-chain batching would fold into a single window.
            "--dependence-chains-per-batch",
            "1",
        ]);
        args.database_url = Some(db_url.to_owned().into());
        args
    }

    /// Fairness regression: a chain whose whole window defers must not stay
    /// at the front of oldest-first acquisition. query_for_work rotates it
    /// to the FIFO back (status 'updated', fresh last_updated_at, no
    /// worker_id), so the next acquisition reaches the younger chain, the
    /// deferring rows stay unstamped, and the chain remains immediately
    /// reachable by a listener re-arm or the escalation path.
    #[tokio::test]
    async fn deferring_chain_rotates_behind_younger_chains(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_db(ImportMode::None).await?;
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(db.db_url())
            .await?;

        let old_chain = vec![0x11_u8; 32];
        let young_chain = vec![0x22_u8; 32];
        for (chain, age_secs) in [(&old_chain, 120_f64), (&young_chain, 1_f64)] {
            sqlx::query(
                "INSERT INTO dependence_chain (dependence_chain_id, status, last_updated_at)
                 VALUES ($1, 'updated', NOW() - make_interval(secs => $2))",
            )
            .bind(chain)
            .bind(age_secs)
            .execute(&pool)
            .await?;
        }
        // Older chain: a bit-0 (transaction-local) consumer whose producer
        // row does not exist — the dangling-local-producer state the code
        // classifies as uninterpretable — so its whole window defers.
        // Younger chain: a valid boundary consumer.
        seed_computation(
            &pool,
            &handle(0x51),
            vec![handle(0x50)],
            mask_with_bits(&[]),
            Some(&old_chain),
            &[0xA1_u8; 32],
            120,
        )
        .await;
        seed_computation(
            &pool,
            &handle(0x61),
            vec![handle(0x60)],
            mask_with_bits(&[0]),
            Some(&young_chain),
            &[0xA2_u8; 32],
            1,
        )
        .await;

        let args = test_args("10", db.db_url());
        let health_check = crate::health_check::HealthCheck::new(
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
        let mut no_progress_cycles = 0;
        let mut cooldown = DeferredTransactionCooldown::new();

        let mut trx = pool.begin().await?;
        let (nodes, _, more) = query_for_work(
            &args,
            &health_check,
            &mut trx,
            &mut locks,
            &mut no_progress_cycles,
            &mut cooldown,
            false,
        )
        .await?;
        trx.commit().await?;
        assert!(more, "drained batch still reports work available");
        assert!(nodes.is_empty(), "older chain's window fully defers");

        // Rotated, not wedged or destroyed: the older chain is back to
        // 'updated' with no owner and a fresh FIFO position, and its rows
        // carry no terminal stamps.
        let (status, owner_cleared): (String, bool) = sqlx::query_as(
            "SELECT status, worker_id IS NULL FROM dependence_chain
             WHERE dependence_chain_id = $1",
        )
        .bind(&old_chain)
        .fetch_one(&pool)
        .await?;
        assert_eq!(status, "updated");
        assert!(owner_cleared, "rotation releases ownership");
        let is_error: bool =
            sqlx::query_scalar("SELECT is_error FROM computations WHERE output_handle = $1")
                .bind(handle(0x51))
                .fetch_one(&pool)
                .await?;
        assert!(!is_error, "deferred rows are never terminally stamped");

        // The very next acquisition reaches the younger chain.
        let mut trx = pool.begin().await?;
        let (nodes, _, _) = query_for_work(
            &args,
            &health_check,
            &mut trx,
            &mut locks,
            &mut no_progress_cycles,
            &mut cooldown,
            false,
        )
        .await?;
        trx.commit().await?;
        assert!(
            !nodes.is_empty(),
            "younger chain schedules instead of starving behind the parked one"
        );
        let status: String = sqlx::query_scalar(
            "SELECT status FROM dependence_chain WHERE dependence_chain_id = $1",
        )
        .bind(&young_chain)
        .fetch_one(&pool)
        .await?;
        assert_eq!(status, "processing");
        Ok(())
    }

    /// A drain-only window (every row terminally stamped, nothing deferred)
    /// must NOT rotate: the chain stays owned, the next cycle finds the
    /// window empty and retires it through the normal no-work path.
    #[tokio::test]
    async fn drain_only_chain_retires_without_rotation() -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_db(ImportMode::None).await?;
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(db.db_url())
            .await?;

        let chain = vec![0x33_u8; 32];
        sqlx::query(
            "INSERT INTO dependence_chain (dependence_chain_id, status) VALUES ($1, 'updated')",
        )
        .bind(&chain)
        .execute(&pool)
        .await?;
        // Unknown opcode: deterministically invalid content, terminally
        // stamped rather than deferred.
        sqlx::query(
            r#"
            INSERT INTO computations (
                output_handle, dependencies, fhe_operation, is_scalar,
                dependence_chain_id, transaction_id, is_allowed,
                host_chain_id, operand_boundary_mask
            ) VALUES ($1, '{}', 127, false, $2, $3, true, 1, $4)
            "#,
        )
        .bind(handle(0x91))
        .bind(&chain)
        .bind([0xC1_u8; 32])
        .bind(vec![0_u8; 32])
        .execute(&pool)
        .await?;

        let args = test_args("10", db.db_url());
        let health_check = crate::health_check::HealthCheck::new(
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
        let mut no_progress_cycles = 0;
        let mut cooldown = DeferredTransactionCooldown::new();

        let mut trx = pool.begin().await?;
        let (nodes, _, more) = query_for_work(
            &args,
            &health_check,
            &mut trx,
            &mut locks,
            &mut no_progress_cycles,
            &mut cooldown,
            false,
        )
        .await?;
        trx.commit().await?;
        assert!(more && nodes.is_empty(), "window drains terminally");
        let is_error: bool =
            sqlx::query_scalar("SELECT is_error FROM computations WHERE output_handle = $1")
                .bind(handle(0x91))
                .fetch_one(&pool)
                .await?;
        assert!(is_error, "invalid content is stamped, not deferred");
        let status: String = sqlx::query_scalar(
            "SELECT status FROM dependence_chain WHERE dependence_chain_id = $1",
        )
        .bind(&chain)
        .fetch_one(&pool)
        .await?;
        assert_eq!(
            status, "processing",
            "drain-only keeps ownership for retirement"
        );

        // Next cycle: nothing pending -> the no-work path retires the chain.
        let mut trx = pool.begin().await?;
        let (nodes, _, more) = query_for_work(
            &args,
            &health_check,
            &mut trx,
            &mut locks,
            &mut no_progress_cycles,
            &mut cooldown,
            false,
        )
        .await?;
        trx.commit().await?;
        assert!(nodes.is_empty() && !more, "empty window reports no work");
        Ok(())
    }

    /// Fairness regression for the lockless fallback: the deferred
    /// transaction enters the cooldown, so the next window slice reaches
    /// the younger transaction instead of repeating the same batch forever.
    #[tokio::test]
    async fn cooldown_rotates_deferring_transactions_when_locking_disabled(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_db(ImportMode::None).await?;
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(db.db_url())
            .await?;

        // Older transaction: dangling bit-0 producer -> defers.
        seed_computation(
            &pool,
            &handle(0x71),
            vec![handle(0x70)],
            mask_with_bits(&[]),
            None,
            &[0xB1_u8; 32],
            120,
        )
        .await;
        seed_computation(
            &pool,
            &handle(0x81),
            vec![handle(0x80)],
            mask_with_bits(&[0]),
            None,
            &[0xB2_u8; 32],
            1,
        )
        .await;

        // Window of one: without the cooldown the older, deferring
        // transaction would occupy it on every poll.
        let args = test_args("1", db.db_url());
        let health_check = crate::health_check::HealthCheck::new(
            db.db_url().to_owned().into(),
            Duration::from_secs(300),
        );
        let mut locks = dependence_chain::LockMngr::new_with_conf(
            Uuid::new_v4(),
            pool.clone(),
            30,
            true, // locking disabled
            None,
            None,
            None,
            3,
        );
        let mut no_progress_cycles = 0;
        let mut cooldown = DeferredTransactionCooldown::new();

        let mut trx = pool.begin().await?;
        let (nodes, _, _) = query_for_work(
            &args,
            &health_check,
            &mut trx,
            &mut locks,
            &mut no_progress_cycles,
            &mut cooldown,
            false,
        )
        .await?;
        trx.commit().await?;
        assert!(nodes.is_empty(), "oldest transaction defers");

        let mut trx = pool.begin().await?;
        let (nodes, _, _) = query_for_work(
            &args,
            &health_check,
            &mut trx,
            &mut locks,
            &mut no_progress_cycles,
            &mut cooldown,
            false,
        )
        .await?;
        trx.commit().await?;
        assert!(
            !nodes.is_empty(),
            "cooldown rotates the window to the younger transaction"
        );
        Ok(())
    }

    /// A recurring retryable stamp advances its attempt count without ever
    /// reporting progress and without ever becoming a verdict.
    ///
    /// Both halves matter. Postgres reports a matched row for an
    /// identical-value UPDATE, so if a repeat counted as progress a
    /// perpetually panicking computation would reset `no_progress_cycles`
    /// every cycle and suppress the parking that paces it. And the stamp text
    /// must never be rewritten: exhausting the attempts DEMOTES the row — the
    /// work window and the chain's completion test stop counting it — but the
    /// row stays retryable, stays pending, and is re-armed by the slow sweep.
    /// Nothing here may turn a transient failure into a condemned cone.
    #[tokio::test]
    async fn retryable_stamp_counts_up_without_progress_or_promotion(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_db(ImportMode::None).await?;
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(db.db_url())
            .await?;

        let output_handle = handle(1);
        let transaction_id = handle(2);
        seed_computation(
            &pool,
            &output_handle,
            vec![],
            vec![0_u8; OPERAND_BOUNDARY_MASK_BYTES],
            None,
            &transaction_id,
            0,
        )
        .await;

        let mut locks = dependence_chain::LockMngr::new_with_conf(
            Uuid::new_v4(),
            pool.clone(),
            30,
            true, // locking disabled: this test is about the stamp, not leases
            None,
            None,
            None,
            3,
        );
        let panic_error =
            std::io::Error::other(format!("SchedulerError::{RETRYABLE_STAMP_MARKER}(sigsegv)"));
        let threshold: i16 = 3;

        let mut trx = pool.begin().await?;
        let first = set_computation_error(
            &output_handle,
            &transaction_id,
            &panic_error,
            true,
            &mut trx,
            &mut locks,
        )
        .await?;
        assert!(first, "a fresh stamp is a state change");

        async fn retry_count(
            trx: &mut sqlx::Transaction<'_, Postgres>,
            output_handle: &[u8],
        ) -> Result<i16, sqlx::Error> {
            sqlx::query_scalar::<_, i16>(
                "SELECT error_retry_count FROM computations WHERE output_handle = $1",
            )
            .bind(output_handle)
            .fetch_one(trx.as_mut())
            .await
        }
        assert_eq!(
            retry_count(&mut trx, &output_handle).await?,
            0,
            "a fresh stamp starts at zero"
        );

        // Well past the threshold: the count keeps climbing, the caller is
        // told nothing happened, and the stamp text never changes.
        for attempt in 1..=(threshold + 2) {
            let repeated = set_computation_error(
                &output_handle,
                &transaction_id,
                &panic_error,
                true,
                &mut trx,
                &mut locks,
            )
            .await?;
            assert!(
                !repeated,
                "attempt {attempt}: a recurring panic must never report progress"
            );
            assert_eq!(
                retry_count(&mut trx, &output_handle).await?,
                attempt,
                "attempt {attempt}: the count advances by exactly one"
            );
            let message: Option<String> = sqlx::query_scalar(
                "SELECT error_message FROM computations WHERE output_handle = $1",
            )
            .bind(&output_handle)
            .fetch_one(trx.as_mut())
            .await?;
            assert!(
                message.unwrap().contains(RETRYABLE_STAMP_MARKER),
                "attempt {attempt}: a demoted row is still retryable — nothing \
                 promotes it, so the dead-producer predicate must not see it \
                 as a dead producer"
            );
        }

        trx.rollback().await?;
        Ok(())
    }

    /// A genuinely terminal verdict — e.g. the producer drained as dead —
    /// supersedes a retryable stamp immediately, without waiting out the
    /// budget. Otherwise a row that became provably unsatisfiable would keep
    /// its chain out of 'processed' for the whole cap.
    #[tokio::test]
    async fn terminal_stamp_supersedes_a_retryable_one_mid_budget(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_db(ImportMode::None).await?;
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(db.db_url())
            .await?;

        let output_handle = handle(3);
        let transaction_id = handle(4);
        seed_computation(
            &pool,
            &output_handle,
            vec![],
            vec![0_u8; OPERAND_BOUNDARY_MASK_BYTES],
            None,
            &transaction_id,
            0,
        )
        .await;

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
        let panic_error =
            std::io::Error::other(format!("SchedulerError::{RETRYABLE_STAMP_MARKER}(sigsegv)"));
        let dead_input = std::io::Error::other("dead boundary input: producer terminally errored");

        let mut trx = pool.begin().await?;
        assert!(
            set_computation_error(
                &output_handle,
                &transaction_id,
                &panic_error,
                true,
                &mut trx,
                &mut locks
            )
            .await?
        );
        assert!(
            set_computation_error(
                &output_handle,
                &transaction_id,
                &dead_input,
                false,
                &mut trx,
                &mut locks
            )
            .await?,
            "a terminal verdict lands immediately and is a state change"
        );

        let message: Option<String> =
            sqlx::query_scalar("SELECT error_message FROM computations WHERE output_handle = $1")
                .bind(&output_handle)
                .fetch_one(trx.as_mut())
                .await?;
        assert_eq!(
            message.as_deref(),
            Some("dead boundary input: producer terminally errored")
        );

        trx.rollback().await?;
        Ok(())
    }

    /// The share is `ceil(work_items_batch_size / acquired_dcids)` when
    /// adaptive batching is on, and the whole window when it is off. The
    /// `false` arm is what `--dcid-adaptive-batch-execution=false` selects,
    /// NOT a default: the flag ships enabled, paired with
    /// `--dcid-batch-execution`.
    #[test]
    fn adaptive_dcid_transaction_share_is_bounded_when_enabled() {
        assert_eq!(dcid_transaction_share(100, 20, true), 5);
        assert_eq!(dcid_transaction_share(100, 3, true), 34);
        assert_eq!(dcid_transaction_share(100, 1, true), 100);
        assert_eq!(dcid_transaction_share(100, 20, false), 100);
    }

    #[tokio::test]
    async fn acquired_dcid_excludes_stale_same_transaction_producer_for_boundary_operand(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_db(ImportMode::None).await?;
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(db.db_url())
            .await?;

        let canonical_dcid = vec![0x11_u8; 32];
        let stale_dcid = vec![0x22_u8; 32];
        let transaction_id = vec![0x33_u8; 32];
        let stale_producer = vec![0x44_u8; 32];
        let canonical_consumer = vec![0x55_u8; 32];

        // Only the canonical component is acquired. The stale row models a
        // fork-exposed database retaining the same transaction under a
        // sibling DCID. The consumer's listener-authored bit says the input
        // is a boundary, so it must be sourced from canonical ciphertext
        // storage rather than forwarded from this stale producer.
        sqlx::query(
            "INSERT INTO dependence_chain (dependence_chain_id, status) VALUES ($1, 'updated')",
        )
        .bind(&canonical_dcid)
        .execute(&pool)
        .await?;
        sqlx::query(
            r#"
            INSERT INTO computations (
                output_handle, dependencies, fhe_operation, is_scalar,
                dependence_chain_id, transaction_id, is_allowed,
                is_completed, is_error, host_chain_id, operand_boundary_mask
            ) VALUES ($1, $2, $3, false, $4, $5, $6, false, false, 1, $7)
            "#,
        )
        .bind(&stale_producer)
        .bind(Vec::<Vec<u8>>::new())
        .bind(SupportedFheOperations::FheTrivialEncrypt as i16)
        .bind(&stale_dcid)
        .bind(&transaction_id)
        .bind(true)
        .bind(vec![0_u8; OPERAND_BOUNDARY_MASK_BYTES])
        .execute(&pool)
        .await?;
        let mut boundary_mask = vec![0_u8; OPERAND_BOUNDARY_MASK_BYTES];
        boundary_mask[OPERAND_BOUNDARY_MASK_BYTES - 1] = 1;
        sqlx::query(
            r#"
            INSERT INTO computations (
                output_handle, dependencies, fhe_operation, is_scalar,
                dependence_chain_id, transaction_id, is_allowed,
                is_completed, is_error, host_chain_id, operand_boundary_mask
            ) VALUES ($1, $2, $3, false, $4, $5, true, false, false, 1, $6)
            "#,
        )
        .bind(&canonical_consumer)
        .bind(vec![stale_producer.clone()])
        .bind(SupportedFheOperations::FheNot as i16)
        .bind(&canonical_dcid)
        .bind(&transaction_id)
        .bind(boundary_mask)
        .execute(&pool)
        .await?;

        let mut args = crate::daemon_cli::Args::parse_from([
            "tfhe-worker",
            "--work-items-batch-size",
            "1",
            "--dependence-chains-per-batch",
            "1",
        ]);
        args.database_url = Some(db.db_url.clone());
        let health_check = crate::health_check::HealthCheck::new(
            db.db_url.clone(),
            std::time::Duration::from_secs(300),
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
        let mut no_progress_cycles = 0;
        let mut cooldown = DeferredTransactionCooldown::new();
        let mut transaction = pool.begin().await?;
        let (nodes, _, found_work) = query_for_work(
            &args,
            &health_check,
            &mut transaction,
            &mut locks,
            &mut no_progress_cycles,
            &mut cooldown,
            false,
        )
        .await?;

        assert!(found_work);
        assert_eq!(nodes.len(), 1, "only the acquired DCID may enter the graph");
        assert_eq!(nodes[0].results, vec![canonical_consumer]);
        assert!(
            nodes[0].inputs.contains_key(&stale_producer),
            "the boundary operand must be fetched canonically even though a stale same-transaction producer exists"
        );

        transaction.rollback().await?;
        Ok(())
    }

    #[tokio::test]
    async fn adaptive_batch_shares_the_work_window_across_acquired_dcids(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_db(ImportMode::None).await?;
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(db.db_url())
            .await?;
        let first_dcid = vec![0x61_u8; 32];
        let second_dcid = vec![0x62_u8; 32];
        for dcid in [&first_dcid, &second_dcid] {
            sqlx::query(
                "INSERT INTO dependence_chain (dependence_chain_id, status) VALUES ($1, 'updated')",
            )
            .bind(dcid)
            .execute(&pool)
            .await?;
        }

        let mut expected_outputs = Vec::new();
        for (dcid_index, dcid) in [&first_dcid, &second_dcid].into_iter().enumerate() {
            for transaction_index in 0..2 {
                let output_handle = vec![0x70 + dcid_index as u8, transaction_index as u8];
                let transaction_id = vec![0x80 + dcid_index as u8, transaction_index as u8];
                if transaction_index == 0 {
                    expected_outputs.push(output_handle.clone());
                }
                sqlx::query(
                    r#"
                    INSERT INTO computations (
                        output_handle, dependencies, fhe_operation, is_scalar,
                        dependence_chain_id, transaction_id, is_allowed,
                        is_completed, is_error, host_chain_id, operand_boundary_mask
                    ) VALUES ($1, $2, $3, false, $4, $5, true, false, false, 1, $6)
                    "#,
                )
                .bind(output_handle)
                .bind(vec![vec![1], vec![0]])
                .bind(SupportedFheOperations::FheTrivialEncrypt as i16)
                .bind(dcid)
                .bind(transaction_id)
                .bind(vec![0_u8; OPERAND_BOUNDARY_MASK_BYTES])
                .execute(&pool)
                .await?;
            }
        }

        let mut args = crate::daemon_cli::Args::parse_from([
            "tfhe-worker",
            "--work-items-batch-size",
            "2",
            "--dependence-chains-per-batch",
            "2",
            "--dcid-adaptive-batch-execution",
        ]);
        args.database_url = Some(db.db_url.clone());
        let health_check = crate::health_check::HealthCheck::new(
            db.db_url.clone(),
            std::time::Duration::from_secs(300),
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
        assert_eq!(
            locks
                .acquire_next_locks(2)
                .await?
                .into_iter()
                .filter_map(|(id, _)| id)
                .count(),
            2,
            "both test DCIDs are acquired"
        );
        let mut no_progress_cycles = 0;
        let mut cooldown = DeferredTransactionCooldown::new();
        let mut transaction = pool.begin().await?;
        let (nodes, _, found_work) = query_for_work(
            &args,
            &health_check,
            &mut transaction,
            &mut locks,
            &mut no_progress_cycles,
            &mut cooldown,
            false,
        )
        .await?;

        assert!(found_work);
        assert_eq!(nodes.len(), 2, "one transaction selected from each DCID");
        let actual_outputs = nodes
            .into_iter()
            .flat_map(|node| node.results)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            actual_outputs,
            expected_outputs.into_iter().collect(),
            "the first transaction from both acquired chains is selected"
        );

        transaction.rollback().await?;
        Ok(())
    }
}

lazy_static! {
    pub static ref TIMING: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
}

lazy_static! {
    static ref WORKER_ERRORS_COUNTER: IntCounter =
        register_int_counter!("coprocessor_worker_errors", "worker errors encountered").unwrap();
    static ref DEFERRED_TRANSACTIONS_COUNTER: IntCounter = register_int_counter!(
        "coprocessor_worker_deferred_transactions",
        "transactions left pending because their work rows could not be interpreted"
    )
    .unwrap();
    /// Companion GAUGE to the counter above, and the one that identifies a
    /// wedge. The counter advances once per deferral per poll, so a single
    /// permanently uninterpretable transaction and a burst of transient ones
    /// are indistinguishable in it — both just climb. This is set every cycle,
    /// zero included, so a value pinned at 1 for hours reads as one stuck
    /// transaction while a spike that returns to zero reads as a burst.
    static ref DEFERRED_TRANSACTIONS_GAUGE: IntGauge = register_int_gauge!(
        "coprocessor_worker_deferred_transactions_current",
        "transactions deferred in the most recent work window of this worker"
    )
    .unwrap();
    static ref WORK_ITEMS_POLL_COUNTER: IntCounter = register_int_counter!(
        "coprocessor_work_items_polls",
        "times work items are polled from database"
    )
    .unwrap();
    static ref WORK_ITEMS_NOTIFICATIONS_COUNTER: IntCounter = register_int_counter!(
        "coprocessor_work_items_notifications",
        "times instant notifications for work items received from the database"
    )
    .unwrap();
    static ref WORK_ITEMS_FOUND_COUNTER: IntCounter = register_int_counter!(
        "coprocessor_work_items_found",
        "work items queried from database"
    )
    .unwrap();
    static ref WORK_ITEMS_ERRORS_COUNTER: IntCounter = register_int_counter!(
        "coprocessor_work_items_errors",
        "work items errored out during computation"
    )
    .unwrap();
    static ref WORK_ITEMS_PROCESSED_COUNTER: IntCounter = register_int_counter!(
        "coprocessor_work_items_processed",
        "work items successfully processed and stored in the database"
    )
    .unwrap();
    static ref WORK_ITEMS_QUERY_HISTOGRAM: Histogram = register_histogram!(
        "coprocessor_tfhe_worker_query_work_items_seconds",
        "Histogram of time spent querying work items in tfhe-worker",
        vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.25, 0.5, 0.75, 1.0, 2.0, 5.0, 10.0]
    )
    .unwrap();
}

/// In-process readiness signal. The benchmark harness starts a real worker
/// and needs a deterministic "installed and serving" edge so process startup
/// falls outside the measured interval. It exists only under `bench`: a
/// production build carries no readiness parameter, no channel and no extra
/// public entry point.
#[cfg(feature = "bench")]
pub type ReadinessSignal = tokio::sync::oneshot::Sender<Result<(), String>>;

/// Readiness plumbing threaded through the startup path. Zero-sized without
/// `bench`, so the production build pays nothing for it.
#[cfg(feature = "bench")]
pub type Readiness = Option<ReadinessSignal>;
#[cfg(not(feature = "bench"))]
pub type Readiness = ();

#[cfg(feature = "bench")]
pub const NO_READINESS: Readiness = None;
#[cfg(not(feature = "bench"))]
pub const NO_READINESS: Readiness = ();

pub async fn run_tfhe_worker(
    args: crate::daemon_cli::Args,
    health_check: crate::health_check::HealthCheck,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    run_tfhe_worker_inner(args, health_check, NO_READINESS).await
}

/// Benchmark-harness entry point: identical to [`run_tfhe_worker`] but
/// signals once the worker is installed and serving.
#[cfg(feature = "bench")]
pub async fn run_tfhe_worker_with_readiness(
    args: crate::daemon_cli::Args,
    health_check: crate::health_check::HealthCheck,
    readiness: Option<ReadinessSignal>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    run_tfhe_worker_inner(args, health_check, readiness).await
}

async fn run_tfhe_worker_inner(
    args: crate::daemon_cli::Args,
    health_check: crate::health_check::HealthCheck,
    mut readiness: Readiness,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Determine worker ID to use for the lifetime of this process
    // In case of a failure in tfhe_worker_cycle, the same id must be reused to quickly unlock any held locks
    let worker_id = args.worker_id.unwrap_or(Uuid::new_v4());

    // GCS mode is auto-detected at startup by comparing this binary's
    // compiled-in `STACK_VERSION` against the live `versioning.stack_version`
    // row.
    let db_url = resolve_database_url_from_option(args.database_url.clone())?;
    let gcs_mode = fhevm_engine_common::versioning::resolve_gcs_mode(db_url.as_str())
        .await
        .map_err(|err| {
            error!(target: "tfhe_worker", error = %err, "Failed to resolve gcs_mode from versioning table");
            err
        })?;

    info!(target: "tfhe_worker", worker_id = %worker_id, gcs_mode = gcs_mode, "Starting tfhe-worker service");

    // Shared GCS activation state. `GCS_NOT_ACTIVATED` means the worker is
    // paused (BCS mode keeps this value for the lifetime of the process).
    let start_block_state = Arc::new(AtomicI64::new(GCS_NOT_ACTIVATED));

    if gcs_mode {
        // Long-lived task that mirrors `upgrade_state.start_block` (stack_role
        // = 'GCS') into the atomic, woken by `event_upgrade_activated`. Lives
        // outside the cycle loop so it survives `tfhe_worker_cycle` restarts.
        let (watcher_pool, _refresh) = connect_pool_with_options(
            &db_url,
            sqlx::postgres::PgPoolOptions::new().max_connections(2),
            None,
        )
        .await?;
        let watcher_state = start_block_state.clone();
        tokio::spawn(async move {
            loop {
                if let Err(err) = run_gcs_activation_watcher(&watcher_pool, &watcher_state).await {
                    error!(target: "tfhe_worker", error = %err, "GCS activation watcher errored; restarting in 5s");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        });
    }

    loop {
        // here we log the errors and make sure we retry
        if let Err(cycle_error) = tfhe_worker_cycle(
            &args,
            worker_id,
            gcs_mode,
            start_block_state.clone(),
            health_check.clone(),
            &mut readiness,
        )
        .await
        {
            WORKER_ERRORS_COUNTER.inc();
            if cycle_error.is_fatal_connection() {
                error!(target: "tfhe_worker", error = %cycle_error, "Fatal DB connection error; exiting for k8s restart");
                fhevm_engine_common::telemetry::flush();
                std::process::exit(1);
            }
            error!(target: "tfhe_worker", { error = %cycle_error }, "Error in background worker, retrying shortly");
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    }
}

async fn tfhe_worker_cycle(
    args: &crate::daemon_cli::Args,
    worker_id: Uuid,
    gcs_mode: bool,
    start_block_state: Arc<AtomicI64>,
    health_check: crate::health_check::HealthCheck,
    readiness: &mut Readiness,
) -> Result<(), CoprocessorError> {
    let db_url = resolve_database_url_from_option(args.database_url.clone())
        .map_err(|e| CoprocessorError::Other(e.into()))?;
    // In --gcs-mode, every connection in the data-plane pool is pinned to
    // `search_path = gcs,public` so unqualified writes land in `gcs.*` and
    // shared read-only tables (keys, crs, host_chains, upgrade_state, …)
    // still resolve from `public` via fallback.
    let (pool, _pool_refresh_handle) = connect_pool_with_options_and_connect_options(
        &db_url,
        sqlx::postgres::PgPoolOptions::new().max_connections(args.pg_pool_max_connections),
        None,
        apply_gcs_mode_search_path(gcs_mode),
    )
    .await?;

    let db_key_cache =
        DbKeyCache::new(args.key_cache_size).map_err(|e| CoprocessorError::Other(e.into()))?;
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen(WORK_AVAILABLE_CHANNEL).await?;

    let mut dcid_mngr = dependence_chain::LockMngr::new_with_conf(
        worker_id,
        pool.clone(),
        args.dcid_ttl_sec,
        args.disable_dcid_locking,
        Some(args.dcid_timeslice_sec),
        Some(args.dcid_cleanup_interval_sec),
        Some(args.processed_dcid_ttl_sec),
        args.computation_retry_demote_threshold,
    );

    // The adaptive work window gives each acquired chain
    // ceil(--work-items-batch-size / acquired chains) transactions, and it
    // switches itself OFF for a cycle that acquires more chains than the
    // window admits. An inverted pair therefore removes the fairness
    // mitigation for the head-of-line blocking that batching creates --
    // silently, only under enough load to fill the batch, and only on the
    // cycles where it matters. Say so once at startup rather than leaving it
    // to be inferred from throughput.
    if args.dcid_batch_execution
        && args.dcid_adaptive_batch_execution
        && args.dependence_chains_per_batch > args.work_items_batch_size
    {
        warn!(
            target: "tfhe_worker",
            work_items_batch_size = args.work_items_batch_size,
            dependence_chains_per_batch = args.dependence_chains_per_batch,
            "--dependence-chains-per-batch exceeds --work-items-batch-size: the adaptive \
             work window will disable itself on any cycle that acquires more chains than \
             the window admits, leaving batching without its fairness mitigation. Raise \
             --work-items-batch-size above --dependence-chains-per-batch."
        );
    }

    // Bound the reservation wait by the lease it runs under. The wait is a
    // blocking loop inside batch execution, and the DCID lease is only renewed
    // BETWEEN worker cycles — so a wait longer than the lease guarantees the
    // lease lapses mid-batch, another worker steals the chain and redundantly
    // recomputes it. Failing the reservation early is strictly better: it
    // leaves no stamp (a reservation error is not a verdict on the operands)
    // and the batch retries with the lease still held.
    //
    // The fraction leaves room to return to the loop and extend before expiry
    // rather than racing it.
    let gpu_reservation_timeout = {
        let configured = std::time::Duration::from_millis(args.gpu_memory_reservation_timeout_ms);
        if args.disable_dcid_locking {
            // No lease to outlive.
            configured
        } else {
            let lease_bound = std::time::Duration::from_secs(u64::from(args.dcid_ttl_sec))
                .mul_f32(GPU_RESERVATION_LEASE_FRACTION);
            if configured > lease_bound {
                warn!(
                    target: "tfhe_worker",
                    configured_ms = args.gpu_memory_reservation_timeout_ms,
                    effective_ms = lease_bound.as_millis() as u64,
                    dcid_ttl_sec = args.dcid_ttl_sec,
                    "capping GPU memory reservation timeout to stay inside the DCID lease"
                );
                lease_bound
            } else {
                configured
            }
        }
    };

    // Release all owned locks on startup to avoid stale locks
    dcid_mngr.release_all_owned_locks().await?;
    dcid_mngr.do_cleanup().await?;

    #[cfg(feature = "bench")]
    {
        let _ = db_key_cache
            .fetch_latest_from_pool(&pool)
            .await
            .map_err(|e| CoprocessorError::Other(e.into()))?;
    }
    // The listener, DCID-acquisition state, and benchmark key cache are
    // installed. This is an in-process readiness point, not merely an
    // observation of an arbitrary database session.
    #[cfg(feature = "bench")]
    if let Some(readiness) = readiness.take() {
        let _ = readiness.send(Ok(()));
    }
    #[cfg(not(feature = "bench"))]
    let _ = readiness;
    let mut immediately_poll_more_work = false;
    let mut no_progress_cycles = 0;
    let mut deferred_cooldown = DeferredTransactionCooldown::new();
    let mut consecutive_batch_failures: u32 = 0;
    loop {
        // GCS gating: skip the iteration entirely until the activation
        // watcher has populated `start_block` in `upgrade_state` for
        // `stack_role='GCS'`. Once that's observed, the schema-isolated
        // `search_path = gcs,public` on this pool's connections routes all
        // writes to `gcs.*` automatically — we no longer need the actual
        // start_block value inside the cycle. In BCS mode this branch is a
        // no-op.
        if gcs_mode && start_block_state.load(Ordering::SeqCst) == GCS_NOT_ACTIVATED {
            info!(target: "tfhe_worker", "GCS not yet activated; sleeping before re-check");
            tokio::time::sleep(GCS_GATE_RECHECK).await;
            continue;
        }

        // only if previous iteration had no work done do the wait
        if !immediately_poll_more_work {
            tokio::select! {
                notification = listener.try_recv() => {
                    match notification? {
                        Some(_) => {
                            WORK_ITEMS_NOTIFICATIONS_COUNTER.inc();
                            info!(target: "tfhe_worker", "Received work_available notification from postgres");
                        }
                        None => {
                            // sqlx already reconnected the LISTEN connection; poll for work.
                            warn!(target: "tfhe_worker", "postgres LISTEN connection reset; reconnected");
                        }
                    }
                },
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(args.worker_polling_interval_ms)) => {
                    WORK_ITEMS_POLL_COUNTER.inc();
                    debug!(target: "tfhe_worker", "Polling the database for more work on timer");
                },
            };
        }

        #[cfg(feature = "bench")]
        let now = std::time::SystemTime::now();
        let loop_span = tracing::info_span!("worker_iteration");
        let acq_span = tracing::info_span!(
            parent: &loop_span,
            "acquire_connection"
        );
        let mut conn = pool.acquire().instrument(acq_span).await?;
        // Begin a write tx under the shared controller lock. A retired BCS stack
        // stops here; a GCS worker skips while a rolled-back dry-run is PAUSED.
        // Holding the lock for the transaction keeps cutover and schema reset
        // from overlapping its reads and writes. Shared locks still allow worker
        // replicas to run concurrently. The state check runs after taking the
        // lock. See versioning::begin_write_guarded.
        let txn_span = tracing::info_span!(parent: &loop_span, "begin_transaction");
        let mut trx = match fhevm_engine_common::versioning::begin_write_guarded_conn(
            &mut conn,
            gcs_mode,
            GcsRollbackPolicy::Skip,
        )
        .instrument(txn_span)
        .await?
        {
            WriteGuard::Proceed(trx) => trx,
            WriteGuard::Stop => {
                info!(target: "tfhe_worker", "Cutover completed — BCS worker exiting cycle");
                // Hand back what this worker holds instead of leaving up to
                // `--dependence-chains-per-batch` chains to be reclaimed by
                // work-stealing after `--dcid-ttl-sec`. The startup release
                // cannot cover this: it filters on `worker_id`, which is a
                // fresh UUID per process unless `--worker-id` is configured.
                if let Err(error) = dcid_mngr.release_all_owned_locks().await {
                    warn!(target: "tfhe_worker", { error = %error },
                        "could not release owned locks on exit; they expire with the lock TTL");
                }
                return Ok(());
            }
            WriteGuard::Skip => {
                debug!(target: "tfhe_worker", "GCS dry-run rolled back — skipping cycle");
                immediately_poll_more_work = false;
                continue;
            }
        };

        // Query for transactions to execute
        let (mut transactions, _, has_more_work) = query_for_work(
            args,
            &health_check,
            &mut trx,
            &mut dcid_mngr,
            &mut no_progress_cycles,
            &mut deferred_cooldown,
            gcs_mode,
        )
        .instrument(loop_span.clone())
        .await?;
        if has_more_work {
            if transactions.is_empty() {
                // Rows were loaded but nothing was schedulable: every
                // transaction deferred (uninterpretable rows) or drained
                // (terminal stamps written above). Persist the stamps —
                // the no-work branch below would drop them with the
                // transaction. query_for_work already rotated a deferring
                // chain to the FIFO back (or cooled the transactions, in
                // lockless mode), and the pace falls back to the normal
                // poll/notify cadence: an unconditional immediate re-poll
                // would hot-loop when everything reachable defers (more
                // wedged chains than fit one lock TTL, or a lockless
                // backlog past the cooldown cap). New real work still
                // wakes the worker instantly via work_available. The
                // global no-progress counter is deliberately untouched: it
                // gates the acquire_early_lock escalation, which must not
                // be reachable through wedged chains alone.
                trx.commit().await?;
                immediately_poll_more_work = false;
                continue;
            }
            // We've fetched work, so we'll poll again without waiting
            // for a notification after this cycle.
            immediately_poll_more_work = true;
        } else {
            // There is no selected work for this batch. Commit the read
            // transaction before releasing individual terminal DCIDs: a
            // release must never become visible ahead of its computation
            // state on another connection.
            trx.commit().await?;
            dcid_mngr.release_completed_locks().await?;
            dcid_mngr.do_cleanup().await?;
            no_progress_cycles = 0;

            // Lock another dependence chain if available and
            // continue processing without waiting for notification
            let dcid_span = tracing::info_span!(
                parent: &loop_span,
                "query_dependence_chain",
                dependence_chain_id = tracing::field::Empty
            );

            let refill_limit = if args.dcid_batch_execution {
                (args.dependence_chains_per_batch - dcid_mngr.get_current_lock_ids().len() as i32)
                    .max(0)
            } else {
                1
            };
            let dependence_chain_id = if refill_limit > 0 {
                dcid_mngr
                    .acquire_next_locks(refill_limit)
                    .instrument(dcid_span.clone())
                    .await?
                    .into_iter()
                    .find_map(|(id, _)| id)
            } else {
                None
            };
            immediately_poll_more_work = dependence_chain_id.is_some();

            dcid_span.record(
                "dependence_chain_id",
                tracing::field::display(
                    dependence_chain_id
                        .as_ref()
                        .map(hex::encode)
                        .unwrap_or_else(|| "none".to_string()),
                ),
            );
            continue;
        }

        if dcid_mngr
            .extend_or_release_current_lock(false)
            .await?
            .is_none()
        {
            // This is a best-effort lease. A competing worker may have
            // acquired an expired DCID before this extension; continuing is
            // safe because result materialization is deterministic, although
            // it can redundantly execute the batch.
            if dcid_mngr.enabled() {
                warn!(target: "tfhe_worker", "Lost dcid lock before processing transactions; continuing with potentially redundant work");
            }
        }

        // Rotation state for a failed batch: `build_transaction_graph_and_execute`
        // drains `transactions`, so collect the transaction ids first — they
        // key the lockless quarantine and identify the failing batch in the
        // rotation alert.
        let batch_transaction_ids: Vec<Vec<u8>> = transactions
            .iter()
            .map(|t| t.transaction_id.clone())
            .collect();
        let mut tx_graph = match build_transaction_graph_and_execute(
            &mut transactions,
            db_key_cache.clone(),
            &health_check,
            &mut trx,
            &dcid_mngr,
            gcs_mode,
            gpu_reservation_timeout,
            args.gpu_streams_per_device,
        )
        .instrument(loop_span.clone())
        .await
        {
            Ok(tx_graph) => {
                consecutive_batch_failures = 0;
                tx_graph
            }
            Err(cycle_error)
                if !cycle_error.is_fatal_connection()
                    && matches!(
                        cycle_error,
                        CoprocessorError::SchedulerError(_)
                            | CoprocessorError::FhevmError(_)
                            | CoprocessorError::Other(_)
                    )
                    && consecutive_batch_failures < 2 =>
            {
                consecutive_batch_failures += 1;
                let dependence_chain_ids: Vec<String> = dcid_mngr
                    .get_current_lock_ids()
                    .iter()
                    .map(hex::encode)
                    .collect();
                // A chain-specific batch-execution failure (a panic inside an
                // FHE op, a scheduler error) must not abort the cycle loop:
                // the restart path re-runs release_all_owned_locks, which puts
                // the held chains back at the FIFO FRONT with their original
                // last_updated_at, so a deterministic failure would be
                // re-acquired and re-failed forever, starving every younger
                // chain. Rotate the held set to the BACK instead — the same
                // fairness move as a deferral — and move on: a deterministic
                // failure rotates at poll cadence with this alert as the
                // signal. GLOBAL failures deliberately still propagate to the
                // run loop's log-and-retry (DbError: transient, retrying the
                // same chains at the front is correct and rotation would
                // scramble FIFO order fleet-wide; MissingKeys: rotating every
                // chain until keys arrive would silently destroy the backlog's
                // block ordering while health stays green), and consecutive
                // failures across different batches escalate the same way —
                // execution-class errors can also be global (a lost GPU device
                // panics outside the per-op catch_unwind and surfaces as
                // Other). Fatal connection errors exit for a k8s restart. The
                // write transaction may be poisoned by the failure, so it is
                // dropped (rolled back) and the rotation goes through the
                // manager's own pool connection.
                WORKER_ERRORS_COUNTER.inc();
                error!(target: "tfhe_worker",
                    { error = %cycle_error,
                      dependence_chain_ids = ?dependence_chain_ids,
                      transaction_ids = ?batch_transaction_ids
                          .iter()
                          .map(hex::encode)
                          .collect::<Vec<_>>() },
                    "batch execution failed; rotating dependence chains to the back of the FIFO");
                drop(trx);
                let now = {
                    let offset = time::OffsetDateTime::now_utc();
                    PrimitiveDateTime::new(offset.date(), offset.time())
                };
                // Best-effort: propagating a failed rotation would restart
                // the cycle loop, whose release_all_owned_locks returns the
                // chains to the FIFO FRONT — re-entering the starvation path
                // this arm exists to prevent, precisely when batch failure
                // and DB pressure correlate. On failure, PARK the in-memory
                // locks: without this the next cycle would extend the leases
                // and re-execute the same failing batch at the front;
                // parked, the leases lapse at their TTL and the chains become
                // stealable.
                if let Err(release_error) = dcid_mngr.release_current_lock(false, Some(now)).await {
                    error!(target: "tfhe_worker", { error = %release_error },
                        "failed to rotate chains after batch failure; parking so the leases lapse at their TTL");
                    dcid_mngr.park_current_lock();
                }
                // Lockless fallback: no chain to rotate; quarantine the
                // batch's transactions so the oldest-first window moves on.
                if !dcid_mngr.enabled() {
                    for transaction_id in &batch_transaction_ids {
                        deferred_cooldown.quarantine(transaction_id);
                    }
                }
                // Keep the backoff the cycle-restart path used to provide:
                // retryable DB errors (deadlocks, pool timeouts) surface
                // here too, and with a backlog of work_available
                // notifications pending the select below would return
                // immediately — hammering a Postgres whose distress caused
                // the failure.
                tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
                immediately_poll_more_work = false;
                continue;
            }
            Err(cycle_error) => return Err(cycle_error),
        };
        // A component can outlive the normal lease TTL. Renew once more
        // immediately before persistence to minimize the window in which a
        // second worker can steal and repeat completed FHE work. We do not
        // cancel after a lost lease: duplicate deterministic work is safer
        // and substantially simpler than interrupting an in-flight batch.
        if dcid_mngr
            .extend_or_release_current_lock(false)
            .await?
            .is_none()
            && dcid_mngr.enabled()
        {
            warn!(target: "tfhe_worker", "Lost dcid lock during transaction execution; persisting deterministic results may be redundant");
        }
        let (has_progressed, panicked_transactions) =
            upload_transaction_graph_results(&mut tx_graph, &mut trx, &mut dcid_mngr)
                .instrument(loop_span.clone())
                .await?;
        if !dcid_mngr.enabled() {
            // Pace retryable panics in the lockless fallback: without this
            // the oldest-first window re-selects a still-failing panic
            // transaction at poll cadence. With locking the no-progress
            // parking below paces the same case at the lock TTL.
            for transaction_id in &panicked_transactions {
                deferred_cooldown.quarantine(transaction_id);
            }
        }
        trx.commit().await?;

        // Releasing after commit makes terminal work visible before another
        // worker can acquire a dependent DCID. Keep unfinished DCIDs leased;
        // query_for_work will refill the freed slots on the next iteration.
        let released_completed_locks = dcid_mngr.release_completed_locks().await?;
        if has_progressed || released_completed_locks > 0 {
            no_progress_cycles = 0;
        } else {
            no_progress_cycles += 1;
            if no_progress_cycles >= args.dcid_max_no_progress_cycles {
                // Stop extending this chain's lock so another chain can run.
                // The parked chain remains pending and can be work-stolen
                // after its existing lock TTL expires.
                info!(target: "tfhe_worker", "no progress on dependence chain, parking until lock expiry");
                dcid_mngr.park_current_lock();
            }
        }
        drop(loop_span);
        #[cfg(feature = "bench")]
        {
            let prev_cycle_time = TIMING.load(std::sync::atomic::Ordering::SeqCst);
            TIMING.store(
                now.elapsed().unwrap().as_micros() as u64 + prev_cycle_time,
                std::sync::atomic::Ordering::SeqCst,
            );
        }
    }
}

#[allow(clippy::type_complexity)]
#[tracing::instrument(name = "query_ciphertext_batch", skip_all, fields(count = cts_to_query.len()))]
async fn query_ciphertexts<'a>(
    cts_to_query: &[Vec<u8>],
    trx: &mut sqlx::Transaction<'a, Postgres>,
    gcs_mode: bool,
) -> Result<HashMap<Vec<u8>, (i16, Vec<u8>)>, CoprocessorError> {
    // BCS: the connection's `search_path = public`, so the unqualified
    // `ciphertexts` resolves to `public.ciphertexts` directly. Done in one
    // query.
    //
    // GCS: the connection's `search_path = gcs,public`, so unqualified
    // `ciphertexts` resolves to `gcs.ciphertexts` — the GCS-owned table
    // populated post-activation. Pre-snapshot ciphertexts (produced by BCS
    // before activation) still live in `public.ciphertexts` and must be
    // fetched explicitly. We do this as a two-step query: try GCS first,
    // then fetch the missing handles from `public.ciphertexts`.
    //
    // The public fallback is *block-gated*: `public.ciphertexts` is the live
    // BCS table and keeps growing throughout the dry-run, so an unbounded read
    // could surface a ciphertext BCS produced *after* the snapshot point.
    // Importing such post-start state (which differs across operators and, for
    // a breaking upgrade, differs byte-for-byte from GCS's own re-derivation)
    // would break the consensus gate. We therefore serve a fallback row only
    // when it is not known to have been produced after its track's start block
    // — see the query below.
    let mut ciphertext_map: HashMap<Vec<u8>, (i16, Vec<u8>)> =
        HashMap::with_capacity(cts_to_query.len());

    let rows: Vec<(Vec<u8>, Vec<u8>, i16)> = sqlx::query_as(
        "SELECT handle, ciphertext, ciphertext_type
         FROM ciphertexts
         WHERE handle = ANY($1::BYTEA[])",
    )
    .bind(cts_to_query)
    .fetch_all(trx.as_mut())
    .await
    .map_err(|err| {
        error!(target: "tfhe_worker", { error = %err }, "error while querying ciphertexts");
        err
    })?;
    for (handle, ciphertext, ciphertext_type) in rows {
        let _ = ciphertext_map.insert(handle, (ciphertext_type, ciphertext));
    }

    if gcs_mode {
        let missing: Vec<Vec<u8>> = cts_to_query
            .iter()
            .filter(|h| !ciphertext_map.contains_key(*h))
            .cloned()
            .collect();
        if !missing.is_empty() {
            // Per-chain start-block bounds for this GCS upgrade. `upgrade_state`
            // is a shared (non-duplicated) control-plane table with one row per
            // host chain; read it fully qualified so the result is unambiguous
            // regardless of search_path.
            let windows = sqlx::query!(
                "SELECT host_chain_id, start_block, gw_start_block
                 FROM public.upgrade_state WHERE stack_role = 'GCS'",
            )
            .fetch_all(trx.as_mut())
            .await
            .map_err(|err| {
                error!(target: "tfhe_worker", { error = %err }, "error while reading GCS upgrade_state bounds");
                err
            })?;

            // Fail safe: without at least one window carrying complete bounds we
            // cannot prove a public row is pre-snapshot, so we serve none. The
            // dependent computation then stalls (no consensus) rather than risking
            // divergence — the intended safety behaviour.
            if !windows
                .iter()
                .any(|w| w.start_block.is_some() && w.gw_start_block.is_some())
            {
                warn!(target: "tfhe_worker", { rows = windows.len(), missing = missing.len() },
                    "GCS upgrade_state bounds incomplete; skipping public.ciphertexts fallback");
                return Ok(ciphertext_map);
            }

            // Block-gated fallback into the live `public.ciphertexts`. Fully
            // qualified to bypass search_path. A missing handle is served only
            // if it is NOT known to have been produced after its track's start
            // block. A pre-snapshot row either carries no block lineage
            // (ancient / not-yet-bound, block_number NULL) or lineage at/below
            // the bound; any post-start BCS write carries lineage strictly
            // above it:
            //   - compute outputs -> public.computations.block_number, scoped to
            //     each output's own host chain (its `start_block`, joined by
            //     host_chain_id);
            //   - ZK input ctxts  -> public.input_handles.block_number, which is
            //     the *Gateway* block (`gw_start_block`).
            // Inputs never have a `computations` row and outputs never have an
            // `input_handles` row, so the two guards route by source without an
            // explicit `is_input` branch.
            let rows = sqlx::query!(
                "SELECT c.handle, c.ciphertext, c.ciphertext_type
                 FROM public.ciphertexts c
                 WHERE c.handle = ANY($1::BYTEA[])
                   AND NOT EXISTS (
                       SELECT 1 FROM public.computations comp
                       JOIN public.upgrade_state us
                         ON us.stack_role = 'GCS' AND us.host_chain_id = comp.host_chain_id
                       WHERE comp.output_handle = c.handle
                         AND comp.block_number >= us.start_block)
                   AND NOT EXISTS (
                       SELECT 1 FROM public.input_handles ih
                       WHERE ih.handle = c.handle
                         AND ih.block_number >= (
                             SELECT MIN(gw_start_block) FROM public.upgrade_state WHERE stack_role = 'GCS'))",
                &missing,
            )
            .fetch_all(trx.as_mut())
            .await
            .map_err(|err| {
                error!(target: "tfhe_worker", { error = %err }, "error while querying public.ciphertexts for pre-snapshot handles");
                err
            })?;
            for row in rows {
                let _ = ciphertext_map.insert(row.handle, (row.ciphertext_type, row.ciphertext));
            }

            // Trace every handle the gate withheld — either absent from
            // `public.ciphertexts` or block-gated as post-snapshot. Without this
            // a stalled dry-run (the dependent computation never reaches
            // consensus) gives no hint which handle could not be resolved.
            let withheld: Vec<String> = missing
                .iter()
                .filter(|h| !ciphertext_map.contains_key(*h))
                .map(|h| format!("0x{}", hex::encode(h)))
                .collect();
            if !withheld.is_empty() {
                let gcs_windows: Vec<String> = windows
                    .iter()
                    .map(|w| {
                        format!(
                            "chain {:?} start={:?} gw={:?}",
                            w.host_chain_id, w.start_block, w.gw_start_block
                        )
                    })
                    .collect();
                debug!(target: "tfhe_worker", { gcs_windows = ?gcs_windows, withheld = ?withheld },
                    "GCS public.ciphertexts fallback withheld handles (absent or block-gated as post-snapshot)");
            }
        }
    }

    Ok(ciphertext_map)
}

/// The definition of a terminally DEAD handle, shared verbatim (alias `c`)
/// by the schema-local and the GCS pre-cutover fallback queries so the two
/// verdicts cannot drift. Dead iff at least one producer row carries a
/// TERMINAL error stamp (a retryable panic stamp — see
/// [`RETRYABLE_STAMP_MARKER`] — is not terminal) and NO row could still
/// deliver bytes: a completed row already has published bytes (including
/// legacy completed-and-stamped contradiction rows, which the heal never
/// touches), a live allowed row may still produce them, and a non-allowed
/// row never persists so it cannot satisfy a consumer either. Deadness
/// always requires an actual error: merely-absent producers keep deferring.
const DEAD_PRODUCER_PREDICATE: &str = "bool_or(c.is_error AND NOT c.is_completed \
        AND (c.error_message IS NULL OR c.error_message NOT LIKE '%RETRYABLE%')) \
     AND bool_and(NOT c.is_allowed OR (c.is_error AND NOT c.is_completed \
        AND (c.error_message IS NULL OR c.error_message NOT LIKE '%RETRYABLE%')))";

/// Boundary dependency handles whose bytes can never exist: every producer
/// row is terminally errored (see [`DEAD_PRODUCER_PREDICATE`]). Consumers of
/// these must drain terminally instead of deferring as missing inputs
/// forever. Handles produced by a live (non-errored) row in this window are
/// excluded — the batch may still deliver them, and if the producer errors
/// instead its stamps are seen by the next cycle's check (an all-errored
/// producer contributes no ops, so its handle leaves the window).
async fn query_dead_boundary_handles<'a>(
    trx: &mut sqlx::Transaction<'a, Postgres>,
    the_work: &[WorkItem],
    gcs_mode: bool,
) -> Result<HashSet<Vec<u8>>, CoprocessorError> {
    let produced_live: HashSet<&[u8]> = the_work
        .iter()
        .filter(|w| !w.is_error)
        .map(|w| w.output_handle.as_slice())
        .collect();
    let mut candidates: Vec<Vec<u8>> = the_work
        .iter()
        .flat_map(|w| w.dependencies.iter())
        .filter(|dh| !produced_live.contains(dh.as_slice()))
        .cloned()
        .collect();
    candidates.sort();
    candidates.dedup();
    if candidates.is_empty() {
        return Ok(HashSet::new());
    }
    let mut dead: HashSet<Vec<u8>> = HashSet::new();
    let mut seen: HashSet<Vec<u8>> = HashSet::new();
    let local_query = format!(
        "SELECT c.output_handle, ({DEAD_PRODUCER_PREDICATE}) AS dead
         FROM computations c
         WHERE c.output_handle = ANY($1::BYTEA[])
         GROUP BY c.output_handle"
    );
    for row in sqlx::query(&local_query)
        .bind(&candidates)
        .fetch_all(trx.as_mut())
        .await?
    {
        use sqlx::Row;
        let handle: Vec<u8> = row.get("output_handle");
        let is_dead: Option<bool> = row.get("dead");
        seen.insert(handle.clone());
        if is_dead.unwrap_or(false) {
            dead.insert(handle);
        }
    }
    // GCS mode: `computations` resolves to gcs.computations, created empty
    // at cutover — a producer that errored terminally in BCS before the
    // snapshot leaves rows only in public.computations. Mirror
    // query_ciphertexts' qualified fallback for handles the GCS schema has
    // never seen, bounded to pre-start blocks so a post-start BCS judgment
    // is never imported (a NULL start_block fails the bound: fail-safe,
    // nothing is judged dead).
    if gcs_mode {
        let unseen: Vec<Vec<u8>> = candidates
            .into_iter()
            .filter(|h| !seen.contains(h))
            .collect();
        if !unseen.is_empty() {
            // The pre-start bound also excludes legacy rows with NULL
            // block_number / host_chain_id (pre-2026-03 schema, never
            // backfilled): those producers are never judged dead, so their
            // consumers defer — fail-safe, at worst the pre-existing
            // deferral behavior for ancient history.
            let fallback_query = format!(
                "SELECT c.output_handle, ({DEAD_PRODUCER_PREDICATE}) AS dead
                 FROM public.computations c
                 JOIN public.upgrade_state us
                   ON us.stack_role = 'GCS' AND us.host_chain_id = c.host_chain_id
                 WHERE c.output_handle = ANY($1::BYTEA[])
                   AND c.block_number < us.start_block
                 GROUP BY c.output_handle"
            );
            for row in sqlx::query(&fallback_query)
                .bind(&unseen)
                .fetch_all(trx.as_mut())
                .await?
            {
                use sqlx::Row;
                let is_dead: Option<bool> = row.get("dead");
                if is_dead.unwrap_or(false) {
                    dead.insert(row.get("output_handle"));
                }
            }
        }
    }
    if !dead.is_empty() {
        warn!(target: "tfhe_worker",
            { dead = ?dead.iter().map(hex::encode).collect::<Vec<_>>() },
            "boundary inputs can never be produced (every producer row errored); draining their consumers");
    }
    Ok(dead)
}

#[tracing::instrument(skip_all)]
async fn query_for_work<'a>(
    args: &crate::daemon_cli::Args,
    health_check: &crate::health_check::HealthCheck,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    deps_chain_mngr: &mut dependence_chain::LockMngr,
    no_progress_cycles: &mut u32,
    deferred_cooldown: &mut DeferredTransactionCooldown,
    gcs_mode: bool,
) -> Result<(Vec<ComponentNode>, PrimitiveDateTime, bool), CoprocessorError> {
    let demote_threshold = args.computation_retry_demote_threshold;
    let s_dcid = tracing::info_span!(
        "query_dependence_chain",
        dependence_chain_id = tracing::field::Empty
    );
    // Lock dependence chain
    let (dependence_chain_ids, locking_reasons) = async {
        let result = match deps_chain_mngr.extend_or_release_current_lock(true).await? {
            // If there is a current lock, we extend it and use its dependence_chain_id
            Some((_id, reason)) => {
                let mut ids = deps_chain_mngr.get_current_lock_ids();
                let mut reasons = vec![reason];
                // A held set must not starve newly-ready chains: without this,
                // chains becoming ready while a long batch is in flight wait
                // for the ENTIRE current set to drain before their first
                // acquisition, serializing independent work behind it.
                if args.dcid_batch_execution {
                    let headroom = args.dependence_chains_per_batch - ids.len() as i32;
                    if headroom > 0 {
                        for (id, joined_reason) in deps_chain_mngr
                            .acquire_next_locks(headroom)
                            .await?
                            .into_iter()
                        {
                            if let Some(id) = id {
                                ids.push(id);
                                reasons.push(joined_reason);
                            }
                        }
                    }
                }
                (ids, reasons)
            }
            None => {
                // Saturating: both operands are operator-supplied u32, and the
                // product is only ever compared, so wrapping here would turn a
                // large configured threshold into a small one.
                if *no_progress_cycles
                    < args
                        .dcid_ignore_dependency_count_threshold
                        .saturating_mul(args.dcid_max_no_progress_cycles)
                {
                    if args.dcid_batch_execution {
                        deps_chain_mngr
                            .acquire_next_locks(args.dependence_chains_per_batch)
                            .await?
                            .into_iter()
                            .filter_map(|(id, reason)| id.map(|id| (id, reason)))
                            .unzip()
                    } else {
                        let (id, reason) = deps_chain_mngr.acquire_next_lock().await?;
                        id.map(|id| (vec![id], vec![reason]))
                            .unwrap_or_else(|| (vec![], vec![]))
                    }
                } else {
                    *no_progress_cycles = 0;
                    let (id, reason) = deps_chain_mngr.acquire_early_lock().await?;
                    id.map(|id| (vec![id], vec![reason]))
                        .unwrap_or_else(|| (vec![], vec![]))
                }
            }
        };
        Ok::<_, CoprocessorError>(result)
    }
    .instrument(s_dcid.clone())
    .await?;
    let (dependence_chain_ids, locking_reasons) =
        if deps_chain_mngr.enabled() && dependence_chain_ids.is_empty() {
            // Nothing matches the normal predicates. Before declaring no
            // work, try the repair path for chains whose dependency gate is
            // stale (every producer processed or gone, count never
            // decremented — lost decrements, reorg-orphaned producers):
            // those match neither normal predicate, and the no-progress
            // escalation is only reachable through some OTHER acquired
            // chain stalling — in an otherwise idle pipeline a stranded
            // chain would sit unprocessed forever.
            let (id, reason) = deps_chain_mngr
                .acquire_stale_gated_lock(args.dcid_stale_gate_age_secs)
                .await?;
            id.map(|id| (vec![id], vec![reason]))
                .unwrap_or_else(|| (vec![], vec![]))
        } else {
            (dependence_chain_ids, locking_reasons)
        };
    if deps_chain_mngr.enabled() && dependence_chain_ids.is_empty() {
        // No dependence chain to lock, so no work to do
        health_check.update_db_access();
        health_check.update_activity();
        info!(target: "tfhe_worker", "No dcid found to process");
        return Ok((vec![], PrimitiveDateTime::MAX, false));
    }
    s_dcid.record(
        "dependence_chain_id",
        tracing::field::display(
            dependence_chain_ids
                .first()
                .map(hex::encode)
                .unwrap_or_else(|| "none".to_string()),
        ),
    );
    let quarantine_deferred = !deps_chain_mngr.enabled();
    let cooled_transactions: Vec<Vec<u8>> = if quarantine_deferred {
        deferred_cooldown.active()
    } else {
        vec![]
    };
    let s_work = tracing::info_span!("query_work_items", count = tracing::field::Empty);
    let transaction_batch_size = args.work_items_batch_size;
    let started_at = SystemTime::now();
    // Schema isolation: BCS connects with `search_path = public`, GCS with
    // `search_path = gcs,public`. Unqualified `computations` therefore
    // resolves to the stack's own schema. No table-name swaps needed in code.
    // With locking disabled, retain the historical all-ready-DCID query. A
    // batch-enabled worker instead binds all of its fenced DCIDs.
    let dcid_filter = deps_chain_mngr
        .enabled()
        .then(|| dependence_chain_ids.clone());
    let adaptive_batch_execution = args.dcid_adaptive_batch_execution
        && dependence_chain_ids.len() > 1
        && dependence_chain_ids.len() <= transaction_batch_size as usize;
    // Keep the normal path byte-for-byte query-compatible. Opt-in adaptive
    // batching bounds each acquired DCID to an equal transaction share, then
    // fills the existing global work window in schedule order. This ensures a
    // large chain cannot keep every smaller ready chain out of the next graph.
    let the_work = if adaptive_batch_execution {
        sqlx::query_as!(
            WorkItem,
            "
SELECT
  c.output_handle,
  c.dependencies,
  c.fhe_operation,
  c.is_scalar,
  c.is_allowed,
  c.is_error,
  c.error_message,
  c.transaction_id,
  c.schedule_order,
  c.operand_boundary_mask,
  c.dependence_chain_id
FROM computations c
WHERE c.transaction_id IN (
    SELECT deduped.transaction_id
    FROM (
      -- Collapse to one row per transaction before the LIMIT. The ranking
      -- below is per (dependence chain, transaction), so a transaction whose
      -- rows a fork-exposed database retained under a second chain is ranked
      -- once per chain and would otherwise spend the window's LIMIT on
      -- duplicates of itself. Ordering keeps the earliest of its ranks, which
      -- is the same anchor the non-adaptive query uses.
      SELECT
        adaptive_schedule_order.transaction_id,
        MIN(adaptive_schedule_order.schedule_order) AS schedule_order
      FROM (
        SELECT
          dependence_chain_id,
          transaction_id,
          MIN(schedule_order) AS schedule_order,
          ROW_NUMBER() OVER (
            PARTITION BY dependence_chain_id
            ORDER BY MIN(schedule_order), transaction_id
          ) AS dcid_transaction_rank
        FROM computations
        WHERE is_completed = FALSE
          -- A retryable stamp stays selectable until it has spent its
          -- attempts for this lane pass; past the threshold the row is
          -- DEMOTED and the slow sweep re-arms it with the count reset.
          -- Not terminal: the stamp is untouched and the row still pending.
          AND (is_error = FALSE
               OR (error_message LIKE '%' || $5 || '%' AND error_retry_count < $6))
          AND is_allowed = TRUE
          AND ($1::bytea[] IS NULL OR dependence_chain_id = ANY($1))
          -- Same lockless-fallback fairness clause as the non-adaptive
          -- query below: deferred transactions sit out their cooldown.
          AND NOT (transaction_id = ANY($4::bytea[]))
        GROUP BY dependence_chain_id, transaction_id
      ) AS adaptive_schedule_order
      WHERE adaptive_schedule_order.dcid_transaction_rank <= $2
      GROUP BY adaptive_schedule_order.transaction_id
    ) AS deduped
    ORDER BY deduped.schedule_order ASC, deduped.transaction_id ASC
    LIMIT $3
)
  -- Like the non-adaptive query below: load ALL rows of the selected
  -- transactions, fork-retained siblings included; ownership is re-applied
  -- in code, which turns rows of other chains into recompute-only
  -- producers excluded from results, persistence and completion.
            ",
            dcid_filter.as_deref(),
            dcid_transaction_share(transaction_batch_size, dependence_chain_ids.len(), true,)
                as i64,
            transaction_batch_size as i64,
            &cooled_transactions,
            RETRYABLE_STAMP_MARKER,
            demote_threshold,
        )
        .fetch_all(trx.as_mut())
        .instrument(s_work.clone())
        .await
    } else {
        sqlx::query_as!(
            WorkItem,
            "
-- Acquire all computations from a transaction set
SELECT
  c.output_handle,
  c.dependencies,
  c.fhe_operation,
  c.is_scalar,
  c.is_allowed,
  c.is_error,
  c.error_message,
  c.transaction_id,
  c.schedule_order,
  c.operand_boundary_mask,
  c.dependence_chain_id
FROM computations c
WHERE c.transaction_id IN (
    SELECT DISTINCT
      c_schedule_order.transaction_id
    FROM (
      SELECT transaction_id
      FROM computations
      WHERE is_completed = FALSE
        -- Same demotion bound as the adaptive query above.
        AND (is_error = FALSE
             OR (error_message LIKE '%' || $4 || '%' AND error_retry_count < $5))
        AND is_allowed = TRUE
        AND ($1::bytea[] IS NULL OR dependence_chain_id = ANY($1))
        -- Lockless fallback fairness: transactions this worker deferred
        -- stay out of the window for a cooldown, or they would occupy the
        -- oldest-first window forever and starve younger work. Empty with
        -- DCID locking enabled, where the chain is parked instead.
        AND NOT (transaction_id = ANY($3::bytea[]))
      ORDER BY schedule_order ASC
      LIMIT $2
    ) as c_schedule_order
  )
  -- The transaction-id expansion deliberately loads ALL rows of a selected
  -- transaction, including rows a fork-exposed DB retained under another
  -- chain (`ON CONFLICT .. DO NOTHING` keeps the sibling's row when a handle
  -- collides across replacement blocks). Ownership is re-applied in code:
  -- rows of another chain are recompute-only producers for this chain's
  -- consumers — a boundary bit 0 obligates raw in-transaction bytes, which
  -- must be recomputed, never read back through the persisted round-trip —
  -- and are excluded from results, persistence and completion.
        ",
            dcid_filter.as_deref(),
            transaction_batch_size as i64,
            &cooled_transactions,
            RETRYABLE_STAMP_MARKER,
            demote_threshold,
        )
        .fetch_all(trx.as_mut())
        .instrument(s_work.clone())
        .await
    }
    .map_err(|err| {
        error!(target: "tfhe_worker", { error = %err }, "error while querying work items");
        err
    })?;

    WORK_ITEMS_QUERY_HISTOGRAM.observe(started_at.elapsed().unwrap_or_default().as_secs_f64());
    s_work.record("count", the_work.len());
    health_check.update_db_access();
    if the_work.is_empty() {
        if !dependence_chain_ids.is_empty() {
            info!(target: "tfhe_worker", dcid_count = dependence_chain_ids.len(), locking_count = locking_reasons.len(), "No work items found to process");
        }
        health_check.update_activity();
        return Ok((vec![], PrimitiveDateTime::MAX, false));
    }
    WORK_ITEMS_FOUND_COUNTER.inc_by(the_work.len() as u64);
    info!(target: "tfhe_worker", count = the_work.len(), dcid_count = dependence_chain_ids.len(), locking_count = locking_reasons.len(), "Processing work items");
    let dead_boundary = query_dead_boundary_handles(trx, &the_work, gcs_mode).await?;
    let s_prep = tracing::info_span!("prepare_dataflow_graphs", work_items = the_work.len());
    let (transactions, earliest_schedule_order, any_deferred) = async {
        let mut earliest_schedule_order = PrimitiveDateTime::MAX;
        let mut any_deferred = false;
        let mut deferred_this_cycle: i64 = 0;
        // Partition work directly by transaction
        let work_by_transaction: HashMap<Handle, Vec<_>> = the_work
            .into_iter()
            .into_group_map_by(|k| k.transaction_id.clone());
        // Traverse transactions and build transaction nodes
        let mut transactions: Vec<ComponentNode> = vec![];
        for (transaction_id, txwork) in work_by_transaction.iter() {
            let transaction_id: &Vec<u8> = transaction_id;
            let prepared =
                match prepare_transaction_ops(txwork, dcid_filter.as_deref(), &dead_boundary) {
                    Ok(prepared) => prepared,
                    Err(reason) => {
                        defer_transaction(transaction_id, &reason);
                        deferred_this_cycle += 1;
                        any_deferred = true;
                        if quarantine_deferred {
                            deferred_cooldown.quarantine(transaction_id);
                        }
                        continue;
                    }
                };
            for (output_handle, error_message) in &prepared.invalid_rows {
                let error = std::io::Error::other(error_message.clone());
                let _ = set_computation_error(
                    output_handle,
                    transaction_id,
                    &error,
                    // Validation verdicts are deterministic properties of the
                    // row: an unknown opcode or a bad operand type fails
                    // identically forever.
                    false,
                    trx,
                    deps_chain_mngr,
                )
                .await?;
            }
            if prepared.ops.is_empty() {
                continue;
            }
            let (mut components, _) = match build_component_nodes(prepared.ops, transaction_id) {
                Ok(components) => components,
                Err(e) => {
                    // A dangling local dependence or otherwise malformed
                    // transaction-local graph means this binary cannot
                    // interpret the rows, not that the computation failed:
                    // defer instead of destroying the rows, and let the
                    // alert below surface the wedge. Unrelated transactions
                    // continue.
                    defer_transaction(transaction_id, &format!("invalid transaction graph: {e}"));
                    deferred_this_cycle += 1;
                    any_deferred = true;
                    if quarantine_deferred {
                        deferred_cooldown.quarantine(transaction_id);
                    }
                    continue;
                }
            };
            if let Some(order) = prepared.earliest_owned_allowed {
                earliest_schedule_order = earliest_schedule_order.min(order);
            }
            transactions.append(&mut components);
        }
        // Set unconditionally, so the gauge falls back to zero on a clean
        // cycle rather than latching the last non-zero window.
        DEFERRED_TRANSACTIONS_GAUGE.set(deferred_this_cycle);
        Ok::<_, CoprocessorError>((transactions, earliest_schedule_order, any_deferred))
    }
    .instrument(s_prep)
    .await?;
    if transactions.is_empty() && any_deferred {
        // Deferrals in a non-empty window: rotate the chain to the BACK of
        // the acquisition FIFO (status 'updated', last_updated_at now), so
        // oldest-first acquisition moves on to younger chains instead of
        // re-acquiring this one after every poll — a single worker would
        // otherwise starve every younger chain behind one uninterpretable
        // one. Rotation rather than a kept-'processing' park: it stays
        // reachable by the escalation path and by a listener re-arm
        // immediately (a parked row's worker_id would block both until the
        // lock TTL). No-op with locking disabled, where the transaction
        // cooldown above rotates the window instead.
        //
        // Drain-only windows (every row terminally stamped, nothing
        // deferred) deliberately do NOT rotate: the next cycle finds the
        // window empty and retires the chain through the normal
        // no-work path.
        let now = {
            let offset = time::OffsetDateTime::now_utc();
            PrimitiveDateTime::new(offset.date(), offset.time())
        };
        deps_chain_mngr
            .release_current_lock(false, Some(now))
            .await?;
    }
    // `true` even when `transactions` is empty (everything deferred or
    // drained as invalid): the caller's no-work branch marks the chain
    // processed — which would silently retire a chain that still has
    // pending rows — and drops the transaction WITHOUT committing, rolling
    // back any terminal error stamps written above. The caller
    // distinguishes the empty case itself: it commits the stamps and moves
    // straight on to the next chain (or the next window slice, in
    // lockless mode).
    Ok((transactions, earliest_schedule_order, true))
}

/// Ops of one transaction, ready for `build_component_nodes`.
struct PreparedTransaction {
    ops: Vec<DFGOp>,
    /// Earliest `schedule_order` among the owned, allowed rows; `None` when
    /// the transaction contributes no schedulable work of its own.
    earliest_owned_allowed: Option<PrimitiveDateTime>,
    /// OWNED rows whose content is deterministically invalid (unknown
    /// opcode, operand-type mismatch) — plus their owned transaction-local
    /// consumers, transitively, which are equally uncomputable (their
    /// obligated raw producer can never exist). The executor validates
    /// content before minting, so these cannot come from ordered ingestion
    /// of a healthy chain, and retrying can never succeed: they are marked
    /// terminally errored, DRAINING the chain, unlike interpretation-layer
    /// states, which defer. Content judgments are binary-relative during a
    /// rolling upgrade (an old binary may reject an opcode a new one
    /// understands), so rows of another chain are never judged here: a
    /// foreign row with uninterpretable content defers the transaction
    /// instead.
    invalid_rows: Vec<(Vec<u8>, String)>,
}

/// Fail-closed deferral: the transaction's rows are left pending untouched,
/// so the chain retries (and re-alerts) on a later cycle. Deferral is
/// reserved for rows this binary cannot interpret; failures of the
/// computation itself still set `is_error` through `set_computation_error`.
fn defer_transaction(transaction_id: &[u8], reason: &str) {
    DEFERRED_TRANSACTIONS_COUNTER.inc();
    error!(
        target: "tfhe_worker",
        transaction_id = %hex::encode(transaction_id),
        reason,
        "deferring transaction with uninterpretable work rows"
    );
}

/// How long a deferred transaction stays out of the work window when DCID
/// locking is disabled. With locking enabled the owning chain is parked
/// until its lock TTL instead, which quarantines at the right granularity,
/// so this only paces the lockless fallback mode.
const DEFERRED_TRANSACTION_COOLDOWN: Duration = Duration::from_secs(60);
/// Upper bound on remembered transactions: expired entries are pruned on
/// every use, and past the cap the soonest-expiring entries are dropped
/// first, so a pathological backlog cannot grow the map without bound.
const DEFERRED_TRANSACTION_COOLDOWN_CAP: usize = 4096;

/// In-memory quarantine for transactions this worker could not interpret,
/// used only with DCID locking disabled: the work window selects the oldest
/// pending transactions, so without a cooldown the same deferring
/// transactions would fill every window and starve younger work forever.
/// In-memory on purpose — deferral is interpretation-relative to this
/// binary, so the state must not outlive the process.
pub(crate) struct DeferredTransactionCooldown {
    until: HashMap<Vec<u8>, std::time::Instant>,
}

impl DeferredTransactionCooldown {
    pub(crate) fn new() -> Self {
        Self {
            until: HashMap::new(),
        }
    }

    fn prune(&mut self) {
        let now = std::time::Instant::now();
        self.until.retain(|_, until| *until > now);
        if self.until.len() > DEFERRED_TRANSACTION_COOLDOWN_CAP {
            let mut deadlines: Vec<std::time::Instant> = self.until.values().copied().collect();
            deadlines.sort_unstable();
            // Keep entries at or above the boundary deadline: ties keep the
            // map slightly over cap rather than re-admitting still-hot
            // transactions to the window early.
            let cutoff = deadlines[self.until.len() - DEFERRED_TRANSACTION_COOLDOWN_CAP];
            self.until.retain(|_, until| *until >= cutoff);
        }
    }

    fn quarantine(&mut self, transaction_id: &[u8]) {
        self.prune();
        self.until.insert(
            transaction_id.to_vec(),
            std::time::Instant::now() + DEFERRED_TRANSACTION_COOLDOWN,
        );
    }

    fn active(&mut self) -> Vec<Vec<u8>> {
        self.prune();
        self.until.keys().cloned().collect()
    }
}

/// Builds the dataflow ops for one transaction's loaded rows.
///
/// Rows filed under the locked chain are the schedulable work. Rows of the
/// same transaction retained under another chain (fork siblings kept by
/// `ON CONFLICT .. DO NOTHING`) join the graph only when an owned consumer
/// reaches them through a transaction-local (boundary bit 0) dependence:
/// bit 0 obligates the consumer to the raw in-transaction bytes, so the
/// producer is recomputed here — byte-identical by construction, since a
/// colliding handle proves identical sourcing — instead of read back
/// through the bit-inexact persisted round-trip. Such foreign rows are
/// forced `is_allowed = false` so they can never reach results,
/// persistence or completion, which stay owned by their own chain.
///
/// `Err(reason)` defers the whole transaction (see `defer_transaction`):
/// partially scheduling a transaction would leave consumers of the skipped
/// rows dangling.
fn prepare_transaction_ops(
    txwork: &[WorkItem],
    locked_dependence_chain_ids: Option<&[Vec<u8>]>,
    dead_boundary: &HashSet<Vec<u8>>,
) -> Result<PreparedTransaction, String> {
    // Handles produced by ANY row of this transaction, either fork spelling.
    let produced: HashSet<&[u8]> = txwork.iter().map(|w| w.output_handle.as_slice()).collect();
    let rows_by_handle: HashMap<&[u8], &WorkItem> = txwork
        .iter()
        .map(|w| (w.output_handle.as_slice(), w))
        .collect();
    let row_is_owned = |w: &WorkItem| match locked_dependence_chain_ids {
        None => true,
        Some(fenced) => w
            .dependence_chain_id
            .as_ref()
            .is_some_and(|dcid| fenced.contains(dcid)),
    };
    // The executor's boundary bit for one operand. Legacy rows (written by a
    // pre-mask listener) carry no mask; for those, fall back to the pre-mask
    // inference this worker used before masks existed: an operand this
    // transaction produced is transaction-local, anything else is consumed
    // in its canonical persisted form.
    let operand_boundary = |w: &WorkItem, idx: usize, dh: &[u8]| -> Result<bool, String> {
        match w.operand_boundary_mask.as_deref() {
            Some(mask) => operand_is_boundary(Some(mask), idx).map_err(|e| e.to_string()),
            None => Ok(!produced.contains(dh)),
        }
    };

    let mut included: HashSet<&[u8]> = HashSet::new();
    let mut queue: VecDeque<&WorkItem> = VecDeque::new();
    // Terminally errored rows are never (re-)executed: re-execution fails
    // identically (errors are deterministic), and re-stamping them would be
    // redundant. They still participate below as dead producers so their
    // transaction-local consumers drain with them. Retryable (panic) stamps
    // are the exception: they re-execute, and success heals them.
    let stamp_is_terminal =
        |w: &WorkItem| w.is_error && !stamp_is_retryable(w.error_message.as_deref());
    for w in txwork
        .iter()
        .filter(|w| row_is_owned(w) && !stamp_is_terminal(w))
    {
        if included.insert(w.output_handle.as_slice()) {
            queue.push_back(w);
        }
    }

    let mut ops: Vec<DFGOp> = vec![];
    let mut earliest_owned_allowed: Option<PrimitiveDateTime> = None;
    let mut invalid_rows: Vec<(Vec<u8>, String)> = vec![];
    let mut dead_input_rows: Vec<Vec<u8>> = vec![];
    while let Some(w) = queue.pop_front() {
        let owned = row_is_owned(w);
        let fhe_op: SupportedFheOperations = match w.fhe_operation.try_into() {
            Ok(op) => op,
            Err(e) => {
                if !owned {
                    return Err(format!(
                        "foreign row 0x{} carries uninterpretable content: invalid FHE operation: {e}",
                        hex::encode(&w.output_handle)
                    ));
                }
                invalid_rows.push((
                    w.output_handle.clone(),
                    format!("invalid FHE operation: {e}"),
                ));
                continue;
            }
        };
        let mut inputs: Vec<DFGTaskInput> = Vec::with_capacity(w.dependencies.len());
        let mut this_comp_inputs: Vec<Vec<u8>> = Vec::with_capacity(w.dependencies.len());
        let mut is_scalar_op_vec: Vec<bool> = Vec::with_capacity(w.dependencies.len());
        let mut dead_input: Option<&Vec<u8>> = None;
        for (idx, dh) in w.dependencies.iter().enumerate() {
            let is_operand_scalar =
                fhe_op.is_operand_scalar(w.is_scalar, idx, w.dependencies.len());
            is_scalar_op_vec.push(is_operand_scalar);
            this_comp_inputs.push(dh.clone());
            if is_operand_scalar {
                inputs.push(DFGTaskInput::Value(SupportedFheCiphertexts::Scalar(
                    dh.clone(),
                )));
            } else if operand_boundary(w, idx, dh)? {
                if dead_boundary.contains(dh) {
                    dead_input = Some(dh);
                }
                inputs.push(DFGTaskInput::BoundaryDependence(dh.clone()));
            } else {
                inputs.push(DFGTaskInput::LocalDependence(dh.clone()));
                if !included.contains(dh.as_slice()) {
                    if let Some(producer) = rows_by_handle.get(dh.as_slice()) {
                        // A terminally errored producer stays out of the
                        // graph; the uncomputability propagation below
                        // drains this consumer instead.
                        if !stamp_is_terminal(producer) {
                            included.insert(producer.output_handle.as_slice());
                            queue.push_back(producer);
                        }
                    }
                    // No row at all for a transaction-local producer: leave
                    // the dependence dangling so build_component_nodes
                    // defers this transaction. Ingestion writes a producer
                    // row for every bit-0 operand atomically with the
                    // consumer's, so this indicates deleted or corrupted
                    // state, never a schedulable one.
                }
            }
        }
        if let Some(dh) = dead_input {
            // The obligated canonical bytes for this operand can never
            // exist: every producer row is terminally errored, and errors
            // are deterministic, so retrying can never succeed. Owned
            // consumers drain terminally at op granularity — independent
            // ops of the same transaction keep computing. Foreign rows are
            // dropped without a stamp: the verdict is database-derived, so
            // their own chain reaches the same conclusion.
            dead_input_rows.push(w.output_handle.clone());
            if owned {
                invalid_rows.push((
                    w.output_handle.clone(),
                    format!(
                        "dead boundary input 0x{}: every producer row is terminally errored",
                        hex::encode(dh)
                    ),
                ));
            }
            continue;
        }
        if let Err(e) =
            check_fhe_operand_types(w.fhe_operation.into(), &this_comp_inputs, &is_scalar_op_vec)
        {
            if !owned {
                return Err(format!(
                    "foreign row 0x{} carries uninterpretable content: invalid FHE operands: {e}",
                    hex::encode(&w.output_handle)
                ));
            }
            invalid_rows.push((
                w.output_handle.clone(),
                format!("invalid FHE operands: {e}"),
            ));
            continue;
        }
        ops.push(DFGOp {
            output_handle: w.output_handle.clone(),
            fhe_op,
            inputs,
            // Foreign rows are recompute-only producers, whatever their own
            // row says: results, persistence and completion belong to the
            // chain that owns them.
            is_allowed: owned && w.is_allowed,
        });
        if owned && w.is_allowed {
            // Only account for owned allowed rows to avoid the reorg case
            // where colliding trivial encrypts of a fork sibling would
            // drag the batch's schedule anchor backwards.
            earliest_owned_allowed = Some(match earliest_owned_allowed {
                Some(current) => current.min(w.schedule_order),
                None => w.schedule_order,
            });
        }
    }
    // Uncomputability propagates through transaction-local dependences: a
    // consumer whose bit-0 producer is terminally errored can never obtain
    // the obligated raw bytes, so it drains with the producer instead of
    // deferring forever behind a dangling dependence. Owned consumers are
    // errored; foreign ones are dropped from the graph without judgment
    // (their own chain owns their fate).
    let mut uncomputable: HashSet<Vec<u8>> = invalid_rows
        .iter()
        .map(|(handle, _)| handle.clone())
        .collect();
    // Rows already stamped `is_error` in the database are equally dead
    // producers: the obligated raw bytes can never exist. Foreign errored
    // rows seed the propagation too — the judgment is about the handle, not
    // the row — while stamping below stays confined to owned consumers.
    uncomputable.extend(
        txwork
            .iter()
            .filter(|w| stamp_is_terminal(w))
            .map(|w| w.output_handle.clone()),
    );
    // Ops dropped for a dead boundary input (owned AND foreign): their own
    // outputs are equally unobtainable, so their transaction-local
    // consumers drain with them.
    uncomputable.extend(dead_input_rows);
    loop {
        let mut progressed = false;
        let mut index = 0;
        while index < ops.len() {
            let blocked = ops[index].inputs.iter().any(|input| match input {
                DFGTaskInput::LocalDependence(dh) => uncomputable.contains(dh),
                // Boundary operands are deliberately not consulted here:
                // dead boundary inputs are judged against the database
                // (query_dead_boundary_handles) and drained before this
                // loop, not through the transaction-local propagation.
                DFGTaskInput::BoundaryDependence(_)
                | DFGTaskInput::Value(_)
                | DFGTaskInput::Compressed(..) => false,
            });
            if blocked {
                let op = ops.remove(index);
                // Ownership derived from the row itself, so no parallel
                // bookkeeping can drift out of alignment with `ops`.
                let owned = rows_by_handle
                    .get(op.output_handle.as_slice())
                    .is_some_and(|row| row_is_owned(row));
                uncomputable.insert(op.output_handle.clone());
                if owned {
                    invalid_rows.push((
                        op.output_handle,
                        "transaction-local producer is terminally errored".to_string(),
                    ));
                }
                progressed = true;
            } else {
                index += 1;
            }
        }
        if !progressed {
            break;
        }
    }
    Ok(PreparedTransaction {
        ops,
        earliest_owned_allowed,
        invalid_rows,
    })
}

#[tracing::instrument(name = "build_and_execute", skip_all)]
#[allow(clippy::too_many_arguments)]
async fn build_transaction_graph_and_execute<'a>(
    txs: &mut Vec<ComponentNode>,
    db_key_cache: DbKeyCache,
    health_check: &crate::health_check::HealthCheck,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    dcid_mngr: &dependence_chain::LockMngr,
    gcs_mode: bool,
    gpu_reservation_timeout: std::time::Duration,
    gpu_streams_per_device: usize,
) -> Result<DFComponentGraph, CoprocessorError> {
    let mut tx_graph = DFComponentGraph::default();
    if txs.is_empty() {
        return Ok(tx_graph);
    }
    let _in_flight_work = health_check.begin_work();
    if let Err(e) = tx_graph.build(txs) {
        // If we had an error while building the graph, we don't
        // execute anything and return to allow any set results
        // (essentially errors) to be set in DB.
        warn!(target: "tfhe_worker", { error = %e }, "error while building transaction graph");
        return Ok(tx_graph);
    }
    let cts_to_query = tx_graph.needed_map.keys().cloned().collect::<Vec<_>>();
    let ciphertext_map = query_ciphertexts(&cts_to_query, trx, gcs_mode).await?;
    let fetched_handles: std::collections::HashSet<_> = ciphertext_map.keys().cloned().collect();
    if cts_to_query.len() != fetched_handles.len() {
        if let Some(dcid_lock) = dcid_mngr.get_current_lock() {
            warn!(target: "tfhe_worker", { missing_inputs = ?(cts_to_query.len() - fetched_handles.len()), dcid = %hex::encode(dcid_lock.dependence_chain_id) },
	  "some inputs are missing to execute the dependence chain");
        }
    }
    // Boundary ciphertexts enter the graph in their CANONICAL COMPRESSED
    // form, on every build. RFC-020's boundary consumption rule requires the
    // base value to be `Decompress(cmp(h))`; keeping the compressed form here
    // and decompressing in the executor means the graph never carries a
    // transaction-level raw value, which is what makes the consensus tripwire
    // in `check_ready_inputs` a live assertion in BOTH build configurations
    // rather than something GPU has to opt out of.
    //
    // The redundancy this used to imply — the same boundary handle
    // decompressed once per consuming op — is removed in the executor by a
    // per-partition memo, which RFC-020 explicitly permits ("the worker may
    // cache the canonical decompressed form ct(h) for the duration of the
    // transaction batch"). Memoizing there rather than materializing here
    // also keeps each decompression on the device that will consume it,
    // instead of pinning every boundary value to device 0 and cloning it out
    // to whichever device the partition actually runs on.
    for (handle, (ct_type, mut ct)) in ciphertext_map.into_iter() {
        tx_graph
            .add_input(
                &handle,
                &DFGTxInput::Compressed((
                    CompressedCiphertext {
                        ct_type,
                        ct_bytes: std::mem::take(&mut ct),
                    },
                    true,
                )),
            )
            .map_err(|e| CoprocessorError::Other(e.into()))?;
    }
    // Resolve deferred cross-transaction dependences: edges whose
    // handle was fetched from DB are dropped (data already available),
    // remaining edges are added after cycle detection.
    if let Err(e) = tx_graph.resolve_dependences(&fetched_handles) {
        warn!(target: "tfhe_worker", { error = %e }, "error resolving cross-transaction dependences");
        return Ok(tx_graph);
    }
    // Execute the DFG
    let s_compute = tracing::info_span!("compute_fhe_ops");
    async {
        // Fetch the latest key from the database
        let keys = match db_key_cache.fetch_latest(trx.as_mut()).await {
            Ok(k) => k,
            Err(err) => {
                // Extract the sqlx error from anyhow so it classifies as a
                // fatal connection (fail fast) instead of looking like missing keys.
                let cerr: CoprocessorError = match err.downcast::<sqlx::Error>() {
                    Ok(sqlx_err) => sqlx_err.into(),
                    Err(other) => CoprocessorError::MissingKeys {
                        reason: other.to_string(),
                    },
                };
                error!(target: "tfhe_worker", { error = %cerr }, "failed to fetch latest key");
                telemetry::set_current_span_error(&cerr);
                WORKER_ERRORS_COUNTER.inc();
                return Err(cerr);
            }
        };

        // Bound concurrent GPU partitions to the configured stream capacity.
        // The limiter is process-wide and sized once from the visible device
        // count; the blocking tasks release their permits themselves.
        //
        // A single OnceLock rather than a map keyed by (device count, streams
        // per device): both are constant for the life of the process — the
        // device count comes from the loaded key set and the stream count from
        // a CLI flag — so a map could only ever hold one entry, while making
        // it *structurally* possible to admit two limiters over one GPU.
        #[cfg(feature = "gpu")]
        let gpu_execution_limiter = {
            static LIMITER: std::sync::OnceLock<GpuExecutionLimiter> = std::sync::OnceLock::new();
            match LIMITER.get() {
                Some(limiter) => limiter.clone(),
                None => {
                    let limiter =
                        GpuExecutionLimiter::new(keys.gpu_sks.len(), gpu_streams_per_device)
                            .map_err(|e| CoprocessorError::Other(e.into()))?;
                    // A racing initializer wins; take whichever is installed so
                    // every partition shares one limiter.
                    LIMITER.get_or_init(|| limiter).clone()
                }
            }
        };
        #[cfg(not(feature = "gpu"))]
        let _ = gpu_streams_per_device;

        // Schedule computations in parallel as dependences allow
        tfhe::set_server_key(keys.sks.clone());
        let mut sched = Scheduler::new(
            &mut tx_graph,
            #[cfg(not(feature = "gpu"))]
            keys.sks.clone(),
            keys.pks.clone(),
            #[cfg(feature = "gpu")]
            keys.gpu_sks.clone(),
            #[cfg(feature = "gpu")]
            gpu_execution_limiter,
            health_check.activity_heartbeat.clone(),
            // The worker has no per-batch cancellation; the token exists so a
            // GPU memory reservation wait can be interrupted when one is
            // introduced, and reservation waits stay bounded by the timeout.
            tokio_util::sync::CancellationToken::new(),
            gpu_reservation_timeout,
        );
        sched
            .schedule()
            .await
            .map_err(|e| CoprocessorError::Other(e.into()))?;
        Ok::<(), CoprocessorError>(())
    }
    .instrument(s_compute)
    .await?;
    Ok(tx_graph)
}

#[tracing::instrument(name = "upload_results", skip_all)]
/// Returns (progress, panicked_transactions): the latter are transactions
/// with an ExecutionPanic result this cycle — the lockless caller quarantines
/// them so the oldest-first window cannot re-select a still-failing panic at
/// poll cadence (DCID locking paces the same case via no-progress parking).
async fn upload_transaction_graph_results<'a>(
    tx_graph: &mut DFComponentGraph,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    deps_mngr: &mut dependence_chain::LockMngr,
) -> Result<(bool, Vec<Vec<u8>>), CoprocessorError> {
    // Schema isolation: the connection's `search_path` already routes
    // unqualified writes to the stack's own schema (`public` for BCS,
    // `gcs` for GCS post-activation). The two-step ciphertext read in
    // `query_ciphertexts` is the only place where the cross-schema fallback
    // is explicit.
    // Get computation results
    let graph_results = tx_graph.get_results();
    let mut handles_to_update = vec![];
    let mut panicked_transactions: Vec<Vec<u8>> = vec![];
    let mut res = false;

    // Traverse computations that have been scheduled and
    // upload their results/errors.
    let mut cts_to_insert = vec![];
    for result in graph_results.into_iter() {
        match result.compressed_ct {
            Ok(cct) => {
                cts_to_insert.push((
                    result.handle.clone(),
                    (cct.ct_bytes, (current_ciphertext_version(), cct.ct_type)),
                ));
                handles_to_update.push((result.handle.clone(), result.transaction_id.clone()));
                WORK_ITEMS_PROCESSED_COUNTER.inc();
            }
            Err(mut err) => {
                #[cfg(feature = "gpu")]
                if matches!(
                    err.downcast_ref::<FhevmError>(),
                    Some(error) if is_retryable_gpu_reservation_error(error)
                ) {
                    warn!(
                        target: "tfhe_worker",
                        error = %err,
                        output_handle = %format!("0x{}", hex::encode(&result.handle)),
                        "transient GPU memory reservation failure; leaving computation pending for retry"
                    );
                    continue;
                }
                let cerr: Box<dyn std::error::Error + Send + Sync> =
                    if let Some(fhevm_error) = err.downcast_mut::<FhevmError>() {
                        let mut swap_val = FhevmError::BadInputs;
                        std::mem::swap(fhevm_error, &mut swap_val);
                        CoprocessorError::FhevmError(swap_val).into()
                    } else {
                        CoprocessorError::SchedulerError(
                            err.downcast_ref::<SchedulerError>()
                                .cloned()
                                .unwrap_or(SchedulerError::SchedulerError),
                        )
                        .into()
                    };
                // Downgrade SchedulerError to warning when the
                // error is not about the operations themselves.
                // Do not set the error flag in the DB in such cases.
                if let Some(err) = cerr.downcast_ref::<CoprocessorError>() {
                    if matches!(
                        err,
                        CoprocessorError::SchedulerError(SchedulerError::DataflowGraphError)
                    ) || matches!(
                        err,
                        CoprocessorError::SchedulerError(SchedulerError::SchedulerError)
                    ) {
                        warn!(target: "tfhe_worker",
                                          { error = cerr,
                        output_handle = format!("0x{}", hex::encode(&result.handle)) },
                                        "scheduler encountered an error while processing work item"
                                    );
                        continue;
                    }
                    if matches!(
                        err,
                        CoprocessorError::SchedulerError(SchedulerError::MissingInputs)
                    ) {
                        // Make sure we don't mark this as an error since this simply means that the
                        // inputs weren't available when we tried scheduling these operations.
                        continue;
                    }
                    if matches!(
                        err,
                        CoprocessorError::SchedulerError(SchedulerError::ReRandomisationError)
                    ) {
                        // A property of the KEY, not of the operands: the
                        // server key carries no re-randomization material.
                        // Every computation under that key fails identically,
                        // so stamping this would condemn the whole backlog —
                        // and everything downstream of it, through the
                        // dead-producer drain — for a provisioning fault that
                        // a key fix clears. `require_re_randomization_support`
                        // already refuses the cycle before execution; this is
                        // the belt to that pair of braces.
                        warn!(target: "tfhe_worker",
                            { error = cerr, output_handle = format!("0x{}", hex::encode(&result.handle)) },
                            "key lacks re-randomization material; leaving computation pending for retry");
                        continue;
                    }
                }
                if let Some(CoprocessorError::SchedulerError(SchedulerError::ExecutionPanic(_))) =
                    cerr.downcast_ref::<CoprocessorError>()
                {
                    panicked_transactions.push(result.transaction_id.clone());
                }
                // A terminal stamp IS progress: a dead-chain drain larger
                // than one batch would otherwise count as no-progress
                // cycles and park the chain mid-drain.
                let retryable = !is_terminal_verdict(cerr.downcast_ref::<CoprocessorError>());
                res |= set_computation_error(
                    &result.handle,
                    &result.transaction_id,
                    &*cerr,
                    retryable,
                    trx,
                    deps_mngr,
                )
                .await?;
            }
        }
    }
    if !cts_to_insert.is_empty() {
        let s_insert = tracing::info_span!("insert_ct_into_db", count = cts_to_insert.len());
        let cts_inserted = async {
            #[allow(clippy::type_complexity)]
            let (handles, (ciphertexts, (ciphertext_versions, ciphertext_types))): (
                Vec<_>,
                (Vec<_>, (Vec<_>, Vec<_>)),
            ) = cts_to_insert.into_iter().unzip();
            let cts_inserted = sqlx::query!(
                "INSERT INTO ciphertexts(handle, ciphertext, ciphertext_version, ciphertext_type)
                 SELECT * FROM UNNEST($1::BYTEA[], $2::BYTEA[], $3::SMALLINT[], $4::SMALLINT[])
                 ON CONFLICT (handle, ciphertext_version) DO NOTHING",
                &handles,
                &ciphertexts,
                &ciphertext_versions,
                &ciphertext_types,
            )
            .execute(trx.as_mut())
            .await.map_err(|err| {
                error!(target: "tfhe_worker", { error = %err }, "error while inserting new ciphertexts");
                err
            })?.rows_affected();
            // Notify all workers that new ciphertext is inserted
            // For now, it's only the SnS workers that are listening for these events
            let _ = sqlx::query!("SELECT pg_notify($1, '')", EVENT_CIPHERTEXT_COMPUTED)
                .execute(trx.as_mut())
                .await?;
            Ok::<u64, CoprocessorError>(cts_inserted)
        }
        .instrument(s_insert)
        .await?;
        res |= cts_inserted > 0;
    }

    if !handles_to_update.is_empty() {
        let s_update = tracing::info_span!("update_computation", count = handles_to_update.len());
        let comp_updated = async {
            let (handles_vec, txn_ids_vec): (Vec<_>, Vec<_>) = handles_to_update.into_iter().unzip();
            let comp_updated = query!(
                "
            -- Bytes are the ground truth of success: a row whose ciphertext
            -- was just inserted is completed, and any error stamp on it was a
            -- nondeterministic-failure artifact (e.g. another worker's
            -- transient panic on the same stolen batch) — heal it, or the
            -- row would stay a terminally errored, never-completed
            -- contradiction next to published bytes. Rows drained for dead
            -- inputs never execute, so no bytes ever arrive to heal them.
            UPDATE computations
            SET is_completed = true, completed_at = CURRENT_TIMESTAMP,
                is_error = false, error_message = NULL, error_retry_count = 0
            WHERE is_completed = false
            AND (output_handle, transaction_id) IN (
                SELECT * FROM unnest($1::BYTEA[], $2::BYTEA[])
            )
            ",
                &handles_vec,
                &txn_ids_vec
            )
            .execute(trx.as_mut())
            .await.map_err(|err| {
                error!(target: "tfhe_worker", { error = %err }, "error while updating computations as completed");
                err
            })?.rows_affected();
            Ok::<u64, CoprocessorError>(comp_updated)
        }
        .instrument(s_update)
        .await?;
        res |= comp_updated > 0;
    }
    panicked_transactions.sort();
    panicked_transactions.dedup();
    Ok((res, panicked_transactions))
}

#[cfg(all(test, feature = "gpu"))]
mod gpu_reservation_error_tests {
    use super::*;
    use fhevm_engine_common::gpu_memory::GpuMemoryReservationError;

    #[test]
    fn timeout_and_cancellation_are_retryable() {
        for reservation_error in [
            GpuMemoryReservationError::TimedOut {
                gpu_idx: 0,
                amount: 1,
                waited_ms: 10,
            },
            GpuMemoryReservationError::Cancelled {
                gpu_idx: 0,
                amount: 1,
            },
        ] {
            assert!(is_retryable_gpu_reservation_error(
                &FhevmError::GpuMemoryReservationError(reservation_error)
            ));
        }
    }

    #[test]
    /// Deployment and bookkeeping faults are retryable too. Neither says
    /// anything about the operands, so neither may strand a computation — or,
    /// through the dead-producer drain, everything downstream of it.
    fn invariant_failures_are_retryable_too() {
        for reservation_error in [
            GpuMemoryReservationError::UnknownDevice { gpu_idx: 7 },
            GpuMemoryReservationError::AccountingOverflow { gpu_idx: 0 },
        ] {
            assert!(is_retryable_gpu_reservation_error(
                &FhevmError::GpuMemoryReservationError(reservation_error)
            ));
        }
    }
}

/// Is this failure a permanent verdict on the computation?
///
/// TERMINAL IFF THE VERDICT IS A PURE FUNCTION OF IMMUTABLE ON-CHAIN DATA —
/// the opcode, the operand types, the shape of the graph. A terminal stamp is
/// never re-selected, its chain retires without it, and the dead-producer
/// drain condemns everything downstream, so it is only correct when re-running
/// the identical inputs could not possibly produce a different answer.
///
/// Everything else is retryable and, if it keeps failing, demoted to the slow
/// lane rather than condemned. Note the polarity: the default arm is
/// RETRYABLE, so a variant added to either enum later fails towards the
/// reversible outcome instead of silently condemning cones. That is the
/// opposite of the catch-all this function replaced, under which any
/// unmatched `FhevmError` was terminal.
fn is_terminal_verdict(error: Option<&CoprocessorError>) -> bool {
    match error {
        Some(CoprocessorError::SchedulerError(scheduler_error)) => match scheduler_error {
            // A dependence cycle is a property of the ingested edges and does
            // not change under re-execution.
            SchedulerError::CyclicDependence => true,
            // MissingLocalProducer is NOT a verdict on the operands. The
            // scenario that produces it — a listener/worker skew where one
            // binary's operand-mask derivation disagrees with the other's
            // reading of it — makes the outcome binary-dependent rather than a
            // function of on-chain data, which is exactly what the rule
            // excludes. Unreachable from here in any case: its only raise path
            // defers the transaction before stamping. Classified with the
            // reversible arm so the code and R8's design table agree.
            SchedulerError::MissingLocalProducer => false,
            // A panic — around the op, a boundary decompression or the output
            // compression — can be device or allocation pressure. Persisted
            // bytes that would not expand can be corruption, but a transient
            // device fault is indistinguishable at this point.
            SchedulerError::ExecutionPanic(_) | SchedulerError::DecompressionError => false,
            // Scheduling state rather than verdicts; these never reach the
            // stamping path at all (handled above), and if one ever did,
            // treating it as retryable is the safe direction.
            SchedulerError::MissingInputs
            | SchedulerError::DataflowGraphError
            | SchedulerError::ReRandomisationError
            | SchedulerError::SchedulerError => false,
        },
        Some(CoprocessorError::FhevmError(fhevm_error)) => match fhevm_error {
            // Type-check, opcode and scalar-shape rules. Every one is decided
            // from the operand types and the operation id, both of which come
            // from the chain.
            //
            // The list must cover the verdicts raised at EXECUTION time, not
            // only the ones `check_fhe_operand_types` catches before the graph
            // is built: `UnsupportedFheTypes` is the engine's most common type
            // verdict and every one of its raise sites is a match on the
            // operation id and the operand type names. Leaving it to the
            // retryable default arm did more than retry it until demotion —
            // the dead-producer predicate reads the stamp, so the errored
            // producer never counted as dead and its cross-transaction
            // consumers deferred as missing-inputs forever.
            FhevmError::UnknownFheOperation(_)
            | FhevmError::UnknownFheType(_)
            | FhevmError::CannotCompressScalar
            | FhevmError::CiphertextCompressionRequiresEmptyCarries
            | FhevmError::CiphertextExpansionUnsupportedCiphertextKind(_)
            | FhevmError::FheOperationOnlyOneOperandCanBeScalar { .. }
            | FhevmError::FheOperationDoesntSupportScalar { .. }
            | FhevmError::FheOperationOnlySecondOperandCanBeScalar { .. }
            | FhevmError::FheOperationDoesntHaveUniformTypesAsInput { .. }
            | FhevmError::FheOperationScalarDivisionByZero { .. }
            | FhevmError::FheOperationDoesntSupportEbytesAsInput { .. }
            | FhevmError::UnexpectedOperandCountForFheOperation { .. }
            | FhevmError::OperationDoesntSupportBooleanInputs { .. }
            | FhevmError::FheIfThenElseUnexpectedOperandTypes { .. }
            | FhevmError::FheIfThenElseMismatchingSecondAndThirdOperatorTypes { .. }
            | FhevmError::UnexpectedCastOperandTypes { .. }
            | FhevmError::UnexpectedCastOperandSizeForScalarOperand { .. }
            | FhevmError::UnknownCastType { .. }
            | FhevmError::AllInputsForTrivialEncryptionMustBeScalar { .. }
            | FhevmError::UnexpectedTrivialEncryptionOperandSizeForScalarOperand { .. }
            | FhevmError::UnexpectedRandOperandSizeForOutputType { .. }
            | FhevmError::RandOperationUpperBoundCannotBeZero { .. }
            | FhevmError::RandOperationInputsMustAllBeScalar { .. }
            | FhevmError::UnsupportedFheTypes { .. } => true,
            // NOT terminal, and this is the correction that motivated turning
            // the list into a rule: a `tfhe::Error` out of expansion or
            // compression can be a device fault that surfaced as `Err` rather
            // than as a panic. The identical fault arriving as a panic is
            // retried, so classifying this one as a verdict was inconsistent.
            // `DeserializationError` carries the same corrupt-versus-transient
            // ambiguity that made `DecompressionError` retryable.
            FhevmError::CiphertextExpansionError(_)
            | FhevmError::CiphertextCompressionError(_)
            | FhevmError::DeserializationError(_) => false,
            // Everything else — including any variant added later.
            _ => false,
        },
        // A non-Coprocessor error reaching the stamp is unclassifiable, which
        // is not the same as deterministic.
        _ => false,
    }
}

#[cfg(test)]
mod terminal_verdict_tests {
    use super::*;

    /// The type verdicts raised at EXECUTION time, not just the ones the
    /// pre-execution check catches. `UnsupportedFheTypes` is the one the
    /// engine raises most, and classifying it as retryable is not a private
    /// matter of pacing: the dead-producer predicate reads the stamp, so a
    /// retryable producer is never dead and its cross-transaction consumers
    /// defer as missing-inputs forever.
    #[test]
    fn execution_time_type_verdicts_are_terminal() {
        for error in [
            FhevmError::UnsupportedFheTypes {
                fhe_operation: "FheSub".to_string(),
                input_types: vec!["FheUint32", "FheUint64"],
            },
            FhevmError::UnknownCastType {
                fhe_operation: "FheCast".to_string(),
                type_to_cast_to: 123,
            },
            FhevmError::RandOperationUpperBoundCannotBeZero {
                fhe_operation: 27,
                fhe_operation_name: "FheRandBounded".to_string(),
                upper_bound_value: "0".to_string(),
            },
        ] {
            let message = error.to_string();
            assert!(
                is_terminal_verdict(Some(&CoprocessorError::FhevmError(error))),
                "expected a terminal verdict for {message}"
            );
        }
    }

    /// The polarity of the default arm, unchanged: a failure that says
    /// nothing about the operands stays reversible.
    #[test]
    fn transient_failures_stay_retryable() {
        assert!(!is_terminal_verdict(Some(&CoprocessorError::FhevmError(
            FhevmError::DeserializationError(Box::new(std::io::Error::other("truncated")))
        ))));
        assert!(!is_terminal_verdict(Some(
            &CoprocessorError::SchedulerError(SchedulerError::ExecutionPanic(
                "device pressure".to_string()
            ))
        )));
        assert!(is_terminal_verdict(Some(
            &CoprocessorError::SchedulerError(SchedulerError::CyclicDependence)
        )));
    }
}

#[tracing::instrument(skip_all)]
async fn set_computation_error<'a>(
    output_handle: &[u8],
    transaction_id: &[u8],
    cerr: &(dyn std::error::Error + Send + Sync),
    retryable: bool,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    deps_mngr: &mut dependence_chain::LockMngr,
) -> Result<bool, CoprocessorError> {
    // The marker is applied HERE rather than being baked into any error's
    // Display, so the retryable class is a property of the stamping decision
    // and lives in exactly one place.
    let err_string = if retryable {
        format!("{RETRYABLE_STAMP_MARKER} {cerr}")
    } else {
        cerr.to_string()
    };

    // A completed row's ciphertext exists — a cascaded error (a poisoned
    // transaction stamps EVERY op of the node, mirroring set_uncomputable's
    // transaction granularity) must never flip it, or the false error would
    // cascade to its consumers next cycle. An already-errored row keeps its
    // original message: the root cause, not the cascade.
    //
    // The one exception is a RETRYABLE stamp, which is not a verdict but a
    // "try again" marker. A TERMINAL stamp supersedes it immediately — a row
    // that first panicked and later became provably unsatisfiable (its
    // producer drained as dead) must be able to reach a terminal state, or
    // the work window re-selects it forever and its chain never completes.
    // The direction is one-way: a terminal stamp is never re-selected, so it
    // can never be overwritten in turn.
    //
    // A repeated retryable stamp only advances `error_retry_count`. At
    // `--computation-retry-demote-threshold` the work window and the chain's
    // completion test both stop counting the row, which DEMOTES it to the
    // slow lane — it is never rewritten into a terminal verdict. Nothing
    // here can turn a transient failure into a condemnation, which is why
    // the resource-pressure exemption this function used to carry could be
    // deleted rather than maintained: there is no longer a promotion for
    // memory pressure to be exempted from.
    //
    // The pre-image is read through a self-join because the progress signal
    // must be a state TRANSITION, not "a row was touched": Postgres reports a
    // matched row for an identical-value UPDATE, so returning rows_affected
    // would make a perpetually panicking computation reset
    // `no_progress_cycles` every cycle and suppress the parking that paces it.
    // A retry that only advances the counter is deliberately NOT progress.
    let stamped = query!(
        "
        UPDATE computations AS c
        SET is_error = true,
            -- Unconditional now that nothing rewrites the stamp: a fresh
            -- error, a terminal stamp superseding a retryable one, and a
            -- repeat of the same retryable failure all write the same text.
            error_message = $1,
            error_retry_count = CASE
                WHEN NOT old.is_error THEN 0
                -- Saturating: the count only has to reach the demote
                -- threshold, and a chain retried for months must not wrap
                -- back under it.
                ELSE LEAST(old.error_retry_count + 1, 32767)::smallint
            END
        FROM computations AS old
        WHERE old.output_handle = c.output_handle
          AND old.transaction_id = c.transaction_id
          AND c.output_handle = $2
          AND c.transaction_id = $3
          AND c.is_completed = false
          AND (c.is_error = false OR c.error_message LIKE '%' || $4 || '%')
        RETURNING (
            NOT old.is_error
            OR $1 NOT LIKE '%' || $4 || '%'
        ) AS \"state_changed!\"
        ",
        err_string,
        output_handle,
        transaction_id,
        RETRYABLE_STAMP_MARKER,
    )
    .fetch_optional(trx.as_mut())
    .await?
    .map(|row| row.state_changed)
    .unwrap_or(false);
    // the chain's root-cause error_message for those would inflate error
    // metrics and bury the first, most informative message.
    if stamped {
        WORKER_ERRORS_COUNTER.inc();
        error!(target: "tfhe_worker", error = %err_string, output_handle = %format!("0x{}", hex::encode(output_handle)), "error while processing work item");
        telemetry::set_current_span_error(&err_string);
        // Scoped to the chain that owns this computation: the stamp is a fact
        // about one row, and with batching the worker may hold ~20 chains.
        let owner: Option<Vec<u8>> = query_scalar!(
            "SELECT dependence_chain_id FROM computations \
             WHERE output_handle = $1 AND transaction_id = $2",
            output_handle,
            transaction_id,
        )
        .fetch_optional(trx.as_mut())
        .await?
        .flatten();
        deps_mngr
            .set_processing_error(Some(err_string), owner.as_deref())
            .await?;
    }
    Ok(stamped)
}
