//! The coprocessor's HTTP interface, shared by coprocessor-proxy, prf-service and their clients:
//! the PRF output route and the error answers, as the
//! [PRF endpoint spec](https://github.com/zama-ai/tech-spec/blob/main/rfcs/038-coprocessor-prf-output-endpoint.md)
//! defines them. What a coprocessor signs, and how a client verifies it, lives in
//! [`ciphertext_attestation`].
//!
//! A new interface version is a new route. `v1` can gain error codes, which older clients read as
//! [`ErrorCode::Unknown`], but no new fields.

pub mod error;
pub mod prf;

pub use error::{ErrorCode, ErrorResponse};
pub use prf::{
    parse_prf_output_path, prf_output_path, MAX_PAD_LENGTH, PRF_ATTESTATION_HEADER,
    PRF_OUTPUT_ROUTE,
};
