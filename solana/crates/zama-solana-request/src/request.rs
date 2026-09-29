//! A Solana user-decryption request in the typed form every authorizer reads: a strictly
//! decoded permit, its signature, and one entry per handle. [`SolanaUserDecryptRequest::assemble`]
//! is the only way to build one, so the relayer's admission and each connector's authorization
//! read the same request under the same rules.
//!
//! An entry claims the account whose allow leaf authorizes it and the encrypted store holding
//! that leaf. It cannot name the store's authority or application `(program, scope)`: those are
//! read from the store account, so a request cannot substitute them. The delegated branch looks
//! its delegation row up by that application, and a request able to name one could name an
//! application the signer holds a delegation for, against a store belonging to another. Adding
//! such a field must stay a compile error:
//!
//! ```compile_fail
//! use zama_solana_request::HandleEntry;
//!
//! let entry = HandleEntry {
//!     handle: [0; 32],
//!     owner_address: [0; 32],
//!     encrypted_store: [0; 32],
//!     program: [0; 32],
//! };
//! ```
//!
//! The same literal without that field compiles, so the example above fails for that reason
//! alone:
//!
//! ```
//! use zama_solana_request::HandleEntry;
//!
//! let entry = HandleEntry {
//!     handle: [0; 32],
//!     owner_address: [0; 32],
//!     encrypted_store: [0; 32],
//! };
//! ```

use zama_solana_permit::{PermitFields, Signature};

/// Upper bound on the handles of one request.
///
/// Every rule is evaluated against one atomic `getMultipleAccounts` snapshot, and a standard
/// Solana RPC node serves at most 100 accounts per call. The worst-case read carries three
/// accounts per entry (the encrypted store plus the exact and wildcard delegation rows), the
/// signer's invalidation record and the Clock sysvar that delegation expiry is checked against:
/// `1 + 1 + N + 2N <= 100` gives 32. The Gateway refuses a larger request before the fee with its
/// own copy, `MAX_SOLANA_DECRYPT_HANDLES`, which a test below pins to this one.
pub const MAX_REQUEST_HANDLES: usize = 32;

/// A Solana user-decryption request: 1 to [`MAX_REQUEST_HANDLES`] handles of one Solana chain,
/// the permit signed for that chain, and the signature. The signature is not verified here; each
/// authorizer verifies it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SolanaUserDecryptRequest {
    pub(crate) permit: PermitFields,
    pub(crate) signature: Signature,
    pub(crate) entries: Vec<HandleEntry>,
}

impl SolanaUserDecryptRequest {
    /// The permit. Its chain id is the one every handle embeds.
    pub fn permit(&self) -> &PermitFields {
        &self.permit
    }

    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    /// In request order; duplicates are kept and each one is authorized.
    pub fn entries(&self) -> &[HandleEntry] {
        &self.entries
    }
}

/// One handle and the unsigned claims that authorize it. A substituted claim can fail the entry
/// against host state and the leaf record, but never widen access.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct HandleEntry {
    pub handle: [u8; 32],
    /// The account whose allow leaf on the handle is checked: the signer for a direct entry, the
    /// delegator for a delegated one. Neither the account-owner program nor the store authority.
    pub owner_address: [u8; 32],
    /// The encrypted store whose history holds that leaf.
    pub encrypted_store: [u8; 32],
}

#[cfg(test)]
mod tests {
    use super::MAX_REQUEST_HANDLES;

    #[test]
    fn the_gateway_admits_the_same_handle_count() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../gateway-contracts/contracts/Decryption.sol"
        );
        let source = std::fs::read_to_string(path).expect("read Decryption.sol");
        let declared = source
            .split("uint8 internal constant MAX_SOLANA_DECRYPT_HANDLES = ")
            .nth(1)
            .and_then(|rest| rest.split(';').next())
            .expect("Decryption.sol declares MAX_SOLANA_DECRYPT_HANDLES");
        assert_eq!(declared.parse::<usize>(), Ok(MAX_REQUEST_HANDLES));
    }
}
