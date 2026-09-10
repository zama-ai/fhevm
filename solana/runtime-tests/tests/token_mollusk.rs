//! Mollusk-based runtime tests for `confidential-token` against the RFC-035 `EncryptedStore`
//! ACL model.
//!
//! Every encrypted field of the token (`balance`, `total_supply`, `transferred_amount`,
//! `burned_amount`) lives in one stable `EncryptedStore` PDA keyed by the token application
//! (`token_app(mint)` = this program in the mint's scope), the value's signing authority (the
//! token-account or total-supply PDA) and a label. Who may decrypt a handle is decided by the write
//! that produces it: each write allows its holder(s) on the new handle and seals one leaf per allow
//! into the value's MMR. The account keeps no viewer list to rotate; granting a viewer is a re-write
//! (`allow_balance_viewers`, `allow_total_supply_viewers`). See `confidential-token/src/fhe`,
//! `zama-host/src/state/encrypted_store.rs`, and `zama_solana_acl` for the model this exercises.
//!
//! Scope note: the suite covers mint/token-account creation, `confidential_transfer`'s persistent
//! outputs and their allow leaves, the viewer re-writes, the deny list keyed by the mint's scope,
//! the two consume paths that are thin consumers of the stateless host `verify_public_decrypt`
//! (DD-040: `disclose_secp` stays idempotent, `redeem_burned_amount` consumes the sequential
//! `PendingBurn` exactly once), confidential burn, cancellation and `wrap_usdc` with SPL Token and
//! Token-2022 fixtures, and the from-value spend arms.
//!
//! Encrypted store is asserted through the cleartext ledger: the tests evaluate the canonical
//! `FheExecuteArgs` captured from the real token -> host CPI and bind those clear values to the
//! handles emitted by the host.

use anchor_lang::{prelude::system_program, AccountDeserialize, Discriminator};
use anchor_spl::associated_token::get_associated_token_address_with_program_id;
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
use std::cell::RefCell;
use std::collections::HashMap;
use zama_host as host;
use zama_solana_test_kit::oracle::CleartextLedger;
use zama_solana_test_kit::signing::{
    amount_attestation_for, amount_attestation_signed_by, amount_public_decrypt_cert,
    amount_public_decrypt_cert_signed_by, coprocessor_signing_key, coprocessor_signing_key_n,
    secp_evm_address,
};
use zama_solana_test_kit::{
    anchor_error_check, anchor_framework_error_check, anchor_ix, canonical_test_context_id,
    cost_snapshot, decode_anchor_event, deny_scope_record_account, encrypted_store_account,
    event_authority, handle_for_chain, new_encrypted_store, read_account, read_encrypted_store,
    read_spl_amount, read_store_handle, serialized_account, spl_mint_account, spl_token_account,
    system_account, u256_be, Ctx, HostConfigParams, BALANCE_FHE_TYPE, DECRYPTION_CONTRACT,
    GATEWAY_CHAIN_ID,
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

/// Runs FHE entry points in the client transaction envelope. Public-only token
/// instructions keep their native fixtures. Payer positions follow the typed ABI above.
fn check_token_instruction(context: &Ctx, ix: &Instruction, checks: &[Check]) -> InstructionResult {
    let payer_index = if [
        token::instruction::ConfidentialTransfer::DISCRIMINATOR,
        token::instruction::ConfidentialTransferFromValue::DISCRIMINATOR,
        token::instruction::ConfidentialBurnFromValue::DISCRIMINATOR,
    ]
    .iter()
    .any(|tag| ix.data.starts_with(tag))
    {
        Some(1)
    } else if [
        token::instruction::InitializeMint::DISCRIMINATOR,
        token::instruction::InitializeTokenAccount::DISCRIMINATOR,
        token::instruction::AllowBalanceViewers::DISCRIMINATOR,
        token::instruction::AllowTotalSupplyViewers::DISCRIMINATOR,
        token::instruction::ConfidentialBurn::DISCRIMINATOR,
        token::instruction::CancelPendingBurn::DISCRIMINATOR,
        token::instruction::WrapUsdc::DISCRIMINATOR,
    ]
    .iter()
    .any(|tag| ix.data.starts_with(tag))
    {
        Some(0)
    } else {
        None
    };
    if ix.program_id == token::ID {
        if let Some(index) = payer_index {
            return zama_solana_test_kit::transaction::process_fhe_instruction(
                context,
                ix.accounts[index].pubkey,
                ix,
                checks,
            );
        }
    }
    context.process_and_validate_instruction(ix, checks)
}

fn transferred_event_handle(result: &InstructionResult) -> [u8; 32] {
    result
        .inner_instructions
        .iter()
        .find_map(|inner| {
            decode_anchor_event::<token::ConfidentialTransferEvent>(&inner.instruction.data)
        })
        .expect("transfer event")
        .transferred_handle
}

/// Token-suite views over the kit's [`CleartextLedger`]: the single-CPI replay every token
/// instruction is expected to issue, and the two encrypted-field reads the assertions use.
trait TokenLedgerExt {
    fn evaluate_fhe_cpi(&mut self, context: &Ctx, result: &InstructionResult) -> usize;
    fn balance(&self, context: &Ctx, token_account: Pubkey) -> u64;
}

impl TokenLedgerExt for CleartextLedger {
    /// Applies the exact FHE execution invoked by the token program and associates each persistent
    /// result with the handle persisted in its canonical `EncryptedStore` account.
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
        self.u64_in_state(
            context,
            token::encrypted_store_address(account.mint, token_account).0,
            token::balance_key(),
        )
    }
}

fn host_config_account(admin: Pubkey, coprocessor_signer: [u8; 20]) -> Account {
    host_config_account_with_flags(admin, &[coprocessor_signer], 1, [0u8; 32], false)
}

fn host_config_account_with_kms_context(
    admin: Pubkey,
    coprocessor_signer: [u8; 20],
    current_kms_context_id: [u8; 32],
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
    host_config_account_with_flags(admin, coprocessor_signers, threshold, [0u8; 32], false)
}

fn host_config_account_with_flags(
    admin: Pubkey,
    coprocessor_signers: &[[u8; 20]],
    coprocessor_threshold: u8,
    current_kms_context_id: [u8; 32],
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
    host_config_account_with_flags(admin, &[coprocessor_signer], 1, [0u8; 32], true)
}

fn read_token_account(context: &Ctx, address: Pubkey) -> token::ConfidentialTokenAccount {
    read_account(context, address)
}

fn read_confidential_mint(context: &Ctx, address: Pubkey) -> token::ConfidentialMint {
    read_account(context, address)
}

/// The MMR leaves one write appends to `encrypted_store`: one historical-access leaf per allowed
/// key on the handle it wrote, in allow order, from `first_index`.
fn allow_leaves(
    encrypted_store: Pubkey,
    first_index: u64,
    handle: [u8; 32],
    allows: &[Pubkey],
) -> Vec<[u8; 32]> {
    allows
        .iter()
        .enumerate()
        .map(|(offset, key)| {
            zama_solana_acl::historical_access_leaf_commitment(
                encrypted_store.to_bytes(),
                first_index + offset as u64,
                handle,
                key.to_bytes(),
            )
        })
        .collect()
}

/// Peaks of a value written exactly once, allowing `allows` on `handle`.
fn expected_allow_peaks(
    encrypted_store: Pubkey,
    handle: [u8; 32],
    allows: &[Pubkey],
) -> Vec<[u8; 32]> {
    zama_solana_acl::mmr_peaks_from_leaves(&allow_leaves(encrypted_store, 0, handle, allows))
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

fn new_test_state(
    app: host::AppScope,
    authority: Pubkey,
    key: [u8; 32],
    handle: [u8; 32],
) -> (Pubkey, host::EncryptedStore) {
    new_encrypted_store(app, authority, [(key, handle)])
}

fn insert_store_slot(
    accounts: &mut HashMap<Pubkey, Account>,
    app: host::AppScope,
    authority: Pubkey,
    key: [u8; 32],
    handle: [u8; 32],
) -> Pubkey {
    let (address, state) = new_test_state(app, authority, key, handle);
    if let Some(account) = accounts.get_mut(&address) {
        let mut current = host::EncryptedStore::try_deserialize(&mut &account.data[..])
            .expect("existing encrypted store");
        current.slots.push(host::EncryptedSlot { key, handle });
        account.data = serialized_account(current);
    } else {
        accounts.insert(address, encrypted_store_account(&state));
    }
    address
}

fn store_handle(state: &host::EncryptedStore, key: [u8; 32]) -> [u8; 32] {
    state.get(&key).expect("encrypted store slot")
}

// ---------------------------------------------------------------------------
// Fixture
// ---------------------------------------------------------------------------

struct TokenFixture {
    owner: Pubkey,
    bob_owner: Pubkey,
    mint: Pubkey,
    underlying_mint: Pubkey,
    host_config: Pubkey,
    alice_token: Pubkey,
    bob_token: Pubkey,
    alice_balance_store: Pubkey,
    bob_balance_store: Pubkey,
    alice_initial: [u8; 32],
    bob_initial: [u8; 32],
    extra_token_owners: RefCell<HashMap<Pubkey, Pubkey>>,
    state_keys: RefCell<HashMap<Pubkey, [u8; 32]>>,
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
        let host_config = host::host_config_address().0;
        let alice_token = token::token_account_address(mint, owner).0;
        let bob_token = token::token_account_address(mint, bob_owner).0;
        let alice_balance_store = token::encrypted_store_address(mint, alice_token).0;
        let bob_balance_store = token::encrypted_store_address(mint, bob_token).0;
        let underlying_mint =
            Pubkey::find_program_address(&[b"test-underlying", mint.as_ref()], &token::id()).0;
        Self {
            owner,
            bob_owner,
            mint,
            underlying_mint,
            host_config,
            alice_token,
            bob_token,
            alice_balance_store,
            bob_balance_store,
            alice_initial: handle_for_chain(1, BALANCE_FHE_TYPE),
            bob_initial: handle_for_chain(2, BALANCE_FHE_TYPE),
            extra_token_owners: RefCell::new(HashMap::new()),
            state_keys: RefCell::new(HashMap::new()),
        }
    }

    /// The token application this mint's values belong to.
    fn app(&self) -> host::AppScope {
        token::token_app(self.mint)
    }

    fn confidential_mint_account(&self) -> Account {
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(token::ConfidentialMint {
                authority: self.owner,
                underlying_mint: self.underlying_mint,
                decimals: 6,
            }),
            owner: token::id(),
            executable: false,
            rent_epoch: 0,
        }
    }

    fn confidential_token_account(&self, owner: Pubkey, _balance_store: Pubkey) -> Account {
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(token::ConfidentialTokenAccount {
                owner,
                mint: self.mint,
                bump: token::token_account_address(self.mint, owner).1,
            }),
            owner: token::id(),
            executable: false,
            rent_epoch: 0,
        }
    }

    fn base_accounts(&self) -> HashMap<Pubkey, Account> {
        let (alice_balance_address, alice_balance_store) = new_encrypted_store(
            self.app(),
            self.alice_token,
            [(token::balance_key(), self.alice_initial)],
        );
        assert_eq!(alice_balance_address, self.alice_balance_store);
        let (bob_balance_address, bob_balance_store) = new_encrypted_store(
            self.app(),
            self.bob_token,
            [(token::balance_key(), self.bob_initial)],
        );
        assert_eq!(bob_balance_address, self.bob_balance_store);

        HashMap::from([
            (self.owner, system_account(5_000_000_000)),
            (self.bob_owner, system_account(5_000_000_000)),
            (self.mint, self.confidential_mint_account()),
            (
                self.host_config,
                host_config_account(self.owner, secp_evm_address(&coprocessor_signing_key())),
            ),
            (
                self.alice_token,
                self.confidential_token_account(self.owner, self.alice_balance_store),
            ),
            (
                self.bob_token,
                self.confidential_token_account(self.bob_owner, self.bob_balance_store),
            ),
            (
                self.alice_balance_store,
                encrypted_store_account(&alice_balance_store),
            ),
            (
                self.bob_balance_store,
                encrypted_store_account(&bob_balance_store),
            ),
            (event_authority(host::id()), system_account(0)),
            (event_authority(token::id()), system_account(0)),
            (self.underlying_mint, spl_mint_account(None, 0)),
            (self.owner_ata(self.owner), system_account(0)),
            (self.owner_ata(self.bob_owner), system_account(0)),
        ])
    }

    fn transferred_amount_store_address(&self, from_token: Pubkey) -> Pubkey {
        token::encrypted_store_address(self.mint, from_token).0
    }

    fn owner_ata(&self, owner: Pubkey) -> Pubkey {
        get_associated_token_address_with_program_id(
            &owner,
            &self.underlying_mint,
            &spl_token::id(),
        )
    }

    fn owner_of_token(&self, token: Pubkey) -> Pubkey {
        if token == self.alice_token {
            self.owner
        } else if token == self.bob_token {
            self.bob_owner
        } else {
            self.extra_token_owners
                .borrow()
                .get(&token)
                .copied()
                .unwrap_or_else(|| panic!("unknown token account {token}"))
        }
    }

    fn register_token_owner(&self, token: Pubkey, owner: Pubkey) {
        self.extra_token_owners.borrow_mut().insert(token, owner);
    }

    fn register_state_key(&self, state: Pubkey, key: [u8; 32]) {
        self.state_keys.borrow_mut().insert(state, key);
    }

    fn key_for_state(&self, state: Pubkey) -> [u8; 32] {
        if let Some(key) = self.state_keys.borrow().get(&state).copied() {
            key
        } else if state == self.alice_balance_store || state == self.bob_balance_store {
            token::balance_key()
        } else {
            panic!("unknown state slot for {state}")
        }
    }

    fn underlying_ata_for_token(&self, token: Pubkey) -> Pubkey {
        self.owner_ata(self.owner_of_token(token))
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
    anchor_ix(
        token::id(),
        token::accounts::InitializeMint {
            transient_store: host::transient_store_address(authority).0,
            instructions: solana_sdk::sysvar::instructions::ID,
            authority,
            mint,
            underlying_mint,
            token_program: spl_token::id(),
            total_supply_authority: token::total_supply_authority_address(mint).0,
            total_supply_encrypted_store: token::encrypted_store_address(
                mint,
                token::total_supply_authority_address(mint).0,
            )
            .0,
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
    let (token_account, _bump) = token::token_account_address(mint, owner);
    anchor_ix(
        token::id(),
        token::accounts::InitializeTokenAccount {
            transient_store: host::transient_store_address(payer).0,
            instructions: solana_sdk::sysvar::instructions::ID,
            payer,
            owner,
            mint,
            token_account,
            balance_encrypted_store: token::encrypted_store_address(mint, token_account).0,
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
    from_store: Pubkey,
    to_store: Pubkey,
    amount_attestation: host::CoprocessorInputAttestation,
) -> Instruction {
    confidential_transfer_ix_with_remaining(
        fixture,
        from_token,
        to_token,
        from_store,
        to_store,
        amount_attestation,
        Vec::new(),
    )
}

fn confidential_self_transfer_with_result_grant_ix(
    fixture: &TokenFixture,
    result_store: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::ConfidentialTransfer {
            transient_store: host::transient_store_address(fixture.owner).0,
            instructions: solana_sdk::sysvar::instructions::ID,
            owner: fixture.owner,
            payer: fixture.owner,
            mint: fixture.mint,
            underlying_mint: fixture.underlying_mint,
            from_ata: fixture.owner_ata(fixture.owner),
            to_ata: fixture.owner_ata(fixture.owner),
            from_account: fixture.alice_token,
            to_account: fixture.alice_token,
            from_store: fixture.alice_balance_store,
            to_store: fixture.alice_balance_store,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            result_store: Some(result_store),

            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::ConfidentialTransfer {
            amount_attestation: sender_attestation(fixture, 9),
        },
    )
}

fn confidential_transfer_ix_with_remaining(
    fixture: &TokenFixture,
    from_token: Pubkey,
    to_token: Pubkey,
    from_store: Pubkey,
    to_store: Pubkey,
    amount_attestation: host::CoprocessorInputAttestation,
    remaining: Vec<Pubkey>,
) -> Instruction {
    confidential_transfer_ix_with_block_cap_accounts(
        fixture,
        from_token,
        to_token,
        from_store,
        to_store,
        amount_attestation,
        remaining,
        None,
        None,
    )
}

/// Block-cap optional accounts threaded through the transfer CPI explicitly; used by the HCU
/// block-cap tests to vary the meter / trust witness. The default unrestricted cap means
/// `confidential_transfer_ix_with_remaining` passes `None`/`None`. Metering keys on the token
/// application (this program in the mint's scope).
#[allow(clippy::too_many_arguments)]
fn confidential_transfer_ix_with_block_cap_accounts(
    fixture: &TokenFixture,
    from_token: Pubkey,
    to_token: Pubkey,
    from_store: Pubkey,
    to_store: Pubkey,
    amount_attestation: host::CoprocessorInputAttestation,
    remaining: Vec<Pubkey>,
    hcu_block_meter: Option<Pubkey>,
    hcu_trusted_app_record: Option<Pubkey>,
) -> Instruction {
    let mut ix = anchor_ix(
        token::id(),
        token::accounts::ConfidentialTransfer {
            transient_store: host::transient_store_address(fixture.owner).0,
            instructions: solana_sdk::sysvar::instructions::ID,
            owner: fixture.owner,
            payer: fixture.owner,
            mint: fixture.mint,
            underlying_mint: fixture.underlying_mint,
            from_ata: fixture.underlying_ata_for_token(from_token),
            to_ata: fixture.underlying_ata_for_token(to_token),
            from_account: from_token,
            to_account: to_token,
            from_store,
            to_store,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            hcu_block_meter,
            hcu_trusted_app_record,
            result_store: None,

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
/// on-chain `EncryptedStore` at `amount_store` (a computed or received handle) rather than a fresh
/// attestation. `signer_owner` signs and pays; it must own `from_token`, and the amount value must
/// be under its own or `from_token`'s authority.
#[allow(clippy::too_many_arguments)]
fn confidential_transfer_from_value_ix(
    fixture: &TokenFixture,
    signer_owner: Pubkey,
    from_token: Pubkey,
    to_token: Pubkey,
    from_store: Pubkey,
    to_store: Pubkey,
    amount_store: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::ConfidentialTransferFromValue {
            transient_store: host::transient_store_address(signer_owner).0,
            instructions: solana_sdk::sysvar::instructions::ID,
            owner: signer_owner,
            payer: signer_owner,
            mint: fixture.mint,
            underlying_mint: fixture.underlying_mint,
            from_ata: fixture.underlying_ata_for_token(from_token),
            to_ata: fixture.underlying_ata_for_token(to_token),
            from_account: from_token,
            to_account: to_token,
            from_store,
            to_store,
            amount_store: Some(amount_store),
            amount_authority: None,

            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::ConfidentialTransferFromValue {
            amount_source: token::TransferInput::Slot {
                key: fixture.key_for_state(amount_store),
            },
        },
    )
}

/// Seeds a spendable amount encrypted store (a stand-in for a computed/received `euint64`
/// handle) of this mint's application under `authority` at `label`, and returns its address.
fn seed_amount_store(
    fixture: &TokenFixture,
    accounts: &mut HashMap<Pubkey, Account>,
    authority: Pubkey,
    encrypted_store_label: [u8; 32],
    handle: [u8; 32],
) -> Pubkey {
    let address = insert_store_slot(
        accounts,
        fixture.app(),
        authority,
        encrypted_store_label,
        handle,
    );
    fixture.register_state_key(address, encrypted_store_label);
    address
}

/// Token `allow_balance_viewers`: the owner-authorized re-write of a balance onto a handle the
/// owner and `viewers` may decrypt, signed on the host by the token-account PDA (the value's
/// authority).
fn allow_balance_viewers_ix(
    owner: Pubkey,
    mint: Pubkey,
    token_account: Pubkey,
    balance_store: Pubkey,
    host_config: Pubkey,
    viewers: Vec<Pubkey>,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::AllowBalanceViewers {
            transient_store: host::transient_store_address(owner).0,
            instructions: solana_sdk::sysvar::instructions::ID,
            payer: owner,
            owner,
            mint,
            token_account,
            balance_store,
            host_config,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            system_program: system_program::ID,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::AllowBalanceViewers { viewers },
    )
}

fn allow_total_supply_viewers_ix(
    fixture: &BurnRedeemFixture,
    authority: Pubkey,
    viewers: Vec<Pubkey>,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::AllowTotalSupplyViewers {
            transient_store: host::transient_store_address(authority).0,
            instructions: solana_sdk::sysvar::instructions::ID,
            payer: authority,
            authority,
            mint: fixture.mint,
            total_supply_authority: fixture.total_supply_authority,
            total_supply_store: fixture.total_supply_store,
            host_config: fixture.host_config,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            system_program: system_program::ID,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::AllowTotalSupplyViewers { viewers },
    )
}

fn make_token_account_handle_public_ix(
    fixture: &BurnRedeemFixture,
    kind: token::DisclosedValueKind,
    encrypted_store: Pubkey,
    handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::MakeTokenAccountHandlePublic {
            payer: fixture.owner,
            owner: fixture.owner,
            mint: fixture.mint,
            token_account: fixture.token_account,
            encrypted_store,
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
            total_supply_store: fixture.total_supply_store,
            host_config: fixture.host_config,
            zama_program: host::id(),
            system_program: system_program::ID,
        },
        token::instruction::MakeTotalSupplyHandlePublic { handle },
    )
}

// ---------------------------------------------------------------------------
// Viewer grants (allow-on-write) and public seals
// ---------------------------------------------------------------------------

#[test]
fn mollusk_mint_authority_allows_total_supply_viewers() {
    let fixture = BurnRedeemFixture::new();
    let auditor = Pubkey::new_unique();
    let context = burn_redeem_mollusk().with_context(fixture.accounts(0));
    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.initial_total_supply, 5_000);

    let result = check_token_instruction(
        &context,
        &allow_total_supply_viewers_ix(&fixture, fixture.owner, vec![auditor]),
        &[Check::success()],
    );
    cleartext.evaluate_fhe_cpi(&context, &result);

    // The grant is a re-write: same cleartext, a fresh handle, one allow leaf for the auditor
    // (nobody is allowed on the supply by default, so the auditor is the only leaf).
    assert_eq!(
        cleartext.u64_in_state(
            &context,
            fixture.total_supply_store,
            token::total_supply_key()
        ),
        5_000
    );
    let value = read_encrypted_store(&context, fixture.total_supply_store);
    assert_ne!(
        store_handle(&value, token::total_supply_key()),
        fixture.initial_total_supply
    );
    assert_eq!(value.leaf_count, 1);
    assert_eq!(
        value.peaks,
        expected_allow_peaks(
            fixture.total_supply_store,
            store_handle(&value, token::total_supply_key()),
            &[auditor]
        )
    );
    let events: Vec<token::TotalSupplyHandleUpdatedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].old_handle, fixture.initial_total_supply);
    assert_eq!(
        events[0].new_handle,
        store_handle(&value, token::total_supply_key())
    );
    assert_eq!(
        events[0].reason,
        token::TotalSupplyUpdateReason::AllowViewers
    );
}

#[test]
fn mollusk_owner_allows_balance_viewers() {
    let fixture = TokenFixture::new();
    let auditor = Pubkey::new_unique();
    let context = mollusk().with_context(fixture.base_accounts());
    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.alice_initial, 1_000);

    let result = check_token_instruction(
        &context,
        &allow_balance_viewers_ix(
            fixture.owner,
            fixture.mint,
            fixture.alice_token,
            fixture.alice_balance_store,
            fixture.host_config,
            vec![auditor],
        ),
        &[Check::success()],
    );
    cleartext.evaluate_fhe_cpi(&context, &result);

    // The owner stays allowed on every balance write; the auditor is allowed on this handle only.
    assert_eq!(cleartext.balance(&context, fixture.alice_token), 1_000);
    let value = read_encrypted_store(&context, fixture.alice_balance_store);
    assert_ne!(
        store_handle(&value, token::balance_key()),
        fixture.alice_initial
    );
    assert_eq!(value.leaf_count, 2);
    assert_eq!(
        value.peaks,
        expected_allow_peaks(
            fixture.alice_balance_store,
            store_handle(&value, token::balance_key()),
            &[fixture.owner, auditor]
        )
    );
    let events: Vec<token::BalanceHandleUpdatedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(events.len(), 1);
    assert_eq!(
        events[0].reason,
        token::BalanceHandleUpdateReason::AllowViewers
    );
    assert_eq!(events[0].old_handle, fixture.alice_initial);
    assert_eq!(
        events[0].new_handle,
        store_handle(&value, token::balance_key())
    );

    // The next balance write allows the owner alone again: the grant covered one handle.
    let transfer = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        amount_attestation_for(
            handle_for_chain(21, BALANCE_FHE_TYPE),
            fixture.owner,
            token::id(),
        ),
    );
    let transfer_result = check_token_instruction(&context, &transfer, &[Check::success()]);
    let transferred_handle: [u8; 32] = transferred_event_handle(&transfer_result);
    let after = read_encrypted_store(&context, fixture.alice_balance_store);
    assert_eq!(after.leaf_count, 5);
    let mut leaves = allow_leaves(
        fixture.alice_balance_store,
        0,
        store_handle(&value, token::balance_key()),
        &[fixture.owner, auditor],
    );
    leaves.extend(allow_leaves(
        fixture.alice_balance_store,
        2,
        transferred_handle,
        &[fixture.owner, fixture.bob_owner],
    ));
    leaves.extend(allow_leaves(
        fixture.alice_balance_store,
        4,
        store_handle(&after, token::balance_key()),
        &[fixture.owner],
    ));
    assert_eq!(after.peaks, zama_solana_acl::mmr_peaks_from_leaves(&leaves));
}

#[test]
fn mollusk_non_owner_cannot_allow_balance_viewers() {
    let fixture = TokenFixture::new();
    let stranger = Pubkey::new_unique();
    let mut accounts = fixture.base_accounts();
    accounts.insert(stranger, system_account(1_000_000_000));
    let context = mollusk().with_context(accounts);

    check_token_instruction(
        &context,
        &allow_balance_viewers_ix(
            stranger,
            fixture.mint,
            fixture.alice_token,
            fixture.alice_balance_store,
            fixture.host_config,
            vec![Pubkey::new_unique()],
        ),
        &[anchor_error(anchor_lang::error::ErrorCode::ConstraintSeeds)],
    );
}

#[test]
fn mollusk_non_mint_authority_cannot_allow_total_supply_viewers() {
    let fixture = BurnRedeemFixture::new();
    let stranger = Pubkey::new_unique();
    let mut accounts = fixture.accounts(0);
    accounts.insert(stranger, system_account(1_000_000_000));
    let context = mollusk().with_context(accounts);

    check_token_instruction(
        &context,
        &allow_total_supply_viewers_ix(&fixture, stranger, vec![Pubkey::new_unique()]),
        &[token_error(
            token::ConfidentialTokenError::MintAuthorityMismatch,
        )],
    );
    check_token_instruction(
        &context,
        &make_total_supply_handle_public_ix(&fixture, stranger, fixture.initial_total_supply),
        &[token_error(
            token::ConfidentialTokenError::MintAuthorityMismatch,
        )],
    );
}

#[test]
fn mollusk_total_supply_allow_rejects_wrong_value_shape() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(0);
    let (_, wrong_value) = new_test_state(
        fixture.app(),
        fixture.total_supply_authority,
        token::balance_key(),
        fixture.initial_total_supply,
    );
    accounts.insert(
        fixture.total_supply_store,
        encrypted_store_account(&wrong_value),
    );
    let context = mollusk().with_context(accounts);

    check_token_instruction(
        &context,
        &allow_total_supply_viewers_ix(&fixture, fixture.owner, vec![Pubkey::new_unique()]),
        &[token_error(
            token::ConfidentialTokenError::TokenEncryptedStoreMismatch,
        )],
    );
}

#[test]
fn mollusk_owner_seals_exact_token_account_state_field() {
    let fixture = BurnRedeemFixture::new();
    let accounts = fixture.accounts(0);
    let context = burn_redeem_mollusk().with_context(accounts);

    check_token_instruction(
        &context,
        &make_token_account_handle_public_ix(
            &fixture,
            token::DisclosedValueKind::Balance,
            fixture.balance_store,
            fixture.initial_balance,
        ),
        &[Check::success()],
    );
    let value = read_encrypted_store(&context, fixture.balance_store);
    assert_eq!(value.leaf_count, 1);
    assert_eq!(
        value.peaks,
        zama_solana_acl::mmr_peaks_from_leaves(&[zama_solana_acl::public_decrypt_leaf_commitment(
            fixture.balance_store.to_bytes(),
            0,
            fixture.initial_balance,
        ),])
    );

    check_token_instruction(
        &context,
        &make_token_account_handle_public_ix(
            &fixture,
            token::DisclosedValueKind::BurnedAmount,
            fixture.balance_store,
            fixture.initial_balance,
        ),
        &[token_error(
            token::ConfidentialTokenError::DisclosedValueBindingMismatch,
        )],
    );
}

#[test]
fn mollusk_mint_authority_seals_total_supply() {
    let fixture = BurnRedeemFixture::new();
    let context = burn_redeem_mollusk().with_context(fixture.accounts(0));

    check_token_instruction(
        &context,
        &make_total_supply_handle_public_ix(&fixture, fixture.owner, fixture.initial_total_supply),
        &[Check::success()],
    );
    assert_eq!(
        read_encrypted_store(&context, fixture.total_supply_store).leaf_count,
        1
    );
}

/// Base accounts with the deny list enabled and the mint's deny record seeded (`denied` or not),
/// returning the record so a transfer can carry it as its witness.
fn deny_enabled_transfer_accounts(
    fixture: &TokenFixture,
    denied: bool,
) -> (HashMap<Pubkey, Account>, Pubkey) {
    let mut accounts = fixture.base_accounts();
    accounts.insert(
        fixture.host_config,
        deny_enabled_host_config_account(
            fixture.owner,
            secp_evm_address(&coprocessor_signing_key()),
        ),
    );
    let (record, account) = deny_scope_record_account(fixture.app(), denied);
    accounts.insert(record, account);
    (accounts, record)
}

// ---------------------------------------------------------------------------
// initialize_mint / initialize_token_account
// ---------------------------------------------------------------------------

#[test]
fn mollusk_initialize_mint_creates_total_supply_encrypted_store() {
    let authority = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let underlying_mint = Pubkey::new_unique();
    let total_supply_authority = token::total_supply_authority_address(mint).0;
    let total_supply_encrypted_store = token::total_supply_slot(mint).0.address();
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
        (total_supply_authority, system_account(0)),
        (total_supply_encrypted_store, system_account(0)),
        (host_config_key, host_config_account(authority, [0u8; 20])),
        (event_authority(host::id()), system_account(0)),
        (event_authority(token::id()), system_account(0)),
    ]));
    let ix = initialize_mint_ix(authority, mint, underlying_mint, host_config_key);

    check_token_instruction(&context, &ix, &[Check::success()]);

    let stored = read_confidential_mint(&context, mint);
    assert_eq!(stored.authority, authority);
    // The supply belongs to the token application in the mint's scope, under the total-supply
    // PDA; nobody is allowed on it by default, so the create seals no leaf.
    let supply_value = read_encrypted_store(&context, total_supply_encrypted_store);
    assert_eq!(supply_value.program, token::id());
    assert_eq!(supply_value.scope, mint.to_bytes());
    assert_eq!(supply_value.authority, total_supply_authority);
    assert!(supply_value.get(&token::total_supply_key()).is_some());
    assert_eq!(supply_value.leaf_count, 0);
}

#[test]
fn mollusk_initialize_token_account_creates_initial_balance_encrypted_store() {
    let fixture = TokenFixture::new();
    let owner = Pubkey::new_unique();
    let (token_account, token_bump) = token::token_account_address(fixture.mint, owner);
    let balance_encrypted_store = token::balance_slot(fixture.mint, token_account).0.address();
    let mut accounts = fixture.base_accounts();
    accounts.insert(owner, system_account(5_000_000_000));
    accounts.insert(token_account, system_account(0));
    accounts.insert(balance_encrypted_store, system_account(0));
    let context = mollusk().with_context(accounts);
    let ix = initialize_token_account_ix(owner, owner, fixture.mint, fixture.host_config);

    let result = check_token_instruction(&context, &ix, &[Check::success()]);

    let stored = read_token_account(&context, token_account);
    assert_eq!(stored.owner, owner);
    assert_eq!(stored.mint, fixture.mint);
    assert_eq!(stored.bump, token_bump);

    // The balance is the token application's value under the token-account PDA, allowed to the
    // owner on its first handle.
    let balance_store = read_encrypted_store(&context, balance_encrypted_store);
    assert_eq!(balance_store.program, token::id());
    assert_eq!(balance_store.scope, fixture.mint.to_bytes());
    assert_eq!(balance_store.authority, token_account);
    assert!(balance_store.get(&token::balance_key()).is_some());
    assert_eq!(balance_store.leaf_count, 1);
    assert_eq!(
        balance_store.peaks,
        expected_allow_peaks(
            balance_encrypted_store,
            store_handle(&balance_store, token::balance_key()),
            &[owner]
        )
    );

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
    assert_eq!(balance_events[0].old_encrypted_store, Pubkey::default());
    assert_eq!(
        balance_events[0].new_handle,
        store_handle(&balance_store, token::balance_key())
    );
    assert_eq!(
        balance_events[0].new_encrypted_store,
        balance_encrypted_store
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
    let balance_encrypted_store = token::balance_slot(fixture.mint, token_account).0.address();
    let mut accounts = fixture.base_accounts();
    accounts.insert(payer, system_account(5_000_000_000));
    accounts.insert(owner, system_account(0));
    accounts.insert(token_account, system_account(0));
    accounts.insert(balance_encrypted_store, system_account(0));
    let context = mollusk().with_context(accounts);
    let ix = initialize_token_account_ix(payer, owner, fixture.mint, fixture.host_config);

    check_token_instruction(&context, &ix, &[Check::success()]);

    let stored = read_token_account(&context, token_account);
    assert_eq!(stored.owner, owner);
    // The sponsor pays; only the owner is allowed on the balance.
    let balance_store = read_encrypted_store(&context, balance_encrypted_store);
    assert_eq!(balance_store.leaf_count, 1);
    assert_eq!(
        balance_store.peaks,
        expected_allow_peaks(
            balance_encrypted_store,
            store_handle(&balance_store, token::balance_key()),
            &[owner]
        )
    );

    let retry = check_token_instruction(&context, &ix, &[]);
    assert!(retry.raw_result.is_err());
    let stored_after_retry = read_token_account(&context, token_account);
    let balance_after_retry = read_encrypted_store(&context, balance_encrypted_store);
    assert_eq!(stored_after_retry.owner, stored.owner);
    assert_eq!(
        store_handle(&balance_after_retry, token::balance_key()),
        store_handle(&balance_store, token::balance_key())
    );
    assert_eq!(balance_after_retry.peaks, balance_store.peaks);
}

// ---------------------------------------------------------------------------
// confidential_transfer
// ---------------------------------------------------------------------------

/// An attestation over `amount_seed`'s handle, authored by the fixture's sender and bound to this
/// program (the `fromExternal` contract binding).
fn sender_attestation(
    fixture: &TokenFixture,
    amount_seed: u8,
) -> host::CoprocessorInputAttestation {
    amount_attestation_for(
        handle_for_chain(amount_seed, BALANCE_FHE_TYPE),
        fixture.owner,
        token::id(),
    )
}

#[test]
fn mollusk_confidential_transfer_self_transfer_is_no_op() {
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let transfer = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.alice_token,
        fixture.alice_balance_store,
        fixture.alice_balance_store,
        sender_attestation(&fixture, 9),
    );

    let result = check_token_instruction(&context, &transfer, &[]);

    assert!(result.raw_result.is_ok());
    assert!(result.inner_instructions.is_empty());
    assert!(result.return_data.is_empty());
    let balance_store = read_encrypted_store(&context, fixture.alice_balance_store);
    assert_eq!(
        store_handle(&balance_store, token::balance_key()),
        fixture.alice_initial
    );
    assert_eq!(balance_store.leaf_count, 0);
}

#[test]
fn mollusk_confidential_transfer_self_transfer_rejects_result_grant() {
    let fixture = TokenFixture::new();
    let consumer_scope = [0x55; 32];
    let (result_store, state) = new_encrypted_store(
        host::AppScope {
            program: token::id(),
            scope: consumer_scope,
        },
        fixture.owner,
        [],
    );
    let mut accounts = fixture.base_accounts();
    accounts.insert(result_store, encrypted_store_account(&state));
    let context = mollusk().with_context(accounts);

    check_token_instruction(
        &context,
        &confidential_self_transfer_with_result_grant_ix(&fixture, result_store),
        &[token_error(
            token::ConfidentialTokenError::ResultGrantMismatch,
        )],
    );
    assert_eq!(
        read_store_handle(&context, fixture.alice_balance_store, token::balance_key(),),
        fixture.alice_initial
    );
}

#[test]
fn mollusk_confidential_transfer_rejects_frozen_sender_ata() {
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    accounts.insert(
        fixture.owner_ata(fixture.owner),
        frozen_spl_token_account(fixture.underlying_mint, fixture.owner),
    );
    let context = mollusk().with_context(accounts);
    let transfer = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        sender_attestation(&fixture, 9),
    );

    check_token_instruction(
        &context,
        &transfer,
        &[token_error(
            token::ConfidentialTokenError::UnderlyingTokenAccountFrozen,
        )],
    );
}

#[test]
fn mollusk_confidential_transfer_rejects_frozen_recipient_ata() {
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    accounts.insert(
        fixture.owner_ata(fixture.bob_owner),
        frozen_spl_token_account(fixture.underlying_mint, fixture.bob_owner),
    );
    let context = mollusk().with_context(accounts);
    let transfer = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        sender_attestation(&fixture, 9),
    );

    check_token_instruction(
        &context,
        &transfer,
        &[token_error(
            token::ConfidentialTokenError::UnderlyingTokenAccountFrozen,
        )],
    );
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
    let transfer = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        sender_attestation(&fixture, 21),
    );

    let result = check_token_instruction(&context, &transfer, &[Check::success()]);
    let persistent_outputs = cleartext.evaluate_fhe_cpi(&context, &result);

    assert_eq!(persistent_outputs, 2);
    assert_eq!(cleartext.balance(&context, fixture.alice_token), 600);
    assert_eq!(cleartext.balance(&context, fixture.bob_token), 500);
    let transferred_handle: [u8; 32] = transferred_event_handle(&result);
    assert_eq!(cleartext.u64_for_handle(transferred_handle), 400);

    // Token account addresses and their balance `EncryptedStore` PDAs stay stable across the
    // transfer: no new balance account is created, the existing values are replaced in place.
    let _alice_token = read_token_account(&context, fixture.alice_token);
    let _bob_token = read_token_account(&context, fixture.bob_token);

    // The sender State records the transient result grants followed by its balance write. The
    // recipient State records its balance write.
    let alice_balance = read_encrypted_store(&context, fixture.alice_balance_store);
    let bob_balance = read_encrypted_store(&context, fixture.bob_balance_store);
    assert_ne!(
        store_handle(&alice_balance, token::balance_key()),
        fixture.alice_initial
    );
    assert_ne!(
        store_handle(&bob_balance, token::balance_key()),
        fixture.bob_initial
    );
    assert_eq!(alice_balance.leaf_count, 3);
    assert_eq!(bob_balance.leaf_count, 1);
    assert_eq!(alice_balance.peaks, {
        let mut leaves = allow_leaves(
            fixture.alice_balance_store,
            0,
            transferred_handle,
            &[fixture.owner, fixture.bob_owner],
        );
        leaves.extend(allow_leaves(
            fixture.alice_balance_store,
            2,
            store_handle(&alice_balance, token::balance_key()),
            &[fixture.owner],
        ));
        zama_solana_acl::mmr_peaks_from_leaves(&leaves)
    });
    assert_eq!(
        bob_balance.peaks,
        expected_allow_peaks(
            fixture.bob_balance_store,
            store_handle(&bob_balance, token::balance_key()),
            &[fixture.bob_owner],
        )
    );

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
    assert_eq!(transfer_events[0].transferred_handle, transferred_handle);
    assert_eq!(
        transfer_events[0].transferred_encrypted_store,
        fixture.alice_balance_store
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
    assert_eq!(
        balance_events[0].new_handle,
        store_handle(&alice_balance, token::balance_key())
    );
    assert_eq!(
        balance_events[1].reason,
        token::BalanceHandleUpdateReason::TransferCredit
    );
    assert_eq!(balance_events[1].old_handle, fixture.bob_initial);
    assert_eq!(
        balance_events[1].new_handle,
        store_handle(&bob_balance, token::balance_key())
    );
}

/// Seeds Charlie's token account + balance encrypted store into `accounts` and returns
/// (charlie_owner, charlie_token, charlie_balance_store).
fn seed_third_account(
    fixture: &TokenFixture,
    accounts: &mut HashMap<Pubkey, Account>,
    initial: [u8; 32],
) -> (Pubkey, Pubkey, Pubkey) {
    let charlie_owner = Pubkey::new_unique();
    let charlie_token = token::token_account_address(fixture.mint, charlie_owner).0;
    let charlie_balance_store = token::balance_slot(fixture.mint, charlie_token).0.address();
    accounts.insert(charlie_owner, system_account(5_000_000_000));
    accounts.insert(
        charlie_token,
        fixture.confidential_token_account(charlie_owner, charlie_balance_store),
    );
    fixture.register_token_owner(charlie_token, charlie_owner);
    accounts.insert(fixture.owner_ata(charlie_owner), system_account(0));
    let (_, charlie_value) =
        new_test_state(fixture.app(), charlie_token, token::balance_key(), initial);
    accounts.insert(
        charlie_balance_store,
        encrypted_store_account(&charlie_value),
    );
    (charlie_owner, charlie_token, charlie_balance_store)
}

#[test]
fn mollusk_confidential_transfer_to_successive_recipients_seals_each_receipt_audience() {
    // Alice -> Bob, Alice -> Charlie, Alice -> Bob: the per-sender transferred value is rewritten
    // each time, allowing the sender and the current recipient on the new handle; every earlier
    // receipt keeps its own leaves, so each recipient can still prove what they received.
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    let (charlie_owner, charlie_token, charlie_balance_store) = seed_third_account(
        &fixture,
        &mut accounts,
        handle_for_chain(3, BALANCE_FHE_TYPE),
    );
    let context = mollusk().with_context(accounts);
    let receipt_address = fixture.transferred_amount_store_address(fixture.alice_token);

    let transfer = |to_token, to_balance, tag| {
        confidential_transfer_ix(
            &fixture,
            fixture.alice_token,
            to_token,
            fixture.alice_balance_store,
            to_balance,
            sender_attestation(&fixture, tag),
        )
    };

    let bob_result = check_token_instruction(
        &context,
        &transfer(fixture.bob_token, fixture.bob_balance_store, 21),
        &[Check::success()],
    );
    let after_bob = read_encrypted_store(&context, receipt_address);
    let bob_handle: [u8; 32] = transferred_event_handle(&bob_result);
    let bob_balance = store_handle(&after_bob, token::balance_key());
    assert_eq!(after_bob.leaf_count, 3);

    let charlie_result = check_token_instruction(
        &context,
        &transfer(charlie_token, charlie_balance_store, 22),
        &[Check::success()],
    );
    let after_charlie = read_encrypted_store(&context, receipt_address);
    let charlie_handle: [u8; 32] = transferred_event_handle(&charlie_result);
    let charlie_balance = store_handle(&after_charlie, token::balance_key());
    assert_eq!(after_charlie.leaf_count, 6);

    let bob_again_result = check_token_instruction(
        &context,
        &transfer(fixture.bob_token, fixture.bob_balance_store, 23),
        &[Check::success()],
    );
    let after_bob_again = read_encrypted_store(&context, receipt_address);
    let bob_again_handle: [u8; 32] = transferred_event_handle(&bob_again_result);
    let bob_again_balance = store_handle(&after_bob_again, token::balance_key());
    assert_eq!(after_bob_again.leaf_count, 9);

    let mut leaves = Vec::new();
    for (result_handle, balance_handle, audience) in [
        (bob_handle, bob_balance, [fixture.owner, fixture.bob_owner]),
        (
            charlie_handle,
            charlie_balance,
            [fixture.owner, charlie_owner],
        ),
        (
            bob_again_handle,
            bob_again_balance,
            [fixture.owner, fixture.bob_owner],
        ),
    ] {
        leaves.extend(allow_leaves(
            receipt_address,
            leaves.len() as u64,
            result_handle,
            &audience,
        ));
        leaves.extend(allow_leaves(
            receipt_address,
            leaves.len() as u64,
            balance_handle,
            &[fixture.owner],
        ));
    }
    assert_eq!(
        after_bob_again.peaks,
        zama_solana_acl::mmr_peaks_from_leaves(&leaves)
    );
}

#[test]
fn mollusk_confidential_transfer_self_transfer_preserves_history_and_returns_no_handle() {
    // A no-op must preserve history and clear the previous transfer's returned handle.
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let receipt_address = fixture.transferred_amount_store_address(fixture.alice_token);

    check_token_instruction(
        &context,
        &confidential_transfer_ix(
            &fixture,
            fixture.alice_token,
            fixture.bob_token,
            fixture.alice_balance_store,
            fixture.bob_balance_store,
            sender_attestation(&fixture, 21),
        ),
        &[Check::success()],
    );
    let receipt_before = read_encrypted_store(&context, receipt_address);
    assert_eq!(receipt_before.leaf_count, 3);

    let result = check_token_instruction(
        &context,
        &confidential_transfer_ix(
            &fixture,
            fixture.alice_token,
            fixture.alice_token,
            fixture.alice_balance_store,
            fixture.alice_balance_store,
            sender_attestation(&fixture, 22),
        ),
        &[],
    );
    assert!(result.raw_result.is_ok());
    assert!(result.return_data.is_empty());
    let receipt_after = read_encrypted_store(&context, receipt_address);
    assert_eq!(receipt_after.leaf_count, receipt_before.leaf_count);
    assert_eq!(receipt_after.peaks, receipt_before.peaks);
}

// ---------------------------------------------------------------------------
// Deny list: one record per application (this program in the mint's scope)
// ---------------------------------------------------------------------------

#[test]
fn mollusk_confidential_transfer_with_deny_list_succeeds_when_mint_is_not_denied() {
    let fixture = TokenFixture::new();
    let (accounts, deny_record) = deny_enabled_transfer_accounts(&fixture, false);
    let context = mollusk().with_context(accounts);
    let mut accounts_for_charlie = HashMap::new();
    let (_, charlie_token, charlie_balance_store) = seed_third_account(
        &fixture,
        &mut accounts_for_charlie,
        handle_for_chain(3, BALANCE_FHE_TYPE),
    );
    context
        .account_store
        .borrow_mut()
        .extend(accounts_for_charlie);

    // Every execution of the mint carries the same witness, whoever the recipient is.
    for (to_token, to_balance, tag) in [
        (fixture.bob_token, fixture.bob_balance_store, 21),
        (charlie_token, charlie_balance_store, 22),
    ] {
        check_token_instruction(
            &context,
            &confidential_transfer_ix_with_remaining(
                &fixture,
                fixture.alice_token,
                to_token,
                fixture.alice_balance_store,
                to_balance,
                sender_attestation(&fixture, tag),
                vec![deny_record],
            ),
            &[Check::success()],
        );
    }

    let alice_balance = read_encrypted_store(&context, fixture.alice_balance_store);
    assert_ne!(
        store_handle(&alice_balance, token::balance_key()),
        fixture.alice_initial
    );
    assert_eq!(
        read_encrypted_store(
            &context,
            fixture.transferred_amount_store_address(fixture.alice_token)
        )
        .leaf_count,
        6
    );
}

#[test]
fn mollusk_confidential_transfer_with_deny_list_rejects_denied_mint_atomically() {
    let fixture = TokenFixture::new();
    let (accounts, deny_record) = deny_enabled_transfer_accounts(&fixture, true);
    let context = mollusk().with_context(accounts);
    let ix = confidential_transfer_ix_with_remaining(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        sender_attestation(&fixture, 24),
        vec![deny_record],
    );

    check_token_instruction(
        &context,
        &ix,
        &[host_error(host::errors::ZamaHostError::ScopeDenied)],
    );

    let alice_balance = read_encrypted_store(&context, fixture.alice_balance_store);
    let bob_balance = read_encrypted_store(&context, fixture.bob_balance_store);
    assert_eq!(
        store_handle(&alice_balance, token::balance_key()),
        fixture.alice_initial
    );
    assert_eq!(
        store_handle(&bob_balance, token::balance_key()),
        fixture.bob_initial
    );
}

#[test]
fn mollusk_confidential_transfer_with_deny_list_requires_the_mints_witness() {
    // Enabled list, no witness: the token fails closed before the host is reached. Another
    // application's record is no witness either.
    let fixture = TokenFixture::new();
    let (mut accounts, _) = deny_enabled_transfer_accounts(&fixture, false);
    let (foreign_record, foreign_account) =
        deny_scope_record_account(token::token_app(Pubkey::new_unique()), false);
    accounts.insert(foreign_record, foreign_account);
    let context = mollusk().with_context(accounts);

    for remaining in [Vec::new(), vec![foreign_record]] {
        check_token_instruction(
            &context,
            &confidential_transfer_ix_with_remaining(
                &fixture,
                fixture.alice_token,
                fixture.bob_token,
                fixture.alice_balance_store,
                fixture.bob_balance_store,
                sender_attestation(&fixture, 25),
                remaining,
            ),
            &[token_error(
                token::ConfidentialTokenError::UnexpectedRemainingAccounts,
            )],
        );
    }
    assert_eq!(
        read_store_handle(&context, fixture.alice_balance_store, token::balance_key()),
        fixture.alice_initial
    );
}

#[test]
fn mollusk_confidential_transfer_rejects_witness_while_deny_list_is_disabled() {
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    let (record, account) = deny_scope_record_account(fixture.app(), false);
    accounts.insert(record, account);
    let context = mollusk().with_context(accounts);

    check_token_instruction(
        &context,
        &confidential_transfer_ix_with_remaining(
            &fixture,
            fixture.alice_token,
            fixture.bob_token,
            fixture.alice_balance_store,
            fixture.bob_balance_store,
            sender_attestation(&fixture, 26),
            vec![record],
        ),
        &[token_error(
            token::ConfidentialTokenError::UnexpectedRemainingAccounts,
        )],
    );
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
    let attestation =
        amount_attestation_signed_by(amount_handle, fixture.owner, token::id(), signing_keys);
    let transfer = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        attestation,
    );
    check_token_instruction(&context, &transfer, checks)
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
        amount_attestation_signed_by(amount_handle, fixture.owner, token::id(), &keys);
    assert_eq!(attestation.signatures.len(), 4);

    let transfer = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
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
    let attestation = amount_attestation_for(amount_handle, fixture.bob_owner, token::id());
    let mut ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        attestation,
    );
    // Sign as bob (matches the attestation) so only the owner-mismatch check can fail.
    for meta in ix.accounts.iter_mut() {
        if meta.pubkey == fixture.owner {
            meta.pubkey = fixture.bob_owner;
        }
    }

    check_token_instruction(
        &context,
        &ix,
        &[token_error(token::ConfidentialTokenError::OwnerMismatch)],
    );

    let alice_balance = read_encrypted_store(&context, fixture.alice_balance_store);
    assert_eq!(
        store_handle(&alice_balance, token::balance_key()),
        fixture.alice_initial
    );
}

#[test]
fn mollusk_confidential_transfer_rejects_attestation_user_mismatch() {
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_handle = handle_for_chain(31, BALANCE_FHE_TYPE);
    // fromExternal binding: an attestation authored by someone other than the transfer authority
    // (owner) must be rejected before any balance update.
    let attestation = amount_attestation_for(amount_handle, fixture.bob_owner, token::id());
    let ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        attestation,
    );

    check_token_instruction(
        &context,
        &ix,
        &[token_error(
            token::ConfidentialTokenError::AttestationUserMismatch,
        )],
    );

    let alice_balance = read_encrypted_store(&context, fixture.alice_balance_store);
    let bob_balance = read_encrypted_store(&context, fixture.bob_balance_store);
    assert_eq!(
        store_handle(&alice_balance, token::balance_key()),
        fixture.alice_initial
    );
    assert_eq!(
        store_handle(&bob_balance, token::balance_key()),
        fixture.bob_initial
    );
}

#[test]
fn mollusk_confidential_transfer_rejects_attestation_contract_mismatch() {
    let fixture = TokenFixture::new();
    let context = mollusk().with_context(fixture.base_accounts());
    let amount_handle = handle_for_chain(32, BALANCE_FHE_TYPE);
    // fromExternal binding: an attestation bound to a contract other than this program must be
    // rejected before any balance update.
    let attestation = amount_attestation_for(amount_handle, fixture.owner, Pubkey::new_unique());
    let ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        attestation,
    );

    check_token_instruction(
        &context,
        &ix,
        &[token_error(
            token::ConfidentialTokenError::AttestationContractMismatch,
        )],
    );

    let alice_balance = read_encrypted_store(&context, fixture.alice_balance_store);
    let bob_balance = read_encrypted_store(&context, fixture.bob_balance_store);
    assert_eq!(
        store_handle(&alice_balance, token::balance_key()),
        fixture.alice_initial
    );
    assert_eq!(
        store_handle(&bob_balance, token::balance_key()),
        fixture.bob_initial
    );
}

fn assert_transfer_rejects_misbound_balance(
    rebind: impl FnOnce(&mut host::EncryptedStore),
    amount_seed: u8,
    expected: Check<'static>,
) {
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    let (_, mut misbound) = new_test_state(
        fixture.app(),
        fixture.alice_token,
        token::balance_key(),
        fixture.alice_initial,
    );
    rebind(&mut misbound);
    accounts.insert(
        fixture.alice_balance_store,
        encrypted_store_account(&misbound),
    );
    let context = mollusk().with_context(accounts);
    let ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        sender_attestation(&fixture, amount_seed),
    );
    check_token_instruction(&context, &ix, &[expected]);
    assert_eq!(
        read_store_handle(&context, fixture.alice_balance_store, token::balance_key(),),
        fixture.alice_initial,
    );
    assert_eq!(
        read_store_handle(&context, fixture.bob_balance_store, token::balance_key(),),
        fixture.bob_initial,
    );
}

#[test]
fn mollusk_confidential_transfer_rejects_balance_in_another_mints_scope() {
    let wrong_mint = Pubkey::new_unique();
    assert_transfer_rejects_misbound_balance(
        |value| value.scope = wrong_mint.to_bytes(),
        34,
        token_error(token::ConfidentialTokenError::CurrentEncryptedStoreMismatch),
    );
}

/// A balance stored under another token account names an authority this transfer does not sign
/// for; account resolution reports the missing authority before any host CPI.
#[test]
fn mollusk_confidential_transfer_rejects_balance_under_another_token_account() {
    let wrong_token_account = Pubkey::new_unique();
    assert_transfer_rejects_misbound_balance(
        |value| value.authority = wrong_token_account,
        35,
        token_error(token::ConfidentialTokenError::CurrentEncryptedStoreMismatch),
    );
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
// after a later burn updates the shared `burned_amount` encrypted store. A burn writes two
// leaves: the owner's allow on the burned handle, then its public-decrypt leaf.
// ---------------------------------------------------------------------------

use anchor_spl::token::spl_token;
use anchor_spl::token_2022::spl_token_2022;
use solana_sdk::program_option::COption;

use zama_solana_test_kit::signing::{kms_signing_key, kms_signing_key_n};

/// A cert committing an explicit KMS context id via v1 `extra_data` (EVM `_extractContextId`
/// parity), for the rotation-grace tests: a cert minted under an old-but-still-live context.
fn kms_public_decrypt_cert_for_context(
    handle: [u8; 32],
    cleartext_amount: u64,
    context_id: [u8; 32],
) -> (Vec<[u8; 65]>, Vec<u8>) {
    let extra_data = zama_solana_test_kit::signing::context_extra_data_v1(context_id);
    let signatures = zama_solana_test_kit::signing::kms_public_decrypt_cert_signed_by(
        handle,
        u256_be(cleartext_amount),
        GATEWAY_CHAIN_ID,
        &DECRYPTION_CONTRACT,
        &extra_data,
        &[kms_signing_key()],
    );
    (signatures, extra_data)
}

fn kms_context_account(context_id: [u8; 32]) -> Account {
    kms_context_account_with_signers(context_id, &[secp_evm_address(&kms_signing_key())], 1)
}

/// Like [`kms_context_account`] but marked `destroyed`, for the revocation-lever test.
fn destroyed_kms_context_account(context_id: [u8; 32]) -> Account {
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
    context_id: [u8; 32],
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

fn frozen_spl_token_account(mint: Pubkey, owner: Pubkey) -> Account {
    let mut data = vec![0u8; spl_token::state::Account::LEN];
    spl_token::state::Account::pack(
        spl_token::state::Account {
            mint,
            owner,
            amount: 0,
            delegate: COption::None,
            state: spl_token::state::AccountState::Frozen,
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

/// Self-contained fixture for the burn/redeem/cancel vertical: one owner, one
/// confidential mint with an SPL-backed vault, and one funded token account.
struct BurnRedeemFixture {
    owner: Pubkey,
    mint: Pubkey,
    host_config: Pubkey,
    token_account: Pubkey,
    balance_store: Pubkey,
    total_supply_authority: Pubkey,
    total_supply_store: Pubkey,
    burned_amount_store: Pubkey,
    underlying_mint: Pubkey,
    token_program: Pubkey,
    vault_authority: Pubkey,
    vault_usdc: Pubkey,
    destination_usdc: Pubkey,
    kms_context_id: [u8; 32],
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
        let host_config = host::host_config_address().0;
        let token_account = token::token_account_address(mint, owner).0;
        let balance_store = token::balance_slot(mint, token_account).0.address();
        let total_supply_authority = token::total_supply_authority_address(mint).0;
        let total_supply_store = token::total_supply_slot(mint).0.address();
        let burned_amount_store = token::encrypted_store_address(mint, token_account).0;
        let vault_authority = token::vault_authority_address(mint).0;
        let vault_usdc = token::vault_token_account_address(mint, underlying_mint, token_program);
        let destination_usdc = Pubkey::new_unique();
        let kms_context_id = canonical_test_context_id(9);
        let kms_context = host::kms_context_address(kms_context_id).0;
        Self {
            owner,
            mint,
            host_config,
            token_account,
            balance_store,
            total_supply_authority,
            total_supply_store,
            burned_amount_store,
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

    fn app(&self) -> host::AppScope {
        token::token_app(self.mint)
    }

    fn confidential_mint_account(&self) -> Account {
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(token::ConfidentialMint {
                authority: self.owner,
                underlying_mint: self.underlying_mint,
                decimals: 6,
            }),
            owner: token::id(),
            executable: false,
            rent_epoch: 0,
        }
    }

    fn accounts(&self, vault_balance: u64) -> HashMap<Pubkey, Account> {
        let (_, balance_store) = new_test_state(
            self.app(),
            self.token_account,
            token::balance_key(),
            self.initial_balance,
        );
        let (_, total_supply_store) = new_test_state(
            self.app(),
            self.total_supply_authority,
            token::total_supply_key(),
            self.initial_total_supply,
        );
        HashMap::from([
            (self.owner, system_account(50_000_000_000)),
            (self.mint, self.confidential_mint_account()),
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
                token_account_account(self.mint, self.owner, self.balance_store),
            ),
            (self.balance_store, encrypted_store_account(&balance_store)),
            (
                self.total_supply_store,
                encrypted_store_account(&total_supply_store),
            ),
            (
                self.underlying_mint,
                if self.token_program == spl_token::id() {
                    spl_mint_account(None, 1_000_000)
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
            (self.owner_ata(), system_account(0)),
            mollusk_svm_programs_token::token::keyed_account(),
            mollusk_svm_programs_token::token2022::keyed_account(),
        ])
    }

    fn owner_ata(&self) -> Pubkey {
        get_associated_token_address_with_program_id(
            &self.owner,
            &self.underlying_mint,
            &self.token_program,
        )
    }

    /// An attestation over `amount_seed`'s handle authored by the owner for this program.
    fn owner_attestation(&self, amount_seed: u8) -> host::CoprocessorInputAttestation {
        amount_attestation_for(
            handle_for_chain(amount_seed, BALANCE_FHE_TYPE),
            self.owner,
            token::id(),
        )
    }
}

fn token_account_account(mint: Pubkey, owner: Pubkey, _balance_store: Pubkey) -> Account {
    Account {
        lamports: 1_000_000_000,
        data: serialized_account(token::ConfidentialTokenAccount {
            owner,
            mint,
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
            transient_store: host::transient_store_address(fixture.owner).0,
            instructions: solana_sdk::sysvar::instructions::ID,
            owner: fixture.owner,
            mint: fixture.mint,
            underlying_mint: fixture.underlying_mint,
            owner_ata: fixture.owner_ata(),
            token_account: fixture.token_account,
            total_supply_authority: fixture.total_supply_authority,
            balance_store: fixture.balance_store,
            total_supply_store: fixture.total_supply_store,
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
/// on-chain `EncryptedStore` at `amount_store` (a computed or received handle) rather than a fresh
/// attestation. `owner` signs as the burn authority (it must own `token_account`, and the amount
/// value must be under its own or the token account's authority) and `payer` pays rent; splitting
/// them lets `owner` be a program PDA.
fn confidential_burn_from_value_ix(
    fixture: &BurnRedeemFixture,
    owner: Pubkey,
    payer: Pubkey,
    amount_store: Pubkey,
    amount_key: [u8; 32],
    pending_burn: Pubkey,
) -> Instruction {
    anchor_ix(
        token::id(),
        token::accounts::ConfidentialBurnFromValue {
            transient_store: host::transient_store_address(payer).0,
            instructions: solana_sdk::sysvar::instructions::ID,
            owner,
            payer,
            mint: fixture.mint,
            underlying_mint: fixture.underlying_mint,
            owner_ata: fixture.owner_ata(),
            token_account: fixture.token_account,
            total_supply_authority: fixture.total_supply_authority,
            balance_store: fixture.balance_store,
            total_supply_store: fixture.total_supply_store,
            pending_burn,
            amount_store,
            zama_event_authority: event_authority(host::id()),
            zama_program: host::id(),
            host_config: fixture.host_config,
            system_program: system_program::ID,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::ConfidentialBurnFromValue { key: amount_key },
    )
}

/// A from-value burn against a freshly emptied canonical pending-burn account.
fn confidential_burn_from_value_auto(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    fixture: &BurnRedeemFixture,
    owner: Pubkey,
    payer: Pubkey,
    amount_store: Pubkey,
    amount_key: [u8; 32],
) -> Instruction {
    let pending_burn = prepare_empty_pending_burn(context, fixture);
    confidential_burn_from_value_ix(
        fixture,
        owner,
        payer,
        amount_store,
        amount_key,
        pending_burn,
    )
}

/// Seeds a spendable amount encrypted store (a stand-in for a computed/received `euint64`
/// handle) of the fixture's application under `authority` at `label`, returning its address.
fn seed_burn_amount_store(
    fixture: &BurnRedeemFixture,
    accounts: &mut HashMap<Pubkey, Account>,
    authority: Pubkey,
    encrypted_store_label: [u8; 32],
    handle: [u8; 32],
) -> Pubkey {
    insert_store_slot(
        accounts,
        fixture.app(),
        authority,
        encrypted_store_label,
        handle,
    )
}

/// The two leaves one burn appends to the burned-amount value from `first_index`: the owner's
/// allow on the burned handle, then its public-decrypt leaf.
fn burn_leaves(
    fixture: &BurnRedeemFixture,
    first_index: u64,
    burned_handle: [u8; 32],
) -> Vec<[u8; 32]> {
    let acct = fixture.burned_amount_store.to_bytes();
    vec![
        zama_solana_acl::historical_access_leaf_commitment(
            acct,
            first_index,
            burned_handle,
            fixture.owner.to_bytes(),
        ),
        zama_solana_acl::public_decrypt_leaf_commitment(acct, first_index + 1, burned_handle),
    ]
}

fn burn_update_leaves(
    fixture: &BurnRedeemFixture,
    first_index: u64,
    balance_handle: [u8; 32],
    burned_handle: [u8; 32],
) -> Vec<[u8; 32]> {
    let mut leaves = burn_leaves(fixture, first_index, burned_handle);
    leaves.extend(allow_leaves(
        fixture.balance_store,
        first_index + 2,
        balance_handle,
        &[fixture.owner],
    ));
    leaves
}

/// Builds the public-decrypt inclusion proof for `fixture.burned_amount_store` after one burn: the
/// public leaf sits at index 1, behind the owner's allow leaf.
fn single_burn_public_decrypt_proof(
    fixture: &BurnRedeemFixture,
    burned_handle: [u8; 32],
) -> host::instructions::MmrInclusionProof {
    let leaves = burn_leaves(fixture, 0, burned_handle);
    let proof =
        zama_solana_acl::mmr_build_proof(&leaves, 1).expect("proof for the public burn leaf");
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
    let (_, mut value) = new_test_state(
        fixture.app(),
        fixture.token_account,
        token::burned_amount_key(),
        burned_handle,
    );
    value.leaf_count = 2;
    value.peaks = zama_solana_acl::mmr_peaks_from_leaves(&burn_leaves(fixture, 0, burned_handle));
    accounts.insert(fixture.burned_amount_store, encrypted_store_account(&value));
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
            burned_amount_store: fixture.burned_amount_store,
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

fn cancel_pending_burn_ix(
    fixture: &BurnRedeemFixture,
    pending_burn: Pubkey,
    deny_record: Option<Pubkey>,
) -> Instruction {
    let mut ix = anchor_ix(
        token::id(),
        token::accounts::CancelPendingBurn {
            transient_store: host::transient_store_address(fixture.owner).0,
            instructions: solana_sdk::sysvar::instructions::ID,
            owner: fixture.owner,
            mint: fixture.mint,
            token_account: fixture.token_account,
            total_supply_authority: fixture.total_supply_authority,
            balance_store: fixture.balance_store,
            total_supply_store: fixture.total_supply_store,
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
    );
    ix.accounts
        .extend(deny_record.map(|record| AccountMeta::new_readonly(record, false)));
    ix
}

/// Burns `amount_seed`'s attested amount and returns the resulting burned handle.
fn run_burn(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    fixture: &BurnRedeemFixture,
    amount_seed: u8,
) -> [u8; 32] {
    let pending_burn = prepare_empty_pending_burn(context, fixture);
    let ix = confidential_burn_ix(
        fixture,
        fixture.owner_attestation(amount_seed),
        pending_burn,
    );
    check_token_instruction(context, &ix, &[Check::success()]);
    read_store_handle(
        context,
        fixture.burned_amount_store,
        token::burned_amount_key(),
    )
}

#[test]
fn mollusk_confidential_burn_rejects_frozen_owner_ata() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(1_000);
    accounts.insert(
        fixture.owner_ata(),
        frozen_spl_token_account(fixture.underlying_mint, fixture.owner),
    );
    let context = burn_redeem_mollusk().with_context(accounts);
    let pending_burn = prepare_empty_pending_burn(&context, &fixture);
    check_token_instruction(
        &context,
        &confidential_burn_ix(&fixture, fixture.owner_attestation(41), pending_burn),
        &[token_error(
            token::ConfidentialTokenError::UnderlyingTokenAccountFrozen,
        )],
    );
}

#[test]
fn mollusk_confidential_burn_makes_burned_amount_publicly_decryptable() {
    let fixture = BurnRedeemFixture::new();
    let context = burn_redeem_mollusk().with_context(fixture.accounts(1_000));

    let burned_handle = run_burn(&context, &fixture, 41);

    // The first burn creates the burned-amount value and appends the owner's allow on the burned
    // handle followed by its public-decrypt leaf.
    let value = read_encrypted_store(&context, fixture.burned_amount_store);
    assert_eq!(
        store_handle(&value, token::burned_amount_key()),
        burned_handle
    );
    assert_eq!(value.leaf_count, 3);
    assert_eq!(
        value.peaks,
        zama_solana_acl::mmr_peaks_from_leaves(&burn_update_leaves(
            &fixture,
            0,
            store_handle(&value, token::balance_key()),
            burned_handle,
        ))
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

        check_token_instruction(
            &context,
            &confidential_burn_ix(&fixture, fixture.owner_attestation(41), pending_burn),
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
        let old_balance = read_store_handle(&context, fixture.balance_store, token::balance_key());
        let old_supply = read_store_handle(
            &context,
            fixture.total_supply_store,
            token::total_supply_key(),
        );

        check_token_instruction(
            &context,
            &confidential_burn_ix(&fixture, fixture.owner_attestation(41), pending_burn),
            &[token_error(
                token::ConfidentialTokenError::PendingBurnAlreadyInitialized,
            )],
        );

        assert_eq!(
            read_store_handle(&context, fixture.balance_store, token::balance_key()),
            old_balance
        );
        assert_eq!(
            read_store_handle(
                &context,
                fixture.total_supply_store,
                token::total_supply_key()
            ),
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

/// Reconstructs the burned-amount value's four leaves after two burns (allow(owner,H1)@0,
/// public(H1)@1, allow(owner,H2)@2, public(H2)@3) and builds a public-decrypt inclusion proof for
/// the leaf at `leaf_index`, returning it with the value's peaks. Leaf 1 proves the historical
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
    let (signatures, extra_data) = amount_public_decrypt_cert(burned_handle, cleartext_amount);
    let pending_burn = seed_pending_burn_in_context(&context, &fixture, burned_handle);
    check_token_instruction(
        &context,
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
    assert!(check_token_instruction(&context, &dup, &[])
        .raw_result
        .is_err());
    assert_eq!(
        read_spl_amount(&context, fixture.destination_usdc),
        cleartext_amount
    );

    // The alternate settlement path is excluded by the same closed account.
    assert!(check_token_instruction(
        &context,
        &cancel_pending_burn_ix(&fixture, pending_burn, None),
        &[]
    )
    .raw_result
    .is_err());
}

#[test]
fn mollusk_redeem_current_burn_with_token_2022() {
    let fixture = BurnRedeemFixture::new_token_2022();
    let burned_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let proof = single_burn_public_decrypt_proof(&fixture, burned_handle);
    let mut accounts = fixture.accounts(1_000);
    seed_single_burn_value_account(&fixture, &mut accounts, burned_handle);
    let context = burn_redeem_mollusk().with_context(accounts);
    let pending_burn = seed_pending_burn_in_context(&context, &fixture, burned_handle);
    let (signatures, extra_data) = amount_public_decrypt_cert(burned_handle, 500);

    check_token_instruction(
        &context,
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
    let (signatures, extra_data) = amount_public_decrypt_cert(burned_handle, 500);

    check_token_instruction(
        &context,
        &redeem_burned_amount_ix(
            &fixture,
            burned_handle,
            500,
            signatures,
            extra_data,
            single_burn_public_decrypt_proof(&fixture, burned_handle),
            pending_burn,
        ),
        &[token_error(
            token::ConfidentialTokenError::UnderlyingTokenAccountFrozen,
        )],
    );
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 1_000);
}

/// An out-of-range leaf index for the two-leaf accumulator fails closed with the host's
/// `PublicDecryptProofInvalid` (surfaced through the CPI) and leaves the vault untouched.
#[test]
fn mollusk_redeem_rejects_out_of_range_public_decrypt_leaf() {
    let fixture = BurnRedeemFixture::new();
    let first_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let mut proof = single_burn_public_decrypt_proof(&fixture, first_handle);

    let mut accounts = fixture.accounts(1_000);
    seed_single_burn_value_account(&fixture, &mut accounts, first_handle);
    let context = burn_redeem_mollusk().with_context(accounts);

    proof.leaf_index = 2; // Outside the two-leaf accumulator.

    let cleartext_amount = 500;
    let (signatures, extra_data) = amount_public_decrypt_cert(first_handle, cleartext_amount);
    let pending_burn = seed_pending_burn_in_context(&context, &fixture, first_handle);
    check_token_instruction(
        &context,
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
            canonical_test_context_id(10),
        ),
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) =
        kms_public_decrypt_cert_for_context(first_handle, cleartext_amount, fixture.kms_context_id);
    let pending_burn = seed_pending_burn_in_context(&context, &fixture, first_handle);
    check_token_instruction(
        &context,
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
    check_token_instruction(
        &context,
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

/// The destination token account may be owned by a third party: owner signature is the
/// theft-prevention, not `destination_usdc.owner == owner`.
#[test]
fn mollusk_redeem_pays_destination_not_owned_by_signer() {
    let fixture = BurnRedeemFixture::new();
    let first_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let proof = single_burn_public_decrypt_proof(&fixture, first_handle);

    let mut accounts = fixture.accounts(1_000);
    seed_single_burn_value_account(&fixture, &mut accounts, first_handle);
    let stranger = Pubkey::new_unique();
    accounts.insert(
        fixture.destination_usdc,
        spl_token_account(fixture.underlying_mint, stranger, 0),
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) = amount_public_decrypt_cert(first_handle, cleartext_amount);
    let pending_burn = seed_pending_burn_in_context(&context, &fixture, first_handle);
    check_token_instruction(
        &context,
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

/// The deny list controls FHE executions, not token settlement. A mint whose application is denied
/// can still redeem an already-authorized pending burn, so the deny list cannot trap funds.
#[test]
fn mollusk_redeem_is_not_blocked_by_grant_deny_list() {
    let fixture = BurnRedeemFixture::new();
    let first_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let proof = single_burn_public_decrypt_proof(&fixture, first_handle);

    let mut accounts = fixture.accounts(1_000);
    seed_single_burn_value_account(&fixture, &mut accounts, first_handle);
    // Enable the deny list on the host config (keeping the fixture's current KMS context) and
    // mark the mint's application denied.
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
    let (deny_record, denied_account) = deny_scope_record_account(fixture.app(), true);
    accounts.insert(deny_record, denied_account);
    let context = burn_redeem_mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) = amount_public_decrypt_cert(first_handle, cleartext_amount);
    let pending_burn = seed_pending_burn_in_context(&context, &fixture, first_handle);
    check_token_instruction(
        &context,
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

    // Execute a real first burn, then redeem its public leaf.
    let first_handle = run_burn(&context, &fixture, 41);
    let first_state = read_encrypted_store(&context, fixture.burned_amount_store);
    let first_balance = store_handle(&first_state, token::balance_key());
    let proof_h1 = single_burn_public_decrypt_proof(&fixture, first_handle);
    let (sig_h1, extra_h1) = amount_public_decrypt_cert(first_handle, 300);
    check_token_instruction(
        &context,
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
    // write appends H2's allow and public leaves behind H1's.
    let second_handle = run_burn(&context, &fixture, 42);
    let second_state = read_encrypted_store(&context, fixture.burned_amount_store);
    let mut leaves = burn_update_leaves(&fixture, 0, first_balance, first_handle);
    leaves.extend(burn_update_leaves(
        &fixture,
        3,
        store_handle(&second_state, token::balance_key()),
        second_handle,
    ));
    let proof = zama_solana_acl::mmr_build_proof(&leaves, 4).expect("second public burn leaf");
    let proof_h2 = host::instructions::MmrInclusionProof {
        leaf_index: proof.leaf_index,
        siblings: proof.siblings,
    };
    assert_eq!(
        second_state.peaks,
        zama_solana_acl::mmr_peaks_from_leaves(&leaves)
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
    check_token_instruction(
        &context,
        &stale,
        &[token_error(
            token::ConfidentialTokenError::PendingBurnMismatch,
        )],
    );
    assert_eq!(read_spl_amount(&context, fixture.destination_usdc), 300);

    // Redeem H2 from leaf 4, reusing the canonical pending account after H1 closed.
    let (sig_h2, extra_h2) = amount_public_decrypt_cert(second_handle, 200);
    check_token_instruction(
        &context,
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
    assert!(check_token_instruction(&context, &dup, &[])
        .raw_result
        .is_err());
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

    let (signatures, extra_data) = amount_public_decrypt_cert(first_handle, 500);
    let pending_burn = seed_pending_burn_in_context(&context, &fixture, first_handle);
    check_token_instruction(
        &context,
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

#[test]
fn mollusk_redeem_rejects_foreign_burn_state_and_preserves_accounts() {
    let fixture = BurnRedeemFixture::new();
    let handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let mut accounts = fixture.accounts(1_000);
    seed_single_burn_value_account(&fixture, &mut accounts, handle);
    let (foreign_address, foreign_state) = new_test_state(
        fixture.app(),
        Pubkey::new_unique(),
        token::burned_amount_key(),
        handle,
    );
    accounts.insert(foreign_address, encrypted_store_account(&foreign_state));
    let context = burn_redeem_mollusk().with_context(accounts);
    let pending = seed_pending_burn_in_context(&context, &fixture, handle);
    let (signatures, extra_data) = amount_public_decrypt_cert(handle, 500);
    let mut ix = redeem_burned_amount_ix(
        &fixture,
        handle,
        500,
        signatures,
        extra_data,
        single_burn_public_decrypt_proof(&fixture, handle),
        pending,
    );
    ix.accounts
        .iter_mut()
        .find(|meta| meta.pubkey == fixture.burned_amount_store)
        .unwrap()
        .pubkey = foreign_address;
    let before = context.account_store.borrow().clone();
    check_token_instruction(
        &context,
        &ix,
        &[token_error(
            token::ConfidentialTokenError::AmountAclMismatch,
        )],
    );
    assert_eq!(*context.account_store.borrow(), before);
}

#[test]
fn mollusk_redeem_rejects_mismatched_pending_identity_without_payout() {
    for field in ["owner", "mint", "token_account"] {
        let fixture = BurnRedeemFixture::new();
        let handle = handle_for_chain(41, BALANCE_FHE_TYPE);
        let mut accounts = fixture.accounts(1_000);
        seed_single_burn_value_account(&fixture, &mut accounts, handle);
        let context = burn_redeem_mollusk().with_context(accounts);
        let pending = seed_pending_burn_in_context(&context, &fixture, handle);
        let data = context.account_store.borrow()[&pending].data.clone();
        let mut record = token::PendingBurn::try_deserialize(&mut data.as_slice()).unwrap();
        match field {
            "owner" => record.owner = Pubkey::new_unique(),
            "mint" => record.mint = Pubkey::new_unique(),
            _ => record.token_account = Pubkey::new_unique(),
        }
        context
            .account_store
            .borrow_mut()
            .insert(pending, pending_burn_account(record));
        let (signatures, extra_data) = amount_public_decrypt_cert(handle, 500);
        let ix = redeem_burned_amount_ix(
            &fixture,
            handle,
            500,
            signatures,
            extra_data,
            single_burn_public_decrypt_proof(&fixture, handle),
            pending,
        );
        let before = context.account_store.borrow().clone();
        check_token_instruction(
            &context,
            &ix,
            &[token_error(
                token::ConfidentialTokenError::PendingBurnMismatch,
            )],
        );
        assert_eq!(*context.account_store.borrow(), before, "{field}");
    }
}

#[test]
fn mollusk_cancel_pending_burn_restores_balance_and_supply() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(1_000);
    let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let amount_store = seed_burn_amount_store(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::transfer_input_key(),
        amount_handle,
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
        amount_store,
        token::transfer_input_key(),
    );
    let burn_result = check_token_instruction(&context, &burn, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &burn_result);
    assert_eq!(cleartext.balance(&context, fixture.token_account), 750);
    assert_eq!(
        cleartext.u64_in_state(
            &context,
            fixture.total_supply_store,
            token::total_supply_key()
        ),
        4_750
    );

    let pending_burn = token::pending_burn_address(fixture.mint, fixture.token_account).0;
    assert!(context
        .account_store
        .borrow()
        .get(&pending_burn)
        .is_some_and(|account| account.owner == token::id() && !account.data.is_empty()));

    // Under the host's deny policy the cancellation carries the mint's witness like every other
    // execution of the mint; a mint that is not denied restores its burn.
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
    let (deny_record, deny_account) = deny_scope_record_account(fixture.app(), false);
    context
        .account_store
        .borrow_mut()
        .insert(deny_record, deny_account);

    let cancel = cancel_pending_burn_ix(&fixture, pending_burn, Some(deny_record));
    let cancel_result = check_token_instruction(&context, &cancel, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &cancel_result);

    assert_eq!(cleartext.balance(&context, fixture.token_account), 1_000);
    assert_eq!(
        cleartext.u64_in_state(
            &context,
            fixture.total_supply_store,
            token::total_supply_key()
        ),
        5_000
    );
    assert_pending_burn_closed(&context, pending_burn);

    // Closing is act-once: neither a second cancel nor the alternate redeem path can settle the
    // same burn after the pending account is gone.
    assert!(check_token_instruction(&context, &cancel, &[])
        .raw_result
        .is_err());
    let burned_handle = read_store_handle(
        &context,
        fixture.burned_amount_store,
        token::burned_amount_key(),
    );
    let (signatures, extra_data) = amount_public_decrypt_cert(burned_handle, 250);
    assert!(check_token_instruction(
        &context,
        &redeem_burned_amount_ix(
            &fixture,
            burned_handle,
            250,
            signatures,
            extra_data,
            single_burn_public_decrypt_proof(&fixture, burned_handle),
            pending_burn,
        ),
        &[]
    )
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
        .get(&fixture.balance_store)
        .expect("balance exists")
        .data
        .clone();
    let old_supply = context
        .account_store
        .borrow()
        .get(&fixture.total_supply_store)
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
    let mut cancel = cancel_pending_burn_ix(&fixture, pending_burn, None);
    let owner_meta = cancel
        .accounts
        .iter_mut()
        .find(|meta| meta.pubkey == fixture.owner && meta.is_signer)
        .expect("owner signer meta");
    owner_meta.pubkey = stranger;

    check_token_instruction(
        &context,
        &cancel,
        &[token_error(token::ConfidentialTokenError::OwnerMismatch)],
    );
    assert_eq!(
        context
            .account_store
            .borrow()
            .get(&fixture.balance_store)
            .expect("balance remains")
            .data,
        old_balance
    );
    assert_eq!(
        context
            .account_store
            .borrow()
            .get(&fixture.total_supply_store)
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
    let old_supply_data = context
        .account_store
        .borrow()
        .get(&fixture.total_supply_store)
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

    let mut burned = read_encrypted_store(&context, fixture.burned_amount_store);
    burned
        .set(
            token::burned_amount_key(),
            Some(store_handle(&burned, token::burned_amount_key())),
            handle_for_chain(42, BALANCE_FHE_TYPE),
        )
        .unwrap();
    context
        .account_store
        .borrow_mut()
        .get_mut(&fixture.burned_amount_store)
        .expect("burned amount exists")
        .data = serialized_account(burned);
    let mutated_balance_data = context
        .account_store
        .borrow()
        .get(&fixture.balance_store)
        .expect("balance remains")
        .data
        .clone();

    check_token_instruction(
        &context,
        &cancel_pending_burn_ix(&fixture, pending_burn, None),
        &[token_error(
            token::ConfidentialTokenError::PendingBurnHandleNotCurrent,
        )],
    );

    assert_eq!(
        context
            .account_store
            .borrow()
            .get(&fixture.balance_store)
            .expect("balance remains")
            .data,
        mutated_balance_data
    );
    assert_eq!(
        context
            .account_store
            .borrow()
            .get(&fixture.total_supply_store)
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
    let amount_store = seed_burn_amount_store(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::transfer_input_key(),
        amount_handle,
    );
    let context = burn_redeem_mollusk().with_context(accounts);
    let pending_burn = prepare_empty_pending_burn(&context, &fixture);

    let burn = confidential_burn_from_value_ix(
        &fixture,
        fixture.owner,
        fixture.owner,
        amount_store,
        token::transfer_input_key(),
        pending_burn,
    );
    check_token_instruction(&context, &burn, &[Check::success()]);

    // The same canonical pending account is occupied, so another burn is rejected before FHE.
    check_token_instruction(
        &context,
        &burn,
        &[token_error(
            token::ConfidentialTokenError::PendingBurnAlreadyInitialized,
        )],
    );

    check_token_instruction(
        &context,
        &cancel_pending_burn_ix(&fixture, pending_burn, None),
        &[Check::success()],
    );
    check_token_instruction(&context, &burn, &[Check::success()]);
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
                transient_store: host::transient_store_address(self.owner).0,
                instructions: solana_sdk::sysvar::instructions::ID,
                owner: self.owner,
                mint: fixture.mint,
                token_account: fixture.token_account,
                underlying_mint: self.underlying_mint,
                user_usdc: self.user_usdc,
                vault_usdc: self.vault_usdc,
                vault_authority: fixture.vault_authority,
                total_supply_authority: self.total_supply_authority,
                balance_store: fixture.balance_store,
                total_supply_store: fixture.total_supply_store,
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
    let result = check_token_instruction(&context, &ix, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &result);

    assert_eq!(cleartext.balance(&context, fixture.token_account), 100);
    assert_eq!(
        cleartext.u64_in_state(
            &context,
            fixture.total_supply_store,
            token::total_supply_key()
        ),
        100
    );
    assert_eq!(read_spl_amount(&context, user_usdc), 900);
    assert_eq!(read_spl_amount(&context, fixture.vault_usdc), 100);
    // The balance write allows the owner; the supply write allows nobody.
    let balance = read_encrypted_store(&context, fixture.balance_store);
    assert_eq!(balance.leaf_count, 1);
    assert_eq!(
        balance.peaks,
        expected_allow_peaks(
            fixture.balance_store,
            store_handle(&balance, token::balance_key()),
            &[fixture.owner]
        )
    );
    assert_eq!(
        read_encrypted_store(&context, fixture.total_supply_store).leaf_count,
        0
    );
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

    let result = check_token_instruction(
        &context,
        &wrap_usdc_ix(&fixture, user_tokens, 100),
        &[Check::success()],
    );
    cleartext.evaluate_fhe_cpi(&context, &result);
    assert_eq!(cleartext.balance(&context, fixture.token_account), 100);
    assert_eq!(
        cleartext.u64_in_state(
            &context,
            fixture.total_supply_store,
            token::total_supply_key()
        ),
        100
    );
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

    check_token_instruction(
        &context,
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

    check_token_instruction(
        &context,
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

    check_token_instruction(
        &context,
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

    check_token_instruction(
        &context,
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
    check_token_instruction(
        &context,
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
    accounts.insert(other_mint, spl_mint_account(None, 1_000_000));
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
    check_token_instruction(
        &context,
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
    check_token_instruction(
        &context,
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
    check_token_instruction(
        &context,
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
// layer adds: the mint-scope binding, the disclosed event, the pinned-handle
// pass-through, and the intentional absence of a replay marker (idempotence).
// ---------------------------------------------------------------------------

/// Self-contained fixture for the disclose consume vertical: one owner, one confidential mint, a
/// balance encrypted store, and one token-scoped amount encrypted store. The
/// fixture's v0 certs resolve to the host's current KMS context, so the fixture's
/// `current_kms_context_id` and seeded `kms_context` share `kms_context_id`.
struct DiscloseFixture {
    owner: Pubkey,
    mint: Pubkey,
    host_config: Pubkey,
    token_account: Pubkey,
    balance_store: Pubkey,
    amount_store: Pubkey,
    kms_context_id: [u8; 32],
    kms_context: Pubkey,
}

impl DiscloseFixture {
    fn new() -> Self {
        let owner = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let host_config = host::host_config_address().0;
        let token_account = token::token_account_address(mint, owner).0;
        let balance_store = token::balance_slot(mint, token_account).0.address();
        // Any token-scoped amount encrypted store discloses the same way; use the
        // burned_amount slot.
        let amount_store = token::encrypted_store_address(mint, token_account).0;
        let kms_context_id = canonical_test_context_id(9);
        let kms_context = host::kms_context_address(kms_context_id).0;
        Self {
            owner,
            mint,
            host_config,
            token_account,
            balance_store,
            amount_store,
            kms_context_id,
            kms_context,
        }
    }

    fn confidential_mint_account(&self) -> Account {
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(token::ConfidentialMint {
                authority: self.owner,
                underlying_mint: Pubkey::new_unique(),
                decimals: 6,
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
                token_account_account(self.mint, self.owner, self.balance_store),
            ),
            (event_authority(token::id()), system_account(0)),
        ])
    }
}

/// Builds an encrypted store of `mint`'s application whose write of `pinned` allowed
/// `allows` and then sealed it public (leaves allow(pinned, a)@0.. then public(pinned)), and the
/// inclusion proof for that public leaf. With `update_to = Some(h2)` the account is grown into a
/// post-update state (the same allow + public layout for h2 appended, current handle h2),
/// modeling the pinned handle becoming historical after it was sealed public.
fn public_leaf_value_account(
    expected_address: Pubkey,
    authority: Pubkey,
    mint: Pubkey,
    encrypted_store_label: [u8; 32],
    allows: &[Pubkey],
    pinned: [u8; 32],
    update_to: Option<[u8; 32]>,
) -> (host::EncryptedStore, host::instructions::MmrInclusionProof) {
    let acct = expected_address.to_bytes();
    let mut leaves = allow_leaves(expected_address, 0, pinned, allows);
    let public_index = leaves.len() as u64;
    leaves.push(zama_solana_acl::public_decrypt_leaf_commitment(
        acct,
        public_index,
        pinned,
    ));
    let current = match update_to {
        Some(h2) => {
            let first = leaves.len() as u64;
            leaves.extend(allow_leaves(expected_address, first, h2, allows));
            leaves.push(zama_solana_acl::public_decrypt_leaf_commitment(
                acct,
                leaves.len() as u64,
                h2,
            ));
            h2
        }
        None => pinned,
    };
    let (address, mut value) = new_test_state(
        token::token_app(mint),
        authority,
        encrypted_store_label,
        current,
    );
    assert_eq!(
        address, expected_address,
        "encrypted store address mismatch"
    );
    value.leaf_count = leaves.len() as u64;
    value.peaks = zama_solana_acl::mmr_peaks_from_leaves(&leaves);
    let proof = zama_solana_acl::mmr_build_proof(&leaves, public_index)
        .expect("proof for the pinned public leaf");
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
    encrypted_store: Pubkey,
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
            encrypted_store,
            host_config: fixture.host_config,
            kms_context: fixture.kms_context,
            zama_program: host::id(),
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::DiscloseSecp {
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
    encrypted_store: Pubkey,
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
            encrypted_store,
            host_config: fixture.host_config,
            kms_context: fixture.kms_context,
            zama_program: host::id(),
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::DiscloseSecp {
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
    encrypted_store: Pubkey,
    cleartext_amount: u64,
    authority: Pubkey,
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
    assert_eq!(events[0].encrypted_store, expected.encrypted_store);
    assert_eq!(events[0].cleartext_amount, expected.cleartext_amount);
    assert_eq!(events[0].authority, expected.authority);
}

#[test]
fn mollusk_disclose_secp_amount_happy_path() {
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(43, BALANCE_FHE_TYPE);
    let (value, proof) = public_leaf_value_account(
        fixture.amount_store,
        fixture.token_account,
        fixture.mint,
        token::burned_amount_key(),
        &[fixture.owner],
        pinned,
        None,
    );
    assert_eq!(store_handle(&value, token::burned_amount_key()), pinned);

    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_store, encrypted_store_account(&value));
    let context = mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) = amount_public_decrypt_cert(pinned, cleartext_amount);
    let result = check_token_instruction(
        &context,
        &disclose_secp_ix(
            &fixture,
            fixture.amount_store,
            pinned,
            u256_be(cleartext_amount),
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
            encrypted_store: fixture.amount_store,
            cleartext_amount,
            authority: fixture.token_account,
        },
    );
}

#[test]
fn mollusk_disclose_secp_balance_happy_path() {
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(33, BALANCE_FHE_TYPE);
    let (value, proof) = public_leaf_value_account(
        fixture.balance_store,
        fixture.token_account,
        fixture.mint,
        token::balance_key(),
        &[fixture.owner],
        pinned,
        None,
    );

    let mut accounts = fixture.base();
    accounts.insert(fixture.balance_store, encrypted_store_account(&value));
    let context = mollusk().with_context(accounts);

    let cleartext_amount = 700;
    let (signatures, extra_data) = amount_public_decrypt_cert(pinned, cleartext_amount);
    let result = check_token_instruction(
        &context,
        &disclose_secp_ix(
            &fixture,
            fixture.balance_store,
            pinned,
            u256_be(cleartext_amount),
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
            encrypted_store: fixture.balance_store,
            cleartext_amount,
            authority: fixture.token_account,
        },
    );
}

#[test]
fn mollusk_disclose_secp_total_supply_requires_no_token_account() {
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(35, BALANCE_FHE_TYPE);
    let authority = token::total_supply_authority_address(fixture.mint).0;
    let total_supply_store = token::total_supply_slot(fixture.mint).0.address();
    // The supply's writes allow nobody; sealing it public is its only leaf.
    let (value, proof) = public_leaf_value_account(
        total_supply_store,
        authority,
        fixture.mint,
        token::total_supply_key(),
        &[],
        pinned,
        None,
    );
    let mut accounts = fixture.base();
    accounts.insert(total_supply_store, encrypted_store_account(&value));
    let context = mollusk().with_context(accounts);
    let cleartext_amount = 10_000;
    let (signatures, extra_data) = amount_public_decrypt_cert(pinned, cleartext_amount);

    let result = check_token_instruction(
        &context,
        &disclose_total_supply_ix(
            &fixture,
            total_supply_store,
            pinned,
            u256_be(cleartext_amount),
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
            encrypted_store: total_supply_store,
            cleartext_amount,
            authority,
        },
    );

    check_token_instruction(
        &context,
        &disclose_secp_ix(
            &fixture,
            total_supply_store,
            pinned,
            u256_be(cleartext_amount),
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
    // encrypted store is replaced to H2 (e.g. an inbound transfer) before the consume
    // lands. The pinned handle must still disclose, authorized by its permanent public-decrypt
    // leaf, not the live handle. This is the host verifier's survives-update property observed one
    // layer up.
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(41, BALANCE_FHE_TYPE);
    let replaced = handle_for_chain(42, BALANCE_FHE_TYPE);
    let (value, proof) = public_leaf_value_account(
        fixture.amount_store,
        fixture.token_account,
        fixture.mint,
        token::burned_amount_key(),
        &[fixture.owner],
        pinned,
        Some(replaced),
    );
    assert_ne!(store_handle(&value, token::burned_amount_key()), pinned);

    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_store, encrypted_store_account(&value));
    let context = mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) = amount_public_decrypt_cert(pinned, cleartext_amount);
    let result = check_token_instruction(
        &context,
        &disclose_secp_ix(
            &fixture,
            fixture.amount_store,
            pinned,
            u256_be(cleartext_amount),
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
            encrypted_store: fixture.amount_store,
            cleartext_amount,
            authority: fixture.token_account,
        },
    );
}

#[test]
fn mollusk_disclose_refreshes_a_proof_after_history_grows() {
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(41, BALANCE_FHE_TYPE);
    let replaced = handle_for_chain(42, BALANCE_FHE_TYPE);
    let (_, stale_proof) = public_leaf_value_account(
        fixture.amount_store,
        fixture.token_account,
        fixture.mint,
        token::burned_amount_key(),
        &[fixture.owner],
        pinned,
        None,
    );
    // Another transaction appends enough leaves to merge the proof's mountain after fetch.
    let (updated, fresh_proof) = public_leaf_value_account(
        fixture.amount_store,
        fixture.token_account,
        fixture.mint,
        token::burned_amount_key(),
        &[fixture.owner],
        pinned,
        Some(replaced),
    );
    assert_ne!(stale_proof.siblings, fresh_proof.siblings);
    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_store, encrypted_store_account(&updated));
    let context = mollusk().with_context(accounts);
    let before = context.account_store.borrow().clone();
    let (signatures, extra_data) = amount_public_decrypt_cert(pinned, 500);
    let instruction = |proof| {
        disclose_secp_ix(
            &fixture,
            fixture.amount_store,
            pinned,
            u256_be(500),
            signatures.clone(),
            extra_data.clone(),
            proof,
        )
    };
    check_token_instruction(
        &context,
        &instruction(stale_proof),
        &[host_error(
            host::errors::ZamaHostError::PublicDecryptProofInvalid,
        )],
    );
    assert_eq!(*context.account_store.borrow(), before);
    // Refresh only the inclusion proof: the certificate still authenticates the same handle.
    let result = check_token_instruction(&context, &instruction(fresh_proof), &[Check::success()]);
    assert_disclosed(
        &result,
        ExpectedDisclosure {
            mint: fixture.mint,
            handle: pinned,
            encrypted_store: fixture.amount_store,
            cleartext_amount: 500,
            authority: fixture.token_account,
        },
    );
}

#[test]
fn mollusk_disclose_secp_is_idempotent_no_replay_marker() {
    // Act-once is intentionally NOT enforced on-chain: disclosure is idempotent information
    // release, so re-running the same cert succeeds again and re-emits the same event. No replay
    // marker PDA exists by design (contrast redeem_burned_amount). Apps that need consume-once
    // track it in their own state.
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(44, BALANCE_FHE_TYPE);
    let (value, proof) = public_leaf_value_account(
        fixture.amount_store,
        fixture.token_account,
        fixture.mint,
        token::burned_amount_key(),
        &[fixture.owner],
        pinned,
        None,
    );

    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_store, encrypted_store_account(&value));
    let context = mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) = amount_public_decrypt_cert(pinned, cleartext_amount);
    let ix = disclose_secp_ix(
        &fixture,
        fixture.amount_store,
        pinned,
        u256_be(cleartext_amount),
        signatures,
        extra_data,
        proof,
    );
    check_token_instruction(&context, &ix, &[Check::success()]);
    // Same cert, same accounts, run again: still succeeds (idempotent, no consume-once).
    check_token_instruction(&context, &ix, &[Check::success()]);
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
        fixture.amount_store,
        fixture.token_account,
        fixture.mint,
        token::burned_amount_key(),
        &[fixture.owner],
        pinned,
        Some(replaced),
    );
    // Leaves: allow(H1)@0, public(H1)@1, allow(H2)@2, public(H2)@3.
    proof.leaf_index = 3; // H2's public-decrypt leaf, not H1's.

    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_store, encrypted_store_account(&value));
    let context = mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) = amount_public_decrypt_cert(pinned, cleartext_amount);
    check_token_instruction(
        &context,
        &disclose_secp_ix(
            &fixture,
            fixture.amount_store,
            pinned,
            u256_be(cleartext_amount),
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
fn mollusk_disclose_secp_rejects_foreign_mint_scope() {
    // The disclosed encrypted store must belong to this mint's application: the token
    // layer binds the value's (program, scope, authority, label) to the mint so the emitted event
    // is genuinely token-scoped. A value in another mint's scope is rejected before the verifier
    // CPI.
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(48, BALANCE_FHE_TYPE);
    let foreign_mint = Pubkey::new_unique();
    // A public encrypted store in a different mint's scope, at its own canonical address
    // so the account still deserializes as a valid EncryptedStore.
    let foreign_value_addr = token::encrypted_store_address(foreign_mint, fixture.token_account).0;
    let (foreign_value, proof) = public_leaf_value_account(
        foreign_value_addr,
        fixture.token_account,
        foreign_mint,
        token::burned_amount_key(),
        &[fixture.owner],
        pinned,
        None,
    );

    let mut accounts = fixture.base();
    accounts.insert(foreign_value_addr, encrypted_store_account(&foreign_value));
    let context = mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) = amount_public_decrypt_cert(pinned, cleartext_amount);
    check_token_instruction(
        &context,
        &disclose_secp_ix(
            &fixture,
            foreign_value_addr,
            pinned,
            u256_be(cleartext_amount),
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
fn mollusk_disclose_secp_rejects_cleartext_wider_than_u64() {
    // Token encrypted stores are euint64, so the certified 32-byte uint256 cleartext must
    // fit in 64 bits. The host verifier accepts any 32-byte cleartext its cert signs over; the
    // token layer then rejects a value with nonzero high bytes rather than silently truncating it
    // to the low 64 bits.
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(49, BALANCE_FHE_TYPE);
    let (value, proof) = public_leaf_value_account(
        fixture.amount_store,
        fixture.token_account,
        fixture.mint,
        token::burned_amount_key(),
        &[fixture.owner],
        pinned,
        None,
    );

    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_store, encrypted_store_account(&value));
    let context = mollusk().with_context(accounts);

    // A cleartext whose value exceeds u64::MAX: a nonzero byte in the high 24 (here index 8).
    let mut wide = [0u8; 32];
    wide[8] = 1;
    let extra_data = vec![0x00u8];
    let signatures = zama_solana_test_kit::signing::kms_public_decrypt_cert(
        pinned,
        wide,
        GATEWAY_CHAIN_ID,
        &DECRYPTION_CONTRACT,
        &extra_data,
    );
    check_token_instruction(
        &context,
        &disclose_secp_ix(
            &fixture,
            fixture.amount_store,
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
// EncryptedStore persistent-output model: `confidential_transfer` reaches `fhe_execute` only by
// CPI, so these tests prove the block cap (ban / metering-band charge / application pinning)
// survives that CPI boundary — not just direct `fhe_execute` calls (see `host_mollusk.rs`).
// ===========================================================================

/// Exact HCU cost of the combined transfer execution (`compute_transfer_handles`): `Ge` on
/// euint64 operands (152_000) + debit `Sub` at euint64 (162_000) + `IfThenElse` at euint64
/// (55_000) + transferred `Sub` at euint64 (162_000) + balance-binding scalar `Add` at euint64
/// (133_000) + credit `Add` at euint64 (162_000). The `VerifiedInput` amount is an operand, not
/// a step, so it adds no HCU. See `zama-host` `HCULimit` tables.
const TRANSFER_BATCH_HCU: u64 = 152_000 + 162_000 + 55_000 + 162_000 + 162_000; // 693_000; no add-zero copy

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

fn read_hcu_block_meter(context: &Ctx, address: Pubkey) -> Option<host::HcuBlockMeter> {
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
    let ix = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        sender_attestation(&fixture, 200),
    );

    check_token_instruction(
        &context,
        &ix,
        &[host_error(
            host::errors::ZamaHostError::HcuBlockLimitExceeded,
        )],
    );

    // Atomic revert: both balances are unchanged.
    assert_eq!(
        read_store_handle(&context, fixture.alice_balance_store, token::balance_key()),
        fixture.alice_initial
    );
    assert_eq!(
        read_store_handle(&context, fixture.bob_balance_store, token::balance_key()),
        fixture.bob_initial
    );
}

#[test]
fn mollusk_confidential_transfer_metering_band_charges_meter_through_cpi() {
    // The Some(meter) CPI shape — the production account set once the cap drops below
    // u64::MAX. With a metering-band cap and the meter threaded through ConfidentialTransfer, the
    // transfer must succeed and the meter must be lazy-created and charged with exactly the
    // execution's HCU, proving the optional accounts survive the token -> zama-fhe -> fhe_execute
    // CPI encoding end to end. The metering identity is the execution's application — this
    // program in the mint's scope — one budget per mint, NOT per sender token account.
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    accounts.insert(
        fixture.host_config,
        host_config_account_with_block_cap(
            fixture.owner,
            secp_evm_address(&coprocessor_signing_key()),
            2_000_000,
        ),
    );
    let context = mollusk().with_context(accounts);
    let meter_pda = host::hcu_block_meter_address(fixture.app()).0;
    let ix = confidential_transfer_ix_with_block_cap_accounts(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        sender_attestation(&fixture, 21),
        Vec::new(),
        Some(meter_pda),
        None,
    );

    check_token_instruction(&context, &ix, &[Check::success()]);

    // The transfer completed: the sender's balance moved off its initial handle.
    let alice_balance = read_encrypted_store(&context, fixture.alice_balance_store);
    assert_ne!(
        store_handle(&alice_balance, token::balance_key()),
        fixture.alice_initial
    );
    // The meter was lazy-created through the CPI, keyed on the mint's application, and charged
    // exactly the transfer execution's HCU at the current slot.
    let meter = read_hcu_block_meter(&context, meter_pda).expect("meter created through CPI");
    assert_eq!(meter.program, token::id());
    assert_eq!(meter.scope, fixture.mint.to_bytes());
    assert_eq!(meter.used_hcu, TRANSFER_BATCH_HCU);
    assert_eq!(meter.last_seen_slot, context.mollusk.sysvars.clock.slot);
    // Regression guard on the metering granularity: nothing accrues under the sender token
    // account's key — a sybil minting fresh token accounts gets no fresh budget.
    assert!(read_hcu_block_meter(
        &context,
        host::hcu_block_meter_address(host::AppScope {
            program: token::id(),
            scope: fixture.alice_token.to_bytes(),
        })
        .0
    )
    .is_none());
}

// ---------------------------------------------------------------------------
// confidential_transfer_from_value (spend an existing encrypted amount, fhevm-internal#1680)
//
// The host admits a stored amount operand only when the value's authority signs the execution:
// the sender's token account (which the token program signs for) or the sender's own wallet.
// A recipient therefore cannot spend the sender's `transferred_amount` value directly; the
// amount must be under an authority the spender controls.
// ---------------------------------------------------------------------------

/// Done-when 1: a transfer spends a computed handle produced under the same mint, with no
/// attestation attached. Here the amount is an existing encrypted store under the
/// sender's token account; the balances move through the same `ge -> sub -> select` debit and
/// `add` credit, and the amount value itself is read-only (never replaced, never consumed).
#[test]
fn mollusk_transfer_from_value_spends_existing_amount() {
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let amount_store = seed_amount_store(
        &fixture,
        &mut accounts,
        fixture.alice_token,
        token::transfer_input_key(),
        amount_handle,
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
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        amount_store,
    );

    let result = check_token_instruction(&context, &transfer, &[Check::success()]);
    let persistent_outputs = cleartext.evaluate_fhe_cpi(&context, &result);

    // Only the two balances and the sender's transferred_amount are updated — the amount is not
    // an output.
    assert_eq!(persistent_outputs, 2);
    assert_eq!(cleartext.balance(&context, fixture.alice_token), 750);
    assert_eq!(cleartext.balance(&context, fixture.bob_token), 350);
    let event: token::ConfidentialTransferEvent = result
        .inner_instructions
        .iter()
        .find_map(|inner| decode_anchor_event(&inner.instruction.data))
        .expect("transfer event");
    assert_eq!(cleartext.u64_for_handle(event.transferred_handle), 250);

    // The amount value is read-only: current handle and history unchanged.
    let amount_after = read_encrypted_store(&context, amount_store);
    assert_eq!(
        store_handle(&amount_after, token::transfer_input_key()),
        amount_handle
    );
}

/// A program PDA owns the token account and supplies a foreign amount through signer privileges
/// forwarded to the token instruction. The real token -> host CPI must carry both deny records.
#[test]
fn mollusk_transfer_from_value_checks_every_application_deny_record() {
    for (foreign, denied) in [(false, false), (true, false), (true, true)] {
        let program = confidential_batcher::id();
        let owner = Pubkey::find_program_address(&[b"amount-owner"], &program).0;
        let fixture = TokenFixture::with_keys(owner, Pubkey::new_unique(), Pubkey::new_unique());
        let (mut accounts, token_record) = deny_enabled_transfer_accounts(&fixture, false);
        let amount_handle = handle_for_chain(45, BALANCE_FHE_TYPE);
        let app = if foreign {
            host::AppScope {
                program,
                scope: [0x77; 32],
            }
        } else {
            fixture.app()
        };
        let authority = if foreign { owner } else { fixture.alice_token };
        let amount_store =
            insert_store_slot(&mut accounts, app, authority, [0x77; 32], amount_handle);
        fixture.register_state_key(amount_store, [0x77; 32]);
        let mut records = vec![token_record];
        if foreign {
            let (record, account) = deny_scope_record_account(app, denied);
            accounts.insert(record, account);
            records.push(record);
        }
        let context = mollusk().with_context(accounts);
        let mut transfer = confidential_transfer_from_value_ix(
            &fixture,
            owner,
            fixture.alice_token,
            fixture.bob_token,
            fixture.alice_balance_store,
            fixture.bob_balance_store,
            amount_store,
        );
        transfer.accounts.extend(
            records
                .into_iter()
                .map(|key| AccountMeta::new_readonly(key, false)),
        );
        if denied {
            check_token_instruction(
                &context,
                &transfer,
                &[anchor_error_check(host::ZamaHostError::ScopeDenied as u32)],
            );
            assert_eq!(
                read_store_handle(&context, fixture.alice_balance_store, token::balance_key()),
                fixture.alice_initial
            );
        } else {
            let mut cleartext = CleartextLedger::default();
            cleartext.seed_amount(fixture.alice_initial, 1_000);
            cleartext.seed_amount(fixture.bob_initial, 100);
            cleartext.seed_amount(amount_handle, 200);
            let result = check_token_instruction(&context, &transfer, &[Check::success()]);
            cleartext.evaluate_fhe_cpi(&context, &result);
            assert_eq!(cleartext.balance(&context, fixture.alice_token), 800);
            assert_eq!(cleartext.balance(&context, fixture.bob_token), 300);
        }
    }
}

/// The recipient of a transfer may decrypt the sender's `transferred_amount` (they are allowed on
/// it) but cannot spend it: the value is under the sender's token account, and only its authority
/// admits it as an operand. The token's spend gate rejects the attempt before any host CPI.
#[test]
fn mollusk_transfer_from_value_rejects_amount_under_foreign_authority() {
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    let amount_handle = handle_for_chain(61, BALANCE_FHE_TYPE);
    // The amount is under Bob's wallet; Alice (the from-account owner and signer) neither is nor
    // controls that authority.
    let amount_store = seed_amount_store(
        &fixture,
        &mut accounts,
        fixture.bob_owner,
        token::transfer_input_key(),
        amount_handle,
    );
    let context = mollusk().with_context(accounts);

    let transfer = confidential_transfer_from_value_ix(
        &fixture,
        fixture.owner,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        amount_store,
    );
    check_token_instruction(
        &context,
        &transfer,
        &[token_error(
            token::ConfidentialTokenError::AmountSpendAuthorityMismatch,
        )],
    );

    // Balances untouched.
    let alice_balance = read_encrypted_store(&context, fixture.alice_balance_store);
    assert_eq!(
        store_handle(&alice_balance, token::balance_key()),
        fixture.alice_initial
    );
}

/// The amount handle must be euint64. A non-balance-typed amount is rejected early by the token
/// for a clear error, before the host's binary type validation would reject the same handle
/// deeper.
#[test]
fn mollusk_transfer_from_value_rejects_non_euint64_amount() {
    let fixture = TokenFixture::new();
    let mut accounts = fixture.base_accounts();
    // FHE type 0 (ebool), not euint64.
    let amount_handle = handle_for_chain(62, 0);
    let amount_store = seed_amount_store(
        &fixture,
        &mut accounts,
        fixture.alice_token,
        token::transfer_input_key(),
        amount_handle,
    );
    let context = mollusk().with_context(accounts);

    let transfer = confidential_transfer_from_value_ix(
        &fixture,
        fixture.owner,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        amount_store,
    );
    check_token_instruction(
        &context,
        &transfer,
        &[token_error(
            token::ConfidentialTokenError::AmountHandleTypeMismatch,
        )],
    );
}

/// Spending the entire balance: the amount encrypted store is the sender's own balance
/// value, so `amount_store` aliases the `from_balance` output account. The execution merges them
/// into one account slot, and the transfer debits the whole balance without tripping
/// duplicate-account resolution.
#[test]
fn mollusk_transfer_from_value_spends_full_balance_with_balance_store_account_as_amount() {
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
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        // Amount aliased to the sender's own balance value: transfer the whole balance.
        fixture.alice_balance_store,
    );
    let result = check_token_instruction(&context, &transfer, &[Check::success()]);
    let persistent_outputs = cleartext.evaluate_fhe_cpi(&context, &result);

    assert_eq!(persistent_outputs, 2);
    assert_eq!(cleartext.balance(&context, fixture.alice_token), 0);
    assert_eq!(cleartext.balance(&context, fixture.bob_token), 1_100);
}

/// Re-sending a sent amount: the sender spends their own `transferred_amount` value, which is
/// also this transfer's `transferred_amount` output account. `amount_store` aliases an output the
/// execution writes, and the merged account slot lets the transfer settle.
#[test]
fn transfer_from_value_instruction_is_smaller_than_attested_arm() {
    let fixture = TokenFixture::new();
    let attested = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        sender_attestation(&fixture, 70),
    );
    let amount_store = Pubkey::new_unique();
    fixture.register_state_key(amount_store, token::transfer_input_key());
    let from_value = confidential_transfer_from_value_ix(
        &fixture,
        fixture.owner,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        amount_store,
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
    let transfer = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        sender_attestation(&fixture, 21),
    );

    let result = check_token_instruction(&context, &transfer, &[Check::success()]);

    cost_snapshot::assert_cost_snapshot(
        "token_mollusk",
        "confidential_transfer/direct",
        &transfer,
        &result,
    );

    // Steady state: the first transfer created the transferred-amount
    // `EncryptedStore` at its canonical per-(mint, source) PDA; later
    // transfers update every touched encrypted store in place and create no
    // accounts. Snapshot the second transfer separately.
    //
    // Both profiles share this fixture/context on purpose, so a mismatch on
    // `direct` fails before `steady_state` is asserted — fix the first drift,
    // then re-run to see whether the second also moved.
    let second_transfer = confidential_transfer_ix(
        &fixture,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        sender_attestation(&fixture, 22),
    );

    let second_result = check_token_instruction(&context, &second_transfer, &[Check::success()]);

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
    let amount_store = seed_amount_store(
        &fixture,
        &mut accounts,
        fixture.alice_token,
        token::transfer_input_key(),
        amount_handle,
    );
    let context = mollusk().with_context(accounts);
    let transfer = confidential_transfer_from_value_ix(
        &fixture,
        fixture.owner,
        fixture.alice_token,
        fixture.bob_token,
        fixture.alice_balance_store,
        fixture.bob_balance_store,
        amount_store,
    );

    let result = check_token_instruction(&context, &transfer, &[Check::success()]);

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
    let balance_encrypted_store = token::balance_slot(fixture.mint, token_account).0.address();
    let mut accounts = fixture.base_accounts();
    accounts.insert(owner, system_account(5_000_000_000));
    accounts.insert(token_account, system_account(0));
    accounts.insert(balance_encrypted_store, system_account(0));
    let context = mollusk().with_context(accounts);
    let ix = initialize_token_account_ix(owner, owner, fixture.mint, fixture.host_config);

    let result = check_token_instruction(&context, &ix, &[Check::success()]);

    cost_snapshot::assert_cost_snapshot("token_mollusk", "initialize_token_account", &ix, &result);
}

#[test]
fn disclose_secp_seven_of_thirteen_verifies_and_bounds_compute() {
    // A realistic 7-of-13 KMS public-decrypt cert verifies through the stateless host verifier
    // (CPIed by disclose_secp) and its compute stays well under budget. Cost is dominated by t
    // secp256k1 recoveries (~25k CU each) on top of the single-sig baseline, so a 7-sig cert
    // lands near ~40k + 6 * ~25k; assert a comfortable ceiling.
    let fixture = DiscloseFixture::new();
    let pinned = handle_for_chain(85, BALANCE_FHE_TYPE);
    let replaced = handle_for_chain(86, BALANCE_FHE_TYPE);
    let (value, proof) = public_leaf_value_account(
        fixture.amount_store,
        fixture.token_account,
        fixture.mint,
        token::burned_amount_key(),
        &[fixture.owner],
        pinned,
        Some(replaced),
    );

    // 13 registered KMS signers, public-decrypt threshold 7; the cert is signed by 7 of them. The
    // v0 cert resolves to the CURRENT context (the fixture's), so override that
    // context account with the 13-signer / threshold-7 set.
    let keys: Vec<k256::ecdsa::SigningKey> = (0..13).map(|i| kms_signing_key_n(0x60 + i)).collect();
    let registered: Vec<[u8; 20]> = keys.iter().map(secp_evm_address).collect();

    let mut accounts = fixture.base();
    accounts.insert(fixture.amount_store, encrypted_store_account(&value));
    accounts.insert(
        fixture.kms_context,
        kms_context_account_with_signers(fixture.kms_context_id, &registered, 7),
    );
    let context = mollusk().with_context(accounts);

    let cleartext_amount = 500;
    let (signatures, extra_data) =
        amount_public_decrypt_cert_signed_by(pinned, cleartext_amount, &keys[..7]);
    let result = check_token_instruction(
        &context,
        &disclose_secp_ix(
            &fixture,
            fixture.amount_store,
            pinned,
            u256_be(cleartext_amount),
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
// The burn-side analog of confidential_transfer_from_value (#1680 / #3238): burn an amount given
// as an existing persistent handle the owner may use, instead of a fresh coprocessor attestation.
// The burned-amount output shape is byte-identical to the attestation path (owner allow leaf then
// public-decrypt leaf at its canonical burned_amount value), so redeem_burned_amount consumes it
// unchanged.
// ---------------------------------------------------------------------------

/// Happy path: burn part of a balance from an existing computed/received `euint64` handle, no
/// attestation attached. The burned delta is created publicly decryptable exactly as the
/// attestation path, the balance and encrypted total supply decrement by the burned amount, and
/// the amount value itself is read-only (never replaced, never consumed).
#[test]
fn mollusk_burn_from_value_burns_existing_amount() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(1_000);
    let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let amount_store = seed_burn_amount_store(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::transfer_input_key(),
        amount_handle,
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
        amount_store,
        token::transfer_input_key(),
    );
    let result = check_token_instruction(&context, &burn, &[Check::success()]);
    let persistent_outputs = cleartext.evaluate_fhe_cpi(&context, &result);

    // Three persistent outputs are updated — balance, burned_amount, total_supply — and the
    // amount is not one.
    assert_eq!(persistent_outputs, 3);
    assert_eq!(cleartext.balance(&context, fixture.token_account), 750);
    assert_eq!(
        cleartext.u64_in_state(
            &context,
            fixture.total_supply_store,
            token::total_supply_key()
        ),
        4_750
    );

    // The burned delta is created publicly decryptable: the first burn creates the value and
    // appends the owner's allow leaf then the public-decrypt leaf for the just-bound burned handle
    // (DD-036 / Vector 2), identical to the attestation path.
    let burned = read_encrypted_store(&context, fixture.burned_amount_store);
    assert_eq!(
        cleartext.u64_in_state(
            &context,
            fixture.burned_amount_store,
            token::burned_amount_key()
        ),
        250
    );
    assert_eq!(burned.leaf_count, 3);
    assert_eq!(
        burned.peaks,
        zama_solana_acl::mmr_peaks_from_leaves(&burn_update_leaves(
            &fixture,
            0,
            store_handle(&burned, token::balance_key()),
            store_handle(&burned, token::burned_amount_key())
        ))
    );

    // The amount value is read-only: current handle and history unchanged.
    let amount_after = read_encrypted_store(&context, amount_store);
    assert_eq!(
        store_handle(&amount_after, token::transfer_input_key()),
        amount_handle
    );
    assert_eq!(amount_after.leaf_count, 3);
}

/// Whole-balance alias regression (the #3238 aliasing class): burning the entire balance uses the
/// account's own balance value AS the amount, so `amount_store` aliases the `balance` output. The
/// execution merges them into one slot, and the dedup skips pushing the amount a second time, so
/// the burn settles without tripping duplicate-account resolution.
#[test]
fn mollusk_burn_from_value_whole_balance_alias() {
    let fixture = BurnRedeemFixture::new();
    let accounts = fixture.accounts(1_000);
    let context = burn_redeem_mollusk().with_context(accounts);

    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.initial_balance, 1_000);
    cleartext.seed_amount(fixture.initial_total_supply, 5_000);

    // Amount aliased to the account's own balance value: burn the whole balance.
    let burn = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        fixture.balance_store,
        token::balance_key(),
    );
    let result = check_token_instruction(&context, &burn, &[Check::success()]);
    let persistent_outputs = cleartext.evaluate_fhe_cpi(&context, &result);

    assert_eq!(persistent_outputs, 3);
    assert_eq!(cleartext.balance(&context, fixture.token_account), 0);
    assert_eq!(
        cleartext.u64_in_state(
            &context,
            fixture.total_supply_store,
            token::total_supply_key()
        ),
        4_000
    );
    assert_eq!(
        cleartext.u64_in_state(
            &context,
            fixture.burned_amount_store,
            token::burned_amount_key()
        ),
        1_000
    );
}

/// Re-burning the burned-amount value (the second alias branch): the second burn spends the
/// `burned_amount` value itself as the amount, so `amount_store` aliases the `burned_amount`
/// output this execution writes. The execution merges the aliased slot (read at the old handle,
/// replaced to the new delta), and the dedup skips pushing the amount a second time — the
/// `amount == burned_amount value` branch. Mirrors
/// `mollusk_transfer_from_value_resends_transferred_amount_that_is_also_this_output`.
#[test]
fn mollusk_burn_from_value_reburns_burned_amount_that_is_also_this_output() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(1_000);
    let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let amount_store = seed_burn_amount_store(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::transfer_input_key(),
        amount_handle,
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.initial_balance, 1_000);
    cleartext.seed_amount(fixture.initial_total_supply, 5_000);
    cleartext.seed_amount(amount_handle, 250);

    // First burn (250) creates the burned_amount value: balance 750, total_supply 4750, burned 250.
    let first = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        amount_store,
        token::transfer_input_key(),
    );
    let first_result = check_token_instruction(&context, &first, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &first_result);
    let first_burned = read_store_handle(
        &context,
        fixture.burned_amount_store,
        token::burned_amount_key(),
    );
    let first_state = read_encrypted_store(&context, fixture.burned_amount_store);
    let first_balance = store_handle(&first_state, token::balance_key());
    assert_eq!(
        cleartext.u64_in_state(
            &context,
            fixture.burned_amount_store,
            token::burned_amount_key()
        ),
        250
    );

    // Settle the first pending burn before opening the next one. Cancellation restores balance
    // and supply while retaining the burned-amount value and its leaves.
    let pending_burn = token::pending_burn_address(fixture.mint, fixture.token_account).0;
    let cancel = cancel_pending_burn_ix(&fixture, pending_burn, None);
    let cancel_result = check_token_instruction(&context, &cancel, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &cancel_result);
    let after_cancel = read_encrypted_store(&context, fixture.burned_amount_store);
    let restored_balance = store_handle(&after_cancel, token::balance_key());

    // The next burn spends the burned-amount value itself as the amount — which is also this
    // burn's burned_amount output account (the alias the dedup must merge, not double-push).
    let again = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        fixture.burned_amount_store,
        token::burned_amount_key(),
    );
    let again_result = check_token_instruction(&context, &again, &[Check::success()]);
    let persistent_outputs = cleartext.evaluate_fhe_cpi(&context, &again_result);

    // Conservation after cancellation: the next burn's amount equals the previous burned delta
    // (250), so the restored balance and encrypted total supply each drop by 250.
    assert_eq!(persistent_outputs, 3);
    assert_eq!(cleartext.balance(&context, fixture.token_account), 750);
    assert_eq!(
        cleartext.u64_in_state(
            &context,
            fixture.total_supply_store,
            token::total_supply_key()
        ),
        4_750
    );
    assert_eq!(
        cleartext.u64_in_state(
            &context,
            fixture.burned_amount_store,
            token::burned_amount_key()
        ),
        250
    );

    // The burned_amount value stays a well-formed two-burn MMR: both burns' handles are present
    // as allow + public-decrypt leaf pairs, even though the first pending burn was cancelled and
    // the second burn read H1 as its amount operand. Only H2 remains pending and redeemable.
    let value = read_encrypted_store(&context, fixture.burned_amount_store);
    assert_eq!(value.leaf_count, 7);
    let mut leaves = burn_update_leaves(&fixture, 0, first_balance, first_burned);
    leaves.extend(allow_leaves(
        fixture.balance_store,
        3,
        restored_balance,
        &[fixture.owner],
    ));
    leaves.extend(burn_update_leaves(
        &fixture,
        4,
        store_handle(&value, token::balance_key()),
        store_handle(&value, token::burned_amount_key()),
    ));
    assert_eq!(value.peaks, zama_solana_acl::mmr_peaks_from_leaves(&leaves));
}

/// PDA-owner CPI driver: the batcher path burns as a program PDA that owns the token account and
/// authorizes the burn via `invoke_signed`. The callee sees only `owner.is_signer` — identical
/// whether a keypair or a program's PDA signed — so the path is exercised by marking the owner
/// PDA a signer and paying rent from a separate keypair (the driver's fee payer, as
/// `invoke_signed` would). The spend gate and owner check both accept the PDA owner.
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
    let amount_store = seed_burn_amount_store(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::transfer_input_key(),
        amount_handle,
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.initial_balance, 1_000);
    cleartext.seed_amount(fixture.initial_total_supply, 5_000);
    cleartext.seed_amount(amount_handle, 400);

    let burn = confidential_burn_from_value_auto(
        &context,
        &fixture,
        pda_owner,
        payer,
        amount_store,
        token::transfer_input_key(),
    );
    let result = check_token_instruction(&context, &burn, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &result);

    assert_eq!(cleartext.balance(&context, fixture.token_account), 600);
    assert_eq!(
        cleartext.u64_in_state(
            &context,
            fixture.burned_amount_store,
            token::burned_amount_key()
        ),
        400
    );
    // The burned value's allow leaf names the PDA owner.
    let burned = read_encrypted_store(&context, fixture.burned_amount_store);
    assert_eq!(
        burned.peaks,
        zama_solana_acl::mmr_peaks_from_leaves(&burn_update_leaves(
            &fixture,
            0,
            store_handle(&burned, token::balance_key()),
            store_handle(&burned, token::burned_amount_key())
        ))
    );
}

/// Downstream compatibility: a burned handle produced by the from-value path feeds
/// `redeem_burned_amount` unchanged. The burned output shape (owner allow then public leaf at the
/// canonical `burned_amount` value) is identical to the attestation path, so the KMS-cert +
/// public-decrypt-proof redeem consumes it and pays out the vault.
#[test]
fn mollusk_burn_from_value_burned_handle_redeems() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(1_000);
    let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let amount_store = seed_burn_amount_store(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::transfer_input_key(),
        amount_handle,
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    // Burn 500 from the existing amount handle; the created-public burned handle is the new
    // value handle.
    let mut cleartext = CleartextLedger::default();
    cleartext.seed_amount(fixture.initial_balance, 1_000);
    cleartext.seed_amount(fixture.initial_total_supply, 5_000);
    cleartext.seed_amount(amount_handle, 500);
    let burn = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        amount_store,
        token::transfer_input_key(),
    );
    let burn_result = check_token_instruction(&context, &burn, &[Check::success()]);
    cleartext.evaluate_fhe_cpi(&context, &burn_result);
    let burned_handle = read_store_handle(
        &context,
        fixture.burned_amount_store,
        token::burned_amount_key(),
    );

    // Redeem the burned handle with a real KMS cert + public-decrypt inclusion proof.
    let cleartext_amount = 500;
    let (signatures, extra_data) = amount_public_decrypt_cert(burned_handle, cleartext_amount);
    let proof = single_burn_public_decrypt_proof(&fixture, burned_handle);
    let pending_burn = token::pending_burn_address(fixture.mint, fixture.token_account).0;
    check_token_instruction(
        &context,
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

/// The from-value burn uses the same application witness rule as transfer, including deduplication.
#[test]
fn mollusk_burn_from_value_checks_every_application_deny_record() {
    for (foreign, denied) in [(false, false), (true, false), (true, true)] {
        let program = confidential_batcher::id();
        let owner = Pubkey::find_program_address(&[b"amount-owner"], &program).0;
        let fixture =
            BurnRedeemFixture::with_keys(owner, Pubkey::new_unique(), Pubkey::new_unique());
        let mut accounts = fixture.accounts(1_000);
        accounts.insert(
            fixture.host_config,
            deny_enabled_host_config_account(owner, secp_evm_address(&coprocessor_signing_key())),
        );
        let (token_record, token_account) = deny_scope_record_account(fixture.app(), false);
        accounts.insert(token_record, token_account);
        let payer = Pubkey::new_unique();
        accounts.insert(payer, system_account(5_000_000_000));
        let amount_handle = handle_for_chain(60, BALANCE_FHE_TYPE);
        let app = if foreign {
            host::AppScope {
                program,
                scope: [0x77; 32],
            }
        } else {
            fixture.app()
        };
        let authority = if foreign {
            owner
        } else {
            fixture.token_account
        };
        let amount_store =
            insert_store_slot(&mut accounts, app, authority, [0x77; 32], amount_handle);
        let mut records = vec![token_record];
        if foreign {
            let (record, account) = deny_scope_record_account(app, denied);
            accounts.insert(record, account);
            records.push(record);
        }
        let context = burn_redeem_mollusk().with_context(accounts);
        let mut burn = confidential_burn_from_value_auto(
            &context,
            &fixture,
            owner,
            payer,
            amount_store,
            [0x77; 32],
        );
        burn.accounts.extend(
            records
                .into_iter()
                .map(|key| AccountMeta::new_readonly(key, false)),
        );
        if denied {
            check_token_instruction(
                &context,
                &burn,
                &[anchor_error_check(host::ZamaHostError::ScopeDenied as u32)],
            );
            assert_eq!(
                read_store_handle(&context, fixture.balance_store, token::balance_key()),
                fixture.initial_balance
            );
        } else {
            let mut cleartext = CleartextLedger::default();
            cleartext.seed_amount(fixture.initial_balance, 1_000);
            cleartext.seed_amount(fixture.initial_total_supply, 5_000);
            cleartext.seed_amount(amount_handle, 300);
            let result = check_token_instruction(&context, &burn, &[Check::success()]);
            cleartext.evaluate_fhe_cpi(&context, &result);
            assert_eq!(cleartext.balance(&context, fixture.token_account), 700);
            assert_eq!(
                cleartext.u64_in_state(
                    &context,
                    fixture.burned_amount_store,
                    token::burned_amount_key()
                ),
                300
            );
        }
    }
}

/// An amount under an authority the signer does not control is rejected by the token's spend gate
/// with its own distinct error, before any host CPI — even though the signer owns the debited
/// token account.
#[test]
fn mollusk_burn_from_value_rejects_amount_under_foreign_authority() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(1_000);
    let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    let stranger = Pubkey::new_unique();
    let amount_store = seed_burn_amount_store(
        &fixture,
        &mut accounts,
        stranger,
        token::transfer_input_key(),
        amount_handle,
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let burn = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        amount_store,
        token::transfer_input_key(),
    );
    check_token_instruction(
        &context,
        &burn,
        &[token_error(
            token::ConfidentialTokenError::AmountSpendAuthorityMismatch,
        )],
    );

    // Balance untouched.
    assert_eq!(
        read_store_handle(&context, fixture.balance_store, token::balance_key()),
        fixture.initial_balance
    );
}

/// The amount handle must be euint64. A non-balance-typed amount is rejected early by the token
/// for a clear error, before the host's binary type validation would reject the same handle
/// deeper.
#[test]
fn mollusk_burn_from_value_rejects_non_euint64_amount() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(1_000);
    // FHE type 0 (ebool), not euint64.
    let amount_handle = handle_for_chain(42, 0);
    let amount_store = seed_burn_amount_store(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::transfer_input_key(),
        amount_handle,
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let burn = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        amount_store,
        token::transfer_input_key(),
    );
    check_token_instruction(
        &context,
        &burn,
        &[token_error(
            token::ConfidentialTokenError::AmountHandleTypeMismatch,
        )],
    );
}

/// The signing owner must own the debited token account. A signer that controls the amount's
/// authority (so the spend gate passes) but is not the token account owner is rejected with
/// `OwnerMismatch`.
#[test]
fn mollusk_burn_from_value_rejects_owner_not_token_account_owner() {
    let fixture = BurnRedeemFixture::new();
    let mut accounts = fixture.accounts(1_000);
    let wrong_owner = Pubkey::new_unique();
    accounts.insert(wrong_owner, system_account(5_000_000_000));
    let amount_handle = handle_for_chain(41, BALANCE_FHE_TYPE);
    // The amount is under wrong_owner's wallet (spend gate passes) but wrong_owner does not own
    // the token account.
    let amount_store = seed_burn_amount_store(
        &fixture,
        &mut accounts,
        wrong_owner,
        token::transfer_input_key(),
        amount_handle,
    );
    let context = burn_redeem_mollusk().with_context(accounts);

    let burn = confidential_burn_from_value_auto(
        &context,
        &fixture,
        wrong_owner,
        wrong_owner,
        amount_store,
        token::transfer_input_key(),
    );
    check_token_instruction(
        &context,
        &burn,
        &[token_error(token::ConfidentialTokenError::OwnerMismatch)],
    );
}

/// The from-value burn carries no 190-byte attestation, so its instruction data is strictly
/// SMALLER than the fresh-attested burn's — the measured wire-size win for a contract-driven
/// execution burn.
#[test]
fn burn_from_value_instruction_is_smaller_than_attested_arm() {
    let fixture = BurnRedeemFixture::new();
    let pending_burn = token::pending_burn_address(fixture.mint, fixture.token_account).0;
    let attested = confidential_burn_ix(&fixture, fixture.owner_attestation(70), pending_burn);
    let from_value = confidential_burn_from_value_ix(
        &fixture,
        fixture.owner,
        fixture.owner,
        Pubkey::new_unique(),
        token::transfer_input_key(),
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
    let amount_store = seed_burn_amount_store(
        &fixture,
        &mut accounts,
        fixture.token_account,
        token::transfer_input_key(),
        amount_handle,
    );
    let context = burn_redeem_mollusk().with_context(accounts);
    let burn = confidential_burn_from_value_auto(
        &context,
        &fixture,
        fixture.owner,
        fixture.owner,
        amount_store,
        token::transfer_input_key(),
    );

    let result = check_token_instruction(&context, &burn, &[Check::success()]);

    cost_snapshot::assert_cost_snapshot(
        "token_mollusk",
        "confidential_burn_from_value/direct",
        &burn,
        &result,
    );
}

#[test]
fn transfer_entry_points_return_the_transferred_handle_to_their_caller() {
    let fixture = TokenFixture::new();
    for from_slot in [false, true] {
        let ix = if from_slot {
            confidential_transfer_from_value_ix(
                &fixture,
                fixture.owner,
                fixture.alice_token,
                fixture.bob_token,
                fixture.alice_balance_store,
                fixture.bob_balance_store,
                fixture.alice_balance_store,
            )
        } else {
            confidential_transfer_ix(
                &fixture,
                fixture.alice_token,
                fixture.bob_token,
                fixture.alice_balance_store,
                fixture.bob_balance_store,
                sender_attestation(&fixture, 21),
            )
        };
        let baseline = mollusk().with_context(fixture.base_accounts());
        let result = check_token_instruction(&baseline, &ix, &[Check::success()]);
        let expected = transferred_event_handle(&result).to_vec();
        let mut svm = mollusk();
        svm.sysvars.clock = baseline.mollusk.sysvars.clock.clone();
        svm.sysvars.slot_hashes =
            solana_sdk::slot_hashes::SlotHashes::new(&baseline.mollusk.sysvars.slot_hashes);
        svm.add_program(&delegator_vault::ID, "delegator_vault");
        let context = svm.with_context(fixture.base_accounts());
        let mut probe = anchor_ix(
            delegator_vault::ID,
            delegator_vault::accounts::CheckCpiReturn { callee: token::ID },
            delegator_vault::instruction::CheckCpiReturn {
                instruction_data: ix.data,
                expected,
            },
        );
        probe.accounts.extend(ix.accounts);
        zama_solana_test_kit::transaction::process_fhe_instruction(
            &context,
            fixture.owner,
            &probe,
            &[Check::success()],
        );
        for address in fixture.base_accounts().keys() {
            assert_eq!(
                context.account_store.borrow().get(address),
                baseline.account_store.borrow().get(address),
                "{address}"
            );
        }
    }
}
