//! Backpressure, timeouts and listener failures. The flow is shared by every route, so these
//! tests only run on the public decryption route, except for the per-route notification channels.

mod common;

use crate::common::{
    PUBLIC_REQUESTS_TABLE, Route, assert_error, complete_public, count_rows, rand_public_request,
    request_status, setup, setup_with, wait_for_request_row,
};
use alloy::transports::http::reqwest::{Client, StatusCode};
use endpoint::core::Config;
use kms_connector_api::{ErrorCode, PUBLIC_DECRYPTION_ROUTE};
use rstest::rstest;
use std::time::{Duration, Instant};
use tracing::info;

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_overloaded_replica_answers_503() -> anyhow::Result<()> {
    let endpoint = setup_with(|config| Config {
        max_in_flight_decryptions: 1,
        ..config
    })
    .await?;

    // Fills the single in-flight slot.
    let first_request = rand_public_request();
    let first = endpoint.spawn_post(first_request.clone());
    wait_for_request_row(&endpoint.db, PUBLIC_REQUESTS_TABLE, first_request.id()).await;

    let second_request = rand_public_request();
    let response = endpoint.post(second_request.clone()).await;
    assert_error(
        response,
        StatusCode::SERVICE_UNAVAILABLE,
        ErrorCode::Overloaded,
        second_request.id(),
    )
    .await;
    // No DB access when overloaded.
    assert_eq!(count_rows(&endpoint.db, PUBLIC_REQUESTS_TABLE).await, 1);

    complete_public(&endpoint.db, first_request.id()).await?;
    assert_eq!(first.await?.status(), StatusCode::OK);

    endpoint.stop().await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_client_disconnect_releases_permit() -> anyhow::Result<()> {
    let endpoint = setup_with(|config| Config {
        max_in_flight_decryptions: 1,
        ..config
    })
    .await?;

    // A client that gives up: its connection is dropped after the request was admitted.
    let impatient = Client::builder()
        .timeout(Duration::from_millis(500))
        .build()?;
    let abandoned = rand_public_request();
    let err = impatient
        .post(endpoint.url(PUBLIC_DECRYPTION_ROUTE))
        .json(&abandoned)
        .send()
        .await
        .unwrap_err();
    assert!(err.is_timeout());
    wait_for_request_row(&endpoint.db, PUBLIC_REQUESTS_TABLE, abandoned.id()).await;

    // The permit was released: the next request gets admitted instead of a 503. Its response is
    // pre-inserted since the permit is taken before any DB access.
    let next = rand_public_request();
    complete_public(&endpoint.db, next.id()).await?;
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let response = endpoint.post(next.clone()).await;
        if response.status() != StatusCode::SERVICE_UNAVAILABLE {
            assert_eq!(response.status(), StatusCode::OK);
            break;
        }
        assert!(Instant::now() < deadline, "permit was never released");
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    endpoint.stop().await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_decryption_timeout_answers_504_and_releases_permit() -> anyhow::Result<()> {
    let endpoint = setup_with(|config| Config {
        max_in_flight_decryptions: 1,
        decryption_timeout: Duration::from_millis(500),
        ..config
    })
    .await?;

    let request = rand_public_request();
    let id = request.id();
    let started = Instant::now();
    let response = endpoint.post(request.clone()).await;
    assert!(started.elapsed() >= Duration::from_millis(500));
    assert_error(
        response,
        StatusCode::GATEWAY_TIMEOUT,
        ErrorCode::Timeout,
        id,
    )
    .await;

    // The request is still being processed, and a resubmission attaches to it.
    assert_eq!(
        request_status(&endpoint.db, PUBLIC_REQUESTS_TABLE, id)
            .await
            .as_deref(),
        Some("pending")
    );
    let resubmitted = endpoint.spawn_post(request);
    complete_public(&endpoint.db, id).await?;
    assert_eq!(resubmitted.await?.status(), StatusCode::OK);
    assert_eq!(count_rows(&endpoint.db, PUBLIC_REQUESTS_TABLE).await, 1);

    // The timed out request released its permit: no 503 on the resubmission above, and none here.
    let next = rand_public_request();
    complete_public(&endpoint.db, next.id()).await?;
    assert_eq!(endpoint.post(next).await.status(), StatusCode::OK);

    endpoint.stop().await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_listener_reconnection_fails_in_flight_requests_fast() -> anyhow::Result<()> {
    let mut endpoint = setup().await?;

    let request = rand_public_request();
    let pending = endpoint.spawn_post(request.clone());
    wait_for_request_row(&endpoint.db, PUBLIC_REQUESTS_TABLE, request.id()).await;

    info!("Killing the listener connection...");
    let killed = sqlx::query(
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity
        WHERE datname = current_database() AND pid <> pg_backend_pid() AND query LIKE 'LISTEN%'",
    )
    .execute(&endpoint.db)
    .await?;
    assert_eq!(
        killed.rows_affected(),
        1,
        "expected exactly one LISTEN connection"
    );

    // The waiter is failed fast: its notification may have been lost during the gap.
    let started = Instant::now();
    let response = pending.await?;
    assert!(
        started.elapsed() < Duration::from_secs(5),
        "waiter was not failed fast"
    );
    assert_error(
        response,
        StatusCode::BAD_GATEWAY,
        ErrorCode::UpstreamTransient,
        request.id(),
    )
    .await;
    endpoint
        .test_instance
        .wait_for_log("connection was lost and re-established")
        .await;

    info!("The service keeps running on a fresh listener connection, for every channel...");
    for route in [Route::Public, Route::User, Route::Pad] {
        let next = route.request();
        let pending = endpoint.spawn_post(next.clone());
        wait_for_request_row(&endpoint.db, route.requests_table(), next.id()).await;
        route.complete(&endpoint.db, next.id()).await?;
        assert_eq!(
            route.decryption_id(pending.await?).await,
            next.id(),
            "{route:?}"
        );
    }

    endpoint.stop().await
}
