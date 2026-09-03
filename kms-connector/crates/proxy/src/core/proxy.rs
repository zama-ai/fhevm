use crate::core::Config;
use tokio_util::sync::CancellationToken;
use tracing::info;

// TODO: this is a dummy implementation. Update it once the `Proxy` service is implemented.
pub struct Proxy;

impl Default for Proxy {
    fn default() -> Self {
        Self::new()
    }
}

impl Proxy {
    pub fn new() -> Self {
        Self
    }

    pub async fn from_config(_config: Config) -> anyhow::Result<Self> {
        Ok(Self::new())
    }

    pub async fn start(self, cancel_token: CancellationToken) {
        info!("Starting Proxy");
        tokio::select! {
            _ = cancel_token.cancelled() => info!("Stopping Proxy"),
            _ = self.run() => (),
        }
    }

    async fn run(&self) {
        todo!("Proxy service is not implemented yet")
    }
}
