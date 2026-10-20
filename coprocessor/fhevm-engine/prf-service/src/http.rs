use crate::{error::PrfError, metrics::REQUESTS_COUNTER, pad_source::mock_pad, path};
use actix_web::{
    dev::Server,
    http::{header::ContentType, Method},
    web, App, HttpRequest, HttpResponse, HttpServer, ResponseError,
};
use alloy::primitives::Address;
use anyhow::anyhow;
use ciphertext_attestation::{
    keccak_b256,
    prf::{Version, PRF_ATTESTATION_HEADER},
    PrfOutputAttestation,
};
use std::net::SocketAddr;
use tokio_util::sync::CancellationToken;
use tracing::debug;

const PRF_ROUTE: &str = "/v1/prf/{prf}/labels/{label}/{pad_length}";
/// Requests finish in milliseconds, so a stop waits only a short time for idle connections.
const SHUTDOWN_TIMEOUT_SECS: u64 = 5;

fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource(PRF_ROUTE)
            .route(web::get().to(prf_output))
            .route(web::head().to(prf_output)),
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
        .bind(endpoint)
        .map_err(|e| anyhow!("Failed to bind HTTP server to {endpoint}: {e}"))
        .map(|server| server.run())
}

async fn prf_output(req: HttpRequest) -> HttpResponse {
    // Other methods get 405 on the PRF route and 404 on other paths.
    if !matches!(*req.method(), Method::GET | Method::HEAD) {
        return HttpResponse::NotFound().finish();
    }
    let response = serve(&req).unwrap_or_else(|e| e.error_response());
    REQUESTS_COUNTER
        .with_label_values(&[req.method().as_str(), response.status().as_str()])
        .inc();
    response
}

fn serve(req: &HttpRequest) -> Result<HttpResponse, PrfError> {
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
