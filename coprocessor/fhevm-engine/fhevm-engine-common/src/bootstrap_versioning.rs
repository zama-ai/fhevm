//! One-shot version setup for a new database, run by the migration container
//! before any service starts.
//!
//! `initialize_db.sh` calls this for a database with no migration history, or one
//! whose setup was interrupted and still carries the setup marker. A database that
//! already has history keeps the versions it has.
//!
//! The marker table `_fhevm_versioning_bootstrap` is what lets it run: the script
//! creates it before the first migration and this module drops it once the versions
//! are set, so a second run refuses.

use sqlx::{Pool, Postgres};

use crate::{CONSENSUS_PROTOCOL_VERSION, STACK_VERSION};

/// Set the versions for a new database.
///
/// The migration script creates a one-time marker before the first migration.
/// This function requires that marker and removes it after a successful update.
pub async fn bootstrap_versioning(pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await?;
    let has_bootstrap_intent: bool =
        sqlx::query_scalar("SELECT to_regclass('public._fhevm_versioning_bootstrap') IS NOT NULL")
            .fetch_one(&mut *transaction)
            .await?;
    anyhow::ensure!(
        has_bootstrap_intent,
        "cannot set initial versions: this is not a new database"
    );

    sqlx::query("LOCK TABLE public._fhevm_versioning_bootstrap IN ACCESS EXCLUSIVE MODE")
        .execute(&mut *transaction)
        .await?;
    let bootstrap_intent_rows: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM public._fhevm_versioning_bootstrap")
            .fetch_one(&mut *transaction)
            .await?;
    anyhow::ensure!(
        bootstrap_intent_rows == 1,
        "cannot set initial versions: expected one setup marker, found {bootstrap_intent_rows}"
    );

    let (live_stack_version, live_consensus_version): (String, i64) = sqlx::query_as(
        "SELECT stack_version, consensus_version
         FROM versioning
         WHERE singleton = TRUE
         FOR UPDATE",
    )
    .fetch_one(&mut *transaction)
    .await?;

    let has_upgrade_history: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM upgrade_state)")
            .fetch_one(&mut *transaction)
            .await?;
    anyhow::ensure!(
        !has_upgrade_history,
        "cannot set initial versions: upgrade history already exists"
    );

    let compiled_consensus_version = i64::from(CONSENSUS_PROTOCOL_VERSION);
    anyhow::ensure!(
        compiled_consensus_version >= live_consensus_version,
        "cannot lower the consensus version from {live_consensus_version} to {compiled_consensus_version}"
    );

    let stored_version = STACK_VERSION.to_string();
    let result = sqlx::query(
        "UPDATE versioning
         SET stack_version = $1, consensus_version = $2, updated_at = NOW()
         WHERE singleton = TRUE",
    )
    .bind(&stored_version)
    .bind(compiled_consensus_version)
    .execute(&mut *transaction)
    .await?;

    anyhow::ensure!(result.rows_affected() == 1, "versioning row is missing");
    sqlx::query("DROP TABLE public._fhevm_versioning_bootstrap")
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;

    tracing::info!(
        previous_stack_version = live_stack_version,
        previous_consensus_version = live_consensus_version,
        stack_version = stored_version,
        consensus_version = CONSENSUS_PROTOCOL_VERSION,
        "set initial database versions"
    );
    Ok(())
}
