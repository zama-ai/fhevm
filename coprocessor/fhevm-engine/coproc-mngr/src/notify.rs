//! Outbound `pg_notify` channels.
//!
//! In this iteration the only consumer of these notifies is *future* code in
//! `host-listener`, `sns-worker`, `tx-sender`, etc. coproc-mngr emits them in
//! the same Postgres transaction that commits the FSM transition; if the
//! transition is rolled back, the notify is too. (Postgres semantics: NOTIFY
//! payloads are buffered until COMMIT.)

use anyhow::Result;
use sqlx::{Postgres, Transaction};

/// Signal: the BCS is fully settled; consumers should treat the snapshot as
/// frozen and prepare for replay. Currently no consumer.
pub const CHAN_SNAPSHOT_START: &str = "event_coproc_mngr_snapshot_start";

/// Signal: GCS host-listener should ingest blocks `(snapshotBlock, evalBlock]`
/// in polling mode.
pub const CHAN_REPLAY_START: &str = "event_coproc_mngr_replay_start";

/// Signal: tx-sender should suppress AddCipher/VerifyProof and enable
/// SignalReady (dry-run mode).
pub const CHAN_DRY_RUN_ON: &str = "event_coproc_mngr_dry_run_on";

/// Signal: tx-sender should re-enable AddCipher/VerifyProof and disable
/// SignalReady (live-run mode).
pub const CHAN_DRY_RUN_OFF: &str = "event_coproc_mngr_dry_run_off";

/// Signal: a new row exists in `signal_ready_pending` for tx-sender to drain.
pub const CHAN_SIGNAL_READY: &str = "event_coproc_mngr_signal_ready";

/// Signal: BCS services should drain in-flight work and stop.
pub const CHAN_BCS_DRAIN: &str = "event_coproc_mngr_bcs_drain";

/// Signal: GCS should promote to LIVE - flip dry-run flags off.
pub const CHAN_GCS_PROMOTE: &str = "event_coproc_mngr_gcs_promote";

pub async fn pg_notify(
    tx: &mut Transaction<'_, Postgres>,
    channel: &str,
    payload: &str,
) -> Result<()> {
    sqlx::query("SELECT pg_notify($1, $2)")
        .bind(channel)
        .bind(payload)
        .execute(&mut **tx)
        .await?;
    Ok(())
}
