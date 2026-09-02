//! Read the generation selected for this stack and activate Green atomically.

use std::{sync::RwLock, time::Duration};

use block_manifest::LEGACY_CONSENSUS_EPOCH;
use fhevm_engine_common::{
    gcs_activation::{EVENT_DRY_RUN_ROLLED_BACK, EVENT_DRY_RUN_STARTED, EVENT_UPGRADE_ACTIVATED},
    versions_equal, STACK_VERSION,
};
use sqlx::{postgres::PgListener, PgPool};
use tracing::info;

use super::super::ExecutionError;

pub(crate) async fn load_generation(pool: &PgPool) -> Result<String, ExecutionError> {
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

pub(crate) async fn load_validated_generation(pool: &PgPool) -> Result<String, ExecutionError> {
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

    let valid = if generation == LEGACY_CONSENSUS_EPOCH {
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
    active_generation: &RwLock<Option<String>>,
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
        let previous = {
            let mut slot = active_generation
                .write()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            std::mem::replace(&mut *slot, next.clone())
        };
        if next != previous {
            if next.is_none() {
                info!(
                    target: "manifest_consensus",
                    generation = previous.as_deref().unwrap_or(""),
                    "Green manifest work parked"
                );
            } else {
                info!(
                    target: "manifest_consensus",
                    generation = next.as_deref().unwrap_or(""),
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

async fn load_gcs_active_generation(pool: &PgPool) -> Result<Option<String>, ExecutionError> {
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

    let valid = if row.generation == LEGACY_CONSENSUS_EPOCH {
        row.outcome.as_deref() == Some("initial")
    } else {
        row.stack_version
            .as_deref()
            .is_some_and(|version| versions_equal(STACK_VERSION, version))
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

        select_generation(&pool, "7", 70).await;
        assert_eq!(
            load_gcs_active_generation(&pool).await.unwrap().as_deref(),
            Some("7")
        );

        sqlx::query("UPDATE upgrade_state SET state = 'PAUSED', status = 'failed'")
            .execute(&pool)
            .await
            .expect("roll back generation 7");
        sqlx::query(
            "UPDATE generation_history \
                SET outcome = 'failed', completed_at = NOW() \
              WHERE generation = '7'",
        )
        .execute(&pool)
        .await
        .expect("finish generation 7");

        // Activate the next window without any manifest worker observing the
        // intermediate PAUSED state.
        select_generation(&pool, "8", 80).await;
        assert_eq!(
            load_gcs_active_generation(&pool).await.unwrap().as_deref(),
            Some("8")
        );

        drop(pool);
        drop(instance);
    }

    #[tokio::test]
    #[serial(db)]
    async fn generation_pin_treats_v_prefix_as_the_same_stack_version() {
        let instance = setup_test_db(ImportMode::None)
            .await
            .expect("create generation pin database");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(instance.db_url())
            .await
            .expect("connect generation pin database");

        let prefixed = format!(
            "v{}",
            STACK_VERSION
                .trim_start_matches('v')
                .trim_start_matches('V')
        );
        select_generation_with_stack_version(&pool, "9", 90, &prefixed).await;
        assert_eq!(
            load_gcs_active_generation(&pool).await.unwrap().as_deref(),
            Some("9")
        );
        sqlx::query("UPDATE blue_green_generation SET generation = '9' WHERE singleton = TRUE")
            .execute(&pool)
            .await
            .expect("pin generation for blue validation");
        assert_eq!(load_validated_generation(&pool).await.unwrap(), "9");
    }

    async fn select_generation(pool: &PgPool, generation: &str, proposal_block: i64) {
        select_generation_with_stack_version(pool, generation, proposal_block, STACK_VERSION).await;
    }

    async fn select_generation_with_stack_version(
        pool: &PgPool,
        generation: &str,
        proposal_block: i64,
        stack_version: &str,
    ) {
        sqlx::query(
            "INSERT INTO generation_history ( \
                 generation, proposal_id, proposal_block, stack_version, outcome \
             ) VALUES ($1, $2, $3, $4, 'pending')",
        )
        .bind(generation)
        .bind(vec![generation.as_bytes()[0]; 32])
        .bind(proposal_block)
        .bind(stack_version)
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
        .bind(vec![generation.as_bytes()[0]; 32])
        .bind(STACK_VERSION)
        .bind(proposal_block)
        .execute(pool)
        .await
        .expect("activate generation");
    }
}
