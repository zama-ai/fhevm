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
//! The last group is not about behaviour at all, but about the bytes. The instruction
//! takes the watermark account unchecked, so its layout appears in no generated interface
//! description, and the off-chain reader that consumes it decodes fixed offsets by hand.
//! Those tests hold both ends of that up against a committed fixture: the program still
//! writes the fixture's bytes, and the fixture's bytes still hand-decode to the fields it
//! declares.
//!
//! Build the program artifact before running these (`bash scripts/check-zama-host-idl.sh`); a
//! stale `.so` makes the whole harness fail on an unrelated missing sysvar rather than on anything
//! these tests assert.

use anchor_lang::{
    prelude::system_program, AccountDeserialize, Discriminator, InstructionData, ToAccountMetas,
};
use mollusk_svm::{result::Check, Mollusk};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use std::{ops::Range, path::PathBuf};
use zama_host::{self as host, PermitInvalidation};
use zama_solana_test_kit::{
    empty_system_account, funded_system_account as funded_wallet, serialized_account as serialized,
    system_program_account,
};

// The schema lives with the fixture rather than in this crate, because the KMS Connector
// includes the same file from its own test target.
#[path = "../../test-fixtures/permit/permit_invalidation_account.rs"]
mod fixture_schema;

use fixture_schema::{
    from_hex, to_hex, AccountData, AddressDerivation, Field, FieldEncoding,
    PermitInvalidationAccountFixture, Production, Program, Seed, SeedKind,
    PERMIT_INVALIDATION_ACCOUNT_SCHEMA,
};

// ---------------------------------------------------------------------------
// Harness
// ---------------------------------------------------------------------------

/// A runtime whose clock reads `unix_timestamp`.
///
/// The clock is the only ambient input this instruction has, so every test states it
/// explicitly rather than inheriting a default.
fn mollusk_at(unix_timestamp: i64) -> Mollusk {
    let mut mollusk = zama_solana_test_kit::svm(&host::id(), "zama_host");
    mollusk.sysvars.clock.slot = 100;
    mollusk.sysvars.clock.unix_timestamp = unix_timestamp;
    mollusk
}

fn anchor_ix<A, D>(accounts: A, args: D) -> Instruction
where
    A: ToAccountMetas,
    D: InstructionData,
{
    zama_solana_test_kit::anchor_ix(host::id(), accounts, args)
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

/// Before the first revocation the user has no account at all — a reader treats that
/// absence as a watermark of zero, which is why there is no initialization step to
/// forget.
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

/// Monotonicity holds over a sequence, not just one step.
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

/// The boundary between "refused" and "recorded", pinned so it cannot drift by one.
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

// ---------------------------------------------------------------------------
// Byte layout, against the committed fixture
// ---------------------------------------------------------------------------

/// Setting this rewrites the committed fixture from the account the program writes.
const UPDATE_ENV: &str = "ZAMA_UPDATE_PERMIT_INVALIDATION_FIXTURE";

/// Where each value sits in the account data. These offsets are all an off-chain reader
/// has: it is handed the account bytes and no description of them.
const DISCRIMINATOR_BYTES: Range<usize> = 0..8;
const USER_BYTES: Range<usize> = 8..40;
const WATERMARK_BYTES: Range<usize> = 40..48;
const BUMP_BYTES: Range<usize> = 48..49;

/// The eight bytes a reader looks for, as a literal rather than as something the framework
/// computes — the point of the fixture is to be comparable against an implementation that
/// does not share our code.
const FIXTURE_DISCRIMINATOR_HEX: &str = "ec8bdba9b922e988";

/// The address seed, spelled out here so the fixture declares it rather than inheriting it.
/// The program does its own derivation and rejects a mismatched address, so a wrong seed
/// here fails the revocation instead of producing a fixture nobody can find the account by.
const FIXTURE_ADDRESS_SEED: &[u8] = b"permit-invalidation";

/// The user the fixture is built for: the identity the normative permit vectors are signed
/// by, so a consumer can hold one key and exercise both fixtures.
const FIXTURE_USER_BASE58: &str = "Dzo7VaLffWBjA59P59wUCbRupUFKLts9BjFeTpM8G2EA";

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../test-fixtures/permit/permit_invalidation_account_v1.json")
}

/// Runs a first revocation for `user` against the account at `invalidation`, and returns
/// the account the program left behind.
fn revoke_and_read(user: Pubkey, invalidation: Pubkey, clock: i64) -> Account {
    let result = mollusk_at(clock).process_and_validate_instruction(
        &revoke_ix(user, invalidation),
        &[
            (user, funded_wallet()),
            (invalidation, empty_system_account()),
            (system_program::ID, system_program_account()),
        ],
        &[Check::success()],
    );
    result
        .get_account(&invalidation)
        .expect("the watermark account exists after a revocation")
        .clone()
}

/// Builds the fixture from a real revocation.
///
/// Only `account` is observed; every declared field value comes from this function's own
/// inputs. That is what makes the hand-decoding test below meaningful rather than
/// circular: it compares what the program wrote against what we asked for, at offsets
/// nothing in the program chooses.
fn build_fixture() -> PermitInvalidationAccountFixture {
    let user: Pubkey = FIXTURE_USER_BASE58.parse().expect("fixture user is base58");
    let (address, bump) =
        Pubkey::find_program_address(&[FIXTURE_ADDRESS_SEED, user.as_ref()], &host::id());
    let watermark = REVOCATION_TIME as u64;
    let account = revoke_and_read(user, address, REVOCATION_TIME);

    PermitInvalidationAccountFixture {
        schema: PERMIT_INVALIDATION_ACCOUNT_SCHEMA.to_string(),
        description: "One PermitInvalidation account, byte for byte as the Solana host \
                      program writes it. The revoke_permits instruction takes this account \
                      unchecked, so no generated interface description carries its layout \
                      and the KMS Connector decodes the offsets below by hand out of an \
                      account snapshot."
            .to_string(),
        regenerate_with: "bash scripts/update-permit-invalidation-fixture.sh".to_string(),
        program: Program {
            id_base58: host::id().to_string(),
            id_hex: to_hex(host::id().as_ref()),
        },
        address: AddressDerivation {
            seeds: vec![
                Seed {
                    name: "permit-invalidation".to_string(),
                    kind: SeedKind::Utf8Literal,
                    hex: to_hex(FIXTURE_ADDRESS_SEED),
                    utf8: Some(
                        String::from_utf8(FIXTURE_ADDRESS_SEED.to_vec()).expect("ascii seed"),
                    ),
                },
                Seed {
                    name: "user".to_string(),
                    kind: SeedKind::Pubkey,
                    hex: to_hex(user.as_ref()),
                    utf8: None,
                },
            ],
            address_base58: address.to_string(),
            address_hex: to_hex(address.as_ref()),
            bump,
        },
        produced_by: Production {
            instruction: "revoke_permits".to_string(),
            prior_account_state: "absent — the user's first revocation, so the stored \
                                  watermark reads as zero and the clock wins"
                .to_string(),
            clock_unix_timestamp: REVOCATION_TIME.to_string(),
        },
        account: AccountData {
            owner_base58: account.owner.to_string(),
            data_len: account.data.len(),
            data_hex: to_hex(&account.data),
            discriminator_hex: FIXTURE_DISCRIMINATOR_HEX.to_string(),
            discriminator_preimage_utf8: "account:PermitInvalidation".to_string(),
            layout: "discriminator(8) | user(32) | invalidation_watermark(u64 little-endian) \
                     | bump(u8)"
                .to_string(),
        },
        fields: vec![
            Field {
                name: "discriminator".to_string(),
                offset: DISCRIMINATOR_BYTES.start,
                length: DISCRIMINATOR_BYTES.len(),
                encoding: FieldEncoding::Bytes,
                value: FIXTURE_DISCRIMINATOR_HEX.to_string(),
                value_base58: None,
                comment: "First eight bytes of sha256(\"account:PermitInvalidation\"). \
                          Anything else at this offset is another account type."
                    .to_string(),
            },
            Field {
                name: "user".to_string(),
                offset: USER_BYTES.start,
                length: USER_BYTES.len(),
                encoding: FieldEncoding::Pubkey,
                value: to_hex(user.as_ref()),
                value_base58: Some(user.to_string()),
                comment: "The user whose permits this watermark governs. Implied by the \
                          address, stored so a decoded account can be checked against the \
                          key it was fetched under."
                    .to_string(),
            },
            Field {
                name: "invalidation_watermark".to_string(),
                offset: WATERMARK_BYTES.start,
                length: WATERMARK_BYTES.len(),
                encoding: FieldEncoding::U64Le,
                value: watermark.to_string(),
                value_base58: None,
                comment: "Unix seconds of the user's last revocation, never decreasing. A \
                          permit whose validity window starts before this is dead. An \
                          absent account reads as zero."
                    .to_string(),
            },
            Field {
                name: "bump".to_string(),
                offset: BUMP_BYTES.start,
                length: BUMP_BYTES.len(),
                encoding: FieldEncoding::U8,
                value: bump.to_string(),
                value_base58: None,
                comment: "The bump the canonical derivation lands on for this user.".to_string(),
            },
        ],
    }
}

fn load_fixture() -> PermitInvalidationAccountFixture {
    if std::env::var_os(UPDATE_ENV).is_some() {
        // In update mode the committed file is about to be replaced; check the build.
        return build_fixture();
    }
    let path = fixture_path();
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    let fixture: PermitInvalidationAccountFixture =
        serde_json::from_str(&text).expect("committed fixture parses against the shared schema");
    assert_eq!(
        fixture.schema, PERMIT_INVALIDATION_ACCOUNT_SCHEMA,
        "the fixture declares a schema this test does not implement"
    );
    fixture
}

/// The committed fixture is what the program writes — or is rewritten from it when the
/// update gate is set.
///
/// The fixture is vendored by the KMS Connector, so a diff here is a change every
/// off-chain reader has to be told about, not a number to be brought back into line.
#[test]
fn the_committed_fixture_is_what_the_program_writes() {
    let built = build_fixture();
    let serialized = format!(
        "{}\n",
        serde_json::to_string_pretty(&built).expect("fixture serializes")
    );
    let path = fixture_path();

    if std::env::var_os(UPDATE_ENV).is_some() {
        std::fs::create_dir_all(path.parent().expect("fixture directory"))
            .expect("create fixture directory");
        std::fs::write(&path, &serialized).expect("write fixture");
        eprintln!("wrote {}", path.display());
        return;
    }

    let committed = std::fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!(
            "cannot read {}: {error}\nregenerate with: bash \
             scripts/update-permit-invalidation-fixture.sh",
            path.display()
        )
    });

    assert_eq!(
        committed, serialized,
        "the committed fixture differs from what the program writes; regenerate with \
         `bash scripts/update-permit-invalidation-fixture.sh` and review the diff"
    );
}

/// Driving the instruction with nothing but the fixture's own inputs reproduces the
/// fixture's bytes exactly.
///
/// The address comes from the fixture too, so the program's own derivation check has to
/// accept it — a fixture pointing at an address no reader would look under fails here
/// rather than shipping.
#[test]
fn revoking_with_the_fixtures_inputs_reproduces_its_bytes() {
    let fixture = load_fixture();
    let user: Pubkey = fixture
        .field("user")
        .expect("the fixture declares a user field")
        .value_base58
        .as_deref()
        .expect("the user field carries a base58 form")
        .parse()
        .expect("the user field is base58");
    let invalidation: Pubkey = fixture
        .address
        .address_base58
        .parse()
        .expect("the fixture address is base58");
    let clock: i64 = fixture
        .produced_by
        .clock_unix_timestamp
        .parse()
        .expect("the fixture clock is a decimal string");

    let account = revoke_and_read(user, invalidation, clock);

    assert_eq!(account.owner.to_string(), fixture.account.owner_base58);
    assert_eq!(account.data.len(), fixture.account.data_len);
    assert_eq!(
        to_hex(&account.data),
        fixture.account.data_hex,
        "the bytes an off-chain reader decodes changed"
    );
}

/// The fixture's bytes hand-decode to the fields it declares.
///
/// Read the way the Connector reads: fixed offsets out of a byte slice, with no borsh
/// derive and no framework decoder in reach. This is what pins field *order*. Reorder the
/// record — watermark and bump exchanged, say — and the account's size, its discriminator
/// and a framework decode of it all still agree; the offsets do not.
#[test]
fn the_fixture_bytes_hand_decode_to_its_declared_fields() {
    let fixture = load_fixture();
    let data = fixture.data_bytes().expect("the account data is hex");
    let declared = |name: &str| {
        fixture
            .field(name)
            .unwrap_or_else(|| panic!("the fixture declares a {name} field"))
    };

    assert_eq!(
        data.len(),
        BUMP_BYTES.end,
        "the account is exactly the record's size"
    );

    assert_eq!(
        to_hex(&data[DISCRIMINATOR_BYTES]),
        declared("discriminator").value
    );

    let user = Pubkey::try_from(&data[USER_BYTES]).expect("thirty-two bytes");
    assert_eq!(to_hex(user.as_ref()), declared("user").value);
    assert_eq!(
        Some(user.to_string()),
        declared("user").value_base58,
        "the base58 and hex forms of the user disagree"
    );

    let watermark = u64::from_le_bytes(
        data[WATERMARK_BYTES]
            .try_into()
            .expect("eight watermark bytes"),
    );
    assert_eq!(
        watermark.to_string(),
        declared("invalidation_watermark").value
    );

    assert_eq!(data[BUMP_BYTES.start].to_string(), declared("bump").value);
}

/// A reader following the fixture's declared seeds lands on its declared address and
/// bump — the lookup that has to happen before any of the bytes above can be read.
#[test]
fn the_fixtures_seeds_derive_its_declared_address() {
    let fixture = load_fixture();
    let program: Pubkey = fixture
        .program
        .id_base58
        .parse()
        .expect("the fixture program id is base58");
    let seeds: Vec<Vec<u8>> = fixture
        .address
        .seeds
        .iter()
        .map(|seed| from_hex(&seed.hex).expect("a seed is hex"))
        .collect();
    let seed_refs: Vec<&[u8]> = seeds.iter().map(Vec::as_slice).collect();

    let (address, bump) = Pubkey::find_program_address(&seed_refs, &program);
    assert_eq!(address.to_string(), fixture.address.address_base58);
    assert_eq!(bump, fixture.address.bump);
}
