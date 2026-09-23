//! The signed application scope.
//!
//! A non-empty signed list narrows the permit to those `(program, scope)` pairs, tested for every
//! entry. An empty list admits everything, as on EVM. The pair comes from the validated encrypted
//! store, never from the request.

use super::encrypted_store::ResolvedEncryptedStore;
use crate::core::solana_acl::SolanaPubkeyBytes;
use zama_solana_permit::{AllowedScopes, Identity};

pub fn check_scope(
    signed_scopes: &AllowedScopes,
    encrypted_store: &ResolvedEncryptedStore,
) -> Result<(), ScopeFailure> {
    let program = encrypted_store.program();
    let scope = encrypted_store.scope();
    if signed_scopes.admits(&Identity::new(program), &Identity::new(scope)) {
        Ok(())
    } else {
        Err(ScopeFailure::ScopeNotAllowed { program, scope })
    }
}

#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum ScopeFailure {
    #[error("application ({program:?}, {scope:?}) is outside the signed scope")]
    ScopeNotAllowed {
        program: SolanaPubkeyBytes,
        scope: SolanaPubkeyBytes,
    },
}
