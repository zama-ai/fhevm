//! The Zama fhevm Solana user-decryption request, for every Rust consumer.
//!
//! The typed request the authorizers read, the one function that builds it from the fields of a
//! request body, and the host chain-id rules a request's handles are checked against.
//!
//! The relayer admits a request before it forwards it, and each KMS party's connector authorizes
//! it. Both call this crate, so the form rules have one definition.
//!
//! What this crate checks is only the request's form: a handle count within the cap, one entry
//! per handle, one Solana chain, and a strictly decoded permit. It does not verify a signature or
//! read a clock or an account: every rule about live state belongs to the consumer that owns it.

/// Building the typed request from the fields of a request body.
pub mod assemble;
/// The typed request.
pub mod request;

pub use assemble::{
    public_request_chain_id, SolanaEntryClaims, SolanaRequestBlob, SolanaRequestError,
    SolanaUserDecryptFields,
};
pub use request::{HandleEntry, SolanaUserDecryptRequest, MAX_REQUEST_HANDLES};
