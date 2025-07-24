use actix_web::{HttpResponse, http::StatusCode};
use anyhow::anyhow;
use opentelemetry::{global, trace::TracerProvider};
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::{Resource, propagation::TraceContextPropagator, trace::SdkTracerProvider};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

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
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(OpenTelemetryLayer::new(trace_provider.tracer("otlp-layer")))
        .init();

    global::set_text_map_propagator(TraceContextPropagator::new());
    global::set_tracer_provider(trace_provider);

    Ok(())
}

/// Responder used to collect metrics of the service.
pub async fn metrics_responder() -> impl actix_web::Responder {
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    match encoder.encode_to_string(&metric_families) {
        Ok(encoded_metrics) => HttpResponse::with_body(StatusCode::OK, encoded_metrics),
        Err(e) => HttpResponse::with_body(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
