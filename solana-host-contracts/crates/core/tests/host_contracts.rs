use solana_host_contracts_core::{
    AclSession, AclState, BinaryOperand, CiphertextVerification, ContextUserInputs, EvmAddress,
    ExecutionMeta, ExecutorState, FheType, Handle, HcuConfig, HcuLimitState, HcuOperationKey,
    HostContractError, HostInstruction, HostProgramConfig, HostProgramSession, HostProgramState,
    InputProofVerifier, InputVerifierSession, InputVerifierState, KmsProofVerifier,
    KmsVerifierState, Operator, ProgramContext, Pubkey, PublicDecryptVerification, Result,
    SignatureThreshold, TransactionMeter, VerifierContextConfig, MAX_DECRYPTED_RESULT_BYTES,
    MAX_VERIFIER_SIGNERS,
};
use std::sync::atomic::{AtomicUsize, Ordering};

struct CountingInputVerifier<'a> {
    calls: &'a AtomicUsize,
}

impl InputProofVerifier for CountingInputVerifier<'_> {
    fn verify(
        &self,
        _payload: &CiphertextVerification,
        signatures: &[Vec<u8>],
        signers: &[EvmAddress],
        threshold: SignatureThreshold,
        _source_chain_id: u64,
        _source_contract: EvmAddress,
    ) -> Result<()> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        assert_eq!(signatures.len(), threshold as usize);
        assert!(signers.len() >= threshold as usize);
        Ok(())
    }
}

struct MockKmsVerifier;

impl KmsProofVerifier for MockKmsVerifier {
    fn verify(
        &self,
        payload: &PublicDecryptVerification,
        signatures: &[Vec<u8>],
        signers: &[EvmAddress],
        threshold: SignatureThreshold,
        _source_chain_id: u64,
        _source_contract: EvmAddress,
    ) -> Result<()> {
        assert!(!payload.ct_handles.is_empty());
        assert!(signers.len() >= threshold as usize);
        assert_eq!(signatures.len(), threshold as usize);
        Ok(())
    }
}

struct AcceptAllInputVerifier;

impl InputProofVerifier for AcceptAllInputVerifier {
    fn verify(
        &self,
        _payload: &CiphertextVerification,
        signatures: &[Vec<u8>],
        signers: &[EvmAddress],
        threshold: SignatureThreshold,
        _source_chain_id: u64,
        _source_contract: EvmAddress,
    ) -> Result<()> {
        assert!(signers.len() >= threshold as usize);
        assert!(signatures.len() >= threshold as usize || signatures.is_empty());
        Ok(())
    }
}

fn pubkey(byte: u8) -> Pubkey {
    Pubkey::new([byte; 32])
}

fn evm(byte: u8) -> EvmAddress {
    EvmAddress::new([byte; 20])
}

fn meta(caller: Pubkey, chain_id: u64) -> ExecutionMeta {
    ExecutionMeta {
        chain_id,
        slot: 42,
        timestamp: 1_700_000_000,
        recent_blockhash: [9; 32],
        caller,
    }
}

fn program_context(caller: Pubkey, chain_id: u64, slot: u64) -> ProgramContext {
    ProgramContext {
        caller,
        chain_id,
        slot,
        timestamp: 1_700_000_000,
        recent_blockhash: [9; 32],
    }
}

fn program_config(
    owner: Pubkey,
    upgrade_authority: Pubkey,
    acl_program: Pubkey,
) -> HostProgramConfig {
    HostProgramConfig {
        owner,
        upgrade_authority,
        acl_program,
        host_chain_id: 999,
        input_verifier: VerifierContextConfig {
            source_contract: evm(20),
            source_chain_id: 54321,
            signers: vec![evm(21), evm(22)],
            threshold: 1,
        },
        kms_verifier: VerifierContextConfig {
            source_contract: evm(30),
            source_chain_id: 54321,
            signers: vec![evm(31), evm(32)],
            threshold: 1,
        },
        hcu: HcuConfig {
            hcu_cap_per_block: 10_000_000,
            max_hcu_depth_per_tx: 8_000_000,
            max_hcu_per_tx: 9_000_000,
        },
    }
}

#[test]
fn executor_binary_op_creates_computation_handle_and_transient_acl_access() {
    let owner = pubkey(1);
    let caller = pubkey(2);
    let acl_program = pubkey(3);

    let mut acl = AclState::new(owner, acl_program);
    let mut session = AclSession::default();

    let input_handle =
        ExecutorState::compute_input_handles(b"ciphertext", &[8], acl_program, 4242).unwrap()[0];

    session.allow(input_handle, caller);
    acl.allow(caller, input_handle, caller, &session).unwrap();

    let mut executor = ExecutorState::new(acl_program);
    let (result, event) = executor
        .binary_op(
            Operator::FheAdd,
            input_handle,
            BinaryOperand::Handle(input_handle),
            meta(caller, 4242),
            &acl,
            &mut session,
            FheType::Uint8,
            None,
        )
        .unwrap();

    assert_eq!(result.index(), 0xff);
    assert_eq!(result.chain_id(), 4242);
    assert_eq!(result.fhe_type().unwrap(), FheType::Uint8);
    assert!(acl.is_allowed(result, caller, &session));
    assert!(matches!(
        event,
        solana_host_contracts_core::HostEvent::Operation {
            op: Operator::FheAdd,
            ..
        }
    ));
}

#[test]
fn acl_delegation_matches_host_semantics() {
    let owner = pubkey(1);
    let delegator = pubkey(2);
    let delegate = pubkey(3);
    let contract = pubkey(4);
    let acl_program = pubkey(5);
    let mut acl = AclState::new(owner, acl_program);
    let session = AclSession::default();

    let handle =
        ExecutorState::compute_input_handles(b"ciphertext", &[8], acl_program, 7).unwrap()[0];

    let mut bootstrap = AclSession::default();
    bootstrap.allow(handle, delegator);
    bootstrap.allow(handle, contract);
    acl.allow(delegator, handle, delegator, &bootstrap).unwrap();
    acl.allow(contract, handle, contract, &bootstrap).unwrap();

    acl.delegate_for_user_decryption(delegator, delegate, contract, 2_000, 1_000, 10)
        .unwrap();

    assert!(
        acl.is_handle_delegated_for_user_decryption(delegator, delegate, contract, handle, 1_500,)
    );

    acl.revoke_delegation_for_user_decryption(delegator, delegate, contract, 11)
        .unwrap();

    assert!(
        !acl.is_handle_delegated_for_user_decryption(delegator, delegate, contract, handle, 1_500,)
    );
    assert!(!acl.is_allowed(handle, delegate, &session));
}

#[test]
fn input_verifier_parses_host_proof_and_uses_session_cache() {
    let source_contract = evm(7);
    let signers = vec![evm(8), evm(9)];
    let mut verifier = InputVerifierState::new(source_contract, 54321, signers, 2).unwrap();
    let chain_id = 111;
    let acl_program = pubkey(3);
    let handle =
        ExecutorState::compute_input_handles(b"ciphertext", &[8, 16], acl_program, chain_id)
            .unwrap()[1];

    let mut proof = Vec::new();
    proof.push(2);
    proof.push(2);
    for input_handle in
        ExecutorState::compute_input_handles(b"ciphertext", &[8, 16], acl_program, chain_id)
            .unwrap()
    {
        proof.extend_from_slice(input_handle.as_bytes());
    }
    proof.extend_from_slice(&[1; 65]);
    proof.extend_from_slice(&[2; 65]);
    proof.extend_from_slice(&[0xAA, 0xBB]);

    let calls = AtomicUsize::new(0);
    let proof_verifier = CountingInputVerifier { calls: &calls };
    let context = ContextUserInputs {
        user_id: pubkey(4),
        contract_id: pubkey(5),
    };
    let mut session = InputVerifierSession::default();

    let verified_once = verifier
        .verify_input(
            context,
            handle,
            &proof,
            &mut session,
            &proof_verifier,
            chain_id,
        )
        .unwrap();
    let verified_twice = verifier
        .verify_input(
            context,
            handle,
            &proof,
            &mut session,
            &proof_verifier,
            chain_id,
        )
        .unwrap();

    assert_eq!(verified_once, handle);
    assert_eq!(verified_twice, handle);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[test]
fn kms_verifier_supports_v1_context_selection() {
    let mut kms = KmsVerifierState::new(evm(1), 54321, vec![evm(2), evm(3)], 2).unwrap();
    kms.define_new_context(vec![evm(4), evm(5), evm(6)], 2)
        .unwrap();
    let current_context = kms.get_current_kms_context_id();

    let handle = Handle::new([0x11; 32]);
    let mut proof = Vec::new();
    proof.push(2);
    proof.extend_from_slice(&[7; 65]);
    proof.extend_from_slice(&[8; 65]);
    proof.push(0x01);
    proof.extend_from_slice(current_context.as_bytes());

    assert!(kms
        .verify_decryption_signatures(vec![handle], vec![1, 2, 3], &proof, &MockKmsVerifier)
        .unwrap());
}

#[test]
fn hcu_meter_enforces_transaction_and_block_caps() {
    let caller = pubkey(9);
    let mut hcu = HcuLimitState::new(100, 80, 90).unwrap();
    hcu.set_pricing(
        HcuOperationKey {
            op: Operator::FheAdd,
            result_type: FheType::Uint8,
            scalar: false,
        },
        40,
    );

    let left = Handle::new([1; 32]);
    let right = Handle::new([2; 32]);
    let result = Handle::new([3; 32]);
    let next_result = Handle::new([4; 32]);
    let mut meter = TransactionMeter::default();

    hcu.charge_for_operation(
        HcuOperationKey {
            op: Operator::FheAdd,
            result_type: FheType::Uint8,
            scalar: false,
        },
        &[left, right],
        result,
        caller,
        77,
        &mut meter,
    )
    .unwrap();

    hcu.charge_for_operation(
        HcuOperationKey {
            op: Operator::FheAdd,
            result_type: FheType::Uint8,
            scalar: false,
        },
        &[result],
        next_result,
        caller,
        77,
        &mut meter,
    )
    .unwrap();

    let third_result = Handle::new([5; 32]);
    let err = hcu
        .charge_for_operation(
            HcuOperationKey {
                op: Operator::FheAdd,
                result_type: FheType::Uint8,
                scalar: false,
            },
            &[next_result],
            third_result,
            caller,
            77,
            &mut meter,
        )
        .unwrap_err();

    assert!(matches!(
        err,
        solana_host_contracts_core::HostContractError::HCUTransactionLimitExceeded
            | solana_host_contracts_core::HostContractError::HCUBlockLimitExceeded
            | solana_host_contracts_core::HostContractError::HCUTransactionDepthLimitExceeded
    ));
}

#[test]
fn input_verifier_context_validation_matches_host_contract_reverts() {
    let mut verifier = InputVerifierState::new(evm(1), 54321, vec![evm(2), evm(3)], 1).unwrap();

    assert_eq!(
        verifier
            .define_new_context(vec![EvmAddress::ZERO], 1)
            .unwrap_err(),
        HostContractError::SignerNull
    );
    assert_eq!(
        verifier
            .define_new_context(vec![evm(4), evm(4)], 1)
            .unwrap_err(),
        HostContractError::SignerAlreadyRegistered
    );
    assert_eq!(
        verifier.define_new_context(vec![], 0).unwrap_err(),
        HostContractError::SignersSetIsEmpty
    );
    assert_eq!(
        verifier.define_new_context(vec![evm(5)], 0).unwrap_err(),
        HostContractError::ThresholdIsNull
    );
    assert_eq!(
        verifier.define_new_context(vec![evm(5)], 2).unwrap_err(),
        HostContractError::ThresholdIsAboveNumberOfSigners
    );

    let event = verifier
        .define_new_context(vec![evm(6), evm(7)], 2)
        .unwrap();
    assert!(matches!(
        event,
        solana_host_contracts_core::HostEvent::InputVerifierContextUpdated { threshold: 2, .. }
    ));
    assert!(verifier.is_signer(evm(6)));
    assert!(!verifier.is_signer(evm(2)));
    assert_eq!(verifier.get_threshold(), 2);
}

#[test]
fn kms_context_validation_and_lifecycle_match_host_contract_reverts() {
    let mut kms = KmsVerifierState::new(evm(1), 54321, vec![evm(2), evm(3)], 1).unwrap();

    assert_eq!(
        kms.define_new_context(vec![EvmAddress::ZERO], 1)
            .unwrap_err(),
        HostContractError::SignerNull
    );
    assert_eq!(
        kms.define_new_context(vec![evm(4), evm(4)], 1).unwrap_err(),
        HostContractError::KmsAlreadySigner
    );
    assert_eq!(
        kms.define_new_context(vec![], 0).unwrap_err(),
        HostContractError::SignersSetIsEmpty
    );
    assert_eq!(
        kms.define_new_context(vec![evm(5)], 0).unwrap_err(),
        HostContractError::ThresholdIsNull
    );
    assert_eq!(
        kms.define_new_context(vec![evm(5)], 2).unwrap_err(),
        HostContractError::ThresholdIsAboveNumberOfSigners
    );

    let old_context = kms.get_current_kms_context_id();
    let event = kms.define_new_context(vec![evm(6)], 1).unwrap();
    let current_context = kms.get_current_kms_context_id();
    assert_ne!(old_context, current_context);
    assert!(matches!(
        event,
        solana_host_contracts_core::HostEvent::KmsContextUpdated {
            kms_context_id,
            threshold: 1,
            ..
        } if kms_context_id == current_context
    ));
    assert_eq!(
        kms.get_signers_for_kms_context(old_context),
        vec![evm(2), evm(3)]
    );
    assert_eq!(kms.get_kms_signers(), vec![evm(6)]);
    assert_eq!(kms.get_threshold(), 1);
    assert!(kms.is_signer(evm(6)));

    assert_eq!(
        kms.destroy_kms_context(current_context).unwrap_err(),
        HostContractError::CurrentKmsContextCannotBeDestroyed
    );

    let destroy_event = kms.destroy_kms_context(old_context).unwrap();
    assert!(matches!(
        destroy_event,
        solana_host_contracts_core::HostEvent::KmsContextDestroyed { kms_context_id } if kms_context_id == old_context
    ));

    let mut extra_data = vec![0x01];
    extra_data.extend_from_slice(old_context.as_bytes());
    assert_eq!(
        kms.get_context_signers_and_threshold_from_extra_data(&extra_data)
            .unwrap_err(),
        HostContractError::InvalidKmsContext(old_context)
    );
}

#[test]
fn program_authority_and_migration_controls_are_enforced() {
    let owner = pubkey(1);
    let upgrade_authority = pubkey(2);
    let acl_program = pubkey(3);
    let non_owner = pubkey(4);
    let mut program =
        HostProgramState::new(program_config(owner, upgrade_authority, acl_program)).unwrap();
    let mut session = HostProgramSession::default();

    let err = program
        .process_instruction(
            &HostInstruction::DefineInputVerifierContext {
                signers: vec![evm(9)],
                threshold: 1,
            },
            program_context(non_owner, 999, 1),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap_err();
    assert_eq!(err, HostContractError::SenderNotAllowed);

    let result = program
        .process_instruction(
            &HostInstruction::DefineInputVerifierContext {
                signers: vec![evm(9), evm(10)],
                threshold: 2,
            },
            program_context(owner, 999, 1),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap();
    assert!(matches!(
        result.events.as_slice(),
        [solana_host_contracts_core::HostEvent::InputVerifierContextUpdated { threshold: 2, .. }]
    ));
    assert_eq!(program.input_verifier().get_threshold(), 2);

    let err = program
        .process_instruction(
            &HostInstruction::Migrate {
                new_state_version: 2,
            },
            program_context(non_owner, 999, 1),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap_err();
    assert_eq!(err, HostContractError::NotUpgradeAuthority);

    program
        .process_instruction(
            &HostInstruction::Migrate {
                new_state_version: 2,
            },
            program_context(upgrade_authority, 999, 1),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap();
    assert_eq!(program.state_version(), 2);

    let err = program
        .process_instruction(
            &HostInstruction::Migrate {
                new_state_version: 2,
            },
            program_context(upgrade_authority, 999, 1),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap_err();
    assert_eq!(
        err,
        HostContractError::InvalidStateVersionTransition {
            current: 2,
            requested: 2,
        }
    );
}

#[test]
fn program_dispatches_executor_and_acl_flows() {
    let owner = pubkey(1);
    let upgrade_authority = pubkey(2);
    let acl_program = pubkey(3);
    let caller = pubkey(4);
    let contract = pubkey(5);
    let chain_id = 777;
    let mut program =
        HostProgramState::new(program_config(owner, upgrade_authority, acl_program)).unwrap();
    let mut session = HostProgramSession::default();

    let input_handle =
        ExecutorState::compute_input_handles(b"ciphertext", &[8], acl_program, chain_id).unwrap()
            [0];
    session.acl_session.allow(input_handle, caller);

    program
        .process_instruction(
            &HostInstruction::Allow {
                handle: input_handle,
                account: caller,
            },
            program_context(caller, chain_id, 42),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap();

    session.acl_session.allow(input_handle, contract);
    program
        .process_instruction(
            &HostInstruction::Allow {
                handle: input_handle,
                account: contract,
            },
            program_context(contract, chain_id, 42),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap();

    let result = program
        .process_instruction(
            &HostInstruction::BinaryOp {
                op: Operator::FheAdd,
                lhs: input_handle,
                rhs: BinaryOperand::Handle(input_handle),
                result_type: FheType::Uint8,
                charge_hcu: false,
            },
            program_context(caller, chain_id, 42),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap();
    let result_handle = result.returned_handle.unwrap();
    assert!(program
        .acl()
        .is_allowed(result_handle, caller, &session.acl_session));

    program
        .process_instruction(
            &HostInstruction::AllowForDecryption {
                handles: vec![result_handle],
            },
            program_context(caller, chain_id, 42),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap();
    assert!(program.acl().is_allowed_for_decryption(result_handle));

    let delegation_result = program
        .process_instruction(
            &HostInstruction::DelegateForUserDecryption {
                delegate: pubkey(6),
                contract_address: contract,
                expiration_date: 1_700_000_100,
            },
            program_context(caller, chain_id, 43),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap();
    assert!(matches!(
        delegation_result.events.as_slice(),
        [solana_host_contracts_core::HostEvent::DelegatedForUserDecryption { .. }]
    ));
    assert!(program.acl().is_handle_delegated_for_user_decryption(
        caller,
        pubkey(6),
        contract,
        input_handle,
        1_700_000_050,
    ));
}

#[test]
fn program_pauser_and_hcu_admin_paths_match_host_expectations() {
    let owner = pubkey(1);
    let upgrade_authority = pubkey(2);
    let acl_program = pubkey(3);
    let pauser = pubkey(4);
    let delegator = pubkey(5);
    let mut program =
        HostProgramState::new(program_config(owner, upgrade_authority, acl_program)).unwrap();
    let mut session = HostProgramSession::default();

    program
        .process_instruction(
            &HostInstruction::AddPauser { account: pauser },
            program_context(owner, 999, 1),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap();
    assert!(program.acl().is_pauser(pauser));

    program
        .process_instruction(
            &HostInstruction::Pause,
            program_context(pauser, 999, 2),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap();
    assert!(program.acl().is_paused());

    let err = program
        .process_instruction(
            &HostInstruction::DelegateForUserDecryption {
                delegate: pubkey(6),
                contract_address: pubkey(7),
                expiration_date: 1_700_000_100,
            },
            program_context(delegator, 999, 3),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap_err();
    assert_eq!(err, HostContractError::Paused);

    let err = program
        .process_instruction(
            &HostInstruction::SetHcuPerBlock { hcu_per_block: 80 },
            program_context(owner, 999, 3),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap_err();
    assert_eq!(err, HostContractError::HCUPerBlockBelowMaxPerTx);

    program
        .process_instruction(
            &HostInstruction::Unpause,
            program_context(owner, 999, 4),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap();
    assert!(!program.acl().is_paused());

    let err = program
        .process_instruction(
            &HostInstruction::SetMaxHcuDepthPerTx {
                max_hcu_depth_per_tx: 9_000_001,
            },
            program_context(owner, 999, 5),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap_err();
    assert_eq!(err, HostContractError::MaxHCUPerTxBelowDepth);

    program
        .process_instruction(
            &HostInstruction::AddToBlockHcuWhitelist { account: pauser },
            program_context(owner, 999, 5),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap();
    assert!(program.hcu_limit().is_block_hcu_whitelisted(pauser));
}

#[test]
fn verifier_context_rejects_signer_sets_above_limit() {
    let too_many_signers = (0..=MAX_VERIFIER_SIGNERS)
        .map(|idx| evm((idx as u8).wrapping_add(1)))
        .collect::<Vec<_>>();

    let err = InputVerifierState::new(evm(7), 54321, too_many_signers.clone(), 1).unwrap_err();
    assert_eq!(
        err,
        HostContractError::TooManySigners {
            max: MAX_VERIFIER_SIGNERS,
        }
    );

    let err = KmsVerifierState::new(evm(8), 54321, too_many_signers, 1).unwrap_err();
    assert_eq!(
        err,
        HostContractError::TooManySigners {
            max: MAX_VERIFIER_SIGNERS,
        }
    );
}

#[test]
fn kms_verifier_rejects_oversized_decrypted_payload() {
    let kms = KmsVerifierState::new(evm(8), 54321, vec![evm(9), evm(10)], 1).unwrap();
    let mut proof = vec![1];
    proof.resize(66, 0);
    let decrypted_result = vec![0_u8; MAX_DECRYPTED_RESULT_BYTES + 1];

    let err = kms
        .verify_decryption_signatures(
            vec![Handle::new([0x11; 32])],
            decrypted_result,
            &proof,
            &MockKmsVerifier,
        )
        .unwrap_err();
    assert_eq!(
        err,
        HostContractError::DecryptedResultTooLarge {
            max: MAX_DECRYPTED_RESULT_BYTES,
        }
    );
}

#[test]
fn delegation_expiration_equal_to_current_timestamp_is_rejected() {
    let owner = pubkey(1);
    let acl_program = pubkey(2);
    let mut acl = AclState::new(owner, acl_program);

    let err = acl
        .delegate_for_user_decryption(pubkey(3), pubkey(4), pubkey(5), 1_000, 1_000, 9)
        .unwrap_err();
    assert_eq!(err, HostContractError::ExpirationDateInThePast);
}

#[test]
fn binary_comparison_requires_boolean_result_type() {
    let owner = pubkey(1);
    let caller = pubkey(2);
    let acl_program = pubkey(3);
    let mut acl = AclState::new(owner, acl_program);
    let mut session = AclSession::default();
    let handle =
        ExecutorState::compute_input_handles(b"ciphertext", &[8], acl_program, 4242).unwrap()[0];

    session.allow(handle, caller);
    acl.allow(caller, handle, caller, &session).unwrap();

    let mut executor = ExecutorState::new(acl_program);
    let err = executor
        .binary_op(
            Operator::FheEq,
            handle,
            BinaryOperand::Handle(handle),
            meta(caller, 4242),
            &acl,
            &mut session,
            FheType::Uint8,
            None,
        )
        .unwrap_err();

    assert_eq!(err, HostContractError::InvalidType);
}

#[test]
fn hcu_is_enforced_even_when_instruction_requests_no_charge() {
    let owner = pubkey(1);
    let upgrade_authority = pubkey(2);
    let acl_program = pubkey(3);
    let caller = pubkey(4);
    let mut config = program_config(owner, upgrade_authority, acl_program);
    config.hcu = HcuConfig {
        hcu_cap_per_block: 50_000,
        max_hcu_depth_per_tx: 50_000,
        max_hcu_per_tx: 50_000,
    };
    let mut program = HostProgramState::new(config).unwrap();
    let mut session = HostProgramSession::default();
    let input_handle =
        ExecutorState::compute_input_handles(b"ciphertext", &[8], acl_program, 999).unwrap()[0];

    session.acl_session.allow(input_handle, caller);
    program
        .process_instruction(
            &HostInstruction::Allow {
                handle: input_handle,
                account: caller,
            },
            program_context(caller, 999, 42),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap();

    let err = program
        .process_instruction(
            &HostInstruction::BinaryOp {
                op: Operator::FheAdd,
                lhs: input_handle,
                rhs: BinaryOperand::Handle(input_handle),
                result_type: FheType::Uint8,
                charge_hcu: false,
            },
            program_context(caller, 999, 42),
            &mut session,
            &AcceptAllInputVerifier,
            &MockKmsVerifier,
        )
        .unwrap_err();

    assert_eq!(err, HostContractError::HCUBlockLimitExceeded);
}
