//! Two-pass containment of ct64 drift through already-computed descendants.
//!
//! Load every ct64 drifted as propagation roots (i.e. not yet healed) of the drifted handle set.
//! For each host chain, read `computations` block by block from
//! the oldest of those roots, always from both execution stacks (`public` and
//! any `gcs-*`). An operation which is already computed and whose encrypted operand is already drifted
//! is immediately inferred drifted and added to the drifted set.
//!
//! `is_contained` is set only on the exclusive pass,
//! so no TFHE batch can compute during the scan. Inferred findings belong
//! to the consumer stack's epoch. An independent local ciphertext hides another
//! epoch's copy of the same handle.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use anyhow::{ensure, Context, Result};
use fhevm_engine_common::database::GCS_SCHEMA_PREFIX;
use fhevm_engine_common::types::{Handle, SupportedFheOperations};
use sqlx::{PgPool, Postgres, Transaction};
use tracing::error;

pub use fhevm_engine_common::drift_containment::DRIFT_CONTAINMENT_BARRIER;

type BlockHash = Vec<u8>;
type ContextId = Vec<u8>;

/// Counts from this pass, durable only after the caller commits its transaction.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct PropagationResult {
    pub inferred_handles: u64,
    pub contained_findings: u64,
}

enum ExecutionStackKind {
    Public,
    Gcs,
}

/// `public` or a `gcs-*` schema, with the consensus epoch of that schema's singleton.
struct ExecutionStack {
    kind: ExecutionStackKind,
    schema: String,
    consensus_epoch: String,
}

/// Drifted ct64 row that seeds this pass.
struct DriftedRoots {
    id: i64,
    consensus_epoch: String,
    coprocessor_context_id: ContextId,
    host_chain_id: i64,
    handle: Handle,
    block_number: i64,
}

/// Per-handle taint carried across blocks. Grows when a computed consumer is
/// inferred, and when a completed intermediate is walked through without a row.
#[derive(Clone)]
struct HandleState {
    coprocessor_context_id: ContextId,
    /// Epochs where the handle has drifted. Could be both.
    epochs: BTreeSet<String>,
}

/// DriftedHandle: roots one and also ones detected during propagation including in-memory handle (intermediate ct).
#[derive(Default)]
struct DriftedHandles {
    by_chain: HashMap<i64, HashMap<Handle, HandleState>>,
}

impl DriftedHandles {
    fn state(&self, chain: i64, handle: &[u8]) -> Option<&HandleState> {
        self.by_chain.get(&chain)?.get(handle)
    }

    fn has_finding_in_epoch(&self, chain: i64, handle: &[u8], epoch: &str) -> bool {
        self.state(chain, handle)
            .is_some_and(|s| s.epochs.contains(epoch))
    }

    fn upsert(
        &mut self,
        chain: i64,
        handle: &[u8],
        coprocessor_context_id: &[u8],
    ) -> &mut HandleState {
        self.by_chain
            .entry(chain)
            .or_default()
            .entry(handle.to_vec())
            .or_insert_with(|| HandleState {
                coprocessor_context_id: coprocessor_context_id.to_vec(),
                epochs: BTreeSet::new(),
            })
    }

    fn add_root(&mut self, finding: &DriftedRoots) {
        let state = self.upsert(
            finding.host_chain_id,
            &finding.handle,
            &finding.coprocessor_context_id,
        );
        state.epochs.insert(finding.consensus_epoch.clone());
    }

    fn add_inferred(
        &mut self,
        chain: i64,
        handle: &[u8],
        epoch: &str,
        source: &HandleState,
    ) -> bool {
        let state = self.upsert(chain, handle, &source.coprocessor_context_id);
        state.epochs.insert(epoch.to_owned())
    }

    /// Keep walking through a completed output that is not itself recorded.
    fn add_in_memory_inferred(
        &mut self,
        chain: i64,
        handle: &[u8],
        coprocessor_context_id: &[u8],
    ) -> bool {
        let handles = self.by_chain.entry(chain).or_default();
        if handles.contains_key(handle) {
            return false;
        }
        handles.insert(
            handle.to_vec(),
            HandleState {
                coprocessor_context_id: coprocessor_context_id.to_vec(),
                epochs: BTreeSet::new(),
            },
        );
        true
    }
}

/// One `computations` row of a single block on a single stack.
#[derive(Clone)]
struct ComputationEvent {
    output_handle: Handle,
    dependencies: Vec<Handle>,
    fhe_operation: i16,
    is_scalar: bool,
    is_completed: bool,
    is_allowed: bool,
    is_error: bool,
    stored: bool,
}

impl ComputationEvent {
    /// Stored ciphertext or a failed allowed op: a result that escaped freeze.
    fn already_computed(&self) -> bool {
        self.stored || (self.is_error && self.is_allowed)
    }
}

/// Inferred row to persist; identity is the handle unique key.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct InferredFinding {
    consensus_epoch: String,
    coprocessor_context_id: ContextId,
    host_chain_id: i64,
    block_number: i64,
    block_hash: BlockHash,
    handle: Handle,
    local_present: bool,
}

/// Commit optimistic propagation first, then wait for participating batches and
/// repeat under exclusivity. A fresh transaction for the protected pass releases
/// optimistic row locks before waiting for the barrier. If the protected pass
/// fails, the optimistic findings remain committed and uncontained.
/// The reactive caller invokes this when an uncontained ct64 finding remains.
pub async fn enforce_guaranteed_containment(pool: &PgPool) -> Result<PropagationResult> {
    // optimistic pass, take no lock so it can cancel the in-fligh computation at write.
    let mut optimistic_trx = pool.begin().await?;
    lock_cutover(&mut optimistic_trx).await?;
    let mut result = propagate_drift(&mut optimistic_trx, false).await?;
    optimistic_trx.commit().await?;

    if count_uncontained_ct64_drifted_handles(pool).await? == 0 {
        return Ok(result);
    }

    // Guaranteed pass: complete with any handle that was not inferred in the first pass.
    let mut guaranteed_trx = pool.begin().await?;
    log_unless_read_committed(&mut guaranteed_trx).await?;
    lock_cutover(&mut guaranteed_trx).await?;
    // this mutually exclude any in flight computation,
    // it can wait for ongoing computation to finish
    fhevm_engine_common::drift_containment::block_ct_computations(&mut guaranteed_trx).await?;
    let protected = propagate_drift(&mut guaranteed_trx, true).await?;
    guaranteed_trx.commit().await?;
    result.inferred_handles += protected.inferred_handles;
    result.contained_findings = protected.contained_findings;
    Ok(result)
}

/// Fresh statement snapshots after the exclusive barrier. Do not fail the pass:
/// Postgres defaults to READ COMMITTED, and a wrong session default should be
/// visible without aborting containment.
async fn log_unless_read_committed(trx: &mut Transaction<'_, Postgres>) -> Result<()> {
    let isolation = sqlx::query_scalar!("SHOW transaction_isolation")
        .fetch_one(trx.as_mut())
        .await?;
    if isolation.as_deref() != Some("read committed") {
        error!(
            isolation = isolation.as_deref(),
            "containment requires READ COMMITTED so the scan cannot use a pre-barrier snapshot"
        );
    }
    Ok(())
}

async fn lock_cutover(trx: &mut Transaction<'_, Postgres>) -> Result<()> {
    // Match batch lock ordering: cutover, containment, then execution rows.
    // Cutover/rollback cannot replace the dependency tables during either pass.
    sqlx::query!(
        "SELECT pg_advisory_xact_lock_shared($1)",
        fhevm_engine_common::versioning::CUTOVER_LOCK_ID
    )
    .execute(trx.as_mut())
    .await?;
    Ok(())
}

/// Load unhealed ct64, sweep each chain from its oldest finding through both
/// stacks, persist new inferred rows, then acknowledge containment iff exclusive.
async fn propagate_drift(
    trx: &mut Transaction<'_, Postgres>,
    is_lock_protected: bool,
) -> Result<PropagationResult> {
    let original_path = sqlx::query_scalar!("SHOW search_path")
        .fetch_one(trx.as_mut())
        .await?
        .context("missing search_path")?;

    let mut drifted = DriftedHandles::default();
    let mut start_block_by_chain = BTreeMap::<i64, i64>::new();
    let mut covered = BTreeSet::new();

    let roots = load_uncontained_drifted(trx).await?;

    if roots.is_empty() {
        return Ok(PropagationResult::default());
    }

    for root in roots {
        drifted.add_root(&root);
        covered.insert(root.id);
        start_block_by_chain
            .entry(root.host_chain_id)
            .and_modify(|b| *b = (*b).min(root.block_number))
            .or_insert(root.block_number);
    }

    let mut to_insert = BTreeSet::new();
    let stacks = execution_stacks(trx).await?;
    for (chain, start_block) in start_block_by_chain {
        scan_blocks_to_infer_drifted(
            trx,
            &stacks,
            chain,
            start_block,
            &mut drifted,
            &mut to_insert,
        )
        .await?;
    }

    let mut result = PropagationResult::default();
    for inferred in to_insert {
        match persist_inferred(trx, &inferred).await? {
            PersistOutcome::Inserted(id) => {
                result.inferred_handles += 1;
                covered.insert(id);
            }
            PersistOutcome::AlreadyPresent(id) => {
                covered.insert(id);
            }
            PersistOutcome::Absent => {}
        }
    }

    if is_lock_protected {
        let ids: Vec<_> = covered.into_iter().collect();
        result.contained_findings = sqlx::query!(
            "UPDATE public.drifted_handle SET is_contained = TRUE WHERE id = ANY($1) AND NOT is_contained",
            &ids,
        )
        .execute(trx.as_mut())
        .await?
        .rows_affected();
    }
    sqlx::query!("SELECT set_config('search_path', $1, true)", original_path)
        .fetch_one(trx.as_mut())
        .await?;
    Ok(result)
}

/// Blue is always `public`. Green is discovered from the catalog: a Blue binary
/// does not know Green's versioned `gcs-*` name. Epoch is the schema singleton.
async fn execution_stacks(trx: &mut Transaction<'_, Postgres>) -> Result<Vec<ExecutionStack>> {
    let mut schemas = vec!["public".to_owned()];
    let gcs_schemas = sqlx::query_scalar!(
        r#"SELECT n.nspname::text AS "schema!"
           FROM pg_namespace n
           WHERE n.nspname LIKE $1
             AND EXISTS (SELECT 1 FROM pg_class c WHERE c.relnamespace = n.oid
                         AND c.relname = 'blue_green_consensus_epoch' AND c.relkind = 'r')
           ORDER BY n.nspname"#,
        format!("{}%", GCS_SCHEMA_PREFIX),
    )
    .fetch_all(trx.as_mut())
    .await?;
    if gcs_schemas.len() > 1 {
        error!(
            count = gcs_schemas.len(),
            schemas = ?gcs_schemas,
            "containment found more than one GCS execution schema"
        );
    }
    schemas.extend(gcs_schemas);
    let mut stacks = Vec::new();
    for schema in schemas {
        set_execution_schema(trx, &schema).await?;
        let epoch = sqlx::query_scalar!(
            "SELECT consensus_epoch FROM blue_green_consensus_epoch WHERE singleton"
        )
        .fetch_optional(trx.as_mut())
        .await?;
        let kind = if &schema == "public" {
            ExecutionStackKind::Public
        } else {
            ExecutionStackKind::Gcs
        };
        if let Some(epoch) = epoch {
            stacks.push(ExecutionStack {
                kind,
                schema,
                consensus_epoch: epoch,
            });
        }
    }
    Ok(stacks)
}

pub async fn count_uncontained_ct64_drifted_handles(pool: &PgPool) -> Result<usize> {
    let n = sqlx::query_scalar!(
        r#"SELECT COUNT(*) FROM public.drifted_handle
            WHERE reason = 'ct64_mismatch' AND healed_at IS NULL AND is_contained = false
            ORDER BY id"#
    )
    .fetch_one(pool)
    .await?;
    Ok(n.unwrap_or(0) as usize)
}

async fn load_uncontained_drifted(
    trx: &mut Transaction<'_, Postgres>,
) -> Result<Vec<DriftedRoots>> {
    let rows = sqlx::query!(
        r#"SELECT id, consensus_epoch, coprocessor_context_id, host_chain_id,
                  handle, block_number, block_hash, is_contained
             FROM public.drifted_handle
            WHERE reason = 'ct64_mismatch' AND healed_at IS NULL AND is_contained = false
            ORDER BY id"#
    )
    .fetch_all(trx.as_mut())
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| DriftedRoots {
            id: row.id,
            consensus_epoch: row.consensus_epoch,
            coprocessor_context_id: row.coprocessor_context_id,
            host_chain_id: row.host_chain_id,
            handle: row.handle,
            block_number: row.block_number,
        })
        .collect())
}

/// Sweep one chain from `start_block` through every later block that still has
/// events on either stack. Re-read a block while the in-memory set grows so
/// same-block A→B is seen without a second SQL shape.
async fn scan_blocks_to_infer_drifted(
    trx: &mut Transaction<'_, Postgres>,
    stacks: &[ExecutionStack],
    chain: i64,
    start_block: i64,
    drifted: &mut DriftedHandles,
    to_insert: &mut BTreeSet<InferredFinding>,
) -> Result<()> {
    let mut blocks = BTreeSet::new();
    for stack in stacks {
        set_execution_schema(trx, &stack.schema).await?;
        let numbers = sqlx::query_scalar!(
            r#"SELECT DISTINCT block_number AS "block_number!"
                FROM computations
                WHERE host_chain_id = $1 AND block_number >= $2
                ORDER BY block_number"#,
            chain,
            start_block,
        )
        .fetch_all(trx.as_mut())
        .await?;
        blocks.extend(numbers);
    }
    for block in blocks {
        infer_drifted_on_block(trx, stacks, chain, block, drifted, to_insert).await?;
    }
    Ok(())
}

async fn infer_drifted_on_block(
    trx: &mut Transaction<'_, Postgres>,
    ordered_execution_stacks: &[ExecutionStack],
    chain: i64,
    block: i64,
    drifted: &mut DriftedHandles,
    to_insert: &mut BTreeSet<InferredFinding>,
) -> Result<()> {
    loop {
        let mut grown = false;
        for stack in ordered_execution_stacks {
            set_execution_schema(trx, &stack.schema).await?;
            let mut cache_has_independent_ct = HashMap::<Handle, bool>::new();
            let events = load_block_events(trx, chain, block).await?;
            for event in events {
                let Some(source) = has_a_drifted_operand(
                    &event,
                    chain,
                    stack,
                    drifted,
                    trx,
                    &mut cache_has_independent_ct,
                )
                .await?
                else {
                    continue;
                };
                grown |=
                    apply_event(trx, stack, chain, &event, &source, drifted, to_insert).await?;
            }
        }
        if !grown {
            break;
        }
    }
    Ok(())
}

async fn load_block_events(
    trx: &mut Transaction<'_, Postgres>,
    chain: i64,
    block: i64,
) -> Result<Vec<ComputationEvent>> {
    let rows = sqlx::query!(
        r#"SELECT c.output_handle, c.dependencies, c.fhe_operation, c.is_scalar,
                  c.is_completed, c.is_allowed, c.is_error,
                  EXISTS (SELECT 1 FROM ciphertexts ct
                          WHERE ct.handle = c.output_handle
                            AND octet_length(ct.ciphertext) > 0) AS "stored!"
             FROM computations c
            WHERE c.host_chain_id = $1 AND c.block_number = $2
            ORDER BY schedule_order"#,
        chain,
        block,
    )
    .fetch_all(trx.as_mut())
    .await?; // schedule_order orders transactions only
    let events: Vec<ComputationEvent> = rows
        .into_iter()
        .map(|row| ComputationEvent {
            output_handle: row.output_handle,
            dependencies: row.dependencies,
            fhe_operation: row.fhe_operation,
            is_scalar: row.is_scalar,
            is_completed: row.is_completed,
            is_allowed: row.is_allowed,
            is_error: row.is_error,
            stored: row.stored,
        })
        .collect();
    Ok(order_block_events(events))
}

/// Same-block consumers must see producers first. `schedule_order` is per tx.
fn order_block_events(events: Vec<ComputationEvent>) -> Vec<ComputationEvent> {
    let n = events.len();
    let mut by_handle = HashMap::with_capacity(n);
    for (i, op) in events.iter().enumerate() {
        by_handle.insert(op.output_handle.as_slice(), i);
    }
    let mut indegree = vec![0usize; n];
    let mut consumers: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, op) in events.iter().enumerate() {
        for (operand, dep) in op.dependencies.iter().enumerate() {
            if is_scalar_operand(op, operand) {
                continue;
            }
            let Some(&producer) = by_handle.get(dep.as_slice()) else {
                continue;
            };
            consumers[producer].push(i);
            indegree[i] += 1;
        }
    }
    let mut ready: std::collections::VecDeque<usize> =
        (0..n).filter(|&i| indegree[i] == 0).collect();
    let mut ordered = Vec::with_capacity(n);
    let mut placed = vec![false; n];
    while let Some(i) = ready.pop_front() {
        if placed[i] {
            continue;
        }
        placed[i] = true;
        ordered.push(events[i].clone());
        for &j in &consumers[i] {
            indegree[j] -= 1;
            if indegree[j] == 0 {
                ready.push_back(j);
            }
        }
    }
    for (i, op) in events.into_iter().enumerate() {
        if !placed[i] {
            ordered.push(op);
        }
    }
    ordered
}

// Return the first dependency that is drifted.
async fn has_a_drifted_operand(
    event: &ComputationEvent,
    chain: i64,
    stack: &ExecutionStack,
    drifted_handles: &DriftedHandles,
    trx: &mut Transaction<'_, Postgres>,
    cache_has_independent_ct: &mut HashMap<Handle, bool>,
) -> Result<Option<HandleState>> {
    for (index, dependency) in event.dependencies.iter().enumerate() {
        if is_scalar_operand(event, index) {
            continue;
        }
        let Some(drifted_handle) = drifted_handles.state(chain, dependency) else {
            continue;
        };
        // Same epoch: poisoning is always real (Public→Public or Gcs→Gcs).
        // Other epoch: GCS may have its own stored copy of the operand.
        let depend_on_a_drifted_ct = drifted_handle.epochs.contains(&stack.consensus_epoch)
            || !gcs_has_independent_copy(dependency, stack, trx, cache_has_independent_ct).await?;
        if depend_on_a_drifted_ct {
            return Ok(Some(drifted_handle.clone()));
        }
    }
    Ok(None)
}

/// First encrypted operand that is already drifted for this stack.
fn is_scalar_operand(event: &ComputationEvent, operand_index: usize) -> bool {
    let Ok(op) = SupportedFheOperations::try_from(event.fhe_operation) else {
        return false; // conservative: verification will catch it, do not infer
    };
    op.is_operand_scalar(event.is_scalar, operand_index, event.dependencies.len())
}

/// Finding on this stack always contaminates. A finding on the other stack
/// contaminates only when this stack has no stored ciphertext for the handle.
async fn gcs_has_independent_copy(
    handle: &[u8],
    stack: &ExecutionStack,
    trx: &mut Transaction<'_, Postgres>,
    independent: &mut HashMap<Handle, bool>,
) -> Result<bool> {
    if matches!(stack.kind, ExecutionStackKind::Public) {
        error!(
            ?handle,
            "Public handle cannot depend on a drifted Gcs handle"
        );
        return Ok(false);
    }
    if let Some(&has_copy) = independent.get(handle) {
        return Ok(has_copy);
    }
    let has_copy = stack_has_independent_copy(trx, handle).await?;
    independent.insert(handle.to_vec(), has_copy);
    Ok(has_copy)
}

// to check if gcs has its own computed copy
async fn stack_has_independent_copy(
    trx: &mut Transaction<'_, Postgres>,
    handle: &[u8],
) -> Result<bool> {
    let stored = sqlx::query_scalar!(
        r#"SELECT EXISTS (
                SELECT 1 FROM ciphertexts ct
                 WHERE ct.handle = $1 AND octet_length(ct.ciphertext) > 0
           ) AS "stored!""#,
        handle,
    )
    .fetch_one(trx.as_mut())
    .await?;
    Ok(stored)
}

/// Record a computed consumer as inferred, or walk through a completed
/// intermediate so later computed descendants still see the taint. Pending
/// work stays implicit (frozen).
async fn apply_event(
    trx: &mut Transaction<'_, Postgres>,
    stack: &ExecutionStack,
    chain: i64,
    event: &ComputationEvent,
    source: &HandleState,
    drifted: &mut DriftedHandles,
    to_insert: &mut BTreeSet<InferredFinding>,
) -> Result<bool> {
    // 3 cases
    // the handle is not yet computed
    // success or error
    if event.already_computed() {
        // avoid re-inserting root events
        if drifted.has_finding_in_epoch(chain, &event.output_handle, &stack.consensus_epoch) {
            return Ok(false);
        }
        let mut any = false;
        for (block_number, block_hash) in
            producer_identities(trx, chain, &event.output_handle).await?
        {
            if output_already_healed(
                trx,
                &stack.consensus_epoch,
                &source.coprocessor_context_id,
                chain,
                &event.output_handle,
                &block_hash,
            )
            .await?
            {
                continue;
            }
            to_insert.insert(InferredFinding {
                consensus_epoch: stack.consensus_epoch.clone(),
                coprocessor_context_id: source.coprocessor_context_id.clone(),
                host_chain_id: chain,
                block_number,
                block_hash,
                handle: event.output_handle.clone(),
                local_present: event.stored,
            });
            any = true;
        }
        if !any {
            return Ok(false);
        }
        return Ok(drifted.add_inferred(
            chain,
            &event.output_handle,
            &stack.consensus_epoch,
            source,
        ));
    }
    if event.is_completed {
        return Ok(drifted.add_in_memory_inferred(
            chain,
            &event.output_handle,
            &source.coprocessor_context_id,
        ));
    }
    Ok(false)
}

async fn output_already_healed(
    trx: &mut Transaction<'_, Postgres>,
    consensus_epoch: &str,
    coprocessor_context_id: &[u8],
    chain: i64,
    handle: &[u8],
    block_hash: &[u8],
) -> Result<bool> {
    let healed = sqlx::query_scalar!(
        r#"SELECT EXISTS (SELECT 1 FROM public.drifted_handle
            WHERE consensus_epoch = $1 AND coprocessor_context_id = $2
              AND host_chain_id = $3 AND handle = $4 AND block_hash = $5
              AND healed_at IS NOT NULL) AS "healed!""#,
        consensus_epoch,
        coprocessor_context_id,
        chain,
        handle,
        block_hash,
    )
    .fetch_one(trx.as_mut())
    .await?;
    Ok(healed)
}

enum PersistOutcome {
    Inserted(i64),
    AlreadyPresent(i64),
    Absent,
}

async fn persist_inferred(
    trx: &mut Transaction<'_, Postgres>,
    inferred: &InferredFinding,
) -> Result<PersistOutcome> {
    let inserted = sqlx::query_scalar!(
        r#"INSERT INTO public.drifted_handle (
            consensus_epoch, coprocessor_context_id, host_chain_id,
            block_number, block_hash, handle, detection_kind, reason,
            local_present, observed_present
        ) SELECT $1, $2, $3, $4, $5, $6, 'inferred', 'ct64_mismatch', $7, FALSE
        WHERE NOT EXISTS (
            SELECT 1 FROM public.drifted_handle
            WHERE consensus_epoch = $1 AND coprocessor_context_id = $2
              AND host_chain_id = $3 AND block_hash = $5 AND handle = $6
              AND reason = 'ct64_mismatch'
        )
        ON CONFLICT DO NOTHING RETURNING id"#,
        inferred.consensus_epoch,
        &inferred.coprocessor_context_id,
        inferred.host_chain_id,
        inferred.block_number,
        &inferred.block_hash,
        &inferred.handle,
        inferred.local_present,
    )
    .fetch_optional(trx.as_mut())
    .await?;
    if let Some(id) = inserted {
        return Ok(PersistOutcome::Inserted(id));
    }
    let existing = sqlx::query_scalar!(
        r#"SELECT id FROM public.drifted_handle
            WHERE consensus_epoch = $1 AND coprocessor_context_id = $2
              AND host_chain_id = $3 AND block_hash = $4 AND handle = $5
              AND reason = 'ct64_mismatch'"#,
        inferred.consensus_epoch,
        &inferred.coprocessor_context_id,
        inferred.host_chain_id,
        &inferred.block_hash,
        &inferred.handle,
    )
    .fetch_optional(trx.as_mut())
    .await?;
    Ok(existing
        .map(PersistOutcome::AlreadyPresent)
        .unwrap_or(PersistOutcome::Absent))
}

async fn set_execution_schema(trx: &mut Transaction<'_, Postgres>, schema: &str) -> Result<()> {
    // quote_ident handles the identifier; no interpolated SQL or session-level change.
    sqlx::query!(
        "SELECT set_config('search_path', quote_ident($1) || ', public', true)",
        schema
    )
    .fetch_one(trx.as_mut())
    .await?;
    Ok(())
}

/// Every recorded producer block. A reorg leaves more than one hash; each
/// becomes its own inferred row. No row is still an error.
async fn producer_identities(
    trx: &mut Transaction<'_, Postgres>,
    chain: i64,
    handle: &[u8],
) -> Result<Vec<(i64, BlockHash)>> {
    let rows = sqlx::query!(
        "SELECT producer_block_number, producer_block_hash FROM handle_producer_block WHERE host_chain_id = $1 AND handle = $2",
        chain,
        handle,
    )
    .fetch_all(trx.as_mut())
    .await?;
    ensure!(
        !rows.is_empty(),
        "missing producer identity for handle {} on chain {chain}",
        hex::encode(handle)
    );
    Ok(rows
        .into_iter()
        .map(|row| (row.producer_block_number, row.producer_block_hash))
        .collect())
}

#[cfg(test)]
mod propagation_tests;
