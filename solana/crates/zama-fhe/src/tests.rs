//! Unit tests for the builder, validators, lowering, and CPI plumbing.

use crate::accounts::*;
use crate::acl::*;
use crate::batch::*;
use crate::builder::*;
use crate::lower::StepTables;
use crate::operand::*;
use crate::types::*;
use crate::BatchBuildError;
use anchor_lang::prelude::Pubkey;
#[cfg(feature = "cpi")]
use anchor_lang::{prelude::AccountInfo, Key};
use zama_host::{
    CoprocessorInputAttestation, FheBinaryOpCode, FheExecuteOperand, FheExecuteOutput,
    FheExecuteStep, FheUnaryOpCode, MAX_FHE_BATCH_OPS,
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

fn app_authority(pubkey: Pubkey) -> BatchAppAuthority {
    BatchAppAuthority::new(pubkey)
}

#[cfg(feature = "cpi")]
fn account_info(pubkey: Pubkey, is_writable: bool) -> AccountInfo<'static> {
    let key = Box::leak(Box::new(pubkey));
    let owner = Box::leak(Box::new(Pubkey::new_unique()));
    let lamports = Box::leak(Box::new(0));
    let data = Box::leak(Vec::new().into_boxed_slice());
    AccountInfo::new(key, false, is_writable, lamports, data, owner, false)
}

fn encrypted_value_id(account: Pubkey, label_tag: u8) -> EncryptedValueId {
    EncryptedValueId::new(
        Pubkey::new_unique(),
        account,
        PersistentLabel::new(handle(label_tag)),
    )
}

fn subjects(subject: Pubkey) -> Vec<Pubkey> {
    vec![subject]
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
    let input_key = encrypted_value_id(primary_authority, 1);
    let input_acl = input_key.address();
    let output_key = encrypted_value_id(primary_authority, 7);
    let output_acl = output_key.address();
    let balance = Uint64Handle::persistent(balance_handle(1), input_key).unwrap();

    let batch = Batch::build(app_authority(primary_authority), |builder| {
        let incremented = builder.add(balance, Scalar::<Uint<64>>::u64(1), Output::transient())?;
        builder.add(
            incremented,
            Scalar::<Uint<64>>::u64(2),
            Output::persistent(PersistentOutput::create(
                output_key,
                subjects(primary_authority),
            )),
        )
    })
    .unwrap();

    assert_eq!(batch.app_authority().pubkey(), primary_authority);
    assert_eq!(
        batch.remaining_accounts,
        vec![
            BatchAccountMeta::readonly(input_acl, BatchAccountPurpose::PersistentInputAcl),
            BatchAccountMeta::writable(output_acl, BatchAccountPurpose::PersistentOutputAcl),
        ]
    );
    assert_eq!(batch.args.steps.len(), 2);
    match &batch.args.steps[1] {
        FheExecuteStep::Binary { lhs, output, .. } => {
            assert_eq!(*lhs, FheExecuteOperand::EarlierStep { producer_index: 0 });
            match output {
                FheExecuteOutput::StoredValue {
                    output_account_authority_index,
                    ..
                } => {
                    assert_eq!(*output_account_authority_index, None);
                }
                other => panic!("unexpected output: {other:?}"),
            }
        }
        other => panic!("unexpected step: {other:?}"),
    }
}

#[test]
fn builder_rejects_post_write_persistent_alias() {
    let authority = Pubkey::new_unique();
    let key = encrypted_value_id(authority, 7);
    let mut builder = BatchBuilder::new(app_authority(authority));
    builder
        .trivial_encrypt_u64(
            7,
            Output::persistent(PersistentOutput::create(key.clone(), subjects(authority))),
        )
        .unwrap();

    let reconstructed = Uint64Handle::persistent(balance_handle(99), key).unwrap();
    let error = builder
        .add(
            reconstructed,
            Scalar::<Uint<64>>::u64(1),
            Output::transient(),
        )
        .unwrap_err();

    assert_eq!(error, BatchBuildError::PersistentOperandWrittenEarlier);
}

#[test]
fn batch_build_lowers_verified_input_operand() {
    let primary_authority = Pubkey::new_unique();
    let output_key = encrypted_value_id(primary_authority, 7);
    let output_acl = output_key.address();
    let input_handle = balance_handle(2);
    let attestation = dummy_attestation(input_handle, primary_authority);

    let batch = Batch::build(app_authority(primary_authority), |builder| {
        let amount: Uint64Handle = builder.verified_input(attestation.clone())?;
        builder.add(
            amount,
            Scalar::<Uint<64>>::u64(1),
            Output::persistent(PersistentOutput::create(
                output_key,
                subjects(primary_authority),
            )),
        )
    })
    .unwrap();

    assert_eq!(batch.args.steps.len(), 1);
    match &batch.args.steps[0] {
        FheExecuteStep::Binary { lhs, rhs, .. } => {
            assert_eq!(
                *lhs,
                FheExecuteOperand::VerifiedInput {
                    attestation: Box::new(attestation.clone())
                }
            );
            assert_eq!(*rhs, FheExecuteOperand::Scalar { value_index: 0 });
            assert_eq!(
                batch.args.dictionary_bytes(0).unwrap(),
                Scalar::<Uint<64>>::u64(1).bytes()
            );
        }
        other => panic!("unexpected step: {other:?}"),
    }
    // A verified input carries no remaining account: the attestation is inline in the operand.
    assert_eq!(
        batch.remaining_accounts,
        vec![BatchAccountMeta::writable(
            output_acl,
            BatchAccountPurpose::PersistentOutputAcl
        )]
    );
}

#[test]
fn verified_input_rejects_type_mismatch() {
    let primary_authority = Pubkey::new_unique();
    // Input handle typed as BOOL (0) but requested as Uint64: caught at build time.
    let attestation = dummy_attestation(typed_handle(2, 0), primary_authority);
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    assert_eq!(
        builder.verified_input::<Uint<64>>(attestation).unwrap_err(),
        BatchBuildError::UnsupportedFheType
    );
}

#[test]
fn batch_build_propagates_closure_and_finish_errors() {
    let primary_authority = Pubkey::new_unique();
    let error = match Batch::build(app_authority(primary_authority), |builder| {
        builder.binary_op(
            FheBinaryOpCode::Ge,
            Operand::persistent(balance_handle(1), Pubkey::new_unique()),
            scalar_operand_u64(2),
            FheType::UINT64,
            Output::transient(),
        )
    }) {
        Ok(_) => panic!("invalid batch unexpectedly built"),
        Err(error) => error,
    };
    assert_eq!(error, BatchBuildError::UnsupportedBinaryOutputType);

    let error = match Batch::build(app_authority(primary_authority), |_builder| Ok(())) {
        Ok(_) => panic!("empty batch unexpectedly built"),
        Err(error) => error,
    };
    assert_eq!(error, BatchBuildError::EmptyOps);
}

#[test]
fn finish_preflights_lowered_remaining_account_indices() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    builder.dictionary.push(balance_handle(1));
    builder.dictionary.push(Scalar::<Uint<64>>::u64(1).bytes());
    builder.steps.push(FheExecuteStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: FheExecuteOperand::StoredValue {
            handle_index: 0,
            encrypted_value_index: 0,
        },
        rhs: FheExecuteOperand::Scalar { value_index: 1 },
        output_fhe_type: FheType::UINT64.byte(),
        output: FheExecuteOutput::Transient,
    });
    builder.produced_types.push(FheType::UINT64.byte());

    assert_eq!(
        builder.finish().unwrap_err(),
        BatchBuildError::InvalidRemainingAccountReference
    );
}

#[test]
fn finish_preflights_lowered_transient_order_and_account_uniqueness() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
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
    builder.produced_types = vec![FheType::UINT64.byte(), FheType::UINT64.byte()];

    assert_eq!(
        builder.finish().unwrap_err(),
        BatchBuildError::InvalidTransientReference
    );

    let input_key = encrypted_value_id(primary_authority, 1);
    let input_acl = input_key.address();
    let balance = Uint64Handle::persistent(balance_handle(1), input_key).unwrap();
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    builder
        .add(balance, Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap();
    builder.remaining_accounts.push(BatchAccountMeta::readonly(
        input_acl,
        BatchAccountPurpose::PersistentInputAcl,
    ));

    assert_eq!(
        builder.finish().unwrap_err(),
        BatchBuildError::InvalidRemainingAccountReference
    );
}

#[test]
fn finish_rejects_dictionary_entry_no_step_references() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    builder
        .trivial_encrypt(Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap();
    builder.dictionary.push([0xAA; 32]);

    assert_eq!(
        builder.finish().unwrap_err(),
        BatchBuildError::UnreferencedDictionaryEntry
    );
}

#[test]
fn finish_rejects_dictionary_index_past_dictionary_end() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
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
        BatchBuildError::DictionaryIndexOutOfBounds
    );
}

#[cfg(feature = "cpi")]
#[test]
fn resolve_accounts_requires_the_cpi_authority_witness() {
    let primary_authority = Pubkey::new_unique();
    let input_key = encrypted_value_id(primary_authority, 1);
    let input_acl = input_key.address();
    let output_key = encrypted_value_id(primary_authority, 7);
    let output_acl = output_key.address();
    let balance = Uint64Handle::persistent(balance_handle(1), input_key).unwrap();
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    builder
        .add(
            balance,
            Scalar::<Uint<64>>::u64(1),
            Output::persistent(PersistentOutput::create(
                output_key,
                subjects(primary_authority),
            )),
        )
        .unwrap();
    let batch = builder.finish().unwrap();

    // The batch's own app authority is an output authority like any other: the caller passes its
    // account info, and leaving it out is an error rather than something the SDK fills in.
    let missing = batch
        .resolve_accounts(
            vec![
                account_info(input_acl, false),
                account_info(output_acl, true),
            ],
            Vec::<AccountInfo<'static>>::new(),
        )
        .unwrap_err();
    assert_eq!(
        missing,
        BatchAccountResolutionError::MissingOutputAuthority {
            authority: BatchOutputAuthorityRequirement {
                pubkey: primary_authority,
                cpi_account_authority: true,
            }
        }
    );

    batch
        .resolve_accounts(
            vec![
                account_info(input_acl, false),
                account_info(output_acl, true),
            ],
            vec![account_info(primary_authority, false)],
        )
        .expect("resolves once the authority witness is supplied");
}

#[test]
fn lowers_mixed_batch_to_stable_remaining_account_indices() {
    let primary_authority = Pubkey::new_unique();
    let balance_key = encrypted_value_id(primary_authority, 1);
    let amount_key = encrypted_value_id(primary_authority, 2);
    let balance_acl = balance_key.address();
    let amount_acl = amount_key.address();
    let output_key = encrypted_value_id(primary_authority, 7);
    let output_acl = output_key.address();
    let balance = Uint64Handle::persistent(balance_handle(1), balance_key).unwrap();
    let amount = Uint64Handle::persistent(balance_handle(2), amount_key).unwrap();
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    let success = builder.ge(balance, amount, Output::transient()).unwrap();
    let debit_candidate = builder.sub(balance, amount, Output::transient()).unwrap();
    builder
        .if_then_else(
            success,
            debit_candidate,
            balance,
            Output::persistent(PersistentOutput::create(
                output_key,
                subjects(primary_authority),
            )),
        )
        .unwrap();

    let batch = builder.finish().unwrap();
    assert_eq!(batch.app_authority().pubkey(), primary_authority);

    assert_eq!(
        batch.remaining_accounts,
        vec![
            BatchAccountMeta::readonly(balance_acl, BatchAccountPurpose::PersistentInputAcl),
            BatchAccountMeta::readonly(amount_acl, BatchAccountPurpose::PersistentInputAcl),
            BatchAccountMeta::writable(output_acl, BatchAccountPurpose::PersistentOutputAcl),
        ]
    );
    assert_eq!(batch.args.steps.len(), 3);
    match &batch.args.steps[0] {
        FheExecuteStep::Binary { op, output, .. } => {
            assert_eq!(*op, FheBinaryOpCode::Ge);
            assert_eq!(*output, FheExecuteOutput::Transient);
        }
        other => panic!("unexpected step: {other:?}"),
    }
    match &batch.args.steps[2] {
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
                FheExecuteOperand::StoredValue {
                    encrypted_value_index,
                    ..
                } => {
                    assert_eq!(*encrypted_value_index, 0)
                }
                other => panic!("unexpected if_false: {other:?}"),
            }
            match output {
                FheExecuteOutput::StoredValue {
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
    let input_key = encrypted_value_id(primary_authority, 1);
    let input_acl = input_key.address();
    let extra_authority = Pubkey::new_unique();
    let output_key = encrypted_value_id(extra_authority, 7);
    let output_acl = output_key.address();
    let input = Uint64Handle::persistent(balance_handle(1), input_key).unwrap();

    let batch = Batch::build(app_authority(primary_authority), |builder| {
        builder.add(
            input,
            Scalar::<Uint<64>>::u64(2),
            Output::persistent(PersistentOutput::create(
                output_key,
                subjects(extra_authority),
            )),
        )
    })
    .unwrap();

    let requirements = batch.dynamic_account_requirements().collect::<Vec<_>>();
    assert_eq!(
        requirements
            .iter()
            .map(BatchAccountRequirement::pubkey)
            .collect::<Vec<_>>(),
        vec![input_acl, output_acl, extra_authority]
    );
    assert_eq!(
        requirements[0].purposes(),
        &[BatchAccountPurpose::PersistentInputAcl]
    );
    assert_eq!(
        requirements[1].purposes(),
        &[BatchAccountPurpose::PersistentOutputAcl]
    );
    assert_eq!(
        requirements[2].purposes(),
        &[BatchAccountPurpose::PersistentOutputAuthority]
    );
    assert!(requirements[1].is_writable());
    assert!(requirements[2].is_signer());
    assert!(!requirements[2].requires_dynamic_account());
    assert!(requirements[2].requires_output_authority());
}

#[test]
fn lowers_explicit_output_authority_witness() {
    let primary_authority = Pubkey::new_unique();
    let input_key = encrypted_value_id(primary_authority, 1);
    let acl_record = input_key.address();
    let authority = Pubkey::new_unique();
    let output_key = encrypted_value_id(authority, 7);
    let output_acl = output_key.address();
    let balance = Uint64Handle::persistent(balance_handle(1), input_key).unwrap();
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    builder
        .add(
            balance,
            Scalar::<Uint<64>>::u64(2),
            Output::persistent(PersistentOutput::create(output_key, subjects(authority))),
        )
        .unwrap();

    let batch = builder.finish().unwrap();
    assert_eq!(batch.app_authority().pubkey(), primary_authority);
    assert_eq!(
        batch.remaining_accounts,
        vec![
            BatchAccountMeta::readonly(acl_record, BatchAccountPurpose::PersistentInputAcl),
            BatchAccountMeta::writable(output_acl, BatchAccountPurpose::PersistentOutputAcl),
            BatchAccountMeta::readonly_signer(
                authority,
                BatchAccountPurpose::PersistentOutputAuthority,
            ),
        ]
    );
    assert_eq!(
        batch.additional_output_authorities().collect::<Vec<_>>(),
        vec![authority]
    );
    let authority_requirements = batch.output_authority_requirements().collect::<Vec<_>>();
    assert_eq!(
        authority_requirements,
        vec![
            BatchOutputAuthorityRequirement {
                pubkey: primary_authority,
                cpi_account_authority: true,
            },
            BatchOutputAuthorityRequirement {
                pubkey: authority,
                cpi_account_authority: false,
            },
        ]
    );
    match &batch.args.steps[0] {
        FheExecuteStep::Binary { output, .. } => match output {
            FheExecuteOutput::StoredValue {
                output_encrypted_value_index,
                output_account_authority_index,
                ..
            } => {
                assert_eq!(*output_encrypted_value_index, 1);
                assert_eq!(*output_account_authority_index, Some(2));
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
    let input_key = encrypted_value_id(primary_authority, 1);
    let input_acl = input_key.address();
    let extra_authority = Pubkey::new_unique();
    let output_key = encrypted_value_id(extra_authority, 7);
    let output_acl = output_key.address();
    let input = Uint64Handle::persistent(balance_handle(1), input_key).unwrap();
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    builder
        .add(
            input,
            Scalar::<Uint<64>>::u64(2),
            Output::persistent(PersistentOutput::create(
                output_key,
                subjects(extra_authority),
            )),
        )
        .unwrap();
    let batch = builder.finish().unwrap();

    let resolved = batch
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

    let duplicate = batch
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
        BatchAccountResolutionError::DuplicateDynamicAccount { pubkey: input_acl }
    );

    let unexpected = batch
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
        BatchAccountResolutionError::UnexpectedDynamicAccount { .. }
    ));

    let missing = batch
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
        BatchAccountResolutionError::MissingDynamicAccount { requirement }
            if requirement.pubkey() == input_acl
    ));

    let readonly = batch
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
        BatchAccountResolutionError::DynamicAccountNotWritable { requirement }
            if requirement.pubkey() == output_acl
    ));

    let duplicate_authority = batch
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
        BatchAccountResolutionError::DuplicateOutputAuthority {
            pubkey: extra_authority
        }
    );

    let unexpected_authority = batch
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
        BatchAccountResolutionError::UnexpectedOutputAuthority { .. }
    ));

    let missing_authority = batch
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
        BatchAccountResolutionError::MissingOutputAuthority {
            authority: BatchOutputAuthorityRequirement {
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
    let input_key = encrypted_value_id(primary_authority, 1);
    let input_acl = input_key.address();
    let extra_authority = Pubkey::new_unique();
    let output_key = encrypted_value_id(extra_authority, 7);
    let output_acl = output_key.address();
    let input = Uint64Handle::persistent(balance_handle(1), input_key).unwrap();
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    builder
        .add(
            input,
            Scalar::<Uint<64>>::u64(2),
            Output::persistent(PersistentOutput::create(
                output_key,
                subjects(extra_authority),
            )),
        )
        .unwrap();
    let batch = builder.finish().unwrap();

    let authority_in_dynamic_bucket = batch
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
        BatchAccountResolutionError::UnexpectedDynamicAccount {
            pubkey: extra_authority
        }
    );

    let input_acl_in_authority_bucket = batch
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
        BatchAccountResolutionError::UnexpectedOutputAuthority { pubkey: input_acl }
    );

    let output_acl_in_authority_bucket = batch
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
        BatchAccountResolutionError::UnexpectedOutputAuthority { pubkey: output_acl }
    );
}

#[test]
fn lowers_create_steps() {
    let primary_authority = Pubkey::new_unique();
    let output_key = encrypted_value_id(primary_authority, 7);
    let output_acl = output_key.address();
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    let trivial = builder.trivial_encrypt_u64(1, Output::transient()).unwrap();
    builder
        .rand_u64(Output::persistent(PersistentOutput::create(
            output_key,
            subjects(primary_authority),
        )))
        .unwrap();
    builder
        .add(trivial, Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap();

    let batch = builder.finish().unwrap();
    assert_eq!(
        batch.remaining_accounts,
        vec![BatchAccountMeta::writable(
            output_acl,
            BatchAccountPurpose::PersistentOutputAcl
        )]
    );
    assert!(matches!(
        batch.args.steps[0],
        FheExecuteStep::TrivialEncrypt { .. }
    ));
    assert!(matches!(batch.args.steps[1], FheExecuteStep::Rand { .. }));
}

#[test]
fn rejects_invalid_references_and_types() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    let error = builder
        .binary_op(
            FheBinaryOpCode::Add,
            Operand::transient(0, builder.scope),
            scalar_operand_u64(1),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    assert_eq!(error, BatchBuildError::InvalidTransientReference);

    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    let error = builder
        .binary_op(
            FheBinaryOpCode::Ge,
            Operand::persistent(balance_handle(1), Pubkey::new_unique()),
            scalar_operand_u64(2),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    assert_eq!(error, BatchBuildError::UnsupportedBinaryOutputType);

    let input_key = encrypted_value_id(primary_authority, 1);
    let input = Uint64Handle::persistent(balance_handle(1), input_key).unwrap();
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
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
    assert_eq!(current_index, BatchBuildError::InvalidTransientReference);

    let future_index = builder
        .binary_op(
            FheBinaryOpCode::Add,
            Operand::transient(9, builder.scope),
            scalar_operand_u64(1),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    assert_eq!(future_index, BatchBuildError::InvalidTransientReference);

    let invalid_rhs = builder
        .binary_op(
            FheBinaryOpCode::Add,
            input.operand(),
            Operand::transient(1, builder.scope),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    assert_eq!(invalid_rhs, BatchBuildError::InvalidTransientReference);
}

#[test]
fn rejects_transients_from_another_builder() {
    let primary_authority = Pubkey::new_unique();
    let input_key = encrypted_value_id(primary_authority, 1);
    let balance = Uint64Handle::persistent(balance_handle(1), input_key).unwrap();

    let mut first = BatchBuilder::new(app_authority(primary_authority));
    let foreign = first
        .add(balance, Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap();

    let mut second = BatchBuilder::new(app_authority(primary_authority));
    second.trivial_encrypt_u64(1, Output::transient()).unwrap();
    let error = second
        .add(foreign, Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap_err();

    assert_eq!(error, BatchBuildError::InvalidTransientReference);
}

#[test]
fn validates_app_authority_and_persistent_account_pubkeys() {
    let mut builder = BatchBuilder::new(app_authority(Pubkey::default()));
    builder.trivial_encrypt_u64(1, Output::transient()).unwrap();
    let error = match builder.finish() {
        Ok(_) => panic!("invalid app authority unexpectedly built"),
        Err(error) => error,
    };
    assert_eq!(error, BatchBuildError::InvalidAppAuthority);

    let invalid_encrypted_value_id = EncryptedValueId::new(
        Pubkey::default(),
        Pubkey::new_unique(),
        PersistentLabel::new(handle(5)),
    );
    assert_eq!(
        Uint64Handle::persistent(balance_handle(1), invalid_encrypted_value_id.clone())
            .unwrap_err(),
        BatchBuildError::InvalidEncryptedValueId
    );
    assert_eq!(
        PersistentOutput::create(invalid_encrypted_value_id, subjects(Pubkey::new_unique()))
            .binding()
            .unwrap_err(),
        BatchBuildError::InvalidEncryptedValueId
    );

    let invalid_account_key = EncryptedValueId::new(
        Pubkey::new_unique(),
        Pubkey::default(),
        PersistentLabel::new(handle(5)),
    );
    assert_eq!(
        Uint64Handle::persistent(balance_handle(1), invalid_account_key.clone()).unwrap_err(),
        BatchBuildError::InvalidEncryptedValueId
    );
    assert_eq!(
        PersistentOutput::create(invalid_account_key, subjects(Pubkey::new_unique()))
            .binding()
            .unwrap_err(),
        BatchBuildError::InvalidEncryptedValueId
    );
}

#[test]
fn binary_validation_rejects_host_type_mismatches() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    let bool_lhs = Operand::persistent(typed_handle(1, FheType::BOOL.byte()), Pubkey::new_unique());
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
    assert_eq!(error, BatchBuildError::BinaryOperandTypeMismatch);

    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    let error = builder
        .binary_op(
            FheBinaryOpCode::Add,
            Operand::persistent(balance_handle(1), Pubkey::new_unique()),
            Operand::persistent(
                typed_handle(2, FheType::UINT32.byte()),
                Pubkey::new_unique(),
            ),
            FheType::UINT64,
            Output::transient(),
        )
        .unwrap_err();
    assert_eq!(error, BatchBuildError::BinaryOperandTypeMismatch);
}

#[test]
fn unary_validation_rejects_same_type_cast_and_bad_operand_types() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    // A cast to a different type is accepted.
    assert!(builder
        .unary_op(
            FheUnaryOpCode::Cast,
            Operand::persistent(balance_handle(1), Pubkey::new_unique()),
            FheType::UINT32,
            Output::transient(),
        )
        .is_ok());
    // A same-type cast is rejected (EVM InvalidType parity).
    assert_eq!(
        builder
            .unary_op(
                FheUnaryOpCode::Cast,
                Operand::persistent(balance_handle(1), Pubkey::new_unique()),
                FheType::UINT64,
                Output::transient(),
            )
            .unwrap_err(),
        BatchBuildError::UnsupportedFheType
    );
    // EVM cast type sets: a Bool input casts to a uint (Bool -> Uint32) is accepted...
    assert!(builder
        .unary_op(
            FheUnaryOpCode::Cast,
            Operand::persistent(typed_handle(2, FheType::BOOL.byte()), Pubkey::new_unique()),
            FheType::UINT32,
            Output::transient(),
        )
        .is_ok());
    // ...but casting TO ebool, TO eaddress, or FROM eaddress is rejected.
    assert_eq!(
        builder
            .unary_op(
                FheUnaryOpCode::Cast,
                Operand::persistent(balance_handle(1), Pubkey::new_unique()),
                FheType::BOOL,
                Output::transient(),
            )
            .unwrap_err(),
        BatchBuildError::UnsupportedFheType
    );
    assert_eq!(
        builder
            .unary_op(
                FheUnaryOpCode::Cast,
                Operand::persistent(balance_handle(1), Pubkey::new_unique()),
                FheType::ADDRESS,
                Output::transient(),
            )
            .unwrap_err(),
        BatchBuildError::UnsupportedFheType
    );
    assert_eq!(
        builder
            .unary_op(
                FheUnaryOpCode::Cast,
                Operand::persistent(
                    typed_handle(3, FheType::ADDRESS.byte()),
                    Pubkey::new_unique()
                ),
                FheType::UINT64,
                Output::transient(),
            )
            .unwrap_err(),
        BatchBuildError::UnsupportedFheType
    );
    // Neg rejects a Bool operand (EVM fheNeg supportedTypes = Uint8..Uint128 + Uint256).
    assert_eq!(
        builder
            .unary_op(
                FheUnaryOpCode::Neg,
                Operand::persistent(typed_handle(1, FheType::BOOL.byte()), Pubkey::new_unique()),
                FheType::BOOL,
                Output::transient(),
            )
            .unwrap_err(),
        BatchBuildError::UnsupportedFheType
    );
}

#[test]
fn mul_div_rejects_zero_divisor() {
    let primary_authority = Pubkey::new_unique();
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    let balance =
        Uint64Handle::persistent(balance_handle(1), encrypted_value_id(primary_authority, 1))
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
        BatchBuildError::MulDivDivisorZero
    );
}

#[test]
fn div_rem_require_nonzero_scalar_divisor() {
    let auth = Pubkey::new_unique();
    let mut builder = BatchBuilder::new(app_authority(auth));
    // Encrypted divisor is rejected — division is scalar-only (EVM `IsNotScalar`).
    let lhs = Uint64Handle::persistent(balance_handle(1), encrypted_value_id(auth, 1)).unwrap();
    let enc_divisor =
        Uint64Handle::persistent(balance_handle(2), encrypted_value_id(auth, 2)).unwrap();
    assert_eq!(
        builder
            .div(lhs, enc_divisor, Output::transient())
            .unwrap_err(),
        BatchBuildError::DivisorMustBeScalar
    );
    // A zero scalar divisor is rejected.
    let lhs2 = Uint64Handle::persistent(balance_handle(1), encrypted_value_id(auth, 1)).unwrap();
    assert_eq!(
        builder
            .rem(lhs2, Scalar::<Uint<64>>::u64(0), Output::transient())
            .unwrap_err(),
        BatchBuildError::DivisionByZero
    );
    // A non-zero scalar divisor is accepted.
    let lhs3 = Uint64Handle::persistent(balance_handle(1), encrypted_value_id(auth, 1)).unwrap();
    assert!(builder
        .div(lhs3, Scalar::<Uint<64>>::u64(3), Output::transient())
        .is_ok());
}

#[test]
fn builder_exposes_the_host_operator_type_surface() {
    // The typed builder must express the host's type matrix: bitwise on Bool/Uint256, neg on Uint256, eq on Bool, is_in on Uint160.
    let auth = Pubkey::new_unique();
    let mut builder = BatchBuilder::new(app_authority(auth));

    let bool_a = Encrypted::<Bool>::persistent(
        typed_handle(1, FheType::BOOL.byte()),
        encrypted_value_id(auth, 1),
    )
    .unwrap();
    let bool_b = Encrypted::<Bool>::persistent(
        typed_handle(2, FheType::BOOL.byte()),
        encrypted_value_id(auth, 2),
    )
    .unwrap();
    assert!(builder.and(bool_a, bool_b, Output::transient()).is_ok());

    let u256_a = Encrypted::<Bytes256>::persistent(
        typed_handle(3, FheType::BYTES256.byte()),
        encrypted_value_id(auth, 3),
    )
    .unwrap();
    let u256_b = Encrypted::<Bytes256>::persistent(
        typed_handle(4, FheType::BYTES256.byte()),
        encrypted_value_id(auth, 4),
    )
    .unwrap();
    assert!(builder.xor(u256_a, u256_b, Output::transient()).is_ok());

    let u256_c = Encrypted::<Bytes256>::persistent(
        typed_handle(5, FheType::BYTES256.byte()),
        encrypted_value_id(auth, 5),
    )
    .unwrap();
    assert!(builder.neg(u256_c, Output::transient()).is_ok());

    let bool_c = Encrypted::<Bool>::persistent(
        typed_handle(6, FheType::BOOL.byte()),
        encrypted_value_id(auth, 6),
    )
    .unwrap();
    let bool_d = Encrypted::<Bool>::persistent(
        typed_handle(7, FheType::BOOL.byte()),
        encrypted_value_id(auth, 7),
    )
    .unwrap();
    assert!(builder.eq(bool_c, bool_d, Output::transient()).is_ok());

    let addr_v = Encrypted::<Address>::persistent(
        typed_handle(8, FheType::ADDRESS.byte()),
        encrypted_value_id(auth, 8),
    )
    .unwrap();
    let addr_s = Encrypted::<Address>::persistent(
        typed_handle(9, FheType::ADDRESS.byte()),
        encrypted_value_id(auth, 9),
    )
    .unwrap();
    assert!(builder.is_in(addr_v, [addr_s], Output::transient()).is_ok());
}

#[test]
fn persistent_output_validates_raw_subjects() {
    let key = encrypted_value_id(Pubkey::new_unique(), 1);
    assert_eq!(
        PersistentOutput::create(key.clone(), vec![])
            .binding()
            .unwrap_err(),
        BatchBuildError::InvalidSubjects
    );
    assert_eq!(
        PersistentOutput::create(key.clone(), vec![Pubkey::default()])
            .binding()
            .unwrap_err(),
        BatchBuildError::InvalidSubjects
    );
    let duplicate = Pubkey::new_unique();
    assert_eq!(
        PersistentOutput::create(key.clone(), vec![duplicate, duplicate])
            .binding()
            .unwrap_err(),
        BatchBuildError::InvalidSubjects
    );
    assert_eq!(
        PersistentOutput::create(
            key,
            (0..=zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS)
                .map(|_| Pubkey::new_unique())
                .collect(),
        )
        .binding()
        .unwrap_err(),
        BatchBuildError::InvalidSubjects
    );
}

#[test]
fn persistent_output_create_matches_batch_lowering() {
    let primary_authority = Pubkey::new_unique();
    let subject = Pubkey::new_unique();
    let output_key = encrypted_value_id(primary_authority, 42);
    let output = PersistentOutput::create(output_key.clone(), subjects(subject));
    let binding = output.binding().unwrap();

    assert_eq!(binding.encrypted_value(), output_key.address());
    assert_eq!(binding.domain(), output_key.domain());
    assert_eq!(binding.account(), output_key.account());
    assert_eq!(binding.label(), output_key.label().bytes());
    assert_eq!(binding.subjects(), subjects(subject));
    assert_eq!(binding.previous_handle(), None);
    assert_eq!(binding.previous_subjects(), None);

    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    builder
        .trivial_encrypt_u64(
            7,
            Output::persistent(PersistentOutput::create(output_key, subjects(subject))),
        )
        .unwrap();
    let batch = builder.finish().unwrap();
    match &batch.args.steps[0] {
        FheExecuteStep::TrivialEncrypt {
            output:
                FheExecuteOutput::StoredValue {
                    output_encrypted_value_index,
                    output_domain_index,
                    output_account_index,
                    output_label_index,
                    output_subject_indexes,
                    previous_state,
                    ..
                },
            ..
        } => {
            let output_encrypted_value =
                batch.remaining_accounts[*output_encrypted_value_index as usize].pubkey;
            assert_eq!(output_encrypted_value, binding.encrypted_value());
            assert_eq!(
                batch.args.dictionary_key(*output_domain_index).unwrap(),
                binding.domain()
            );
            assert_eq!(
                batch.args.dictionary_key(*output_account_index).unwrap(),
                binding.account()
            );
            assert_eq!(
                batch.args.dictionary_bytes(*output_label_index).unwrap(),
                binding.label()
            );
            let output_subjects: Vec<Pubkey> = output_subject_indexes
                .iter()
                .map(|index| batch.args.dictionary_key(*index).unwrap())
                .collect();
            assert_eq!(output_subjects, binding.host_subjects());
            assert_eq!(*previous_state, binding.previous_state());
        }
        other => panic!("unexpected step: {other:?}"),
    }
}

#[test]
fn persistent_output_update_carries_current_state() {
    let primary_authority = Pubkey::new_unique();
    let subject = Pubkey::new_unique();
    let output_key = encrypted_value_id(primary_authority, 42);
    let previous_handle = balance_handle(1);
    let previous_subjects = vec![subject];
    let current = zama_host::EncryptedValue {
        domain: primary_authority,
        account: Pubkey::new_unique(),
        label: [42; 32],
        current_handle: previous_handle,
        subjects: previous_subjects.clone(),
        leaf_count: 7,
        peaks: vec![],
        bump: 1,
    };
    let output = PersistentOutput::update(output_key, subjects(subject), &current);
    let binding = output.binding().unwrap();

    assert_eq!(binding.previous_handle(), Some(previous_handle));
    assert_eq!(
        binding.previous_subjects(),
        Some(previous_subjects.as_slice())
    );
}

#[test]
fn rejects_transients_from_another_batch() {
    let primary_authority = Pubkey::new_unique();
    let input_key = encrypted_value_id(primary_authority, 1);
    let balance = Uint64Handle::persistent(balance_handle(1), input_key).unwrap();

    let mut first = BatchBuilder::new(app_authority(primary_authority));
    let foreign = first
        .add(balance, Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap();

    let mut second = BatchBuilder::new(app_authority(primary_authority));
    second.trivial_encrypt_u64(1, Output::transient()).unwrap();
    let error = second
        .add(foreign, Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap_err();

    assert_eq!(error, BatchBuildError::InvalidTransientReference);
}

#[test]
fn typed_handle_constructor_rejects_mismatched_handle_tag() {
    let error = Uint64Handle::persistent(
        typed_handle(1, FheType::UINT32.byte()),
        encrypted_value_id(Pubkey::new_unique(), 7),
    )
    .unwrap_err();
    assert_eq!(error, BatchBuildError::UnsupportedFheType);
}

#[test]
fn rand_rejects_address_type_like_host() {
    let mut builder = BatchBuilder::new(app_authority(Pubkey::new_unique()));
    let error = builder
        .rand_raw(FheType::ADDRESS, Output::transient())
        .unwrap_err();
    assert_eq!(error, BatchBuildError::UnsupportedFheType);
}

#[test]
fn finish_rejects_empty_steps() {
    let primary_authority = Pubkey::new_unique();
    assert!(matches!(
        BatchBuilder::new(app_authority(primary_authority)).finish(),
        Err(BatchBuildError::EmptyOps)
    ));
}

#[test]
fn rejects_more_than_max_ops() {
    let primary_authority = Pubkey::new_unique();
    let input_key = encrypted_value_id(primary_authority, 1);
    let balance = Uint64Handle::persistent(balance_handle(1), input_key).unwrap();
    let mut builder = BatchBuilder::new(app_authority(primary_authority));
    for index in 0..MAX_FHE_BATCH_OPS {
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
    assert_eq!(error, BatchBuildError::TooManyOps);
}

#[test]
fn step_tables_rollback_undoes_promotions_and_appends() {
    let shared = Pubkey::new_unique();
    let mut remaining_accounts = vec![BatchAccountMeta::readonly(
        shared,
        BatchAccountPurpose::PersistentInputAcl,
    )];
    let mut dictionary = vec![handle(1)];
    let mut persistent_producers = vec![(Pubkey::new_unique(), 0u8)];
    let accounts_before = remaining_accounts.clone();
    let dictionary_before = dictionary.clone();
    let producers_before = persistent_producers.clone();

    let mut tables = StepTables::open(
        &mut remaining_accounts,
        &mut dictionary,
        &mut persistent_producers,
    );
    // Promote the same entry twice — first writable, then signer — so undoing in the wrong order
    // would leave the entry with the flags the first promotion set.
    assert_eq!(
        tables
            .account_index(BatchAccountMeta::writable(
                shared,
                BatchAccountPurpose::PersistentOutputAcl,
            ))
            .unwrap(),
        0
    );
    assert_eq!(
        tables
            .account_index(BatchAccountMeta::readonly_signer(
                shared,
                BatchAccountPurpose::PersistentOutputAuthority,
            ))
            .unwrap(),
        0
    );
    assert_eq!(
        tables
            .account_index(BatchAccountMeta::readonly(
                Pubkey::new_unique(),
                BatchAccountPurpose::PersistentInputAcl,
            ))
            .unwrap(),
        1
    );
    assert_eq!(tables.dictionary_index(handle(2)).unwrap(), 1);
    assert_eq!(tables.dictionary_index(handle(1)).unwrap(), 0);
    tables.rollback();

    assert_eq!(remaining_accounts, accounts_before);
    assert_eq!(dictionary, dictionary_before);
    assert_eq!(persistent_producers, producers_before);
}

#[test]
fn step_that_fails_after_interning_leaves_the_builder_untouched() {
    let authority = Pubkey::new_unique();
    let written_key = encrypted_value_id(authority, 7);
    let mut builder = BatchBuilder::new(app_authority(authority));
    builder
        .trivial_encrypt_u64(
            7,
            Output::persistent(PersistentOutput::create(
                written_key.clone(),
                subjects(authority),
            )),
        )
        .unwrap();
    let accounts_before = builder.remaining_accounts.clone();
    let dictionary_before = builder.dictionary.clone();
    let producers_before = builder.persistent_producers.clone();

    // The left operand interns a fresh handle and a fresh input-ACL account; the right operand then
    // fails because the step above already wrote its account.
    let fresh = Uint64Handle::persistent(balance_handle(1), encrypted_value_id(authority, 1))
        .expect("fresh operand");
    let written =
        Uint64Handle::persistent(balance_handle(2), written_key).expect("written operand");
    let error = builder
        .add(fresh, written, Output::transient())
        .unwrap_err();

    assert_eq!(error, BatchBuildError::PersistentOperandWrittenEarlier);
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
