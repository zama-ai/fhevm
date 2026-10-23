use crate::{
    error::{PrfError, NO_STORE},
    metrics::REQUESTS_COUNTER,
    pads::{Admission, PadOutput, PadState},
    path, State,
};
use actix_web::{
    dev::Server,
    guard,
    http::{
        header::{ContentType, ALLOW, AUTHORIZATION, CACHE_CONTROL},
        Method,
    },
    web, App, HttpRequest, HttpResponse, HttpServer, ResponseError,
};
use anyhow::anyhow;
use ciphertext_attestation::prf::PRF_ATTESTATION_HEADER;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio_util::sync::CancellationToken;
use tracing::debug;

/// A stop waits only a short time: a held request is answered again by another pod on retry.
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
    state: Arc<State>,
    cancel_token: CancellationToken,
) -> anyhow::Result<Server> {
    let state = web::Data::from(state);
    HttpServer::new(move || App::new().app_data(state.clone()).configure(configure))
        .shutdown_signal(cancel_token.cancelled_owned())
        .shutdown_timeout(SHUTDOWN_TIMEOUT_SECS)
        .keep_alive(KEEP_ALIVE)
        // Treat a client EOF as a disconnect, to free the place of the request in `InFlight`.
        .h1_allow_half_closed(false)
        .bind(endpoint)
        .map_err(|e| anyhow!("Failed to bind HTTP server to {endpoint}: {e}"))
        .map(|server| server.run())
}

async fn prf_output(req: HttpRequest, state: web::Data<State>) -> HttpResponse {
    // Before the metric, so its method label takes only GET and HEAD.
    if !matches!(*req.method(), Method::GET | Method::HEAD) {
        return HttpResponse::MethodNotAllowed()
            .insert_header((ALLOW, "GET, HEAD"))
            .insert_header((CACHE_CONTROL, NO_STORE))
            .finish();
    }
    let response = match serve(&req, &state).await {
        Ok(output) => HttpResponse::Ok()
            .content_type(ContentType::octet_stream())
            .insert_header((
                PRF_ATTESTATION_HEADER,
                serde_json::to_string(&output.attestation).expect("attestation serializes"),
            ))
            .body(output.ciphertext128),
        Err(e) => {
            if let PrfError::Unavailable(err) = &e {
                tracing::warn!(error = %err, "PRF output storage error");
            }
            e.error_response()
        }
    };
    REQUESTS_COUNTER
        .with_label_values(&[req.method().as_str(), response.status().as_str()])
        .inc();
    response
}

async fn serve(req: &HttpRequest, state: &State) -> Result<PadOutput, PrfError> {
    // The proxy also refuses a query string. This check is for a request that did not pass the proxy.
    if req.uri().query().is_some() {
        return Err(PrfError::NonCanonicalPath);
    }
    let target = req.uri().path_and_query().ok_or(PrfError::NonCanonicalPath)?;
    let path = path::parse(target.path()).ok_or(PrfError::NonCanonicalPath)?;
    path::block_bits(path.prf).ok_or(PrfError::UnknownPrf)?;
    // actix-web is on `http` 0.2, fhevm-sig-auth on `http` 1.
    let signed_target = fhevm_sig_auth::PathAndQuery::try_from(target.as_str())
        .map_err(|_| PrfError::NonCanonicalPath)?;
    let (connector, _) = state
        .auth
        .recover(
            req.headers().get(AUTHORIZATION).map(|v| v.as_bytes()),
            &signed_target,
            fhevm_sig_auth::unix_now(),
        )
        .map_err(PrfError::Unauthenticated)?;
    debug!(prf = path.prf, label = %path.label, pad_length = path.pad_length, %connector, "PRF output request");

    let limited = || PrfError::ConcurrencyLimited {
        retry_after: state.retry_after,
    };
    let _in_flight = state.in_flight.start(connector, &path).ok_or_else(limited)?;
    let state_after_admission = match state.admissions.admit(&path, connector).await? {
        Admission::OverBudget => return Err(limited()),
        Admission::Existing(PadState::Pending) | Admission::Created => {
            state.waiter.wait(&path).await?
        }
        Admission::Existing(done) => done,
    };
    match state_after_admission {
        PadState::Complete(output) => Ok(output),
        PadState::Failed => Err(PrfError::EvaluationFailed),
        PadState::Pending => unreachable!("the waiter returns only a complete or failed row"),
    }
}
