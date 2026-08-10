//! Mollusk-based runtime tests for `confidential-token` against the RFC-024 `EncryptedValue`
//! ACL model.
//!
//! Migrated from the old keyed-nonce `AclRecord`/`AclPermission` model (deleted along with
//! `balance_acl_record`/`next_balance_nonce_sequence`-style per-creation PDAs, `assert_acl_record`,
//! and the single-op `fhe_*` instructions) to the new stateless-indexing `EncryptedValue` encrypted value account:
//! `ConfidentialTokenAccount`/`ConfidentialMint` now each point at one stable
//! `EncryptedValue` PDA per encrypted field (`balance_encrypted_value`,
//! `total_supply_encrypted_value`) that is *replaced in place* on every update instead of
//! rotating to a new per-nonce account. See `confidential-token/src/fhe.rs`,
//! `zama-host/src/state/encrypted_value.rs`, and `zama_solana_acl` for the model this exercises.
//!
//! Scope note: this migration focuses the suite on the surface that changed with the ACL rewrite
//! (mint/token-account creation and `confidential_transfer`'s persistent-output update), plus
//! the token-level end-to-end coverage requested for this pass (stable addressing across a
//! transfer, a `transferred_amount` encrypted value account entry, and self-transfer no-op). It also covers the
//! two consume paths that are thin consumers of the stateless host `verify_public_decrypt`
//! (DD-040). `disclose_secp` supports historical publicly sealed handles and remains idempotent;
//! `redeem_burned_amount` additionally requires the current sequential `PendingBurn` and consumes it
//! exactly once. Their foreign-proof, live/destroyed-context, binding, and replay behavior is
//! exercised directly here. Confidential burn, cancellation, and `wrap_usdc` are also covered with
//! SPL Token and Token-2022 fixtures.
//!
//! Also dropped: the old file's event-shaped, `u128`-limited `support::fhe_runtime` simulator.
//! The confidential-transfer test below instead evaluates the canonical `FheExecuteArgs` captured
//! from the real token -> host CPI and binds those clear values to the handles emitted by the host.

use anchor_lang::{prelude::system_program, AccountDeserialize};
use confidential_token as token;
use mollusk_svm::{
    result::{Check, InstructionResult},
    Mollusk,
};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    program_pack::Pack,
    pubkey::Pubkey,
};
use std::collections::HashMap;
use zama_host as host;
use zama_solana_test_kit::kms::{
    amount_attestation_for, amount_attestation_signed_by, coprocessor_signing_key,
    coprocessor_signing_key_n, secp_evm_address,
};
use zama_solana_test_kit::oracle::CleartextLedger;
use zama_solana_test_kit::snapshot as cost_snapshot;
use zama_solana_test_kit::{
    account_is_system_owned_and_empty, anchor_error_check, anchor_framework_error_check,
    anchor_ix, decode_anchor_event, deny_subject_record_account, encrypted_value_account,
    event_authority, handle_for_chain, new_encrypted_value, read_account, read_encrypted_value,
    serialized_account, system_account, Ctx, HostConfigParams, BALANCE_FHE_TYPE,
    DECRYPTION_CONTRACT, GATEWAY_CHAIN_ID,
};

// ---------------------------------------------------------------------------
// Harness
// ---------------------------------------------------------------------------

fn mollusk() -> Mollusk {
    let mut mollusk = zama_solana_test_kit::svm(&token::id(), "confidential_token");
    mollusk.add_program(&host::id(), "zama_host");
    mollusk_svm_programs_token::token::add_program(&mut mollusk);
    mollusk_svm_programs_token::token2022::add_program(&mut mollusk);
    zama_solana_test_kit::set_previous_bank_hash_sysvars(&mut mollusk);
    // A transfer (secp attestation recovery + three persistent bindings) exceeds
    // the 200k default; real transactions request a higher limit the same way.
    mollusk.compute_budget.compute_unit_limit = 1_400_000;
    mollusk
}

/// Token-suite views over the kit's [`CleartextLedger`]: the single-CPI replay every token
/// instruction is expected to issue, and the two encrypted-field reads the assertions use.
trait TokenLedgerExt {
    fn evaluate_fhe_cpi(&mut self, context: &Ctx, result: &InstructionResult) -> usize;
    fn balance(&self, context: &Ctx, token_account: Pubkey) -> u64;
    fn transferred_amount(&self, context: &Ctx, mint: Pubkey, from_token: Pubkey) -> u64;
}

impl TokenLedgerExt for CleartextLedger {
    /// Applies the exact FHE execution invoked by the token program and associates each persistent
    /// result with the handle persisted in its canonical `EncryptedValue` account.
    fn evaluate_fhe_cpi(&mut self, context: &Ctx, result: &InstructionResult) -> usize {
        let replay = self.replay_fhe_cpis(context, result);
        assert_eq!(
            replay.executions, 1,
            "expected one token -> host fhe_execute CPI"
        );
        replay.persistent_outputs
    }

    fn balance(&self, context: &Ctx, token_account: Pubkey) -> u64 {
        let account = read_token_account(context, token_account);
        self.u64_at(context, account.balance_encrypted_value)
    }

    fn transferred_amount(&self, context: &Ctx, mint: Pubkey, from_token: Pubkey) -> u64 {
        self.u64_at(
            context,
            token::encrypted_value_address(
                mint,
                from_token,
                token::encrypted_transferred_amount_label(),
            )
            .0,
        )
    }
}

fn host_config_account(admin: Pubkey, coprocessor_signer: [u8; 20]) -> Account {
    host_config_account_with_flags(admin, &[coprocessor_signer], 1, 0, false)
}

fn host_config_account_with_kms_context(
    admin: Pubkey,
    coprocessor_signer: [u8; 20],
    current_kms_context_id: u64,
) -> Account {
    host_config_account_with_flags(
        admin,
        &[coprocessor_signer],
        1,
        current_kms_context_id,
        false,
    )
}

/// Builds a `HostConfig` account carrying a multi-signer coprocessor set at `threshold` (used by
/// the n-of-m input-attestation tests).
fn host_config_account_with_signer_set(
    admin: Pubkey,
    coprocessor_signers: &[[u8; 20]],
    threshold: u8,
) -> Account {
    host_config_account_with_flags(admin, coprocessor_signers, threshold, 0, false)
}

fn host_config_account_with_flags(
    admin: Pubkey,
    coprocessor_signers: &[[u8; 20]],
    coprocessor_threshold: u8,
    current_kms_context_id: u64,
    grant_deny_list_enabled: bool,
) -> Account {
    zama_solana_test_kit::host_config_account(&HostConfigParams {
        coprocessor_signers: coprocessor_signers.to_vec(),
        coprocessor_threshold,
        current_kms_context_id,
        grant_deny_list_enabled,
        ..HostConfigParams::new(admin)
    })
    .1
}

fn deny_enabled_host_config_account(admin: Pubkey, coprocessor_signer: [u8; 20]) -> Account {
    host_config_account_with_flags(admin, &[coprocessor_signer], 1, 0, true)
}

fn read_token_account(context: &Ctx, address: Pubkey) -> token::ConfidentialTokenAccount {
    read_account(context, address)
}

fn read_confidential_mint(context: &Ctx, address: Pubkey) -> token::ConfidentialMint {
    read_account(context, address)
}

fn expected_historical_peaks(
    encrypted_value: Pubkey,
    old_handle: [u8; 32],
    subjects: &[Pubkey],
) -> Vec<[u8; 32]> {
    let leaves: Vec<[u8; 32]> = subjects
        .iter()
        .enumerate()
        .map(|(index, subject)| {
            zama_solana_acl::historical_access_leaf_commitment(
                encrypted_value.to_bytes(),
                index as u64,
                old_handle,
                subject.to_bytes(),
            )
        })
        .collect();
    zama_solana_acl::mmr_peaks_from_leaves(&leaves)
}

fn token_error(error: token::ConfidentialTokenError) -> Check<'static> {
    anchor_error_check(error as u32)
}

fn host_error(error: host::errors::ZamaHostError) -> Check<'static> {
    anchor_error_check(error as u32)
}

fn anchor_error(error: anchor_lang::error::ErrorCode) -> Check<'static> {
    anchor_framework_error_check(error)
}

// ---------------------------------------------------------------------------
// Fixture
// ---------------------------------------------------------------------------

struct TokenFixture {
    owner: Pubkey,
    bob_owner: Pubkey,
    mint: Pubkey,
    compute_signer: Pubkey,
    host_config: Pubkey,
    alice_token: Pubkey,
    bob_token: Pubkey,
    alice_balance_value: Pubkey,
    bob_balance_value: Pubkey,
    alice_initial: [u8; 32],
    bob_initial: [u8; 32],
}

impl TokenFixture {
    fn new() -> Self {
        Self::with_keys(
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
        )
    }

    /// Fixed-key variant for cost snapshots: PDA bump searches are part of the
    /// measured compute, so profile addresses must not change between runs.
    fn with_keys(owner: Pubkey, bob_owner: Pubkey, mint: Pubkey) -> Self {
        let compute_signer = token::compute_signer_address(mint).0;
        let host_config = host::host_config_address().0;
        let alice_token = token::token_account_address(mint, owner).0;
        let bob_token = token::token_account_address(mint, bob_owner).0;
        let alice_balance_value = token::balance_encrypted_value_address(mint, alice_token).0;
        let bob_balance_value = token::balance_encrypted_value_address(mint, bob_token).0;
        Self {
            owner,
            bob_owner,
            mint,
            compute_signer,
            host_config,
            alice_token,
            bob_token,
            alice_balance_value,
            bob_balance_value,
            alice_initial: handle_for_chain(1, BALANCE_FHE_TYPE),
            bob_initial: handle_for_chain(2, BALANCE_FHE_TYPE),
        }
    }

    fn confidential_mint_account(&self) -> Account {
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(token::ConfidentialMint {
                authority: self.owner,
                domain: self.mint,
                compute_signer: self.compute_signer,
                underlying_mint: Pubkey::new_unique(),
                decimals: 6,
                total_supply_encrypted_value: token::total_supply_encrypted_value_address(
                    self.mint,
                    token::total_supply_authority_address(self.mint).0,
                )
                .0,
            }),
            owner: token::id(),
            executable: false,
            rent_epoch: 0,
        }
    }

    fn confidential_token_account(&self, owner: Pubkey, balance_value: Pubkey) -> Account {
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(token::ConfidentialTokenAccount {
                owner,
                mint: self.mint,
                balance_encrypted_value: balance_value,
                bump: token::token_account_address(self.mint, owner).1,
            }),
            owner: token::id(),
            executable: false,
            rent_epoch: 0,
        }
    }

    fn base_accounts(&self) -> HashMap<Pubkey, Account> {
        let (alice_balance_address, alice_balance_value) = new_encrypted_value(
            self.mint,
            self.alice_token,
            token::encrypted_balance_label(),
            self.alice_initial,
            &[self.owner, self.compute_signer],
        );
        assert_eq!(alice_balance_address, self.alice_balance_value);
        let (bob_balance_address, bob_balance_value) = new_encrypted_value(
            self.mint,
            self.bob_token,
            token::encrypted_balance_label(),
            self.bob_initial,
            &[self.bob_owner, self.compute_signer],
        );
        assert_eq!(bob_balance_address, self.bob_balance_value);

        HashMap::from([
            (self.owner, system_account(5_000_000_000)),
            (self.bob_owner, system_account(5_000_000_000)),
            (self.mint, self.confidential_mint_account()),
            (self.compute_signer, system_account(0)),
            (
                self.host_config,
                host_config_account(self.owner, secp_evm_address(&coprocessor_signing_key())),
            ),
            (
                self.alice_token,
                self.confidential_token_account(self.owner, self.alice_balance_value),
            ),
            (
                self.bob_token,
                self.confidential_token_account(self.bob_owner, self.bob_balance_value),
            ),
            (
                self.alice_balance_value,
                encrypted_value_account(&alice_balance_value),
            ),
            (
                self.bob_balance_value,
                encrypted_value_account(&bob_balance_value),
            ),
            (event_authority(host::id()), system_account(0)),
            (event_authority(token::id()), system_account(0)),
        ])
    }

    fn transferred_amount_value_address(&self, from_token: Pubkey) -> Pubkey {
        token::encrypted_value_address(
            self.mint,
            from_token,
            token::encrypted_transferred_amount_label(),
        )
        .0
    }
}

// ---------------------------------------------------------------------------
// Instruction builders
// ---------------------------------------------------------------------------

fn initialize_mint_ix(
    authority: Pubkey,
    mint: Pubkey,
    underlying_mint: Pubkey,
    host_config: Pubkey,
) -> Instruction {
    let compute_signer = token::compute_signer_address(mint).0;
    let total_supply_authority = token::total_supply_authority_address(mint).0;
    let total_supply_encrypted_value =
        token::total_supply_encrypted_value_address(mint, total_supply_authority).0;
    anchor_ix(
        token::id(),
        token::accounts::InitializeMint {
            authority,
            mint,
            underlying_mint,
            token_program: spl_token::id(),
            compute_signer,
            total_supply_authority,
            total_supply_encrypted_value,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config,
            system_program: system_program::ID,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::InitializeMint {},
    )
}

fn initialize_token_account_ix(
    payer: Pubkey,
    owner: Pubkey,
    mint: Pubkey,
    host_config: Pubkey,
) -> Instruction {
    let compute_signer = token::compute_signer_address(mint).0;
    let (token_account, _bump) = token::token_account_address(mint, owner);
    let balance_encrypted_value = token::balance_encrypted_value_address(mint, token_account).0;
    anchor_ix(
        token::id(),
        token::accounts::InitializeTokenAccount {
            payer,
            owner,
            mint,
            compute_signer,
            token_account,
            balance_encrypted_value,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config,
            system_program: system_program::ID,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::InitializeTokenAccount {},
    )
}

fn confidential_transfer_ix(
    fixture: &TokenFixture,
    from_token: Pubkey,
    to_token: Pubkey,
    from_balance_value: Pubkey,
    to_balance_value: Pubkey,
    amount_attestation: host::CoprocessorInputAttestation,
) -> Instruction {
    confidential_transfer_ix_with_remaining(
        fixture,
        from_token,
        to_token,
        from_balance_value,
        to_balance_value,
        amount_attestation,
        Vec::new(),
    )
}

fn confidential_transfer_ix_with_remaining(
    fixture: &TokenFixture,
    from_token: Pubkey,
    to_token: Pubkey,
    from_balance_value: Pubkey,
    to_balance_value: Pubkey,
    amount_attestation: host::CoprocessorInputAttestation,
    remaining: Vec<Pubkey>,
) -> Instruction {
    confidential_transfer_ix_with_block_cap_accounts(
        fixture,
        from_token,
        to_token,
        from_balance_value,
        to_balance_value,
        amount_attestation,
        remaining,
        None,
        None,
    )
}

/// Block-cap optional accounts threaded through the transfer CPI explicitly; used by the HCU
/// block-cap tests to vary the meter / trust witness. The default unrestricted cap means
/// `confidential_transfer_ix_with_remaining` passes `None`/`None`. Metering keys on the mint's
/// compute signer PDA — there is no separate HCU authority account.
#[allow(clippy::too_many_arguments)]
fn confidential_transfer_ix_with_block_cap_accounts(
    fixture: &TokenFixture,
    from_token: Pubkey,
    to_token: Pubkey,
    from_balance_value: Pubkey,
    to_balance_value: Pubkey,
    amount_attestation: host::CoprocessorInputAttestation,
    remaining: Vec<Pubkey>,
    hcu_block_meter: Option<Pubkey>,
    hcu_trusted_app_record: Option<Pubkey>,
) -> Instruction {
    let mut ix = anchor_ix(
        token::id(),
        token::accounts::ConfidentialTransfer {
            owner: fixture.owner,
            payer: fixture.owner,
            mint: fixture.mint,
            from_account: from_token,
            to_account: to_token,
            compute_signer: fixture.compute_signer,
            from_balance_value,
            to_balance_value,
            transferred_amount_value: fixture.transferred_amount_value_address(from_token),
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            hcu_block_meter,
            hcu_trusted_app_record,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::ConfidentialTransfer { amount_attestation },
    );
    ix.accounts.extend(
        remaining
            .into_iter()
            .map(|pubkey| AccountMeta::new_readonly(pubkey, false)),
    );
    ix
}

/// Builds a `confidential_transfer_from_value` instruction: the amount is taken from the existing
/// on-chain `EncryptedValue` at `amount_value` (a computed or received handle) rather than a fresh
/// attestation. `signer_owner` signs and pays; it must own `from_token` and be in the amount
/// value's subject set.
#[allow(clippy::too_many_arguments)]
fn confidential_transfer_from_value_ix(
    fixture: &TokenFixture,
    signer_owner: Pubkey,
    from_token: Pubkey,
    to_token: Pubkey,
    from_balance_value: Pubkey,
    to_balance_value: Pubkey,
    amount_value: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::ConfidentialTransferFromValue {
            owner: signer_owner,
            payer: signer_owner,
            mint: fixture.mint,
            from_account: from_token,
            to_account: to_token,
            compute_signer: fixture.compute_signer,
            from_balance_value,
            to_balance_value,
            transferred_amount_value: fixture.transferred_amount_value_address(from_token),
            amount_value,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::ConfidentialTransferFromValue {},
    )
}

/// Seeds a spendable amount encrypted value account (a stand-in for a computed/received `euint64` handle) at the
/// canonical PDA `(mint, account, label)` with the given subjects and current handle, and
/// returns its address.
fn seed_amount_value(
    fixture: &TokenFixture,
    accounts: &mut HashMap<Pubkey, Account>,
    account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subjects: &[Pubkey],
) -> Pubkey {
    let (address, value) = new_encrypted_value(
        fixture.mint,
        account,
        encrypted_value_label,
        handle,
        subjects,
    );
    accounts.insert(address, encrypted_value_account(&value));
    address
}

/// Token `allow_token_account_subjects` instruction: the owner-authorized CPI wrapper that grants
/// `subject` on a token-account-scoped `encrypted_value` by signing the host `allow_subjects` call
/// as the `token_account` PDA (`EncryptedValue.encrypted_value_account_authority`), since host subject-list mutation requires
/// the signer to equal `EncryptedValue.encrypted_value_account_authority` (fhevm-internal#1862 #13) and wallet owners are
/// decrypt/compute subjects, not ACL admins.
fn allow_token_account_subjects_ix(
    owner: Pubkey,
    mint: Pubkey,
    token_account: Pubkey,
    encrypted_value: Pubkey,
    host_config: Pubkey,
    subject: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::AllowTokenAccountSubjects {
            payer: owner,
            owner,
            mint,
            token_account,
            encrypted_value,
            host_config,
            deny_subject_record: None,
            zama_program: host::id(),
            system_program: system_program::ID,
        },
        token::instruction::AllowTokenAccountSubjects {
            subjects: vec![subject],
        },
    )
}

fn remove_token_account_subject_ix(
    owner: Pubkey,
    mint: Pubkey,
    token_account: Pubkey,
    encrypted_value: Pubkey,
    host_config: Pubkey,
    subject: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::RemoveTokenAccountSubject {
            owner,
            mint,
            token_account,
            encrypted_value,
            host_config,
            zama_program: host::id(),
        },
        token::instruction::RemoveTokenAccountSubject { subject },
    )
}

fn allow_total_supply_subjects_ix(
    fixture: &BurnRedeemFixture,
    authority: Pubkey,
    subject: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::AllowTotalSupplySubjects {
            payer: authority,
            authority,
            mint: fixture.mint,
            total_supply_authority: fixture.total_supply_authority,
            total_supply_value: fixture.total_supply_value,
            host_config: fixture.host_config,
            deny_subject_record: None,
            zama_program: host::id(),
            system_program: system_program::ID,
        },
        token::instruction::AllowTotalSupplySubjects {
            subjects: vec![subject],
        },
    )
}

fn remove_total_supply_subject_ix(
    fixture: &BurnRedeemFixture,
    authority: Pubkey,
    subject: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::RemoveTotalSupplySubject {
            authority,
            mint: fixture.mint,
            total_supply_authority: fixture.total_supply_authority,
            total_supply_value: fixture.total_supply_value,
            host_config: fixture.host_config,
            zama_program: host::id(),
        },
        token::instruction::RemoveTotalSupplySubject { subject },
    )
}

fn make_token_account_handle_public_ix(
    fixture: &BurnRedeemFixture,
    kind: token::DisclosedValueKind,
    encrypted_value: Pubkey,
    handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::MakeTokenAccountHandlePublic {
            payer: fixture.owner,
            owner: fixture.owner,
            mint: fixture.mint,
            token_account: fixture.token_account,
            encrypted_value,
            host_config: fixture.host_config,
            zama_program: host::id(),
            system_program: system_program::ID,
        },
        token::instruction::MakeTokenAccountHandlePublic { kind, handle },
    )
}

fn make_total_supply_handle_public_ix(
    fixture: &BurnRedeemFixture,
    authority: Pubkey,
    handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::MakeTotalSupplyHandlePublic {
            payer: authority,
            authority,
            mint: fixture.mint,
            total_supply_authority: fixture.total_supply_authority,
            total_supply_value: fixture.total_supply_value,
            host_config: fixture.host_config,
            zama_program: host::id(),
            system_program: system_program::ID,
        },
        token::instruction::MakeTotalSupplyHandlePublic { handle },
    )
}

#[test]
fn mollusk_mint_authority_rotates_total_supply_subjects() {
    let fixture = BurnRedeemFixture::new();
    let auditor = Pubkey::new_unique();
    let context = burn_redeem_mollusk().with_context(fixture.accounts(0));

    context.process_and_validate_instruction(
        &allow_total_supply_subjects_ix(&fixture, fixture.owner, auditor),
        &[Check::success()],
    );
    assert!(read_encrypted_value(&context, fixture.total_supply_value).has_subject(auditor));

    context.process_and_validate_instruction(
        &remove_total_supply_subject_ix(&fixture, fixture.owner, auditor),
        &[Check::success()],
    );
    assert!(!read_encrypted_value(&context, fixture.total_supply_value).has_subject(auditor));
}

#[test]
fn mollusk_owner_rotates_token_account_subjects() {
    let fixture = TokenFixture::new();
    let auditor = Pubkey::new_unique();
    let context = mollusk().with_context(fixture.base_accounts());

    context.process_and_validate_instruction(
        &allow_token_account_subjects_ix(
            fixture.owner,
            fixture.mint,
            fixture.alice_token,
            fixture.alice_balance_value,
            fixture.host_config,
            auditor,
        ),
        &[Check::success()],
    );
    assert!(read_encrypted_value(&context, fixture.alice_balance_value).has_subject(auditor));

    context.process_and_validate_instruction(
        &remove_token_account_subject_ix(
            fixture.owner,
            fixture.mint,
            fixture.alice_token,
            fixture.alice_balance_value,
            fixture.host_config,
            auditor,
        ),
        &[Check::success()],
    );
    assert!(!read_encrypted_value(&context, fixture.alice_balance_value).has_subject(auditor));
}

#[test]
fn mollusk_non_owner_cannot_rotate_token_account_subjects() {
    let fixture = TokenFixture::new();
    let stranger = Pubkey::new_unique();
    let mut accounts = fixture.base_accounts();
    accounts.insert(stranger, system_account(1_000_000_000));
    let context = mollusk().with_context(accounts);

    for instruction in [
        allow_token_account_subjects_ix(
            stranger,
            fixture.mint,
            fixture.alice_token,
            fixture.alice_balance_value,
            fixture.host_config,
            Pubkey::new_unique(),
        ),
        remove_token_account_subject_ix(
            stranger,
            fixture.mint,
            fixture.alice_token,
            fixture.alice_balance_value,
            fixture.host_config,
            fixture.owner,
        ),
    ] {
        context.process_and_validate_instruction(
            &instruction,
            &[anchor_error(anchor_lang::error::ErrorCode::ConstraintSeeds)],
        );
    }
}

#[test]
fn mollusk_non_mint_authority_cannot_rotate_total_supply_subjects() {
    let fixture = BurnRedeemFixture::new();
    let stranger = Pubkey::new_unique();
    let mut accounts = fixture.accounts(0);
    accounts.insert(stranger, system_account(1_000_000_000));
    let context = mollusk().with_context(accounts);

    context.process_and_validate_instruction(
        &allow_total_supply_subjects_ix(&fixture, stranger, Pubkey::new_unique()),
        &[token_error(
            token::ConfidentialTokenError::MintAuthorityMismatch,
        )],
    );
    context.process_and_validate_instruction(
        &make_total_supply_handle_public_ix(&fixture, stranger, fixture.initial_total_supply),
        &[token_error(
            token::ConfidentialTokenError::MintAuthorityMismatch,
        )],
    );
}

#[test]
fn mollusk_total_supply_rotation_rejects_wrong_value_shape() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(0);
    let (_, wrong_value) = new_encrypted_value(
        fixture.mint,
        fixture.total_supply_authority,
        token::encrypted_balance_label(),
        fixture.initial_total_supply,
        &[fixture.compute_signer],
    );
    accounts.insert(
        fixture.total_supply_value,
        encrypted_value_account(&wrong_value),
    );
    let context = mollusk().with_context(accounts);

    context.process_and_validate_instruction(
        &allow_total_supply_subjects_ix(&fixture, fixture.owner, Pubkey::new_unique()),
        &[token_error(
            token::ConfidentialTokenError::TotalSupplyValueMismatch,
        )],
    );
}

#[test]
fn mollusk_owner_seals_exact_token_account_state_field() {
    let fixture = BurnRedeemFixture::new();
    let transferred_handle = handle_for_chain(77, BALANCE_FHE_TYPE);
    let transferred_value = token::encrypted_value_address(
        fixture.mint,
        fixture.token_account,
        token::encrypted_transferred_amount_label(),
    )
    .0;
    let mut accounts = fixture.accounts(0);
    accounts.insert(
        transferred_value,
        encrypted_value_account(
            &new_encrypted_value(
                fixture.mint,
                fixture.token_account,
                token::encrypted_transferred_amount_label(),
                transferred_handle,
                &[fixture.owner, fixture.compute_signer],
            )
            .1,
        ),
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    context.process_and_validate_instruction(
        &make_token_account_handle_public_ix(
            &fixture,
            token::DisclosedValueKind::Balance,
            fixture.balance_value,
            fixture.initial_balance,
        ),
        &[Check::success()],
    );
    let value = read_encrypted_value(&context, fixture.balance_value);
    assert_eq!(value.leaf_count, 1);
    assert_eq!(
        value.peaks,
        zama_solana_acl::mmr_peaks_from_leaves(&[zama_solana_acl::public_decrypt_leaf_commitment(
            fixture.balance_value.to_bytes(),
            0,
            fixture.initial_balance,
        ),])
    );

    context.process_and_validate_instruction(
        &make_token_account_handle_public_ix(
            &fixture,
            token::DisclosedValueKind::BurnedAmount,
            fixture.balance_value,
            fixture.initial_balance,
        ),
        &[token_error(
            token::ConfidentialTokenError::DisclosedValueBindingMismatch,
        )],
    );

    context.process_and_validate_instruction(
        &make_token_account_handle_public_ix(
            &fixture,
            token::DisclosedValueKind::TransferredAmount,
            transferred_value,
            transferred_handle,
        ),
        &[Check::success()],
    );
    assert_eq!(
        read_encrypted_value(&context, transferred_value).leaf_count,
        1
    );
}

#[test]
fn mollusk_mint_authority_seals_total_supply() {
    let fixture = BurnRedeemFixture::new();
    let context = burn_redeem_mollusk().with_context(fixture.accounts(0));

    context.process_and_validate_instruction(
        &make_total_supply_handle_public_ix(&fixture, fixture.owner, fixture.initial_total_supply),
        &[Check::success()],
    );
    assert_eq!(
        read_encrypted_value(&context, fixture.total_supply_value).leaf_count,
        1
    );
}

fn deny_enabled_transfer_accounts(
    fixture: &TokenFixture,
    denied_authority: Option<Pubkey>,
) -> (HashMap<Pubkey, Account>, Vec<Pubkey>) {
    let mut accounts = fixture.base_accounts();
    accounts.insert(
        fixture.host_config,
        deny_enabled_host_config_account(
            fixture.owner,
            secp_evm_address(&coprocessor_signing_key()),
        ),
    );

    let from_deny = host::deny_subject_address(fixture.alice_token).0;
    let to_deny = host::deny_subject_address(fixture.bob_token).0;
    let from_account = if denied_authority == Some(fixture.alice_token) {
        deny_subject_record_account(fixture.alice_token, true).1
    } else {
        system_account(0)
    };
    let to_account = if denied_authority == Some(fixture.bob_token) {
        deny_subject_record_account(fixture.bob_token, true).1
    } else {
        system_account(0)
    };
    accounts.insert(from_deny, from_account);
    accounts.insert(to_deny, to_account);
    // The transferred-amount create grants [owner, bob_owner, compute_signer] under the sender
    // token-account authority. The recipient balance update preserves its audience, so the
    // recipient token-account authority is not part of the grant policy.
    let mut records = vec![from_deny];
    for subject in [fixture.owner, fixture.bob_owner, fixture.compute_signer] {
        let record = host::deny_subject_address(subject).0;
        accounts.insert(record, system_account(0));
        records.push(record);
    }
    (accounts, records)
}

// ---------------------------------------------------------------------------
// initialize_mint / initialize_token_account
// ---------------------------------------------------------------------------

#[test]
fn mollusk_initialize_mint_creates_total_supply_encrypted_value() {
    let authority = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let underlying_mint = Pubkey::new_unique();
    let compute_signer = token::compute_signer_address(mint).0;
    let total_supply_authority = token::total_supply_authority_address(mint).0;
    let total_supply_encrypted_value =
        token::total_supply_encrypted_value_address(mint, total_supply_authority).0;
    let host_config_key = host::host_config_address().0;
    let context = mollusk().with_context(HashMap::from([
        (authority, system_account(5_000_000_000)),
        (mint, system_account(0)),
        (
            underlying_mint,
            Account {
                lamports: 1_000_000_000,
                data: {
                    let mut data = vec![0u8; anchor_spl::token::spl_token::state::Mint::LEN];
                    anchor_spl::token::spl_token::state::Mint::pack(
                        anchor_spl::token::spl_token::state::Mint {
                            mint_authority: solana_sdk::program_option::COption::Some(authority),
                            supply: 0,
                            decimals: 6,
                            is_initialized: true,
                            freeze_authority: solana_sdk::program_option::COption::None,
                        },
                        &mut data,
                    )
                    .unwrap();
                    data
                },
                owner: anchor_spl::token::spl_token::id(),
                executable: false,
                rent_epoch: 0,
            },
        ),
        (compute_signer, system_account(0)),
        (total_supply_authority, system_account(0)),
        (total_supply_encrypted_value, system_account(0)),
        (host_config_key, host_config_account(authority, [0u8; 20])),
        (event_authority(host::id()), system_account(0)),
        (event_authority(token::id()), system_account(0)),
    ]));
    let ix = initialize_mint_ix(authority, mint, underlying_mint, host_config_key);

    context.process_and_validate_instruction(&ix, &[Check::success()]);

    let stored = read_confidential_mint(&context, mint);
    assert_eq!(stored.authority, authority);
    assert_eq!(stored.domain, mint);
    assert_eq!(stored.compute_signer, compute_signer);
    assert_eq!(
        stored.total_supply_encrypted_value,
        total_supply_encrypted_value
    );
    let supply_value = read_encrypted_value(&context, total_supply_encrypted_value);
    assert_eq!(supply_value.domain, mint);
    assert_eq!(
        supply_value.encrypted_value_account_authority,
        total_supply_authority
    );
    assert_eq!(supply_value.label, token::encrypted_total_supply_label());
    assert!(supply_value.has_subject(compute_signer));
}

#[test]
fn mollusk_initialize_token_account_creates_initial_balance_encrypted_value() {
    let fixture = TokenFixture::new();
    let owner = Pubkey::new_unique();
    let (token_account, token_bump) = token::token_account_address(fixture.mint, owner);
    let balance_encrypted_value =
        token::balance_encrypted_value_address(fixture.mint, token_account).0;
    let mut accounts = fixture.base_accounts();
    accounts.insert(owner, system_account(5_000_000_000));
    accounts.insert(token_account, system_account(0));
    accounts.insert(balance_encrypted_value, system_account(0));
    let context = mollusk().with_context(accounts);
    let ix = initialize_token_account_ix(owner, owner, fixture.mint, fixture.host_config);

    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    let stored = read_token_account(&context, token_account);
    assert_eq!(stored.owner, owner);
    assert_eq!(stored.mint, fixture.mint);
    assert_eq!(stored.bump, token_bump);
    assert_eq!(stored.balance_encrypted_value, balance_encrypted_value);

    let balance_value = read_encrypted_value(&context, balance_encrypted_value);
    assert_eq!(balance_value.domain, fixture.mint);
    assert_eq!(
        balance_value.encrypted_value_account_authority,
        token_account
    );
    assert_eq!(balance_value.label, token::encrypted_balance_label());
    assert!(balance_value.has_subject(owner));
    assert!(balance_value.has_subject(fixture.compute_signer));

    let balance_events: Vec<token::BalanceHandleUpdatedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(balance_events.len(), 1);
    assert_eq!(balance_events[0].mint, fixture.mint);
    assert_eq!(balance_events[0].owner, owner);
    assert_eq!(balance_events[0].token_account, token_account);
    assert_eq!(balance_events[0].old_handle, [0; 32]);
    assert_eq!(balance_events[0].old_encrypted_value, Pubkey::default());
    assert_eq!(balance_events[0].new_handle, balance_value.current_handle);
    assert_eq!(
        balance_events[0].new_encrypted_value,
        balance_encrypted_value
    );
    assert_eq!(
        balance_events[0].reason,
        token::BalanceHandleUpdateReason::Initialize
    );
}

#[test]
fn mollusk_initialize_token_account_allows_distinct_sponsor_and_owner() {
    let fixture = TokenFixture::new();
    let payer = Pubkey::new_unique();
    let owner = Pubkey::new_unique();
    let (token_account, _bump) = token::token_account_address(fixture.mint, owner);
    let balance_encrypted_value =
        token::balance_encrypted_value_address(fixture.mint, token_account).0;
    let mut accounts = fixture.base_accounts();
    accounts.insert(payer, system_account(5_000_000_000));
    accounts.insert(owner, system_account(0));
    accounts.insert(token_account, system_account(0));
    accounts.insert(balance_encrypted_value, system_account(0));
    let context = mollusk().with_context(accounts);
    let ix = initialize_token_account_ix(payer, owner, fixture.mint, fixture.host_config);

    context.process_and_validate_instruction(&ix, &[Check::success()]);

    let stored = read_token_account(&context, token_account);
    assert_eq!(stored.owner, owner);
    let balance_value = read_encrypted_value(&context, balance_encrypted_value);
    assert!(balance_value.has_subject(owner));
    assert!(!balance_value.has_subject(payer));

    let retry = context.process_instruction(&ix);
    assert!(retry.raw_result.is_err());
    let stored_after_retry = read_token_account(&context, token_account);
    let balance_after_retry = read_encrypted_value(&context, balance_encrypted_value);
    assert_eq!(stored_after_retry.owner, stored.owner);
    assert_eq!(
        balance_after_retry.current_handle,
        balance_value.current_handle
    );
    assert_eq!(balance_after_retry.subjects, balance_value.subjects);
}

// ---------------------------------------------------------------------------
// confidential_transfer
// ---------------------------------------------------------------------------

#[test]
fn mollusk_confidential_transfer_self_transfer_is_no_op() {
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_handle = handle_for_chain(9, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let transfer = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.alice_token,
        fixture.alice_balance_value,
        fixture.alice_balance_value,
        attestation,
    );

    let result = context.process_instruction(&transfer);

    assert!(result.raw_result.is_ok());
    assert!(result.inner_instructions.is_empty());
    let balance_value = read_encrypted_value(&context, fixture.alice_balance_value);
    assert_eq!(balance_value.current_handle, fixture.alice_initial);
    assert_eq!(balance_value.leaf_count, 0);
    assert!(account_is_system_owned_and_empty(
        &context,
        fixture.transferred_amount_value_address(fixture.alice_token)
    ));
}

#[test]
fn mollusk_confidential_transfer_updates_value_accounts_and_cleartext_balances() {
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_handle = handle_for_chain(21, BALANCE_FHE_TYPE);
    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.alice_initial, 1_000);
    cleartext.seed_amount(fixture.bob_initial, 100);
    cleartext.seed_amount(amount_handle, 400);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let transferred_value_address = fixture.transferred_amount_value_address(fixture.alice_token);
    let transfer = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
    );

    let result = context.process_and_validate_instruction(&transfer, &[Check::success()]);
    let persistent_outputs = cleartext.evaluate_fhe_cpi(&context, &result);

    assert_eq!(persistent_outputs, 3);
    assert_eq!(cleartext.balance(&context, fixture.alice_token), 600);
    assert_eq!(cleartext.balance(&context, fixture.bob_token), 500);
    assert_eq!(
        cleartext.transferred_amount(&context, fixture.mint, fixture.alice_token),
        400
    );

    // Token account addresses and their balance `EncryptedValue` PDAs stay stable across the
    // transfer: no new balance account is created, the existing encrypted value accounts are replaced in place.
    let alice_token = read_token_account(&context, fixture.alice_token);
    let bob_token = read_token_account(&context, fixture.bob_token);
    assert_eq!(
        alice_token.balance_encrypted_value,
        fixture.alice_balance_value
    );
    assert_eq!(bob_token.balance_encrypted_value, fixture.bob_balance_value);

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    let bob_balance = read_encrypted_value(&context, fixture.bob_balance_value);
    assert_ne!(alice_balance.current_handle, fixture.alice_initial);
    assert_ne!(bob_balance.current_handle, fixture.bob_initial);
    // Update appended exactly one historical leaf per allowed subject.
    assert_eq!(alice_balance.leaf_count, 2);
    assert_eq!(bob_balance.leaf_count, 2);
    assert_eq!(
        alice_balance.subjects,
        vec![fixture.owner, fixture.compute_signer]
    );
    assert_eq!(
        alice_balance.peaks,
        expected_historical_peaks(
            fixture.alice_balance_value,
            fixture.alice_initial,
            &[fixture.owner, fixture.compute_signer],
        )
    );
    assert_eq!(
        bob_balance.subjects,
        vec![fixture.bob_owner, fixture.compute_signer]
    );
    assert_eq!(
        bob_balance.peaks,
        expected_historical_peaks(
            fixture.bob_balance_value,
            fixture.bob_initial,
            &[fixture.bob_owner, fixture.compute_signer],
        )
    );

    // An encrypted value account entry for the transferred amount was created (first bind) at the canonical PDA.
    let transferred = read_encrypted_value(&context, transferred_value_address);
    assert_eq!(transferred.domain, fixture.mint);
    assert_eq!(
        transferred.encrypted_value_account_authority,
        fixture.alice_token
    );
    assert_eq!(
        transferred.label,
        token::encrypted_transferred_amount_label()
    );
    assert!(transferred.has_subject(fixture.owner));
    assert!(transferred.has_subject(fixture.bob_owner));
    assert!(transferred.has_subject(fixture.compute_signer));
    assert_eq!(transferred.leaf_count, 0); // creation: no update yet.

    let transfer_events: Vec<token::ConfidentialTransferEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(transfer_events.len(), 1);
    assert_eq!(transfer_events[0].mint, fixture.mint);
    assert_eq!(transfer_events[0].from_owner, fixture.owner);
    assert_eq!(transfer_events[0].from_token_account, fixture.alice_token);
    assert_eq!(transfer_events[0].to_owner, fixture.bob_owner);
    assert_eq!(transfer_events[0].to_token_account, fixture.bob_token);
    assert_eq!(
        transfer_events[0].transferred_handle,
        transferred.current_handle
    );
    assert_eq!(
        transfer_events[0].transferred_encrypted_value,
        transferred_value_address
    );

    let balance_events: Vec<token::BalanceHandleUpdatedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(balance_events.len(), 2);
    assert_eq!(
        balance_events[0].reason,
        token::BalanceHandleUpdateReason::TransferDebit
    );
    assert_eq!(balance_events[0].old_handle, fixture.alice_initial);
    assert_eq!(balance_events[0].new_handle, alice_balance.current_handle);
    assert_eq!(
        balance_events[1].reason,
        token::BalanceHandleUpdateReason::TransferCredit
    );
    assert_eq!(balance_events[1].old_handle, fixture.bob_initial);
    assert_eq!(balance_events[1].new_handle, bob_balance.current_handle);
}

#[test]
fn mollusk_confidential_transfer_to_second_recipient_rotates_transferred_value_account_subjects() {
    // Regression: a sender's second transfer to a DIFFERENT recipient must succeed. The
    // per-sender transferred-amount encrypted value account rotates its audience to the new recipient, sealing
    // the first receipt's audience into historical leaves (previously reverted with PreviousStateMismatch).
    let fixture = TokenFixture::new();
    let charlie_owner = Pubkey::new_unique();
    let charlie_token = token::token_account_address(fixture.mint, charlie_owner).0;
    let charlie_balance_value =
        token::balance_encrypted_value_address(fixture.mint, charlie_token).0;
    let charlie_initial = handle_for_chain(3, BALANCE_FHE_TYPE);

    let mut accounts = fixture.base_accounts();
    accounts.insert(charlie_owner, system_account(5_000_000_000));
    accounts.insert(
        charlie_token,
        fixture.confidential_token_account(charlie_owner, charlie_balance_value),
    );
    let (_, charlie_value) = new_encrypted_value(
        fixture.mint,
        charlie_token,
        token::encrypted_balance_label(),
        charlie_initial,
        &[charlie_owner, fixture.compute_signer],
    );
    accounts.insert(
        charlie_balance_value,
        encrypted_value_account(&charlie_value),
    );
    let context = mollusk().with_context(accounts);

    let transferred_value_address = fixture.transferred_amount_value_address(fixture.alice_token);

    // First transfer Alice -> Bob: creates the transferred encrypted value account with audience
    // {alice_owner, bob_owner, compute_signer}.
    let first_amount = handle_for_chain(21, BALANCE_FHE_TYPE);
    let first = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        amount_attestation_for(first_amount, fixture.owner, fixture.compute_signer),
    );
    context.process_and_validate_instruction(&first, &[Check::success()]);

    let first_receipt = read_encrypted_value(&context, transferred_value_address);
    assert_eq!(first_receipt.leaf_count, 0);
    // fhevm-internal#1745: the transferred-amount audience must always contain the sender's owner
    // key and the mint compute signer; this exact-equality assertion pins both membership and order.
    assert_eq!(
        first_receipt.subjects,
        vec![fixture.owner, fixture.bob_owner, fixture.compute_signer]
    );
    let first_receipt_handle = first_receipt.current_handle;

    // Second transfer Alice -> Charlie: must now SUCCEED and rotate the encrypted value account audience.
    let second_amount = handle_for_chain(22, BALANCE_FHE_TYPE);
    let second = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        charlie_token,
        fixture.alice_balance_value,
        charlie_balance_value,
        amount_attestation_for(second_amount, fixture.owner, fixture.compute_signer),
    );
    context.process_and_validate_instruction(&second, &[Check::success()]);

    let receipt = read_encrypted_value(&context, transferred_value_address);
    // Audience replaced with the new recipient, still keeping the sender's owner key and the mint
    // compute signer after rotation across two recipients (fhevm-internal#1745).
    assert_eq!(
        receipt.subjects,
        vec![fixture.owner, charlie_owner, fixture.compute_signer]
    );
    // Historical leaves seal the FIRST receipt's audience {alice_owner, bob_owner, compute_signer}.
    assert_eq!(receipt.leaf_count, 3);
    assert_eq!(
        receipt.peaks,
        expected_historical_peaks(
            transferred_value_address,
            first_receipt_handle,
            &[fixture.owner, fixture.bob_owner, fixture.compute_signer],
        )
    );
}

/// Seeds Charlie's token account + balance encrypted value account into `accounts` and returns
/// (charlie_owner, charlie_token, charlie_balance_value).
fn seed_third_account(
    fixture: &TokenFixture,
    accounts: &mut HashMap<Pubkey, Account>,
    initial: [u8; 32],
) -> (Pubkey, Pubkey, Pubkey) {
    let charlie_owner = Pubkey::new_unique();
    let charlie_token = token::token_account_address(fixture.mint, charlie_owner).0;
    let charlie_balance_value =
        token::balance_encrypted_value_address(fixture.mint, charlie_token).0;
    accounts.insert(charlie_owner, system_account(5_000_000_000));
    accounts.insert(
        charlie_token,
        fixture.confidential_token_account(charlie_owner, charlie_balance_value),
    );
    let (_, charlie_value) = new_encrypted_value(
        fixture.mint,
        charlie_token,
        token::encrypted_balance_label(),
        initial,
        &[charlie_owner, fixture.compute_signer],
    );
    accounts.insert(
        charlie_balance_value,
        encrypted_value_account(&charlie_value),
    );
    (charlie_owner, charlie_token, charlie_balance_value)
}

#[test]
fn mollusk_confidential_transfer_rotates_back_to_previous_recipient() {
    // Alice -> Bob, Alice -> Charlie, Alice -> Bob: the per-sender transferred encrypted value account rotates its
    // audience each time and seals every outgoing audience into historical leaves.
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    let (charlie_owner, charlie_token, charlie_balance_value) = seed_third_account(
        &fixture,
        &mut accounts,
        handle_for_chain(3, BALANCE_FHE_TYPE),
    );
    let context = mollusk().with_context(accounts);
    let receipt_address = fixture.transferred_amount_value_address(fixture.alice_token);

    let transfer = |to_token, to_balance, tag| {
        confidential_transfer_ix(
            &fixture,
            fixture.alice_token,
            to_token,
            fixture.alice_balance_value,
            to_balance,
            amount_attestation_for(
                handle_for_chain(tag, BALANCE_FHE_TYPE),
                fixture.owner,
                fixture.compute_signer,
            ),
        )
    };

    context.process_and_validate_instruction(
        &transfer(fixture.bob_token, fixture.bob_balance_value, 21),
        &[Check::success()],
    );
    let handle_after_bob = read_encrypted_value(&context, receipt_address).current_handle;

    context.process_and_validate_instruction(
        &transfer(charlie_token, charlie_balance_value, 22),
        &[Check::success()],
    );
    let after_charlie = read_encrypted_value(&context, receipt_address);
    assert_eq!(after_charlie.leaf_count, 3);
    assert_eq!(
        after_charlie.subjects,
        vec![fixture.owner, charlie_owner, fixture.compute_signer]
    );
    let handle_after_charlie = after_charlie.current_handle;

    context.process_and_validate_instruction(
        &transfer(fixture.bob_token, fixture.bob_balance_value, 23),
        &[Check::success()],
    );
    let after_bob_again = read_encrypted_value(&context, receipt_address);

    // Audience replaced back to Bob; both prior audiences are sealed in order.
    assert_eq!(
        after_bob_again.subjects,
        vec![fixture.owner, fixture.bob_owner, fixture.compute_signer]
    );
    assert_eq!(after_bob_again.leaf_count, 6);
    let mut expected_peaks = Vec::new();
    let mut count = 0u64;
    for (handle, audience) in [
        (
            handle_after_bob,
            [fixture.owner, fixture.bob_owner, fixture.compute_signer],
        ),
        (
            handle_after_charlie,
            [fixture.owner, charlie_owner, fixture.compute_signer],
        ),
    ] {
        for subject in audience {
            let leaf = zama_solana_acl::historical_access_leaf_commitment(
                receipt_address.to_bytes(),
                count,
                handle,
                subject.to_bytes(),
            );
            zama_solana_acl::mmr_append(&mut expected_peaks, &mut count, leaf).unwrap();
        }
    }
    assert_eq!(after_bob_again.peaks, expected_peaks);
}

#[test]
fn mollusk_confidential_transfer_self_transfer_after_receipt_is_no_op() {
    // A self-transfer short-circuits before the execution (execute_transfer returns early when
    // from == to), so it never updates the receipt. After a real transfer created the receipt,
    // a subsequent A -> A succeeds and leaves that receipt untouched.
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let receipt_address = fixture.transferred_amount_value_address(fixture.alice_token);

    context.process_and_validate_instruction(
        &confidential_transfer_ix(
            &fixture,
            fixture.alice_token,
            fixture.bob_token,
            fixture.alice_balance_value,
            fixture.bob_balance_value,
            amount_attestation_for(
                handle_for_chain(21, BALANCE_FHE_TYPE),
                fixture.owner,
                fixture.compute_signer,
            ),
        ),
        &[Check::success()],
    );
    let receipt_before = read_encrypted_value(&context, receipt_address);
    assert_eq!(
        receipt_before.subjects,
        vec![fixture.owner, fixture.bob_owner, fixture.compute_signer]
    );

    // Self-transfer: succeeds as a no-op.
    context.process_and_validate_instruction(
        &confidential_transfer_ix(
            &fixture,
            fixture.alice_token,
            fixture.alice_token,
            fixture.alice_balance_value,
            fixture.alice_balance_value,
            amount_attestation_for(
                handle_for_chain(22, BALANCE_FHE_TYPE),
                fixture.owner,
                fixture.compute_signer,
            ),
        ),
        &[Check::success()],
    );
    let receipt_after = read_encrypted_value(&context, receipt_address);
    assert_eq!(receipt_after.subjects, receipt_before.subjects);
    assert_eq!(receipt_after.current_handle, receipt_before.current_handle);
    assert_eq!(receipt_after.leaf_count, receipt_before.leaf_count);
}

#[test]
fn mollusk_confidential_transfer_deny_list_enabled_rotation_to_new_recipient_succeeds() {
    // Deny-list ENABLED + rotation: the second transfer to a new recipient adds that recipient's
    // owner to the transferred audience, so its deny record must reach the host and clear.
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    accounts.insert(
        fixture.host_config,
        deny_enabled_host_config_account(
            fixture.owner,
            secp_evm_address(&coprocessor_signing_key()),
        ),
    );
    let (charlie_owner, charlie_token, charlie_balance_value) = seed_third_account(
        &fixture,
        &mut accounts,
        handle_for_chain(3, BALANCE_FHE_TYPE),
    );

    let alice_deny = host::deny_subject_address(fixture.alice_token).0;
    let bob_deny = host::deny_subject_address(fixture.bob_token).0;
    let charlie_token_deny = host::deny_subject_address(charlie_token).0;
    let charlie_owner_deny = host::deny_subject_address(charlie_owner).0;
    let owner_deny = host::deny_subject_address(fixture.owner).0;
    let bob_owner_deny = host::deny_subject_address(fixture.bob_owner).0;
    let compute_deny = host::deny_subject_address(fixture.compute_signer).0;
    for record in [
        alice_deny,
        bob_deny,
        charlie_token_deny,
        charlie_owner_deny,
        owner_deny,
        bob_owner_deny,
        compute_deny,
    ] {
        accounts.insert(record, system_account(0));
    }
    let context = mollusk().with_context(accounts);
    let receipt_address = fixture.transferred_amount_value_address(fixture.alice_token);

    // First transfer (create): the sender authority plus every created subject of the
    // transferred-amount value are deny-checked.
    context.process_and_validate_instruction(
        &confidential_transfer_ix_with_remaining(
            &fixture,
            fixture.alice_token,
            fixture.bob_token,
            fixture.alice_balance_value,
            fixture.bob_balance_value,
            amount_attestation_for(
                handle_for_chain(21, BALANCE_FHE_TYPE),
                fixture.owner,
                fixture.compute_signer,
            ),
            vec![alice_deny, owner_deny, bob_owner_deny, compute_deny],
        ),
        &[Check::success()],
    );

    // Second transfer (rotation): the sender authority and rotation-added subject charlie_owner
    // each need a non-denied witness. Charlie's balance update preserves its audience.
    context.process_and_validate_instruction(
        &confidential_transfer_ix_with_remaining(
            &fixture,
            fixture.alice_token,
            charlie_token,
            fixture.alice_balance_value,
            charlie_balance_value,
            amount_attestation_for(
                handle_for_chain(22, BALANCE_FHE_TYPE),
                fixture.owner,
                fixture.compute_signer,
            ),
            vec![alice_deny, charlie_owner_deny],
        ),
        &[Check::success()],
    );

    let receipt = read_encrypted_value(&context, receipt_address);
    assert_eq!(
        receipt.subjects,
        vec![fixture.owner, charlie_owner, fixture.compute_signer]
    );
}

#[test]
fn mollusk_confidential_transfer_deny_list_rejects_denied_rotation_added_subject() {
    // Deny-list ENABLED + rotation where the added recipient's owner IS denied: the transfer must
    // fail with the deny error (not InvalidFheExecuteAccount from an unconsumed remaining account).
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    accounts.insert(
        fixture.host_config,
        deny_enabled_host_config_account(
            fixture.owner,
            secp_evm_address(&coprocessor_signing_key()),
        ),
    );
    let (charlie_owner, charlie_token, charlie_balance_value) = seed_third_account(
        &fixture,
        &mut accounts,
        handle_for_chain(3, BALANCE_FHE_TYPE),
    );

    let alice_deny = host::deny_subject_address(fixture.alice_token).0;
    let bob_deny = host::deny_subject_address(fixture.bob_token).0;
    let charlie_token_deny = host::deny_subject_address(charlie_token).0;
    let charlie_owner_deny = host::deny_subject_address(charlie_owner).0;
    let owner_deny = host::deny_subject_address(fixture.owner).0;
    let bob_owner_deny = host::deny_subject_address(fixture.bob_owner).0;
    let compute_deny = host::deny_subject_address(fixture.compute_signer).0;
    for record in [
        alice_deny,
        bob_deny,
        charlie_token_deny,
        owner_deny,
        bob_owner_deny,
        compute_deny,
    ] {
        accounts.insert(record, system_account(0));
    }
    // charlie_owner is denied.
    accounts.insert(
        charlie_owner_deny,
        deny_subject_record_account(charlie_owner, true).1,
    );
    let context = mollusk().with_context(accounts);
    let receipt_address = fixture.transferred_amount_value_address(fixture.alice_token);

    context.process_and_validate_instruction(
        &confidential_transfer_ix_with_remaining(
            &fixture,
            fixture.alice_token,
            fixture.bob_token,
            fixture.alice_balance_value,
            fixture.bob_balance_value,
            amount_attestation_for(
                handle_for_chain(21, BALANCE_FHE_TYPE),
                fixture.owner,
                fixture.compute_signer,
            ),
            vec![alice_deny, owner_deny, bob_owner_deny, compute_deny],
        ),
        &[Check::success()],
    );

    context.process_and_validate_instruction(
        &confidential_transfer_ix_with_remaining(
            &fixture,
            fixture.alice_token,
            charlie_token,
            fixture.alice_balance_value,
            charlie_balance_value,
            amount_attestation_for(
                handle_for_chain(22, BALANCE_FHE_TYPE),
                fixture.owner,
                fixture.compute_signer,
            ),
            vec![alice_deny, charlie_owner_deny],
        ),
        &[host_error(host::errors::ZamaHostError::SubjectDenied)],
    );

    // The denied rotation left the receipt at its first-transfer audience.
    let receipt = read_encrypted_value(&context, receipt_address);
    assert_eq!(
        receipt.subjects,
        vec![fixture.owner, fixture.bob_owner, fixture.compute_signer]
    );
}

#[test]
fn mollusk_confidential_transfer_with_deny_list_succeeds_when_neither_authority_is_denied() {
    let fixture = TokenFixture::new();
    let (accounts, deny_records) = deny_enabled_transfer_accounts(&fixture, None);
    let context = mollusk().with_context(accounts);
    let amount_handle = handle_for_chain(23, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let ix = confidential_transfer_ix_with_remaining(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
        deny_records,
    );

    context.process_and_validate_instruction(&ix, &[Check::success()]);

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    let bob_balance = read_encrypted_value(&context, fixture.bob_balance_value);
    assert_ne!(alice_balance.current_handle, fixture.alice_initial);
    assert_ne!(bob_balance.current_handle, fixture.bob_initial);
    assert!(!account_is_system_owned_and_empty(
        &context,
        fixture.transferred_amount_value_address(fixture.alice_token)
    ));
}

#[test]
fn mollusk_confidential_transfer_with_deny_list_rejects_denied_from_authority() {
    let fixture = TokenFixture::new();
    let (accounts, deny_records) =
        deny_enabled_transfer_accounts(&fixture, Some(fixture.alice_token));
    let context = mollusk().with_context(accounts);
    let amount_handle = handle_for_chain(24, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let ix = confidential_transfer_ix_with_remaining(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
        deny_records,
    );

    context.process_and_validate_instruction(
        &ix,
        &[host_error(host::errors::ZamaHostError::SubjectDenied)],
    );

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    let bob_balance = read_encrypted_value(&context, fixture.bob_balance_value);
    assert_eq!(alice_balance.current_handle, fixture.alice_initial);
    assert_eq!(bob_balance.current_handle, fixture.bob_initial);
    assert!(account_is_system_owned_and_empty(
        &context,
        fixture.transferred_amount_value_address(fixture.alice_token)
    ));
}

#[test]
fn mollusk_confidential_transfer_with_deny_list_allows_denied_non_grant_output_authority() {
    let fixture = TokenFixture::new();
    let (accounts, deny_records) =
        deny_enabled_transfer_accounts(&fixture, Some(fixture.bob_token));
    let context = mollusk().with_context(accounts);
    let amount_handle = handle_for_chain(25, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let ix = confidential_transfer_ix_with_remaining(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
        deny_records,
    );

    context.process_and_validate_instruction(&ix, &[Check::success()]);

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    let bob_balance = read_encrypted_value(&context, fixture.bob_balance_value);
    assert_ne!(alice_balance.current_handle, fixture.alice_initial);
    assert_ne!(bob_balance.current_handle, fixture.bob_initial);
    assert!(!account_is_system_owned_and_empty(
        &context,
        fixture.transferred_amount_value_address(fixture.alice_token)
    ));
}

// ---------------------------------------------------------------------------
// Registered coprocessor signer set + threshold (EVM InputVerifier parity)
// ---------------------------------------------------------------------------

/// Base accounts with the singleton `host_config` overridden to carry `coprocessor_signers` at
/// `threshold` (n-of-m), keeping every other account identical to `base_accounts`.
fn accounts_with_coprocessor_set(
    fixture: &TokenFixture,
    coprocessor_signers: &[[u8; 20]],
    threshold: u8,
) -> HashMap<Pubkey, Account> {
    let mut accounts = fixture.base_accounts();
    accounts.insert(
        fixture.host_config,
        host_config_account_with_signer_set(fixture.owner, coprocessor_signers, threshold),
    );
    accounts
}

/// Runs a transfer of `amount` whose attestation is signed by `signing_keys`, against a config that
/// registers `registered_keys` at `threshold`, and validates against `checks`.
fn run_multisig_transfer(
    registered_keys: &[k256::ecdsa::SigningKey],
    threshold: u8,
    signing_keys: &[k256::ecdsa::SigningKey],
    amount_seed: u8,
    checks: &[Check],
) -> InstructionResult {
    let fixture = TokenFixture::new();
    let registered: Vec<[u8; 20]> = registered_keys.iter().map(secp_evm_address).collect();
    let context = mollusk().with_context(accounts_with_coprocessor_set(
        &fixture,
        &registered,
        threshold,
    ));
    let amount_handle = handle_for_chain(amount_seed, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_signed_by(
        amount_handle,
        fixture.owner,
        fixture.compute_signer,
        signing_keys,
    );
    let transfer = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
    );
    context.process_and_validate_instruction(&transfer, checks)
}

#[test]
fn mollusk_confidential_transfer_two_of_three_accepts_exactly_threshold_signatures() {
    // 2-of-3: two valid signatures from registered signers clear the threshold.
    let keys = [
        coprocessor_signing_key_n(0x41),
        coprocessor_signing_key_n(0x42),
        coprocessor_signing_key_n(0x43),
    ];
    let result = run_multisig_transfer(&keys, 2, &keys[..2], 60, &[Check::success()]);
    assert!(result.raw_result.is_ok());
}

#[test]
fn mollusk_confidential_transfer_two_of_three_rejects_below_threshold_signatures() {
    // 2-of-3 with a single valid signature is below threshold: the host rejects the attestation.
    let keys = [
        coprocessor_signing_key_n(0x41),
        coprocessor_signing_key_n(0x42),
        coprocessor_signing_key_n(0x43),
    ];
    run_multisig_transfer(
        &keys,
        2,
        &keys[..1],
        61,
        &[host_error(
            host::errors::ZamaHostError::InvalidInputAttestation,
        )],
    );
}

#[test]
fn mollusk_confidential_transfer_rejects_signature_from_unregistered_signer() {
    // 2-of-3: one registered signature + one from a signer outside the set. The signature count is
    // 2, but only one recovers to a registered signer, so the distinct-in-set count is below the
    // threshold and the attestation is rejected.
    let keys = [
        coprocessor_signing_key_n(0x41),
        coprocessor_signing_key_n(0x42),
        coprocessor_signing_key_n(0x43),
    ];
    let outsider = coprocessor_signing_key_n(0x99);
    let signing = [keys[0].clone(), outsider];
    run_multisig_transfer(
        &keys,
        2,
        &signing,
        62,
        &[host_error(
            host::errors::ZamaHostError::InvalidInputAttestation,
        )],
    );
}

#[test]
fn mollusk_confidential_transfer_duplicate_signature_does_not_count_twice() {
    // 2-of-3 with two signatures from the SAME registered signer counts as one distinct signer, so
    // the threshold is not met (verify_threshold counts DISTINCT recovered addresses).
    let keys = [
        coprocessor_signing_key_n(0x41),
        coprocessor_signing_key_n(0x42),
        coprocessor_signing_key_n(0x43),
    ];
    let signing = [keys[0].clone(), keys[0].clone()];
    run_multisig_transfer(
        &keys,
        2,
        &signing,
        63,
        &[host_error(
            host::errors::ZamaHostError::InvalidInputAttestation,
        )],
    );
}

/// Builds the full `confidential_transfer` legacy transaction carrying a threshold-4 attestation
/// (4 × 65-byte signatures) over the real token account list, and asserts the bincode-serialized
/// `Transaction` stays within the Solana packet limit. This pins the transaction-size ceiling the
/// day multi-coprocessor input verification (t > 1) lands: the carried signature payload scales with
/// the threshold, and a 4-of-m transfer is the heaviest realistic case.
#[test]
fn confidential_transfer_with_threshold_four_attestation_fits_in_one_packet() {
    use solana_sdk::message::Message;
    use solana_sdk::transaction::Transaction;

    let fixture = TokenFixture::new();
    let keys: Vec<k256::ecdsa::SigningKey> = (0..4)
        .map(|i| coprocessor_signing_key_n(0x41 + i))
        .collect();
    let amount_handle = handle_for_chain(70, BALANCE_FHE_TYPE);
    // Four signatures — a threshold-4 attestation (payload scales with t, not the set size).
    let attestation =
        amount_attestation_signed_by(amount_handle, fixture.owner, fixture.compute_signer, &keys);
    assert_eq!(attestation.signatures.len(), 4);

    let transfer = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
    );

    // A legacy transaction with unsigned (default) signatures: `new_unsigned` reserves one 64-byte
    // slot per required signer, so the bincode size already reflects the real wire size.
    let message = Message::new(&[transfer], Some(&fixture.owner));
    let tx = Transaction::new_unsigned(message);
    let serialized = bincode::serialize(&tx).expect("serialize transaction");
    eprintln!(
        "threshold-4 confidential_transfer tx: {} bytes (limit {})",
        serialized.len(),
        solana_packet::PACKET_DATA_SIZE
    );

    assert!(
        serialized.len() <= solana_packet::PACKET_DATA_SIZE,
        "threshold-4 confidential_transfer tx is {} bytes, exceeds the {}-byte packet limit",
        serialized.len(),
        solana_packet::PACKET_DATA_SIZE,
    );
}

#[test]
fn mollusk_confidential_transfer_rejects_owner_mismatch() {
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_handle = handle_for_chain(30, BALANCE_FHE_TYPE);
    // Attestation correctly authored by bob, but bob is not `from_account`'s owner: the
    // instruction's own owner-signer check must reject this before any ACL work happens.
    let attestation =
        amount_attestation_for(amount_handle, fixture.bob_owner, fixture.compute_signer);
    let mut ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
    );
    // Sign as bob (matches the attestation) so only the owner-mismatch check can fail.
    for meta in ix.accounts.iter_mut() {
        if meta.pubkey == fixture.owner {
            meta.pubkey = fixture.bob_owner;
        }
    }

    context.process_and_validate_instruction(
        &ix,
        &[token_error(token::ConfidentialTokenError::OwnerMismatch)],
    );

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    assert_eq!(alice_balance.current_handle, fixture.alice_initial);
}

#[test]
fn mollusk_confidential_transfer_rejects_attestation_user_mismatch() {
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_handle = handle_for_chain(31, BALANCE_FHE_TYPE);
    // fromExternal binding: an attestation authored by someone other than the transfer authority
    // (owner) must be rejected before any balance update.
    let attestation =
        amount_attestation_for(amount_handle, fixture.bob_owner, fixture.compute_signer);
    let ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
    );

    context.process_and_validate_instruction(
        &ix,
        &[token_error(
            token::ConfidentialTokenError::AttestationUserMismatch,
        )],
    );

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    let bob_balance = read_encrypted_value(&context, fixture.bob_balance_value);
    assert_eq!(alice_balance.current_handle, fixture.alice_initial);
    assert_eq!(bob_balance.current_handle, fixture.bob_initial);
    assert!(account_is_system_owned_and_empty(
        &context,
        fixture.transferred_amount_value_address(fixture.alice_token)
    ));
}

#[test]
fn mollusk_confidential_transfer_rejects_attestation_contract_mismatch() {
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_handle = handle_for_chain(32, BALANCE_FHE_TYPE);
    // fromExternal binding: an attestation bound to a contract other than the mint compute-signer
    // PDA must be rejected before any balance update.
    let attestation = amount_attestation_for(amount_handle, fixture.owner, Pubkey::new_unique());
    let ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
    );

    context.process_and_validate_instruction(
        &ix,
        &[token_error(
            token::ConfidentialTokenError::AttestationContractMismatch,
        )],
    );

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    let bob_balance = read_encrypted_value(&context, fixture.bob_balance_value);
    assert_eq!(alice_balance.current_handle, fixture.alice_initial);
    assert_eq!(bob_balance.current_handle, fixture.bob_initial);
}

#[test]
fn mollusk_confidential_transfer_rejects_stale_balance_encrypted_value() {
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_handle = handle_for_chain(33, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    // A balance `EncryptedValue` address that does not match `from_account.balance_encrypted_value`
    // must be rejected by the account's `address = from_account.balance_encrypted_value` constraint.
    let (stale_address, stale_value) = new_encrypted_value(
        fixture.mint,
        fixture.alice_token,
        token::encrypted_transfer_amount_label(),
        fixture.alice_initial,
        &[fixture.owner],
    );
    context
        .account_store
        .borrow_mut()
        .insert(stale_address, encrypted_value_account(&stale_value));
    let ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        stale_address,
        fixture.bob_balance_value,
        attestation,
    );

    context.process_and_validate_instruction(
        &ix,
        &[anchor_error(
            anchor_lang::error::ErrorCode::ConstraintAddress,
        )],
    );

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    assert_eq!(alice_balance.current_handle, fixture.alice_initial);
}

#[test]
fn mollusk_confidential_transfer_rejects_balance_wrong_mint_acl_domain() {
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    let wrong_mint = Pubkey::new_unique();
    let (_, mut wrong_domain_value) = new_encrypted_value(
        fixture.mint,
        fixture.alice_token,
        token::encrypted_balance_label(),
        fixture.alice_initial,
        &[fixture.owner, fixture.compute_signer],
    );
    wrong_domain_value.domain = wrong_mint;
    accounts.insert(
        fixture.alice_balance_value,
        encrypted_value_account(&wrong_domain_value),
    );
    let context = mollusk().with_context(accounts);
    let amount_handle = handle_for_chain(34, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
    );

    context.process_and_validate_instruction(
        &ix,
        &[host_error(
            host::errors::ZamaHostError::EncryptedValuePdaMismatch,
        )],
    );

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    let bob_balance = read_encrypted_value(&context, fixture.bob_balance_value);
    assert_eq!(alice_balance.domain, wrong_mint);
    assert_eq!(alice_balance.current_handle, fixture.alice_initial);
    assert_eq!(bob_balance.current_handle, fixture.bob_initial);
    assert!(account_is_system_owned_and_empty(
        &context,
        fixture.transferred_amount_value_address(fixture.alice_token)
    ));
}

#[test]
fn mollusk_confidential_transfer_rejects_balance_wrong_token_account_authority() {
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    let wrong_token_account = Pubkey::new_unique();
    let (_, mut wrong_account_value) = new_encrypted_value(
        fixture.mint,
        fixture.alice_token,
        token::encrypted_balance_label(),
        fixture.alice_initial,
        &[fixture.owner, fixture.compute_signer],
    );
    wrong_account_value.encrypted_value_account_authority = wrong_token_account;
    accounts.insert(
        fixture.alice_balance_value,
        encrypted_value_account(&wrong_account_value),
    );
    let context = mollusk().with_context(accounts);
    let amount_handle = handle_for_chain(35, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
    );

    context.process_and_validate_instruction(
        &ix,
        &[host_error(
            host::errors::ZamaHostError::EncryptedValuePdaMismatch,
        )],
    );

    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    let bob_balance = read_encrypted_value(&context, fixture.bob_balance_value);
    assert_eq!(
        alice_balance.encrypted_value_account_authority,
        wrong_token_account
    );
    assert_eq!(alice_balance.current_handle, fixture.alice_initial);
    assert_eq!(bob_balance.current_handle, fixture.bob_initial);
    assert!(account_is_system_owned_and_empty(
        &context,
        fixture.transferred_amount_value_address(fixture.alice_token)
    ));
}

// ---------------------------------------------------------------------------
// confidential_burn -> redeem_burned_amount
//
// The BurnRedemptionRequest witness lifecycle was dissolved (fhevm-internal#1763): redeem is now a
// single thin consumer of the stateless host `verify_public_decrypt`, verifying the KMS cert
// against the live KMS context it names (any non-destroyed context, fhevm-internal#1765) plus an
// exact-handle MMR public-decrypt proof, then paying out
// and closing the token account's `PendingBurn` (rent to owner).
//
// Vector 2 (burn-stranding) fix, unchanged: every burn is created publicly decryptable at the burn
// instant (ERC-7984 `unwrap` parity, DD-036), so a historical burned handle stays redeemable even
// after a later burn updates the shared `burned_amount` encrypted value account.
// ---------------------------------------------------------------------------

use anchor_spl::token::spl_token;
use anchor_spl::token_2022::spl_token_2022;
use solana_sdk::program_option::COption;

use zama_solana_test_kit::kms::{cleartext_u256, kms_signing_key, kms_signing_key_n};

/// Builds a KMS `PublicDecryptVerification` secp256k1 cert (`signatures`, `extra_data`)
/// over `handle`/`cleartext_amount`, verified by the host `verify_public_decrypt` CPI.
/// `extra_data == [0x00]` is a v0 cert that binds only through the current context's signer set.
fn kms_public_decrypt_cert(handle: [u8; 32], cleartext_amount: u64) -> (Vec<[u8; 65]>, Vec<u8>) {
    kms_public_decrypt_cert_signed_by(handle, cleartext_amount, &[kms_signing_key()])
}

/// Like `kms_public_decrypt_cert`, but produces one signature per key in `keys` (t-of-n cert
/// building — the carried payload scales with the threshold t, not the party count n).
fn kms_public_decrypt_cert_signed_by(
    handle: [u8; 32],
    cleartext_amount: u64,
    keys: &[k256::ecdsa::SigningKey],
) -> (Vec<[u8; 65]>, Vec<u8>) {
    let extra_data = vec![0x00u8];
    let signatures = zama_solana_test_kit::kms::kms_public_decrypt_cert_signed_by(
        handle,
        cleartext_u256(cleartext_amount),
        GATEWAY_CHAIN_ID,
        &DECRYPTION_CONTRACT,
        &extra_data,
        keys,
    );
    (signatures, extra_data)
}

/// A cert committing an explicit KMS context id via v1 `extra_data` (EVM `_extractContextId`
/// parity), for the rotation-grace tests: a cert minted under an old-but-still-live context.
fn kms_public_decrypt_cert_for_context(
    handle: [u8; 32],
    cleartext_amount: u64,
    context_id: u64,
) -> (Vec<[u8; 65]>, Vec<u8>) {
    let extra_data = zama_solana_test_kit::kms::context_extra_data_v1(context_id);
    let signatures = zama_solana_test_kit::kms::kms_public_decrypt_cert_signed_by(
        handle,
        cleartext_u256(cleartext_amount),
        GATEWAY_CHAIN_ID,
        &DECRYPTION_CONTRACT,
        &extra_data,
        &[kms_signing_key()],
    );
    (signatures, extra_data)
}

fn kms_context_account(context_id: u64) -> Account {
    kms_context_account_with_signers(context_id, &[secp_evm_address(&kms_signing_key())], 1)
}

/// Like [`kms_context_account`] but marked `destroyed`, for the revocation-lever test.
fn destroyed_kms_context_account(context_id: u64) -> Account {
    let (_, bump) = host::kms_context_address(context_id);
    Account {
        lamports: 1_000_000_000,
        data: serialized_account(host::KmsContext {
            context_id,
            signers: vec![secp_evm_address(&kms_signing_key())],
            thresholds: host::KmsThresholds {
                public_decryption: 1,
                user_decryption: 1,
                kms_gen: 1,
                mpc: 1,
            },
            destroyed: true,
            bump,
        }),
        owner: host::id(),
        executable: false,
        rent_epoch: 0,
    }
}

/// Builds a `KmsContext` account registering `signers` with `public_decryption` threshold set to
/// `public_threshold` (the other thresholds are pinned to a satisfiable value for the set).
fn kms_context_account_with_signers(
    context_id: u64,
    signers: &[[u8; 20]],
    public_threshold: u8,
) -> Account {
    let (_, bump) = host::kms_context_address(context_id);
    Account {
        lamports: 1_000_000_000,
        data: serialized_account(host::KmsContext {
            context_id,
            signers: signers.to_vec(),
            thresholds: host::KmsThresholds {
                public_decryption: public_threshold,
                user_decryption: 1,
                kms_gen: 1,
                mpc: 1,
            },
            destroyed: false,
            bump,
        }),
        owner: host::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn spl_mint_account(decimals: u8) -> Account {
    let mut data = vec![0u8; spl_token::state::Mint::LEN];
    spl_token::state::Mint::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 1_000_000,
            decimals,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        &mut data,
    )
    .unwrap();
    Account {
        lamports: 1_000_000_000,
        data,
        owner: spl_token::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn spl_token_account(mint: Pubkey, owner: Pubkey, amount: u64) -> Account {
    let mut data = vec![0u8; spl_token::state::Account::LEN];
    spl_token::state::Account::pack(
        spl_token::state::Account {
            mint,
            owner,
            amount,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        &mut data,
    )
    .unwrap();
    Account {
        lamports: 1_000_000_000,
        data,
        owner: spl_token::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn token_2022_mint_account(decimals: u8) -> Account {
    let mut data = vec![0u8; spl_token_2022::state::Mint::LEN];
    spl_token_2022::state::Mint::pack(
        spl_token_2022::state::Mint {
            mint_authority: COption::None,
            supply: 1_000_000,
            decimals,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        &mut data,
    )
    .unwrap();
    Account {
        lamports: 1_000_000_000,
        data,
        owner: spl_token_2022::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn token_2022_non_transferable_mint_account(decimals: u8) -> Account {
    use spl_token_2022::extension::{
        non_transferable::NonTransferable, BaseStateWithExtensionsMut, ExtensionType,
        StateWithExtensionsMut,
    };
    let len = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(&[
        ExtensionType::NonTransferable,
    ])
    .unwrap();
    let mut data = vec![0u8; len];
    let mut state =
        StateWithExtensionsMut::<spl_token_2022::state::Mint>::unpack_uninitialized(&mut data)
            .unwrap();
    state.base = spl_token_2022::state::Mint {
        mint_authority: COption::None,
        supply: 1_000_000,
        decimals,
        is_initialized: true,
        freeze_authority: COption::None,
    };
    state.init_extension::<NonTransferable>(true).unwrap();
    state.init_account_type().unwrap();
    state.pack_base();
    Account {
        lamports: 1_000_000_000,
        data,
        owner: spl_token_2022::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn token_2022_token_account(
    mint: Pubkey,
    owner: Pubkey,
    amount: u64,
    state: spl_token_2022::state::AccountState,
) -> Account {
    let mut data = vec![0u8; spl_token_2022::state::Account::LEN];
    spl_token_2022::state::Account::pack(
        spl_token_2022::state::Account {
            mint,
            owner,
            amount,
            delegate: COption::None,
            state,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        &mut data,
    )
    .unwrap();
    Account {
        lamports: 1_000_000_000,
        data,
        owner: spl_token_2022::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn token_2022_immutable_owner_account(
    mint: Pubkey,
    owner: Pubkey,
    amount: u64,
    account_state: spl_token_2022::state::AccountState,
) -> Account {
    use spl_token_2022::extension::{
        immutable_owner::ImmutableOwner, BaseStateWithExtensionsMut, ExtensionType,
        StateWithExtensionsMut,
    };
    let len = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Account>(&[
        ExtensionType::ImmutableOwner,
    ])
    .unwrap();
    let mut data = vec![0u8; len];
    let mut state =
        StateWithExtensionsMut::<spl_token_2022::state::Account>::unpack_uninitialized(&mut data)
            .unwrap();
    state.base = spl_token_2022::state::Account {
        mint,
        owner,
        amount,
        delegate: COption::None,
        state: account_state,
        is_native: COption::None,
        delegated_amount: 0,
        close_authority: COption::None,
    };
    state.init_extension::<ImmutableOwner>(true).unwrap();
    state.init_account_type().unwrap();
    state.pack_base();
    Account {
        lamports: 1_000_000_000,
        data,
        owner: spl_token_2022::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn token_2022_cpi_guard_account(mint: Pubkey, owner: Pubkey, amount: u64) -> Account {
    use spl_token_2022::extension::{
        cpi_guard::CpiGuard, BaseStateWithExtensionsMut, ExtensionType, StateWithExtensionsMut,
    };
    let len = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Account>(&[
        ExtensionType::CpiGuard,
    ])
    .unwrap();
    let mut data = vec![0u8; len];
    let mut state =
        StateWithExtensionsMut::<spl_token_2022::state::Account>::unpack_uninitialized(&mut data)
            .unwrap();
    state.base = spl_token_2022::state::Account {
        mint,
        owner,
        amount,
        delegate: COption::None,
        state: spl_token_2022::state::AccountState::Initialized,
        is_native: COption::None,
        delegated_amount: 0,
        close_authority: COption::None,
    };
    state.init_extension::<CpiGuard>(true).unwrap();
    state.init_account_type().unwrap();
    state.pack_base();
    Account {
        lamports: 1_000_000_000,
        data,
        owner: spl_token_2022::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn read_spl_amount(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> u64 {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("missing spl token account")
        .clone();
    if account.owner == spl_token::id() {
        spl_token::state::Account::unpack(&account.data)
            .expect("valid classic token account")
            .amount
    } else {
        use spl_token_2022::extension::StateWithExtensions;
        StateWithExtensions::<spl_token_2022::state::Account>::unpack(&account.data)
            .expect("valid Token-2022 account")
            .base
            .amount
    }
}

/// Self-contained fixture for the burn/redeem/cancel vertical: one owner, one
/// confidential mint with an SPL-backed vault, and one funded token account.
struct BurnRedeemFixture {
    owner: Pubkey,
    mint: Pubkey,
    compute_signer: Pubkey,
    host_config: Pubkey,
    token_account: Pubkey,
    balance_value: Pubkey,
    total_supply_authority: Pubkey,
    total_supply_value: Pubkey,
    burned_amount_value: Pubkey,
    underlying_mint: Pubkey,
    token_program: Pubkey,
    vault_authority: Pubkey,
    vault_usdc: Pubkey,
    destination_usdc: Pubkey,
    kms_context_id: u64,
    kms_context: Pubkey,
    initial_balance: [u8; 32],
    initial_total_supply: [u8; 32],
}

impl BurnRedeemFixture {
    fn new() -> Self {
        Self::with_keys(
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
        )
    }

    fn new_token_2022() -> Self {
        Self::with_keys_and_token_program(
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            spl_token_2022::id(),
        )
    }

    /// Fixed-key variant for cost snapshots and the PDA-owner CPI-driver test: PDA bump searches are
    /// part of the measured compute, and the owner must be a chosen program PDA, so the addresses
    /// must not change between runs.
    fn with_keys(owner: Pubkey, mint: Pubkey, underlying_mint: Pubkey) -> Self {
        Self::with_keys_and_token_program(owner, mint, underlying_mint, spl_token::id())
    }

    fn with_keys_and_token_program(
        owner: Pubkey,
        mint: Pubkey,
        underlying_mint: Pubkey,
        token_program: Pubkey,
    ) -> Self {
        let compute_signer = token::compute_signer_address(mint).0;
        let host_config = host::host_config_address().0;
        let token_account = token::token_account_address(mint, owner).0;
        let balance_value = token::balance_encrypted_value_address(mint, token_account).0;
        let total_supply_authority = token::total_supply_authority_address(mint).0;
        let total_supply_value =
            token::total_supply_encrypted_value_address(mint, total_supply_authority).0;
        let burned_amount_value = token::encrypted_value_address(
            mint,
            token_account,
            token::encrypted_burned_amount_label(),
        )
        .0;
        let vault_authority = token::vault_authority_address(mint).0;
        let vault_usdc = token::vault_token_account_address(mint, underlying_mint, token_program);
        let destination_usdc = Pubkey::new_unique();
        let kms_context_id = 9;
        let kms_context = host::kms_context_address(kms_context_id).0;
        Self {
            owner,
            mint,
            compute_signer,
            host_config,
            token_account,
            balance_value,
            total_supply_authority,
            total_supply_value,
            burned_amount_value,
            underlying_mint,
            token_program,
            vault_authority,
            vault_usdc,
            destination_usdc,
            kms_context_id,
            kms_context,
            initial_balance: handle_for_chain(1, BALANCE_FHE_TYPE),
            initial_total_supply: handle_for_chain(2, BALANCE_FHE_TYPE),
        }
    }

    fn confidential_mint_account(&self) -> Account {
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(token::ConfidentialMint {
                authority: self.owner,
                domain: self.mint,
                compute_signer: self.compute_signer,
                underlying_mint: self.underlying_mint,
                decimals: 6,
                total_supply_encrypted_value: self.total_supply_value,
            }),
            owner: token::id(),
            executable: false,
            rent_epoch: 0,
        }
    }

    fn accounts(&self, vault_balance: u64) -> HashMap<Pubkey, Account> {
        let (_, balance_value) = new_encrypted_value(
            self.mint,
            self.token_account,
            token::encrypted_balance_label(),
            self.initial_balance,
            &[self.owner, self.compute_signer],
        );
        let (_, total_supply_value) = new_encrypted_value(
            self.mint,
            self.total_supply_authority,
            token::encrypted_total_supply_label(),
            self.initial_total_supply,
            &[self.compute_signer],
        );
        HashMap::from([
            (self.owner, system_account(50_000_000_000)),
            (self.mint, self.confidential_mint_account()),
            (self.compute_signer, system_account(0)),
            (self.total_supply_authority, system_account(0)),
            (self.vault_authority, system_account(0)),
            (
                self.host_config,
                host_config_account_with_kms_context(
                    self.owner,
                    secp_evm_address(&coprocessor_signing_key()),
                    self.kms_context_id,
                ),
            ),
            (self.kms_context, kms_context_account(self.kms_context_id)),
            (
                self.token_account,
                token_account_account(self.mint, self.owner, self.balance_value),
            ),
            (self.balance_value, encrypted_value_account(&balance_value)),
            (
                self.total_supply_value,
                encrypted_value_account(&total_supply_value),
            ),
            (self.burned_amount_value, system_account(0)),
            (
                self.underlying_mint,
                if self.token_program == spl_token::id() {
                    spl_mint_account(6)
                } else {
                    token_2022_mint_account(6)
                },
            ),
            (
                self.vault_usdc,
                if self.token_program == spl_token::id() {
                    spl_token_account(self.underlying_mint, self.vault_authority, vault_balance)
                } else {
                    token_2022_immutable_owner_account(
                        self.underlying_mint,
                        self.vault_authority,
                        vault_balance,
                        spl_token_2022::state::AccountState::Initialized,
                    )
                },
            ),
            (
                self.destination_usdc,
                if self.token_program == spl_token::id() {
                    spl_token_account(self.underlying_mint, self.owner, 0)
                } else {
                    token_2022_immutable_owner_account(
                        self.underlying_mint,
                        self.owner,
                        0,
                        spl_token_2022::state::AccountState::Initialized,
                    )
                },
            ),
            (event_authority(host::id()), system_account(0)),
            (event_authority(token::id()), system_account(0)),
            mollusk_svm_programs_token::token::keyed_account(),
            mollusk_svm_programs_token::token2022::keyed_account(),
        ])
    }
}

fn token_account_account(mint: Pubkey, owner: Pubkey, balance_value: Pubkey) -> Account {
    Account {
        lamports: 1_000_000_000,
        data: serialized_account(token::ConfidentialTokenAccount {
            owner,
            mint,
            balance_encrypted_value: balance_value,
            bump: token::token_account_address(mint, owner).1,
        }),
        owner: token::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn burn_redeem_mollusk() -> Mollusk {
    mollusk()
}

fn pending_burn_account(pending: token::PendingBurn) -> Account {
    Account {
        lamports: 1_000_000,
        data: serialized_account(pending),
        owner: token::id(),
        executable: false,
        rent_epoch: 0,
    }
}

/// Seeds an initialized `PendingBurn` account (for redeem-only tests that skip a real burn).
fn seed_pending_burn_in_context(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    fixture: &BurnRedeemFixture,
    burned_handle: [u8; 32],
) -> Pubkey {
    let (address, bump) = token::pending_burn_address(fixture.mint, fixture.token_account);
    let pending = token::PendingBurn {
        mint: fixture.mint,
        owner: fixture.owner,
        token_account: fixture.token_account,
        burned_handle,
        burned_encrypted_value: fixture.burned_amount_value,
        bump,
    };
    context
        .account_store
        .borrow_mut()
        .insert(address, pending_burn_account(pending));
    address
}

fn prepare_empty_pending_burn(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    fixture: &BurnRedeemFixture,
) -> Pubkey {
    let pending_burn = token::pending_burn_address(fixture.mint, fixture.token_account).0;
    context
        .account_store
        .borrow_mut()
        .insert(pending_burn, system_account(0));
    pending_burn
}

fn assert_pending_burn_closed(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    pending_burn: Pubkey,
) {
    let account = context.account_store.borrow().get(&pending_burn).cloned();
    match account {
        None => {}
        Some(account) => {
            assert_eq!(account.owner, system_program::ID);
            assert!(account.data.is_empty());
        }
    }
}

fn confidential_burn_ix(
    fixture: &BurnRedeemFixture,
    amount_attestation: host::CoprocessorInputAttestation,
    pending_burn: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::ConfidentialBurn {
            owner: fixture.owner,
            mint: fixture.mint,
            token_account: fixture.token_account,
            compute_signer: fixture.compute_signer,
            total_supply_authority: fixture.total_supply_authority,
            balance_value: fixture.balance_value,
            total_supply_value: fixture.total_supply_value,
            burned_amount_value: fixture.burned_amount_value,
            pending_burn,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::ConfidentialBurn { amount_attestation },
    )
}

/// Builds a `confidential_burn_from_value` instruction: the amount is taken from the existing
/// on-chain `EncryptedValue` at `amount_value` (a computed or received handle) rather than a fresh
/// attestation. `owner` signs as the burn authority (it must own `token_account` and be in the
/// amount value's subject set) and `payer` pays rent; splitting them lets `owner` be a program PDA.
fn confidential_burn_from_value_ix(
    fixture: &BurnRedeemFixture,
    owner: Pubkey,
    payer: Pubkey,
    amount_value: Pubkey,
    pending_burn: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::ConfidentialBurnFromValue {
            owner,
            payer,
            mint: fixture.mint,
            token_account: fixture.token_account,
            compute_signer: fixture.compute_signer,
            total_supply_authority: fixture.total_supply_authority,
            balance_value: fixture.balance_value,
            total_supply_value: fixture.total_supply_value,
            burned_amount_value: fixture.burned_amount_value,
            pending_burn,
            amount_value,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::ConfidentialBurnFromValue {},
    )
}

/// Seeds a spendable amount encrypted value account (a stand-in for a computed/received `euint64` handle) at the
/// canonical PDA `(mint, account, label)` with the given subjects and current handle into the
/// burn fixture's account map, returning its address.
fn confidential_burn_from_value_auto(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    fixture: &BurnRedeemFixture,
    owner: Pubkey,
    payer: Pubkey,
    amount_value: Pubkey,
) -> Instruction {
    let pending_burn = prepare_empty_pending_burn(context, fixture);
    confidential_burn_from_value_ix(fixture, owner, payer, amount_value, pending_burn)
}

fn seed_burn_amount_value(
    fixture: &BurnRedeemFixture,
    accounts: &mut HashMap<Pubkey, Account>,
    account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subjects: &[Pubkey],
) -> Pubkey {
    let (address, value) = new_encrypted_value(
        fixture.mint,
        account,
        encrypted_value_label,
        handle,
        subjects,
    );
    accounts.insert(address, encrypted_value_account(&value));
    address
}

/// Builds a single-leaf public-decrypt inclusion proof for `fixture.burned_amount_value` after one
/// burn (the sole leaf, at index 0, is `burned_handle`'s public-decrypt commitment), returning it
/// with the encrypted value account peaks the burn wrote.
fn single_burn_public_decrypt_proof(
    fixture: &BurnRedeemFixture,
    burned_handle: [u8; 32],
) -> host::instructions::MmrInclusionProof {
    let acct = fixture.burned_amount_value.to_bytes();
    let leaves = vec![zama_solana_acl::public_decrypt_leaf_commitment(
        acct,
        0,
        burned_handle,
    )];
    let proof = zama_solana_acl::mmr_build_proof(&leaves, 0).expect("proof for the sole burn leaf");
    host::instructions::MmrInclusionProof {
        leaf_index: proof.leaf_index,
        siblings: proof.siblings,
    }
}

fn seed_single_burn_value_account(
    fixture: &BurnRedeemFixture,
    accounts: &mut HashMap<Pubkey, Account>,
    burned_handle: [u8; 32],
) {
    let (_, mut value) = new_encrypted_value(
        fixture.mint,
        fixture.token_account,
        token::encrypted_burned_amount_label(),
        burned_handle,
        &[fixture.owner, fixture.compute_signer],
    );
    value.leaf_count = 1;
    value.peaks =
        zama_solana_acl::mmr_peaks_from_leaves(&[zama_solana_acl::public_decrypt_leaf_commitment(
            fixture.burned_amount_value.to_bytes(),
            0,
            burned_handle,
        )]);
    accounts.insert(fixture.burned_amount_value, encrypted_value_account(&value));
}

#[allow(clippy::too_many_arguments)]
fn redeem_burned_amount_ix(
    fixture: &BurnRedeemFixture,
    burned_handle: [u8; 32],
    cleartext_amount: u64,
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
    proof: host::instructions::MmrInclusionProof,
    pending_burn: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::RedeemBurnedAmount {
            owner: fixture.owner,
            mint: fixture.mint,
            token_account: fixture.token_account,
            underlying_mint: fixture.underlying_mint,
            vault_usdc: fixture.vault_usdc,
            destination_usdc: fixture.destination_usdc,
            vault_authority: fixture.vault_authority,
            burned_amount_value: fixture.burned_amount_value,
            pending_burn,
            host_config: fixture.host_config,
            kms_context: fixture.kms_context,
            zama_program: host::id(),
            token_program: fixture.token_program,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::RedeemBurnedAmount {
            burned_handle,
            cleartext_amount,
            signatures,
            extra_data,
            proof,
        },
    )
}

/// Burns `amount_seed`'s attested amount and returns the resulting burned handle.
fn run_burn(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    fixture: &BurnRedeemFixture,
    amount_seed: u8,
) -> [u8; 32] {
    let pending_burn = prepare_empty_pending_burn(context, fixture);
    let amount_handle = handle_for_chain(amount_seed, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let ix = confidential_burn_ix(fixture, attestation, pending_burn);
    context.process_and_validate_instruction(&ix, &[Check::success()]);
    read_encrypted_value(context, fixture.burned_amount_value).current_handle
}

#[test]
fn mollusk_confidential_burn_makes_burned_amount_publicly_decryptable() {
    let fixture = BurnRedeemFixture::new();
    let context = burn_redeem_mollusk().with_context(fixture.accounts(1_000));

    let burned_handle = run_burn(&context, &fixture, 41);

    // First burn creates the encrypted value account (no create leaf) and then appends exactly one
    // public-decrypt leaf for the just-bound burned handle.
    let value = read_encrypted_value(&context, fixture.burned_amount_value);
    assert_eq!(value.current_handle, burned_handle);
    assert_eq!(value.subjects, vec![fixture.owner, fixture.compute_signer]);
    assert_eq!(value.leaf_count, 1);
    let public_leaf = zama_solana_acl::public_decrypt_leaf_commitment(
        fixture.burned_amount_value.to_bytes(),
        0,
        burned_handle,
    );
    assert_eq!(
        value.peaks,
        zama_solana_acl::mmr_peaks_from_leaves(&[public_leaf])
    );
}

#[test]
fn mollusk_confidential_burn_accepts_prefunded_pending_pda() {
    for lamports in [1, 10_000_000_000] {
        let fixture = BurnRedeemFixture::new();
        let pending_burn = token::pending_burn_address(fixture.mint, fixture.token_account).0;
        let mut accounts = fixture.accounts(1_000);
        accounts.insert(pending_burn, system_account(lamports));
        let context = burn_redeem_mollusk().with_context(accounts);
        let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
        let attestation =
            amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);

        context.process_and_validate_instruction(
            &confidential_burn_ix(&fixture, attestation, pending_burn),
            &[Check::success()],
        );

        let pending = context
            .account_store
            .borrow()
            .get(&pending_burn)
            .expect("pending burn created")
            .clone();
        assert_eq!(pending.owner, token::id());
        assert!(!pending.data.is_empty());
    }
}

#[test]
fn mollusk_confidential_burn_rejects_occupied_pending_pda_atomically() {
    let fixture = BurnRedeemFixture::new();
    let pending_burn = token::pending_burn_address(fixture.mint, fixture.token_account).0;
    for occupied in [
        Account {
            lamports: 1,
            data: Vec::new(),
            owner: Pubkey::new_unique(),
            executable: false,
            rent_epoch: 0,
        },
        Account {
            lamports: 1,
            data: vec![0x01],
            owner: system_program::ID,
            executable: false,
            rent_epoch: 0,
        },
    ] {
        let mut accounts = fixture.accounts(1_000);
        accounts.insert(pending_burn, occupied.clone());
        let context = burn_redeem_mollusk().with_context(accounts);
        let old_balance = read_encrypted_value(&context, fixture.balance_value).current_handle;
        let old_supply = read_encrypted_value(&context, fixture.total_supply_value).current_handle;
        let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
        let attestation =
            amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);

        context.process_and_validate_instruction(
            &confidential_burn_ix(&fixture, attestation, pending_burn),
            &[token_error(
                token::ConfidentialTokenError::PendingBurnAlreadyInitialized,
            )],
        );

        assert_eq!(
            read_encrypted_value(&context, fixture.balance_value).current_handle,
            old_balance
        );
        assert_eq!(
            read_encrypted_value(&context, fixture.total_supply_value).current_handle,
            old_supply
        );
        let unchanged = context
            .account_store
            .borrow()
            .get(&pending_burn)
            .expect("occupied account remains")
            .clone();
        assert_eq!(unchanged.owner, occupied.owner);
        assert_eq!(unchanged.data, occupied.data);
    }
}

/// Reconstructs the burned_amount encrypted value account's four leaves after two burns
/// (public(H1)@0, hist(H1,owner)@1, hist(H1,compute)@2, public(H2)@3) and builds a
/// public-decrypt inclusion proof for the leaf at `leaf_index`, returning it with the
/// encrypted value account peaks. Leaf 0 proves the historical first burn; leaf 3 proves the current second.
fn two_burn_value_account_proof(
    fixture: &BurnRedeemFixture,
    first_handle: [u8; 32],
    second_handle: [u8; 32],
    leaf_index: u64,
) -> (host::instructions::MmrInclusionProof, Vec<[u8; 32]>) {
    let acct = fixture.burned_amount_value.to_bytes();
    let leaves = vec![
        zama_solana_acl::public_decrypt_leaf_commitment(acct, 0, first_handle),
        zama_solana_acl::historical_access_leaf_commitment(
            acct,
            1,
            first_handle,
            fixture.owner.to_bytes(),
        ),
        zama_solana_acl::historical_access_leaf_commitment(
            acct,
            2,
            first_handle,
            fixture.compute_signer.to_bytes(),
        ),
        zama_solana_acl::public_decrypt_leaf_commitment(acct, 3, second_handle),
    ];
    let proof =
        zama_solana_acl::mmr_build_proof(&leaves, leaf_index).expect("proof for requested leaf");
    (
        host::instructions::MmrInclusionProof {
            leaf_index: proof.leaf_index,
            siblings: proof.siblings,
        },
        zama_solana_acl::mmr_peaks_from_leaves(&leaves),
    )
}

/// Redeem the current pending burn, then reject a second redemption because the pending account is
/// already closed.
#[test]
fn mollusk_redeem_current_pending_burn_then_rejects_double_settlement() {
    let fixture = BurnRedeemFixture::new();
    let burned_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let proof = single_burn_public_decrypt_proof(&fixture, burned_handle);

    let mut accounts = fixture.accounts(1_000);
    seed_single_burn_value_account(&fixture, &mut accounts, burned_handle);
    let context = burn_redeem_mollusk().with_context(accounts);

    // redeem(H1) with the public-decrypt proof + certificate releases the current pending amount.
    let cleartext_amount = 500;
    let (signatures, extra_data) = kms_public_decrypt_cert(burned_handle, cleartext_amount);
    let pending_burn = seed_pending_burn_in_context(&context, &fixture, burned_handle);
    context.process_and_validate_instruction(
        &redeem_burned_amount_ix(
            &fixture,
            burned_handle,
            cleartext_amount,
            signatures.clone(),
            extra_data.clone(),
            proof.clone(),
            pending_burn,
        ),
        &[Check::success()],
    );

    assert_eq!(
        read_spl_amount(&context, fixture.destination_usdc),
        cleartext_amount
    );
    assert_eq!(
        read_spl_amount(&context, fixture.vault_usdc),
        1_000 - cleartext_amount
    );
    assert_pending_burn_closed(&context, pending_burn);

    // A second redemption fails because the single pending-burn account was closed.
    let dup = redeem_burned_amount_ix(
        &fixture,
        burned_handle,
        cleartext_amount,
        signatures,
        extra_data,
        proof,
        pending_burn,
    );
    assert!(context.process_instruction(&dup).raw_result.is_err());
    assert_eq!(
        read_spl_amount(&context, fixture.destination_usdc),
        cleartext_amount
    );

    // The alternate settlement path is excluded by the same closed account.
    assert!(context
        .process_instruction(&cancel_pending_burn_ix(&fixture, pending_burn))
        .raw_result
        .is_err());
}

#[test]
fn mollusk_redeem_current_burn_with_token_2022() {
    let fixture = BurnRedeemFixture::new_token_2022();
    let burned_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let proof = single_burn_public_decrypt_proof(&fixture, burned_handle);
    let (_, mut burned_value) = new_encrypted_value(
        fixture.mint,
        fixture.token_account,
        token::encrypted_burned_amount_label(),
        burned_handle,
        &[fixture.owner, fixture.compute_signer],
    );
    burned_value.leaf_count = 1;
    burned_value.peaks =
        zama_solana_acl::mmr_peaks_from_leaves(&[zama_solana_acl::public_decrypt_leaf_commitment(
            fixture.burned_amount_value.to_bytes(),
            0,
            burned_handle,
        )]);
    let mut accounts = fixture.accounts(1_000);
    accounts.insert(
        fixture.burned_amount_value,
        encrypted_value_account(&burned_value),
    );
    let context = burn_redeem_mollusk().with_context(accounts);
    let pending_burn = seed_pending_burn_in_context(&context, &fixture, burned_handle);
    let (signatures, extra_data) = kms_public_decrypt_cert(burned_handle, 500);

    context.process_and_validate_instruction(
        &redeem_burned_amount_ix(
            &fixture,
            burned_handle,
            500,
            signatures,
            extra_data,
            proof,
            pending_burn,
        ),
        &[Check::success()],
    );
    assert_eq!(read_spl_amount(&context, fixture.destination_usdc), 500);
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 500);
}

#[test]
fn mollusk_redeem_rejects_frozen_token_2022_destination() {
    let fixture = BurnRedeemFixture::new_token_2022();
    let burned_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let mut accounts = fixture.accounts(1_000);
    accounts.insert(
        fixture.destination_usdc,
        token_2022_token_account(
            fixture.underlying_mint,
            fixture.owner,
            0,
            spl_token_2022::state::AccountState::Frozen,
        ),
    );
    let context = burn_redeem_mollusk().with_context(accounts);
    let pending_burn = seed_pending_burn_in_context(&context, &fixture, burned_handle);
    let (signatures, extra_data) = kms_public_decrypt_cert(burned_handle, 500);

    context.process_and_validate_instruction(
        &redeem_burned_amount_ix(
            &fixture,
            burned_handle,
            500,
            signatures,
            extra_data,
            single_burn_public_decrypt_proof(&fixture, burned_handle),
            pending_burn,
        ),
        // Anchor's token-interface account loader treats a frozen destination as unavailable and
        // rejects it before the handler runs. The vault therefore remains untouched.
        &[anchor_error(
            anchor_lang::error::ErrorCode::AccountNotInitialized,
        )],
    );
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 1_000);
}

/// An out-of-range leaf index for the one-leaf accumulator fails closed with the host's
/// `PublicDecryptProofInvalid` (surfaced through the CPI) and leaves the vault untouched.
#[test]
fn mollusk_redeem_rejects_out_of_range_public_decrypt_leaf() {
    let fixture = BurnRedeemFixture::new();
    let first_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let mut proof = single_burn_public_decrypt_proof(&fixture, first_handle);

    let mut accounts = fixture.accounts(1_000);
    seed_single_burn_value_account(&fixture, &mut accounts, first_handle);
    let context = burn_redeem_mollusk().with_context(accounts);

    proof.leaf_index = 1; // Outside the one-leaf accumulator.

    let cleartext_amount = 500;
    let (signatures, extra_data) = kms_public_decrypt_cert(first_handle, cleartext_amount);
    let pending_burn = seed_pending_burn_in_context(&context, &fixture, first_handle);
    context.process_and_validate_instruction(
        &redeem_burned_amount_ix(
            &fixture,
            first_handle,
            cleartext_amount,
            signatures,
            extra_data,
            proof,
            pending_burn,
        ),
        &[host_error(
            host::errors::ZamaHostError::PublicDecryptProofInvalid,
        )],
    );

    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 1_000);
    assert_eq!(read_spl_amount(&context, fixture.destination_usdc), 0);
}

/// Accept-any-live-context (EVM parity): a cert committing the fixture's context id 9 still redeems
/// after the operator rotates the host's current context to 10, because context 9's account persists
/// and is not destroyed. Redemption accepts any live context, so the payout goes through.
#[test]
fn mollusk_redeem_accepts_live_rotated_out_kms_context() {
    let fixture = BurnRedeemFixture::new();
    let first_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let proof = single_burn_public_decrypt_proof(&fixture, first_handle);

    let mut accounts = fixture.accounts(1_000);
    seed_single_burn_value_account(&fixture, &mut accounts, first_handle);
    // Rotate the host's current context id to 10; the fixture's context-9 account stays live in the
    // account set. The cert commits id 9, so verification binds to that still-live context.
    accounts.insert(
        fixture.host_config,
        host_config_account_with_kms_context(
            fixture.owner,
            secp_evm_address(&coprocessor_signing_key()),
            fixture.kms_context_id + 1,
        ),
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) =
        kms_public_decrypt_cert_for_context(first_handle, cleartext_amount, fixture.kms_context_id);
    let pending_burn = seed_pending_burn_in_context(&context, &fixture, first_handle);
    context.process_and_validate_instruction(
        &redeem_burned_amount_ix(
            &fixture,
            first_handle,
            cleartext_amount,
            signatures,
            extra_data,
            proof,
            pending_burn,
        ),
        &[Check::success()],
    );
    assert_eq!(
        read_spl_amount(&context, fixture.destination_usdc),
        cleartext_amount
    );
    assert_eq!(
        read_spl_amount(&context, fixture.vault_usdc),
        1_000 - cleartext_amount
    );
}

/// Destroy is the revocation lever: a cert committing context id 9 is rejected once that context is
/// destroyed, so no destroyed signer set can cash out. Rejected by the host verifier with
/// `InvalidKmsContext`, surfaced one layer up at the redeem boundary; the vault is untouched.
#[test]
fn mollusk_redeem_rejects_destroyed_kms_context() {
    let fixture = BurnRedeemFixture::new();
    let first_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let proof = single_burn_public_decrypt_proof(&fixture, first_handle);

    let mut accounts = fixture.accounts(1_000);
    seed_single_burn_value_account(&fixture, &mut accounts, first_handle);
    // The fixture's context 9 has been destroyed (rotated for compromise, then revoked).
    accounts.insert(
        fixture.kms_context,
        destroyed_kms_context_account(fixture.kms_context_id),
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) =
        kms_public_decrypt_cert_for_context(first_handle, cleartext_amount, fixture.kms_context_id);
    let pending_burn = seed_pending_burn_in_context(&context, &fixture, first_handle);
    context.process_and_validate_instruction(
        &redeem_burned_amount_ix(
            &fixture,
            first_handle,
            cleartext_amount,
            signatures,
            extra_data,
            proof,
            pending_burn,
        ),
        &[host_error(host::errors::ZamaHostError::InvalidKmsContext)],
    );
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 1_000);
}

/// The destination token account must be owned by the signer: an account of the right mint owned by
/// someone else is rejected by the `destination_usdc.owner == owner` constraint, before any payout.
#[test]
fn mollusk_redeem_rejects_destination_not_owned_by_signer() {
    let fixture = BurnRedeemFixture::new();
    let first_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let proof = single_burn_public_decrypt_proof(&fixture, first_handle);

    let mut accounts = fixture.accounts(1_000);
    seed_single_burn_value_account(&fixture, &mut accounts, first_handle);
    // Re-own the destination account by a stranger (right mint, wrong owner).
    let stranger = Pubkey::new_unique();
    accounts.insert(
        fixture.destination_usdc,
        spl_token_account(fixture.underlying_mint, stranger, 0),
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) = kms_public_decrypt_cert(first_handle, cleartext_amount);
    let pending_burn = seed_pending_burn_in_context(&context, &fixture, first_handle);
    context.process_and_validate_instruction(
        &redeem_burned_amount_ix(
            &fixture,
            first_handle,
            cleartext_amount,
            signatures,
            extra_data,
            proof,
            pending_burn,
        ),
        &[token_error(token::ConfidentialTokenError::OwnerMismatch)],
    );
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 1_000);
}

/// The deny-list controls future ACL grants, not token settlement. An owner who is denied from new
/// grants can still redeem an already-authorized pending burn, so the deny-list cannot trap funds.
#[test]
fn mollusk_redeem_is_not_blocked_by_grant_deny_list() {
    let fixture = BurnRedeemFixture::new();
    let first_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let proof = single_burn_public_decrypt_proof(&fixture, first_handle);

    let mut accounts = fixture.accounts(1_000);
    seed_single_burn_value_account(&fixture, &mut accounts, first_handle);
    // Enable the grant deny-list on the host config (keeping the fixture's current KMS context)
    // and mark the signer denied.
    accounts.insert(
        fixture.host_config,
        host_config_account_with_flags(
            fixture.owner,
            &[secp_evm_address(&coprocessor_signing_key())],
            1,
            fixture.kms_context_id,
            true,
        ),
    );
    let (deny_record, denied_account) = deny_subject_record_account(fixture.owner, true);
    accounts.insert(deny_record, denied_account);
    let context = burn_redeem_mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) = kms_public_decrypt_cert(first_handle, cleartext_amount);
    let pending_burn = seed_pending_burn_in_context(&context, &fixture, first_handle);
    context.process_and_validate_instruction(
        &redeem_burned_amount_ix(
            &fixture,
            first_handle,
            cleartext_amount,
            signatures,
            extra_data,
            proof,
            pending_burn,
        ),
        &[Check::success()],
    );
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 500);
    assert_eq!(read_spl_amount(&context, fixture.destination_usdc), 500);
}

/// Two burns can be redeemed sequentially after the first pending account closes.
#[test]
fn mollusk_two_sequential_burns_each_redeemable_exactly_once() {
    let fixture = BurnRedeemFixture::new();
    let context = burn_redeem_mollusk().with_context(fixture.accounts(1_000));
    let pending_burn = token::pending_burn_address(fixture.mint, fixture.token_account).0;

    // Execute a real first burn, then redeem its sole public leaf.
    let first_handle = run_burn(&context, &fixture, 41);
    let proof_h1 = single_burn_public_decrypt_proof(&fixture, first_handle);
    let (sig_h1, extra_h1) = kms_public_decrypt_cert(first_handle, 300);
    context.process_and_validate_instruction(
        &redeem_burned_amount_ix(
            &fixture,
            first_handle,
            300,
            sig_h1.clone(),
            extra_h1.clone(),
            proof_h1.clone(),
            pending_burn,
        ),
        &[Check::success()],
    );

    // Only after H1 settles can a real second burn reopen the canonical pending account. Its
    // update seals H1's historical audience before appending public(H2) at leaf 3.
    let second_handle = run_burn(&context, &fixture, 42);
    let (proof_h2, expected_peaks) =
        two_burn_value_account_proof(&fixture, first_handle, second_handle, 3);
    assert_eq!(
        read_encrypted_value(&context, fixture.burned_amount_value).peaks,
        expected_peaks
    );

    // The old H1 proof/certificate cannot consume the reopened H2 pending burn.
    let stale = redeem_burned_amount_ix(
        &fixture,
        first_handle,
        300,
        sig_h1.clone(),
        extra_h1.clone(),
        proof_h1.clone(),
        pending_burn,
    );
    context.process_and_validate_instruction(
        &stale,
        &[token_error(
            token::ConfidentialTokenError::PendingBurnMismatch,
        )],
    );
    assert_eq!(read_spl_amount(&context, fixture.destination_usdc), 300);

    // Redeem H2 from leaf 3, reusing the canonical pending account after H1 closed.
    let (sig_h2, extra_h2) = kms_public_decrypt_cert(second_handle, 200);
    context.process_and_validate_instruction(
        &redeem_burned_amount_ix(
            &fixture,
            second_handle,
            200,
            sig_h2,
            extra_h2,
            proof_h2,
            pending_burn,
        ),
        &[Check::success()],
    );

    assert_eq!(read_spl_amount(&context, fixture.destination_usdc), 500);
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 500);

    // Re-redeem(H1) fails because no pending account remains.
    let dup = redeem_burned_amount_ix(
        &fixture,
        first_handle,
        300,
        sig_h1,
        extra_h1,
        proof_h1,
        pending_burn,
    );
    assert!(context.process_instruction(&dup).raw_result.is_err());
    assert_eq!(read_spl_amount(&context, fixture.destination_usdc), 500);
}

/// Builds the fixture's host config with `paused = true` so the redeem is rejected at the pause gate.
fn paused_redeem_host_config(fixture: &BurnRedeemFixture) -> Account {
    let mut account = host_config_account_with_kms_context(
        fixture.owner,
        secp_evm_address(&coprocessor_signing_key()),
        fixture.kms_context_id,
    );
    let mut config = host::HostConfig::try_deserialize(&mut account.data.as_slice())
        .expect("host config deserializes");
    config.paused = true;
    account.data = serialized_account(config);
    account
}

/// A paused host config rejects the redeem at the pause gate (`assert_host_config_allows_token_response`)
/// before any vault movement, with `RequestWitnessUnavailable`.
#[test]
fn mollusk_redeem_rejected_when_host_paused() {
    let fixture = BurnRedeemFixture::new();
    let first_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let proof = single_burn_public_decrypt_proof(&fixture, first_handle);

    let mut accounts = fixture.accounts(1_000);
    seed_single_burn_value_account(&fixture, &mut accounts, first_handle);
    accounts.insert(fixture.host_config, paused_redeem_host_config(&fixture));
    let context = burn_redeem_mollusk().with_context(accounts);

    let (signatures, extra_data) = kms_public_decrypt_cert(first_handle, 500);
    let pending_burn = seed_pending_burn_in_context(&context, &fixture, first_handle);
    context.process_and_validate_instruction(
        &redeem_burned_amount_ix(
            &fixture,
            first_handle,
            500,
            signatures,
            extra_data,
            proof,
            pending_burn,
        ),
        &[token_error(
            token::ConfidentialTokenError::RequestWitnessUnavailable,
        )],
    );
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 1_000);
    assert_eq!(read_spl_amount(&context, fixture.destination_usdc), 0);
}

/// A balance encrypted value account passed as `burned_amount_value` is rejected by the pending
/// burn binding (`PendingBurnMismatch`) when the account does not match the pending burn's stored
/// `burned_encrypted_value`, before the
/// verifier CPI and before any payout.
#[test]
fn mollusk_redeem_rejects_wrong_value_account_label() {
    let fixture = BurnRedeemFixture::new();
    let first_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let proof = single_burn_public_decrypt_proof(&fixture, first_handle);

    let mut accounts = fixture.accounts(1_000);
    seed_single_burn_value_account(&fixture, &mut accounts, first_handle);
    let context = burn_redeem_mollusk().with_context(accounts);

    let (signatures, extra_data) = kms_public_decrypt_cert(first_handle, 500);
    let pending_burn = seed_pending_burn_in_context(&context, &fixture, first_handle);
    // Substitute the balance encrypted value account (balance label) for the burned_amount encrypted value account.
    let mut ix = redeem_burned_amount_ix(
        &fixture,
        first_handle,
        500,
        signatures,
        extra_data,
        proof,
        pending_burn,
    );
    let burned_meta = ix
        .accounts
        .iter_mut()
        .find(|meta| meta.pubkey == fixture.burned_amount_value)
        .expect("burned_amount encrypted value account meta");
    burned_meta.pubkey = fixture.balance_value;
    context.process_and_validate_instruction(
        &ix,
        &[token_error(
            token::ConfidentialTokenError::PendingBurnMismatch,
        )],
    );
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 1_000);
    assert_eq!(read_spl_amount(&context, fixture.destination_usdc), 0);
}

fn cancel_pending_burn_ix(fixture: &BurnRedeemFixture, pending_burn: Pubkey) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::CancelPendingBurn {
            owner: fixture.owner,
            mint: fixture.mint,
            token_account: fixture.token_account,
            compute_signer: fixture.compute_signer,
            total_supply_authority: fixture.total_supply_authority,
            balance_value: fixture.balance_value,
            total_supply_value: fixture.total_supply_value,
            burned_amount_value: fixture.burned_amount_value,
            pending_burn,
            host_config: fixture.host_config,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            system_program: system_program::ID,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::CancelPendingBurn {},
    )
}

/// Burn then cancel: the pending account closes and confidential balance + supply restore.
#[test]
fn mollusk_cancel_pending_burn_restores_balance_and_supply() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(1_000);
    let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let amount_value = seed_burn_amount_value(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::encrypted_transfer_amount_label(),
        amount_handle,
        &[fixture.owner, fixture.compute_signer],
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.initial_balance, 1_000);
    cleartext.seed_amount(fixture.initial_total_supply, 5_000);
    cleartext.seed_amount(amount_handle, 250);

    let burn = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        amount_value,
    );
    let burn_result = context.process_and_validate_instruction(&burn, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &burn_result);
    assert_eq!(cleartext.balance(&context, fixture.token_account), 750);
    assert_eq!(
        cleartext.u64_at(&context, fixture.total_supply_value),
        4_750
    );

    let pending_burn = token::pending_burn_address(fixture.mint, fixture.token_account).0;
    assert!(context
        .account_store
        .borrow()
        .get(&pending_burn)
        .is_some_and(|account| account.owner == token::id() && !account.data.is_empty()));

    // Denial governs future grants only. Even denied existing output authorities must be able to
    // restore unchanged audiences, otherwise policy changes could trap a pending burn.
    context.account_store.borrow_mut().insert(
        fixture.host_config,
        host_config_account_with_flags(
            fixture.owner,
            &[secp_evm_address(&coprocessor_signing_key())],
            1,
            fixture.kms_context_id,
            true,
        ),
    );
    for authority in [fixture.token_account, fixture.total_supply_authority] {
        let (record, account) = deny_subject_record_account(authority, true);
        context.account_store.borrow_mut().insert(record, account);
    }

    let cancel = cancel_pending_burn_ix(&fixture, pending_burn);
    let cancel_result = context.process_and_validate_instruction(&cancel, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &cancel_result);

    assert_eq!(cleartext.balance(&context, fixture.token_account), 1_000);
    assert_eq!(
        cleartext.u64_at(&context, fixture.total_supply_value),
        5_000
    );
    assert_pending_burn_closed(&context, pending_burn);

    // Closing is act-once: neither a second cancel nor the alternate redeem path can settle the
    // same burn after the pending account is gone.
    assert!(context.process_instruction(&cancel).raw_result.is_err());
    let burned_handle = read_encrypted_value(&context, fixture.burned_amount_value).current_handle;
    let (signatures, extra_data) = kms_public_decrypt_cert(burned_handle, 250);
    assert!(context
        .process_instruction(&redeem_burned_amount_ix(
            &fixture,
            burned_handle,
            250,
            signatures,
            extra_data,
            single_burn_public_decrypt_proof(&fixture, burned_handle),
            pending_burn,
        ))
        .raw_result
        .is_err());
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 1_000);
}

/// A signer other than the confidential token account owner cannot cancel its pending burn.
#[test]
fn mollusk_cancel_pending_burn_rejects_non_owner_atomically() {
    let fixture = BurnRedeemFixture::new();
    let context = burn_redeem_mollusk().with_context(fixture.accounts(1_000));
    run_burn(&context, &fixture, 41);

    let pending_burn = token::pending_burn_address(fixture.mint, fixture.token_account).0;
    let old_balance = context
        .account_store
        .borrow()
        .get(&fixture.balance_value)
        .expect("balance exists")
        .data
        .clone();
    let old_supply = context
        .account_store
        .borrow()
        .get(&fixture.total_supply_value)
        .expect("total supply exists")
        .data
        .clone();
    let old_pending = context
        .account_store
        .borrow()
        .get(&pending_burn)
        .expect("pending burn exists")
        .clone();

    let stranger = Pubkey::new_unique();
    context
        .account_store
        .borrow_mut()
        .insert(stranger, system_account(1_000_000_000));
    let mut cancel = cancel_pending_burn_ix(&fixture, pending_burn);
    let owner_meta = cancel
        .accounts
        .iter_mut()
        .find(|meta| meta.pubkey == fixture.owner && meta.is_signer)
        .expect("owner signer meta");
    owner_meta.pubkey = stranger;

    context.process_and_validate_instruction(
        &cancel,
        &[token_error(token::ConfidentialTokenError::OwnerMismatch)],
    );
    assert_eq!(
        context
            .account_store
            .borrow()
            .get(&fixture.balance_value)
            .expect("balance remains")
            .data,
        old_balance
    );
    assert_eq!(
        context
            .account_store
            .borrow()
            .get(&fixture.total_supply_value)
            .expect("total supply remains")
            .data,
        old_supply
    );
    assert_eq!(
        context
            .account_store
            .borrow()
            .get(&pending_burn)
            .expect("pending burn remains"),
        &old_pending
    );
}

/// Cancellation fails atomically if the burned-amount account no longer points at the handle
/// pinned by the pending burn.
#[test]
fn mollusk_cancel_pending_burn_rejects_stale_current_handle_atomically() {
    let fixture = BurnRedeemFixture::new();
    let context = burn_redeem_mollusk().with_context(fixture.accounts(1_000));
    run_burn(&context, &fixture, 41);

    let pending_burn = token::pending_burn_address(fixture.mint, fixture.token_account).0;
    let old_balance_data = context
        .account_store
        .borrow()
        .get(&fixture.balance_value)
        .expect("balance exists")
        .data
        .clone();
    let old_supply_data = context
        .account_store
        .borrow()
        .get(&fixture.total_supply_value)
        .expect("total supply exists")
        .data
        .clone();
    let old_pending = context
        .account_store
        .borrow()
        .get(&pending_burn)
        .expect("pending burn exists")
        .clone();
    let old_vault_amount = read_spl_amount(&context, fixture.vault_usdc);

    let mut burned = read_encrypted_value(&context, fixture.burned_amount_value);
    burned.current_handle = handle_for_chain(42, BALANCE_FHE_TYPE);
    context
        .account_store
        .borrow_mut()
        .get_mut(&fixture.burned_amount_value)
        .expect("burned amount exists")
        .data = serialized_account(burned);

    context.process_and_validate_instruction(
        &cancel_pending_burn_ix(&fixture, pending_burn),
        &[token_error(
            token::ConfidentialTokenError::PendingBurnHandleNotCurrent,
        )],
    );

    assert_eq!(
        context
            .account_store
            .borrow()
            .get(&fixture.balance_value)
            .expect("balance remains")
            .data,
        old_balance_data
    );
    assert_eq!(
        context
            .account_store
            .borrow()
            .get(&fixture.total_supply_value)
            .expect("total supply remains")
            .data,
        old_supply_data
    );
    assert_eq!(
        read_spl_amount(&context, fixture.vault_usdc),
        old_vault_amount
    );
    assert_eq!(
        context
            .account_store
            .borrow()
            .get(&pending_burn)
            .expect("pending burn remains"),
        &old_pending
    );
}

/// A token account cannot burn again until its pending burn is redeemed or cancelled.
#[test]
fn mollusk_confidential_burn_is_sequential_until_cancelled() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(1_000);
    let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let amount_value = seed_burn_amount_value(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::encrypted_transfer_amount_label(),
        amount_handle,
        &[fixture.owner, fixture.compute_signer],
    );
    let context = burn_redeem_mollusk().with_context(accounts);
    let pending_burn = prepare_empty_pending_burn(&context, &fixture);

    let burn = confidential_burn_from_value_ix(
        &fixture,
        fixture.owner,
        fixture.owner,
        amount_value,
        pending_burn,
    );
    context.process_and_validate_instruction(&burn, &[Check::success()]);

    // The same canonical pending account is occupied, so another burn is rejected before FHE.
    context.process_and_validate_instruction(
        &burn,
        &[token_error(
            token::ConfidentialTokenError::PendingBurnAlreadyInitialized,
        )],
    );

    context.process_and_validate_instruction(
        &cancel_pending_burn_ix(&fixture, pending_burn),
        &[Check::success()],
    );
    context.process_and_validate_instruction(&burn, &[Check::success()]);
}

// ---------------------------------------------------------------------------
// wrap_usdc: escrows public USDC into the mint's SPL vault and credits the
// confidential balance and total supply by the wrapped amount. Reuses
// `BurnRedeemFixture`'s SPL-backed vault/underlying-mint accounts;
// `destination_usdc` (an owner-owned SPL account of the underlying mint,
// already present in `BurnRedeemFixture::accounts`) doubles as the wrap
// source (`user_usdc`) since its shape is identical to what wrap needs.
// ---------------------------------------------------------------------------

/// Builder for a `wrap_usdc` instruction against `BurnRedeemFixture`'s accounts, with every
/// account that a negative test needs to substitute exposed as a mutable field (defaulting to
/// the fixture's canonical accounts).
struct WrapUsdcParams<'a> {
    fixture: &'a BurnRedeemFixture,
    owner: Pubkey,
    underlying_mint: Pubkey,
    user_usdc: Pubkey,
    vault_usdc: Pubkey,
    total_supply_authority: Pubkey,
    token_program: Pubkey,
    amount: u64,
}

impl<'a> WrapUsdcParams<'a> {
    fn new(fixture: &'a BurnRedeemFixture, user_usdc: Pubkey, amount: u64) -> Self {
        Self {
            fixture,
            owner: fixture.owner,
            underlying_mint: fixture.underlying_mint,
            user_usdc,
            vault_usdc: fixture.vault_usdc,
            total_supply_authority: fixture.total_supply_authority,
            token_program: fixture.token_program,
            amount,
        }
    }

    fn build(self) -> Instruction {
        let fixture = self.fixture;
        anchor_ix(
            token::id(),
            token::accounts::WrapUsdc {
                owner: self.owner,
                mint: fixture.mint,
                token_account: fixture.token_account,
                underlying_mint: self.underlying_mint,
                user_usdc: self.user_usdc,
                vault_usdc: self.vault_usdc,
                vault_authority: fixture.vault_authority,
                compute_signer: fixture.compute_signer,
                total_supply_authority: self.total_supply_authority,
                balance_value: fixture.balance_value,
                total_supply_value: fixture.total_supply_value,
                zama_event_authority: event_authority(host::id()),
                zama_program: host::id(),
                host_config: fixture.host_config,
                token_program: self.token_program,
                system_program: system_program::ID,
                hcu_block_meter: None,
                hcu_trusted_app_record: None,
                event_authority: event_authority(token::id()),
                program: token::id(),
            },
            token::instruction::WrapUsdc {
                amount: self.amount,
            },
        )
    }
}

fn wrap_usdc_ix(fixture: &BurnRedeemFixture, user_usdc: Pubkey, amount: u64) -> Instruction {
    WrapUsdcParams::new(fixture, user_usdc, amount).build()
}

/// Funds `fixture.destination_usdc` (reused as the wrap source account) with `balance` and
/// returns its address for use as `user_usdc`.
fn fund_wrap_source(
    accounts: &mut HashMap<Pubkey, Account>,
    fixture: &BurnRedeemFixture,
    balance: u64,
) -> Pubkey {
    accounts.insert(
        fixture.destination_usdc,
        if fixture.token_program == spl_token::id() {
            spl_token_account(fixture.underlying_mint, fixture.owner, balance)
        } else {
            token_2022_immutable_owner_account(
                fixture.underlying_mint,
                fixture.owner,
                balance,
                spl_token_2022::state::AccountState::Initialized,
            )
        },
    );
    fixture.destination_usdc
}

/// Happy-path smoke: wrapping 100 escrows 100 into the vault, debits the source USDC account,
/// and credits the confidential balance and total supply by 100 (both started at cleartext 0).
#[test]
fn mollusk_wrap_usdc_credits_balance_and_total_supply() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(0);
    let user_usdc = fund_wrap_source(&mut accounts, &fixture, 1_000);
    let context = burn_redeem_mollusk().with_context(accounts);

    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.initial_balance, 0);
    cleartext.seed_amount(fixture.initial_total_supply, 0);

    let ix = wrap_usdc_ix(&fixture, user_usdc, 100);
    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &result);

    assert_eq!(cleartext.balance(&context, fixture.token_account), 100);
    assert_eq!(cleartext.u64_at(&context, fixture.total_supply_value), 100);
    assert_eq!(read_spl_amount(&context, user_usdc), 900);
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 100);
}

#[test]
fn mollusk_wrap_token_2022_credits_balance_and_total_supply() {
    let fixture = BurnRedeemFixture::new_token_2022();
    let mut accounts = fixture.accounts(0);
    let user_tokens = fund_wrap_source(&mut accounts, &fixture, 1_000);
    let context = burn_redeem_mollusk().with_context(accounts);

    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.initial_balance, 0);
    cleartext.seed_amount(fixture.initial_total_supply, 0);

    let result = context.process_and_validate_instruction(
        &wrap_usdc_ix(&fixture, user_tokens, 100),
        &[Check::success()],
    );
    cleartext.evaluate_fhe_cpi(&context, &result);
    assert_eq!(cleartext.balance(&context, fixture.token_account), 100);
    assert_eq!(cleartext.u64_at(&context, fixture.total_supply_value), 100);
    assert_eq!(read_spl_amount(&context, user_tokens), 900);
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 100);
}

#[test]
fn mollusk_wrap_rejects_classic_program_for_token_2022_accounts() {
    let fixture = BurnRedeemFixture::new_token_2022();
    let mut accounts = fixture.accounts(0);
    let user_tokens = fund_wrap_source(&mut accounts, &fixture, 1_000);
    let context = burn_redeem_mollusk().with_context(accounts);
    let mut params = WrapUsdcParams::new(&fixture, user_tokens, 100);
    params.token_program = spl_token::id();

    context.process_and_validate_instruction(
        &params.build(),
        &[token_error(
            token::ConfidentialTokenError::UnderlyingTokenProgramMismatch,
        )],
    );
    assert_eq!(read_spl_amount(&context, user_tokens), 1_000);
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 0);
}

#[test]
fn mollusk_wrap_rejects_frozen_token_2022_source() {
    let fixture = BurnRedeemFixture::new_token_2022();
    let mut accounts = fixture.accounts(0);
    accounts.insert(
        fixture.destination_usdc,
        token_2022_token_account(
            fixture.underlying_mint,
            fixture.owner,
            1_000,
            spl_token_2022::state::AccountState::Frozen,
        ),
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    context.process_and_validate_instruction(
        &wrap_usdc_ix(&fixture, fixture.destination_usdc, 100),
        &[token_error(
            token::ConfidentialTokenError::UnderlyingTokenAccountFrozen,
        )],
    );
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 0);
}

#[test]
fn mollusk_wrap_rejects_token_2022_mint_extensions() {
    let fixture = BurnRedeemFixture::new_token_2022();
    let mut accounts = fixture.accounts(0);
    accounts.insert(
        fixture.underlying_mint,
        token_2022_non_transferable_mint_account(6),
    );
    let user_tokens = fund_wrap_source(&mut accounts, &fixture, 1_000);
    let context = burn_redeem_mollusk().with_context(accounts);

    context.process_and_validate_instruction(
        &wrap_usdc_ix(&fixture, user_tokens, 100),
        &[token_error(
            token::ConfidentialTokenError::UnsupportedToken2022Extension,
        )],
    );
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 0);
}

#[test]
fn mollusk_wrap_rejects_token_2022_account_extensions() {
    let fixture = BurnRedeemFixture::new_token_2022();
    let mut accounts = fixture.accounts(0);
    accounts.insert(
        fixture.destination_usdc,
        token_2022_cpi_guard_account(fixture.underlying_mint, fixture.owner, 1_000),
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    context.process_and_validate_instruction(
        &wrap_usdc_ix(&fixture, fixture.destination_usdc, 100),
        &[token_error(
            token::ConfidentialTokenError::UnsupportedToken2022Extension,
        )],
    );
    assert_eq!(read_spl_amount(&context, fixture.destination_usdc), 1_000);
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 0);
}

/// The signer must be the token account's stored owner: a stranger whose own USDC account
/// satisfies the Anchor-level `user_usdc.owner == owner` constraint still fails the handler's
/// `token_account.owner == owner` check.
#[test]
fn mollusk_wrap_usdc_rejects_wrong_owner() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(0);
    let stranger = Pubkey::new_unique();
    accounts.insert(stranger, system_account(1_000_000_000));
    let stranger_usdc = Pubkey::new_unique();
    accounts.insert(
        stranger_usdc,
        spl_token_account(fixture.underlying_mint, stranger, 1_000),
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let mut params = WrapUsdcParams::new(&fixture, stranger_usdc, 100);
    params.owner = stranger;
    context.process_and_validate_instruction(
        &params.build(),
        &[token_error(token::ConfidentialTokenError::OwnerMismatch)],
    );
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 0);
}

/// A source/vault pair for a mint the confidential mint does NOT wrap satisfies every Anchor-level
/// account constraint (mint/owner equality) but fails the handler's `mint.underlying_mint` check.
#[test]
fn mollusk_wrap_usdc_rejects_wrong_underlying_mint() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(0);
    let other_mint = Pubkey::new_unique();
    accounts.insert(other_mint, spl_mint_account(6));
    let other_user_usdc = Pubkey::new_unique();
    accounts.insert(
        other_user_usdc,
        spl_token_account(other_mint, fixture.owner, 1_000),
    );
    let other_vault_usdc = Pubkey::new_unique();
    accounts.insert(
        other_vault_usdc,
        spl_token_account(other_mint, fixture.vault_authority, 0),
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let mut params = WrapUsdcParams::new(&fixture, other_user_usdc, 100);
    params.underlying_mint = other_mint;
    params.vault_usdc = other_vault_usdc;
    context.process_and_validate_instruction(
        &params.build(),
        &[token_error(
            token::ConfidentialTokenError::UnderlyingMintMismatch,
        )],
    );
}

/// A vault account with the right mint and right owner (`vault_authority`) but a non-canonical
/// address (not the `(vault_authority, underlying_mint)` associated-token-account) fails the
/// handler's canonical-vault check even though every Anchor-level constraint is satisfied.
#[test]
fn mollusk_wrap_usdc_rejects_noncanonical_vault() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(0);
    let user_usdc = fund_wrap_source(&mut accounts, &fixture, 1_000);
    let bogus_vault = Pubkey::new_unique();
    accounts.insert(
        bogus_vault,
        spl_token_account(fixture.underlying_mint, fixture.vault_authority, 0),
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let mut params = WrapUsdcParams::new(&fixture, user_usdc, 100);
    params.vault_usdc = bogus_vault;
    context.process_and_validate_instruction(
        &params.build(),
        &[token_error(
            token::ConfidentialTokenError::VaultAccountMismatch,
        )],
    );
    assert_eq!(read_spl_amount(&context, user_usdc), 1_000);
}

/// `total_supply_authority` is declared `seeds = [b"total-supply", mint.key()], bump`, so Anchor
/// itself derives and enforces the canonical PDA before the handler's (structurally identical,
/// defense-in-depth) `TotalSupplyAuthorityMismatch` re-check ever runs: any non-canonical account
/// is rejected by Anchor's own seeds constraint first. Substituting the canonical PDA for a
/// *different* mint reaches exactly that: `ConstraintSeeds`, not the token error.
#[test]
fn mollusk_wrap_usdc_rejects_wrong_total_supply_authority_pda() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(0);
    let user_usdc = fund_wrap_source(&mut accounts, &fixture, 1_000);
    let context = burn_redeem_mollusk().with_context(accounts);

    let mut params = WrapUsdcParams::new(&fixture, user_usdc, 100);
    params.total_supply_authority = token::total_supply_authority_address(Pubkey::new_unique()).0;
    context.process_and_validate_instruction(
        &params.build(),
        &[anchor_error(anchor_lang::error::ErrorCode::ConstraintSeeds)],
    );
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 0);
}

// Note: `amount = 0` is intentionally not covered as a negative case. `wrap_usdc` has no
// `amount > 0` guard of its own (see `instructions/wrap_usdc.rs`), and the underlying
// `spl_token::transfer_checked` CPI does not reject a zero-amount transfer either — a zero-amount
// wrap is a valid (if useless) no-op that would succeed, not a negative test.

// ---------------------------------------------------------------------------
// disclose_secp consume: the whole disclosure "consume" path after the
// DisclosureRequest lifecycle was dissolved (fhevm-internal#1704, DD-040). One
// generic thin instruction CPIs the stateless host `verify_public_decrypt`,
// asserts the proven handle equals the caller-pinned handle, and emits a
// token-scoped `HandleDisclosedEvent`. The host verifier's own negatives
// (destroyed context, sub-threshold cert, handle/proof mismatch, non-canonical
// context, survives-update) are covered directly in `host_mollusk.rs` and are
// deliberately NOT duplicated here — the token tests cover only what the token
// layer adds: the mint-domain binding, the disclosed event, the pinned-handle
// pass-through, and the intentional absence of a replay marker (idempotence).
// ---------------------------------------------------------------------------

/// Self-contained fixture for the disclose consume vertical: one owner, one
/// confidential mint, a balance encrypted value account, and one token-scoped amount encrypted value account.
/// The fixture's v0 certs resolve to the host's current KMS context, so the fixture's
/// `current_kms_context_id` and seeded `kms_context` share `kms_context_id`.
struct DiscloseFixture {
    owner: Pubkey,
    mint: Pubkey,
    compute_signer: Pubkey,
    host_config: Pubkey,
    token_account: Pubkey,
    balance_value: Pubkey,
    amount_value: Pubkey,
    kms_context_id: u64,
    kms_context: Pubkey,
}

impl DiscloseFixture {
    fn new() -> Self {
        let owner = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let compute_signer = token::compute_signer_address(mint).0;
        let host_config = host::host_config_address().0;
        let token_account = token::token_account_address(mint, owner).0;
        let balance_value = token::balance_encrypted_value_address(mint, token_account).0;
        // Any token-scoped amount encrypted value account discloses the same way; use the burned_amount slot.
        let amount_value = token::encrypted_value_address(
            mint,
            token_account,
            token::encrypted_burned_amount_label(),
        )
        .0;
        let kms_context_id = 9;
        let kms_context = host::kms_context_address(kms_context_id).0;
        Self {
            owner,
            mint,
            compute_signer,
            host_config,
            token_account,
            balance_value,
            amount_value,
            kms_context_id,
            kms_context,
        }
    }

    fn confidential_mint_account(&self) -> Account {
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(token::ConfidentialMint {
                authority: self.owner,
                domain: self.mint,
                compute_signer: self.compute_signer,
                underlying_mint: Pubkey::new_unique(),
                decimals: 6,
                total_supply_encrypted_value: token::total_supply_encrypted_value_address(
                    self.mint,
                    token::total_supply_authority_address(self.mint).0,
                )
                .0,
            }),
            owner: token::id(),
            executable: false,
            rent_epoch: 0,
        }
    }

    fn base(&self) -> HashMap<Pubkey, Account> {
        HashMap::from([
            (self.owner, system_account(50_000_000_000)),
            (self.mint, self.confidential_mint_account()),
            (
                self.host_config,
                host_config_account_with_kms_context(
                    self.owner,
                    secp_evm_address(&coprocessor_signing_key()),
                    self.kms_context_id,
                ),
            ),
            (self.kms_context, kms_context_account(self.kms_context_id)),
            (
                self.token_account,
                token_account_account(self.mint, self.owner, self.balance_value),
            ),
            (event_authority(token::id()), system_account(0)),
        ])
    }
}

/// Builds an encrypted value account carrying a public-decrypt leaf (leaf 0) for `pinned`, and the inclusion
/// proof for it. With `update_to = Some(h2)` the encrypted value account is grown into a post-update
/// state (public(pinned)@0, hist(pinned,subj0)@1, hist(pinned,subj1)@2, public(h2)@3, current
/// handle h2), modeling the pinned handle becoming historical after it was sealed public.
/// `subjects` must hold at least two entries when updating.
fn public_leaf_value_account(
    expected_address: Pubkey,
    account: Pubkey,
    mint: Pubkey,
    encrypted_value_label: [u8; 32],
    subjects: &[Pubkey],
    pinned: [u8; 32],
    update_to: Option<[u8; 32]>,
) -> (host::EncryptedValue, host::instructions::MmrInclusionProof) {
    let acct = expected_address.to_bytes();
    let mut leaves = vec![zama_solana_acl::public_decrypt_leaf_commitment(
        acct, 0, pinned,
    )];
    let current = match update_to {
        Some(h2) => {
            leaves.push(zama_solana_acl::historical_access_leaf_commitment(
                acct,
                1,
                pinned,
                subjects[0].to_bytes(),
            ));
            leaves.push(zama_solana_acl::historical_access_leaf_commitment(
                acct,
                2,
                pinned,
                subjects[1].to_bytes(),
            ));
            leaves.push(zama_solana_acl::public_decrypt_leaf_commitment(acct, 3, h2));
            h2
        }
        None => pinned,
    };
    let (address, mut value) =
        new_encrypted_value(mint, account, encrypted_value_label, current, subjects);
    assert_eq!(
        address, expected_address,
        "encrypted value account address mismatch"
    );
    value.leaf_count = leaves.len() as u64;
    value.peaks = zama_solana_acl::mmr_peaks_from_leaves(&leaves);
    let proof = zama_solana_acl::mmr_build_proof(&leaves, 0).expect("proof for leaf 0");
    (
        value,
        host::instructions::MmrInclusionProof {
            leaf_index: proof.leaf_index,
            siblings: proof.siblings,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn disclose_secp_ix(
    fixture: &DiscloseFixture,
    encrypted_value: Pubkey,
    handle: [u8; 32],
    cleartext: [u8; 32],
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
    proof: host::instructions::MmrInclusionProof,
) -> Instruction {
    let kind = if encrypted_value == fixture.balance_value {
        token::DisclosedValueKind::Balance
    } else {
        token::DisclosedValueKind::BurnedAmount
    };
    disclose_secp_ix_with_kind(
        fixture,
        encrypted_value,
        kind,
        handle,
        cleartext,
        signatures,
        extra_data,
        proof,
    )
}

#[allow(clippy::too_many_arguments)]
fn disclose_secp_ix_with_kind(
    fixture: &DiscloseFixture,
    encrypted_value: Pubkey,
    kind: token::DisclosedValueKind,
    handle: [u8; 32],
    cleartext: [u8; 32],
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
    proof: host::instructions::MmrInclusionProof,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::DiscloseSecp {
            mint: fixture.mint,
            token_account: Some(fixture.token_account),
            encrypted_value,
            host_config: fixture.host_config,
            kms_context: fixture.kms_context,
            zama_program: host::id(),
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::DiscloseSecp {
            kind,
            handle,
            cleartext,
            signatures,
            extra_data,
            proof,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn disclose_total_supply_ix(
    fixture: &DiscloseFixture,
    encrypted_value: Pubkey,
    handle: [u8; 32],
    cleartext: [u8; 32],
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
    proof: host::instructions::MmrInclusionProof,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::DiscloseSecp {
            mint: fixture.mint,
            token_account: None,
            encrypted_value,
            host_config: fixture.host_config,
            kms_context: fixture.kms_context,
            zama_program: host::id(),
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::DiscloseSecp {
            kind: token::DisclosedValueKind::TotalSupply,
            handle,
            cleartext,
            signatures,
            extra_data,
            proof,
        },
    )
}

/// Asserts `disclose_secp` succeeds and emits exactly one `HandleDisclosedEvent` with the expected
/// fields. (The host verifier's `return_data` — `handle ++ cleartext` — is asserted directly in
/// `host_mollusk.rs`; it is consumed inside the token program and not re-surfaced at the top level.)
struct ExpectedDisclosure {
    mint: Pubkey,
    handle: [u8; 32],
    encrypted_value: Pubkey,
    cleartext_amount: u64,
    kind: token::DisclosedValueKind,
    authority: Pubkey,
    encrypted_value_label: [u8; 32],
}

fn assert_disclosed(result: &InstructionResult, expected: ExpectedDisclosure) {
    let events: Vec<token::HandleDisclosedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].mint, expected.mint);
    assert_eq!(events[0].handle, expected.handle);
    assert_eq!(events[0].encrypted_value, expected.encrypted_value);
    assert_eq!(events[0].cleartext_amount, expected.cleartext_amount);
    assert_eq!(events[0].kind, expected.kind);
    assert_eq!(
        events[0].encrypted_value_account_authority,
        expected.authority
    );
    assert_eq!(
        events[0].encrypted_value_label,
        expected.encrypted_value_label
    );
}

#[test]
fn mollusk_disclose_secp_amount_happy_path() {
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(43, BALANCE_FHE_TYPE);
    let (value, proof) = public_leaf_value_account(
        fixture.amount_value,
        fixture.token_account,
        fixture.mint,
        token::encrypted_burned_amount_label(),
        &[fixture.owner, fixture.compute_signer],
        pinned,
        None,
    );
    assert_eq!(value.current_handle, pinned);

    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_value, encrypted_value_account(&value));
    let context = mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) = kms_public_decrypt_cert(pinned, cleartext_amount);
    let result = context.process_and_validate_instruction(
        &disclose_secp_ix(
            &fixture,
            fixture.amount_value,
            pinned,
            cleartext_u256(cleartext_amount),
            signatures,
            extra_data,
            proof,
        ),
        &[Check::success()],
    );
    assert_disclosed(
        &result,
        ExpectedDisclosure {
            mint: fixture.mint,
            handle: pinned,
            encrypted_value: fixture.amount_value,
            cleartext_amount,
            kind: token::DisclosedValueKind::BurnedAmount,
            authority: fixture.token_account,
            encrypted_value_label: token::encrypted_burned_amount_label(),
        },
    );
}

#[test]
fn mollusk_disclose_secp_rejects_kind_label_mismatch() {
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(43, BALANCE_FHE_TYPE);
    let (value, proof) = public_leaf_value_account(
        fixture.amount_value,
        fixture.token_account,
        fixture.mint,
        token::encrypted_burned_amount_label(),
        &[fixture.owner, fixture.compute_signer],
        pinned,
        None,
    );
    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_value, encrypted_value_account(&value));
    let context = mollusk().with_context(accounts);
    let (signatures, extra_data) = kms_public_decrypt_cert(pinned, 500);

    context.process_and_validate_instruction(
        &disclose_secp_ix_with_kind(
            &fixture,
            fixture.amount_value,
            token::DisclosedValueKind::Balance,
            pinned,
            cleartext_u256(500),
            signatures,
            extra_data,
            proof,
        ),
        &[token_error(
            token::ConfidentialTokenError::DisclosedValueBindingMismatch,
        )],
    );
}

#[test]
fn mollusk_disclose_secp_balance_happy_path() {
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(33, BALANCE_FHE_TYPE);
    let (value, proof) = public_leaf_value_account(
        fixture.balance_value,
        fixture.token_account,
        fixture.mint,
        token::encrypted_balance_label(),
        &[fixture.owner, fixture.compute_signer],
        pinned,
        None,
    );

    let mut accounts = fixture.base();
    accounts.insert(fixture.balance_value, encrypted_value_account(&value));
    let context = mollusk().with_context(accounts);

    let cleartext_amount = 700;
    let (signatures, extra_data) = kms_public_decrypt_cert(pinned, cleartext_amount);
    let result = context.process_and_validate_instruction(
        &disclose_secp_ix(
            &fixture,
            fixture.balance_value,
            pinned,
            cleartext_u256(cleartext_amount),
            signatures,
            extra_data,
            proof,
        ),
        &[Check::success()],
    );
    assert_disclosed(
        &result,
        ExpectedDisclosure {
            mint: fixture.mint,
            handle: pinned,
            encrypted_value: fixture.balance_value,
            cleartext_amount,
            kind: token::DisclosedValueKind::Balance,
            authority: fixture.token_account,
            encrypted_value_label: token::encrypted_balance_label(),
        },
    );
}

#[test]
fn mollusk_disclose_secp_transferred_amount_happy_path() {
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(34, BALANCE_FHE_TYPE);
    let transferred_value = token::encrypted_value_address(
        fixture.mint,
        fixture.token_account,
        token::encrypted_transferred_amount_label(),
    )
    .0;
    let (value, proof) = public_leaf_value_account(
        transferred_value,
        fixture.token_account,
        fixture.mint,
        token::encrypted_transferred_amount_label(),
        &[fixture.owner, fixture.compute_signer],
        pinned,
        None,
    );
    let mut accounts = fixture.base();
    accounts.insert(transferred_value, encrypted_value_account(&value));
    let context = mollusk().with_context(accounts);
    let cleartext_amount = 125;
    let (signatures, extra_data) = kms_public_decrypt_cert(pinned, cleartext_amount);

    let result = context.process_and_validate_instruction(
        &disclose_secp_ix_with_kind(
            &fixture,
            transferred_value,
            token::DisclosedValueKind::TransferredAmount,
            pinned,
            cleartext_u256(cleartext_amount),
            signatures,
            extra_data,
            proof,
        ),
        &[Check::success()],
    );
    assert_disclosed(
        &result,
        ExpectedDisclosure {
            mint: fixture.mint,
            handle: pinned,
            encrypted_value: transferred_value,
            cleartext_amount,
            kind: token::DisclosedValueKind::TransferredAmount,
            authority: fixture.token_account,
            encrypted_value_label: token::encrypted_transferred_amount_label(),
        },
    );
}

#[test]
fn mollusk_disclose_secp_total_supply_requires_no_token_account() {
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(35, BALANCE_FHE_TYPE);
    let authority = token::total_supply_authority_address(fixture.mint).0;
    let total_supply_value = token::total_supply_encrypted_value_address(fixture.mint, authority).0;
    let (value, proof) = public_leaf_value_account(
        total_supply_value,
        authority,
        fixture.mint,
        token::encrypted_total_supply_label(),
        &[fixture.compute_signer],
        pinned,
        None,
    );
    let mut accounts = fixture.base();
    accounts.insert(total_supply_value, encrypted_value_account(&value));
    let context = mollusk().with_context(accounts);
    let cleartext_amount = 10_000;
    let (signatures, extra_data) = kms_public_decrypt_cert(pinned, cleartext_amount);

    let result = context.process_and_validate_instruction(
        &disclose_total_supply_ix(
            &fixture,
            total_supply_value,
            pinned,
            cleartext_u256(cleartext_amount),
            signatures.clone(),
            extra_data.clone(),
            proof.clone(),
        ),
        &[Check::success()],
    );
    assert_disclosed(
        &result,
        ExpectedDisclosure {
            mint: fixture.mint,
            handle: pinned,
            encrypted_value: total_supply_value,
            cleartext_amount,
            kind: token::DisclosedValueKind::TotalSupply,
            authority,
            encrypted_value_label: token::encrypted_total_supply_label(),
        },
    );

    context.process_and_validate_instruction(
        &disclose_secp_ix_with_kind(
            &fixture,
            total_supply_value,
            token::DisclosedValueKind::TotalSupply,
            pinned,
            cleartext_u256(cleartext_amount),
            signatures,
            extra_data,
            proof,
        ),
        &[token_error(
            token::ConfidentialTokenError::DisclosedValueBindingMismatch,
        )],
    );
}

#[test]
fn mollusk_disclose_secp_after_an_update_consumes_with_public_proof() {
    // The griefing case preserved end-to-end: the handle is sealed public while current, then the
    // encrypted value account is replaced to H2 (e.g. an inbound transfer) before the consume lands. The pinned
    // handle must still disclose, authorized by its permanent public-decrypt leaf, not the live
    // handle. This is the host verifier's survives-update property observed one layer up.
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(41, BALANCE_FHE_TYPE);
    let replaced = handle_for_chain(42, BALANCE_FHE_TYPE);
    let (value, proof) = public_leaf_value_account(
        fixture.amount_value,
        fixture.token_account,
        fixture.mint,
        token::encrypted_burned_amount_label(),
        &[fixture.owner, fixture.compute_signer],
        pinned,
        Some(replaced),
    );
    assert_ne!(value.current_handle, pinned);

    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_value, encrypted_value_account(&value));
    let context = mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) = kms_public_decrypt_cert(pinned, cleartext_amount);
    let result = context.process_and_validate_instruction(
        &disclose_secp_ix(
            &fixture,
            fixture.amount_value,
            pinned,
            cleartext_u256(cleartext_amount),
            signatures,
            extra_data,
            proof,
        ),
        &[Check::success()],
    );
    assert_disclosed(
        &result,
        ExpectedDisclosure {
            mint: fixture.mint,
            handle: pinned,
            encrypted_value: fixture.amount_value,
            cleartext_amount,
            kind: token::DisclosedValueKind::BurnedAmount,
            authority: fixture.token_account,
            encrypted_value_label: token::encrypted_burned_amount_label(),
        },
    );
}

#[test]
fn mollusk_disclose_secp_is_idempotent_no_replay_marker() {
    // Act-once is intentionally NOT enforced on-chain: disclosure is idempotent information release,
    // so re-running the same cert succeeds again and re-emits the same event. No replay marker PDA
    // exists by design (contrast redeem_burned_amount). Apps that need consume-once track it in
    // their own state.
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(44, BALANCE_FHE_TYPE);
    let (value, proof) = public_leaf_value_account(
        fixture.amount_value,
        fixture.token_account,
        fixture.mint,
        token::encrypted_burned_amount_label(),
        &[fixture.owner, fixture.compute_signer],
        pinned,
        None,
    );

    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_value, encrypted_value_account(&value));
    let context = mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) = kms_public_decrypt_cert(pinned, cleartext_amount);
    let ix = disclose_secp_ix(
        &fixture,
        fixture.amount_value,
        pinned,
        cleartext_u256(cleartext_amount),
        signatures,
        extra_data,
        proof,
    );
    context.process_and_validate_instruction(&ix, &[Check::success()]);
    // Same cert, same accounts, run again: still succeeds (idempotent, no consume-once).
    context.process_and_validate_instruction(&ix, &[Check::success()]);
}

#[test]
fn mollusk_disclose_secp_rejects_foreign_public_decrypt_proof() {
    // A structurally valid proof aimed at the WRONG leaf position (H2's public leaf, not H1's):
    // the host verifier recomputes public(H1)@leaf_index against the peaks and rejects it, so the
    // consume fails closed and emits no cleartext. This is the token layer surfacing the host's
    // proof check through the CPI — the wrong-handle rejection at the token boundary.
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(46, BALANCE_FHE_TYPE);
    let replaced = handle_for_chain(47, BALANCE_FHE_TYPE);
    let (value, mut proof) = public_leaf_value_account(
        fixture.amount_value,
        fixture.token_account,
        fixture.mint,
        token::encrypted_burned_amount_label(),
        &[fixture.owner, fixture.compute_signer],
        pinned,
        Some(replaced),
    );
    proof.leaf_index = 3; // H2's public-decrypt leaf, not H1's.

    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_value, encrypted_value_account(&value));
    let context = mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) = kms_public_decrypt_cert(pinned, cleartext_amount);
    context.process_and_validate_instruction(
        &disclose_secp_ix(
            &fixture,
            fixture.amount_value,
            pinned,
            cleartext_u256(cleartext_amount),
            signatures,
            extra_data,
            proof,
        ),
        &[host_error(
            host::errors::ZamaHostError::PublicDecryptProofInvalid,
        )],
    );
}

#[test]
fn mollusk_disclose_secp_rejects_foreign_mint_domain() {
    // The disclosed encrypted value account must belong to this mint's ACL domain: the token layer binds
    // encrypted_value.domain to the mint so the emitted event is genuinely token-scoped.
    // An encrypted value account under a different domain is rejected before the verifier CPI.
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(48, BALANCE_FHE_TYPE);
    let foreign_mint = Pubkey::new_unique();
    // A public encrypted value account whose domain is a different mint, but whose canonical address is
    // computed under that foreign domain so the account still deserializes as a valid EncryptedValue.
    let (foreign_value, proof) = public_leaf_value_account(
        token::encrypted_value_address(
            foreign_mint,
            fixture.token_account,
            token::encrypted_burned_amount_label(),
        )
        .0,
        fixture.token_account,
        foreign_mint,
        token::encrypted_burned_amount_label(),
        &[fixture.owner, fixture.compute_signer],
        pinned,
        None,
    );
    let foreign_value_addr = token::encrypted_value_address(
        foreign_mint,
        fixture.token_account,
        token::encrypted_burned_amount_label(),
    )
    .0;

    let mut accounts = fixture.base();
    accounts.insert(foreign_value_addr, encrypted_value_account(&foreign_value));
    let context = mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) = kms_public_decrypt_cert(pinned, cleartext_amount);
    context.process_and_validate_instruction(
        &disclose_secp_ix(
            &fixture,
            foreign_value_addr,
            pinned,
            cleartext_u256(cleartext_amount),
            signatures,
            extra_data,
            proof,
        ),
        &[token_error(token::ConfidentialTokenError::DomainMismatch)],
    );
}

#[test]
fn mollusk_disclose_secp_rejects_cleartext_wider_than_u64() {
    // Token encrypted value accounts are euint64, so the certified 32-byte uint256 cleartext must fit in 64 bits.
    // The host verifier accepts any 32-byte cleartext its cert signs over; the token layer then
    // rejects a value with nonzero high bytes rather than silently truncating it to the low 64 bits.
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(49, BALANCE_FHE_TYPE);
    let (value, proof) = public_leaf_value_account(
        fixture.amount_value,
        fixture.token_account,
        fixture.mint,
        token::encrypted_burned_amount_label(),
        &[fixture.owner, fixture.compute_signer],
        pinned,
        None,
    );

    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_value, encrypted_value_account(&value));
    let context = mollusk().with_context(accounts);

    // A cleartext whose value exceeds u64::MAX: a nonzero byte in the high 24 (here index 8).
    let mut wide = [0u8; 32];
    wide[8] = 1;
    let extra_data = vec![0x00u8];
    let signatures = zama_solana_test_kit::kms::kms_public_decrypt_cert(
        pinned,
        wide,
        GATEWAY_CHAIN_ID,
        &DECRYPTION_CONTRACT,
        &extra_data,
    );
    context.process_and_validate_instruction(
        &disclose_secp_ix(
            &fixture,
            fixture.amount_value,
            pinned,
            wide,
            signatures,
            extra_data,
            proof,
        ),
        &[token_error(
            token::ConfidentialTokenError::CleartextExceedsEuint64,
        )],
    );
}

// ===========================================================================
// HCU per-app block cap enforced through the confidential-token -> fhe_execute CPI.
//
// Ported from PR #2991 ("per-app HCU limit per block"), rewritten against the merged
// EncryptedValue persistent-output model: `confidential_transfer` reaches `fhe_execute` only by CPI,
// so these tests prove the block cap (ban / metering-band charge / canonical-authority pinning)
// survives that CPI boundary — not just direct `fhe_execute` calls (see `host_mollusk.rs`).
//
// The `create_random_amount` HCU tests from the same PR are not here because the instruction no
// longer exists: it was a demo faucet behind a `poc` feature with no caller anywhere, and it was
// deleted along with the feature.
// ===========================================================================

/// Exact HCU cost of the combined transfer execution (`compute_transfer_handles`): `Ge` at ebool
/// (21_000) + debit `Sub` at euint64 (38_000) + `IfThenElse` at euint64 (45_000) + transferred
/// `Sub` at euint64 (38_000) + balance-binding scalar `Add` at euint64 (33_250) + credit `Add`
/// at euint64 (38_000). The `VerifiedInput` amount is an operand, not a step, so it adds no HCU.
const TRANSFER_BATCH_HCU: u64 = 21_000 + 38_000 + 45_000 + 38_000 + 33_250 + 38_000; // 213_250

/// The fixture's host config with the per-app block cap overridden to `cap`.
fn host_config_account_with_block_cap(
    admin: Pubkey,
    coprocessor_signer: [u8; 20],
    cap: u64,
) -> Account {
    let mut account = host_config_account(admin, coprocessor_signer);
    let mut config = {
        let mut data = account.data.as_slice();
        host::HostConfig::try_deserialize(&mut data).expect("valid host config")
    };
    config.hcu_block_cap_per_app = cap;
    account.data = serialized_account(config);
    account
}

fn read_hcu_block_meter(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<host::HcuBlockMeter> {
    let store = context.account_store.borrow();
    let account = store.get(&address)?;
    if account.owner != host::id() {
        return None;
    }
    let mut data = account.data.as_slice();
    host::HcuBlockMeter::try_deserialize(&mut data).ok()
}

#[test]
fn mollusk_confidential_transfer_block_cap_ban_is_enforced_through_cpi() {
    // A confidential transfer reaches fhe_execute only by CPI. With the cap at the ban sentinel
    // (0) and no trust witness threaded, the block-cap breach must surface through the CPI and
    // roll the whole transfer back atomically — exactly as a direct fhe_execute call is rejected.
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    accounts.insert(
        fixture.host_config,
        host_config_account_with_block_cap(
            fixture.owner,
            secp_evm_address(&coprocessor_signing_key()),
            0,
        ),
    );
    let context = mollusk().with_context(accounts);
    let amount_handle = handle_for_chain(200, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
    );

    context.process_and_validate_instruction(
        &ix,
        &[host_error(
            host::errors::ZamaHostError::HcuBlockLimitExceeded,
        )],
    );

    // Atomic revert: both balances are unchanged.
    assert_eq!(
        read_encrypted_value(&context, fixture.alice_balance_value).current_handle,
        fixture.alice_initial
    );
    assert_eq!(
        read_encrypted_value(&context, fixture.bob_balance_value).current_handle,
        fixture.bob_initial
    );
}

#[test]
fn mollusk_confidential_transfer_metering_band_charges_meter_through_cpi() {
    // The Some(meter) CPI shape — the production account set once the cap drops below
    // u64::MAX. With a metering-band cap and the meter threaded through ConfidentialTransfer, the
    // transfer must succeed and the meter must be lazy-created and charged with exactly the execution's
    // HCU, proving the optional accounts survive the token -> zama-fhe -> fhe_execute CPI encoding end
    // to end. The metering identity is the execution's `compute_subject` — here the mint's
    // ["fhe-compute", mint] compute-signer PDA — one budget per mint, NOT per sender token account,
    // and with no separate HCU authority account.
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    accounts.insert(
        fixture.host_config,
        host_config_account_with_block_cap(
            fixture.owner,
            secp_evm_address(&coprocessor_signing_key()),
            500_000,
        ),
    );
    let context = mollusk().with_context(accounts);
    let amount_handle = handle_for_chain(21, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let meter_pda = host::hcu_block_meter_address(fixture.compute_signer).0;
    let ix = confidential_transfer_ix_with_block_cap_accounts(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
        Vec::new(),
        Some(meter_pda),
        None,
    );

    context.process_and_validate_instruction(&ix, &[Check::success()]);

    // The transfer completed: the sender's balance encrypted value account moved off its initial handle.
    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    assert_ne!(alice_balance.current_handle, fixture.alice_initial);
    // The meter was lazy-created through the CPI, keyed on the mint's compute signer, and
    // charged exactly the transfer execution's HCU at the current slot.
    let meter = read_hcu_block_meter(&context, meter_pda).expect("meter created through CPI");
    assert_eq!(meter.app, fixture.compute_signer);
    assert_eq!(meter.used_hcu, TRANSFER_BATCH_HCU);
    assert_eq!(meter.last_seen_slot, context.mollusk.sysvars.clock.slot);
    // Regression guard on the metering granularity: nothing accrues to the sender token
    // account's key — a sybil minting fresh token accounts gets no fresh budget.
    assert!(read_hcu_block_meter(
        &context,
        host::hcu_block_meter_address(fixture.alice_token).0
    )
    .is_none());
}

// ---------------------------------------------------------------------------
// confidential_transfer_from_value (spend an existing encrypted amount, fhevm-internal#1680)
// ---------------------------------------------------------------------------

/// Done-when 1: a transfer spends a computed handle produced under the same mint, with no
/// attestation attached. Here the amount is an existing encrypted value account carrying the sender + compute
/// subjects; the balances move through the same `ge -> sub -> select` debit and `add` credit, and
/// the amount value itself is read-only (never replaced, never consumed).
#[test]
fn mollusk_transfer_from_value_spends_existing_amount() {
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let amount_value = seed_amount_value(
        &fixture,
        &mut accounts,
        fixture.alice_token,
        token::encrypted_transfer_amount_label(),
        amount_handle,
        &[fixture.owner, fixture.compute_signer],
    );
    let context = mollusk().with_context(accounts);

    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.alice_initial, 1_000);
    cleartext.seed_amount(fixture.bob_initial, 100);
    cleartext.seed_amount(amount_handle, 250);

    let transfer = confidential_transfer_from_value_ix(
        &fixture,
        fixture.owner,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        amount_value,
    );

    let result = context.process_and_validate_instruction(&transfer, &[Check::success()]);
    let persistent_outputs = cleartext.evaluate_fhe_cpi(&context, &result);

    // Only the two balance encrypted value accounts and the sender's transferred_amount are updated — the amount is not
    // an output.
    assert_eq!(persistent_outputs, 3);
    assert_eq!(cleartext.balance(&context, fixture.alice_token), 750);
    assert_eq!(cleartext.balance(&context, fixture.bob_token), 350);

    // The amount value is read-only: current handle, subjects, and history all unchanged.
    let amount_after = read_encrypted_value(&context, amount_value);
    assert_eq!(amount_after.current_handle, amount_handle);
    assert_eq!(amount_after.leaf_count, 0);
    assert_eq!(
        amount_after.subjects,
        vec![fixture.owner, fixture.compute_signer]
    );
}

/// Done-when 1 (follow-up): the RECIPIENT of a transfer spends the received `transferred_amount`
/// into a transfer to a third party — the exact forwarding flow — with no decryption anywhere.
#[test]
fn mollusk_transfer_from_value_recipient_forwards_received_amount() {
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    let carol_initial = handle_for_chain(3, BALANCE_FHE_TYPE);
    let (_carol_owner, carol_token, carol_balance_value) =
        seed_third_account(&fixture, &mut accounts, carol_initial);
    let context = mollusk().with_context(accounts);

    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.alice_initial, 1_000);
    cleartext.seed_amount(fixture.bob_initial, 0);
    cleartext.seed_amount(carol_initial, 0);

    // Alice -> Bob (fresh attested): produces Alice's transferred_amount encrypted value account whose subjects
    // include Bob, so Bob may now spend that received handle.
    let alice_amount = handle_for_chain(50, BALANCE_FHE_TYPE);
    cleartext.seed_amount(alice_amount, 300);
    let first = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        amount_attestation_for(alice_amount, fixture.owner, fixture.compute_signer),
    );
    let first_result = context.process_and_validate_instruction(&first, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &first_result);
    assert_eq!(cleartext.balance(&context, fixture.bob_token), 300);

    let received = fixture.transferred_amount_value_address(fixture.alice_token);
    let received_before = read_encrypted_value(&context, received);
    assert!(received_before.has_subject(fixture.bob_owner));

    // Bob -> Carol, spending the received transferred_amount handle directly (no attestation, no
    // decryption). Bob is a subject of the received amount, so the spend gate passes.
    let forward = confidential_transfer_from_value_ix(
        &fixture,
        fixture.bob_owner,
        fixture.bob_token,
        carol_token,
        fixture.bob_balance_value,
        carol_balance_value,
        received,
    );
    let forward_result = context.process_and_validate_instruction(&forward, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &forward_result);

    assert_eq!(cleartext.balance(&context, fixture.bob_token), 0);
    assert_eq!(cleartext.balance(&context, carol_token), 300);

    // Alice's transferred_amount encrypted value account — the forwarded amount — is untouched by Bob's spend.
    let received_after = read_encrypted_value(&context, received);
    assert_eq!(
        received_after.current_handle,
        received_before.current_handle
    );
    assert_eq!(received_after.leaf_count, received_before.leaf_count);
    assert_eq!(received_after.subjects, received_before.subjects);
}

/// Done-when 5: the RFQ settlement shape — an amount computed via a `select(...)` execution producing a
/// persistent output, then transferred — proven end to end. A transfer's `transferred_amount` is
/// exactly `sub(from_balance, if_then_else(ge, debit, from_balance))`, i.e. a select-computed
/// persistent `euint64`; spending it is the RFQ `eMoved` settlement move.
#[test]
fn mollusk_transfer_from_value_settles_select_computed_amount() {
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    let carol_initial = handle_for_chain(4, BALANCE_FHE_TYPE);
    let (_carol_owner, carol_token, carol_balance_value) =
        seed_third_account(&fixture, &mut accounts, carol_initial);
    let context = mollusk().with_context(accounts);

    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.alice_initial, 900);
    cleartext.seed_amount(fixture.bob_initial, 50);
    cleartext.seed_amount(carol_initial, 0);

    // Compute the amount: Alice -> Bob transfers 400. The select picks the full 400 (balance
    // sufficient), yielding a persistent transferred_amount = 400.
    let alice_amount = handle_for_chain(51, BALANCE_FHE_TYPE);
    cleartext.seed_amount(alice_amount, 400);
    let compute = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        amount_attestation_for(alice_amount, fixture.owner, fixture.compute_signer),
    );
    let compute_result = context.process_and_validate_instruction(&compute, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &compute_result);

    let computed_amount = fixture.transferred_amount_value_address(fixture.alice_token);
    assert_eq!(
        cleartext.transferred_amount(&context, fixture.mint, fixture.alice_token),
        400
    );

    // Settle: Bob transfers the select-computed 400 to Carol.
    let settle = confidential_transfer_from_value_ix(
        &fixture,
        fixture.bob_owner,
        fixture.bob_token,
        carol_token,
        fixture.bob_balance_value,
        carol_balance_value,
        computed_amount,
    );
    let settle_result = context.process_and_validate_instruction(&settle, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &settle_result);

    // Bob had 50 + 400 = 450; settling 400 leaves 50; Carol receives 400.
    assert_eq!(cleartext.balance(&context, fixture.bob_token), 50);
    assert_eq!(cleartext.balance(&context, carol_token), 400);
}

/// A token-owned handle whose subjects lack the mint's compute subject fails at the host's
/// compute-read check; after the token owner grants that subject through the token wrapper it
/// succeeds. Foreign-app values require the equivalent grant from their own account authority.
#[test]
fn mollusk_transfer_from_value_requires_compute_subject_grant() {
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    let amount_handle = handle_for_chain(60, BALANCE_FHE_TYPE);
    // Alice is a subject (she may spend it), but the mint's compute subject is not yet allowed.
    let amount_value = seed_amount_value(
        &fixture,
        &mut accounts,
        fixture.alice_token,
        token::encrypted_transfer_amount_label(),
        amount_handle,
        &[fixture.owner],
    );
    let context = mollusk().with_context(accounts);

    let transfer = confidential_transfer_from_value_ix(
        &fixture,
        fixture.owner,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        amount_value,
    );
    // Without the grant, the host rejects the persistent operand at its compute-read check.
    context.process_and_validate_instruction(
        &transfer,
        &[host_error(host::errors::ZamaHostError::SubjectNotFound)],
    );

    // The token owner grants the mint's compute subject through the token CPI wrapper. Host
    // `allow_subjects` requires the signer to be the token-account PDA that owns this value.
    let grant = allow_token_account_subjects_ix(
        fixture.owner,
        fixture.mint,
        fixture.alice_token,
        amount_value,
        fixture.host_config,
        fixture.compute_signer,
    );
    context.process_and_validate_instruction(&grant, &[Check::success()]);

    // The same transfer now succeeds.
    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.alice_initial, 1_000);
    cleartext.seed_amount(fixture.bob_initial, 100);
    cleartext.seed_amount(amount_handle, 200);
    let transfer_again = confidential_transfer_from_value_ix(
        &fixture,
        fixture.owner,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        amount_value,
    );
    let result = context.process_and_validate_instruction(&transfer_again, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &result);
    assert_eq!(cleartext.balance(&context, fixture.alice_token), 800);
    assert_eq!(cleartext.balance(&context, fixture.bob_token), 300);
}

/// Done-when 3: a signer outside the amount handle's subject set is rejected by the token's spend
/// gate with its own distinct error, before any host CPI.
#[test]
fn mollusk_transfer_from_value_rejects_non_subject_signer() {
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    let amount_handle = handle_for_chain(61, BALANCE_FHE_TYPE);
    // The amount's subjects are Bob + compute; Alice (the from-account owner and signer) is NOT a
    // subject, so she may not spend it even though she owns the debited balance.
    let amount_value = seed_amount_value(
        &fixture,
        &mut accounts,
        fixture.alice_token,
        token::encrypted_transfer_amount_label(),
        amount_handle,
        &[fixture.bob_owner, fixture.compute_signer],
    );
    let context = mollusk().with_context(accounts);

    let transfer = confidential_transfer_from_value_ix(
        &fixture,
        fixture.owner,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        amount_value,
    );
    context.process_and_validate_instruction(
        &transfer,
        &[token_error(
            token::ConfidentialTokenError::AmountSpendSubjectMismatch,
        )],
    );

    // Balances untouched.
    let alice_balance = read_encrypted_value(&context, fixture.alice_balance_value);
    assert_eq!(alice_balance.current_handle, fixture.alice_initial);
}

/// The amount handle must be euint64. A non-balance-typed amount is rejected early by the token for
/// a clear error, before the host's binary type validation would reject the same handle deeper.
#[test]
fn mollusk_transfer_from_value_rejects_non_euint64_amount() {
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    // FHE type 0 (ebool), not euint64.
    let amount_handle = handle_for_chain(62, 0);
    let amount_value = seed_amount_value(
        &fixture,
        &mut accounts,
        fixture.alice_token,
        token::encrypted_transfer_amount_label(),
        amount_handle,
        &[fixture.owner, fixture.compute_signer],
    );
    let context = mollusk().with_context(accounts);

    let transfer = confidential_transfer_from_value_ix(
        &fixture,
        fixture.owner,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        amount_value,
    );
    context.process_and_validate_instruction(
        &transfer,
        &[token_error(
            token::ConfidentialTokenError::AmountHandleTypeMismatch,
        )],
    );
}

/// Spending the entire balance: the amount encrypted value account is the sender's own balance value, so
/// `amount_value` aliases the `from_balance` output account. The execution merges them into one
/// account slot, and the transfer debits the whole balance without tripping duplicate-account
/// resolution.
#[test]
fn mollusk_transfer_from_value_spends_full_balance_with_balance_value_account_as_amount() {
    let fixture = TokenFixture::new();
    let accounts = fixture.base_accounts();
    let context = mollusk().with_context(accounts);

    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.alice_initial, 1_000);
    cleartext.seed_amount(fixture.bob_initial, 100);

    let transfer = confidential_transfer_from_value_ix(
        &fixture,
        fixture.owner,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        // Amount aliased to the sender's own balance encrypted value account: transfer the whole balance.
        fixture.alice_balance_value,
    );
    let result = context.process_and_validate_instruction(&transfer, &[Check::success()]);
    let persistent_outputs = cleartext.evaluate_fhe_cpi(&context, &result);

    assert_eq!(persistent_outputs, 3);
    assert_eq!(cleartext.balance(&context, fixture.alice_token), 0);
    assert_eq!(cleartext.balance(&context, fixture.bob_token), 1_100);
}

/// Re-sending a received amount: the sender spends their own `transferred_amount` encrypted value account, which is
/// also this transfer's `transferred_amount` output account. `amount_value` aliases an output the
/// execution writes, and the merged account slot lets the transfer settle.
#[test]
fn mollusk_transfer_from_value_resends_transferred_amount_that_is_also_this_output() {
    let fixture = TokenFixture::new();
    let accounts = fixture.base_accounts();
    let context = mollusk().with_context(accounts);

    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.alice_initial, 1_000);
    cleartext.seed_amount(fixture.bob_initial, 0);

    // First transfer (attested, 300) creates Alice's transferred_amount encrypted value account.
    let alice_amount = handle_for_chain(90, BALANCE_FHE_TYPE);
    cleartext.seed_amount(alice_amount, 300);
    let first = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        amount_attestation_for(alice_amount, fixture.owner, fixture.compute_signer),
    );
    let first_result = context.process_and_validate_instruction(&first, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &first_result);
    assert_eq!(cleartext.balance(&context, fixture.bob_token), 300);

    // Alice sends the same amount again by spending her own transferred_amount encrypted value account, which is
    // also this transfer's transferred_amount output account.
    let own_transferred = fixture.transferred_amount_value_address(fixture.alice_token);
    let again = confidential_transfer_from_value_ix(
        &fixture,
        fixture.owner,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        own_transferred,
    );
    let again_result = context.process_and_validate_instruction(&again, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &again_result);
    assert_eq!(cleartext.balance(&context, fixture.alice_token), 400);
    assert_eq!(cleartext.balance(&context, fixture.bob_token), 600);
}

/// Done-when 4 (tx-size half): the new arm carries no 190-byte attestation, so its instruction data
/// is strictly SMALLER than the fresh-attested arm's. This is the measured wire-size win that lets a
/// contract-driven settlement pack more into a packet.
#[test]
fn transfer_from_value_instruction_is_smaller_than_attested_arm() {
    let fixture = TokenFixture::new();
    let amount_handle = handle_for_chain(70, BALANCE_FHE_TYPE);
    let attested = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer),
    );
    let from_value = confidential_transfer_from_value_ix(
        &fixture,
        fixture.owner,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        Pubkey::new_unique(),
    );
    eprintln!(
        "confidential_transfer ix data: {} bytes; confidential_transfer_from_value ix data: {} bytes",
        attested.data.len(),
        from_value.data.len(),
    );
    assert!(
        from_value.data.len() < attested.data.len(),
        "from_value arm ({} bytes) must be smaller than the attested arm ({} bytes)",
        from_value.data.len(),
        attested.data.len(),
    );
}

// ---------------------------------------------------------------------------
// Cost snapshots (zama-solana-test-kit::snapshot). Dedicated tests so cost
// drift never fails a behavior test; regenerate with
// `bash scripts/update-cost-snapshots.sh`.
// ---------------------------------------------------------------------------

#[test]
fn cost_snapshot_confidential_transfer_direct() {
    let fixture = TokenFixture::with_keys(
        Pubkey::new_from_array([0x11; 32]),
        Pubkey::new_from_array([0x12; 32]),
        Pubkey::new_from_array([0x13; 32]),
    );
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_handle = handle_for_chain(21, BALANCE_FHE_TYPE);
    let attestation = amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer);
    let transfer = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        attestation,
    );

    let result = context.process_and_validate_instruction(&transfer, &[Check::success()]);

    cost_snapshot::assert_cost_snapshot(
        "token_mollusk",
        "confidential_transfer/direct",
        &transfer,
        &result,
    );

    // Steady state: the first transfer created the transferred-amount
    // `EncryptedValue` at its canonical per-(mint, source) PDA; later
    // transfers update every touched encrypted value account in place and create no
    // accounts. Snapshot the second transfer separately.
    //
    // Both profiles share this fixture/context on purpose, so a mismatch on
    // `direct` fails before `steady_state` is asserted — fix the first drift,
    // then re-run to see whether the second also moved.
    let second_handle = handle_for_chain(22, BALANCE_FHE_TYPE);
    let second_attestation =
        amount_attestation_for(second_handle, fixture.owner, fixture.compute_signer);
    let second_transfer = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        second_attestation,
    );

    let second_result =
        context.process_and_validate_instruction(&second_transfer, &[Check::success()]);

    cost_snapshot::assert_cost_snapshot(
        "token_mollusk",
        "confidential_transfer/steady_state",
        &second_transfer,
        &second_result,
    );
}

#[test]
fn cost_snapshot_confidential_transfer_from_value() {
    let fixture = TokenFixture::with_keys(
        Pubkey::new_from_array([0x11; 32]),
        Pubkey::new_from_array([0x12; 32]),
        Pubkey::new_from_array([0x13; 32]),
    );
    let mut accounts = fixture.base_accounts();
    let amount_handle = handle_for_chain(21, BALANCE_FHE_TYPE);
    let amount_value = seed_amount_value(
        &fixture,
        &mut accounts,
        fixture.alice_token,
        token::encrypted_transfer_amount_label(),
        amount_handle,
        &[fixture.owner, fixture.compute_signer],
    );
    let context = mollusk().with_context(accounts);
    let transfer = confidential_transfer_from_value_ix(
        &fixture,
        fixture.owner,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_value,
        fixture.bob_balance_value,
        amount_value,
    );

    let result = context.process_and_validate_instruction(&transfer, &[Check::success()]);

    cost_snapshot::assert_cost_snapshot(
        "token_mollusk",
        "confidential_transfer_from_value/direct",
        &transfer,
        &result,
    );
}

#[test]
fn cost_snapshot_initialize_token_account() {
    let fixture = TokenFixture::with_keys(
        Pubkey::new_from_array([0x11; 32]),
        Pubkey::new_from_array([0x12; 32]),
        Pubkey::new_from_array([0x13; 32]),
    );
    let owner = Pubkey::new_from_array([0x14; 32]);
    let (token_account, _bump) = token::token_account_address(fixture.mint, owner);
    let balance_encrypted_value =
        token::balance_encrypted_value_address(fixture.mint, token_account).0;
    let mut accounts = fixture.base_accounts();
    accounts.insert(owner, system_account(5_000_000_000));
    accounts.insert(token_account, system_account(0));
    accounts.insert(balance_encrypted_value, system_account(0));
    let context = mollusk().with_context(accounts);
    let ix = initialize_token_account_ix(owner, owner, fixture.mint, fixture.host_config);

    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    cost_snapshot::assert_cost_snapshot("token_mollusk", "initialize_token_account", &ix, &result);
}

#[test]
fn disclose_secp_seven_of_thirteen_verifies_and_bounds_compute() {
    // A realistic 7-of-13 KMS public-decrypt cert verifies through the stateless host verifier (CPIed
    // by disclose_secp) and its compute stays well under budget. Cost is dominated by t secp256k1
    // recoveries (~25k CU each) on top of the single-sig baseline, so a 7-sig cert lands near
    // ~40k + 6 * ~25k; assert a comfortable ceiling.
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(85, BALANCE_FHE_TYPE);
    let replaced = handle_for_chain(86, BALANCE_FHE_TYPE);
    let (value, proof) = public_leaf_value_account(
        fixture.amount_value,
        fixture.token_account,
        fixture.mint,
        token::encrypted_burned_amount_label(),
        &[fixture.owner, fixture.compute_signer],
        pinned,
        Some(replaced),
    );

    // 13 registered KMS signers, public-decrypt threshold 7; the cert is signed by 7 of them. The
    // v0 cert resolves to the CURRENT context (the fixture's), so override that
    // context account with the 13-signer / threshold-7 set.
    let keys: Vec<k256::ecdsa::SigningKey> = (0..13).map(|i| kms_signing_key_n(0x60 + i)).collect();
    let registered: Vec<[u8; 20]> = keys.iter().map(secp_evm_address).collect();

    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_value, encrypted_value_account(&value));
    accounts.insert(
        fixture.kms_context,
        kms_context_account_with_signers(fixture.kms_context_id, &registered, 7),
    );
    let context = mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) =
        kms_public_decrypt_cert_signed_by(pinned, cleartext_amount, &keys[..7]);
    let result = context.process_and_validate_instruction(
        &disclose_secp_ix(
            &fixture,
            fixture.amount_value,
            pinned,
            cleartext_u256(cleartext_amount),
            signatures,
            extra_data,
            proof,
        ),
        &[Check::success()],
    );

    eprintln!(
        "disclose_secp 7-of-13 compute units consumed: {}",
        result.compute_units_consumed
    );
    assert!(
        result.compute_units_consumed < 400_000,
        "7-of-13 disclose consumed {} CU, exceeds the 400k ceiling",
        result.compute_units_consumed
    );
}

// ---------------------------------------------------------------------------
// confidential_burn_from_value (burn an existing encrypted amount, fhevm-internal#1755)
//
// The burn-side analog of confidential_transfer_from_value (#1680 / #3238): burn an amount given as
// an existing persistent handle the owner may use, instead of a fresh coprocessor attestation. The
// burned-amount output shape is byte-identical to the attestation path (created publicly decryptable at
// its canonical burned_amount encrypted value account), so redeem_burned_amount consumes it unchanged.
// ---------------------------------------------------------------------------

/// Happy path: burn part of a balance from an existing computed/received `euint64` handle, no
/// attestation attached. The burned delta is created publicly decryptable exactly as the attestation
/// path, the balance and encrypted total supply decrement by the burned amount, and the amount value
/// itself is read-only (never replaced, never consumed).
#[test]
fn mollusk_burn_from_value_burns_existing_amount() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(1_000);
    let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let amount_value = seed_burn_amount_value(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::encrypted_transfer_amount_label(),
        amount_handle,
        &[fixture.owner, fixture.compute_signer],
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.initial_balance, 1_000);
    cleartext.seed_amount(fixture.initial_total_supply, 5_000);
    cleartext.seed_amount(amount_handle, 250);

    let burn = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        amount_value,
    );
    let result = context.process_and_validate_instruction(&burn, &[Check::success()]);
    let persistent_outputs = cleartext.evaluate_fhe_cpi(&context, &result);

    // Three persistent outputs are updated — balance, burned_amount, total_supply — and the amount is not one.
    assert_eq!(persistent_outputs, 3);
    assert_eq!(cleartext.balance(&context, fixture.token_account), 750);
    assert_eq!(
        cleartext.u64_at(&context, fixture.total_supply_value),
        4_750
    );

    // The burned delta is created publicly decryptable: the first burn creates the encrypted value account and appends
    // exactly one public-decrypt leaf for the just-bound burned handle (DD-036 / Vector 2), identical
    // to the attestation path.
    let burned = read_encrypted_value(&context, fixture.burned_amount_value);
    assert_eq!(cleartext.u64_at(&context, fixture.burned_amount_value), 250);
    assert_eq!(burned.leaf_count, 1);
    assert_eq!(burned.subjects, vec![fixture.owner, fixture.compute_signer]);
    let public_leaf = zama_solana_acl::public_decrypt_leaf_commitment(
        fixture.burned_amount_value.to_bytes(),
        0,
        burned.current_handle,
    );
    assert_eq!(
        burned.peaks,
        zama_solana_acl::mmr_peaks_from_leaves(&[public_leaf])
    );

    // The amount value is read-only: current handle, history, and subjects all unchanged.
    let amount_after = read_encrypted_value(&context, amount_value);
    assert_eq!(amount_after.current_handle, amount_handle);
    assert_eq!(amount_after.leaf_count, 0);
    assert_eq!(
        amount_after.subjects,
        vec![fixture.owner, fixture.compute_signer]
    );
}

/// Whole-balance alias regression (the #3238 aliasing class): burning the entire balance uses the
/// account's own balance encrypted value account AS the amount, so `amount_value` aliases the `balance` output. The
/// execution merges them into one slot, and the dedup skips pushing the amount a second time, so the
/// burn settles without tripping duplicate-account resolution.
#[test]
fn mollusk_burn_from_value_whole_balance_alias() {
    let fixture = BurnRedeemFixture::new();
    let accounts = fixture.accounts(1_000);
    let context = burn_redeem_mollusk().with_context(accounts);

    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.initial_balance, 1_000);
    cleartext.seed_amount(fixture.initial_total_supply, 5_000);

    // Amount aliased to the account's own balance encrypted value account: burn the whole balance.
    let burn = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        fixture.balance_value,
    );
    let result = context.process_and_validate_instruction(&burn, &[Check::success()]);
    let persistent_outputs = cleartext.evaluate_fhe_cpi(&context, &result);

    assert_eq!(persistent_outputs, 3);
    assert_eq!(cleartext.balance(&context, fixture.token_account), 0);
    assert_eq!(
        cleartext.u64_at(&context, fixture.total_supply_value),
        4_000
    );
    assert_eq!(
        cleartext.u64_at(&context, fixture.burned_amount_value),
        1_000
    );
}

/// Re-burning the burned-amount encrypted value account (the second alias branch): the second burn spends the
/// `burned_amount` encrypted value account itself as the amount, so `amount_value` aliases the `burned_amount` output
/// this execution writes. The execution merges the aliased slot (read at the old handle, replaced to
/// the new delta), and the dedup skips pushing the amount a second time — the `amount == burned_amount
/// encrypted value account` branch. Mirrors `mollusk_transfer_from_value_resends_transferred_amount_that_is_also_this_output`.
#[test]
fn mollusk_burn_from_value_reburns_burned_amount_that_is_also_this_output() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(1_000);
    let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let amount_value = seed_burn_amount_value(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::encrypted_transfer_amount_label(),
        amount_handle,
        &[fixture.owner, fixture.compute_signer],
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.initial_balance, 1_000);
    cleartext.seed_amount(fixture.initial_total_supply, 5_000);
    cleartext.seed_amount(amount_handle, 250);

    // First burn (250) creates the burned_amount encrypted value account: balance 750, total_supply 4750, burned 250.
    let first = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        amount_value,
    );
    let first_result = context.process_and_validate_instruction(&first, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &first_result);
    let first_burned = read_encrypted_value(&context, fixture.burned_amount_value).current_handle;
    assert_eq!(cleartext.u64_at(&context, fixture.burned_amount_value), 250);

    // Settle the first pending burn before opening the next one. Cancellation restores balance and
    // supply while retaining the burned-amount encrypted value account and its public leaf.
    let pending_burn = token::pending_burn_address(fixture.mint, fixture.token_account).0;
    let cancel = cancel_pending_burn_ix(&fixture, pending_burn);
    let cancel_result = context.process_and_validate_instruction(&cancel, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &cancel_result);

    // The next burn spends the burned-amount encrypted value account itself as the amount — which is also this burn's
    // burned_amount output account (the alias the dedup must merge, not double-push).
    let again = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        fixture.burned_amount_value,
    );
    let again_result = context.process_and_validate_instruction(&again, &[Check::success()]);
    let persistent_outputs = cleartext.evaluate_fhe_cpi(&context, &again_result);

    // Conservation after cancellation: the next burn's amount equals the previous burned delta
    // (250), so the restored balance and encrypted total supply each drop by 250.
    assert_eq!(persistent_outputs, 3);
    assert_eq!(cleartext.balance(&context, fixture.token_account), 750);
    assert_eq!(
        cleartext.u64_at(&context, fixture.total_supply_value),
        4_750
    );
    assert_eq!(cleartext.u64_at(&context, fixture.burned_amount_value), 250);

    // The burned_amount encrypted value account stays a well-formed two-burn MMR: both burns' handles are present as
    // public-decrypt leaves (public(H1)@0 and public(H2)@3), even though the first pending burn was
    // cancelled and the second burn read H1 as its amount operand. Only H2 remains pending and
    // redeemable.
    let second_burned = read_encrypted_value(&context, fixture.burned_amount_value).current_handle;
    let value = read_encrypted_value(&context, fixture.burned_amount_value);
    assert_eq!(value.current_handle, second_burned);
    assert_eq!(value.leaf_count, 4);
    let (_proof, expected_peaks) =
        two_burn_value_account_proof(&fixture, first_burned, second_burned, 0);
    assert_eq!(value.peaks, expected_peaks);
}

/// PDA-owner CPI driver: the batcher path burns as a program PDA that owns the token account and
/// authorizes the burn via `invoke_signed`. The callee sees only `owner.is_signer` — identical
/// whether a keypair or a program's PDA signed — so the path is exercised by marking the owner PDA a
/// signer and paying rent from a separate keypair (the driver's fee payer, as `invoke_signed`
/// would). The spend gate and owner check both accept the PDA owner.
#[test]
fn mollusk_burn_from_value_pda_owner_via_invoke_signed() {
    // A program PDA stands in for the batcher authority that owns the token account.
    let driver_program = Pubkey::new_from_array([0x42; 32]);
    let (pda_owner, _bump) = Pubkey::find_program_address(&[b"batcher"], &driver_program);
    let fixture = BurnRedeemFixture::with_keys(
        pda_owner,
        Pubkey::new_from_array([0x21; 32]),
        Pubkey::new_from_array([0x22; 32]),
    );
    let mut accounts = fixture.accounts(1_000);
    // A separate keypair pays rent, exactly as invoke_signed would — the PDA is not the fee payer.
    let payer = Pubkey::new_unique();
    accounts.insert(payer, system_account(5_000_000_000));
    let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let amount_value = seed_burn_amount_value(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::encrypted_transfer_amount_label(),
        amount_handle,
        &[pda_owner, fixture.compute_signer],
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.initial_balance, 1_000);
    cleartext.seed_amount(fixture.initial_total_supply, 5_000);
    cleartext.seed_amount(amount_handle, 400);

    let burn =
        confidential_burn_from_value_auto(&context, &fixture, pda_owner, payer, amount_value);
    let result = context.process_and_validate_instruction(&burn, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &result);

    assert_eq!(cleartext.balance(&context, fixture.token_account), 600);
    assert_eq!(cleartext.u64_at(&context, fixture.burned_amount_value), 400);
    // The burned encrypted value account is owned by the PDA owner and the compute signer.
    assert_eq!(
        read_encrypted_value(&context, fixture.burned_amount_value).subjects,
        vec![pda_owner, fixture.compute_signer]
    );
}

/// Downstream compatibility: a burned handle produced by the from-value path feeds
/// `redeem_burned_amount` unchanged. The burned output shape (created-public, canonical `burned_amount`
/// encrypted value account, audience owner + compute) is identical to the attestation path, so the KMS-cert +
/// single-leaf public-decrypt-proof redeem consumes it and pays out the vault.
#[test]
fn mollusk_burn_from_value_burned_handle_redeems() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(1_000);
    let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let amount_value = seed_burn_amount_value(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::encrypted_transfer_amount_label(),
        amount_handle,
        &[fixture.owner, fixture.compute_signer],
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    // Burn 500 from the existing amount handle; the created-public burned handle is the new encrypted value account handle.
    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.initial_balance, 1_000);
    cleartext.seed_amount(fixture.initial_total_supply, 5_000);
    cleartext.seed_amount(amount_handle, 500);
    let burn = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        amount_value,
    );
    let burn_result = context.process_and_validate_instruction(&burn, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &burn_result);
    let burned_handle = read_encrypted_value(&context, fixture.burned_amount_value).current_handle;

    // Redeem the burned handle with a real KMS cert + single-leaf public-decrypt inclusion proof.
    let cleartext_amount = 500;
    let (signatures, extra_data) = kms_public_decrypt_cert(burned_handle, cleartext_amount);
    let proof = single_burn_public_decrypt_proof(&fixture, burned_handle);
    let pending_burn = token::pending_burn_address(fixture.mint, fixture.token_account).0;
    context.process_and_validate_instruction(
        &redeem_burned_amount_ix(
            &fixture,
            burned_handle,
            cleartext_amount,
            signatures,
            extra_data,
            proof,
            pending_burn,
        ),
        &[Check::success()],
    );

    assert_eq!(
        read_spl_amount(&context, fixture.destination_usdc),
        cleartext_amount
    );
    assert_eq!(
        read_spl_amount(&context, fixture.vault_usdc),
        1_000 - cleartext_amount
    );
}

/// A signer outside the amount handle's subject set is rejected by the token's spend gate with its
/// own distinct error, before any host CPI — even though it owns the debited token account.
#[test]
fn mollusk_burn_from_value_rejects_non_subject_signer() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(1_000);
    let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    // The amount's subjects are a stranger + compute; the owner (and signer) is NOT a subject, so it
    // may not burn the amount even though it owns the balance.
    let stranger = Pubkey::new_unique();
    let amount_value = seed_burn_amount_value(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::encrypted_transfer_amount_label(),
        amount_handle,
        &[stranger, fixture.compute_signer],
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let burn = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        amount_value,
    );
    context.process_and_validate_instruction(
        &burn,
        &[token_error(
            token::ConfidentialTokenError::AmountSpendSubjectMismatch,
        )],
    );

    // Balance untouched.
    assert_eq!(
        read_encrypted_value(&context, fixture.balance_value).current_handle,
        fixture.initial_balance
    );
}

/// The amount handle must be euint64. A non-balance-typed amount is rejected early by the token for a
/// clear error, before the host's binary type validation would reject the same handle deeper.
#[test]
fn mollusk_burn_from_value_rejects_non_euint64_amount() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(1_000);
    // FHE type 0 (ebool), not euint64.
    let amount_handle = handle_for_chain(42, 0);
    let amount_value = seed_burn_amount_value(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::encrypted_transfer_amount_label(),
        amount_handle,
        &[fixture.owner, fixture.compute_signer],
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let burn = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        amount_value,
    );
    context.process_and_validate_instruction(
        &burn,
        &[token_error(
            token::ConfidentialTokenError::AmountHandleTypeMismatch,
        )],
    );
}

/// The signing owner must own the debited token account. A signer that is a subject of the amount
/// (so the spend gate passes) but is not the token account owner is rejected with `OwnerMismatch`.
#[test]
fn mollusk_burn_from_value_rejects_owner_not_token_account_owner() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(1_000);
    let wrong_owner = Pubkey::new_unique();
    accounts.insert(wrong_owner, system_account(5_000_000_000));
    let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    // wrong_owner is a subject (spend gate passes) but does not own the token account.
    let amount_value = seed_burn_amount_value(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::encrypted_transfer_amount_label(),
        amount_handle,
        &[wrong_owner, fixture.compute_signer],
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let burn = confidential_burn_from_value_auto(
        &context,
        &fixture,
        wrong_owner,
        wrong_owner,
        amount_value,
    );
    context.process_and_validate_instruction(
        &burn,
        &[token_error(token::ConfidentialTokenError::OwnerMismatch)],
    );
}

/// A token-owned amount handle whose subjects lack the mint's compute subject fails at the host's
/// compute-read check; after the token owner grants that subject through the token wrapper the same
/// burn succeeds. Foreign-app values require the equivalent grant from their own account authority.
#[test]
fn mollusk_burn_from_value_requires_compute_subject_grant() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(1_000);
    let amount_handle = handle_for_chain(60, BALANCE_FHE_TYPE);
    // The owner may spend this handle, but the mint's compute subject is not yet allowed on it.
    let amount_value = seed_burn_amount_value(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::encrypted_transfer_amount_label(),
        amount_handle,
        &[fixture.owner],
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let burn = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        amount_value,
    );
    // Without the grant, the host rejects the persistent operand at its compute-read check.
    context.process_and_validate_instruction(
        &burn,
        &[host_error(host::errors::ZamaHostError::SubjectNotFound)],
    );

    // The token owner grants the mint's compute subject through the token CPI wrapper. Host
    // `allow_subjects` requires the signer to be the token-account PDA that owns this value.
    let grant = allow_token_account_subjects_ix(
        fixture.owner,
        fixture.mint,
        fixture.token_account,
        amount_value,
        fixture.host_config,
        fixture.compute_signer,
    );
    context.process_and_validate_instruction(&grant, &[Check::success()]);

    // The same burn now succeeds.
    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.initial_balance, 1_000);
    cleartext.seed_amount(fixture.initial_total_supply, 5_000);
    cleartext.seed_amount(amount_handle, 300);
    let burn_again = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        amount_value,
    );
    let result = context.process_and_validate_instruction(&burn_again, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &result);
    assert_eq!(cleartext.balance(&context, fixture.token_account), 700);
    assert_eq!(cleartext.u64_at(&context, fixture.burned_amount_value), 300);
}

/// The from-value burn carries no 190-byte attestation, so its instruction data is strictly SMALLER
/// than the fresh-attested burn's — the measured wire-size win for a contract-driven execution burn.
#[test]
fn burn_from_value_instruction_is_smaller_than_attested_arm() {
    let fixture = BurnRedeemFixture::new();
    let amount_handle = handle_for_chain(70, BALANCE_FHE_TYPE);
    let pending_burn = token::pending_burn_address(fixture.mint, fixture.token_account).0;
    let attested = confidential_burn_ix(
        &fixture,
        amount_attestation_for(amount_handle, fixture.owner, fixture.compute_signer),
        pending_burn,
    );
    let from_value = confidential_burn_from_value_ix(
        &fixture,
        fixture.owner,
        fixture.owner,
        Pubkey::new_unique(),
        pending_burn,
    );
    assert!(
        from_value.data.len() < attested.data.len(),
        "from_value arm ({} bytes) must be smaller than the attested arm ({} bytes)",
        from_value.data.len(),
        attested.data.len(),
    );
}

#[test]
fn cost_snapshot_confidential_burn_from_value() {
    let fixture = BurnRedeemFixture::with_keys(
        Pubkey::new_from_array([0x11; 32]),
        Pubkey::new_from_array([0x12; 32]),
        Pubkey::new_from_array([0x13; 32]),
    );
    let mut accounts = fixture.accounts(1_000);
    let amount_handle = handle_for_chain(21, BALANCE_FHE_TYPE);
    let amount_value = seed_burn_amount_value(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::encrypted_transfer_amount_label(),
        amount_handle,
        &[fixture.owner, fixture.compute_signer],
    );
    let context = burn_redeem_mollusk().with_context(accounts);
    let burn = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        amount_value,
    );

    let result = context.process_and_validate_instruction(&burn, &[Check::success()]);

    cost_snapshot::assert_cost_snapshot(
        "token_mollusk",
        "confidential_burn_from_value/direct",
        &burn,
        &result,
    );
}
