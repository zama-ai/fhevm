use crate::{
    error::{PrfError, NO_STORE},
    in_flight::InFlight,
    instance,
    metrics::REQUESTS_COUNTER,
    pads::{self, Admission, Admissions, PadState},
    waiter::PadWaiter,
};
use actix_web::{
    body::{BodySize, MessageBody},
    dev::Server,
    http::{
        header::{ContentType, ALLOW, CACHE_CONTROL},
        Method,
    },
    web::{self, Bytes},
    App, HttpRequest, HttpResponse, HttpServer, ResponseError,
};
use alloy::primitives::Address;
use anyhow::anyhow;
use ciphertext_attestation::{prf::PRF_ATTESTATION_HEADER, PrfOutputRef};
use sqlx::PgPool;
use std::{
    convert::Infallible,
    net::SocketAddr,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};

/// The signer that coprocessor-proxy verified with `FhevmSig`. The proxy drops a client copy of it.
pub use fhevm_sig_auth::SIGNER_HEADER;
/// A stop does not wait for held requests: the connector retries, and the retry joins the same row.
const SHUTDOWN_TIMEOUT_SECS: u64 = 5;
/// Above the 10 s idle time of the proxy pool, so the proxy closes idle connections first.
const KEEP_ALIVE: Duration = Duration::from_secs(15);

pub(crate) struct AppState {
    pub pool: PgPool,
    pub in_flight: InFlight,
    pub admissions: Admissions,
    pub waiter: Arc<PadWaiter>,
}

fn configure(cfg: &mut web::ServiceConfig) {
    cfg.default_service(web::to(prf_output));
}

pub(crate) fn run_server(
    endpoint: SocketAddr,
    state: web::Data<AppState>,
    cancel_token: CancellationToken,
) -> anyhow::Result<Server> {
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

async fn prf_output(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    // Before the metric, so its method label takes only GET and HEAD.
    if !matches!(*req.method(), Method::GET | Method::HEAD) {
        return HttpResponse::MethodNotAllowed()
            .insert_header((ALLOW, "GET, HEAD"))
            .insert_header((CACHE_CONTROL, NO_STORE))
            .finish();
    }
    let response = serve(&req, &state)
        .await
        .unwrap_or_else(|e| e.error_response());
    REQUESTS_COUNTER
        .with_label_values(&[req.method().as_str(), response.status().as_str()])
        .inc();
    response
}

async fn serve(req: &HttpRequest, state: &AppState) -> Result<HttpResponse, PrfError> {
    let connector = signer(req)?;
    // The proxy also refuses a query string. This check is for a request that did not pass the proxy.
    if req.uri().query().is_some() {
        return Err(PrfError::NonCanonicalPath);
    }
    let path = PrfOutputRef::from_path(req.uri().path()).ok_or(PrfError::NonCanonicalPath)?;
    instance::block_bits(path.prf_id).ok_or(PrfError::UnknownPrf)?;
    debug!(prf = path.prf_id, label = %path.label, pad_length = path.pad_length, %connector, "PRF output request");

    // Held until the request ends, also when the client disconnects.
    let _in_flight = state
        .in_flight
        .start(connector, &path)
        .ok_or(PrfError::ConcurrencyLimited)?;
    let pad = match state
        .admissions
        .admit(&path, connector)
        .await
        .map_err(unavailable)?
    {
        Admission::OverBudget => return Err(PrfError::ConcurrencyLimited),
        Admission::Created | Admission::Existing(PadState::Pending) => {
            state.waiter.wait(&path).await.map_err(unavailable)?
        }
        Admission::Existing(pad) => pad,
    };
    let output = match pad {
        PadState::Complete(output) => output,
        PadState::Failed => return Err(PrfError::EvaluationFailed),
        // `wait` returns only a complete or failed row.
        PadState::Pending => return Err(PrfError::Unavailable),
    };

    let mut response = HttpResponse::Ok();
    response
        .content_type(ContentType::octet_stream())
        .insert_header((
            PRF_ATTESTATION_HEADER,
            serde_json::to_string(&output.attestation).expect("attestation serializes"),
        ));
    if req.method() == Method::HEAD {
        return Ok(response.body(HeadBody(output.body_length)));
    }
    // No body: the row was deleted after the read.
    let body = pads::read_body(&state.pool, &path)
        .await
        .map_err(unavailable)?
        .ok_or(PrfError::Unavailable)?;
    Ok(response.body(body))
}

/// A `HEAD` body: the `Content-Length` of the `GET` body, and no bytes.
struct HeadBody(u64);

impl MessageBody for HeadBody {
    type Error = Infallible;

    fn size(&self) -> BodySize {
        BodySize::Sized(self.0)
    }

    fn poll_next(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        Poll::Ready(None)
    }
}

/// The connector that pays for the request.
fn signer(req: &HttpRequest) -> Result<Address, PrfError> {
    let value = req
        .headers()
        .get(SIGNER_HEADER)
        .ok_or(PrfError::MissingSigner)?;
    value
        .to_str()
        .ok()
        .and_then(|s| s.parse().ok())
        .ok_or(PrfError::MalformedSigner)
}

fn unavailable(e: anyhow::Error) -> PrfError {
    warn!(error = %e, "database error");
    PrfError::Unavailable
}
