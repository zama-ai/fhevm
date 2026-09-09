//! The signed application scope.
//!
//! A non-empty signed list narrows a permit to those `(program, scope)` pairs, and the test is
//! per handle: the pair of *every* entry's encrypted value account must be in the set. Checking
//! only the first entry would let a narrowly scoped permit decrypt foreign handles mixed into the
//! batch.
//!
//! An empty list is permissive and skips this rule entirely — deliberately, for parity with
//! the EVM path, where both entry points accept the empty list on chain. Permissive widens
//! both branches: it opens all of the signer's own handles and every delegation the signer
//! currently holds. What it never touches is the allow leaf and the delegation themselves, which
//! are unconditional.
//!
//! The pair being tested comes from the validated encrypted value account. A request has no field
//! for it, and this rule has no parameter through which one could arrive:
//!
//! ```compile_fail
//! use kms_worker::core::solana::scope::{check_scope, ScopeFailure};
//! use kms_worker::core::solana_acl::SolanaPubkeyBytes;
//! use zama_solana_permit::AllowedScopes;
//!
//! // Taking the pair as values would make the caller the authority on which application a
//! // handle belongs to.
//! let check: fn(&AllowedScopes, SolanaPubkeyBytes, SolanaPubkeyBytes) -> Result<(), ScopeFailure> =
//!     check_scope;
//! ```

use super::encrypted_value_account::ResolvedEncryptedValueAccount;
use crate::core::solana_acl::SolanaPubkeyBytes;
use zama_solana_permit::{AllowedScopes, Identity};

/// Tests one entry's `(program, scope)` against the signed scope list.
pub fn check_scope(
    signed_scopes: &AllowedScopes,
    encrypted_value_account: &ResolvedEncryptedValueAccount,
) -> Result<(), ScopeFailure> {
    let program = encrypted_value_account.program();
    let scope = encrypted_value_account.scope();
    // `admits` is permissive on the empty list, for parity with the EVM path.
    if signed_scopes.admits(&Identity::new(program), &Identity::new(scope)) {
        Ok(())
    } else {
        Err(ScopeFailure::ScopeNotAllowed { program, scope })
    }
}

/// Why an entry fell outside the signed scope.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum ScopeFailure {
    /// The encrypted value account's `(program, scope)` is not in the signed set.
    #[error("application ({program:?}, {scope:?}) is outside the signed scope")]
    ScopeNotAllowed {
        /// The application program the encrypted value account belongs to.
        program: SolanaPubkeyBytes,
        /// The program-declared scope within it.
        scope: SolanaPubkeyBytes,
    },
}
