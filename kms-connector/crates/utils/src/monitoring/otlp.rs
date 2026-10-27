use actix_web::HttpResponse;
use anyhow::anyhow;
use opentelemetry::{
    global,
    propagation::{Extractor, Injector},
    trace::TracerProvider,
};
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::{Resource, propagation::TraceContextPropagator, trace::SdkTracerProvider};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{Dispatch, Level, Metadata, Subscriber};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{
    EnvFilter, Layer,
    filter::{FilterFn, filter_fn},
    fmt,
    layer::SubscriberExt,
    registry::LookupSpan,
    util::SubscriberInitExt,
};

/// Returns a default tracing dispatcher.
///
/// Used temporarily during config parsing as OTLP setup needs the service name that is in the
/// config.
pub fn default_dispatcher() -> Dispatch {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(log_filters(env_filter()))
        .into()
}

/// Configures the tracing, OpenTelemetry and Prometheus setup for the app.
///
/// - An `actix_web::HttpServer` is started to expose Prometheus metrics for collection.
/// - Opentelemetry is configured to export traces to `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`.
pub fn init_otlp_setup(service_name: String) -> anyhow::Result<()> {
    let service_name_resource = Resource::builder().with_service_name(service_name).build();
    let span_exporter = SpanExporter::builder()
        .with_tonic()
        .build()
        .map_err(|e| anyhow!("Failed to create span exporter: {e}"))?;
    let trace_provider = SdkTracerProvider::builder()
        .with_resource(service_name_resource)
        .with_batch_exporter(span_exporter)
        .build();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(log_filters(env_filter()))
        .with(OpenTelemetryLayer::new(trace_provider.tracer("otlp-layer")))
        .init();

    global::set_text_map_propagator(TraceContextPropagator::new());
    global::set_tracer_provider(trace_provider);

    Ok(())
}

/// The log filter set by `RUST_LOG`, defaulting to `info`.
fn env_filter() -> EnvFilter {
    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
}

/// The filters of every tracing subscriber of the app: `env_filter`, which `hide_rpc_urls`
/// overrides.
pub fn log_filters<S>(env_filter: EnvFilter) -> impl Layer<S>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    env_filter.and_then(hide_rpc_urls())
}

/// Drops the debug and trace spans and events of `alloy_transport_http`, whatever `RUST_LOG` says:
/// its request spans hold the full RPC node URL, which may contain an API key.
fn hide_rpc_urls() -> FilterFn<impl Fn(&Metadata<'_>) -> bool> {
    filter_fn(|metadata| {
        !(metadata.target().starts_with("alloy_transport_http") && *metadata.level() > Level::INFO)
    })
}

/// The content type header for Prometheus text-based metrics format.
///
/// For more details, check https://prometheus.io/docs/instrumenting/exposition_formats/#text-based-format
const PROMETHEUS_TEXT_FORMAT_HEADER: (&str, &str) = ("Content-Type", prometheus::TEXT_FORMAT);

/// Responder used to collect metrics of the service.
pub async fn metrics_responder() -> impl actix_web::Responder {
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    match encoder.encode_to_string(&metric_families) {
        Ok(encoded_metrics) => HttpResponse::Ok()
            .insert_header(PROMETHEUS_TEXT_FORMAT_HEADER)
            .body(encoded_metrics),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PropagationContext(HashMap<String, String>);

impl Default for PropagationContext {
    fn default() -> Self {
        Self::empty()
    }
}

impl PropagationContext {
    pub fn empty() -> Self {
        Self(HashMap::new())
    }

    pub fn inject(context: &opentelemetry::Context) -> Self {
        global::get_text_map_propagator(|propagator| {
            let mut propagation_context = PropagationContext::empty();
            propagator.inject_context(context, &mut propagation_context);
            propagation_context
        })
    }

    pub fn extract(&self) -> opentelemetry::Context {
        global::get_text_map_propagator(|propagator| propagator.extract(self))
    }
}

impl Injector for PropagationContext {
    fn set(&mut self, key: &str, value: String) {
        self.0.insert(key.to_string(), value);
    }
}

impl Extractor for PropagationContext {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|v| v.as_ref())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|k| k.as_ref()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test that the default value for otlp_context inserted in database corresponds to an empty
    // `PropagationContext`.
    fn test_propagation_context() {
        let context: PropagationContext =
            bc2wrap::deserialize_slice(&alloy::hex::decode("0000000000000000").unwrap()).unwrap();
        assert_eq!(context, PropagationContext::empty());
    }
}
