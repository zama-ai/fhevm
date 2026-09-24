//! Advisory, negative-only pre-check of Solana delegated user-decrypt entries.
//!
//! The EVM host-ACL pre-check exists because a request the KMS connectors will reject still
//! costs a gateway transaction and, with no rejection channel in the Decryption contract, the
//! requester only ever learns of the refusal by timeout (recorded motives, in order: gas,
//! faster feedback, defense in depth, UX). This module is the Solana delegated-flow analogue.
//! The authoritative check remains with the KMS connectors — this one is advisory and biased
//! the only safe way an advisory check can be:
//!
//! * it refuses ONLY what the authoritative check could not possibly authorize — the
//!   delegation rows of the entry's tuple are dead (absent, revoked, or expired at the host's
//!   Clock) in this read. "In this read" is a real caveat: a node lagging at `confirmed` shows a
//!   freshly granted delegation as absent, or a refreshed one as expired, and the refusal is
//!   then one the connector, reading later, would not repeat. That window is accepted policy
//!   rather than an oversight — the EVM pre-check reading `latest` has carried the same
//!   exposure since it was introduced, passing absent rows through would send the common case (no
//!   grant ever existed) to a doomed gateway transaction, and a caller inside the window
//!   succeeds by resubmitting;
//! * every ambiguity of *data* passes: a live row, an unreadable or misshapen account, an
//!   unresolvable encrypted store. A false pass costs one doomed gateway transaction (what the
//!   connector-side check is for); a false refusal would block an authorized user, so a
//!   fetched world this check cannot judge always passes;
//! * a *transport* failure is not ambiguity: an RPC that cannot be read at all, retries
//!   exhausted, refuses the request (`HostAclError::CallFailed`) — the same policy as the EVM
//!   pre-check, so the client-visible contract does not fork by host chain. A node that is
//!   merely *behind* is not that failure: the row read requires the encrypted-state read's slot
//!   (`minContextSlot`) and is re-checked against it on arrival, and a node that never catches
//!   up passes rather than refusing. The two reads are therefore never two views of the chain
//!   in the wrong order — the ordering the connector gets from its own `deciding_after` gate.
//!
//! Direct entries (`owner_address == user_address`) are not pre-checked at all: their
//! authorization is an allow leaf the connector fetches from the coprocessors and verifies
//! against the account, and there is no cheaper reading of it here than the connector's own.
//!
//! One byte-level implementation, not a second one: the record decoder and the liveness rule
//! come from `zama-solana-acl` — the same crate the connector's authoritative check reads the
//! same bytes through.
//!
//! Everything except the two RPC reads lives here, pure and tested without a network: the
//! address derivations, the plan built between the reads ([`plan_row_reads`]), and the pairing
//! of fetched rows back to their entries ([`judge_planned_entries`]). The transport in
//! `acl_checker` only carries bytes between these functions.

use zama_solana_acl::delegation::{decode_user_decryption_delegation, WILDCARD_APP};
use zama_solana_acl::{
    decode_clock_unix_timestamp, decode_encrypted_store, delegation_seeds, CLOCK_SYSVAR_ID,
};

/// One fetched account, exactly as the RPC returned it.
#[derive(Clone, Debug)]
pub(crate) struct RawAccount {
    pub owner: [u8; 32],
    pub data: Vec<u8>,
}

/// What this advisory check concluded about one delegated entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum EntryVerdict {
    /// A live delegation row authorizes the entry at the host's Clock.
    Allowed,
    /// Definitively dead at the host's Clock — the authoritative check can only agree.
    NotAllowed { reason: String },
    /// Anything this check cannot judge; the connector decides.
    Indeterminate,
}

/// The inputs of one entry's verdict: the claimed identities, the three fetched accounts and
/// the host's Clock.
pub(crate) struct EntryVerdictInputs<'a> {
    /// The deployment's host program id.
    pub program_id: [u8; 32],
    /// The address of the encrypted store the entry names.
    pub encrypted_store_key: [u8; 32],
    /// The entry's owner address — the delegator whose access is asked for.
    pub delegator: [u8; 32],
    /// The permit's signer — the delegate.
    pub delegate: [u8; 32],
    /// The encrypted store at that address, if it exists.
    pub encrypted_store: Option<&'a RawAccount>,
    /// The row of the store's application, if it exists (fetched second, once the application
    /// is known from the encrypted store).
    pub exact_row: Option<&'a RawAccount>,
    /// The delegator's wildcard row, if it exists.
    pub wildcard_row: Option<&'a RawAccount>,
    /// The host Clock's Unix time in the row read, which the liveness rule is evaluated at.
    pub now: u64,
}

/// The application a delegation row is keyed by: the store's program and scope, or
/// [`WILDCARD_APP`] in both positions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct AppScope {
    pub program: [u8; 32],
    pub scope: [u8; 32],
}

impl AppScope {
    const WILDCARD: Self = Self {
        program: WILDCARD_APP,
        scope: WILDCARD_APP,
    };
}

/// Decides one delegated entry. Pure: every ambiguity is [`EntryVerdict::Indeterminate`].
pub(crate) fn entry_verdict(inputs: &EntryVerdictInputs) -> EntryVerdict {
    let Some(app) = resolve_encrypted_store_application(
        inputs.program_id,
        inputs.encrypted_store_key,
        inputs.encrypted_store,
    ) else {
        return EntryVerdict::Indeterminate;
    };

    let exact = row_state(inputs.exact_row, inputs, app);
    let wildcard = row_state(inputs.wildcard_row, inputs, AppScope::WILDCARD);

    match (exact, wildcard) {
        (RowState::Live, _) | (_, RowState::Live) => EntryVerdict::Allowed,
        // Only two definitively dead rows refuse: the authoritative check can only agree.
        (RowState::Dead(exact_reason), RowState::Dead(wildcard_reason)) => {
            EntryVerdict::NotAllowed {
                reason: format!(
                    "no live delegation of (delegator, delegate, application) at the host's \
                     clock: application row {exact_reason}; wildcard row {wildcard_reason}"
                ),
            }
        }
        // A row this check cannot read is not a row it may refuse on.
        (RowState::Unreadable, _) | (_, RowState::Unreadable) => EntryVerdict::Indeterminate,
    }
}

/// The application the entry's encrypted store belongs to, when the account resolves cleanly:
/// present, host-owned, decodable, deriving the address it was read from, and not the wildcard
/// sentinel. `None` is "this advisory check cannot judge the encrypted store" — never a
/// refusal.
pub(crate) fn resolve_encrypted_store_application(
    program_id: [u8; 32],
    encrypted_store_key: [u8; 32],
    encrypted_store: Option<&RawAccount>,
) -> Option<AppScope> {
    let account = encrypted_store?;
    if account.owner != program_id {
        return None;
    }
    let state = decode_encrypted_store(&account.data).ok()?;
    if encrypted_store_address(&state, program_id) != Some(encrypted_store_key) {
        return None;
    }
    // The sentinel-program case is the connector's own guard; this check stands aside.
    if state.program == WILDCARD_APP {
        return None;
    }
    Some(AppScope {
        program: state.program,
        scope: state.scope,
    })
}

/// What one delegation row contributes to the verdict.
enum RowState {
    /// Exists, names the expected tuple, and is live at the host's Clock.
    Live,
    /// Definitively dead: absent, or naming the tuple while revoked or expired.
    Dead(&'static str),
    /// Anything else — a state this advisory reader must leave to the connector.
    Unreadable,
}

fn row_state(row: Option<&RawAccount>, inputs: &EntryVerdictInputs, app: AppScope) -> RowState {
    let Some(account) = row else {
        return RowState::Dead("is absent");
    };
    if account.owner != inputs.program_id {
        return RowState::Unreadable;
    }
    let Ok(record) = decode_user_decryption_delegation(&account.data) else {
        return RowState::Unreadable;
    };
    if !record.names(
        &inputs.delegator,
        &inputs.delegate,
        &app.program,
        &app.scope,
    ) {
        return RowState::Unreadable;
    }
    // The liveness boundary is the shared crate's, not a re-spelling; only the reason of a
    // dead row is named locally. A grant always expires after the time it was made, so only a
    // revocation leaves 0.
    if record.is_live_at(inputs.now) {
        return RowState::Live;
    }
    if record.expires_at == 0 {
        return RowState::Dead("is revoked");
    }
    RowState::Dead("is expired")
}

/// One delegated entry as admission handed it over: the claimed identities, plus the handle
/// for refusal attribution.
pub(crate) struct DelegatedEntry {
    pub handle_hex: String,
    /// The entry's owner address, which on a delegated entry is the delegator.
    pub delegator: [u8; 32],
    pub encrypted_store: [u8; 32],
}

/// An entry whose encrypted-state read resolved to a judgeable account. It
/// carries that account with it — no index into someone else's array — so a later
/// filter cannot silently
/// re-pair entries with accounts.
pub(crate) struct PlannedEntry {
    pub handle_hex: String,
    pub delegator: [u8; 32],
    pub encrypted_store_key: [u8; 32],
    pub encrypted_store: RawAccount,
}

/// What the encrypted-state read planned for the row read: `entries[i]`'s rows sit at
/// `addresses[2i]` (application) and `addresses[2i + 1]` (wildcard), and the Clock sysvar is
/// last — built by [`plan_row_reads`], consumed by [`judge_planned_entries`]. The Clock rides in
/// the same read so liveness is judged at the time of the slot the rows were read at.
pub(crate) struct RowReadPlan {
    pub entries: Vec<PlannedEntry>,
    pub addresses: Vec<[u8; 32]>,
}

/// One refused entry: the handle and the reason the rows are definitively dead.
pub(crate) struct EntryRefusal {
    pub handle_hex: String,
    pub reason: String,
}

/// A read this module refuses to judge on: an account list that does not match the entries it
/// was fetched for, or a row read without a readable Clock. The advisory check passes (with a
/// warning) rather than misattribute accounts to entries or guess the time.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ReadDefect {
    EncryptedStores { entries: usize, accounts: usize },
    RowAccounts { entries: usize, accounts: usize },
    Clock,
}

/// The addresses of the encrypted-state read, one per entry in order: the entry names
/// the account by address, and the account's own fields are checked against it once read.
pub(crate) fn encrypted_store_read_addresses(entries: &[DelegatedEntry]) -> Vec<[u8; 32]> {
    entries.iter().map(|entry| entry.encrypted_store).collect()
}

/// Plans the row read from the encrypted-state read's result. Entries this check cannot
/// judge from their encrypted store drop out here (indeterminate — the connector
/// decides); each surviving entry contributes its two row addresses in the interleaving
/// [`RowReadPlan`] documents.
pub(crate) fn plan_row_reads(
    program_id: [u8; 32],
    delegate: [u8; 32],
    entries: Vec<DelegatedEntry>,
    encrypted_stores: Vec<Option<RawAccount>>,
) -> Result<RowReadPlan, ReadDefect> {
    if encrypted_stores.len() != entries.len() {
        return Err(ReadDefect::EncryptedStores {
            entries: entries.len(),
            accounts: encrypted_stores.len(),
        });
    }
    let mut plan = RowReadPlan {
        entries: Vec::new(),
        addresses: Vec::new(),
    };
    for (entry, encrypted_store) in entries.into_iter().zip(encrypted_stores) {
        let Some(encrypted_store) = encrypted_store else {
            continue;
        };
        let Some(app) = resolve_encrypted_store_application(
            program_id,
            entry.encrypted_store,
            Some(&encrypted_store),
        ) else {
            continue;
        };
        let (exact_address, wildcard_address) =
            solana_delegation_row_addresses(&entry.delegator, &delegate, app, program_id);
        plan.addresses.push(exact_address);
        plan.addresses.push(wildcard_address);
        plan.entries.push(PlannedEntry {
            handle_hex: entry.handle_hex,
            delegator: entry.delegator,
            encrypted_store_key: entry.encrypted_store,
            encrypted_store,
        });
    }
    plan.addresses.push(CLOCK_SYSVAR_ID);
    Ok(plan)
}

/// Pairs the fetched rows back to their entries and collects the refusals. The pairing is the
/// plan's interleaving read back with `chunks_exact(2)` — no index arithmetic at the call
/// site, and an account count that does not match the plan is a defect, never a
/// misattribution.
pub(crate) fn judge_planned_entries(
    program_id: [u8; 32],
    delegate: [u8; 32],
    entries: &[PlannedEntry],
    accounts: &[Option<RawAccount>],
) -> Result<Vec<EntryRefusal>, ReadDefect> {
    let Some((clock, row_accounts)) = accounts
        .split_last()
        .filter(|(_, rows)| rows.len() == entries.len() * 2)
    else {
        return Err(ReadDefect::RowAccounts {
            entries: entries.len(),
            accounts: accounts.len(),
        });
    };
    let now = clock
        .as_ref()
        .and_then(|clock| decode_clock_unix_timestamp(&clock.owner, &clock.data).ok())
        .ok_or(ReadDefect::Clock)?;
    Ok(entries
        .iter()
        .zip(row_accounts.chunks_exact(2))
        .filter_map(|(entry, rows)| {
            let verdict = entry_verdict(&EntryVerdictInputs {
                program_id,
                encrypted_store_key: entry.encrypted_store_key,
                delegator: entry.delegator,
                delegate,
                encrypted_store: Some(&entry.encrypted_store),
                exact_row: rows[0].as_ref(),
                wildcard_row: rows[1].as_ref(),
                now,
            });
            match verdict {
                EntryVerdict::Allowed | EntryVerdict::Indeterminate => None,
                EntryVerdict::NotAllowed { reason } => Some(EntryRefusal {
                    handle_hex: entry.handle_hex.clone(),
                    reason,
                }),
            }
        })
        .collect())
}

/// The canonical PDA for `seeds` under `program_id` — the same derivation every other side
/// runs. The delegation seed tuple this crate spells (below) is pinned against the host
/// program's own derivation by fixture literals shared with the runtime-test SDK cross-pins.
fn solana_pda(seeds: &[&[u8]], program_id: [u8; 32]) -> [u8; 32] {
    let program_id = solana_pubkey::Pubkey::new_from_array(program_id);
    let (address, _) = solana_pubkey::Pubkey::find_program_address(seeds, &program_id);
    address.to_bytes()
}

/// The address an encrypted store's own fields derive, with its stored bump: the
/// connector's rule for "this account is the one the entry names", run here on the same fields.
/// `None` when the stored bump does not give a valid PDA.
fn encrypted_store_address(
    state: &zama_solana_acl::EncryptedStore,
    program_id: [u8; 32],
) -> Option<[u8; 32]> {
    let bump = [state.bump];
    let mut seeds: Vec<&[u8]> = state.seeds().to_vec();
    seeds.push(&bump);
    solana_pubkey::Pubkey::create_program_address(
        &seeds,
        &solana_pubkey::Pubkey::new_from_array(program_id),
    )
    .ok()
    .map(|address| address.to_bytes())
}

/// Both delegation row addresses of one `(delegator, delegate)` couple: the application's row
/// and the wildcard row it falls back to.
fn solana_delegation_row_addresses(
    delegator: &[u8; 32],
    delegate: &[u8; 32],
    app: AppScope,
    program_id: [u8; 32],
) -> ([u8; 32], [u8; 32]) {
    let row = |app: AppScope| {
        solana_pda(
            &delegation_seeds(delegator, delegate, &app.program, &app.scope),
            program_id,
        )
    };
    (row(app), row(AppScope::WILDCARD))
}

#[cfg(test)]
mod tests {
    use super::*;
    use zama_solana_acl::delegation::{
        UserDecryptionDelegationRecord, USER_DECRYPTION_DELEGATION_DISCRIMINATOR,
    };
    use zama_solana_acl::{encrypted_store_discriminator, EncryptedStore, SYSVAR_OWNER_ID};

    const PROGRAM_ID: [u8; 32] = [7; 32];
    const APP_PROGRAM: [u8; 32] = [1; 32];
    const SCOPE: [u8; 32] = [4; 32];
    const DELEGATOR: [u8; 32] = [0x11; 32];
    const DELEGATE: [u8; 32] = [0x22; 32];
    const AUTHORITY: [u8; 32] = [0x33; 32];
    const NOW: u64 = 1_700_000_000;

    /// An encrypted store of `(program, scope)` at the address its fields derive.
    fn encrypted_store_for(program: [u8; 32], scope: [u8; 32]) -> (RawAccount, [u8; 32]) {
        let (address, bump) = solana_pubkey::Pubkey::find_program_address(
            &[
                zama_solana_acl::ENCRYPTED_STORE_SEED,
                &program,
                &AUTHORITY,
                &scope,
            ],
            &solana_pubkey::Pubkey::new_from_array(PROGRAM_ID),
        );
        let state = EncryptedStore {
            program,
            authority: AUTHORITY,
            scope,
            slots: vec![],
            leaf_count: 0,
            peaks: vec![],
            bump,
        };
        let mut data = encrypted_store_discriminator().to_vec();
        borsh::BorshSerialize::serialize(&state, &mut data).expect("serializes");
        (
            RawAccount {
                owner: PROGRAM_ID,
                data,
            },
            address.to_bytes(),
        )
    }

    fn encrypted_store() -> RawAccount {
        encrypted_store_for(APP_PROGRAM, SCOPE).0
    }

    fn fixture_encrypted_store_key() -> [u8; 32] {
        encrypted_store_for(APP_PROGRAM, SCOPE).1
    }

    /// The record bytes, spelled field by field in the order the host's layout sets.
    fn row(record: &UserDecryptionDelegationRecord) -> RawAccount {
        let mut data = USER_DECRYPTION_DELEGATION_DISCRIMINATOR.to_vec();
        data.extend_from_slice(&record.delegator);
        data.extend_from_slice(&record.delegate);
        data.extend_from_slice(&record.program);
        data.extend_from_slice(&record.scope);
        data.extend_from_slice(&record.expires_at.to_le_bytes());
        data.extend_from_slice(&record.delegation_counter.to_le_bytes());
        data.extend_from_slice(&record.last_update_slot.to_le_bytes());
        data.push(record.bump);
        RawAccount {
            owner: PROGRAM_ID,
            data,
        }
    }

    /// The Clock sysvar at `unix_timestamp`: four leading fields, then the time.
    fn clock(unix_timestamp: i64) -> RawAccount {
        let mut data = Vec::new();
        for field in [500u64, 0, 1, 2] {
            data.extend_from_slice(&field.to_le_bytes());
        }
        data.extend_from_slice(&unix_timestamp.to_le_bytes());
        RawAccount {
            owner: SYSVAR_OWNER_ID,
            data,
        }
    }

    fn live_exact() -> UserDecryptionDelegationRecord {
        UserDecryptionDelegationRecord {
            delegator: DELEGATOR,
            delegate: DELEGATE,
            program: APP_PROGRAM,
            scope: SCOPE,
            expires_at: NOW + 100,
            delegation_counter: 1,
            last_update_slot: 490,
            bump: 250,
        }
    }

    fn live_wildcard() -> UserDecryptionDelegationRecord {
        UserDecryptionDelegationRecord {
            program: WILDCARD_APP,
            scope: WILDCARD_APP,
            ..live_exact()
        }
    }

    fn revoked(record: UserDecryptionDelegationRecord) -> UserDecryptionDelegationRecord {
        UserDecryptionDelegationRecord {
            expires_at: 0,
            ..record
        }
    }

    fn verdict(
        encrypted_store: Option<&RawAccount>,
        exact_row: Option<&RawAccount>,
        wildcard_row: Option<&RawAccount>,
    ) -> EntryVerdict {
        entry_verdict(&EntryVerdictInputs {
            program_id: PROGRAM_ID,
            encrypted_store_key: fixture_encrypted_store_key(),
            delegator: DELEGATOR,
            delegate: DELEGATE,
            encrypted_store,
            exact_row,
            wildcard_row,
            now: NOW,
        })
    }

    #[test]
    fn a_live_row_of_the_stores_application_allows() {
        let value = encrypted_store();
        let exact = row(&live_exact());
        assert_eq!(
            verdict(Some(&value), Some(&exact), None),
            EntryVerdict::Allowed
        );
    }

    #[test]
    fn a_live_wildcard_row_allows_when_the_exact_row_is_dead() {
        let value = encrypted_store();
        let exact = row(&revoked(live_exact()));
        let wildcard = row(&live_wildcard());
        assert_eq!(
            verdict(Some(&value), Some(&exact), Some(&wildcard)),
            EntryVerdict::Allowed
        );
    }

    /// `expires_at` is exclusive, as EVM's `expirationDate > block.timestamp`: the last live
    /// second is the one before it.
    #[test]
    fn a_delegation_ends_at_its_expiry_second() {
        let value = encrypted_store();
        let last_second = row(&UserDecryptionDelegationRecord {
            expires_at: NOW + 1,
            ..live_exact()
        });
        assert_eq!(
            verdict(Some(&value), Some(&last_second), None),
            EntryVerdict::Allowed
        );
        let at_expiry = row(&UserDecryptionDelegationRecord {
            expires_at: NOW,
            ..live_exact()
        });
        assert_eq!(
            verdict(Some(&value), Some(&at_expiry), None),
            EntryVerdict::NotAllowed {
                reason: "no live delegation of (delegator, delegate, application) at the host's \
                         clock: application row is expired; wildcard row is absent"
                    .to_string()
            }
        );
    }

    #[test]
    fn a_revoked_exact_row_with_no_wildcard_refuses() {
        let value = encrypted_store();
        let exact = row(&revoked(live_exact()));
        assert_eq!(
            verdict(Some(&value), Some(&exact), None),
            EntryVerdict::NotAllowed {
                reason: "no live delegation of (delegator, delegate, application) at the host's \
                         clock: application row is revoked; wildcard row is absent"
                    .to_string()
            }
        );
    }

    #[test]
    fn an_expired_exact_row_beside_a_revoked_wildcard_refuses() {
        let value = encrypted_store();
        let exact = row(&UserDecryptionDelegationRecord {
            expires_at: NOW - 1,
            ..live_exact()
        });
        let wildcard = row(&revoked(live_wildcard()));
        assert!(matches!(
            verdict(Some(&value), Some(&exact), Some(&wildcard)),
            EntryVerdict::NotAllowed { .. }
        ));
    }

    #[test]
    fn no_row_at_all_refuses() {
        let value = encrypted_store();
        assert!(matches!(
            verdict(Some(&value), None, None),
            EntryVerdict::NotAllowed { .. }
        ));
    }

    #[test]
    fn an_absent_encrypted_store_is_indeterminate() {
        assert_eq!(verdict(None, None, None), EntryVerdict::Indeterminate);
    }

    #[test]
    fn a_foreign_owned_encrypted_store_is_indeterminate() {
        let mut value = encrypted_store();
        value.owner = [9; 32];
        assert_eq!(
            verdict(Some(&value), None, None),
            EntryVerdict::Indeterminate
        );
    }

    #[test]
    fn an_encrypted_store_at_another_address_is_indeterminate() {
        let value = encrypted_store();
        let verdict = entry_verdict(&EntryVerdictInputs {
            program_id: PROGRAM_ID,
            encrypted_store_key: [0xaa; 32],
            delegator: DELEGATOR,
            delegate: DELEGATE,
            encrypted_store: Some(&value),
            exact_row: None,
            wildcard_row: None,
            now: NOW,
        });
        assert_eq!(verdict, EntryVerdict::Indeterminate);
    }

    /// A store whose program is the wildcard sentinel is the connector's guard to reject; the
    /// advisory check stands aside rather than refusing on the wildcard's own row.
    #[test]
    fn a_store_of_the_sentinel_program_is_indeterminate() {
        let (value, key) = encrypted_store_for(WILDCARD_APP, WILDCARD_APP);
        let verdict = entry_verdict(&EntryVerdictInputs {
            program_id: PROGRAM_ID,
            encrypted_store_key: key,
            delegator: DELEGATOR,
            delegate: DELEGATE,
            encrypted_store: Some(&value),
            exact_row: None,
            wildcard_row: None,
            now: NOW,
        });
        assert_eq!(verdict, EntryVerdict::Indeterminate);
    }

    #[test]
    fn an_unreadable_row_never_refuses() {
        let value = encrypted_store();
        let garbage = RawAccount {
            owner: PROGRAM_ID,
            data: vec![0xde, 0xad],
        };
        assert_eq!(
            verdict(Some(&value), Some(&garbage), None),
            EntryVerdict::Indeterminate
        );
    }

    #[test]
    fn a_row_naming_another_tuple_never_refuses() {
        let value = encrypted_store();
        for stranger in [
            UserDecryptionDelegationRecord {
                delegate: [0x99; 32],
                ..revoked(live_exact())
            },
            UserDecryptionDelegationRecord {
                scope: [0x99; 32],
                ..revoked(live_exact())
            },
        ] {
            let exact = row(&stranger);
            assert_eq!(
                verdict(Some(&value), Some(&exact), None),
                EntryVerdict::Indeterminate
            );
        }
    }

    // ---- the plan and the pairing ----

    fn entry(handle: &str, encrypted_store: [u8; 32]) -> DelegatedEntry {
        DelegatedEntry {
            handle_hex: handle.to_string(),
            delegator: DELEGATOR,
            encrypted_store,
        }
    }

    fn planned(handle: &str) -> PlannedEntry {
        PlannedEntry {
            handle_hex: handle.to_string(),
            delegator: DELEGATOR,
            encrypted_store_key: fixture_encrypted_store_key(),
            encrypted_store: encrypted_store(),
        }
    }

    /// An unjudgeable entry between two judgeable ones drops out of the plan without
    /// shifting the pairing: the surviving entries keep the row addresses of their own
    /// applications, and the Clock closes the read.
    #[test]
    fn the_plan_drops_unjudgeable_entries_without_shifting_the_pairing() {
        let (value_a, id_a) = encrypted_store_for(APP_PROGRAM, [0x41; 32]);
        let (value_c, id_c) = encrypted_store_for(APP_PROGRAM, [0x43; 32]);

        let plan = plan_row_reads(
            PROGRAM_ID,
            DELEGATE,
            vec![
                entry("0xaa", id_a),
                entry("0xbb", [0xbb; 32]),
                entry("0xcc", id_c),
            ],
            vec![Some(value_a), None, Some(value_c)],
        )
        .expect("aligned inputs plan");

        let handles: Vec<&str> = plan
            .entries
            .iter()
            .map(|planned| planned.handle_hex.as_str())
            .collect();
        assert_eq!(handles, ["0xaa", "0xcc"]);

        let rows = |scope: [u8; 32]| {
            solana_delegation_row_addresses(
                &DELEGATOR,
                &DELEGATE,
                AppScope {
                    program: APP_PROGRAM,
                    scope,
                },
                PROGRAM_ID,
            )
        };
        let (rows_a, rows_c) = (rows([0x41; 32]), rows([0x43; 32]));
        assert_ne!(
            rows_a.0, rows_c.0,
            "the two applications have their own rows"
        );
        assert_eq!(
            plan.addresses,
            vec![rows_a.0, rows_a.1, rows_c.0, rows_c.1, CLOCK_SYSVAR_ID],
            "each surviving entry keeps its own rows, application before wildcard"
        );
    }

    /// A refusal lands on the entry whose rows are dead, wherever it sits in the batch.
    #[test]
    fn a_refusal_is_attributed_to_the_entry_the_rows_belong_to() {
        let live_row = Some(row(&live_exact()));
        let now = Some(clock(NOW as i64));

        let refusals = judge_planned_entries(
            PROGRAM_ID,
            DELEGATE,
            &[planned("0xdead"), planned("0xa11e")],
            &[None, None, live_row.clone(), None, now.clone()],
        )
        .expect("a matching account count judges");
        assert_eq!(refusals.len(), 1);
        assert_eq!(refusals[0].handle_hex, "0xdead");

        // Mirrored order: the refusal follows the entry, not the position.
        let refusals = judge_planned_entries(
            PROGRAM_ID,
            DELEGATE,
            &[planned("0xa11e"), planned("0xdead")],
            &[live_row, None, None, None, now],
        )
        .expect("a matching account count judges");
        assert_eq!(refusals.len(), 1);
        assert_eq!(refusals[0].handle_hex, "0xdead");
    }

    /// Pins the exact-then-wildcard interleaving: a revoked record in the application position
    /// refuses (both rows dead), while the same record in the wildcard position names the
    /// wrong tuple and refuses nothing.
    #[test]
    fn the_application_row_is_read_before_the_wildcard_row() {
        let revoked_row = Some(row(&revoked(live_exact())));
        let now = Some(clock(NOW as i64));

        let refusals = judge_planned_entries(
            PROGRAM_ID,
            DELEGATE,
            &[planned("0xdead")],
            &[revoked_row.clone(), None, now.clone()],
        )
        .expect("a matching account count judges");
        assert_eq!(
            refusals.len(),
            1,
            "revoked application row, no wildcard row"
        );

        let refusals = judge_planned_entries(
            PROGRAM_ID,
            DELEGATE,
            &[planned("0xdead")],
            &[None, revoked_row, now],
        )
        .expect("a matching account count judges");
        assert!(
            refusals.is_empty(),
            "an application row read as the wildcard"
        );
    }

    /// Liveness is judged at the Clock the row read carried, not at any other time: the same
    /// row is live before its expiry and refused at it.
    #[test]
    fn liveness_is_judged_at_the_clock_of_the_row_read() {
        let judge = |unix: u64| {
            judge_planned_entries(
                PROGRAM_ID,
                DELEGATE,
                &[planned("0xdead")],
                &[Some(row(&live_exact())), None, Some(clock(unix as i64))],
            )
            .expect("a matching account count judges")
            .len()
        };
        assert_eq!(judge(NOW + 99), 0);
        assert_eq!(judge(NOW + 100), 1);
    }

    /// Without a Clock it can trust, the check does not guess the time: it judges nothing.
    #[test]
    fn a_row_read_without_a_readable_clock_is_a_defect() {
        let mut foreign = clock(NOW as i64);
        foreign.owner = PROGRAM_ID;
        let mut short = clock(NOW as i64);
        short.data.pop();
        for bad_clock in [None, Some(foreign), Some(short), Some(clock(-1))] {
            assert_eq!(
                judge_planned_entries(
                    PROGRAM_ID,
                    DELEGATE,
                    &[planned("0xdead")],
                    &[None, None, bad_clock],
                )
                .err(),
                Some(ReadDefect::Clock)
            );
        }
    }

    /// An account count that does not match the plan is a defect, never a misattribution — and
    /// an encrypted-state count that does not match the entries likewise.
    #[test]
    fn mismatched_account_counts_are_defects_not_verdicts() {
        for accounts in [vec![], vec![None, None]] {
            assert_eq!(
                judge_planned_entries(PROGRAM_ID, DELEGATE, &[planned("0xdead")], &accounts).err(),
                Some(ReadDefect::RowAccounts {
                    entries: 1,
                    accounts: accounts.len(),
                })
            );
        }

        let (_, id) = encrypted_store_for(APP_PROGRAM, [0x41; 32]);
        assert_eq!(
            plan_row_reads(PROGRAM_ID, DELEGATE, vec![entry("0xaa", id)], vec![]).err(),
            Some(ReadDefect::EncryptedStores {
                entries: 1,
                accounts: 0
            })
        );
    }

    /// The PDA derivations, pinned against the host program's own: the same inputs and
    /// literals are asserted in
    /// `solana/runtime-tests/tests/user_decryption_delegation_mollusk.rs`, so a seed-order
    /// drift on either side breaks both suites on the same bytes. A drift here would read
    /// absent rows for every delegated entry — a deterministic false refusal invisible to
    /// the verdict tests above, which are fed pre-fetched accounts.
    #[test]
    fn solana_pda_derivations_match_the_host_program_fixtures() {
        let program_id = crate::http::utils::solana_address::decode_solana_address(
            "DPq5y89RDZPq9NcMh9X1NgjBWgYmSXg3QoipSBV3ZMzQ",
        )
        .expect("the zama-host program id");
        let as_base58 =
            |address: [u8; 32]| solana_pubkey::Pubkey::new_from_array(address).to_string();

        let (exact_row, wildcard_row) = solana_delegation_row_addresses(
            &[0x11; 32],
            &[0x22; 32],
            AppScope {
                program: [0x33; 32],
                scope: [0x44; 32],
            },
            program_id,
        );
        assert_eq!(
            as_base58(exact_row),
            "GkmqVNMzqxopBjPkSkZvuLuDE6Jze3iA3Mq5ZHr6SrtJ"
        );
        assert_eq!(
            as_base58(wildcard_row),
            "J4BMamYLJvJroFATJp48L6AeQJDQqv86YAQyPqvBcKq1"
        );
    }
}
