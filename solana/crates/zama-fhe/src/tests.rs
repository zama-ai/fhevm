//! Unit tests for the builder, validators, lowering, and CPI plumbing.

use crate::accounts::*;
use crate::acl::*;
use crate::builder::*;
use crate::execution::*;
use crate::lower::StepTables;
use crate::operand::*;
use crate::state::{StateId, StateOutput};
use crate::types::*;
use crate::validate::{validate_binary_step, validate_unary_step};
use crate::FheExecutionBuildError;
use anchor_lang::prelude::Pubkey;
#[cfg(feature = "cpi")]
use anchor_lang::{prelude::AccountInfo, Key};
use zama_host::{
    CoprocessorInputAttestation, FheBinaryOpCode, FheExecuteOperand, FheExecuteOutput,
    FheExecuteStep, FheUnaryOpCode, MAX_FHE_EXECUTION_STEPS,
};

fn handle(tag: u8) -> [u8; 32] {
    [tag; 32]
}

fn typed_handle(tag: u8, fhe_type: u8) -> [u8; 32] {
    let mut handle = [tag; 32];
    handle[30] = fhe_type;
    handle
}

fn balance_handle(tag: u8) -> [u8; 32] {
    typed_handle(tag, 5)
}

fn execution_authority(pubkey: Pubkey) -> ExecutionAuthority {
    ExecutionAuthority::new(pubkey)
}

#[cfg(feature = "cpi")]
fn account_info(pubkey: Pubkey, is_writable: bool) -> AccountInfo<'static> {
    let key = Box::leak(Box::new(pubkey));
    let owner = Box::leak(Box::new(Pubkey::new_unique()));
    let lamports = Box::leak(Box::new(0));
    let data = Box::leak(Vec::new().into_boxed_slice());
    AccountInfo::new(key, false, is_writable, lamports, data, owner, false)
}

/// The one application every test value belongs to: an execution may not mix two.
fn app() -> AppScope {
    AppScope {
        program: Pubkey::new_from_array([0xA9; 32]),
        scope: [0xA5; 32],
    }
}

#[derive(Clone)]
struct TestStateSlot {
    state: StateId,
    key: [u8; 32],
}

impl TestStateSlot {
    fn new(app: AppScope, authority: Pubkey, key: [u8; 32]) -> Self {
        Self {
            state: StateId::new(app.program, authority, app.scope),
            key,
        }
    }

    fn address(&self) -> Pubkey {
        self.state.address()
    }
}

fn state_output(key: TestStateSlot) -> StateOutput {
    let account = zama_host::EncryptedState {
        program: key.state.app().program,
        authority: key.state.authority(),
        scope: key.state.app().scope,
        slots: vec![],
        leaf_count: 0,
        peaks: vec![],
        bump: 0,
    };
    crate::State::new(&account).set(key.key)
}

fn state_slot_operand(handle: [u8; 32], key: &TestStateSlot) -> Operand {
    Operand(OperandKind::StateSlot {
        state: key.state,
        key: key.key,
        handle,
    })
}

fn typed_state_slot<T: FheTyped>(
    handle: [u8; 32],
    key: TestStateSlot,
) -> crate::Result<FheHandle<T>> {
    let account = zama_host::EncryptedState {
        program: key.state.app().program,
        authority: key.state.authority(),
        scope: key.state.app().scope,
        slots: vec![zama_host::EncryptedSlot {
            key: key.key,
            handle,
        }],
        leaf_count: 0,
        peaks: vec![],
        bump: 0,
    };
    crate::State::new(&account).get(key.key)
}

fn test_state_slot(account: Pubkey, label_tag: u8) -> TestStateSlot {
    TestStateSlot::new(app(), account, handle(label_tag))
}

fn foreign_operand(handle: [u8; 32]) -> Operand {
    state_slot_operand(handle, &test_state_slot(Pubkey::new_unique(), 0xEE))
}

fn scalar_operand_u64(value: u64) -> Operand {
    Operand::scalar(Scalar::<Uint<64>>::u64(value).bytes())
}

fn dummy_attestation(input_handle: [u8; 32], contract: Pubkey) -> CoprocessorInputAttestation {
    CoprocessorInputAttestation {
        input_handle,
        ct_handles: vec![input_handle],
        handle_index: 0,
        user_address: Pubkey::new_unique().to_bytes(),
        contract_address: contract.to_bytes(),
        contract_chain_id: 1,
        extra_data: vec![],
        signatures: vec![[0u8; 65]],
    }
}

#[test]
fn batch_build_runs_closure_and_finishes_batch() {
    let primary_authority = Pubkey::new_unique();
    let input_key = test_state_slot(primary_authority, 1);
    let input_acl = input_key.address();
    let output_key = test_state_slot(primary_authority, 7);
    let output_acl = output_key.address();
    let balance = typed_state_slot::<Uint<64>>(balance_handle(1), input_key).unwrap();

    let execution = FheExecution::build(execution_authority(primary_authority), |builder| {
        let incremented = builder.add(balance, Scalar::<Uint<64>>::u64(1), Output::transient())?;
        builder.add(
            incremented,
            Scalar::<Uint<64>>::u64(2),
            Output::state(state_output(output_key).allow(primary_authority)),
        )?;
        Ok(())
    })
    .unwrap();

    assert_eq!(execution.authority().pubkey(), primary_authority);
    assert_eq!(input_acl, output_acl);
    assert_eq!(execution.remaining_accounts.len(), 1);
    assert_eq!(execution.remaining_accounts[0].pubkey, input_acl);
    assert!(execution.remaining_accounts[0].is_writable);
    assert_eq!(
        execution.remaining_accounts[0].purposes.as_slice(),
        &[
            ExecutionAccountPurpose::StateInput,
            ExecutionAccountPurpose::StateOutput,
        ]
    );
    assert_eq!(execution.args.steps.len(), 2);
    match &execution.args.steps[1] {
        FheExecuteStep::Binary { lhs, output, .. } => {
            assert_eq!(*lhs, FheExecuteOperand::EarlierStep { producer_index: 0 });
            match output {
                FheExecuteOutput::State { state_index, .. } => {
                    assert_eq!(*state_index, 0);
                }
                other => panic!("unexpected output: {other:?}"),
            }
        }
        other => panic!("unexpected step: {other:?}"),
    }
}

#[test]
fn builder_rejects_reading_a_state_slot_after_writing_that_slot() {
    let authority = Pubkey::new_unique();
    let key = test_state_slot(authority, 7);
    let mut builder = FheExecutionBuilder::new(execution_authority(authority));
    builder
        .trivial_encrypt_u64(7, Output::state(state_output(key.clone()).allow(authority)))
        .unwrap();

    let reconstructed = typed_state_slot::<Uint<64>>(balance_handle(99), key).unwrap();
    let error = builder
        .add(
            reconstructed,
            Scalar::<Uint<64>>::u64(1),
            Output::transient(),
        )
        .unwrap_err();

    assert_eq!(error, FheExecutionBuildError::StateSlotWrittenEarlier);
}

#[test]
fn batch_build_lowers_verified_input_operand() {
    let primary_authority = Pubkey::new_unique();
    let output_key = test_state_slot(primary_authority, 7);
    let output_acl = output_key.address();
    let input_handle = balance_handle(2);
    let attestation = dummy_attestation(input_handle, primary_authority);

    let execution = FheExecution::build(execution_authority(primary_authority), |builder| {
        let amount = builder.verified_input::<Uint<64>>(attestation.clone())?;
        builder.add(
            amount,
            Scalar::<Uint<64>>::u64(1),
            Output::state(state_output(output_key).allow(primary_authority)),
        )?;
        Ok(())
    })
    .unwrap();

    assert_eq!(execution.args.steps.len(), 1);
    match &execution.args.steps[0] {
        FheExecuteStep::Binary { lhs, rhs, .. } => {
            assert_eq!(
                *lhs,
                FheExecuteOperand::VerifiedInput {
                    attestation: Box::new(attestation.clone())
                }
            );
            assert_eq!(*rhs, FheExecuteOperand::Scalar { value_index: 0 });
            assert_eq!(
                execution.args.dictionary_bytes(0).unwrap(),
                Scalar::<Uint<64>>::u64(1).bytes()
            );
        }
        other => panic!("unexpected step: {other:?}"),
    }
    // A verified input carries no remaining account: the attestation is inline in the operand.
    assert_eq!(
        execution.remaining_accounts,
        vec![ExecutionAccountMeta::writable(
            output_acl,
            ExecutionAccountPurpose::StateOutput
        )]
    );
}

#[test]
fn verified_input_rejects_type_mismatch() {
    let primary_authority = Pubkey::new_unique();
    // Input handle typed as BOOL (0) but requested as Uint64: caught at build time.
    let attestation = dummy_attestation(typed_handle(2, 0), primary_authority);
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    assert_eq!(
        builder.verified_input::<Uint<64>>(attestation).unwrap_err(),
        FheExecutionBuildError::UnsupportedFheType
    );
}

#[test]
fn batch_build_propagates_closure_and_finish_errors() {
    let primary_authority = Pubkey::new_unique();
    let error = match FheExecution::build(execution_authority(primary_authority), |builder| {
        builder.binary_op(
            FheBinaryOpCode::Ge,
            foreign_operand(balance_handle(1)),
            scalar_operand_u64(2),
            FheType::UINT64,
            Output::transient(),
        )?;
        Ok(())
    }) {
        Ok(_) => panic!("invalid execution unexpectedly built"),
        Err(error) => error,
    };
    assert_eq!(error, FheExecutionBuildError::UnsupportedBinaryOutputType);

    let error = match FheExecution::build(execution_authority(primary_authority), |_builder| Ok(()))
    {
        Ok(_) => panic!("empty execution unexpectedly built"),
        Err(error) => error,
    };
    assert_eq!(error, FheExecutionBuildError::EmptySteps);
}

#[test]
fn finish_preflights_lowered_remaining_account_indices() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    builder.dictionary.push(balance_handle(1));
    builder.dictionary.push(Scalar::<Uint<64>>::u64(1).bytes());
    builder.steps.push(FheExecuteStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: FheExecuteOperand::StateSlot {
            handle_index: 0,
            state_index: 0,
            key_index: 0,
        },
        rhs: FheExecuteOperand::Scalar { value_index: 1 },
        output_fhe_type: FheType::UINT64.byte(),
        output: FheExecuteOutput::Transient,
    });
    builder.produced_types.push(FheType::UINT64.byte());

    assert_eq!(
        builder.finish().unwrap_err(),
        FheExecutionBuildError::InvalidRemainingAccountReference
    );
}

#[test]
fn finish_preflights_lowered_transient_order_and_account_uniqueness() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    builder.steps.push(FheExecuteStep::TrivialEncrypt {
        plaintext: Scalar::<Uint<64>>::u64(1).bytes(),
        fhe_type: FheType::UINT64.byte(),
        output: FheExecuteOutput::Transient,
    });
    builder.dictionary.push(Scalar::<Uint<64>>::u64(1).bytes());
    builder.steps.push(FheExecuteStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: FheExecuteOperand::EarlierStep { producer_index: 1 },
        rhs: FheExecuteOperand::Scalar { value_index: 0 },
        output_fhe_type: FheType::UINT64.byte(),
        output: FheExecuteOutput::Transient,
    });
    builder.produced_types.push(FheType::UINT64.byte());
    builder.produced_types.push(FheType::UINT64.byte());

    assert_eq!(
        builder.finish().unwrap_err(),
        FheExecutionBuildError::InvalidTransientReference
    );

    let input_key = test_state_slot(primary_authority, 1);
    let input_acl = input_key.address();
    let balance = typed_state_slot::<Uint<64>>(balance_handle(1), input_key).unwrap();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    builder
        .add(balance, Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap();
    builder
        .remaining_accounts
        .push(ExecutionAccountMeta::readonly(
            input_acl,
            ExecutionAccountPurpose::StateInput,
        ));

    assert_eq!(
        builder.finish().unwrap_err(),
        FheExecutionBuildError::InvalidRemainingAccountReference
    );
}

#[test]
fn finish_rejects_dictionary_entry_no_step_references() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    builder
        .trivial_encrypt(Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap();
    builder.dictionary.push([0xAA; 32]);

    assert_eq!(
        builder.finish().unwrap_err(),
        FheExecutionBuildError::UnreferencedDictionaryEntry
    );
}

#[test]
fn finish_rejects_dictionary_index_past_dictionary_end() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    builder
        .trivial_encrypt(Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap();
    builder.steps.push(FheExecuteStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: FheExecuteOperand::EarlierStep { producer_index: 0 },
        rhs: FheExecuteOperand::Scalar { value_index: 3 },
        output_fhe_type: FheType::UINT64.byte(),
        output: FheExecuteOutput::Transient,
    });
    builder.produced_types.push(FheType::UINT64.byte());

    assert_eq!(
        builder.finish().unwrap_err(),
        FheExecutionBuildError::DictionaryIndexOutOfBounds
    );
}

#[cfg(feature = "cpi")]
#[test]
fn resolve_accounts_requires_the_cpi_authority_witness() {
    let primary_authority = Pubkey::new_unique();
    let input_key = test_state_slot(primary_authority, 1);
    let output_key = test_state_slot(primary_authority, 7);
    let output_acl = output_key.address();
    let balance = typed_state_slot::<Uint<64>>(balance_handle(1), input_key).unwrap();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    builder
        .add(
            balance,
            Scalar::<Uint<64>>::u64(1),
            Output::state(state_output(output_key).allow(primary_authority)),
        )
        .unwrap();
    let execution = builder.finish().unwrap();

    // The execution's own authority is an output authority like any other: the caller passes its
    // account info, and leaving it out is an error rather than something the SDK fills in.
    let missing = execution
        .resolve_accounts(
            vec![account_info(output_acl, true)],
            Vec::<AccountInfo<'static>>::new(),
        )
        .unwrap_err();
    assert_eq!(
        missing,
        ExecutionAccountResolutionError::MissingStateAuthority {
            authority: ExecutionAuthorityRequirement {
                pubkey: primary_authority,
            }
        }
    );

    execution
        .resolve_accounts(
            vec![account_info(output_acl, true)],
            vec![account_info(primary_authority, false)],
        )
        .expect("resolves once the authority witness is supplied");
}

#[test]
fn lowers_mixed_batch_to_stable_remaining_account_indices() {
    let primary_authority = Pubkey::new_unique();
    let balance_key = test_state_slot(primary_authority, 1);
    let amount_key = test_state_slot(primary_authority, 2);
    let balance_acl = balance_key.address();
    let amount_acl = amount_key.address();
    let output_key = test_state_slot(primary_authority, 7);
    let output_acl = output_key.address();
    let balance = typed_state_slot::<Uint<64>>(balance_handle(1), balance_key).unwrap();
    let amount = typed_state_slot::<Uint<64>>(balance_handle(2), amount_key).unwrap();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    let success = builder.ge(balance, amount, Output::transient()).unwrap();
    let debit_candidate = builder.sub(balance, amount, Output::transient()).unwrap();
    builder
        .if_then_else(
            success,
            debit_candidate,
            balance,
            Output::state(state_output(output_key).allow(primary_authority)),
        )
        .unwrap();

    let execution = builder.finish().unwrap();
    assert_eq!(execution.authority().pubkey(), primary_authority);

    assert_eq!(balance_acl, amount_acl);
    assert_eq!(amount_acl, output_acl);
    assert_eq!(execution.remaining_accounts.len(), 1);
    assert_eq!(execution.remaining_accounts[0].pubkey, balance_acl);
    assert!(execution.remaining_accounts[0].is_writable);
    assert_eq!(
        execution.remaining_accounts[0].purposes.as_slice(),
        &[
            ExecutionAccountPurpose::StateInput,
            ExecutionAccountPurpose::StateOutput,
        ]
    );
    assert_eq!(execution.args.steps.len(), 3);
    match &execution.args.steps[0] {
        FheExecuteStep::Binary { op, output, .. } => {
            assert_eq!(*op, FheBinaryOpCode::Ge);
            assert_eq!(*output, FheExecuteOutput::Transient);
        }
        other => panic!("unexpected step: {other:?}"),
    }
    match &execution.args.steps[2] {
        FheExecuteStep::Ternary {
            control,
            if_true,
            if_false,
            output,
            ..
        } => {
            assert_eq!(
                *control,
                FheExecuteOperand::EarlierStep { producer_index: 0 }
            );
            assert_eq!(
                *if_true,
                FheExecuteOperand::EarlierStep { producer_index: 1 }
            );
            match if_false {
                FheExecuteOperand::StateSlot { state_index, .. } => {
                    assert_eq!(*state_index, 0)
                }
                other => panic!("unexpected if_false: {other:?}"),
            }
            match output {
                FheExecuteOutput::State { state_index, .. } => {
                    assert_eq!(*state_index, 0)
                }
                other => panic!("unexpected output: {other:?}"),
            }
        }
        other => panic!("unexpected step: {other:?}"),
    }
}

#[test]
fn dynamic_account_requirements_expose_order_roles_and_purposes() {
    let primary_authority = Pubkey::new_unique();
    let input_key = test_state_slot(primary_authority, 1);
    let input_acl = input_key.address();
    let extra_authority = Pubkey::new_unique();
    let output_key = test_state_slot(extra_authority, 7);
    let output_acl = output_key.address();
    let input = typed_state_slot::<Uint<64>>(balance_handle(1), input_key).unwrap();

    let execution = FheExecution::build(execution_authority(primary_authority), |builder| {
        builder.add(
            input,
            Scalar::<Uint<64>>::u64(2),
            Output::state(state_output(output_key).allow(extra_authority)),
        )?;
        Ok(())
    })
    .unwrap();

    let requirements = execution.dynamic_account_requirements().collect::<Vec<_>>();
    assert_eq!(
        requirements
            .iter()
            .map(ExecutionAccountRequirement::pubkey)
            .collect::<Vec<_>>(),
        vec![input_acl, output_acl, extra_authority]
    );
    assert_eq!(
        requirements[0].purposes(),
        &[ExecutionAccountPurpose::StateInput]
    );
    assert_eq!(
        requirements[1].purposes(),
        &[ExecutionAccountPurpose::StateOutput]
    );
    assert_eq!(
        requirements[2].purposes(),
        &[ExecutionAccountPurpose::StateAuthority]
    );
    assert!(requirements[1].is_writable());
    assert!(requirements[2].is_signer());
    assert!(!requirements[2].requires_dynamic_account());
    assert!(requirements[2].requires_state_authority());
}

#[test]
fn lowers_explicit_output_authority_witness() {
    let primary_authority = Pubkey::new_unique();
    let input_key = test_state_slot(primary_authority, 1);
    let acl_record = input_key.address();
    let authority = Pubkey::new_unique();
    let output_key = test_state_slot(authority, 7);
    let output_acl = output_key.address();
    let balance = typed_state_slot::<Uint<64>>(balance_handle(1), input_key).unwrap();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    builder
        .add(
            balance,
            Scalar::<Uint<64>>::u64(2),
            Output::state(state_output(output_key).allow(authority)),
        )
        .unwrap();

    let execution = builder.finish().unwrap();
    assert_eq!(execution.authority().pubkey(), primary_authority);
    assert_eq!(
        execution.remaining_accounts,
        vec![
            ExecutionAccountMeta::readonly(acl_record, ExecutionAccountPurpose::StateInput),
            ExecutionAccountMeta::writable(output_acl, ExecutionAccountPurpose::StateOutput),
            ExecutionAccountMeta::readonly_signer(
                authority,
                ExecutionAccountPurpose::StateAuthority,
            ),
        ]
    );
    assert_eq!(
        execution.additional_value_authorities().collect::<Vec<_>>(),
        vec![authority]
    );
    let authority_requirements = execution.state_authority_requirements().collect::<Vec<_>>();
    assert_eq!(
        authority_requirements,
        vec![
            ExecutionAuthorityRequirement {
                pubkey: primary_authority,
            },
            ExecutionAuthorityRequirement { pubkey: authority },
        ]
    );
    match &execution.args.steps[0] {
        FheExecuteStep::Binary { output, .. } => match output {
            FheExecuteOutput::State { state_index, .. } => {
                assert_eq!(*state_index, 1);
            }
            other => panic!("unexpected output: {other:?}"),
        },
        other => panic!("unexpected step: {other:?}"),
    }
}

#[cfg(feature = "cpi")]
#[test]
fn resolve_accounts_orders_and_validates_batch_requirements() {
    let primary_authority = Pubkey::new_unique();
    let input_key = test_state_slot(primary_authority, 1);
    let input_acl = input_key.address();
    let extra_authority = Pubkey::new_unique();
    let output_key = test_state_slot(extra_authority, 7);
    let output_acl = output_key.address();
    let input = typed_state_slot::<Uint<64>>(balance_handle(1), input_key).unwrap();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    builder
        .add(
            input,
            Scalar::<Uint<64>>::u64(2),
            Output::state(state_output(output_key).allow(extra_authority)),
        )
        .unwrap();
    let execution = builder.finish().unwrap();

    let resolved = execution
        .resolve_accounts(
            vec![
                account_info(output_acl, true),
                account_info(input_acl, false),
            ],
            vec![
                account_info(primary_authority, false),
                account_info(extra_authority, false),
            ],
        )
        .unwrap();
    assert_eq!(
        resolved
            .account_infos()
            .iter()
            .map(|account| account.key())
            .collect::<Vec<_>>(),
        vec![input_acl, output_acl, extra_authority]
    );

    let duplicate = execution
        .resolve_accounts(
            vec![
                account_info(input_acl, false),
                account_info(input_acl, false),
            ],
            vec![
                account_info(primary_authority, false),
                account_info(extra_authority, false),
            ],
        )
        .unwrap_err();
    assert_eq!(
        duplicate,
        ExecutionAccountResolutionError::DuplicateDynamicAccount { pubkey: input_acl }
    );

    let unexpected = execution
        .resolve_accounts(
            vec![account_info(Pubkey::new_unique(), false)],
            vec![
                account_info(primary_authority, false),
                account_info(extra_authority, false),
            ],
        )
        .unwrap_err();
    assert!(matches!(
        unexpected,
        ExecutionAccountResolutionError::UnexpectedDynamicAccount { .. }
    ));

    let missing = execution
        .resolve_accounts(
            vec![account_info(output_acl, true)],
            vec![
                account_info(primary_authority, false),
                account_info(extra_authority, false),
            ],
        )
        .unwrap_err();
    assert!(matches!(
        missing,
        ExecutionAccountResolutionError::MissingDynamicAccount { requirement }
            if requirement.pubkey() == input_acl
    ));

    let readonly = execution
        .resolve_accounts(
            vec![
                account_info(input_acl, false),
                account_info(output_acl, false),
            ],
            vec![
                account_info(primary_authority, false),
                account_info(extra_authority, false),
            ],
        )
        .unwrap_err();
    assert!(matches!(
        readonly,
        ExecutionAccountResolutionError::DynamicAccountNotWritable { requirement }
            if requirement.pubkey() == output_acl
    ));

    let duplicate_authority = execution
        .resolve_accounts(
            vec![
                account_info(input_acl, false),
                account_info(output_acl, true),
            ],
            vec![
                account_info(primary_authority, false),
                account_info(extra_authority, false),
                account_info(extra_authority, false),
            ],
        )
        .unwrap_err();
    assert_eq!(
        duplicate_authority,
        ExecutionAccountResolutionError::DuplicateStateAuthority {
            pubkey: extra_authority
        }
    );

    let unexpected_authority = execution
        .resolve_accounts(
            vec![
                account_info(input_acl, false),
                account_info(output_acl, true),
            ],
            vec![
                account_info(primary_authority, false),
                account_info(extra_authority, false),
                account_info(Pubkey::new_unique(), false),
            ],
        )
        .unwrap_err();
    assert!(matches!(
        unexpected_authority,
        ExecutionAccountResolutionError::UnexpectedStateAuthority { .. }
    ));

    let missing_authority = execution
        .resolve_accounts(
            vec![
                account_info(input_acl, false),
                account_info(output_acl, true),
            ],
            vec![account_info(primary_authority, false)],
        )
        .unwrap_err();
    assert_eq!(
        missing_authority,
        ExecutionAccountResolutionError::MissingStateAuthority {
            authority: ExecutionAuthorityRequirement {
                pubkey: extra_authority,
            }
        }
    );
}

#[cfg(feature = "cpi")]
#[test]
fn resolve_accounts_rejects_known_accounts_in_wrong_bucket() {
    let primary_authority = Pubkey::new_unique();
    let input_key = test_state_slot(primary_authority, 1);
    let input_acl = input_key.address();
    let extra_authority = Pubkey::new_unique();
    let output_key = test_state_slot(extra_authority, 7);
    let output_acl = output_key.address();
    let input = typed_state_slot::<Uint<64>>(balance_handle(1), input_key).unwrap();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    builder
        .add(
            input,
            Scalar::<Uint<64>>::u64(2),
            Output::state(state_output(output_key).allow(extra_authority)),
        )
        .unwrap();
    let execution = builder.finish().unwrap();

    let authority_in_dynamic_bucket = execution
        .resolve_accounts(
            vec![
                account_info(input_acl, false),
                account_info(output_acl, true),
                account_info(extra_authority, false),
            ],
            vec![
                account_info(primary_authority, false),
                account_info(extra_authority, false),
            ],
        )
        .unwrap_err();
    assert_eq!(
        authority_in_dynamic_bucket,
        ExecutionAccountResolutionError::UnexpectedDynamicAccount {
            pubkey: extra_authority
        }
    );

    let input_acl_in_authority_bucket = execution
        .resolve_accounts(
            vec![
                account_info(input_acl, false),
                account_info(output_acl, true),
            ],
            vec![
                account_info(primary_authority, false),
                account_info(extra_authority, false),
                account_info(input_acl, false),
            ],
        )
        .unwrap_err();
    assert_eq!(
        input_acl_in_authority_bucket,
        ExecutionAccountResolutionError::UnexpectedStateAuthority { pubkey: input_acl }
    );

    let output_acl_in_authority_bucket = execution
        .resolve_accounts(
            vec![
                account_info(input_acl, false),
                account_info(output_acl, true),
            ],
            vec![
                account_info(primary_authority, false),
                account_info(extra_authority, false),
                account_info(output_acl, false),
            ],
        )
        .unwrap_err();
    assert_eq!(
        output_acl_in_authority_bucket,
        ExecutionAccountResolutionError::UnexpectedStateAuthority { pubkey: output_acl }
    );
}

#[test]
fn lowers_create_steps() {
    let primary_authority = Pubkey::new_unique();
    let output_key = test_state_slot(primary_authority, 7);
    let output_acl = output_key.address();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    let trivial = builder.trivial_encrypt_u64(1, Output::transient()).unwrap();
    builder
        .rand_u64(Output::state(
            state_output(output_key).allow(primary_authority),
        ))
        .unwrap();
    builder
        .add(trivial, Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap();

    let execution = builder.finish().unwrap();
    assert_eq!(
        execution.remaining_accounts,
        vec![ExecutionAccountMeta::writable(
            output_acl,
            ExecutionAccountPurpose::StateOutput
        )]
    );
    assert!(matches!(
        execution.args.steps[0],
        FheExecuteStep::TrivialEncrypt { .. }
    ));
    assert!(matches!(
        execution.args.steps[1],
        FheExecuteStep::Rand { .. }
    ));
}

#[test]
fn rejects_invalid_references_and_types() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    let error = builder
        .binary_op(
            FheBinaryOpCode::Add,
            Operand::transient(0),
            scalar_operand_u64(1),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    assert_eq!(error, FheExecutionBuildError::InvalidTransientReference);

    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    let error = builder
        .binary_op(
            FheBinaryOpCode::Ge,
            foreign_operand(balance_handle(1)),
            scalar_operand_u64(2),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    assert_eq!(error, FheExecutionBuildError::UnsupportedBinaryOutputType);

    let input_key = test_state_slot(primary_authority, 1);
    let input = typed_state_slot::<Uint<64>>(balance_handle(1), input_key).unwrap();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    builder.trivial_encrypt_u64(1, Output::transient()).unwrap();
    let current_index = builder
        .binary_op(
            FheBinaryOpCode::Add,
            Operand::transient(1),
            scalar_operand_u64(1),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    assert_eq!(
        current_index,
        FheExecutionBuildError::InvalidTransientReference
    );

    let future_index = builder
        .binary_op(
            FheBinaryOpCode::Add,
            Operand::transient(9),
            scalar_operand_u64(1),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    assert_eq!(
        future_index,
        FheExecutionBuildError::InvalidTransientReference
    );

    let invalid_rhs = builder
        .binary_op(
            FheBinaryOpCode::Add,
            Encrypted::from(input).operand(),
            Operand::transient(1),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    assert_eq!(
        invalid_rhs,
        FheExecutionBuildError::InvalidTransientReference
    );
}

#[test]
fn validates_execution_authority_pubkey() {
    let mut builder = FheExecutionBuilder::new(execution_authority(Pubkey::default()));
    builder.trivial_encrypt_u64(1, Output::transient()).unwrap();
    let error = match builder.finish() {
        Ok(_) => panic!("invalid encrypted State authority unexpectedly built"),
        Err(error) => error,
    };
    assert_eq!(error, FheExecutionBuildError::InvalidExecutionAuthority);
}

#[test]
fn binary_validation_rejects_host_type_mismatches() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    let bool_lhs = foreign_operand(typed_handle(1, FheType::BOOL.byte()));
    let error = builder
        .binary_op(
            FheBinaryOpCode::Add,
            bool_lhs,
            scalar_operand_u64(1),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    // Add gates its output to uint types, and the operand must equal that output type, so a
    // Bool lhs against a Uint64 output is a type mismatch (host + client agree).
    assert_eq!(error, FheExecutionBuildError::BinaryOperandTypeMismatch);

    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    let error = builder
        .binary_op(
            FheBinaryOpCode::Add,
            foreign_operand(balance_handle(1)),
            foreign_operand(typed_handle(2, FheType::UINT32.byte())),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    assert_eq!(error, FheExecutionBuildError::BinaryOperandTypeMismatch);
}

#[test]
fn unary_validation_rejects_same_type_cast_and_bad_operand_types() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    // A cast to a different type is accepted.
    assert!(builder
        .unary_op(
            FheUnaryOpCode::Cast,
            foreign_operand(balance_handle(1)),
            FheType::UINT32,
            Output::transient(),
        )
        .is_ok());
    // A same-type cast is rejected (EVM InvalidType parity).
    assert_eq!(
        builder
            .unary_op(
                FheUnaryOpCode::Cast,
                foreign_operand(balance_handle(1)),
                FheType::UINT64,
                Output::transient(),
            )
            .unwrap_err(),
        FheExecutionBuildError::UnsupportedFheType
    );
    // EVM cast type sets: a Bool input casts to a uint (Bool -> Uint32) is accepted...
    assert!(builder
        .unary_op(
            FheUnaryOpCode::Cast,
            foreign_operand(typed_handle(2, FheType::BOOL.byte())),
            FheType::UINT32,
            Output::transient(),
        )
        .is_ok());
    // ...but casting TO ebool, TO type 7, FROM type 7, or FROM/TO type 8 is rejected.
    assert_eq!(
        builder
            .unary_op(
                FheUnaryOpCode::Cast,
                foreign_operand(balance_handle(1)),
                FheType::BOOL,
                Output::transient(),
            )
            .unwrap_err(),
        FheExecutionBuildError::UnsupportedFheType
    );
    assert_eq!(
        validate_unary_step(
            FheUnaryOpCode::Cast,
            &foreign_operand(balance_handle(1)),
            7u8,
            0,
            |_| None,
        )
        .unwrap_err(),
        FheExecutionBuildError::UnsupportedFheType
    );
    assert_eq!(
        builder
            .unary_op(
                FheUnaryOpCode::Cast,
                foreign_operand(typed_handle(3, 7u8)),
                FheType::UINT64,
                Output::transient(),
            )
            .unwrap_err(),
        FheExecutionBuildError::UnsupportedFheType
    );
    assert_eq!(
        builder
            .unary_op(
                FheUnaryOpCode::Cast,
                foreign_operand(typed_handle(4, 8u8)),
                FheType::UINT64,
                Output::transient(),
            )
            .unwrap_err(),
        FheExecutionBuildError::UnsupportedFheType
    );
    assert_eq!(
        validate_unary_step(
            FheUnaryOpCode::Cast,
            &foreign_operand(balance_handle(1)),
            8u8,
            0,
            |_| None,
        )
        .unwrap_err(),
        FheExecutionBuildError::UnsupportedFheType
    );
    // Neg rejects a Bool operand (EVM fheNeg supportedTypes = Uint8..Uint128).
    assert_eq!(
        builder
            .unary_op(
                FheUnaryOpCode::Neg,
                foreign_operand(typed_handle(1, FheType::BOOL.byte())),
                FheType::BOOL,
                Output::transient(),
            )
            .unwrap_err(),
        FheExecutionBuildError::UnsupportedFheType
    );
}

#[test]
fn mul_div_rejects_zero_divisor() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    let balance =
        typed_state_slot::<Uint<64>>(balance_handle(1), test_state_slot(primary_authority, 1))
            .unwrap();
    assert_eq!(
        builder
            .mul_div(
                balance,
                Scalar::<Uint<64>>::u64(3),
                Scalar::<Uint<64>>::u64(0),
                Output::transient(),
            )
            .unwrap_err(),
        FheExecutionBuildError::MulDivDivisorZero
    );
}

#[test]
fn div_rem_require_nonzero_scalar_divisor() {
    let auth = Pubkey::new_unique();
    let mut builder = FheExecutionBuilder::new(execution_authority(auth));
    // Encrypted divisor is rejected — division is scalar-only (EVM `IsNotScalar`).
    let lhs = typed_state_slot::<Uint<64>>(balance_handle(1), test_state_slot(auth, 1)).unwrap();
    let enc_divisor =
        typed_state_slot::<Uint<64>>(balance_handle(2), test_state_slot(auth, 2)).unwrap();
    assert_eq!(
        builder
            .div(lhs, enc_divisor, Output::transient())
            .unwrap_err(),
        FheExecutionBuildError::DivisorMustBeScalar
    );
    // A zero scalar divisor is rejected.
    let lhs2 = typed_state_slot::<Uint<64>>(balance_handle(1), test_state_slot(auth, 1)).unwrap();
    assert_eq!(
        builder
            .rem(lhs2, Scalar::<Uint<64>>::u64(0), Output::transient())
            .unwrap_err(),
        FheExecutionBuildError::DivisionByZero
    );
    // A non-zero scalar divisor is accepted.
    let lhs3 = typed_state_slot::<Uint<64>>(balance_handle(1), test_state_slot(auth, 1)).unwrap();
    assert!(builder
        .div(lhs3, Scalar::<Uint<64>>::u64(3), Output::transient())
        .is_ok());
}

#[test]
fn builder_exposes_the_host_operator_type_surface() {
    // The typed builder must express the host's type matrix: bitwise/eq on Bool. Types 7 and 8
    // are outside the type gate (Solana host max is euint128).
    let auth = Pubkey::new_unique();
    let mut builder = FheExecutionBuilder::new(execution_authority(auth));

    let bool_a = typed_state_slot::<Bool>(
        typed_handle(1, FheType::BOOL.byte()),
        test_state_slot(auth, 1),
    )
    .unwrap();
    let bool_b = typed_state_slot::<Bool>(
        typed_handle(2, FheType::BOOL.byte()),
        test_state_slot(auth, 2),
    )
    .unwrap();
    assert!(builder.and(bool_a, bool_b, Output::transient()).is_ok());

    let bool_c = typed_state_slot::<Bool>(
        typed_handle(6, FheType::BOOL.byte()),
        test_state_slot(auth, 6),
    )
    .unwrap();
    let bool_d = typed_state_slot::<Bool>(
        typed_handle(7, FheType::BOOL.byte()),
        test_state_slot(auth, 7),
    )
    .unwrap();
    assert!(builder.eq(bool_c, bool_d, Output::transient()).is_ok());

    let u256 = foreign_operand(typed_handle(3, 8u8));
    assert_eq!(
        validate_binary_step(FheBinaryOpCode::Xor, &u256, &u256, 8u8, 0, |_| None).unwrap_err(),
        FheExecutionBuildError::UnsupportedFheType
    );
    assert_eq!(
        validate_unary_step(FheUnaryOpCode::Neg, &u256, 8u8, 0, |_| None).unwrap_err(),
        FheExecutionBuildError::UnsupportedFheType
    );
    assert_eq!(
        FheType::from_host_byte(7u8).unwrap_err(),
        FheExecutionBuildError::UnsupportedFheType
    );
    assert_eq!(
        FheType::from_host_byte(8u8).unwrap_err(),
        FheExecutionBuildError::UnsupportedFheType
    );
}

/// Reading a value whose authority is not the execution's fixed signer needs that authority to
/// sign: the builder adds the signer slot and the CPI resolver demands the witness.
#[test]
fn state_slot_with_its_own_authority_requires_that_signer() {
    let primary_authority = Pubkey::new_unique();
    let other_authority = Pubkey::new_unique();
    let input_key = test_state_slot(other_authority, 1);
    let input = typed_state_slot::<Uint<64>>(balance_handle(1), input_key.clone()).unwrap();
    let execution = FheExecution::build(execution_authority(primary_authority), |builder| {
        builder.add(input, Scalar::<Uint<64>>::u64(1), Output::transient())?;
        Ok(())
    })
    .unwrap();
    assert_eq!(
        execution.remaining_accounts,
        vec![
            ExecutionAccountMeta::readonly(
                input_key.address(),
                ExecutionAccountPurpose::StateInput
            ),
            ExecutionAccountMeta::readonly_signer(
                other_authority,
                ExecutionAccountPurpose::StateAuthority,
            ),
        ]
    );
    assert_eq!(
        execution.additional_value_authorities().collect::<Vec<_>>(),
        vec![other_authority]
    );
}

/// One execution, one application: a second `(program, scope)` is refused where it appears,
/// whether as an operand or as an output, and the builder is left untouched.
#[test]
fn mixed_scopes_are_rejected_at_the_step_that_mixes() {
    let authority = Pubkey::new_unique();
    let other_app = AppScope {
        program: Pubkey::new_unique(),
        scope: [7; 32],
    };
    let same =
        typed_state_slot::<Uint<64>>(balance_handle(1), test_state_slot(authority, 1)).unwrap();
    let other = typed_state_slot::<Uint<64>>(
        balance_handle(2),
        TestStateSlot::new(other_app, authority, [2; 32]),
    )
    .unwrap();
    let mut builder = FheExecutionBuilder::new(execution_authority(authority));
    builder
        .add(same, Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap();
    let steps_before = builder.steps.len();
    assert_eq!(
        builder
            .add(other, Scalar::<Uint<64>>::u64(1), Output::transient())
            .unwrap_err(),
        FheExecutionBuildError::MixedScopes
    );
    assert_eq!(
        builder
            .trivial_encrypt_u64(
                1,
                Output::state(state_output(TestStateSlot::new(
                    other_app, authority, [3; 32],
                ))),
            )
            .unwrap_err(),
        FheExecutionBuildError::MixedScopes
    );
    assert_eq!(builder.steps.len(), steps_before);
    assert_eq!(builder.finish().unwrap().app(), Some(app()));
}

#[test]
fn transient_only_execution_has_no_application() {
    let execution = FheExecution::build(execution_authority(Pubkey::new_unique()), |builder| {
        builder.trivial_encrypt_u64(1, Output::transient())?;
        Ok(())
    })
    .unwrap();
    assert_eq!(execution.app(), None);
    assert!(!execution.has_rand_step());
}

#[test]
fn typed_handle_constructor_rejects_mismatched_handle_tag() {
    let error = typed_state_slot::<Uint<64>>(
        typed_handle(1, FheType::UINT32.byte()),
        test_state_slot(Pubkey::new_unique(), 7),
    )
    .unwrap_err();
    assert_eq!(error, FheExecutionBuildError::UnsupportedFheType);
}

#[test]
fn state_slot_rejects_unsupported_handle_type_bytes() {
    let key = test_state_slot(Pubkey::new_unique(), 1);
    assert_eq!(
        typed_state_slot::<Uint<64>>(typed_handle(1, 7u8), key.clone()).unwrap_err(),
        FheExecutionBuildError::UnsupportedFheType
    );
    assert_eq!(
        typed_state_slot::<Uint<64>>(typed_handle(1, 8u8), key).unwrap_err(),
        FheExecutionBuildError::UnsupportedFheType
    );
}

#[test]
fn rand_rejects_unsupported_type_like_host() {
    let mut builder = FheExecutionBuilder::new(execution_authority(Pubkey::new_unique()));
    let error = builder.rand_raw(7u8, Output::transient()).unwrap_err();
    assert_eq!(error, FheExecutionBuildError::UnsupportedFheType);
    let error = builder.rand_raw(8u8, Output::transient()).unwrap_err();
    assert_eq!(error, FheExecutionBuildError::UnsupportedFheType);
}

/// `rand_bounded_u64` is the builder's only typed constructor for `FheExecuteStep::RandBounded`, an
/// operator the host still executes (`operator_conformance.rs` covers evaluation, but it hand-builds
/// the step rather than going through the builder). Its one in-repo caller used to be the token
/// program's PoC `create_random_bounded_amount` helper; when that went, this became untested SBF-facing
/// API — so the step it lowers, and the upper-bound rejection it inherits from the host, are pinned here.
#[test]
fn lowers_bounded_rand_step_and_rejects_non_power_of_two_bounds() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    builder
        .rand_bounded_u64(
            BoundedU64UpperBound::power_of_two(1 << 20).unwrap(),
            Output::transient(),
        )
        .unwrap();
    let execution = builder.finish().unwrap();
    // A rand step needs the host's nonce account on the invoke; the execution says so.
    assert!(execution.has_rand_step());
    let mut expected_bound = [0u8; 32];
    expected_bound[24..].copy_from_slice(&(1u64 << 20).to_be_bytes());
    assert!(matches!(
        execution.args.steps[0],
        FheExecuteStep::RandBounded {
            upper_bound,
            fhe_type,
            ..
        } if upper_bound == expected_bound && fhe_type == FheType::UINT64.byte()
    ));

    assert_eq!(
        BoundedU64UpperBound::power_of_two(3).unwrap_err(),
        FheExecutionBuildError::InvalidRandomUpperBound
    );
}

#[test]
fn finish_rejects_empty_steps() {
    let primary_authority = Pubkey::new_unique();
    assert!(matches!(
        FheExecutionBuilder::new(execution_authority(primary_authority)).finish(),
        Err(FheExecutionBuildError::EmptySteps)
    ));
}

#[test]
fn rejects_more_than_max_ops() {
    let primary_authority = Pubkey::new_unique();
    let input_key = test_state_slot(primary_authority, 1);
    let balance = typed_state_slot::<Uint<64>>(balance_handle(1), input_key).unwrap();
    let mut builder = FheExecutionBuilder::new(execution_authority(primary_authority));
    for index in 0..MAX_FHE_EXECUTION_STEPS {
        builder
            .add(
                balance,
                Scalar::<Uint<64>>::u64(index as u64),
                Output::transient(),
            )
            .unwrap();
    }
    let error = builder
        .add(balance, Scalar::<Uint<64>>::u64(99), Output::transient())
        .unwrap_err();
    assert_eq!(error, FheExecutionBuildError::TooManySteps);
}

#[test]
fn step_tables_rollback_undoes_promotions_and_appends() {
    let shared = Pubkey::new_unique();
    let mut budget = crate::heap_tally::HeapBudget::new();
    let mut remaining_accounts = crate::heap_tally::TalliedVec::new();
    remaining_accounts
        .try_push(
            &mut budget,
            ExecutionAccountMeta::readonly(shared, ExecutionAccountPurpose::StateInput),
        )
        .unwrap();
    let mut dictionary = crate::heap_tally::TalliedVec::new();
    dictionary.try_push(&mut budget, handle(1)).unwrap();
    let mut persistent_producers = crate::heap_tally::TalliedVec::new();
    persistent_producers
        .try_push(&mut budget, (0, None))
        .unwrap();
    let accounts_before = remaining_accounts.clone();
    let dictionary_before = dictionary.clone();
    let producers_before = persistent_producers.clone();

    let mut tables = StepTables::open(
        &mut remaining_accounts,
        &mut dictionary,
        &mut persistent_producers,
        &mut budget,
    );
    // Promote the same entry twice — first writable, then signer — so undoing in the wrong order
    // would leave the entry with the flags the first promotion set.
    assert_eq!(
        tables
            .account_index(ExecutionAccountMeta::writable(
                shared,
                ExecutionAccountPurpose::StateOutput,
            ))
            .unwrap(),
        0
    );
    assert_eq!(
        tables
            .account_index(ExecutionAccountMeta::readonly_signer(
                shared,
                ExecutionAccountPurpose::StateAuthority,
            ))
            .unwrap(),
        0
    );
    assert_eq!(
        tables
            .account_index(ExecutionAccountMeta::readonly(
                Pubkey::new_unique(),
                ExecutionAccountPurpose::StateInput,
            ))
            .unwrap(),
        1
    );
    assert_eq!(tables.dictionary_index(handle(2)).unwrap(), 1);
    assert_eq!(tables.dictionary_index(handle(1)).unwrap(), 0);
    // Uncommitted drop rolls back interned tables.
    drop(tables);

    assert_eq!(remaining_accounts, accounts_before);
    assert_eq!(dictionary, dictionary_before);
    assert_eq!(persistent_producers, producers_before);
}

#[test]
fn step_that_fails_after_interning_leaves_the_builder_untouched() {
    let authority = Pubkey::new_unique();
    let written_key = test_state_slot(authority, 7);
    let mut builder = FheExecutionBuilder::new(execution_authority(authority));
    builder
        .trivial_encrypt_u64(
            7,
            Output::state(state_output(written_key.clone()).allow(authority)),
        )
        .unwrap();
    let accounts_before = builder.remaining_accounts.clone();
    let dictionary_before = builder.dictionary.clone();
    let producers_before = builder.persistent_producers.clone();

    // The left operand interns a fresh handle and a fresh input-ACL account; the right operand then
    // fails because the step above already wrote its account.
    let fresh = typed_state_slot::<Uint<64>>(balance_handle(1), test_state_slot(authority, 1))
        .expect("fresh operand");
    let written =
        typed_state_slot::<Uint<64>>(balance_handle(2), written_key).expect("written operand");
    let error = builder
        .add(fresh, written, Output::transient())
        .unwrap_err();

    assert_eq!(error, FheExecutionBuildError::StateSlotWrittenEarlier);
    assert_eq!(builder.remaining_accounts, accounts_before);
    assert_eq!(builder.dictionary, dictionary_before);
    assert_eq!(builder.persistent_producers, producers_before);
    assert_eq!(builder.steps.len(), 1);
    assert_eq!(builder.produced_types.len(), 1);
}

#[test]
fn scalar_u64_uses_big_endian_low_bytes() {
    let mut expected = [0u8; 32];
    expected[24..].copy_from_slice(&0x0102_0304_0506_0708u64.to_be_bytes());
    assert_eq!(
        Scalar::<Uint<64>>::u64(0x0102_0304_0506_0708).bytes(),
        expected
    );
}
