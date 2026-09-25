//! The Zama fhevm Solana user-decryption request, for every Rust consumer.
//!
//! The typed request the authorizers read, the one function that builds it from the two carriers
//! the Gateway uses, the single encoder/decoder pair for the opaque blob, and the host chain-id
//! rules a request's handles are checked against.
//!
//! The relayer builds the bytes because it submits the Gateway transaction; each KMS party's
//! connector reads them because it authorizes. Both call this crate, so the layout and the form
//! rules have one definition.
//!
//! What this crate checks is only the request's form: a handle count within the cap, one entry
//! per handle, one Solana chain, and a strictly decoded permit. It does not verify a signature or
//! read a clock or an account: every rule about live state belongs to the consumer that owns it.

/// Building the typed request from the Gateway fields and the blob.
pub mod assemble;
/// The canonical byte layout of the blob: version byte and borsh body.
pub mod codec;
/// The kind of host chain a chain id names.
pub mod host_chain;
/// The typed request.
pub mod request;

pub use assemble::{
    public_request_chain_id, SolanaEntryClaims, SolanaRequestBlob, SolanaRequestError,
    SolanaUserDecryptFields,
};
pub use codec::{
    decode_solana_request, encode_solana_request, SolanaRequestDecodeError,
    SolanaRequestEncodeError, SOLANA_REQUEST_VERSION,
};
pub use request::{HandleEntry, SolanaUserDecryptRequest, MAX_REQUEST_HANDLES};
