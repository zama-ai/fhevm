//! Representative real-host conformance for Solana `fhe_execute` operator families.
//!
//! Each test executes one canonical instruction against the compiled `zama_host` program, then
//! evaluates that exact execution in the test-owned cleartext evaluator. The exhaustive semantic
//! contract stays in `operator_conformance`; this target covers only materially different host
//! admission and result-binding shapes.

use std::collections::HashMap;

use anchor_lang::{prelude::system_program, AccountDeserialize};
use mollusk_svm::{result::Check, Mollusk};
use solana_program::keccak::hashv as keccak_hashv;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use zama_host::{
    self as host, AppScope, FheBinaryOpCode, FheExecuteArgs, FheExecuteEffect, FheExecuteOperand,
    FheExecuteStep, FheUnaryOpCode, SlotWrite,
};
use zama_solana_test_kit::oracle::{evaluate, ClearInputs, TypedClearValue};
use zama_solana_test_kit::{
    anchor_error_check, anchor_ix, empty_system_account, encrypted_state_account, event_authority,
    funded_system_account, handle_for_chain, host_config_account, new_encrypted_state,
    rand_nonce_account, system_program_account, HostConfigParams,
};

mod host_fixtures;
use host_fixtures::{fixture_scope, sole_state_authority, StateAuthority};

const PREVIOUS_BANK_HASH: [u8; 32] = [0x44; 32];
const UNIX_TIMESTAMP: i64 = 0;

#[test]
fn encrypted_encrypted_add_executes_then_reads_cleartext_outcome() {
    let mut flow = ExecutionFlow::new();
    let lhs = flow.encrypted(5, 40);
    let rhs = flow.encrypted(5, 2);
    let outcome = flow.execute(FheExecuteStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: lhs.clone(),
        rhs: rhs.clone(),
        output_fhe_type: 5,
    });

    outcome.assert_u64(5, 42);
    outcome.assert_handle(expected_binary_handle(
        FheBinaryOpCode::Add,
        operand_handle(&lhs),
        operand_handle(&rhs),
        false,
        5,
    ));
}

#[test]
fn encrypted_scalar_add_executes_then_reads_cleartext_outcome() {
    let mut flow = ExecutionFlow::new();
    let lhs = flow.encrypted(5, 40);
    let rhs = scalar(be(2));
    let outcome = flow.execute(FheExecuteStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: lhs.clone(),
        rhs: rhs.clone(),
        output_fhe_type: 5,
    });

    outcome.assert_u64(5, 42);
    outcome.assert_handle(expected_binary_handle(
        FheBinaryOpCode::Add,
        operand_handle(&lhs),
        operand_handle(&rhs),
        true,
        5,
    ));
}

#[test]
fn comparison_executes_then_reads_bool_outcome() {
    let mut flow = ExecutionFlow::new();
    let lhs = flow.encrypted(5, 42);
    let rhs = flow.encrypted(5, 42);
    let outcome = flow.execute(FheExecuteStep::Binary {
        op: FheBinaryOpCode::Eq,
        lhs: lhs.clone(),
        rhs: rhs.clone(),
        output_fhe_type: 0,
    });

    outcome.assert_u64(0, 1);
    outcome.assert_handle(expected_binary_handle(
        FheBinaryOpCode::Eq,
        operand_handle(&lhs),
        operand_handle(&rhs),
        false,
        0,
    ));
}

#[test]
fn cast_executes_then_reads_widened_outcome() {
    let mut flow = ExecutionFlow::new();
    let operand = flow.encrypted(2, 255);
    let outcome = flow.execute(FheExecuteStep::Unary {
        op: FheUnaryOpCode::Cast,
        operand: operand.clone(),
        output_fhe_type: 5,
    });

    outcome.assert_u64(5, 255);
    outcome.assert_handle(expected_unary_handle(
        FheUnaryOpCode::Cast,
        operand_handle(&operand),
        5,
    ));
}

#[test]
fn unary_not_executes_then_reads_width_bounded_outcome() {
    let mut flow = ExecutionFlow::new();
    let operand = flow.encrypted(2, 0b1010);
    let outcome = flow.execute(FheExecuteStep::Unary {
        op: FheUnaryOpCode::Not,
        operand: operand.clone(),
        output_fhe_type: 2,
    });

    outcome.assert_u64(2, 0b1111_0101);
    outcome.assert_handle(expected_unary_handle(
        FheUnaryOpCode::Not,
        operand_handle(&operand),
        2,
    ));
}

#[test]
fn membership_executes_then_reads_present_outcome() {
    let mut flow = ExecutionFlow::new();
    let value = flow.encrypted(5, 42);
    let first = flow.encrypted(5, 7);
    let second = flow.encrypted(5, 42);
    let set = vec![first, second];
    let outcome = flow.execute(FheExecuteStep::IsIn {
        value: value.clone(),
        set: set.clone(),
        fhe_type: 5,
    });

    outcome.assert_u64(0, 1);
    outcome.assert_handle(expected_is_in_handle(
        operand_handle(&value),
        &operand_handles(&set),
        5,
    ));
}

#[test]
fn random_executes_then_binds_seed_and_type() {
    let outcome = ExecutionFlow::new().execute(FheExecuteStep::Rand { fhe_type: 5 });

    assert_eq!(outcome.only_cleartext().fhe_type, 5);
    outcome.assert_handle(expected_rand_handle(expected_rand_seed(outcome.app), 5));
}

#[test]
fn bounded_random_executes_then_binds_bound_into_result_handle() {
    let outcome = ExecutionFlow::new().execute(FheExecuteStep::RandBounded {
        upper_bound: be(16),
        fhe_type: 5,
    });

    assert!(outcome.only_u64() < 16);
    outcome.assert_handle(expected_rand_bounded_handle(
        be(16),
        expected_rand_seed(outcome.app),
        5,
    ));
}

#[test]
fn mismatched_encrypted_operand_types_are_rejected() {
    let mut flow = ExecutionFlow::new();
    let lhs = flow.encrypted(5, 40);
    let rhs = flow.encrypted(4, 2);

    flow.rejects(
        FheExecuteStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs,
            output_fhe_type: 5,
        },
        host::errors::ZamaHostError::BinaryOperandTypeMismatch,
    );
}

#[test]
fn system_owned_encrypted_operand_is_rejected() {
    let mut flow = ExecutionFlow::new();
    let lhs = flow.encrypted(5, 40);
    flow.make_last_encrypted_account_system_owned();

    flow.rejects(
        FheExecuteStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs: scalar(be(2)),
            output_fhe_type: 5,
        },
        host::errors::ZamaHostError::EncryptedStatePdaMismatch,
    );
}

#[test]
fn readonly_persistent_output_is_rejected() {
    let mut flow = ExecutionFlow::new();
    let lhs = flow.encrypted(5, 40);
    let output = flow.readonly_persistent_output();
    flow.effects.push(output);

    flow.rejects(
        FheExecuteStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs: scalar(be(2)),
            output_fhe_type: 5,
        },
        host::errors::ZamaHostError::InvalidFheExecuteAccount,
    );
}

struct ExecutionFlow {
    payer: Pubkey,
    /// The application: one value authority PDA of a fresh program, signing for every value it
    /// reads and writes in `fixture_scope()`.
    authority: StateAuthority,
    host_config: Pubkey,
    accounts: Vec<(Pubkey, Account)>,
    remaining: Vec<AccountMeta>,
    cleartext: ClearInputs,
    next_seed: u8,
    /// The host's rand nonce account, present iff the execution draws randomness.
    rand_nonce: Option<Pubkey>,
    effects: Vec<FheExecuteEffect>,
}

// FheExecution constants (operand handles, scalar values, output ACL metadata) live in the
// execution's interned dictionary and are referenced by `u8` index (fhevm-internal#1853 W7). The
// flow helpers intern through a thread-local dictionary while a test assembles its single
// execution; `instruction` snapshots it into the finished args and byte-mirror helpers keep
// resolving through it afterwards. Each test runs on its own thread and each flow
// clears the dictionary on construction, so dictionaries never mix across tests.
std::thread_local! {
    static INTERNED_DICTIONARY: std::cell::RefCell<Vec<[u8; 32]>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

fn intern(bytes: [u8; 32]) -> u8 {
    INTERNED_DICTIONARY.with(|dictionary| {
        let mut dictionary = dictionary.borrow_mut();
        if let Some(index) = dictionary.iter().position(|entry| *entry == bytes) {
            return u8::try_from(index).expect("test dictionary fits u8");
        }
        let index = u8::try_from(dictionary.len()).expect("test dictionary fits u8");
        dictionary.push(bytes);
        index
    })
}

fn pool_entry(index: u8) -> [u8; 32] {
    INTERNED_DICTIONARY.with(|dictionary| dictionary.borrow()[usize::from(index)])
}

fn scalar(value: [u8; 32]) -> FheExecuteOperand {
    FheExecuteOperand::Scalar {
        value_index: intern(value),
    }
}

impl ExecutionFlow {
    fn new() -> Self {
        INTERNED_DICTIONARY.with(|dictionary| dictionary.borrow_mut().clear());
        let payer = Pubkey::new_unique();
        let authority = sole_state_authority(Pubkey::new_unique());
        let (host_config, host_config_account) = host_config_account(&HostConfigParams::new(payer));
        let (state_address, state) =
            new_encrypted_state(authority.app(fixture_scope()), authority.key, []);
        Self {
            payer,
            authority,
            host_config,
            accounts: vec![
                (system_program::ID, system_program_account()),
                (payer, funded_system_account()),
                (state_address, encrypted_state_account(&state)),
                (authority.key, empty_system_account()),
                (host_config, host_config_account),
                (event_authority(host::id()), Account::default()),
            ],
            remaining: vec![AccountMeta::new_readonly(state_address, false)],
            cleartext: HashMap::new(),
            next_seed: 1,
            rand_nonce: None,
            effects: vec![],
        }
    }

    fn app(&self) -> AppScope {
        self.authority.app(fixture_scope())
    }

    fn encrypted(&mut self, fhe_type: u8, plaintext: u64) -> FheExecuteOperand {
        let seed = self.next_seed;
        self.next_seed += 1;
        let handle = handle_for_chain(seed, fhe_type);
        self.cleartext
            .insert(handle, TypedClearValue::from_u64(fhe_type, plaintext));
        let (address, mut state) = new_encrypted_state(self.app(), self.authority.key, []);
        let state_index = if let Some(index) = self
            .remaining
            .iter()
            .position(|meta| meta.pubkey == address)
        {
            let account = &mut self
                .accounts
                .iter_mut()
                .find(|(key, _)| *key == address)
                .unwrap()
                .1;
            state = host::EncryptedState::try_deserialize(&mut &account.data[..]).unwrap();
            state.slots.push(host::EncryptedSlot {
                key: [seed; 32],
                handle,
            });
            *account = encrypted_state_account(&state);
            index as u8
        } else {
            state.slots.push(host::EncryptedSlot {
                key: [seed; 32],
                handle,
            });
            let index = self.remaining.len() as u8;
            self.remaining
                .push(AccountMeta::new_readonly(address, false));
            self.accounts
                .push((address, encrypted_state_account(&state)));
            index
        };
        FheExecuteOperand::StateSlot {
            handle_index: intern(handle),
            state_index,
            key_index: intern([seed; 32]),
        }
    }

    fn make_last_encrypted_account_system_owned(&mut self) {
        let address = self.remaining.last().unwrap().pubkey;
        self.accounts
            .iter_mut()
            .find(|(key, _)| *key == address)
            .unwrap()
            .1
            .owner = system_program::ID;
    }

    fn persistent_output(&mut self, key: [u8; 32], writable: bool) -> (FheExecuteEffect, Pubkey) {
        let (address, state) = new_encrypted_state(self.app(), self.authority.key, []);
        let state_index = if let Some(index) = self
            .remaining
            .iter()
            .position(|meta| meta.pubkey == address)
        {
            self.remaining[index].is_writable = writable;
            index as u8
        } else {
            let index = self.remaining.len() as u8;
            self.remaining.push(if writable {
                AccountMeta::new(address, false)
            } else {
                AccountMeta::new_readonly(address, false)
            });
            self.accounts
                .push((address, encrypted_state_account(&state)));
            index
        };
        (
            FheExecuteEffect {
                result: host::ExecutionResultRef {
                    step_index: 0,
                    output_index: 0,
                },
                state_index,
                previous_leaf_count: 0,
                slot: Some(SlotWrite {
                    key_index: intern(key),
                    previous_handle_index: None,
                }),
                allow_indexes: vec![intern(self.authority.key.to_bytes())],
                make_public: false,
                grants: vec![],
            },
            address,
        )
    }

    fn readonly_persistent_output(&mut self) -> FheExecuteEffect {
        self.persistent_output([99; 32], false).0
    }

    fn writable_persistent_output(&mut self) -> (FheExecuteEffect, Pubkey) {
        self.persistent_output([100; 32], true)
    }

    fn execute(mut self, step: FheExecuteStep) -> ExecutionOutcome {
        if matches!(
            step,
            FheExecuteStep::Rand { .. } | FheExecuteStep::RandBounded { .. }
        ) {
            let (address, account) = rand_nonce_account(0);
            self.accounts.push((address, account));
            self.rand_nonce = Some(address);
        }
        let (output, output_address) = self.writable_persistent_output();
        self.effects.push(output);
        let (args, instruction) = self.instruction(step);
        let result = zama_solana_test_kit::transaction::check_fhe_instruction(
            &mollusk(),
            self.payer,
            &instruction,
            &self.accounts,
            &[Check::success()],
        );
        let cleartext = evaluate(&args, &self.cleartext)
            .expect("accepted host execution must have valid cleartext semantics");
        let output_account = result.get_account(&output_address).unwrap();
        let mut output_data: &[u8] = &output_account.data;
        let output_handle = host::EncryptedState::try_deserialize(&mut output_data)
            .expect("state result account")
            .get(&[100; 32])
            .expect("output slot");
        ExecutionOutcome {
            cleartext,
            output_handle,
            app: self.app(),
        }
    }

    fn rejects(self, step: FheExecuteStep, error: host::errors::ZamaHostError) {
        let (_, instruction) = self.instruction(step);
        zama_solana_test_kit::transaction::check_fhe_instruction(
            &mollusk(),
            self.payer,
            &instruction,
            &self.accounts,
            &[custom_error(error)],
        );
    }

    fn instruction(&self, step: FheExecuteStep) -> (FheExecuteArgs, Instruction) {
        let args = FheExecuteArgs {
            execution_state_index: 0,
            effects: self.effects.clone(),
            returned_results: Vec::new(),
            account_count: u8::try_from(self.remaining.len()).expect("test accounts fit u8"),
            dictionary: INTERNED_DICTIONARY.with(|dictionary| dictionary.borrow().clone()),
            steps: vec![step],
        };
        let mut instruction = anchor_ix(
            host::id(),
            host::accounts::FheExecute {
                payer: self.payer,
                authority: self.authority.key,
                host_config: self.host_config,
                system_program: system_program::ID,
                hcu_block_meter: None,
                hcu_trusted_app_record: None,
                rand_nonce: self.rand_nonce,
                scratch: host::transient_address(self.payer).0,
                instructions: solana_sdk::sysvar::instructions::ID,
                event_authority: event_authority(host::id()),
                program: host::id(),
            },
            host::instruction::FheExecute { args: args.clone() },
        );
        instruction.accounts.extend(self.remaining.clone());
        (args, instruction)
    }
}

struct ExecutionOutcome {
    cleartext: Vec<TypedClearValue>,
    output_handle: [u8; 32],
    /// The application the execution ran as; the rand seed binds to it.
    app: AppScope,
}

impl ExecutionOutcome {
    fn only_cleartext(&self) -> TypedClearValue {
        assert_eq!(self.cleartext.len(), 1);
        self.cleartext[0]
    }

    fn only_u64(&self) -> u64 {
        let value = self.only_cleartext().value;
        assert_eq!(value[..24], [0; 24]);
        u64::from_be_bytes(value[24..].try_into().unwrap())
    }

    fn assert_u64(&self, fhe_type: u8, value: u64) {
        assert_eq!(self.only_cleartext().fhe_type, fhe_type);
        assert_eq!(self.only_u64(), value);
    }

    fn assert_handle(&self, expected: [u8; 32]) {
        assert_eq!(self.output_handle, expected);
    }
}

/// Local on purpose (not [`zama_solana_test_kit::host_svm`]): the expected-handle math below
/// hashes the previous bank hash and timestamp into the result, so both must be the fixed
/// `PREVIOUS_BANK_HASH` / `UNIX_TIMESTAMP` rather than the kit's fresh values.
fn mollusk() -> Mollusk {
    let mut mollusk = zama_solana_test_kit::svm(&host::id(), "zama_host");
    mollusk.sysvars.clock.slot = 100;
    mollusk.sysvars.clock.unix_timestamp = UNIX_TIMESTAMP;
    mollusk.sysvars.slot_hashes = solana_sdk::slot_hashes::SlotHashes::new(&[(
        99,
        solana_sdk::hash::Hash::new_from_array(PREVIOUS_BANK_HASH),
    )]);
    mollusk
}

fn custom_error(error: host::errors::ZamaHostError) -> Check<'static> {
    anchor_error_check(error as u32)
}

fn operand_handle(operand: &FheExecuteOperand) -> [u8; 32] {
    match operand {
        FheExecuteOperand::StateSlot { handle_index, .. } => pool_entry(*handle_index),
        FheExecuteOperand::Scalar { value_index } => pool_entry(*value_index),
        _ => panic!("representative flow uses only persistent or scalar operands"),
    }
}

fn operand_handles(operands: &[FheExecuteOperand]) -> Vec<[u8; 32]> {
    operands.iter().map(operand_handle).collect()
}

fn expected_binary_handle(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
) -> [u8; 32] {
    finish_handle(
        keccak_hashv(&[
            b"FHE_eval",
            &[op.as_u8()],
            &lhs,
            &rhs,
            &[u8::from(scalar)],
            &be(if scalar { 1 } else { 3 }),
            host::id().as_ref(),
            &host::SOLANA_POC_CHAIN_ID.to_be_bytes(),
            &PREVIOUS_BANK_HASH,
            &UNIX_TIMESTAMP.to_be_bytes(),
        ])
        .to_bytes(),
        fhe_type,
    )
}

fn expected_unary_handle(op: FheUnaryOpCode, operand: [u8; 32], fhe_type: u8) -> [u8; 32] {
    let op_byte = [op.as_u8()];
    let fhe_type_byte = [fhe_type];
    let program_id = host::id();
    let chain_id = host::SOLANA_POC_CHAIN_ID.to_be_bytes();
    let timestamp = UNIX_TIMESTAMP.to_be_bytes();
    let mut parts: Vec<&[u8]> = vec![b"FHE_eval_unary", &op_byte, &operand];
    if matches!(op, FheUnaryOpCode::Cast) {
        parts.push(&fhe_type_byte);
    }
    let mask = be(1);
    parts.extend_from_slice(&[
        &mask,
        program_id.as_ref(),
        &chain_id,
        &PREVIOUS_BANK_HASH,
        &timestamp,
    ]);
    finish_handle(keccak_hashv(&parts).to_bytes(), fhe_type)
}

fn expected_is_in_handle(value: [u8; 32], set: &[[u8; 32]], fhe_type: u8) -> [u8; 32] {
    let fhe_type_byte = [fhe_type];
    let program_id = host::id();
    let chain_id = host::SOLANA_POC_CHAIN_ID.to_be_bytes();
    let timestamp = UNIX_TIMESTAMP.to_be_bytes();
    let mut parts: Vec<&[u8]> = vec![b"FHE_eval_is_in", &fhe_type_byte, &value];
    parts.extend(set.iter().map(<[u8; 32]>::as_ref));
    let mask = be((1 << (set.len() + 1)) - 1);
    parts.extend_from_slice(&[
        &mask,
        program_id.as_ref(),
        &chain_id,
        &PREVIOUS_BANK_HASH,
        &timestamp,
    ]);
    finish_handle(keccak_hashv(&parts).to_bytes(), 0)
}

/// The seed of the execution's only rand step: the fixture's fresh nonce account starts at 0.
fn expected_rand_seed(app: AppScope) -> [u8; 16] {
    host::computed_eval_rand_seed(
        0,
        app,
        0,
        &host::HandleDerivationContext {
            chain_id: host::SOLANA_POC_CHAIN_ID,
            previous_bank_hash: PREVIOUS_BANK_HASH,
            unix_timestamp: UNIX_TIMESTAMP,
        },
    )
}

fn expected_rand_handle(seed: [u8; 16], fhe_type: u8) -> [u8; 32] {
    finish_handle(
        keccak_hashv(&[
            b"FHE_comp",
            &[3],
            &[fhe_type],
            &seed,
            host::id().as_ref(),
            &host::SOLANA_POC_CHAIN_ID.to_be_bytes(),
        ])
        .to_bytes(),
        fhe_type,
    )
}

fn expected_rand_bounded_handle(upper_bound: [u8; 32], seed: [u8; 16], fhe_type: u8) -> [u8; 32] {
    finish_handle(
        keccak_hashv(&[
            b"FHE_comp",
            &[4],
            &upper_bound,
            &[fhe_type],
            &seed,
            host::id().as_ref(),
            &host::SOLANA_POC_CHAIN_ID.to_be_bytes(),
        ])
        .to_bytes(),
        fhe_type,
    )
}

fn finish_handle(mut handle: [u8; 32], fhe_type: u8) -> [u8; 32] {
    handle[21..32].fill(0);
    handle[21] = 0xff;
    handle[22..30].copy_from_slice(&host::SOLANA_POC_CHAIN_ID.to_be_bytes());
    handle[30] = fhe_type;
    handle[31] = host::HANDLE_VERSION;
    handle
}

fn be(value: u64) -> [u8; 32] {
    zama_solana_test_kit::u256_be(value)
}
