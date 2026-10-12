//! Durable containment for changes to committed chain history.
//!
//! State is shared by both stacks and survives their schema lifecycle. Recovery
//! is an operator action: a listener must never automatically clear a halt.
use sqlx::{PgPool, Postgres, Transaction};

use crate::versioning::{begin_write_guarded, GcsRollbackPolicy, CUTOVER_LOCK_ID};

#[derive(Clone, Copy)]
pub enum ListenerSource {
    Host,
    Gateway,
}

impl ListenerSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Host => "host",
            Self::Gateway => "gateway",
        }
    }
}

pub fn stack_role(gcs_mode: bool) -> &'static str {
    if gcs_mode {
        "GCS"
    } else {
        "BCS"
    }
}

/// Resolve the Gateway cursor's actual namespace while holding the shared
/// cutover fence. A copied StackMode can outlive promotion while waiting for
/// that fence; the pool then routes to public although the copy still says GCS.
pub async fn gateway_role_tx(tx: &mut Transaction<'_, Postgres>) -> anyhow::Result<bool> {
    anyhow::ensure!(
        !crate::versioning::is_retired(tx).await?,
        "retired Gateway stack cannot write progress"
    );
    let schema: String = sqlx::query_scalar(
        "SELECT n.nspname::text FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
         WHERE c.oid = 'gw_listener_last_block'::regclass",
    )
    .fetch_one(&mut **tx)
    .await?;
    match schema.as_str() {
        "public" => Ok(false),
        crate::database::GCS_SCHEMA => Ok(true),
        _ => anyhow::bail!("unexpected Gateway cursor schema: {schema}"),
    }
}

/// Taken after the shared cutover fence. Serialize identity checks with
/// admission for a single source/role/chain, including across replicas.
pub async fn lock_history_tx(
    tx: &mut Transaction<'_, Postgres>,
    source: ListenerSource,
    gcs_mode: bool,
    chain_id: i64,
) -> Result<(), sqlx::Error> {
    let key = format!(
        "listener-history:{}:{}:{chain_id}",
        source.as_str(),
        stack_role(gcs_mode)
    );
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1, 0))")
        .bind(key)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

pub struct HistoryConflict {
    pub block_number: i64,
    pub expected_hash: Vec<u8>,
    pub observed_hash: Vec<u8>,
    pub reason: &'static str,
}

impl HistoryConflict {
    pub fn observation(
        &self,
        source: ListenerSource,
        gcs_mode: bool,
        chain_id: i64,
    ) -> HistoryMismatch<'_> {
        HistoryMismatch {
            source,
            gcs_mode,
            chain_id,
            block_number: self.block_number,
            expected_hash: Some(&self.expected_hash),
            observed_hash: Some(&self.observed_hash),
            reason: self.reason,
        }
    }
}

pub async fn is_halted(pool: &PgPool) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM public.chain_recovery_halts WHERE resolved_at IS NULL)",
    )
    .fetch_one(pool)
    .await
}

pub async fn is_halted_tx(tx: &mut Transaction<'_, Postgres>) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM public.chain_recovery_halts WHERE resolved_at IS NULL)",
    )
    .fetch_one(&mut **tx)
    .await
}

pub async fn readiness_blocked(pool: &PgPool) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM public.chain_recovery_halts WHERE resolved_at IS NULL)
             OR EXISTS(SELECT 1 FROM public.listener_replay_state WHERE pending)",
    )
    .fetch_one(pool)
    .await
}

pub async fn replay_pending_tx(tx: &mut Transaction<'_, Postgres>) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM public.listener_replay_state WHERE pending)")
        .fetch_one(&mut **tx)
        .await
}

pub struct HistoryMismatch<'a> {
    pub source: ListenerSource,
    pub gcs_mode: bool,
    pub chain_id: i64,
    pub block_number: i64,
    pub expected_hash: Option<&'a [u8]>,
    pub observed_hash: Option<&'a [u8]>,
    pub reason: &'a str,
}

/// Drain fenced writers before committing the halt. A committed halt prevents
/// all subsequent guarded ingestion, result acceptance and publication writes.
pub async fn record_halt(pool: &PgPool, mismatch: HistoryMismatch<'_>) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(CUTOVER_LOCK_ID)
        .execute(&mut *tx)
        .await?;
    record_halt_tx(&mut tx, mismatch).await?;
    tx.commit().await
}

/// The caller must already hold CUTOVER_LOCK_ID exclusively. Used when a
/// controller discovers a conflict while reconciling promotion state.
pub async fn record_halt_tx(
    tx: &mut Transaction<'_, Postgres>,
    mismatch: HistoryMismatch<'_>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO public.chain_recovery_halts
             (source, stack_role, chain_id, block_number, expected_hash, observed_hash, reason)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         ON CONFLICT (source, stack_role, chain_id, block_number) WHERE resolved_at IS NULL
         DO UPDATE SET last_detected_at = NOW(), observed_hash = EXCLUDED.observed_hash,
                       reason = EXCLUDED.reason",
    )
    .bind(mismatch.source.as_str())
    .bind(stack_role(mismatch.gcs_mode))
    .bind(mismatch.chain_id)
    .bind(mismatch.block_number)
    .bind(mismatch.expected_hash)
    .bind(mismatch.observed_hash)
    .bind(mismatch.reason)
    .execute(&mut **tx)
    .await?;
    // Revoke the persisted Gateway release latch as well as closing readiness.
    // This does not repair or delete any proof that might already be consumed.
    sqlx::query(
        "UPDATE public.upgrade_state SET gw_dry_run_started = FALSE, updated_at = NOW()
                 WHERE stack_role = 'GCS' AND status = 'in_progress'",
    )
    .execute(&mut **tx)
    .await?;
    for channel in [crate::gcs_activation::EVENT_GW_NEW_BLOCK, "event_new_block"] {
        sqlx::query("SELECT pg_notify($1, 'chain recovery halt')")
            .bind(channel)
            .execute(&mut **tx)
            .await?;
    }
    Ok(())
}

#[derive(sqlx::FromRow)]
pub struct ReplayProgress {
    pub from_block: i64,
    pub to_block: i64,
    pub pending: bool,
    pub generation: i64,
}

/// Ownership is captured under the same fence as the startup cursor. Completion
/// must retain it even if a notification changes the process's current mode.
pub struct HostReplay {
    pub progress: ReplayProgress,
    gcs_mode: bool,
    chain_id: i64,
}

impl HostReplay {
    /// Returns whether this generation still owns an unfinished obligation.
    pub async fn complete(&self, pool: &PgPool, through_block: i64) -> anyhow::Result<bool> {
        let progress = complete_replay(
            pool,
            ListenerSource::Host,
            self.gcs_mode,
            self.chain_id,
            self.progress.from_block,
            through_block,
            self.progress.generation,
        )
        .await?;
        Ok(progress.pending && progress.generation == self.progress.generation)
    }
}

pub enum HostReplayStartup {
    /// Resolve the configured initial height outside the database fence, then
    /// retry: another startup or promotion may establish a cursor meanwhile.
    NeedsSeed,
    Ready {
        cursor: i64,
        replay: Option<HostReplay>,
    },
}

/// Read/initialize the routed host cursor and adopt its replay atomically.
/// A copied Green hint may be stale after waiting for promotion; only the
/// actual namespace under the fence determines ownership and the replay range.
pub async fn prepare_host_replay(
    pool: &PgPool,
    gcs_hint: bool,
    chain_id: i64,
    replay_window: u64,
    seed: Option<i64>,
) -> anyhow::Result<HostReplayStartup> {
    anyhow::ensure!(
        (1..=10_000).contains(&replay_window),
        "invalid host replay window"
    );
    let Some(mut tx) = begin_write_guarded(pool, gcs_hint, GcsRollbackPolicy::RawIngestion)
        .await?
        .into_tx()
    else {
        anyhow::bail!("chain recovery or stack lifecycle blocks host startup");
    };
    anyhow::ensure!(
        !crate::versioning::is_retired(&mut tx).await?,
        "retired host stack cannot register replay"
    );
    let schema: String = sqlx::query_scalar(
        "SELECT n.nspname::text FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
         WHERE c.oid = 'host_listener_poller_state'::regclass",
    )
    .fetch_one(&mut *tx)
    .await?;
    let gcs_mode = match schema.as_str() {
        "public" => false,
        crate::database::GCS_SCHEMA => true,
        _ => anyhow::bail!("unexpected host cursor schema: {schema}"),
    };
    lock_history_tx(&mut tx, ListenerSource::Host, gcs_mode, chain_id).await?;
    let cursor: Option<i64> = sqlx::query_scalar(
        "SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id = $1 FOR UPDATE",
    )
    .bind(chain_id)
    .fetch_optional(&mut *tx)
    .await?;
    let (cursor, seeded) = if let Some(cursor) = cursor {
        (cursor, false)
    } else {
        let Some(seed) = seed else {
            tx.rollback().await?;
            return Ok(HostReplayStartup::NeedsSeed);
        };
        anyhow::ensure!(seed >= 0, "host seed cannot be negative");
        let inserted = sqlx::query(
            "INSERT INTO host_listener_poller_state (chain_id, last_caught_up_block)
             VALUES ($1, $2) ON CONFLICT (chain_id) DO NOTHING",
        )
        .bind(chain_id)
        .bind(seed)
        .execute(&mut *tx)
        .await?
        .rows_affected();
        let cursor: i64 = sqlx::query_scalar(
            "SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id = $1 FOR UPDATE",
        )
        .bind(chain_id)
        .fetch_one(&mut *tx)
        .await?;
        (cursor, inserted == 1)
    };
    anyhow::ensure!(cursor >= 0, "last_caught_up_block cannot be negative");
    let pending: Option<ReplayProgress> = sqlx::query_as(
        "SELECT from_block, to_block, pending, generation FROM public.listener_replay_state
         WHERE source = 'host' AND stack_role = $1 AND chain_id = $2 AND pending",
    )
    .bind(stack_role(gcs_mode))
    .bind(chain_id)
    .fetch_optional(&mut *tx)
    .await?;
    let window = if !seeded && cursor > 0 {
        Some(((cursor - (replay_window as i64 - 1)).max(1), cursor))
    } else {
        pending.map(|progress| (progress.from_block, progress.to_block))
    };
    let replay = if let Some((from_block, to_block)) = window {
        Some(HostReplay {
            progress: begin_replay_tx(
                &mut tx,
                ListenerSource::Host,
                gcs_mode,
                chain_id,
                from_block,
                to_block,
            )
            .await?,
            gcs_mode,
            chain_id,
        })
    } else {
        None
    };
    tx.commit().await?;
    Ok(HostReplayStartup::Ready { cursor, replay })
}

pub async fn pending_replay(
    pool: &PgPool,
    source: ListenerSource,
    gcs_mode: bool,
    chain_id: i64,
) -> Result<Option<ReplayProgress>, sqlx::Error> {
    sqlx::query_as(
        "SELECT from_block, to_block, pending, generation FROM public.listener_replay_state
         WHERE source = $1 AND stack_role = $2 AND chain_id = $3 AND pending",
    )
    .bind(source.as_str())
    .bind(stack_role(gcs_mode))
    .bind(chain_id)
    .fetch_optional(pool)
    .await
}

/// The caller holds the exclusive cutover lock and discards the green schema
/// in this same transaction. Invalidate completion tokens, retaining the audit
/// row and all genuine history halts. Blue's obligations are unaffected.
pub async fn reset_green_replay_tx(tx: &mut Transaction<'_, Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE public.listener_replay_state
         SET pending = FALSE, generation = generation + 1, updated_at = NOW()
         WHERE stack_role = 'GCS'",
    )
    .execute(&mut **tx)
    .await?;
    Ok(())
}

async fn replay_progress(
    tx: &mut Transaction<'_, Postgres>,
    source: ListenerSource,
    gcs_mode: bool,
    chain_id: i64,
) -> anyhow::Result<ReplayProgress> {
    let progress: ReplayProgress = sqlx::query_as(
        "SELECT from_block, to_block, pending, generation FROM public.listener_replay_state
         WHERE source = $1 AND stack_role = $2 AND chain_id = $3",
    )
    .bind(source.as_str())
    .bind(stack_role(gcs_mode))
    .bind(chain_id)
    .fetch_one(&mut **tx)
    .await?;
    anyhow::ensure!(
        progress.to_block - progress.from_block < 10_000,
        "unfinished replay exceeds 10000 blocks; operator recovery required"
    );
    Ok(progress)
}

/// Adopt unfinished ranges atomically and create a new completion generation.
/// Earlier replicas cannot release this replay, even with the same range.
pub async fn begin_replay(
    pool: &PgPool,
    source: ListenerSource,
    gcs_mode: bool,
    chain_id: i64,
    from_block: i64,
    to_block: i64,
) -> anyhow::Result<ReplayProgress> {
    let Some(mut tx) = begin_write_guarded(pool, gcs_mode, GcsRollbackPolicy::RawIngestion)
        .await?
        .into_tx()
    else {
        anyhow::bail!("chain recovery or stack lifecycle blocks replay progress");
    };
    let progress =
        begin_replay_tx(&mut tx, source, gcs_mode, chain_id, from_block, to_block).await?;
    tx.commit().await?;
    Ok(progress)
}

/// Explicit-owner primitive. Production host startup resolves the namespace
/// before calling this, holding the cutover fence throughout cursor selection.
async fn begin_replay_tx(
    tx: &mut Transaction<'_, Postgres>,
    source: ListenerSource,
    gcs_mode: bool,
    chain_id: i64,
    from_block: i64,
    to_block: i64,
) -> anyhow::Result<ReplayProgress> {
    sqlx::query(
        "INSERT INTO public.listener_replay_state
            (source, stack_role, chain_id, from_block, to_block, pending, generation)
         VALUES ($1, $2, $3, $4, $5, TRUE, 1)
         ON CONFLICT (source, stack_role, chain_id) DO UPDATE
             SET from_block = CASE WHEN listener_replay_state.pending
                     THEN LEAST(listener_replay_state.from_block, EXCLUDED.from_block)
                     ELSE EXCLUDED.from_block END,
                 to_block = CASE WHEN listener_replay_state.pending
                     THEN GREATEST(listener_replay_state.to_block, EXCLUDED.to_block)
                     ELSE EXCLUDED.to_block END,
                 pending = TRUE, generation = listener_replay_state.generation + 1,
                 updated_at = NOW()",
    )
    .bind(source.as_str())
    .bind(stack_role(gcs_mode))
    .bind(chain_id)
    .bind(from_block)
    .bind(to_block)
    .execute(&mut **tx)
    .await?;
    replay_progress(tx, source, gcs_mode, chain_id).await
}

pub async fn complete_replay(
    pool: &PgPool,
    source: ListenerSource,
    gcs_mode: bool,
    chain_id: i64,
    from_block: i64,
    to_block: i64,
    generation: i64,
) -> anyhow::Result<ReplayProgress> {
    let Some(mut tx) = begin_write_guarded(pool, gcs_mode, GcsRollbackPolicy::RawIngestion)
        .await?
        .into_tx()
    else {
        anyhow::bail!("chain recovery or stack lifecycle blocks replay progress");
    };
    sqlx::query(
        "UPDATE public.listener_replay_state SET pending = FALSE, updated_at = NOW()
         WHERE source = $1 AND stack_role = $2 AND chain_id = $3
           AND from_block >= $4 AND to_block <= $5 AND generation = $6",
    )
    .bind(source.as_str())
    .bind(stack_role(gcs_mode))
    .bind(chain_id)
    .bind(from_block)
    .bind(to_block)
    .bind(generation)
    .execute(&mut *tx)
    .await?;
    let progress = replay_progress(&mut tx, source, gcs_mode, chain_id).await?;
    tx.commit().await?;
    Ok(progress)
}

pub async fn checkpoint_hash(
    pool: &PgPool,
    source: ListenerSource,
    gcs_mode: bool,
    chain_id: i64,
    block_number: i64,
) -> Result<Option<Vec<u8>>, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT block_hash FROM public.listener_block_checkpoints
        WHERE source = $1 AND stack_role = $2 AND chain_id = $3 AND block_number = $4",
    )
    .bind(source.as_str())
    .bind(stack_role(gcs_mode))
    .bind(chain_id)
    .bind(block_number)
    .fetch_optional(pool)
    .await
}

#[derive(Clone, Debug)]
pub struct ListenerHeader {
    pub number: i64,
    pub hash: Vec<u8>,
    pub parent_hash: Vec<u8>,
}

/// Pin a coherent range in the caller's transaction, including its link to
/// both retained neighbors. Callers roll back before recording a conflict:
/// containment needs the exclusive fence, never a shared-to-exclusive upgrade.
pub async fn pin_headers_tx(
    tx: &mut Transaction<'_, Postgres>,
    source: ListenerSource,
    gcs_mode: bool,
    chain_id: i64,
    headers: &[ListenerHeader],
) -> Result<Option<HistoryConflict>, sqlx::Error> {
    lock_history_tx(tx, source, gcs_mode, chain_id).await?;
    for header in headers {
        let rows: Vec<(i64, Vec<u8>, Option<Vec<u8>>)> = sqlx::query_as(
            "SELECT block_number, block_hash, parent_hash FROM public.listener_block_checkpoints
             WHERE source = $1 AND stack_role = $2 AND chain_id = $3
               AND block_number BETWEEN $4 - 1 AND $4 + 1",
        )
        .bind(source.as_str())
        .bind(stack_role(gcs_mode))
        .bind(chain_id)
        .bind(header.number)
        .fetch_all(&mut **tx)
        .await?;
        for (number, hash, parent) in rows {
            let (boundary, expected, observed) = if number == header.number {
                (number, hash, header.hash.clone())
            } else if number == header.number - 1 {
                (number, hash, header.parent_hash.clone())
            } else if let Some(parent) = parent {
                (header.number, parent, header.hash.clone())
            } else {
                continue;
            };
            if expected != observed {
                return Ok(Some(HistoryConflict {
                    block_number: boundary,
                    expected_hash: expected,
                    observed_hash: observed,
                    reason: "canonical listener checkpoint or parent changed after admission",
                }));
            }
        }
        sqlx::query(
            "INSERT INTO public.listener_block_checkpoints
                (source, stack_role, chain_id, block_number, block_hash, parent_hash)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (source, stack_role, chain_id, block_number)
             DO UPDATE SET parent_hash = COALESCE(listener_block_checkpoints.parent_hash, EXCLUDED.parent_hash)",
        ).bind(source.as_str()).bind(stack_role(gcs_mode)).bind(chain_id)
            .bind(header.number).bind(&header.hash).bind(&header.parent_hash)
            .execute(&mut **tx).await?;
    }
    Ok(None)
}

pub async fn save_checkpoints(
    pool: &PgPool,
    source: ListenerSource,
    gcs_mode: bool,
    chain_id: i64,
    headers: &[ListenerHeader],
) -> anyhow::Result<()> {
    // An incoherent response that has never been admitted is retryable. Do
    // not turn its internal discontinuity into a durable history halt.
    for pair in headers.windows(2) {
        anyhow::ensure!(
            pair[0].number.checked_add(1) == Some(pair[1].number)
                && pair[1].parent_hash == pair[0].hash,
            "listener headers do not form one contiguous canonical range"
        );
    }
    let Some(mut tx) = begin_write_guarded(pool, gcs_mode, GcsRollbackPolicy::RawIngestion)
        .await?
        .into_tx()
    else {
        anyhow::bail!("chain recovery or stack lifecycle blocks checkpoint acceptance");
    };
    let gcs_mode = match source {
        ListenerSource::Gateway => gateway_role_tx(&mut tx).await?,
        ListenerSource::Host => gcs_mode,
    };
    if let Some(conflict) = pin_headers_tx(&mut tx, source, gcs_mode, chain_id, headers).await? {
        tx.rollback().await?;
        record_halt(pool, conflict.observation(source, gcs_mode, chain_id)).await?;
        anyhow::bail!("Committed chain checkpoint changed; operator recovery required");
    }
    tx.commit().await?;
    Ok(())
}
