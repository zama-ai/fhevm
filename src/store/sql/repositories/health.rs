use crate::orchestrator::HealthCheck;

use super::Repositories;

#[async_trait::async_trait]
impl HealthCheck for Repositories {
    async fn check(&self) -> anyhow::Result<()> {
        // Check both app and cron pools for health
        let app_result = tokio::time::timeout(
            self.health_timeout,
            sqlx::query("SELECT 1").execute(&self.pg_client.get_app_pool()),
        )
        .await;

        let cron_result = tokio::time::timeout(
            self.health_timeout,
            sqlx::query("SELECT 1").execute(&self.pg_client.get_cron_pool()),
        )
        .await;

        match (app_result, cron_result) {
            (Err(_), _) => Err(anyhow::anyhow!(
                "App pool health check timed out after {:?}",
                self.health_timeout
            )),
            (_, Err(_)) => Err(anyhow::anyhow!(
                "Cron pool health check timed out after {:?}",
                self.health_timeout
            )),
            (Ok(Err(e)), _) => Err(anyhow::anyhow!("App pool health check failed: {}", e)),
            (_, Ok(Err(e))) => Err(anyhow::anyhow!("Cron pool health check failed: {}", e)),
            (Ok(Ok(_)), Ok(Ok(_))) => Ok(()),
        }
    }
}
