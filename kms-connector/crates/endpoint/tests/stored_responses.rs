//! How stored request and response rows are served, retried or attached to, on every route.

mod common;

use crate::common::{
    Route, age_response, assert_error, count_rows, error_body, replace_error_with_success,
    request_status, setup, wait_for_request_row, wait_for_request_status,
};
use alloy::transports::http::reqwest::StatusCode;
use connector_utils::types::db::OperationStatus;
use kms_connector_api::ErrorCode;
use rstest::rstest;
use sqlx::types::chrono::Utc;
use std::time::Duration;
use tracing::info;

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_duplicate_submissions_share_one_request(
    #[values(Route::Public, Route::User, Route::Pad)] route: Route,
) -> anyhow::Result<()> {
    let endpoint = setup().await?;

    let request = route.request();
    let id = request.id();
    let first = endpoint.spawn_post(request.clone());
    let second = endpoint.spawn_post(request);

    wait_for_request_row(&endpoint.db, route.requests_table(), id).await;
    route.complete(&endpoint.db, id).await?;

    for pending in [first, second] {
        let response = pending.await?;
        assert_eq!(route.decryption_id(response).await, id);
    }
    assert_eq!(count_rows(&endpoint.db, route.requests_table()).await, 1);

    endpoint.stop().await
}

/// A stored payload is served whatever its age, even for a request past its validity window.
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_existing_payload_row_is_served_immediately(
    #[values(Route::Public, Route::User, Route::Pad)] route: Route,
) -> anyhow::Result<()> {
    let endpoint = setup().await?;

    let request = route
        .request()
        .with_validity(Utc::now().timestamp() as u64 - 600, 300);
    let id = request.id();
    route.complete(&endpoint.db, id).await?;
    age_response(
        &endpoint.db,
        route.responses_table(),
        id,
        Duration::from_hours(3650 * 24),
    )
    .await?;

    let response = endpoint.post(request).await;
    assert_eq!(route.decryption_id(response).await, id);
    // Served from the response table: no request row was inserted.
    assert_eq!(count_rows(&endpoint.db, route.requests_table()).await, 0);

    endpoint.stop().await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_admitted_request_can_receive_result_after_expiry(
    #[values(Route::User, Route::Pad)] route: Route,
) -> anyhow::Result<()> {
    let endpoint = setup().await?;

    let start = Utc::now().timestamp() as u64;
    let request = route.request().with_validity(start, 2);
    let id = request.id();
    let pending = endpoint.spawn_post(request.clone());
    wait_for_request_row(&endpoint.db, route.requests_table(), id).await;
    while Utc::now().timestamp() as u64 <= start + 2 {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    route.complete(&endpoint.db, id).await?;
    assert_eq!(route.decryption_id(pending.await?).await, id);

    // Later submissions also receive the stored success after expiry.
    let response = endpoint.post(request).await;
    assert_eq!(route.decryption_id(response).await, id);
    assert_eq!(count_rows(&endpoint.db, route.responses_table()).await, 1);

    endpoint.stop().await
}

#[rstest]
#[case(ErrorCode::KmsContextDestroyed, StatusCode::GONE)]
#[case(ErrorCode::Unprocessable, StatusCode::UNPROCESSABLE_ENTITY)]
#[case(ErrorCode::UserDecryptionExpired, StatusCode::FORBIDDEN)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_non_retryable_error_row_is_served(
    #[values(Route::Public, Route::User, Route::Pad)] route: Route,
    #[case] code: ErrorCode,
    #[case] expected_status: StatusCode,
) -> anyhow::Result<()> {
    let endpoint = setup().await?;

    let request = route
        .request()
        .with_validity(Utc::now().timestamp() as u64 - 600, 300);
    let id = request.id();
    route.fail(&endpoint.db, id, code.as_str(), "why").await?;
    age_response(
        &endpoint.db,
        route.responses_table(),
        id,
        Duration::from_mins(30),
    )
    .await?;

    let response = endpoint.post(request).await;
    let body = assert_error(response, expected_status, code, id).await;
    assert_eq!(body.message, "why");
    // Served from the response table: no request row was inserted.
    assert_eq!(count_rows(&endpoint.db, route.requests_table()).await, 0);

    endpoint.stop().await
}

/// Irrecoverable errors past the serve window use the retry path, so a fresh error or a successful
/// payload can replace the old error.
#[rstest]
#[case::fresh_error(false)]
#[case::success(true)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_expired_irrecoverable_error_is_reprocessed(
    #[values(Route::Public, Route::User, Route::Pad)] route: Route,
    #[case] success: bool,
) -> anyhow::Result<()> {
    let endpoint = setup().await?;

    let request = route.request();
    let id = request.id();
    request
        .insert(&endpoint.db, OperationStatus::Failed)
        .await?;
    route
        .fail(&endpoint.db, id, "unprocessable", "before")
        .await?;
    age_response(
        &endpoint.db,
        route.responses_table(),
        id,
        Duration::from_hours(2),
    )
    .await?;

    let pending = endpoint.spawn_post(request.clone());
    wait_for_request_status(&endpoint.db, route.requests_table(), id, "pending").await;
    if success {
        replace_error_with_success(
            &endpoint.db,
            route.responses_table(),
            route.payload_column(),
            id,
        )
        .await?;
        assert_eq!(route.decryption_id(pending.await?).await, id);
        assert_eq!(route.decryption_id(endpoint.post(request).await).await, id);
    } else {
        route
            .fail(&endpoint.db, id, "kms_context_destroyed", "after")
            .await?;
        assert_eq!(error_body(pending.await?).await.message, "after");
        // The fresh error is within its serve window: served as is.
        assert_error(
            endpoint.post(request).await,
            StatusCode::GONE,
            ErrorCode::KmsContextDestroyed,
            id,
        )
        .await;
    }
    assert_eq!(count_rows(&endpoint.db, route.requests_table()).await, 1);
    assert_eq!(count_rows(&endpoint.db, route.responses_table()).await, 1);

    endpoint.stop().await
}

/// A stored retryable error is never served: the `failed` request has its status reset to
/// `pending` so the kms-worker re-processes it, and the client gets the fresh outcome (here, a
/// new error row overriding the previous one, as the kms-worker does).
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_retryable_error_row_resets_failed_request(
    #[values(Route::Public, Route::User, Route::Pad)] route: Route,
) -> anyhow::Result<()> {
    let endpoint = setup().await?;

    let retryable_codes = [
        "acl_denied",
        "user_signature_rejected",
        "ciphertext_not_found",
        "kms_context_invalid",
        "copro_consensus_failed",
        "upstream_transient",
        "some_future_code",
    ];
    for (i, stored_code) in retryable_codes.into_iter().enumerate() {
        info!("Decrypting after a `{stored_code}` failure...");
        let request = route.request();
        let id = request.id();
        request
            .insert(&endpoint.db, OperationStatus::Failed)
            .await?;
        route.fail(&endpoint.db, id, stored_code, "before").await?;

        let pending = endpoint.spawn_post(request);
        wait_for_request_status(&endpoint.db, route.requests_table(), id, "pending").await;
        assert_eq!(
            count_rows(&endpoint.db, route.requests_table()).await,
            i as i64 + 1
        );
        route
            .fail(&endpoint.db, id, "unprocessable", "after")
            .await?;

        let body = assert_error(
            pending.await?,
            StatusCode::UNPROCESSABLE_ENTITY,
            ErrorCode::Unprocessable,
            id,
        )
        .await;
        assert_eq!(body.message, "after", "{stored_code}");
    }

    endpoint.stop().await
}

/// A retryable error row whose request is already being re-processed does not reset anything: the
/// connection attaches to the in-flight processing.
#[rstest]
#[case::recoverable("acl_denied", Duration::ZERO)]
#[case::expired_irrecoverable("unprocessable", Duration::from_hours(2))]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_retryable_or_expired_error_row_attaches_to_reprocessing(
    #[values(Route::Public, Route::User, Route::Pad)] route: Route,
    #[case] code: &str,
    #[case] age: Duration,
) -> anyhow::Result<()> {
    let endpoint = setup().await?;

    info!("Already under process by the kms-worker: attach, do not touch the request...");
    let request = route.request();
    let id = request.id();
    request
        .insert(&endpoint.db, OperationStatus::UnderProcess)
        .await?;
    route.fail(&endpoint.db, id, code, "before").await?;
    age_response(&endpoint.db, route.responses_table(), id, age).await?;

    let pending = endpoint.spawn_post(request);
    // Nothing observable happens in DB until the worker answers, so give the route time to run.
    tokio::time::sleep(Duration::from_millis(300)).await;
    assert_eq!(
        request_status(&endpoint.db, route.requests_table(), id)
            .await
            .as_deref(),
        Some("under_process")
    );
    route
        .fail(&endpoint.db, id, "kms_context_destroyed", "after")
        .await?;
    let response = pending.await?;
    assert_eq!(response.status(), StatusCode::GONE);
    assert_eq!(error_body(response).await.message, "after");

    endpoint.stop().await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_error_row_notified_while_waiting(
    #[values(Route::Public, Route::User, Route::Pad)] route: Route,
) -> anyhow::Result<()> {
    let endpoint = setup().await?;

    let request = route.request();
    let id = request.id();
    let pending = endpoint.spawn_post(request);
    wait_for_request_row(&endpoint.db, route.requests_table(), id).await;

    route
        .fail(&endpoint.db, id, "acl_denied", "not allowed")
        .await?;

    let response = pending.await?;
    let body = assert_error(response, StatusCode::FORBIDDEN, ErrorCode::AclDenied, id).await;
    assert_eq!(body.message, "not allowed");

    endpoint.stop().await
}
