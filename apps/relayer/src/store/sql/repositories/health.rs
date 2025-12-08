use crate::http::HealthCheck;

use super::Repositories;

#[async_trait::async_trait]
impl HealthCheck for Repositories {
    async fn check(&self) -> anyhow::Result<()> {
        match tokio::time::timeout(
            self.health_timeout,
            sqlx::query("SELECT 1").execute(&self.pg_client.get_pool()),
        )
        .await
        {
            Err(_) => Err(anyhow::anyhow!(
                "Database health check timed out after {:?}",
                self.health_timeout
            )),
            Ok(Err(e)) => Err(anyhow::anyhow!("Database health check failed: {}", e)),
            Ok(Ok(_)) => Ok(()),
        }
    }
}
