pub mod coproc_upgraded;
pub mod proposed_upgrade;

use anyhow::Result;
use sqlx::{PgPool, Row};
use tracing::{info, warn};

use crate::config::ConfigSettings;

/// One row in `upgrade_events` that the dispatcher saw via NOTIFY or polling.
#[derive(Clone, Debug)]
pub struct UpgradeEvent {
    pub id: i64,
    pub kind: EventKind,
    pub proposal_id: Vec<u8>,
    pub version: Option<String>,
    pub snapshot_block: Option<i64>,
    pub eval_block: Option<i64>,
    pub state_commitment: Option<Vec<u8>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventKind {
    ProposedUpgrade,
    CoprocUpgraded,
}

impl EventKind {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "proposedUpgrade" => Some(Self::ProposedUpgrade),
            "CoprocUpgraded" => Some(Self::CoprocUpgraded),
            _ => None,
        }
    }
}

pub async fn dispatch(pool: &PgPool, conf: &ConfigSettings, ev: UpgradeEvent) -> Result<()> {
    info!(
        kind = ?ev.kind,
        proposal_id = %hex::encode(&ev.proposal_id),
        "Handling upgrade event"
    );
    let res = match ev.kind {
        EventKind::ProposedUpgrade => proposed_upgrade::handle(pool, conf, &ev).await,
        EventKind::CoprocUpgraded => coproc_upgraded::handle(pool, conf, &ev).await,
    };
    if let Err(ref e) = res {
        warn!(error = format!("{e:#}").as_str(), "handler returned error");
    }
    mark_handled(pool, ev.id).await?;
    res
}

async fn mark_handled(pool: &PgPool, id: i64) -> Result<()> {
    sqlx::query("UPDATE upgrade_events SET handled = TRUE WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn fetch_unhandled(pool: &PgPool, limit: i64) -> Result<Vec<UpgradeEvent>> {
    let rows = sqlx::query(
        r#"
        SELECT id, kind, proposal_id, version, snapshot_block, eval_block, state_commitment
        FROM   upgrade_events
        WHERE  handled = FALSE
        ORDER  BY created_at ASC
        LIMIT  $1
        "#,
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        let kind_raw: String = r.try_get("kind")?;
        let Some(kind) = EventKind::parse(&kind_raw) else {
            warn!(kind = %kind_raw, "unknown upgrade_events.kind, skipping");
            continue;
        };
        out.push(UpgradeEvent {
            id: r.try_get("id")?,
            kind,
            proposal_id: r.try_get("proposal_id")?,
            version: r.try_get("version")?,
            snapshot_block: r.try_get("snapshot_block")?,
            eval_block: r.try_get("eval_block")?,
            state_commitment: r.try_get("state_commitment")?,
        });
    }
    Ok(out)
}
