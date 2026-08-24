//! Canonical transport form of the Zama fhevm Solana user-decryption request.
//!
//! One implementation of the request canon for every Rust consumer: the wire types the
//! sender fills in, and the single encoder/decoder pair for the opaque blob the gateway
//! carries between them.
//!
//! The relayer builds the bytes because it submits the gateway transaction; each KMS party's
//! connector reads them because it authorizes. Both call this crate, so the layout has one
//! definition and a field added to it is a compile error on both sides. The cross-language
//! agreement with the borsh-js encoder in the browser SDK is a separate matter, and stays
//! pinned by the committed vectors — no amount of shared Rust can establish it.
//!
//! What this crate deliberately does not do: validate, verify a signature, read a clock or
//! an account. The wire form is untrusted by construction, and the consumer that authorizes
//! owns its own validated type and every rule about live state.

/// The canonical byte layout: version byte, borsh body, and the event-parity rule.
pub mod codec;
/// The untyped request form as it arrives from a sender.
pub mod wire;

pub use codec::{
    check_handle_list_parity, decode_solana_request, encode_solana_request,
    SolanaRequestDecodeError, SolanaRequestEncodeError, SOLANA_REQUEST_VERSION,
};
pub use wire::{
    SolanaHandleEntryWire, SolanaUserDecryptRequestWire, MAX_ACCESS_PROOF_SIBLINGS,
    MAX_REQUEST_HANDLES,
};
