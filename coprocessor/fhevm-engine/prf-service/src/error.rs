use actix_web::{
    http::{
        header::{CACHE_CONTROL, RETRY_AFTER, WWW_AUTHENTICATE},
        StatusCode,
    },
    HttpResponse, ResponseError,
};
use serde::Serialize;

pub(crate) const NO_STORE: &str = "no-store";

#[derive(Debug, thiserror::Error)]
pub enum PrfError {
    #[error("no signer from the proxy")]
    MissingSigner,
    #[error("malformed signer from the proxy")]
    MalformedSigner,
    #[error("non-canonical or out-of-range path parameter")]
    NonCanonicalPath,
    #[error("prf is not a supported instance")]
    UnknownPrf,
    #[error("per-connector limit reached")]
    ConcurrencyLimited,
    #[error("PRF evaluation failed")]
    EvaluationFailed,
    #[error("temporarily unable to serve")]
    Unavailable,
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
            Self::MissingSigner => (StatusCode::UNAUTHORIZED, "auth_missing", false),
            Self::MalformedSigner => (StatusCode::UNAUTHORIZED, "auth_malformed", false),
            Self::NonCanonicalPath => (StatusCode::BAD_REQUEST, "non_canonical_path", false),
            Self::UnknownPrf => (StatusCode::NOT_FOUND, "unknown_prf", false),
            Self::ConcurrencyLimited => {
                (StatusCode::TOO_MANY_REQUESTS, "concurrency_limited", true)
            }
            Self::EvaluationFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "prf_evaluation_failed",
                false,
            ),
            Self::Unavailable => (StatusCode::SERVICE_UNAVAILABLE, "unavailable", true),
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
        match self {
            // A `HEAD` answer has no body, so a `401` names its code in the challenge.
            Self::MissingSigner | Self::MalformedSigner => {
                response.insert_header((WWW_AUTHENTICATE, format!("FhevmSig error=\"{code}\"")));
            }
            Self::ConcurrencyLimited => {
                response.insert_header((RETRY_AFTER, "1"));
            }
            Self::NonCanonicalPath
            | Self::UnknownPrf
            | Self::EvaluationFailed
            | Self::Unavailable => {}
        }
        response.json(ErrorBody {
            code,
            message: self.to_string(),
            retryable,
        })
    }
}
