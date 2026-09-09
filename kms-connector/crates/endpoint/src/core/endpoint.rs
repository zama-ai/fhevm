use crate::{
    core::{Config, ResponseListener, WaiterRegistry, http},
    monitoring::health::State,
};
use anyhow::anyhow;
use connector_utils::conn::connect_to_db;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

/// The `Endpoint` service: the `v1` HTTP decryption interface of the KMS Connector.
pub struct Endpoint {
    app_state: http::AppState,
    response_listener: ResponseListener,
}

impl Endpoint {
    /// Creates the `Endpoint` from its configuration, and the `State` used to monitor it.
    pub async fn from_config(config: Config) -> anyhow::Result<(Self, State)> {
        let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;
        let waiter_registry = Arc::new(WaiterRegistry::new());
        let response_listener =
            ResponseListener::connect(&db_pool, Arc::clone(&waiter_registry)).await?;

        let config = Arc::new(config);
        let app_state = http::AppState::new(Arc::clone(&config), db_pool.clone(), waiter_registry);
        let state = State::new(
            db_pool,
            config.http_endpoint,
            Arc::clone(&app_state.in_flight_limiter),
            config.max_in_flight_decryptions,
            config.healthcheck_timeout,
        );

        let endpoint = Self {
            app_state,
            response_listener,
        };
        Ok((endpoint, state))
    }

    /// Starts the HTTP server and response listener.
    pub async fn start(self, cancel_token: CancellationToken) -> anyhow::Result<()> {
        info!("Starting Endpoint");

        let http_endpoint = self.app_state.config.http_endpoint;
        let server = http::run_server(self.app_state, http_endpoint)?;
        info!("HTTP server listening at: {http_endpoint}");
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
