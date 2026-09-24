//! Delegation records. A delegated entry needs a live `delegator → signer` record: either the row
//! for the encrypted store's application `(program, scope)`, or the delegator's wildcard row, as
//! with the EVM ACL's wildcard delegation. Neither row can veto the other, so narrowing a
//! delegation to fewer applications means revoking the wildcard row too.
//!
//! Live means `expires_at` is after the Unix time of the deciding read's Clock, as EVM's
//! `expirationDate > block.timestamp`. A revoked row holds 0, so it reads like one never granted.
//! The record's `delegation_counter` is not checked: pinning it would invalidate in-flight
//! requests on every unrelated delegation update.

use super::snapshot::{HostSnapshot, UnreadAccount};
use super::{SolanaPubkeyBytes, delegation_address};
use zama_solana_acl::{EncryptedStore, WILDCARD_APP};

/// Which row carried a delegated authorization, for the audit log.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AuthorizedRow {
    Exact,
    Wildcard,
}

/// Checks that `delegator` has a live delegation to `delegate` in the application of `store`, at
/// the Unix time `now`.
pub fn check_delegation(
    snapshot: &HostSnapshot,
    program_id: SolanaPubkeyBytes,
    now: u64,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
    store: &EncryptedStore,
) -> Result<AuthorizedRow, DelegationFailure> {
    let row = |app_program, app_scope| Row {
        snapshot,
        program_id,
        now,
        delegator,
        delegate,
        app_program,
        app_scope,
    };
    let Err(exact) = row(store.program, store.scope).check()? else {
        return Ok(AuthorizedRow::Exact);
    };
    let Err(wildcard) = row(WILDCARD_APP, WILDCARD_APP).check()? else {
        return Ok(AuthorizedRow::Wildcard);
    };
    Err(DelegationFailure::NoLiveDelegation {
        exact: Box::new(exact),
        wildcard: Box::new(wildcard),
    })
}

/// One delegation row to look up.
struct Row<'a> {
    snapshot: &'a HostSnapshot,
    program_id: SolanaPubkeyBytes,
    now: u64,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
    app_program: SolanaPubkeyBytes,
    app_scope: SolanaPubkeyBytes,
}

impl Row<'_> {
    /// The outer error is a lookup that could not be made; the inner one is why the row does not
    /// authorize, so the two can never be confused in [`DelegationFailure::NoLiveDelegation`].
    fn check(&self) -> Result<Result<(), DelegationFailure>, UnreadAccount> {
        let (account_key, canonical_bump) = delegation_address(
            self.program_id,
            self.delegator,
            self.delegate,
            self.app_program,
            self.app_scope,
        );
        let Some(account) = self
            .snapshot
            .account(&account_key)?
            .filter(|account| !account.is_uninitialized_pda())
        else {
            return Ok(Err(DelegationFailure::Absent { account_key }));
        };
        if account.owner != self.program_id {
            return Ok(Err(DelegationFailure::ForeignOwner {
                account_key,
                owner: account.owner,
            }));
        }
        let Ok(record) = zama_solana_acl::decode_user_decryption_delegation(&account.data) else {
            return Ok(Err(DelegationFailure::NotADelegationRecord { account_key }));
        };
        if !record.names(
            &self.delegator,
            &self.delegate,
            &self.app_program,
            &self.app_scope,
        ) {
            return Ok(Err(DelegationFailure::TupleMismatch { account_key }));
        }
        if record.bump != canonical_bump {
            return Ok(Err(DelegationFailure::NotADelegationRecord { account_key }));
        }
        if !record.is_live_at(self.now) {
            return Ok(Err(DelegationFailure::NotLive {
                expires_at: record.expires_at,
                now: self.now,
            }));
        }
        Ok(Ok(()))
    }
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
    /// Expired or revoked; a revoked row holds 0.
    #[error("delegation is not live: expires at {expires_at}, now {now}")]
    NotLive { expires_at: u64, now: u64 },
    /// Neither row authorizes, so both reasons are reported.
    #[error("no live delegation: application row: {exact}; wildcard row: {wildcard}")]
    NoLiveDelegation {
        exact: Box<DelegationFailure>,
        wildcard: Box<DelegationFailure>,
    },
    #[error(transparent)]
    UnreadAccount(#[from] UnreadAccount),
}
