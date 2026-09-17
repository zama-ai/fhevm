//! Lock shared by TFHE batches and consensus-detector containment.
/// Database-wide advisory lock key, ASCII `FHEVMCNT`.
/// Acquire after the Blue/Green cutover guard and before computation row locks.
pub const DRIFT_CONTAINMENT_BARRIER: i64 = i64::from_be_bytes(*b"FHEVMCNT");

/// Hold from before scheduling through result persistence and transaction end.
/// Shared holders run concurrently; guaranteed propagation waits for all of them.
pub async fn acquire_ct_computation_permit(
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "SELECT pg_advisory_xact_lock_shared($1)",
        DRIFT_CONTAINMENT_BARRIER
    )
    .execute(trx.as_mut())
    .await?;
    Ok(())
}

/// Wait for running batches to finish, then prevent any batch from acquiring a
/// computation permit until this transaction commits or rolls back.
/// Acquire after the Blue/Green cutover guard, before execution row locks.
pub async fn block_ct_computations(
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "SELECT pg_advisory_xact_lock($1)",
        DRIFT_CONTAINMENT_BARRIER
    )
    .execute(trx.as_mut())
    .await?;
    Ok(())
}
