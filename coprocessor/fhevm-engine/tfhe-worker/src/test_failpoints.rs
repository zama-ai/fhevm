//! Opt-in test hooks. The runner creates and removes the control table in an isolated test database.
use sqlx::PgPool;

pub async fn hit(pool: &PgPool, stage: &str, chains: Vec<Vec<u8>>) -> Result<(), sqlx::Error> {
    if chains.is_empty() {
        return Ok(());
    }
    let available: bool =
        sqlx::query_scalar("SELECT to_regclass('public.consensus_test_failpoints') IS NOT NULL")
            .fetch_one(pool)
            .await?;
    if !available {
        return Ok(());
    }
    let hit: Option<Vec<u8>> = sqlx::query_scalar(
        "UPDATE public.consensus_test_failpoints SET reached = true, observed_at = clock_timestamp() \
         WHERE stage = $1 AND dependence_chain_id = ANY($2) AND NOT reached \
         RETURNING dependence_chain_id")
        .bind(stage).bind(chains).fetch_optional(pool).await?;
    if let Some(chain) = hit {
        tracing::warn!(stage, chain = %hex::encode(&chain), "consensus test failpoint reached");
        loop {
            // Deleting the control row releases a surviving worker during cleanup.
            let held: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM public.consensus_test_failpoints WHERE stage = $1 AND dependence_chain_id = $2)")
                .bind(stage).bind(&chain).fetch_one(pool).await?;
            if !held {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::hit;
    use sqlx::PgPool;
    use std::time::Duration;
    use test_harness::instance::{setup_test_db, ImportMode};

    #[tokio::test]
    async fn hook_targets_one_boundary_once_and_cleanup_releases_it() {
        let instance = setup_test_db(ImportMode::None).await.unwrap();
        let pool = PgPool::connect(instance.db_url()).await.unwrap();
        let chain = vec![7u8; 32];
        hit(&pool, "after-commit", vec![chain.clone()])
            .await
            .unwrap();
        sqlx::query("CREATE TABLE public.consensus_test_failpoints(stage text PRIMARY KEY, dependence_chain_id bytea NOT NULL, reached boolean NOT NULL DEFAULT false, observed_at timestamptz)")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO public.consensus_test_failpoints(stage, dependence_chain_id) VALUES ('after-commit', $1)")
            .bind(&chain).execute(&pool).await.unwrap();
        hit(&pool, "before-commit", vec![chain.clone()])
            .await
            .unwrap();
        hit(&pool, "after-commit", vec![vec![8u8; 32]])
            .await
            .unwrap();
        let task = tokio::spawn({
            let pool = pool.clone();
            let chain = chain.clone();
            async move { hit(&pool, "after-commit", vec![chain]).await }
        });
        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                let reached: bool =
                    sqlx::query_scalar("SELECT reached FROM public.consensus_test_failpoints")
                        .fetch_one(&pool)
                        .await
                        .unwrap();
                if reached {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .unwrap();
        assert!(!task.is_finished(), "the selected boundary must stay held");
        // A replacement process skips a consumed hook, so it can recover the same work.
        hit(&pool, "after-commit", vec![chain]).await.unwrap();
        sqlx::query("DELETE FROM public.consensus_test_failpoints")
            .execute(&pool)
            .await
            .unwrap();
        tokio::time::timeout(Duration::from_secs(2), task)
            .await
            .unwrap()
            .unwrap()
            .unwrap();
    }
}
