//! Mollusk-based runtime tests for the RFC 035 `EncryptedValue` model.
//!
//! An encrypted value belongs to an application `(program, scope)` and is controlled by a value
//! authority that is a PDA of `program`. Who may decrypt a handle is decided by the write that
//! produces it: every allowed key is sealed as one MMR leaf on the new handle, and
//! `make_handle_public` seals a public-decrypt leaf. There is no membership list to edit; the
//! deny list, the HCU block meter and the trust registry key on the application.
//!
//! Covered here: `fhe_execute` persistent outputs (create with the PDA proof, update against the
//! stored handle, allows and leaves), the reads an execution admits, application folding, the
//! deny and pause gates, off-chain reconstruction and proofs, the public-outputs event, the HCU
//! block cap, the rand nonce, `verify_public_decrypt`, and the cost snapshots.

use anchor_lang::{
    prelude::system_program, AccountDeserialize, AnchorDeserialize, Discriminator, InstructionData,
};
use mollusk_svm::result::Check;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use std::collections::HashMap;
use zama_host::encode::ExecutionDictionary;
use zama_host::{
    self as host, AppScope, EncryptedValue, FheBinaryOpCode, FheExecuteArgs, FheExecuteOperand,
    FheExecuteOutput, FheExecuteStep, FheTernaryOpCode, HostConfig,
};
use zama_solana_acl::encrypted_value_account::EncryptedValueAccountEvent;
use zama_solana_test_kit::{
    anchor_error_check, anchor_framework_error_check, anchor_ix, canonical_test_context_id,
    cost_snapshot, deny_scope_record_account, empty_system_account, encrypted_value_account,
    event_authority, funded_system_account, handle_for_chain, host_svm as mollusk,
    host_svm_without_previous_bank_hash as mollusk_without_previous_bank_hash, label,
    new_encrypted_value as new_encrypted_value_account, rand_nonce_account, read_encrypted_value,
    read_encrypted_value_from_result, readonly, readonly_signer, serialized_account, signing,
    system_account, system_program_account, writable, DECRYPTION_CONTRACT, GATEWAY_CHAIN_ID,
    INPUT_VERIFICATION_CONTRACT,
};

mod host_fixtures;
use host_fixtures::{
    created_public_batch, fhe_execute_ix, fhe_execute_ix_with_extras, fixture_scope,
    host_config_account, host_config_account_with_flags, mollusk_execute_context, read_host_config,
    sole_value_authority, value_authority, FheExecuteExtras, ValueAuthority,
};

// ---------------------------------------------------------------------------
// Harness
// ---------------------------------------------------------------------------

/// A fixture application: the value authority that signs for its values (a PDA of a fresh
/// program) and the scope they live in. Every value a test seeds or writes through it is canonical
/// for `(program, authority, scope, label)`, which is what the host rederives on read and write.
#[derive(Clone, Copy)]
struct App {
    authority: ValueAuthority,
    scope: [u8; 32],
}

impl App {
    fn new() -> Self {
        Self::with_program(Pubkey::new_unique())
    }

    /// An application on a caller-fixed program, for cost snapshots: PDA bump searches are part of
    /// the measured compute, so recorded addresses must not move between runs.
    fn with_program(program: Pubkey) -> Self {
        Self {
            authority: sole_value_authority(program),
            scope: fixture_scope(),
        }
    }

    /// Another application of the same program: a second scope.
    fn sibling_scope(&self, scope: [u8; 32]) -> Self {
        Self {
            authority: self.authority,
            scope,
        }
    }

    /// The same application under a second value authority of its program.
    fn sibling_authority(&self, seed_key: Pubkey) -> Self {
        Self {
            authority: value_authority(self.authority.program, seed_key),
            scope: self.scope,
        }
    }

    fn app(&self) -> AppScope {
        self.authority.app(self.scope)
    }

    fn program(&self) -> Pubkey {
        self.authority.program
    }

    /// The signer of every read and write of this application's values.
    fn key(&self) -> Pubkey {
        self.authority.key
    }

    /// A canonical value at `name` carrying `handle` and no history.
    fn value(&self, name: &str, handle: [u8; 32]) -> (Pubkey, EncryptedValue) {
        new_encrypted_value_account(self.app(), self.key(), label(name), handle)
    }

    fn address(&self, name: &str) -> Pubkey {
        self.authority.value_address(self.scope, label(name))
    }

    /// A persistent output of this application: a create when `previous_handle` is `None`
    /// (carrying the PDA proof), an update otherwise.
    fn stored_output(
        &self,
        dictionary: &mut ExecutionDictionary,
        output_encrypted_value_index: u8,
        name: &str,
        allows: &[Pubkey],
        previous_handle: Option<[u8; 32]>,
        make_public: bool,
    ) -> FheExecuteOutput {
        self.authority.stored_output(
            dictionary,
            output_encrypted_value_index,
            self.scope,
            label(name),
            allows,
            previous_handle,
            make_public,
        )
    }
}

fn paused_host_config_account(admin: Pubkey) -> (Pubkey, Account) {
    host_config_account_with_flags(admin, true, false)
}

fn deny_enabled_host_config_account(admin: Pubkey) -> (Pubkey, Account) {
    host_config_account_with_flags(admin, false, true)
}

/// The accounts every single-signer execution of `app` runs against.
fn execution_accounts(
    payer: Pubkey,
    app: &App,
    host_config: Pubkey,
    host_config_account: Account,
) -> Vec<(Pubkey, Account)> {
    vec![
        (system_program::ID, system_program_account()),
        (payer, funded_system_account()),
        (app.key(), empty_system_account()),
        (host_config, host_config_account),
        (event_authority(host::id()), Account::default()),
    ]
}

/// One `TrivialEncrypt` written over the stored value at `address`, allowing `allows` on the new
/// handle. Returns the updated account.
#[allow(clippy::too_many_arguments)]
fn update_with_fhe_execute(
    payer: Pubkey,
    app: &App,
    host_config: Pubkey,
    host_config_account: Account,
    address: Pubkey,
    value: &EncryptedValue,
    allows: &[Pubkey],
    plaintext_tag: u8,
) -> EncryptedValue {
    let result = run_update(
        payer,
        app,
        host_config,
        host_config_account,
        address,
        value,
        Some(value.current_handle),
        allows,
        plaintext_tag,
        Check::success(),
    );
    read_encrypted_value_from_result(&result, address)
}

#[allow(clippy::too_many_arguments)]
fn run_update(
    payer: Pubkey,
    app: &App,
    host_config: Pubkey,
    host_config_account: Account,
    address: Pubkey,
    value: &EncryptedValue,
    previous_handle: Option<[u8; 32]>,
    allows: &[Pubkey],
    plaintext_tag: u8,
    expected: Check<'static>,
) -> mollusk_svm::result::InstructionResult {
    assert_eq!(value.encrypted_value_account_authority, app.key());
    let mut dictionary = ExecutionDictionary::default();
    let steps = vec![FheExecuteStep::TrivialEncrypt {
        plaintext: [plaintext_tag; 32],
        fhe_type: 5,
        output: app.authority.stored_output(
            &mut dictionary,
            0,
            value.scope,
            value.label,
            allows,
            previous_handle,
            false,
        ),
    }];
    let args = FheExecuteArgs {
        account_count: 0,
        dictionary: dictionary.into_entries(),
        steps,
    };
    let ix = fhe_execute_ix(payer, app.key(), host_config, args, vec![writable(address)]);
    let mut accounts = execution_accounts(payer, app, host_config, host_config_account);
    accounts.push((address, encrypted_value_account(value)));
    mollusk().process_and_validate_instruction(&ix, &accounts, &[expected])
}

fn custom_error(error: host::errors::ZamaHostError) -> Check<'static> {
    anchor_error_check(error as u32)
}

/// The leaves a write of `handle` allowing `keys` seals, appended onto `peaks`/`count`.
fn append_allow_leaves(
    address: Pubkey,
    handle: [u8; 32],
    keys: &[Pubkey],
    peaks: &mut Vec<[u8; 32]>,
    count: &mut u64,
) {
    for key in keys {
        let leaf = zama_solana_acl::historical_access_leaf_commitment(
            address.to_bytes(),
            *count,
            handle,
            key.to_bytes(),
        );
        zama_solana_acl::mmr_append(peaks, count, leaf).unwrap();
    }
}

fn allowed_events(handle: [u8; 32], keys: &[Pubkey]) -> Vec<EncryptedValueAccountEvent> {
    keys.iter()
        .map(|key| EncryptedValueAccountEvent::Allowed {
            handle,
            key: key.to_bytes(),
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Instruction builders
// ---------------------------------------------------------------------------

fn make_handle_public_ix(
    payer: Pubkey,
    authority: Pubkey,
    encrypted_value: Pubkey,
    host_config: Pubkey,
    handle: [u8; 32],
    deny_scope_record: Option<Pubkey>,
) -> Instruction {
    anchor_ix(
        host::id(),
        host::accounts::MakeEncryptedValueHandlePublic {
            payer,
            authority,
            encrypted_value,
            host_config,
            deny_scope_record,
            system_program: system_program::ID,
        },
        host::instruction::MakeHandlePublic { handle },
    )
}

/// The accounts of a `make_handle_public` on `address` by `app`.
fn make_public_accounts(
    payer: Pubkey,
    app: &App,
    address: Pubkey,
    value: &EncryptedValue,
    host_config: Pubkey,
    host_config_account: Account,
) -> Vec<(Pubkey, Account)> {
    vec![
        (system_program::ID, system_program_account()),
        (payer, funded_system_account()),
        (app.key(), empty_system_account()),
        (address, encrypted_value_account(value)),
        (host_config, host_config_account),
    ]
}

/// A one-step create of `app`'s value `name`, allowing `allows`, as instruction plus accounts.
fn create_case(
    payer: Pubkey,
    app: &App,
    host_config: Pubkey,
    host_config_account: Account,
    name: &str,
    allows: &[Pubkey],
    extras: FheExecuteExtras,
) -> (Pubkey, Instruction, Vec<(Pubkey, Account)>) {
    let output = app.address(name);
    let mut dictionary = ExecutionDictionary::default();
    let steps = vec![FheExecuteStep::TrivialEncrypt {
        plaintext: [1; 32],
        fhe_type: 5,
        output: app.stored_output(&mut dictionary, 0, name, allows, None, false),
    }];
    let ix = fhe_execute_ix_with_extras(
        payer,
        app.key(),
        host_config,
        FheExecuteArgs {
            account_count: 0,
            dictionary: dictionary.into_entries(),
            steps,
        },
        vec![writable(output)],
        extras,
    );
    let mut accounts = execution_accounts(payer, app, host_config, host_config_account);
    accounts.push((output, empty_system_account()));
    (output, ix, accounts)
}

#[test]
fn mollusk_fhe_execute_fails_closed_without_previous_bank_hash() {
    let payer = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_account(payer);
    let (_, ix, accounts) = create_case(
        payer,
        &app,
        host_config,
        host_config_account,
        "balance",
        &[payer],
        FheExecuteExtras::default(),
    );
    mollusk_without_previous_bank_hash().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::PreviousBankHashUnavailable,
        )],
    );
}

// ---------------------------------------------------------------------------
// Persistent writes: allows are sealed on the write
// ---------------------------------------------------------------------------

#[test]
fn mollusk_fhe_execute_update_seals_one_leaf_per_allowed_key_on_the_new_handle() {
    let payer = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_account(payer);
    let viewer_a = Pubkey::new_unique();
    let viewer_b = Pubkey::new_unique();
    let old_handle = handle_for_chain(3, 5);
    let (address, value) = app.value("balance", old_handle);

    let updated = update_with_fhe_execute(
        payer,
        &app,
        host_config,
        host_config_account,
        address,
        &value,
        &[viewer_a, viewer_b],
        4,
    );
    assert_ne!(updated.current_handle, old_handle);
    assert_eq!(updated.leaf_count, 2);

    // The leaves commit to the NEW handle in declared allow order: nobody was allowed on the old
    // one, and the write that installed it said nothing about it.
    let mut expected_peaks = Vec::new();
    let mut expected_count = 0u64;
    append_allow_leaves(
        address,
        updated.current_handle,
        &[viewer_a, viewer_b],
        &mut expected_peaks,
        &mut expected_count,
    );
    assert_eq!(updated.peaks, expected_peaks);
    // The identity fields never move on an update.
    assert_eq!(updated.program, app.program());
    assert_eq!(updated.encrypted_value_account_authority, app.key());
    assert_eq!(updated.scope, app.scope);
    assert_eq!(updated.label, value.label);
}

#[test]
fn mollusk_fhe_execute_update_with_no_allows_seals_no_leaf() {
    // Allowing nobody is legal (the token's total supply is written this way): the handle moves
    // and the history is untouched.
    let payer = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_account(payer);
    let old_handle = handle_for_chain(3, 5);
    let (address, value) = app.value("supply", old_handle);

    let updated = update_with_fhe_execute(
        payer,
        &app,
        host_config,
        host_config_account,
        address,
        &value,
        &[],
        4,
    );
    assert_ne!(updated.current_handle, old_handle);
    assert_eq!(updated.leaf_count, 0);
    assert!(updated.peaks.is_empty());
}

#[test]
fn mollusk_fhe_execute_rejects_duplicate_and_zero_allow_keys() {
    let payer = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_account(payer);
    let (address, value) = app.value("balance", handle_for_chain(3, 5));
    let viewer = Pubkey::new_unique();

    for allows in [vec![viewer, viewer], vec![viewer, Pubkey::default()]] {
        let result = run_update(
            payer,
            &app,
            host_config,
            host_config_account.clone(),
            address,
            &value,
            Some(value.current_handle),
            &allows,
            5,
            custom_error(host::errors::ZamaHostError::InvalidAllowKey),
        );
        assert_eq!(
            read_encrypted_value_from_result(&result, address).current_handle,
            value.current_handle
        );
    }
}

#[test]
fn mollusk_fhe_execute_rejects_stale_previous_handle() {
    let payer = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_account(payer);
    let (address, value) = app.value("balance", handle_for_chain(3, 5));
    run_update(
        payer,
        &app,
        host_config,
        host_config_account,
        address,
        &value,
        Some(handle_for_chain(99, 5)),
        &[payer],
        6,
        custom_error(host::errors::ZamaHostError::PreviousStateMismatch),
    );
}

#[test]
fn mollusk_fhe_execute_rejects_wallet_as_value_authority_on_create() {
    // A wallet can never be a value authority: the create carries the seeds that must derive the
    // authority from the program, and no seeds derive a wallet.
    let wallet = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_account(wallet);
    let output_label = label("wallet-owned");
    let output = host::encrypted_value_address(app.program(), wallet, app.scope, output_label).0;
    let mut dictionary = ExecutionDictionary::default();
    let steps = vec![FheExecuteStep::TrivialEncrypt {
        plaintext: [1; 32],
        fhe_type: 5,
        output: FheExecuteOutput::StoredValue {
            output_encrypted_value_index: 0,
            output_authority_index: None,
            output_program_index: dictionary.intern_key(app.program()),
            output_authority_key_index: dictionary.intern_key(wallet),
            output_scope_index: dictionary.intern(app.scope),
            output_label_index: dictionary.intern(output_label),
            output_authority_seeds: app.authority.seeds(&mut dictionary),
            output_allow_indexes: dictionary.intern_keys([wallet]),
            previous_handle_index: None,
            make_public: false,
        },
    }];
    let ix = fhe_execute_ix(
        wallet,
        wallet,
        host_config,
        FheExecuteArgs {
            account_count: 0,
            dictionary: dictionary.into_entries(),
            steps,
        },
        vec![writable(output)],
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (wallet, funded_system_account()),
        (host_config, host_config_account),
        (event_authority(host::id()), Account::default()),
        (output, empty_system_account()),
    ];
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::EncryptedValueAuthorityNotProgramPda,
        )],
    );
}

#[test]
fn mollusk_fhe_execute_rejects_create_whose_seeds_derive_under_another_program() {
    // The authority is a real PDA, but of a different program than the one the output names: the
    // proof fails, so no program can create values that claim another program's application.
    let payer = Pubkey::new_unique();
    let app = App::new();
    let claimed_program = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(payer);
    let output_label = label("claimed");
    let output =
        host::encrypted_value_address(claimed_program, app.key(), app.scope, output_label).0;
    let mut dictionary = ExecutionDictionary::default();
    let steps = vec![FheExecuteStep::TrivialEncrypt {
        plaintext: [1; 32],
        fhe_type: 5,
        output: FheExecuteOutput::StoredValue {
            output_encrypted_value_index: 0,
            output_authority_index: None,
            output_program_index: dictionary.intern_key(claimed_program),
            output_authority_key_index: dictionary.intern_key(app.key()),
            output_scope_index: dictionary.intern(app.scope),
            output_label_index: dictionary.intern(output_label),
            output_authority_seeds: app.authority.seeds(&mut dictionary),
            output_allow_indexes: dictionary.intern_keys([payer]),
            previous_handle_index: None,
            make_public: false,
        },
    }];
    let ix = fhe_execute_ix(
        payer,
        app.key(),
        host_config,
        FheExecuteArgs {
            account_count: 0,
            dictionary: dictionary.into_entries(),
            steps,
        },
        vec![writable(output)],
    );
    let mut accounts = execution_accounts(payer, &app, host_config, host_config_account);
    accounts.push((output, empty_system_account()));
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::EncryptedValueAuthorityNotProgramPda,
        )],
    );
}

/// `app` reads `foreign`'s value and writes its own output; `foreign_signs` decides whether the
/// foreign authority is a signing remaining account.
fn read_foreign_value_case(
    payer: Pubkey,
    app: &App,
    foreign: &App,
    foreign_signs: bool,
    expected: Check<'static>,
) -> mollusk_svm::result::InstructionResult {
    let (host_config, host_config_account) = host_config_account(payer);
    let foreign_handle = handle_for_chain(60, 5);
    let (foreign_address, foreign_value) = foreign.value("theirs", foreign_handle);
    let output = app.address("mine");
    let mut dictionary = ExecutionDictionary::default();
    let steps = vec![FheExecuteStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: FheExecuteOperand::StoredValue {
            handle_index: dictionary.intern(foreign_handle),
            encrypted_value_index: 0,
        },
        rhs: FheExecuteOperand::Scalar {
            value_index: dictionary.intern([0; 32]),
        },
        output_fhe_type: 5,
        output: app.stored_output(&mut dictionary, 1, "mine", &[payer], None, false),
    }];
    let mut remaining = vec![writable(foreign_address), writable(output)];
    if foreign_signs {
        remaining.push(readonly_signer(foreign.key()));
    }
    let ix = fhe_execute_ix(
        payer,
        app.key(),
        host_config,
        FheExecuteArgs {
            account_count: 0,
            dictionary: dictionary.into_entries(),
            steps,
        },
        remaining,
    );
    let mut accounts = execution_accounts(payer, app, host_config, host_config_account);
    accounts.push((foreign.key(), empty_system_account()));
    accounts.push((foreign_address, encrypted_value_account(&foreign_value)));
    accounts.push((output, empty_system_account()));
    mollusk().process_and_validate_instruction(&ix, &accounts, &[expected])
}

#[test]
fn mollusk_fhe_execute_rejects_read_of_a_value_whose_authority_did_not_sign() {
    // Compute over a value is admitted by its authority's signature and nothing else: another
    // program's authority cannot read it by naming it.
    let payer = Pubkey::new_unique();
    read_foreign_value_case(
        payer,
        &App::new(),
        &App::new(),
        false,
        custom_error(host::errors::ZamaHostError::EncryptedValueAccountAuthorityMismatch),
    );
}

#[test]
fn mollusk_fhe_execute_admits_read_signed_by_an_additional_authority() {
    // The same read passes once the value's authority signs as a remaining account — how one
    // program composes over another's values (the batcher reading a token balance). The foreign
    // value stays in its own application: the output belongs to `app` alone.
    let payer = Pubkey::new_unique();
    let app = App::new();
    let foreign = App::new();
    let result = read_foreign_value_case(payer, &app, &foreign, true, Check::success());
    let output = read_encrypted_value_from_result(&result, app.address("mine"));
    assert_eq!(output.program, app.program());
    assert_eq!(output.scope, app.scope);
    assert_eq!(output.leaf_count, 1);
}

#[test]
fn mollusk_fhe_execute_rejects_values_of_two_scopes_under_one_authority() {
    // One execution is one application: reading scope 1 and writing scope 2 through the same
    // default authority is refused, so metering and the deny list see a single `(program, scope)`.
    let payer = Pubkey::new_unique();
    let app = App::new();
    let other_scope = app.sibling_scope(label("other-scope"));
    let (host_config, host_config_account) = host_config_account(payer);
    let handle = handle_for_chain(61, 5);
    let (input_address, input_value) = app.value("in", handle);
    let output = other_scope.address("out");
    let mut dictionary = ExecutionDictionary::default();
    let steps = vec![FheExecuteStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: FheExecuteOperand::StoredValue {
            handle_index: dictionary.intern(handle),
            encrypted_value_index: 0,
        },
        rhs: FheExecuteOperand::Scalar {
            value_index: dictionary.intern([0; 32]),
        },
        output_fhe_type: 5,
        output: other_scope.stored_output(&mut dictionary, 1, "out", &[payer], None, false),
    }];
    let ix = fhe_execute_ix(
        payer,
        app.key(),
        host_config,
        FheExecuteArgs {
            account_count: 0,
            dictionary: dictionary.into_entries(),
            steps,
        },
        vec![writable(input_address), writable(output)],
    );
    let mut accounts = execution_accounts(payer, &app, host_config, host_config_account);
    accounts.push((input_address, encrypted_value_account(&input_value)));
    accounts.push((output, empty_system_account()));
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::FheExecuteMixedScopes,
        )],
    );
}

// ---------------------------------------------------------------------------
// make_handle_public
// ---------------------------------------------------------------------------

#[test]
fn mollusk_make_handle_public_appends_public_decrypt_leaf() {
    let payer = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_account(payer);
    let handle = handle_for_chain(5, 5);
    let (address, value) = app.value("balance", handle);
    let ix = make_handle_public_ix(payer, app.key(), address, host_config, handle, None);
    let accounts = make_public_accounts(
        payer,
        &app,
        address,
        &value,
        host_config,
        host_config_account,
    );
    let result = mollusk().process_and_validate_instruction(&ix, &accounts, &[Check::success()]);
    let updated = read_encrypted_value_from_result(&result, address);
    assert_eq!(updated.leaf_count, 1);
    let expected = zama_solana_acl::public_decrypt_leaf_commitment(address.to_bytes(), 0, handle);
    let mut expected_peaks = Vec::new();
    let mut expected_count = 0u64;
    zama_solana_acl::mmr_append(&mut expected_peaks, &mut expected_count, expected).unwrap();
    assert_eq!(updated.peaks, expected_peaks);
}

#[test]
fn mollusk_make_handle_public_twice_appends_an_equivalent_leaf() {
    let payer = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_account(payer);
    let handle = handle_for_chain(5, 5);
    let (address, value) = app.value("balance", handle);
    let ix = make_handle_public_ix(payer, app.key(), address, host_config, handle, None);
    let accounts = |value: &EncryptedValue| {
        make_public_accounts(
            payer,
            &app,
            address,
            value,
            host_config,
            host_config_account.clone(),
        )
    };
    let sealed = read_encrypted_value_from_result(
        &mollusk().process_and_validate_instruction(&ix, &accounts(&value), &[Check::success()]),
        address,
    );
    // Sealing an already-sealed handle is accepted rather than guarded — INVARIANTS #53.
    let resealed = read_encrypted_value_from_result(
        &mollusk().process_and_validate_instruction(&ix, &accounts(&sealed), &[Check::success()]),
        address,
    );

    assert_eq!(resealed.leaf_count, 2);
    let events = [
        EncryptedValueAccountEvent::MarkedPublic { handle },
        EncryptedValueAccountEvent::MarkedPublic { handle },
    ];
    let shared = resealed.to_shared();
    for leaf_index in 0..resealed.leaf_count {
        let proof = zama_solana_acl::encrypted_value_account::build_verified_proof_from_events(
            address.to_bytes(),
            &events,
            &resealed.peaks,
            resealed.leaf_count,
            leaf_index,
        )
        .unwrap();
        // Both leaves commit to the same (account, handle) fact, so the duplicate authorizes
        // exactly what the first leaf already did and nothing more.
        assert!(
            zama_solana_acl::authorize_public(address.to_bytes(), &shared, handle, &proof).is_ok()
        );
    }
    // What a redundant seal costs is bounded by the MMR shape — one peak per set bit of
    // leaf_count — so no number of reseals can grow the account past MAX_MMR_PEAKS peaks.
    assert_eq!(
        resealed.peaks.len(),
        resealed.leaf_count.count_ones() as usize
    );
    assert!(resealed.peaks.len() <= zama_solana_acl::MAX_MMR_PEAKS);
}

#[test]
fn mollusk_make_handle_public_rejects_wrong_expected_handle() {
    let payer = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_account(payer);
    let handle = handle_for_chain(5, 5);
    let (address, value) = app.value("balance", handle);
    let ix = make_handle_public_ix(
        payer,
        app.key(),
        address,
        host_config,
        handle_for_chain(6, 5),
        None,
    );
    let accounts = make_public_accounts(
        payer,
        &app,
        address,
        &value,
        host_config,
        host_config_account,
    );
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::EncryptedValuePublicHandleMismatch,
        )],
    );
}

#[test]
fn mollusk_make_handle_public_rejects_signer_that_is_not_the_value_authority() {
    // Being allowed on the handle grants decryption, not control: only the value authority seals.
    let payer = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_account(payer);
    let handle = handle_for_chain(5, 5);
    let (address, value) = app.value("balance", handle);
    let ix = make_handle_public_ix(payer, payer, address, host_config, handle, None);
    let accounts = make_public_accounts(
        payer,
        &app,
        address,
        &value,
        host_config,
        host_config_account,
    );
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::EncryptedValueAccountAuthorityMismatch,
        )],
    );
}

// ---------------------------------------------------------------------------
// Deny-list and pause gates
// ---------------------------------------------------------------------------

#[test]
fn mollusk_denied_application_cannot_write_or_seal_but_its_sibling_scope_can() {
    // The deny list keys on `(program, scope)`: a denied application can neither produce a handle
    // (every write is an allow) nor make one public, while another scope of the same program is
    // untouched — its record is simply never initialized.
    let payer = Pubkey::new_unique();
    let denied = App::new();
    let sibling = denied.sibling_scope(label("clean-scope"));
    let (host_config, host_config_account) = deny_enabled_host_config_account(payer);
    let (denied_record, denied_record_account) = deny_scope_record_account(denied.app(), true);
    let sibling_record = host::deny_scope_address(sibling.app()).0;

    let (_, denied_ix, mut accounts) = create_case(
        payer,
        &denied,
        host_config,
        host_config_account.clone(),
        "denied-create",
        &[payer],
        FheExecuteExtras {
            deny_scope_record: Some(denied_record),
            rand_nonce: None,
        },
    );
    accounts.push((denied_record, denied_record_account.clone()));
    mollusk().process_and_validate_instruction(
        &denied_ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::ScopeDenied)],
    );

    let (address, value) = denied.value("denied-seal", handle_for_chain(51, 5));
    let seal_ix = make_handle_public_ix(
        payer,
        denied.key(),
        address,
        host_config,
        value.current_handle,
        Some(denied_record),
    );
    let mut accounts = make_public_accounts(
        payer,
        &denied,
        address,
        &value,
        host_config,
        host_config_account.clone(),
    );
    accounts.push((denied_record, denied_record_account));
    mollusk().process_and_validate_instruction(
        &seal_ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::ScopeDenied)],
    );

    let (sibling_output, sibling_ix, mut accounts) = create_case(
        payer,
        &sibling,
        host_config,
        host_config_account.clone(),
        "sibling-create",
        &[payer],
        FheExecuteExtras {
            deny_scope_record: Some(sibling_record),
            rand_nonce: None,
        },
    );
    accounts.push((sibling_record, empty_system_account()));
    let result =
        mollusk().process_and_validate_instruction(&sibling_ix, &accounts, &[Check::success()]);
    let created = read_encrypted_value_from_result(&result, sibling_output);
    let seal_ix = make_handle_public_ix(
        payer,
        sibling.key(),
        sibling_output,
        host_config,
        created.current_handle,
        Some(sibling_record),
    );
    let mut accounts = make_public_accounts(
        payer,
        &sibling,
        sibling_output,
        &created,
        host_config,
        host_config_account,
    );
    accounts.push((sibling_record, empty_system_account()));
    mollusk().process_and_validate_instruction(&seal_ix, &accounts, &[Check::success()]);
}

#[test]
fn mollusk_deny_list_requires_exactly_the_applications_record() {
    let payer = Pubkey::new_unique();
    let app = App::new();
    let (deny_host_config, deny_host_config_account) = deny_enabled_host_config_account(payer);
    let (other_record, other_record_account) = deny_scope_record_account(App::new().app(), false);

    // Enabled list, no record: fail closed on the missing witness rather than skip the check.
    let (_, ix, accounts) = create_case(
        payer,
        &app,
        deny_host_config,
        deny_host_config_account.clone(),
        "no-witness",
        &[payer],
        FheExecuteExtras::default(),
    );
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::DenyRecordMissing)],
    );

    // Enabled list, another application's record: fhe_execute locates the witness by its
    // canonical key among the remaining accounts, so a foreign record is no witness at all.
    let (_, ix, mut accounts) = create_case(
        payer,
        &app,
        deny_host_config,
        deny_host_config_account.clone(),
        "wrong-witness",
        &[payer],
        FheExecuteExtras {
            deny_scope_record: Some(other_record),
            rand_nonce: None,
        },
    );
    accounts.push((other_record, other_record_account.clone()));
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::DenyRecordMissing)],
    );

    // make_handle_public names the witness slot: a foreign record there is a mismatch.
    let (address, value) = app.value("sealed", handle_for_chain(52, 5));
    let seal_ix = make_handle_public_ix(
        payer,
        app.key(),
        address,
        deny_host_config,
        value.current_handle,
        Some(other_record),
    );
    let mut accounts = make_public_accounts(
        payer,
        &app,
        address,
        &value,
        deny_host_config,
        deny_host_config_account,
    );
    accounts.push((other_record, other_record_account));
    mollusk().process_and_validate_instruction(
        &seal_ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::DenyRecordMismatch,
        )],
    );

    // Disabled list: a record has no meaning and is rejected outright.
    let (host_config, host_config_account) = host_config_account(payer);
    let (record, record_account) = deny_scope_record_account(app.app(), false);
    let seal_ix = make_handle_public_ix(
        payer,
        app.key(),
        address,
        host_config,
        value.current_handle,
        Some(record),
    );
    let mut accounts = make_public_accounts(
        payer,
        &app,
        address,
        &value,
        host_config,
        host_config_account,
    );
    accounts.push((record, record_account));
    mollusk().process_and_validate_instruction(
        &seal_ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::DenyRecordMismatch,
        )],
    );
}

/// A create of `receiver`'s value written under its own signing authority from `app`'s execution
/// (the transfer receipt shape), with `app` also creating a value of its own when `anchored`. The
/// deny list is enabled; `receiver_record` is `receiver`'s deny record as passed.
fn additional_authority_write_case(
    payer: Pubkey,
    app: &App,
    receiver: &App,
    anchored: bool,
    receiver_record: (Pubkey, Account),
) -> (Pubkey, Instruction, Vec<(Pubkey, Account)>) {
    let (host_config, host_config_account) = deny_enabled_host_config_account(payer);
    let app_record = host::deny_scope_address(app.app()).0;
    let mine = app.address("mine");
    let theirs = receiver.address("theirs");
    let mut dictionary = ExecutionDictionary::default();
    let mut remaining = Vec::new();
    let mut accounts = execution_accounts(payer, app, host_config, host_config_account);
    let mut steps = Vec::new();
    if anchored {
        steps.push(FheExecuteStep::TrivialEncrypt {
            plaintext: [3; 32],
            fhe_type: 5,
            output: app.stored_output(&mut dictionary, 0, "mine", &[payer], None, false),
        });
        remaining.push(writable(mine));
        accounts.push((mine, empty_system_account()));
    }
    let theirs_index = remaining.len() as u8;
    let receipt = match receiver.stored_output(
        &mut dictionary,
        theirs_index,
        "theirs",
        &[payer],
        None,
        false,
    ) {
        FheExecuteOutput::StoredValue {
            output_encrypted_value_index,
            output_program_index,
            output_authority_key_index,
            output_scope_index,
            output_label_index,
            output_authority_seeds,
            output_allow_indexes,
            previous_handle_index,
            make_public,
            ..
        } => FheExecuteOutput::StoredValue {
            output_encrypted_value_index,
            output_authority_index: Some(theirs_index + 1),
            output_program_index,
            output_authority_key_index,
            output_scope_index,
            output_label_index,
            output_authority_seeds,
            output_allow_indexes,
            previous_handle_index,
            make_public,
        },
        FheExecuteOutput::Transient => unreachable!(),
    };
    steps.push(FheExecuteStep::TrivialEncrypt {
        plaintext: [4; 32],
        fhe_type: 5,
        output: receipt,
    });
    remaining.push(writable(theirs));
    remaining.push(readonly_signer(receiver.key()));
    remaining.push(readonly(receiver_record.0));
    accounts.push((theirs, empty_system_account()));
    accounts.push((receiver.key(), empty_system_account()));
    accounts.push(receiver_record);
    if anchored {
        accounts.push((app_record, empty_system_account()));
    }
    let ix = fhe_execute_ix_with_extras(
        payer,
        app.key(),
        host_config,
        FheExecuteArgs {
            account_count: 0,
            dictionary: dictionary.into_entries(),
            steps,
        },
        remaining,
        FheExecuteExtras {
            deny_scope_record: anchored.then_some(app_record),
            rand_nonce: None,
        },
    );
    (theirs, ix, accounts)
}

#[test]
fn mollusk_fhe_execute_output_of_an_additional_authority_keeps_its_own_application() {
    // An output whose authority is a second signing authority of another program belongs to that
    // program's application: it is not folded into the execution's application (the meter stays
    // the default authority's), but it is one the execution touches, so its deny record travels
    // with the execution too. This is the transfer receipt: the token writes into the batcher's
    // value inside the token's own execution.
    let payer = Pubkey::new_unique();
    let app = App::new();
    let receiver = App::new();
    let receiver_record = host::deny_scope_address(receiver.app()).0;
    let (theirs, ix, accounts) = additional_authority_write_case(
        payer,
        &app,
        &receiver,
        true,
        (receiver_record, empty_system_account()),
    );
    let result = mollusk().process_and_validate_instruction(&ix, &accounts, &[Check::success()]);
    let created = read_encrypted_value_from_result(&result, theirs);
    assert_eq!(created.program, receiver.program());
    assert_eq!(created.encrypted_value_account_authority, receiver.key());
    assert_eq!(created.scope, receiver.scope);
    assert_eq!(created.leaf_count, 1);
}

#[test]
fn mollusk_fhe_execute_denies_a_write_under_an_additional_authority_into_a_denied_application() {
    // A write is an allow in the value's own application, whoever signed for it: the receipt into
    // a denied application fails whether or not the execution also anchors on its own application.
    let payer = Pubkey::new_unique();
    let app = App::new();
    let denied = App::new();
    for anchored in [true, false] {
        let (_, ix, accounts) = additional_authority_write_case(
            payer,
            &app,
            &denied,
            anchored,
            deny_scope_record_account(denied.app(), true),
        );
        mollusk().process_and_validate_instruction(
            &ix,
            &accounts,
            &[custom_error(host::errors::ZamaHostError::ScopeDenied)],
        );
    }
}

#[test]
fn mollusk_paused_state_blocks_execution_output_and_public_sealing() {
    let payer = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = paused_host_config_account(payer);

    let (address, value) = app.value("pause-seal", handle_for_chain(55, 5));
    let seal_ix = make_handle_public_ix(
        payer,
        app.key(),
        address,
        host_config,
        value.current_handle,
        None,
    );
    let accounts = make_public_accounts(
        payer,
        &app,
        address,
        &value,
        host_config,
        host_config_account.clone(),
    );
    mollusk().process_and_validate_instruction(
        &seal_ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::HostConfigPaused)],
    );

    let (_, execute_ix, accounts) = create_case(
        payer,
        &app,
        host_config,
        host_config_account,
        "pause-execution",
        &[payer],
        FheExecuteExtras::default(),
    );
    mollusk().process_and_validate_instruction(
        &execute_ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::HostConfigPaused)],
    );
}

// ---------------------------------------------------------------------------
// Off-chain reconstruction and proofs
// ---------------------------------------------------------------------------

#[test]
fn mollusk_updated_encrypted_value_account_matches_offchain_reconstruction() {
    let payer = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_account(payer);
    let viewer_a = Pubkey::new_unique();
    let viewer_b = Pubkey::new_unique();
    let (address, value0) = app.value("balance", handle_for_chain(10, 5));

    let value1 = update_with_fhe_execute(
        payer,
        &app,
        host_config,
        host_config_account.clone(),
        address,
        &value0,
        &[viewer_a, viewer_b],
        11,
    );
    let value2 = update_with_fhe_execute(
        payer,
        &app,
        host_config,
        host_config_account,
        address,
        &value1,
        &[viewer_a, viewer_b],
        12,
    );

    // Rebuild the leaves purely from what the two instructions declared — the handle each
    // installed and the keys it allowed — exactly as an off-chain indexer would.
    let mut events = allowed_events(value1.current_handle, &[viewer_a, viewer_b]);
    events.extend(allowed_events(value2.current_handle, &[viewer_a, viewer_b]));
    let reconstructed =
        zama_solana_acl::encrypted_value_account::reconstruct(address.to_bytes(), &events);
    assert!(reconstructed.peaks_match(&value2.peaks, value2.leaf_count));
    assert_eq!(reconstructed.leaf_count, 4); // 2 allows x 2 writes
}

#[test]
fn mollusk_historical_proof_round_trip_after_two_updates() {
    let payer = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_account(payer);
    let viewer = Pubkey::new_unique();
    let other = Pubkey::new_unique();
    let (address, value0) = app.value("balance", handle_for_chain(20, 5));

    let value1 = update_with_fhe_execute(
        payer,
        &app,
        host_config,
        host_config_account.clone(),
        address,
        &value0,
        &[viewer],
        21,
    );
    let value2 = update_with_fhe_execute(
        payer,
        &app,
        host_config,
        host_config_account,
        address,
        &value1,
        &[viewer],
        22,
    );

    let mut events = allowed_events(value1.current_handle, &[viewer]);
    events.extend(allowed_events(value2.current_handle, &[viewer]));
    // Leaf 0 authorizes (handle1, viewer) historically against the live peaks.
    let proof0 = zama_solana_acl::encrypted_value_account::build_verified_proof_from_events(
        address.to_bytes(),
        &events,
        &value2.peaks,
        value2.leaf_count,
        0,
    )
    .unwrap();
    let shared_value2 = value2.to_shared();
    assert!(zama_solana_acl::authorize_historical(
        address.to_bytes(),
        &shared_value2,
        value1.current_handle,
        viewer.to_bytes(),
        &proof0,
    )
    .is_ok());
    // Wrong key rejected.
    assert!(zama_solana_acl::authorize_historical(
        address.to_bytes(),
        &shared_value2,
        value1.current_handle,
        other.to_bytes(),
        &proof0,
    )
    .is_err());
    // Wrong handle rejected.
    assert!(zama_solana_acl::authorize_historical(
        address.to_bytes(),
        &shared_value2,
        value2.current_handle,
        viewer.to_bytes(),
        &proof0,
    )
    .is_err());
}

#[test]
fn mollusk_public_decrypt_proof_has_no_roll_forward() {
    let payer = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_account(payer);
    let viewer = Pubkey::new_unique();
    let handle0 = handle_for_chain(30, 5);
    let (address, value0) = app.value("balance", handle0);

    let make_public_ix =
        make_handle_public_ix(payer, app.key(), address, host_config, handle0, None);
    let accounts0 = make_public_accounts(
        payer,
        &app,
        address,
        &value0,
        host_config,
        host_config_account.clone(),
    );
    let result0 = mollusk().process_and_validate_instruction(
        &make_public_ix,
        &accounts0,
        &[Check::success()],
    );
    let value_public = read_encrypted_value_from_result(&result0, address);

    let final_value = update_with_fhe_execute(
        payer,
        &app,
        host_config,
        host_config_account,
        address,
        &value_public,
        &[viewer],
        31,
    );

    let mut events = vec![EncryptedValueAccountEvent::MarkedPublic { handle: handle0 }];
    events.extend(allowed_events(final_value.current_handle, &[viewer]));
    let proof = zama_solana_acl::encrypted_value_account::build_verified_proof_from_events(
        address.to_bytes(),
        &events,
        &final_value.peaks,
        final_value.leaf_count,
        0,
    )
    .unwrap();
    let shared_final = final_value.to_shared();
    assert!(
        zama_solana_acl::authorize_public(address.to_bytes(), &shared_final, handle0, &proof)
            .is_ok()
    );
    // A proof built for the old handle never authorizes the newer handle: no roll-forward.
    assert!(zama_solana_acl::authorize_public(
        address.to_bytes(),
        &shared_final,
        final_value.current_handle,
        &proof
    )
    .is_err());
}

// ---------------------------------------------------------------------------
// fhe_execute: persistent output create + update through the real CPI-free path
// ---------------------------------------------------------------------------

#[test]
fn mollusk_fhe_execute_creates_persistent_output_from_local_binary_add() {
    let payer = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_account(payer);
    let lhs = handle_for_chain(40, 5);
    let rhs = handle_for_chain(41, 5);
    let (lhs_address, lhs_value) = app.value("lhs", lhs);
    let (rhs_address, rhs_value) = app.value("rhs", rhs);
    let output_address = app.address("sum");

    let mut dictionary = ExecutionDictionary::default();
    let steps = vec![FheExecuteStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: FheExecuteOperand::StoredValue {
            handle_index: dictionary.intern(lhs),
            encrypted_value_index: 0,
        },
        rhs: FheExecuteOperand::StoredValue {
            handle_index: dictionary.intern(rhs),
            encrypted_value_index: 1,
        },
        output_fhe_type: 5,
        output: app.stored_output(&mut dictionary, 2, "sum", &[payer], None, false),
    }];
    let ix = fhe_execute_ix(
        payer,
        app.key(),
        host_config,
        FheExecuteArgs {
            account_count: 0,
            dictionary: dictionary.into_entries(),
            steps,
        },
        vec![
            writable(lhs_address),
            writable(rhs_address),
            writable(output_address),
        ],
    );
    let mut accounts = execution_accounts(payer, &app, host_config, host_config_account);
    accounts.push((lhs_address, encrypted_value_account(&lhs_value)));
    accounts.push((rhs_address, encrypted_value_account(&rhs_value)));
    accounts.push((output_address, empty_system_account()));
    let result = mollusk().process_and_validate_instruction(&ix, &accounts, &[Check::success()]);
    let output = read_encrypted_value_from_result(&result, output_address);
    assert_eq!(output.program, app.program());
    assert_eq!(output.encrypted_value_account_authority, app.key());
    assert_eq!(output.scope, app.scope);
    assert_eq!(output.label, label("sum"));
    // The create seals its one allow.
    assert_eq!(output.leaf_count, 1);
    let mut expected_peaks = Vec::new();
    let mut expected_count = 0u64;
    append_allow_leaves(
        output_address,
        output.current_handle,
        &[payer],
        &mut expected_peaks,
        &mut expected_count,
    );
    assert_eq!(output.peaks, expected_peaks);
}

#[test]
fn mollusk_fhe_execute_updates_persistent_output_with_previous_handle() {
    let payer = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_account(payer);
    let input_handle = handle_for_chain(42, 5);
    let (input_address, input_value) = app.value("in", input_handle);
    let output_handle = handle_for_chain(43, 5);
    let (output_address, output_value) = app.value("out", output_handle);

    let mut dictionary = ExecutionDictionary::default();
    let steps = vec![FheExecuteStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: FheExecuteOperand::StoredValue {
            handle_index: dictionary.intern(input_handle),
            encrypted_value_index: 0,
        },
        rhs: FheExecuteOperand::Scalar {
            value_index: dictionary.intern([0; 32]),
        },
        output_fhe_type: 5,
        output: app.stored_output(
            &mut dictionary,
            1,
            "out",
            &[payer],
            Some(output_handle),
            false,
        ),
    }];
    let ix = fhe_execute_ix(
        payer,
        app.key(),
        host_config,
        FheExecuteArgs {
            account_count: 0,
            dictionary: dictionary.into_entries(),
            steps,
        },
        vec![writable(input_address), writable(output_address)],
    );
    let mut accounts = execution_accounts(payer, &app, host_config, host_config_account);
    accounts.push((input_address, encrypted_value_account(&input_value)));
    accounts.push((output_address, encrypted_value_account(&output_value)));
    let result = mollusk().process_and_validate_instruction(&ix, &accounts, &[Check::success()]);
    let updated_output = read_encrypted_value_from_result(&result, output_address);
    assert_ne!(updated_output.current_handle, output_handle);
    // The update seals one leaf for its one allow.
    assert_eq!(updated_output.leaf_count, 1);
}

// ---------------------------------------------------------------------------
// fhe_execute: narrow produced-public lifecycle execution
// ---------------------------------------------------------------------------

/// The bytes an event-CPI inner instruction starts with: Anchor's event-CPI instruction tag, then
/// the event's own discriminator.
fn event_cpi_prefix<T: Discriminator>() -> Vec<u8> {
    anchor_lang::event::EVENT_IX_TAG_LE
        .iter()
        .copied()
        .chain(T::DISCRIMINATOR.iter().copied())
        .collect()
}

/// Every `T` the host emitted during `result`, read back out of the inner instructions. Every event
/// the program emits travels this way and none of them are logged, so one reader serves both the
/// compute events and the admin ones.
fn emitted_events<T: Discriminator + AnchorDeserialize>(
    result: &mollusk_svm::result::InstructionResult,
) -> Vec<T> {
    let message = result.message.as_ref().expect("compiled Mollusk message");
    let account_keys = message.account_keys();
    let prefix = event_cpi_prefix::<T>();
    result
        .inner_instructions
        .iter()
        .filter_map(|inner| {
            if account_keys.get(inner.instruction.program_id_index as usize) != Some(&host::id()) {
                return None;
            }
            let payload = inner.instruction.data.strip_prefix(prefix.as_slice())?;
            T::deserialize(&mut &*payload).ok()
        })
        .collect()
}

/// Asserts `result` carries exactly one `T`, on an inner instruction addressed to the host with the
/// event authority as its only account, and returns the decoded event.
fn sole_emitted_event<T: Discriminator + AnchorDeserialize>(
    result: &mollusk_svm::result::InstructionResult,
) -> T {
    let prefix = event_cpi_prefix::<T>();
    let message = result.message.as_ref().expect("compiled Mollusk message");
    let account_keys = message.account_keys();
    let carriers: Vec<_> = result
        .inner_instructions
        .iter()
        .filter(|inner| {
            account_keys.get(inner.instruction.program_id_index as usize) == Some(&host::id())
                && inner.instruction.data.starts_with(&prefix)
        })
        .collect();
    assert_eq!(carriers.len(), 1, "expected exactly one event CPI");
    let inner = carriers[0];
    assert_eq!(inner.instruction.accounts.len(), 1);
    assert_eq!(
        account_keys.get(inner.instruction.accounts[0] as usize),
        Some(&event_authority(host::id()))
    );
    let mut events = emitted_events::<T>(result);
    assert_eq!(events.len(), 1);
    events.remove(0)
}

fn created_public_events(
    result: &mollusk_svm::result::InstructionResult,
) -> Vec<host::PublicOutputsProducedEvent> {
    emitted_events(result)
}

fn assert_created_public_batch(
    result: &mollusk_svm::result::InstructionResult,
    expected_outputs: &[(u16, Pubkey)],
) {
    let event = sole_emitted_event::<host::PublicOutputsProducedEvent>(result);
    assert_eq!(event.version, host::EVENT_VERSION);
    assert_eq!(event.outputs.len(), expected_outputs.len());
    for (record, (step_index, encrypted_value)) in event.outputs.iter().zip(expected_outputs.iter())
    {
        assert_eq!(record.step_index, *step_index);
        assert_eq!(record.encrypted_value, *encrypted_value);
        assert_eq!(
            record.output_handle,
            read_encrypted_value_from_result(result, *encrypted_value).current_handle
        );
    }
}

#[test]
fn mollusk_fhe_execute_without_created_public_output_emits_no_lifecycle_batch() {
    let execution = created_public_batch(1, &[]);
    let result = mollusk().process_and_validate_instruction(
        &execution.instruction,
        &execution.accounts,
        &[Check::success()],
    );
    assert!(created_public_events(&result).is_empty());
}

#[test]
fn mollusk_fhe_execute_emits_one_created_public_lifecycle_batch() {
    let execution = created_public_batch(1, &[0]);
    let result = mollusk().process_and_validate_instruction(
        &execution.instruction,
        &execution.accounts,
        &[Check::success()],
    );
    assert_created_public_batch(&result, &execution.outputs);
}

#[test]
fn mollusk_fhe_execute_batches_multiple_created_public_outputs_in_step_order() {
    let execution = created_public_batch(3, &[0, 2]);
    let result = mollusk().process_and_validate_instruction(
        &execution.instruction,
        &execution.accounts,
        &[Check::success()],
    );
    assert_created_public_batch(&result, &execution.outputs);
}

#[test]
fn mollusk_fhe_execute_maximum_created_public_batch_fits_one_cpi() {
    // The largest executable all-created-public execution still emits its DD-038 lifecycle records in
    // exactly one execution CPI. (The full MAX_FHE_EXECUTION_STEPS execution serialization is covered by the
    // event-transport unit test; the host wall itself lives in fhe_execute_boundary.rs.)
    let created_public_steps = zama_fhe::MAX_PERSISTENT_CREATES;
    let execution = created_public_batch(
        created_public_steps,
        &(0..created_public_steps).collect::<Vec<_>>(),
    );
    let result = mollusk().process_and_validate_instruction(
        &execution.instruction,
        &execution.accounts,
        &[Check::success()],
    );
    assert_created_public_batch(&result, &execution.outputs);
}

#[test]
fn mollusk_fhe_execute_wrong_event_authority_fails_without_output() {
    let mut execution = created_public_batch(1, &[0]);
    let wrong_event_authority = Pubkey::new_unique();
    let event_authority_meta = execution
        .instruction
        .accounts
        .iter_mut()
        .find(|meta| meta.pubkey == event_authority(host::id()))
        .expect("event authority account meta");
    event_authority_meta.pubkey = wrong_event_authority;
    execution
        .accounts
        .push((wrong_event_authority, Account::default()));

    let result = mollusk().process_instruction(&execution.instruction, &execution.accounts);
    assert!(result.program_result.is_err());
    assert!(created_public_events(&result).is_empty());
    let output = result.get_account(&execution.outputs[0].1).unwrap();
    assert_eq!(output.owner, system_program::ID);
    assert!(output.data.is_empty());
}

#[test]
fn mollusk_transaction_later_failure_rolls_back_created_public_output() {
    let execution = created_public_batch(1, &[0]);
    let transaction = mollusk().process_transaction_instructions(
        &[execution.instruction.clone(), execution.instruction],
        &execution.accounts,
    );
    assert!(transaction.program_result.is_err());
    let output = transaction.get_account(&execution.outputs[0].1).unwrap();
    assert_eq!(output.owner, system_program::ID);
    assert!(output.data.is_empty());
}

// ---------------------------------------------------------------------------
// HCU admin setters, trust registry, KMS contexts
// ---------------------------------------------------------------------------

/// Exact HCU cost of the fixture's persistent-output execution: `Ge` + `Sub` + `IfThenElse` on
/// euint64 operands (HCULimit.sol parity).
const FIXTURE_BATCH_HCU: u64 = 369_000;
/// Exact HCU cost of the fixture's transient-only execution: a single `Ge` on euint64 operands.
const TRANSIENT_BATCH_HCU: u64 = 152_000;

fn anchor_error(error: anchor_lang::error::ErrorCode) -> Check<'static> {
    anchor_framework_error_check(error)
}

fn unique_app() -> AppScope {
    AppScope {
        program: Pubkey::new_unique(),
        scope: label("app"),
    }
}

/// Like [`host_config_account`] but with the two per-execution HCU limits pre-set.
fn host_config_account_with_hcu_limits(
    admin: Pubkey,
    max_hcu_per_tx: u64,
    max_hcu_depth_per_tx: u64,
) -> (Pubkey, Account) {
    let (key, mut account) = host_config_account(admin);
    let mut config = {
        let mut data = account.data.as_slice();
        HostConfig::try_deserialize(&mut data).expect("valid host config")
    };
    config.max_hcu_per_tx = max_hcu_per_tx;
    config.max_hcu_depth_per_tx = max_hcu_depth_per_tx;
    account.data = serialized_account(config);
    (key, account)
}

/// Like [`host_config_account`] but with the per-app block cap overridden to `cap`. Seeded
/// directly, bypassing the setter ordering guard.
fn host_config_account_with_block_cap(admin: Pubkey, cap: u64) -> (Pubkey, Account) {
    let (key, mut account) = host_config_account(admin);
    let mut config = {
        let mut data = account.data.as_slice();
        HostConfig::try_deserialize(&mut data).expect("valid host config")
    };
    config.hcu_block_cap_per_app = cap;
    account.data = serialized_account(config);
    (key, account)
}

fn read_program_account<T: AccountDeserialize>(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<T> {
    let store = context.account_store.borrow();
    let account = store.get(&address)?;
    if account.owner != host::id() {
        return None;
    }
    let mut data = account.data.as_slice();
    T::try_deserialize(&mut data).ok()
}

fn read_hcu_block_meter(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<host::HcuBlockMeter> {
    read_program_account(context, address)
}

fn read_hcu_trusted_app_record(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<host::HcuTrustedAppRecord> {
    read_program_account(context, address)
}

fn read_kms_context(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<host::KmsContext> {
    read_program_account(context, address)
}

fn read_deny_scope_record(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<host::DenyScopeRecord> {
    read_program_account(context, address)
}

// ---- admin-setter / trust-registry instruction builders ----

fn host_admin_ix(admin: Pubkey, host_config: Pubkey, data: impl InstructionData) -> Instruction {
    anchor_ix(
        host::id(),
        host::accounts::HostAdmin {
            admin,
            host_config,
            event_authority: event_authority(host::id()),
            program: host::id(),
        },
        data,
    )
}

fn set_max_hcu_per_tx_ix(admin: Pubkey, host_config: Pubkey, value: u64) -> Instruction {
    host_admin_ix(
        admin,
        host_config,
        host::instruction::SetMaxHcuPerTx { value },
    )
}

fn set_max_hcu_depth_per_tx_ix(admin: Pubkey, host_config: Pubkey, value: u64) -> Instruction {
    host_admin_ix(
        admin,
        host_config,
        host::instruction::SetMaxHcuDepthPerTx { value },
    )
}

fn set_hcu_block_cap_per_app_ix(admin: Pubkey, host_config: Pubkey, value: u64) -> Instruction {
    host_admin_ix(
        admin,
        host_config,
        host::instruction::SetHcuBlockCapPerApp { value },
    )
}

fn set_coprocessor_signers_ix(
    admin: Pubkey,
    host_config: Pubkey,
    signers: Vec<[u8; 20]>,
    threshold: u8,
) -> Instruction {
    host_admin_ix(
        admin,
        host_config,
        host::instruction::SetCoprocessorSigners { signers, threshold },
    )
}

fn set_deny_scope_ix(
    payer: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    app: AppScope,
    denied: bool,
) -> Instruction {
    anchor_ix(
        host::id(),
        host::accounts::SetDenyScope {
            payer,
            admin,
            host_config,
            deny_scope_record: host::deny_scope_address(app).0,
            system_program: system_program::ID,
            event_authority: event_authority(host::id()),
            program: host::id(),
        },
        host::instruction::SetDenyScope {
            program: app.program,
            scope: app.scope,
            denied,
        },
    )
}

fn define_kms_context_ix(
    admin: Pubkey,
    host_config: Pubkey,
    context_id: [u8; 32],
    signers: Vec<[u8; 20]>,
    thresholds: host::KmsThresholds,
) -> Instruction {
    let kms_context = host::kms_context_address(context_id).0;
    anchor_ix(
        host::id(),
        host::accounts::DefineKmsContext {
            admin,
            host_config,
            kms_context,
            system_program: system_program::ID,
            event_authority: event_authority(host::id()),
            program: host::id(),
        },
        host::instruction::DefineKmsContext {
            context_id,
            signers,
            thresholds,
        },
    )
}

fn destroy_kms_context_ix(admin: Pubkey, host_config: Pubkey, context_id: [u8; 32]) -> Instruction {
    anchor_ix(
        host::id(),
        host::accounts::DestroyKmsContext {
            admin,
            host_config,
            kms_context: host::kms_context_address(context_id).0,
            event_authority: event_authority(host::id()),
            program: host::id(),
        },
        host::instruction::DestroyKmsContext { context_id },
    )
}

fn set_hcu_app_trusted_ix(
    payer: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    app: AppScope,
    trusted: bool,
) -> Instruction {
    set_hcu_app_trusted_ix_with_record(
        payer,
        admin,
        host_config,
        host::hcu_trusted_app_address(app).0,
        app,
        trusted,
    )
}

fn set_hcu_app_trusted_ix_with_record(
    payer: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    record: Pubkey,
    app: AppScope,
    trusted: bool,
) -> Instruction {
    anchor_ix(
        host::id(),
        host::accounts::SetHcuAppTrusted {
            payer,
            admin,
            host_config,
            hcu_trusted_app_record: record,
            system_program: system_program::ID,
            event_authority: event_authority(host::id()),
            program: host::id(),
        },
        host::instruction::SetHcuAppTrusted {
            program: app.program,
            scope: app.scope,
            trusted,
        },
    )
}

// ---- HCU state account fixtures ----

/// A program-owned trust record at the canonical `("hcu-trusted", program, scope)` PDA.
fn hcu_trusted_app_record_account(app: AppScope, trusted: bool) -> (Pubkey, Account) {
    let (key, bump) = host::hcu_trusted_app_address(app);
    (
        key,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(host::HcuTrustedAppRecord {
                program: app.program,
                scope: app.scope,
                trusted,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

/// A program-owned meter at the canonical `("hcu-block-meter", program, scope)` PDA, pre-loaded
/// with `used_hcu` as of `last_seen_slot`.
fn hcu_block_meter_account(app: AppScope, last_seen_slot: u64, used_hcu: u64) -> (Pubkey, Account) {
    let (key, bump) = host::hcu_block_meter_address(app);
    (
        key,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(host::HcuBlockMeter {
                program: app.program,
                scope: app.scope,
                last_seen_slot,
                used_hcu,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

// ---- set_max_hcu_per_tx: block-cap ordering enforced from the other side ----

#[test]
fn mollusk_set_max_hcu_per_tx_rejects_above_block_cap_band() {
    // The block-cap ordering guard from the other side: with the cap in the metering band
    // (500k), raising max_hcu_per_tx above it would make a single legal max-per-tx execution
    // structurally unable to pass the block cap -> rejected, no mutation.
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account_with_block_cap(admin, 500_000);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_max_hcu_per_tx_ix(admin, host_config, 600_000),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockCapBelowMaxPerTx,
        )],
    );
    let config = read_host_config(&context, host_config).expect("config");
    assert_eq!(config.max_hcu_per_tx, u64::MAX);
    assert_eq!(config.hcu_block_cap_per_app, 500_000);

    // At the boundary (== cap) the guard is silent: a total equal to the band cap is accepted.
    context.process_and_validate_instruction(
        &set_max_hcu_per_tx_ix(admin, host_config, 500_000),
        &[Check::success()],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .max_hcu_per_tx,
        500_000
    );
}

#[test]
fn mollusk_set_max_hcu_per_tx_unrestricted_block_cap_accepts_any_total() {
    // With the cap at the unrestricted sentinel (the ship default), the block-cap ordering
    // guard is vacuous and any total is accepted.
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_max_hcu_per_tx_ix(admin, host_config, 20_000_000),
        &[Check::success()],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .max_hcu_per_tx,
        20_000_000
    );
}

#[test]
fn mollusk_set_max_hcu_setters_reject_zero() {
    // u64::MAX is the single "unlimited" sentinel across every HCU knob; 0 is
    // rejected at set time on the per-tx knobs (it would reject every execution),
    // and stays meaningful only on the block cap (ban untrusted apps).
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_max_hcu_per_tx_ix(admin, host_config, 0),
        &[custom_error(
            host::errors::ZamaHostError::HcuLimitZeroReserved,
        )],
    );
    context.process_and_validate_instruction(
        &set_max_hcu_depth_per_tx_ix(admin, host_config, 0),
        &[custom_error(
            host::errors::ZamaHostError::HcuLimitZeroReserved,
        )],
    );
    let config = read_host_config(&context, host_config).expect("config");
    assert_eq!(config.max_hcu_per_tx, u64::MAX);
    assert_eq!(config.max_hcu_depth_per_tx, u64::MAX);
}

#[test]
fn mollusk_set_max_hcu_depth_per_tx_persists_nonzero() {
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_max_hcu_depth_per_tx_ix(admin, host_config, 20_000_000),
        &[Check::success()],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .max_hcu_depth_per_tx,
        20_000_000
    );
}

#[test]
fn mollusk_set_deny_scope_creates_record_and_emits_event() {
    let admin = Pubkey::new_unique();
    let app = unique_app();
    let (host_config, account) = host_config_account(admin);
    let deny_record = host::deny_scope_address(app).0;
    let context = mollusk_execute_context(
        admin,
        vec![(host_config, account), (deny_record, system_account(0))],
    );

    let result = context.process_and_validate_instruction(
        &set_deny_scope_ix(admin, admin, host_config, app, true),
        &[Check::success()],
    );
    let record = read_deny_scope_record(&context, deny_record).expect("deny record");
    assert_eq!(record.program, app.program);
    assert_eq!(record.scope, app.scope);
    assert!(record.denied);
    let event = sole_emitted_event::<host::DenyScopeUpdatedEvent>(&result);
    assert_eq!(event.deny_scope_record, deny_record);
    assert_eq!(event.program, app.program);
    assert_eq!(event.scope, app.scope);
    assert!(event.denied);

    // Re-admitting keeps the record and flips the flag.
    context.process_and_validate_instruction(
        &set_deny_scope_ix(admin, admin, host_config, app, false),
        &[Check::success()],
    );
    assert!(
        !read_deny_scope_record(&context, deny_record)
            .expect("deny record")
            .denied
    );
}

#[test]
fn mollusk_destroy_kms_context_rejects_current() {
    let admin = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_with_context(admin, KMS_CONTEXT_ID);
    let (kms_context, kms_context_acct) = kms_context_account(KMS_CONTEXT_ID);
    let context = mollusk_execute_context(
        admin,
        vec![
            (host_config, host_config_account),
            (kms_context, kms_context_acct),
        ],
    );

    context.process_and_validate_instruction(
        &destroy_kms_context_ix(admin, host_config, KMS_CONTEXT_ID),
        &[custom_error(
            host::errors::ZamaHostError::CurrentKmsContextCannotBeDestroyed,
        )],
    );
}

#[test]
fn mollusk_destroy_kms_context_rejects_already_destroyed() {
    let admin = Pubkey::new_unique();
    let other = canonical_test_context_id(2);
    let (host_config, host_config_account) = host_config_with_context(admin, KMS_CONTEXT_ID);
    let (kms_context, kms_context_acct) =
        kms_context_account_with(other, kms_context_signers(), 1, true);
    let context = mollusk_execute_context(
        admin,
        vec![
            (host_config, host_config_account),
            (kms_context, kms_context_acct),
        ],
    );

    context.process_and_validate_instruction(
        &destroy_kms_context_ix(admin, host_config, other),
        &[custom_error(host::errors::ZamaHostError::InvalidKmsContext)],
    );
}

// ---- set_hcu_block_cap_per_app (admin cap setter) ----

#[test]
fn mollusk_set_hcu_block_cap_metering_band_persists_and_advances_slot() {
    // With the per-execution cap disabled, any positive band value is accepted, persisted, and
    // stamps updated_slot.
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_hcu_block_cap_per_app_ix(admin, host_config, 500_000),
        &[Check::success()],
    );
    let config = read_host_config(&context, host_config).expect("config");
    assert_eq!(config.hcu_block_cap_per_app, 500_000);
    assert_eq!(config.updated_slot, context.mollusk.sysvars.clock.slot);
}

#[test]
fn mollusk_set_hcu_block_cap_at_max_per_tx_boundary_is_accepted() {
    // A band value exactly equal to max_hcu_per_tx is the tightest legal cap: it must be
    // accepted so a single max-cost execution stays possible on a fresh meter.
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account_with_hcu_limits(admin, 20_000_000, u64::MAX);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_hcu_block_cap_per_app_ix(admin, host_config, 20_000_000),
        &[Check::success()],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .hcu_block_cap_per_app,
        20_000_000
    );
}

#[test]
fn mollusk_set_hcu_block_cap_below_max_per_tx_is_rejected() {
    // A band value under max_hcu_per_tx would make a single legal max-per-tx execution
    // structurally impossible (other than the deliberate ban); reject without mutation.
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account_with_hcu_limits(admin, 20_000_000, u64::MAX);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_hcu_block_cap_per_app_ix(admin, host_config, 19_000_000),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockCapBelowMaxPerTx,
        )],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .hcu_block_cap_per_app,
        u64::MAX
    );
}

#[test]
fn mollusk_set_hcu_block_cap_with_max_per_tx_unlimited_accepts_any_band_value() {
    // max_hcu_per_tx == u64::MAX means the per-execution cap is unlimited, so the ordering guard
    // is vacuous and even a tiny band value is accepted.
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_hcu_block_cap_per_app_ix(admin, host_config, 1),
        &[Check::success()],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .hcu_block_cap_per_app,
        1
    );
}

#[test]
fn mollusk_set_hcu_block_cap_ban_and_unrestricted_sentinels_bypass_ordering() {
    // The two sentinels — 0 (ban untrusted apps) and u64::MAX (unrestricted) — are always
    // accepted, even below max_hcu_per_tx, because neither is a metering-band value.
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account_with_hcu_limits(admin, 20_000_000, u64::MAX);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_hcu_block_cap_per_app_ix(admin, host_config, 0),
        &[Check::success()],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .hcu_block_cap_per_app,
        0
    );

    context.process_and_validate_instruction(
        &set_hcu_block_cap_per_app_ix(admin, host_config, u64::MAX),
        &[Check::success()],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .hcu_block_cap_per_app,
        u64::MAX
    );
}

#[test]
fn mollusk_set_hcu_block_cap_is_idempotent() {
    // Setting the current value is a no-op: it does not advance updated_slot (mirrors the
    // other admin setters).
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account_with_block_cap(admin, 750_000);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_hcu_block_cap_per_app_ix(admin, host_config, 750_000),
        &[Check::success()],
    );
    let config = read_host_config(&context, host_config).expect("config");
    assert_eq!(config.hcu_block_cap_per_app, 750_000);
    assert_eq!(config.updated_slot, 0);
}

#[test]
fn mollusk_set_hcu_block_cap_rejects_wrong_admin() {
    // A valid signer that is not the stored admin cannot change the cap.
    let admin = Pubkey::new_unique();
    let wrong_admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(
        admin,
        vec![
            (host_config, account),
            (wrong_admin, funded_system_account()),
        ],
    );

    context.process_and_validate_instruction(
        &set_hcu_block_cap_per_app_ix(wrong_admin, host_config, 500_000),
        &[custom_error(
            host::errors::ZamaHostError::HostConfigAdminMismatch,
        )],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .hcu_block_cap_per_app,
        u64::MAX
    );
}

#[test]
fn mollusk_set_hcu_block_cap_rejects_remaining_accounts() {
    // A trailing account meta is rejected before any mutation.
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    let mut ix = set_hcu_block_cap_per_app_ix(admin, host_config, 500_000);
    ix.accounts
        .push(AccountMeta::new_readonly(Pubkey::new_unique(), false));
    context.process_and_validate_instruction(
        &ix,
        &[custom_error(
            host::errors::ZamaHostError::UnexpectedRemainingAccounts,
        )],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .hcu_block_cap_per_app,
        u64::MAX
    );
}

// ---- set_hcu_app_trusted (admin trust registry) ----

#[test]
fn mollusk_set_hcu_app_trusted_creates_trusted_record() {
    // A first trust-set lazy-creates the canonical record with trusted = true.
    let admin = Pubkey::new_unique();
    let app = unique_app();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    let result = context.process_and_validate_instruction(
        &set_hcu_app_trusted_ix(admin, admin, host_config, app, true),
        &[Check::success()],
    );
    let record = read_hcu_trusted_app_record(&context, host::hcu_trusted_app_address(app).0)
        .expect("record");
    assert_eq!(record.program, app.program);
    assert_eq!(record.scope, app.scope);
    assert!(record.trusted);
    let event = sole_emitted_event::<host::HcuAppTrustUpdatedEvent>(&result);
    assert_eq!(event.program, app.program);
    assert_eq!(event.scope, app.scope);
    assert!(event.trusted);
}

#[test]
fn mollusk_set_hcu_app_trusted_writes_untrusted_false_record() {
    // A well-formed record may carry trusted = false; that is an explicit "metered", not an error.
    let admin = Pubkey::new_unique();
    let app = unique_app();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    // Register trusted, then clear it back to false.
    context.process_and_validate_instruction(
        &set_hcu_app_trusted_ix(admin, admin, host_config, app, true),
        &[Check::success()],
    );
    context.process_and_validate_instruction(
        &set_hcu_app_trusted_ix(admin, admin, host_config, app, false),
        &[Check::success()],
    );
    let record = read_hcu_trusted_app_record(&context, host::hcu_trusted_app_address(app).0)
        .expect("record");
    assert!(!record.trusted);
}

#[test]
fn mollusk_set_hcu_app_trusted_is_idempotent() {
    // Re-setting the current trust value is a no-op and leaves the record intact.
    let admin = Pubkey::new_unique();
    let app = unique_app();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_hcu_app_trusted_ix(admin, admin, host_config, app, true),
        &[Check::success()],
    );
    context.process_and_validate_instruction(
        &set_hcu_app_trusted_ix(admin, admin, host_config, app, true),
        &[Check::success()],
    );
    assert!(
        read_hcu_trusted_app_record(&context, host::hcu_trusted_app_address(app).0)
            .expect("record")
            .trusted
    );
}

#[test]
fn mollusk_set_hcu_app_trusted_rejects_wrong_record_pda() {
    // A record account that is not the canonical ("hcu-trusted", program, scope) PDA is rejected.
    let admin = Pubkey::new_unique();
    let app = unique_app();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    // A record derived for a *different* application is the wrong PDA for `app`.
    let wrong_record = host::hcu_trusted_app_address(unique_app()).0;
    context.process_and_validate_instruction(
        &set_hcu_app_trusted_ix_with_record(admin, admin, host_config, wrong_record, app, true),
        &[custom_error(
            host::errors::ZamaHostError::HcuTrustedAppRecordMismatch,
        )],
    );
    assert!(read_hcu_trusted_app_record(&context, host::hcu_trusted_app_address(app).0).is_none());
}

#[test]
fn mollusk_set_hcu_app_trusted_rejects_wrong_admin() {
    // Only the stored admin may register trust — an app cannot self-trust.
    let admin = Pubkey::new_unique();
    let wrong_admin = Pubkey::new_unique();
    let app = unique_app();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(
        admin,
        vec![
            (host_config, account),
            (wrong_admin, funded_system_account()),
        ],
    );

    context.process_and_validate_instruction(
        &set_hcu_app_trusted_ix(wrong_admin, wrong_admin, host_config, app, true),
        &[custom_error(
            host::errors::ZamaHostError::HostConfigAdminMismatch,
        )],
    );
    assert!(read_hcu_trusted_app_record(&context, host::hcu_trusted_app_address(app).0).is_none());
}

#[test]
fn mollusk_set_hcu_app_trusted_rejects_remaining_accounts() {
    // A trailing account meta is rejected before any write.
    let admin = Pubkey::new_unique();
    let app = unique_app();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    let mut ix = set_hcu_app_trusted_ix(admin, admin, host_config, app, true);
    ix.accounts
        .push(AccountMeta::new_readonly(Pubkey::new_unique(), false));
    context.process_and_validate_instruction(
        &ix,
        &[custom_error(
            host::errors::ZamaHostError::UnexpectedRemainingAccounts,
        )],
    );
    assert!(read_hcu_trusted_app_record(&context, host::hcu_trusted_app_address(app).0).is_none());
}

// ---- coprocessor signer set + threshold (EVM InputVerifier parity) ----

#[test]
fn mollusk_set_coprocessor_signers_rotates_the_set_and_threshold() {
    // The admin setter replaces the registered set + threshold in place.
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account_with_flags(admin, false, false);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    let signers = vec![[0xAAu8; 20], [0xBBu8; 20], [0xCCu8; 20]];
    context.process_and_validate_instruction(
        &set_coprocessor_signers_ix(admin, host_config, signers.clone(), 2),
        &[Check::success()],
    );

    let config = read_host_config(&context, host_config).expect("config");
    assert_eq!(config.active_coprocessor_signers(), signers.as_slice());
    assert_eq!(config.coprocessor_threshold, 2);
}

#[test]
fn mollusk_set_coprocessor_signers_rejects_non_admin() {
    let admin = Pubkey::new_unique();
    let intruder = Pubkey::new_unique();
    let (host_config, account) = host_config_account_with_flags(admin, false, false);
    let context = mollusk_execute_context(intruder, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_coprocessor_signers_ix(intruder, host_config, vec![[0xAAu8; 20]], 1),
        &[custom_error(
            host::errors::ZamaHostError::HostConfigAdminMismatch,
        )],
    );
}

#[test]
fn mollusk_set_coprocessor_signers_rejects_invalid_set() {
    // The setter enforces the same invariants as init (duplicate signer here).
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account_with_flags(admin, false, false);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_coprocessor_signers_ix(admin, host_config, vec![[0xAAu8; 20], [0xAAu8; 20]], 1),
        &[custom_error(
            host::errors::ZamaHostError::DuplicateCoprocessorSigner,
        )],
    );
}

#[test]
fn mollusk_define_kms_context_at_realistic_signer_count() {
    // Exercises the KMS-context definition path at a realistic mainnet-ish size (n=13 signers,
    // public-decrypt threshold 7). `KmsContext::MAX_SIGNERS` (16) bounds the account, so 13 fits.
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account_with_flags(admin, false, false);
    let context_id = canonical_test_context_id(1);
    let kms_context = host::kms_context_address(context_id).0;
    let context = mollusk_execute_context(
        admin,
        vec![(host_config, account), (kms_context, system_account(0))],
    );

    let signers: Vec<[u8; 20]> = (0..13u8).map(|i| [0x30 + i; 20]).collect();
    let thresholds = host::KmsThresholds {
        public_decryption: 7,
        user_decryption: 7,
        kms_gen: 1,
        mpc: 1,
    };
    let result = context.process_and_validate_instruction(
        &define_kms_context_ix(admin, host_config, context_id, signers.clone(), thresholds),
        &[Check::success()],
    );

    let stored = read_kms_context(&context, kms_context).expect("kms context");
    assert_eq!(stored.signers, signers);
    assert_eq!(stored.thresholds.public_decryption, 7);

    // The admin half of the event transport, checked against the real SBF artifact: an admin
    // instruction reaches an off-chain reader through the same event CPI the compute events use, and
    // the payload survives a round trip at a realistic signer count. Nothing is logged.
    let event = sole_emitted_event::<host::NewKmsContextEvent>(&result);
    assert_eq!(event.version, host::EVENT_VERSION);
    assert_eq!(event.kms_context_id, context_id);
    assert_eq!(event.signers, signers);
    assert_eq!(event.public_decryption_threshold, 7);
    assert_eq!(event.user_decryption_threshold, 7);
}

fn default_kms_thresholds() -> host::KmsThresholds {
    host::KmsThresholds {
        public_decryption: 1,
        user_decryption: 1,
        kms_gen: 1,
        mpc: 1,
    }
}

fn run_define_kms_context_expecting(signers: Vec<[u8; 20]>, expected: Check<'static>) {
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account_with_flags(admin, false, false);
    let context_id = canonical_test_context_id(1);
    let kms_context = host::kms_context_address(context_id).0;
    let context = mollusk_execute_context(
        admin,
        vec![(host_config, account), (kms_context, system_account(0))],
    );
    context.process_and_validate_instruction(
        &define_kms_context_ix(
            admin,
            host_config,
            context_id,
            signers,
            default_kms_thresholds(),
        ),
        &[expected],
    );
}

#[test]
fn mollusk_define_kms_context_rejects_empty_signer_set() {
    run_define_kms_context_expecting(
        vec![],
        custom_error(host::errors::ZamaHostError::EmptyKmsContext),
    );
}

#[test]
fn mollusk_define_kms_context_rejects_too_many_signers() {
    let signers: Vec<[u8; 20]> = (1..=17).map(|i| [i; 20]).collect();
    run_define_kms_context_expecting(
        signers,
        custom_error(host::errors::ZamaHostError::TooManyKmsSigners),
    );
}

#[test]
fn mollusk_define_kms_context_rejects_duplicate_signer() {
    run_define_kms_context_expecting(
        vec![[0xAAu8; 20], [0xAAu8; 20]],
        custom_error(host::errors::ZamaHostError::DuplicateKmsSigner),
    );
}

#[test]
fn mollusk_define_kms_context_rejects_zero_signer() {
    run_define_kms_context_expecting(
        vec![[0u8; 20]],
        custom_error(host::errors::ZamaHostError::ZeroKmsSigner),
    );
}

// ---------------------------------------------------------------------------
// FheExecutionFixture: a persistent-output execution for block-cap enforcement
// ---------------------------------------------------------------------------

/// One application (`app`) with two stored inputs, executing under a configurable block cap.
/// The application is what the cap meters: its meter and trust record key on `(program, scope)`,
/// and the fixture's value authority is the default signer of every execution.
struct FheExecutionFixture {
    payer: Pubkey,
    app: App,
    host_config: Pubkey,
    balance_handle: [u8; 32],
    amount_handle: [u8; 32],
    balance_value: Pubkey,
    amount_value: Pubkey,
    output_value: Pubkey,
    context: mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
}

const FIXTURE_OUTPUT: &str = "output-hcu-fixture";

impl FheExecutionFixture {
    /// A fixture whose config carries a per-app block cap; per-execution HCU limits stay off.
    fn with_block_cap(cap: u64) -> Self {
        Self::with_block_cap_keys(cap, Pubkey::new_unique(), Pubkey::new_unique())
    }

    /// Fixed-key variant for cost snapshots: PDA bump searches are part of the
    /// measured compute, so profile addresses must not change between runs.
    fn with_block_cap_keys(cap: u64, payer: Pubkey, program: Pubkey) -> Self {
        let app = App::with_program(program);
        let (host_config, host_config_account) = host_config_account_with_block_cap(payer, cap);
        let balance_handle = handle_for_chain(151, 5);
        let amount_handle = handle_for_chain(152, 5);
        let (balance_value, balance_ev) = app.value("balance-hcu-fixture", balance_handle);
        let (amount_value, amount_ev) = app.value("amount-hcu-fixture", amount_handle);
        let output_value = app.address(FIXTURE_OUTPUT);
        let context = mollusk_execute_context(
            payer,
            vec![
                (host_config, host_config_account),
                (balance_value, encrypted_value_account(&balance_ev)),
                (amount_value, encrypted_value_account(&amount_ev)),
                (app.key(), empty_system_account()),
            ],
        );
        Self {
            payer,
            app,
            host_config,
            balance_handle,
            amount_handle,
            balance_value,
            amount_value,
            output_value,
            context,
        }
    }

    /// The identity both HCU PDAs are keyed on: the application `(program, scope)`.
    fn block_cap_app(&self) -> AppScope {
        self.app.app()
    }

    fn meter_pda(&self) -> Pubkey {
        host::hcu_block_meter_address(self.block_cap_app()).0
    }

    fn trust_pda(&self) -> Pubkey {
        host::hcu_trusted_app_address(self.block_cap_app()).0
    }

    fn seed_account(&self, key: Pubkey, account: Account) {
        self.context.account_store.borrow_mut().insert(key, account);
    }

    fn balance_operand(&self, dictionary: &mut ExecutionDictionary) -> FheExecuteOperand {
        FheExecuteOperand::StoredValue {
            handle_index: dictionary.intern(self.balance_handle),
            encrypted_value_index: 0,
        }
    }

    fn amount_operand(&self, dictionary: &mut ExecutionDictionary) -> FheExecuteOperand {
        FheExecuteOperand::StoredValue {
            handle_index: dictionary.intern(self.amount_handle),
            encrypted_value_index: 1,
        }
    }

    /// `Ge` (euint64 operands → ebool) + `Sub` (euint64) + `IfThenElse` (euint64, persistent
    /// output) — costs exactly `FIXTURE_BATCH_HCU`.
    fn success_batch(&self) -> FheExecuteArgs {
        let mut dictionary = ExecutionDictionary::default();
        let steps = vec![
            FheExecuteStep::Binary {
                op: FheBinaryOpCode::Ge,
                lhs: self.balance_operand(&mut dictionary),
                rhs: self.amount_operand(&mut dictionary),
                output_fhe_type: 0,
                output: FheExecuteOutput::Transient,
            },
            FheExecuteStep::Binary {
                op: FheBinaryOpCode::Sub,
                lhs: self.balance_operand(&mut dictionary),
                rhs: self.amount_operand(&mut dictionary),
                output_fhe_type: 5,
                output: FheExecuteOutput::Transient,
            },
            FheExecuteStep::Ternary {
                op: FheTernaryOpCode::IfThenElse,
                control: FheExecuteOperand::EarlierStep { producer_index: 0 },
                if_true: FheExecuteOperand::EarlierStep { producer_index: 1 },
                if_false: self.balance_operand(&mut dictionary),
                output_fhe_type: 5,
                output: self.app.stored_output(
                    &mut dictionary,
                    2,
                    FIXTURE_OUTPUT,
                    &[self.payer],
                    None,
                    false,
                ),
            },
        ];
        FheExecuteArgs {
            account_count: 0,
            dictionary: dictionary.into_entries(),
            steps,
        }
    }

    /// `args` executed by the fixture application over `remaining`, threading the optional
    /// block-cap and rand accounts.
    fn instruction(
        &self,
        args: FheExecuteArgs,
        remaining: Vec<AccountMeta>,
        meter: Option<Pubkey>,
        trust: Option<Pubkey>,
        rand_nonce: Option<Pubkey>,
    ) -> Instruction {
        let args = FheExecuteArgs {
            account_count: u8::try_from(remaining.len()).expect("remaining accounts fit u8"),
            ..args
        };
        let mut ix = anchor_ix(
            host::id(),
            host::accounts::FheExecute {
                payer: self.payer,
                encrypted_value_account_authority: self.app.key(),
                host_config: self.host_config,
                system_program: system_program::ID,
                hcu_block_meter: meter,
                hcu_trusted_app_record: trust,
                rand_nonce,
                event_authority: event_authority(host::id()),
                program: host::id(),
            },
            host::instruction::FheExecute { args },
        );
        ix.accounts.extend(remaining);
        ix
    }

    /// The standard persistent-output execution, threading the two optional block-cap accounts.
    fn block_cap_instruction(&self, meter: Option<Pubkey>, trust: Option<Pubkey>) -> Instruction {
        self.instruction(
            self.success_batch(),
            vec![
                writable(self.balance_value),
                writable(self.amount_value),
                writable(self.output_value),
            ],
            meter,
            trust,
            None,
        )
    }

    /// An execution at `MAX_FHE_EXECUTION_STEPS`: `Ge` control, alternating `Sub`/`Add` transient
    /// steps, and the persistent `IfThenElse` output. Same accounts and output shape as
    /// `block_cap_instruction`, so the compute-unit delta against the three-op profile
    /// isolates the additional host-side fhe_execute steps.
    fn max_ops_instruction(&self) -> Instruction {
        let mut dictionary = ExecutionDictionary::default();
        let mut steps = vec![FheExecuteStep::Binary {
            op: FheBinaryOpCode::Ge,
            lhs: self.balance_operand(&mut dictionary),
            rhs: self.amount_operand(&mut dictionary),
            output_fhe_type: 0,
            output: FheExecuteOutput::Transient,
        }];
        let last_transient_index = u8::try_from(host::MAX_FHE_EXECUTION_STEPS - 2)
            .expect("MAX_FHE_EXECUTION_STEPS must fit producer indices");
        for index in 1..=last_transient_index {
            let op = if index % 2 == 1 {
                FheBinaryOpCode::Sub
            } else {
                FheBinaryOpCode::Add
            };
            // The first arithmetic step starts from the euint64 balance; later
            // ones chain the previous arithmetic output (step 0 is the ebool
            // control and cannot feed an arithmetic operand).
            let lhs = if index == 1 {
                self.balance_operand(&mut dictionary)
            } else {
                FheExecuteOperand::EarlierStep {
                    producer_index: index - 1,
                }
            };
            steps.push(FheExecuteStep::Binary {
                op,
                lhs,
                rhs: self.amount_operand(&mut dictionary),
                output_fhe_type: 5,
                output: FheExecuteOutput::Transient,
            });
        }
        steps.push(FheExecuteStep::Ternary {
            op: FheTernaryOpCode::IfThenElse,
            control: FheExecuteOperand::EarlierStep { producer_index: 0 },
            if_true: FheExecuteOperand::EarlierStep {
                producer_index: last_transient_index,
            },
            if_false: self.balance_operand(&mut dictionary),
            output_fhe_type: 5,
            output: self.app.stored_output(
                &mut dictionary,
                2,
                FIXTURE_OUTPUT,
                &[self.payer],
                None,
                false,
            ),
        });
        self.instruction(
            FheExecuteArgs {
                account_count: 0,
                dictionary: dictionary.into_entries(),
                steps,
            },
            vec![
                writable(self.balance_value),
                writable(self.amount_value),
                writable(self.output_value),
            ],
            None,
            None,
            None,
        )
    }

    /// A transient-only execution (single step, `Transient` output) — produces no persistent
    /// output; the application comes solely from the values it reads.
    fn transient_only_instruction(
        &self,
        meter: Option<Pubkey>,
        trust: Option<Pubkey>,
    ) -> Instruction {
        let mut dictionary = ExecutionDictionary::default();
        let steps = vec![FheExecuteStep::Binary {
            op: FheBinaryOpCode::Ge,
            lhs: self.balance_operand(&mut dictionary),
            rhs: self.amount_operand(&mut dictionary),
            output_fhe_type: 0,
            output: FheExecuteOutput::Transient,
        }];
        self.instruction(
            FheExecuteArgs {
                account_count: 0,
                dictionary: dictionary.into_entries(),
                steps,
            },
            vec![writable(self.balance_value), writable(self.amount_value)],
            meter,
            trust,
            None,
        )
    }

    /// A persist-nothing execution: `steps` over no stored value at all — no persistent input,
    /// no verified input, no persistent output. Names no application, so under a finite cap
    /// there is nothing to meter (fhevm-internal#1744).
    fn unanchored_instruction(
        &self,
        steps: Vec<FheExecuteStep>,
        meter: Option<Pubkey>,
        trust: Option<Pubkey>,
        rand_nonce: Option<Pubkey>,
    ) -> Instruction {
        self.instruction(
            FheExecuteArgs {
                account_count: 0,
                dictionary: Vec::new(),
                steps,
            },
            Vec::new(),
            meter,
            trust,
            rand_nonce,
        )
    }

    fn persist_nothing_instruction(
        &self,
        meter: Option<Pubkey>,
        trust: Option<Pubkey>,
    ) -> Instruction {
        self.unanchored_instruction(
            vec![FheExecuteStep::TrivialEncrypt {
                plaintext: [7; 32],
                fhe_type: 5,
                output: FheExecuteOutput::Transient,
            }],
            meter,
            trust,
            None,
        )
    }

    /// A single transient `Rand` step, with or without the host rand nonce account.
    fn rand_instruction(&self, rand_nonce: Option<Pubkey>) -> Instruction {
        self.unanchored_instruction(
            vec![FheExecuteStep::Rand {
                fhe_type: 5,
                output: FheExecuteOutput::Transient,
            }],
            None,
            None,
            rand_nonce,
        )
    }

    /// An input-free execution that DOES persist: a single `TrivialEncrypt` creating `name`
    /// under `authority`, paid by `payer` — the legitimate bootstrap/mint path. Returns the
    /// output address and the instruction.
    fn bootstrap_instruction(
        &self,
        payer: Pubkey,
        authority: &App,
        name: &str,
        meter: Option<Pubkey>,
    ) -> (Pubkey, Instruction) {
        let output_value = authority.address(name);
        let mut dictionary = ExecutionDictionary::default();
        let steps = vec![FheExecuteStep::TrivialEncrypt {
            plaintext: [7; 32],
            fhe_type: 5,
            output: authority.stored_output(&mut dictionary, 0, name, &[payer], None, false),
        }];
        let mut ix = anchor_ix(
            host::id(),
            host::accounts::FheExecute {
                payer,
                encrypted_value_account_authority: authority.key(),
                host_config: self.host_config,
                system_program: system_program::ID,
                hcu_block_meter: meter,
                hcu_trusted_app_record: None,
                rand_nonce: None,
                event_authority: event_authority(host::id()),
                program: host::id(),
            },
            host::instruction::FheExecute {
                args: FheExecuteArgs {
                    account_count: 1,
                    dictionary: dictionary.into_entries(),
                    steps,
                },
            },
        );
        ix.accounts.push(writable(output_value));
        (output_value, ix)
    }

    /// Asserts the persistent output was never created, from a returned `InstructionResult`
    /// (works whether or not the output account was ever persisted into `self.context`).
    fn assert_no_output(&self, result: &mollusk_svm::result::InstructionResult) {
        let owner = result
            .resulting_accounts
            .iter()
            .find(|(key, _)| *key == self.output_value)
            .map(|(_, account)| account.owner);
        assert_ne!(
            owner,
            Some(host::id()),
            "output EncryptedValue should not have been created"
        );
    }
}

// ---- fhe_execute block-cap enforcement ----

#[test]
fn mollusk_fhe_execute_unrestricted_cap_none_none_succeeds() {
    // The default (u64::MAX) short-circuits: with neither optional account supplied, the
    // execution binds its persistent output and no meter is ever created or touched.
    let fixture = FheExecutionFixture::with_block_cap(u64::MAX);
    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, None),
        &[Check::success()],
    );
    read_encrypted_value(&fixture.context, fixture.output_value);
    assert!(read_hcu_block_meter(&fixture.context, fixture.meter_pda()).is_none());
}

#[test]
fn mollusk_fhe_execute_unsigned_value_authority_is_rejected() {
    // The value authority must SIGN: it is what admits every read and write of the
    // application's values, and the application the cap meters is derived from them. A
    // supplied-but-unsigned authority is rejected by the account layer, so no caller can name a
    // victim's authority to compute over its values or drain its in-slot budget.
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let mut ix = fixture.block_cap_instruction(Some(fixture.meter_pda()), None);
    let authority = fixture.app.key();
    for meta in ix.accounts.iter_mut() {
        if meta.pubkey == authority {
            meta.is_signer = false;
        }
    }
    let result = fixture.context.process_and_validate_instruction(
        &ix,
        &[anchor_error(
            anchor_lang::error::ErrorCode::AccountNotSigner,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_execute_unrestricted_cap_ignores_supplied_accounts() {
    // Even when both optional accounts are supplied, the unrestricted short-circuit touches
    // neither: a pre-loaded meter is left byte-for-byte unchanged.
    let fixture = FheExecutionFixture::with_block_cap(u64::MAX);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    let (meter_pda, meter_account) = hcu_block_meter_account(fixture.block_cap_app(), slot, 999);
    fixture.seed_account(meter_pda, meter_account);
    let (trust_pda, trust_account) = hcu_trusted_app_record_account(fixture.block_cap_app(), true);
    fixture.seed_account(trust_pda, trust_account);

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), Some(trust_pda)),
        &[Check::success()],
    );
    assert_eq!(
        read_hcu_block_meter(&fixture.context, meter_pda)
            .expect("meter")
            .used_hcu,
        999
    );
}

#[test]
fn mollusk_fhe_execute_ban_cap_zero_untrusted_no_meter_is_rejected() {
    // cap == 0 bans untrusted apps outright — rejected even with no meter supplied, and no
    // persistent output is created.
    let fixture = FheExecutionFixture::with_block_cap(0);
    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, None),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockLimitExceeded,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_execute_ban_cap_zero_untrusted_with_meter_is_rejected_unchanged() {
    // The ban trips before the meter is consulted: a supplied meter is left unchanged.
    let fixture = FheExecutionFixture::with_block_cap(0);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    let (meter_pda, meter_account) = hcu_block_meter_account(fixture.block_cap_app(), slot, 0);
    fixture.seed_account(meter_pda, meter_account);

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockLimitExceeded,
        )],
    );
    assert_eq!(
        read_hcu_block_meter(&fixture.context, meter_pda)
            .expect("meter")
            .used_hcu,
        0
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_execute_ban_cap_zero_trusted_witness_bypasses() {
    // Trusted apps are never banned: with a valid trust witness the execution succeeds even at
    // cap == 0, without any meter.
    let fixture = FheExecutionFixture::with_block_cap(0);
    let (trust_pda, trust_account) = hcu_trusted_app_record_account(fixture.block_cap_app(), true);
    fixture.seed_account(trust_pda, trust_account);

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, Some(trust_pda)),
        &[Check::success()],
    );
    read_encrypted_value(&fixture.context, fixture.output_value);
}

#[test]
fn mollusk_fhe_execute_untrusted_missing_meter_fails_closed() {
    // In the metering band, an untrusted app that forwards neither a meter nor a trust
    // witness is rejected — never silently un-metered. (This is also the CPI rollout hazard:
    // a caller that forwards neither account breaks, rather than bypassing the cap.)
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, None),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockMeterMissing,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_execute_trusted_witness_bypasses_and_creates_no_meter() {
    // A valid trust witness bypasses metering entirely: the execution succeeds with no meter and
    // none is lazily created (contention-free trusted path).
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let (trust_pda, trust_account) = hcu_trusted_app_record_account(fixture.block_cap_app(), true);
    fixture.seed_account(trust_pda, trust_account);

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, Some(trust_pda)),
        &[Check::success()],
    );
    read_encrypted_value(&fixture.context, fixture.output_value);
    assert!(read_hcu_block_meter(&fixture.context, fixture.meter_pda()).is_none());
}

#[test]
fn mollusk_fhe_execute_untrusted_false_witness_requires_meter() {
    // A well-formed record with trusted == false is not a bypass — it falls through to the
    // metering path, so a missing meter still fails closed.
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let (trust_pda, trust_account) = hcu_trusted_app_record_account(fixture.block_cap_app(), false);
    fixture.seed_account(trust_pda, trust_account);

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, Some(trust_pda)),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockMeterMissing,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_execute_wrong_pda_trust_witness_is_rejected() {
    // A witness for a different application (wrong PDA) cannot bypass this application's cap.
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let (other_trust_pda, other_trust_account) = hcu_trusted_app_record_account(unique_app(), true);
    fixture.seed_account(other_trust_pda, other_trust_account);

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, Some(other_trust_pda)),
        &[custom_error(
            host::errors::ZamaHostError::HcuTrustedAppRecordMismatch,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_execute_malformed_trust_witness_is_rejected() {
    // A witness at the canonical PDA but not program-owned (self-made) is rejected — an app
    // cannot forge its own trust. Only an *absent* witness is benign.
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let trust_pda = fixture.trust_pda();
    fixture.seed_account(
        trust_pda,
        Account {
            lamports: 1_000_000,
            data: vec![1u8; 8],
            owner: system_program::ID,
            executable: false,
            rent_epoch: 0,
        },
    );

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, Some(trust_pda)),
        &[custom_error(
            host::errors::ZamaHostError::HcuTrustedAppRecordMismatch,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_execute_wrong_app_meter_is_rejected() {
    // A meter that belongs to a different application (wrong PDA / record identity) cannot be
    // charged for this application.
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    let (other_meter_pda, other_meter_account) = hcu_block_meter_account(unique_app(), slot, 0);
    fixture.seed_account(other_meter_pda, other_meter_account);

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(other_meter_pda), None),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockMeterMismatch,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_execute_squatted_meter_with_data_is_rejected() {
    // A pre-squatted (system-owned, non-empty DATA) account at the meter PDA fails
    // lazy-creation rather than being adopted as a counter. An attacker cannot actually put
    // data on the PDA (allocate needs the PDA's signature), so this guards against a genuinely
    // malformed account.
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let meter_pda = fixture.meter_pda();
    fixture.seed_account(
        meter_pda,
        Account {
            lamports: 1_000_000,
            data: vec![7u8; 16],
            owner: system_program::ID,
            executable: false,
            rent_epoch: 0,
        },
    );

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[custom_error(
            host::errors::ZamaHostError::PdaCreationMismatch,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_execute_prefunded_empty_meter_is_created_not_griefed() {
    // Anti-griefing: the meter PDA address is predictable, so a third party can pre-fund it
    // with a bare lamport transfer (system-owned, EMPTY data) before the app's first metered
    // execution. The fused `create_account` would abort on any pre-funded target
    // (AccountAlreadyInUse) and wedge every metered execution forever; the
    // fund-shortfall+allocate+assign path absorbs the donation and creates the meter normally.
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let meter_pda = fixture.meter_pda();
    fixture.seed_account(meter_pda, system_account(1));

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[Check::success()],
    );
    let meter = read_hcu_block_meter(&fixture.context, meter_pda).expect("meter created");
    assert_eq!(meter.program, fixture.block_cap_app().program);
    assert_eq!(meter.scope, fixture.block_cap_app().scope);
    assert_eq!(meter.used_hcu, FIXTURE_BATCH_HCU);
    // The donated lamport was topped up to at least rent-exempt.
    let lamports = fixture
        .context
        .account_store
        .borrow()
        .get(&meter_pda)
        .expect("meter account")
        .lamports;
    assert!(
        lamports
            >= anchor_lang::prelude::Rent::default()
                .minimum_balance(8 + host::HcuBlockMeter::SPACE)
    );
    read_encrypted_value(&fixture.context, fixture.output_value);
}

#[test]
fn mollusk_fhe_execute_overfunded_empty_meter_is_created_preserving_surplus() {
    // A donation far above rent is equally harmless: no top-up transfer occurs, the meter is
    // created, and the surplus lamports are preserved (the account is simply
    // more-than-rent-exempt).
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let meter_pda = fixture.meter_pda();
    let donated = 5_000_000_000u64;
    fixture.seed_account(meter_pda, system_account(donated));

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[Check::success()],
    );
    let meter = read_hcu_block_meter(&fixture.context, meter_pda).expect("meter created");
    assert_eq!(meter.used_hcu, FIXTURE_BATCH_HCU);
    let lamports = fixture
        .context
        .account_store
        .borrow()
        .get(&meter_pda)
        .expect("meter account")
        .lamports;
    assert_eq!(lamports, donated);
}

#[test]
fn mollusk_fhe_execute_prefunded_output_value_is_created_not_griefed() {
    // The same anti-griefing property for the persistent output path (`create_pda_strict`): its
    // address is predictable too, so a pre-funded (system-owned, empty) donation at the output
    // PDA must not block the execution. Asserted under the unrestricted cap so the meter path is
    // inert and only the output creation is exercised.
    let fixture = FheExecutionFixture::with_block_cap(u64::MAX);
    fixture.seed_account(fixture.output_value, system_account(1));

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, None),
        &[Check::success()],
    );
    read_encrypted_value(&fixture.context, fixture.output_value);
}

#[test]
fn mollusk_fhe_execute_trust_pda_supplied_as_meter_is_rejected() {
    // Role confusion: the trust record's PDA is not the meter PDA, so passing it in the meter
    // slot fails the meter's PDA check.
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let trust_pda = fixture.trust_pda();
    let (_, trust_account) = hcu_trusted_app_record_account(fixture.block_cap_app(), true);
    fixture.seed_account(trust_pda, trust_account);

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(trust_pda), None),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockMeterMismatch,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_execute_over_cap_trips_in_admission_without_output_or_mutation() {
    // An execution whose cost exceeds the cap trips in the read-only admission pass: no persistent
    // output is created and the meter is left unchanged (breach before any write).
    let fixture = FheExecutionFixture::with_block_cap(FIXTURE_BATCH_HCU - 1);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    let (meter_pda, meter_account) = hcu_block_meter_account(fixture.block_cap_app(), slot, 0);
    fixture.seed_account(meter_pda, meter_account);

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockLimitExceeded,
        )],
    );
    assert_eq!(
        read_hcu_block_meter(&fixture.context, meter_pda)
            .expect("meter")
            .used_hcu,
        0
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_execute_charge_accumulates_onto_prior_slot_usage() {
    // Within a slot, a successful charge adds the execution cost onto the meter's existing usage
    // (monotonic; the meter is reused, not reset).
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    let (meter_pda, meter_account) = hcu_block_meter_account(fixture.block_cap_app(), slot, 50_000);
    fixture.seed_account(meter_pda, meter_account);

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[Check::success()],
    );
    let meter = read_hcu_block_meter(&fixture.context, meter_pda).expect("meter");
    assert_eq!(meter.used_hcu, 50_000 + FIXTURE_BATCH_HCU);
    assert_eq!(meter.last_seen_slot, slot);
    read_encrypted_value(&fixture.context, fixture.output_value);
}

#[test]
fn mollusk_fhe_execute_over_cap_with_prior_usage_is_rejected_unchanged() {
    // Prior in-slot usage plus this execution exceeds the cap -> rejected, meter unchanged.
    let fixture = FheExecutionFixture::with_block_cap(150_000);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    let (meter_pda, meter_account) =
        hcu_block_meter_account(fixture.block_cap_app(), slot, 100_000);
    fixture.seed_account(meter_pda, meter_account);

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockLimitExceeded,
        )],
    );
    assert_eq!(
        read_hcu_block_meter(&fixture.context, meter_pda)
            .expect("meter")
            .used_hcu,
        100_000
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_execute_lazy_reset_zeroes_prior_slot_usage() {
    // A meter last written in a different slot is treated as used = 0 for this slot's execution:
    // even a value that would exceed the cap in-slot no longer blocks, and the meter is
    // rewritten at the current slot with just this execution's cost.
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    // Seed as-of a different slot with usage that would exceed the cap if it carried over.
    let (meter_pda, meter_account) =
        hcu_block_meter_account(fixture.block_cap_app(), slot.wrapping_add(1), 490_000);
    fixture.seed_account(meter_pda, meter_account);

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[Check::success()],
    );
    let meter = read_hcu_block_meter(&fixture.context, meter_pda).expect("meter");
    assert_eq!(meter.used_hcu, FIXTURE_BATCH_HCU);
    assert_eq!(meter.last_seen_slot, slot);
}

#[test]
fn mollusk_fhe_execute_clean_first_call_lazy_creates_meter_at_batch_cost() {
    // A first metered execution lazy-creates a program-owned meter initialized to exactly the
    // execution's cost, stamped at the current slot and keyed on this application.
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let meter_pda = fixture.meter_pda();

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[Check::success()],
    );
    let meter = read_hcu_block_meter(&fixture.context, meter_pda).expect("meter created");
    assert_eq!(meter.program, fixture.block_cap_app().program);
    assert_eq!(meter.scope, fixture.block_cap_app().scope);
    assert_eq!(meter.used_hcu, FIXTURE_BATCH_HCU);
    assert_eq!(
        meter.last_seen_slot,
        fixture.context.mollusk.sysvars.clock.slot
    );
    read_encrypted_value(&fixture.context, fixture.output_value);
}

#[test]
fn mollusk_fhe_execute_per_app_meters_are_isolated_under_uniform_cap() {
    // The cap is uniform, but each application has its own meter: one application being maxed
    // out this slot does not throttle a different one, and does not draw down its budget.
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    // A different application is maxed out for the slot.
    let (other_meter_pda, other_meter_account) =
        hcu_block_meter_account(unique_app(), slot, 500_000);
    fixture.seed_account(other_meter_pda, other_meter_account);

    // The fixture app's own execution still succeeds against its own fresh meter.
    let meter_pda = fixture.meter_pda();
    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[Check::success()],
    );
    assert_eq!(
        read_hcu_block_meter(&fixture.context, other_meter_pda)
            .expect("other meter")
            .used_hcu,
        500_000
    );
    assert_eq!(
        read_hcu_block_meter(&fixture.context, meter_pda)
            .expect("meter")
            .used_hcu,
        FIXTURE_BATCH_HCU
    );
}

#[test]
fn mollusk_fhe_execute_same_application_accumulates_across_payers_and_authorities_and_trips_cap() {
    // #1708 regression: the block cap keys on the application `(program, scope)`, so a caller
    // cannot mint a fresh per-slot meter by rotating any account it controls. Two executions in
    // the same slot share the application but vary everything else a caller could vary — a
    // different payer AND a different value authority of the same program, each creating its
    // own output. The cap fits exactly one execution, so the second accumulates onto the same
    // meter and trips the cap rather than getting a fresh budget.
    let fixture = FheExecutionFixture::with_block_cap(FIXTURE_BATCH_HCU);
    let meter_pda = fixture.meter_pda();

    // Execution 1: its own payer, the fixture authority.
    let payer1 = Pubkey::new_unique();
    fixture.seed_account(payer1, funded_system_account());
    let mut ix1 = fixture.block_cap_instruction(Some(meter_pda), None);
    for meta in ix1.accounts.iter_mut() {
        if meta.pubkey == fixture.payer {
            meta.pubkey = payer1;
        }
    }
    fixture
        .context
        .process_and_validate_instruction(&ix1, &[Check::success()]);
    assert_eq!(
        read_hcu_block_meter(&fixture.context, meter_pda)
            .expect("meter created")
            .used_hcu,
        FIXTURE_BATCH_HCU
    );

    // Execution 2: a different payer and a second value authority of the same program and
    // scope, same slot. Even its 32-HCU bootstrap does not fit the exhausted budget.
    let payer2 = Pubkey::new_unique();
    let second_authority = fixture.app.sibling_authority(Pubkey::new_unique());
    fixture.seed_account(payer2, funded_system_account());
    fixture.seed_account(second_authority.key(), empty_system_account());
    let (out2, ix2) = fixture.bootstrap_instruction(
        payer2,
        &second_authority,
        "execution-2-out",
        Some(meter_pda),
    );
    let result = fixture.context.process_and_validate_instruction(
        &ix2,
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockLimitExceeded,
        )],
    );
    // The tripped execution accumulated onto the SAME meter (no fresh budget) and, breaching in the
    // read-only admission pass, left it unchanged and created no output.
    assert_eq!(
        read_hcu_block_meter(&fixture.context, meter_pda)
            .expect("meter")
            .used_hcu,
        FIXTURE_BATCH_HCU
    );
    let out2_owner = result
        .resulting_accounts
        .iter()
        .find(|(key, _)| *key == out2)
        .map(|(_, account)| account.owner);
    assert_ne!(out2_owner, Some(host::id()));
}

#[test]
fn mollusk_fhe_execute_extra_remaining_account_still_rejected_with_block_cap() {
    // The two block-cap accounts are named context accounts, not remaining_accounts, so a
    // trailing extra account is still rejected — since W7 by the execution's self-described
    // `account_count` (DD-033), before the per-account usage checks.
    let fixture = FheExecutionFixture::with_block_cap(u64::MAX);
    let mut ix = fixture.block_cap_instruction(None, None);
    ix.accounts
        .push(AccountMeta::new_readonly(Pubkey::new_unique(), false));
    fixture.context.process_and_validate_instruction(
        &ix,
        &[custom_error(
            host::errors::ZamaHostError::FheExecuteAccountCountMismatch,
        )],
    );
}

#[test]
fn mollusk_fhe_execute_transient_only_batch_is_metered_via_the_values_it_reads() {
    // A transient-only execution (all Transient outputs) creates no persistent record, but the
    // stored values it reads name the application, so the execution is still charged in full.
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let meter_pda = fixture.meter_pda();
    let result = fixture.context.process_and_validate_instruction(
        &fixture.transient_only_instruction(Some(meter_pda), None),
        &[Check::success()],
    );
    // No persistent output was produced...
    fixture.assert_no_output(&result);
    // ...yet the execution accrued onto the application's meter.
    let meter = read_hcu_block_meter(&fixture.context, meter_pda).expect("meter created");
    assert_eq!(meter.program, fixture.block_cap_app().program);
    assert_eq!(meter.scope, fixture.block_cap_app().scope);
    assert_eq!(meter.used_hcu, TRANSIENT_BATCH_HCU);
}

#[test]
fn mollusk_fhe_execute_finite_cap_rejects_persist_nothing_batch() {
    // fhevm-internal#1744: under a finite cap, an execution that touches no stored value and no
    // verified input names no application — there would be nothing to meter, and a caller could
    // compute for free. Rejected in preflight, before compute, so no meter is created even
    // though one is supplied.
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let meter_pda = fixture.meter_pda();
    fixture.context.process_and_validate_instruction(
        &fixture.persist_nothing_instruction(Some(meter_pda), None),
        &[custom_error(
            host::errors::ZamaHostError::FheExecuteUnanchoredUnderBlockCap,
        )],
    );
    assert!(read_hcu_block_meter(&fixture.context, meter_pda).is_none());
}

#[test]
fn mollusk_fhe_execute_finite_cap_allows_input_free_persistent_output_bootstrap() {
    // The bootstrap/mint path (trivial-encrypt -> persistent output) is input-free but creates
    // a value of the application, so it anchors the execution and stays legal under a finite cap.
    let fixture = FheExecutionFixture::with_block_cap(500_000);
    let meter_pda = fixture.meter_pda();
    let (output_value, ix) = fixture.bootstrap_instruction(
        fixture.payer,
        &fixture.app,
        "input-free-bootstrap",
        Some(meter_pda),
    );
    fixture
        .context
        .process_and_validate_instruction(&ix, &[Check::success()]);
    read_encrypted_value(&fixture.context, output_value);
    // The execution WAS metered onto the application (a single euint64 TrivialEncrypt, 32 HCU
    // in HCULimit.sol `checkHCUForTrivialEncrypt`).
    const TRIVIAL_ENCRYPT_EUINT64_HCU: u64 = 32;
    let meter = read_hcu_block_meter(&fixture.context, meter_pda).expect("meter created");
    assert_eq!(meter.program, fixture.block_cap_app().program);
    assert_eq!(meter.used_hcu, TRIVIAL_ENCRYPT_EUINT64_HCU);
}

#[test]
fn mollusk_fhe_execute_deactivated_cap_allows_persist_nothing_batch() {
    // Under the ship default (u64::MAX) the persist-nothing rejection short-circuits, so behavior
    // is unchanged wherever a finite cap is not deployed.
    let fixture = FheExecutionFixture::with_block_cap(u64::MAX);
    fixture.context.process_and_validate_instruction(
        &fixture.persist_nothing_instruction(None, None),
        &[Check::success()],
    );
}

#[test]
fn mollusk_fhe_execute_meter_accumulation_overflow_fails_closed() {
    // Accumulating this execution onto a near-max in-slot usage would overflow u64. The checked
    // add must fail closed (reject, never wrap), and the meter is left unchanged. The cap is a
    // huge band value so it is the overflow — not the cap comparison — that trips.
    let fixture = FheExecutionFixture::with_block_cap(u64::MAX - 1);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    let (meter_pda, meter_account) =
        hcu_block_meter_account(fixture.block_cap_app(), slot, u64::MAX - 1_000);
    fixture.seed_account(meter_pda, meter_account);

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockLimitExceeded,
        )],
    );
    assert_eq!(
        read_hcu_block_meter(&fixture.context, meter_pda)
            .expect("meter")
            .used_hcu,
        u64::MAX - 1_000
    );
    fixture.assert_no_output(&result);
}

// ---- fhe_execute rand nonce ----

#[test]
fn mollusk_fhe_execute_rand_without_nonce_account_is_rejected() {
    // Rand seeds derive from the host nonce; an execution with a Rand step that does not carry
    // the nonce account has no seed source and is rejected before compute.
    let fixture = FheExecutionFixture::with_block_cap(u64::MAX);
    fixture.context.process_and_validate_instruction(
        &fixture.rand_instruction(None),
        &[custom_error(
            host::errors::ZamaHostError::FheExecuteRandNonceMissing,
        )],
    );
}

#[test]
fn mollusk_fhe_execute_nonce_account_without_rand_is_rejected() {
    // The nonce is a write lock shared by every rand execution on the chain: an execution that
    // draws no randomness may not take it, or it would serialize against them for nothing.
    let fixture = FheExecutionFixture::with_block_cap(u64::MAX);
    let (nonce, nonce_account) = rand_nonce_account(0);
    fixture.seed_account(nonce, nonce_account);
    let mut ix = fixture.block_cap_instruction(None, None);
    ix.accounts = fixture
        .instruction(
            fixture.success_batch(),
            vec![
                writable(fixture.balance_value),
                writable(fixture.amount_value),
                writable(fixture.output_value),
            ],
            None,
            None,
            Some(nonce),
        )
        .accounts;
    fixture.context.process_and_validate_instruction(
        &ix,
        &[custom_error(
            host::errors::ZamaHostError::InvalidFheExecuteAccount,
        )],
    );
}

#[test]
fn mollusk_fhe_execute_rand_consumes_the_nonce_and_never_repeats_a_seed() {
    // Two byte-identical rand executions in one slot draw different seeds because each consumes
    // the nonce, and the nonce is what the seed commits to.
    let fixture = FheExecutionFixture::with_block_cap(u64::MAX);
    let (nonce, nonce_account) = rand_nonce_account(7);
    fixture.seed_account(nonce, nonce_account);
    let ix = fixture.rand_instruction(Some(nonce));

    let first = fixture
        .context
        .process_and_validate_instruction(&ix, &[Check::success()]);
    let second = fixture
        .context
        .process_and_validate_instruction(&ix, &[Check::success()]);

    let first_seeds = sole_emitted_event::<host::FheExecuteRandomSeedsEvent>(&first);
    let second_seeds = sole_emitted_event::<host::FheExecuteRandomSeedsEvent>(&second);
    assert_eq!(first_seeds.version, host::EVENT_VERSION);
    assert_eq!(first_seeds.seeds.len(), 1);
    assert_eq!(first_seeds.seeds[0].step_index, 0);
    assert_eq!(second_seeds.seeds.len(), 1);
    assert_ne!(first_seeds.seeds[0].seed, second_seeds.seeds[0].seed);
    let stored: host::RandNonce =
        read_program_account(&fixture.context, nonce).expect("rand nonce");
    assert_eq!(stored.nonce, 9);
}

#[test]
fn mollusk_fhe_execute_rand_rejects_non_canonical_nonce_account() {
    // Only the host's own nonce PDA is accepted: a caller-supplied account with the right data
    // at another address is not a nonce.
    let fixture = FheExecutionFixture::with_block_cap(u64::MAX);
    let (_, nonce_account) = rand_nonce_account(0);
    let impostor = Pubkey::new_unique();
    fixture.seed_account(impostor, nonce_account);
    let result = fixture
        .context
        .process_instruction(&fixture.rand_instruction(Some(impostor)));
    assert!(result.program_result.is_err());
}

// ---------------------------------------------------------------------------
// verify_public_decrypt: stateless pull-oracle verifier (fhevm-internal#1704)
// ---------------------------------------------------------------------------

const KMS_CONTEXT_ID: [u8; 32] = {
    let mut id = [0u8; 32];
    id[31] = 1;
    id
};

fn kms_context_signers() -> Vec<[u8; 20]> {
    vec![signing::secp_evm_address(&signing::kms_signing_key())]
}

/// Host config with an active KMS context id and the fixtures' gateway EIP-712 domain.
fn host_config_with_context(admin: Pubkey, context_id: [u8; 32]) -> (Pubkey, Account) {
    let (host_config, bump) = host::host_config_address();
    (
        host_config,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(HostConfig {
                admin,
                chain_id: host::SOLANA_POC_CHAIN_ID,
                gateway_chain_id: GATEWAY_CHAIN_ID,
                input_verification_contract: INPUT_VERIFICATION_CONTRACT,
                coprocessor_signers: host::pack_coprocessor_signers(&[[0x11u8; 20]]),
                coprocessor_signer_count: 1,
                coprocessor_threshold: 1,
                decryption_contract: DECRYPTION_CONTRACT,
                current_kms_context_id: context_id,
                paused: false,
                grant_deny_list_enabled: false,
                max_hcu_per_tx: u64::MAX,
                max_hcu_depth_per_tx: u64::MAX,
                hcu_block_cap_per_app: u64::MAX,
                updated_slot: 0,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

/// Canonical `KmsContext` PDA for `context_id`, with the given signer set / public-decrypt threshold.
fn kms_context_account_with(
    context_id: [u8; 32],
    signers: Vec<[u8; 20]>,
    public_decryption: u8,
    destroyed: bool,
) -> (Pubkey, Account) {
    let (address, bump) = host::kms_context_address(context_id);
    (
        address,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(host::KmsContext {
                context_id,
                signers,
                thresholds: host::KmsThresholds {
                    public_decryption,
                    user_decryption: 1,
                    kms_gen: 1,
                    mpc: 1,
                },
                destroyed,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

/// The canonical single-signer, threshold-1 KMS context the fixtures pin.
fn kms_context_account(context_id: [u8; 32]) -> (Pubkey, Account) {
    kms_context_account_with(context_id, kms_context_signers(), 1, false)
}

#[allow(clippy::too_many_arguments)]
fn verify_public_decrypt_ix(
    host_config: Pubkey,
    kms_context: Pubkey,
    encrypted_value: Pubkey,
    handle: [u8; 32],
    cleartext: [u8; 32],
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
    proof: host::instructions::MmrInclusionProof,
) -> Instruction {
    anchor_ix(
        host::id(),
        host::accounts::VerifyPublicDecrypt {
            host_config,
            kms_context,
            encrypted_value,
        },
        host::instruction::VerifyPublicDecrypt {
            handle,
            cleartext,
            signatures,
            extra_data,
            proof,
        },
    )
}

fn mmr_inclusion_proof(proof: zama_solana_acl::MmrProof) -> host::instructions::MmrInclusionProof {
    host::instructions::MmrInclusionProof {
        leaf_index: proof.leaf_index,
        siblings: proof.siblings,
    }
}

/// Seals `handle` public on a fresh value of `app` via `make_handle_public`, returning the value's
/// address, the resulting on-chain value, and a verified inclusion proof for the sealed leaf.
fn seal_public_leaf(
    payer: Pubkey,
    app: &App,
    host_config: Pubkey,
    host_config_account: &Account,
    handle: [u8; 32],
) -> (
    Pubkey,
    EncryptedValue,
    host::instructions::MmrInclusionProof,
) {
    let (address, value) = app.value("balance", handle);
    let seal_ix = make_handle_public_ix(payer, app.key(), address, host_config, handle, None);
    let seal_accounts = make_public_accounts(
        payer,
        app,
        address,
        &value,
        host_config,
        host_config_account.clone(),
    );
    let sealed = read_encrypted_value_from_result(
        &mollusk().process_and_validate_instruction(&seal_ix, &seal_accounts, &[Check::success()]),
        address,
    );
    let events = [EncryptedValueAccountEvent::MarkedPublic { handle }];
    let proof = mmr_inclusion_proof(
        zama_solana_acl::encrypted_value_account::build_verified_proof_from_events(
            address.to_bytes(),
            &events,
            &sealed.peaks,
            sealed.leaf_count,
            0,
        )
        .unwrap(),
    );
    (address, sealed, proof)
}

/// A KMS public-decrypt certificate for `handle` -> 4242 under the fixtures' gateway domain.
fn public_decrypt_cert(handle: [u8; 32], extra_data: &[u8]) -> ([u8; 32], Vec<[u8; 65]>) {
    let cleartext = zama_solana_test_kit::u256_be(4242);
    let signatures = signing::kms_public_decrypt_cert(
        handle,
        cleartext,
        GATEWAY_CHAIN_ID,
        &DECRYPTION_CONTRACT,
        extra_data,
    );
    (cleartext, signatures)
}

/// `handle ++ cleartext ++ context_id`: what a successful verification returns.
fn expected_return_data(handle: [u8; 32], cleartext: [u8; 32], context_id: [u8; 32]) -> Vec<u8> {
    let mut expected = handle.to_vec();
    expected.extend_from_slice(&cleartext);
    expected.extend_from_slice(&context_id);
    expected
}

#[test]
fn mollusk_verify_public_decrypt_returns_handle_and_cleartext() {
    let admin = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_with_context(admin, KMS_CONTEXT_ID);
    let (kms_context, kms_context_acct) = kms_context_account(KMS_CONTEXT_ID);
    let handle = handle_for_chain(5, 5);
    let (address, sealed, proof) =
        seal_public_leaf(admin, &app, host_config, &host_config_account, handle);

    let extra_data = vec![0x00u8]; // v0: bind to the current context
    let (cleartext, signatures) = public_decrypt_cert(handle, &extra_data);
    let ix = verify_public_decrypt_ix(
        host_config,
        kms_context,
        address,
        handle,
        cleartext,
        signatures,
        extra_data,
        proof,
    );
    let accounts = vec![
        (host_config, host_config_account),
        (kms_context, kms_context_acct),
        (address, encrypted_value_account(&sealed)),
    ];
    // v0 extra_data resolves to the current context, so return_data carries the current id.
    let expected = expected_return_data(handle, cleartext, KMS_CONTEXT_ID);
    let result = mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[Check::success(), Check::return_data(&expected)],
    );
    // return_data is exactly handle ++ cleartext ++ context_id, and nothing was written back.
    assert_eq!(result.return_data, expected);
    let unchanged = read_encrypted_value_from_result(&result, address);
    assert_eq!(unchanged.current_handle, sealed.current_handle);
    assert_eq!(unchanged.leaf_count, sealed.leaf_count);
    assert_eq!(unchanged.peaks, sealed.peaks);
}

/// Rotates the current context `KMS_CONTEXT_ID` -> `KMS_CONTEXT_ID + 1` through the real
/// `define_kms_context`, returning the rotated host config (now current = 2), the new context's
/// address, and its account. Context 1's account is untouched by rotation (a new PDA is created for
/// 2), so callers keep verifying under it.
fn rotate_to_next_context(
    admin: Pubkey,
    host_config: Pubkey,
    host_config_account: Account,
) -> (Account, Pubkey, Account) {
    let next_context_id = canonical_test_context_id(2);
    let (next_kms_context, _) = host::kms_context_address(next_context_id);
    let define_ix = define_kms_context_ix(
        admin,
        host_config,
        next_context_id,
        kms_context_signers(),
        default_kms_thresholds(),
    );
    let define_accounts = vec![
        (system_program::ID, system_program_account()),
        (admin, funded_system_account()),
        (host_config, host_config_account),
        (next_kms_context, empty_system_account()),
        (event_authority(host::id()), Account::default()),
    ];
    let define_result = mollusk().process_and_validate_instruction(
        &define_ix,
        &define_accounts,
        &[Check::success()],
    );
    let rotated_host_config = define_result
        .get_account(&host_config)
        .expect("rotated host config")
        .clone();
    let next_kms_context_acct = define_result
        .get_account(&next_kms_context)
        .expect("new kms context")
        .clone();
    (rotated_host_config, next_kms_context, next_kms_context_acct)
}

#[test]
fn mollusk_verify_public_decrypt_accepts_live_rotated_out_context() {
    // EVM-parity liveness: a cert minted under context 1 stays verifiable after the operator rotates
    // to context 2, because context 1's account persists and is not destroyed. return_data carries 1.
    let admin = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_with_context(admin, KMS_CONTEXT_ID);
    let handle = handle_for_chain(5, 5);
    let (address, sealed, proof) =
        seal_public_leaf(admin, &app, host_config, &host_config_account, handle);

    // Rotate 1 -> 2; context 1 is now the old (but still live) context.
    let (rotated_host_config, _next_kms_context, _next_acct) =
        rotate_to_next_context(admin, host_config, host_config_account);
    let (kms_context, kms_context_acct) = kms_context_account(KMS_CONTEXT_ID);

    let extra_data = signing::context_extra_data_v1(KMS_CONTEXT_ID); // commit the old context id
    let (cleartext, signatures) = public_decrypt_cert(handle, &extra_data);
    let ix = verify_public_decrypt_ix(
        host_config,
        kms_context,
        address,
        handle,
        cleartext,
        signatures,
        extra_data,
        proof,
    );
    let accounts = vec![
        (host_config, rotated_host_config),
        (kms_context, kms_context_acct),
        (address, encrypted_value_account(&sealed)),
    ];
    let expected = expected_return_data(handle, cleartext, KMS_CONTEXT_ID);
    let result = mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[Check::success(), Check::return_data(&expected)],
    );
    assert_eq!(result.return_data, expected);
}

#[test]
fn mollusk_verify_public_decrypt_rejects_after_destroy() {
    // The revocation lever end to end: rotate 1 -> 2, then `destroy_kms_context(1)`. The same cert
    // that verified while 1 was live now fails closed — destroy invalidates every outstanding 1-cert.
    let admin = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_with_context(admin, KMS_CONTEXT_ID);
    let handle = handle_for_chain(5, 5);
    let (address, sealed, proof) =
        seal_public_leaf(admin, &app, host_config, &host_config_account, handle);

    // Rotate 1 -> 2 so context 1 is no longer current and may be destroyed.
    let (rotated_host_config, _next_kms_context, _next_acct) =
        rotate_to_next_context(admin, host_config, host_config_account);
    let (kms_context, kms_context_acct) = kms_context_account(KMS_CONTEXT_ID);

    // Destroy context 1 through the real `destroy_kms_context`.
    let destroy_accounts = vec![
        (admin, funded_system_account()),
        (host_config, rotated_host_config.clone()),
        (kms_context, kms_context_acct),
        (event_authority(host::id()), Account::default()),
    ];
    let destroy_result = mollusk().process_and_validate_instruction(
        &destroy_kms_context_ix(admin, host_config, KMS_CONTEXT_ID),
        &destroy_accounts,
        &[Check::success()],
    );
    let destroyed_kms_context_acct = destroy_result
        .get_account(&kms_context)
        .expect("destroyed kms context")
        .clone();

    let extra_data = signing::context_extra_data_v1(KMS_CONTEXT_ID);
    let (cleartext, signatures) = public_decrypt_cert(handle, &extra_data);
    let ix = verify_public_decrypt_ix(
        host_config,
        kms_context,
        address,
        handle,
        cleartext,
        signatures,
        extra_data,
        proof,
    );
    let accounts = vec![
        (host_config, rotated_host_config),
        (kms_context, destroyed_kms_context_acct),
        (address, encrypted_value_account(&sealed)),
    ];
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::InvalidKmsContext)],
    );
}

#[test]
fn mollusk_verify_public_decrypt_rejects_destroyed_context() {
    // A destroyed context account supplied directly (canonical PDA, cert commits its id) is rejected
    // on the `!destroyed` check.
    let admin = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_with_context(admin, KMS_CONTEXT_ID);
    let (kms_context, kms_context_acct) =
        kms_context_account_with(KMS_CONTEXT_ID, kms_context_signers(), 1, true);
    let handle = handle_for_chain(5, 5);
    let (address, sealed, proof) =
        seal_public_leaf(admin, &app, host_config, &host_config_account, handle);

    let extra_data = vec![0x00u8];
    let (cleartext, signatures) = public_decrypt_cert(handle, &extra_data);
    let ix = verify_public_decrypt_ix(
        host_config,
        kms_context,
        address,
        handle,
        cleartext,
        signatures,
        extra_data,
        proof,
    );
    let accounts = vec![
        (host_config, host_config_account),
        (kms_context, kms_context_acct),
        (address, encrypted_value_account(&sealed)),
    ];
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::InvalidKmsContext)],
    );
}

#[test]
fn mollusk_verify_public_decrypt_rejects_context_account_mismatch() {
    // Adversarial: the cert commits context id 1, but the caller supplies a DIFFERENT live context's
    // account (context 2's canonical PDA). The cert-id -> canonical-PDA binding fails: context 2's
    // PDA is not the canonical PDA for id 1, so verification is rejected.
    let admin = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_with_context(admin, KMS_CONTEXT_ID);
    let handle = handle_for_chain(5, 5);
    let (address, sealed, proof) =
        seal_public_leaf(admin, &app, host_config, &host_config_account, handle);

    // Rotate 1 -> 2 to obtain a real, live context-2 account at its canonical PDA.
    let (rotated_host_config, next_kms_context, next_kms_context_acct) =
        rotate_to_next_context(admin, host_config, host_config_account);

    // Cert commits id 1, but we pass context 2's account.
    let extra_data = signing::context_extra_data_v1(KMS_CONTEXT_ID);
    let (cleartext, signatures) = public_decrypt_cert(handle, &extra_data);
    let ix = verify_public_decrypt_ix(
        host_config,
        next_kms_context,
        address,
        handle,
        cleartext,
        signatures,
        extra_data,
        proof,
    );
    let accounts = vec![
        (host_config, rotated_host_config),
        (next_kms_context, next_kms_context_acct),
        (address, encrypted_value_account(&sealed)),
    ];
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::InvalidKmsContext)],
    );
}

#[test]
fn mollusk_verify_public_decrypt_rejects_nonexistent_context_id() {
    // The cert commits a context id that has no on-chain account. The canonical PDA for that id has
    // no `KmsContext` (a system-owned placeholder stands in), so Anchor's account loader rejects it
    // before the handler: there is no live signer set to verify against.
    let admin = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_with_context(admin, KMS_CONTEXT_ID);
    let handle = handle_for_chain(5, 5);
    let (address, sealed, proof) =
        seal_public_leaf(admin, &app, host_config, &host_config_account, handle);

    let nonexistent_context_id = canonical_test_context_id(99);
    let (nonexistent_kms_context, _) = host::kms_context_address(nonexistent_context_id);
    let extra_data = signing::context_extra_data_v1(nonexistent_context_id);
    let (cleartext, signatures) = public_decrypt_cert(handle, &extra_data);
    let ix = verify_public_decrypt_ix(
        host_config,
        nonexistent_kms_context,
        address,
        handle,
        cleartext,
        signatures,
        extra_data,
        proof,
    );
    let accounts = vec![
        (host_config, host_config_account),
        // No KmsContext exists at the canonical PDA for id 99; a system-owned account stands in.
        (nonexistent_kms_context, funded_system_account()),
        (address, encrypted_value_account(&sealed)),
    ];
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[anchor_error(
            anchor_lang::error::ErrorCode::AccountOwnedByWrongProgram,
        )],
    );
}

#[test]
fn mollusk_verify_public_decrypt_rejects_sub_threshold_signatures() {
    let admin = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_with_context(admin, KMS_CONTEXT_ID);
    // Context requires two distinct signers; the cert carries only one.
    let (kms_context, kms_context_acct) = kms_context_account_with(
        KMS_CONTEXT_ID,
        vec![
            signing::secp_evm_address(&signing::kms_signing_key()),
            [0xABu8; 20],
        ],
        2,
        false,
    );
    let handle = handle_for_chain(5, 5);
    let (address, sealed, proof) =
        seal_public_leaf(admin, &app, host_config, &host_config_account, handle);

    let extra_data = vec![0x00u8];
    let (cleartext, signatures) = public_decrypt_cert(handle, &extra_data);
    let ix = verify_public_decrypt_ix(
        host_config,
        kms_context,
        address,
        handle,
        cleartext,
        signatures,
        extra_data,
        proof,
    );
    let accounts = vec![
        (host_config, host_config_account),
        (kms_context, kms_context_acct),
        (address, encrypted_value_account(&sealed)),
    ];
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::InvalidKmsCertificate,
        )],
    );
}

#[test]
fn mollusk_verify_public_decrypt_rejects_handle_proof_mismatch() {
    let admin = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_with_context(admin, KMS_CONTEXT_ID);
    let (kms_context, kms_context_acct) = kms_context_account(KMS_CONTEXT_ID);
    let sealed_handle = handle_for_chain(5, 5);
    let (address, sealed, proof) = seal_public_leaf(
        admin,
        &app,
        host_config,
        &host_config_account,
        sealed_handle,
    );

    // A cert valid over a DIFFERENT handle, presented with the sealed handle's proof: the cert check
    // passes but the exact-handle inclusion proof does not authorize the unsealed handle.
    let other_handle = handle_for_chain(6, 5);
    let extra_data = vec![0x00u8];
    let (cleartext, signatures) = public_decrypt_cert(other_handle, &extra_data);
    let ix = verify_public_decrypt_ix(
        host_config,
        kms_context,
        address,
        other_handle,
        cleartext,
        signatures,
        extra_data,
        proof,
    );
    let accounts = vec![
        (host_config, host_config_account),
        (kms_context, kms_context_acct),
        (address, encrypted_value_account(&sealed)),
    ];
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::PublicDecryptProofInvalid,
        )],
    );
}

#[test]
fn mollusk_verify_public_decrypt_rejects_non_canonical_kms_context() {
    let admin = Pubkey::new_unique();
    let app = App::new();
    let (host_config, host_config_account) = host_config_with_context(admin, KMS_CONTEXT_ID);
    // Canonical context data, placed at a non-canonical address.
    let (_, kms_context_acct) = kms_context_account(KMS_CONTEXT_ID);
    let wrong_kms_context = Pubkey::new_unique();
    let handle = handle_for_chain(5, 5);
    let (address, sealed, proof) =
        seal_public_leaf(admin, &app, host_config, &host_config_account, handle);

    let extra_data = vec![0x00u8];
    let (cleartext, signatures) = public_decrypt_cert(handle, &extra_data);
    let ix = verify_public_decrypt_ix(
        host_config,
        wrong_kms_context,
        address,
        handle,
        cleartext,
        signatures,
        extra_data,
        proof,
    );
    let accounts = vec![
        (host_config, host_config_account),
        (wrong_kms_context, kms_context_acct),
        (address, encrypted_value_account(&sealed)),
    ];
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::InvalidKmsContext)],
    );
}

#[test]
fn mollusk_verify_public_decrypt_survives_update_after_seal() {
    // The dust-race claim: an update between seal and consume moves the MMR peaks but can neither
    // invalidate nor retarget the sealed leaf. The OLD handle still verifies with a proof rebuilt
    // against the updated peaks.
    let admin = Pubkey::new_unique();
    let app = App::new();
    let viewer = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_with_context(admin, KMS_CONTEXT_ID);
    let (kms_context, kms_context_acct) = kms_context_account(KMS_CONTEXT_ID);
    let handle0 = handle_for_chain(30, 5);
    let (address, sealed, _) =
        seal_public_leaf(admin, &app, host_config, &host_config_account, handle0);

    // Update the value (dust transfer analog) after the seal.
    let final_value = update_with_fhe_execute(
        admin,
        &app,
        host_config,
        host_config_account.clone(),
        address,
        &sealed,
        &[viewer],
        31,
    );
    assert_ne!(final_value.current_handle, handle0);

    // Rebuild the proof for the sealed leaf 0 against the post-update peaks.
    let mut events = vec![EncryptedValueAccountEvent::MarkedPublic { handle: handle0 }];
    events.extend(allowed_events(final_value.current_handle, &[viewer]));
    let proof = mmr_inclusion_proof(
        zama_solana_acl::encrypted_value_account::build_verified_proof_from_events(
            address.to_bytes(),
            &events,
            &final_value.peaks,
            final_value.leaf_count,
            0,
        )
        .unwrap(),
    );

    let extra_data = vec![0x00u8];
    let (cleartext, signatures) = public_decrypt_cert(handle0, &extra_data);
    let ix = verify_public_decrypt_ix(
        host_config,
        kms_context,
        address,
        handle0,
        cleartext,
        signatures,
        extra_data,
        proof,
    );
    let accounts = vec![
        (host_config, host_config_account),
        (kms_context, kms_context_acct),
        (address, encrypted_value_account(&final_value)),
    ];
    let expected = expected_return_data(handle0, cleartext, KMS_CONTEXT_ID);
    let result = mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[Check::success(), Check::return_data(&expected)],
    );
    assert_eq!(result.return_data, expected);
}

#[test]
fn mollusk_verify_public_decrypt_rejects_historical_only_leaf() {
    // Public-vs-historical leaf domain separation: a value written WITHOUT make_handle_public has
    // only historical-access leaves. A proof for such a leaf must not authorize a public decrypt,
    // even though the leaf genuinely exists — the two use distinct leaf commitments.
    let admin = Pubkey::new_unique();
    let app = App::new();
    let viewer = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_with_context(admin, KMS_CONTEXT_ID);
    let (kms_context, kms_context_acct) = kms_context_account(KMS_CONTEXT_ID);
    let (address, value0) = app.value("balance", handle_for_chain(40, 5));

    // The update seals a historical-access leaf for (new handle, viewer); no public-decrypt leaf.
    let final_value = update_with_fhe_execute(
        admin,
        &app,
        host_config,
        host_config_account.clone(),
        address,
        &value0,
        &[viewer],
        41,
    );
    let handle1 = final_value.current_handle;

    let events = allowed_events(handle1, &[viewer]);
    let shared_proof = zama_solana_acl::encrypted_value_account::build_verified_proof_from_events(
        address.to_bytes(),
        &events,
        &final_value.peaks,
        final_value.leaf_count,
        0,
    )
    .unwrap();
    // The leaf really exists (it authorizes historically), but the public-decrypt domain rejects it.
    let shared = final_value.to_shared();
    assert!(zama_solana_acl::authorize_historical(
        address.to_bytes(),
        &shared,
        handle1,
        viewer.to_bytes(),
        &shared_proof,
    )
    .is_ok());
    assert!(
        zama_solana_acl::authorize_public(address.to_bytes(), &shared, handle1, &shared_proof)
            .is_err()
    );

    let extra_data = vec![0x00u8];
    let (cleartext, signatures) = public_decrypt_cert(handle1, &extra_data);
    let ix = verify_public_decrypt_ix(
        host_config,
        kms_context,
        address,
        handle1,
        cleartext,
        signatures,
        extra_data,
        mmr_inclusion_proof(shared_proof),
    );
    let accounts = vec![
        (host_config, host_config_account),
        (kms_context, kms_context_acct),
        (address, encrypted_value_account(&final_value)),
    ];
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::PublicDecryptProofInvalid,
        )],
    );
}

// ---------------------------------------------------------------------------
// Cost snapshots (zama-solana-test-kit::snapshot). Dedicated tests so cost
// drift never fails a behavior test; regenerate with
// `bash scripts/update-cost-snapshots.sh`.
// ---------------------------------------------------------------------------

#[test]
fn cost_snapshot_verify_public_decrypt() {
    // Happy-path stateless verify with fixed fixture keys: three read-only accounts, one secp
    // recovery, one MMR inclusion check. Per-consume CU is the price of statelessness (#1704).
    let admin = Pubkey::new_from_array([0x31; 32]);
    let app = App::with_program(Pubkey::new_from_array([0x33; 32]));
    let (host_config, host_config_account) = host_config_with_context(admin, KMS_CONTEXT_ID);
    let (kms_context, kms_context_acct) = kms_context_account(KMS_CONTEXT_ID);
    let handle = handle_for_chain(5, 5);
    let (address, sealed, proof) =
        seal_public_leaf(admin, &app, host_config, &host_config_account, handle);

    let extra_data = vec![0x00u8];
    let (cleartext, signatures) = public_decrypt_cert(handle, &extra_data);
    let ix = verify_public_decrypt_ix(
        host_config,
        kms_context,
        address,
        handle,
        cleartext,
        signatures,
        extra_data,
        proof,
    );
    let accounts = vec![
        (host_config, host_config_account),
        (kms_context, kms_context_acct),
        (address, encrypted_value_account(&sealed)),
    ];
    let result = mollusk().process_and_validate_instruction(&ix, &accounts, &[Check::success()]);
    cost_snapshot::assert_cost_snapshot(
        "host_mollusk",
        "verify_public_decrypt/happy",
        &ix,
        &result,
    );
}

fn snapshot_fixture() -> FheExecutionFixture {
    FheExecutionFixture::with_block_cap_keys(
        u64::MAX,
        Pubkey::new_from_array([0x21; 32]),
        Pubkey::new_from_array([0x22; 32]),
    )
}

#[test]
fn cost_snapshot_fhe_execute_three_steps() {
    // Unrestricted HCU cap, no optional meter/trust accounts: the minimal
    // canonical execution (`FheExecutionFixture::success_batch`) with one persistent
    // output create.
    let fixture = snapshot_fixture();
    let ix = fixture.block_cap_instruction(None, None);

    let result = fixture
        .context
        .process_and_validate_instruction(&ix, &[Check::success()]);

    cost_snapshot::assert_cost_snapshot("host_mollusk", "fhe_execute/three_steps", &ix, &result);
}

#[test]
fn cost_snapshot_fhe_execute_max_steps() {
    // An execution at MAX_FHE_EXECUTION_STEPS with the same fixture keys, accounts, and
    // persistent-output shape as the three-op profile. The compute-unit delta
    // isolates the extra direct host-side fhe_execute steps; it does not include
    // work performed by an application before invoking the host program.
    let fixture = snapshot_fixture();
    let ix = fixture.max_ops_instruction();

    let result = fixture
        .context
        .process_and_validate_instruction(&ix, &[Check::success()]);

    cost_snapshot::assert_cost_snapshot("host_mollusk", "fhe_execute/max_steps", &ix, &result);
}

#[test]
fn mollusk_fhe_execute_max_op_transaction_fits_packet() {
    // MAX_FHE_EXECUTION_STEPS is derived from measured budgets (fhevm-internal#1853 W8). This is the
    // byte-budget half: the whole signed transaction for the max-op execution — envelope included —
    // must fit one 1,232-byte packet, with headroom for realistic envelopes (more persistent
    // accounts, more dictionary entries) so the cap is not set at the packet edge.
    let fixture = snapshot_fixture();
    // The Solana transaction packet limit (solana-packet's PACKET_DATA_SIZE: 1280-byte
    // IPv6 minimum MTU minus 48 bytes of headers).
    const PACKET_DATA_SIZE: usize = 1_232;
    let ix = fixture.max_ops_instruction();
    let message =
        solana_sdk::message::Message::new(std::slice::from_ref(&ix), Some(&fixture.payer));
    let signature_bytes = 1 + 64 * usize::from(message.header.num_required_signatures);
    let transaction_bytes = signature_bytes + message.serialize().len();

    assert!(
        transaction_bytes + 150 <= PACKET_DATA_SIZE,
        "max-op fhe_execute transaction is {transaction_bytes} bytes; it must fit a \
         {PACKET_DATA_SIZE}-byte packet with >=150 bytes of envelope headroom"
    );
}
