//! The signed ACL-domain scope.
//!
//! A non-empty signed list narrows a permit to those domains, and the test is per handle: the
//! domain of *every* entry must be in the set. Checking only the first entry would let a
//! narrowly scoped permit decrypt foreign-domain handles mixed into the batch.
//!
//! An empty list is permissive and skips this rule entirely — deliberately, for parity with
//! the EVM path, where both entry points accept the empty list on chain. Permissive widens
//! both branches: it opens all of the signer's own handles and every delegation the signer
//! currently holds. What it never touches is ownership and delegation themselves, which are
//! unconditional.
//!
//! The domain being tested comes from the validated lineage. A request has no field for it, and
//! this rule has no parameter through which one could arrive:
//!
//! ```compile_fail
//! use kms_worker::core::solana::scope::{check_scope, ScopeFailure};
//! use kms_worker::core::solana_acl::SolanaPubkeyBytes;
//! use zama_solana_permit::AclDomainKeys;
//!
//! // Taking the domain as a value would make the caller the authority on which domain a
//! // handle belongs to.
//! let check: fn(&AclDomainKeys, SolanaPubkeyBytes) -> Result<(), ScopeFailure> = check_scope;
//! ```

use super::lineage::ResolvedLineage;
use crate::core::solana_acl::SolanaPubkeyBytes;
use zama_solana_permit::AclDomainKeys;

/// Tests one entry's lineage domain against the signed scope.
pub fn check_scope(
    signed_scope: &AclDomainKeys,
    lineage: &ResolvedLineage,
) -> Result<(), ScopeFailure> {
    // An empty signed list is permissive and skips the rule, for parity with the EVM path.
    if signed_scope.is_permissive() {
        return Ok(());
    }
    let domain = lineage.domain();
    if signed_scope
        .as_slice()
        .iter()
        .any(|signed| signed.as_bytes() == &domain)
    {
        Ok(())
    } else {
        Err(ScopeFailure::DomainNotAllowed { domain })
    }
}

/// Why an entry fell outside the signed scope.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum ScopeFailure {
    /// The lineage's ACL domain is not in the signed set.
    #[error("lineage domain {domain:?} is outside the signed scope")]
    DomainNotAllowed {
        /// The domain the lineage belongs to.
        domain: SolanaPubkeyBytes,
    },
}
