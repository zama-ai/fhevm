//! Delegation records. A delegated entry needs a live `delegator → signer` record: either the row
//! for the encrypted store's authority, or the delegator's wildcard row, as with the EVM ACL's
//! wildcard delegation. Neither row can veto the other, so narrowing a delegation to fewer
//! authorities means revoking the wildcard row too.
//!
//! Live means, at the observed slot: not revoked, not expired, and not written after the
//! observation. The record's `delegation_counter` is not checked: pinning it would invalidate
//! in-flight requests on every unrelated delegation update.

use super::snapshot::{HostSnapshot, SnapshotError};
use super::{SolanaPubkeyBytes, delegation_address};
use zama_solana_acl::WILDCARD_AUTHORITY;

/// Which row carried a delegated authorization, for the audit log.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AuthorizedRow {
    Exact,
    Wildcard,
}

/// Checks that `delegator` has a live delegation to `delegate` covering `authority`.
pub fn check_delegation(
    snapshot: &HostSnapshot,
    program_id: SolanaPubkeyBytes,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
    authority: SolanaPubkeyBytes,
) -> Result<AuthorizedRow, DelegationFailure> {
    let Err(exact) = check_row(snapshot, program_id, delegator, delegate, authority)? else {
        return Ok(AuthorizedRow::Exact);
    };
    let Err(wildcard) = check_row(
        snapshot,
        program_id,
        delegator,
        delegate,
        WILDCARD_AUTHORITY,
    )?
    else {
        return Ok(AuthorizedRow::Wildcard);
    };
    if matches!(wildcard, DelegationFailure::Absent { .. }) && exact.is_recoverable() {
        return Err(exact);
    }
    Err(DelegationFailure::NoLiveGrant {
        exact: Box::new(exact),
        wildcard: Box::new(wildcard),
    })
}

/// The outer error is a lookup that could not be made; the inner one is why the row does not
/// authorize, so the two can never be confused in [`DelegationFailure::NoLiveGrant`].
fn check_row(
    snapshot: &HostSnapshot,
    program_id: SolanaPubkeyBytes,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
    authority: SolanaPubkeyBytes,
) -> Result<Result<(), DelegationFailure>, SnapshotError> {
    let (account_key, canonical_bump) =
        delegation_address(program_id, delegator, delegate, authority);
    let Some(account) = snapshot
        .account(&account_key)?
        .filter(|account| !account.is_uninitialized_pda())
    else {
        return Ok(Err(DelegationFailure::Absent { account_key }));
    };
    if account.owner != program_id {
        return Ok(Err(DelegationFailure::ForeignOwner {
            account_key,
            owner: account.owner,
        }));
    }
    let Ok(record) = zama_solana_acl::decode_user_decryption_delegation(&account.data) else {
        return Ok(Err(DelegationFailure::NotADelegationRecord { account_key }));
    };
    if record.delegator != delegator || record.delegate != delegate || record.authority != authority
    {
        return Ok(Err(DelegationFailure::TupleMismatch { account_key }));
    }
    if record.bump != canonical_bump {
        return Ok(Err(DelegationFailure::NotADelegationRecord { account_key }));
    }
    if record.revoked {
        return Ok(Err(DelegationFailure::Revoked));
    }
    let observed_slot = snapshot.observed_slot();
    if !record.is_live_at(observed_slot) {
        return Ok(Err(DelegationFailure::Expired {
            expiration_slot: record.expiration_slot,
            observed_slot,
        }));
    }
    if record.last_update_slot > observed_slot {
        return Ok(Err(DelegationFailure::NewerThanObservation {
            last_update_slot: record.last_update_slot,
            observed_slot,
        }));
    }
    Ok(Ok(()))
}

#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum DelegationFailure {
    #[error("no delegation record at {account_key:?} at the observed slot")]
    Absent { account_key: SolanaPubkeyBytes },
    #[error("delegation record {account_key:?} is owned by {owner:?}")]
    ForeignOwner {
        account_key: SolanaPubkeyBytes,
        owner: SolanaPubkeyBytes,
    },
    #[error("account {account_key:?} is not a canonical delegation record")]
    NotADelegationRecord { account_key: SolanaPubkeyBytes },
    #[error("delegation record {account_key:?} names a different tuple")]
    TupleMismatch { account_key: SolanaPubkeyBytes },
    #[error("delegation is revoked")]
    Revoked,
    #[error("delegation expired: expiration slot {expiration_slot} < observed {observed_slot}")]
    Expired {
        expiration_slot: u64,
        observed_slot: u64,
    },
    /// A coherent node cannot report this: the observation is behind the write.
    #[error("delegation written at {last_update_slot}, after the observed slot {observed_slot}")]
    NewerThanObservation {
        last_update_slot: u64,
        observed_slot: u64,
    },
    /// Both rows exist and neither authorizes, so both reasons are reported.
    #[error("no live delegation: authority row: {exact}; wildcard row: {wildcard}")]
    NoLiveGrant {
        exact: Box<DelegationFailure>,
        wildcard: Box<DelegationFailure>,
    },
    #[error(transparent)]
    Snapshot(#[from] SnapshotError),
}
