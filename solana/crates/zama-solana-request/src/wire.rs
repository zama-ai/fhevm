//! The full request, as [`crate::assemble_solana_request`] builds it and before anything about it
//! has been established.
//!
//! Widths are `Vec<u8>` because a wrong width has to be representable somewhere: this is the
//! form a sender controls, and the consumer that authorizes turns it into its own validated
//! type. Nothing here is trusted, and only the chain id is derived rather than claimed.
//!
//! Three absences are deliberate: no `authority` field, no
//! `(program, scope)` field, and no proof. The first two are properties of the handle's
//! encrypted store, and the only way to learn them is to read and validate that
//! account. A request cannot name them, so a substituted authority is not a check that can
//! be forgotten — it is a value that does not exist. The proof is fetched by the verifier
//! from the coprocessor's leaf record and verified against the account's own peaks; a proof
//! a client could hand in would be a proof the verifier has to be talked into trusting.
//!
//! # The application is not a request field
//!
//! Naming the store's `(program, scope)` in a request must stay a compile error, because the
//! delegated branch is looked up by it: a request that could name one could name an application
//! the signer does hold a delegation for, against an encrypted store belonging to another.
//!
//! ```compile_fail
//! use zama_solana_request::SolanaHandleEntryWire;
//!
//! let entry = SolanaHandleEntryWire {
//!     handle: vec![0; 32],
//!     owner_address: vec![0; 32],
//!     encrypted_store: vec![0; 32],
//!     program: vec![0; 32],
//! };
//! ```
//!
//! The same literal without that field compiles. The pair matters: a `compile_fail` example
//! passes when compilation fails for *any* reason, so on its own it would also pass on a typo.
//!
//! ```
//! use zama_solana_request::SolanaHandleEntryWire;
//!
//! let entry = SolanaHandleEntryWire {
//!     handle: vec![0; 32],
//!     owner_address: vec![0; 32],
//!     encrypted_store: vec![0; 32],
//! };
//! ```

use zama_solana_permit::PermitWireFields;

/// Upper bound on handle entries accepted from a request.
///
/// Every rule is evaluated against one atomic `getMultipleAccounts` snapshot, and a standard
/// Solana RPC node serves at most 100 accounts per call. The worst-case read carries three
/// accounts per entry (the encrypted store plus the exact and wildcard delegation rows), the
/// signer's invalidation record and the Clock sysvar that delegation expiry is checked against:
/// `1 + 1 + N + 2N <= 100` gives 32.
///
/// It lives here, next to the wire form, because both ends need the same number: the relayer
/// refuses an oversized request before it submits one, and the connector refuses one that
/// reached it anyway. The Gateway refuses it before the fee with its own copy,
/// `MAX_SOLANA_DECRYPT_HANDLES`, which a test below pins to this one. That copy also caps a
/// Solana public decryption, whose read (one store per handle) is smaller.
pub const MAX_REQUEST_HANDLES: usize = 32;

/// The full request: permit fields, the signature over their envelope, and the handle entries.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct SolanaUserDecryptRequestWire {
    /// The eight signed permit fields, in transport form. `chain_id` is the one the handles embed.
    pub permit: PermitWireFields,
    /// Claimed Ed25519 signature over the reconstructed envelope.
    pub signature: Vec<u8>,
    /// Handle entries, in request order.
    pub handles: Vec<SolanaHandleEntryWire>,
}

/// One handle entry as it arrives. None of these fields are signed: they are claims,
/// self-authenticating against host state and the leaf record, and a substituted value can
/// fail the request but never widen access.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct SolanaHandleEntryWire {
    /// Claimed 32-byte ciphertext handle.
    pub handle: Vec<u8>,
    /// Claimed 32-byte account whose permission authorizes this entry, i.e. whose allow leaf on
    /// the handle is checked: the requester itself for a direct entry, the delegator for a
    /// delegated one. It is neither the Solana account-owner program nor the store authority.
    pub owner_address: Vec<u8>,
    /// Claimed 32-byte address of the encrypted store whose history contains the handle.
    pub encrypted_store: Vec<u8>,
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
