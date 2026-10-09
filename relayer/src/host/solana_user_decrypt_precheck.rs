//! Advisory, negative-only pre-check of a Solana user-decrypt request, the Solana analogue of the
//! EVM host-ACL pre-check: it saves a gateway transaction the KMS connectors would refuse. The
//! connectors' check stays authoritative, so this one only refuses what they cannot authorize:
//!
//! * it refuses an entry whose encrypted store's application `(program, scope)` the permit's
//!   allowed scopes do not cover, as EVM's `Decryption.sol` refuses a contract outside the signed
//!   contract addresses before the fee. A store's application is its seeds, so no later read can
//!   change this answer;
//! * it refuses a delegated entry whose delegation rows are both dead (absent, revoked, or expired
//!   at the host's Clock) in this read. A lagging node can show a fresh grant as absent; the caller
//!   then resubmits, the same exposure the EVM pre-check accepts;
//! * any account it cannot judge passes: an invalid row, a misshapen account, an unresolvable
//!   encrypted store;
//! * a transport failure refuses (`HostAclError::CallFailed`), as on EVM. A node behind the first
//!   read's slot is not a failure: the row read requires that slot (`minContextSlot`) and passes if
//!   the node never reaches it.
//!
//! The allow leaf of a direct entry is not pre-checked: it comes from the coprocessors, and the
//! connector reads it no more cheaply than this would.
//!
//! The store validation and the row verdicts are `zama-solana-acl`'s, and the scope rule is
//! `zama-solana-permit`'s: the functions the connector judges the same bytes with. Everything but
//! the two RPC reads in `acl_checker` is pure here.

use solana_pubkey::Pubkey;
use zama_solana_acl::{
    decode_clock_unix_timestamp, find_delegation_record_address, judge_delegation,
    judge_delegation_row, validate_store, AccountView, DelegationVerdict, CLOCK_SYSVAR_ID,
    WILDCARD_APP,
};
use zama_solana_permit::{AllowedScopes, Identity};

/// One fetched account, exactly as the RPC returned it.
#[derive(Clone, Debug)]
pub(crate) struct RawAccount {
    pub owner: [u8; 32],
    pub data: Vec<u8>,
}

impl RawAccount {
    fn view(&self) -> AccountView<'_> {
        AccountView {
            owner: &self.owner,
            data: &self.data,
        }
    }
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

/// A delegation row's address, the canonical bump that derives it, and the application it is
/// keyed by: what its record must name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct RowAddress {
    pub address: [u8; 32],
    pub bump: u8,
    pub app: AppScope,
}

/// The application of the encrypted store at `encrypted_store_key`, when the account is a store
/// the host program wrote there (`zama_solana_acl::validate_store`, the connector's rule). `None`
/// is "this advisory check cannot judge the encrypted store" — never a refusal.
pub(crate) fn store_application(
    program_id: [u8; 32],
    encrypted_store_key: [u8; 32],
    encrypted_store: Option<&RawAccount>,
) -> Option<AppScope> {
    validate_store(
        &program_id,
        &encrypted_store_key,
        encrypted_store.map(RawAccount::view),
    )
    .ok()
    .map(|store| AppScope {
        program: store.program,
        scope: store.scope,
    })
}

/// Decides one delegated entry from its two rows, application row first, at the host's Clock
/// `now`. The rows are judged by the connector's rules; only the policy on a row the host program
/// could not have written differs: the connector refuses it, this advisory check stands aside.
pub(crate) fn entry_verdict(
    program_id: [u8; 32],
    delegator: [u8; 32],
    delegate: [u8; 32],
    rows: &[RowAddress; 2],
    accounts: [Option<&RawAccount>; 2],
    now: u64,
) -> EntryVerdict {
    let [exact, wildcard] = [0, 1].map(|i| {
        let row = rows[i];
        judge_delegation_row(
            &program_id,
            accounts[i].map(RawAccount::view),
            row.bump,
            [&delegator, &delegate, &row.app.program, &row.app.scope],
            now,
        )
    });
    match judge_delegation(exact, wildcard) {
        DelegationVerdict::Authorized(_) => EntryVerdict::Allowed,
        DelegationVerdict::NoLiveDelegation { exact, wildcard } => EntryVerdict::NotAllowed {
            reason: format!(
                "no live delegation of (delegator, delegate, application) at the host's clock: \
                 application row {exact}; wildcard row {wildcard}"
            ),
        },
        DelegationVerdict::InvalidRow(_) => EntryVerdict::Indeterminate,
    }
}

/// One entry as admission handed it over: the claimed identities, plus the handle for refusal
/// attribution.
pub(crate) struct RequestEntry {
    pub handle_hex: String,
    /// The entry's owner address: the requester on a direct entry, the delegator on a delegated
    /// one.
    pub owner: [u8; 32],
    pub encrypted_store: [u8; 32],
}

/// A delegated entry whose encrypted store resolved to an application, with the two rows that
/// application gives it.
pub(crate) struct PlannedEntry {
    pub handle_hex: String,
    pub delegator: [u8; 32],
    pub rows: [RowAddress; 2],
}

/// What the encrypted-store read planned for the row read: `entries[i]`'s rows sit at
/// `addresses[2i]` (application) and `addresses[2i + 1]` (wildcard), and the Clock sysvar is
/// last — built by [`plan_row_reads`], consumed by [`judge_planned_entries`]. The Clock rides in
/// the same read so liveness is judged at the time of the slot the rows were read at.
pub(crate) struct RowReadPlan {
    pub entries: Vec<PlannedEntry>,
    pub addresses: Vec<[u8; 32]>,
}

/// One refused entry: the handle and the reason the connectors cannot authorize it.
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

/// The addresses of the encrypted-store read, one per entry in order: the entry names the
/// account by address, and the account's own fields are checked against it once read.
pub(crate) fn encrypted_store_read_addresses(entries: &[RequestEntry]) -> Vec<[u8; 32]> {
    entries.iter().map(|entry| entry.encrypted_store).collect()
}

/// The application of each entry's encrypted store, in entry order ([`store_application`]; `None`
/// where this check cannot judge the store).
pub(crate) fn entry_applications(
    program_id: [u8; 32],
    entries: &[RequestEntry],
    encrypted_stores: Vec<Option<RawAccount>>,
) -> Result<Vec<Option<AppScope>>, ReadDefect> {
    if encrypted_stores.len() != entries.len() {
        return Err(ReadDefect::EncryptedStores {
            entries: entries.len(),
            accounts: encrypted_stores.len(),
        });
    }
    Ok(entries
        .iter()
        .zip(encrypted_stores)
        .map(|(entry, encrypted_store)| {
            store_application(program_id, entry.encrypted_store, encrypted_store.as_ref())
        })
        .collect())
}

/// The entries whose store's application the permit's allowed scopes do not cover, by the rule
/// the connector applies (`AllowedScopes::admits`). An entry whose store this check cannot judge
/// is not refused.
pub(crate) fn scope_refusals(
    scopes: &AllowedScopes,
    entries: &[RequestEntry],
    applications: &[Option<AppScope>],
) -> Vec<EntryRefusal> {
    entries
        .iter()
        .zip(applications)
        .filter_map(|(entry, application)| {
            let app = (*application)?;
            (!scopes.admits(&Identity::new(app.program), &Identity::new(app.scope))).then(|| {
                EntryRefusal {
                    handle_hex: entry.handle_hex.clone(),
                    reason: format!(
                        "the permit's allowed scopes do not cover the encrypted store's \
                         application (program {}, scope {})",
                        Pubkey::new_from_array(app.program),
                        Pubkey::new_from_array(app.scope),
                    ),
                }
            })
        })
        .collect()
}

/// Plans the row read for the delegated entries, those whose owner is not `delegate`. A delegated
/// entry whose encrypted store this check cannot judge drops out here (indeterminate — the
/// connector decides); each surviving entry contributes its two row addresses in the interleaving
/// [`RowReadPlan`] documents.
pub(crate) fn plan_row_reads(
    program_id: [u8; 32],
    delegate: [u8; 32],
    entries: Vec<RequestEntry>,
    applications: Vec<Option<AppScope>>,
) -> RowReadPlan {
    let mut plan = RowReadPlan {
        entries: Vec::new(),
        addresses: Vec::new(),
    };
    for (entry, application) in entries.into_iter().zip(applications) {
        let Some(app) = application.filter(|_| entry.owner != delegate) else {
            continue;
        };
        let rows = delegation_rows(&entry.owner, &delegate, app, program_id);
        plan.addresses.extend(rows.map(|row| row.address));
        plan.entries.push(PlannedEntry {
            handle_hex: entry.handle_hex,
            delegator: entry.owner,
            rows,
        });
    }
    plan.addresses.push(CLOCK_SYSVAR_ID);
    plan
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
            match entry_verdict(
                program_id,
                entry.delegator,
                delegate,
                &entry.rows,
                [rows[0].as_ref(), rows[1].as_ref()],
                now,
            ) {
                EntryVerdict::Allowed | EntryVerdict::Indeterminate => None,
                EntryVerdict::NotAllowed { reason } => Some(EntryRefusal {
                    handle_hex: entry.handle_hex.clone(),
                    reason,
                }),
            }
        })
        .collect())
}

/// Both delegation rows of one `(delegator, delegate)` couple: the application's row and the
/// wildcard row it falls back to. The seed tuple is pinned against the host program's own
/// derivation by fixture literals shared with the runtime-test SDK cross-pins.
fn delegation_rows(
    delegator: &[u8; 32],
    delegate: &[u8; 32],
    app: AppScope,
    program_id: [u8; 32],
) -> [RowAddress; 2] {
    [app, AppScope::WILDCARD].map(|app| {
        let (address, bump) = find_delegation_record_address(
            &program_id,
            delegator,
            delegate,
            &app.program,
            &app.scope,
        );
        RowAddress { address, bump, app }
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use zama_solana_acl::{
        encode_clock, encode_user_decryption_delegation, encrypted_store_discriminator,
        EncryptedStore, UserDecryptionDelegationRecord, SYSTEM_PROGRAM_ID, SYSVAR_OWNER_ID,
    };

    pub(crate) const PROGRAM_ID: [u8; 32] = [7; 32];
    pub(crate) const APP_PROGRAM: [u8; 32] = [1; 32];
    pub(crate) const SCOPE: [u8; 32] = [4; 32];
    const APP: AppScope = AppScope {
        program: APP_PROGRAM,
        scope: SCOPE,
    };
    pub(crate) const DELEGATOR: [u8; 32] = [0x11; 32];
    pub(crate) const DELEGATE: [u8; 32] = [0x22; 32];
    const AUTHORITY: [u8; 32] = [0x33; 32];
    pub(crate) const NOW: u64 = 1_700_000_000;

    /// An encrypted store of `(program, scope)` at the address its fields derive.
    pub(crate) fn encrypted_store_for(
        program: [u8; 32],
        scope: [u8; 32],
    ) -> (RawAccount, [u8; 32]) {
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

    fn rows() -> [RowAddress; 2] {
        delegation_rows(&DELEGATOR, &DELEGATE, APP, PROGRAM_ID)
    }

    pub(crate) fn row(record: &UserDecryptionDelegationRecord) -> RawAccount {
        RawAccount {
            owner: PROGRAM_ID,
            data: encode_user_decryption_delegation(record),
        }
    }

    pub(crate) fn clock(unix_timestamp: u64) -> RawAccount {
        RawAccount {
            owner: SYSVAR_OWNER_ID,
            data: encode_clock(unix_timestamp),
        }
    }

    /// The record the host writes at `row`, live until `NOW + 100`.
    fn live(row: RowAddress) -> UserDecryptionDelegationRecord {
        UserDecryptionDelegationRecord {
            delegator: DELEGATOR,
            delegate: DELEGATE,
            program: row.app.program,
            scope: row.app.scope,
            expires_at: NOW + 100,
            delegation_counter: 1,
            last_update_slot: 490,
            bump: row.bump,
        }
    }

    pub(crate) fn live_exact() -> UserDecryptionDelegationRecord {
        live(rows()[0])
    }

    fn live_wildcard() -> UserDecryptionDelegationRecord {
        live(rows()[1])
    }

    fn revoked(record: UserDecryptionDelegationRecord) -> UserDecryptionDelegationRecord {
        UserDecryptionDelegationRecord {
            expires_at: 0,
            ..record
        }
    }

    fn verdict(exact_row: Option<&RawAccount>, wildcard_row: Option<&RawAccount>) -> EntryVerdict {
        entry_verdict(
            PROGRAM_ID,
            DELEGATOR,
            DELEGATE,
            &rows(),
            [exact_row, wildcard_row],
            NOW,
        )
    }

    fn refusal(exact: &str, wildcard: &str) -> EntryVerdict {
        EntryVerdict::NotAllowed {
            reason: format!(
                "no live delegation of (delegator, delegate, application) at the host's clock: \
                 application row {exact}; wildcard row {wildcard}"
            ),
        }
    }

    #[test]
    fn a_live_row_of_the_stores_application_allows() {
        assert_eq!(
            verdict(Some(&row(&live_exact())), None),
            EntryVerdict::Allowed
        );
    }

    #[test]
    fn a_live_wildcard_row_allows_when_the_exact_row_is_dead() {
        let exact = row(&revoked(live_exact()));
        let wildcard = row(&live_wildcard());
        assert_eq!(
            verdict(Some(&exact), Some(&wildcard)),
            EntryVerdict::Allowed
        );
    }

    /// `expires_at` is exclusive, as EVM's `expirationDate > block.timestamp`: the last live
    /// second is the one before it.
    #[test]
    fn a_delegation_ends_at_its_expiry_second() {
        let ending_at = |expires_at| {
            row(&UserDecryptionDelegationRecord {
                expires_at,
                ..live_exact()
            })
        };
        assert_eq!(
            verdict(Some(&ending_at(NOW + 1)), None),
            EntryVerdict::Allowed
        );
        assert_eq!(
            verdict(Some(&ending_at(NOW)), None),
            refusal(&format!("expired at {NOW}"), "absent")
        );
    }

    #[test]
    fn two_dead_rows_refuse() {
        let revoked_exact = row(&revoked(live_exact()));
        let revoked_wildcard = row(&revoked(live_wildcard()));
        assert_eq!(
            verdict(Some(&revoked_exact), None),
            refusal("revoked", "absent")
        );
        assert_eq!(
            verdict(None, Some(&revoked_wildcard)),
            refusal("absent", "revoked")
        );
        assert_eq!(verdict(None, None), refusal("absent", "absent"));
    }

    /// Anyone can fund a derivable address before the host creates the row there; the host reads
    /// that account as no row, and so does this check, as the connector does.
    #[test]
    fn a_funded_empty_row_address_is_an_absent_row() {
        let funded = RawAccount {
            owner: SYSTEM_PROGRAM_ID,
            data: vec![],
        };
        assert_eq!(verdict(Some(&funded), None), refusal("absent", "absent"));
    }

    /// A row the host program could not have written is the connector's to refuse: this check
    /// stands aside, even beside a live row.
    #[test]
    fn a_row_the_host_could_not_have_written_never_refuses() {
        let garbage = RawAccount {
            owner: PROGRAM_ID,
            data: vec![0xde, 0xad],
        };
        let foreign = RawAccount {
            owner: [9; 32],
            ..row(&revoked(live_exact()))
        };
        let wrong_bump = row(&UserDecryptionDelegationRecord {
            bump: rows()[0].bump.wrapping_sub(1),
            ..revoked(live_exact())
        });
        let other_tuple = [
            UserDecryptionDelegationRecord {
                delegate: [0x99; 32],
                ..revoked(live_exact())
            },
            UserDecryptionDelegationRecord {
                scope: [0x99; 32],
                ..revoked(live_exact())
            },
        ]
        .map(|record| row(&record));
        for invalid in [
            &garbage,
            &foreign,
            &wrong_bump,
            &other_tuple[0],
            &other_tuple[1],
        ] {
            assert_eq!(verdict(Some(invalid), None), EntryVerdict::Indeterminate);
            assert_eq!(
                verdict(Some(invalid), Some(&row(&live_wildcard()))),
                EntryVerdict::Indeterminate
            );
        }
    }

    #[test]
    fn only_a_store_the_host_wrote_there_has_an_application() {
        let (store, key) = encrypted_store_for(APP_PROGRAM, SCOPE);
        assert_eq!(store_application(PROGRAM_ID, key, Some(&store)), Some(APP));

        assert_eq!(store_application(PROGRAM_ID, key, None), None);
        let foreign = RawAccount {
            owner: [9; 32],
            ..store.clone()
        };
        assert_eq!(store_application(PROGRAM_ID, key, Some(&foreign)), None);
        assert_eq!(
            store_application(PROGRAM_ID, [0xaa; 32], Some(&store)),
            None
        );

        // A store naming the wildcard sentinel is the connector's to reject.
        let other = [0x44; 32];
        for (program, scope) in [(WILDCARD_APP, other), (other, WILDCARD_APP)] {
            let (store, key) = encrypted_store_for(program, scope);
            assert_eq!(store_application(PROGRAM_ID, key, Some(&store)), None);
        }
    }

    // ---- the plan and the pairing ----

    fn entry(handle: &str, encrypted_store: [u8; 32]) -> RequestEntry {
        RequestEntry {
            handle_hex: handle.to_string(),
            owner: DELEGATOR,
            encrypted_store,
        }
    }

    fn plan(entries: Vec<RequestEntry>, stores: Vec<Option<RawAccount>>) -> RowReadPlan {
        let applications =
            entry_applications(PROGRAM_ID, &entries, stores).expect("aligned inputs resolve");
        plan_row_reads(PROGRAM_ID, DELEGATE, entries, applications)
    }

    fn planned(handle: &str) -> PlannedEntry {
        PlannedEntry {
            handle_hex: handle.to_string(),
            delegator: DELEGATOR,
            rows: rows(),
        }
    }

    /// An unjudgeable entry between two judgeable ones drops out of the plan without
    /// shifting the pairing: the surviving entries keep the row addresses of their own
    /// applications, and the Clock closes the read.
    #[test]
    fn the_plan_drops_unjudgeable_entries_without_shifting_the_pairing() {
        let (value_a, id_a) = encrypted_store_for(APP_PROGRAM, [0x41; 32]);
        let (value_c, id_c) = encrypted_store_for(APP_PROGRAM, [0x43; 32]);

        let plan = plan(
            vec![
                entry("0xaa", id_a),
                entry("0xbb", [0xbb; 32]),
                entry("0xcc", id_c),
            ],
            vec![Some(value_a), None, Some(value_c)],
        );

        let handles: Vec<&str> = plan
            .entries
            .iter()
            .map(|planned| planned.handle_hex.as_str())
            .collect();
        assert_eq!(handles, ["0xaa", "0xcc"]);

        let rows = |scope: [u8; 32]| {
            delegation_rows(
                &DELEGATOR,
                &DELEGATE,
                AppScope {
                    program: APP_PROGRAM,
                    scope,
                },
                PROGRAM_ID,
            )
            .map(|row| row.address)
        };
        let (rows_a, rows_c) = (rows([0x41; 32]), rows([0x43; 32]));
        assert_ne!(
            rows_a[0], rows_c[0],
            "the two applications have their own rows"
        );
        assert_eq!(
            plan.addresses,
            vec![rows_a[0], rows_a[1], rows_c[0], rows_c[1], CLOCK_SYSVAR_ID],
            "each surviving entry keeps its own rows, application before wildcard"
        );
    }

    /// A direct entry has no delegation row to read.
    #[test]
    fn the_plan_reads_rows_for_delegated_entries_only() {
        let (store, id) = encrypted_store_for(APP_PROGRAM, SCOPE);
        let direct = RequestEntry {
            owner: DELEGATE,
            ..entry("0xd1", id)
        };
        let plan = plan(
            vec![direct, entry("0xde", id)],
            vec![Some(store.clone()), Some(store)],
        );
        let handles: Vec<&str> = plan
            .entries
            .iter()
            .map(|planned| planned.handle_hex.as_str())
            .collect();
        assert_eq!(handles, ["0xde"]);
    }

    // ---- the permit's scopes ----

    fn scopes(applications: &[AppScope]) -> AllowedScopes {
        AllowedScopes::new(
            applications
                .iter()
                .map(|app| {
                    zama_solana_permit::ApplicationScope::new(
                        Identity::new(app.program),
                        Identity::new(app.scope),
                    )
                })
                .collect(),
        )
        .expect("one scope is a valid list")
    }

    /// The connector's rule (`AllowedScopes::admits`): a signed list admits exactly its pairs, an
    /// empty one admits every application, and an entry this check cannot judge is never refused.
    #[test]
    fn an_entry_is_refused_only_when_its_known_application_is_outside_signed_scopes() {
        let other = AppScope {
            program: APP_PROGRAM,
            scope: [0x99; 32],
        };
        let entries = [entry("0xaa", [0xaa; 32]), entry("0xbb", [0xbb; 32])];
        let refused = |allowed: &[AppScope], applications: &[Option<AppScope>]| {
            scope_refusals(&scopes(allowed), &entries, applications)
                .into_iter()
                .map(|refusal| refusal.handle_hex)
                .collect::<Vec<_>>()
        };
        assert_eq!(refused(&[APP], &[Some(APP), Some(other)]), ["0xbb"]);
        assert_eq!(refused(&[other], &[Some(APP), Some(other)]), ["0xaa"]);
        assert!(refused(&[], &[Some(APP), Some(other)]).is_empty());
        assert!(refused(&[other], &[None, Some(other)]).is_empty());
    }

    /// A refusal lands on the entry whose rows are dead, wherever it sits in the batch.
    #[test]
    fn a_refusal_is_attributed_to_the_entry_the_rows_belong_to() {
        let live_row = Some(row(&live_exact()));
        let now = Some(clock(NOW));

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
        let now = Some(clock(NOW));

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
                &[Some(row(&live_exact())), None, Some(clock(unix))],
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
        let mut foreign = clock(NOW);
        foreign.owner = PROGRAM_ID;
        let mut short = clock(NOW);
        short.data.pop();
        // The Clock's time is an `i64`: these bytes read as -1.
        let negative = clock(u64::MAX);
        for bad_clock in [None, Some(foreign), Some(short), Some(negative)] {
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
            entry_applications(PROGRAM_ID, &[entry("0xaa", id)], vec![]).err(),
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

        let [exact_row, wildcard_row] = delegation_rows(
            &[0x11; 32],
            &[0x22; 32],
            AppScope {
                program: [0x33; 32],
                scope: [0x44; 32],
            },
            program_id,
        );
        assert_eq!(
            as_base58(exact_row.address),
            "GkmqVNMzqxopBjPkSkZvuLuDE6Jze3iA3Mq5ZHr6SrtJ"
        );
        assert_eq!(
            as_base58(wildcard_row.address),
            "J4BMamYLJvJroFATJp48L6AeQJDQqv86YAQyPqvBcKq1"
        );
    }
}
