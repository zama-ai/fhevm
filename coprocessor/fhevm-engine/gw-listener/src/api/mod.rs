//! V2 API module for Gateway Listener
//!
//! This module implements the HTTP API endpoints for Gateway V2:
//! - POST /v1/verify-input: Receive input verification payload
//! - GET /v1/ciphertext/{handle}: Retrieve ciphertext material
//! - GET /v1/health: Health check for load balancers
//!
//! See WORKER_API_SPEC.md for complete endpoint definitions.

pub mod handlers;
pub mod signing;
pub mod types;

#[cfg(test)]
mod tests;
