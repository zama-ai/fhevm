//! Read the generation selected for this stack and activate Green atomically.

use std::{
    sync::atomic::{AtomicI64, Ordering},
    time::Duration,
};

use fhevm_engine_common::{
    gcs_activation::{
        EVENT_DRY_RUN_ROLLED_BACK, EVENT_DRY_RUN_STARTED, EVENT_UPGRADE_ACTIVATED,
        GCS_NOT_ACTIVATED,
    },
    STACK_VERSION,
};
use sqlx::{postgres::PgListener, PgPool};
use tracing::info;

use super::super::ExecutionError;

pub(crate) async fn load_generation(pool: &PgPool) -> Result<i64, ExecutionError> {
    let generation = sqlx::query_scalar!(
        r#"
        SELECT generation
          FROM blue_green_generation
         WHERE singleton = TRUE
        "#,
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| {
        ExecutionError::InternalError("blue_green_generation singleton is missing".to_owned())
    })?;
    Ok(generation)
}

pub(crate) async fn load_validated_generation(pool: &PgPool) -> Result<i64, ExecutionError> {
    let generation = load_generation(pool).await?;
    let history = sqlx::query!(
        r#"
        SELECT stack_version, outcome
          FROM generation_history
         WHERE generation = $1
        "#,
        generation,
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| {
        ExecutionError::InternalError(format!(
            "active manifest generation {generation} is absent from generation_history"
        ))
    })?;

    let valid = if generation == 0 {
        history.outcome == "initial"
    } else {
        history.stack_version.as_deref() == Some(STACK_VERSION)
            && matches!(history.outcome.as_str(), "pending" | "succeeded")
    };
    if !valid {
        return Err(ExecutionError::InternalError(format!(
            "manifest generation {generation} is not valid for stack version {STACK_VERSION} (history version {:?}, outcome {})",
            history.stack_version, history.outcome,
        )));
    }
    Ok(generation)
}

/// Mirrors Green's durable activation and selected generation into one value.
///
/// Keeping these together prevents a rollback followed by a rapid reactivation
/// from combining the new active window with the previous generation cached by
/// a manifest worker.
pub(crate) async fn run_gcs_active_generation_watcher(
    pool: &PgPool,
    active_generation: &AtomicI64,
) -> Result<(), ExecutionError> {
    let mut listener = PgListener::connect_with(pool).await?;
    listener.listen(EVENT_UPGRADE_ACTIVATED).await?;
    listener.listen(EVENT_DRY_RUN_STARTED).await?;
    listener.listen(EVENT_DRY_RUN_ROLLED_BACK).await?;
    info!(
        target: "manifest_consensus",
        "Green manifest generation watcher listening"
    );

    loop {
        let next = load_gcs_active_generation(pool).await?;
        let next = next.unwrap_or(GCS_NOT_ACTIVATED);
        let previous = active_generation.swap(next, Ordering::SeqCst);
        if next != previous {
            if next == GCS_NOT_ACTIVATED {
                info!(
                    target: "manifest_consensus",
                    generation = previous,
                    "Green manifest work parked"
                );
            } else {
                info!(
                    target: "manifest_consensus",
                    generation = next,
                    "Green manifest generation activated"
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

async fn load_gcs_active_generation(pool: &PgPool) -> Result<Option<i64>, ExecutionError> {
    let row = sqlx::query!(
        r#"
        SELECT upgrade.state,
               selector.generation,
               history.stack_version AS "stack_version?",
               history.outcome AS "outcome?"
          FROM upgrade_state upgrade
          CROSS JOIN blue_green_generation selector
          LEFT JOIN public.generation_history history
            ON history.generation = selector.generation
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

    let valid = if row.generation == 0 {
        row.outcome.as_deref() == Some("initial")
    } else {
        row.stack_version.as_deref() == Some(STACK_VERSION)
            && matches!(row.outcome.as_deref(), Some("pending" | "succeeded"))
    };
    if !valid {
        return Err(ExecutionError::InternalError(format!(
            "active manifest generation {} is not valid for stack version {STACK_VERSION} (history version {:?}, outcome {:?})",
            row.generation, row.stack_version, row.outcome,
        )));
    }
    Ok(Some(row.generation))
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use test_harness::instance::{setup_test_db, ImportMode};

    use super::*;

    #[tokio::test]
    #[serial(db)]
    async fn rollback_and_reactivation_select_the_new_generation_without_an_intermediate_poll() {
        let instance = setup_test_db(ImportMode::None)
            .await
            .expect("create generation gate database");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(instance.db_url())
            .await
            .expect("connect generation gate database");

        select_generation(&pool, 7, 70).await;
        assert_eq!(load_gcs_active_generation(&pool).await.unwrap(), Some(7));

        sqlx::query("UPDATE upgrade_state SET state = 'PAUSED', status = 'failed'")
            .execute(&pool)
            .await
            .expect("roll back generation 7");
        sqlx::query(
            "UPDATE generation_history \
                SET outcome = 'failed', completed_at = NOW() \
              WHERE generation = 7",
        )
        .execute(&pool)
        .await
        .expect("finish generation 7");

        // Activate the next window without any manifest worker observing the
        // intermediate PAUSED state.
        select_generation(&pool, 8, 80).await;
        assert_eq!(load_gcs_active_generation(&pool).await.unwrap(), Some(8));

        drop(pool);
        drop(instance);
    }

    async fn select_generation(pool: &PgPool, generation: i64, proposal_block: i64) {
        sqlx::query(
            "INSERT INTO generation_history ( \
                 generation, proposal_id, proposal_block, stack_version, outcome \
             ) VALUES ($1, $2, $3, $4, 'pending')",
        )
        .bind(generation)
        .bind(vec![u8::try_from(generation).unwrap(); 32])
        .bind(proposal_block)
        .bind(STACK_VERSION)
        .execute(pool)
        .await
        .expect("allocate generation");
        sqlx::query(
            "UPDATE blue_green_generation \
                SET generation = $1, updated_at = NOW() \
              WHERE singleton = TRUE",
        )
        .bind(generation)
        .execute(pool)
        .await
        .expect("select generation");
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
        .bind(vec![u8::try_from(generation).unwrap(); 32])
        .bind(STACK_VERSION)
        .bind(proposal_block)
        .execute(pool)
        .await
        .expect("activate generation");
    }
}
