use alloy::{
    primitives::{Address, B256, Bytes, U256},
    transports::http::reqwest::{self, Client, Response, StatusCode},
};
use connector_utils::{
    tests::{
        db::{
            requests::{
                InsertRequestOptions, insert_rand_public_decryption_request,
                insert_rand_user_decryption_request_v2,
            },
            responses::{
                insert_rand_public_decrypt_response, insert_rand_user_decrypt_response,
                insert_test_public_decrypt_error_response, insert_test_user_decrypt_error_response,
            },
        },
        setup::{TestInstance, TestInstanceBuilder, pick_free_port},
    },
    types::db::{OperationStatus, RequestSource},
};
use endpoint::{
    core::{Config, Endpoint},
    monitoring::metrics,
};
use kms_connector_api::{
    AttestationType, ErrorCode, ErrorResponse, HandleEntry, PUBLIC_DECRYPTION_ROUTE,
    PublicDecryptionRequest, PublicDecryptionResponse, RequestValidity, USER_DECRYPTION_ROUTE,
    UserDecryptionPayload, UserDecryptionRequest, UserDecryptionResponse, VERSION_ROUTE,
    VersionResponse,
};
use rstest::rstest;
use sqlx::{Pool, Postgres, Row, types::chrono::Utc};
use std::{
    net::SocketAddr,
    str::FromStr,
    time::{Duration, Instant},
};
use tfhe::FheTypes;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::info;

const CHAIN_ID: u64 = 31337;
const EUINT64: u8 = FheTypes::Uint64 as u8;
const PUBLIC_REQUESTS: &str = "public_decryption_requests";
const USER_REQUESTS: &str = "user_decryption_requests";

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_public_decrypt_happy_path() -> anyhow::Result<()> {
    use opentelemetry::trace::{TraceContextExt, TracerProvider};
    use opentelemetry_sdk::{
        propagation::TraceContextPropagator,
        trace::{InMemorySpanExporter, SdkTracerProvider},
    };
    use tracing_subscriber::layer::SubscriberExt;
    let exporter = InMemorySpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(tracing_opentelemetry::layer().with_tracer(provider.tracer("endpoint-test"))),
    )?;
    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());
    let mut endpoint = setup().await?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "traceparent",
        "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01".parse()?,
    );
    endpoint.client = Client::builder().default_headers(headers).build()?;

    let response = reqwest::get(endpoint.url(VERSION_ROUTE)).await?;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.json::<VersionResponse>().await?,
        VersionResponse::default()
    );

    let request = public_request();
    let id = request.id();
    let pending = endpoint.spawn_pdec(&request);

    info!("Checking the stored request row...");
    let row = wait_for_request_row(&endpoint.db, PUBLIC_REQUESTS, id).await;
    assert_eq!(row.get::<RequestSource, _>("source"), RequestSource::Http);
    let persisted: connector_utils::monitoring::otlp::PropagationContext =
        bc2wrap::deserialize_slice(&row.get::<Vec<u8>, _>("otlp_context"))?;
    let stored_context = persisted.extract().span().span_context().clone();
    assert_eq!(
        stored_context.trace_id().to_string(),
        "4bf92f3577b34da6a3ce929d0e0e4736"
    );
    assert_eq!(
        row.get::<OperationStatus, _>("status"),
        OperationStatus::Pending
    );
    let ct_handles: Vec<Vec<u8>> = row.get("ct_handles");
    assert_eq!(
        ct_handles,
        request
            .ctHandles
            .iter()
            .map(|h| h.to_vec())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        row.get::<Vec<u8>, _>("extra_data"),
        request.extraData.to_vec()
    );

    info!("Playing the worker: inserting the response row...");
    let inserted = complete_public(&endpoint.db, id).await?;

    let response = pending.await?;
    assert_eq!(response.status(), StatusCode::OK);
    let body: PublicDecryptionResponse = response.json().await?;
    assert_eq!(body.decryption_id, id);
    assert_eq!(body.decrypted_result.to_vec(), inserted.decrypted_result);
    assert_eq!(body.signature.to_vec(), inserted.signature);
    assert_eq!(body.extra_data.to_vec(), inserted.extra_data);

    assert_requests("public_decrypt", "200", "success", 1).await;
    assert_eq!(lookups("public_decrypt", "stored"), 1);
    assert_eq!(
        metrics::DECRYPTION_WAIT_LATENCY_HISTOGRAM
            .with_label_values(&["public_decrypt"])
            .get_sample_count(),
        1
    );
    assert_eq!(admitted_requests(), 0);
    endpoint.stop().await?;
    provider.force_flush()?;
    let spans = exporter.get_finished_spans()?;
    let handler = spans
        .iter()
        .find(|span| span.span_context.span_id() == stored_context.span_id())
        .expect("stored handler span");
    let http = spans
        .iter()
        .find(|span| span.span_context.span_id() == handler.parent_span_id)
        .expect("HTTP parent span");
    assert_eq!(http.name, "http_request");
    assert_eq!(http.parent_span_id.to_string(), "00f067aa0ba902b7");
    assert_eq!(
        spans
            .iter()
            .filter(|span| span.name == "http_request")
            .count(),
        1,
        "version probes must not export spans"
    );
    Ok(())
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_user_decrypt_happy_path() -> anyhow::Result<()> {
    let endpoint = setup().await?;

    let request = user_request();
    let id = request.id();
    let pending = endpoint.spawn_udec(&request);

    info!("Checking the stored RFC016 request row...");
    let row = wait_for_request_row(&endpoint.db, USER_REQUESTS, id).await;
    let payload = &request.payload;
    assert_eq!(row.get::<RequestSource, _>("source"), RequestSource::Http);
    assert_eq!(
        row.get::<OperationStatus, _>("status"),
        OperationStatus::Pending
    );
    assert_eq!(
        row.get::<Vec<Vec<u8>>, _>("ct_handles"),
        vec![payload.handles[0].handle.to_vec()]
    );
    assert_eq!(
        row.get::<Vec<Vec<u8>>, _>("handle_owner_addresses"),
        vec![payload.handles[0].ownerAddress.to_vec()]
    );
    assert_eq!(
        row.get::<Vec<Vec<u8>>, _>("handle_contract_addresses"),
        vec![payload.handles[0].contractAddress.to_vec()]
    );
    assert_eq!(
        row.get::<Vec<Vec<u8>>, _>("allowed_contracts"),
        vec![payload.allowedContracts[0].to_vec()]
    );
    assert_eq!(
        row.get::<Vec<u8>, _>("user_address"),
        payload.userAddress.to_vec()
    );
    assert_eq!(
        row.get::<Vec<u8>, _>("public_key"),
        payload.publicKey.to_vec()
    );
    assert_eq!(
        row.get::<i64, _>("start_timestamp"),
        payload.requestValidity.startTimestamp as i64
    );
    assert_eq!(
        row.get::<i64, _>("duration_seconds"),
        payload.requestValidity.durationSeconds as i64
    );
    assert_eq!(
        row.get::<Option<Vec<u8>>, _>("signature"),
        Some(request.signature.to_vec())
    );

    info!("Playing the worker: inserting the response row...");
    let inserted = complete_udec(&endpoint.db, id).await?;

    let response = pending.await?;
    assert_eq!(response.status(), StatusCode::OK);
    let body: UserDecryptionResponse = response.json().await?;
    assert_eq!(body.decryption_id, id);
    assert_eq!(
        body.user_decrypted_shares.to_vec(),
        inserted.user_decrypted_shares
    );
    assert_eq!(body.signature.to_vec(), inserted.signature);

    assert_requests("user_decrypt", "200", "success", 1).await;
    assert_eq!(lookups("user_decrypt", "stored"), 1);
    endpoint.stop().await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_duplicate_submissions_share_one_request() -> anyhow::Result<()> {
    let endpoint = setup().await?;

    let request = public_request();
    let id = request.id();
    let first = endpoint.spawn_pdec(&request);
    let second = endpoint.spawn_pdec(&request);

    wait_for_request_row(&endpoint.db, PUBLIC_REQUESTS, id).await;
    wait_for_counter(|| lookups("public_decrypt", "already_in_progress"), 1).await;
    assert_eq!(admitted_requests(), 2);
    complete_public(&endpoint.db, id).await?;

    for pending in [first, second] {
        let response = pending.await?;
        assert_eq!(response.status(), StatusCode::OK);
        let body: PublicDecryptionResponse = response.json().await?;
        assert_eq!(body.decryption_id, id);
    }
    assert_eq!(count_rows(&endpoint.db, PUBLIC_REQUESTS).await, 1);

    assert_requests("public_decrypt", "200", "success", 2).await;
    assert_eq!(lookups("public_decrypt", "stored"), 1);
    assert_eq!(lookups("public_decrypt", "already_in_progress"), 1);
    assert_eq!(admitted_requests(), 0);
    endpoint.stop().await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_existing_payload_row_is_served_immediately() -> anyhow::Result<()> {
    let endpoint = setup().await?;

    let public = public_request();
    let inserted = complete_public(&endpoint.db, public.id()).await?;
    age_response(
        &endpoint.db,
        "public_decryption_responses",
        public.id(),
        Duration::from_hours(3650 * 24),
    )
    .await?;
    let response = endpoint.post_public(&public).await;
    assert_eq!(response.status(), StatusCode::OK);
    let body: PublicDecryptionResponse = response.json().await?;
    assert_eq!(body.decryption_id, public.id());
    assert_eq!(body.decrypted_result.to_vec(), inserted.decrypted_result);

    let mut user = user_request();
    user.payload.requestValidity.startTimestamp = Utc::now().timestamp() as u64 - 600;
    let inserted = complete_udec(&endpoint.db, user.id()).await?;
    age_response(
        &endpoint.db,
        "user_decryption_responses",
        user.id(),
        Duration::from_hours(3650 * 24),
    )
    .await?;
    let response = endpoint.post_user(&user).await;
    assert_eq!(response.status(), StatusCode::OK);
    let body: UserDecryptionResponse = response.json().await?;
    assert_eq!(body.decryption_id, user.id());
    assert_eq!(
        body.user_decrypted_shares.to_vec(),
        inserted.user_decrypted_shares
    );

    // Served from the response table: no request row was inserted.
    assert_eq!(count_rows(&endpoint.db, PUBLIC_REQUESTS).await, 0);
    assert_eq!(count_rows(&endpoint.db, USER_REQUESTS).await, 0);

    for route in ["public_decrypt", "user_decrypt"] {
        assert_eq!(lookups(route, "served"), 1);
        assert_eq!(
            metrics::DECRYPTION_WAIT_LATENCY_HISTOGRAM
                .with_label_values(&[route])
                .get_sample_count(),
            0
        );
    }
    endpoint.stop().await
}

#[tokio::test]
async fn admitted_request_can_receive_result_after_expiry() -> anyhow::Result<()> {
    let endpoint = setup().await?;
    let mut request = user_request();
    request.payload.requestValidity.startTimestamp = Utc::now().timestamp() as u64;
    request.payload.requestValidity.durationSeconds = 2;
    let id = request.id();
    let pending = endpoint.spawn_udec(&request);
    wait_for_request_row(&endpoint.db, USER_REQUESTS, id).await;
    let end = request.payload.requestValidity.startTimestamp + 2;
    while Utc::now().timestamp() as u64 <= end {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    complete_udec(&endpoint.db, id).await?;
    let response = pending.await?;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .json::<UserDecryptionResponse>()
            .await?
            .decryption_id,
        id
    );
    // Later submissions also receive the stored success after expiry.
    let response = endpoint.post_user(&request).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .json::<UserDecryptionResponse>()
            .await?
            .decryption_id,
        id
    );
    assert_eq!(
        count_rows(&endpoint.db, "user_decryption_responses").await,
        1
    );
    endpoint.stop().await
}

#[rstest]
#[case(
    "kms_context_destroyed",
    ErrorCode::KmsContextDestroyed,
    StatusCode::GONE
)]
#[case(
    "unprocessable",
    ErrorCode::Unprocessable,
    StatusCode::UNPROCESSABLE_ENTITY
)]
#[case(
    "user_decryption_expired",
    ErrorCode::UserDecryptionExpired,
    StatusCode::FORBIDDEN
)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_non_retryable_error_row_is_served(
    #[case] stored_code: &str,
    #[case] expected_code: ErrorCode,
    #[case] expected_status: StatusCode,
) -> anyhow::Result<()> {
    let endpoint = setup().await?;

    let public = public_request();
    fail_public(&endpoint.db, public.id(), stored_code, "why").await?;
    age_response(
        &endpoint.db,
        "public_decryption_responses",
        public.id(),
        Duration::from_mins(30),
    )
    .await?;
    let response = endpoint.post_public(&public).await;
    let body = assert_error(response, expected_status, expected_code, public.id()).await;
    assert_eq!(body.message, "why");

    let mut user = user_request();
    user.payload.requestValidity.startTimestamp = Utc::now().timestamp() as u64 - 600;
    fail_user(&endpoint.db, user.id(), stored_code, "why").await?;
    age_response(
        &endpoint.db,
        "user_decryption_responses",
        user.id(),
        Duration::from_mins(30),
    )
    .await?;
    let response = endpoint.post_user(&user).await;
    assert_error(response, expected_status, expected_code, user.id()).await;

    // Served from the response table: no request row was inserted.
    assert_eq!(count_rows(&endpoint.db, PUBLIC_REQUESTS).await, 0);
    assert_eq!(count_rows(&endpoint.db, USER_REQUESTS).await, 0);

    endpoint.stop().await
}

/// Irrecoverable errors past the serve window use the existing retry path on both routes.
/// A fresh error or a successful payload can replace the old error.
#[rstest]
#[case::fresh_error(false)]
#[case::success(true)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_expired_irrecoverable_error_is_reprocessed(
    #[case] success: bool,
) -> anyhow::Result<()> {
    let endpoint = setup().await?;
    let public = public_request();
    let id = public.id();
    insert_rand_public_decryption_request(&endpoint.db, failed_http_request(id)).await?;
    fail_public(&endpoint.db, id, "unprocessable", "before").await?;
    age_response(
        &endpoint.db,
        "public_decryption_responses",
        id,
        Duration::from_hours(2),
    )
    .await?;

    let pending = endpoint.spawn_pdec(&public);
    wait_for_request_status(&endpoint.db, PUBLIC_REQUESTS, id, "pending").await;
    if success {
        replace_error_with_success(
            &endpoint.db,
            "public_decryption_responses",
            "decrypted_result",
            id,
        )
        .await?;
    } else {
        fail_public(&endpoint.db, id, "kms_context_destroyed", "after").await?;
    }
    let response = pending.await?;
    if success {
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(endpoint.post_public(&public).await.status(), StatusCode::OK);
    } else {
        assert_eq!(error_body(response).await.message, "after");
        assert_error(
            endpoint.post_public(&public).await,
            StatusCode::GONE,
            ErrorCode::KmsContextDestroyed,
            id,
        )
        .await;
    }

    let user = user_request();
    let id = user.id();
    insert_rand_user_decryption_request_v2(&endpoint.db, failed_http_request(id)).await?;
    fail_user(&endpoint.db, id, "unprocessable", "before").await?;
    age_response(
        &endpoint.db,
        "user_decryption_responses",
        id,
        Duration::from_hours(2),
    )
    .await?;

    let pending = endpoint.spawn_udec(&user);
    wait_for_request_status(&endpoint.db, USER_REQUESTS, id, "pending").await;
    if success {
        replace_error_with_success(
            &endpoint.db,
            "user_decryption_responses",
            "user_decrypted_shares",
            id,
        )
        .await?;
    } else {
        fail_user(&endpoint.db, id, "kms_context_destroyed", "after").await?;
    }
    let response = pending.await?;
    if success {
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(endpoint.post_user(&user).await.status(), StatusCode::OK);
    } else {
        assert_eq!(error_body(response).await.message, "after");
        assert_error(
            endpoint.post_user(&user).await,
            StatusCode::GONE,
            ErrorCode::KmsContextDestroyed,
            id,
        )
        .await;
    }
    assert_eq!(count_rows(&endpoint.db, PUBLIC_REQUESTS).await, 1);
    assert_eq!(count_rows(&endpoint.db, USER_REQUESTS).await, 1);
    assert_eq!(
        count_rows(&endpoint.db, "public_decryption_responses").await,
        1
    );
    assert_eq!(
        count_rows(&endpoint.db, "user_decryption_responses").await,
        1
    );
    for route in ["public_decrypt", "user_decrypt"] {
        assert_eq!(lookups(route, "retried"), 1);
        assert_eq!(lookups(route, "served"), 1);
    }
    endpoint.stop().await
}

/// A stored retryable error is never served: the `failed` request has its status reset to
/// `pending` so the kms-worker re-processes it, and the client gets the fresh outcome (here, a
/// new error row overriding the previous one, as the kms-worker does).
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_retryable_error_row_resets_failed_request() -> anyhow::Result<()> {
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
        info!("Public decrypt after a `{stored_code}` failure...");
        let public = public_request();
        let id = public.id();
        insert_rand_public_decryption_request(&endpoint.db, failed_http_request(id)).await?;
        fail_public(&endpoint.db, id, stored_code, "before").await?;

        let pending = endpoint.spawn_pdec(&public);
        wait_for_request_status(&endpoint.db, PUBLIC_REQUESTS, id, "pending").await;
        assert_eq!(
            count_rows(&endpoint.db, PUBLIC_REQUESTS).await,
            i as i64 + 1
        );
        fail_public(&endpoint.db, id, "unprocessable", "after").await?;

        let response = pending.await?;
        let body = assert_error(
            response,
            StatusCode::UNPROCESSABLE_ENTITY,
            ErrorCode::Unprocessable,
            id,
        )
        .await;
        assert_eq!(body.message, "after", "{stored_code}");
    }

    info!("User decrypt: same flow...");
    let user = user_request();
    let id = user.id();
    insert_rand_user_decryption_request_v2(&endpoint.db, failed_http_request(id)).await?;
    fail_user(&endpoint.db, id, "acl_denied", "before").await?;

    let pending = endpoint.spawn_udec(&user);
    wait_for_request_status(&endpoint.db, USER_REQUESTS, id, "pending").await;
    assert_eq!(count_rows(&endpoint.db, USER_REQUESTS).await, 1);
    fail_user(&endpoint.db, id, "unprocessable", "after").await?;

    let response = pending.await?;
    let body = assert_error(
        response,
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorCode::Unprocessable,
        id,
    )
    .await;
    assert_eq!(body.message, "after");

    assert_eq!(
        lookups("public_decrypt", "retried"),
        retryable_codes.len() as u64
    );
    assert_eq!(lookups("user_decrypt", "retried"), 1);
    endpoint.stop().await
}

/// A retryable error row whose request is already being re-processed (or whose request row is
/// gone) does not reset anything: the connection attaches to the in-flight processing, or a new
/// request is stored.
#[rstest]
#[timeout(Duration::from_secs(60))]
#[case::recoverable("acl_denied", Duration::ZERO)]
#[case::expired_irrecoverable("unprocessable", Duration::from_hours(2))]
#[tokio::test]
async fn test_retryable_or_expired_error_row_attaches_to_reprocessing(
    #[case] code: &str,
    #[case] age: Duration,
) -> anyhow::Result<()> {
    let endpoint = setup().await?;

    // Already under process by the kms-worker: attach, do not touch the request.
    let public = public_request();
    let public_id = public.id();
    insert_rand_public_decryption_request(
        &endpoint.db,
        failed_http_request(public_id).with_status(OperationStatus::UnderProcess),
    )
    .await?;
    fail_public(&endpoint.db, public_id, code, "before").await?;
    age_response(&endpoint.db, "public_decryption_responses", public_id, age).await?;

    let pending = endpoint.spawn_pdec(&public);
    // Attachment is observable without changing the request row or consuming the old error.
    wait_for_counter(|| lookups("public_decrypt", "already_in_progress"), 1).await;
    assert_eq!(
        request_status(&endpoint.db, PUBLIC_REQUESTS, public_id)
            .await
            .as_deref(),
        Some("under_process")
    );
    fail_public(&endpoint.db, public_id, "kms_context_destroyed", "after").await?;
    let response = pending.await?;
    assert_eq!(response.status(), StatusCode::GONE);
    assert_eq!(error_body(response).await.message, "after");

    // Request row gone (garbage collected) but error row still there: a fresh request is stored.
    let user = user_request();
    let user_id = user.id();
    fail_user(&endpoint.db, user_id, code, "before").await?;
    age_response(&endpoint.db, "user_decryption_responses", user_id, age).await?;
    let pending = endpoint.spawn_udec(&user);
    wait_for_request_status(&endpoint.db, USER_REQUESTS, user_id, "pending").await;
    fail_user(&endpoint.db, user_id, "unprocessable", "after").await?;
    let response = pending.await?;
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(error_body(response).await.message, "after");

    assert_eq!(lookups("public_decrypt", "already_in_progress"), 1);
    assert_eq!(lookups("public_decrypt", "retried"), 0);
    assert_eq!(lookups("user_decrypt", "retried"), 1);
    endpoint.stop().await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_error_row_notified_while_waiting() -> anyhow::Result<()> {
    let endpoint = setup().await?;

    let request = user_request();
    let id = request.id();
    let pending = endpoint.spawn_udec(&request);
    wait_for_request_row(&endpoint.db, USER_REQUESTS, id).await;

    fail_user(&endpoint.db, id, "acl_denied", "not allowed").await?;

    let response = pending.await?;
    let body = assert_error(response, StatusCode::FORBIDDEN, ErrorCode::AclDenied, id).await;
    assert_eq!(body.message, "not allowed");

    endpoint.stop().await
}

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
    let first_request = public_request();
    let first = endpoint.spawn_pdec(&first_request);
    wait_for_request_row(&endpoint.db, PUBLIC_REQUESTS, first_request.id()).await;

    assert_eq!(admitted_requests(), 1);
    let second_request = public_request();
    let response = endpoint.post_public(&second_request).await;
    assert_error(
        response,
        StatusCode::SERVICE_UNAVAILABLE,
        ErrorCode::Overloaded,
        second_request.id(),
    )
    .await;
    // No DB access when overloaded.
    assert_eq!(count_rows(&endpoint.db, PUBLIC_REQUESTS).await, 1);

    complete_public(&endpoint.db, first_request.id()).await?;
    assert_eq!(first.await?.status(), StatusCode::OK);

    assert_requests("public_decrypt", "503", "overloaded", 1).await;
    assert_eq!(admitted_requests(), 0);
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
    let abandoned = public_request();
    let err = impatient
        .post(endpoint.url(PUBLIC_DECRYPTION_ROUTE))
        .json(&abandoned)
        .send()
        .await
        .unwrap_err();
    assert!(err.is_timeout());
    wait_for_request_row(&endpoint.db, PUBLIC_REQUESTS, abandoned.id()).await;

    // The permit was released: the next request gets admitted instead of a 503. Its response is
    // pre-inserted since the permit is taken before any DB access.
    let next = public_request();
    complete_public(&endpoint.db, next.id()).await?;
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let response = endpoint.post_public(&next).await;
        if response.status() != StatusCode::SERVICE_UNAVAILABLE {
            assert_eq!(response.status(), StatusCode::OK);
            break;
        }
        assert!(Instant::now() < deadline, "permit was never released");
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    assert_requests("public_decrypt", "0", "cancelled", 1).await;
    assert_eq!(admitted_requests(), 0);
    endpoint.stop().await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_malformed_requests_answer_400_without_db_access() -> anyhow::Result<()> {
    let endpoint = setup_with(|config| Config {
        max_body_bytes: 1024,
        ..config
    })
    .await?;

    // Invalid JSON.
    let response = endpoint
        .post_raw(PUBLIC_DECRYPTION_ROUTE, "{not json")
        .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = error_body(response).await;
    assert_eq!(body.code, ErrorCode::Malformed);
    assert!(!body.retryable);
    assert_eq!(body.decryption_id, None);

    // Unknown field.
    let raw = format!(
        r#"{{"ctHandles":["{}"],"extraData":"0x00","sneaky":1}}"#,
        rand_handle(CHAIN_ID)
    );
    let response = endpoint.post_raw(PUBLIC_DECRYPTION_ROUTE, raw).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(error_body(response).await.code, ErrorCode::Malformed);

    // Oversized body.
    let response = endpoint
        .post_public(&PublicDecryptionRequest {
            ctHandles: vec![rand_handle(CHAIN_ID); 32],
            extraData: Bytes::from(vec![0x00]),
        })
        .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(error_body(response).await.code, ErrorCode::Malformed);

    // Semantic validation failures.
    let response = endpoint
        .post_public(&PublicDecryptionRequest {
            ctHandles: vec![],
            extraData: Bytes::new(),
        })
        .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let response = endpoint
        .post_public(&PublicDecryptionRequest {
            ctHandles: vec![rand_handle(CHAIN_ID + 1)],
            extraData: Bytes::new(),
        })
        .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = error_body(response).await;
    assert_eq!(body.code, ErrorCode::Malformed);
    assert!(body.message.contains("unsupported chain id"));

    // Empty handle list.
    let mut request = user_request();
    request.payload.handles.clear();
    let response = endpoint.post_user(&request).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(error_body(response).await.code, ErrorCode::Malformed);

    // Unsupported attestation type.
    let mut request = user_request();
    request.attestationType = "random_attestation_type".to_owned();
    let response = endpoint.post_user(&request).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = error_body(response).await;
    assert_eq!(body.code, ErrorCode::UnsupportedAttestationType);
    assert!(!body.retryable);
    assert_eq!(body.decryption_id, None);

    assert_eq!(count_rows(&endpoint.db, PUBLIC_REQUESTS).await, 0);
    assert_eq!(count_rows(&endpoint.db, USER_REQUESTS).await, 0);

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

    let request = public_request();
    let id = request.id();
    let started = Instant::now();
    let response = endpoint.post_public(&request).await;
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
        request_status(&endpoint.db, PUBLIC_REQUESTS, id)
            .await
            .as_deref(),
        Some("pending")
    );
    let resubmitted = endpoint.spawn_pdec(&request);
    complete_public(&endpoint.db, id).await?;
    assert_eq!(resubmitted.await?.status(), StatusCode::OK);
    assert_eq!(count_rows(&endpoint.db, PUBLIC_REQUESTS).await, 1);

    // The timed out request released its permit: no 503 on the resubmission above, and none here.
    let next = public_request();
    complete_public(&endpoint.db, next.id()).await?;
    assert_eq!(endpoint.post_public(&next).await.status(), StatusCode::OK);

    assert_requests("public_decrypt", "504", "timeout", 1).await;
    assert_eq!(admitted_requests(), 0);
    endpoint.stop().await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_listener_reconnection_fails_in_flight_requests_fast() -> anyhow::Result<()> {
    let mut endpoint = setup().await?;

    let request = public_request();
    let pending = endpoint.spawn_pdec(&request);
    wait_for_request_row(&endpoint.db, PUBLIC_REQUESTS, request.id()).await;

    let duplicate = endpoint.spawn_pdec(&request);
    wait_for_counter(|| lookups("public_decrypt", "already_in_progress"), 1).await;
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

    assert_error(
        duplicate.await?,
        StatusCode::BAD_GATEWAY,
        ErrorCode::UpstreamTransient,
        request.id(),
    )
    .await;
    assert_eq!(metrics::RESPONSE_LISTENER_RECONNECTION_COUNTER.get(), 1);
    assert_eq!(metrics::RESPONSE_LISTENER_WAITERS_FAILED_COUNTER.get(), 2);
    assert_requests("public_decrypt", "502", "upstream_transient", 2).await;
    info!("The service keeps running on a fresh listener connection...");
    let next = public_request();
    let pending = endpoint.spawn_pdec(&next);
    wait_for_request_row(&endpoint.db, PUBLIC_REQUESTS, next.id()).await;
    complete_public(&endpoint.db, next.id()).await?;
    assert_eq!(pending.await?.status(), StatusCode::OK);

    endpoint.stop().await
}

//////////////////////////////////////////////////////////////////////////////////////////////////
//                                          Harness                                             //
//////////////////////////////////////////////////////////////////////////////////////////////////

/// A running endpoint, the DB it works on and the test instance owning them.
struct RunningEndpoint {
    test_instance: TestInstance,
    db: Pool<Postgres>,
    base_url: String,
    cancel_token: CancellationToken,
    task: JoinHandle<anyhow::Result<()>>,
    client: Client,
}

async fn setup() -> anyhow::Result<RunningEndpoint> {
    setup_with(|config| config).await
}

async fn setup_with(configure: impl FnOnce(Config) -> Config) -> anyhow::Result<RunningEndpoint> {
    let mut test_instance = TestInstanceBuilder::db_setup().await?;
    let db = test_instance.db().clone();
    let config = configure(Config {
        database_url: test_instance.db_url().to_string(),
        database_pool_size: 3,
        http_endpoint: SocketAddr::from_str(&format!("127.0.0.1:{}", pick_free_port())).unwrap(),
        supported_chain_ids: vec![CHAIN_ID],
        ..Config::default()
    });

    let base_url = format!("http://{}", config.http_endpoint);
    let (endpoint, _state) = Endpoint::from_config(config).await?;
    endpoint.register_metrics()?;
    let cancel_token = CancellationToken::new();
    let task = tokio::spawn(endpoint.start(cancel_token.clone()));
    test_instance.wait_for_log("HTTP server listening at").await;

    Ok(RunningEndpoint {
        test_instance,
        db,
        base_url,
        cancel_token,
        task,
        client: Client::new(),
    })
}

impl RunningEndpoint {
    fn url(&self, route: &str) -> String {
        format!("{}{}", self.base_url, route)
    }

    async fn post_public(&self, request: &PublicDecryptionRequest) -> Response {
        self.client
            .post(self.url(PUBLIC_DECRYPTION_ROUTE))
            .json(request)
            .send()
            .await
            .expect("public decrypt request failed")
    }

    async fn post_user(&self, request: &UserDecryptionRequest) -> Response {
        self.client
            .post(self.url(USER_DECRYPTION_ROUTE))
            .json(request)
            .send()
            .await
            .expect("user decrypt request failed")
    }

    /// Posts a raw JSON body to `route`.
    async fn post_raw(&self, route: &str, body: impl Into<reqwest::Body>) -> Response {
        self.client
            .post(self.url(route))
            .header("content-type", "application/json")
            .body(body)
            .send()
            .await
            .expect("raw request failed")
    }

    /// Fires a public-decrypt request in a background task.
    fn spawn_pdec(&self, request: &PublicDecryptionRequest) -> JoinHandle<Response> {
        let client = self.client.clone();
        let url = self.url(PUBLIC_DECRYPTION_ROUTE);
        let request = request.clone();
        tokio::spawn(async move {
            client
                .post(url)
                .json(&request)
                .send()
                .await
                .expect("request failed")
        })
    }

    /// Fires a user-decrypt request in a background task.
    fn spawn_udec(&self, request: &UserDecryptionRequest) -> JoinHandle<Response> {
        let client = self.client.clone();
        let url = self.url(USER_DECRYPTION_ROUTE);
        let request = request.clone();
        tokio::spawn(async move {
            client
                .post(url)
                .json(&request)
                .send()
                .await
                .expect("request failed")
        })
    }

    async fn stop(self) -> anyhow::Result<()> {
        self.cancel_token.cancel();
        self.task.await?
    }
}

/// Builds a well-formed handle for `chain_id` with a random prefix.
fn rand_handle(chain_id: u64) -> B256 {
    let mut bytes: [u8; 32] = rand::random();
    bytes[22..30].copy_from_slice(&chain_id.to_be_bytes());
    bytes[30] = EUINT64;
    bytes[31] = 0;
    B256::from(bytes)
}

fn public_request() -> PublicDecryptionRequest {
    PublicDecryptionRequest {
        ctHandles: vec![rand_handle(CHAIN_ID), rand_handle(CHAIN_ID)],
        extraData: Bytes::from(vec![0x00]),
    }
}

fn user_request() -> UserDecryptionRequest {
    UserDecryptionRequest {
        attestationType: AttestationType::Eip712UnifiedUserDecryptV1.to_string(),
        payload: UserDecryptionPayload {
            handles: vec![HandleEntry {
                handle: rand_handle(CHAIN_ID),
                contractAddress: Address::repeat_byte(0x33),
                ownerAddress: Address::repeat_byte(0x44),
            }],
            userAddress: Address::repeat_byte(0x55),
            publicKey: Bytes::from(vec![0x20; 32]),
            allowedContracts: vec![Address::repeat_byte(0x33)],
            requestValidity: RequestValidity {
                startTimestamp: Utc::now().timestamp() as u64 - 60,
                durationSeconds: 300,
            },
            extraData: Bytes::from(vec![0x00]),
        },
        signature: Bytes::from(vec![0x66; 65]),
    }
}

/// The `U256` the worker-side helpers use for an interface id.
fn db_id(id: B256) -> U256 {
    U256::from_be_bytes(id.0)
}

/// Plays the kms-worker: stores a completed public decryption for `id`.
async fn complete_public(
    db: &Pool<Postgres>,
    id: B256,
) -> anyhow::Result<connector_utils::types::PublicDecryptionResponse> {
    insert_rand_public_decrypt_response(
        db,
        Some(db_id(id)),
        Some(OperationStatus::Completed),
        RequestSource::Http,
    )
    .await
}

/// Plays the kms-worker: stores a completed user decryption for `id`.
async fn complete_udec(
    db: &Pool<Postgres>,
    id: B256,
) -> anyhow::Result<connector_utils::types::UserDecryptionResponse> {
    insert_rand_user_decrypt_response(
        db,
        Some(db_id(id)),
        Some(OperationStatus::Completed),
        RequestSource::Http,
    )
    .await
}

/// Plays the kms-worker: stores (or overrides) a public decryption error for `id`.
async fn fail_public(
    db: &Pool<Postgres>,
    id: B256,
    code: &str,
    details: &str,
) -> anyhow::Result<()> {
    insert_test_public_decrypt_error_response(db, Some(db_id(id)), code, details).await?;
    Ok(())
}

/// Plays the kms-worker: stores (or overrides) a user decryption error for `id`.
async fn fail_user(db: &Pool<Postgres>, id: B256, code: &str, details: &str) -> anyhow::Result<()> {
    insert_test_user_decrypt_error_response(db, Some(db_id(id)), code, details).await?;
    Ok(())
}

/// Plays a successful worker retry, replacing an error payload in place.
async fn replace_error_with_success(
    db: &Pool<Postgres>,
    table: &str,
    payload_column: &str,
    id: B256,
) -> anyhow::Result<()> {
    let updated = sqlx::query(&format!(
        "UPDATE {table} SET {payload_column} = $1, signature = $1, error_code = NULL,
        error_details = NULL, created_at = $2 WHERE decryption_id = $3"
    ))
    .bind(vec![0x42_u8; 65])
    .bind(Utc::now())
    .bind(db_id(id).as_le_slice().to_vec())
    .execute(db)
    .await?;
    assert_eq!(updated.rows_affected(), 1);
    Ok(())
}

/// Ages a result without changing its identity or notifying waiters.
async fn age_response(
    db: &Pool<Postgres>,
    table: &str,
    id: B256,
    age: Duration,
) -> anyhow::Result<()> {
    let updated = sqlx::query(&format!(
        "UPDATE {table} SET created_at = $1 WHERE decryption_id = $2"
    ))
    .bind(Utc::now() - age)
    .bind(db_id(id).as_le_slice())
    .execute(db)
    .await?;
    assert_eq!(updated.rows_affected(), 1);
    Ok(())
}

/// Options for a `failed` HTTP-sourced request row with `id`, as left by a kms-worker rejection.
fn failed_http_request(id: B256) -> InsertRequestOptions {
    InsertRequestOptions::new()
        .with_id(db_id(id))
        .with_status(OperationStatus::Failed)
        .with_source(RequestSource::Http)
}

async fn count_rows(db: &Pool<Postgres>, table: &str) -> i64 {
    sqlx::query(&format!("SELECT COUNT(*) AS n FROM {table}"))
        .fetch_one(db)
        .await
        .unwrap()
        .get::<i64, _>("n")
}

async fn wait_for_request_row(db: &Pool<Postgres>, table: &str, id: B256) -> sqlx::postgres::PgRow {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        let row = sqlx::query(&format!("SELECT * FROM {table} WHERE decryption_id = $1"))
            .bind(db_id(id).as_le_slice().to_vec())
            .fetch_optional(db)
            .await
            .unwrap();
        if let Some(row) = row {
            return row;
        }
        assert!(Instant::now() < deadline, "request row never appeared");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// The request row's status, `None` if the row does not exist (yet).
async fn request_status(db: &Pool<Postgres>, table: &str, id: B256) -> Option<String> {
    sqlx::query(&format!(
        "SELECT status::TEXT AS status FROM {table} WHERE decryption_id = $1"
    ))
    .bind(db_id(id).as_le_slice().to_vec())
    .fetch_optional(db)
    .await
    .unwrap()
    .map(|row| row.get::<String, _>("status"))
}

async fn wait_for_request_status(db: &Pool<Postgres>, table: &str, id: B256, expected: &str) {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if request_status(db, table, id).await.as_deref() == Some(expected) {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "request never reached `{expected}`"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

async fn error_body(response: Response) -> ErrorResponse {
    response.json::<ErrorResponse>().await.unwrap()
}

/// Asserts the status, code, `retryable` flag and id of an error response, and returns its body
/// for further checks.
async fn assert_error(
    response: Response,
    status: StatusCode,
    code: ErrorCode,
    id: B256,
) -> ErrorResponse {
    assert_eq!(response.status(), status);
    let route = connector_utils::monitoring::http::route_label(response.url().path());
    let body = error_body(response).await;
    assert!(
        metrics::HTTP_METRICS
            .requests
            .with_label_values(&[route, &status.as_u16().to_string(), code.as_str()])
            .get()
            > 0,
        "missing metric for {code:?}"
    );
    assert_eq!(body.code, code);
    assert_eq!(body.retryable, code.retryable());
    assert_eq!(body.decryption_id, Some(id));
    body
}

async fn wait_for_counter(value: impl Fn() -> u64, expected: u64) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while value() < expected {
        assert!(
            Instant::now() < deadline,
            "counter did not reach {expected}"
        );
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    assert_eq!(value(), expected);
}

// Check that we see `expected` requests in the metrics within a 5s window.
async fn assert_requests(route: &str, status: &str, code: &str, expected: u64) {
    wait_for_counter(
        || {
            metrics::HTTP_METRICS
                .requests
                .with_label_values(&[route, status, code])
                .get()
        },
        expected,
    )
    .await;
}

fn admitted_requests() -> u64 {
    let family = prometheus::gather()
        .into_iter()
        .find(|family| family.name() == "kms_connector_endpoint_decryptions_in_flight")
        .unwrap();
    family.get_metric()[0].get_gauge().value() as u64
}

fn lookups(route: &str, outcome: &str) -> u64 {
    metrics::DECRYPTION_LOOKUP_COUNTER
        .with_label_values(&[route, outcome])
        .get()
}
