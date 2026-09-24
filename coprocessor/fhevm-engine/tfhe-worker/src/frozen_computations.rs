//! Drop selected work that depends on unhealed ct64 drift.

use super::WorkItem;
use crate::types::CoprocessorError;
use fhevm_engine_common::types::{Handle, TxHash};
use lazy_static::lazy_static;
use prometheus::{register_int_counter, IntCounter};
use sqlx::{PgPool, Postgres, Transaction};
use std::collections::{HashMap, HashSet};
use time::{Duration, OffsetDateTime, PrimitiveDateTime};

lazy_static! {
    static ref DROPPED_SCHEDULED: IntCounter = register_int_counter!(
        "coprocessor_containment_dropped_scheduled_total",
        "Computation rows dropped from a work pick because a dependency is unhealed ct64 drift"
    )
    .unwrap();
    static ref DROPPED_COMPUTED: IntCounter = register_int_counter!(
        "coprocessor_containment_dropped_computed_total",
        "Computed outputs or errors discarded at persist because their dependencies became frozen"
    )
    .unwrap();
    static ref DROPPED_TRANSACTIONS: IntCounter = register_int_counter!(
        "coprocessor_containment_dropped_transactions_total",
        "Transactions whose every selected row was dropped by containment filtering"
    )
    .unwrap();
    static ref DROPPED_BATCHES: IntCounter = register_int_counter!(
        "coprocessor_containment_dropped_batches_total",
        "Work picks whose every selected row was dropped by containment filtering"
    )
    .unwrap();
    static ref TRANSACTIONS_FOUND: IntCounter = register_int_counter!(
        "coprocessor_containment_transactions_found_total",
        "Transactions with at least one row kept after containment filtering"
    )
    .unwrap();
    static ref BATCHES_FOUND: IntCounter = register_int_counter!(
        "coprocessor_containment_batches_found_total",
        "Work picks that selected at least one computation row"
    )
    .unwrap();
}

pub(super) fn note_discarded(n: usize) {
    DROPPED_COMPUTED.inc_by(n as u64);
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct ScheduleFilterCounts {
    pub dropped_scheduled: u64,
    pub dropped_transactions: u64,
    pub dropped_batches: u64,
    pub transactions_found: u64,
    pub batches_found: u64,
}

pub(super) fn schedule_filter_counts(
    selected: usize,
    filtered: &FilteredWork,
) -> ScheduleFilterCounts {
    if selected == 0 {
        return ScheduleFilterCounts::default();
    }
    let mut found_txs = HashSet::with_capacity(filtered.kept.len());
    for row in &filtered.kept {
        found_txs.insert(&row.transaction_id);
    }
    ScheduleFilterCounts {
        dropped_scheduled: (selected - filtered.kept.len()) as u64,
        dropped_transactions: filtered.empty_transactions.len() as u64,
        dropped_batches: u64::from(filtered.kept.is_empty()),
        transactions_found: found_txs.len() as u64,
        batches_found: 1,
    }
}

fn record_schedule_filter(selected: usize, filtered: &FilteredWork) {
    let counts = schedule_filter_counts(selected, filtered);
    DROPPED_SCHEDULED.inc_by(counts.dropped_scheduled);
    DROPPED_TRANSACTIONS.inc_by(counts.dropped_transactions);
    DROPPED_BATCHES.inc_by(counts.dropped_batches);
    TRANSACTIONS_FOUND.inc_by(counts.transactions_found);
    BATCHES_FOUND.inc_by(counts.batches_found);
}

pub(super) struct FilteredWork {
    pub kept: Vec<WorkItem>,
    /// Selected txs that have no remaining row after the drop.
    pub empty_transactions: Vec<TxHash>,
    pub freeze: Freeze,
}

/// Freeze of one scheduling cycle, plus the batch needed to revise weights
/// if drift appears before persist.
#[derive(Default)]
pub(super) struct Freeze {
    pub frozen: HashSet<(TxHash, Handle)>,
    pub batch: Vec<BatchRow>,
    /// Drifted handle → Smith weight `n` already in `tx_unlock_potential`.
    pub weights: HashMap<Handle, f64>,
}

/// One selected computation.
#[derive(Clone)]
pub(super) struct BatchRow {
    pub output: Handle,
    pub tx: TxHash,
    pub deps: Vec<Handle>,
}

/// `empty_transactions` get `schedule_order` bumped in the same pick trx.
/// `frozen` can skip persist for those outputs; reload before insert in case
/// drift arrived during execution.
///
/// `tx_unlock_potential` is written on `pool` into `drifted_handle_demand`
/// so healing sees it before the batch commits and does not share a row
/// lock with `drifted_handle`.
pub(super) async fn containment_filter(
    pool: &PgPool,
    work: Vec<WorkItem>,
) -> Result<FilteredWork, CoprocessorError> {
    let drifted = drifted_ct64_handles(pool).await?;
    let selected = work.len();
    let filtered = filter_work(work, &drifted);
    record_schedule_filter(selected, &filtered);
    update_tx_unlock_potential(pool, &filtered.freeze.weights).await?;
    Ok(filtered)
}

pub(super) async fn drifted_ct64_handles(
    pool: &PgPool,
) -> Result<HashSet<Handle>, CoprocessorError> {
    Ok(sqlx::query_scalar!(
        r#"SELECT DISTINCT handle
           FROM public.drifted_handle
           WHERE reason = 'ct64_mismatch' AND healed_at IS NULL"#
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .collect())
}

/// Walks the whole selection: a frozen output in tx A freezes consumers in tx B.
pub(super) fn filter_work(work: Vec<WorkItem>, drifted: &HashSet<Handle>) -> FilteredWork {
    let batch: Vec<BatchRow> = work
        .iter()
        .map(|row| BatchRow {
            output: row.output_handle.clone(),
            tx: row.transaction_id.clone(),
            deps: row.dependencies.clone(),
        })
        .collect();
    let (dropped, weights) = score(&batch, drifted);
    let mut remaining: HashMap<TxHash, usize> = HashMap::new();
    for row in &work {
        *remaining.entry(row.transaction_id.clone()).or_insert(0) += 1;
    }
    let mut kept = Vec::with_capacity(work.len());
    let mut frozen = HashSet::new();
    for (row, dropped) in work.into_iter().zip(dropped) {
        if dropped {
            *remaining.get_mut(&row.transaction_id).expect("counted") -= 1;
            frozen.insert((row.transaction_id.clone(), row.output_handle));
        } else {
            kept.push(row);
        }
    }
    let empty_transactions = remaining
        .into_iter()
        .filter(|(_, left)| *left == 0)
        .map(|(tx, _)| tx)
        .collect();
    FilteredWork {
        kept,
        empty_transactions,
        freeze: Freeze {
            frozen,
            batch,
            weights,
        },
    }
}

pub(super) async fn penalize_frozen_transactions(
    trx: &mut Transaction<'_, Postgres>,
    transaction_ids: &[TxHash],
    window_max: PrimitiveDateTime,
) -> Result<(), CoprocessorError> {
    if transaction_ids.is_empty() {
        return Ok(());
    }
    let now = {
        let n = OffsetDateTime::now_utc();
        PrimitiveDateTime::new(n.date(), n.time())
    };
    let bumped = window_max.checked_add(Duration::seconds(1)).unwrap_or(now);
    let schedule_order = if bumped > now { now } else { bumped };
    sqlx::query!(
        r#"UPDATE computations
           SET schedule_order = $1
           WHERE transaction_id = ANY($2) AND is_completed = FALSE"#,
        schedule_order,
        transaction_ids
    )
    .execute(trx.as_mut())
    .await?;
    Ok(())
}

/// Reloads drift and re-scores `freeze.batch`.
///
/// Late drifted handles are extra blockers on the same batch: in-batch
/// transitivity is the same walk. Newly frozen outputs freeze their consumers;
/// `k` grows for txs that gain a root. Replaces last update `n1` with `n2`
/// in the same EMA slot.
pub(super) async fn revise(
    pool: &PgPool,
    freeze: &Freeze,
) -> Result<HashSet<(TxHash, Handle)>, CoprocessorError> {
    if freeze.batch.is_empty() {
        return Ok(HashSet::new());
    }
    let drifted = drifted_ct64_handles(pool).await?;
    let (dropped, n2) = score(&freeze.batch, &drifted);
    replace_last_update_tx_unlock_potential(pool, &freeze.weights, &n2).await?;
    Ok(freeze
        .batch
        .iter()
        .zip(dropped)
        .filter(|(_, dropped)| *dropped)
        .map(|(row, _)| (row.tx.clone(), row.output.clone()))
        .collect())
}

fn score(batch: &[BatchRow], drifted: &HashSet<Handle>) -> (Vec<bool>, HashMap<Handle, f64>) {
    let dropped = dropped(
        batch.iter().map(|row| (&row.output, row.deps.as_slice())),
        drifted,
    );
    let roots = roots(
        batch.iter().map(|row| (&row.output, row.deps.as_slice())),
        &dropped,
        drifted,
    );
    let weights = shares(
        batch
            .iter()
            .enumerate()
            .filter_map(|(i, row)| dropped[i].then_some((&row.tx, &roots[i]))),
    );
    (dropped, weights)
}

/// `dropped[i]`: op `i` is frozen (skip at schedule, skip insert later).
fn dropped<'a>(
    rows: impl IntoIterator<Item = (&'a Handle, &'a [Handle])>,
    drifted: &HashSet<Handle>,
) -> Vec<bool> {
    let rows: Vec<_> = rows.into_iter().collect();
    let mut dropped = vec![false; rows.len()];
    let mut frozen_handles: HashSet<&Handle> = HashSet::new();
    loop {
        let mut changed = false;
        for (i, (output, dependencies)) in rows.iter().enumerate() {
            if dropped[i] {
                continue;
            }
            if dependencies
                .iter()
                .any(|dep| drifted.contains(dep) || frozen_handles.contains(dep))
            {
                dropped[i] = true;
                frozen_handles.insert(*output);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    dropped
}

/// Drifted handles that freeze each dropped row, walking in-batch frozen deps.
fn roots<'a>(
    rows: impl IntoIterator<Item = (&'a Handle, &'a [Handle])>,
    dropped: &[bool],
    drifted: &HashSet<Handle>,
) -> Vec<HashSet<Handle>> {
    let rows: Vec<_> = rows.into_iter().collect();
    let mut index = HashMap::new();
    for (i, (output, _)) in rows.iter().enumerate() {
        index.insert(*output, i);
    }
    let mut roots = vec![HashSet::new(); rows.len()];
    loop {
        let mut changed = false;
        for (i, (_, dependencies)) in rows.iter().enumerate() {
            if !dropped[i] {
                continue;
            }
            for dep in *dependencies {
                if drifted.contains(dep) {
                    changed |= roots[i].insert(dep.clone());
                }
                if let Some(&j) = index.get(dep) {
                    if dropped[j] {
                        let extra: Vec<_> = roots[j].iter().cloned().collect();
                        for handle in extra {
                            changed |= roots[i].insert(handle);
                        }
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }
    roots
}

/// Additive unlock share per drifted handle in this batch.
///
/// A stalled tx blocked by `k` drifted handles contributes `1/k` to each.
/// That is a linear relaxation of AND-unlock (the tx runs only after every
/// blocker is healed). On that relaxation the share is a Smith weight `w_j`;
/// WSPT sequences heals by `w_j / p_j` (W. E. Smith, Naval Research Logistics
/// Quarterly 3(1–2):59–66, 1956). The stored EMA is `tx_unlock_potential`.
/// With uniform heal time, WSPT on the relaxation is `w_j` desc. Exact WSPT
/// when each stalled tx has a single blocker (`k = 1`, the usual chain/tree).
fn shares<'a>(
    counted: impl IntoIterator<Item = (&'a TxHash, &'a HashSet<Handle>)>,
) -> HashMap<Handle, f64> {
    let mut blockers: HashMap<TxHash, HashSet<Handle>> = HashMap::new();
    for (tx, roots) in counted {
        blockers
            .entry(tx.clone())
            .or_default()
            .extend(roots.iter().cloned());
    }
    let mut w = HashMap::new();
    for handles in blockers.values() {
        let k = handles.len() as f64;
        if k == 0.0 {
            continue;
        }
        let part = 1.0 / k;
        for handle in handles {
            *w.entry(handle.clone()).or_insert(0.0) += part;
        }
    }
    w
}

/// Write EMA sample `n`: `tx_unlock_potential = (tx_unlock_potential + n) / 2`.
/// One score per handle on `drifted_handle_demand`.
async fn update_tx_unlock_potential(
    pool: &PgPool,
    weights: &HashMap<Handle, f64>,
) -> Result<(), CoprocessorError> {
    if weights.is_empty() {
        return Ok(());
    }
    let (handles, values): (Vec<_>, Vec<_>) = weights
        .iter()
        .map(|(handle, n)| (handle.clone(), *n))
        .unzip();
    sqlx::query!(
        r#"INSERT INTO drifted_handle_demand AS d (handle, tx_unlock_potential)
           SELECT src.handle, src.n / 2.0
             FROM unnest($1::bytea[], $2::float8[]) AS src(handle, n)
           ON CONFLICT (handle) DO UPDATE
           SET tx_unlock_potential = d.tx_unlock_potential / 2.0
               + EXCLUDED.tx_unlock_potential"#,
        &handles,
        &values
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Replace last update `n1` with `n2` in the same EMA slot:
/// `s += (n2 - n1) / 2` if `n1` was written; else first update `(s + n2) / 2`.
/// Clamped at 0: a concurrent EMA step halves our `n1 / 2` before we remove
/// it, so the exact delta can overshoot below 0 (e.g. when the handle heals).
async fn replace_last_update_tx_unlock_potential(
    pool: &PgPool,
    n1: &HashMap<Handle, f64>,
    n2: &HashMap<Handle, f64>,
) -> Result<(), CoprocessorError> {
    let mut delta_handles = Vec::new();
    let mut delta_values = Vec::new();
    let mut first = HashMap::new();
    for (handle, before) in n1 {
        let after = n2.get(handle).copied().unwrap_or(0.0);
        let delta = after - before;
        if delta != 0.0 {
            delta_handles.push(handle.clone());
            delta_values.push(delta);
        }
    }
    for (handle, after) in n2 {
        if !n1.contains_key(handle) {
            first.insert(handle.clone(), *after);
        }
    }
    if !delta_handles.is_empty() {
        sqlx::query!(
            r#"UPDATE drifted_handle_demand AS d
                  SET tx_unlock_potential =
                      GREATEST(0.0, d.tx_unlock_potential + src.delta / 2.0)
                 FROM unnest($1::bytea[], $2::float8[]) AS src(handle, delta)
                WHERE d.handle = src.handle"#,
            &delta_handles,
            &delta_values
        )
        .execute(pool)
        .await?;
    }
    update_tx_unlock_potential(pool, &first).await
}
