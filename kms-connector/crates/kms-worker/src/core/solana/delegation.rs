//! Delegation records. A delegated entry needs a live `delegator → signer` record: either the row
//! for the encrypted store's application `(program, scope)`, or the delegator's wildcard row, as
//! with the EVM ACL's wildcard delegation. A dead row cannot veto a live one, so narrowing a
//! delegation to fewer applications means revoking the wildcard row too. A row the host program
//! could not have written fails the entry whatever the other row says.
//!
//! Live means `expires_at` is after the Unix time of the deciding read's Clock, as EVM's
//! `expirationDate > block.timestamp`. A revoked row holds 0, so it authorizes no more than a row
//! never granted.
//! The record's `delegation_counter` is not checked: pinning it would invalidate in-flight
//! requests on every unrelated delegation update.

use super::failure::InvalidHostRecord;
use super::snapshot::{ObservedRow, ObservedRows};
use solana_pubkey::Pubkey;
use zama_solana_acl::{EncryptedStore, WILDCARD_APP};

/// Which row carried a delegated authorization, for the audit log.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AuthorizedRow {
    Exact,
    Wildcard,
}

/// Checks that `delegator` has a live delegation to `delegate` in the application of `store`, at
/// the Clock of the read of `rows`. Both rows are judged first: an invalid row fails the entry
/// even when the other is live.
pub fn check_delegation(
    rows: &ObservedRows,
    program_id: Pubkey,
    delegator: Pubkey,
    delegate: Pubkey,
    store: &EncryptedStore,
) -> Result<AuthorizedRow, DelegationFailure> {
    let judge = |row, app_program, app_scope| {
        judge_row(
            row,
            program_id,
            rows.now,
            [delegator, delegate, app_program, app_scope],
        )
    };
    let wildcard_app = Pubkey::new_from_array(WILDCARD_APP);
    let exact = judge(
        &rows.exact,
        Pubkey::new_from_array(store.program),
        Pubkey::new_from_array(store.scope),
    )?;
    let wildcard = judge(&rows.wildcard, wildcard_app, wildcard_app)?;
    match (exact, wildcard) {
        (None, _) => Ok(AuthorizedRow::Exact),
        (_, None) => Ok(AuthorizedRow::Wildcard),
        (Some(exact), Some(wildcard)) => Err(DelegationFailure::NoLiveDelegation {
            exact,
            wildcard,
            now: rows.now,
        }),
    }
}

/// Why the row the Connector derived for `[delegator, delegate, program, scope]` is dead at `now`,
/// or `None` when it is live.
fn judge_row(
    row: &ObservedRow,
    program_id: Pubkey,
    now: u64,
    [delegator, delegate, app_program, app_scope]: [Pubkey; 4],
) -> Result<Option<DeadRow>, InvalidHostRecord> {
    let Some(account) = row
        .account
        .as_ref()
        .filter(|account| !account.is_uninitialized_pda())
    else {
        return Ok(Some(DeadRow::Absent));
    };
    let invalid = InvalidHostRecord {
        account_key: row.key,
    };
    if account.owner != program_id {
        return Err(invalid);
    }
    let record =
        zama_solana_acl::decode_user_decryption_delegation(&account.data).map_err(|_| invalid)?;
    if !record.names(
        delegator.as_array(),
        delegate.as_array(),
        app_program.as_array(),
        app_scope.as_array(),
    ) || record.bump != row.bump
    {
        return Err(invalid);
    }
    Ok((!record.is_live_at(now)).then_some(DeadRow::NotLive {
        expires_at: record.expires_at,
    }))
}

/// Why one delegation row does not authorize. A revoked row holds 0, so it is `NotLive`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeadRow {
    Absent,
    NotLive { expires_at: u64 },
}

impl std::fmt::Display for DeadRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Absent => f.write_str("absent"),
            Self::NotLive { expires_at } => write!(f, "expires at {expires_at}"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum DelegationFailure {
    /// Neither row authorizes, so both reasons are reported.
    #[error("no live delegation at {now}: application row {exact}; wildcard row {wildcard}")]
    NoLiveDelegation {
        exact: DeadRow,
        wildcard: DeadRow,
        now: u64,
    },
    #[error(transparent)]
    InvalidHostRecord(#[from] InvalidHostRecord),
}
