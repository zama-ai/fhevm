use crate::core::{Config, ResponseListener, Waiters, http};
use anyhow::anyhow;
use connector_utils::conn::connect_to_db;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

/// The `Endpoint` service: the `v1` HTTP decryption interface of the KMS Connector.
pub struct Endpoint {
    config: Arc<Config>,
    db_pool: Pool<Postgres>,
    waiters: Arc<Waiters>,
    response_listener: ResponseListener,
}

impl Endpoint {
    pub async fn from_config(config: Config) -> anyhow::Result<Self> {
        let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;
        let waiters = Arc::new(Waiters::new());
        let response_listener = ResponseListener::connect(&db_pool, waiters.clone()).await?;

        Ok(Self {
            config: Arc::new(config),
            db_pool,
            waiters,
            response_listener,
        })
    }

    /// Starts the HTTP server and response listener.
    pub async fn start(self, cancel_token: CancellationToken) -> anyhow::Result<()> {
        info!("Starting Endpoint");

        let state = http::AppState::new(Arc::clone(&self.config), self.db_pool, self.waiters);
        let server = http::run_server(state, self.config.http_endpoint)?;
        info!("HTTP server listening at: {}", self.config.http_endpoint);
        let server_handle = server.handle();

        let mut listener_task = tokio::spawn(self.response_listener.start());
        let mut server_task = tokio::spawn(server);

        let result = tokio::select! {
            _ = cancel_token.cancelled() => {
                info!("Stopping Endpoint");
                Ok(())
            }
            res = &mut listener_task => {
                error!("Response listener exited, stopping the service");
                res.map_err(|e| anyhow!("Response listener task panicked: {e}"))
                    .and(Err(anyhow!("Response listener exited")))
            }
            res = &mut server_task => {
                error!("HTTP server exited unexpectedly");
                match res {
                    Ok(Ok(())) => Err(anyhow!("HTTP server exited")),
                    Ok(Err(e)) => Err(anyhow!("HTTP server error: {e}")),
                    Err(e) => Err(anyhow!("HTTP server task panicked: {e}")),
                }
            }
        };

        // Graceful stop: in-flight requests drain (actix default 30s shutdown timeout).
        server_handle.stop(true).await;
        listener_task.abort();
        server_task.abort();
        info!("Endpoint stopped");
        result
    }
}
