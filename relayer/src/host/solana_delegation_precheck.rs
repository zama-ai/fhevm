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
//!   delegation rows of the entry's tuple are definitively dead (absent, revoked, or expired)
//!   at the slot of this read;
//! * every ambiguity of *data* passes: a live row, an unreadable or misshapen account, an
//!   unresolvable encrypted value account. A false pass costs one doomed gateway transaction (what the
//!   connector-side check is for); a false refusal would block an authorized user, so a
//!   fetched world this check cannot judge always passes;
//! * a *transport* failure is not ambiguity: an RPC that cannot be read at all, retries
//!   exhausted, refuses the request (`HostAclError::CallFailed`) — the same policy as the EVM
//!   pre-check, so the client-visible contract does not fork by host chain.
//!
//! Direct entries (`subject == user_pubkey`) are not pre-checked at all: their authorization
//! is membership in the encrypted value account, and there is no cheaper reading of it here
//! than the connector's own.
//!
//! One byte-level implementation, not a second one: the record decoder and the liveness rule
//! come from `zama-solana-acl` — the same crate the connector's authoritative check reads the
//! same bytes through.
//!
//! Everything except the two RPC reads lives here, pure and tested without a network: the
//! address derivations, the plan built between the reads ([`plan_row_reads`]), and the pairing
//! of fetched rows back to their entries ([`judge_planned_entries`]). The transport in
//! `acl_checker` only carries bytes between these functions.

use zama_solana_acl::decode_on_chain_account;
use zama_solana_acl::delegation::{
    decode_user_decryption_delegation, UserDecryptionDelegationRecord,
    WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY,
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
    /// A live delegation row authorizes the entry at the read slot.
    Allowed,
    /// Definitively dead at the read slot — the authoritative check can only agree.
    NotAllowed { reason: String },
    /// Anything this check cannot judge; the connector decides.
    Indeterminate,
}

/// The inputs of one entry's verdict: the claimed identities and the three fetched accounts.
pub(crate) struct EntryVerdictInputs<'a> {
    /// The deployment's host program id.
    pub program_id: [u8; 32],
    /// The entry's claimed encrypted value id.
    pub encrypted_value_id: [u8; 32],
    /// The entry's subject — the delegator whose access is asked for.
    pub subject: [u8; 32],
    /// The permit's signer — the delegate.
    pub delegate: [u8; 32],
    /// The encrypted value account at the id's canonical address, if it exists.
    pub encrypted_value_account: Option<&'a RawAccount>,
    /// The authority-specific delegation row, if it exists (fetched second, once the
    /// authority is known from the encrypted value account).
    pub exact_row: Option<&'a RawAccount>,
    /// The delegator's wildcard row, if it exists.
    pub wildcard_row: Option<&'a RawAccount>,
    /// The slot of the row read, which the liveness rule is evaluated at.
    pub slot: u64,
}

/// Decides one delegated entry. Pure: every ambiguity is [`EntryVerdict::Indeterminate`].
pub(crate) fn entry_verdict(inputs: &EntryVerdictInputs) -> EntryVerdict {
    let Some(authority) = resolve_encrypted_value_account_authority(
        inputs.program_id,
        inputs.encrypted_value_id,
        inputs.encrypted_value_account,
    ) else {
        return EntryVerdict::Indeterminate;
    };

    let exact = row_state(
        inputs.exact_row,
        inputs.program_id,
        inputs.subject,
        inputs.delegate,
        authority,
        inputs.slot,
    );
    let wildcard = row_state(
        inputs.wildcard_row,
        inputs.program_id,
        inputs.subject,
        inputs.delegate,
        WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY,
        inputs.slot,
    );

    match (exact, wildcard) {
        (RowState::Live, _) | (_, RowState::Live) => EntryVerdict::Allowed,
        // Only two definitively dead rows refuse: the authoritative check can only agree.
        (RowState::Dead(exact_reason), RowState::Dead(wildcard_reason)) => {
            EntryVerdict::NotAllowed {
                reason: format!(
                    "no live delegation of (delegator, delegate, authority) at the read slot: \
                     authority-specific row {exact_reason}; wildcard row {wildcard_reason}"
                ),
            }
        }
        // A row this check cannot read is not a row it may refuse on.
        (RowState::Unreadable, _) | (_, RowState::Unreadable) => EntryVerdict::Indeterminate,
    }
}

/// The authority the entry's encrypted value account names, when the account resolves cleanly:
/// present, host-owned, decodable, reproducing the claimed id, and not the wildcard sentinel.
/// `None` is "this advisory check cannot judge the encrypted value account" — never a refusal.
pub(crate) fn resolve_encrypted_value_account_authority(
    program_id: [u8; 32],
    encrypted_value_id: [u8; 32],
    encrypted_value_account: Option<&RawAccount>,
) -> Option<[u8; 32]> {
    let account = encrypted_value_account?;
    if account.owner != program_id {
        return None;
    }
    let value = decode_on_chain_account(&account.data).ok()?;
    if value.encrypted_value_id() != encrypted_value_id {
        return None;
    }
    // The sentinel-authority case is the connector's own guard; this check stands aside.
    if value.encrypted_value_account_authority == WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY {
        return None;
    }
    Some(value.encrypted_value_account_authority)
}

/// What one delegation row contributes to the verdict.
enum RowState {
    /// Exists, names the expected tuple, and is live at the read slot.
    Live,
    /// Definitively dead: absent, or naming the tuple while revoked or expired.
    Dead(&'static str),
    /// Anything else — a state this advisory reader must leave to the connector.
    Unreadable,
}

fn row_state(
    row: Option<&RawAccount>,
    program_id: [u8; 32],
    delegator: [u8; 32],
    delegate: [u8; 32],
    authority: [u8; 32],
    slot: u64,
) -> RowState {
    let Some(account) = row else {
        return RowState::Dead("is absent");
    };
    if account.owner != program_id {
        return RowState::Unreadable;
    }
    let Ok(record) = decode_user_decryption_delegation(&account.data) else {
        return RowState::Unreadable;
    };
    if !names_tuple(&record, delegator, delegate, authority) {
        return RowState::Unreadable;
    }
    // The liveness boundary is the shared crate's, not a re-spelling; only the reason of a
    // dead row is named locally.
    if record.is_live_at(slot) {
        return RowState::Live;
    }
    if record.revoked {
        return RowState::Dead("is revoked");
    }
    RowState::Dead("is expired")
}

fn names_tuple(
    record: &UserDecryptionDelegationRecord,
    delegator: [u8; 32],
    delegate: [u8; 32],
    authority: [u8; 32],
) -> bool {
    record.delegator == delegator
        && record.delegate == delegate
        && record.encrypted_value_account_authority == authority
}

/// One delegated entry as admission handed it over: the claimed identities, plus the handle
/// for refusal attribution.
pub(crate) struct DelegatedEntry {
    pub handle_hex: String,
    pub subject: [u8; 32],
    pub encrypted_value_id: [u8; 32],
}

/// An entry whose encrypted-value-account read resolved to a judgeable account. It
/// carries that account with it — no index into someone else's array — so a later
/// filter cannot silently
/// re-pair entries with accounts.
pub(crate) struct PlannedEntry {
    pub handle_hex: String,
    pub subject: [u8; 32],
    pub encrypted_value_id: [u8; 32],
    pub encrypted_value_account: RawAccount,
}

/// What the encrypted-value-account read planned for the row read: `entries[i]`'s rows sit at
/// `addresses[2i]` (authority-specific) and `addresses[2i + 1]` (wildcard) — built by the one
/// loop of [`plan_row_reads`], consumed by the `chunks_exact(2)` of [`judge_planned_entries`].
pub(crate) struct RowReadPlan {
    pub entries: Vec<PlannedEntry>,
    pub addresses: Vec<[u8; 32]>,
}

/// One refused entry: the handle and the reason the rows are definitively dead.
pub(crate) struct EntryRefusal {
    pub handle_hex: String,
    pub reason: String,
}

/// A caller-side pairing defect: the account list does not match the entries it was fetched
/// for. This module refuses to judge on it — the advisory check passes (with a warning)
/// rather than misattribute accounts to entries.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum PairingDefect {
    EncryptedValueAccounts { entries: usize, accounts: usize },
    RowAccounts { entries: usize, accounts: usize },
}

/// The addresses of the encrypted-value-account read, one per entry in order.
pub(crate) fn encrypted_value_read_addresses(
    entries: &[DelegatedEntry],
    program_id: [u8; 32],
) -> Vec<[u8; 32]> {
    entries
        .iter()
        .map(|entry| solana_encrypted_value_address(&entry.encrypted_value_id, program_id))
        .collect()
}

/// Plans the row read from the encrypted-value-account read's result. Entries whose encrypted
/// value account this check cannot judge drop out here (indeterminate — the connector
/// decides); each surviving entry
/// contributes its two row addresses in the interleaving [`RowReadPlan`] documents.
pub(crate) fn plan_row_reads(
    program_id: [u8; 32],
    delegate: [u8; 32],
    entries: Vec<DelegatedEntry>,
    encrypted_value_accounts: Vec<Option<RawAccount>>,
) -> Result<RowReadPlan, PairingDefect> {
    if encrypted_value_accounts.len() != entries.len() {
        return Err(PairingDefect::EncryptedValueAccounts {
            entries: entries.len(),
            accounts: encrypted_value_accounts.len(),
        });
    }
    let mut plan = RowReadPlan {
        entries: Vec::new(),
        addresses: Vec::new(),
    };
    for (entry, encrypted_value_account) in entries.into_iter().zip(encrypted_value_accounts) {
        let Some(encrypted_value_account) = encrypted_value_account else {
            continue;
        };
        let Some(authority) =
            resolve_encrypted_value_account_authority(program_id, entry.encrypted_value_id, Some(&encrypted_value_account))
        else {
            continue;
        };
        let (exact_address, wildcard_address) =
            solana_delegation_row_addresses(&entry.subject, &delegate, &authority, program_id);
        plan.addresses.push(exact_address);
        plan.addresses.push(wildcard_address);
        plan.entries.push(PlannedEntry {
            handle_hex: entry.handle_hex,
            subject: entry.subject,
            encrypted_value_id: entry.encrypted_value_id,
            encrypted_value_account,
        });
    }
    Ok(plan)
}

/// Pairs the fetched rows back to their entries and collects the refusals. The pairing is the
/// plan's interleaving read back with `chunks_exact(2)` — no index arithmetic at the call
/// site, and a row count that does not match the plan is a defect, never a misattribution.
pub(crate) fn judge_planned_entries(
    program_id: [u8; 32],
    delegate: [u8; 32],
    entries: &[PlannedEntry],
    row_accounts: &[Option<RawAccount>],
    slot: u64,
) -> Result<Vec<EntryRefusal>, PairingDefect> {
    if row_accounts.len() != entries.len() * 2 {
        return Err(PairingDefect::RowAccounts {
            entries: entries.len(),
            accounts: row_accounts.len(),
        });
    }
    Ok(entries
        .iter()
        .zip(row_accounts.chunks_exact(2))
        .filter_map(|(entry, rows)| {
            let verdict = entry_verdict(&EntryVerdictInputs {
                program_id,
                encrypted_value_id: entry.encrypted_value_id,
                subject: entry.subject,
                delegate,
                encrypted_value_account: Some(&entry.encrypted_value_account),
                exact_row: rows[0].as_ref(),
                wildcard_row: rows[1].as_ref(),
                slot,
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
/// runs. The two seed tuples this crate spells (below) are pinned against the host program's
/// own derivations by fixture literals shared with the runtime-test SDK cross-pins.
fn solana_pda(seeds: &[&[u8]], program_id: [u8; 32]) -> [u8; 32] {
    let program_id = solana_pubkey::Pubkey::new_from_array(program_id);
    let (address, _) = solana_pubkey::Pubkey::find_program_address(seeds, &program_id);
    address.to_bytes()
}

/// The canonical `EncryptedValue` account address for an encrypted value id.
fn solana_encrypted_value_address(
    encrypted_value_id: &[u8; 32],
    program_id: [u8; 32],
) -> [u8; 32] {
    solana_pda(
        &[zama_solana_acl::ENCRYPTED_VALUE_SEED, encrypted_value_id],
        program_id,
    )
}

/// Both delegation row addresses of one `(delegator, delegate)` couple: the authority-specific
/// row and the wildcard row it falls back to. One function so the seed order — delegator,
/// delegate, authority — has exactly one spelling in this crate.
fn solana_delegation_row_addresses(
    delegator: &[u8; 32],
    delegate: &[u8; 32],
    encrypted_value_account_authority: &[u8; 32],
    program_id: [u8; 32],
) -> ([u8; 32], [u8; 32]) {
    let row = |authority: &[u8; 32]| {
        solana_pda(
            &[
                zama_solana_acl::delegation::DELEGATION_SEED,
                delegator,
                delegate,
                authority,
            ],
            program_id,
        )
    };
    (
        row(encrypted_value_account_authority),
        row(&WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use zama_solana_acl::delegation::{
        UserDecryptionDelegationRecord, USER_DECRYPTION_DELEGATION_DISCRIMINATOR,
    };
    use zama_solana_acl::{encrypted_value_discriminator, EncryptedValue};

    const PROGRAM_ID: [u8; 32] = [7; 32];
    const SUBJECT: [u8; 32] = [0x11; 32];
    const DELEGATE: [u8; 32] = [0x22; 32];
    const AUTHORITY: [u8; 32] = [0x33; 32];
    const SLOT: u64 = 500;

    fn encrypted_value_account() -> RawAccount {
        let value = EncryptedValue {
            domain: [1; 32],
            encrypted_value_account_authority: AUTHORITY,
            label: [2; 32],
            current_handle: [3; 32],
            subjects: vec![SUBJECT],
            leaf_count: 0,
            peaks: vec![],
            bump: 254,
        };
        let mut data = encrypted_value_discriminator().to_vec();
        borsh::BorshSerialize::serialize(&value, &mut data).expect("serializes");
        RawAccount {
            owner: PROGRAM_ID,
            data,
        }
    }

    fn fixture_encrypted_value_id() -> [u8; 32] {
        EncryptedValue {
            domain: [1; 32],
            encrypted_value_account_authority: AUTHORITY,
            label: [2; 32],
            ..Default::default()
        }
        .encrypted_value_id()
    }

    fn row(record: &UserDecryptionDelegationRecord) -> RawAccount {
        let mut data = USER_DECRYPTION_DELEGATION_DISCRIMINATOR.to_vec();
        data.extend_from_slice(&record.delegator);
        data.extend_from_slice(&record.delegate);
        data.extend_from_slice(&record.encrypted_value_account_authority);
        data.extend_from_slice(&record.expiration_slot.to_le_bytes());
        data.extend_from_slice(&record.delegation_counter.to_le_bytes());
        data.extend_from_slice(&record.last_update_slot.to_le_bytes());
        data.push(record.revoked as u8);
        data.push(record.bump);
        RawAccount {
            owner: PROGRAM_ID,
            data,
        }
    }

    fn live_exact() -> UserDecryptionDelegationRecord {
        UserDecryptionDelegationRecord {
            delegator: SUBJECT,
            delegate: DELEGATE,
            encrypted_value_account_authority: AUTHORITY,
            expiration_slot: SLOT + 100,
            delegation_counter: 1,
            last_update_slot: SLOT - 10,
            revoked: false,
            bump: 250,
        }
    }

    fn live_wildcard() -> UserDecryptionDelegationRecord {
        UserDecryptionDelegationRecord {
            encrypted_value_account_authority: WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY,
            ..live_exact()
        }
    }

    fn verdict(
        encrypted_value_account: Option<&RawAccount>,
        exact_row: Option<&RawAccount>,
        wildcard_row: Option<&RawAccount>,
    ) -> EntryVerdict {
        entry_verdict(&EntryVerdictInputs {
            program_id: PROGRAM_ID,
            encrypted_value_id: fixture_encrypted_value_id(),
            subject: SUBJECT,
            delegate: DELEGATE,
            encrypted_value_account,
            exact_row,
            wildcard_row,
            slot: SLOT,
        })
    }

    #[test]
    fn a_live_authority_specific_row_allows() {
        let value = encrypted_value_account();
        let exact = row(&live_exact());
        assert_eq!(
            verdict(Some(&value), Some(&exact), None),
            EntryVerdict::Allowed
        );
    }

    #[test]
    fn a_live_wildcard_row_allows_when_the_exact_row_is_dead() {
        let value = encrypted_value_account();
        let mut revoked = live_exact();
        revoked.revoked = true;
        let exact = row(&revoked);
        let wildcard = row(&live_wildcard());
        assert_eq!(
            verdict(Some(&value), Some(&exact), Some(&wildcard)),
            EntryVerdict::Allowed
        );
    }

    #[test]
    fn the_expiration_slot_itself_is_still_live() {
        let value = encrypted_value_account();
        let mut boundary = live_exact();
        boundary.expiration_slot = SLOT;
        let exact = row(&boundary);
        assert_eq!(
            verdict(Some(&value), Some(&exact), None),
            EntryVerdict::Allowed
        );
    }

    #[test]
    fn a_revoked_exact_row_with_no_wildcard_refuses() {
        let value = encrypted_value_account();
        let mut revoked = live_exact();
        revoked.revoked = true;
        let exact = row(&revoked);
        assert!(matches!(
            verdict(Some(&value), Some(&exact), None),
            EntryVerdict::NotAllowed { .. }
        ));
    }

    #[test]
    fn an_expired_exact_row_beside_a_revoked_wildcard_refuses() {
        let value = encrypted_value_account();
        let mut expired = live_exact();
        expired.expiration_slot = SLOT - 1;
        let mut revoked_wildcard = live_wildcard();
        revoked_wildcard.revoked = true;
        let exact = row(&expired);
        let wildcard = row(&revoked_wildcard);
        assert!(matches!(
            verdict(Some(&value), Some(&exact), Some(&wildcard)),
            EntryVerdict::NotAllowed { .. }
        ));
    }

    #[test]
    fn no_row_at_all_refuses() {
        let value = encrypted_value_account();
        assert!(matches!(
            verdict(Some(&value), None, None),
            EntryVerdict::NotAllowed { .. }
        ));
    }

    #[test]
    fn an_absent_encrypted_value_account_is_indeterminate() {
        assert_eq!(verdict(None, None, None), EntryVerdict::Indeterminate);
    }

    #[test]
    fn a_foreign_owned_encrypted_value_account_is_indeterminate() {
        let mut value = encrypted_value_account();
        value.owner = [9; 32];
        assert_eq!(
            verdict(Some(&value), None, None),
            EntryVerdict::Indeterminate
        );
    }

    #[test]
    fn a_encrypted_value_account_that_derives_another_id_is_indeterminate() {
        let value = encrypted_value_account();
        let verdict = entry_verdict(&EntryVerdictInputs {
            program_id: PROGRAM_ID,
            encrypted_value_id: [0xaa; 32],
            subject: SUBJECT,
            delegate: DELEGATE,
            encrypted_value_account: Some(&value),
            exact_row: None,
            wildcard_row: None,
            slot: SLOT,
        });
        assert_eq!(verdict, EntryVerdict::Indeterminate);
    }

    #[test]
    fn an_unreadable_row_never_refuses() {
        let value = encrypted_value_account();
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
        let value = encrypted_value_account();
        let mut stranger = live_exact();
        stranger.delegate = [0x99; 32];
        stranger.revoked = true;
        let exact = row(&stranger);
        assert_eq!(
            verdict(Some(&value), Some(&exact), None),
            EntryVerdict::Indeterminate
        );
    }

    // ---- the plan and the pairing ----

    fn encrypted_value_account_for(authority: [u8; 32], label: [u8; 32]) -> (RawAccount, [u8; 32]) {
        let value = EncryptedValue {
            domain: [1; 32],
            encrypted_value_account_authority: authority,
            label,
            current_handle: [3; 32],
            subjects: vec![SUBJECT],
            leaf_count: 0,
            peaks: vec![],
            bump: 254,
        };
        let id = value.encrypted_value_id();
        let mut data = encrypted_value_discriminator().to_vec();
        borsh::BorshSerialize::serialize(&value, &mut data).expect("serializes");
        (
            RawAccount {
                owner: PROGRAM_ID,
                data,
            },
            id,
        )
    }

    fn entry(handle: &str, id: [u8; 32]) -> DelegatedEntry {
        DelegatedEntry {
            handle_hex: handle.to_string(),
            subject: SUBJECT,
            encrypted_value_id: id,
        }
    }

    /// An unjudgeable entry between two judgeable ones drops out of the plan without
    /// shifting the pairing: the surviving entries keep the row addresses of their own
    /// tuples, derived from their own authorities.
    #[test]
    fn the_plan_drops_unjudgeable_entries_without_shifting_the_pairing() {
        let (value_a, id_a) = encrypted_value_account_for([0x41; 32], [0xa1; 32]);
        let (value_c, id_c) = encrypted_value_account_for([0x43; 32], [0xa3; 32]);

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

        let rows_a = solana_delegation_row_addresses(&SUBJECT, &DELEGATE, &[0x41; 32], PROGRAM_ID);
        let rows_c = solana_delegation_row_addresses(&SUBJECT, &DELEGATE, &[0x43; 32], PROGRAM_ID);
        assert_eq!(
            plan.addresses,
            vec![rows_a.0, rows_a.1, rows_c.0, rows_c.1],
            "each surviving entry keeps its own rows, authority-specific before wildcard"
        );
    }

    /// A refusal lands on the entry whose rows are dead, wherever it sits in the batch.
    #[test]
    fn a_refusal_is_attributed_to_the_entry_the_rows_belong_to() {
        let value = encrypted_value_account();
        let doomed = PlannedEntry {
            handle_hex: "0xdead".to_string(),
            subject: SUBJECT,
            encrypted_value_id: fixture_encrypted_value_id(),
            encrypted_value_account: value.clone(),
        };
        let granted = PlannedEntry {
            handle_hex: "0xa11e".to_string(),
            subject: SUBJECT,
            encrypted_value_id: fixture_encrypted_value_id(),
            encrypted_value_account: value,
        };
        let live_row = row(&live_exact());

        // (doomed, granted): rows [None, None, live, None].
        let refusals = judge_planned_entries(
            PROGRAM_ID,
            DELEGATE,
            &[doomed, granted],
            &[None, None, Some(live_row.clone()), None],
            SLOT,
        )
        .expect("a matching row count judges");
        assert_eq!(refusals.len(), 1);
        assert_eq!(refusals[0].handle_hex, "0xdead");

        // Mirrored order: the refusal follows the entry, not the position.
        let value = encrypted_value_account();
        let granted = PlannedEntry {
            handle_hex: "0xa11e".to_string(),
            subject: SUBJECT,
            encrypted_value_id: fixture_encrypted_value_id(),
            encrypted_value_account: value.clone(),
        };
        let doomed = PlannedEntry {
            handle_hex: "0xdead".to_string(),
            subject: SUBJECT,
            encrypted_value_id: fixture_encrypted_value_id(),
            encrypted_value_account: value,
        };
        let refusals = judge_planned_entries(
            PROGRAM_ID,
            DELEGATE,
            &[granted, doomed],
            &[Some(live_row), None, None, None],
            SLOT,
        )
        .expect("a matching row count judges");
        assert_eq!(refusals.len(), 1);
        assert_eq!(refusals[0].handle_hex, "0xdead");
    }

    /// Pins the exact-then-wildcard interleaving: a revoked record in the authority-specific
    /// position refuses (both rows dead), while the same world read with the positions
    /// swapped would be indeterminate — the record names the wrong tuple for the wildcard
    /// position — and refuse nothing.
    #[test]
    fn the_authority_specific_row_is_read_before_the_wildcard_row() {
        let mut revoked = live_exact();
        revoked.revoked = true;
        let planned = PlannedEntry {
            handle_hex: "0xdead".to_string(),
            subject: SUBJECT,
            encrypted_value_id: fixture_encrypted_value_id(),
            encrypted_value_account: encrypted_value_account(),
        };

        let refusals = judge_planned_entries(
            PROGRAM_ID,
            DELEGATE,
            &[planned],
            &[Some(row(&revoked)), None],
            SLOT,
        )
        .expect("a matching row count judges");
        assert_eq!(refusals.len(), 1, "revoked exact row beside no wildcard row");
    }

    /// A row count that does not match the plan is a defect, never a misattribution — and a
    /// encrypted-value-account count that does not match the entries likewise.
    #[test]
    fn mismatched_account_counts_are_defects_not_verdicts() {
        let planned = PlannedEntry {
            handle_hex: "0xdead".to_string(),
            subject: SUBJECT,
            encrypted_value_id: fixture_encrypted_value_id(),
            encrypted_value_account: encrypted_value_account(),
        };
        assert_eq!(
            judge_planned_entries(PROGRAM_ID, DELEGATE, &[planned], &[None], SLOT).err(),
            Some(PairingDefect::RowAccounts {
                entries: 1,
                accounts: 1
            })
        );

        let (_, id) = encrypted_value_account_for([0x41; 32], [0xa1; 32]);
        assert_eq!(
            plan_row_reads(PROGRAM_ID, DELEGATE, vec![entry("0xaa", id)], vec![]).err(),
            Some(PairingDefect::EncryptedValueAccounts {
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
            "6AtbvED1rfX68aCT1tYgU1aeu4kFksPDxZG9gtB1Fgtu",
        )
        .expect("the zama-host program id");
        let as_base58 =
            |address: [u8; 32]| solana_pubkey::Pubkey::new_from_array(address).to_string();

        let (exact_row, wildcard_row) =
            solana_delegation_row_addresses(&[0x11; 32], &[0x22; 32], &[0x33; 32], program_id);
        assert_eq!(
            as_base58(exact_row),
            "5bK6ZBSpCgC13c5JT5g2LHjRfTjBM6Fjaybcv8tQqUUX"
        );
        assert_eq!(
            as_base58(wildcard_row),
            "DjwWqTLQmSDxxCEXS8KmJBqvvmjhYTWKGsZyh343cKJJ"
        );

        assert_eq!(
            as_base58(solana_encrypted_value_address(&[0x55; 32], program_id)),
            "5K29xw8jynL8Vw63cRm6cUeQK1dfs5M2Vx3r5inwos5p"
        );
    }
}
