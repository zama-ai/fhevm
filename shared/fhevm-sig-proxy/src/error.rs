//! The errors the proxy answers itself. Each deployment renders them in the error body of its own
//! interface, through the renderer given to [`crate::Proxy::from_config`].

use fhevm_sig_auth::AuthError;
use http::{Method, StatusCode};
use std::fmt::Display;

/// Why the proxy answers a request itself instead of forwarding it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProxyError {
    /// The request target is not an origin-form UTF-8 path.
    NotOriginForm,
    /// No route has the request path.
    NotFound,
    /// A route has the request path but not its method; the proxy sends `Allow` with the methods.
    MethodNotAllowed(Vec<Method>),
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
            Self::NotFound => f.write_str("no route for this path"),
            Self::MethodNotAllowed(_) => f.write_str("method not allowed on this path"),
            Self::BodyTooLarge => f.write_str("request body too large"),
            Self::Auth(error) => error.fmt(f),
            Self::EndpointUnavailable => f.write_str("endpoint unavailable"),
            Self::EndpointTimeout => f.write_str("endpoint timed out"),
        }
    }
}

impl std::error::Error for ProxyError {}

/// A [`ProxyError`] rendered by a deployment: the status, and the JSON body. An empty `json` is
/// answered with no body and no `Content-Type`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorBody {
    pub status: StatusCode,
    pub json: Vec<u8>,
}
