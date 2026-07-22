//! Concurrent HTTP + ingest-writer + shutdown-signal supervision.
//!
//! A single [`CancellationToken`] drives signal handling, forced writer-exit
//! shutdown, and Axum graceful shutdown.

use std::future::Future;
use std::io;
#[cfg(test)]
use std::pin::Pin;
use std::time::Duration;

use anyhow::Context;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::ingest_health::IngestHealth;

/// Bound how long shutdown waits for the ingest writer after cancel.
pub const INGEST_SHUTDOWN_DEADLINE: Duration = Duration::from_secs(15);

/// Connections reserved for ingest writer + readiness (not available to proofs).
pub const RESERVED_DB_CONNECTIONS: u32 = 2;

/// Proof admission slots so the shared pool always keeps
/// [`RESERVED_DB_CONNECTIONS`] free for writer/readiness.
pub fn proof_admission_limit(max_connections: u32) -> usize {
    max_connections
        .saturating_sub(RESERVED_DB_CONNECTIONS)
        .max(1) as usize
}

/// Runs HTTP server, ingest writer, and shutdown signal together.
///
/// - Writer panic/exit while serving cancels the shared token and fails after
///   HTTP stops.
/// - Signal completion cancels the token (graceful HTTP shutdown) then joins
///   the writer.
/// - Signal registration / handler errors fail the process after cleanup.
/// - Server completion always cancels and joins the writer (even on server Err).
pub async fn supervise_http_and_writer(
    server: impl Future<Output = io::Result<()>>,
    mut writer: JoinHandle<()>,
    shutdown_signal: impl Future<Output = anyhow::Result<()>>,
    health: &IngestHealth,
    cancel: CancellationToken,
    writer_join_deadline: Duration,
) -> anyhow::Result<()> {
    let mut server = std::pin::pin!(server);
    let mut shutdown_signal = std::pin::pin!(shutdown_signal);

    enum Stopped {
        Server(io::Result<()>),
        Writer(Result<(), tokio::task::JoinError>),
        Signal(anyhow::Result<()>),
    }

    let stopped = tokio::select! {
        result = &mut server => Stopped::Server(result),
        join = &mut writer => Stopped::Writer(join),
        signal = &mut shutdown_signal => Stopped::Signal(signal),
    };

    match stopped {
        Stopped::Server(result) => {
            cancel.cancel();
            await_ingest_writer(writer, health, writer_join_deadline).await;
            result.context("HTTP server error")?;
            Ok(())
        }
        Stopped::Writer(join) => {
            let unexpected = !cancel.is_cancelled();
            record_ingest_join(join, health, unexpected);
            cancel.cancel();
            let result = server.as_mut().await;
            result.context("HTTP server error")?;
            if unexpected {
                anyhow::bail!("ingest writer exited while HTTP server was still running");
            }
            Ok(())
        }
        Stopped::Signal(signal_result) => {
            cancel.cancel();
            let server_result = server.as_mut().await;
            await_ingest_writer(writer, health, writer_join_deadline).await;
            signal_result.context("shutdown signal handler failed")?;
            server_result.context("HTTP server error")?;
            Ok(())
        }
    }
}

/// Waits until `cancel` is cancelled (signal, unexpected writer exit, or stop).
pub async fn wait_for_shutdown(cancel: CancellationToken) {
    cancel.cancelled().await;
}

pub fn record_ingest_join(
    join: Result<(), tokio::task::JoinError>,
    ingest: &IngestHealth,
    unexpected: bool,
) {
    match join {
        Ok(()) => {
            if unexpected {
                tracing::error!(
                    terminal = ?ingest.terminal(),
                    "ingest writer exited while HTTP server was still running"
                );
            } else {
                tracing::info!("ingest writer stopped");
            }
        }
        Err(join_err) => {
            let reason = if join_err.is_panic() {
                format!("ingest writer panicked: {join_err}")
            } else {
                format!("ingest writer join failed: {join_err}")
            };
            tracing::error!(%reason, "ingest writer supervision failure");
            if ingest.writer_running() || ingest.terminal().is_none() {
                ingest.mark_crashed(reason);
            }
        }
    }
}

pub async fn await_ingest_writer(
    mut handle: JoinHandle<()>,
    ingest: &IngestHealth,
    deadline: Duration,
) {
    match tokio::time::timeout(deadline, &mut handle).await {
        Ok(Ok(())) => {
            tracing::info!("ingest writer stopped");
        }
        Ok(Err(join_err)) => {
            record_ingest_join(Err(join_err), ingest, false);
        }
        Err(_) => {
            tracing::error!(
                deadline_secs = deadline.as_secs(),
                "ingest writer did not exit within shutdown deadline; aborting"
            );
            handle.abort();
            match handle.await {
                Ok(()) | Err(_) => {}
            }
            ingest.mark_crashed("ingest writer shutdown deadline exceeded");
        }
    }
}

/// Test helper: server future that completes when `cancel` fires.
#[cfg(test)]
pub fn server_until_cancel(
    cancel: CancellationToken,
) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send>> {
    Box::pin(async move {
        wait_for_shutdown(cancel).await;
        Ok(())
    })
}

#[cfg(test)]
fn pending_signal() -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(std::future::pending())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ingest_health::IngestTerminal;
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn proof_admission_reserves_ops_slots() {
        assert_eq!(proof_admission_limit(10), 8);
        assert_eq!(proof_admission_limit(3), 1);
        assert_eq!(proof_admission_limit(2), 1);
        assert_eq!(proof_admission_limit(1), 1);
    }

    #[tokio::test]
    async fn writer_panic_cancels_and_fails_supervision() {
        let health = IngestHealth::new();
        health.mark_started();
        health.mark_progress(1);
        let cancel = CancellationToken::new();
        let writer = tokio::spawn(async {
            panic!("ingest boom");
        });
        let server = server_until_cancel(cancel.clone());

        let err = supervise_http_and_writer(
            server,
            writer,
            pending_signal(),
            &health,
            cancel.clone(),
            Duration::from_secs(2),
        )
        .await
        .unwrap_err();
        assert!(err.to_string().contains("ingest writer exited"));
        assert!(cancel.is_cancelled());
        assert!(matches!(
            health.terminal(),
            Some(IngestTerminal::Crashed { .. })
        ));
        assert!(!health.writer_running());
    }

    #[tokio::test]
    async fn writer_exit_while_serving_is_unexpected() {
        let health = IngestHealth::new();
        health.mark_started();
        let cancel = CancellationToken::new();
        let health_task = Arc::clone(&health);
        let writer = tokio::spawn(async move {
            health_task.mark_finished(Ok(()));
        });
        let server = server_until_cancel(cancel.clone());

        let err = supervise_http_and_writer(
            server,
            writer,
            pending_signal(),
            &health,
            cancel.clone(),
            Duration::from_secs(2),
        )
        .await
        .unwrap_err();
        assert!(err.to_string().contains("ingest writer exited"));
        assert!(cancel.is_cancelled());
        assert!(matches!(health.terminal(), Some(IngestTerminal::Cancelled)));
    }

    #[tokio::test]
    async fn server_error_still_joins_writer() {
        let health = IngestHealth::new();
        health.mark_started();
        let cancel = CancellationToken::new();
        let health_task = Arc::clone(&health);
        let cancel_task = cancel.clone();
        let writer = tokio::spawn(async move {
            cancel_task.cancelled().await;
            health_task.mark_finished(Ok(()));
        });
        let server = async { Err(io::Error::other("listener failed")) };

        let err = supervise_http_and_writer(
            server,
            writer,
            pending_signal(),
            &health,
            cancel.clone(),
            Duration::from_secs(2),
        )
        .await
        .unwrap_err();
        assert!(err.to_string().contains("HTTP server error"));
        assert!(cancel.is_cancelled());
        assert!(matches!(health.terminal(), Some(IngestTerminal::Cancelled)));
        assert!(!health.writer_running());
    }

    #[tokio::test]
    async fn normal_signal_cancels_and_joins_cleanly() {
        let health = IngestHealth::new();
        health.mark_started();
        let cancel = CancellationToken::new();
        let health_task = Arc::clone(&health);
        let cancel_task = cancel.clone();
        let writer = tokio::spawn(async move {
            cancel_task.cancelled().await;
            health_task.mark_finished(Ok(()));
        });
        let server = server_until_cancel(cancel.clone());
        let signal = async { Ok(()) };

        supervise_http_and_writer(
            server,
            writer,
            signal,
            &health,
            cancel.clone(),
            Duration::from_secs(2),
        )
        .await
        .unwrap();
        assert!(cancel.is_cancelled());
        assert!(matches!(health.terminal(), Some(IngestTerminal::Cancelled)));
    }

    #[tokio::test]
    async fn signal_registration_error_fails_after_cleanup() {
        let health = IngestHealth::new();
        health.mark_started();
        let cancel = CancellationToken::new();
        let health_task = Arc::clone(&health);
        let cancel_task = cancel.clone();
        let writer = tokio::spawn(async move {
            cancel_task.cancelled().await;
            health_task.mark_finished(Ok(()));
        });
        let server = server_until_cancel(cancel.clone());
        let signal = async { Err(anyhow::anyhow!("failed to install SIGTERM handler")) };

        let err = supervise_http_and_writer(
            server,
            writer,
            signal,
            &health,
            cancel.clone(),
            Duration::from_secs(2),
        )
        .await
        .unwrap_err();
        assert!(err.to_string().contains("shutdown signal handler failed"));
        assert!(cancel.is_cancelled());
        assert!(!health.writer_running());
    }

    #[tokio::test]
    async fn normal_cancel_then_writer_exit_is_clean() {
        let health = IngestHealth::new();
        health.mark_started();
        let cancel = CancellationToken::new();
        cancel.cancel();
        let health_task = Arc::clone(&health);
        let writer = tokio::spawn(async move {
            health_task.mark_finished(Ok(()));
        });
        let server = server_until_cancel(cancel.clone());

        supervise_http_and_writer(
            server,
            writer,
            pending_signal(),
            &health,
            cancel,
            Duration::from_secs(2),
        )
        .await
        .unwrap();
        assert!(matches!(health.terminal(), Some(IngestTerminal::Cancelled)));
    }

    #[tokio::test]
    async fn shutdown_timeout_marks_writer_crashed() {
        let health = IngestHealth::new();
        health.mark_started();
        let cancel = CancellationToken::new();
        let writer = tokio::spawn(async {
            std::future::pending::<()>().await;
        });
        let server = async { Ok(()) };

        supervise_http_and_writer(
            server,
            writer,
            pending_signal(),
            &health,
            cancel.clone(),
            Duration::from_millis(20),
        )
        .await
        .unwrap();
        assert!(cancel.is_cancelled());
        assert!(matches!(
            health.terminal(),
            Some(IngestTerminal::Crashed { .. })
        ));
        assert!(!health.writer_running());
    }
}
