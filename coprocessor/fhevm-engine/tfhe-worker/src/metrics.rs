use tracing::info;

async fn metrics() -> impl actix_web::Responder {
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder
        .encode_to_string(&metric_families)
        .expect("can't encode metrics")
}

async fn healthcheck() -> impl actix_web::Responder {
    "OK"
}

pub async fn run_metrics_server(
    args: crate::daemon_cli::Args,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!(
        metrics_server_address = args.metrics_addr,
        "metrics server starting"
    );
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .route("/metrics", actix_web::web::to(metrics))
            .route("/health", actix_web::web::to(healthcheck))
    })
    .bind(&args.metrics_addr)
    .expect("can't bind to metrics server address")
    .workers(1)
    .run()
    .await?;

    Ok(())
}
