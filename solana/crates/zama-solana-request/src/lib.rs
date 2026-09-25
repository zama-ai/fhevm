//! Canonical transport form of the Zama fhevm Solana user-decryption request.
//!
//! One implementation of the request canon for every Rust consumer: the wire types the
//! authorizer reads, the single encoder/decoder pair for the opaque blob the gateway carries,
//! and the one function that joins that blob with the fields the gateway types.
//!
//! The relayer builds the bytes because it submits the gateway transaction; each KMS party's
//! connector reads them because it authorizes. Both call this crate, so the layout has one
//! definition and a field added to it is a compile error on both sides.
//!
//! What this crate checks is only whether the two carriers make one request: a handle count
//! within the cap, one entry per handle, 32-byte handles, one chain. It does not type the permit,
//! verify a signature, or read a clock or an account: the consumer that authorizes owns its own
//! validated type and every rule about live state.

/// Joining the gateway-typed fields and the blob into the full request.
pub mod assemble;
/// The canonical byte layout of the blob: version byte and borsh body.
pub mod codec;
/// The assembled request, before the consumer types it.
pub mod wire;

pub use assemble::{
    assemble_solana_request, SolanaEntryClaims, SolanaRequestAssemblyError, SolanaRequestBlob,
    SolanaUserDecryptFields,
};
pub use codec::{
    decode_solana_request, encode_solana_request, SolanaRequestDecodeError,
    SolanaRequestEncodeError, SOLANA_REQUEST_VERSION,
};
pub use wire::{SolanaHandleEntryWire, SolanaUserDecryptRequestWire, MAX_REQUEST_HANDLES};
