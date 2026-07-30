//! Unit tests for the builder, validators, lowering, and CPI plumbing.

use crate::accounts::*;
use crate::acl::*;
use crate::builder::*;
#[cfg(feature = "cpi")]
use crate::cpi::*;
use crate::operand::*;
use crate::plan::*;
use crate::types::*;
use crate::EvalBuildError;
use anchor_lang::prelude::Pubkey;
#[cfg(feature = "cpi")]
use anchor_lang::{prelude::AccountInfo, Key};
use zama_host::{
    CoprocessorInputAttestation, FheBinaryOpCode, FheEvalOperand, FheEvalOutput, FheEvalStep,
    FheUnaryOpCode, MAX_ACL_SUBJECTS, MAX_FHE_EVAL_OPS,
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

fn app_authority(pubkey: Pubkey) -> EvalAppAuthority {
    EvalAppAuthority::new(pubkey)
}

#[cfg(feature = "cpi")]
fn account_info(pubkey: Pubkey, is_writable: bool) -> AccountInfo<'static> {
    let key = Box::leak(Box::new(pubkey));
    let owner = Box::leak(Box::new(Pubkey::new_unique()));
    let lamports = Box::leak(Box::new(0));
    let data = Box::leak(Vec::new().into_boxed_slice());
    AccountInfo::new(key, false, is_writable, lamports, data, owner, false)
}

fn durable_slot(account: Pubkey, label_tag: u8) -> DurableSlot {
    DurableSlot::new(
        Pubkey::new_unique(),
        account,
        DurableLabel::new(handle(label_tag)),
    )
}

fn access_policy(subject: Pubkey) -> AccessPolicy {
    AccessPolicy::for_owner(subject).unwrap()
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

#[cfg(feature = "cpi")]
fn cpi_accounts(app_authority: Pubkey) -> EvalCpiAccounts<'static, 'static> {
    EvalCpiAccounts {
        payer: account_info(Pubkey::new_unique(), true),
        compute_subject: account_info(Pubkey::new_unique(), false),
        app_account_authority: account_info(app_authority, false),
        host_config: account_info(Pubkey::new_unique(), false),
        deny_subject_records: &[],
        system_program: account_info(Pubkey::new_unique(), false),
        hcu_block_meter: None,
        hcu_trusted_app_record: None,
        event_authority: account_info(Pubkey::new_unique(), false),
        program: account_info(Pubkey::new_unique(), false),
    }
}

#[test]
fn eval_plan_build_runs_closure_and_finishes_plan() {
    let primary_authority = Pubkey::new_unique();
    let input_slot = durable_slot(primary_authority, 1);
    let input_acl = input_slot.address();
    let output_slot = durable_slot(primary_authority, 7);
    let output_acl = output_slot.address();
    let balance = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();

    let plan = EvalPlan::build(app_authority(primary_authority), |builder| {
        let incremented = builder.add(balance, Scalar::<Uint<64>>::u64(1), Output::transient())?;
        builder.add(
            incremented,
            Scalar::<Uint<64>>::u64(2),
            Output::durable(output_slot, access_policy(primary_authority)),
        )
    })
    .unwrap();

    assert_eq!(plan.app_authority().pubkey(), primary_authority);
    assert_eq!(
        plan.remaining_accounts,
        vec![
            EvalAccountMeta::readonly(input_acl, EvalAccountPurpose::DurableInputAcl),
            EvalAccountMeta::writable(output_acl, EvalAccountPurpose::DurableOutputAcl),
        ]
    );
    assert_eq!(plan.args.steps.len(), 2);
    match &plan.args.steps[1] {
        FheEvalStep::Binary { lhs, output, .. } => {
            assert_eq!(*lhs, FheEvalOperand::AllowedLocal { producer_index: 0 });
            match output {
                FheEvalOutput::AllowedDurable {
                    output_app_account_authority_index,
                    ..
                } => {
                    assert_eq!(*output_app_account_authority_index, None);
                }
                other => panic!("unexpected output: {other:?}"),
            }
        }
        other => panic!("unexpected step: {other:?}"),
    }
}

#[test]
fn eval_plan_build_lowers_verified_input_operand() {
    let primary_authority = Pubkey::new_unique();
    let output_slot = durable_slot(primary_authority, 7);
    let output_acl = output_slot.address();
    let input_handle = balance_handle(2);
    let attestation = dummy_attestation(input_handle, primary_authority);

    let plan = EvalPlan::build(app_authority(primary_authority), |builder| {
        let amount: Uint64Handle = builder.verified_input(attestation.clone())?;
        builder.add(
            amount,
            Scalar::<Uint<64>>::u64(1),
            Output::durable(output_slot, access_policy(primary_authority)),
        )
    })
    .unwrap();

    assert_eq!(plan.args.steps.len(), 1);
    match &plan.args.steps[0] {
        FheEvalStep::Binary { lhs, rhs, .. } => {
            assert_eq!(
                *lhs,
                FheEvalOperand::VerifiedInput {
                    attestation: Box::new(attestation.clone())
                }
            );
            assert_eq!(*rhs, FheEvalOperand::Scalar { value_index: 0 });
            assert_eq!(
                plan.args.pool_bytes(0).unwrap(),
                Scalar::<Uint<64>>::u64(1).bytes()
            );
        }
        other => panic!("unexpected step: {other:?}"),
    }
    // A verified input carries no remaining account: the attestation is inline in the operand.
    assert_eq!(
        plan.remaining_accounts,
        vec![EvalAccountMeta::writable(
            output_acl,
            EvalAccountPurpose::DurableOutputAcl
        )]
    );
}

#[test]
fn verified_input_rejects_type_mismatch() {
    let primary_authority = Pubkey::new_unique();
    // Input handle typed as BOOL (0) but requested as Uint64: caught at build time.
    let attestation = dummy_attestation(typed_handle(2, 0), primary_authority);
    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    assert_eq!(
        builder.verified_input::<Uint<64>>(attestation).unwrap_err(),
        EvalBuildError::UnsupportedFheType
    );
}

#[test]
fn eval_plan_build_propagates_closure_and_finish_errors() {
    let primary_authority = Pubkey::new_unique();
    let error = match EvalPlan::build(app_authority(primary_authority), |builder| {
        builder.binary_op(
            FheBinaryOpCode::Ge,
            Operand::durable(balance_handle(1), Pubkey::new_unique()),
            scalar_operand_u64(2),
            FheType::UINT64,
            Output::transient(),
        )
    }) {
        Ok(_) => panic!("invalid frame unexpectedly built"),
        Err(error) => error,
    };
    assert_eq!(error, EvalBuildError::UnsupportedBinaryOutputType);

    let error = match EvalPlan::build(app_authority(primary_authority), |_builder| Ok(())) {
        Ok(_) => panic!("empty frame unexpectedly built"),
        Err(error) => error,
    };
    assert_eq!(error, EvalBuildError::EmptyOps);
}

#[test]
fn finish_preflights_lowered_remaining_account_indices() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    builder.pool.push(balance_handle(1));
    builder.pool.push(Scalar::<Uint<64>>::u64(1).bytes());
    builder.steps.push(FheEvalStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: FheEvalOperand::AllowedDurable {
            handle_index: 0,
            encrypted_value_index: 0,
        },
        rhs: FheEvalOperand::Scalar { value_index: 1 },
        output_fhe_type: FheType::UINT64.byte(),
        output: FheEvalOutput::AllowedLocal,
    });
    builder.produced_types.push(FheType::UINT64.byte());

    assert_eq!(
        builder.finish().unwrap_err(),
        EvalBuildError::InvalidRemainingAccountReference
    );
}

#[test]
fn finish_preflights_lowered_transient_order_and_account_uniqueness() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    builder.steps.push(FheEvalStep::TrivialEncrypt {
        plaintext: Scalar::<Uint<64>>::u64(1).bytes(),
        fhe_type: FheType::UINT64.byte(),
        output: FheEvalOutput::AllowedLocal,
    });
    builder.pool.push(Scalar::<Uint<64>>::u64(1).bytes());
    builder.steps.push(FheEvalStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: FheEvalOperand::AllowedLocal { producer_index: 1 },
        rhs: FheEvalOperand::Scalar { value_index: 0 },
        output_fhe_type: FheType::UINT64.byte(),
        output: FheEvalOutput::AllowedLocal,
    });
    builder.produced_types = vec![FheType::UINT64.byte(), FheType::UINT64.byte()];

    assert_eq!(
        builder.finish().unwrap_err(),
        EvalBuildError::InvalidTransientReference
    );

    let input_slot = durable_slot(primary_authority, 1);
    let input_acl = input_slot.address();
    let balance = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();
    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    builder
        .add(balance, Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap();
    builder.remaining_accounts.push(EvalAccountMeta::readonly(
        input_acl,
        EvalAccountPurpose::DurableInputAcl,
    ));

    assert_eq!(
        builder.finish().unwrap_err(),
        EvalBuildError::InvalidRemainingAccountReference
    );
}

#[cfg(feature = "cpi")]
#[test]
fn invoke_eval_signed_with_builder_reports_build_errors_before_resolution() {
    let primary_authority = Pubkey::new_unique();
    let error = invoke_eval_signed_with_builder(
        app_authority(primary_authority),
        cpi_accounts(primary_authority),
        Vec::<AccountInfo<'static>>::new(),
        Vec::<AccountInfo<'static>>::new(),
        &[],
        |_builder| Ok(()),
    )
    .unwrap_err();

    assert!(matches!(
        error,
        EvalInvokeError::Build(EvalBuildError::EmptyOps)
    ));
}

#[cfg(feature = "cpi")]
#[test]
fn invoke_eval_signed_with_builder_adds_fixed_authority_before_resolution() {
    let primary_authority = Pubkey::new_unique();
    let input_slot = durable_slot(primary_authority, 1);
    let input_acl = input_slot.address();
    let output_slot = durable_slot(primary_authority, 7);
    let output_acl = output_slot.address();
    let balance = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();

    let error = invoke_eval_signed_with_builder(
        app_authority(primary_authority),
        cpi_accounts(primary_authority),
        vec![account_info(input_acl, false)],
        Vec::<AccountInfo<'static>>::new(),
        &[],
        |builder| {
            builder.add(
                balance,
                Scalar::<Uint<64>>::u64(1),
                Output::durable(output_slot, access_policy(primary_authority)),
            )
        },
    )
    .unwrap_err();

    assert!(matches!(
        error,
        EvalInvokeError::AccountResolution(
            EvalAccountResolutionError::MissingDynamicAccount { requirement }
        ) if requirement.pubkey() == output_acl
    ));
}

#[cfg(feature = "cpi")]
#[test]
fn invoke_eval_signed_with_builder_requires_additional_output_authorities() {
    let primary_authority = Pubkey::new_unique();
    let extra_authority = Pubkey::new_unique();
    let input_slot = durable_slot(primary_authority, 1);
    let input_acl = input_slot.address();
    let output_slot = durable_slot(extra_authority, 7);
    let output_acl = output_slot.address();
    let balance = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();

    let error = invoke_eval_signed_with_builder(
        app_authority(primary_authority),
        cpi_accounts(primary_authority),
        vec![
            account_info(input_acl, false),
            account_info(output_acl, true),
        ],
        Vec::<AccountInfo<'static>>::new(),
        &[],
        |builder| {
            builder.add(
                balance,
                Scalar::<Uint<64>>::u64(1),
                Output::durable(output_slot, access_policy(extra_authority)),
            )
        },
    )
    .unwrap_err();

    assert!(matches!(
        error,
        EvalInvokeError::AccountResolution(
            EvalAccountResolutionError::MissingOutputAuthority { authority }
        ) if authority.pubkey() == extra_authority
    ));
}

#[test]
fn lowers_mixed_eval_to_stable_remaining_account_indices() {
    let primary_authority = Pubkey::new_unique();
    let balance_slot = durable_slot(primary_authority, 1);
    let amount_slot = durable_slot(primary_authority, 2);
    let balance_acl = balance_slot.address();
    let amount_acl = amount_slot.address();
    let output_slot = durable_slot(primary_authority, 7);
    let output_acl = output_slot.address();
    let balance = Uint64Handle::durable(balance_handle(1), balance_slot).unwrap();
    let amount = Uint64Handle::durable(balance_handle(2), amount_slot).unwrap();
    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    let success = builder.ge(balance, amount, Output::transient()).unwrap();
    let debit_candidate = builder.sub(balance, amount, Output::transient()).unwrap();
    builder
        .if_then_else(
            success,
            debit_candidate,
            balance,
            Output::durable(output_slot, access_policy(primary_authority)),
        )
        .unwrap();

    let plan = builder.finish().unwrap();
    assert_eq!(plan.app_authority().pubkey(), primary_authority);

    assert_eq!(
        plan.remaining_accounts,
        vec![
            EvalAccountMeta::readonly(balance_acl, EvalAccountPurpose::DurableInputAcl),
            EvalAccountMeta::readonly(amount_acl, EvalAccountPurpose::DurableInputAcl),
            EvalAccountMeta::writable(output_acl, EvalAccountPurpose::DurableOutputAcl),
        ]
    );
    assert_eq!(plan.args.steps.len(), 3);
    match &plan.args.steps[0] {
        FheEvalStep::Binary { op, output, .. } => {
            assert_eq!(*op, FheBinaryOpCode::Ge);
            assert_eq!(*output, FheEvalOutput::AllowedLocal);
        }
        other => panic!("unexpected step: {other:?}"),
    }
    match &plan.args.steps[2] {
        FheEvalStep::Ternary {
            control,
            if_true,
            if_false,
            output,
            ..
        } => {
            assert_eq!(*control, FheEvalOperand::AllowedLocal { producer_index: 0 });
            assert_eq!(*if_true, FheEvalOperand::AllowedLocal { producer_index: 1 });
            match if_false {
                FheEvalOperand::AllowedDurable {
                    encrypted_value_index,
                    ..
                } => {
                    assert_eq!(*encrypted_value_index, 0)
                }
                other => panic!("unexpected if_false: {other:?}"),
            }
            match output {
                FheEvalOutput::AllowedDurable {
                    output_encrypted_value_index,
                    ..
                } => {
                    assert_eq!(*output_encrypted_value_index, 2)
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
    let input_slot = durable_slot(primary_authority, 1);
    let input_acl = input_slot.address();
    let extra_authority = Pubkey::new_unique();
    let output_slot = durable_slot(extra_authority, 7);
    let output_acl = output_slot.address();
    let input = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();

    let plan = EvalPlan::build(app_authority(primary_authority), |builder| {
        builder.add(
            input,
            Scalar::<Uint<64>>::u64(2),
            Output::durable(output_slot, access_policy(extra_authority)),
        )
    })
    .unwrap();

    let requirements = plan.dynamic_account_requirements().collect::<Vec<_>>();
    assert_eq!(
        requirements
            .iter()
            .map(EvalAccountRequirement::pubkey)
            .collect::<Vec<_>>(),
        vec![input_acl, output_acl, extra_authority]
    );
    assert_eq!(
        requirements[0].purposes(),
        &[EvalAccountPurpose::DurableInputAcl]
    );
    assert_eq!(
        requirements[1].purposes(),
        &[EvalAccountPurpose::DurableOutputAcl]
    );
    assert_eq!(
        requirements[2].purposes(),
        &[EvalAccountPurpose::DurableOutputAuthority]
    );
    assert!(requirements[1].is_writable());
    assert!(requirements[2].is_signer());
    assert!(!requirements[2].requires_dynamic_account());
    assert!(requirements[2].requires_output_authority());
}

#[test]
fn lowers_explicit_output_authority_witness() {
    let primary_authority = Pubkey::new_unique();
    let input_slot = durable_slot(primary_authority, 1);
    let acl_record = input_slot.address();
    let authority = Pubkey::new_unique();
    let output_slot = durable_slot(authority, 7);
    let output_acl = output_slot.address();
    let balance = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();
    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    builder
        .add(
            balance,
            Scalar::<Uint<64>>::u64(2),
            Output::durable(output_slot, access_policy(authority)),
        )
        .unwrap();

    let plan = builder.finish().unwrap();
    assert_eq!(plan.app_authority().pubkey(), primary_authority);
    assert_eq!(
        plan.remaining_accounts,
        vec![
            EvalAccountMeta::readonly(acl_record, EvalAccountPurpose::DurableInputAcl),
            EvalAccountMeta::writable(output_acl, EvalAccountPurpose::DurableOutputAcl),
            EvalAccountMeta::readonly_signer(authority, EvalAccountPurpose::DurableOutputAuthority,),
        ]
    );
    assert_eq!(
        plan.additional_output_authorities().collect::<Vec<_>>(),
        vec![authority]
    );
    let authority_requirements = plan.output_authority_requirements().collect::<Vec<_>>();
    assert_eq!(
        authority_requirements,
        vec![
            EvalOutputAuthorityRequirement {
                pubkey: primary_authority,
                cpi_account_authority: true,
            },
            EvalOutputAuthorityRequirement {
                pubkey: authority,
                cpi_account_authority: false,
            },
        ]
    );
    match &plan.args.steps[0] {
        FheEvalStep::Binary { output, .. } => match output {
            FheEvalOutput::AllowedDurable {
                output_encrypted_value_index,
                output_app_account_authority_index,
                ..
            } => {
                assert_eq!(*output_encrypted_value_index, 1);
                assert_eq!(*output_app_account_authority_index, Some(2));
            }
            other => panic!("unexpected output: {other:?}"),
        },
        other => panic!("unexpected step: {other:?}"),
    }
}

#[cfg(feature = "cpi")]
#[test]
fn resolve_accounts_orders_and_validates_plan_requirements() {
    let primary_authority = Pubkey::new_unique();
    let input_slot = durable_slot(primary_authority, 1);
    let input_acl = input_slot.address();
    let extra_authority = Pubkey::new_unique();
    let output_slot = durable_slot(extra_authority, 7);
    let output_acl = output_slot.address();
    let input = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();
    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    builder
        .add(
            input,
            Scalar::<Uint<64>>::u64(2),
            Output::durable(output_slot, access_policy(extra_authority)),
        )
        .unwrap();
    let plan = builder.finish().unwrap();

    let resolved = plan
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

    let duplicate = plan
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
        EvalAccountResolutionError::DuplicateDynamicAccount { pubkey: input_acl }
    );

    let unexpected = plan
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
        EvalAccountResolutionError::UnexpectedDynamicAccount { .. }
    ));

    let missing = plan
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
        EvalAccountResolutionError::MissingDynamicAccount { requirement }
            if requirement.pubkey() == input_acl
    ));

    let readonly = plan
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
        EvalAccountResolutionError::DynamicAccountNotWritable { requirement }
            if requirement.pubkey() == output_acl
    ));

    let duplicate_authority = plan
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
        EvalAccountResolutionError::DuplicateOutputAuthority {
            pubkey: extra_authority
        }
    );

    let unexpected_authority = plan
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
        EvalAccountResolutionError::UnexpectedOutputAuthority { .. }
    ));

    let missing_authority = plan
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
        EvalAccountResolutionError::MissingOutputAuthority {
            authority: EvalOutputAuthorityRequirement {
                pubkey: extra_authority,
                cpi_account_authority: false,
            }
        }
    );
}

#[cfg(feature = "cpi")]
#[test]
fn resolve_accounts_rejects_known_accounts_in_wrong_bucket() {
    let primary_authority = Pubkey::new_unique();
    let input_slot = durable_slot(primary_authority, 1);
    let input_acl = input_slot.address();
    let extra_authority = Pubkey::new_unique();
    let output_slot = durable_slot(extra_authority, 7);
    let output_acl = output_slot.address();
    let input = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();
    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    builder
        .add(
            input,
            Scalar::<Uint<64>>::u64(2),
            Output::durable(output_slot, access_policy(extra_authority)),
        )
        .unwrap();
    let plan = builder.finish().unwrap();

    let authority_in_dynamic_bucket = plan
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
        EvalAccountResolutionError::UnexpectedDynamicAccount {
            pubkey: extra_authority
        }
    );

    let input_acl_in_authority_bucket = plan
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
        EvalAccountResolutionError::UnexpectedOutputAuthority { pubkey: input_acl }
    );

    let output_acl_in_authority_bucket = plan
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
        EvalAccountResolutionError::UnexpectedOutputAuthority { pubkey: output_acl }
    );
}

#[test]
fn lowers_birth_steps() {
    let primary_authority = Pubkey::new_unique();
    let output_slot = durable_slot(primary_authority, 7);
    let output_acl = output_slot.address();
    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    let trivial = builder.trivial_encrypt_u64(1, Output::transient()).unwrap();
    builder
        .rand_u64(Output::durable(
            output_slot,
            access_policy(primary_authority),
        ))
        .unwrap();
    builder
        .add(trivial, Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap();

    let plan = builder.finish().unwrap();
    assert_eq!(
        plan.remaining_accounts,
        vec![EvalAccountMeta::writable(
            output_acl,
            EvalAccountPurpose::DurableOutputAcl
        )]
    );
    assert!(matches!(
        plan.args.steps[0],
        FheEvalStep::TrivialEncrypt { .. }
    ));
    assert!(matches!(plan.args.steps[1], FheEvalStep::Rand { .. }));
}

#[test]
fn rejects_invalid_references_and_types() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    let error = builder
        .binary_op(
            FheBinaryOpCode::Add,
            Operand::transient(0, builder.scope),
            scalar_operand_u64(1),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    assert_eq!(error, EvalBuildError::InvalidTransientReference);

    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    let error = builder
        .binary_op(
            FheBinaryOpCode::Ge,
            Operand::durable(balance_handle(1), Pubkey::new_unique()),
            scalar_operand_u64(2),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    assert_eq!(error, EvalBuildError::UnsupportedBinaryOutputType);

    let input_slot = durable_slot(primary_authority, 1);
    let input = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();
    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    builder.trivial_encrypt_u64(1, Output::transient()).unwrap();
    let current_index = builder
        .binary_op(
            FheBinaryOpCode::Add,
            Operand::transient(1, builder.scope),
            scalar_operand_u64(1),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    assert_eq!(current_index, EvalBuildError::InvalidTransientReference);

    let future_index = builder
        .binary_op(
            FheBinaryOpCode::Add,
            Operand::transient(9, builder.scope),
            scalar_operand_u64(1),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    assert_eq!(future_index, EvalBuildError::InvalidTransientReference);

    let invalid_rhs = builder
        .binary_op(
            FheBinaryOpCode::Add,
            input.operand(),
            Operand::transient(1, builder.scope),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    assert_eq!(invalid_rhs, EvalBuildError::InvalidTransientReference);
}

#[test]
fn rejects_transients_from_another_builder() {
    let primary_authority = Pubkey::new_unique();
    let input_slot = durable_slot(primary_authority, 1);
    let balance = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();

    let mut first = EvalBuilder::new(app_authority(primary_authority));
    let foreign = first
        .add(balance, Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap();

    let mut second = EvalBuilder::new(app_authority(primary_authority));
    second.trivial_encrypt_u64(1, Output::transient()).unwrap();
    let error = second
        .add(foreign, Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap_err();

    assert_eq!(error, EvalBuildError::InvalidTransientReference);
}

#[test]
fn validates_app_authority_and_durable_account_pubkeys() {
    let mut builder = EvalBuilder::new(app_authority(Pubkey::default()));
    builder.trivial_encrypt_u64(1, Output::transient()).unwrap();
    let error = match builder.finish() {
        Ok(_) => panic!("invalid app authority unexpectedly built"),
        Err(error) => error,
    };
    assert_eq!(error, EvalBuildError::InvalidAppAuthority);

    let invalid_namespace_slot = DurableSlot::new(
        Pubkey::default(),
        Pubkey::new_unique(),
        DurableLabel::new(handle(5)),
    );
    assert_eq!(
        Uint64Handle::durable(balance_handle(1), invalid_namespace_slot.clone()).unwrap_err(),
        EvalBuildError::InvalidDurableSlot
    );
    assert_eq!(
        DurableOutput::create(invalid_namespace_slot, access_policy(Pubkey::new_unique()))
            .birth()
            .unwrap_err(),
        EvalBuildError::InvalidDurableSlot
    );

    let invalid_account_slot = DurableSlot::new(
        Pubkey::new_unique(),
        Pubkey::default(),
        DurableLabel::new(handle(5)),
    );
    assert_eq!(
        Uint64Handle::durable(balance_handle(1), invalid_account_slot.clone()).unwrap_err(),
        EvalBuildError::InvalidDurableSlot
    );
    assert_eq!(
        DurableOutput::create(invalid_account_slot, access_policy(Pubkey::new_unique()))
            .birth()
            .unwrap_err(),
        EvalBuildError::InvalidDurableSlot
    );
}

#[test]
fn binary_validation_rejects_host_type_mismatches() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    let bool_lhs = Operand::durable(typed_handle(1, FheType::BOOL.byte()), Pubkey::new_unique());
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
    assert_eq!(error, EvalBuildError::BinaryOperandTypeMismatch);

    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    let error = builder
        .binary_op(
            FheBinaryOpCode::Add,
            Operand::durable(balance_handle(1), Pubkey::new_unique()),
            Operand::durable(
                typed_handle(2, FheType::UINT32.byte()),
                Pubkey::new_unique(),
            ),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    assert_eq!(error, EvalBuildError::BinaryOperandTypeMismatch);
}

#[test]
fn unary_validation_rejects_same_type_cast_and_bad_operand_types() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    // A cast to a different type is accepted.
    assert!(builder
        .unary_op(
            FheUnaryOpCode::Cast,
            Operand::durable(balance_handle(1), Pubkey::new_unique()),
            FheType::UINT32,
            Output::transient(),
        )
        .is_ok());
    // A same-type cast is rejected (EVM InvalidType parity).
    assert_eq!(
        builder
            .unary_op(
                FheUnaryOpCode::Cast,
                Operand::durable(balance_handle(1), Pubkey::new_unique()),
                FheType::UINT64,
                Output::transient(),
            )
            .unwrap_err(),
        EvalBuildError::UnsupportedFheType
    );
    // EVM cast type sets: a Bool input casts to a uint (Bool -> Uint32) is accepted...
    assert!(builder
        .unary_op(
            FheUnaryOpCode::Cast,
            Operand::durable(typed_handle(2, FheType::BOOL.byte()), Pubkey::new_unique()),
            FheType::UINT32,
            Output::transient(),
        )
        .is_ok());
    // ...but casting TO ebool, TO eaddress, or FROM eaddress is rejected.
    assert_eq!(
        builder
            .unary_op(
                FheUnaryOpCode::Cast,
                Operand::durable(balance_handle(1), Pubkey::new_unique()),
                FheType::BOOL,
                Output::transient(),
            )
            .unwrap_err(),
        EvalBuildError::UnsupportedFheType
    );
    assert_eq!(
        builder
            .unary_op(
                FheUnaryOpCode::Cast,
                Operand::durable(balance_handle(1), Pubkey::new_unique()),
                FheType::ADDRESS,
                Output::transient(),
            )
            .unwrap_err(),
        EvalBuildError::UnsupportedFheType
    );
    assert_eq!(
        builder
            .unary_op(
                FheUnaryOpCode::Cast,
                Operand::durable(
                    typed_handle(3, FheType::ADDRESS.byte()),
                    Pubkey::new_unique()
                ),
                FheType::UINT64,
                Output::transient(),
            )
            .unwrap_err(),
        EvalBuildError::UnsupportedFheType
    );
    // Neg rejects a Bool operand (EVM fheNeg supportedTypes = Uint8..Uint128 + Uint256).
    assert_eq!(
        builder
            .unary_op(
                FheUnaryOpCode::Neg,
                Operand::durable(typed_handle(1, FheType::BOOL.byte()), Pubkey::new_unique()),
                FheType::BOOL,
                Output::transient(),
            )
            .unwrap_err(),
        EvalBuildError::UnsupportedFheType
    );
}

#[test]
fn mul_div_rejects_zero_divisor() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    let balance =
        Uint64Handle::durable(balance_handle(1), durable_slot(primary_authority, 1)).unwrap();
    assert_eq!(
        builder
            .mul_div(
                balance,
                Scalar::<Uint<64>>::u64(3),
                Scalar::<Uint<64>>::u64(0),
                Output::transient(),
            )
            .unwrap_err(),
        EvalBuildError::MulDivDivisorZero
    );
}

#[test]
fn div_rem_require_nonzero_scalar_divisor() {
    let auth = Pubkey::new_unique();
    let mut builder = EvalBuilder::new(app_authority(auth));
    // Encrypted divisor is rejected — division is scalar-only (EVM `IsNotScalar`).
    let lhs = Uint64Handle::durable(balance_handle(1), durable_slot(auth, 1)).unwrap();
    let enc_divisor = Uint64Handle::durable(balance_handle(2), durable_slot(auth, 2)).unwrap();
    assert_eq!(
        builder
            .div(lhs, enc_divisor, Output::transient())
            .unwrap_err(),
        EvalBuildError::DivisorMustBeScalar
    );
    // A zero scalar divisor is rejected.
    let lhs2 = Uint64Handle::durable(balance_handle(1), durable_slot(auth, 1)).unwrap();
    assert_eq!(
        builder
            .rem(lhs2, Scalar::<Uint<64>>::u64(0), Output::transient())
            .unwrap_err(),
        EvalBuildError::DivisionByZero
    );
    // A non-zero scalar divisor is accepted.
    let lhs3 = Uint64Handle::durable(balance_handle(1), durable_slot(auth, 1)).unwrap();
    assert!(builder
        .div(lhs3, Scalar::<Uint<64>>::u64(3), Output::transient())
        .is_ok());
}

#[test]
fn builder_exposes_the_host_operator_type_surface() {
    // The typed builder must express the host's type matrix: bitwise on Bool/Uint256, neg on Uint256, eq on Bool, is_in on Uint160.
    let auth = Pubkey::new_unique();
    let mut builder = EvalBuilder::new(app_authority(auth));

    let bool_a =
        Encrypted::<Bool>::durable(typed_handle(1, FheType::BOOL.byte()), durable_slot(auth, 1))
            .unwrap();
    let bool_b =
        Encrypted::<Bool>::durable(typed_handle(2, FheType::BOOL.byte()), durable_slot(auth, 2))
            .unwrap();
    assert!(builder.and(bool_a, bool_b, Output::transient()).is_ok());

    let u256_a = Encrypted::<Bytes256>::durable(
        typed_handle(3, FheType::BYTES256.byte()),
        durable_slot(auth, 3),
    )
    .unwrap();
    let u256_b = Encrypted::<Bytes256>::durable(
        typed_handle(4, FheType::BYTES256.byte()),
        durable_slot(auth, 4),
    )
    .unwrap();
    assert!(builder.xor(u256_a, u256_b, Output::transient()).is_ok());

    let u256_c = Encrypted::<Bytes256>::durable(
        typed_handle(5, FheType::BYTES256.byte()),
        durable_slot(auth, 5),
    )
    .unwrap();
    assert!(builder.neg(u256_c, Output::transient()).is_ok());

    let bool_c =
        Encrypted::<Bool>::durable(typed_handle(6, FheType::BOOL.byte()), durable_slot(auth, 6))
            .unwrap();
    let bool_d =
        Encrypted::<Bool>::durable(typed_handle(7, FheType::BOOL.byte()), durable_slot(auth, 7))
            .unwrap();
    assert!(builder.eq(bool_c, bool_d, Output::transient()).is_ok());

    let addr_v = Encrypted::<Address>::durable(
        typed_handle(8, FheType::ADDRESS.byte()),
        durable_slot(auth, 8),
    )
    .unwrap();
    let addr_s = Encrypted::<Address>::durable(
        typed_handle(9, FheType::ADDRESS.byte()),
        durable_slot(auth, 9),
    )
    .unwrap();
    assert!(builder.is_in(addr_v, [addr_s], Output::transient()).is_ok());
}

#[test]
fn access_policy_constructors_validate_immediately() {
    assert_eq!(
        AccessPolicy::for_owner(Pubkey::default()).unwrap_err(),
        EvalBuildError::InvalidAccessPolicy
    );
    let subject = Pubkey::new_unique();
    assert_eq!(
        AccessPolicy::for_owner(subject)
            .unwrap()
            .with_compute(subject)
            .unwrap_err(),
        EvalBuildError::InvalidAccessPolicy
    );

    let mut policy = AccessPolicy::for_owner(Pubkey::new_unique()).unwrap();
    for _ in 1..MAX_ACL_SUBJECTS {
        policy = policy.with_use_only(Pubkey::new_unique()).unwrap();
    }
    assert_eq!(
        policy.with_use_only(Pubkey::new_unique()).unwrap_err(),
        EvalBuildError::InvalidAccessPolicy
    );

    assert_eq!(
        AccessPolicy::from_subjects(Vec::<AccessSubject>::new()).unwrap_err(),
        EvalBuildError::InvalidAccessPolicy
    );
}

#[test]
fn durable_output_birth_matches_eval_lowering() {
    let primary_authority = Pubkey::new_unique();
    let subject = Pubkey::new_unique();
    let output_slot = durable_slot(primary_authority, 42);
    let output = DurableOutput::create(output_slot.clone(), access_policy(subject));
    let birth = output.birth().unwrap();

    assert_eq!(birth.encrypted_value(), output_slot.address());
    assert_eq!(birth.acl_domain_key(), output_slot.namespace());
    assert_eq!(birth.app_account(), output_slot.account());
    assert_eq!(birth.encrypted_value_label(), output_slot.label().bytes());
    assert_eq!(birth.subjects(), access_policy(subject).subjects());
    assert_eq!(birth.previous_handle(), None);
    assert_eq!(birth.previous_subjects(), None);

    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    builder
        .trivial_encrypt_u64(7, Output::durable(output_slot, access_policy(subject)))
        .unwrap();
    let plan = builder.finish().unwrap();
    match &plan.args.steps[0] {
        FheEvalStep::TrivialEncrypt {
            output:
                FheEvalOutput::AllowedDurable {
                    output_encrypted_value_index,
                    output_acl_domain_key_index,
                    output_app_account_index,
                    output_encrypted_value_label_index,
                    output_subject_indexes,
                    previous_handle,
                    previous_subjects,
                    ..
                },
            ..
        } => {
            let output_encrypted_value =
                plan.remaining_accounts[*output_encrypted_value_index as usize].pubkey;
            assert_eq!(output_encrypted_value, birth.encrypted_value());
            assert_eq!(
                plan.args.pool_key(*output_acl_domain_key_index).unwrap(),
                birth.acl_domain_key()
            );
            assert_eq!(
                plan.args.pool_key(*output_app_account_index).unwrap(),
                birth.app_account()
            );
            assert_eq!(
                plan.args
                    .pool_bytes(*output_encrypted_value_label_index)
                    .unwrap(),
                birth.encrypted_value_label()
            );
            let output_subjects: Vec<Pubkey> = output_subject_indexes
                .iter()
                .map(|index| plan.args.pool_key(*index).unwrap())
                .collect();
            assert_eq!(output_subjects, birth.host_subjects());
            assert_eq!(*previous_handle, birth.previous_handle());
            assert_eq!(previous_subjects.as_deref(), birth.previous_subjects());
        }
        other => panic!("unexpected step: {other:?}"),
    }
}

#[test]
fn durable_output_supersede_carries_previous_state() {
    let primary_authority = Pubkey::new_unique();
    let subject = Pubkey::new_unique();
    let output_slot = durable_slot(primary_authority, 42);
    let previous_handle = balance_handle(1);
    let previous_subjects = vec![subject];
    let output = DurableOutput::supersede(
        output_slot,
        access_policy(subject),
        previous_handle,
        previous_subjects.clone(),
    );
    let birth = output.birth().unwrap();

    assert_eq!(birth.previous_handle(), Some(previous_handle));
    assert_eq!(
        birth.previous_subjects(),
        Some(previous_subjects.as_slice())
    );
}

#[test]
fn rejects_transients_from_another_frame() {
    let primary_authority = Pubkey::new_unique();
    let input_slot = durable_slot(primary_authority, 1);
    let balance = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();

    let mut first = EvalBuilder::new(app_authority(primary_authority));
    let foreign = first
        .add(balance, Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap();

    let mut second = EvalBuilder::new(app_authority(primary_authority));
    second.trivial_encrypt_u64(1, Output::transient()).unwrap();
    let error = second
        .add(foreign, Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap_err();

    assert_eq!(error, EvalBuildError::InvalidTransientReference);
}

#[test]
fn typed_handle_constructor_rejects_mismatched_handle_tag() {
    let error = Uint64Handle::durable(
        typed_handle(1, FheType::UINT32.byte()),
        durable_slot(Pubkey::new_unique(), 7),
    )
    .unwrap_err();
    assert_eq!(error, EvalBuildError::UnsupportedFheType);
}

#[test]
fn rand_rejects_address_type_like_host() {
    let mut builder = EvalBuilder::new(app_authority(Pubkey::new_unique()));
    let error = builder
        .rand_raw(FheType::ADDRESS, Output::transient())
        .unwrap_err();
    assert_eq!(error, EvalBuildError::UnsupportedFheType);
}

#[test]
fn finish_rejects_empty_steps() {
    let primary_authority = Pubkey::new_unique();
    assert!(matches!(
        EvalBuilder::new(app_authority(primary_authority)).finish(),
        Err(EvalBuildError::EmptyOps)
    ));
}

#[test]
fn rejects_more_than_max_ops() {
    let primary_authority = Pubkey::new_unique();
    let input_slot = durable_slot(primary_authority, 1);
    let balance = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();
    let mut builder = EvalBuilder::new(app_authority(primary_authority));
    for index in 0..MAX_FHE_EVAL_OPS {
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
    assert_eq!(error, EvalBuildError::TooManyOps);
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
