//! Mollusk runtime tests for the user-decryption delegation instructions — grant, re-grant,
//! revoke.
//!
//! The record these instructions maintain is what the KMS Connector's delegated branch reads,
//! so nearly everything worth testing is a negative: a record another party could create,
//! refresh or keep alive against the delegator's will would let a delegate decrypt what the
//! delegator never allowed. The delegator's signature and the canonical PDA are the whole
//! authorization story on this side.
//!
//! Two behaviours are pinned deliberately rather than discovered:
//!
//! * a grant whose authority is the wildcard sentinel `[0xff; 32]` is legal and writes an
//!   ordinary record — that row is the delegator's grant across every authority of theirs,
//!   the same rule the EVM ACL applies to its wildcard delegation address;
//! * both grant and revoke are pause-gated. During an incident pause the delegator's only
//!   lever over existing delegations is blocked while those delegations keep authorizing
//!   (the Connector's reads are pause-blind) — an asymmetry shared with the EVM side and
//!   raised there as an open question; these tests pin the current parity, not endorse it.
//!
//! Build the program artifact before running these (`bash scripts/check-zama-host-idl.sh`); a
//! stale `.so` makes the whole harness fail on an unrelated missing sysvar rather than on
//! anything these tests assert.

use anchor_lang::{
    prelude::system_program, AccountDeserialize, Discriminator, InstructionData, ToAccountMetas,
};
use delegator_vault as vault;
use mollusk_svm::{result::Check, Mollusk};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use zama_host::{self as host, UserDecryptionDelegation};
use zama_solana_test_kit::{
    anchor_framework_error_check, empty_system_account, funded_system_account as funded_wallet,
    host_config_account as host_config_account_from, serialized_account as serialized,
    system_program_account, HostConfigParams,
};

// ---------------------------------------------------------------------------
// Harness
// ---------------------------------------------------------------------------

/// The slot every test runs at unless it says otherwise. Existing records are written at
/// [`EARLIER_SLOT`], so the once-per-slot guard does not interfere with tests that are not
/// about it.
const CURRENT_SLOT: u64 = 100;
const EARLIER_SLOT: u64 = 50;
/// The expiration every grant asks for, comfortably above [`CURRENT_SLOT`].
const EXPIRATION: u64 = 500;

fn mollusk_at_slot(slot: u64) -> Mollusk {
    let mut mollusk = zama_solana_test_kit::svm(&host::id(), "zama_host");
    mollusk.sysvars.clock.slot = slot;
    mollusk
}

fn mollusk() -> Mollusk {
    mollusk_at_slot(CURRENT_SLOT)
}

fn anchor_ix<A, D>(accounts: A, args: D) -> Instruction
where
    A: ToAccountMetas,
    D: InstructionData,
{
    zama_solana_test_kit::anchor_ix(host::id(), accounts, args)
}

fn custom_error(error: host::errors::ZamaHostError) -> Check<'static> {
    zama_solana_test_kit::anchor_error_check(error as u32)
}

fn host_config_account(paused: bool) -> (Pubkey, Account) {
    host_config_account_from(&HostConfigParams {
        paused,
        ..HostConfigParams::new(Pubkey::new_unique())
    })
}

fn grant_ix(
    payer: Pubkey,
    delegator: Pubkey,
    delegation_record: Pubkey,
    delegate: Pubkey,
    encrypted_value_account_authority: Pubkey,
    expiration_slot: u64,
) -> Instruction {
    let (host_config, _) = host::host_config_address();
    anchor_ix(
        host::accounts::DelegateForUserDecryption {
            payer,
            delegator,
            host_config,
            delegation_record,
            system_program: system_program::ID,
        },
        host::instruction::DelegateForUserDecryption {
            delegate,
            encrypted_value_account_authority,
            expiration_slot,
        },
    )
}

fn revoke_ix(delegator: Pubkey, delegation_record: Pubkey) -> Instruction {
    let (host_config, _) = host::host_config_address();
    anchor_ix(
        host::accounts::RevokeDelegationForUserDecryption {
            delegator,
            host_config,
            delegation_record,
        },
        host::instruction::RevokeDelegationForUserDecryption {},
    )
}

/// The tuple every test grants over, with a payer who is not the delegator: paying rent and
/// granting are different roles, and keeping them different keys in every test means a handler
/// that conflated them would fail here.
struct Actors {
    payer: Pubkey,
    delegator: Pubkey,
    delegate: Pubkey,
    authority: Pubkey,
    record_key: Pubkey,
    record_bump: u8,
}

fn actors() -> Actors {
    let delegator = Pubkey::new_unique();
    let delegate = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let (record_key, record_bump) =
        host::user_decryption_delegation_address(delegator, delegate, authority);
    Actors {
        payer: Pubkey::new_unique(),
        delegator,
        delegate,
        authority,
        record_key,
        record_bump,
    }
}

/// A live record of the actors' tuple, written at [`EARLIER_SLOT`] with counter 1 — what the
/// first grant produces, aged by one re-grantable slot gap.
fn live_record(actors: &Actors) -> UserDecryptionDelegation {
    UserDecryptionDelegation {
        delegator: actors.delegator,
        delegate: actors.delegate,
        encrypted_value_account_authority: actors.authority,
        expiration_slot: EXPIRATION,
        delegation_counter: 1,
        last_update_slot: EARLIER_SLOT,
        revoked: false,
        bump: actors.record_bump,
    }
}

fn record_account(record: &UserDecryptionDelegation) -> Account {
    Account {
        lamports: 1_000_000_000,
        data: serialized(record.clone()),
        owner: host::id(),
        executable: false,
        rent_epoch: 0,
    }
}

/// The five accounts a grant takes, with the record slot holding `record_account`.
fn grant_accounts(actors: &Actors, record: Account, paused: bool) -> Vec<(Pubkey, Account)> {
    let (host_config, host_config_account) = host_config_account(paused);
    vec![
        (actors.payer, funded_wallet()),
        (actors.delegator, funded_wallet()),
        (host_config, host_config_account),
        (actors.record_key, record),
        (system_program::ID, system_program_account()),
    ]
}

/// The three accounts a revoke takes.
fn revoke_accounts(actors: &Actors, record: Account, paused: bool) -> Vec<(Pubkey, Account)> {
    let (host_config, host_config_account) = host_config_account(paused);
    vec![
        (actors.delegator, funded_wallet()),
        (host_config, host_config_account),
        (actors.record_key, record),
    ]
}

fn decode_record(data: &[u8]) -> UserDecryptionDelegation {
    let mut slice: &[u8] = data;
    UserDecryptionDelegation::try_deserialize(&mut slice)
        .expect("the resulting account decodes as a delegation record")
}

// ---------------------------------------------------------------------------
// Grant
// ---------------------------------------------------------------------------

/// The first grant creates the record at the canonical PDA and writes every field the
/// Connector will later read. The payer is not the delegator: rent and consent are separate
/// roles.
#[test]
fn a_grant_creates_the_record_at_the_canonical_pda() {
    let actors = actors();
    let accounts = grant_accounts(&actors, empty_system_account(), false);

    let result = mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            actors.record_key,
            actors.delegate,
            actors.authority,
            EXPIRATION,
        ),
        &accounts,
        &[Check::success()],
    );

    let created = result
        .get_account(&actors.record_key)
        .expect("the record exists after the grant");
    assert_eq!(created.owner, host::id(), "the record is program-owned");
    assert_eq!(
        created.data.len(),
        8 + UserDecryptionDelegation::SPACE,
        "the record is exactly the declared size"
    );
    assert_eq!(
        &created.data[..8],
        UserDecryptionDelegation::DISCRIMINATOR,
        "the record carries the delegation discriminator"
    );
    let record = decode_record(&created.data);
    assert_eq!(record.delegator, actors.delegator);
    assert_eq!(record.delegate, actors.delegate);
    assert_eq!(record.encrypted_value_account_authority, actors.authority);
    assert_eq!(record.expiration_slot, EXPIRATION);
    assert_eq!(record.delegation_counter, 1, "the first grant counts one");
    assert_eq!(record.last_update_slot, CURRENT_SLOT);
    assert!(!record.revoked);
    assert_eq!(record.bump, actors.record_bump);
}

/// The common wallet path: one key pays and grants. The instruction then carries the same
/// pubkey as two metas — a writable signer (payer) and a readonly signer (delegator) — which
/// is exactly the account-dedup shape a runtime or framework regression would trip on, and
/// which every other test here misses on purpose (they split the roles to catch conflation).
#[test]
fn a_grant_whose_payer_is_the_delegator_creates_the_record() {
    let actors = actors();
    let (host_config, host_config_acc) = host_config_account(false);

    let result = mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.delegator,
            actors.delegator,
            actors.record_key,
            actors.delegate,
            actors.authority,
            EXPIRATION,
        ),
        &[
            (actors.delegator, funded_wallet()),
            (host_config, host_config_acc),
            (actors.record_key, empty_system_account()),
            (system_program::ID, system_program_account()),
        ],
        &[Check::success()],
    );

    let created = result
        .get_account(&actors.record_key)
        .expect("the record exists after the self-paid grant");
    let record = decode_record(&created.data);
    assert_eq!(record.delegator, actors.delegator);
    assert_eq!(record.delegation_counter, 1);
}

/// The record's seeds are public, so a third party can pre-fund the address with a bare
/// transfer before the delegator's first grant. The creation path tolerates the donation
/// (fund/allocate/assign, not `create_account`) — otherwise a dusting transfer would block the
/// tuple's delegation permanently.
#[test]
fn a_grant_survives_a_record_address_prefunded_by_a_third_party() {
    let actors = actors();
    let prefunded = Account {
        lamports: 5_000_000_000,
        data: vec![],
        owner: system_program::ID,
        executable: false,
        rent_epoch: 0,
    };
    let accounts = grant_accounts(&actors, prefunded, false);

    let result = mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            actors.record_key,
            actors.delegate,
            actors.authority,
            EXPIRATION,
        ),
        &accounts,
        &[Check::success()],
    );

    let created = result
        .get_account(&actors.record_key)
        .expect("the record exists after the grant");
    assert_eq!(created.owner, host::id(), "the donation did not block adoption");
    let record = decode_record(&created.data);
    assert_eq!(record.delegation_counter, 1, "this is a first grant, not a re-grant");
    assert!(
        created.lamports >= 5_000_000_000,
        "the donation stays on the record (more-than-rent-exempt is harmless)"
    );
}

/// An account owned by another program squatting the canonical record address cannot be
/// adopted: the grant refuses instead of writing a record over foreign bytes or trusting
/// whatever they hold.
#[test]
fn a_grant_refuses_a_record_address_owned_by_a_foreign_program() {
    let actors = actors();
    let foreign = Account {
        lamports: 1_000_000_000,
        data: vec![0; 16],
        owner: Pubkey::new_unique(),
        executable: false,
        rent_epoch: 0,
    };
    let accounts = grant_accounts(&actors, foreign, false);

    mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            actors.record_key,
            actors.delegate,
            actors.authority,
            EXPIRATION,
        ),
        &accounts,
        &[custom_error(host::errors::ZamaHostError::PdaCreationMismatch)],
    );
}

/// A grant whose authority is the wildcard sentinel is legal and writes an ordinary record:
/// the delegator's grant across every authority of theirs. The sentinel is refused only in
/// the delegate position — a real party has to be named on the receiving end.
#[test]
fn a_wildcard_authority_grant_is_legal_and_writes_the_record() {
    let delegator = Pubkey::new_unique();
    let delegate = Pubkey::new_unique();
    let wildcard = Pubkey::new_from_array(host::WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY_BYTES);
    let (record_key, _) = host::user_decryption_delegation_address(delegator, delegate, wildcard);
    let payer = Pubkey::new_unique();
    let (host_config, host_config_acc) = host_config_account(false);

    let result = mollusk().process_and_validate_instruction(
        &grant_ix(payer, delegator, record_key, delegate, wildcard, EXPIRATION),
        &[
            (payer, funded_wallet()),
            (delegator, funded_wallet()),
            (host_config, host_config_acc),
            (record_key, empty_system_account()),
            (system_program::ID, system_program_account()),
        ],
        &[Check::success()],
    );

    let record = decode_record(&result.get_account(&record_key).expect("record").data);
    assert_eq!(record.encrypted_value_account_authority, wildcard);
    assert_eq!(record.delegation_counter, 1);
}

/// A default delegate names nobody.
#[test]
fn a_grant_to_the_default_pubkey_is_rejected() {
    let actors = actors();
    let (record_key, _) = host::user_decryption_delegation_address(
        actors.delegator,
        Pubkey::default(),
        actors.authority,
    );
    let mut accounts = grant_accounts(&actors, empty_system_account(), false);
    accounts[3].0 = record_key;

    mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            record_key,
            Pubkey::default(),
            actors.authority,
            EXPIRATION,
        ),
        &accounts,
        &[custom_error(host::errors::ZamaHostError::InvalidDelegation)],
    );
}

/// A default authority scopes nothing.
#[test]
fn a_grant_over_the_default_authority_is_rejected() {
    let actors = actors();
    let (record_key, _) = host::user_decryption_delegation_address(
        actors.delegator,
        actors.delegate,
        Pubkey::default(),
    );
    let mut accounts = grant_accounts(&actors, empty_system_account(), false);
    accounts[3].0 = record_key;

    mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            record_key,
            actors.delegate,
            Pubkey::default(),
            EXPIRATION,
        ),
        &accounts,
        &[custom_error(host::errors::ZamaHostError::InvalidDelegation)],
    );
}

/// The sentinel is refused in the delegate position: "everyone" cannot be a delegate, only a
/// scope.
#[test]
fn a_grant_to_the_wildcard_sentinel_is_rejected() {
    let actors = actors();
    let wildcard = Pubkey::new_from_array(host::WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY_BYTES);
    let (record_key, _) =
        host::user_decryption_delegation_address(actors.delegator, wildcard, actors.authority);
    let mut accounts = grant_accounts(&actors, empty_system_account(), false);
    accounts[3].0 = record_key;

    mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            record_key,
            wildcard,
            actors.authority,
            EXPIRATION,
        ),
        &accounts,
        &[custom_error(host::errors::ZamaHostError::InvalidDelegation)],
    );
}

/// Delegating to oneself is meaningless and refused, as are the two overlaps with the
/// authority: the three tuple positions must name three different parties.
#[test]
fn a_grant_between_overlapping_parties_is_rejected() {
    let actors = actors();
    let overlaps = [
        (actors.delegator, actors.authority), // delegate == delegator
        (actors.delegate, actors.delegator),  // authority == delegator
        (actors.delegate, actors.delegate),   // authority == delegate
    ];

    for (delegate, authority) in overlaps {
        let (record_key, _) =
            host::user_decryption_delegation_address(actors.delegator, delegate, authority);
        let mut accounts = grant_accounts(&actors, empty_system_account(), false);
        accounts[3].0 = record_key;

        mollusk().process_and_validate_instruction(
            &grant_ix(
                actors.payer,
                actors.delegator,
                record_key,
                delegate,
                authority,
                EXPIRATION,
            ),
            &accounts,
            &[custom_error(host::errors::ZamaHostError::InvalidDelegation)],
        );
    }
}

/// The expiration must lie strictly beyond the current slot: a grant expiring in the very
/// slot it lands in was never live at any observation point.
#[test]
fn a_grant_expiring_at_the_current_slot_is_rejected() {
    let actors = actors();
    let accounts = grant_accounts(&actors, empty_system_account(), false);

    mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            actors.record_key,
            actors.delegate,
            actors.authority,
            CURRENT_SLOT,
        ),
        &accounts,
        &[custom_error(host::errors::ZamaHostError::InvalidDelegation)],
    );
}

/// Granting is pause-gated.
#[test]
fn a_grant_while_paused_is_rejected() {
    let actors = actors();
    let accounts = grant_accounts(&actors, empty_system_account(), true);

    mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            actors.record_key,
            actors.delegate,
            actors.authority,
            EXPIRATION,
        ),
        &accounts,
        &[custom_error(host::errors::ZamaHostError::HostConfigPaused)],
    );
}

/// The account list is closed: an extra account is refused rather than ignored, so nothing
/// can be smuggled in for a future handler change to trip over.
#[test]
fn a_grant_with_remaining_accounts_is_rejected() {
    let actors = actors();
    let stowaway = Pubkey::new_unique();
    let mut accounts = grant_accounts(&actors, empty_system_account(), false);
    accounts.push((stowaway, funded_wallet()));
    let mut instruction = grant_ix(
        actors.payer,
        actors.delegator,
        actors.record_key,
        actors.delegate,
        actors.authority,
        EXPIRATION,
    );
    instruction
        .accounts
        .push(AccountMeta::new_readonly(stowaway, false));

    mollusk().process_and_validate_instruction(
        &instruction,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::UnexpectedRemainingAccounts,
        )],
    );
}

/// A record account at any address other than the canonical PDA of the granted tuple is
/// refused: the address is derived, never trusted from the caller.
#[test]
fn a_grant_into_a_non_canonical_address_is_rejected() {
    let actors = actors();
    let elsewhere = Pubkey::new_unique();
    let mut accounts = grant_accounts(&actors, empty_system_account(), false);
    accounts[3].0 = elsewhere;

    mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            elsewhere,
            actors.delegate,
            actors.authority,
            EXPIRATION,
        ),
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::DelegationPdaMismatch,
        )],
    );
}

/// The delegator must sign. Without this, anyone could grant delegations away from anyone.
#[test]
fn a_grant_requires_the_delegators_signature() {
    let actors = actors();
    let accounts = grant_accounts(&actors, empty_system_account(), false);
    let mut instruction = grant_ix(
        actors.payer,
        actors.delegator,
        actors.record_key,
        actors.delegate,
        actors.authority,
        EXPIRATION,
    );
    // The delegator is the second account of the instruction; clear its signer flag.
    instruction.accounts[1] = AccountMeta::new_readonly(actors.delegator, false);

    let result = mollusk().process_instruction(&instruction, &accounts);
    assert!(
        result.program_result.is_err(),
        "an unsigned grant must fail"
    );
}

/// A grant handed a foreign account in the host-config position is refused by the config's
/// own discriminator — a delegation record is program-owned too, which is exactly why the
/// discriminator has to be the thing checked.
#[test]
fn a_grant_with_a_substituted_host_config_is_rejected() {
    let actors = actors();
    let record = live_record(&actors);
    let (host_config, _) = host::host_config_address();
    let accounts = vec![
        (actors.payer, funded_wallet()),
        (actors.delegator, funded_wallet()),
        // The delegation record itself, standing where the config should be.
        (host_config, record_account(&record)),
        (actors.record_key, empty_system_account()),
        (system_program::ID, system_program_account()),
    ];

    mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            actors.record_key,
            actors.delegate,
            actors.authority,
            EXPIRATION,
        ),
        &accounts,
        &[anchor_framework_error_check(
            anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch,
        )],
    );
}

// ---------------------------------------------------------------------------
// Re-grant
// ---------------------------------------------------------------------------

/// A re-grant rewrites the expiration, counts one more, and stamps the current slot.
#[test]
fn a_regrant_increments_the_counter_and_rewrites_the_fields() {
    let actors = actors();
    let existing = live_record(&actors);
    let accounts = grant_accounts(&actors, record_account(&existing), false);
    let new_expiration = EXPIRATION + 100;

    let result = mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            actors.record_key,
            actors.delegate,
            actors.authority,
            new_expiration,
        ),
        &accounts,
        &[Check::success()],
    );

    let record = decode_record(&result.get_account(&actors.record_key).expect("record").data);
    assert_eq!(record.expiration_slot, new_expiration);
    assert_eq!(record.delegation_counter, 2, "a re-grant counts one more");
    assert_eq!(record.last_update_slot, CURRENT_SLOT);
    assert!(!record.revoked);
}

/// A record mutates at most once per slot — that is what makes every
/// `(delegation_counter, last_update_slot)` pair unambiguous to an off-chain reader.
#[test]
fn a_regrant_in_the_records_own_update_slot_is_rejected() {
    let actors = actors();
    let mut existing = live_record(&actors);
    existing.last_update_slot = CURRENT_SLOT;
    let accounts = grant_accounts(&actors, record_account(&existing), false);

    mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            actors.record_key,
            actors.delegate,
            actors.authority,
            EXPIRATION + 100,
        ),
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::DelegationUpdatedInCurrentSlot,
        )],
    );
}

/// A re-grant that changes nothing is refused: a no-op write would still bump the counter
/// and the update slot, spending the once-per-slot budget on nothing.
#[test]
fn a_regrant_with_the_same_expiration_is_rejected() {
    let actors = actors();
    let existing = live_record(&actors);
    let accounts = grant_accounts(&actors, record_account(&existing), false);

    mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            actors.record_key,
            actors.delegate,
            actors.authority,
            EXPIRATION,
        ),
        &accounts,
        &[custom_error(host::errors::ZamaHostError::InvalidDelegation)],
    );
}

/// Re-granting a revoked record reinstates it — same expiration allowed, because the change
/// is the revocation flag itself. The counter still moves, so the two states are never
/// confusable.
#[test]
fn a_regrant_after_revocation_reinstates_the_delegation() {
    let actors = actors();
    let mut revoked = live_record(&actors);
    revoked.revoked = true;
    revoked.expiration_slot = 0;
    revoked.delegation_counter = 2;
    let accounts = grant_accounts(&actors, record_account(&revoked), false);

    let result = mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            actors.record_key,
            actors.delegate,
            actors.authority,
            EXPIRATION,
        ),
        &accounts,
        &[Check::success()],
    );

    let record = decode_record(&result.get_account(&actors.record_key).expect("record").data);
    assert!(!record.revoked, "the re-grant reinstates the delegation");
    assert_eq!(record.expiration_slot, EXPIRATION);
    assert_eq!(record.delegation_counter, 3);
}

/// A counter at the maximum cannot move, so the record cannot be re-granted. Unreachable in
/// practice — the counter moves at most once per slot — but the refusal is a statement that
/// the counter never wraps, which the off-chain freshness contract relies on.
#[test]
fn a_counter_at_the_maximum_cannot_be_regranted() {
    let actors = actors();
    let mut existing = live_record(&actors);
    existing.delegation_counter = u64::MAX;
    let accounts = grant_accounts(&actors, record_account(&existing), false);

    mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            actors.record_key,
            actors.delegate,
            actors.authority,
            EXPIRATION + 100,
        ),
        &accounts,
        &[custom_error(host::errors::ZamaHostError::InvalidDelegation)],
    );
}

/// A record at the canonical address whose own fields name another tuple is refused. Only
/// the program writes program-owned bytes, so nothing an attacker can arrange — but a record
/// that disagrees with its address is corruption, and overwriting it would destroy the
/// evidence.
#[test]
fn an_existing_record_naming_another_tuple_is_rejected() {
    let actors = actors();
    let mut existing = live_record(&actors);
    existing.delegate = Pubkey::new_unique();
    let accounts = grant_accounts(&actors, record_account(&existing), false);

    mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            actors.record_key,
            actors.delegate,
            actors.authority,
            EXPIRATION + 100,
        ),
        &accounts,
        &[custom_error(host::errors::ZamaHostError::InvalidDelegation)],
    );
}

/// A program-owned account of the wrong size at the record address is not a delegation
/// record, whatever its first bytes say.
#[test]
fn an_existing_record_of_the_wrong_size_is_rejected() {
    let actors = actors();
    let mut truncated = record_account(&live_record(&actors));
    truncated.data.truncate(truncated.data.len() - 1);
    let accounts = grant_accounts(&actors, truncated, false);

    mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            actors.record_key,
            actors.delegate,
            actors.authority,
            EXPIRATION + 100,
        ),
        &accounts,
        &[custom_error(host::errors::ZamaHostError::InvalidDelegation)],
    );
}

/// A stored bump other than the canonical one is not this record.
#[test]
fn an_existing_record_with_a_non_canonical_bump_is_rejected() {
    let actors = actors();
    let mut existing = live_record(&actors);
    existing.bump = actors.record_bump.wrapping_sub(1);
    let accounts = grant_accounts(&actors, record_account(&existing), false);

    mollusk().process_and_validate_instruction(
        &grant_ix(
            actors.payer,
            actors.delegator,
            actors.record_key,
            actors.delegate,
            actors.authority,
            EXPIRATION + 100,
        ),
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::DelegationPdaMismatch,
        )],
    );
}

// ---------------------------------------------------------------------------
// Revoke
// ---------------------------------------------------------------------------

/// Revocation flips the flag, zeroes the expiration, counts one more and stamps the slot —
/// the exact state the Connector's freshness rule reads as terminally dead.
#[test]
fn a_revocation_marks_the_record_revoked() {
    let actors = actors();
    let existing = live_record(&actors);
    let accounts = revoke_accounts(&actors, record_account(&existing), false);

    let result = mollusk().process_and_validate_instruction(
        &revoke_ix(actors.delegator, actors.record_key),
        &accounts,
        &[Check::success()],
    );

    let record = decode_record(&result.get_account(&actors.record_key).expect("record").data);
    assert!(record.revoked);
    assert_eq!(record.expiration_slot, 0);
    assert_eq!(record.delegation_counter, 2);
    assert_eq!(record.last_update_slot, CURRENT_SLOT);
}

/// Only the delegator can revoke — not the delegate, and not a stranger.
#[test]
fn a_stranger_cannot_revoke_anothers_delegation() {
    let actors = actors();
    let existing = live_record(&actors);
    let stranger = Pubkey::new_unique();
    let (host_config, host_config_acc) = host_config_account(false);
    let accounts = vec![
        (stranger, funded_wallet()),
        (host_config, host_config_acc),
        (actors.record_key, record_account(&existing)),
    ];

    mollusk().process_and_validate_instruction(
        &revoke_ix(stranger, actors.record_key),
        &accounts,
        &[custom_error(host::errors::ZamaHostError::InvalidDelegation)],
    );
}

/// A second revocation is refused rather than absorbed: the counter must not move for a
/// state that did not change.
#[test]
fn a_second_revocation_is_rejected() {
    let actors = actors();
    let mut revoked = live_record(&actors);
    revoked.revoked = true;
    revoked.expiration_slot = 0;
    let accounts = revoke_accounts(&actors, record_account(&revoked), false);

    mollusk().process_and_validate_instruction(
        &revoke_ix(actors.delegator, actors.record_key),
        &accounts,
        &[custom_error(host::errors::ZamaHostError::DelegationRevoked)],
    );
}

/// The once-per-slot guard holds for revocation too.
#[test]
fn a_revocation_in_the_records_own_update_slot_is_rejected() {
    let actors = actors();
    let mut existing = live_record(&actors);
    existing.last_update_slot = CURRENT_SLOT;
    let accounts = revoke_accounts(&actors, record_account(&existing), false);

    mollusk().process_and_validate_instruction(
        &revoke_ix(actors.delegator, actors.record_key),
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::DelegationUpdatedInCurrentSlot,
        )],
    );
}

/// A well-formed record sitting at a non-canonical address is refused: the expected address
/// is re-derived from the record's own tuple and has to match the account handed in.
#[test]
fn a_revocation_of_a_record_at_a_non_canonical_address_is_rejected() {
    let actors = actors();
    let existing = live_record(&actors);
    let elsewhere = Pubkey::new_unique();
    let (host_config, host_config_acc) = host_config_account(false);
    let accounts = vec![
        (actors.delegator, funded_wallet()),
        (host_config, host_config_acc),
        (elsewhere, record_account(&existing)),
    ];

    mollusk().process_and_validate_instruction(
        &revoke_ix(actors.delegator, elsewhere),
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::DelegationPdaMismatch,
        )],
    );
}

/// A stored bump other than the canonical one is refused on revocation too.
#[test]
fn a_record_with_a_non_canonical_bump_is_rejected_on_revocation() {
    let actors = actors();
    let mut existing = live_record(&actors);
    existing.bump = actors.record_bump.wrapping_sub(1);
    let accounts = revoke_accounts(&actors, record_account(&existing), false);

    mollusk().process_and_validate_instruction(
        &revoke_ix(actors.delegator, actors.record_key),
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::DelegationPdaMismatch,
        )],
    );
}

/// Revocation is pause-gated — pinned as current EVM parity, not endorsed: during a pause
/// the delegator's only lever over a live delegation is blocked while the Connector keeps
/// authorizing against existing records. Raised as an open question to both ecosystems; if
/// revocation is ever exempted from the pause gate, this is the test that changes.
#[test]
fn a_revocation_while_paused_is_rejected() {
    let actors = actors();
    let existing = live_record(&actors);
    let accounts = revoke_accounts(&actors, record_account(&existing), true);

    mollusk().process_and_validate_instruction(
        &revoke_ix(actors.delegator, actors.record_key),
        &accounts,
        &[custom_error(host::errors::ZamaHostError::HostConfigPaused)],
    );
}

/// The revoke account list is closed too.
#[test]
fn a_revocation_with_remaining_accounts_is_rejected() {
    let actors = actors();
    let existing = live_record(&actors);
    let stowaway = Pubkey::new_unique();
    let mut accounts = revoke_accounts(&actors, record_account(&existing), false);
    accounts.push((stowaway, funded_wallet()));
    let mut instruction = revoke_ix(actors.delegator, actors.record_key);
    instruction
        .accounts
        .push(AccountMeta::new_readonly(stowaway, false));

    mollusk().process_and_validate_instruction(
        &instruction,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::UnexpectedRemainingAccounts,
        )],
    );
}

/// The delegator must sign the revocation.
#[test]
fn a_revocation_requires_the_delegators_signature() {
    let actors = actors();
    let existing = live_record(&actors);
    let accounts = revoke_accounts(&actors, record_account(&existing), false);
    let mut instruction = revoke_ix(actors.delegator, actors.record_key);
    instruction.accounts[0] = AccountMeta::new_readonly(actors.delegator, false);

    let result = mollusk().process_instruction(&instruction, &accounts);
    assert!(
        result.program_result.is_err(),
        "an unsigned revocation must fail"
    );
}

/// The role-for-role substitution matrix: each program-owned account of this instruction
/// pair, standing in the other's position, is refused by a discriminator check rather than
/// misread. Both accounts are owned by the same program, which is exactly why ownership
/// alone would not be enough.
#[test]
fn substituted_program_accounts_are_refused_by_their_discriminators() {
    let actors = actors();
    let (host_config, host_config_acc) = host_config_account(false);

    // The host config, standing where the delegation record should be.
    let record_position = vec![
        (actors.delegator, funded_wallet()),
        (host_config, host_config_acc.clone()),
        (actors.record_key, host_config_acc.clone()),
    ];
    mollusk().process_and_validate_instruction(
        &revoke_ix(actors.delegator, actors.record_key),
        &record_position,
        &[anchor_framework_error_check(
            anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch,
        )],
    );

    // The delegation record, standing where the host config should be.
    let config_position = vec![
        (actors.delegator, funded_wallet()),
        (host_config, record_account(&live_record(&actors))),
        (actors.record_key, record_account(&live_record(&actors))),
    ];
    mollusk().process_and_validate_instruction(
        &revoke_ix(actors.delegator, actors.record_key),
        &config_position,
        &[anchor_framework_error_check(
            anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch,
        )],
    );
}

// ---------------------------------------------------------------------------
// A program-controlled delegator
// ---------------------------------------------------------------------------

/// A runtime holding both the wrapper and the host, at [`CURRENT_SLOT`].
fn mollusk_with_vault() -> Mollusk {
    let mut mollusk = zama_solana_test_kit::svm(&vault::id(), "delegator_vault");
    mollusk.add_program(&host::id(), "zama_host");
    mollusk.sysvars.clock.slot = CURRENT_SLOT;
    mollusk
}

/// The accounts of one vault-delegated tuple: the executor's vault PDA is the delegator, and
/// the record address is derived from it exactly as from a wallet.
struct VaultActors {
    executor: Pubkey,
    vault: Pubkey,
    delegate: Pubkey,
    authority: Pubkey,
    record_key: Pubkey,
}

fn vault_actors() -> VaultActors {
    let executor = Pubkey::new_unique();
    let (vault_pda, _) = vault::vault_address(executor);
    let delegate = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let (record_key, _) = host::user_decryption_delegation_address(vault_pda, delegate, authority);
    VaultActors {
        executor,
        vault: vault_pda,
        delegate,
        authority,
        record_key,
    }
}

/// The host program's account entry, as the wrapper's `Program<ZamaHost>` sees it. The code
/// itself comes from the Mollusk program cache; this is only the executable-flagged shell.
/// Owned by the non-upgradeable loader deliberately: an upgradeable-loader shell would have to
/// carry a decodable programdata pointer in its data.
fn host_program_account() -> Account {
    Account {
        lamports: 1,
        data: Vec::new(),
        owner: solana_sdk::bpf_loader::ID,
        executable: true,
        rent_epoch: 0,
    }
}

fn vault_accounts(actors: &VaultActors, record: Account) -> Vec<(Pubkey, Account)> {
    let (host_config, host_config_account) = host_config_account(false);
    vec![
        (actors.executor, funded_wallet()),
        (actors.vault, empty_system_account()),
        (host_config, host_config_account),
        (actors.record_key, record),
        (host::id(), host_program_account()),
        (system_program::ID, system_program_account()),
    ]
}

fn grant_via_vault_ix(actors: &VaultActors, expiration_slot: u64) -> Instruction {
    let (host_config, _) = host::host_config_address();
    zama_solana_test_kit::anchor_ix(
        vault::id(),
        vault::accounts::VaultDelegation {
            executor: actors.executor,
            vault: actors.vault,
            host_config,
            delegation_record: actors.record_key,
            zama_host: host::id(),
            system_program: system_program::ID,
        },
        vault::instruction::GrantViaVault {
            delegate: actors.delegate,
            encrypted_value_account_authority: actors.authority,
            expiration_slot,
        },
    )
}

fn revoke_via_vault_ix(actors: &VaultActors) -> Instruction {
    let (host_config, _) = host::host_config_address();
    zama_solana_test_kit::anchor_ix(
        vault::id(),
        vault::accounts::VaultDelegation {
            executor: actors.executor,
            vault: actors.vault,
            host_config,
            delegation_record: actors.record_key,
            zama_host: host::id(),
            system_program: system_program::ID,
        },
        vault::instruction::RevokeViaVault {},
    )
}

/// The Squads-model evidence, reduced to what the host actually verifies: the delegator is a
/// PDA of another program, no private key exists for it, and the grant lands because
/// `invoke_signed` satisfies the host's `delegator: Signer`. The record is indistinguishable
/// from a wallet's — the Connector needs no notion of "program-controlled" at all.
#[test]
fn a_vault_pda_grants_a_delegation_via_cpi() {
    let actors = vault_actors();
    let accounts = vault_accounts(&actors, empty_system_account());

    let result = mollusk_with_vault().process_and_validate_instruction(
        &grant_via_vault_ix(&actors, EXPIRATION),
        &accounts,
        &[Check::success()],
    );

    let record = decode_record(&result.get_account(&actors.record_key).expect("record").data);
    assert_eq!(
        record.delegator, actors.vault,
        "the delegator is the vault PDA, not the executor"
    );
    assert_eq!(record.delegate, actors.delegate);
    assert_eq!(record.encrypted_value_account_authority, actors.authority);
    assert_eq!(record.delegation_counter, 1);
    assert!(!record.revoked);
}

/// The vault revokes what it granted, through the same seeds.
#[test]
fn a_vault_pda_revokes_its_delegation_via_cpi() {
    let actors = vault_actors();
    let (_, record_bump) =
        host::user_decryption_delegation_address(actors.vault, actors.delegate, actors.authority);
    let existing = UserDecryptionDelegation {
        delegator: actors.vault,
        delegate: actors.delegate,
        encrypted_value_account_authority: actors.authority,
        expiration_slot: EXPIRATION,
        delegation_counter: 1,
        last_update_slot: EARLIER_SLOT,
        revoked: false,
        bump: record_bump,
    };
    let accounts = vault_accounts(&actors, record_account(&existing));

    let result = mollusk_with_vault().process_and_validate_instruction(
        &revoke_via_vault_ix(&actors),
        &accounts,
        &[Check::success()],
    );

    let record = decode_record(&result.get_account(&actors.record_key).expect("record").data);
    assert!(record.revoked);
    assert_eq!(record.expiration_slot, 0);
    assert_eq!(record.delegation_counter, 2);
}

/// An executor cannot act through somebody else's vault: the wrapper holds the vault account
/// to the seeds of *its own* executor, so the substitution dies before any CPI.
#[test]
fn an_executor_cannot_use_anothers_vault() {
    let actors = vault_actors();
    let stranger = Pubkey::new_unique();
    let (strangers_vault, _) = vault::vault_address(stranger);
    let mut accounts = vault_accounts(&actors, empty_system_account());
    accounts[1].0 = strangers_vault;
    let mut instruction = grant_via_vault_ix(&actors, EXPIRATION);
    instruction.accounts[1] = AccountMeta::new_readonly(strangers_vault, false);

    mollusk_with_vault().process_and_validate_instruction(
        &instruction,
        &accounts,
        &[anchor_framework_error_check(
            anchor_lang::error::ErrorCode::ConstraintSeeds,
        )],
    );
}

// ---------------------------------------------------------------------------
// SDK cross-pins
// ---------------------------------------------------------------------------
// The TS SDK derives these addresses and builds these instruction bytes without this crate. The
// literals below are the same ones asserted in
// `sdk/js-sdk/src/solana/actions/userDecryptionDelegation.test.ts` and
// `sdk/js-sdk/src/solana/actions/revokePermits.test.ts`, so a drift in either side's codec or
// derivation breaks both suites on the same bytes rather than surfacing on a live cluster.

fn fixture_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn sdk_fixture_delegation_address_and_instruction_bytes() {
    let delegator = Pubkey::new_from_array([0x11; 32]);
    let delegate = Pubkey::new_from_array([0x22; 32]);
    let authority = Pubkey::new_from_array([0x33; 32]);

    let (address, _) = host::user_decryption_delegation_address(delegator, delegate, authority);
    assert_eq!(address.to_string(), "5bK6ZBSpCgC13c5JT5g2LHjRfTjBM6Fjaybcv8tQqUUX");

    let grant = host::instruction::DelegateForUserDecryption {
        delegate,
        encrypted_value_account_authority: authority,
        expiration_slot: 500,
    }
    .data();
    assert_eq!(
        fixture_hex(&grant),
        "f0f8d7586df401672222222222222222222222222222222222222222222222222222222222222222\
         3333333333333333333333333333333333333333333333333333333333333333f401000000000000"
    );

    let revoke = host::instruction::RevokeDelegationForUserDecryption {}.data();
    assert_eq!(fixture_hex(&revoke), "931b7e35412576e1");

    // The record as the program serializes it, for the SDK's hand-rolled account decoder.
    let record = serialized(UserDecryptionDelegation {
        delegator,
        delegate,
        encrypted_value_account_authority: authority,
        expiration_slot: 500,
        delegation_counter: 7,
        last_update_slot: 400,
        revoked: false,
        bump: 254,
    });
    assert_eq!(
        fixture_hex(&record),
        "25058b21493501f81111111111111111111111111111111111111111111111111111111111111111\
         2222222222222222222222222222222222222222222222222222222222222222\
         3333333333333333333333333333333333333333333333333333333333333333\
         f4010000000000000700000000000000900100000000000000fe"
    );
}

/// The relayer's advisory pre-check derives the same addresses from raw seeds
/// (`relayer/src/host/acl_checker.rs`), where these literals are asserted against the same
/// inputs — a seed-order drift on either side breaks both suites on the same bytes. The
/// authority-specific row shares the `5bK6ZBSp…` literal of the SDK fixture above.
#[test]
fn relayer_fixture_wildcard_row_and_encrypted_value_addresses() {
    let delegator = Pubkey::new_from_array([0x11; 32]);
    let delegate = Pubkey::new_from_array([0x22; 32]);
    let wildcard = Pubkey::new_from_array(host::WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY_BYTES);

    let (wildcard_row, _) =
        host::user_decryption_delegation_address(delegator, delegate, wildcard);
    assert_eq!(
        wildcard_row.to_string(),
        "DjwWqTLQmSDxxCEXS8KmJBqvvmjhYTWKGsZyh343cKJJ"
    );

    let (value_address, _) = host::encrypted_value_address([0x55; 32]);
    assert_eq!(
        value_address.to_string(),
        "5K29xw8jynL8Vw63cRm6cUeQK1dfs5M2Vx3r5inwos5p"
    );
}

#[test]
fn sdk_fixture_permit_invalidation_address_and_revoke_permits_bytes() {
    let user = Pubkey::new_from_array([0x44; 32]);

    let (address, _) = host::permit_invalidation_address(user);
    assert_eq!(address.to_string(), "9mDnXemtzZPxnmXJ6ocXABXsmfXwATkWQC9basgU5q2U");

    let revoke_permits = host::instruction::RevokePermits {}.data();
    assert_eq!(fixture_hex(&revoke_permits), "3319597d7d5ac882");
}
