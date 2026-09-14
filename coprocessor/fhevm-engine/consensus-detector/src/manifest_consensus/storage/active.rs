//! Read the consensus_epoch selected for this stack and activate Green atomically.

use std::{sync::RwLock, time::Duration};

use block_manifest::LEGACY_CONSENSUS_EPOCH;
use fhevm_engine_common::{
    gcs_activation::{EVENT_DRY_RUN_ROLLED_BACK, EVENT_DRY_RUN_STARTED, EVENT_UPGRADE_ACTIVATED},
    versions_equal, STACK_VERSION,
};
use sqlx::{postgres::PgListener, PgPool};
use tracing::info;

use super::super::ExecutionError;

pub(crate) async fn load_consensus_epoch(pool: &PgPool) -> Result<String, ExecutionError> {
    let consensus_epoch = sqlx::query_scalar!(
        r#"
        SELECT consensus_epoch
          FROM blue_green_consensus_epoch
         WHERE singleton = TRUE
        "#,
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| {
        ExecutionError::InternalError("blue_green_consensus_epoch singleton is missing".to_owned())
    })?;
    Ok(consensus_epoch)
}

pub(crate) async fn load_validated_consensus_epoch(
    pool: &PgPool,
) -> Result<String, ExecutionError> {
    let consensus_epoch = load_consensus_epoch(pool).await?;
    let history = sqlx::query!(
        r#"
        SELECT stack_version, outcome
          FROM consensus_epoch_history
         WHERE consensus_epoch = $1
        "#,
        consensus_epoch,
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| {
        ExecutionError::InternalError(format!(
            "active manifest consensus_epoch {consensus_epoch} is absent from consensus_epoch_history"
        ))
    })?;

    let valid = if consensus_epoch == LEGACY_CONSENSUS_EPOCH {
        history.outcome == "initial"
    } else {
        history
            .stack_version
            .as_deref()
            .is_some_and(|version| versions_equal(STACK_VERSION, version))
            && matches!(history.outcome.as_str(), "pending" | "succeeded")
    };
    if !valid {
        return Err(ExecutionError::InternalError(format!(
            "manifest consensus_epoch {consensus_epoch} is not valid for stack version {STACK_VERSION} (history version {:?}, outcome {})",
            history.stack_version, history.outcome,
        )));
    }
    Ok(consensus_epoch)
}

/// Mirrors Green's durable activation and selected consensus_epoch into one value.
///
/// Keeping these together prevents a rollback followed by a rapid reactivation
/// from combining the new active window with the previous consensus_epoch cached by
/// a manifest worker.
pub(crate) async fn run_gcs_active_consensus_epoch_watcher(
    pool: &PgPool,
    active_consensus_epoch: &RwLock<Option<String>>,
) -> Result<(), ExecutionError> {
    let mut listener = PgListener::connect_with(pool).await?;
    listener.listen(EVENT_UPGRADE_ACTIVATED).await?;
    listener.listen(EVENT_DRY_RUN_STARTED).await?;
    listener.listen(EVENT_DRY_RUN_ROLLED_BACK).await?;
    info!(
        target: "manifest_consensus",
        "Green manifest consensus_epoch watcher listening"
    );

    loop {
        let next = load_gcs_active_consensus_epoch(pool).await?;
        let previous = {
            let mut slot = active_consensus_epoch
                .write()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            std::mem::replace(&mut *slot, next.clone())
        };
        if next != previous {
            if next.is_none() {
                info!(
                    target: "manifest_consensus",
                    consensus_epoch = previous.as_deref().unwrap_or(""),
                    "Green manifest work parked"
                );
            } else {
                info!(
                    target: "manifest_consensus",
                    consensus_epoch = next.as_deref().unwrap_or(""),
                    "Green manifest consensus_epoch activated"
                );
            }
        }

        tokio::select! {
            result = listener.recv() => {
                result?;
            }
            _ = tokio::time::sleep(Duration::from_secs(30)) => {}
        }
    }
}

async fn load_gcs_active_consensus_epoch(pool: &PgPool) -> Result<Option<String>, ExecutionError> {
    let row = sqlx::query!(
        r#"
        SELECT upgrade.state,
               selector.consensus_epoch,
               history.stack_version AS "stack_version?",
               history.outcome AS "outcome?"
          FROM upgrade_state upgrade
          CROSS JOIN blue_green_consensus_epoch selector
          LEFT JOIN public.consensus_epoch_history history
            ON history.consensus_epoch = selector.consensus_epoch
         WHERE upgrade.stack_role = 'GCS'
           AND selector.singleton = TRUE
         ORDER BY upgrade.proposal_block DESC NULLS LAST,
                  upgrade.host_chain_id
         LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await?;
    let Some(row) = row else {
        return Ok(None);
    };
    if !matches!(
        row.state.as_str(),
        "DryRunStarted" | "UpgradeAuthorized" | "LIVE"
    ) {
        return Ok(None);
    }

    let valid = if row.consensus_epoch == LEGACY_CONSENSUS_EPOCH {
        row.outcome.as_deref() == Some("initial")
    } else {
        row.stack_version
            .as_deref()
            .is_some_and(|version| versions_equal(STACK_VERSION, version))
            && matches!(row.outcome.as_deref(), Some("pending" | "succeeded"))
    };
    if !valid {
        return Err(ExecutionError::InternalError(format!(
            "active manifest consensus_epoch {} is not valid for stack version {STACK_VERSION} (history version {:?}, outcome {:?})",
            row.consensus_epoch, row.stack_version, row.outcome,
        )));
    }
    Ok(Some(row.consensus_epoch))
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use test_harness::instance::{setup_test_db, ImportMode};

    use super::*;

    #[tokio::test]
    #[serial(db)]
    async fn rollback_and_reactivation_select_the_new_consensus_epoch_without_an_intermediate_poll()
    {
        let instance = setup_test_db(ImportMode::None)
            .await
            .expect("create consensus_epoch gate database");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(instance.db_url())
            .await
            .expect("connect consensus_epoch gate database");

        select_consensus_epoch(&pool, "7", 70).await;
        assert_eq!(
            load_gcs_active_consensus_epoch(&pool)
                .await
                .unwrap()
                .as_deref(),
            Some("7")
        );

        sqlx::query("UPDATE upgrade_state SET state = 'PAUSED', status = 'failed'")
            .execute(&pool)
            .await
            .expect("roll back consensus_epoch 7");
        sqlx::query(
            "UPDATE consensus_epoch_history \
                SET outcome = 'failed', completed_at = NOW() \
              WHERE consensus_epoch = '7'",
        )
        .execute(&pool)
        .await
        .expect("finish consensus_epoch 7");

        // Activate the next window without any manifest worker observing the
        // intermediate PAUSED state.
        select_consensus_epoch(&pool, "8", 80).await;
        assert_eq!(
            load_gcs_active_consensus_epoch(&pool)
                .await
                .unwrap()
                .as_deref(),
            Some("8")
        );

        drop(pool);
        drop(instance);
    }

    #[tokio::test]
    #[serial(db)]
    async fn consensus_epoch_pin_treats_v_prefix_as_the_same_stack_version() {
        let instance = setup_test_db(ImportMode::None)
            .await
            .expect("create consensus_epoch pin database");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(instance.db_url())
            .await
            .expect("connect consensus_epoch pin database");

        let prefixed = format!(
            "v{}",
            STACK_VERSION
                .trim_start_matches('v')
                .trim_start_matches('V')
        );
        select_consensus_epoch_with_stack_version(&pool, "9", 90, &prefixed).await;
        assert_eq!(
            load_gcs_active_consensus_epoch(&pool)
                .await
                .unwrap()
                .as_deref(),
            Some("9")
        );
        sqlx::query(
            "UPDATE blue_green_consensus_epoch SET consensus_epoch = '9' WHERE singleton = TRUE",
        )
        .execute(&pool)
        .await
        .expect("pin consensus_epoch for blue validation");
        assert_eq!(load_validated_consensus_epoch(&pool).await.unwrap(), "9");
    }

    async fn select_consensus_epoch(pool: &PgPool, consensus_epoch: &str, proposal_block: i64) {
        select_consensus_epoch_with_stack_version(
            pool,
            consensus_epoch,
            proposal_block,
            STACK_VERSION,
        )
        .await;
    }

    async fn select_consensus_epoch_with_stack_version(
        pool: &PgPool,
        consensus_epoch: &str,
        proposal_block: i64,
        stack_version: &str,
    ) {
        sqlx::query(
            "INSERT INTO consensus_epoch_history ( \
                 consensus_epoch, proposal_id, proposal_block, stack_version, outcome \
             ) VALUES ($1, $2, $3, $4, 'pending')",
        )
        .bind(consensus_epoch)
        .bind(vec![consensus_epoch.as_bytes()[0]; 32])
        .bind(proposal_block)
        .bind(stack_version)
        .execute(pool)
        .await
        .expect("allocate consensus_epoch");
        sqlx::query(
            "UPDATE blue_green_consensus_epoch \
                SET consensus_epoch = $1, updated_at = NOW() \
              WHERE singleton = TRUE",
        )
        .bind(consensus_epoch)
        .execute(pool)
        .await
        .expect("select consensus_epoch");
        sqlx::query(
            "INSERT INTO upgrade_state ( \
                 stack_role, state, status, proposal_id, version, start_block, \
                 end_block, host_chain_id, proposal_block \
             ) VALUES ('GCS', 'DryRunStarted', 'in_progress', $1, $2, 100, 200, 1, $3) \
             ON CONFLICT (stack_role, host_chain_id) DO UPDATE \
             SET state = EXCLUDED.state, status = EXCLUDED.status, \
                 proposal_id = EXCLUDED.proposal_id, \
                 proposal_block = EXCLUDED.proposal_block, updated_at = NOW()",
        )
        .bind(vec![consensus_epoch.as_bytes()[0]; 32])
        .bind(STACK_VERSION)
        .bind(proposal_block)
        .execute(pool)
        .await
        .expect("activate consensus_epoch");
    }
}
