//! Delegation freshness.
//!
//! A delegated entry needs a live delegation record `delegator → signer` for the lineage's
//! app account. Live means, against the observed slot: not revoked, not expired, and not
//! written after the observation. The last clause is what keeps a record "from the future"
//! relative to the snapshot from authorizing anything — it would be a state the rest of the
//! authorization never saw.
//!
//! The record's `delegation_counter` takes no part in any of this. It is not signed and is
//! pinned nowhere in the request: pinning it would kill mixed-delegator batches and permit
//! reuse, because any update to any delegation record would invalidate requests already in
//! flight. The counter still exists in the on-chain layout — decoding walks past it — but no
//! check reads it and no signature commits to it.
//!
//! The app account comes from the validated lineage. That is what makes the delegated branch
//! safe against an attacker naming an app they do hold a delegation for: they cannot name it
//! at all.

use super::snapshot::{HostSnapshot, SnapshotError};
use crate::core::solana_acl::{SolanaPubkeyBytes, decode_user_decryption_delegation_witness};

/// The canonical delegation-record address for a `(delegator, delegate, app_account)` tuple.
pub fn delegation_address(
    program_id: SolanaPubkeyBytes,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
    app_account: SolanaPubkeyBytes,
) -> (SolanaPubkeyBytes, u8) {
    crate::core::solana_acl::user_decryption_delegation_address(
        program_id,
        delegator,
        delegate,
        app_account,
    )
}

/// Checks that `delegator` has a live delegation to `delegate` for `app_account` at this
/// observation point.
///
/// Note the parameter list: a snapshot, three identities and a program id. No counter, and no
/// reader — the record is read from the observation the rest of the authorization used.
pub fn check_delegation(
    snapshot: &HostSnapshot,
    program_id: SolanaPubkeyBytes,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
    app_account: SolanaPubkeyBytes,
) -> Result<(), DelegationFailure> {
    let (account_key, _) = delegation_address(program_id, delegator, delegate, app_account);

    let Some(account) = snapshot.account(&account_key)? else {
        // Includes the case of a delegation granted for another app: that record lives at
        // another address, and the address derived from this lineage's app is simply empty.
        return Err(DelegationFailure::Absent { account_key });
    };

    if account.owner != program_id {
        return Err(DelegationFailure::ForeignOwner {
            account_key,
            owner: account.owner,
            expected: program_id,
        });
    }

    // The layout decoder is the one this connector already reads delegation records with, so the
    // two paths cannot drift on the byte layout. Every failure it can report — wrong length,
    // wrong discriminator, an invalid field — says the same thing about these bytes, which is
    // what the variant below is named for.
    let record =
        decode_user_decryption_delegation_witness(account_key, account.owner, &account.data)
            .map_err(|_| DelegationFailure::NotADelegationRecord { account_key })?;

    // The address is not taken as proof of what the record says.
    if record.delegator != delegator
        || record.delegate != delegate
        || record.app_account != app_account
    {
        return Err(DelegationFailure::TupleMismatch { account_key });
    }

    if record.revoked {
        return Err(DelegationFailure::Revoked);
    }

    // Both slot bounds are inclusive, and both are against the observation rather than a local
    // clock. `record.delegation_counter` is decoded by the call above and read by nothing here:
    // pinning it would invalidate in-flight requests on every unrelated delegation update.
    let observed_slot = snapshot.observed_slot();
    if record.expiration_slot < observed_slot {
        return Err(DelegationFailure::Expired {
            expiration_slot: record.expiration_slot,
            observed_slot,
        });
    }
    if record.last_update_slot > observed_slot {
        return Err(DelegationFailure::NewerThanObservation {
            last_update_slot: record.last_update_slot,
            observed_slot,
        });
    }

    Ok(())
}

/// Why a delegated entry was not authorized.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum DelegationFailure {
    /// No delegation record exists for the tuple at this observation point.
    #[error("no delegation record at {account_key:?} at the observed slot")]
    Absent {
        /// The canonical address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The record exists but is not owned by the deployment's program.
    #[error("delegation record {account_key:?} is owned by {owner:?}, expected {expected:?}")]
    ForeignOwner {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
        /// Who owns it.
        owner: SolanaPubkeyBytes,
        /// The deployment's program id.
        expected: SolanaPubkeyBytes,
    },
    /// The account is host-owned but is not a delegation record, or does not decode.
    #[error("account {account_key:?} is not a decodable delegation record")]
    NotADelegationRecord {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The record's own fields name a different tuple than the address implies.
    #[error("delegation record {account_key:?} names a different tuple")]
    TupleMismatch {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The delegator revoked it. Subsequent requests stop immediately, even while the
    /// delegate's own permit remains valid.
    #[error("delegation is revoked")]
    Revoked,
    /// It expired at or before the observed slot.
    #[error("delegation expired: expiration slot {expiration_slot} < observed {observed_slot}")]
    Expired {
        /// The record's expiration slot.
        expiration_slot: u64,
        /// The observation point.
        observed_slot: u64,
    },
    /// It was written after the observation point, so it is not part of the state this
    /// authorization saw.
    #[error(
        "delegation is newer than the observation: written at {last_update_slot} > {observed_slot}"
    )]
    NewerThanObservation {
        /// When the record was last written.
        last_update_slot: u64,
        /// The observation point.
        observed_slot: u64,
    },
    /// The snapshot was asked for an account it never read.
    #[error(transparent)]
    Snapshot(#[from] SnapshotError),
}
