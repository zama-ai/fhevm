//! RFC 033 Relayer ↔ KMS Connector `v1` HTTP interface.
//!
//! This crate is the shared definition of the direct HTTP decryption interface between clients
//! (relayers for example) and the KMS Connector: the request/response DTOs ([`types`]), the
//! error contract ([`error`]), the route/version constants ([`version`]).

pub mod error;
pub mod types;
pub mod version;

pub use error::{ErrorCode, ErrorResponse};
pub use types::{
    HandleEntry, PublicDecryptionRequest, PublicDecryptionResponse, RequestValidity,
    UserDecryptionRequest, UserDecryptionResponse,
};
pub use version::{
    INTERFACE_VERSION, PUBLIC_DECRYPTION_ROUTE, USER_DECRYPTION_ROUTE, VERSION_ROUTE,
    VersionResponse,
};
