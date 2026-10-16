//! The public HTTPS entry point of a coprocessor: an [`fhevm_sig_proxy`] deployment. It checks
//! each request's `FhevmSig` against the KMS tx-senders `ProtocolConfig` lists, forwards an
//! accepted request to the endpoints of its route, and returns their answer unchanged.

pub mod config;
mod error;
pub mod health;
