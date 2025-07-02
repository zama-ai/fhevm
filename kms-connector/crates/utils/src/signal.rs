use tokio::signal::unix::{SignalKind, signal};
use tokio_util::sync::CancellationToken;
use tracing::info;

/// Helper function to handle UNIX signals such as SIGINT (CTRL-C).
pub fn install_signal_handlers(cancel_token: CancellationToken) -> anyhow::Result<()> {
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    tokio::spawn(async move {
        tokio::select! {
            _ = sigint.recv() => info!("Received SIGTERM signal"),
            _ = sigterm.recv() => info!("Received SIGINT signal"),
        }
        cancel_token.cancel();
    });
    Ok(())
}
