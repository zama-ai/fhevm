//! Delegation records, judged against the Unix time of the deciding read's Clock. The record's
//! `delegation_counter` is not checked: pinning it would invalidate in-flight requests on every
//! unrelated delegation update.

use super::failure::InvalidHostRecord;
use super::snapshot::{ObservedRow, ObservedRows, SnapshotAccount};
use solana_pubkey::Pubkey;
use zama_solana_acl::{
    DeadRow, DelegationRow, DelegationVerdict, EncryptedStore, WILDCARD_APP, judge_delegation,
    judge_delegation_row,
};

/// Checks that `delegator` has a live delegation to `delegate` in the application of `store`, at
/// the Clock of the read of `rows`, by the rule the relayer's pre-check also applies
/// ([`judge_delegation`]). A row the host program could not have written fails the entry.
pub fn check_delegation(
    rows: &ObservedRows,
    program_id: Pubkey,
    delegator: Pubkey,
    delegate: Pubkey,
    store: &EncryptedStore,
) -> Result<DelegationRow, DelegationFailure> {
    let judge = |row: &ObservedRow, app_program: &[u8; 32], app_scope: &[u8; 32]| {
        judge_delegation_row(
            program_id.as_array(),
            row.account.as_ref().map(SnapshotAccount::view),
            row.bump,
            [
                delegator.as_array(),
                delegate.as_array(),
                app_program,
                app_scope,
            ],
            rows.now,
        )
    };
    let exact = judge(&rows.exact, &store.program, &store.scope);
    let wildcard = judge(&rows.wildcard, &WILDCARD_APP, &WILDCARD_APP);
    match judge_delegation(exact, wildcard) {
        DelegationVerdict::Authorized(row) => Ok(row),
        DelegationVerdict::NoLiveDelegation { exact, wildcard } => {
            Err(DelegationFailure::NoLiveDelegation {
                exact,
                wildcard,
                now: rows.now,
            })
        }
        DelegationVerdict::InvalidRow(row) => {
            let account_key = match row {
                DelegationRow::Exact => rows.exact.key,
                DelegationRow::Wildcard => rows.wildcard.key,
            };
            Err(InvalidHostRecord { account_key }.into())
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
