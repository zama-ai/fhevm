//! Durable containment for changes to committed chain history.
//!
//! State is shared by both stacks and survives their schema lifecycle. Recovery
//! is an operator action: a listener must never automatically clear a halt.
use sqlx::{PgPool, Postgres, Transaction};

use crate::versioning::{begin_write_guarded, GcsRollbackPolicy, CUTOVER_LOCK_ID};

/// Wake readiness waiters when recovery changes independently of chain activity.
pub const EVENT_RECOVERY_CHANGED: &str = "event_chain_recovery_changed";

#[derive(Clone, Copy)]
pub enum ListenerSource {
    Host,
}

impl ListenerSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Host => "host",
        }
    }
}

/// Resolve host ownership from the actual cursor namespace under the shared
/// cutover fence. StackMode notifications may lag behind schema promotion.
pub async fn host_role_tx(tx: &mut Transaction<'_, Postgres>) -> Result<bool, sqlx::Error> {
    if crate::versioning::is_retired(tx).await? {
        return Err(sqlx::Error::Protocol(
            "retired host stack cannot admit history".into(),
        ));
    }
    let schema: String = sqlx::query_scalar(
        "SELECT n.nspname::text FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
         WHERE c.oid = 'host_listener_poller_state'::regclass",
    )
    .fetch_one(&mut **tx)
    .await?;
    match schema.as_str() {
        "public" => Ok(false),
        crate::database::GCS_SCHEMA => Ok(true),
        _ => Err(sqlx::Error::Protocol(format!(
            "unexpected host cursor schema: {schema}"
        ))),
    }
}

pub fn stack_role(gcs_mode: bool) -> &'static str {
    if gcs_mode {
        "GCS"
    } else {
        "BCS"
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
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1, 0))")
        .bind(history_lock_key(source, gcs_mode, chain_id))
        .execute(&mut **tx)
        .await?;
    Ok(())
}

/// Background publication skips a busy chain instead of retaining a connection
/// while ingestion (or another publisher) owns its history.
pub async fn try_lock_history_tx(
    tx: &mut Transaction<'_, Postgres>,
    source: ListenerSource,
    gcs_mode: bool,
    chain_id: i64,
) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar("SELECT pg_try_advisory_xact_lock(hashtextextended($1, 0))")
        .bind(history_lock_key(source, gcs_mode, chain_id))
        .fetch_one(&mut **tx)
        .await
}

fn history_lock_key(source: ListenerSource, gcs_mode: bool, chain_id: i64) -> String {
    format!(
        "listener-history:{}:{}:{chain_id}",
        source.as_str(),
        stack_role(gcs_mode)
    )
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
    .execute(&mut *tx)
    .await?;
    // Revoke the persisted Gateway release latch as well as closing readiness.
    // This does not repair or delete any proof that might already be consumed.
    sqlx::query(
        "UPDATE public.upgrade_state SET gw_dry_run_started = FALSE, updated_at = NOW()
                 WHERE stack_role = 'GCS' AND status = 'in_progress'",
    )
    .execute(&mut *tx)
    .await?;
    for channel in [crate::gcs_activation::EVENT_GW_NEW_BLOCK, "event_new_block"] {
        sqlx::query("SELECT pg_notify($1, 'chain recovery halt')")
            .bind(channel)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    tracing::error!(
        event = "chain_history_halt",
        source = mismatch.source.as_str(),
        stack_role = stack_role(mismatch.gcs_mode),
        chain_id = mismatch.chain_id,
        block_number = mismatch.block_number,
        reason = mismatch.reason,
        "Committed history conflict contained; audited operator recovery required"
    );
    Ok(())
}

#[derive(Debug, PartialEq, Eq, sqlx::FromRow)]
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
    fn new(mut progress: ReplayProgress, gcs_mode: bool, chain_id: i64) -> Self {
        // Keep the full obligation durable; an adopter works a bounded prefix.
        progress.to_block = progress
            .to_block
            .min(progress.from_block.saturating_add(9_999));
        Self {
            progress,
            gcs_mode,
            chain_id,
        }
    }

    pub fn same_chunk(&self, other: &Self) -> bool {
        self.gcs_mode == other.gcs_mode
            && self.chain_id == other.chain_id
            && self.progress == other.progress
    }

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
    let Some(mut tx) = begin_write_guarded(pool, gcs_hint, GcsRollbackPolicy::Continue)
        .await?
        .into_tx()
    else {
        anyhow::bail!("chain recovery or stack lifecycle blocks host startup");
    };
    let gcs_mode = host_role_tx(&mut tx).await?;
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
        Some(HostReplay::new(
            begin_replay_tx(
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
        ))
    } else {
        None
    };
    tx.commit().await?;
    Ok(HostReplayStartup::Ready { cursor, replay })
}

/// Join an existing generation without superseding another replica. Running
/// pollers must observe new obligations too: the pod that started one may die.
/// Namespace resolution and inspection use the same fence as startup.
pub async fn pending_host_replay(
    pool: &PgPool,
    gcs_hint: bool,
    chain_id: i64,
) -> anyhow::Result<Option<HostReplay>> {
    let Some(mut tx) = begin_write_guarded(pool, gcs_hint, GcsRollbackPolicy::Continue)
        .await?
        .into_tx()
    else {
        anyhow::bail!("chain recovery or stack lifecycle blocks replay adoption");
    };
    let gcs_mode = host_role_tx(&mut tx).await?;
    let progress: Option<ReplayProgress> = sqlx::query_as(
        "SELECT from_block, to_block, pending, generation FROM public.listener_replay_state
         WHERE source = 'host' AND stack_role = $1 AND chain_id = $2 AND pending",
    )
    .bind(stack_role(gcs_mode))
    .bind(chain_id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(progress.map(|progress| HostReplay::new(progress, gcs_mode, chain_id)))
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
    let changed = sqlx::query(
        "UPDATE public.listener_replay_state
         SET pending = FALSE, generation = generation + 1, updated_at = NOW()
         WHERE stack_role = 'GCS'",
    )
    .execute(&mut **tx)
    .await?
    .rows_affected();
    if changed > 0 {
        sqlx::query("SELECT pg_notify($1, '')")
            .bind(EVENT_RECOVERY_CHANGED)
            .execute(&mut **tx)
            .await?;
    }
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
    let Some(mut tx) = begin_write_guarded(pool, gcs_mode, GcsRollbackPolicy::Continue)
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
    let Some(mut tx) = begin_write_guarded(pool, gcs_mode, GcsRollbackPolicy::Continue)
        .await?
        .into_tx()
    else {
        anyhow::bail!("chain recovery or stack lifecycle blocks replay progress");
    };
    let changed = sqlx::query(
        "UPDATE public.listener_replay_state
         SET pending = to_block > $5,
             from_block = CASE WHEN to_block > $5 THEN $5 + 1 ELSE from_block END,
             updated_at = NOW()
         WHERE source = $1 AND stack_role = $2 AND chain_id = $3
           AND from_block >= $4 AND from_block <= $5 AND generation = $6 AND pending",
    )
    .bind(source.as_str())
    .bind(stack_role(gcs_mode))
    .bind(chain_id)
    .bind(from_block)
    .bind(to_block)
    .bind(generation)
    .execute(&mut *tx)
    .await?
    .rows_affected();
    if changed > 0 {
        // Transactional delivery: adopters observe the remaining range, and
        // host/Gateway waiters observe completion even without another block.
        sqlx::query("SELECT pg_notify($1, '')")
            .bind(EVENT_RECOVERY_CHANGED)
            .execute(&mut *tx)
            .await?;
    }
    let progress = replay_progress(&mut tx, source, gcs_mode, chain_id).await?;
    tx.commit().await?;
    Ok(progress)
}
