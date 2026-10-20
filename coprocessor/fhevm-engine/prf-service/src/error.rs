use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum PrfError {
    #[error("non-canonical or out-of-range path parameter")]
    NonCanonicalPath,
    #[error("prf is not a supported instance")]
    UnknownPrf,
}

/// Error body: the same shape as the kms-connector endpoint errors.
#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
    retryable: bool,
}

impl PrfError {
    fn code(&self) -> &'static str {
        match self {
            Self::NonCanonicalPath => "non_canonical_path",
            Self::UnknownPrf => "unknown_prf",
        }
    }
}

impl ResponseError for PrfError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NonCanonicalPath => StatusCode::BAD_REQUEST,
            Self::UnknownPrf => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorBody {
            code: self.code(),
            message: self.to_string(),
            retryable: false,
        })
    }
}
