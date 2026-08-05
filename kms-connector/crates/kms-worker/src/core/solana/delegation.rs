//! Delegation freshness.
//!
//! A delegated entry needs a live delegation record `delegator → signer`. Two records can carry
//! one: the row for the encrypted value account's own app account, and the delegator's wildcard row
//! — the same derivation with the reserved app-context sentinel in place of an app account, which
//! is how a delegator grants across every one of their apps at once. Either row being live
//! authorizes the entry, which is the rule the EVM ACL applies to its own wildcard delegation.
//!
//! Live means, against the observed slot: not revoked, not expired, and not written after the
//! observation. The last clause is what keeps a record "from the future" relative to the snapshot
//! from authorizing anything — it would be a state the rest of the authorization never saw.
//!
//! The rows are tried app-specific first, and neither can veto the other: an exact row that is
//! revoked, expired or newer than the observation still leaves a live wildcard row authorizing, and
//! the same holds the other way around.
//!
//! What follows from that is deliberate rather than incidental: revoking the app-specific row does
//! not stop a delegate who also holds a wildcard row. Scope-by-app is a property of a row, not of
//! the delegation as a whole, so a delegator narrowing one app has to revoke the wildcard row as
//! well — and the host program's revocation instruction takes one record per call, so that is two
//! transactions rather than one.
//!
//! The record's `delegation_counter` takes no part in any of this. It is not signed and is pinned
//! nowhere in the request: pinning it would kill mixed-delegator batches and permit reuse, because
//! any update to any delegation record would invalidate requests already in flight. The counter
//! still exists in the on-chain layout — decoding walks past it — but no check reads it and no
//! signature commits to it.
//!
//! The app account comes from the validated encrypted value account. That is what makes the
//! delegated branch safe against an attacker naming an app they do hold a delegation for: they
//! cannot name it at all.

use super::snapshot::{HostSnapshot, SnapshotError};
use crate::core::solana_acl::{SolanaPubkeyBytes, decode_user_decryption_delegation_witness};

/// The app-context sentinel a wildcard row carries in place of an app account. Reserved by the
/// host program, which is why no real app account can collide with it.
pub use crate::core::solana_acl::WILDCARD_APP_CONTEXT;

/// The canonical delegation-record address for a `(delegator, delegate, encrypted_value_account_authority)` tuple.
pub fn delegation_address(
    program_id: SolanaPubkeyBytes,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
    encrypted_value_account_authority: SolanaPubkeyBytes,
) -> (SolanaPubkeyBytes, u8) {
    crate::core::solana_acl::user_decryption_delegation_address(
        program_id,
        delegator,
        delegate,
        encrypted_value_account_authority,
    )
}

/// The canonical address of the wildcard row of `(delegator, delegate)`.
///
/// One function rather than a sentinel passed by callers: the key planner and the rule have to
/// derive the same address, and a sentinel spelled out at two call sites is a sentinel that can be
/// spelled out differently at two call sites.
pub fn wildcard_delegation_address(
    program_id: SolanaPubkeyBytes,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
) -> (SolanaPubkeyBytes, u8) {
    delegation_address(program_id, delegator, delegate, WILDCARD_APP_CONTEXT)
}

/// Checks that `delegator` has a live delegation to `delegate` covering `encrypted_value_account_authority` at this
/// observation point: the row for that app, or the delegator's wildcard row.
///
/// Note the parameter list: a snapshot, three identities and a program id. No counter, and no
/// reader — both records are read from the observation the rest of the authorization used.
pub fn check_delegation(
    snapshot: &HostSnapshot,
    program_id: SolanaPubkeyBytes,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
    encrypted_value_account_authority: SolanaPubkeyBytes,
) -> Result<(), DelegationFailure> {
    let exact = match check_row(
        snapshot,
        program_id,
        delegator,
        delegate,
        encrypted_value_account_authority,
    )? {
        RowOutcome::Live => return Ok(()),
        RowOutcome::NotLive(reason) => reason,
    };
    let wildcard = match check_row(
        snapshot,
        program_id,
        delegator,
        delegate,
        WILDCARD_APP_CONTEXT,
    )? {
        RowOutcome::Live => return Ok(()),
        RowOutcome::NotLive(reason) => reason,
    };

    // Holding no wildcard row at all is the ordinary case, and in it the app-specific row's reason
    // is the whole story — reporting a pair whose second half is always "and you have no wildcard
    // grant either" would say nothing and would rename every existing diagnostic.
    if let DelegationFailure::Absent { .. } = wildcard {
        return Err(exact);
    }
    Err(DelegationFailure::NoLiveGrant {
        exact: Box::new(exact),
        wildcard: Box::new(wildcard),
    })
}

/// Whether one row authorizes, and if not, why.
///
/// Separate from the reason type because a lookup that cannot be made is not an outcome of the row:
/// it is returned as an error, so it can never be folded into a pair of reasons and reported as
/// "no live grant".
enum RowOutcome {
    /// The row exists and is live at this observation.
    Live,
    /// The row does not authorize. Only row-level reasons appear here.
    NotLive(DelegationFailure),
}

/// Evaluates the single row at the canonical address of `(delegator, delegate, encrypted_value_account_authority)`.
fn check_row(
    snapshot: &HostSnapshot,
    program_id: SolanaPubkeyBytes,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
    encrypted_value_account_authority: SolanaPubkeyBytes,
) -> Result<RowOutcome, SnapshotError> {
    let (account_key, canonical_bump) = delegation_address(
        program_id,
        delegator,
        delegate,
        encrypted_value_account_authority,
    );

    let Some(account) = snapshot.account(&account_key)? else {
        // Includes the case of a delegation granted for another app: that record lives at another
        // address, and the address derived from this encrypted value account's app is simply empty.
        return Ok(RowOutcome::NotLive(DelegationFailure::Absent {
            account_key,
        }));
    };

    if account.owner != program_id {
        return Ok(RowOutcome::NotLive(DelegationFailure::ForeignOwner {
            account_key,
            owner: account.owner,
            expected: program_id,
        }));
    }

    // The layout decoder is the one this connector already reads delegation records with, so the
    // two paths cannot drift on the byte layout. Every failure it can report — wrong length,
    // wrong discriminator, an invalid field — says the same thing about these bytes, which is
    // what the variant below is named for.
    let Ok(record) =
        decode_user_decryption_delegation_witness(account_key, account.owner, &account.data)
    else {
        return Ok(RowOutcome::NotLive(
            DelegationFailure::NotADelegationRecord { account_key },
        ));
    };

    // The address is not taken as proof of what the record says.
    if record.delegator != delegator
        || record.delegate != delegate
        || record.encrypted_value_account_authority != encrypted_value_account_authority
    {
        return Ok(RowOutcome::NotLive(DelegationFailure::TupleMismatch {
            account_key,
        }));
    }

    // The stored bump has to be the canonical one for this address. Not an attacker check — only
    // the owning program can write these bytes, and the address was derived here — but a record
    // whose bump is not the one this derivation produces is not the record this reader reads, and
    // saying so here is cheaper than discovering it as a rule that mysteriously stopped matching.
    if record.bump != canonical_bump {
        return Ok(RowOutcome::NotLive(
            DelegationFailure::NotADelegationRecord { account_key },
        ));
    }

    if record.revoked {
        return Ok(RowOutcome::NotLive(DelegationFailure::Revoked));
    }

    // Both slot bounds are inclusive, and both are against the observation rather than a local
    // clock. `record.delegation_counter` is decoded by the call above and read by nothing here:
    // pinning it would invalidate in-flight requests on every unrelated delegation update.
    let observed_slot = snapshot.observed_slot();
    if record.expiration_slot < observed_slot {
        return Ok(RowOutcome::NotLive(DelegationFailure::Expired {
            expiration_slot: record.expiration_slot,
            observed_slot,
        }));
    }
    if record.last_update_slot > observed_slot {
        return Ok(RowOutcome::NotLive(
            DelegationFailure::NewerThanObservation {
                last_update_slot: record.last_update_slot,
                observed_slot,
            },
        ));
    }

    Ok(RowOutcome::Live)
}

/// Why a delegated entry was not authorized.
///
/// The row-level variants describe one record. They reach a client as themselves when the delegator
/// holds no wildcard row — the ordinary case — and as the two halves of
/// [`DelegationFailure::NoLiveGrant`] when both rows exist and neither authorizes.
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
    /// The account is host-owned but is not a delegation record: it does not decode, or it stores a
    /// bump other than the canonical one for its address.
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
    /// Both rows exist and neither authorizes, so both reasons are reported: naming only one would
    /// send a delegator to fix a row that was not the one standing in the way.
    #[error("no live delegation: app-specific row: {exact}; wildcard row: {wildcard}")]
    NoLiveGrant {
        /// Why the row for the encrypted value account's app account did not authorize.
        exact: Box<DelegationFailure>,
        /// Why the delegator's wildcard row did not authorize.
        wildcard: Box<DelegationFailure>,
    },
    /// The snapshot was asked for an account it never read.
    #[error(transparent)]
    Snapshot(#[from] SnapshotError),
}
