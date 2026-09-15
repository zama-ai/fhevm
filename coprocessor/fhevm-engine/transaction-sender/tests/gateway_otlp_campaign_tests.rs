use alloy::providers::{Provider, ProviderBuilder};
use fhevm_engine_common::telemetry;
use transaction_sender::{
    diagnostics::{gateway_tracing_filter, safe_rpc_error},
    gateway_http_client,
};

/// The Python campaign runner captures actual gRPC export payloads and JSON
/// stdout, requires the positive marker, and rejects every synthetic secret.
#[tokio::test]
#[ignore = "requires campaign OTLP receiver"]
async fn actual_json_and_otlp_exports_exclude_gateway_credentials() -> anyhow::Result<()> {
    let guard = telemetry::init_json_subscriber_with_filter(
        tracing::Level::TRACE,
        "gateway-privacy-campaign",
        "campaign",
        gateway_tracing_filter,
    )
    .map_err(|_| anyhow::anyhow!("tracing setup failed"))?;
    let socket = std::net::TcpListener::bind("127.0.0.1:0")?;
    let port = socket.local_addr()?.port();
    drop(socket);
    let url = format!("http://GW_USER_SECRET:GW_PASSWORD_SECRET@127.0.0.1:{port}/GW_PATH_SECRET?key=GW_QUERY_SECRET").parse()?;
    let provider = ProviderBuilder::new().connect_reqwest(gateway_http_client(&url)?, url);
    {
        let span = tracing::info_span!("campaign_export_positive_marker");
        let _entered = span.enter();
        tracing::trace!(target: "alloy_rpc_client", "GW_ALLOY_SECRET");
        tracing::trace!(target: "rustls::client::hs", "GW_TLS_SECRET");
        let err = provider.get_chain_id().await.unwrap_err();
        tracing::warn!(error = %safe_rpc_error(&err), "campaign_export_positive_marker");
    }
    // Exporter shutdown is blocking, so avoid blocking this Tokio runtime.
    tokio::task::spawn_blocking(move || drop(guard)).await?;
    Ok(())
}
