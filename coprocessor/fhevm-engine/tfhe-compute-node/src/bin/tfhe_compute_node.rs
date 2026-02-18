use tokio::signal::unix::{self, SignalKind};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    let args = tfhe_compute_node::cli::parse();
    let cancel = CancellationToken::new();
    install_signal_handlers(cancel.clone());

    if let Err(e) = tfhe_compute_node::async_main(args, cancel.clone()).await {
        error!(target: "main", { error = e }, "Runtime error");
    }
}

fn install_signal_handlers(cancel_token: CancellationToken) {
    let mut sigint = unix::signal(SignalKind::interrupt()).unwrap();
    let mut sigterm = unix::signal(SignalKind::terminate()).unwrap();
    tokio::spawn(async move {
        tokio::select! {
            _ = sigint.recv() => { info!("received SIGINT"); },
            _ = sigterm.recv() => { info!("received SIGTERM"); },
        }
        cancel_token.cancel();
        info!("Cancellation signal sent over the token");
    });
}
