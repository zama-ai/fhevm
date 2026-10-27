//! Runs in its own test binary: tracing caches whether a callsite is enabled process-wide, so
//! tests calling RPC nodes from other threads would interfere with the capture of the traces.

use alloy::{providers::Provider, transports::http::reqwest::Url};
use connector_utils::{conn::connect_to_rpc_node, monitoring::otlp::log_filters};
use std::sync::{Arc, Mutex};
use tracing_subscriber::{EnvFilter, Layer, Registry, fmt::format::FmtSpan, layer::SubscriberExt};

/// An API key, put in the URL path as some RPC providers require.
const API_KEY: &str = "secret-api-key";

#[tokio::test]
async fn traces_do_not_log_the_api_key() {
    // Without the app's filters, alloy's request span shows the URL: this guards the test.
    let unfiltered = traces_of_a_refused_call(EnvFilter::new("trace").boxed()).await;
    assert!(unfiltered.contains(API_KEY), "{unfiltered}");

    let filtered = traces_of_a_refused_call(log_filters(EnvFilter::new("trace")).boxed()).await;
    assert!(!filtered.contains(API_KEY), "{filtered}");
    // The request's INFO span is still there.
    assert!(filtered.contains("eth_blockNumber"), "{filtered}");
}

/// Returns the traces of a call refused by the RPC node, with an API key in its URL, as printed
/// with `filters`, spans included.
async fn traces_of_a_refused_call(filters: Box<dyn Layer<Registry> + Send + Sync>) -> String {
    let traces = CapturedTraces::default();
    let subscriber = tracing_subscriber::registry().with(filters).with(
        tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_span_events(FmtSpan::NEW)
            .with_writer({
                let traces = traces.clone();
                move || traces.clone()
            }),
    );
    let _guard = tracing::subscriber::set_default(subscriber);

    // The port of a listener bound then dropped refuses connections.
    let address = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap();
    let url = Url::parse(&format!("http://{address}/v2/{API_KEY}")).unwrap();
    let provider = connect_to_rpc_node(url, 1).await.unwrap();
    provider.get_block_number().await.unwrap_err();

    String::from_utf8(traces.0.lock().unwrap().clone()).unwrap()
}

/// The traces printed by a `fmt` layer.
#[derive(Clone, Default)]
struct CapturedTraces(Arc<Mutex<Vec<u8>>>);

impl std::io::Write for CapturedTraces {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
