use actix_web::{
    http::{
        header::{CACHE_CONTROL, RETRY_AFTER},
        StatusCode,
    },
    HttpResponse, ResponseError,
};
use serde::Serialize;
use std::time::Duration;

pub(crate) const NO_STORE: &str = "no-store";

#[derive(Debug, thiserror::Error)]
pub enum PrfError {
    #[error("non-canonical or out-of-range path parameter")]
    NonCanonicalPath,
    #[error("prf is not a supported instance")]
    UnknownPrf,
    /// The proxy forwards only requests it authenticated, so this is a deployment error.
    #[error("no connector can be recovered from the FhevmSig header: {0}")]
    Unauthenticated(fhevm_sig_auth::AuthError),
    #[error("evaluation cap of this connector reached")]
    ConcurrencyLimited { retry_after: Duration },
    /// The pad-worker gave up on this output. A new request would read the same failed row.
    #[error("PRF evaluation failed")]
    EvaluationFailed,
    #[error("PRF output storage unavailable")]
    Unavailable(#[from] anyhow::Error),
}

/// Error body: the same shape as the kms-connector endpoint errors.
#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
    retryable: bool,
}

impl PrfError {
    fn parts(&self) -> (StatusCode, &'static str, bool) {
        match self {
            Self::NonCanonicalPath => (StatusCode::BAD_REQUEST, "non_canonical_path", false),
            Self::UnknownPrf => (StatusCode::NOT_FOUND, "unknown_prf", false),
            Self::Unauthenticated(e) => (
                StatusCode::UNAUTHORIZED,
                e.code(),
                matches!(e, fhevm_sig_auth::AuthError::Expired),
            ),
            Self::ConcurrencyLimited { .. } => {
                (StatusCode::TOO_MANY_REQUESTS, "concurrency_limited", true)
            }
            Self::EvaluationFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "prf_evaluation_failed",
                false,
            ),
            Self::Unavailable(_) => (StatusCode::SERVICE_UNAVAILABLE, "unavailable", true),
        }
    }
}

impl ResponseError for PrfError {
    fn status_code(&self) -> StatusCode {
        self.parts().0
    }

    fn error_response(&self) -> HttpResponse {
        let (status, code, retryable) = self.parts();
        let mut response = HttpResponse::build(status);
        // Only a `200` answer can be kept in a cache.
        response.insert_header((CACHE_CONTROL, NO_STORE));
        if let Self::ConcurrencyLimited { retry_after } = self {
            response.insert_header((RETRY_AFTER, retry_after.as_secs().max(1).to_string()));
        }
        // Keep storage details in the logs.
        let message = match self {
            Self::Unavailable(_) => "PRF output storage unavailable".to_owned(),
            other => other.to_string(),
        };
        response.json(ErrorBody {
            code,
            message,
            retryable,
        })
    }
}
