use crate::{
    error::{PrfError, NO_STORE},
    metrics::REQUESTS_COUNTER,
    pad_source::mock_pad,
    path,
};
use actix_web::{
    dev::Server,
    guard,
    http::{
        header::{ContentType, ALLOW, CACHE_CONTROL},
        Method,
    },
    web, App, HttpRequest, HttpResponse, HttpServer, ResponseError,
};
use alloy::primitives::Address;
use anyhow::anyhow;
use ciphertext_attestation::{
    keccak_b256,
    prf::{Version, PRF_ATTESTATION_HEADER},
    PrfOutputAttestation,
};
use std::{net::SocketAddr, time::Duration};
use tokio_util::sync::CancellationToken;
use tracing::debug;

/// Requests finish in milliseconds, so a stop waits only a short time for idle connections.
const SHUTDOWN_TIMEOUT_SECS: u64 = 5;
/// Above the 10 s idle time of the proxy pool, so the proxy closes idle connections first.
const KEEP_ALIVE: Duration = Duration::from_secs(15);

fn configure(cfg: &mut web::ServiceConfig) {
    // For the endpoint probe of coprocessor-proxy, which checks the port that it forwards to.
    cfg.route(
        "/healthz",
        web::route()
            .guard(guard::Any(guard::Get()).or(guard::Head()))
            .to(HttpResponse::Ok),
    )
    .default_service(web::to(prf_output));
}

pub(crate) fn run_server(
    endpoint: SocketAddr,
    cancel_token: CancellationToken,
) -> anyhow::Result<Server> {
    HttpServer::new(|| App::new().configure(configure))
        .shutdown_signal(cancel_token.cancelled_owned())
        .shutdown_timeout(SHUTDOWN_TIMEOUT_SECS)
        .keep_alive(KEEP_ALIVE)
        // Treat a client EOF as a disconnect, to free the place of the request in `InFlight`.
        .h1_allow_half_closed(false)
        .bind(endpoint)
        .map_err(|e| anyhow!("Failed to bind HTTP server to {endpoint}: {e}"))
        .map(|server| server.run())
}

async fn prf_output(req: HttpRequest) -> HttpResponse {
    // Before the metric, so its method label takes only GET and HEAD.
    if !matches!(*req.method(), Method::GET | Method::HEAD) {
        return HttpResponse::MethodNotAllowed()
            .insert_header((ALLOW, "GET, HEAD"))
            .insert_header((CACHE_CONTROL, NO_STORE))
            .finish();
    }
    let response = serve(&req).unwrap_or_else(|e| e.error_response());
    REQUESTS_COUNTER
        .with_label_values(&[req.method().as_str(), response.status().as_str()])
        .inc();
    response
}

fn serve(req: &HttpRequest) -> Result<HttpResponse, PrfError> {
    // The proxy also refuses a query string. This check is for a request that did not pass the proxy.
    if req.uri().query().is_some() {
        return Err(PrfError::NonCanonicalPath);
    }
    let path = path::parse(req.uri().path()).ok_or(PrfError::NonCanonicalPath)?;
    let block_bits = path::block_bits(path.prf).ok_or(PrfError::UnknownPrf)?;
    debug!(prf = path.prf, label = %path.label, pad_length = path.pad_length, "PRF output request");

    let pad = mock_pad(&path, block_bits);
    // TODO: send the attestation that the pad-worker signed. This mock has no signature.
    let attestation = PrfOutputAttestation {
        version: Version::V1,
        ciphertext_digest: keccak_b256(&pad.ct64),
        sns_ciphertext_digest: keccak_b256(&pad.ct128),
        signer: Address::ZERO,
        signature: vec![0; 65],
    };
    let attestation = serde_json::to_string(&attestation).expect("attestation serializes");

    Ok(HttpResponse::Ok()
        .content_type(ContentType::octet_stream())
        .insert_header((PRF_ATTESTATION_HEADER, attestation))
        .body(pad.ct128))
}
