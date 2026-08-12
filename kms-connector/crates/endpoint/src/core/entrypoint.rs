use crate::core::Config;
use tokio_util::sync::CancellationToken;
use tracing::info;

/// Entrypoint of the `Endpoint` service.
// TODO: this is a dummy implementation. Update it once the `Endpoint` service is implemented.
pub struct Entrypoint;

impl Default for Entrypoint {
    fn default() -> Self {
        Self::new()
    }
}

impl Entrypoint {
    /// Creates a new `Entrypoint`.
    pub fn new() -> Self {
        Self
    }

    /// Creates a new `Entrypoint` instance from a valid `Config`.
    pub async fn from_config(_config: Config) -> anyhow::Result<Self> {
        Ok(Self::new())
    }

    /// Starts the `Entrypoint`.
    pub async fn start(self, cancel_token: CancellationToken) {
        info!("Starting Entrypoint");
        tokio::select! {
            _ = cancel_token.cancelled() => info!("Stopping Entrypoint"),
            _ = self.run() => (),
        }
    }

    /// Runs the `Entrypoint`.
    async fn run(&self) {
        todo!("Endpoint service is not implemented yet")
    }
}
