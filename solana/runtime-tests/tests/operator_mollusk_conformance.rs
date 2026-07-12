//! Representative real-host conformance for Solana `fhe_eval` operator families.
//!
//! Each test executes one canonical instruction against the compiled `zama_host` program, then
//! evaluates that exact plan in the test-owned cleartext evaluator. The exhaustive semantic
//! contract stays in `operator_conformance`; this target covers only materially different host
//! admission and result-binding shapes.

mod support;

use std::{collections::HashMap, path::PathBuf, sync::Once};

use anchor_lang::{
    prelude::system_program, AccountSerialize, AnchorDeserialize, Discriminator, InstructionData,
    ToAccountMetas,
};
use mollusk_svm::{
    result::{Check, InstructionResult},
    Mollusk,
};
use solana_program::keccak::hashv as keccak_hashv;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use solana_sha256_hasher::hashv as sha256_hashv;
use support::cleartext_fhe_eval::{evaluate, ClearInputs, TypedClearValue};
use zama_host::{
    self as host, FheBinaryOpCode, FheEvalArgs, FheEvalOperand, FheEvalOutput, FheEvalStep,
    FheUnaryOpCode,
};

const CONTEXT_ID: [u8; 32] = [7; 32];
// The PoC sentinel chain with test shims enabled deliberately substitutes zero birth entropy.
const PREVIOUS_BANK_HASH: [u8; 32] = [0; 32];
const UNIX_TIMESTAMP: i64 = 0;

#[test]
fn encrypted_encrypted_add_executes_then_reads_cleartext_outcome() {
    let mut flow = EvalFlow::new();
    let lhs = flow.encrypted(5, 40);
    let rhs = flow.encrypted(5, 2);
    let outcome = flow.execute(FheEvalStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: lhs.clone(),
        rhs: rhs.clone(),
        output_fhe_type: 5,
        output: FheEvalOutput::AllowedLocal,
    });

    outcome.assert_u64(5, 42);
    let event: host::FheBinaryOpEvent = outcome.event();
    assert_binary_event(&outcome, &event, FheBinaryOpCode::Add, &lhs, &rhs, 5);
}

#[test]
fn encrypted_scalar_add_executes_then_reads_cleartext_outcome() {
    let mut flow = EvalFlow::new();
    let lhs = flow.encrypted(5, 40);
    let rhs = FheEvalOperand::Scalar(be(2));
    let outcome = flow.execute(FheEvalStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: lhs.clone(),
        rhs: rhs.clone(),
        output_fhe_type: 5,
        output: FheEvalOutput::AllowedLocal,
    });

    outcome.assert_u64(5, 42);
    let event: host::FheBinaryOpEvent = outcome.event();
    assert_binary_event(&outcome, &event, FheBinaryOpCode::Add, &lhs, &rhs, 5);
}

#[test]
fn comparison_executes_then_reads_bool_outcome() {
    let mut flow = EvalFlow::new();
    let lhs = flow.encrypted(5, 42);
    let rhs = flow.encrypted(5, 42);
    let outcome = flow.execute(FheEvalStep::Binary {
        op: FheBinaryOpCode::Eq,
        lhs: lhs.clone(),
        rhs: rhs.clone(),
        output_fhe_type: 0,
        output: FheEvalOutput::AllowedLocal,
    });

    outcome.assert_u64(0, 1);
    let event: host::FheBinaryOpEvent = outcome.event();
    assert_binary_event(&outcome, &event, FheBinaryOpCode::Eq, &lhs, &rhs, 0);
}

#[test]
fn cast_executes_then_reads_widened_outcome() {
    let mut flow = EvalFlow::new();
    let operand = flow.encrypted(2, 255);
    let outcome = flow.execute(FheEvalStep::Unary {
        op: FheUnaryOpCode::Cast,
        operand: operand.clone(),
        output_fhe_type: 5,
        output: FheEvalOutput::AllowedLocal,
    });

    outcome.assert_u64(5, 255);
    let event: host::FheUnaryOpEvent = outcome.event();
    outcome.assert_envelope(event.version, event.subject);
    assert_eq!(event.op, FheUnaryOpCode::Cast);
    assert_eq!(event.operand, operand_handle(&operand));
    assert_eq!(
        event.result,
        expected_unary_handle(event.op, event.operand, 5)
    );
}

#[test]
fn unary_not_executes_then_reads_width_bounded_outcome() {
    let mut flow = EvalFlow::new();
    let operand = flow.encrypted(2, 0b1010);
    let outcome = flow.execute(FheEvalStep::Unary {
        op: FheUnaryOpCode::Not,
        operand: operand.clone(),
        output_fhe_type: 2,
        output: FheEvalOutput::AllowedLocal,
    });

    outcome.assert_u64(2, 0b1111_0101);
    let event: host::FheUnaryOpEvent = outcome.event();
    outcome.assert_envelope(event.version, event.subject);
    assert_eq!(event.op, FheUnaryOpCode::Not);
    assert_eq!(event.operand, operand_handle(&operand));
    assert_eq!(
        event.result,
        expected_unary_handle(event.op, event.operand, 2)
    );
}

#[test]
fn membership_executes_then_reads_present_outcome() {
    let mut flow = EvalFlow::new();
    let value = flow.encrypted(5, 42);
    let first = flow.encrypted(5, 7);
    let second = flow.encrypted(5, 42);
    let set = vec![first, second];
    let outcome = flow.execute(FheEvalStep::IsIn {
        value: value.clone(),
        set: set.clone(),
        fhe_type: 5,
        output: FheEvalOutput::AllowedLocal,
    });

    outcome.assert_u64(0, 1);
    let event: host::FheIsInEvent = outcome.event();
    outcome.assert_envelope(event.version, event.subject);
    assert_eq!(event.value, operand_handle(&value));
    assert_eq!(event.set, operand_handles(&set));
    assert_eq!(event.fhe_type, 5);
    assert_eq!(
        event.result,
        expected_is_in_handle(event.value, &event.set, 5)
    );
}

#[test]
fn random_executes_then_binds_seed_and_type() {
    let outcome = EvalFlow::new().execute(FheEvalStep::Rand {
        fhe_type: 5,
        output: FheEvalOutput::AllowedLocal,
    });

    assert_eq!(outcome.only_cleartext().fhe_type, 5);
    let event: host::FheRandEvent = outcome.event();
    outcome.assert_envelope(event.version, event.subject);
    assert_eq!(event.seed, expected_rand_seed());
    assert_eq!(event.fhe_type, 5);
    assert_eq!(event.result, expected_rand_handle(event.seed, 5));
}

#[test]
fn bounded_random_executes_then_binds_bound_into_result_handle() {
    let outcome = EvalFlow::new().execute(FheEvalStep::RandBounded {
        upper_bound: be(16),
        fhe_type: 5,
        output: FheEvalOutput::AllowedLocal,
    });

    assert!(outcome.only_u64() < 16);
    let event: host::FheRandBoundedEvent = outcome.event();
    outcome.assert_envelope(event.version, event.subject);
    assert_eq!(event.upper_bound, be(16));
    assert_eq!(event.seed, expected_rand_seed());
    assert_eq!(event.fhe_type, 5);
    assert_eq!(
        event.result,
        expected_rand_bounded_handle(event.upper_bound, event.seed, 5)
    );
}

#[test]
fn mismatched_encrypted_operand_types_are_rejected() {
    let mut flow = EvalFlow::new();
    let lhs = flow.encrypted(5, 40);
    let rhs = flow.encrypted(4, 2);

    flow.rejects(
        FheEvalStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs,
            output_fhe_type: 5,
            output: FheEvalOutput::AllowedLocal,
        },
        host::errors::ZamaHostError::BinaryOperandTypeMismatch,
    );
}

#[test]
fn system_owned_encrypted_operand_is_rejected() {
    let mut flow = EvalFlow::new();
    let lhs = flow.encrypted(5, 40);
    flow.make_last_encrypted_account_system_owned();

    flow.rejects(
        FheEvalStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs: FheEvalOperand::Scalar(be(2)),
            output_fhe_type: 5,
            output: FheEvalOutput::AllowedLocal,
        },
        host::errors::ZamaHostError::EncryptedValueAccountInvalid,
    );
}

#[test]
fn readonly_durable_output_is_rejected() {
    let mut flow = EvalFlow::new();
    let lhs = flow.encrypted(5, 40);
    let output = flow.readonly_durable_output();

    flow.rejects(
        FheEvalStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs: FheEvalOperand::Scalar(be(2)),
            output_fhe_type: 5,
            output,
        },
        host::errors::ZamaHostError::InvalidFheEvalAccount,
    );
}

fn assert_binary_event(
    outcome: &EvalOutcome,
    event: &host::FheBinaryOpEvent,
    op: FheBinaryOpCode,
    lhs: &FheEvalOperand,
    rhs: &FheEvalOperand,
    output_type: u8,
) {
    outcome.assert_envelope(event.version, event.subject);
    assert_eq!(event.op, op);
    assert_eq!(event.lhs, operand_handle(lhs));
    assert_eq!(event.rhs, operand_handle(rhs));
    assert_eq!(event.scalar, matches!(rhs, FheEvalOperand::Scalar(_)));
    assert_eq!(
        event.result,
        expected_binary_handle(op, event.lhs, event.rhs, event.scalar, output_type,)
    );
}

struct EvalFlow {
    authority: Pubkey,
    host_config: Pubkey,
    accounts: Vec<(Pubkey, Account)>,
    remaining: Vec<AccountMeta>,
    cleartext: ClearInputs,
    next_seed: u8,
}

impl EvalFlow {
    fn new() -> Self {
        let authority = Pubkey::new_unique();
        let (host_config, host_config_account) = host_config_account(authority);
        Self {
            authority,
            host_config,
            accounts: vec![
                (system_program::ID, system_program_account()),
                (authority, funded_system_account()),
                (host_config, host_config_account),
                (event_authority(host::id()), Account::default()),
            ],
            remaining: Vec::new(),
            cleartext: HashMap::new(),
            next_seed: 1,
        }
    }

    fn encrypted(&mut self, fhe_type: u8, plaintext: u64) -> FheEvalOperand {
        let seed = self.next_seed;
        self.next_seed += 1;
        let handle = handle_for_chain(seed, fhe_type);
        self.cleartext
            .insert(handle, TypedClearValue::from_u64(fhe_type, plaintext));
        let (address, value) = new_lineage(self.authority, [seed; 32], handle);
        let encrypted_value_index = self.remaining.len() as u16;
        self.remaining
            .push(AccountMeta::new_readonly(address, false));
        self.accounts
            .push((address, encrypted_value_account(&value)));
        FheEvalOperand::AllowedDurable {
            handle,
            encrypted_value_index,
        }
    }

    fn make_last_encrypted_account_system_owned(&mut self) {
        self.accounts.last_mut().unwrap().1.owner = system_program::ID;
    }

    fn readonly_durable_output(&mut self) -> FheEvalOutput {
        let label = [99; 32];
        let value_key = zama_solana_acl::derive_value_key(
            self.authority.to_bytes(),
            self.authority.to_bytes(),
            label,
        );
        let address = host::encrypted_value_address(value_key).0;
        let output_encrypted_value_index = self.remaining.len() as u16;
        self.remaining
            .push(AccountMeta::new_readonly(address, false));
        self.accounts.push((address, empty_system_account()));
        FheEvalOutput::AllowedDurable {
            output_encrypted_value_index,
            output_app_account_authority_index: None,
            output_acl_domain_key: self.authority,
            output_app_account: self.authority,
            output_encrypted_value_label: label,
            output_subjects: vec![host::AclSubjectEntry {
                pubkey: self.authority,
            }],
            previous_handle: None,
            previous_subjects: None,
            make_public: false,
        }
    }

    fn execute(self, step: FheEvalStep) -> EvalOutcome {
        let (args, instruction) = self.instruction(step);
        let result = mollusk().process_and_validate_instruction(
            &instruction,
            &self.accounts,
            &[Check::success()],
        );
        let cleartext = evaluate(&args, &self.cleartext)
            .expect("accepted host plan must have valid cleartext semantics");
        EvalOutcome {
            subject: self.authority,
            result,
            cleartext,
        }
    }

    fn rejects(self, step: FheEvalStep, error: host::errors::ZamaHostError) {
        let (_, instruction) = self.instruction(step);
        mollusk().process_and_validate_instruction(
            &instruction,
            &self.accounts,
            &[custom_error(error)],
        );
    }

    fn instruction(&self, step: FheEvalStep) -> (FheEvalArgs, Instruction) {
        let args = FheEvalArgs {
            context_id: CONTEXT_ID,
            steps: vec![step],
        };
        let mut instruction = anchor_ix(
            host::id(),
            host::accounts::FheEval {
                payer: self.authority,
                compute_subject: self.authority,
                app_account_authority: self.authority,
                host_config: self.host_config,
                system_program: system_program::ID,
                hcu_authority: self.authority,
                hcu_block_meter: None,
                hcu_trusted_app_record: None,
                event_authority: event_authority(host::id()),
                program: host::id(),
            },
            host::instruction::FheEval { args: args.clone() },
        );
        instruction.accounts.extend(self.remaining.clone());
        (args, instruction)
    }
}

struct EvalOutcome {
    subject: Pubkey,
    result: InstructionResult,
    cleartext: Vec<TypedClearValue>,
}

impl EvalOutcome {
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

    fn assert_envelope(&self, version: u8, subject: [u8; 32]) {
        assert_eq!(version, host::EVENT_VERSION);
        assert_eq!(subject, self.subject.to_bytes());
    }

    fn event<T>(&self) -> T
    where
        T: AnchorDeserialize + Discriminator,
    {
        event(&self.result)
    }
}

fn mollusk() -> Mollusk {
    static SET_SBF_OUT_DIR: Once = Once::new();
    SET_SBF_OUT_DIR.call_once(|| {
        let deploy_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy");
        unsafe { std::env::set_var("SBF_OUT_DIR", deploy_dir) };
    });
    let mut mollusk = Mollusk::new(&host::id(), "zama_host");
    mollusk.sysvars.clock.slot = 100;
    mollusk.sysvars.clock.unix_timestamp = UNIX_TIMESTAMP;
    mollusk
}

fn anchor_ix<A: ToAccountMetas, D: InstructionData>(
    program_id: Pubkey,
    accounts: A,
    args: D,
) -> Instruction {
    Instruction {
        program_id,
        accounts: accounts.to_account_metas(None),
        data: args.data(),
    }
}

fn host_config_account(admin: Pubkey) -> (Pubkey, Account) {
    let (address, bump) = host::host_config_address();
    (
        address,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(host::HostConfig {
                admin,
                chain_id: host::SOLANA_POC_CHAIN_ID,
                input_verifier_authority: admin,
                gateway_chain_id: 31337,
                input_verification_contract: [0xcd; 20],
                coprocessor_signer: [0; 20],
                decryption_contract: [0xde; 20],
                current_kms_context_id: 0,
                material_authority: admin,
                test_authority: admin,
                paused: false,
                mock_input_enabled: false,
                test_shims_enabled: true,
                grant_deny_list_enabled: false,
                max_hcu_per_tx: 0,
                max_hcu_depth_per_tx: 0,
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

fn new_lineage(
    authority: Pubkey,
    label: [u8; 32],
    handle: [u8; 32],
) -> (Pubkey, host::EncryptedValue) {
    let value_key =
        zama_solana_acl::derive_value_key(authority.to_bytes(), authority.to_bytes(), label);
    let (address, bump) = host::encrypted_value_address(value_key);
    (
        address,
        host::EncryptedValue {
            acl_domain_key: authority,
            app_account: authority,
            encrypted_value_label: label,
            current_handle: handle,
            subjects: vec![authority],
            leaf_count: 0,
            peaks: vec![],
            bump,
        },
    )
}

fn serialized_account<T: AccountSerialize>(value: T) -> Vec<u8> {
    let mut data = Vec::new();
    value.try_serialize(&mut data).unwrap();
    data
}

fn encrypted_value_account(value: &host::EncryptedValue) -> Account {
    Account {
        lamports: 10_000_000_000,
        data: serialized_account(value.clone()),
        owner: host::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn funded_system_account() -> Account {
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

fn event_authority(program_id: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"__event_authority"], &program_id).0
}

fn custom_error(error: host::errors::ZamaHostError) -> Check<'static> {
    Check::err(solana_sdk::program_error::ProgramError::Custom(
        anchor_lang::error::ERROR_CODE_OFFSET + error as u32,
    ))
}

fn handle_for_chain(seed: u8, fhe_type: u8) -> [u8; 32] {
    let mut handle = [seed; 32];
    handle[21] = 0;
    handle[22..30].copy_from_slice(&host::SOLANA_POC_CHAIN_ID.to_be_bytes());
    handle[30] = fhe_type;
    handle[31] = host::HANDLE_VERSION;
    handle
}

fn operand_handle(operand: &FheEvalOperand) -> [u8; 32] {
    match operand {
        FheEvalOperand::AllowedDurable { handle, .. } => *handle,
        FheEvalOperand::Scalar(value) => *value,
        _ => panic!("representative flow uses only durable or scalar operands"),
    }
}

fn operand_handles(operands: &[FheEvalOperand]) -> Vec<[u8; 32]> {
    operands.iter().map(operand_handle).collect()
}

fn event<T>(result: &InstructionResult) -> T
where
    T: AnchorDeserialize + Discriminator,
{
    let prefix = anchor_lang::event::EVENT_IX_TAG_LE
        .iter()
        .copied()
        .chain(T::DISCRIMINATOR.iter().copied())
        .collect::<Vec<_>>();
    let events = result
        .inner_instructions
        .iter()
        .filter_map(|inner| {
            let payload = inner.instruction.data.strip_prefix(prefix.as_slice())?;
            T::deserialize(&mut &*payload).ok()
        })
        .collect::<Vec<_>>();
    assert_eq!(events.len(), 1, "expected one emitted operation event");
    events.into_iter().next().unwrap()
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
            &CONTEXT_ID,
            &0_u16.to_be_bytes(),
            &[op.as_u8()],
            &lhs,
            &rhs,
            &[u8::from(scalar)],
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
    let op_index = 0_u16.to_be_bytes();
    let timestamp = UNIX_TIMESTAMP.to_be_bytes();
    let mut parts: Vec<&[u8]> = vec![
        b"FHE_eval_unary",
        &CONTEXT_ID,
        &op_index,
        &op_byte,
        &operand,
    ];
    if matches!(op, FheUnaryOpCode::Cast) {
        parts.push(&fhe_type_byte);
    }
    parts.extend_from_slice(&[
        program_id.as_ref(),
        &chain_id,
        &PREVIOUS_BANK_HASH,
        &timestamp,
    ]);
    finish_handle(sha256_hashv(&parts).to_bytes(), fhe_type)
}

fn expected_is_in_handle(value: [u8; 32], set: &[[u8; 32]], fhe_type: u8) -> [u8; 32] {
    let fhe_type_byte = [fhe_type];
    let program_id = host::id();
    let chain_id = host::SOLANA_POC_CHAIN_ID.to_be_bytes();
    let op_index = 0_u16.to_be_bytes();
    let timestamp = UNIX_TIMESTAMP.to_be_bytes();
    let mut parts: Vec<&[u8]> = vec![
        b"FHE_eval_is_in",
        &CONTEXT_ID,
        &op_index,
        &fhe_type_byte,
        &value,
    ];
    parts.extend(set.iter().map(<[u8; 32]>::as_ref));
    parts.extend_from_slice(&[
        program_id.as_ref(),
        &chain_id,
        &PREVIOUS_BANK_HASH,
        &timestamp,
    ]);
    finish_handle(sha256_hashv(&parts).to_bytes(), 0)
}

fn expected_rand_seed() -> [u8; 16] {
    let hash = sha256_hashv(&[
        b"FHE_eval_seed",
        &CONTEXT_ID,
        &0_u16.to_be_bytes(),
        host::id().as_ref(),
        &host::SOLANA_POC_CHAIN_ID.to_be_bytes(),
        &PREVIOUS_BANK_HASH,
        &UNIX_TIMESTAMP.to_be_bytes(),
    ])
    .to_bytes();
    hash[..16].try_into().unwrap()
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
    let mut bytes = [0; 32];
    bytes[24..].copy_from_slice(&value.to_be_bytes());
    bytes
}
