//! Handler: `CoprocUpgraded` event -> drive both BCS and GCS through cutover.
//!
//! This handler runs against either DB. On the BCS DB it transitions
//! `LIVE -> DRAINING -> STOPPED`. On the GCS DB it transitions
//! `SIGNALING -> CUTTING_OVER -> LIVE`. We pick the side by reading the
//! `service_state` row that's already in a non-trivial state.

use anyhow::{anyhow, Context, Result};
use sqlx::PgPool;
use tracing::{info, warn};

use crate::config::ConfigSettings;
use crate::handlers::UpgradeEvent;
use crate::metrics::UPGRADE_EVENT_SUCCESS_COUNTER;
use crate::notify;
use crate::state::{read_state, transition, BcsState, FsmState, GcsState, StackRole};

pub async fn handle(pool: &PgPool, _conf: &ConfigSettings, ev: &UpgradeEvent) -> Result<()> {
    let bcs = read_state(pool, StackRole::BCS).await?;
    let gcs = read_state(pool, StackRole::GCS).await?;
    info!(
        bcs = bcs.state.as_str(),
        gcs = gcs.state.as_str(),
        "CoprocUpgraded received"
    );

    match (bcs.state, gcs.state) {
        // GCS side: we were the SIGNALING party; promote to LIVE.
        (_, FsmState::Gcs(GcsState::SIGNALING)) => promote_gcs(pool, ev).await,
        // BCS side: we were LIVE; drain.
        (FsmState::Bcs(BcsState::LIVE), FsmState::Gcs(GcsState::OFFLINE)) => {
            drain_bcs(pool, ev).await
        }
        // Idempotent: already done. Log and move on.
        (FsmState::Bcs(BcsState::STOPPED), _) | (_, FsmState::Gcs(GcsState::LIVE)) => {
            warn!(
                bcs = bcs.state.as_str(),
                gcs = gcs.state.as_str(),
                "CoprocUpgraded already applied, idempotent skip"
            );
            Ok(())
        }
        _ => Err(anyhow!(
            "CoprocUpgraded received in unexpected state combo: bcs={}, gcs={}",
            bcs.state.as_str(),
            gcs.state.as_str()
        )),
    }
}

async fn drain_bcs(pool: &PgPool, ev: &UpgradeEvent) -> Result<()> {
    // BCS LIVE -> DRAINING. In v1 this is a transition + notify only;
    // BCS workers don't yet listen on `event_coproc_mngr_bcs_drain`, so
    // they continue running. Real drain semantics land in a later
    // iteration.
    transition_with_notify(
        pool,
        StackRole::BCS,
        FsmState::Bcs(BcsState::LIVE),
        FsmState::Bcs(BcsState::DRAINING),
        ev,
        Some(notify::CHAN_BCS_DRAIN),
    )
    .await?;
    info!("PLACEHOLDER: would wait for BCS in-flight queues to drain");

    // BCS DRAINING -> STOPPED.
    transition_with_notify(
        pool,
        StackRole::BCS,
        FsmState::Bcs(BcsState::DRAINING),
        FsmState::Bcs(BcsState::STOPPED),
        ev,
        None,
    )
    .await?;

    UPGRADE_EVENT_SUCCESS_COUNTER
        .with_label_values(&["CoprocUpgraded.bcs_drain"])
        .inc();
    Ok(())
}

async fn promote_gcs(pool: &PgPool, ev: &UpgradeEvent) -> Result<()> {
    // GCS SIGNALING -> CUTTING_OVER (notify gcs_promote so future tx-sender
    // flips dry-run flags).
    transition_with_notify(
        pool,
        StackRole::GCS,
        FsmState::Gcs(GcsState::SIGNALING),
        FsmState::Gcs(GcsState::CUTTING_OVER),
        ev,
        Some(notify::CHAN_GCS_PROMOTE),
    )
    .await?;
    info!("PLACEHOLDER: would catch up GCS from evalBlock to head + flip dry-run off");
    notify_only(pool, notify::CHAN_DRY_RUN_OFF, ev).await?;

    // GCS CUTTING_OVER -> LIVE.
    transition_with_notify(
        pool,
        StackRole::GCS,
        FsmState::Gcs(GcsState::CUTTING_OVER),
        FsmState::Gcs(GcsState::LIVE),
        ev,
        None,
    )
    .await?;

    UPGRADE_EVENT_SUCCESS_COUNTER
        .with_label_values(&["CoprocUpgraded.gcs_promote"])
        .inc();
    Ok(())
}

async fn transition_with_notify(
    pool: &PgPool,
    role: StackRole,
    from: FsmState,
    to: FsmState,
    ev: &UpgradeEvent,
    channel: Option<&str>,
) -> Result<()> {
    let mut tx = pool.begin().await?;
    transition(
        &mut tx,
        role,
        from,
        to,
        Some(&ev.proposal_id),
        ev.version.as_deref(),
        ev.snapshot_block,
        ev.eval_block,
        ev.state_commitment.as_deref(),
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
        role = role.as_ref(),
        from = from.as_str(),
        to = to.as_str(),
        notify_channel = channel.unwrap_or("(none)"),
        "FSM transition committed"
    );
    Ok(())
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
