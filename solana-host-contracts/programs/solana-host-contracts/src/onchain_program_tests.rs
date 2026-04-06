use crate::{
    onchain::{
        decode_state, encode_instruction, find_session_pda, find_state_pda, id as program_id,
        process_instruction, required_session_account_len, required_state_account_len,
        state_account_len_with_reserve, OnchainInstruction,
    },
    ContextUserInputs, EvmAddress, ExecutorState, FheType, HcuConfig, HostInstruction,
    HostProgramConfig, Pubkey, PublicDecryptVerification, Secp256k1ProofVerifier,
    VerifierContextConfig,
};
use k256::ecdsa::SigningKey;
use sha3::{Digest, Keccak256};
use solana_program::{
    account_info::AccountInfo, clock::Clock, hash::Hash, program_error::ProgramError,
    pubkey::Pubkey as SolanaPubkey, sysvar,
};
use solana_slot_hashes::SlotHashes;
use solana_system_interface::program as system_program;

struct TestAccount {
    key: SolanaPubkey,
    lamports: u64,
    data: Vec<u8>,
    owner: SolanaPubkey,
    is_signer: bool,
    is_writable: bool,
}

impl TestAccount {
    fn new(
        key: SolanaPubkey,
        owner: SolanaPubkey,
        is_signer: bool,
        is_writable: bool,
        data: Vec<u8>,
    ) -> Self {
        Self {
            key,
            lamports: 10_000_000,
            data,
            owner,
            is_signer,
            is_writable,
        }
    }

    fn account_info(&mut self) -> AccountInfo<'_> {
        AccountInfo::new(
            &self.key,
            self.is_signer,
            self.is_writable,
            &mut self.lamports,
            &mut self.data,
            &self.owner,
            false,
        )
    }
}

fn custom_pubkey(byte: u8) -> Pubkey {
    Pubkey::new([byte; 32])
}

fn evm(byte: u8) -> EvmAddress {
    EvmAddress::new([byte; 20])
}

fn host_program_config_with_signers(
    owner: Pubkey,
    input_signers: Vec<EvmAddress>,
    kms_signers: Vec<EvmAddress>,
) -> HostProgramConfig {
    HostProgramConfig {
        owner,
        upgrade_authority: custom_pubkey(2),
        acl_program: custom_pubkey(3),
        host_chain_id: 777,
        input_verifier: VerifierContextConfig {
            source_contract: evm(20),
            source_chain_id: 54321,
            signers: input_signers,
            threshold: 1,
        },
        kms_verifier: VerifierContextConfig {
            source_contract: evm(30),
            source_chain_id: 54321,
            signers: kms_signers,
            threshold: 1,
        },
        hcu: HcuConfig {
            hcu_cap_per_block: 10_000_000,
            max_hcu_depth_per_tx: 8_000_000,
            max_hcu_per_tx: 9_000_000,
        },
    }
}

fn host_program_config(owner: Pubkey) -> HostProgramConfig {
    host_program_config_with_signers(owner, vec![evm(21), evm(22)], vec![evm(31), evm(32)])
}

fn clock_account(slot: u64, unix_timestamp: i64) -> TestAccount {
    let clock = Clock {
        slot,
        epoch_start_timestamp: unix_timestamp - 10,
        epoch: 1,
        leader_schedule_epoch: 1,
        unix_timestamp,
    };
    TestAccount::new(
        solana_program::sysvar::clock::id(),
        sysvar::id(),
        false,
        false,
        bincode::serialize(&clock).unwrap(),
    )
}

fn slot_hashes_account(slot: u64, hash_byte: u8) -> TestAccount {
    let slot_hashes = SlotHashes::new(&[(slot, Hash::new_from_array([hash_byte; 32]))]);
    TestAccount::new(
        solana_program::sysvar::slot_hashes::id(),
        sysvar::id(),
        false,
        false,
        bincode::serialize(&slot_hashes).unwrap(),
    )
}

fn system_program_account() -> TestAccount {
    TestAccount::new(system_program::id(), system_program::id(), false, false, Vec::new())
}

fn rent_account() -> TestAccount {
    TestAccount::new(
        solana_program::sysvar::rent::id(),
        sysvar::id(),
        false,
        false,
        bincode::serialize(&solana_program::rent::Rent::default()).unwrap(),
    )
}

fn session_account(program_id: SolanaPubkey, caller: Pubkey) -> TestAccount {
    let (session_pda, _) = find_session_pda(&program_id, &SolanaPubkey::from(caller));
    TestAccount::new(
        session_pda,
        program_id,
        false,
        true,
        vec![0; required_session_account_len().unwrap() + 4096],
    )
}

fn signing_key(byte: u8) -> SigningKey {
    let mut secret = [0_u8; 32];
    secret[31] = byte;
    SigningKey::from_bytes((&secret).into()).unwrap()
}

fn evm_address(signing_key: &SigningKey) -> EvmAddress {
    let encoded = signing_key.verifying_key().to_encoded_point(false);
    let pubkey = &encoded.as_bytes()[1..65];
    let hash = Keccak256::digest(pubkey);
    let mut address = [0_u8; 20];
    address.copy_from_slice(&hash[12..]);
    EvmAddress::new(address)
}

fn sign_message(signing_key: &SigningKey, message: &[u8]) -> Vec<u8> {
    let (signature, recovery_id) = signing_key
        .sign_digest_recoverable(Keccak256::new_with_prefix(message))
        .unwrap();
    let mut bytes = signature.to_bytes().to_vec();
    bytes.push(recovery_id.to_byte());
    bytes
}

#[test]
fn initialize_pda_and_admin_calls_persist_on_chain_state() {
    let program_id = program_id();
    let owner = custom_pubkey(1);
    let pauser = custom_pubkey(9);
    let (state_pda, _) = find_state_pda(&program_id);
    let config = host_program_config(owner);
    let mut owner_account = TestAccount::new(
        SolanaPubkey::from(owner),
        SolanaPubkey::default(),
        true,
        false,
        Vec::new(),
    );
    let mut pauser_account = TestAccount::new(
        SolanaPubkey::from(pauser),
        SolanaPubkey::default(),
        true,
        false,
        Vec::new(),
    );
    let mut state_account = TestAccount::new(
        state_pda,
        program_id,
        false,
        true,
        vec![0; state_account_len_with_reserve(&config, 512).unwrap()],
    );
    let mut owner_session = session_account(program_id, owner);
    let mut pauser_session = session_account(program_id, pauser);

    let init_ix = encode_instruction(&OnchainInstruction::InitializePda { config }).unwrap();
    {
        let init_accounts = vec![owner_account.account_info(), state_account.account_info()];
        process_instruction(&program_id, &init_accounts, &init_ix).unwrap();
    }

    let state = decode_state(&state_account.data).unwrap();
    assert_eq!(state.owner(), owner);
    assert_eq!(state.host_chain_id(), 777);
    assert!(!state.acl().is_paused());

    let mut clock = clock_account(42, 1_700_000_050);
    let mut slot_hashes = slot_hashes_account(41, 7);
    let mut system_program = system_program_account();
    let mut rent = rent_account();
    let add_pauser_ix = encode_instruction(&OnchainInstruction::Execute {
        instruction: HostInstruction::AddPauser { account: pauser },
        session_nonce: 1,
        recent_blockhash: [7; 32],
    })
    .unwrap();
    {
        let add_pauser_accounts = vec![
            owner_account.account_info(),
            state_account.account_info(),
            owner_session.account_info(),
            clock.account_info(),
            slot_hashes.account_info(),
            system_program.account_info(),
            rent.account_info(),
        ];
        process_instruction(&program_id, &add_pauser_accounts, &add_pauser_ix).unwrap();
    }

    let mut clock = clock_account(43, 1_700_000_060);
    let mut slot_hashes = slot_hashes_account(42, 8);
    let mut system_program = system_program_account();
    let mut rent = rent_account();
    let pause_ix = encode_instruction(&OnchainInstruction::Execute {
        instruction: HostInstruction::Pause,
        session_nonce: 1,
        recent_blockhash: [8; 32],
    })
    .unwrap();
    {
        let pause_accounts = vec![
            pauser_account.account_info(),
            state_account.account_info(),
            pauser_session.account_info(),
            clock.account_info(),
            slot_hashes.account_info(),
            system_program.account_info(),
            rent.account_info(),
        ];
        process_instruction(&program_id, &pause_accounts, &pause_ix).unwrap();
    }

    let state = decode_state(&state_account.data).unwrap();
    assert!(state.acl().is_pauser(pauser));
    assert!(state.acl().is_paused());
}

#[test]
fn execute_fhe_rand_updates_program_state() {
    let program_id = program_id();
    let owner = custom_pubkey(1);
    let mut owner_account = TestAccount::new(
        SolanaPubkey::from(owner),
        SolanaPubkey::default(),
        true,
        false,
        Vec::new(),
    );
    let mut state_account = TestAccount::new(
        SolanaPubkey::new_from_array([0x66; 32]),
        program_id,
        false,
        true,
        vec![0; required_state_account_len(&host_program_config(owner)).unwrap()],
    );
    let mut owner_session = session_account(program_id, owner);

    let init_ix = encode_instruction(&OnchainInstruction::Initialize {
        config: host_program_config(owner),
    })
    .unwrap();
    {
        let init_accounts = vec![owner_account.account_info(), state_account.account_info()];
        process_instruction(&program_id, &init_accounts, &init_ix).unwrap();
    }

    let mut clock = clock_account(77, 1_700_000_100);
    let mut slot_hashes = slot_hashes_account(76, 0xAB);
    let mut system_program = system_program_account();
    let mut rent = rent_account();
    let execute_ix = encode_instruction(&OnchainInstruction::Execute {
        instruction: HostInstruction::FheRand {
            rand_type: FheType::Uint8,
            charge_hcu: false,
        },
        session_nonce: 1,
        recent_blockhash: [0xAB; 32],
    })
    .unwrap();
    {
        let execute_accounts = vec![
            owner_account.account_info(),
            state_account.account_info(),
            owner_session.account_info(),
            clock.account_info(),
            slot_hashes.account_info(),
            system_program.account_info(),
            rent.account_info(),
        ];
        process_instruction(&program_id, &execute_accounts, &execute_ix).unwrap();
    }

    let state = decode_state(&state_account.data).unwrap();
    assert_eq!(state.executor().counter_rand(), 1);
}

#[test]
fn execute_verify_input_returns_verified_handle_on_chain() {
    let program_id = program_id();
    let owner = custom_pubkey(1);
    let signer1 = signing_key(1);
    let signer2 = signing_key(2);
    let config = host_program_config_with_signers(
        owner,
        vec![evm_address(&signer1), evm_address(&signer2)],
        vec![evm(31), evm(32)],
    );
    let acl_program = config.acl_program;
    let host_chain_id = config.host_chain_id;
    let mut owner_account = TestAccount::new(
        SolanaPubkey::from(owner),
        SolanaPubkey::default(),
        true,
        false,
        Vec::new(),
    );
    let mut state_account = TestAccount::new(
        SolanaPubkey::new_from_array([0x77; 32]),
        program_id,
        false,
        true,
        vec![0; required_state_account_len(&config).unwrap()],
    );
    let mut owner_session = session_account(program_id, owner);

    let init_ix = encode_instruction(&OnchainInstruction::Initialize { config }).unwrap();
    {
        let init_accounts = vec![owner_account.account_info(), state_account.account_info()];
        process_instruction(&program_id, &init_accounts, &init_ix).unwrap();
    }

    let handles =
        ExecutorState::compute_input_handles(b"ciphertext", &[8, 16], acl_program, host_chain_id)
            .unwrap();
    let context = ContextUserInputs {
        user_address: evm(4),
        contract_address: evm(5),
    };
    let extra_data = vec![0xAA, 0xBB, 0xCC];
    let payload = crate::CiphertextVerification {
        ct_handles: handles.clone(),
        user_address: context.user_address,
        contract_address: context.contract_address,
        contract_chain_id: host_chain_id,
        extra_data: extra_data.clone(),
    };

    let message = Secp256k1ProofVerifier::input_verification_message(
        &payload,
        54321,
        EvmAddress::new([20; 20]),
    );
    let signature1 = sign_message(&signer1, &message);
    let signature2 = sign_message(&signer2, &message);

    let mut proof = Vec::new();
    proof.push(handles.len() as u8);
    proof.push(2);
    for handle in &handles {
        proof.extend_from_slice(handle.as_bytes());
    }
    proof.extend_from_slice(&signature1);
    proof.extend_from_slice(&signature2);
    proof.extend_from_slice(&extra_data);

    let mut clock = clock_account(80, 1_700_000_120);
    let mut slot_hashes = slot_hashes_account(79, 0xBC);
    let mut system_program = system_program_account();
    let mut rent = rent_account();
    let verify_ix = encode_instruction(&OnchainInstruction::Execute {
        instruction: HostInstruction::VerifyInput {
            context,
            input_handle: handles[1],
            input_proof: proof,
        },
        session_nonce: 1,
        recent_blockhash: [0xBC; 32],
    })
    .unwrap();
    {
        let verify_accounts = vec![
            owner_account.account_info(),
            state_account.account_info(),
            owner_session.account_info(),
            clock.account_info(),
            slot_hashes.account_info(),
            system_program.account_info(),
            rent.account_info(),
        ];
        process_instruction(&program_id, &verify_accounts, &verify_ix).unwrap();
    }

    let state = decode_state(&state_account.data).unwrap();
    assert_eq!(state.owner(), owner);
    assert_eq!(state.input_verifier().get_threshold(), 1);
}

#[test]
fn execute_verify_decryption_signatures_returns_true_on_chain() {
    let program_id = program_id();
    let owner = custom_pubkey(1);
    let signer1 = signing_key(11);
    let signer2 = signing_key(12);
    let config = host_program_config_with_signers(
        owner,
        vec![evm(21), evm(22)],
        vec![evm_address(&signer1), evm_address(&signer2)],
    );
    let mut owner_account = TestAccount::new(
        SolanaPubkey::from(owner),
        SolanaPubkey::default(),
        true,
        false,
        Vec::new(),
    );
    let mut state_account = TestAccount::new(
        SolanaPubkey::new_from_array([0x88; 32]),
        program_id,
        false,
        true,
        vec![0; required_state_account_len(&config).unwrap()],
    );
    let mut owner_session = session_account(program_id, owner);

    let init_ix = encode_instruction(&OnchainInstruction::Initialize { config }).unwrap();
    {
        let init_accounts = vec![owner_account.account_info(), state_account.account_info()];
        process_instruction(&program_id, &init_accounts, &init_ix).unwrap();
    }

    let payload = PublicDecryptVerification {
        ct_handles: vec![
            crate::Handle::new([0x11; 32]),
            crate::Handle::new([0x22; 32]),
        ],
        decrypted_result: vec![1, 2, 3, 4],
        extra_data: vec![0x00],
    };
    let message =
        Secp256k1ProofVerifier::decryption_message(&payload, 54321, EvmAddress::new([30; 20]));
    let signature1 = sign_message(&signer1, &message);
    let signature2 = sign_message(&signer2, &message);

    let mut proof = Vec::new();
    proof.push(2);
    proof.extend_from_slice(&signature1);
    proof.extend_from_slice(&signature2);
    proof.extend_from_slice(&payload.extra_data);

    let mut clock = clock_account(81, 1_700_000_130);
    let mut slot_hashes = slot_hashes_account(80, 0xCD);
    let mut system_program = system_program_account();
    let mut rent = rent_account();
    let verify_ix = encode_instruction(&OnchainInstruction::Execute {
        instruction: HostInstruction::VerifyDecryptionSignatures {
            handles_list: payload.ct_handles.clone(),
            decrypted_result: payload.decrypted_result.clone(),
            decryption_proof: proof,
        },
        session_nonce: 1,
        recent_blockhash: [0xCD; 32],
    })
    .unwrap();
    {
        let verify_accounts = vec![
            owner_account.account_info(),
            state_account.account_info(),
            owner_session.account_info(),
            clock.account_info(),
            slot_hashes.account_info(),
            system_program.account_info(),
            rent.account_info(),
        ];
        process_instruction(&program_id, &verify_accounts, &verify_ix).unwrap();
    }

    let state = decode_state(&state_account.data).unwrap();
    assert_eq!(state.owner(), owner);
    assert_eq!(state.kms_verifier().get_threshold(), 1);
}

#[test]
fn execute_batch_rejects_oversized_instruction_batches() {
    let program_id = program_id();
    let owner = custom_pubkey(1);
    let config = host_program_config(owner);
    let mut owner_account = TestAccount::new(
        SolanaPubkey::from(owner),
        SolanaPubkey::default(),
        true,
        false,
        Vec::new(),
    );
    let mut state_account = TestAccount::new(
        SolanaPubkey::new_from_array([0x99; 32]),
        program_id,
        false,
        true,
        vec![0; required_state_account_len(&config).unwrap()],
    );
    let mut owner_session = session_account(program_id, owner);

    let init_ix = encode_instruction(&OnchainInstruction::Initialize { config }).unwrap();
    {
        let init_accounts = vec![owner_account.account_info(), state_account.account_info()];
        process_instruction(&program_id, &init_accounts, &init_ix).unwrap();
    }

    let instructions = (0..65)
        .map(|_| HostInstruction::FheRand {
            rand_type: FheType::Uint8,
            charge_hcu: false,
        })
        .collect();
    let mut clock = clock_account(90, 1_700_000_140);
    let mut slot_hashes = slot_hashes_account(89, 0xDD);
    let mut system_program = system_program_account();
    let mut rent = rent_account();
    let batch_ix = encode_instruction(&OnchainInstruction::ExecuteBatch {
        instructions,
        session_nonce: 1,
        recent_blockhash: [0xDD; 32],
    })
    .unwrap();
    let err = {
        let batch_accounts = vec![
            owner_account.account_info(),
            state_account.account_info(),
            owner_session.account_info(),
            clock.account_info(),
            slot_hashes.account_info(),
            system_program.account_info(),
            rent.account_info(),
        ];
        process_instruction(&program_id, &batch_accounts, &batch_ix).unwrap_err()
    };

    assert_eq!(err, ProgramError::Custom(10));
}

#[test]
fn execute_requires_slot_hashes_sysvar() {
    let program_id = program_id();
    let owner = custom_pubkey(1);
    let config = host_program_config(owner);
    let mut owner_account = TestAccount::new(
        SolanaPubkey::from(owner),
        SolanaPubkey::default(),
        true,
        false,
        Vec::new(),
    );
    let mut state_account = TestAccount::new(
        SolanaPubkey::new_from_array([0xAA; 32]),
        program_id,
        false,
        true,
        vec![0; required_state_account_len(&config).unwrap()],
    );
    let mut owner_session = session_account(program_id, owner);

    let init_ix = encode_instruction(&OnchainInstruction::Initialize { config }).unwrap();
    {
        let init_accounts = vec![owner_account.account_info(), state_account.account_info()];
        process_instruction(&program_id, &init_accounts, &init_ix).unwrap();
    }

    let mut clock = clock_account(77, 1_700_000_100);
    let mut wrong_slot_hashes = system_program_account();
    let mut system_program = system_program_account();
    let mut rent = rent_account();
    let execute_ix = encode_instruction(&OnchainInstruction::Execute {
        instruction: HostInstruction::FheRand {
            rand_type: FheType::Uint8,
            charge_hcu: false,
        },
        session_nonce: 1,
        recent_blockhash: [0xAB; 32],
    })
    .unwrap();

    let err = {
        let execute_accounts = vec![
            owner_account.account_info(),
            state_account.account_info(),
            owner_session.account_info(),
            clock.account_info(),
            wrong_slot_hashes.account_info(),
            system_program.account_info(),
            rent.account_info(),
        ];
        process_instruction(&program_id, &execute_accounts, &execute_ix).unwrap_err()
    };

    assert_eq!(err, ProgramError::Custom(16));
}

#[test]
fn execute_reuses_transient_session_across_calls_until_cleaned() {
    let program_id = program_id();
    let owner = custom_pubkey(1);
    let signer1 = signing_key(1);
    let signer2 = signing_key(2);
    let config = host_program_config_with_signers(
        owner,
        vec![evm_address(&signer1), evm_address(&signer2)],
        vec![evm(31), evm(32)],
    );
    let acl_program = config.acl_program;
    let host_chain_id = config.host_chain_id;
    let mut owner_account = TestAccount::new(
        SolanaPubkey::from(owner),
        SolanaPubkey::default(),
        true,
        false,
        Vec::new(),
    );
    let mut state_account = TestAccount::new(
        SolanaPubkey::new_from_array([0xAB; 32]),
        program_id,
        false,
        true,
        vec![0; required_state_account_len(&config).unwrap()],
    );
    let mut owner_session = session_account(program_id, owner);

    let init_ix = encode_instruction(&OnchainInstruction::Initialize { config }).unwrap();
    {
        let init_accounts = vec![owner_account.account_info(), state_account.account_info()];
        process_instruction(&program_id, &init_accounts, &init_ix).unwrap();
    }

    let handles =
        ExecutorState::compute_input_handles(b"ciphertext", &[8, 16], acl_program, host_chain_id)
            .unwrap();
    let context = ContextUserInputs {
        user_address: evm(4),
        contract_address: evm(5),
    };
    let extra_data = vec![0xAA, 0xBB];
    let payload = crate::CiphertextVerification {
        ct_handles: handles.clone(),
        user_address: context.user_address,
        contract_address: context.contract_address,
        contract_chain_id: host_chain_id,
        extra_data: extra_data.clone(),
    };
    let message = Secp256k1ProofVerifier::input_verification_message(
        &payload,
        54321,
        EvmAddress::new([20; 20]),
    );
    let signature1 = sign_message(&signer1, &message);
    let signature2 = sign_message(&signer2, &message);

    let mut proof = Vec::new();
    proof.push(handles.len() as u8);
    proof.push(2);
    for handle in &handles {
        proof.extend_from_slice(handle.as_bytes());
    }
    proof.extend_from_slice(&signature1);
    proof.extend_from_slice(&signature2);
    proof.extend_from_slice(&extra_data);

    let session_nonce = 9;

    let mut clock = clock_account(80, 1_700_000_120);
    let mut slot_hashes = slot_hashes_account(79, 0xBC);
    let mut system_program = system_program_account();
    let mut rent = rent_account();
    let verify_ix = encode_instruction(&OnchainInstruction::Execute {
        instruction: HostInstruction::VerifyInput {
            context,
            input_handle: handles[0],
            input_proof: proof,
        },
        session_nonce,
        recent_blockhash: [0xBC; 32],
    })
    .unwrap();
    {
        let verify_accounts = vec![
            owner_account.account_info(),
            state_account.account_info(),
            owner_session.account_info(),
            clock.account_info(),
            slot_hashes.account_info(),
            system_program.account_info(),
            rent.account_info(),
        ];
        process_instruction(&program_id, &verify_accounts, &verify_ix).unwrap();
    }

    let mut clock = clock_account(80, 1_700_000_120);
    let mut slot_hashes = slot_hashes_account(79, 0xBC);
    let mut system_program = system_program_account();
    let mut rent = rent_account();
    let binary_ix = encode_instruction(&OnchainInstruction::Execute {
        instruction: HostInstruction::BinaryOp {
            op: crate::Operator::FheAdd,
            lhs: handles[0],
            rhs: crate::BinaryOperand::Handle(handles[0]),
            result_type: FheType::Uint8,
            charge_hcu: false,
        },
        session_nonce,
        recent_blockhash: [0xBC; 32],
    })
    .unwrap();
    {
        let binary_accounts = vec![
            owner_account.account_info(),
            state_account.account_info(),
            owner_session.account_info(),
            clock.account_info(),
            slot_hashes.account_info(),
            system_program.account_info(),
            rent.account_info(),
        ];
        process_instruction(&program_id, &binary_accounts, &binary_ix).unwrap();
    }

    let mut clock = clock_account(80, 1_700_000_120);
    let mut slot_hashes = slot_hashes_account(79, 0xBC);
    let mut system_program = system_program_account();
    let mut rent = rent_account();
    let clean_ix = encode_instruction(&OnchainInstruction::Execute {
        instruction: HostInstruction::CleanTransientStorage,
        session_nonce,
        recent_blockhash: [0xBC; 32],
    })
    .unwrap();
    {
        let clean_accounts = vec![
            owner_account.account_info(),
            state_account.account_info(),
            owner_session.account_info(),
            clock.account_info(),
            slot_hashes.account_info(),
            system_program.account_info(),
            rent.account_info(),
        ];
        process_instruction(&program_id, &clean_accounts, &clean_ix).unwrap();
    }

    let mut clock = clock_account(80, 1_700_000_120);
    let mut slot_hashes = slot_hashes_account(79, 0xBC);
    let mut system_program = system_program_account();
    let mut rent = rent_account();
    let err = {
        let binary_accounts = vec![
            owner_account.account_info(),
            state_account.account_info(),
            owner_session.account_info(),
            clock.account_info(),
            slot_hashes.account_info(),
            system_program.account_info(),
            rent.account_info(),
        ];
        process_instruction(&program_id, &binary_accounts, &binary_ix).unwrap_err()
    };

    assert_eq!(err, ProgramError::Custom(1));
}
