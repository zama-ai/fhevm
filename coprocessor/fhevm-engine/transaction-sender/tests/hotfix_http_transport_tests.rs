//! HTTP-only production client policy, without Anvil/database prerequisites.
use alloy::{
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
};
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde_json::{json, Value};
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::Notify;
use transaction_sender::{gateway_http_client, NonceManagedProvider};

#[test]
fn websocket_urls_are_rejected_without_rewriting_them() {
    for url in ["ws://localhost:8545", "wss://example.invalid/rpc"] {
        assert!(gateway_http_client(&url.parse().unwrap()).is_err());
    }
    for url in ["http://localhost:8545", "https://example.invalid/rpc"] {
        assert!(gateway_http_client(&url.parse().unwrap()).is_ok());
    }
}

#[tokio::test]
async fn http_errors_are_not_retried_inside_submission() -> anyhow::Result<()> {
    let calls = Arc::new(AtomicUsize::new(0));
    let app = Router::new()
        .route(
            "/",
            post(|State(calls): State<Arc<AtomicUsize>>| async move {
                calls.fetch_add(1, Ordering::SeqCst);
                StatusCode::SERVICE_UNAVAILABLE
            }),
        )
        .with_state(calls.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let url = format!("http://{}", listener.local_addr()?).parse()?;
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let provider: alloy::providers::RootProvider = ProviderBuilder::default().connect_reqwest(gateway_http_client(&url)?, url);
    let result = tokio::time::timeout(
        Duration::from_secs(2),
        provider.raw_request::<_, Value>("eth_sendRawTransaction".into(), ["0x00"]),
    )
    .await;
    server.abort();
    let _ = server.await;
    assert!(result?.is_err());
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}

#[tokio::test]
async fn abandoned_http_estimates_are_not_replayed_after_recovery() -> anyhow::Result<()> {
    let calls = Arc::new(AtomicUsize::new(0));
    let release = Arc::new(Notify::new());
    let state = (calls.clone(), release.clone());
    let app = Router::new()
        .route(
            "/",
            post(
                |State((calls, release)): State<(Arc<AtomicUsize>, Arc<Notify>)>,
                 Json(req): Json<Value>| async move {
                    assert_eq!(req["method"], "eth_estimateGas");
                    let index = calls.fetch_add(1, Ordering::SeqCst);
                    if index < 4 {
                        release.notified().await;
                    }
                    Json(json!({"jsonrpc":"2.0", "id":req["id"], "result":"0x5208"}))
                },
            ),
        )
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let url = format!("http://{}", listener.local_addr()?).parse()?;
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let result = tokio::time::timeout(Duration::from_secs(10), async {
        let rpc = ProviderBuilder::default().connect_reqwest(gateway_http_client(&url)?, url);
        let provider = NonceManagedProvider::new(rpc, None).with_max_inflight(1);
        for _ in 0..4 {
            let error = provider
                .send_sync_with_overprovision(
                    TransactionRequest::default(),
                    300,
                    Duration::from_secs(1),
                    Duration::from_millis(150),
                    Duration::from_secs(1),
                )
                .await
                .unwrap_err();
            assert!(
                error.to_string().contains("gas estimation timeout"),
                "{error}"
            );
        }
        assert_eq!(calls.load(Ordering::SeqCst), 4);
        release.notify_waiters();
        let fresh = provider
            .overprovision_gas_limit(TransactionRequest::default(), 300)
            .await?;
        assert_eq!(fresh.gas, Some(63_000));
        tokio::time::sleep(Duration::from_millis(300)).await;
        assert_eq!(calls.load(Ordering::SeqCst), 5);
        anyhow::Ok(())
    })
    .await;
    release.notify_waiters();
    server.abort();
    let _ = server.await;
    result??;
    Ok(())
}
