//! Mollusk runtime tests for the permit-invalidation watermark instruction.
//!
//! The instruction writes one number, and almost everything worth testing about it is a
//! negative: the watermark must never go down, and one user must never be able to move
//! another's. Both matter because the watermark is a user's only lever for killing
//! outstanding permits — a lever that can be lowered, bypassed, or moved by somebody
//! else is not a lever.
//!
//! Account validation in the handler is manual, so the account-shape tests here are not
//! testing framework behaviour: each of them corresponds to a check that has to be
//! written by hand, and each would pass silently if that check were dropped.
//!
//! Build the program artifact with the `poc` feature before running these; a default
//! or stale `.so` makes the whole harness fail on an unrelated missing sysvar rather
//! than on anything these tests assert.

use anchor_lang::{
    prelude::system_program, AccountDeserialize, AccountSerialize, Discriminator, InstructionData,
    ToAccountMetas,
};
use mollusk_svm::{result::Check, Mollusk};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use std::path::PathBuf;
use zama_host::{self as host, PermitInvalidation};

// ---------------------------------------------------------------------------
// Harness
// ---------------------------------------------------------------------------

/// A runtime whose clock reads `unix_timestamp`.
///
/// The clock is the only ambient input this instruction has, so every test states it
/// explicitly rather than inheriting a default.
fn mollusk_at(unix_timestamp: i64) -> Mollusk {
    let deploy_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy");
    unsafe {
        std::env::set_var("SBF_OUT_DIR", deploy_dir);
    }
    let mut mollusk = Mollusk::new(&host::id(), "zama_host");
    mollusk.sysvars.clock.slot = 100;
    mollusk.sysvars.clock.unix_timestamp = unix_timestamp;
    mollusk
}

fn anchor_ix<A, D>(accounts: A, args: D) -> Instruction
where
    A: ToAccountMetas,
    D: InstructionData,
{
    Instruction {
        program_id: host::id(),
        accounts: accounts.to_account_metas(None),
        data: args.data(),
    }
}

fn revoke_ix(user: Pubkey, invalidation: Pubkey) -> Instruction {
    anchor_ix(
        host::accounts::RevokePermits {
            user,
            invalidation,
            system_program: system_program::ID,
        },
        host::instruction::RevokePermits {},
    )
}

fn funded_wallet() -> Account {
    Account {
        lamports: 10_000_000_000,
        data: vec![],
        owner: system_program::ID,
        executable: false,
        rent_epoch: 0,
    }
}

fn empty_system_account() -> Account {
    Account {
        lamports: 0,
        data: vec![],
        owner: system_program::ID,
        executable: false,
        rent_epoch: 0,
    }
}

fn system_program_account() -> Account {
    Account {
        lamports: 1,
        data: b"system_program".to_vec(),
        owner: solana_sdk::native_loader::ID,
        executable: true,
        rent_epoch: 0,
    }
}

fn serialized<T: AccountSerialize>(account: T) -> Vec<u8> {
    let mut data = Vec::new();
    account.try_serialize(&mut data).unwrap();
    data
}

/// An existing watermark account for `user`, already carrying `watermark`.
fn existing_watermark_account(user: Pubkey, watermark: u64) -> (Pubkey, Account) {
    let (address, bump) = host::permit_invalidation_address(user);
    (
        address,
        Account {
            lamports: 1_000_000_000,
            data: serialized(PermitInvalidation {
                user,
                invalidation_watermark: watermark,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

/// Reads the watermark out of a resulting account.
fn watermark_of(data: &[u8]) -> u64 {
    let mut slice: &[u8] = data;
    PermitInvalidation::try_deserialize(&mut slice)
        .expect("resulting account decodes as a watermark record")
        .invalidation_watermark
}

/// The three accounts the instruction takes, with the watermark slot absent.
fn accounts_with_absent_watermark(user: Pubkey) -> (Pubkey, Vec<(Pubkey, Account)>) {
    let (invalidation, _) = host::permit_invalidation_address(user);
    (
        invalidation,
        vec![
            (user, funded_wallet()),
            (invalidation, empty_system_account()),
            (system_program::ID, system_program_account()),
        ],
    )
}

const REVOCATION_TIME: i64 = 1_767_229_380; // 2026-01-01T01:03:00Z

// ---------------------------------------------------------------------------
// Creating the watermark
// ---------------------------------------------------------------------------

/// The first revocation creates the account and records the clock. Before it, the user
/// has no account at all — a reader treats that absence as a watermark of zero, which
/// is why there is no initialization step to forget.
#[test]
fn first_revocation_creates_the_account_and_records_the_clock() {
    let user = Pubkey::new_unique();
    let (invalidation, accounts) = accounts_with_absent_watermark(user);

    let result = mollusk_at(REVOCATION_TIME).process_and_validate_instruction(
        &revoke_ix(user, invalidation),
        &accounts,
        &[Check::success()],
    );

    let created = result
        .get_account(&invalidation)
        .expect("the watermark account exists after the first revocation");
    assert_eq!(created.owner, host::id(), "the account is program-owned");
    assert_eq!(
        created.data.len(),
        8 + PermitInvalidation::SPACE,
        "the account is exactly the record's size"
    );
    assert_eq!(
        &created.data[..8],
        PermitInvalidation::DISCRIMINATOR,
        "the account carries the record's discriminator"
    );
    assert_eq!(watermark_of(&created.data), REVOCATION_TIME as u64);
}

/// The stored record names the user it belongs to, so a decoded account can be checked
/// against the address it came from rather than trusted.
#[test]
fn the_created_record_names_its_user_and_bump() {
    let user = Pubkey::new_unique();
    let (invalidation, accounts) = accounts_with_absent_watermark(user);
    let (_, expected_bump) = host::permit_invalidation_address(user);

    let result = mollusk_at(REVOCATION_TIME).process_and_validate_instruction(
        &revoke_ix(user, invalidation),
        &accounts,
        &[Check::success()],
    );

    let mut data: &[u8] = &result.get_account(&invalidation).expect("account").data;
    let record = PermitInvalidation::try_deserialize(&mut data).expect("decodes");
    assert_eq!(record.user, user);
    assert_eq!(record.bump, expected_bump);
}

/// The instruction consults no configuration account, so pausing the host cannot take
/// the revocation lever away from users. There is no config account in the account list
/// to supply — this test states the consequence: revocation succeeds with nothing but
/// the user, the slot, and the system program.
#[test]
fn revocation_does_not_depend_on_host_configuration() {
    let user = Pubkey::new_unique();
    let (invalidation, accounts) = accounts_with_absent_watermark(user);

    mollusk_at(REVOCATION_TIME).process_and_validate_instruction(
        &revoke_ix(user, invalidation),
        &accounts,
        &[Check::success()],
    );
}

// ---------------------------------------------------------------------------
// Monotonicity
// ---------------------------------------------------------------------------

/// A later revocation raises the watermark.
#[test]
fn revocation_with_a_later_clock_raises_the_watermark() {
    let user = Pubkey::new_unique();
    let (invalidation, existing) = existing_watermark_account(user, REVOCATION_TIME as u64);
    let later = REVOCATION_TIME + 3600;

    let result = mollusk_at(later).process_and_validate_instruction(
        &revoke_ix(user, invalidation),
        &[
            (user, funded_wallet()),
            (invalidation, existing),
            (system_program::ID, system_program_account()),
        ],
        &[Check::success()],
    );

    assert_eq!(
        watermark_of(&result.get_account(&invalidation).expect("account").data),
        later as u64
    );
}

/// A revocation landing in a slot whose clock reads *earlier* than the stored watermark
/// must not lower it.
///
/// This is the test the whole instruction exists for. Solana's clock is not monotonic
/// across slots in the way one would like, and a watermark that could move backwards
/// would resurrect permits the user had already killed — silently, and with no way for
/// the user to know.
#[test]
fn revocation_with_an_earlier_clock_does_not_lower_the_watermark() {
    let user = Pubkey::new_unique();
    let stored = REVOCATION_TIME as u64;
    let (invalidation, existing) = existing_watermark_account(user, stored);

    let result = mollusk_at(REVOCATION_TIME - 7200).process_and_validate_instruction(
        &revoke_ix(user, invalidation),
        &[
            (user, funded_wallet()),
            (invalidation, existing),
            (system_program::ID, system_program_account()),
        ],
        &[Check::success()],
    );

    assert_eq!(
        watermark_of(&result.get_account(&invalidation).expect("account").data),
        stored,
        "the watermark must never move backwards"
    );
}

/// A revocation at exactly the stored moment leaves the watermark where it is, and
/// still succeeds — a user pressing the button twice is not an error.
#[test]
fn revocation_with_an_equal_clock_is_idempotent() {
    let user = Pubkey::new_unique();
    let stored = REVOCATION_TIME as u64;
    let (invalidation, existing) = existing_watermark_account(user, stored);

    let result = mollusk_at(REVOCATION_TIME).process_and_validate_instruction(
        &revoke_ix(user, invalidation),
        &[
            (user, funded_wallet()),
            (invalidation, existing),
            (system_program::ID, system_program_account()),
        ],
        &[Check::success()],
    );

    assert_eq!(
        watermark_of(&result.get_account(&invalidation).expect("account").data),
        stored
    );
}

/// Monotonicity holds over a sequence, not just one step: a run of revocations at
/// jittering clocks leaves the watermark at the maximum seen.
#[test]
fn the_watermark_ends_at_the_maximum_over_a_sequence() {
    let user = Pubkey::new_unique();
    let (invalidation, _) = host::permit_invalidation_address(user);

    let offsets: [i64; 7] = [0, 600, -1200, 300, 5400, -600, 1800];
    let mut account = empty_system_account();
    let mut expected = 0u64;

    for offset in offsets {
        let now = REVOCATION_TIME + offset;
        expected = expected.max(now as u64);

        let result = mollusk_at(now).process_and_validate_instruction(
            &revoke_ix(user, invalidation),
            &[
                (user, funded_wallet()),
                (invalidation, account.clone()),
                (system_program::ID, system_program_account()),
            ],
            &[Check::success()],
        );

        account = result.get_account(&invalidation).expect("account").clone();
        assert_eq!(
            watermark_of(&account.data),
            expected,
            "after a revocation at offset {offset}"
        );
    }
}

// ---------------------------------------------------------------------------
// Who may move which watermark
// ---------------------------------------------------------------------------

/// One user cannot move another user's watermark.
///
/// The attacker signs their own transaction and supplies the victim's watermark
/// account. The account is a perfectly valid canonical account — just not theirs. If
/// this were accepted, anyone could kill anyone's permits.
#[test]
fn one_user_cannot_move_another_users_watermark() {
    let victim = Pubkey::new_unique();
    let attacker = Pubkey::new_unique();
    let (victim_watermark, victim_account) = existing_watermark_account(victim, 1_000);

    mollusk_at(REVOCATION_TIME).process_and_validate_instruction(
        &revoke_ix(attacker, victim_watermark),
        &[
            (attacker, funded_wallet()),
            (victim_watermark, victim_account),
            (system_program::ID, system_program_account()),
        ],
        &[Check::err(
            anchor_lang::solana_program::program_error::ProgramError::Custom(
                host::ZamaHostError::PermitInvalidationPdaMismatch.into(),
            ),
        )],
    );
}

/// The same attempt against a *fresh* watermark account is also rejected: an attacker
/// cannot create a watermark on somebody else's behalf either, even though creating one
/// at the current clock would look harmless.
#[test]
fn one_user_cannot_create_another_users_watermark() {
    let victim = Pubkey::new_unique();
    let attacker = Pubkey::new_unique();
    let (victim_watermark, _) = host::permit_invalidation_address(victim);

    mollusk_at(REVOCATION_TIME).process_and_validate_instruction(
        &revoke_ix(attacker, victim_watermark),
        &[
            (attacker, funded_wallet()),
            (victim_watermark, empty_system_account()),
            (system_program::ID, system_program_account()),
        ],
        &[Check::err(
            anchor_lang::solana_program::program_error::ProgramError::Custom(
                host::ZamaHostError::PermitInvalidationPdaMismatch.into(),
            ),
        )],
    );
}

/// The user must sign. Without a signature, anyone could raise anyone's watermark by
/// merely naming them.
#[test]
fn revocation_requires_the_users_signature() {
    let user = Pubkey::new_unique();
    let (invalidation, accounts) = accounts_with_absent_watermark(user);

    // Same instruction, with the signer flag cleared.
    let mut instruction = revoke_ix(user, invalidation);
    instruction.accounts[0] = AccountMeta::new(user, false);

    let result = mollusk_at(REVOCATION_TIME).process_instruction(&instruction, &accounts);
    assert!(
        result.program_result.is_err(),
        "an unsigned revocation must fail"
    );
}

// ---------------------------------------------------------------------------
// Account shape
// ---------------------------------------------------------------------------

/// A watermark account at a non-canonical address is rejected. Without this check the
/// program would happily write a watermark into any account it was handed, and the
/// reader — which looks *only* at the canonical address — would never see it.
#[test]
fn revocation_rejects_a_non_canonical_account() {
    let user = Pubkey::new_unique();
    let unrelated = Pubkey::new_unique();

    mollusk_at(REVOCATION_TIME).process_and_validate_instruction(
        &revoke_ix(user, unrelated),
        &[
            (user, funded_wallet()),
            (unrelated, empty_system_account()),
            (system_program::ID, system_program_account()),
        ],
        &[Check::err(
            anchor_lang::solana_program::program_error::ProgramError::Custom(
                host::ZamaHostError::PermitInvalidationPdaMismatch.into(),
            ),
        )],
    );
}

/// An account at the right address but owned by another program is rejected: ownership
/// is what makes the data trustworthy, and a foreign-owned account's contents were
/// written by somebody else.
#[test]
fn revocation_rejects_an_account_owned_by_another_program() {
    let user = Pubkey::new_unique();
    let (invalidation, mut account) = existing_watermark_account(user, 1_000);
    account.owner = Pubkey::new_unique();

    mollusk_at(REVOCATION_TIME).process_and_validate_instruction(
        &revoke_ix(user, invalidation),
        &[
            (user, funded_wallet()),
            (invalidation, account),
            (system_program::ID, system_program_account()),
        ],
        &[Check::err(
            anchor_lang::solana_program::program_error::ProgramError::Custom(
                host::ZamaHostError::PermitInvalidationAccountInvalid.into(),
            ),
        )],
    );
}

/// A program-owned account at the right address but of the wrong size is rejected
/// rather than reinterpreted. Both directions are covered: too short (a truncated or
/// foreign record) and too long (a different account type that happens to live here).
#[test]
fn revocation_rejects_an_account_of_the_wrong_size() {
    let user = Pubkey::new_unique();
    let (invalidation, canonical) = existing_watermark_account(user, 1_000);

    for length in [
        0usize,
        8,
        8 + PermitInvalidation::SPACE - 1,
        8 + PermitInvalidation::SPACE + 1,
    ] {
        let mut account = canonical.clone();
        account.data.resize(length, 0);

        mollusk_at(REVOCATION_TIME).process_and_validate_instruction(
            &revoke_ix(user, invalidation),
            &[
                (user, funded_wallet()),
                (invalidation, account),
                (system_program::ID, system_program_account()),
            ],
            &[Check::err(
                anchor_lang::solana_program::program_error::ProgramError::Custom(
                    host::ZamaHostError::PermitInvalidationAccountInvalid.into(),
                ),
            )],
        );
    }
}

/// A program-owned account of the right size carrying a *different* record type is
/// rejected: the discriminator is checked, so another host account cannot be
/// reinterpreted as a watermark.
#[test]
fn revocation_rejects_an_account_with_a_foreign_discriminator() {
    let user = Pubkey::new_unique();
    let (invalidation, mut account) = existing_watermark_account(user, 1_000);
    account.data[..8].copy_from_slice(&[0xAA; 8]);

    mollusk_at(REVOCATION_TIME).process_and_validate_instruction(
        &revoke_ix(user, invalidation),
        &[
            (user, funded_wallet()),
            (invalidation, account),
            (system_program::ID, system_program_account()),
        ],
        &[Check::err(
            anchor_lang::solana_program::program_error::ProgramError::Custom(
                host::ZamaHostError::PermitInvalidationAccountInvalid.into(),
            ),
        )],
    );
}

/// A record whose stored user disagrees with the signer is rejected, even at the
/// canonical address for that signer. The address and the contents must agree; if they
/// cannot, the account was not written by this instruction.
#[test]
fn revocation_rejects_a_record_naming_another_user() {
    let user = Pubkey::new_unique();
    let (invalidation, _) = host::permit_invalidation_address(user);
    let (_, bump) = host::permit_invalidation_address(user);

    let account = Account {
        lamports: 1_000_000_000,
        data: serialized(PermitInvalidation {
            user: Pubkey::new_unique(), // somebody else
            invalidation_watermark: 1_000,
            bump,
        }),
        owner: host::id(),
        executable: false,
        rent_epoch: 0,
    };

    mollusk_at(REVOCATION_TIME).process_and_validate_instruction(
        &revoke_ix(user, invalidation),
        &[
            (user, funded_wallet()),
            (invalidation, account),
            (system_program::ID, system_program_account()),
        ],
        &[Check::err(
            anchor_lang::solana_program::program_error::ProgramError::Custom(
                host::ZamaHostError::PermitInvalidationAccountInvalid.into(),
            ),
        )],
    );
}

/// Extra accounts are rejected rather than ignored — the repository's standing rule, so
/// that an instruction's account list cannot quietly grow meaning.
#[test]
fn revocation_rejects_unexpected_remaining_accounts() {
    let user = Pubkey::new_unique();
    let (invalidation, mut accounts) = accounts_with_absent_watermark(user);
    let extra = Pubkey::new_unique();

    let mut instruction = revoke_ix(user, invalidation);
    instruction
        .accounts
        .push(AccountMeta::new_readonly(extra, false));
    accounts.push((extra, empty_system_account()));

    mollusk_at(REVOCATION_TIME).process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(
            anchor_lang::solana_program::program_error::ProgramError::Custom(
                host::ZamaHostError::UnexpectedRemainingAccounts.into(),
            ),
        )],
    );
}

// ---------------------------------------------------------------------------
// The clock
// ---------------------------------------------------------------------------

/// A clock reading before the unix epoch is refused instead of being coerced.
///
/// The watermark is unsigned seconds. Casting a negative time into it would produce a
/// number near the top of the range and permanently kill every permit that user will
/// ever sign — an unrecoverable state reached by a cast. Failing closed is the only
/// behaviour that cannot destroy an account.
#[test]
fn revocation_rejects_a_clock_before_the_epoch() {
    let user = Pubkey::new_unique();
    let (invalidation, accounts) = accounts_with_absent_watermark(user);

    for unix_timestamp in [-1i64, -86_400, i64::MIN] {
        mollusk_at(unix_timestamp).process_and_validate_instruction(
            &revoke_ix(user, invalidation),
            &accounts,
            &[Check::err(
                anchor_lang::solana_program::program_error::ProgramError::Custom(
                    host::ZamaHostError::ClockBeforeEpoch.into(),
                ),
            )],
        );
    }
}

/// A clock at exactly the epoch is accepted and records zero — the boundary between
/// "refused" and "recorded", pinned so it cannot drift by one.
#[test]
fn revocation_at_the_epoch_records_zero() {
    let user = Pubkey::new_unique();
    let (invalidation, accounts) = accounts_with_absent_watermark(user);

    let result = mollusk_at(0).process_and_validate_instruction(
        &revoke_ix(user, invalidation),
        &accounts,
        &[Check::success()],
    );

    assert_eq!(
        watermark_of(&result.get_account(&invalidation).expect("account").data),
        0,
        "a watermark of zero is indistinguishable from an absent account, and that is fine"
    );
}
