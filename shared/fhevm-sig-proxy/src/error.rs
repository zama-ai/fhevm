//! The errors the proxy answers itself. Each deployment renders them in the error body of its own
//! interface, through [`crate::ProxyConfig::error_body`].

use fhevm_sig_auth::AuthError;
use http::StatusCode;
use std::fmt::Display;

/// Why the proxy answers a request itself instead of forwarding it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProxyError {
    /// The request target is not an origin-form UTF-8 path.
    NotOriginForm,
    /// The request body exceeds the route's cap.
    BodyTooLarge,
    /// The `FhevmSig` header failed a check.
    Auth(AuthError),
    /// No endpoint could take the request.
    EndpointUnavailable,
    /// The endpoint did not answer in time.
    EndpointTimeout,
}

impl Display for ProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotOriginForm => f.write_str("request target must be an origin-form UTF-8 path"),
            Self::BodyTooLarge => f.write_str("request body too large"),
            Self::Auth(error) => error.fmt(f),
            Self::EndpointUnavailable => f.write_str("endpoint unavailable"),
            Self::EndpointTimeout => f.write_str("endpoint timed out"),
        }
    }
}

impl std::error::Error for ProxyError {}

/// A [`ProxyError`] rendered by a deployment: the status, and the JSON body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorBody {
    pub status: StatusCode,
    pub json: Vec<u8>,
}
