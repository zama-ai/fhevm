pub fn setup_tracing() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let otlp_exporter = opentelemetry_otlp::new_exporter().tonic();

    let trace_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    opentelemetry::global::set_tracer_provider(trace_provider);

    Ok(())
}
