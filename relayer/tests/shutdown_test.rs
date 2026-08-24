//! Graceful shutdown tests.
//!
//! These assert only that shutdown terminates and that no step reached its budget - a
//! regression guard for adding a background task without a shutdown token.
//!
//! The test config sets `shutdown.lb_propagation_wait` to zero, so the times below measure
//! the work of shutting down rather than the wait for a load balancer that is not there.
//!
//! Not covered: SIGTERM is never sent - these cancel the token directly and do not run as
//! PID 1. Remaining gaps are in the T007 task notes.

mod common;

use crate::common::utils::TestSetup;
use rstest::rstest;
use std::time::{Duration, Instant};
use tokio::time::timeout;

/// Observed drains are microseconds and the smallest budget is 3s, so this sits clear of
/// both.
const COOPERATIVE_SHUTDOWN_MAX: Duration = Duration::from_secs(2);

/// Separates "slow" from "hung" so a deadlock fails instead of stalling the suite.
const SHUTDOWN_HANG_LIMIT: Duration = Duration::from_secs(60);

/// Shutdown terminates, and no step falls back to aborting.
#[rstest]
#[tokio::test]
async fn test_shutdown_completes_without_reaching_budgets() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let start = Instant::now();
    timeout(SHUTDOWN_HANG_LIMIT, setup.shutdown())
        .await
        .expect("shutdown never completed");
    let elapsed = start.elapsed();

    assert!(
        elapsed < COOPERATIVE_SHUTDOWN_MAX,
        "shutdown took {elapsed:?}, so a step drained to its budget and aborted a task \
         instead of the task returning on cancellation"
    );
}

/// The same, once every task is idle at its steady-state wait - the case that catches a task
/// which only notices cancellation while handling an inbound item.
#[rstest]
#[tokio::test]
async fn test_shutdown_from_idle_relayer() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    tokio::time::sleep(Duration::from_millis(500)).await;

    let start = Instant::now();
    timeout(SHUTDOWN_HANG_LIMIT, setup.shutdown())
        .await
        .expect("shutdown never completed from idle");
    let elapsed = start.elapsed();

    assert!(
        elapsed < COOPERATIVE_SHUTDOWN_MAX,
        "idle shutdown took {elapsed:?}, so a step drained to its budget"
    );
}

/// The relayer keeps serving while its health check fails, so a request routed before the
/// endpoint removal reaches the ingress controller gets a 503 from a pod still listening
/// rather than a refused connection.
#[rstest]
#[tokio::test]
async fn test_health_reports_503_while_still_serving() {
    let setup = TestSetup::new_with_lb_propagation_wait("5s")
        .await
        .expect("Failed to create test setup");
    let url = format!("http://localhost:{}/healthz", setup.http_port);

    assert_eq!(
        reqwest::get(&url).await.unwrap().status(),
        200,
        "the relayer should be healthy before shutdown starts"
    );

    setup.begin_shutdown();
    tokio::time::sleep(Duration::from_millis(500)).await;

    let response = reqwest::get(&url)
        .await
        .expect("the HTTP server closed inside the propagation window");
    assert_eq!(
        response.status(),
        503,
        "readiness should already be failing while the server still serves"
    );

    setup.shutdown().await;
}
