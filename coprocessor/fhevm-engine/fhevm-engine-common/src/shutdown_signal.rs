use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;
use tracing::info;

/// Cancels `cancel` on the first SIGINT or SIGTERM. Kubernetes stops a pod with SIGTERM, so a
/// service waiting for SIGINT only is killed without draining. Failing to install either handler
/// is a startup error for the caller.
pub fn cancel_on_shutdown_signal(cancel: CancellationToken) -> std::io::Result<()> {
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    tokio::spawn(async move {
        let received = tokio::select! {
            _ = sigint.recv() => "SIGINT",
            _ = sigterm.recv() => "SIGTERM",
        };
        info!(signal = received, "Shutting down");
        cancel.cancel();
    });
    Ok(())
}
