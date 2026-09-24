//! Delegation records. A delegated entry needs a live `delegator → signer` record: either the row
//! for the encrypted store's application `(program, scope)`, or the delegator's wildcard row, as
//! with the EVM ACL's wildcard delegation. Neither row can veto the other, so narrowing a
//! delegation to fewer applications means revoking the wildcard row too.
//!
//! Live means `expires_at` is after the Unix time of the deciding read's Clock, as EVM's
//! `expirationDate > block.timestamp`. A revoked row holds 0, so it reads like one never granted.
//! The record's `delegation_counter` is not checked: pinning it would invalidate in-flight
//! requests on every unrelated delegation update.

use super::SolanaPubkeyBytes;
use super::snapshot::{ObservedRow, ObservedRows};
use zama_solana_acl::{EncryptedStore, WILDCARD_APP};

/// Which row carried a delegated authorization, for the audit log.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AuthorizedRow {
    Exact,
    Wildcard,
}

/// Checks that `delegator` has a live delegation to `delegate` in the application of `store`, at
/// the Clock of the read of `rows`.
pub fn check_delegation(
    rows: &ObservedRows,
    program_id: SolanaPubkeyBytes,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
    store: &EncryptedStore,
) -> Result<AuthorizedRow, DelegationFailure> {
    let check = |row, app_program, app_scope| {
        check_row(
            row,
            program_id,
            rows.now,
            [delegator, delegate, app_program, app_scope],
        )
    };
    let Err(exact) = check(&rows.exact, store.program, store.scope) else {
        return Ok(AuthorizedRow::Exact);
    };
    let Err(wildcard) = check(&rows.wildcard, WILDCARD_APP, WILDCARD_APP) else {
        return Ok(AuthorizedRow::Wildcard);
    };
    Err(DelegationFailure::NoLiveDelegation {
        exact: Box::new(exact),
        wildcard: Box::new(wildcard),
    })
}

/// Why the row the Connector derived for `[delegator, delegate, program, scope]` does not
/// authorize at `now`, if it does not.
fn check_row(
    row: &ObservedRow,
    program_id: SolanaPubkeyBytes,
    now: u64,
    tuple: [SolanaPubkeyBytes; 4],
) -> Result<(), DelegationFailure> {
    let account_key = row.key;
    let Some(account) = row
        .account
        .as_ref()
        .filter(|account| !account.is_uninitialized_pda())
    else {
        return Err(DelegationFailure::Absent { account_key });
    };
    if account.owner != program_id {
        return Err(DelegationFailure::ForeignOwner {
            account_key,
            owner: account.owner,
        });
    }
    let Ok(record) = zama_solana_acl::decode_user_decryption_delegation(&account.data) else {
        return Err(DelegationFailure::NotADelegationRecord { account_key });
    };
    let [delegator, delegate, app_program, app_scope] = tuple;
    if !record.names(&delegator, &delegate, &app_program, &app_scope) {
        return Err(DelegationFailure::TupleMismatch { account_key });
    }
    if record.bump != row.bump {
        return Err(DelegationFailure::NotADelegationRecord { account_key });
    }
    if !record.is_live_at(now) {
        return Err(DelegationFailure::NotLive {
            expires_at: record.expires_at,
            now,
        });
    }
    Ok(())
}

#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum DelegationFailure {
    #[error("no delegation record at {account_key:?}")]
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
    /// Expired or revoked; a revoked row holds 0.
    #[error("delegation is not live: expires at {expires_at}, now {now}")]
    NotLive { expires_at: u64, now: u64 },
    /// Neither row authorizes, so both reasons are reported.
    #[error("no live delegation: application row: {exact}; wildcard row: {wildcard}")]
    NoLiveDelegation {
        exact: Box<DelegationFailure>,
        wildcard: Box<DelegationFailure>,
    },
}
