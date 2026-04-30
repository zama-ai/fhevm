//! FSM persisted in the `service_state` table.
//!
//! Two rows exist on every coprocessor DB instance, one per stack role.
//! Only the row matching this DB's actual role is mutated. (BCS DB only
//! transitions BCS states; GCS DB only transitions GCS states.)
//!
//! Every transition is committed in the same SQL transaction as the
//! corresponding side effect (a pg_notify, an INSERT into
//! `signal_ready_pending`, etc.) so a crash mid-transition is atomic from
//! the system's point of view: either both happen or neither.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Row, Transaction};
use strum::{AsRefStr, Display, EnumString};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Display, EnumString, AsRefStr, Serialize, Deserialize)]
#[strum(serialize_all = "UPPERCASE")]
pub enum StackRole {
    BCS,
    GCS,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Display, EnumString, AsRefStr, Serialize, Deserialize)]
pub enum BcsState {
    LIVE,
    DRAINING,
    STOPPED,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Display, EnumString, AsRefStr, Serialize, Deserialize)]
pub enum GcsState {
    OFFLINE,
    SNAPSHOTTING,
    REPLAYING,
    READY,
    SIGNALING,
    CUTTING_OVER,
    LIVE,
}

/// What state a row can be in. The same column holds either set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FsmState {
    Bcs(BcsState),
    Gcs(GcsState),
}

impl FsmState {
    pub fn parse(role: StackRole, raw: &str) -> Result<Self> {
        match role {
            StackRole::BCS => Ok(FsmState::Bcs(
                raw.parse().map_err(|_| anyhow!("invalid BCS state: {raw}"))?,
            )),
            StackRole::GCS => Ok(FsmState::Gcs(
                raw.parse().map_err(|_| anyhow!("invalid GCS state: {raw}"))?,
            )),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            FsmState::Bcs(BcsState::LIVE) => "LIVE",
            FsmState::Bcs(BcsState::DRAINING) => "DRAINING",
            FsmState::Bcs(BcsState::STOPPED) => "STOPPED",
            FsmState::Gcs(GcsState::OFFLINE) => "OFFLINE",
            FsmState::Gcs(GcsState::SNAPSHOTTING) => "SNAPSHOTTING",
            FsmState::Gcs(GcsState::REPLAYING) => "REPLAYING",
            FsmState::Gcs(GcsState::READY) => "READY",
            FsmState::Gcs(GcsState::SIGNALING) => "SIGNALING",
            FsmState::Gcs(GcsState::CUTTING_OVER) => "CUTTING_OVER",
            FsmState::Gcs(GcsState::LIVE) => "LIVE",
        }
    }
}

/// Validate a transition. Encodes the allowed edges of the FSM.
pub fn is_legal_transition(from: FsmState, to: FsmState) -> bool {
    match (from, to) {
        // BCS lifecycle on CoprocUpgraded.
        (FsmState::Bcs(BcsState::LIVE), FsmState::Bcs(BcsState::DRAINING)) => true,
        (FsmState::Bcs(BcsState::DRAINING), FsmState::Bcs(BcsState::STOPPED)) => true,

        // GCS lifecycle on proposedUpgrade through to LIVE.
        (FsmState::Gcs(GcsState::OFFLINE), FsmState::Gcs(GcsState::SNAPSHOTTING)) => true,
        (FsmState::Gcs(GcsState::SNAPSHOTTING), FsmState::Gcs(GcsState::REPLAYING)) => true,
        (FsmState::Gcs(GcsState::REPLAYING), FsmState::Gcs(GcsState::READY)) => true,
        (FsmState::Gcs(GcsState::READY), FsmState::Gcs(GcsState::SIGNALING)) => true,
        (FsmState::Gcs(GcsState::SIGNALING), FsmState::Gcs(GcsState::CUTTING_OVER)) => true,
        (FsmState::Gcs(GcsState::CUTTING_OVER), FsmState::Gcs(GcsState::LIVE)) => true,

        _ => false,
    }
}

/// One row of `service_state`.
#[derive(Clone, Debug)]
pub struct ServiceStateRow {
    pub stack_role: StackRole,
    pub state: FsmState,
    pub proposal_id: Option<Vec<u8>>,
    pub version: Option<String>,
    pub snapshot_block: Option<i64>,
    pub eval_block: Option<i64>,
    pub state_commitment: Option<Vec<u8>>,
}

pub async fn read_state(pool: &PgPool, role: StackRole) -> Result<ServiceStateRow> {
    let row = sqlx::query(
        r#"
        SELECT state,
               proposal_id,
               version,
               snapshot_block,
               eval_block,
               state_commitment
        FROM service_state
        WHERE stack_role = $1
        "#,
    )
    .bind(role.as_ref())
    .fetch_one(pool)
    .await
    .with_context(|| format!("read_state({role:?})"))?;

    let state_raw: String = row.try_get("state")?;
    Ok(ServiceStateRow {
        stack_role: role,
        state: FsmState::parse(role, &state_raw)?,
        proposal_id: row.try_get("proposal_id")?,
        version: row.try_get("version")?,
        snapshot_block: row.try_get("snapshot_block")?,
        eval_block: row.try_get("eval_block")?,
        state_commitment: row.try_get("state_commitment")?,
    })
}

/// Atomically advance the FSM. The caller must already have validated the
/// transition with `is_legal_transition`; this function re-checks under the
/// row's `FOR UPDATE` lock to handle racing writers.
pub async fn transition(
    tx: &mut Transaction<'_, Postgres>,
    role: StackRole,
    expected_from: FsmState,
    to: FsmState,
    proposal_id: Option<&[u8]>,
    version: Option<&str>,
    snapshot_block: Option<i64>,
    eval_block: Option<i64>,
    state_commitment: Option<&[u8]>,
) -> Result<()> {
    if !is_legal_transition(expected_from, to) {
        return Err(anyhow!(
            "illegal transition {} -> {}",
            expected_from.as_str(),
            to.as_str()
        ));
    }

    let cur = sqlx::query(
        r#"SELECT state FROM service_state WHERE stack_role = $1 FOR UPDATE"#,
    )
    .bind(role.as_ref())
    .fetch_one(&mut **tx)
    .await?;

    let cur_raw: String = cur.try_get("state")?;
    let cur_state = FsmState::parse(role, &cur_raw)?;
    if cur_state != expected_from {
        return Err(anyhow!(
            "concurrent transition: expected {} but row is {}",
            expected_from.as_str(),
            cur_state.as_str()
        ));
    }

    sqlx::query(
        r#"
        UPDATE service_state
        SET    state            = $2,
               proposal_id      = COALESCE($3, proposal_id),
               version          = COALESCE($4, version),
               snapshot_block   = COALESCE($5, snapshot_block),
               eval_block       = COALESCE($6, eval_block),
               state_commitment = COALESCE($7, state_commitment),
               updated_at       = NOW()
        WHERE  stack_role = $1
        "#,
    )
    .bind(role.as_ref())
    .bind(to.as_str())
    .bind(proposal_id)
    .bind(version)
    .bind(snapshot_block)
    .bind(eval_block)
    .bind(state_commitment)
    .execute(&mut **tx)
    .await?;

    Ok(())
}
