//! Only compiled into explicitly requested isolated-test binaries.
use sqlx::{PgPool, Row};

pub async fn hit(pool: &PgPool, stage: &str) -> Result<(), sqlx::Error> {
    let available: bool =
        sqlx::query_scalar("SELECT to_regclass('public.consensus_test_upgrade_fault') IS NOT NULL")
            .fetch_one(pool)
            .await?;
    if !available {
        return Ok(());
    }
    // A consumed hook is skipped by the replacement controller. The matching
    // real proposal must exist; unrelated upgrades cannot acknowledge it.
    let observed = sqlx::query(
        "UPDATE public.consensus_test_upgrade_fault f SET reached=true, observed_at=clock_timestamp() \
         WHERE f.stage=$1 AND NOT reached AND EXISTS \
         (SELECT 1 FROM public.upgrade_state u WHERE u.stack_role='GCS' AND u.version=f.version) \
         RETURNING f.version",
    ).bind(stage).fetch_optional(pool).await?;
    if let Some(row) = observed {
        let version: String = row.get("version");
        tracing::warn!(stage, version, "upgrade test boundary reached");
        loop {
            let held: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM public.consensus_test_upgrade_fault WHERE stage=$1 AND version=$2)",
            ).bind(stage).bind(&version).fetch_one(pool).await?;
            if !held {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }
    Ok(())
}
