//! An HTTPS reverse proxy on Pingora that authenticates each request's `FhevmSig` (RFC-038)
//! header before forwarding it to an endpoint, and passes the endpoint's answer back unchanged.
//!
//! The deployments (the KMS Connector, the coprocessor) are thin binaries: each loads its own
//! configuration into a [`ProxyConfig`], runs the [`Proxy`] and serves its [`health::State`].

mod config;
#[cfg(feature = "conformance")]
pub mod conformance;
pub mod defaults;
mod error;
pub mod health;
mod http_proxy;
mod proxy;
mod routing;

pub use config::{ProxyConfig, SignerSource, TlsConfig, Upstream};
pub use error::{ErrorBody, ProxyError};
pub use proxy::Proxy;
pub use routing::Route;
