//! Handler: `proposedUpgrade` event -> drive GCS through OFFLINE -> SNAPSHOTTING
//! -> REPLAYING -> READY -> SIGNALING.
//!
//! In this iteration each "do the work" step is a `pg_notify` plus a state
//! transition. The actual workers (host-listener, sns-worker, tx-sender) do
//! not yet listen on these channels, so the side-effects are observability
//! only. The state machine itself runs to completion regardless.

use std::process::Stdio;

use anyhow::{anyhow, bail, Context, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::process::Command;
use tokio::time::{sleep, Instant};
use tracing::{info, warn};

/// Table set duplicated from BCS DB into GCS DB at `snapshotBlock`. Matches
/// the snapshot list in RFC-021 § Database Migrations Isolation. Kept in
/// code so changes are deliberate and code-reviewed.
const SNAPSHOT_TABLES: &[&str] = &[
    "ciphertexts",
    "ciphertext_digest",
    "keys",
    "host_chain_blocks_valid",
    "gw_listener_last_block",
    "host_listener_poller_state",
];

use crate::commitment;
use crate::config::ConfigSettings;
use crate::handlers::UpgradeEvent;
use crate::metrics::{
    UPGRADE_EVENT_FAIL_COUNTER, UPGRADE_EVENT_SUCCESS_COUNTER,
};
use crate::notify;
use crate::readiness;
use crate::state::{transition, FsmState, GcsState, StackRole};

pub async fn handle(pool: &PgPool, conf: &ConfigSettings, ev: &UpgradeEvent) -> Result<()> {
    let snapshot_block = ev
        .snapshot_block
        .ok_or_else(|| anyhow!("proposedUpgrade missing snapshotBlock"))?;
    let eval_block = ev
        .eval_block
        .ok_or_else(|| anyhow!("proposedUpgrade missing evalBlock"))?;
    let version = ev
        .version
        .clone()
        .ok_or_else(|| anyhow!("proposedUpgrade missing version"))?;

    if eval_block <= snapshot_block {
        bail!("evalBlock ({eval_block}) must be > snapshotBlock ({snapshot_block})");
    }

    info!(
        snapshot_block,
        eval_block,
        version = %version,
        "Driving GCS through proposedUpgrade lifecycle"
    );

    // 1. Wait until BCS is fully settled at snapshotBlock. The check runs
    // against the BCS DB (read-only). The pool is opened here and closed
    // as soon as the predicate passes; coproc-mngr does not hold an open
    // BCS connection outside this window.
    {
        let bcs_pool = open_bcs_pool(conf)
            .await
            .context("opening BCS DB pool for snapshotBlock readiness check")?;
        wait_for_settled(&bcs_pool, snapshot_block, conf).await?;
        bcs_pool.close().await;
    }

    // 2. OFFLINE -> SNAPSHOTTING (+ pg_notify snapshot_start), then run
    // the actual pg_dump | pg_restore. If the snapshot fails, the FSM
    // stays in SNAPSHOTTING and the operator must drop the GCS DB and
    // re-issue the proposal (per RFC-021 § Database Migrations Isolation).
    transition_with_notify(
        pool,
        FsmState::Gcs(GcsState::OFFLINE),
        FsmState::Gcs(GcsState::SNAPSHOTTING),
        ev,
        notify::CHAN_SNAPSHOT_START,
    )
    .await?;
    run_pg_dump_restore(conf)
        .await
        .context("pg_dump | pg_restore from BCS into GCS")?;

    // 3. SNAPSHOTTING -> REPLAYING (+ pg_notify replay_start + dry_run_on).
    transition_with_notify(
        pool,
        FsmState::Gcs(GcsState::SNAPSHOTTING),
        FsmState::Gcs(GcsState::REPLAYING),
        ev,
        notify::CHAN_REPLAY_START,
    )
    .await?;
    notify_only(pool, notify::CHAN_DRY_RUN_ON, ev).await?;
    info!(
        snapshot_block,
        eval_block,
        "PLACEHOLDER: would instruct host-listener to replay (snapshotBlock, evalBlock]"
    );

    // 4. REPLAYING -> READY once GCS reaches evalBlock and is settled.
    // No-op in v1 because nothing is actually replaying. The wait loop
    // exits immediately when the predicate is trivially true on an empty
    // GCS DB.
    wait_for_settled(pool, eval_block, conf).await?;

    transition_with_notify_payload(
        pool,
        FsmState::Gcs(GcsState::REPLAYING),
        FsmState::Gcs(GcsState::READY),
        ev,
        None,
        None,
    )
    .await?;

    // 5. Compute stateCommitment over the GCS-replayed handles in
    // (snapshotBlock, evalBlock] and insert into signal_ready_pending.
    let state_commitment = commitment::compute_state_commitment(
        pool,
        snapshot_block,
        eval_block,
    )
    .await
    .context("computing stateCommitment")?;
    info!(
        proposal_id = %hex::encode(&ev.proposal_id),
        version = %version,
        state_commitment = %hex::encode(state_commitment),
        "stateCommitment ready"
    );
    insert_signal_ready_pending(pool, ev, &version, &state_commitment).await?;

    // 6. READY -> SIGNALING (+ pg_notify signal_ready).
    transition_with_notify_payload(
        pool,
        FsmState::Gcs(GcsState::READY),
        FsmState::Gcs(GcsState::SIGNALING),
        ev,
        Some(&state_commitment),
        Some(notify::CHAN_SIGNAL_READY),
    )
    .await?;

    UPGRADE_EVENT_SUCCESS_COUNTER
        .with_label_values(&["proposedUpgrade"])
        .inc();
    info!("proposedUpgrade lifecycle reached SIGNALING - awaiting CoprocUpgraded");
    Ok(())
}

async fn wait_for_settled(pool: &PgPool, block: i64, conf: &ConfigSettings) -> Result<()> {
    let deadline = Instant::now() + conf.readiness_timeout;
    loop {
        let r = readiness::check_settled_at(pool, block).await?;
        if r.fully_settled() {
            return Ok(());
        }
        if Instant::now() > deadline {
            UPGRADE_EVENT_FAIL_COUNTER
                .with_label_values(&["readiness_timeout"])
                .inc();
            bail!("readiness timeout waiting for block {block} to settle: {r:?}");
        }
        warn!(?r, block, "not yet settled, sleeping");
        sleep(conf.readiness_poll_interval).await;
    }
}

async fn transition_with_notify(
    pool: &PgPool,
    from: FsmState,
    to: FsmState,
    ev: &UpgradeEvent,
    channel: &str,
) -> Result<()> {
    transition_with_notify_payload(pool, from, to, ev, None, Some(channel)).await
}

async fn notify_only(pool: &PgPool, channel: &str, ev: &UpgradeEvent) -> Result<()> {
    let mut tx = pool.begin().await?;
    let payload = serde_json::json!({
        "proposal_id": hex::encode(&ev.proposal_id),
    })
    .to_string();
    notify::pg_notify(&mut tx, channel, &payload).await?;
    tx.commit().await?;
    Ok(())
}

async fn transition_with_notify_payload(
    pool: &PgPool,
    from: FsmState,
    to: FsmState,
    ev: &UpgradeEvent,
    state_commitment: Option<&[u8]>,
    channel: Option<&str>,
) -> Result<()> {
    let mut tx = pool.begin().await?;
    transition(
        &mut tx,
        StackRole::GCS,
        from,
        to,
        Some(&ev.proposal_id),
        ev.version.as_deref(),
        ev.snapshot_block,
        ev.eval_block,
        state_commitment,
    )
    .await
    .with_context(|| format!("transition {} -> {}", from.as_str(), to.as_str()))?;

    if let Some(chan) = channel {
        let payload = serde_json::json!({
            "proposal_id": hex::encode(&ev.proposal_id),
            "to": to.as_str(),
        })
        .to_string();
        notify::pg_notify(&mut tx, chan, &payload).await?;
    }

    tx.commit().await?;
    info!(
        proposal_id = %hex::encode(&ev.proposal_id),
        from = from.as_str(),
        to = to.as_str(),
        notify_channel = channel.unwrap_or("(none)"),
        "FSM transition committed"
    );
    Ok(())
}

async fn insert_signal_ready_pending(
    pool: &PgPool,
    ev: &UpgradeEvent,
    version: &str,
    state_commitment: &[u8],
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO signal_ready_pending (proposal_id, state_commitment, version)
        VALUES ($1, $2, $3)
        ON CONFLICT (proposal_id) DO NOTHING
        "#,
    )
    .bind(&ev.proposal_id)
    .bind(state_commitment)
    .bind(version)
    .execute(pool)
    .await?;
    Ok(())
}

/// Open a read-only-intent pool against the BCS DB. The caller is expected
/// to .close() the pool as soon as the check completes; coproc-mngr does
/// not hold a long-lived BCS connection.
async fn open_bcs_pool(conf: &crate::config::ConfigSettings) -> Result<PgPool> {
    let url = conf
        .bcs_database_url
        .as_ref()
        .ok_or_else(|| anyhow!("--bcs-database-url is required to handle proposedUpgrade"))?;
    PgPoolOptions::new()
        .max_connections(2)
        .connect(url.as_str())
        .await
        .context("connect BCS DB")
}

/// Run `pg_dump --data-only --format=custom <snapshot tables> <BCS_URL>`
/// piped into `pg_restore --data-only --dbname=<GCS_URL>`.
///
/// The schema on GCS is assumed to already exist (db-migration applied
/// independently). We only copy data for the snapshot table set. Both
/// processes are spawned with `kill_on_drop(true)` so that a cancelled
/// coproc-mngr cleans them up rather than leaking processes.
///
/// On any error: the GCS DB may be in a partially-populated state. Recovery
/// per RFC-021 is to drop the GCS DB and reissue the proposal; coproc-mngr
/// itself does not attempt cleanup.
async fn run_pg_dump_restore(conf: &crate::config::ConfigSettings) -> Result<()> {
    let bcs_url = conf
        .bcs_database_url
        .as_ref()
        .ok_or_else(|| anyhow!("--bcs-database-url is required for pg_dump"))?;
    let gcs_url = &conf.database_url;

    info!(
        snapshot_tables = ?SNAPSHOT_TABLES,
        "Starting pg_dump | pg_restore from BCS into GCS"
    );
    let started_at = Instant::now();

    // pg_dump: read from BCS, emit custom-format archive to stdout.
    let mut dump = {
        let mut cmd = Command::new("pg_dump");
        cmd.args([
            "--format=custom",
            "--no-owner",
            "--no-acl",
            "--data-only",
        ]);
        for t in SNAPSHOT_TABLES {
            cmd.arg(format!("--table={t}"));
        }
        cmd.arg(bcs_url.as_str())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .context("spawning pg_dump")?
    };

    let dump_stdout: Stdio = dump
        .stdout
        .take()
        .ok_or_else(|| anyhow!("pg_dump stdout pipe missing"))?
        .try_into()
        .context("converting pg_dump stdout to Stdio")?;

    // pg_restore: read custom-format archive on stdin, write to GCS.
    let restore = Command::new("pg_restore")
        .args(["--no-owner", "--no-acl", "--data-only", "--dbname"])
        .arg(gcs_url.as_str())
        .stdin(dump_stdout)
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .context("spawning pg_restore")?;

    // Both must run concurrently; pg_dump only finishes once pg_restore has
    // drained its stdout, and pg_restore only finishes once pg_dump has
    // closed it.
    let (dump_out, restore_out) =
        tokio::join!(dump.wait_with_output(), restore.wait_with_output());
    let dump_out = dump_out.context("waiting on pg_dump")?;
    let restore_out = restore_out.context("waiting on pg_restore")?;

    if !dump_out.status.success() {
        bail!(
            "pg_dump failed (exit={}): {}",
            dump_out.status,
            String::from_utf8_lossy(&dump_out.stderr).trim()
        );
    }
    if !restore_out.status.success() {
        bail!(
            "pg_restore failed (exit={}): {}",
            restore_out.status,
            String::from_utf8_lossy(&restore_out.stderr).trim()
        );
    }

    info!(
        elapsed_ms = started_at.elapsed().as_millis() as u64,
        "pg_dump | pg_restore completed"
    );
    Ok(())
}

