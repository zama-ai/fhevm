use actix_web::{
    http::{
        header::{CACHE_CONTROL, RETRY_AFTER, WWW_AUTHENTICATE},
        StatusCode,
    },
    HttpResponse, ResponseError,
};
use coprocessor_api::{ErrorCode, ErrorResponse};

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

impl PrfError {
    fn code(&self) -> ErrorCode {
        match self {
            Self::MissingSigner => ErrorCode::AuthMissing,
            Self::MalformedSigner => ErrorCode::AuthMalformed,
            Self::NonCanonicalPath => ErrorCode::NonCanonicalPath,
            Self::UnknownPrf => ErrorCode::UnknownPrf,
            Self::ConcurrencyLimited => ErrorCode::ConcurrencyLimited,
            Self::EvaluationFailed => ErrorCode::PrfEvaluationFailed,
            Self::Unavailable => ErrorCode::Unavailable,
        }
    }
}

impl ResponseError for PrfError {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.code().http_status()).expect("error codes map to valid statuses")
    }

    fn error_response(&self) -> HttpResponse {
        let code = self.code();
        let mut response = HttpResponse::build(self.status_code());
        // Only a `200` answer can be kept in a cache.
        response.insert_header((CACHE_CONTROL, NO_STORE));
        match self {
            // A `HEAD` answer has no body, so a `401` names its code in the challenge.
            Self::MissingSigner | Self::MalformedSigner => {
                let challenge = format!("{} error=\"{}\"", fhevm_sig_auth::SCHEME, code.as_str());
                response.insert_header((WWW_AUTHENTICATE, challenge));
            }
            Self::ConcurrencyLimited => {
                response.insert_header((RETRY_AFTER, "1"));
            }
            Self::NonCanonicalPath
            | Self::UnknownPrf
            | Self::EvaluationFailed
            | Self::Unavailable => {}
        }
        response.json(ErrorResponse::new(code, self.to_string()))
    }
}
