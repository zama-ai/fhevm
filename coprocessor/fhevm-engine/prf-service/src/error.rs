use actix_web::{
    http::{header::CACHE_CONTROL, StatusCode},
    HttpResponse, ResponseError,
};
use serde::Serialize;

pub(crate) const NO_STORE: &str = "no-store";

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
    fn parts(&self) -> (StatusCode, &'static str) {
        match self {
            Self::NonCanonicalPath => (StatusCode::BAD_REQUEST, "non_canonical_path"),
            Self::UnknownPrf => (StatusCode::NOT_FOUND, "unknown_prf"),
        }
    }
}

impl ResponseError for PrfError {
    fn status_code(&self) -> StatusCode {
        self.parts().0
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            // Only a `200` answer can be kept in a cache.
            .insert_header((CACHE_CONTROL, NO_STORE))
            .json(ErrorBody {
                code: self.parts().1,
                message: self.to_string(),
                retryable: false,
            })
    }
}
