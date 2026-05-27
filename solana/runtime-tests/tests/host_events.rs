// Test builders mirror Anchor instruction surfaces and LiteSVM result types.
#![allow(clippy::result_large_err, clippy::too_many_arguments)]

use std::path::PathBuf;

mod support;

use anchor_lang::{
    prelude::{system_instruction, system_program},
    AccountDeserialize, AccountSerialize, AnchorDeserialize, Discriminator, InstructionData,
    ToAccountMetas,
};
use anchor_litesvm::{AnchorLiteSVM, Program, TestHelpers};
use anchor_spl::{
    associated_token::{
        get_associated_token_address_with_program_id, spl_associated_token_account,
    },
    token::spl_token,
};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use confidential_token::{
    self as token, AmountDisclosedEvent, AmountDisclosureRequestedEvent, BalanceDisclosedEvent,
    BalanceDisclosureRequestedEvent, BalanceHandleUpdateReason, BalanceHandleUpdatedEvent,
    BurnRedeemedEvent, ConfidentialAmountKind, ConfidentialBurnEvent, ConfidentialTransferEvent,
    RandomAmountCreatedEvent, TotalSupplyHandleUpdatedEvent, TotalSupplyUpdateReason,
};
use confidential_token_receiver as receiver;
use litesvm::{
    types::{TransactionMetadata, TransactionResult},
    LiteSVM,
};
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_sdk::{
    account::Account,
    clock::Clock,
    ed25519_program,
    instruction::AccountMeta,
    instruction::Instruction,
    message::{Message, VersionedMessage},
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    slot_hashes::SlotHashes,
    sysvar,
    transaction::VersionedTransaction,
};
use zama_host as host;
use zama_host::{
    AclAllowedEvent, AclPermission, AclRecord, AclRecordBoundEvent, AclSubjectAllowedEvent,
    AclSubjectEntry, DenySubjectRecord, DenySubjectUpdatedEvent, FheBinaryOpCode, FheBinaryOpEvent,
    FheEvalArgs, FheEvalOp, FheEvalOperand, FheEvalOutput, FheRandBoundedEvent, FheRandEvent,
    FheTernaryOpCode, FheTernaryOpEvent, HandleMaterialCommitment, HandleMaterialCommittedEvent,
    HandleMaterialSealedEvent, HostConfig, HostConfigUpdatedEvent, PublicDecryptAllowedEvent,
    TransientCapabilityGrant, TransientSession, TrivialEncryptEvent, UserDecryptionDelegation,
    UserDecryptionDelegationUpdatedEvent,
};

use support::fhe_runtime::{ClearValue, CleartextBackend, FheBackend, TypedClearValue};

const DEFAULT_INPUT_NONCE_SEQUENCE: u64 = 0;
const TOKEN_COMPUTE_UNIT_LIMIT: u32 = 600_000;
const TOKEN_HEAP_FRAME_BYTES: u32 = 256 * 1024;

#[test]
fn test_emit_trivial_encrypt_emits_anchor_cpi_event() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let ix = anchor_ix(
        program_id,
        host::accounts::TestEmitProtocolEvent {
            test_authority: payer.pubkey(),
            host_config,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::TestEmitTrivialEncrypt {
            subject: payer.pubkey(),
            plaintext: [7; 32],
            fhe_type: 5,
            result: [8; 32],
        },
    );

    let message =
        Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &svm.latest_blockhash());
    let account_keys = message.account_keys.clone();
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), &[&payer]).unwrap();

    let meta = svm.send_transaction(tx).unwrap();
    let self_cpi = meta
        .inner_instructions
        .iter()
        .flatten()
        .find(|ix| *ix.instruction.program_id(&account_keys) == program_id)
        .expect("expected emit_cpi! self-CPI instruction");

    let event_prefix = anchor_event_prefix(TrivialEncryptEvent::DISCRIMINATOR);
    assert!(
        self_cpi.instruction.data.starts_with(&event_prefix),
        "self-CPI data did not start with Anchor event prefix"
    );
}

#[test]
fn test_emit_trivial_encrypt_requires_test_shim_gate() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config_with_flags(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
        true,
        false,
        false,
    );

    let ix = anchor_ix(
        program_id,
        host::accounts::TestEmitProtocolEvent {
            test_authority: payer.pubkey(),
            host_config,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::TestEmitTrivialEncrypt {
            subject: payer.pubkey(),
            plaintext: [7; 32],
            fhe_type: 5,
            result: [8; 32],
        },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
}

#[test]
fn fhe_rand_and_bind_creates_acl_record_and_emits_seeded_event() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("randomness");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let nonce_sequence = 7;
    let output_acl_record = acl_record_address(program_id, nonce_key, nonce_sequence);
    let expected_seed = current_rand_seed(&svm, nonce_key, nonce_sequence);
    let expected_handle = host::computed_rand_handle(expected_seed, 3, host::SOLANA_POC_CHAIN_ID);

    let ix = anchor_ix(
        program_id,
        host::accounts::FheRandAndBind {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheRandAndBind {
            fhe_type: 3,
            output_nonce_key: nonce_key,
            output_nonce_sequence: nonce_sequence,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(payer.pubkey())],
            output_public_decrypt: false,
        },
    );

    let (meta, account_keys) = send_with_meta(&mut svm, &payer, ix);

    let record = read_acl_record(&svm, output_acl_record).expect("expected output ACL");
    assert_eq!(record.handle, expected_handle);
    assert_eq!(record.nonce_key, nonce_key);
    assert_eq!(record.nonce_sequence, nonce_sequence);
    assert_eq!(record.acl_domain_key, acl_domain_key);
    assert_eq!(record.app_account, app_account);
    assert_eq!(record.encrypted_value_label, encrypted_value_label);
    assert_eq!(record_subjects(&record), vec![payer.pubkey()]);
    assert!(!record.public_decrypt);
    assert_eq!(record.created_slot, current_slot(&svm));

    let bound_events = acl_record_bound_events(&meta);
    assert_eq!(bound_events.len(), 1);
    assert_eq!(bound_events[0].acl_record, output_acl_record);
    assert_eq!(bound_events[0].handle, expected_handle);
    assert_eq!(bound_events[0].nonce_key, nonce_key);
    assert_eq!(bound_events[0].nonce_sequence, nonce_sequence);
    assert_eq!(bound_events[0].acl_domain_key, acl_domain_key);
    assert_eq!(bound_events[0].app_account, app_account);
    assert_eq!(bound_events[0].encrypted_value_label, encrypted_value_label);
    assert_eq!(bound_events[0].subject_count, 1);
    assert!(!bound_events[0].public_decrypt);
    assert_eq!(bound_events[0].created_slot, record.created_slot);

    let rand_events = fhe_rand_events(&meta, &account_keys, program_id);
    assert_eq!(rand_events.len(), 1);
    assert_eq!(rand_events[0].subject, payer.pubkey().to_bytes());
    assert_eq!(rand_events[0].seed, expected_seed);
    assert_eq!(rand_events[0].fhe_type, 3);
    assert_eq!(rand_events[0].result, expected_handle);

    let acl_events = acl_allowed_events(&meta, &account_keys, program_id);
    assert_eq!(acl_events.len(), 1);
    assert_eq!(acl_events[0].handle, expected_handle);
    assert_eq!(acl_events[0].subject, payer.pubkey().to_bytes());
}

#[test]
fn fhe_rand_and_bind_rejects_unsupported_rand_type() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("bad-rand-type");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let nonce_sequence = 10;
    let output_acl_record = acl_record_address(program_id, nonce_key, nonce_sequence);
    let unsupported_rand_type = 7;

    let ix = anchor_ix(
        program_id,
        host::accounts::FheRandAndBind {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheRandAndBind {
            fhe_type: unsupported_rand_type,
            output_nonce_key: nonce_key,
            output_nonce_sequence: nonce_sequence,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(payer.pubkey())],
            output_public_decrypt: false,
        },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    assert!(read_acl_record(&svm, output_acl_record).is_none());
}

#[test]
fn fhe_rand_bounded_and_bind_creates_acl_record_and_emits_bounded_event() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("bounded-randomness");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let nonce_sequence = 8;
    let output_acl_record = acl_record_address(program_id, nonce_key, nonce_sequence);
    let upper_bound = upper_bound_be(256);
    let expected_seed = current_rand_seed(&svm, nonce_key, nonce_sequence);
    let expected_handle = host::computed_rand_bounded_handle(
        upper_bound,
        expected_seed,
        3,
        host::SOLANA_POC_CHAIN_ID,
    );

    let ix = anchor_ix(
        program_id,
        host::accounts::FheRandBoundedAndBind {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheRandBoundedAndBind {
            upper_bound,
            fhe_type: 3,
            output_nonce_key: nonce_key,
            output_nonce_sequence: nonce_sequence,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(payer.pubkey())],
            output_public_decrypt: false,
        },
    );

    let (meta, account_keys) = send_with_meta(&mut svm, &payer, ix);

    let record = read_acl_record(&svm, output_acl_record).expect("expected output ACL");
    assert_eq!(record.handle, expected_handle);
    assert_eq!(record.nonce_key, nonce_key);
    assert_eq!(record.nonce_sequence, nonce_sequence);
    assert_eq!(record.acl_domain_key, acl_domain_key);
    assert_eq!(record.app_account, app_account);
    assert_eq!(record.encrypted_value_label, encrypted_value_label);
    assert_eq!(record_subjects(&record), vec![payer.pubkey()]);

    let rand_events = fhe_rand_bounded_events(&meta, &account_keys, program_id);
    assert_eq!(rand_events.len(), 1);
    assert_eq!(rand_events[0].subject, payer.pubkey().to_bytes());
    assert_eq!(rand_events[0].upper_bound, upper_bound);
    assert_eq!(rand_events[0].seed, expected_seed);
    assert_eq!(rand_events[0].fhe_type, 3);
    assert_eq!(rand_events[0].result, expected_handle);
}

#[test]
fn fhe_rand_bounded_and_bind_rejects_non_power_of_two_bound() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("bad-bounded-randomness");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let nonce_sequence = 9;
    let output_acl_record = acl_record_address(program_id, nonce_key, nonce_sequence);
    let ix = anchor_ix(
        program_id,
        host::accounts::FheRandBoundedAndBind {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheRandBoundedAndBind {
            upper_bound: upper_bound_be(3),
            fhe_type: 3,
            output_nonce_key: nonce_key,
            output_nonce_sequence: nonce_sequence,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(payer.pubkey())],
            output_public_decrypt: false,
        },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    assert!(read_acl_record(&svm, output_acl_record).is_none());
}

#[test]
fn mock_input_verified_and_bind_requires_gate_and_configured_verifier() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config_with_flags(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
        false,
        true,
        false,
    );
    let ix = mock_input_verified_ix(program_id, payer.pubkey(), payer.pubkey(), host_config, 0);

    assert!(try_send(&mut svm, &payer, ix).is_err());

    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let wrong_verifier = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config_with_flags(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
        true,
        true,
        false,
    );
    let ix = mock_input_verified_ix(
        program_id,
        payer.pubkey(),
        wrong_verifier.pubkey(),
        host_config,
        0,
    );

    assert!(send_with_signers(&mut svm, &payer.pubkey(), ix, &[&payer, &wrong_verifier]).is_err());
}

#[test]
fn mock_input_verified_and_bind_rejects_unsupported_handle_metadata() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config_with_flags(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
        true,
        true,
        false,
    );
    let input_handle = input_handle_for_chain_with_type(7, 1);
    let nonce_sequence = 0;
    let label = label("input-gate");
    let nonce_key = token::nonce_key(payer.pubkey(), payer.pubkey(), label);
    let acl_record = acl_record_address(program_id, nonce_key, nonce_sequence);
    let ix = mock_input_verified_ix_with_handle(
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        host_config,
        nonce_sequence,
        input_handle,
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    assert!(read_acl_record(&svm, acl_record).is_none());
}

#[test]
fn trivial_encrypt_and_bind_rejects_unsupported_output_type() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("bad-trivial-type");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_acl_record = acl_record_address(program_id, nonce_key, 0);
    let ix = anchor_ix(
        program_id,
        host::accounts::TrivialEncryptAndBind {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::TrivialEncryptAndBind {
            plaintext: amount_plaintext(7),
            fhe_type: 1,
            output_nonce_key: nonce_key,
            output_nonce_sequence: 0,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(payer.pubkey())],
            output_public_decrypt: false,
        },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    assert!(read_acl_record(&svm, output_acl_record).is_none());
}

#[test]
fn trivial_encrypt_and_bind_nonce_separates_equal_plaintexts() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("trivial-nonce");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let first_acl = acl_record_address(program_id, nonce_key, 0);
    let second_acl = acl_record_address(program_id, nonce_key, 1);
    let first_ix = trivial_encrypt_and_bind_ix(
        program_id,
        payer.pubkey(),
        host_config,
        first_acl,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        nonce_key,
        0,
        7,
    );
    let second_ix = trivial_encrypt_and_bind_ix(
        program_id,
        payer.pubkey(),
        host_config,
        second_acl,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        nonce_key,
        1,
        7,
    );

    let (meta, account_keys) = send_many_with_meta(&mut svm, &payer, vec![first_ix, second_ix]);
    let first = read_acl_record(&svm, first_acl).expect("expected first trivial ACL");
    let second = read_acl_record(&svm, second_acl).expect("expected second trivial ACL");
    assert_ne!(first.handle, second.handle);

    let events = trivial_encrypt_events(&meta, &account_keys, program_id);
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].plaintext, amount_plaintext(7));
    assert_eq!(events[0].result, first.handle);
    assert_eq!(events[1].plaintext, amount_plaintext(7));
    assert_eq!(events[1].result, second.handle);
    assert_ne!(events[0].result, events[1].result);
}

#[test]
fn trivial_encrypt_and_bind_rejects_public_decrypt_at_birth() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("trivial-public-birth");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let acl_record = acl_record_address(program_id, nonce_key, 0);
    let ix = trivial_encrypt_and_bind_ix_with_public_decrypt(
        program_id,
        payer.pubkey(),
        host_config,
        acl_record,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        nonce_key,
        0,
        7,
        true,
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    assert!(read_acl_record(&svm, acl_record).is_none());
}

#[test]
fn verify_input_and_bind_accepts_signed_proof_and_binds_selected_handle() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let verifier = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config_with_flags(
        &mut svm,
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        payer.pubkey(),
        false,
        true,
        false,
    );
    let input_handle = input_handle_for_chain_with_index(7, 1);
    let proof = host::SolanaInputProof {
        handles: vec![input_handle_for_chain(8), input_handle],
        handle_index: 1,
        user: payer.pubkey(),
        app_account: payer.pubkey(),
        acl_domain_key: payer.pubkey(),
        extra_data: b"transcipher-proof-context".to_vec(),
    };
    let encrypted_value_label = label("input-proof");
    let nonce_key = token::nonce_key(
        proof.acl_domain_key,
        proof.app_account,
        encrypted_value_label,
    );
    let acl_record = acl_record_address(program_id, nonce_key, 0);
    let output_subjects = vec![AclSubjectEntry::compute(payer.pubkey())];
    let bind_intent = input_bind_intent(
        nonce_key,
        0,
        payer.pubkey(),
        payer.pubkey(),
        encrypted_value_label,
        output_subjects.clone(),
        false,
    );
    let ed25519_ix = ed25519_verify_ix(
        &verifier,
        &host::input_proof_message(&proof, &bind_intent, program_id, host::SOLANA_POC_CHAIN_ID),
    );
    let bind_ix = verify_input_and_bind_ix(
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        host_config,
        acl_record,
        input_handle,
        proof,
        nonce_key,
        0,
        payer.pubkey(),
        payer.pubkey(),
        encrypted_value_label,
        output_subjects,
        false,
    );

    send_many_with_signers(
        &mut svm,
        &payer.pubkey(),
        vec![ed25519_ix, bind_ix],
        &[&payer],
    )
    .unwrap();

    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record.handle, input_handle);
    assert_eq!(record.acl_domain_key, payer.pubkey());
    assert_eq!(record.app_account, payer.pubkey());
    assert!(record.inline_subject_has_role(payer.pubkey(), host::ACL_ROLE_COMPUTE_SUBJECT));
}

#[test]
fn verify_input_and_bind_rejects_public_decrypt_at_birth() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let verifier = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config_with_flags(
        &mut svm,
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        payer.pubkey(),
        false,
        true,
        false,
    );
    let input_handle = input_handle_for_chain(7);
    let proof = host::SolanaInputProof {
        handles: vec![input_handle],
        handle_index: 0,
        user: payer.pubkey(),
        app_account: payer.pubkey(),
        acl_domain_key: payer.pubkey(),
        extra_data: b"public-birth-context".to_vec(),
    };
    let encrypted_value_label = label("input-public-birth");
    let nonce_key = token::nonce_key(payer.pubkey(), payer.pubkey(), encrypted_value_label);
    let acl_record = acl_record_address(program_id, nonce_key, 0);
    let output_subjects = vec![AclSubjectEntry::user(payer.pubkey())];
    let bind_intent = input_bind_intent(
        nonce_key,
        0,
        payer.pubkey(),
        payer.pubkey(),
        encrypted_value_label,
        output_subjects.clone(),
        true,
    );
    let ed25519_ix = ed25519_verify_ix(
        &verifier,
        &host::input_proof_message(&proof, &bind_intent, program_id, host::SOLANA_POC_CHAIN_ID),
    );
    let bind_ix = verify_input_and_bind_ix(
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        host_config,
        acl_record,
        input_handle,
        proof,
        nonce_key,
        0,
        payer.pubkey(),
        payer.pubkey(),
        encrypted_value_label,
        output_subjects,
        true,
    );

    assert!(send_many_with_signers(
        &mut svm,
        &payer.pubkey(),
        vec![ed25519_ix, bind_ix],
        &[&payer],
    )
    .is_err());
    assert!(read_acl_record(&svm, acl_record).is_none());
}

#[test]
fn verify_input_and_bind_requires_ed25519_preinstruction() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let verifier = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config_with_flags(
        &mut svm,
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        payer.pubkey(),
        false,
        true,
        false,
    );
    let input_handle = input_handle_for_chain(7);
    let encrypted_value_label = label("input-nosig");
    let nonce_key = token::nonce_key(payer.pubkey(), payer.pubkey(), encrypted_value_label);
    let acl_record = acl_record_address(program_id, nonce_key, 0);
    let proof = host::SolanaInputProof {
        handles: vec![input_handle],
        handle_index: 0,
        user: payer.pubkey(),
        app_account: payer.pubkey(),
        acl_domain_key: payer.pubkey(),
        extra_data: Vec::new(),
    };
    let bind_ix = verify_input_and_bind_ix(
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        host_config,
        acl_record,
        input_handle,
        proof,
        nonce_key,
        0,
        payer.pubkey(),
        payer.pubkey(),
        encrypted_value_label,
        vec![AclSubjectEntry::compute(payer.pubkey())],
        false,
    );

    assert!(try_send(&mut svm, &payer, bind_ix).is_err());
    assert!(read_acl_record(&svm, acl_record).is_none());
}

#[test]
fn verify_input_and_bind_rejects_mismatched_selected_handle() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let verifier = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config_with_flags(
        &mut svm,
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        payer.pubkey(),
        false,
        true,
        false,
    );
    let input_handle = input_handle_for_chain(7);
    let proof = host::SolanaInputProof {
        handles: vec![input_handle_for_chain(8)],
        handle_index: 0,
        user: payer.pubkey(),
        app_account: payer.pubkey(),
        acl_domain_key: payer.pubkey(),
        extra_data: Vec::new(),
    };
    let encrypted_value_label = label("input-mismatch");
    let nonce_key = token::nonce_key(
        proof.acl_domain_key,
        proof.app_account,
        encrypted_value_label,
    );
    let acl_record = acl_record_address(program_id, nonce_key, 0);
    let output_subjects = vec![AclSubjectEntry::compute(payer.pubkey())];
    let bind_intent = input_bind_intent(
        nonce_key,
        0,
        payer.pubkey(),
        payer.pubkey(),
        encrypted_value_label,
        output_subjects.clone(),
        false,
    );
    let ed25519_ix = ed25519_verify_ix(
        &verifier,
        &host::input_proof_message(&proof, &bind_intent, program_id, host::SOLANA_POC_CHAIN_ID),
    );
    let bind_ix = verify_input_and_bind_ix(
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        host_config,
        acl_record,
        input_handle,
        proof,
        nonce_key,
        0,
        payer.pubkey(),
        payer.pubkey(),
        encrypted_value_label,
        output_subjects,
        false,
    );

    assert!(send_many_with_signers(
        &mut svm,
        &payer.pubkey(),
        vec![ed25519_ix, bind_ix],
        &[&payer],
    )
    .is_err());
    assert!(read_acl_record(&svm, acl_record).is_none());
}

#[test]
fn verify_input_and_bind_rejects_signature_replayed_with_different_acl_policy() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let verifier = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config_with_flags(
        &mut svm,
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        payer.pubkey(),
        false,
        true,
        false,
    );
    let input_handle = input_handle_for_chain(7);
    let proof = host::SolanaInputProof {
        handles: vec![input_handle],
        handle_index: 0,
        user: payer.pubkey(),
        app_account: payer.pubkey(),
        acl_domain_key: payer.pubkey(),
        extra_data: b"replay-context".to_vec(),
    };
    let signed_label = label("input-signed");
    let signed_nonce_key = token::nonce_key(payer.pubkey(), payer.pubkey(), signed_label);
    let signed_subjects = vec![AclSubjectEntry::compute(payer.pubkey())];
    let signed_intent = input_bind_intent(
        signed_nonce_key,
        0,
        payer.pubkey(),
        payer.pubkey(),
        signed_label,
        signed_subjects,
        false,
    );
    let ed25519_ix = ed25519_verify_ix(
        &verifier,
        &host::input_proof_message(
            &proof,
            &signed_intent,
            program_id,
            host::SOLANA_POC_CHAIN_ID,
        ),
    );

    let replay_label = label("input-replay");
    let replay_nonce_key = token::nonce_key(payer.pubkey(), payer.pubkey(), replay_label);
    let replay_acl_record = acl_record_address(program_id, replay_nonce_key, 0);
    let bind_ix = verify_input_and_bind_ix(
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        host_config,
        replay_acl_record,
        input_handle,
        proof,
        replay_nonce_key,
        0,
        payer.pubkey(),
        payer.pubkey(),
        replay_label,
        vec![AclSubjectEntry::compute(payer.pubkey())],
        true,
    );

    assert!(send_many_with_signers(
        &mut svm,
        &payer.pubkey(),
        vec![ed25519_ix, bind_ix],
        &[&payer],
    )
    .is_err());
    assert!(read_acl_record(&svm, replay_acl_record).is_none());
}

#[test]
fn verify_input_and_bind_rejects_noncanonical_input_handle_index() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let verifier = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config_with_flags(
        &mut svm,
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        payer.pubkey(),
        false,
        true,
        false,
    );
    let input_handle = input_handle_for_chain_with_index(7, 2);
    let proof = host::SolanaInputProof {
        handles: vec![input_handle],
        handle_index: 0,
        user: payer.pubkey(),
        app_account: payer.pubkey(),
        acl_domain_key: payer.pubkey(),
        extra_data: Vec::new(),
    };
    let encrypted_value_label = label("input-index");
    let nonce_key = token::nonce_key(payer.pubkey(), payer.pubkey(), encrypted_value_label);
    let acl_record = acl_record_address(program_id, nonce_key, 0);
    let output_subjects = vec![AclSubjectEntry::compute(payer.pubkey())];
    let bind_intent = input_bind_intent(
        nonce_key,
        0,
        payer.pubkey(),
        payer.pubkey(),
        encrypted_value_label,
        output_subjects.clone(),
        false,
    );
    let ed25519_ix = ed25519_verify_ix(
        &verifier,
        &host::input_proof_message(&proof, &bind_intent, program_id, host::SOLANA_POC_CHAIN_ID),
    );
    let bind_ix = verify_input_and_bind_ix(
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        host_config,
        acl_record,
        input_handle,
        proof,
        nonce_key,
        0,
        payer.pubkey(),
        payer.pubkey(),
        encrypted_value_label,
        output_subjects,
        false,
    );

    assert!(send_many_with_signers(
        &mut svm,
        &payer.pubkey(),
        vec![ed25519_ix, bind_ix],
        &[&payer],
    )
    .is_err());
    assert!(read_acl_record(&svm, acl_record).is_none());
}

#[test]
fn verify_input_and_bind_rejects_wrong_ed25519_signer() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let verifier = svm.create_funded_account(1_000_000_000).unwrap();
    let wrong_verifier = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config_with_flags(
        &mut svm,
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        payer.pubkey(),
        false,
        true,
        false,
    );
    let input_handle = input_handle_for_chain(7);
    let proof = host::SolanaInputProof {
        handles: vec![input_handle],
        handle_index: 0,
        user: payer.pubkey(),
        app_account: payer.pubkey(),
        acl_domain_key: payer.pubkey(),
        extra_data: Vec::new(),
    };
    let encrypted_value_label = label("input-signer");
    let nonce_key = token::nonce_key(payer.pubkey(), payer.pubkey(), encrypted_value_label);
    let acl_record = acl_record_address(program_id, nonce_key, 0);
    let output_subjects = vec![AclSubjectEntry::compute(payer.pubkey())];
    let bind_intent = input_bind_intent(
        nonce_key,
        0,
        payer.pubkey(),
        payer.pubkey(),
        encrypted_value_label,
        output_subjects.clone(),
        false,
    );
    let ed25519_ix = ed25519_verify_ix(
        &wrong_verifier,
        &host::input_proof_message(&proof, &bind_intent, program_id, host::SOLANA_POC_CHAIN_ID),
    );
    let bind_ix = verify_input_and_bind_ix(
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        host_config,
        acl_record,
        input_handle,
        proof,
        nonce_key,
        0,
        payer.pubkey(),
        payer.pubkey(),
        encrypted_value_label,
        output_subjects,
        false,
    );

    assert!(send_many_with_signers(
        &mut svm,
        &payer.pubkey(),
        vec![ed25519_ix, bind_ix],
        &[&payer],
    )
    .is_err());
    assert!(read_acl_record(&svm, acl_record).is_none());
}

#[test]
fn verify_input_and_bind_rejects_computed_marker_input_handle() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let verifier = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config_with_flags(
        &mut svm,
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        payer.pubkey(),
        false,
        true,
        false,
    );
    let mut input_handle = input_handle_for_chain(7);
    input_handle[21] = 0xff;
    let proof = host::SolanaInputProof {
        handles: vec![input_handle],
        handle_index: 0,
        user: payer.pubkey(),
        app_account: payer.pubkey(),
        acl_domain_key: payer.pubkey(),
        extra_data: Vec::new(),
    };
    let encrypted_value_label = label("input-computed");
    let nonce_key = token::nonce_key(payer.pubkey(), payer.pubkey(), encrypted_value_label);
    let acl_record = acl_record_address(program_id, nonce_key, 0);
    let output_subjects = vec![AclSubjectEntry::compute(payer.pubkey())];
    let bind_intent = input_bind_intent(
        nonce_key,
        0,
        payer.pubkey(),
        payer.pubkey(),
        encrypted_value_label,
        output_subjects.clone(),
        false,
    );
    let ed25519_ix = ed25519_verify_ix(
        &verifier,
        &host::input_proof_message(&proof, &bind_intent, program_id, host::SOLANA_POC_CHAIN_ID),
    );
    let bind_ix = verify_input_and_bind_ix(
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        host_config,
        acl_record,
        input_handle,
        proof,
        nonce_key,
        0,
        payer.pubkey(),
        payer.pubkey(),
        encrypted_value_label,
        output_subjects,
        false,
    );

    assert!(send_many_with_signers(
        &mut svm,
        &payer.pubkey(),
        vec![ed25519_ix, bind_ix],
        &[&payer],
    )
    .is_err());
    assert!(read_acl_record(&svm, acl_record).is_none());
}

#[test]
fn verify_input_and_bind_rejects_unsupported_input_handle_type() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let verifier = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config_with_flags(
        &mut svm,
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        payer.pubkey(),
        false,
        true,
        false,
    );
    let input_handle = input_handle_for_chain_with_type(7, 1);
    let proof = host::SolanaInputProof {
        handles: vec![input_handle],
        handle_index: 0,
        user: payer.pubkey(),
        app_account: payer.pubkey(),
        acl_domain_key: payer.pubkey(),
        extra_data: Vec::new(),
    };
    let encrypted_value_label = label("input-type");
    let nonce_key = token::nonce_key(payer.pubkey(), payer.pubkey(), encrypted_value_label);
    let acl_record = acl_record_address(program_id, nonce_key, 0);
    let output_subjects = vec![AclSubjectEntry::compute(payer.pubkey())];
    let bind_intent = input_bind_intent(
        nonce_key,
        0,
        payer.pubkey(),
        payer.pubkey(),
        encrypted_value_label,
        output_subjects.clone(),
        false,
    );
    let ed25519_ix = ed25519_verify_ix(
        &verifier,
        &host::input_proof_message(&proof, &bind_intent, program_id, host::SOLANA_POC_CHAIN_ID),
    );
    let bind_ix = verify_input_and_bind_ix(
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        host_config,
        acl_record,
        input_handle,
        proof,
        nonce_key,
        0,
        payer.pubkey(),
        payer.pubkey(),
        encrypted_value_label,
        output_subjects,
        false,
    );

    assert!(send_many_with_signers(
        &mut svm,
        &payer.pubkey(),
        vec![ed25519_ix, bind_ix],
        &[&payer],
    )
    .is_err());
    assert!(read_acl_record(&svm, acl_record).is_none());
}

#[test]
fn delegation_rejects_zero_delegate_or_app_context() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let delegate = Pubkey::new_unique();
    let app_account = Pubkey::new_unique();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let current_slot = svm.get_sysvar::<Clock>().slot;

    for (bad_delegate, bad_app_account) in [
        (Pubkey::default(), app_account),
        (delegate, Pubkey::default()),
    ] {
        let delegation_record =
            host::user_decryption_delegation_address(payer.pubkey(), bad_delegate, bad_app_account)
                .0;
        let grant = delegate_for_user_decryption_ix(
            program_id,
            payer.pubkey(),
            host_config,
            delegation_record,
            bad_delegate,
            bad_app_account,
            current_slot + 100,
        );
        assert!(try_send(&mut svm, &payer, grant).is_err());
        assert!(read_delegation_record(&svm, delegation_record).is_none());
    }
}

#[test]
fn delegation_rejects_wildcard_delegate_sentinel() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let delegate = Pubkey::new_from_array([0xff; 32]);
    let app_account = Pubkey::new_unique();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let current_slot = svm.get_sysvar::<Clock>().slot;
    let delegation_record =
        host::user_decryption_delegation_address(payer.pubkey(), delegate, app_account).0;
    let grant = delegate_for_user_decryption_ix(
        program_id,
        payer.pubkey(),
        host_config,
        delegation_record,
        delegate,
        app_account,
        current_slot + 100,
    );

    assert!(try_send(&mut svm, &payer, grant).is_err());
    assert!(read_delegation_record(&svm, delegation_record).is_none());
}

#[test]
fn delegation_counter_tracks_regrant_and_revoke() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let delegate = Pubkey::new_unique();
    let app_account = Pubkey::new_unique();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let delegation_record =
        host::user_decryption_delegation_address(payer.pubkey(), delegate, app_account).0;
    let current_slot = svm.get_sysvar::<Clock>().slot;
    let first_expiration = current_slot + 100;

    let first_grant = delegate_for_user_decryption_ix(
        program_id,
        payer.pubkey(),
        host_config,
        delegation_record,
        delegate,
        app_account,
        first_expiration,
    );
    let (first_meta, _) = send_with_meta(&mut svm, &payer, first_grant);
    let first_events = delegation_updated_events(&first_meta);
    assert_eq!(first_events.len(), 1);
    assert_eq!(first_events[0].delegator, payer.pubkey());
    assert_eq!(first_events[0].delegate, delegate);
    assert_eq!(first_events[0].app_account, app_account);
    assert_eq!(first_events[0].delegation_counter, 1);
    assert_eq!(first_events[0].old_expiration_slot, 0);
    assert_eq!(first_events[0].new_expiration_slot, first_expiration);
    assert!(!first_events[0].revoked);
    let first = read_delegation_record(&svm, delegation_record).expect("expected delegation");
    assert_eq!(first.delegator, payer.pubkey());
    assert_eq!(first.delegate, delegate);
    assert_eq!(first.app_account, app_account);
    assert_eq!(first.expiration_slot, first_expiration);
    assert_eq!(first.delegation_counter, 1);
    assert!(!first.revoked);

    let unchanged_grant = delegate_for_user_decryption_ix(
        program_id,
        payer.pubkey(),
        host_config,
        delegation_record,
        delegate,
        app_account,
        first_expiration,
    );
    assert!(try_send(&mut svm, &payer, unchanged_grant).is_err());

    svm.warp_to_slot(first.last_update_slot + 1);
    let second_expiration = first_expiration + 10;
    let second_grant = delegate_for_user_decryption_ix(
        program_id,
        payer.pubkey(),
        host_config,
        delegation_record,
        delegate,
        app_account,
        second_expiration,
    );
    let (second_meta, _) = send_with_meta(&mut svm, &payer, second_grant);
    let second_events = delegation_updated_events(&second_meta);
    assert_eq!(second_events.len(), 1);
    assert_eq!(second_events[0].delegation_counter, 2);
    assert_eq!(second_events[0].old_expiration_slot, first_expiration);
    assert_eq!(second_events[0].new_expiration_slot, second_expiration);
    assert!(!second_events[0].revoked);
    let second = read_delegation_record(&svm, delegation_record).expect("expected delegation");
    assert_eq!(second.expiration_slot, second_expiration);
    assert_eq!(second.delegation_counter, 2);
    assert!(second.last_update_slot > first.last_update_slot);
    assert!(!second.revoked);

    svm.warp_to_slot(second.last_update_slot + 1);
    let revoke = anchor_ix(
        program_id,
        host::accounts::RevokeDelegationForUserDecryption {
            delegator: payer.pubkey(),
            host_config,
            delegation_record,
        },
        host::instruction::RevokeDelegationForUserDecryption {},
    );
    let (revoke_meta, _) = send_with_meta(&mut svm, &payer, revoke);
    let revoke_events = delegation_updated_events(&revoke_meta);
    assert_eq!(revoke_events.len(), 1);
    assert_eq!(revoke_events[0].delegation_counter, 3);
    assert_eq!(revoke_events[0].old_expiration_slot, second_expiration);
    assert_eq!(revoke_events[0].new_expiration_slot, 0);
    assert!(revoke_events[0].revoked);
    let revoked = read_delegation_record(&svm, delegation_record).expect("expected delegation");
    assert_eq!(revoked.expiration_slot, 0);
    assert_eq!(revoked.delegation_counter, 3);
    assert!(revoked.last_update_slot > second.last_update_slot);
    assert!(revoked.revoked);

    svm.warp_to_slot(revoked.last_update_slot + 1);
    let third_expiration = second_expiration + 10;
    let third_grant = delegate_for_user_decryption_ix(
        program_id,
        payer.pubkey(),
        host_config,
        delegation_record,
        delegate,
        app_account,
        third_expiration,
    );
    let (third_meta, _) = send_with_meta(&mut svm, &payer, third_grant);
    let third_events = delegation_updated_events(&third_meta);
    assert_eq!(third_events.len(), 1);
    assert_eq!(third_events[0].delegation_counter, 4);
    assert_eq!(third_events[0].old_expiration_slot, 0);
    assert_eq!(third_events[0].new_expiration_slot, third_expiration);
    assert!(!third_events[0].revoked);
    let regranted = read_delegation_record(&svm, delegation_record).expect("expected delegation");
    assert_eq!(regranted.expiration_slot, third_expiration);
    assert_eq!(regranted.delegation_counter, 4);
    assert!(regranted.last_update_slot > revoked.last_update_slot);
    assert!(!regranted.revoked);
}

#[test]
fn delegation_rejects_same_slot_double_update() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let delegate = Pubkey::new_unique();
    let app_account = Pubkey::new_unique();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let delegation_record =
        host::user_decryption_delegation_address(payer.pubkey(), delegate, app_account).0;
    let current_slot = svm.get_sysvar::<Clock>().slot;
    let first_grant = delegate_for_user_decryption_ix(
        program_id,
        payer.pubkey(),
        host_config,
        delegation_record,
        delegate,
        app_account,
        current_slot + 100,
    );
    let second_grant = delegate_for_user_decryption_ix(
        program_id,
        payer.pubkey(),
        host_config,
        delegation_record,
        delegate,
        app_account,
        current_slot + 101,
    );

    assert!(try_send_many(&mut svm, &payer, vec![first_grant, second_grant]).is_err());
    assert!(read_delegation_record(&svm, delegation_record).is_none());
}

#[test]
fn delegation_rejects_existing_record_with_wrong_stored_bump() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let delegate = Pubkey::new_unique();
    let app_account = Pubkey::new_unique();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    svm.warp_to_slot(10);
    let current_slot = svm.get_sysvar::<Clock>().slot;
    let (delegation_record, bump) =
        host::user_decryption_delegation_address(payer.pubkey(), delegate, app_account);
    let wrong_bump = bump.wrapping_add(1);
    let data = serialized_account(UserDecryptionDelegation {
        delegator: payer.pubkey(),
        delegate,
        app_account,
        expiration_slot: current_slot + 100,
        delegation_counter: 1,
        last_update_slot: current_slot - 1,
        revoked: false,
        bump: wrong_bump,
    });
    svm.set_account(
        delegation_record,
        Account {
            lamports: svm.minimum_balance_for_rent_exemption(data.len()),
            data,
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();

    let regrant = delegate_for_user_decryption_ix(
        program_id,
        payer.pubkey(),
        host_config,
        delegation_record,
        delegate,
        app_account,
        current_slot + 101,
    );
    assert!(try_send(&mut svm, &payer, regrant).is_err());
    let stored = read_delegation_record(&svm, delegation_record).expect("expected delegation");
    assert_eq!(stored.delegation_counter, 1);
    assert_eq!(stored.bump, wrong_bump);
    assert!(!stored.revoked);

    let revoke = anchor_ix(
        program_id,
        host::accounts::RevokeDelegationForUserDecryption {
            delegator: payer.pubkey(),
            host_config,
            delegation_record,
        },
        host::instruction::RevokeDelegationForUserDecryption {},
    );
    assert!(try_send(&mut svm, &payer, revoke).is_err());
    let stored = read_delegation_record(&svm, delegation_record).expect("expected delegation");
    assert_eq!(stored.delegation_counter, 1);
    assert_eq!(stored.bump, wrong_bump);
    assert!(!stored.revoked);
}

#[test]
fn allow_acl_subjects_extends_existing_canonical_record() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let nonce_sequence = 42;
    let new_subject = Pubkey::new_unique();
    let handle = [9; 32];
    let acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        payer.pubkey(),
    );

    let ix = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry::user(new_subject)],
        },
    );

    let (meta, _) = send_with_meta(&mut svm, &payer, ix);
    let events = acl_subject_allowed_events(&meta);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].acl_record, acl_record);
    assert_eq!(events[0].handle, handle);
    assert_eq!(events[0].authority_subject, payer.pubkey());
    assert_eq!(events[0].subject, new_subject.to_bytes());
    assert_eq!(events[0].role_flags, host::ACL_ROLE_USER);
    assert_eq!(events[0].overflow_permission_record, Pubkey::default());
    assert_eq!(events[0].inline_index, 1);
    assert_eq!(events[0].updated_slot, current_slot(&svm));

    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record.handle, handle);
    assert_eq!(record.nonce_key, nonce_key);
    assert_eq!(record.nonce_sequence, nonce_sequence);
    assert_eq!(record.acl_domain_key, acl_domain_key);
    assert_eq!(record.app_account, app_account);
    assert_eq!(record.encrypted_value_label, encrypted_value_label);
    assert_eq!(record_subjects(&record), vec![payer.pubkey(), new_subject]);
    assert!(!record.public_decrypt);

    let assert_ix = anchor_ix(
        program_id,
        host::accounts::AssertAclRecord {
            acl_record,
            subject_permission_record: None,
        },
        host::instruction::AssertAclRecord {
            nonce_key,
            nonce_sequence,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            handle,
            subject: new_subject,
        },
    );
    send(&mut svm, &payer, assert_ix);
}

#[test]
fn allow_acl_subjects_is_idempotent_for_existing_subject() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let nonce_sequence = 42;
    let handle = [9; 32];
    let acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        payer.pubkey(),
    );

    let ix = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry::user(payer.pubkey())],
        },
    );

    let (meta, _) = send_with_meta(&mut svm, &payer, ix);
    let events = acl_subject_allowed_events(&meta);
    assert!(events.is_empty());

    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record_subjects(&record), vec![payer.pubkey()]);
}

#[test]
fn allow_acl_subjects_rejects_authority_permission_for_inline_authority() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("extra-authority");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [9; 32];
    let acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        payer.pubkey(),
    );
    let (authority_permission_record, bump) =
        host::acl_permission_address(acl_record, payer.pubkey());
    seed_acl_permission(
        &mut svm,
        program_id,
        authority_permission_record,
        AclPermission {
            acl_record,
            subject: payer.pubkey(),
            role_flags: host::ACL_ROLE_GRANT,
            bump,
        },
        0,
    );

    let new_subject = Pubkey::new_unique();
    let ix = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            authority_permission_record: Some(authority_permission_record),
            acl_record,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry::use_only(new_subject)],
        },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record_subjects(&record), vec![payer.pubkey()]);
    assert!(!record.inline_subject_has_role(new_subject, host::ACL_ROLE_USE));
}

#[test]
fn set_deny_subject_skips_idempotent_default_and_repeated_updates() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let subject = Pubkey::new_unique();
    let deny_subject_record = host::deny_subject_address(subject).0;

    let set_false = set_deny_subject_ix(
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        host_config,
        deny_subject_record,
        subject,
        false,
    );
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(200_001),
            set_false,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 0);
    assert!(read_deny_subject_record(&svm, deny_subject_record).is_none());

    let set_true = set_deny_subject_ix(
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        host_config,
        deny_subject_record,
        subject,
        true,
    );
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(200_002),
            set_true,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 1);
    let events = deny_subject_updated_events(&meta);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].deny_subject_record, deny_subject_record);
    assert_eq!(events[0].subject, subject);
    assert!(events[0].denied);
    assert_eq!(events[0].updated_slot, current_slot(&svm));
    assert!(
        read_deny_subject_record(&svm, deny_subject_record)
            .expect("expected deny record")
            .denied
    );

    let repeat_true = set_deny_subject_ix(
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        host_config,
        deny_subject_record,
        subject,
        true,
    );
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(200_003),
            repeat_true,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 0);
    assert!(
        read_deny_subject_record(&svm, deny_subject_record)
            .expect("expected deny record")
            .denied
    );

    let clear = set_deny_subject_ix(
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        host_config,
        deny_subject_record,
        subject,
        false,
    );
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(200_004),
            clear,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 1);
    let events = deny_subject_updated_events(&meta);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].deny_subject_record, deny_subject_record);
    assert_eq!(events[0].subject, subject);
    assert!(!events[0].denied);
    assert_eq!(events[0].updated_slot, current_slot(&svm));
    assert!(
        !read_deny_subject_record(&svm, deny_subject_record)
            .expect("expected deny record")
            .denied
    );

    let repeat_clear = set_deny_subject_ix(
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        host_config,
        deny_subject_record,
        subject,
        false,
    );
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(200_005),
            repeat_clear,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 0);
    assert!(
        !read_deny_subject_record(&svm, deny_subject_record)
            .expect("expected deny record")
            .denied
    );
}

#[test]
fn grant_deny_list_rejects_noncanonical_account_length() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config_with_flags(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
        true,
        true,
        true,
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("deny-exact");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [9; 32];
    let acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        payer.pubkey(),
    );
    let (deny_subject_record, bump) = host::deny_subject_address(payer.pubkey());
    seed_deny_subject_record(
        &mut svm,
        program_id,
        deny_subject_record,
        DenySubjectRecord {
            subject: payer.pubkey(),
            denied: false,
            bump,
        },
        1,
    );

    let grant = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: Some(deny_subject_record),
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry::user(Pubkey::new_unique())],
        },
    );
    assert!(try_send_many(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(200_101),
            grant,
        ],
    )
    .is_err());
    assert_eq!(
        record_subjects(&read_acl_record(&svm, acl_record).expect("expected ACL record")),
        vec![payer.pubkey()]
    );
    assert_eq!(
        svm.get_account(&deny_subject_record)
            .expect("expected deny record")
            .data
            .len(),
        8 + DenySubjectRecord::SPACE + 1
    );
}

#[test]
fn set_host_pause_skips_idempotent_repeated_status_updates() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let set_false = set_host_pause_ix(program_id, payer.pubkey(), host_config, false);
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(201_001),
            set_false,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 0);
    let initial_config = read_host_config(&svm, host_config);
    assert!(!initial_config.paused);

    let set_true = set_host_pause_ix(program_id, payer.pubkey(), host_config, true);
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(201_002),
            set_true,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 1);
    let config = read_host_config(&svm, host_config);
    assert!(config.paused);
    assert_eq!(config.updated_slot, current_slot(&svm));
    assert_host_config_updated_event(&meta, host_config, payer.pubkey(), &config);
    let paused_updated_slot = config.updated_slot;

    let repeat_true = set_host_pause_ix(program_id, payer.pubkey(), host_config, true);
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(201_003),
            repeat_true,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 0);
    let config = read_host_config(&svm, host_config);
    assert!(config.paused);
    assert_eq!(config.updated_slot, paused_updated_slot);

    let clear = set_host_pause_ix(program_id, payer.pubkey(), host_config, false);
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(201_004),
            clear,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 1);
    let config = read_host_config(&svm, host_config);
    assert!(!config.paused);
    assert_eq!(config.updated_slot, current_slot(&svm));
    assert_host_config_updated_event(&meta, host_config, payer.pubkey(), &config);
    let clear_updated_slot = config.updated_slot;

    let repeat_clear = set_host_pause_ix(program_id, payer.pubkey(), host_config, false);
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(201_005),
            repeat_clear,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 0);
    let config = read_host_config(&svm, host_config);
    assert!(!config.paused);
    assert_eq!(config.updated_slot, clear_updated_slot);
}

#[test]
fn initialize_host_config_rejects_zero_profile_fields() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let (host_config, _) = Pubkey::find_program_address(&[host::HOST_CONFIG_SEED], &program_id);

    let cases: [fn(&mut host::InitializeHostConfigArgs); 4] = [
        |args| args.chain_id = 0,
        |args| args.input_verifier_authority = Pubkey::default(),
        |args| args.material_authority = Pubkey::default(),
        |args| args.test_authority = Pubkey::default(),
    ];

    for mutate in cases {
        let mut args = host::InitializeHostConfigArgs {
            chain_id: host::SOLANA_POC_CHAIN_ID,
            input_verifier_authority: payer.pubkey(),
            material_authority: payer.pubkey(),
            test_authority: payer.pubkey(),
            mock_input_enabled: true,
            test_shims_enabled: true,
            grant_deny_list_enabled: false,
        };
        mutate(&mut args);

        let ix = anchor_ix(
            program_id,
            host::accounts::InitializeHostConfig {
                payer: payer.pubkey(),
                admin: payer.pubkey(),
                host_config,
                system_program: system_program::ID,
            },
            host::instruction::InitializeHostConfig { args },
        );

        assert!(try_send(&mut svm, &payer, ix).is_err());
        assert!(svm.get_account(&host_config).is_none());
    }
}

#[test]
fn host_config_flag_setters_skip_idempotent_repeated_updates() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let repeat_mock_true = set_mock_input_enabled_ix(program_id, payer.pubkey(), host_config, true);
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(201_201),
            repeat_mock_true,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 0);
    assert!(read_host_config(&svm, host_config).mock_input_enabled);

    let set_mock_false = set_mock_input_enabled_ix(program_id, payer.pubkey(), host_config, false);
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(201_202),
            set_mock_false,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 1);
    let config = read_host_config(&svm, host_config);
    assert!(!config.mock_input_enabled);
    assert_host_config_updated_event(&meta, host_config, payer.pubkey(), &config);

    let repeat_mock_false =
        set_mock_input_enabled_ix(program_id, payer.pubkey(), host_config, false);
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(201_203),
            repeat_mock_false,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 0);
    assert!(!read_host_config(&svm, host_config).mock_input_enabled);

    let repeat_shims_true =
        set_test_shims_enabled_ix(program_id, payer.pubkey(), host_config, true);
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(201_204),
            repeat_shims_true,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 0);
    assert!(read_host_config(&svm, host_config).test_shims_enabled);

    let set_shims_false = set_test_shims_enabled_ix(program_id, payer.pubkey(), host_config, false);
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(201_205),
            set_shims_false,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 1);
    let config = read_host_config(&svm, host_config);
    assert!(!config.test_shims_enabled);
    assert_host_config_updated_event(&meta, host_config, payer.pubkey(), &config);

    let repeat_shims_false =
        set_test_shims_enabled_ix(program_id, payer.pubkey(), host_config, false);
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(201_206),
            repeat_shims_false,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 0);
    assert!(!read_host_config(&svm, host_config).test_shims_enabled);

    let repeat_deny_false =
        set_grant_deny_list_enabled_ix(program_id, payer.pubkey(), host_config, false);
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(201_207),
            repeat_deny_false,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 0);
    assert!(!read_host_config(&svm, host_config).grant_deny_list_enabled);

    let set_deny_true =
        set_grant_deny_list_enabled_ix(program_id, payer.pubkey(), host_config, true);
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(201_208),
            set_deny_true,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 1);
    let config = read_host_config(&svm, host_config);
    assert!(config.grant_deny_list_enabled);
    assert_host_config_updated_event(&meta, host_config, payer.pubkey(), &config);

    let repeat_deny_true =
        set_grant_deny_list_enabled_ix(program_id, payer.pubkey(), host_config, true);
    let (meta, _) = send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(201_209),
            repeat_deny_true,
        ],
    );
    assert_eq!(program_data_log_count(&meta), 0);
    assert!(read_host_config(&svm, host_config).grant_deny_list_enabled);
}

#[test]
fn host_config_rejects_oversized_account() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    extend_host_config(&mut svm, host_config, 1);

    let set_true = set_host_pause_ix(program_id, payer.pubkey(), host_config, true);
    assert!(try_send_many(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(201_101),
            set_true,
        ],
    )
    .is_err());
    assert!(!read_host_config(&svm, host_config).paused);

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("config-exact");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [9; 32];
    let new_subject = Pubkey::new_unique();
    let acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        42,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        payer.pubkey(),
    );
    let grant = allow_acl_subjects_ix(
        program_id,
        payer.pubkey(),
        host_config,
        acl_record,
        handle,
        new_subject,
    );
    assert!(try_send_many(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(201_102),
            grant
        ],
    )
    .is_err());
    assert_eq!(
        record_subjects(&read_acl_record(&svm, acl_record).expect("expected ACL record")),
        vec![payer.pubkey()]
    );
}

#[test]
fn host_pause_blocks_acl_grants_but_allows_admin_unpause() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let nonce_sequence = 42;
    let handle = [9; 32];
    let new_subject = Pubkey::new_unique();
    let acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        payer.pubkey(),
    );

    send(
        &mut svm,
        &payer,
        set_host_pause_ix(program_id, payer.pubkey(), host_config, true),
    );
    assert!(read_host_config(&svm, host_config).paused);

    let paused_grant = allow_acl_subjects_ix(
        program_id,
        payer.pubkey(),
        host_config,
        acl_record,
        handle,
        new_subject,
    );
    assert!(try_send_many(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(202_001),
            paused_grant,
        ],
    )
    .is_err());
    assert_eq!(
        record_subjects(&read_acl_record(&svm, acl_record).expect("expected ACL record")),
        vec![payer.pubkey()]
    );

    send(
        &mut svm,
        &payer,
        set_host_pause_ix(program_id, payer.pubkey(), host_config, false),
    );
    assert!(!read_host_config(&svm, host_config).paused);

    let unpaused_grant = allow_acl_subjects_ix(
        program_id,
        payer.pubkey(),
        host_config,
        acl_record,
        handle,
        new_subject,
    );
    send_many_with_meta(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(202_002),
            unpaused_grant,
        ],
    );
    assert_eq!(
        record_subjects(&read_acl_record(&svm, acl_record).expect("expected ACL record")),
        vec![payer.pubkey(), new_subject]
    );
}

#[test]
fn host_pause_allows_transient_session_close_cleanup() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let current_slot = current_slot(&svm);
    let authority_close_nonce = label("pause-close-auth");
    let expired_close_nonce = label("pause-close-exp");
    let (authority_close_session, _) =
        host::transient_session_address(payer.pubkey(), authority_close_nonce);
    let (expired_close_session, _) =
        host::transient_session_address(payer.pubkey(), expired_close_nonce);

    send(
        &mut svm,
        &payer,
        create_transient_session_ix(
            program_id,
            payer.pubkey(),
            host_config,
            authority_close_session,
            authority_close_nonce,
            payer.pubkey(),
            payer.pubkey(),
            current_slot,
            1,
        ),
    );
    send(
        &mut svm,
        &payer,
        create_transient_session_ix(
            program_id,
            payer.pubkey(),
            host_config,
            expired_close_session,
            expired_close_nonce,
            payer.pubkey(),
            payer.pubkey(),
            current_slot,
            1,
        ),
    );
    send(
        &mut svm,
        &payer,
        set_host_pause_ix(program_id, payer.pubkey(), host_config, true),
    );
    assert!(read_host_config(&svm, host_config).paused);

    let authority_close_ix = close_transient_session_ix(
        program_id,
        Some(payer.pubkey()),
        host_config,
        authority_close_session,
        payer.pubkey(),
    );
    send(&mut svm, &payer, authority_close_ix);
    assert!(read_transient_session(&svm, authority_close_session).is_none());

    svm.warp_to_slot(current_slot + 1);
    let expired_close_ix = close_transient_session_ix(
        program_id,
        None,
        host_config,
        expired_close_session,
        payer.pubkey(),
    );
    send(&mut svm, &payer, expired_close_ix);
    assert!(read_transient_session(&svm, expired_close_session).is_none());
}

#[test]
fn assert_acl_record_rejects_noncanonical_acl_record_address() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let _host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = Pubkey::new_unique();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let nonce_sequence = 42;
    let subject = payer.pubkey();
    let handle = [9; 32];
    let noncanonical_acl_record = Pubkey::new_unique();
    let mut subjects = [Pubkey::default(); host::MAX_ACL_SUBJECTS];
    let mut subject_roles = [0_u8; host::MAX_ACL_SUBJECTS];
    subjects[0] = subject;
    subject_roles[0] = host::ACL_ROLE_ALL;

    svm.set_account(
        noncanonical_acl_record,
        Account {
            lamports: 1_000_000_000,
            data: serialized_acl_record(AclRecord {
                handle,
                nonce_key,
                nonce_sequence,
                acl_domain_key,
                app_account,
                encrypted_value_label,
                subjects,
                subject_roles,
                subject_count: 1,
                overflow_subject_count: 0,
                public_decrypt: false,
                material_commitment: Pubkey::default(),
                material_commitment_hash: [0; 32],
                material_key_id: [0; 32],
                created_slot: 0,
                bump: 0,
            }),
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();

    let ix = anchor_ix(
        program_id,
        host::accounts::AssertAclRecord {
            acl_record: noncanonical_acl_record,
            subject_permission_record: None,
        },
        host::instruction::AssertAclRecord {
            nonce_key,
            nonce_sequence,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            handle,
            subject,
        },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
}

#[test]
fn assert_acl_record_rejects_malformed_subject_slots() {
    fn invalid_subject_count(record: &mut AclRecord) {
        record.subject_count = (host::MAX_ACL_SUBJECTS + 1) as u8;
    }
    fn duplicate_active_subject(record: &mut AclRecord) {
        record.subjects[1] = record.subjects[0];
        record.subject_roles[1] = host::ACL_ROLE_USE;
        record.subject_count = 2;
    }
    fn nonzero_unused_subject(record: &mut AclRecord) {
        record.subjects[1] = Pubkey::new_unique();
    }
    fn unknown_active_role(record: &mut AclRecord) {
        record.subject_roles[0] = host::ACL_ROLE_USE | 0x80;
    }

    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let _host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let cases: [(u64, &str, fn(&mut AclRecord)); 4] = [
        (50, "bad-count", invalid_subject_count),
        (51, "dup-subject", duplicate_active_subject),
        (52, "unused-subject", nonzero_unused_subject),
        (53, "unknown-role", unknown_active_role),
    ];

    for (nonce_sequence, label_name, mutate) in cases {
        let acl_domain_key = Pubkey::new_unique();
        let app_account = payer.pubkey();
        let encrypted_value_label = label(label_name);
        let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
        let handle = [nonce_sequence as u8; 32];
        let acl_record = seed_authorizing_acl_record(
            &mut svm,
            program_id,
            nonce_key,
            nonce_sequence,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            handle,
            payer.pubkey(),
        );
        mutate_acl_record(&mut svm, acl_record, mutate);

        let ix = assert_acl_record_ix(
            program_id,
            acl_record,
            None,
            nonce_key,
            nonce_sequence,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            handle,
            payer.pubkey(),
        );
        assert!(try_send(&mut svm, &payer, ix).is_err());
    }
}

#[test]
fn fhe_binary_op_scalar_rhs_skips_rhs_acl_but_encrypted_rhs_requires_it() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let dummy_rhs_account = svm.create_funded_account(1_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(7);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );
    let output_acl_record = acl_record_address(program_id, nonce_key, 1);
    let rhs_scalar = amount_plaintext(5);
    let result = current_bound_binary_handle(
        &svm,
        FheBinaryOpCode::Add,
        lhs,
        rhs_scalar,
        true,
        5,
        nonce_key,
        1,
    );

    let compute_and_bind_ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOpAndBindOutput {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record: dummy_rhs_account.pubkey(),
            rhs_permission_record: None,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOpAndBindOutput {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs: rhs_scalar,
            scalar: true,
            output_fhe_type: 5,
            result,
            output_nonce_key: nonce_key,
            output_nonce_sequence: 1,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(payer.pubkey())],
            output_public_decrypt: false,
        },
    );

    let mut cleartext = CleartextBackend::default();
    cleartext.seed_cleartext(lhs, TypedClearValue::uint64(10));
    let (meta, account_keys) = send_with_meta(&mut svm, &payer, compute_and_bind_ix);
    cleartext
        .ingest_transaction(&meta, &account_keys, program_id)
        .unwrap();
    let output_record = read_acl_record(&svm, output_acl_record).expect("expected output ACL");
    let events = binary_op_events(&meta, &account_keys, program_id);
    assert_eq!(events.len(), 1);
    assert!(events[0].scalar);
    assert_eq!(events[0].rhs, rhs_scalar);
    assert_eq!(
        cleartext.decrypt_cleartext(output_record.handle),
        Some(TypedClearValue::uint64(15))
    );

    let encrypted_rhs_output = acl_record_address(program_id, nonce_key, 2);
    let encrypted_rhs = input_handle_for_chain(9);
    let encrypted_rhs_result =
        current_binary_handle(&svm, FheBinaryOpCode::Add, lhs, encrypted_rhs, false, 5);
    let encrypted_rhs_ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOp {
            compute_subject: payer.pubkey(),
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record: dummy_rhs_account.pubkey(),
            rhs_permission_record: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOp {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs: encrypted_rhs,
            scalar: false,
            output_fhe_type: 5,
            result: encrypted_rhs_result,
        },
    );
    assert!(try_send(&mut svm, &payer, encrypted_rhs_ix).is_err());
    assert!(read_acl_record(&svm, encrypted_rhs_output).is_none());
}

#[test]
fn fhe_binary_op_scalar_rhs_rejects_unused_rhs_permission_witness() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let dummy_rhs_account = svm.create_funded_account(1_000_000).unwrap();
    let unused_rhs_permission = svm.create_funded_account(1_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("scalar-extra-rhs");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(23);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );
    let rhs_scalar = amount_plaintext(7);

    let result = current_binary_handle(&svm, FheBinaryOpCode::Add, lhs, rhs_scalar, true, 5);
    let scalar_ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOp {
            compute_subject: payer.pubkey(),
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record: dummy_rhs_account.pubkey(),
            rhs_permission_record: Some(unused_rhs_permission.pubkey()),
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOp {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs: rhs_scalar,
            scalar: true,
            output_fhe_type: 5,
            result,
        },
    );
    assert!(try_send(&mut svm, &payer, scalar_ix).is_err());

    let output_acl_record = acl_record_address(program_id, nonce_key, 1);
    let bound_result = current_bound_binary_handle(
        &svm,
        FheBinaryOpCode::Add,
        lhs,
        rhs_scalar,
        true,
        5,
        nonce_key,
        1,
    );
    let bind_ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOpAndBindOutput {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record: dummy_rhs_account.pubkey(),
            rhs_permission_record: Some(unused_rhs_permission.pubkey()),
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOpAndBindOutput {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs: rhs_scalar,
            scalar: true,
            output_fhe_type: 5,
            result: bound_result,
            output_nonce_key: nonce_key,
            output_nonce_sequence: 1,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(payer.pubkey())],
            output_public_decrypt: false,
        },
    );
    assert!(try_send(&mut svm, &payer, bind_ix).is_err());
    assert!(read_acl_record(&svm, output_acl_record).is_none());
}

#[test]
fn fhe_binary_op_rejects_unsupported_output_type() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let dummy_rhs_account = svm.create_funded_account(1_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("bad-binary-type");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(25);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );
    let rhs_scalar = amount_plaintext(7);
    let unsupported_type = 1;
    let result = current_binary_handle(
        &svm,
        FheBinaryOpCode::Add,
        lhs,
        rhs_scalar,
        true,
        unsupported_type,
    );
    let direct_ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOp {
            compute_subject: payer.pubkey(),
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record: dummy_rhs_account.pubkey(),
            rhs_permission_record: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOp {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs: rhs_scalar,
            scalar: true,
            output_fhe_type: unsupported_type,
            result,
        },
    );
    assert!(try_send(&mut svm, &payer, direct_ix).is_err());

    let output_acl_record = acl_record_address(program_id, nonce_key, 1);
    let bound_result = current_bound_binary_handle(
        &svm,
        FheBinaryOpCode::Add,
        lhs,
        rhs_scalar,
        true,
        unsupported_type,
        nonce_key,
        1,
    );
    let bind_ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOpAndBindOutput {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record: dummy_rhs_account.pubkey(),
            rhs_permission_record: None,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOpAndBindOutput {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs: rhs_scalar,
            scalar: true,
            output_fhe_type: unsupported_type,
            result: bound_result,
            output_nonce_key: nonce_key,
            output_nonce_sequence: 1,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(payer.pubkey())],
            output_public_decrypt: false,
        },
    );
    assert!(try_send(&mut svm, &payer, bind_ix).is_err());
    assert!(read_acl_record(&svm, output_acl_record).is_none());
}

#[test]
fn binary_host_paths_reject_operator_output_type_mismatch() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let dummy_rhs_account = svm.create_funded_account(1_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("bad-op-output-type");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(26);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );
    let rhs_scalar = amount_plaintext(7);
    let wrong_output_type = 5;

    let direct_result = current_binary_handle(
        &svm,
        FheBinaryOpCode::Ge,
        lhs,
        rhs_scalar,
        true,
        wrong_output_type,
    );
    let direct_ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOp {
            compute_subject: payer.pubkey(),
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record: dummy_rhs_account.pubkey(),
            rhs_permission_record: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOp {
            op: FheBinaryOpCode::Ge,
            lhs,
            rhs: rhs_scalar,
            scalar: true,
            output_fhe_type: wrong_output_type,
            result: direct_result,
        },
    );
    assert!(try_send(&mut svm, &payer, direct_ix).is_err());

    let output_acl_record = acl_record_address(program_id, nonce_key, 1);
    let bound_result = current_bound_binary_handle(
        &svm,
        FheBinaryOpCode::Ge,
        lhs,
        rhs_scalar,
        true,
        wrong_output_type,
        nonce_key,
        1,
    );
    let bind_ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOpAndBindOutput {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record: dummy_rhs_account.pubkey(),
            rhs_permission_record: None,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOpAndBindOutput {
            op: FheBinaryOpCode::Ge,
            lhs,
            rhs: rhs_scalar,
            scalar: true,
            output_fhe_type: wrong_output_type,
            result: bound_result,
            output_nonce_key: nonce_key,
            output_nonce_sequence: 1,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(payer.pubkey())],
            output_public_decrypt: false,
        },
    );
    assert!(try_send(&mut svm, &payer, bind_ix).is_err());
    assert!(read_acl_record(&svm, output_acl_record).is_none());

    let context_id = label("bad-eval-op-type");
    let eval_result = current_eval_handle(
        &svm,
        FheBinaryOpCode::Ge,
        lhs,
        rhs_scalar,
        true,
        wrong_output_type,
        context_id,
        0,
    );
    let mut eval_ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            system_program: system_program::ID,
            instructions_sysvar: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                ops: vec![FheEvalOp {
                    op: FheBinaryOpCode::Ge,
                    lhs: FheEvalOperand::Durable {
                        handle: lhs,
                        acl_record_index: 0,
                        permission_index: None,
                    },
                    rhs: FheEvalOperand::Scalar(rhs_scalar),
                    output_fhe_type: wrong_output_type,
                    result: eval_result,
                    output: FheEvalOutput::Transient,
                }],
            },
        },
    );
    eval_ix
        .accounts
        .push(AccountMeta::new_readonly(lhs_acl_record, false));
    assert!(try_send(&mut svm, &payer, eval_ix).is_err());
}

#[test]
fn binary_host_paths_reject_operand_type_mismatch() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let dummy_rhs_account = svm.create_funded_account(1_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let lhs_label = label("binary-type-lhs");
    let rhs_label = label("binary-type-rhs");
    let output_label = label("binary-type-output");
    let lhs_nonce_key = token::nonce_key(acl_domain_key, app_account, lhs_label);
    let rhs_nonce_key = token::nonce_key(acl_domain_key, app_account, rhs_label);
    let output_nonce_key = token::nonce_key(acl_domain_key, app_account, output_label);
    let lhs = input_handle_for_chain_with_type(34, 5);
    let rhs = input_handle_for_chain_with_type(35, 4);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        lhs_nonce_key,
        0,
        acl_domain_key,
        app_account,
        lhs_label,
        lhs,
        payer.pubkey(),
    );
    let rhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        rhs_nonce_key,
        0,
        acl_domain_key,
        app_account,
        rhs_label,
        rhs,
        payer.pubkey(),
    );
    let rhs_scalar = amount_plaintext(7);

    let wrong_result_type = 4;
    let direct_result = current_binary_handle(
        &svm,
        FheBinaryOpCode::Add,
        lhs,
        rhs_scalar,
        true,
        wrong_result_type,
    );
    let direct_ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOp {
            compute_subject: payer.pubkey(),
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record: dummy_rhs_account.pubkey(),
            rhs_permission_record: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOp {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs: rhs_scalar,
            scalar: true,
            output_fhe_type: wrong_result_type,
            result: direct_result,
        },
    );
    assert!(try_send(&mut svm, &payer, direct_ix).is_err());

    let output_acl_record = acl_record_address(program_id, output_nonce_key, 0);
    let bound_result = current_bound_binary_handle(
        &svm,
        FheBinaryOpCode::Add,
        lhs,
        rhs,
        false,
        5,
        output_nonce_key,
        0,
    );
    let bind_ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOpAndBindOutput {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record,
            rhs_permission_record: None,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOpAndBindOutput {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs,
            scalar: false,
            output_fhe_type: 5,
            result: bound_result,
            output_nonce_key,
            output_nonce_sequence: 0,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: output_label,
            output_subjects: vec![AclSubjectEntry::user(payer.pubkey())],
            output_public_decrypt: false,
        },
    );
    assert!(try_send(&mut svm, &payer, bind_ix).is_err());
    assert!(read_acl_record(&svm, output_acl_record).is_none());

    let context_id = label("bad-eval-operand-type");
    let eval_result = current_eval_handle(
        &svm,
        FheBinaryOpCode::Add,
        lhs,
        rhs,
        false,
        5,
        context_id,
        0,
    );
    let mut eval_ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            system_program: system_program::ID,
            instructions_sysvar: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                ops: vec![FheEvalOp {
                    op: FheBinaryOpCode::Add,
                    lhs: FheEvalOperand::Durable {
                        handle: lhs,
                        acl_record_index: 0,
                        permission_index: None,
                    },
                    rhs: FheEvalOperand::Durable {
                        handle: rhs,
                        acl_record_index: 1,
                        permission_index: None,
                    },
                    output_fhe_type: 5,
                    result: eval_result,
                    output: FheEvalOutput::Transient,
                }],
            },
        },
    );
    eval_ix
        .accounts
        .push(AccountMeta::new_readonly(lhs_acl_record, false));
    eval_ix
        .accounts
        .push(AccountMeta::new_readonly(rhs_acl_record, false));
    assert!(try_send(&mut svm, &payer, eval_ix).is_err());
}

#[test]
fn fhe_binary_op_rejects_unexpected_remaining_account() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let dummy_rhs_account = svm.create_funded_account(1_000_000).unwrap();
    let unexpected_account = svm.create_funded_account(1_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("extra-binary");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(31);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );
    let rhs_scalar = amount_plaintext(4);
    let result = current_binary_handle(&svm, FheBinaryOpCode::Add, lhs, rhs_scalar, true, 5);
    let mut ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOp {
            compute_subject: payer.pubkey(),
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record: dummy_rhs_account.pubkey(),
            rhs_permission_record: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOp {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs: rhs_scalar,
            scalar: true,
            output_fhe_type: 5,
            result,
        },
    );
    ix.accounts.push(AccountMeta::new_readonly(
        unexpected_account.pubkey(),
        false,
    ));
    assert!(try_send(&mut svm, &payer, ix).is_err());
}

#[test]
fn fhe_binary_op_does_not_create_durable_acl_without_allow_step() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let dummy_rhs_account = svm.create_funded_account(1_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(7);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );
    let rhs_scalar = amount_plaintext(5);
    let result = current_binary_handle(&svm, FheBinaryOpCode::Add, lhs, rhs_scalar, true, 5);
    let output_acl_record = acl_record_address(program_id, nonce_key, 1);

    let ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOp {
            compute_subject: payer.pubkey(),
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record: dummy_rhs_account.pubkey(),
            rhs_permission_record: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOp {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs: rhs_scalar,
            scalar: true,
            output_fhe_type: 5,
            result,
        },
    );

    let (meta, account_keys) = send_with_meta(&mut svm, &payer, ix);

    let events = binary_op_events(&meta, &account_keys, program_id);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].result, result);
    assert!(read_acl_record(&svm, output_acl_record).is_none());
}

#[test]
fn fhe_eval_uses_transient_intermediate_and_binds_only_final_output() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(7);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );
    let output_acl_record = acl_record_address(program_id, nonce_key, 1);
    let context_id = label("eval-context");
    let rhs_first = amount_plaintext(5);
    let transient_result = current_eval_handle(
        &svm,
        FheBinaryOpCode::Add,
        lhs,
        rhs_first,
        true,
        5,
        context_id,
        0,
    );
    let rhs_second = amount_plaintext(3);
    let final_result = current_bound_eval_handle(
        &svm,
        FheBinaryOpCode::Add,
        transient_result,
        rhs_second,
        true,
        5,
        context_id,
        1,
        nonce_key,
        1,
    );
    let mut ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            system_program: system_program::ID,
            instructions_sysvar: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                ops: vec![
                    FheEvalOp {
                        op: FheBinaryOpCode::Add,
                        lhs: FheEvalOperand::Durable {
                            handle: lhs,
                            acl_record_index: 0,
                            permission_index: None,
                        },
                        rhs: FheEvalOperand::Scalar(rhs_first),
                        output_fhe_type: 5,
                        result: transient_result,
                        output: FheEvalOutput::Transient,
                    },
                    FheEvalOp {
                        op: FheBinaryOpCode::Add,
                        lhs: FheEvalOperand::Transient { producer_index: 0 },
                        rhs: FheEvalOperand::Scalar(rhs_second),
                        output_fhe_type: 5,
                        result: final_result,
                        output: FheEvalOutput::Durable {
                            output_acl_record_index: 1,
                            output_nonce_key: nonce_key,
                            output_nonce_sequence: 1,
                            output_acl_domain_key: acl_domain_key,
                            output_app_account: app_account,
                            output_encrypted_value_label: encrypted_value_label,
                            output_subjects: vec![AclSubjectEntry::user(payer.pubkey())],
                            output_public_decrypt: false,
                        },
                    },
                ],
            },
        },
    );
    ix.accounts
        .push(AccountMeta::new_readonly(lhs_acl_record, false));
    ix.accounts.push(AccountMeta::new(output_acl_record, false));

    let mut cleartext = CleartextBackend::default();
    cleartext.seed_cleartext(lhs, TypedClearValue::uint64(10));
    let (meta, account_keys) = send_with_meta(&mut svm, &payer, ix);
    cleartext
        .ingest_transaction(&meta, &account_keys, program_id)
        .unwrap();

    let events = binary_op_events(&meta, &account_keys, program_id);
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].result, transient_result);
    assert_eq!(events[1].lhs, transient_result);
    let output_record = read_acl_record(&svm, output_acl_record).expect("expected output ACL");
    assert_eq!(output_record.handle, final_result);
    assert_eq!(
        cleartext.decrypt_cleartext(final_result),
        Some(TypedClearValue::uint64(18))
    );
}

#[test]
fn fhe_eval_rejects_unsupported_output_type() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("bad-eval-type");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(27);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );
    let context_id = label("bad-eval-output");
    let rhs = amount_plaintext(4);
    let unsupported_type = 1;
    let result = current_eval_handle(
        &svm,
        FheBinaryOpCode::Add,
        lhs,
        rhs,
        true,
        unsupported_type,
        context_id,
        0,
    );
    let mut ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            system_program: system_program::ID,
            instructions_sysvar: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                ops: vec![FheEvalOp {
                    op: FheBinaryOpCode::Add,
                    lhs: FheEvalOperand::Durable {
                        handle: lhs,
                        acl_record_index: 0,
                        permission_index: None,
                    },
                    rhs: FheEvalOperand::Scalar(rhs),
                    output_fhe_type: unsupported_type,
                    result,
                    output: FheEvalOutput::Transient,
                }],
            },
        },
    );
    ix.accounts
        .push(AccountMeta::new_readonly(lhs_acl_record, false));

    assert!(try_send(&mut svm, &payer, ix).is_err());
}

#[test]
fn fhe_eval_rejects_unused_dynamic_accounts() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let extra_account = svm.create_funded_account(1_000_000).unwrap();
    let unused_sysvar_slot = svm.create_funded_account(1_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("eval-extra");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(29);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );

    let build_eval_ix = |svm: &LiteSVM, context_id: [u8; 32], instructions_sysvar| {
        let rhs = amount_plaintext(2);
        let result =
            current_eval_handle(svm, FheBinaryOpCode::Add, lhs, rhs, true, 5, context_id, 0);
        anchor_ix(
            program_id,
            host::accounts::FheEval {
                payer: payer.pubkey(),
                compute_subject: payer.pubkey(),
                app_account_authority: app_account,
                host_config,
                system_program: system_program::ID,
                instructions_sysvar,
                event_authority: event_authority(program_id),
                program: program_id,
            },
            host::instruction::FheEval {
                args: FheEvalArgs {
                    context_id,
                    ops: vec![FheEvalOp {
                        op: FheBinaryOpCode::Add,
                        lhs: FheEvalOperand::Durable {
                            handle: lhs,
                            acl_record_index: 0,
                            permission_index: None,
                        },
                        rhs: FheEvalOperand::Scalar(rhs),
                        output_fhe_type: 5,
                        result,
                        output: FheEvalOutput::Transient,
                    }],
                },
            },
        )
    };

    let mut extra_remaining_ix = build_eval_ix(&svm, label("eval-extra-acct"), None);
    extra_remaining_ix
        .accounts
        .push(AccountMeta::new_readonly(lhs_acl_record, false));
    extra_remaining_ix
        .accounts
        .push(AccountMeta::new_readonly(extra_account.pubkey(), false));
    assert!(try_send(&mut svm, &payer, extra_remaining_ix).is_err());

    let mut unused_sysvar_ix = build_eval_ix(
        &svm,
        label("eval-unused-sysvar"),
        Some(unused_sysvar_slot.pubkey()),
    );
    unused_sysvar_ix
        .accounts
        .push(AccountMeta::new_readonly(lhs_acl_record, false));
    assert!(try_send(&mut svm, &payer, unused_sysvar_ix).is_err());
}

#[test]
fn fhe_eval_rejects_unproduced_transient_operand() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let context_id = label("bad-eval-context");
    let ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            instructions_sysvar: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                ops: vec![FheEvalOp {
                    op: FheBinaryOpCode::Add,
                    lhs: FheEvalOperand::Transient { producer_index: 0 },
                    rhs: FheEvalOperand::Scalar(amount_plaintext(1)),
                    output_fhe_type: 5,
                    result: [9; 32],
                    output: FheEvalOutput::Transient,
                }],
            },
        },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
}

#[test]
fn fhe_eval_transient_output_is_not_reusable_by_later_instruction() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let dummy_rhs_account = svm.create_funded_account(1_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(7);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );
    let context_id = label("one-shot-eval");
    let rhs_first = amount_plaintext(5);
    let transient_result = current_eval_handle(
        &svm,
        FheBinaryOpCode::Add,
        lhs,
        rhs_first,
        true,
        5,
        context_id,
        0,
    );
    let mut eval_ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            system_program: system_program::ID,
            instructions_sysvar: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                ops: vec![FheEvalOp {
                    op: FheBinaryOpCode::Add,
                    lhs: FheEvalOperand::Durable {
                        handle: lhs,
                        acl_record_index: 0,
                        permission_index: None,
                    },
                    rhs: FheEvalOperand::Scalar(rhs_first),
                    output_fhe_type: 5,
                    result: transient_result,
                    output: FheEvalOutput::Transient,
                }],
            },
        },
    );
    eval_ix
        .accounts
        .push(AccountMeta::new_readonly(lhs_acl_record, false));
    send(&mut svm, &payer, eval_ix);

    let rhs_second = amount_plaintext(3);
    let later_result = current_binary_handle(
        &svm,
        FheBinaryOpCode::Add,
        transient_result,
        rhs_second,
        true,
        5,
    );
    let later_ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOp {
            compute_subject: payer.pubkey(),
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record: dummy_rhs_account.pubkey(),
            rhs_permission_record: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOp {
            op: FheBinaryOpCode::Add,
            lhs: transient_result,
            rhs: rhs_second,
            scalar: true,
            output_fhe_type: 5,
            result: later_result,
        },
    );
    assert!(try_send(&mut svm, &payer, later_ix).is_err());
}

#[test]
fn transient_session_allows_sealed_capability_once() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(7);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );
    let session_nonce = label("transient-session-1");
    let (session, _) = host::transient_session_address(payer.pubkey(), session_nonce);
    let current_slot = current_slot(&svm);
    let capability = transient_capability(
        payer.pubkey(),
        program_id,
        acl_domain_key,
        app_account,
        false,
    );
    let create_ix = create_transient_session_ix(
        program_id,
        payer.pubkey(),
        host_config,
        session,
        session_nonce,
        payer.pubkey(),
        payer.pubkey(),
        current_slot,
        1,
    );
    let allow_ix = anchor_ix(
        program_id,
        host::accounts::AllowTransientHandle {
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record: lhs_acl_record,
            session,
            host_config,
            deny_subject_record: None,
        },
        host::instruction::AllowTransientHandle {
            handle: lhs,
            capability,
        },
    );
    let seal_ix = seal_transient_session_ix(program_id, payer.pubkey(), host_config, session);
    let context_id = label("session-consume");
    let rhs = amount_plaintext(5);
    let result = current_eval_handle(&svm, FheBinaryOpCode::Add, lhs, rhs, true, 5, context_id, 0);
    let consume_ix = session_consume_eval_ix(
        program_id,
        payer.pubkey(),
        host_config,
        session,
        context_id,
        lhs,
        rhs,
        result,
        FheEvalOutput::Transient,
    );

    send_many_with_signers(
        &mut svm,
        &payer.pubkey(),
        vec![create_ix, allow_ix, seal_ix, consume_ix],
        &[&payer],
    )
    .unwrap();

    let session_account = read_transient_session(&svm, session).expect("expected session");
    assert_eq!(session_account.state, host::TRANSIENT_SESSION_STATE_SEALED);
    assert_eq!(session_account.entries.len(), 1);
    assert_eq!(session_account.entries[0].used_count, 1);

    let second_result = current_eval_handle(
        &svm,
        FheBinaryOpCode::Add,
        lhs,
        rhs,
        true,
        5,
        label("session-reuse"),
        0,
    );
    let second_consume_ix = session_consume_eval_ix(
        program_id,
        payer.pubkey(),
        host_config,
        session,
        label("session-reuse"),
        lhs,
        rhs,
        second_result,
        FheEvalOutput::Transient,
    );
    assert!(try_send(&mut svm, &payer, second_consume_ix).is_err());

    let close_ix = close_transient_session_ix(
        program_id,
        Some(payer.pubkey()),
        host_config,
        session,
        payer.pubkey(),
    );
    send(&mut svm, &payer, close_ix);
    assert!(read_transient_session(&svm, session).is_none());
}

#[test]
fn transient_session_rejects_noncanonical_account_length() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let session_nonce = label("transient-long-account");
    let (session, bump) = host::transient_session_address(payer.pubkey(), session_nonce);
    let current_slot = current_slot(&svm);
    let handle = input_handle_for_chain(7);
    let capability = transient_capability(
        payer.pubkey(),
        program_id,
        Pubkey::new_unique(),
        payer.pubkey(),
        false,
    );
    let mut data = serialized_account(TransientSession {
        session_nonce,
        authority: payer.pubkey(),
        refund_recipient: payer.pubkey(),
        compute_subject: payer.pubkey(),
        created_slot: current_slot,
        expires_slot: current_slot,
        state: host::TRANSIENT_SESSION_STATE_SEALED,
        max_entries: 1,
        entries: vec![host::TransientCapability {
            handle,
            grant: capability,
            used_count: 0,
        }],
        bump,
    });
    data.push(0);
    svm.set_account(
        session,
        Account {
            lamports: svm.minimum_balance_for_rent_exemption(data.len()),
            data,
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();

    let context_id = label("long-session");
    let rhs = amount_plaintext(5);
    let result = current_eval_handle(
        &svm,
        FheBinaryOpCode::Add,
        handle,
        rhs,
        true,
        5,
        context_id,
        0,
    );
    let consume_ix = session_consume_eval_ix(
        program_id,
        payer.pubkey(),
        host_config,
        session,
        context_id,
        handle,
        rhs,
        result,
        FheEvalOutput::Transient,
    );
    assert!(try_send(&mut svm, &payer, consume_ix).is_err());
    let session_account = read_transient_session(&svm, session).expect("expected session");
    assert_eq!(session_account.entries[0].used_count, 0);

    let close_ix = close_transient_session_ix(
        program_id,
        Some(payer.pubkey()),
        host_config,
        session,
        payer.pubkey(),
    );
    assert!(try_send(&mut svm, &payer, close_ix).is_err());
    assert!(read_transient_session(&svm, session).is_some());
}

#[test]
fn transient_session_rejects_multi_entry_or_multi_use_capability() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let bad_capacity_nonce = label("transient-capacity");
    let (bad_capacity_session, _) =
        host::transient_session_address(payer.pubkey(), bad_capacity_nonce);
    let bad_capacity_ix = create_transient_session_ix(
        program_id,
        payer.pubkey(),
        host_config,
        bad_capacity_session,
        bad_capacity_nonce,
        payer.pubkey(),
        payer.pubkey(),
        current_slot(&svm),
        2,
    );
    assert!(try_send(&mut svm, &payer, bad_capacity_ix).is_err());
    assert!(read_transient_session(&svm, bad_capacity_session).is_none());

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = input_handle_for_chain(7);
    let acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        payer.pubkey(),
    );
    let session_nonce = label("transient-max-uses");
    let (session, _) = host::transient_session_address(payer.pubkey(), session_nonce);
    let create_ix = create_transient_session_ix(
        program_id,
        payer.pubkey(),
        host_config,
        session,
        session_nonce,
        payer.pubkey(),
        payer.pubkey(),
        current_slot(&svm),
        1,
    );
    send(&mut svm, &payer, create_ix);

    let mut capability = transient_capability(
        payer.pubkey(),
        program_id,
        acl_domain_key,
        app_account,
        false,
    );
    capability.max_uses = 2;
    let allow_ix = anchor_ix(
        program_id,
        host::accounts::AllowTransientHandle {
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record,
            session,
            host_config,
            deny_subject_record: None,
        },
        host::instruction::AllowTransientHandle { handle, capability },
    );
    assert!(try_send(&mut svm, &payer, allow_ix).is_err());
    let session_account = read_transient_session(&svm, session).expect("expected session");
    assert!(session_account.entries.is_empty());

    let mut capability = transient_capability(
        payer.pubkey(),
        program_id,
        acl_domain_key,
        app_account,
        false,
    );
    capability.role_flags = host::ACL_ROLE_USER;
    let allow_ix = anchor_ix(
        program_id,
        host::accounts::AllowTransientHandle {
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record,
            session,
            host_config,
            deny_subject_record: None,
        },
        host::instruction::AllowTransientHandle { handle, capability },
    );
    assert!(try_send(&mut svm, &payer, allow_ix).is_err());
    let session_account = read_transient_session(&svm, session).expect("expected session");
    assert!(session_account.entries.is_empty());
}

#[test]
fn transient_session_rejects_durable_output_without_policy() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(7);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );
    let session_nonce = label("transient-session-2");
    let (session, _) = host::transient_session_address(payer.pubkey(), session_nonce);
    let create_ix = create_transient_session_ix(
        program_id,
        payer.pubkey(),
        host_config,
        session,
        session_nonce,
        payer.pubkey(),
        payer.pubkey(),
        current_slot(&svm),
        1,
    );
    let allow_ix = anchor_ix(
        program_id,
        host::accounts::AllowTransientHandle {
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record: lhs_acl_record,
            session,
            host_config,
            deny_subject_record: None,
        },
        host::instruction::AllowTransientHandle {
            handle: lhs,
            capability: transient_capability(
                payer.pubkey(),
                program_id,
                acl_domain_key,
                app_account,
                false,
            ),
        },
    );
    let seal_ix = seal_transient_session_ix(program_id, payer.pubkey(), host_config, session);

    let output_acl_record = acl_record_address(program_id, nonce_key, 1);
    let rhs = amount_plaintext(5);
    let context_id = label("durable-denied");
    let result = current_bound_eval_handle(
        &svm,
        FheBinaryOpCode::Add,
        lhs,
        rhs,
        true,
        5,
        context_id,
        0,
        nonce_key,
        1,
    );
    let mut durable_ix = session_consume_eval_ix(
        program_id,
        payer.pubkey(),
        host_config,
        session,
        context_id,
        lhs,
        rhs,
        result,
        FheEvalOutput::Durable {
            output_acl_record_index: 1,
            output_nonce_key: nonce_key,
            output_nonce_sequence: 1,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(payer.pubkey())],
            output_public_decrypt: false,
        },
    );
    durable_ix
        .accounts
        .push(AccountMeta::new(output_acl_record, false));

    assert!(try_send_many(
        &mut svm,
        &payer,
        vec![create_ix, allow_ix, seal_ix, durable_ix]
    )
    .is_err());
    assert!(read_acl_record(&svm, output_acl_record).is_none());
}

#[test]
fn transient_session_rejects_durable_output_roles_broader_than_policy() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(7);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );
    let session_nonce = label("transient-policy-roles");
    let (session, _) = host::transient_session_address(payer.pubkey(), session_nonce);
    let create_ix = create_transient_session_ix(
        program_id,
        payer.pubkey(),
        host_config,
        session,
        session_nonce,
        payer.pubkey(),
        payer.pubkey(),
        current_slot(&svm),
        1,
    );
    let allow_ix = anchor_ix(
        program_id,
        host::accounts::AllowTransientHandle {
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record: lhs_acl_record,
            session,
            host_config,
            deny_subject_record: None,
        },
        host::instruction::AllowTransientHandle {
            handle: lhs,
            capability: transient_capability(
                payer.pubkey(),
                program_id,
                acl_domain_key,
                app_account,
                true,
            ),
        },
    );
    let seal_ix = seal_transient_session_ix(program_id, payer.pubkey(), host_config, session);

    let output_acl_record = acl_record_address(program_id, nonce_key, 1);
    let rhs = amount_plaintext(5);
    let context_id = label("durable-role-denied");
    let result = current_bound_eval_handle(
        &svm,
        FheBinaryOpCode::Add,
        lhs,
        rhs,
        true,
        5,
        context_id,
        0,
        nonce_key,
        1,
    );
    let mut durable_ix = session_consume_eval_ix(
        program_id,
        payer.pubkey(),
        host_config,
        session,
        context_id,
        lhs,
        rhs,
        result,
        FheEvalOutput::Durable {
            output_acl_record_index: 1,
            output_nonce_key: nonce_key,
            output_nonce_sequence: 1,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(payer.pubkey())],
            output_public_decrypt: false,
        },
    );
    durable_ix
        .accounts
        .push(AccountMeta::new(output_acl_record, false));

    assert!(try_send_many(
        &mut svm,
        &payer,
        vec![create_ix, allow_ix, seal_ix, durable_ix]
    )
    .is_err());
    assert!(read_acl_record(&svm, output_acl_record).is_none());
}

#[test]
fn transient_session_allows_durable_output_with_policy_subject_roles() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(7);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );
    let session_nonce = label("transient-policy-allow");
    let (session, _) = host::transient_session_address(payer.pubkey(), session_nonce);
    let create_ix = create_transient_session_ix(
        program_id,
        payer.pubkey(),
        host_config,
        session,
        session_nonce,
        payer.pubkey(),
        payer.pubkey(),
        current_slot(&svm),
        1,
    );
    let mut capability = transient_capability(
        payer.pubkey(),
        program_id,
        acl_domain_key,
        app_account,
        true,
    );
    capability.role_flags = host::ACL_ROLE_COMPUTE_SUBJECT;
    let allow_ix = anchor_ix(
        program_id,
        host::accounts::AllowTransientHandle {
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record: lhs_acl_record,
            session,
            host_config,
            deny_subject_record: None,
        },
        host::instruction::AllowTransientHandle {
            handle: lhs,
            capability,
        },
    );
    let seal_ix = seal_transient_session_ix(program_id, payer.pubkey(), host_config, session);

    let output_acl_record = acl_record_address(program_id, nonce_key, 1);
    let rhs = amount_plaintext(5);
    let context_id = label("durable-role-allow");
    let result = current_bound_eval_handle(
        &svm,
        FheBinaryOpCode::Add,
        lhs,
        rhs,
        true,
        5,
        context_id,
        0,
        nonce_key,
        1,
    );
    let mut durable_ix = session_consume_eval_ix(
        program_id,
        payer.pubkey(),
        host_config,
        session,
        context_id,
        lhs,
        rhs,
        result,
        FheEvalOutput::Durable {
            output_acl_record_index: 1,
            output_nonce_key: nonce_key,
            output_nonce_sequence: 1,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::compute(payer.pubkey())],
            output_public_decrypt: false,
        },
    );
    durable_ix
        .accounts
        .push(AccountMeta::new(output_acl_record, false));

    send_many_with_signers(
        &mut svm,
        &payer.pubkey(),
        vec![create_ix, allow_ix, seal_ix, durable_ix],
        &[&payer],
    )
    .unwrap();
    let output_record = read_acl_record(&svm, output_acl_record).expect("expected output ACL");
    assert_eq!(output_record.handle, result);
    assert!(output_record.inline_subject_has_role(payer.pubkey(), host::ACL_ROLE_COMPUTE));
    let session_account = read_transient_session(&svm, session).expect("expected session");
    assert_eq!(session_account.entries[0].used_count, 1);
}

#[test]
fn fhe_eval_can_append_and_consume_transient_session_output_in_one_transaction() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(7);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );
    let session_nonce = label("transient-session-3");
    let (session, _) = host::transient_session_address(payer.pubkey(), session_nonce);
    let create_ix = create_transient_session_ix(
        program_id,
        payer.pubkey(),
        host_config,
        session,
        session_nonce,
        payer.pubkey(),
        payer.pubkey(),
        current_slot(&svm),
        1,
    );

    let first_context = label("append-session");
    let rhs_first = amount_plaintext(5);
    let session_handle = current_eval_handle(
        &svm,
        FheBinaryOpCode::Add,
        lhs,
        rhs_first,
        true,
        5,
        first_context,
        0,
    );
    let mut append_ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            host_config,
            system_program: system_program::ID,
            instructions_sysvar: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id: first_context,
                ops: vec![FheEvalOp {
                    op: FheBinaryOpCode::Add,
                    lhs: FheEvalOperand::Durable {
                        handle: lhs,
                        acl_record_index: 0,
                        permission_index: None,
                    },
                    rhs: FheEvalOperand::Scalar(rhs_first),
                    output_fhe_type: 5,
                    result: session_handle,
                    output: FheEvalOutput::TransientSession {
                        session_index: 1,
                        capability: transient_capability(
                            payer.pubkey(),
                            program_id,
                            acl_domain_key,
                            app_account,
                            false,
                        ),
                    },
                }],
            },
        },
    );
    append_ix
        .accounts
        .push(AccountMeta::new_readonly(lhs_acl_record, false));
    append_ix.accounts.push(AccountMeta::new(session, false));

    let seal_ix = seal_transient_session_ix(program_id, payer.pubkey(), host_config, session);

    let second_context = label("consume-appended");
    let rhs_second = amount_plaintext(3);
    let final_result = current_eval_handle(
        &svm,
        FheBinaryOpCode::Add,
        session_handle,
        rhs_second,
        true,
        5,
        second_context,
        0,
    );
    let consume_ix = session_consume_eval_ix(
        program_id,
        payer.pubkey(),
        host_config,
        session,
        second_context,
        session_handle,
        rhs_second,
        final_result,
        FheEvalOutput::Transient,
    );
    send_many_with_signers(
        &mut svm,
        &payer.pubkey(),
        vec![create_ix, append_ix, seal_ix, consume_ix],
        &[&payer],
    )
    .unwrap();
    let session_account = read_transient_session(&svm, session).expect("expected session");
    assert_eq!(session_account.entries[0].handle, session_handle);
    assert_eq!(session_account.entries[0].used_count, 1);
}

#[test]
fn transient_session_rejects_consume_without_current_transaction_create() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let session_nonce = label("transient-late-consume");
    let (session, bump) = host::transient_session_address(payer.pubkey(), session_nonce);
    let current_slot = current_slot(&svm);
    let handle = input_handle_for_chain(7);
    let capability = transient_capability(
        payer.pubkey(),
        program_id,
        Pubkey::new_unique(),
        payer.pubkey(),
        false,
    );
    svm.set_account(
        session,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(TransientSession {
                session_nonce,
                authority: payer.pubkey(),
                refund_recipient: payer.pubkey(),
                compute_subject: payer.pubkey(),
                created_slot: current_slot,
                expires_slot: current_slot,
                state: host::TRANSIENT_SESSION_STATE_SEALED,
                max_entries: 1,
                entries: vec![host::TransientCapability {
                    handle,
                    grant: capability,
                    used_count: 0,
                }],
                bump,
            }),
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();

    let rhs = amount_plaintext(3);
    let context_id = label("late-consume");
    let result = current_eval_handle(
        &svm,
        FheBinaryOpCode::Add,
        handle,
        rhs,
        true,
        5,
        context_id,
        0,
    );
    let consume_ix = session_consume_eval_ix(
        program_id,
        payer.pubkey(),
        host_config,
        session,
        context_id,
        handle,
        rhs,
        result,
        FheEvalOutput::Transient,
    );
    assert!(try_send(&mut svm, &payer, consume_ix).is_err());
}

#[test]
fn allow_transient_handle_does_not_emit_legacy_acl_allowed_event() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = input_handle_for_chain(7);
    let acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        payer.pubkey(),
    );
    let session_nonce = label("transient-session-4");
    let (session, _) = host::transient_session_address(payer.pubkey(), session_nonce);
    let create_ix = create_transient_session_ix(
        program_id,
        payer.pubkey(),
        host_config,
        session,
        session_nonce,
        payer.pubkey(),
        payer.pubkey(),
        current_slot(&svm),
        1,
    );
    let allow_ix = anchor_ix(
        program_id,
        host::accounts::AllowTransientHandle {
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record,
            session,
            host_config,
            deny_subject_record: None,
        },
        host::instruction::AllowTransientHandle {
            handle,
            capability: transient_capability(
                payer.pubkey(),
                program_id,
                acl_domain_key,
                app_account,
                false,
            ),
        },
    );

    let message = Message::new_with_blockhash(
        &[create_ix, allow_ix],
        Some(&payer.pubkey()),
        &svm.latest_blockhash(),
    );
    let account_keys = message.account_keys.clone();
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), &[&payer]).unwrap();
    let meta = svm.send_transaction(tx).unwrap();

    assert!(acl_allowed_events(&meta, &account_keys, program_id).is_empty());
}

#[test]
fn fhe_binary_op_and_bind_output_rejects_wrong_result_handle() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let dummy_rhs_account = svm.create_funded_account(1_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(7);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );
    let output_acl_record = acl_record_address(program_id, nonce_key, 1);
    let rhs_scalar = amount_plaintext(5);
    let wrong_result = [42; 32];

    let ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOpAndBindOutput {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record: dummy_rhs_account.pubkey(),
            rhs_permission_record: None,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOpAndBindOutput {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs: rhs_scalar,
            scalar: true,
            output_fhe_type: 5,
            result: wrong_result,
            output_nonce_key: nonce_key,
            output_nonce_sequence: 1,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(payer.pubkey())],
            output_public_decrypt: false,
        },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    assert!(read_acl_record(&svm, output_acl_record).is_none());
}

#[test]
fn fhe_binary_op_and_bind_output_domain_separates_result_by_acl_nonce() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let dummy_rhs_account = svm.create_funded_account(1_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(7);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );
    let rhs_scalar = amount_plaintext(5);
    let first_output = acl_record_address(program_id, nonce_key, 1);
    let second_output = acl_record_address(program_id, nonce_key, 2);
    let build_ix = |output_acl_record, output_nonce_sequence| {
        let result = current_bound_binary_handle(
            &svm,
            FheBinaryOpCode::Add,
            lhs,
            rhs_scalar,
            true,
            5,
            nonce_key,
            output_nonce_sequence,
        );
        anchor_ix(
            program_id,
            host::accounts::FheBinaryOpAndBindOutput {
                payer: payer.pubkey(),
                compute_subject: payer.pubkey(),
                app_account_authority: app_account,
                host_config,
                lhs_acl_record,
                lhs_permission_record: None,
                rhs_acl_record: dummy_rhs_account.pubkey(),
                rhs_permission_record: None,
                output_acl_record,
                system_program: system_program::ID,
                event_authority: event_authority(program_id),
                program: program_id,
            },
            host::instruction::FheBinaryOpAndBindOutput {
                op: FheBinaryOpCode::Add,
                lhs,
                rhs: rhs_scalar,
                scalar: true,
                output_fhe_type: 5,
                result,
                output_nonce_key: nonce_key,
                output_nonce_sequence,
                output_acl_domain_key: acl_domain_key,
                output_app_account: app_account,
                output_encrypted_value_label: encrypted_value_label,
                output_subjects: vec![AclSubjectEntry::user(payer.pubkey())],
                output_public_decrypt: false,
            },
        )
    };

    let instructions = vec![build_ix(first_output, 1), build_ix(second_output, 2)];
    send_many_with_signers(&mut svm, &payer.pubkey(), instructions, &[&payer]).unwrap();

    let first = read_acl_record(&svm, first_output).expect("expected first output ACL");
    let second = read_acl_record(&svm, second_output).expect("expected second output ACL");
    assert_ne!(first.handle, second.handle);
    assert_eq!(first.nonce_sequence, 1);
    assert_eq!(second.nonce_sequence, 2);
}

#[test]
fn fhe_binary_op_and_bind_output_rejects_unsupported_acl_handle_metadata() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let dummy_rhs_account = svm.create_funded_account(1_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("bad-compute-handle");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let mut lhs = input_handle_for_chain(113);
    lhs[31] = host::HANDLE_VERSION.saturating_add(1);
    let lhs_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        payer.pubkey(),
    );
    let rhs = amount_plaintext(5);
    let output_acl_record = acl_record_address(program_id, nonce_key, 1);
    let result =
        current_bound_binary_handle(&svm, FheBinaryOpCode::Add, lhs, rhs, true, 5, nonce_key, 1);
    let ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOpAndBindOutput {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record: dummy_rhs_account.pubkey(),
            rhs_permission_record: None,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOpAndBindOutput {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs,
            scalar: true,
            output_fhe_type: 5,
            result,
            output_nonce_key: nonce_key,
            output_nonce_sequence: 1,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(payer.pubkey())],
            output_public_decrypt: false,
        },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    assert!(read_acl_record(&svm, output_acl_record).is_none());
}

#[test]
fn allow_acl_subjects_rejects_wrong_handle() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [9; 32];
    let acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        payer.pubkey(),
    );
    let ix = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle: [7; 32],
            subjects: vec![AclSubjectEntry::user(payer.pubkey())],
        },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record_subjects(&record), vec![payer.pubkey()]);
}

#[test]
fn allow_acl_subjects_rejects_unallowed_authority() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let alice = Keypair::new();
    let mallory = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        mallory.pubkey(),
        mallory.pubkey(),
        mallory.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = Pubkey::new_unique();
    let encrypted_value_label = label("balance");
    let handle = input_handle_for_chain(7);
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        alice.pubkey(),
    );

    let ix = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer: mallory.pubkey(),
            authority: mallory.pubkey(),
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry::user(mallory.pubkey())],
        },
    );

    assert!(try_send(&mut svm, &mallory, ix).is_err());
    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record_subjects(&record), vec![alice.pubkey()]);
}

#[test]
fn initialize_token_account_rejects_nonzero_initial_balance() {
    let mut fixture = token_fixture();
    let carol = fixture.svm.create_funded_account(1_000_000_000).unwrap();
    let carol_token = token_account_address(
        fixture.token_program_id,
        fixture.mint.pubkey(),
        carol.pubkey(),
    );
    let carol_acl = balance_acl_record_address(
        fixture.host_program_id,
        fixture.mint.pubkey(),
        carol_token,
        0,
    );
    let ix = anchor_ix(
        fixture.token_program_id,
        token::accounts::InitializeTokenAccount {
            owner: carol.pubkey(),
            mint: fixture.mint.pubkey(),
            compute_signer: fixture.compute_signer,
            token_account: carol_token,
            acl_record: carol_acl,
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::InitializeTokenAccount { initial_balance: 1 },
    );

    assert!(try_send(&mut fixture.svm, &carol, ix).is_err());
    assert!(fixture.svm.get_account(&carol_token).is_none());
    assert!(fixture.svm.get_account(&carol_acl).is_none());
}

#[test]
fn confidential_transfer_rotates_balance_handles_and_binds_output_acl() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    let mut cleartext = CleartextBackend::default();
    cleartext.seed_cleartext(fixture.alice_initial, TypedClearValue::uint64(125));
    cleartext.seed_cleartext(fixture.bob_initial, TypedClearValue::uint64(20));
    cleartext.seed_cleartext(amount_handle, TypedClearValue::uint64(9));

    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, output, amount_handle);
    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);
    cleartext
        .ingest_transaction(&meta, &account_keys, fixture.host_program_id)
        .unwrap();
    let alice_record = read_acl_record(&fixture.svm, output.alice).expect("expected Alice ACL");
    let bob_record = read_acl_record(&fixture.svm, output.bob).expect("expected Bob ACL");
    let success_record =
        read_acl_record(&fixture.svm, output.success).expect("expected success ACL");
    let debit_candidate_record =
        read_acl_record(&fixture.svm, output.debit_candidate).expect("expected debit ACL");
    let transferred_record =
        read_acl_record(&fixture.svm, output.transferred).expect("expected transferred ACL");
    let new_alice = alice_record.handle;
    let new_bob = bob_record.handle;
    let success = success_record.handle;
    let debit_candidate = debit_candidate_record.handle;
    let transferred = transferred_record.handle;
    let events = binary_op_events(&meta, &account_keys, fixture.host_program_id);
    let ternary_events = ternary_op_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(events.len(), 4);
    assert_eq!(ternary_events.len(), 1);
    assert_eq!(events[0].version, 0);
    assert_eq!(events[0].op, FheBinaryOpCode::Ge);
    assert_eq!(events[0].subject, fixture.compute_signer.to_bytes());
    assert_eq!(events[0].lhs, fixture.alice_initial);
    assert_eq!(events[0].rhs, amount_handle);
    assert!(!events[0].scalar);
    assert_eq!(events[0].result, success);
    assert_eq!(events[1].version, 0);
    assert_eq!(events[1].op, FheBinaryOpCode::Sub);
    assert_eq!(events[1].subject, fixture.compute_signer.to_bytes());
    assert_eq!(events[1].lhs, fixture.alice_initial);
    assert_eq!(events[1].rhs, amount_handle);
    assert!(!events[1].scalar);
    assert_eq!(events[1].result, debit_candidate);
    assert_eq!(ternary_events[0].version, 0);
    assert_eq!(ternary_events[0].op, FheTernaryOpCode::IfThenElse);
    assert_eq!(ternary_events[0].control, success);
    assert_eq!(ternary_events[0].if_true, debit_candidate);
    assert_eq!(ternary_events[0].if_false, fixture.alice_initial);
    assert_eq!(ternary_events[0].result, new_alice);
    assert_eq!(events[2].op, FheBinaryOpCode::Sub);
    assert_eq!(events[2].lhs, fixture.alice_initial);
    assert_eq!(events[2].rhs, new_alice);
    assert_eq!(events[2].result, transferred);
    assert_eq!(events[3].op, FheBinaryOpCode::Add);
    assert_eq!(events[3].lhs, fixture.bob_initial);
    assert_eq!(events[3].rhs, transferred);
    assert_eq!(events[3].result, new_bob);
    let balance_events =
        balance_handle_updated_events(&meta, &account_keys, fixture.token_program_id);
    let transfer_events =
        confidential_transfer_events(&meta, &account_keys, fixture.token_program_id);
    assert_eq!(transfer_events.len(), 1);
    assert_eq!(transfer_events[0].version, 0);
    assert_eq!(transfer_events[0].mint, fixture.mint.pubkey());
    assert_eq!(transfer_events[0].from_owner, fixture.alice.pubkey());
    assert_eq!(transfer_events[0].from_token_account, fixture.alice_token);
    assert_eq!(transfer_events[0].to_owner, fixture.bob.pubkey());
    assert_eq!(transfer_events[0].to_token_account, fixture.bob_token);
    assert_eq!(transfer_events[0].transferred_handle, transferred);
    assert_eq!(
        transfer_events[0].transferred_acl_record,
        output.transferred
    );
    assert_eq!(balance_events.len(), 2);
    assert_eq!(
        balance_events[0].reason,
        BalanceHandleUpdateReason::TransferDebit
    );
    assert_eq!(balance_events[0].old_handle, fixture.alice_initial);
    assert_eq!(balance_events[0].new_handle, new_alice);
    assert_eq!(
        balance_events[0].old_acl_record,
        fixture.alice_current_compute_acl
    );
    assert_eq!(balance_events[0].new_acl_record, output.alice);
    assert_eq!(
        balance_events[1].reason,
        BalanceHandleUpdateReason::TransferCredit
    );
    assert_eq!(balance_events[1].old_handle, fixture.bob_initial);
    assert_eq!(balance_events[1].new_handle, new_bob);
    assert_eq!(
        balance_events[1].old_acl_record,
        fixture.bob_current_compute_acl
    );
    assert_eq!(balance_events[1].new_acl_record, output.bob);
    assert_eq!(
        cleartext.decrypt_cleartext(new_alice),
        Some(TypedClearValue::uint64(116))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(new_bob),
        Some(TypedClearValue::uint64(29))
    );

    let alice_account = token_account(&fixture.svm, fixture.alice_token);
    let bob_account = token_account(&fixture.svm, fixture.bob_token);
    assert_eq!(alice_account.balance_handle, new_alice);
    assert_eq!(alice_account.balance_acl_record, output.alice);
    assert_eq!(alice_account.next_balance_nonce_sequence, 2);
    assert_eq!(bob_account.balance_handle, new_bob);
    assert_eq!(bob_account.balance_acl_record, output.bob);
    assert_eq!(bob_account.next_balance_nonce_sequence, 2);
    assert_eq!(success_record.app_account, fixture.alice_token);
    assert_eq!(
        success_record.encrypted_value_label,
        token::transfer_success_label()
    );
    assert_eq!(debit_candidate_record.app_account, fixture.alice_token);
    assert_eq!(
        debit_candidate_record.encrypted_value_label,
        token::debit_candidate_label()
    );
    assert_eq!(transferred_record.app_account, fixture.alice_token);
    assert_eq!(
        transferred_record.encrypted_value_label,
        token::transferred_amount_label()
    );
    assert_eq!(
        record_subjects(&transferred_record),
        vec![
            fixture.alice.pubkey(),
            fixture.bob.pubkey(),
            fixture.compute_signer,
        ]
    );
    assert!(transferred_record
        .inline_subject_has_role(fixture.alice.pubkey(), host::ACL_ROLE_PUBLIC_DECRYPT));
    assert!(transferred_record
        .inline_subject_has_role(fixture.bob.pubkey(), host::ACL_ROLE_PUBLIC_DECRYPT));

    let disclose_transferred = request_disclose_amount_ix(
        &fixture,
        fixture.bob.pubkey(),
        output.transferred,
        transferred,
    );
    let (disclose_meta, disclose_keys) =
        send_with_meta(&mut fixture.svm, &fixture.bob, disclose_transferred);
    let transferred_record =
        read_acl_record(&fixture.svm, output.transferred).expect("expected transferred ACL");
    assert!(transferred_record.public_decrypt);
    let host_disclose_events =
        public_decrypt_allowed_events(&disclose_meta, &disclose_keys, fixture.host_program_id);
    assert_eq!(host_disclose_events.len(), 1);
    assert_eq!(host_disclose_events[0].acl_record, output.transferred);
    assert_eq!(host_disclose_events[0].handle, transferred);
    assert_eq!(
        host_disclose_events[0].authority,
        fixture.bob.pubkey().to_bytes()
    );
    let amount_disclose_events = amount_disclosure_requested_events(
        &disclose_meta,
        &disclose_keys,
        fixture.token_program_id,
    );
    assert_eq!(amount_disclose_events.len(), 1);
    assert_eq!(amount_disclose_events[0].mint, fixture.mint.pubkey());
    assert_eq!(amount_disclose_events[0].requester, fixture.bob.pubkey());
    assert_eq!(amount_disclose_events[0].handle, transferred);
    assert_eq!(amount_disclose_events[0].acl_record, output.transferred);

    assert_balance_acl(
        &fixture.svm,
        output.alice,
        fixture.mint.pubkey(),
        fixture.alice_token,
        1,
        new_alice,
        &[fixture.alice.pubkey(), fixture.compute_signer],
    );
    assert_balance_acl(
        &fixture.svm,
        output.bob,
        fixture.mint.pubkey(),
        fixture.bob_token,
        1,
        new_bob,
        &[fixture.bob.pubkey(), fixture.compute_signer],
    );
}

#[test]
fn random_bounded_transfer_amount_can_drive_confidential_transfer() {
    let mut fixture = token_fixture();
    let mut cleartext = CleartextBackend::default();
    cleartext.seed_cleartext(fixture.alice_initial, TypedClearValue::uint64(125));
    cleartext.seed_cleartext(fixture.bob_initial, TypedClearValue::uint64(20));

    let amount_nonce_sequence = DEFAULT_INPUT_NONCE_SEQUENCE;
    let amount_acl = random_amount_acl_address(
        &fixture,
        fixture.alice.pubkey(),
        ConfidentialAmountKind::Transfer,
        amount_nonce_sequence,
    );
    let upper_bound = upper_bound_be(8);
    let create_ix =
        create_random_bounded_amount_ix(&fixture, ConfidentialAmountKind::Transfer, upper_bound);
    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, create_ix);
    cleartext
        .ingest_transaction(&meta, &account_keys, fixture.host_program_id)
        .unwrap();

    let amount_record = read_acl_record(&fixture.svm, amount_acl).expect("expected amount ACL");
    let amount_handle = amount_record.handle;
    assert_eq!(amount_record.acl_domain_key, fixture.mint.pubkey());
    assert_eq!(amount_record.app_account, fixture.alice.pubkey());
    assert_eq!(
        amount_record.encrypted_value_label,
        token::transfer_amount_label()
    );
    assert_eq!(
        amount_record.nonce_key,
        token::nonce_key(
            fixture.mint.pubkey(),
            fixture.alice.pubkey(),
            token::transfer_amount_label(),
        )
    );
    assert!(amount_record.inline_subject_has_role(fixture.compute_signer, host::ACL_ROLE_USE));

    let random_events = fhe_rand_bounded_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(random_events.len(), 1);
    assert_eq!(random_events[0].subject, fixture.compute_signer.to_bytes());
    assert_eq!(random_events[0].upper_bound, upper_bound);
    assert_eq!(random_events[0].fhe_type, 5);
    assert_eq!(random_events[0].result, amount_handle);

    let created_events =
        random_amount_created_events(&meta, &account_keys, fixture.token_program_id);
    assert_eq!(created_events.len(), 1);
    assert_eq!(created_events[0].version, 0);
    assert_eq!(created_events[0].mint, fixture.mint.pubkey());
    assert_eq!(created_events[0].owner, fixture.alice.pubkey());
    assert_eq!(created_events[0].token_account, fixture.alice_token);
    assert_eq!(
        created_events[0].amount_kind,
        ConfidentialAmountKind::Transfer
    );
    assert!(created_events[0].bounded);
    assert_eq!(created_events[0].upper_bound, upper_bound);
    assert_eq!(created_events[0].handle, amount_handle);
    assert_eq!(created_events[0].acl_record, amount_acl);
    assert_eq!(created_events[0].nonce_sequence, amount_nonce_sequence);
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).next_amount_nonce_sequence,
        1
    );

    let random_amount = match cleartext
        .decrypt_cleartext(amount_handle)
        .expect("random amount cleartext")
    {
        TypedClearValue {
            fhe_type: 5,
            value: ClearValue::Uint(value),
        } => value as u64,
        other => panic!("unexpected random amount cleartext: {other:?}"),
    };
    assert!(random_amount < 8);

    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix_with_amount_acl(
        &fixture,
        fixture.alice_current_compute_acl,
        fixture.bob_current_compute_acl,
        amount_acl,
        output,
        amount_handle,
    );
    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);
    cleartext
        .ingest_transaction(&meta, &account_keys, fixture.host_program_id)
        .unwrap();
    let alice_record = read_acl_record(&fixture.svm, output.alice).expect("expected Alice ACL");
    let bob_record = read_acl_record(&fixture.svm, output.bob).expect("expected Bob ACL");

    assert_eq!(
        cleartext.decrypt_cleartext(alice_record.handle),
        Some(TypedClearValue::uint64(125 - random_amount))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(bob_record.handle),
        Some(TypedClearValue::uint64(20 + random_amount))
    );
}

#[test]
fn random_amount_nonce_allocator_advances_across_amount_kinds() {
    let mut fixture = token_fixture();
    let transfer_acl = random_amount_acl_address(
        &fixture,
        fixture.alice.pubkey(),
        ConfidentialAmountKind::Transfer,
        0,
    );
    let transfer_ix = create_random_bounded_amount_ix(
        &fixture,
        ConfidentialAmountKind::Transfer,
        upper_bound_be(8),
    );
    let (transfer_meta, transfer_keys) =
        send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);
    let transfer_events =
        random_amount_created_events(&transfer_meta, &transfer_keys, fixture.token_program_id);
    assert_eq!(transfer_events.len(), 1);
    assert_eq!(transfer_events[0].nonce_sequence, 0);
    assert_eq!(transfer_events[0].acl_record, transfer_acl);
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).next_amount_nonce_sequence,
        1
    );

    let burn_acl = random_amount_acl_address(
        &fixture,
        fixture.alice.pubkey(),
        ConfidentialAmountKind::Burn,
        1,
    );
    let burn_ix = create_random_amount_ix(&fixture, ConfidentialAmountKind::Burn);
    let (burn_meta, burn_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, burn_ix);
    let burn_events =
        random_amount_created_events(&burn_meta, &burn_keys, fixture.token_program_id);
    assert_eq!(burn_events.len(), 1);
    assert_eq!(burn_events[0].amount_kind, ConfidentialAmountKind::Burn);
    assert_eq!(burn_events[0].nonce_sequence, 1);
    assert_eq!(burn_events[0].acl_record, burn_acl);
    assert!(!burn_events[0].bounded);
    assert_eq!(burn_events[0].upper_bound, [0; 32]);
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).next_amount_nonce_sequence,
        2
    );

    let burn_record = read_acl_record(&fixture.svm, burn_acl).expect("expected burn amount ACL");
    assert_eq!(burn_record.app_account, fixture.alice.pubkey());
    assert_eq!(
        burn_record.encrypted_value_label,
        token::burn_amount_label()
    );
    assert_eq!(burn_record.nonce_sequence, 1);
    let host_rand_events = fhe_rand_events(&burn_meta, &burn_keys, fixture.host_program_id);
    assert_eq!(host_rand_events.len(), 1);
    assert_eq!(host_rand_events[0].result, burn_record.handle);
}

#[test]
fn confidential_transfer_over_balance_transfers_zero_without_underflow() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(10);
    let mut cleartext = CleartextBackend::default();
    cleartext.seed_cleartext(fixture.alice_initial, TypedClearValue::uint64(125));
    cleartext.seed_cleartext(fixture.bob_initial, TypedClearValue::uint64(20));
    cleartext.seed_cleartext(amount_handle, TypedClearValue::uint64(200));

    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, output, amount_handle);
    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);
    cleartext
        .ingest_transaction(&meta, &account_keys, fixture.host_program_id)
        .unwrap();

    let alice_record = read_acl_record(&fixture.svm, output.alice).expect("expected Alice ACL");
    let bob_record = read_acl_record(&fixture.svm, output.bob).expect("expected Bob ACL");
    let success_record =
        read_acl_record(&fixture.svm, output.success).expect("expected success ACL");
    let transferred_record =
        read_acl_record(&fixture.svm, output.transferred).expect("expected transferred ACL");

    assert_eq!(
        cleartext.decrypt_cleartext(success_record.handle),
        Some(TypedClearValue {
            fhe_type: 0,
            value: ClearValue::Uint(0),
        })
    );
    assert_eq!(
        cleartext.decrypt_cleartext(transferred_record.handle),
        Some(TypedClearValue::uint64(0))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(alice_record.handle),
        Some(TypedClearValue::uint64(125))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(bob_record.handle),
        Some(TypedClearValue::uint64(20))
    );

    let alice_account = token_account(&fixture.svm, fixture.alice_token);
    let bob_account = token_account(&fixture.svm, fixture.bob_token);
    assert_eq!(alice_account.balance_handle, alice_record.handle);
    assert_eq!(bob_account.balance_handle, bob_record.handle);
    assert_eq!(alice_account.next_balance_nonce_sequence, 2);
    assert_eq!(bob_account.next_balance_nonce_sequence, 2);
}

#[test]
fn transfer_callback_settlement_refunds_failed_callback() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(61);
    let callback_success_handle = input_handle_for_chain_with_type(62, 0);
    let mut cleartext = CleartextBackend::default();
    cleartext.seed_cleartext(fixture.alice_initial, TypedClearValue::uint64(125));
    cleartext.seed_cleartext(fixture.bob_initial, TypedClearValue::uint64(20));
    cleartext.seed_cleartext(amount_handle, TypedClearValue::uint64(9));
    cleartext.seed_cleartext(
        callback_success_handle,
        TypedClearValue {
            fhe_type: 0,
            value: ClearValue::Uint(0),
        },
    );

    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    authorize_callback_success_acl(
        &mut fixture,
        callback_success_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let predicted_sent_handle = predicted_transfer_sent_handle(&fixture, amount_handle);
    let callback_success_acl = callback_success_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE);
    let transfer_output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, transfer_output, amount_handle);
    let hook_ix = accept_transfer_receiver_hook_ix(
        &fixture,
        transfer_output.transferred,
        predicted_sent_handle,
        callback_success_acl,
        callback_success_handle,
    );
    let (transfer_meta, transfer_keys) =
        send_many_with_meta(&mut fixture.svm, &fixture.alice, vec![transfer_ix, hook_ix]);
    cleartext
        .ingest_transaction(&transfer_meta, &transfer_keys, fixture.host_program_id)
        .unwrap();
    let sent_record = read_acl_record(&fixture.svm, transfer_output.transferred)
        .expect("expected transferred ACL");
    assert_eq!(sent_record.handle, predicted_sent_handle);
    let old_alice_record =
        read_acl_record(&fixture.svm, transfer_output.alice).expect("expected Alice transfer ACL");
    let old_bob_record =
        read_acl_record(&fixture.svm, transfer_output.bob).expect("expected Bob transfer ACL");
    let settlement_output = callback_settlement_output_accounts(&fixture, sent_record.handle, 2, 2);
    let prepare_ix = prepare_transfer_callback_ix(
        &fixture,
        transfer_output.bob,
        transfer_output.transferred,
        sent_record.handle,
        callback_success_acl,
        callback_success_handle,
        settlement_output,
    );
    let (prepare_meta, prepare_keys) = send_with_meta(&mut fixture.svm, &fixture.bob, prepare_ix);
    cleartext
        .ingest_transaction(&prepare_meta, &prepare_keys, fixture.host_program_id)
        .unwrap();

    let bob_record =
        read_acl_record(&fixture.svm, settlement_output.bob).expect("expected Bob ACL");
    let requested_refund_record = read_acl_record(&fixture.svm, settlement_output.requested_refund)
        .expect("expected requested refund ACL");
    let refund_record =
        read_acl_record(&fixture.svm, settlement_output.refund).expect("expected refund ACL");
    let prepare_balance_events =
        balance_handle_updated_events(&prepare_meta, &prepare_keys, fixture.token_program_id);

    assert_eq!(
        cleartext.decrypt_cleartext(requested_refund_record.handle),
        Some(TypedClearValue::uint64(9))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(refund_record.handle),
        Some(TypedClearValue::uint64(9))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(bob_record.handle),
        Some(TypedClearValue::uint64(20))
    );
    let prepared = read_transfer_callback_settlement(&fixture.svm, settlement_output.settlement)
        .expect("expected prepared callback settlement");
    assert_eq!(prepared.status, token::CALLBACK_SETTLEMENT_PREPARED);
    assert_eq!(prepared.to_balance_handle, bob_record.handle);
    assert_eq!(prepared.refund_handle, refund_record.handle);
    assert_eq!(prepare_balance_events.len(), 1);
    assert_eq!(prepare_balance_events[0].mint, fixture.mint.pubkey());
    assert_eq!(prepare_balance_events[0].owner, fixture.bob.pubkey());
    assert_eq!(prepare_balance_events[0].token_account, fixture.bob_token);
    assert_eq!(prepare_balance_events[0].old_handle, old_bob_record.handle);
    assert_eq!(
        prepare_balance_events[0].old_acl_record,
        transfer_output.bob
    );
    assert_eq!(prepare_balance_events[0].new_handle, bob_record.handle);
    assert_eq!(
        prepare_balance_events[0].new_acl_record,
        settlement_output.bob
    );
    assert_eq!(
        prepare_balance_events[0].reason,
        BalanceHandleUpdateReason::TransferCallbackRefundDebit
    );

    let finalize_ix = finalize_transfer_callback_ix(
        &fixture,
        transfer_output.alice,
        transfer_output.transferred,
        settlement_output,
    );
    let (finalize_meta, finalize_keys) =
        send_with_meta(&mut fixture.svm, &fixture.bob, finalize_ix);
    cleartext
        .ingest_transaction(&finalize_meta, &finalize_keys, fixture.host_program_id)
        .unwrap();

    let alice_record =
        read_acl_record(&fixture.svm, settlement_output.alice).expect("expected Alice ACL");
    let final_transferred_record = read_acl_record(&fixture.svm, settlement_output.transferred)
        .expect("expected final transfer ACL");
    let finalize_balance_events =
        balance_handle_updated_events(&finalize_meta, &finalize_keys, fixture.token_program_id);
    let finalize_transfer_events =
        confidential_transfer_events(&finalize_meta, &finalize_keys, fixture.token_program_id);
    assert_eq!(
        cleartext.decrypt_cleartext(alice_record.handle),
        Some(TypedClearValue::uint64(125))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(final_transferred_record.handle),
        Some(TypedClearValue::uint64(0))
    );
    assert_eq!(finalize_transfer_events.len(), 1);
    assert_eq!(finalize_transfer_events[0].mint, fixture.mint.pubkey());
    assert_eq!(finalize_transfer_events[0].from_owner, fixture.bob.pubkey());
    assert_eq!(
        finalize_transfer_events[0].from_token_account,
        fixture.bob_token
    );
    assert_eq!(finalize_transfer_events[0].to_owner, fixture.alice.pubkey());
    assert_eq!(
        finalize_transfer_events[0].to_token_account,
        fixture.alice_token
    );
    assert_eq!(
        finalize_transfer_events[0].transferred_handle,
        refund_record.handle
    );
    assert_eq!(
        finalize_transfer_events[0].transferred_acl_record,
        settlement_output.refund
    );
    assert_eq!(finalize_balance_events.len(), 1);
    assert_eq!(finalize_balance_events[0].mint, fixture.mint.pubkey());
    assert_eq!(finalize_balance_events[0].owner, fixture.alice.pubkey());
    assert_eq!(
        finalize_balance_events[0].token_account,
        fixture.alice_token
    );
    assert_eq!(
        finalize_balance_events[0].old_handle,
        old_alice_record.handle
    );
    assert_eq!(
        finalize_balance_events[0].old_acl_record,
        transfer_output.alice
    );
    assert_eq!(finalize_balance_events[0].new_handle, alice_record.handle);
    assert_eq!(
        finalize_balance_events[0].new_acl_record,
        settlement_output.alice
    );
    assert_eq!(
        finalize_balance_events[0].reason,
        BalanceHandleUpdateReason::TransferCallbackRefundCredit
    );

    let settlement = read_transfer_callback_settlement(&fixture.svm, settlement_output.settlement)
        .expect("expected callback settlement");
    assert_eq!(settlement.status, token::CALLBACK_SETTLEMENT_FINALIZED);
    assert_eq!(settlement.sent_handle, sent_record.handle);
    assert_eq!(settlement.sent_acl_record, transfer_output.transferred);
    assert_eq!(settlement.callback_success_handle, callback_success_handle);
    assert_eq!(settlement.refund_handle, refund_record.handle);
    assert_eq!(settlement.from_balance_handle, alice_record.handle);
    assert_eq!(
        settlement.transferred_handle,
        final_transferred_record.handle
    );

    let replay_prepare_ix = prepare_transfer_callback_ix(
        &fixture,
        transfer_output.bob,
        transfer_output.transferred,
        sent_record.handle,
        callback_success_acl,
        callback_success_handle,
        settlement_output,
    );
    assert!(try_send(&mut fixture.svm, &fixture.bob, replay_prepare_ix).is_err());
    let replay_finalize_ix = finalize_transfer_callback_ix(
        &fixture,
        settlement_output.alice,
        transfer_output.transferred,
        settlement_output,
    );
    assert!(try_send(&mut fixture.svm, &fixture.bob, replay_finalize_ix).is_err());
}

#[test]
fn transfer_callback_refund_settlement_does_not_require_recipient_signer() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(111);
    let callback_success_handle = input_handle_for_chain_with_type(112, 0);
    let mut cleartext = CleartextBackend::default();
    cleartext.seed_cleartext(fixture.alice_initial, TypedClearValue::uint64(125));
    cleartext.seed_cleartext(fixture.bob_initial, TypedClearValue::uint64(20));
    cleartext.seed_cleartext(amount_handle, TypedClearValue::uint64(9));
    cleartext.seed_cleartext(
        callback_success_handle,
        TypedClearValue {
            fhe_type: 0,
            value: ClearValue::Uint(0),
        },
    );

    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    authorize_callback_success_acl(
        &mut fixture,
        callback_success_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let predicted_sent_handle = predicted_transfer_sent_handle(&fixture, amount_handle);
    let callback_success_acl = callback_success_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE);
    let transfer_output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, transfer_output, amount_handle);
    let hook_ix = accept_transfer_receiver_hook_ix(
        &fixture,
        transfer_output.transferred,
        predicted_sent_handle,
        callback_success_acl,
        callback_success_handle,
    );
    let (transfer_meta, transfer_keys) =
        send_many_with_meta(&mut fixture.svm, &fixture.alice, vec![transfer_ix, hook_ix]);
    cleartext
        .ingest_transaction(&transfer_meta, &transfer_keys, fixture.host_program_id)
        .unwrap();
    let sent_record = read_acl_record(&fixture.svm, transfer_output.transferred)
        .expect("expected transferred ACL");
    assert_eq!(sent_record.handle, predicted_sent_handle);

    let settlement_output = callback_settlement_output_accounts(&fixture, sent_record.handle, 2, 2);
    let prepare_ix = prepare_transfer_callback_ix_with_payer(
        &fixture,
        fixture.alice.pubkey(),
        transfer_output.bob,
        transfer_output.transferred,
        sent_record.handle,
        callback_success_acl,
        callback_success_handle,
        settlement_output,
    );
    let (prepare_meta, prepare_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, prepare_ix);
    cleartext
        .ingest_transaction(&prepare_meta, &prepare_keys, fixture.host_program_id)
        .unwrap();

    let finalize_ix = finalize_transfer_callback_ix_with_payer(
        &fixture,
        fixture.alice.pubkey(),
        transfer_output.alice,
        transfer_output.transferred,
        settlement_output,
    );
    let (finalize_meta, finalize_keys) =
        send_with_meta(&mut fixture.svm, &fixture.alice, finalize_ix);
    cleartext
        .ingest_transaction(&finalize_meta, &finalize_keys, fixture.host_program_id)
        .unwrap();

    let bob_record =
        read_acl_record(&fixture.svm, settlement_output.bob).expect("expected Bob ACL");
    let alice_record =
        read_acl_record(&fixture.svm, settlement_output.alice).expect("expected Alice ACL");
    let final_transferred_record = read_acl_record(&fixture.svm, settlement_output.transferred)
        .expect("expected final transfer ACL");
    let settlement = read_transfer_callback_settlement(&fixture.svm, settlement_output.settlement)
        .expect("expected finalized callback settlement");

    assert_eq!(settlement.status, token::CALLBACK_SETTLEMENT_FINALIZED);
    assert_eq!(
        cleartext.decrypt_cleartext(bob_record.handle),
        Some(TypedClearValue::uint64(20))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(alice_record.handle),
        Some(TypedClearValue::uint64(125))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(final_transferred_record.handle),
        Some(TypedClearValue::uint64(0))
    );
}

#[test]
fn transfer_callback_settlement_rejects_wrong_bump_or_length() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(73);
    let callback_success_handle = input_handle_for_chain_with_type(74, 0);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    authorize_callback_success_acl(
        &mut fixture,
        callback_success_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let predicted_sent_handle = predicted_transfer_sent_handle(&fixture, amount_handle);
    let callback_success_acl = callback_success_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE);
    let transfer_output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, transfer_output, amount_handle);
    let hook_ix = accept_transfer_receiver_hook_ix(
        &fixture,
        transfer_output.transferred,
        predicted_sent_handle,
        callback_success_acl,
        callback_success_handle,
    );
    send_many_with_signers(
        &mut fixture.svm,
        &fixture.alice.pubkey(),
        vec![transfer_ix, hook_ix],
        &[&fixture.alice],
    )
    .unwrap();
    let sent_record = read_acl_record(&fixture.svm, transfer_output.transferred)
        .expect("expected transferred ACL");
    assert_eq!(sent_record.handle, predicted_sent_handle);
    let settlement_output = callback_settlement_output_accounts(&fixture, sent_record.handle, 2, 2);
    let prepare_ix = prepare_transfer_callback_ix(
        &fixture,
        transfer_output.bob,
        transfer_output.transferred,
        sent_record.handle,
        callback_success_acl,
        callback_success_handle,
        settlement_output,
    );
    send(&mut fixture.svm, &fixture.bob, prepare_ix);

    let alice_before = token_account(&fixture.svm, fixture.alice_token);
    let mut settlement =
        read_transfer_callback_settlement(&fixture.svm, settlement_output.settlement)
            .expect("expected prepared callback settlement");
    let bump = settlement.bump;
    settlement.bump = bump.wrapping_add(1);
    seed_transfer_callback_settlement(
        &mut fixture.svm,
        fixture.token_program_id,
        settlement_output.settlement,
        &settlement,
        0,
    );
    let finalize_ix = finalize_transfer_callback_ix(
        &fixture,
        transfer_output.alice,
        transfer_output.transferred,
        settlement_output,
    );
    assert!(try_send(&mut fixture.svm, &fixture.bob, finalize_ix).is_err());
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_handle,
        alice_before.balance_handle
    );
    assert!(read_acl_record(&fixture.svm, settlement_output.alice).is_none());

    settlement.bump = bump;
    seed_transfer_callback_settlement(
        &mut fixture.svm,
        fixture.token_program_id,
        settlement_output.settlement,
        &settlement,
        1,
    );
    let finalize_ix = finalize_transfer_callback_ix(
        &fixture,
        transfer_output.alice,
        transfer_output.transferred,
        settlement_output,
    );
    assert!(try_send(&mut fixture.svm, &fixture.bob, finalize_ix).is_err());
    assert_eq!(
        fixture
            .svm
            .get_account(&settlement_output.settlement)
            .expect("expected callback settlement")
            .data
            .len(),
        8 + token::TransferCallbackSettlement::SPACE + 1
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_handle,
        alice_before.balance_handle
    );
    assert!(read_acl_record(&fixture.svm, settlement_output.alice).is_none());
}

#[test]
fn transfer_receiver_hook_validates_callback_return_before_settlement() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(63);
    let callback_success_handle = input_handle_for_chain_with_type(65, 0);
    let mut cleartext = CleartextBackend::default();
    cleartext.seed_cleartext(fixture.alice_initial, TypedClearValue::uint64(125));
    cleartext.seed_cleartext(fixture.bob_initial, TypedClearValue::uint64(20));
    cleartext.seed_cleartext(amount_handle, TypedClearValue::uint64(9));
    cleartext.seed_cleartext(
        callback_success_handle,
        TypedClearValue {
            fhe_type: 0,
            value: ClearValue::Uint(0),
        },
    );

    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    authorize_callback_success_acl(
        &mut fixture,
        callback_success_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let callback_success_acl = callback_success_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE);
    let predicted_sent_handle = predicted_transfer_sent_handle(&fixture, amount_handle);
    let transfer_output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, transfer_output, amount_handle);

    let receiver_data = receiver::instruction::AcceptConfidentialTransfer {
        mint: fixture.mint.pubkey(),
        from_token_account: fixture.alice_token,
        to_token_account: fixture.bob_token,
        sent_handle: predicted_sent_handle,
        sent_acl_record: transfer_output.transferred,
        callback_success_handle,
        callback_success_acl_record: callback_success_acl,
    }
    .data();
    let hook_ix = call_transfer_receiver_ix(
        &fixture,
        transfer_output.transferred,
        predicted_sent_handle,
        callback_success_acl,
        callback_success_handle,
        fixture.receiver_program_id,
        receiver_data,
    );
    let (transfer_meta, transfer_keys) =
        send_many_with_meta(&mut fixture.svm, &fixture.alice, vec![transfer_ix, hook_ix]);
    cleartext
        .ingest_transaction(&transfer_meta, &transfer_keys, fixture.host_program_id)
        .unwrap();
    let sent_record = read_acl_record(&fixture.svm, transfer_output.transferred)
        .expect("expected transferred ACL");
    assert_eq!(sent_record.handle, predicted_sent_handle);
    let hook_record_address =
        token::transfer_receiver_hook_address(fixture.mint.pubkey(), sent_record.handle).0;
    let hook_record = read_transfer_receiver_hook_call(&fixture.svm, hook_record_address)
        .expect("expected receiver hook marker");
    assert_eq!(hook_record.mint, fixture.mint.pubkey());
    assert_eq!(hook_record.from_token_account, fixture.alice_token);
    assert_eq!(hook_record.to_token_account, fixture.bob_token);
    assert_eq!(hook_record.sent_handle, sent_record.handle);
    assert_eq!(hook_record.sent_acl_record, transfer_output.transferred);
    assert_eq!(hook_record.callback_success_handle, callback_success_handle);
    assert_eq!(
        hook_record.callback_success_acl_record,
        callback_success_acl
    );
    assert_eq!(hook_record.receiver_program, fixture.receiver_program_id);
    assert_eq!(hook_record.caller, fixture.alice.pubkey());

    let settlement_output = callback_settlement_output_accounts(&fixture, sent_record.handle, 2, 2);
    let prepare_ix = prepare_transfer_callback_ix(
        &fixture,
        transfer_output.bob,
        transfer_output.transferred,
        sent_record.handle,
        callback_success_acl,
        callback_success_handle,
        settlement_output,
    );
    let (prepare_meta, prepare_keys) = send_with_meta(&mut fixture.svm, &fixture.bob, prepare_ix);
    cleartext
        .ingest_transaction(&prepare_meta, &prepare_keys, fixture.host_program_id)
        .unwrap();

    let settlement = read_transfer_callback_settlement(&fixture.svm, settlement_output.settlement)
        .expect("expected prepared callback settlement");
    assert_eq!(settlement.status, token::CALLBACK_SETTLEMENT_PREPARED);
    assert_eq!(settlement.callback_success_handle, callback_success_handle);
    assert_eq!(settlement.callback_success_acl_record, callback_success_acl);
}

#[test]
fn transfer_receiver_hook_is_one_shot_per_sent_handle() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(94);
    let callback_success_handle = input_handle_for_chain_with_type(95, 0);

    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    authorize_callback_success_acl(
        &mut fixture,
        callback_success_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let callback_success_acl = callback_success_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE);
    let predicted_sent_handle = predicted_transfer_sent_handle(&fixture, amount_handle);
    let transfer_output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, transfer_output, amount_handle);

    let receiver_data = receiver::instruction::AcceptConfidentialTransfer {
        mint: fixture.mint.pubkey(),
        from_token_account: fixture.alice_token,
        to_token_account: fixture.bob_token,
        sent_handle: predicted_sent_handle,
        sent_acl_record: transfer_output.transferred,
        callback_success_handle,
        callback_success_acl_record: callback_success_acl,
    }
    .data();
    let hook_ix = call_transfer_receiver_ix(
        &fixture,
        transfer_output.transferred,
        predicted_sent_handle,
        callback_success_acl,
        callback_success_handle,
        fixture.receiver_program_id,
        receiver_data.clone(),
    );
    send_many_with_signers(
        &mut fixture.svm,
        &fixture.alice.pubkey(),
        vec![transfer_ix, hook_ix],
        &[&fixture.alice],
    )
    .unwrap();
    let sent_record = read_acl_record(&fixture.svm, transfer_output.transferred)
        .expect("expected transferred ACL");
    assert_eq!(sent_record.handle, predicted_sent_handle);

    let hook_record =
        token::transfer_receiver_hook_address(fixture.mint.pubkey(), sent_record.handle).0;
    let data_before = fixture
        .svm
        .get_account(&hook_record)
        .expect("expected receiver hook marker")
        .data;
    let duplicate_hook_ix = call_transfer_receiver_ix(
        &fixture,
        transfer_output.transferred,
        sent_record.handle,
        callback_success_acl,
        callback_success_handle,
        fixture.receiver_program_id,
        receiver_data,
    );

    assert!(try_send(&mut fixture.svm, &fixture.alice, duplicate_hook_ix).is_err());
    let data_after = fixture
        .svm
        .get_account(&hook_record)
        .expect("expected receiver hook marker")
        .data;
    assert_eq!(data_after, data_before);
}

#[test]
fn transfer_callback_prepare_requires_receiver_hook_marker() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(96);
    let callback_success_handle = input_handle_for_chain_with_type(97, 0);

    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    authorize_callback_success_acl(
        &mut fixture,
        callback_success_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let callback_success_acl = callback_success_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE);
    let transfer_output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, transfer_output, amount_handle);
    send(&mut fixture.svm, &fixture.alice, transfer_ix);
    let sent_record = read_acl_record(&fixture.svm, transfer_output.transferred)
        .expect("expected transferred ACL");

    let settlement_output = callback_settlement_output_accounts(&fixture, sent_record.handle, 2, 2);
    let prepare_ix = prepare_transfer_callback_ix(
        &fixture,
        transfer_output.bob,
        transfer_output.transferred,
        sent_record.handle,
        callback_success_acl,
        callback_success_handle,
        settlement_output,
    );

    assert!(try_send(&mut fixture.svm, &fixture.bob, prepare_ix).is_err());
    assert!(
        read_transfer_callback_settlement(&fixture.svm, settlement_output.settlement).is_none()
    );
    assert!(read_acl_record(&fixture.svm, settlement_output.zero).is_none());
}

#[test]
fn transfer_receiver_hook_rejects_standalone_prior_transfer() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(98);
    let callback_success_handle = input_handle_for_chain_with_type(99, 0);

    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    authorize_callback_success_acl(
        &mut fixture,
        callback_success_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let callback_success_acl = callback_success_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE);
    let transfer_output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, transfer_output, amount_handle);
    send(&mut fixture.svm, &fixture.alice, transfer_ix);
    let sent_record = read_acl_record(&fixture.svm, transfer_output.transferred)
        .expect("expected transferred ACL");

    let hook_ix = accept_transfer_receiver_hook_ix(
        &fixture,
        transfer_output.transferred,
        sent_record.handle,
        callback_success_acl,
        callback_success_handle,
    );

    assert!(try_send(&mut fixture.svm, &fixture.alice, hook_ix).is_err());
    let hook_record =
        token::transfer_receiver_hook_address(fixture.mint.pubkey(), sent_record.handle).0;
    assert!(fixture.svm.get_account(&hook_record).is_none());
}

#[test]
fn transfer_receiver_hook_rejects_mismatched_callback_return() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(66);
    let callback_success_handle = input_handle_for_chain_with_type(67, 0);

    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    authorize_callback_success_acl(
        &mut fixture,
        callback_success_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let callback_success_acl = callback_success_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE);
    let predicted_sent_handle = predicted_transfer_sent_handle(&fixture, amount_handle);
    let transfer_output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, transfer_output, amount_handle);

    let mut wrong_callback_success_handle = callback_success_handle;
    wrong_callback_success_handle[0] ^= 0xff;
    let receiver_data = receiver::instruction::AcceptConfidentialTransfer {
        mint: fixture.mint.pubkey(),
        from_token_account: fixture.alice_token,
        to_token_account: fixture.bob_token,
        sent_handle: predicted_sent_handle,
        sent_acl_record: transfer_output.transferred,
        callback_success_handle: wrong_callback_success_handle,
        callback_success_acl_record: callback_success_acl,
    }
    .data();
    let hook_ix = call_transfer_receiver_ix(
        &fixture,
        transfer_output.transferred,
        predicted_sent_handle,
        callback_success_acl,
        callback_success_handle,
        fixture.receiver_program_id,
        receiver_data,
    );

    assert!(try_send_many(&mut fixture.svm, &fixture.alice, vec![transfer_ix, hook_ix]).is_err());
    let hook_record =
        token::transfer_receiver_hook_address(fixture.mint.pubkey(), predicted_sent_handle).0;
    assert!(fixture.svm.get_account(&hook_record).is_none());
}

#[test]
fn transfer_receiver_hook_rejects_extra_accounts_for_empty_receiver_contract() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(72);
    let callback_success_handle = input_handle_for_chain_with_type(73, 0);

    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    authorize_callback_success_acl(
        &mut fixture,
        callback_success_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let callback_success_acl = callback_success_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE);
    let predicted_sent_handle = predicted_transfer_sent_handle(&fixture, amount_handle);
    let transfer_output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, transfer_output, amount_handle);

    let receiver_data = receiver::instruction::AcceptConfidentialTransfer {
        mint: fixture.mint.pubkey(),
        from_token_account: fixture.alice_token,
        to_token_account: fixture.bob_token,
        sent_handle: predicted_sent_handle,
        sent_acl_record: transfer_output.transferred,
        callback_success_handle,
        callback_success_acl_record: callback_success_acl,
    }
    .data();
    let mut hook_ix = call_transfer_receiver_ix(
        &fixture,
        transfer_output.transferred,
        predicted_sent_handle,
        callback_success_acl,
        callback_success_handle,
        fixture.receiver_program_id,
        receiver_data,
    );
    let unexpected_account = fixture.svm.create_funded_account(1_000_000).unwrap();
    hook_ix.accounts.push(AccountMeta::new_readonly(
        unexpected_account.pubkey(),
        false,
    ));

    assert!(try_send_many(&mut fixture.svm, &fixture.alice, vec![transfer_ix, hook_ix]).is_err());
}

#[test]
fn transfer_callback_settlement_keeps_successful_callback_transfer() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(63);
    let callback_success_handle = input_handle_for_chain_with_type(64, 0);
    let mut cleartext = CleartextBackend::default();
    cleartext.seed_cleartext(fixture.alice_initial, TypedClearValue::uint64(125));
    cleartext.seed_cleartext(fixture.bob_initial, TypedClearValue::uint64(20));
    cleartext.seed_cleartext(amount_handle, TypedClearValue::uint64(9));
    cleartext.seed_cleartext(
        callback_success_handle,
        TypedClearValue {
            fhe_type: 0,
            value: ClearValue::Uint(1),
        },
    );

    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    authorize_callback_success_acl(
        &mut fixture,
        callback_success_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let predicted_sent_handle = predicted_transfer_sent_handle(&fixture, amount_handle);
    let callback_success_acl = callback_success_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE);
    let transfer_output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, transfer_output, amount_handle);
    let hook_ix = accept_transfer_receiver_hook_ix(
        &fixture,
        transfer_output.transferred,
        predicted_sent_handle,
        callback_success_acl,
        callback_success_handle,
    );
    let (transfer_meta, transfer_keys) =
        send_many_with_meta(&mut fixture.svm, &fixture.alice, vec![transfer_ix, hook_ix]);
    cleartext
        .ingest_transaction(&transfer_meta, &transfer_keys, fixture.host_program_id)
        .unwrap();
    let sent_record = read_acl_record(&fixture.svm, transfer_output.transferred)
        .expect("expected transferred ACL");
    assert_eq!(sent_record.handle, predicted_sent_handle);
    let settlement_output = callback_settlement_output_accounts(&fixture, sent_record.handle, 2, 2);
    let prepare_ix = prepare_transfer_callback_ix(
        &fixture,
        transfer_output.bob,
        transfer_output.transferred,
        sent_record.handle,
        callback_success_acl,
        callback_success_handle,
        settlement_output,
    );
    let (prepare_meta, prepare_keys) = send_with_meta(&mut fixture.svm, &fixture.bob, prepare_ix);
    cleartext
        .ingest_transaction(&prepare_meta, &prepare_keys, fixture.host_program_id)
        .unwrap();

    let bob_record =
        read_acl_record(&fixture.svm, settlement_output.bob).expect("expected Bob ACL");
    let refund_record =
        read_acl_record(&fixture.svm, settlement_output.refund).expect("expected refund ACL");
    assert_eq!(
        cleartext.decrypt_cleartext(refund_record.handle),
        Some(TypedClearValue::uint64(0))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(bob_record.handle),
        Some(TypedClearValue::uint64(29))
    );
    assert_eq!(
        read_transfer_callback_settlement(&fixture.svm, settlement_output.settlement)
            .expect("expected prepared callback settlement")
            .status,
        token::CALLBACK_SETTLEMENT_PREPARED
    );

    let finalize_ix = finalize_transfer_callback_ix(
        &fixture,
        transfer_output.alice,
        transfer_output.transferred,
        settlement_output,
    );
    let (finalize_meta, finalize_keys) =
        send_with_meta(&mut fixture.svm, &fixture.bob, finalize_ix);
    cleartext
        .ingest_transaction(&finalize_meta, &finalize_keys, fixture.host_program_id)
        .unwrap();

    let alice_record =
        read_acl_record(&fixture.svm, settlement_output.alice).expect("expected Alice ACL");
    let final_transferred_record = read_acl_record(&fixture.svm, settlement_output.transferred)
        .expect("expected final transfer ACL");
    assert_eq!(
        cleartext.decrypt_cleartext(final_transferred_record.handle),
        Some(TypedClearValue::uint64(9))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(alice_record.handle),
        Some(TypedClearValue::uint64(116))
    );
    assert_eq!(
        read_transfer_callback_settlement(&fixture.svm, settlement_output.settlement)
            .expect("expected finalized callback settlement")
            .status,
        token::CALLBACK_SETTLEMENT_FINALIZED
    );
}

#[test]
fn confidential_self_transfer_is_no_op() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let mut output = transfer_output_accounts(&fixture, 1);
    output.bob = output.alice;
    let ix = self_transfer_ix(&fixture, output, amount_handle);

    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, ix);

    assert!(binary_op_events(&meta, &account_keys, fixture.host_program_id).is_empty());
    assert!(ternary_op_events(&meta, &account_keys, fixture.host_program_id).is_empty());
    assert!(
        balance_handle_updated_events(&meta, &account_keys, fixture.token_program_id).is_empty()
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_handle,
        fixture.alice_initial
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_acl_record,
        fixture.alice_current_compute_acl
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).next_balance_nonce_sequence,
        1
    );
    assert!(read_acl_record(&fixture.svm, output.alice).is_none());
    assert!(read_acl_record(&fixture.svm, output.bob).is_none());
    assert!(read_acl_record(&fixture.svm, output.success).is_none());
    assert!(read_acl_record(&fixture.svm, output.debit_candidate).is_none());
    assert!(read_acl_record(&fixture.svm, output.transferred).is_none());
}

#[test]
fn confidential_self_transfer_rejects_noncanonical_unused_output_accounts() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(94);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = transfer_output_accounts(&fixture, 1);
    let ix = self_transfer_ix(&fixture, output, amount_handle);

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_handle,
        fixture.alice_initial
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_acl_record,
        fixture.alice_current_compute_acl
    );
    assert!(read_acl_record(&fixture.svm, output.alice).is_none());
    assert!(read_acl_record(&fixture.svm, output.bob).is_none());
    assert!(read_acl_record(&fixture.svm, output.success).is_none());
    assert!(read_acl_record(&fixture.svm, output.debit_candidate).is_none());
    assert!(read_acl_record(&fixture.svm, output.transferred).is_none());
}

#[test]
fn user_decrypt_model_uses_acl_domain_key_and_acl_record_authentication() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let first_output = transfer_output_accounts(&fixture, 1);
    let first_ix = transfer_ix(&fixture, first_output, amount_handle);
    send(&mut fixture.svm, &fixture.alice, first_ix);
    let first_alice = read_acl_record(&fixture.svm, first_output.alice)
        .expect("expected first Alice ACL")
        .handle;

    let second_amount_handle = input_handle_for_chain(8);
    authorize_input_compute_acl(&mut fixture, second_amount_handle, 1);
    let second_output = transfer_output_accounts(&fixture, 2);
    let second_ix = transfer_ix_with_current_acl_and_amount_nonce(
        &fixture,
        first_output.alice,
        first_output.bob,
        second_output,
        second_amount_handle,
        1,
    );
    send(&mut fixture.svm, &fixture.alice, second_ix);
    let second_alice = read_acl_record(&fixture.svm, second_output.alice)
        .expect("expected second Alice ACL")
        .handle;

    let current_request =
        signed_current_balance_user_decrypt_request(&fixture, fixture.alice_token, &fixture.alice);
    assert_eq!(current_request.handles[0].handle, second_alice);
    assert_eq!(current_request.handles[0].acl_record, second_output.alice);
    assert!(kms_like_user_decrypt_check(&fixture.svm, &current_request));

    let historical_request = signed_user_decrypt_request(
        &fixture,
        &fixture.alice,
        vec![UserDecryptHandleEntry {
            handle: first_alice,
            owner: fixture.alice.pubkey(),
            acl_record: first_output.alice,
        }],
    );
    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &historical_request
    ));

    let mut wrong_domain = current_request.clone();
    wrong_domain.authorization.allowed_acl_domain_keys = vec![Pubkey::new_unique()];
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &wrong_domain));

    let mut wrong_signature = current_request.clone();
    wrong_signature.signature = fixture
        .bob
        .sign_message(&authorization_payload_bytes(&wrong_signature.authorization));
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &wrong_signature));

    let mut wrong_acl_locator = current_request.clone();
    wrong_acl_locator.handles[0].acl_record = second_output.bob;
    assert!(!kms_like_user_decrypt_check(
        &fixture.svm,
        &wrong_acl_locator
    ));

    let mut wrong_handle = current_request;
    wrong_handle.handles[0].handle = [99; 32];
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &wrong_handle));
}

#[test]
fn public_decrypt_model_uses_acl_record_flag() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, output, amount_handle);
    send(&mut fixture.svm, &fixture.alice, transfer_ix);
    let alice_handle = read_acl_record(&fixture.svm, output.alice)
        .expect("expected Alice ACL")
        .handle;

    let entry = PublicDecryptHandleEntry {
        handle: alice_handle,
        acl_record: output.alice,
    };
    assert!(!kms_like_public_decrypt_check(&fixture.svm, &[entry]));

    let before = read_acl_record(&fixture.svm, output.alice).expect("expected ACL record");
    send(
        &mut fixture.svm,
        &fixture.alice,
        allow_for_decryption_ix(
            fixture.host_program_id,
            fixture.alice.pubkey(),
            output.alice,
            alice_handle,
        ),
    );

    let record = read_acl_record(&fixture.svm, output.alice).expect("expected ACL record");
    assert_eq!(record.handle, before.handle);
    assert_eq!(record.nonce_key, before.nonce_key);
    assert_eq!(record.nonce_sequence, before.nonce_sequence);
    assert_eq!(record.acl_domain_key, before.acl_domain_key);
    assert_eq!(record.app_account, before.app_account);
    assert_eq!(record.encrypted_value_label, before.encrypted_value_label);
    assert_eq!(record_subjects(&record), record_subjects(&before));
    assert!(record.public_decrypt);
    assert!(kms_like_public_decrypt_check(&fixture.svm, &[entry]));

    assert!(!kms_like_public_decrypt_check(
        &fixture.svm,
        &[PublicDecryptHandleEntry {
            handle: [99; 32],
            acl_record: output.alice,
        }]
    ));
    assert!(!kms_like_public_decrypt_check(
        &fixture.svm,
        &[PublicDecryptHandleEntry {
            handle: [4; 32],
            acl_record: output.bob,
        }]
    ));
}

#[test]
fn allow_for_decryption_rejects_unsupported_acl_handle_metadata() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let _host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("bad-public-handle");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let mut handle = input_handle_for_chain(114);
    handle[31] = host::HANDLE_VERSION.saturating_add(1);
    let acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        payer.pubkey(),
    );
    let ix = allow_for_decryption_ix(program_id, payer.pubkey(), acl_record, handle);

    assert!(try_send(&mut svm, &payer, ix).is_err());
    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert!(!record.public_decrypt);
}

#[test]
fn material_commitment_adds_decryptability_witness_for_public_decrypt() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(109);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, output, amount_handle);
    send(&mut fixture.svm, &fixture.alice, transfer_ix);
    let alice_handle = read_acl_record(&fixture.svm, output.alice)
        .expect("expected Alice ACL")
        .handle;
    let material_commitment = host::handle_material_address(output.alice).0;
    let entry = PublicDecryptWithMaterialEntry {
        handle: alice_handle,
        acl_record: output.alice,
        material_commitment,
    };

    assert!(!kms_like_public_decrypt_with_material_check(
        &fixture.svm,
        &[entry]
    ));
    send(
        &mut fixture.svm,
        &fixture.alice,
        allow_for_decryption_ix(
            fixture.host_program_id,
            fixture.alice.pubkey(),
            output.alice,
            alice_handle,
        ),
    );
    assert!(!kms_like_public_decrypt_with_material_check(
        &fixture.svm,
        &[entry]
    ));

    let key_id = [21; 32];
    let ciphertext_digest = [22; 32];
    let sns_ciphertext_digest = [23; 32];
    let coprocessor_set_digest = [24; 32];
    let commit_ix = commit_handle_material_ix(
        fixture.host_program_id,
        fixture.alice.pubkey(),
        fixture.verifier.pubkey(),
        fixture.host_config,
        output.alice,
        material_commitment,
        key_id,
        ciphertext_digest,
        sns_ciphertext_digest,
        coprocessor_set_digest,
    );
    let (meta, account_keys) = send_many_with_signers_with_meta(
        &mut fixture.svm,
        &fixture.alice.pubkey(),
        vec![commit_ix.clone()],
        &[&fixture.alice, &fixture.verifier],
    );

    let material =
        read_material_commitment(&fixture.svm, material_commitment).expect("expected material");
    assert_eq!(material.acl_record, output.alice);
    assert_eq!(material.handle, alice_handle);
    assert_eq!(material.key_id, key_id);
    assert_eq!(material.ciphertext_digest, ciphertext_digest);
    assert_eq!(material.sns_ciphertext_digest, sns_ciphertext_digest);
    assert_eq!(material.coprocessor_set_digest, coprocessor_set_digest);
    assert_eq!(material.state, host::HANDLE_MATERIAL_STATE_COMMITTED);
    assert_eq!(
        material.material_commitment_hash,
        host::handle_material_commitment_hash(
            material_commitment,
            output.alice,
            key_id,
            ciphertext_digest,
            sns_ciphertext_digest,
            coprocessor_set_digest,
        )
    );
    assert_eq!(material.created_slot, current_slot(&fixture.svm));

    let committed_events =
        handle_material_committed_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(committed_events.len(), 1);
    assert_eq!(committed_events[0].material_commitment, material_commitment);
    assert_eq!(committed_events[0].acl_record, output.alice);
    assert_eq!(committed_events[0].handle, alice_handle);
    assert_eq!(committed_events[0].key_id, key_id);
    assert_eq!(committed_events[0].ciphertext_digest, ciphertext_digest);
    assert_eq!(
        committed_events[0].sns_ciphertext_digest,
        sns_ciphertext_digest
    );
    assert_eq!(
        committed_events[0].coprocessor_set_digest,
        coprocessor_set_digest
    );
    assert_eq!(
        committed_events[0].material_commitment_hash,
        material.material_commitment_hash
    );
    assert_eq!(committed_events[0].created_slot, material.created_slot);

    let sealed_acl = read_acl_record(&fixture.svm, output.alice).expect("expected sealed ACL");
    assert_eq!(sealed_acl.material_commitment, material_commitment);
    assert_eq!(
        sealed_acl.material_commitment_hash,
        material.material_commitment_hash
    );
    assert_eq!(sealed_acl.material_key_id, key_id);
    let sealed_events =
        handle_material_sealed_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(sealed_events.len(), 1);
    assert_eq!(sealed_events[0].material_commitment, material_commitment);
    assert_eq!(sealed_events[0].acl_record, output.alice);
    assert_eq!(sealed_events[0].handle, alice_handle);
    assert_eq!(sealed_events[0].key_id, key_id);
    assert_eq!(
        sealed_events[0].material_commitment_hash,
        material.material_commitment_hash
    );
    assert_eq!(sealed_events[0].updated_slot, material.created_slot);
    assert!(kms_like_public_decrypt_with_material_check(
        &fixture.svm,
        &[entry]
    ));

    assert!(send_many_with_signers(
        &mut fixture.svm,
        &fixture.alice.pubkey(),
        vec![commit_ix],
        &[&fixture.alice, &fixture.verifier],
    )
    .is_err());
}

#[test]
fn material_commitment_rejects_wrong_authority() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(110);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, output, amount_handle);
    send(&mut fixture.svm, &fixture.alice, transfer_ix);
    let material_commitment = host::handle_material_address(output.alice).0;

    let commit_ix = commit_handle_material_ix(
        fixture.host_program_id,
        fixture.alice.pubkey(),
        fixture.bob.pubkey(),
        fixture.host_config,
        output.alice,
        material_commitment,
        [31; 32],
        [32; 32],
        [33; 32],
        [34; 32],
    );
    assert!(send_many_with_signers(
        &mut fixture.svm,
        &fixture.alice.pubkey(),
        vec![commit_ix],
        &[&fixture.alice, &fixture.bob],
    )
    .is_err());
    assert!(read_material_commitment(&fixture.svm, material_commitment).is_none());
}

#[test]
fn material_commitment_rejects_unsupported_handle_metadata() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let verifier = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        verifier.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("bad-material-handle");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let mut handle = input_handle_for_chain(111);
    handle[31] = host::HANDLE_VERSION.saturating_add(1);
    let acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        payer.pubkey(),
    );
    let material_commitment = host::handle_material_address(acl_record).0;
    let commit_ix = commit_handle_material_ix(
        program_id,
        payer.pubkey(),
        verifier.pubkey(),
        host_config,
        acl_record,
        material_commitment,
        [41; 32],
        [42; 32],
        [43; 32],
        [44; 32],
    );

    assert!(send_many_with_signers(
        &mut svm,
        &payer.pubkey(),
        vec![commit_ix],
        &[&payer, &verifier],
    )
    .is_err());
    assert!(read_material_commitment(&svm, material_commitment).is_none());
}

#[test]
fn request_disclose_balance_marks_current_balance_public_decrypt() {
    let mut fixture = token_fixture();
    let entry = PublicDecryptHandleEntry {
        handle: fixture.alice_initial,
        acl_record: fixture.alice_current_compute_acl,
    };
    assert!(!kms_like_public_decrypt_check(&fixture.svm, &[entry]));

    let ix = request_disclose_balance_ix(
        &fixture,
        fixture.alice.pubkey(),
        fixture.alice_token,
        fixture.alice_current_compute_acl,
    );
    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, ix);

    let record = read_acl_record(&fixture.svm, fixture.alice_current_compute_acl)
        .expect("expected Alice ACL");
    assert_eq!(record.handle, fixture.alice_initial);
    assert_eq!(record.app_account, fixture.alice_token);
    assert!(record.public_decrypt);
    assert!(kms_like_public_decrypt_check(&fixture.svm, &[entry]));

    let host_events = public_decrypt_allowed_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(host_events.len(), 1);
    assert_eq!(host_events[0].acl_record, fixture.alice_current_compute_acl);
    assert_eq!(host_events[0].handle, fixture.alice_initial);
    assert_eq!(host_events[0].authority, fixture.alice.pubkey().to_bytes());
    assert_eq!(host_events[0].updated_slot, current_slot(&fixture.svm));

    let token_events =
        balance_disclosure_requested_events(&meta, &account_keys, fixture.token_program_id);
    assert_eq!(token_events.len(), 1);
    assert_eq!(token_events[0].mint, fixture.mint.pubkey());
    assert_eq!(token_events[0].owner, fixture.alice.pubkey());
    assert_eq!(token_events[0].token_account, fixture.alice_token);
    assert_eq!(token_events[0].handle, fixture.alice_initial);
    assert_eq!(
        token_events[0].acl_record,
        fixture.alice_current_compute_acl
    );
}

#[test]
fn allow_for_decryption_is_idempotent_without_duplicate_state_event() {
    let mut fixture = token_fixture();
    let disclose = request_disclose_balance_ix(
        &fixture,
        fixture.alice.pubkey(),
        fixture.alice_token,
        fixture.alice_current_compute_acl,
    );
    let (first_meta, first_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, disclose);
    let first_events =
        public_decrypt_allowed_events(&first_meta, &first_keys, fixture.host_program_id);
    assert_eq!(first_events.len(), 1);
    assert_eq!(
        first_events[0].acl_record,
        fixture.alice_current_compute_acl
    );
    assert_eq!(first_events[0].handle, fixture.alice_initial);
    assert_eq!(first_events[0].authority, fixture.alice.pubkey().to_bytes());
    assert_eq!(first_events[0].updated_slot, current_slot(&fixture.svm));

    let disclose_again = request_disclose_balance_ix(
        &fixture,
        fixture.alice.pubkey(),
        fixture.alice_token,
        fixture.alice_current_compute_acl,
    );
    let (second_meta, second_keys) = send_many_with_meta(
        &mut fixture.svm,
        &fixture.alice,
        vec![
            ComputeBudgetInstruction::set_compute_unit_price(1),
            disclose_again,
        ],
    );
    assert!(
        public_decrypt_allowed_events(&second_meta, &second_keys, fixture.host_program_id)
            .is_empty()
    );
    assert_eq!(
        balance_disclosure_requested_events(&second_meta, &second_keys, fixture.token_program_id)
            .len(),
        1
    );
}

#[test]
fn request_disclose_balance_rejects_wrong_owner() {
    let mut fixture = token_fixture();
    let ix = request_disclose_balance_ix(
        &fixture,
        fixture.bob.pubkey(),
        fixture.alice_token,
        fixture.alice_current_compute_acl,
    );

    assert!(
        send_with_signers(&mut fixture.svm, &fixture.bob.pubkey(), ix, &[&fixture.bob]).is_err()
    );
    let record = read_acl_record(&fixture.svm, fixture.alice_current_compute_acl)
        .expect("expected Alice ACL");
    assert!(!record.public_decrypt);
}

#[test]
fn request_disclose_balance_rejects_deny_witness_when_deny_list_disabled() {
    let mut fixture = token_fixture();
    let deny_subject_record = host::deny_subject_address(fixture.alice.pubkey()).0;
    fixture
        .svm
        .set_account(
            deny_subject_record,
            Account {
                lamports: fixture.svm.minimum_balance_for_rent_exemption(0),
                data: Vec::new(),
                owner: system_program::ID,
                executable: false,
                rent_epoch: 0,
            },
        )
        .unwrap();

    let ix = anchor_ix(
        fixture.token_program_id,
        token::accounts::RequestDiscloseBalance {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            token_account: fixture.alice_token,
            balance_acl_record: fixture.alice_current_compute_acl,
            authority_permission_record: None,
            deny_subject_record: Some(deny_subject_record),
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            host_config: fixture.host_config,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::RequestDiscloseBalance {},
    );

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    let record = read_acl_record(&fixture.svm, fixture.alice_current_compute_acl)
        .expect("expected Alice ACL");
    assert!(!record.public_decrypt);
}

#[test]
fn request_disclose_amount_rejects_acl_without_token_compute_authority() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(31);
    let nonce_key = token::nonce_key(
        fixture.mint.pubkey(),
        fixture.alice.pubkey(),
        token::transfer_amount_label(),
    );
    let amount_acl = seed_acl_record_with_subject_entries(
        &mut fixture.svm,
        fixture.host_program_id,
        nonce_key,
        DEFAULT_INPUT_NONCE_SEQUENCE,
        fixture.mint.pubkey(),
        fixture.alice.pubkey(),
        token::transfer_amount_label(),
        amount_handle,
        &[AclSubjectEntry::user(fixture.alice.pubkey())],
    );

    let ix =
        request_disclose_amount_ix(&fixture, fixture.alice.pubkey(), amount_acl, amount_handle);

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    let record = read_acl_record(&fixture.svm, amount_acl).expect("expected amount ACL");
    assert!(!record.public_decrypt);
}

#[test]
fn request_disclose_amount_rejects_non_amount_acl_label() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(32);
    let nonce_key = token::nonce_key(
        fixture.mint.pubkey(),
        fixture.alice.pubkey(),
        token::balance_label(),
    );
    let amount_acl = seed_acl_record_with_subject_entries(
        &mut fixture.svm,
        fixture.host_program_id,
        nonce_key,
        DEFAULT_INPUT_NONCE_SEQUENCE,
        fixture.mint.pubkey(),
        fixture.alice.pubkey(),
        token::balance_label(),
        amount_handle,
        &[
            AclSubjectEntry::user(fixture.alice.pubkey()),
            AclSubjectEntry::compute(fixture.compute_signer),
        ],
    );

    let ix =
        request_disclose_amount_ix(&fixture, fixture.alice.pubkey(), amount_acl, amount_handle);

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    let record = read_acl_record(&fixture.svm, amount_acl).expect("expected amount ACL");
    assert!(!record.public_decrypt);
}

#[test]
fn disclose_balance_accepts_kms_signed_cleartext() {
    let mut fixture = token_fixture();
    let cleartext_amount = 125;
    let alice_acl = fixture.alice_current_compute_acl;
    let request_ix = request_disclose_balance_ix(
        &fixture,
        fixture.alice.pubkey(),
        fixture.alice_token,
        alice_acl,
    );
    send(&mut fixture.svm, &fixture.alice, request_ix);
    commit_material_for_acl(&mut fixture, alice_acl, 120);
    let ed25519_ix = disclosure_ed25519_ix(&fixture, fixture.alice_initial, cleartext_amount);
    let disclose_ix = disclose_balance_ix(
        &fixture,
        fixture.alice_token,
        fixture.alice_current_compute_acl,
        cleartext_amount,
    );

    let (meta, account_keys) = send_many_with_meta(
        &mut fixture.svm,
        &fixture.alice,
        vec![ed25519_ix, disclose_ix],
    );
    let events = balance_disclosed_events(&meta, &account_keys, fixture.token_program_id);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].version, 0);
    assert_eq!(events[0].mint, fixture.mint.pubkey());
    assert_eq!(events[0].owner, fixture.alice.pubkey());
    assert_eq!(events[0].token_account, fixture.alice_token);
    assert_eq!(events[0].handle, fixture.alice_initial);
    assert_eq!(events[0].cleartext_amount, cleartext_amount);
}

#[test]
fn disclose_amount_accepts_kms_signed_cleartext() {
    let mut fixture = token_fixture();
    let cleartext_amount = 125;
    let amount_handle = input_handle_for_chain(33);
    let amount_acl = seed_disclosable_amount_acl(&mut fixture, amount_handle);
    let request_ix =
        request_disclose_amount_ix(&fixture, fixture.alice.pubkey(), amount_acl, amount_handle);
    send(&mut fixture.svm, &fixture.alice, request_ix);
    commit_material_for_acl(&mut fixture, amount_acl, 121);
    let ed25519_ix = disclosure_ed25519_ix(&fixture, amount_handle, cleartext_amount);
    let disclose_ix = disclose_amount_ix(&fixture, amount_acl, amount_handle, cleartext_amount);

    let (meta, account_keys) = send_many_with_meta(
        &mut fixture.svm,
        &fixture.alice,
        vec![ed25519_ix, disclose_ix],
    );
    let events = amount_disclosed_events(&meta, &account_keys, fixture.token_program_id);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].version, 0);
    assert_eq!(events[0].mint, fixture.mint.pubkey());
    assert_eq!(events[0].handle, amount_handle);
    assert_eq!(events[0].cleartext_amount, cleartext_amount);
}

#[test]
fn disclose_amount_rejects_acl_record_with_wrong_stored_bump() {
    let mut fixture = token_fixture();
    let cleartext_amount = 125;
    let amount_handle = input_handle_for_chain(34);
    let amount_acl = seed_disclosable_amount_acl(&mut fixture, amount_handle);
    let request_ix =
        request_disclose_amount_ix(&fixture, fixture.alice.pubkey(), amount_acl, amount_handle);
    send(&mut fixture.svm, &fixture.alice, request_ix);
    commit_material_for_acl(&mut fixture, amount_acl, 121);
    mutate_acl_record(&mut fixture.svm, amount_acl, |record| {
        record.bump = record.bump.wrapping_add(1);
    });

    let ed25519_ix = disclosure_ed25519_ix(&fixture, amount_handle, cleartext_amount);
    let disclose_ix = disclose_amount_ix(&fixture, amount_acl, amount_handle, cleartext_amount);

    assert!(try_send_many(
        &mut fixture.svm,
        &fixture.alice,
        vec![ed25519_ix, disclose_ix],
    )
    .is_err());
}

#[test]
fn disclose_amount_rejects_acl_record_with_noncanonical_nonce_sequence() {
    let mut fixture = token_fixture();
    let cleartext_amount = 125;
    let amount_handle = input_handle_for_chain(35);
    let amount_acl = seed_disclosable_amount_acl(&mut fixture, amount_handle);
    let request_ix =
        request_disclose_amount_ix(&fixture, fixture.alice.pubkey(), amount_acl, amount_handle);
    send(&mut fixture.svm, &fixture.alice, request_ix);
    commit_material_for_acl(&mut fixture, amount_acl, 122);
    mutate_acl_record(&mut fixture.svm, amount_acl, |record| {
        record.nonce_sequence = record.nonce_sequence.wrapping_add(1);
    });

    let ed25519_ix = disclosure_ed25519_ix(&fixture, amount_handle, cleartext_amount);
    let disclose_ix = disclose_amount_ix(&fixture, amount_acl, amount_handle, cleartext_amount);

    assert!(try_send_many(
        &mut fixture.svm,
        &fixture.alice,
        vec![ed25519_ix, disclose_ix],
    )
    .is_err());
}

#[test]
fn disclose_amount_rejects_without_public_decrypt_release() {
    let mut fixture = token_fixture();
    let cleartext_amount = 125;
    let amount_handle = input_handle_for_chain(36);
    let amount_acl = seed_disclosable_amount_acl(&mut fixture, amount_handle);
    commit_material_for_acl(&mut fixture, amount_acl, 124);
    let ed25519_ix = disclosure_ed25519_ix(&fixture, amount_handle, cleartext_amount);
    let disclose_ix = disclose_amount_ix(&fixture, amount_acl, amount_handle, cleartext_amount);

    assert!(try_send_many(
        &mut fixture.svm,
        &fixture.alice,
        vec![ed25519_ix, disclose_ix],
    )
    .is_err());
}

#[test]
fn disclose_balance_rejects_missing_material_commitment() {
    let mut fixture = token_fixture();
    let cleartext_amount = 125;
    let ed25519_ix = disclosure_ed25519_ix(&fixture, fixture.alice_initial, cleartext_amount);
    let disclose_ix = disclose_balance_ix(
        &fixture,
        fixture.alice_token,
        fixture.alice_current_compute_acl,
        cleartext_amount,
    );

    assert!(try_send_many(
        &mut fixture.svm,
        &fixture.alice,
        vec![ed25519_ix, disclose_ix],
    )
    .is_err());
}

#[test]
fn disclose_balance_rejects_unsealed_material_commitment() {
    let mut fixture = token_fixture();
    let cleartext_amount = 125;
    let alice_acl = fixture.alice_current_compute_acl;
    let request_ix = request_disclose_balance_ix(
        &fixture,
        fixture.alice.pubkey(),
        fixture.alice_token,
        alice_acl,
    );
    send(&mut fixture.svm, &fixture.alice, request_ix);
    seed_unsealed_material_commitment(
        &mut fixture.svm,
        fixture.host_program_id,
        alice_acl,
        fixture.alice_initial,
        125,
    );
    let ed25519_ix = disclosure_ed25519_ix(&fixture, fixture.alice_initial, cleartext_amount);
    let disclose_ix = disclose_balance_ix(
        &fixture,
        fixture.alice_token,
        fixture.alice_current_compute_acl,
        cleartext_amount,
    );

    assert!(try_send_many(
        &mut fixture.svm,
        &fixture.alice,
        vec![ed25519_ix, disclose_ix],
    )
    .is_err());
}

#[test]
fn disclose_balance_rejects_oversized_material_commitment() {
    let mut fixture = token_fixture();
    let cleartext_amount = 125;
    let alice_acl = fixture.alice_current_compute_acl;
    let request_ix = request_disclose_balance_ix(
        &fixture,
        fixture.alice.pubkey(),
        fixture.alice_token,
        alice_acl,
    );
    send(&mut fixture.svm, &fixture.alice, request_ix);
    let material_commitment = commit_material_for_acl(&mut fixture, alice_acl, 126);
    extend_material_commitment(&mut fixture.svm, material_commitment, 1);
    let ed25519_ix = disclosure_ed25519_ix(&fixture, fixture.alice_initial, cleartext_amount);
    let disclose_ix = disclose_balance_ix(
        &fixture,
        fixture.alice_token,
        fixture.alice_current_compute_acl,
        cleartext_amount,
    );

    assert!(try_send_many(
        &mut fixture.svm,
        &fixture.alice,
        vec![ed25519_ix, disclose_ix],
    )
    .is_err());
    assert_eq!(
        fixture
            .svm
            .get_account(&material_commitment)
            .expect("expected material commitment")
            .data
            .len(),
        8 + HandleMaterialCommitment::SPACE + 1
    );
}

#[test]
fn disclose_balance_rejects_without_public_decrypt_release() {
    let mut fixture = token_fixture();
    let cleartext_amount = 125;
    let alice_acl = fixture.alice_current_compute_acl;
    commit_material_for_acl(&mut fixture, alice_acl, 124);
    let ed25519_ix = disclosure_ed25519_ix(&fixture, fixture.alice_initial, cleartext_amount);
    let disclose_ix = disclose_balance_ix(
        &fixture,
        fixture.alice_token,
        fixture.alice_current_compute_acl,
        cleartext_amount,
    );

    assert!(try_send_many(
        &mut fixture.svm,
        &fixture.alice,
        vec![ed25519_ix, disclose_ix],
    )
    .is_err());
}

#[test]
fn disclose_balance_rejects_mismatched_kms_signature_message() {
    let mut fixture = token_fixture();
    let signed_amount = 124;
    let claimed_amount = 125;
    let alice_acl = fixture.alice_current_compute_acl;
    let request_ix = request_disclose_balance_ix(
        &fixture,
        fixture.alice.pubkey(),
        fixture.alice_token,
        alice_acl,
    );
    send(&mut fixture.svm, &fixture.alice, request_ix);
    commit_material_for_acl(&mut fixture, alice_acl, 122);
    let ed25519_ix = disclosure_ed25519_ix(&fixture, fixture.alice_initial, signed_amount);
    let disclose_ix = disclose_balance_ix(
        &fixture,
        fixture.alice_token,
        fixture.alice_current_compute_acl,
        claimed_amount,
    );

    assert!(try_send_many(
        &mut fixture.svm,
        &fixture.alice,
        vec![ed25519_ix, disclose_ix],
    )
    .is_err());
}

#[test]
fn disclose_balance_rejects_wrong_kms_verifier() {
    let mut fixture = token_fixture();
    let cleartext_amount = 125;
    let alice_acl = fixture.alice_current_compute_acl;
    let request_ix = request_disclose_balance_ix(
        &fixture,
        fixture.alice.pubkey(),
        fixture.alice_token,
        alice_acl,
    );
    send(&mut fixture.svm, &fixture.alice, request_ix);
    commit_material_for_acl(&mut fixture, alice_acl, 123);
    let wrong_verifier = Keypair::new();
    let message = token::disclosure_proof_message(
        fixture.mint.pubkey(),
        fixture.alice_initial,
        cleartext_amount,
        fixture.token_program_id,
    );
    let ed25519_ix = ed25519_verify_ix(&wrong_verifier, &message);
    let disclose_ix = disclose_balance_ix(
        &fixture,
        fixture.alice_token,
        fixture.alice_current_compute_acl,
        cleartext_amount,
    );

    assert!(try_send_many(
        &mut fixture.svm,
        &fixture.alice,
        vec![ed25519_ix, disclose_ix],
    )
    .is_err());
}

#[test]
fn allow_for_decryption_rejects_unallowed_signer() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, output, amount_handle);
    send(&mut fixture.svm, &fixture.alice, transfer_ix);
    let alice_handle = read_acl_record(&fixture.svm, output.alice)
        .expect("expected Alice ACL")
        .handle;

    let ix = allow_for_decryption_ix(
        fixture.host_program_id,
        fixture.bob.pubkey(),
        output.alice,
        alice_handle,
    );
    assert!(try_send(&mut fixture.svm, &fixture.bob, ix).is_err());

    let record = read_acl_record(&fixture.svm, output.alice).expect("expected ACL record");
    assert!(!record.public_decrypt);
}

#[test]
fn allow_for_decryption_rejects_subject_without_public_role() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let acl_record = seed_acl_record_with_subject_entries(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &[AclSubjectEntry::compute(payer.pubkey())],
    );

    let ix = anchor_ix(
        program_id,
        host::accounts::AllowForDecryption {
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowForDecryption { handle },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert!(!record.public_decrypt);
}

#[test]
fn allow_for_decryption_rejects_deny_witness_when_deny_list_disabled() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("disabled-deny-witness");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let acl_record = seed_acl_record_with_subject_entries(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &[AclSubjectEntry::user(payer.pubkey())],
    );
    let deny_subject_record = host::deny_subject_address(payer.pubkey()).0;
    svm.set_account(
        deny_subject_record,
        Account {
            lamports: svm.minimum_balance_for_rent_exemption(0),
            data: Vec::new(),
            owner: system_program::ID,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();

    let ix = anchor_ix(
        program_id,
        host::accounts::AllowForDecryption {
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: Some(deny_subject_record),
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowForDecryption { handle },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert!(!record.public_decrypt);
}

#[test]
fn allow_for_decryption_rejects_overflow_role_override_for_inline_subject() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("inline-overflow-public");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let acl_record = seed_acl_record_with_subject_entries(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &[AclSubjectEntry::compute(payer.pubkey())],
    );
    let (permission_record, bump) = host::acl_permission_address(acl_record, payer.pubkey());
    seed_acl_permission(
        &mut svm,
        program_id,
        permission_record,
        AclPermission {
            acl_record,
            subject: payer.pubkey(),
            role_flags: host::ACL_ROLE_PUBLIC_DECRYPT,
            bump,
        },
        0,
    );

    let ix = anchor_ix(
        program_id,
        host::accounts::AllowForDecryption {
            authority: payer.pubkey(),
            authority_permission_record: Some(permission_record),
            acl_record,
            host_config,
            deny_subject_record: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowForDecryption { handle },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert!(!record.public_decrypt);
}

#[test]
fn allow_for_decryption_rejects_permission_witness_for_inline_authority() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("extra-public-witness");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let acl_record = seed_acl_record_with_subject_entries(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &[AclSubjectEntry::user(payer.pubkey())],
    );
    let (permission_record, bump) = host::acl_permission_address(acl_record, payer.pubkey());
    seed_acl_permission(
        &mut svm,
        program_id,
        permission_record,
        AclPermission {
            acl_record,
            subject: payer.pubkey(),
            role_flags: host::ACL_ROLE_PUBLIC_DECRYPT,
            bump,
        },
        0,
    );

    let ix = anchor_ix(
        program_id,
        host::accounts::AllowForDecryption {
            authority: payer.pubkey(),
            authority_permission_record: Some(permission_record),
            acl_record,
            host_config,
            deny_subject_record: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowForDecryption { handle },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert!(!record.public_decrypt);
}

#[test]
fn allow_acl_subjects_rejects_compute_only_authority() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let acl_record = seed_acl_record_with_subject_entries(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &[AclSubjectEntry::compute(payer.pubkey())],
    );
    let new_subject = Pubkey::new_unique();

    let ix = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry::user(new_subject)],
        },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record.subject_count, 1);
}

#[test]
fn output_acl_rejects_unknown_subject_role_bits() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("unknown-role-output");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let nonce_sequence = 44;
    let output_acl_record = acl_record_address(program_id, nonce_key, nonce_sequence);
    let ix = anchor_ix(
        program_id,
        host::accounts::FheRandAndBind {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheRandAndBind {
            fhe_type: 3,
            output_nonce_key: nonce_key,
            output_nonce_sequence: nonce_sequence,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry {
                pubkey: payer.pubkey(),
                role_flags: host::ACL_ROLE_USE | 0x80,
            }],
            output_public_decrypt: false,
        },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    assert!(svm.get_account(&output_acl_record).is_none());
}

#[test]
fn output_acl_rejects_duplicate_subjects() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("duplicate-output");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let nonce_sequence = 45;
    let output_acl_record = acl_record_address(program_id, nonce_key, nonce_sequence);
    let ix = anchor_ix(
        program_id,
        host::accounts::FheRandAndBind {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            host_config,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheRandAndBind {
            fhe_type: 3,
            output_nonce_key: nonce_key,
            output_nonce_sequence: nonce_sequence,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![
                AclSubjectEntry::compute(payer.pubkey()),
                AclSubjectEntry::compute(payer.pubkey()),
            ],
            output_public_decrypt: false,
        },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    assert!(svm.get_account(&output_acl_record).is_none());
}

#[test]
fn allow_acl_subjects_rejects_unknown_subject_role_bits() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("unknown-role-grant");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let acl_record = seed_acl_record_with_subject_entries(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &[AclSubjectEntry::user(payer.pubkey())],
    );

    let ix = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry {
                pubkey: Pubkey::new_unique(),
                role_flags: host::ACL_ROLE_USE | 0x80,
            }],
        },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record.subject_count, 1);
}

#[test]
fn allow_acl_subjects_uses_overflow_permission_for_ninth_subject() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let mut subjects = vec![AclSubjectEntry::user(payer.pubkey())];
    subjects.extend((0..7).map(|_| AclSubjectEntry::use_only(Pubkey::new_unique())));
    let acl_record = seed_acl_record_with_subject_entries(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &subjects,
    );
    let overflow_subject = Pubkey::new_unique();
    let (permission_record, _) = host::acl_permission_address(acl_record, overflow_subject);

    let mut ix = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry::use_only(overflow_subject)],
        },
    );
    ix.accounts.push(AccountMeta::new(permission_record, false));

    let (meta, _) = send_with_meta(&mut svm, &payer, ix);
    let events = acl_subject_allowed_events(&meta);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].acl_record, acl_record);
    assert_eq!(events[0].handle, handle);
    assert_eq!(events[0].authority_subject, payer.pubkey());
    assert_eq!(events[0].subject, overflow_subject.to_bytes());
    assert_eq!(events[0].role_flags, host::ACL_ROLE_USE);
    assert_eq!(events[0].overflow_permission_record, permission_record);
    assert_eq!(events[0].inline_index, u8::MAX);
    assert_eq!(events[0].updated_slot, current_slot(&svm));

    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record.subject_count as usize, host::MAX_ACL_SUBJECTS);
    assert_eq!(record.overflow_subject_count, 1);
    assert!(svm.get_account(&permission_record).is_some());
}

#[test]
fn allow_acl_subjects_rejects_dirty_overflow_permission_target() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("dirty-overflow");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let mut subjects = vec![AclSubjectEntry::user(payer.pubkey())];
    subjects.extend((0..7).map(|_| AclSubjectEntry::use_only(Pubkey::new_unique())));
    let acl_record = seed_acl_record_with_subject_entries(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &subjects,
    );
    let overflow_subject = Pubkey::new_unique();
    let (permission_record, _) = host::acl_permission_address(acl_record, overflow_subject);
    svm.set_account(
        permission_record,
        Account {
            lamports: svm.minimum_balance_for_rent_exemption(1),
            data: vec![1],
            owner: system_program::ID,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();

    let mut ix = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry::use_only(overflow_subject)],
        },
    );
    ix.accounts.push(AccountMeta::new(permission_record, false));

    assert!(try_send(&mut svm, &payer, ix).is_err());
    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record.overflow_subject_count, 0);
    let dirty_target = svm
        .get_account(&permission_record)
        .expect("expected dirty target");
    assert_eq!(dirty_target.owner, system_program::ID);
    assert_eq!(dirty_target.data, vec![1]);
}

#[test]
fn allow_acl_subjects_rejects_missing_overflow_witness() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("missing-overflow");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let mut subjects = vec![AclSubjectEntry::user(payer.pubkey())];
    subjects.extend((0..7).map(|_| AclSubjectEntry::use_only(Pubkey::new_unique())));
    let acl_record = seed_acl_record_with_subject_entries(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &subjects,
    );
    let overflow_subject = Pubkey::new_unique();
    let ix = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry::use_only(overflow_subject)],
        },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record.subject_count as usize, host::MAX_ACL_SUBJECTS);
    assert_eq!(record.overflow_subject_count, 0);
}

#[test]
fn allow_acl_subjects_rejects_extra_overflow_witness_for_inline_grant() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("extra-inline");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let acl_record = seed_acl_record_with_subject_entries(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &[AclSubjectEntry::user(payer.pubkey())],
    );
    let new_subject = Pubkey::new_unique();
    let (extra_permission_record, _) =
        host::acl_permission_address(acl_record, Pubkey::new_unique());

    let mut ix = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry::use_only(new_subject)],
        },
    );
    ix.accounts
        .push(AccountMeta::new(extra_permission_record, false));

    assert!(try_send(&mut svm, &payer, ix).is_err());
    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record.subject_count, 1);
    assert!(!record.inline_subject_has_role(new_subject, host::ACL_ROLE_USE));
    assert!(svm.get_account(&extra_permission_record).is_none());
}

#[test]
fn allow_acl_subjects_rejects_extra_overflow_witness_after_required_overflow() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("extra-overflow");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let mut subjects = vec![AclSubjectEntry::user(payer.pubkey())];
    subjects.extend((0..7).map(|_| AclSubjectEntry::use_only(Pubkey::new_unique())));
    let acl_record = seed_acl_record_with_subject_entries(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &subjects,
    );
    let overflow_subject = Pubkey::new_unique();
    let (permission_record, _) = host::acl_permission_address(acl_record, overflow_subject);
    let (extra_permission_record, _) =
        host::acl_permission_address(acl_record, Pubkey::new_unique());

    let mut ix = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry::use_only(overflow_subject)],
        },
    );
    ix.accounts.push(AccountMeta::new(permission_record, false));
    ix.accounts
        .push(AccountMeta::new(extra_permission_record, false));

    assert!(try_send(&mut svm, &payer, ix).is_err());
    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record.subject_count as usize, host::MAX_ACL_SUBJECTS);
    assert_eq!(record.overflow_subject_count, 0);
    assert!(svm.get_account(&permission_record).is_none());
    assert!(svm.get_account(&extra_permission_record).is_none());
}

#[test]
fn allow_acl_subjects_is_idempotent_for_existing_overflow_subject() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("overflow-idem");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let mut subjects = vec![AclSubjectEntry::user(payer.pubkey())];
    subjects.extend((0..7).map(|_| AclSubjectEntry::use_only(Pubkey::new_unique())));
    let acl_record = seed_acl_record_with_subject_entries(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &subjects,
    );
    let overflow_subject = Pubkey::new_unique();
    let (permission_record, _) = host::acl_permission_address(acl_record, overflow_subject);

    for role_flags in [host::ACL_ROLE_USE, host::ACL_ROLE_GRANT] {
        let mut ix = anchor_ix(
            program_id,
            host::accounts::AllowAclSubjects {
                payer: payer.pubkey(),
                authority: payer.pubkey(),
                authority_permission_record: None,
                acl_record,
                host_config,
                deny_subject_record: None,
                system_program: system_program::ID,
                event_authority: event_authority(program_id),
                program: program_id,
            },
            host::instruction::AllowAclSubjects {
                handle,
                subjects: vec![AclSubjectEntry {
                    pubkey: overflow_subject,
                    role_flags,
                }],
            },
        );
        ix.accounts.push(AccountMeta::new(permission_record, false));
        send(&mut svm, &payer, ix);
    }

    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record.subject_count as usize, host::MAX_ACL_SUBJECTS);
    assert_eq!(record.overflow_subject_count, 1);
    let permission = read_acl_permission(&svm, permission_record).expect("expected permission");
    assert_eq!(permission.acl_record, acl_record);
    assert_eq!(permission.subject, overflow_subject);
    assert_eq!(
        permission.role_flags,
        host::ACL_ROLE_USE | host::ACL_ROLE_GRANT
    );
}

#[test]
fn overflow_permission_rejects_wrong_bump_or_length() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("overflow-exact");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [8; 32];
    let mut subjects = vec![AclSubjectEntry::user(payer.pubkey())];
    subjects.extend((0..7).map(|_| AclSubjectEntry::use_only(Pubkey::new_unique())));
    let acl_record = seed_acl_record_with_subject_entries(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &subjects,
    );
    let overflow_subject = Pubkey::new_unique();
    let (permission_record, bump) = host::acl_permission_address(acl_record, overflow_subject);
    seed_acl_permission(
        &mut svm,
        program_id,
        permission_record,
        AclPermission {
            acl_record,
            subject: overflow_subject,
            role_flags: host::ACL_ROLE_USE,
            bump: bump.wrapping_add(1),
        },
        0,
    );

    let grant_ix = allow_acl_subjects_with_permission_ix(
        program_id,
        payer.pubkey(),
        host_config,
        acl_record,
        permission_record,
        handle,
        overflow_subject,
        host::ACL_ROLE_GRANT,
    );
    assert!(try_send_many(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(203_001),
            grant_ix,
        ],
    )
    .is_err());
    let permission = read_acl_permission(&svm, permission_record).expect("expected permission");
    assert_eq!(permission.role_flags, host::ACL_ROLE_USE);
    assert_ne!(permission.bump, bump);

    let witness_ix = assert_acl_record_ix(
        program_id,
        acl_record,
        Some(permission_record),
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        overflow_subject,
    );
    assert!(try_send(&mut svm, &payer, witness_ix).is_err());

    seed_acl_permission(
        &mut svm,
        program_id,
        permission_record,
        AclPermission {
            acl_record,
            subject: overflow_subject,
            role_flags: host::ACL_ROLE_USE,
            bump,
        },
        1,
    );

    let grant_ix = allow_acl_subjects_with_permission_ix(
        program_id,
        payer.pubkey(),
        host_config,
        acl_record,
        permission_record,
        handle,
        overflow_subject,
        host::ACL_ROLE_GRANT,
    );
    assert!(try_send_many(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(203_002),
            grant_ix,
        ],
    )
    .is_err());
    assert_eq!(
        svm.get_account(&permission_record)
            .expect("expected permission")
            .data
            .len(),
        8 + AclPermission::SPACE + 1
    );

    let witness_ix = assert_acl_record_ix(
        program_id,
        acl_record,
        Some(permission_record),
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        overflow_subject,
    );
    assert!(try_send_many(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(203_003),
            witness_ix,
        ],
    )
    .is_err());
}

#[test]
fn acl_record_rejects_noncanonical_account_length() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("acl-exact");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let nonce_sequence = 0;
    let handle = [8; 32];
    let acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        payer.pubkey(),
    );
    extend_acl_record(&mut svm, acl_record, 1);

    let grant_ix = allow_acl_subjects_ix(
        program_id,
        payer.pubkey(),
        host_config,
        acl_record,
        handle,
        Pubkey::new_unique(),
    );
    assert!(try_send_many(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(203_101),
            grant_ix,
        ],
    )
    .is_err());

    let witness_ix = assert_acl_record_ix(
        program_id,
        acl_record,
        None,
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        payer.pubkey(),
    );
    assert!(try_send_many(
        &mut svm,
        &payer,
        vec![
            ComputeBudgetInstruction::set_compute_unit_limit(203_102),
            witness_ix,
        ],
    )
    .is_err());
    assert_eq!(
        svm.get_account(&acl_record)
            .expect("expected ACL record")
            .data
            .len(),
        8 + AclRecord::SPACE + 1
    );
}

#[test]
fn assert_acl_record_accepts_overflow_permission_witness() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("overflow-assert");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [8; 32];
    let mut subjects = vec![AclSubjectEntry::user(payer.pubkey())];
    subjects.extend((0..7).map(|_| AclSubjectEntry::use_only(Pubkey::new_unique())));
    let acl_record = seed_acl_record_with_subject_entries(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &subjects,
    );
    let overflow_subject = Pubkey::new_unique();
    let (permission_record, _) = host::acl_permission_address(acl_record, overflow_subject);

    let mut grant_ix = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry::use_only(overflow_subject)],
        },
    );
    grant_ix
        .accounts
        .push(AccountMeta::new(permission_record, false));
    send(&mut svm, &payer, grant_ix);

    let no_witness_ix = anchor_ix(
        program_id,
        host::accounts::AssertAclRecord {
            acl_record,
            subject_permission_record: None,
        },
        host::instruction::AssertAclRecord {
            nonce_key,
            nonce_sequence: 0,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            handle,
            subject: overflow_subject,
        },
    );
    assert!(try_send(&mut svm, &payer, no_witness_ix).is_err());

    let witness_ix = anchor_ix(
        program_id,
        host::accounts::AssertAclRecord {
            acl_record,
            subject_permission_record: Some(permission_record),
        },
        host::instruction::AssertAclRecord {
            nonce_key,
            nonce_sequence: 0,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            handle,
            subject: overflow_subject,
        },
    );
    send(&mut svm, &payer, witness_ix);
}

#[test]
fn assert_acl_record_rejects_overflow_role_override_for_inline_subject() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("inline-overflow-use");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [8; 32];
    let acl_record = seed_acl_record_with_subject_entries(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &[AclSubjectEntry {
            pubkey: payer.pubkey(),
            role_flags: host::ACL_ROLE_PUBLIC_DECRYPT,
        }],
    );
    let (permission_record, bump) = host::acl_permission_address(acl_record, payer.pubkey());
    seed_acl_permission(
        &mut svm,
        program_id,
        permission_record,
        AclPermission {
            acl_record,
            subject: payer.pubkey(),
            role_flags: host::ACL_ROLE_USE,
            bump,
        },
        0,
    );

    let witness_ix = anchor_ix(
        program_id,
        host::accounts::AssertAclRecord {
            acl_record,
            subject_permission_record: Some(permission_record),
        },
        host::instruction::AssertAclRecord {
            nonce_key,
            nonce_sequence: 0,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            handle,
            subject: payer.pubkey(),
        },
    );
    assert!(try_send(&mut svm, &payer, witness_ix).is_err());
}

#[test]
fn assert_acl_record_rejects_permission_witness_for_inline_subject() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("extra-assert-witness");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [8; 32];
    let acl_record = seed_acl_record_with_subject_entries(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &[AclSubjectEntry::user(payer.pubkey())],
    );
    let (permission_record, bump) = host::acl_permission_address(acl_record, payer.pubkey());
    seed_acl_permission(
        &mut svm,
        program_id,
        permission_record,
        AclPermission {
            acl_record,
            subject: payer.pubkey(),
            role_flags: host::ACL_ROLE_USE,
            bump,
        },
        0,
    );

    let witness_ix = anchor_ix(
        program_id,
        host::accounts::AssertAclRecord {
            acl_record,
            subject_permission_record: Some(permission_record),
        },
        host::instruction::AssertAclRecord {
            nonce_key,
            nonce_sequence: 0,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            handle,
            subject: payer.pubkey(),
        },
    );
    assert!(try_send(&mut svm, &payer, witness_ix).is_err());
}

#[test]
fn fhe_binary_op_accepts_overflow_permission_witness_for_compute_subject() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let compute_subject = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        program_id,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
    );
    let dummy_rhs_account = svm.create_funded_account(1_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("overflow-compute");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(8);
    let mut subjects = vec![AclSubjectEntry::user(payer.pubkey())];
    subjects.extend((0..7).map(|_| AclSubjectEntry::use_only(Pubkey::new_unique())));
    let lhs_acl_record = seed_acl_record_with_subject_entries(
        &mut svm,
        program_id,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        &subjects,
    );
    let (permission_record, _) =
        host::acl_permission_address(lhs_acl_record, compute_subject.pubkey());

    let mut grant_ix = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            authority_permission_record: None,
            acl_record: lhs_acl_record,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle: lhs,
            subjects: vec![AclSubjectEntry::use_only(compute_subject.pubkey())],
        },
    );
    grant_ix
        .accounts
        .push(AccountMeta::new(permission_record, false));
    send(&mut svm, &payer, grant_ix);

    let rhs_scalar = amount_plaintext(5);
    let output_acl_record = acl_record_address(program_id, nonce_key, 1);
    let build_ix = |lhs_permission_record, result| {
        anchor_ix(
            program_id,
            host::accounts::FheBinaryOpAndBindOutput {
                payer: payer.pubkey(),
                compute_subject: compute_subject.pubkey(),
                app_account_authority: app_account,
                host_config,
                lhs_acl_record,
                lhs_permission_record,
                rhs_acl_record: dummy_rhs_account.pubkey(),
                rhs_permission_record: None,
                output_acl_record,
                system_program: system_program::ID,
                event_authority: event_authority(program_id),
                program: program_id,
            },
            host::instruction::FheBinaryOpAndBindOutput {
                op: FheBinaryOpCode::Add,
                lhs,
                rhs: rhs_scalar,
                scalar: true,
                output_fhe_type: 5,
                result,
                output_nonce_key: nonce_key,
                output_nonce_sequence: 1,
                output_acl_domain_key: acl_domain_key,
                output_app_account: app_account,
                output_encrypted_value_label: encrypted_value_label,
                output_subjects: vec![AclSubjectEntry::user(compute_subject.pubkey())],
                output_public_decrypt: false,
            },
        )
    };

    let missing_witness_result = current_bound_binary_handle(
        &svm,
        FheBinaryOpCode::Add,
        lhs,
        rhs_scalar,
        true,
        5,
        nonce_key,
        1,
    );
    assert!(send_with_signers(
        &mut svm,
        &payer.pubkey(),
        build_ix(None, missing_witness_result),
        &[&payer, &compute_subject]
    )
    .is_err());

    let result = current_bound_binary_handle(
        &svm,
        FheBinaryOpCode::Add,
        lhs,
        rhs_scalar,
        true,
        5,
        nonce_key,
        1,
    );
    send_with_signers(
        &mut svm,
        &payer.pubkey(),
        build_ix(Some(permission_record), result),
        &[&payer, &compute_subject],
    )
    .unwrap();

    let output = read_acl_record(&svm, output_acl_record).expect("expected output ACL");
    assert_eq!(output.handle, result);
}

#[test]
fn wrap_usdc_escrows_spl_tokens_and_rotates_confidential_balance() {
    let mut fixture = token_fixture();
    let amount = 100_000_000;
    let mut cleartext = CleartextBackend::default();
    cleartext.seed_cleartext(fixture.alice_initial, TypedClearValue::uint64(125));
    cleartext.seed_cleartext(fixture.total_supply_initial, TypedClearValue::uint64(0));

    let output = wrap_output_accounts(&fixture, 1);
    let ix = wrap_usdc_ix(&fixture, output, amount);

    let alice_usdc_before = spl_token_amount(&fixture.svm, fixture.alice_usdc);
    let vault_usdc_before = spl_token_amount(&fixture.svm, fixture.vault_usdc);
    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, ix);
    cleartext
        .ingest_transaction(&meta, &account_keys, fixture.host_program_id)
        .unwrap();

    assert_eq!(
        spl_token_amount(&fixture.svm, fixture.alice_usdc),
        alice_usdc_before - amount
    );
    assert_eq!(
        spl_token_amount(&fixture.svm, fixture.vault_usdc),
        vault_usdc_before + amount
    );

    let trivial_events = trivial_encrypt_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(trivial_events.len(), 1);
    assert_eq!(trivial_events[0].subject, fixture.compute_signer.to_bytes());
    assert_eq!(trivial_events[0].plaintext, amount_plaintext(amount));

    let amount_record = read_acl_record(&fixture.svm, output.amount).expect("expected amount ACL");
    let amount_handle = amount_record.handle;
    assert_eq!(trivial_events[0].result, amount_handle);
    assert_eq!(
        cleartext.decrypt_cleartext(amount_handle),
        Some(TypedClearValue::uint64(amount))
    );

    let events = binary_op_events(&meta, &account_keys, fixture.host_program_id);
    let output_record = read_acl_record(&fixture.svm, output.balance).expect("expected output ACL");
    let supply_record =
        read_acl_record(&fixture.svm, output.total_supply).expect("expected total supply ACL");
    let new_alice = output_record.handle;
    let new_total_supply = supply_record.handle;
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].op, FheBinaryOpCode::Add);
    assert_eq!(events[0].subject, fixture.compute_signer.to_bytes());
    assert_eq!(events[0].lhs, fixture.alice_initial);
    assert_eq!(events[0].rhs, amount_handle);
    assert_eq!(events[0].result, new_alice);
    assert_eq!(events[1].op, FheBinaryOpCode::Add);
    assert_eq!(events[1].subject, fixture.compute_signer.to_bytes());
    assert_eq!(events[1].lhs, fixture.total_supply_initial);
    assert_eq!(events[1].rhs, amount_handle);
    assert_eq!(events[1].result, new_total_supply);
    let balance_events =
        balance_handle_updated_events(&meta, &account_keys, fixture.token_program_id);
    let supply_events =
        total_supply_handle_updated_events(&meta, &account_keys, fixture.token_program_id);
    assert_eq!(balance_events.len(), 1);
    assert_eq!(balance_events[0].reason, BalanceHandleUpdateReason::Wrap);
    assert_eq!(balance_events[0].old_handle, fixture.alice_initial);
    assert_eq!(
        balance_events[0].old_acl_record,
        fixture.alice_current_compute_acl
    );
    assert_eq!(balance_events[0].new_handle, new_alice);
    assert_eq!(balance_events[0].new_acl_record, output.balance);
    assert_eq!(
        cleartext.decrypt_cleartext(new_alice),
        Some(TypedClearValue::uint64(100_000_125))
    );
    assert_eq!(supply_events.len(), 1);
    assert_eq!(supply_events[0].reason, TotalSupplyUpdateReason::Wrap);
    assert_eq!(supply_events[0].old_handle, fixture.total_supply_initial);
    assert_eq!(
        supply_events[0].old_acl_record,
        fixture.total_supply_current_acl
    );
    assert_eq!(supply_events[0].new_handle, new_total_supply);
    assert_eq!(supply_events[0].new_acl_record, output.total_supply);
    assert_eq!(
        cleartext.decrypt_cleartext(new_total_supply),
        Some(TypedClearValue::uint64(amount))
    );

    assert_acl_record(
        &fixture.svm,
        output.amount,
        fixture.mint.pubkey(),
        fixture.alice_token,
        token::wrap_amount_label(),
        1,
        amount_handle,
        &[fixture.compute_signer],
    );

    let alice_account = token_account(&fixture.svm, fixture.alice_token);
    assert_eq!(alice_account.balance_handle, new_alice);
    assert_eq!(alice_account.balance_acl_record, output.balance);
    assert_eq!(alice_account.next_balance_nonce_sequence, 2);
    let mint_account = mint_account(&fixture.svm, fixture.mint.pubkey());
    assert_eq!(mint_account.total_supply_handle, new_total_supply);
    assert_eq!(mint_account.total_supply_acl_record, output.total_supply);
    assert_eq!(mint_account.next_total_supply_nonce_sequence, 2);

    assert_balance_acl(
        &fixture.svm,
        output.balance,
        fixture.mint.pubkey(),
        fixture.alice_token,
        1,
        new_alice,
        &[fixture.alice.pubkey(), fixture.compute_signer],
    );
    assert_acl_record(
        &fixture.svm,
        output.total_supply,
        fixture.mint.pubkey(),
        fixture.total_supply_authority,
        token::total_supply_label(),
        1,
        new_total_supply,
        &[fixture.compute_signer],
    );
}

#[test]
fn wrap_usdc_rejects_noncanonical_vault_account() {
    let mut fixture = token_fixture();
    let noncanonical_vault = create_noncanonical_vault_token_account(&mut fixture);
    let output = wrap_output_accounts(&fixture, 1);
    let ix = wrap_usdc_ix_with_vault(&fixture, output, 100_000_000, noncanonical_vault);

    let alice_usdc_before = spl_token_amount(&fixture.svm, fixture.alice_usdc);
    let canonical_vault_before = spl_token_amount(&fixture.svm, fixture.vault_usdc);
    let noncanonical_vault_before = spl_token_amount(&fixture.svm, noncanonical_vault);

    assert!(send_with_signers(
        &mut fixture.svm,
        &fixture.alice.pubkey(),
        ix,
        &[&fixture.alice]
    )
    .is_err());
    assert_eq!(
        spl_token_amount(&fixture.svm, fixture.alice_usdc),
        alice_usdc_before
    );
    assert_eq!(
        spl_token_amount(&fixture.svm, fixture.vault_usdc),
        canonical_vault_before
    );
    assert_eq!(
        spl_token_amount(&fixture.svm, noncanonical_vault),
        noncanonical_vault_before
    );
}

#[test]
fn confidential_burn_rotates_balance_and_total_supply_handles() {
    let mut fixture = token_fixture();
    let wrap_amount = 100_000_000;
    let burn_amount_handle = input_handle_for_chain(77);
    let mut cleartext = CleartextBackend::default();
    cleartext.seed_cleartext(fixture.alice_initial, TypedClearValue::uint64(125));
    cleartext.seed_cleartext(fixture.total_supply_initial, TypedClearValue::uint64(0));

    let wrap_output = wrap_output_accounts(&fixture, 1);
    let wrap_ix = wrap_usdc_ix(&fixture, wrap_output, wrap_amount);
    let (wrap_meta, wrap_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, wrap_ix);
    cleartext
        .ingest_transaction(&wrap_meta, &wrap_keys, fixture.host_program_id)
        .unwrap();
    let alice_after_wrap = read_acl_record(&fixture.svm, wrap_output.balance)
        .expect("expected wrapped balance ACL")
        .handle;
    let supply_after_wrap = read_acl_record(&fixture.svm, wrap_output.total_supply)
        .expect("expected wrapped supply ACL")
        .handle;
    cleartext.seed_cleartext(burn_amount_handle, TypedClearValue::uint64(9));
    authorize_burn_input_compute_acl(
        &mut fixture,
        burn_amount_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );

    let output = burn_output_accounts(&fixture, 2, 2);
    let burn_ix = burn_ix_with_current_acls(
        &fixture,
        wrap_output.balance,
        wrap_output.total_supply,
        output,
        burn_amount_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, burn_ix);
    cleartext
        .ingest_transaction(&meta, &account_keys, fixture.host_program_id)
        .unwrap();

    let balance_record =
        read_acl_record(&fixture.svm, output.balance).expect("expected burn balance ACL");
    let success_record =
        read_acl_record(&fixture.svm, output.success).expect("expected burn success ACL");
    let debit_candidate_record =
        read_acl_record(&fixture.svm, output.debit_candidate).expect("expected debit ACL");
    let burned_record =
        read_acl_record(&fixture.svm, output.burned).expect("expected burned amount ACL");
    let supply_record =
        read_acl_record(&fixture.svm, output.total_supply).expect("expected supply ACL");
    let new_balance = balance_record.handle;
    let success = success_record.handle;
    let debit_candidate = debit_candidate_record.handle;
    let burned = burned_record.handle;
    let new_total_supply = supply_record.handle;

    let events = binary_op_events(&meta, &account_keys, fixture.host_program_id);
    let ternary_events = ternary_op_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(events.len(), 4);
    assert_eq!(ternary_events.len(), 1);
    assert_eq!(events[0].op, FheBinaryOpCode::Ge);
    assert_eq!(events[0].lhs, alice_after_wrap);
    assert_eq!(events[0].rhs, burn_amount_handle);
    assert_eq!(events[0].result, success);
    assert_eq!(events[1].op, FheBinaryOpCode::Sub);
    assert_eq!(events[1].lhs, alice_after_wrap);
    assert_eq!(events[1].rhs, burn_amount_handle);
    assert_eq!(events[1].result, debit_candidate);
    assert_eq!(ternary_events[0].op, FheTernaryOpCode::IfThenElse);
    assert_eq!(ternary_events[0].control, success);
    assert_eq!(ternary_events[0].if_true, debit_candidate);
    assert_eq!(ternary_events[0].if_false, alice_after_wrap);
    assert_eq!(ternary_events[0].result, new_balance);
    assert_eq!(events[2].op, FheBinaryOpCode::Sub);
    assert_eq!(events[2].lhs, alice_after_wrap);
    assert_eq!(events[2].rhs, new_balance);
    assert_eq!(events[2].result, burned);
    assert_eq!(events[3].op, FheBinaryOpCode::Sub);
    assert_eq!(events[3].lhs, supply_after_wrap);
    assert_eq!(events[3].rhs, burned);
    assert_eq!(events[3].result, new_total_supply);

    let burn_events = confidential_burn_events(&meta, &account_keys, fixture.token_program_id);
    assert_eq!(burn_events.len(), 1);
    assert_eq!(burn_events[0].version, 0);
    assert_eq!(burn_events[0].mint, fixture.mint.pubkey());
    assert_eq!(burn_events[0].owner, fixture.alice.pubkey());
    assert_eq!(burn_events[0].token_account, fixture.alice_token);
    assert_eq!(burn_events[0].burned_handle, burned);
    assert_eq!(burn_events[0].burned_acl_record, output.burned);

    let balance_events =
        balance_handle_updated_events(&meta, &account_keys, fixture.token_program_id);
    let supply_events =
        total_supply_handle_updated_events(&meta, &account_keys, fixture.token_program_id);
    assert_eq!(balance_events.len(), 1);
    assert_eq!(
        balance_events[0].reason,
        BalanceHandleUpdateReason::BurnDebit
    );
    assert_eq!(balance_events[0].old_handle, alice_after_wrap);
    assert_eq!(balance_events[0].old_acl_record, wrap_output.balance);
    assert_eq!(balance_events[0].new_handle, new_balance);
    assert_eq!(balance_events[0].new_acl_record, output.balance);
    assert_eq!(supply_events.len(), 1);
    assert_eq!(supply_events[0].reason, TotalSupplyUpdateReason::Burn);
    assert_eq!(supply_events[0].old_handle, supply_after_wrap);
    assert_eq!(supply_events[0].old_acl_record, wrap_output.total_supply);
    assert_eq!(supply_events[0].new_handle, new_total_supply);
    assert_eq!(supply_events[0].new_acl_record, output.total_supply);

    assert_eq!(
        cleartext.decrypt_cleartext(success),
        Some(TypedClearValue {
            fhe_type: 0,
            value: ClearValue::Uint(1),
        })
    );
    assert_eq!(
        cleartext.decrypt_cleartext(new_balance),
        Some(TypedClearValue::uint64(100_000_116))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(burned),
        Some(TypedClearValue::uint64(9))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(new_total_supply),
        Some(TypedClearValue::uint64(99_999_991))
    );

    let alice_account = token_account(&fixture.svm, fixture.alice_token);
    let mint_account = mint_account(&fixture.svm, fixture.mint.pubkey());
    assert_eq!(alice_account.balance_handle, new_balance);
    assert_eq!(alice_account.balance_acl_record, output.balance);
    assert_eq!(alice_account.next_balance_nonce_sequence, 3);
    assert_eq!(mint_account.total_supply_handle, new_total_supply);
    assert_eq!(mint_account.total_supply_acl_record, output.total_supply);
    assert_eq!(mint_account.next_total_supply_nonce_sequence, 3);

    assert_acl_record(
        &fixture.svm,
        burn_input_compute_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE),
        fixture.mint.pubkey(),
        fixture.alice.pubkey(),
        token::burn_amount_label(),
        DEFAULT_INPUT_NONCE_SEQUENCE,
        burn_amount_handle,
        &[fixture.compute_signer],
    );
    assert_acl_record(
        &fixture.svm,
        output.success,
        fixture.mint.pubkey(),
        fixture.alice_token,
        token::burn_success_label(),
        2,
        success,
        &[fixture.compute_signer],
    );
    assert_acl_record(
        &fixture.svm,
        output.debit_candidate,
        fixture.mint.pubkey(),
        fixture.alice_token,
        token::burn_debit_candidate_label(),
        2,
        debit_candidate,
        &[fixture.compute_signer],
    );
    assert_balance_acl(
        &fixture.svm,
        output.balance,
        fixture.mint.pubkey(),
        fixture.alice_token,
        2,
        new_balance,
        &[fixture.alice.pubkey(), fixture.compute_signer],
    );
    assert_acl_record(
        &fixture.svm,
        output.burned,
        fixture.mint.pubkey(),
        fixture.alice_token,
        token::burned_amount_label(),
        2,
        burned,
        &[fixture.alice.pubkey(), fixture.compute_signer],
    );
    assert!(burned_record
        .inline_subject_has_role(fixture.alice.pubkey(), host::ACL_ROLE_PUBLIC_DECRYPT));
    assert_acl_record(
        &fixture.svm,
        output.total_supply,
        fixture.mint.pubkey(),
        fixture.total_supply_authority,
        token::total_supply_label(),
        2,
        new_total_supply,
        &[fixture.compute_signer],
    );

    let disclose_burned =
        request_disclose_amount_ix(&fixture, fixture.alice.pubkey(), output.burned, burned);
    let (disclose_meta, disclose_keys) =
        send_with_meta(&mut fixture.svm, &fixture.alice, disclose_burned);
    let burned_record =
        read_acl_record(&fixture.svm, output.burned).expect("expected burned amount ACL");
    assert!(burned_record.public_decrypt);
    let disclose_events = amount_disclosure_requested_events(
        &disclose_meta,
        &disclose_keys,
        fixture.token_program_id,
    );
    assert_eq!(disclose_events.len(), 1);
    assert_eq!(disclose_events[0].requester, fixture.alice.pubkey());
    assert_eq!(disclose_events[0].handle, burned);
    assert_eq!(disclose_events[0].acl_record, output.burned);
}

#[test]
fn redeem_burned_amount_releases_vault_once_with_kms_certificate() {
    let mut fixture = token_fixture();
    let (output, burned) = wrap_and_burn_for_redeem(&mut fixture, 80);
    release_burned_amount_for_redeem(&mut fixture, output.burned, burned);
    commit_material_for_acl(&mut fixture, output.burned, 130);
    let cleartext_amount = 9;
    let redemption_record = token::burn_redemption_address(fixture.mint.pubkey(), burned).0;
    let vault_before = spl_token_amount(&fixture.svm, fixture.vault_usdc);
    let alice_usdc_before = spl_token_amount(&fixture.svm, fixture.alice_usdc);

    let ed25519_ix = disclosure_ed25519_ix(&fixture, burned, cleartext_amount);
    let redeem_ix = redeem_burned_amount_ix(
        &fixture,
        output.burned,
        redemption_record,
        burned,
        cleartext_amount,
    );
    let (meta, account_keys) = send_many_with_meta(
        &mut fixture.svm,
        &fixture.alice,
        vec![ed25519_ix, redeem_ix],
    );

    assert_eq!(
        spl_token_amount(&fixture.svm, fixture.vault_usdc),
        vault_before - cleartext_amount
    );
    assert_eq!(
        spl_token_amount(&fixture.svm, fixture.alice_usdc),
        alice_usdc_before + cleartext_amount
    );
    let redemption =
        read_burn_redemption(&fixture.svm, redemption_record).expect("expected redemption marker");
    assert_eq!(redemption.mint, fixture.mint.pubkey());
    assert_eq!(redemption.owner, fixture.alice.pubkey());
    assert_eq!(redemption.token_account, fixture.alice_token);
    assert_eq!(redemption.burned_handle, burned);
    assert_eq!(redemption.burned_acl_record, output.burned);
    assert_eq!(redemption.cleartext_amount, cleartext_amount);

    let events = burn_redeemed_events(&meta, &account_keys, fixture.token_program_id);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].version, 0);
    assert_eq!(events[0].mint, fixture.mint.pubkey());
    assert_eq!(events[0].owner, fixture.alice.pubkey());
    assert_eq!(events[0].token_account, fixture.alice_token);
    assert_eq!(events[0].burned_handle, burned);
    assert_eq!(events[0].burned_acl_record, output.burned);
    assert_eq!(events[0].destination_usdc, fixture.alice_usdc);
    assert_eq!(events[0].cleartext_amount, cleartext_amount);

    let replay_ed25519 = disclosure_ed25519_ix(&fixture, burned, cleartext_amount);
    let replay_redeem = redeem_burned_amount_ix(
        &fixture,
        output.burned,
        redemption_record,
        burned,
        cleartext_amount,
    );
    assert!(try_send_many(
        &mut fixture.svm,
        &fixture.alice,
        vec![replay_ed25519, replay_redeem],
    )
    .is_err());
}

#[test]
fn redeem_burned_amount_rejects_without_public_decrypt_release() {
    let mut fixture = token_fixture();
    let (output, burned) = wrap_and_burn_for_redeem(&mut fixture, 81);
    commit_material_for_acl(&mut fixture, output.burned, 131);
    let cleartext_amount = 9;
    let redemption_record = token::burn_redemption_address(fixture.mint.pubkey(), burned).0;
    let vault_before = spl_token_amount(&fixture.svm, fixture.vault_usdc);
    let alice_usdc_before = spl_token_amount(&fixture.svm, fixture.alice_usdc);

    let ed25519_ix = disclosure_ed25519_ix(&fixture, burned, cleartext_amount);
    let redeem_ix = redeem_burned_amount_ix(
        &fixture,
        output.burned,
        redemption_record,
        burned,
        cleartext_amount,
    );

    assert!(try_send_many(
        &mut fixture.svm,
        &fixture.alice,
        vec![ed25519_ix, redeem_ix],
    )
    .is_err());
    assert!(read_burn_redemption(&fixture.svm, redemption_record).is_none());
    assert_eq!(
        spl_token_amount(&fixture.svm, fixture.vault_usdc),
        vault_before
    );
    assert_eq!(
        spl_token_amount(&fixture.svm, fixture.alice_usdc),
        alice_usdc_before
    );
    let burned_record =
        read_acl_record(&fixture.svm, output.burned).expect("expected burned amount ACL");
    assert!(!burned_record.public_decrypt);
}

#[test]
fn redeem_burned_amount_rejects_noncanonical_vault_account() {
    let mut fixture = token_fixture();
    let (output, burned) = wrap_and_burn_for_redeem(&mut fixture, 82);
    release_burned_amount_for_redeem(&mut fixture, output.burned, burned);
    commit_material_for_acl(&mut fixture, output.burned, 132);
    let cleartext_amount = 9;
    let redemption_record = token::burn_redemption_address(fixture.mint.pubkey(), burned).0;
    let noncanonical_vault = create_noncanonical_vault_token_account(&mut fixture);
    fixture
        .svm
        .mint_to(
            &fixture.underlying_mint.pubkey(),
            &noncanonical_vault,
            &fixture.alice,
            100,
        )
        .unwrap();
    let canonical_vault_before = spl_token_amount(&fixture.svm, fixture.vault_usdc);
    let noncanonical_vault_before = spl_token_amount(&fixture.svm, noncanonical_vault);
    let alice_usdc_before = spl_token_amount(&fixture.svm, fixture.alice_usdc);

    let ed25519_ix = disclosure_ed25519_ix(&fixture, burned, cleartext_amount);
    let redeem_ix = redeem_burned_amount_ix_with_vault(
        &fixture,
        output.burned,
        redemption_record,
        burned,
        cleartext_amount,
        noncanonical_vault,
    );

    assert!(try_send_many(
        &mut fixture.svm,
        &fixture.alice,
        vec![ed25519_ix, redeem_ix]
    )
    .is_err());
    assert!(read_burn_redemption(&fixture.svm, redemption_record).is_none());
    assert_eq!(
        spl_token_amount(&fixture.svm, fixture.vault_usdc),
        canonical_vault_before
    );
    assert_eq!(
        spl_token_amount(&fixture.svm, noncanonical_vault),
        noncanonical_vault_before
    );
    assert_eq!(
        spl_token_amount(&fixture.svm, fixture.alice_usdc),
        alice_usdc_before
    );
}

#[test]
fn redeem_burned_amount_rejects_mismatched_kms_cleartext() {
    let mut fixture = token_fixture();
    let (output, burned) = wrap_and_burn_for_redeem(&mut fixture, 83);
    release_burned_amount_for_redeem(&mut fixture, output.burned, burned);
    commit_material_for_acl(&mut fixture, output.burned, 133);
    let signed_amount = 8;
    let claimed_amount = 9;
    let redemption_record = token::burn_redemption_address(fixture.mint.pubkey(), burned).0;
    let vault_before = spl_token_amount(&fixture.svm, fixture.vault_usdc);
    let alice_usdc_before = spl_token_amount(&fixture.svm, fixture.alice_usdc);

    let ed25519_ix = disclosure_ed25519_ix(&fixture, burned, signed_amount);
    let redeem_ix = redeem_burned_amount_ix(
        &fixture,
        output.burned,
        redemption_record,
        burned,
        claimed_amount,
    );
    assert!(try_send_many(
        &mut fixture.svm,
        &fixture.alice,
        vec![ed25519_ix, redeem_ix],
    )
    .is_err());
    assert_eq!(
        spl_token_amount(&fixture.svm, fixture.vault_usdc),
        vault_before
    );
    assert_eq!(
        spl_token_amount(&fixture.svm, fixture.alice_usdc),
        alice_usdc_before
    );
    assert!(read_burn_redemption(&fixture.svm, redemption_record).is_none());
}

#[test]
fn confidential_burn_over_balance_burns_zero_without_underflow() {
    let mut fixture = token_fixture();
    let wrap_amount = 100_000_000;
    let burn_amount_handle = input_handle_for_chain(78);
    let mut cleartext = CleartextBackend::default();
    cleartext.seed_cleartext(fixture.alice_initial, TypedClearValue::uint64(125));
    cleartext.seed_cleartext(fixture.total_supply_initial, TypedClearValue::uint64(0));

    let wrap_output = wrap_output_accounts(&fixture, 1);
    let wrap_ix = wrap_usdc_ix(&fixture, wrap_output, wrap_amount);
    let (wrap_meta, wrap_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, wrap_ix);
    cleartext
        .ingest_transaction(&wrap_meta, &wrap_keys, fixture.host_program_id)
        .unwrap();
    let alice_after_wrap = read_acl_record(&fixture.svm, wrap_output.balance)
        .expect("expected wrapped balance ACL")
        .handle;
    let supply_after_wrap = read_acl_record(&fixture.svm, wrap_output.total_supply)
        .expect("expected wrapped supply ACL")
        .handle;
    cleartext.seed_cleartext(burn_amount_handle, TypedClearValue::uint64(200_000_000));
    authorize_burn_input_compute_acl(
        &mut fixture,
        burn_amount_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );

    let output = burn_output_accounts(&fixture, 2, 2);
    let burn_ix = burn_ix_with_current_acls(
        &fixture,
        wrap_output.balance,
        wrap_output.total_supply,
        output,
        burn_amount_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, burn_ix);
    cleartext
        .ingest_transaction(&meta, &account_keys, fixture.host_program_id)
        .unwrap();

    let success = read_acl_record(&fixture.svm, output.success)
        .expect("expected success ACL")
        .handle;
    let new_balance = read_acl_record(&fixture.svm, output.balance)
        .expect("expected balance ACL")
        .handle;
    let burned = read_acl_record(&fixture.svm, output.burned)
        .expect("expected burned ACL")
        .handle;
    let new_total_supply = read_acl_record(&fixture.svm, output.total_supply)
        .expect("expected total supply ACL")
        .handle;

    assert_eq!(
        cleartext.decrypt_cleartext(success),
        Some(TypedClearValue {
            fhe_type: 0,
            value: ClearValue::Uint(0),
        })
    );
    assert_eq!(
        cleartext.decrypt_cleartext(new_balance),
        Some(TypedClearValue::uint64(100_000_125))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(burned),
        Some(TypedClearValue::uint64(0))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(new_total_supply),
        Some(TypedClearValue::uint64(100_000_000))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(alice_after_wrap),
        Some(TypedClearValue::uint64(100_000_125))
    );
    assert_eq!(
        cleartext.decrypt_cleartext(supply_after_wrap),
        Some(TypedClearValue::uint64(100_000_000))
    );
}

#[test]
fn confidential_burn_rejects_transfer_amount_acl_label() {
    let mut fixture = token_fixture();
    let wrap_amount = 100_000_000;
    let amount_handle = input_handle_for_chain(79);
    let wrap_output = wrap_output_accounts(&fixture, 1);
    let wrap_ix = wrap_usdc_ix(&fixture, wrap_output, wrap_amount);
    send(&mut fixture.svm, &fixture.alice, wrap_ix);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);

    let output = burn_output_accounts(&fixture, 2, 2);
    let ix = burn_ix_with_amount_acl(
        &fixture,
        wrap_output.balance,
        wrap_output.total_supply,
        input_compute_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE),
        output,
        amount_handle,
    );

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_acl_record,
        wrap_output.balance
    );
    assert_eq!(
        mint_account(&fixture.svm, fixture.mint.pubkey()).total_supply_acl_record,
        wrap_output.total_supply
    );
    assert_eq!(created_burn_acl_count(&fixture.svm, output), 0);
}

#[test]
fn confidential_token_e2e_wrap_transfer_and_decrypts_current_and_historical_balances() {
    let mut fixture = token_fixture();

    // 1. Alice wraps underlying USDC into her confidential balance.
    let wrap_amount = 100_000_000;
    let wrap_output = wrap_output_accounts(&fixture, 1);
    let wrap_ix = wrap_usdc_ix(&fixture, wrap_output, wrap_amount);
    send(&mut fixture.svm, &fixture.alice, wrap_ix);
    let alice_after_wrap = read_acl_record(&fixture.svm, wrap_output.balance)
        .expect("expected wrap ACL")
        .handle;

    let alice_account_after_wrap = token_account(&fixture.svm, fixture.alice_token);
    assert_eq!(alice_account_after_wrap.balance_handle, alice_after_wrap);
    assert_eq!(
        alice_account_after_wrap.balance_acl_record,
        wrap_output.balance
    );
    assert_eq!(alice_account_after_wrap.next_balance_nonce_sequence, 2);
    assert_balance_acl(
        &fixture.svm,
        wrap_output.balance,
        fixture.mint.pubkey(),
        fixture.alice_token,
        1,
        alice_after_wrap,
        &[fixture.alice.pubkey(), fixture.compute_signer],
    );

    // 2. Alice transfers a confidential amount to Bob.
    let transfer_amount_handle = input_handle_for_chain(8);
    authorize_input_compute_acl(
        &mut fixture,
        transfer_amount_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let transfer_output = TransferOutputAccounts {
        alice: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.alice_token,
            2,
        ),
        bob: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.bob_token,
            1,
        ),
        success: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.alice_token,
                token::transfer_success_label(),
            ),
            2,
        ),
        debit_candidate: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.alice_token,
                token::debit_candidate_label(),
            ),
            2,
        ),
        transferred: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.alice_token,
                token::transferred_amount_label(),
            ),
            2,
        ),
    };
    let transfer_ix = transfer_ix_with_current_acl(
        &fixture,
        wrap_output.balance,
        fixture.bob_current_compute_acl,
        transfer_output,
        transfer_amount_handle,
    );
    send(&mut fixture.svm, &fixture.alice, transfer_ix);
    let alice_after_transfer = read_acl_record(&fixture.svm, transfer_output.alice)
        .expect("expected Alice transfer ACL")
        .handle;
    let bob_after_transfer = read_acl_record(&fixture.svm, transfer_output.bob)
        .expect("expected Bob transfer ACL")
        .handle;

    let alice_account_after_transfer = token_account(&fixture.svm, fixture.alice_token);
    let bob_account_after_transfer = token_account(&fixture.svm, fixture.bob_token);
    assert_eq!(
        alice_account_after_transfer.balance_handle,
        alice_after_transfer
    );
    assert_eq!(
        alice_account_after_transfer.balance_acl_record,
        transfer_output.alice
    );
    assert_eq!(alice_account_after_transfer.next_balance_nonce_sequence, 3);
    assert_eq!(
        bob_account_after_transfer.balance_handle,
        bob_after_transfer
    );
    assert_eq!(
        bob_account_after_transfer.balance_acl_record,
        transfer_output.bob
    );
    assert_eq!(bob_account_after_transfer.next_balance_nonce_sequence, 2);

    // 3. Alice can decrypt her current balance through the current ACL record.
    let alice_current_request =
        signed_current_balance_user_decrypt_request(&fixture, fixture.alice_token, &fixture.alice);
    assert_eq!(
        alice_current_request.handles[0].handle,
        alice_after_transfer
    );
    assert_eq!(
        alice_current_request.handles[0].acl_record,
        transfer_output.alice
    );
    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &alice_current_request
    ));

    // 4. Alice can still decrypt the historical wrapped balance while that ACL record exists.
    let alice_historical_request = signed_user_decrypt_request(
        &fixture,
        &fixture.alice,
        vec![UserDecryptHandleEntry {
            handle: alice_after_wrap,
            owner: fixture.alice.pubkey(),
            acl_record: wrap_output.balance,
        }],
    );
    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &alice_historical_request
    ));

    // 5. Bob can decrypt his current balance through Bob's ACL record.
    let bob_current_request =
        signed_current_balance_user_decrypt_request(&fixture, fixture.bob_token, &fixture.bob);
    assert_eq!(bob_current_request.handles[0].handle, bob_after_transfer);
    assert_eq!(
        bob_current_request.handles[0].acl_record,
        transfer_output.bob
    );
    assert!(kms_like_user_decrypt_check(
        &fixture.svm,
        &bob_current_request
    ));

    // 6. The same request shape rejects spoofed owner, ACL record, handle, and domain.
    let mut alice_claims_bob_balance = bob_current_request.clone();
    alice_claims_bob_balance.authorization.user = fixture.alice.pubkey();
    alice_claims_bob_balance.signature = fixture.alice.sign_message(&authorization_payload_bytes(
        &alice_claims_bob_balance.authorization,
    ));
    assert!(!kms_like_user_decrypt_check(
        &fixture.svm,
        &alice_claims_bob_balance
    ));

    let mut wrong_acl_record = alice_current_request.clone();
    wrong_acl_record.handles[0].acl_record = transfer_output.bob;
    assert!(!kms_like_user_decrypt_check(
        &fixture.svm,
        &wrong_acl_record
    ));

    let mut wrong_handle = alice_current_request.clone();
    wrong_handle.handles[0].handle = [99; 32];
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &wrong_handle));

    let mut wrong_domain = alice_current_request;
    wrong_domain.authorization.allowed_acl_domain_keys = vec![Pubkey::new_unique()];
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &wrong_domain));
}

#[test]
fn confidential_transfer_budget_snapshot() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, output, amount_handle);
    let top_level_metas = transfer_ix.accounts.len();
    let writable_metas = transfer_ix
        .accounts
        .iter()
        .filter(|account| account.is_writable)
        .count();
    let signer_metas = transfer_ix
        .accounts
        .iter()
        .filter(|account| account.is_signer)
        .count();

    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);
    let inner_instructions = meta.inner_instructions.iter().flatten().count();
    let host_events = binary_op_events(&meta, &account_keys, fixture.host_program_id).len()
        + ternary_op_events(&meta, &account_keys, fixture.host_program_id).len();
    let app_events =
        balance_handle_updated_events(&meta, &account_keys, fixture.token_program_id).len();
    let max_cpi_depth = max_cpi_depth(&meta);

    assert_eq!(top_level_metas, 19);
    assert_eq!(writable_metas, 8);
    assert_eq!(signer_metas, 1);
    assert_eq!(host_events, 5);
    assert_eq!(app_events, 2);
    assert_eq!(created_acl_count(&fixture.svm, output), 5);
    assert!(
        inner_instructions <= 32,
        "inner instructions: {inner_instructions}"
    );
    assert!(
        meta.compute_units_consumed <= 350_000,
        "compute units: {}",
        meta.compute_units_consumed
    );
    assert_eq!(max_cpi_depth, 3);
}

#[test]
fn confidential_transfer_rejects_stale_current_acl() {
    let mut fixture = token_fixture();
    let first_amount_handle = input_handle_for_chain(9);
    authorize_input_compute_acl(
        &mut fixture,
        first_amount_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let first_output = transfer_output_accounts(&fixture, 1);
    let first_ix = transfer_ix(&fixture, first_output, first_amount_handle);
    send(&mut fixture.svm, &fixture.alice, first_ix);

    let second_amount_handle = input_handle_for_chain(8);
    authorize_input_compute_acl(&mut fixture, second_amount_handle, 1);
    let stale_ix = transfer_ix_with_amount_nonce(
        &fixture,
        transfer_output_accounts(&fixture, 2),
        second_amount_handle,
        1,
    );
    assert!(try_send(&mut fixture.svm, &fixture.alice, stale_ix).is_err());
}

#[test]
fn confidential_transfer_rejects_wrong_current_acl_record() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let ix = transfer_ix_with_current_acl(
        &fixture,
        fixture.bob_current_compute_acl,
        fixture.bob_current_compute_acl,
        transfer_output_accounts(&fixture, 1),
        amount_handle,
    );
    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
}

#[test]
fn confidential_transfer_rejects_oversized_current_acl_record() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let current_acl = fixture.alice_current_compute_acl;
    extend_acl_record(&mut fixture.svm, current_acl, 1);

    let output = transfer_output_accounts(&fixture, 1);
    let ix = transfer_ix(&fixture, output, amount_handle);

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_handle,
        fixture.alice_initial
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.bob_token).balance_handle,
        fixture.bob_initial
    );
    assert_eq!(created_acl_count(&fixture.svm, output), 0);
}

#[test]
fn confidential_transfer_rejects_malformed_current_acl_subject_slots() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let current_acl = fixture.alice_current_compute_acl;
    mutate_acl_record(&mut fixture.svm, current_acl, |record| {
        record.subject_count = (host::MAX_ACL_SUBJECTS + 1) as u8;
    });

    let output = transfer_output_accounts(&fixture, 1);
    let ix = transfer_ix(&fixture, output, amount_handle);

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_handle,
        fixture.alice_initial
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.bob_token).balance_handle,
        fixture.bob_initial
    );
    assert_eq!(created_acl_count(&fixture.svm, output), 0);
}

#[test]
fn confidential_transfer_rejects_current_acl_for_wrong_mint_domain() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let current_acl = fixture.alice_current_compute_acl;
    mutate_acl_record(&mut fixture.svm, current_acl, |record| {
        record.acl_domain_key = Pubkey::new_unique();
    });

    let output = transfer_output_accounts(&fixture, 1);
    let ix = transfer_ix(&fixture, output, amount_handle);

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_handle,
        fixture.alice_initial
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.bob_token).balance_handle,
        fixture.bob_initial
    );
    assert_eq!(created_acl_count(&fixture.svm, output), 0);
}

#[test]
fn confidential_transfer_rejects_wrong_amount_acl() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    let wrong_amount_handle = input_handle_for_chain(8);
    authorize_input_compute_acl(
        &mut fixture,
        wrong_amount_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );

    let output = transfer_output_accounts(&fixture, 1);
    let ix = transfer_ix_with_amount_acl(
        &fixture,
        fixture.alice_current_compute_acl,
        fixture.bob_current_compute_acl,
        input_compute_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE),
        output,
        amount_handle,
    );

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_handle,
        fixture.alice_initial
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.bob_token).balance_handle,
        fixture.bob_initial
    );
    assert_eq!(created_acl_count(&fixture.svm, output), 0);
}

#[test]
fn confidential_transfer_rejects_oversized_amount_acl_record() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let amount_acl = input_compute_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE);
    extend_acl_record(&mut fixture.svm, amount_acl, 1);

    let output = transfer_output_accounts(&fixture, 1);
    let ix = transfer_ix(&fixture, output, amount_handle);

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_handle,
        fixture.alice_initial
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.bob_token).balance_handle,
        fixture.bob_initial
    );
    assert_eq!(created_acl_count(&fixture.svm, output), 0);
}

#[test]
fn confidential_transfer_rejects_malformed_amount_acl_subject_slots() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let amount_acl = input_compute_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE);
    mutate_acl_record(&mut fixture.svm, amount_acl, |record| {
        record.subject_roles[1] = host::ACL_ROLE_USE;
    });

    let output = transfer_output_accounts(&fixture, 1);
    let ix = transfer_ix(&fixture, output, amount_handle);

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_handle,
        fixture.alice_initial
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.bob_token).balance_handle,
        fixture.bob_initial
    );
    assert_eq!(created_acl_count(&fixture.svm, output), 0);
}

#[test]
fn confidential_transfer_rejects_wrong_amount_handle_type() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain_with_type(9, 4);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);

    let output = transfer_output_accounts(&fixture, 1);
    let ix = transfer_ix(&fixture, output, amount_handle);

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    assert_eq!(created_acl_count(&fixture.svm, output), 0);
}

#[test]
fn confidential_transfer_rejects_amount_acl_for_wrong_scope() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    let wrong_nonce_key = token::nonce_key(
        fixture.mint.pubkey(),
        fixture.bob.pubkey(),
        token::transfer_amount_label(),
    );
    let wrong_amount_acl = seed_acl_record_with_subject_entries(
        &mut fixture.svm,
        fixture.host_program_id,
        wrong_nonce_key,
        DEFAULT_INPUT_NONCE_SEQUENCE,
        fixture.mint.pubkey(),
        fixture.bob.pubkey(),
        token::transfer_amount_label(),
        amount_handle,
        &[AclSubjectEntry::compute(fixture.compute_signer)],
    );

    let output = transfer_output_accounts(&fixture, 1);
    let ix = transfer_ix_with_amount_acl(
        &fixture,
        fixture.alice_current_compute_acl,
        fixture.bob_current_compute_acl,
        wrong_amount_acl,
        output,
        amount_handle,
    );

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    assert_eq!(created_acl_count(&fixture.svm, output), 0);
}

#[test]
fn confidential_transfer_rejects_amount_acl_for_wrong_mint_domain() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    let wrong_mint = Pubkey::new_unique();
    let wrong_nonce_key = token::nonce_key(
        wrong_mint,
        fixture.alice.pubkey(),
        token::transfer_amount_label(),
    );
    let wrong_amount_acl = seed_acl_record_with_subject_entries(
        &mut fixture.svm,
        fixture.host_program_id,
        wrong_nonce_key,
        DEFAULT_INPUT_NONCE_SEQUENCE,
        wrong_mint,
        fixture.alice.pubkey(),
        token::transfer_amount_label(),
        amount_handle,
        &[AclSubjectEntry::compute(fixture.compute_signer)],
    );

    let output = transfer_output_accounts(&fixture, 1);
    let ix = transfer_ix_with_amount_acl(
        &fixture,
        fixture.alice_current_compute_acl,
        fixture.bob_current_compute_acl,
        wrong_amount_acl,
        output,
        amount_handle,
    );

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    assert_eq!(created_acl_count(&fixture.svm, output), 0);
}

#[test]
fn confidential_transfer_rejects_amount_acl_for_wrong_label() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    let wrong_label = label("wrong-transfer-label");
    let wrong_nonce_key =
        token::nonce_key(fixture.mint.pubkey(), fixture.alice.pubkey(), wrong_label);
    let wrong_amount_acl = seed_acl_record_with_subject_entries(
        &mut fixture.svm,
        fixture.host_program_id,
        wrong_nonce_key,
        DEFAULT_INPUT_NONCE_SEQUENCE,
        fixture.mint.pubkey(),
        fixture.alice.pubkey(),
        wrong_label,
        amount_handle,
        &[AclSubjectEntry::compute(fixture.compute_signer)],
    );

    let output = transfer_output_accounts(&fixture, 1);
    let ix = transfer_ix_with_amount_acl(
        &fixture,
        fixture.alice_current_compute_acl,
        fixture.bob_current_compute_acl,
        wrong_amount_acl,
        output,
        amount_handle,
    );

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    assert_eq!(created_acl_count(&fixture.svm, output), 0);
}

#[test]
fn confidential_transfer_rejects_output_acl_for_wrong_token_account() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let correct_output = transfer_output_accounts(&fixture, 1);
    let swapped_output = TransferOutputAccounts {
        alice: correct_output.bob,
        bob: correct_output.alice,
        success: correct_output.success,
        debit_candidate: correct_output.debit_candidate,
        transferred: correct_output.transferred,
    };

    let ix = transfer_ix(&fixture, swapped_output, amount_handle);

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_handle,
        fixture.alice_initial
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.bob_token).balance_handle,
        fixture.bob_initial
    );
    assert_eq!(created_acl_count(&fixture.svm, correct_output), 0);
}

#[test]
fn confidential_transfer_rejects_reused_output_acl_record() {
    let mut fixture = token_fixture();
    let amount_handle = input_handle_for_chain(9);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = TransferOutputAccounts {
        alice: fixture.alice_current_compute_acl,
        bob: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.bob_token,
            1,
        ),
        success: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.alice_token,
                token::transfer_success_label(),
            ),
            1,
        ),
        debit_candidate: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.alice_token,
                token::debit_candidate_label(),
            ),
            1,
        ),
        transferred: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.alice_token,
                token::transferred_amount_label(),
            ),
            1,
        ),
    };

    let ix = transfer_ix(&fixture, output, amount_handle);

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_handle,
        fixture.alice_initial
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.bob_token).balance_handle,
        fixture.bob_initial
    );
    assert!(read_acl_record(&fixture.svm, output.bob).is_none());
}

#[test]
fn confidential_transfer_from_uses_active_operator_without_owner_signature() {
    let mut fixture = token_fixture();
    let operator = fixture.svm.create_funded_account(1_000_000_000).unwrap();
    let operator_record = token::operator_record_address(fixture.alice_token, operator.pubkey()).0;
    let expiration_slot = current_slot(&fixture.svm) + 100;
    let set_operator = set_operator_ix(
        &fixture,
        operator.pubkey(),
        operator_record,
        expiration_slot,
    );
    send(&mut fixture.svm, &fixture.alice, set_operator);
    let record = read_operator_record(&fixture.svm, operator_record).expect("expected operator");
    assert_eq!(record.token_account, fixture.alice_token);
    assert_eq!(record.owner, fixture.alice.pubkey());
    assert_eq!(record.operator, operator.pubkey());
    assert_eq!(record.expiration_slot, expiration_slot);

    let amount_handle = input_handle_for_chain(9);
    authorize_transfer_amount_compute_acl_for_signer(
        &mut fixture,
        &operator,
        amount_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let amount_acl = input_compute_acl_address_for_authority(
        &fixture,
        operator.pubkey(),
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let output = transfer_output_accounts(&fixture, 1);
    let transfer = transfer_from_ix_with_amount_acl(
        &fixture,
        operator.pubkey(),
        operator_record,
        amount_acl,
        output,
        amount_handle,
    );
    send_with_signers(&mut fixture.svm, &operator.pubkey(), transfer, &[&operator]).unwrap();

    let alice_account = token_account(&fixture.svm, fixture.alice_token);
    let bob_account = token_account(&fixture.svm, fixture.bob_token);
    assert_eq!(alice_account.balance_acl_record, output.alice);
    assert_eq!(bob_account.balance_acl_record, output.bob);
    assert_eq!(alice_account.next_balance_nonce_sequence, 2);
    assert_eq!(bob_account.next_balance_nonce_sequence, 2);
}

#[test]
fn confidential_transfer_from_rejects_owner_scoped_amount_acl_for_operator() {
    let mut fixture = token_fixture();
    let operator = fixture.svm.create_funded_account(1_000_000_000).unwrap();
    let operator_record = token::operator_record_address(fixture.alice_token, operator.pubkey()).0;
    let expiration_slot = current_slot(&fixture.svm) + 100;
    let set_operator = set_operator_ix(
        &fixture,
        operator.pubkey(),
        operator_record,
        expiration_slot,
    );
    send(&mut fixture.svm, &fixture.alice, set_operator);

    let amount_handle = input_handle_for_chain(94);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer = transfer_from_ix(
        &fixture,
        operator.pubkey(),
        operator_record,
        output,
        amount_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );

    assert!(
        send_with_signers(&mut fixture.svm, &operator.pubkey(), transfer, &[&operator]).is_err()
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_handle,
        fixture.alice_initial
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.bob_token).balance_handle,
        fixture.bob_initial
    );
    assert_eq!(created_acl_count(&fixture.svm, output), 0);
}

#[test]
fn confidential_transfer_from_accepts_operator_scoped_amount_acl() {
    let mut fixture = token_fixture();
    let operator = fixture.svm.create_funded_account(1_000_000_000).unwrap();
    let operator_record = token::operator_record_address(fixture.alice_token, operator.pubkey()).0;
    let expiration_slot = current_slot(&fixture.svm) + 100;
    let set_operator = set_operator_ix(
        &fixture,
        operator.pubkey(),
        operator_record,
        expiration_slot,
    );
    send(&mut fixture.svm, &fixture.alice, set_operator);

    let amount_handle = input_handle_for_chain(92);
    authorize_transfer_amount_compute_acl_for_signer(
        &mut fixture,
        &operator,
        amount_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let amount_acl = input_compute_acl_address_for_authority(
        &fixture,
        operator.pubkey(),
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let output = transfer_output_accounts(&fixture, 1);
    let transfer = transfer_from_ix_with_amount_acl(
        &fixture,
        operator.pubkey(),
        operator_record,
        amount_acl,
        output,
        amount_handle,
    );
    send_with_signers(&mut fixture.svm, &operator.pubkey(), transfer, &[&operator]).unwrap();

    let transferred_record =
        read_acl_record(&fixture.svm, output.transferred).expect("expected transferred ACL");
    assert_eq!(
        record_subjects(&transferred_record),
        vec![
            fixture.alice.pubkey(),
            fixture.bob.pubkey(),
            fixture.compute_signer,
        ]
    );
    assert!(!transferred_record.inline_subject_has_role(operator.pubkey(), host::ACL_ROLE_USE));
}

#[test]
fn confidential_transfer_from_rejects_third_party_scoped_amount_acl() {
    let mut fixture = token_fixture();
    let operator = fixture.svm.create_funded_account(1_000_000_000).unwrap();
    let mallory = fixture.svm.create_funded_account(1_000_000_000).unwrap();
    let operator_record = token::operator_record_address(fixture.alice_token, operator.pubkey()).0;
    let expiration_slot = current_slot(&fixture.svm) + 100;
    let set_operator = set_operator_ix(
        &fixture,
        operator.pubkey(),
        operator_record,
        expiration_slot,
    );
    send(&mut fixture.svm, &fixture.alice, set_operator);

    let amount_handle = input_handle_for_chain(93);
    authorize_transfer_amount_compute_acl_for_signer(
        &mut fixture,
        &mallory,
        amount_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let amount_acl = input_compute_acl_address_for_authority(
        &fixture,
        mallory.pubkey(),
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let output = transfer_output_accounts(&fixture, 1);
    let transfer = transfer_from_ix_with_amount_acl(
        &fixture,
        operator.pubkey(),
        operator_record,
        amount_acl,
        output,
        amount_handle,
    );

    assert!(
        send_with_signers(&mut fixture.svm, &operator.pubkey(), transfer, &[&operator]).is_err()
    );
    assert_eq!(created_acl_count(&fixture.svm, output), 0);
}

#[test]
fn transfer_receiver_hook_accepts_active_operator_after_transfer_from() {
    let mut fixture = token_fixture();
    let operator = fixture.svm.create_funded_account(1_000_000_000).unwrap();
    let operator_record = token::operator_record_address(fixture.alice_token, operator.pubkey()).0;
    let expiration_slot = current_slot(&fixture.svm) + 100;
    let set_operator = set_operator_ix(
        &fixture,
        operator.pubkey(),
        operator_record,
        expiration_slot,
    );
    send(&mut fixture.svm, &fixture.alice, set_operator);

    let amount_handle = input_handle_for_chain(68);
    let callback_success_handle = input_handle_for_chain_with_type(69, 0);
    authorize_transfer_amount_compute_acl_for_signer(
        &mut fixture,
        &operator,
        amount_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let amount_acl = input_compute_acl_address_for_authority(
        &fixture,
        operator.pubkey(),
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    authorize_callback_success_acl(
        &mut fixture,
        callback_success_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let callback_success_acl = callback_success_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE);

    let predicted_sent_handle = predicted_transfer_sent_handle(&fixture, amount_handle);
    let transfer_output = transfer_output_accounts(&fixture, 1);
    let transfer = transfer_from_ix_with_amount_acl(
        &fixture,
        operator.pubkey(),
        operator_record,
        amount_acl,
        transfer_output,
        amount_handle,
    );
    let receiver_data = token::instruction::TestReceiverReturnCallback {
        mint: fixture.mint.pubkey(),
        from_token_account: fixture.alice_token,
        to_token_account: fixture.bob_token,
        sent_handle: predicted_sent_handle,
        sent_acl_record: transfer_output.transferred,
        callback_success_handle,
        callback_success_acl_record: callback_success_acl,
    }
    .data();
    let hook_ix = call_transfer_receiver_from_ix(
        &fixture,
        operator.pubkey(),
        operator_record,
        transfer_output.transferred,
        predicted_sent_handle,
        callback_success_acl,
        callback_success_handle,
        fixture.token_program_id,
        receiver_data,
    );
    send_many_with_signers(
        &mut fixture.svm,
        &operator.pubkey(),
        vec![transfer, hook_ix],
        &[&operator],
    )
    .unwrap();
    let sent_record = read_acl_record(&fixture.svm, transfer_output.transferred)
        .expect("expected transferred ACL");
    assert_eq!(sent_record.handle, predicted_sent_handle);
}

#[test]
fn transfer_receiver_hook_rejects_revoked_operator() {
    let mut fixture = token_fixture();
    let operator = fixture.svm.create_funded_account(1_000_000_000).unwrap();
    let operator_record = token::operator_record_address(fixture.alice_token, operator.pubkey()).0;
    let set_operator = set_operator_ix(&fixture, operator.pubkey(), operator_record, 0);
    send(&mut fixture.svm, &fixture.alice, set_operator);

    let amount_handle = input_handle_for_chain(70);
    let callback_success_handle = input_handle_for_chain_with_type(71, 0);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    authorize_callback_success_acl(
        &mut fixture,
        callback_success_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    let callback_success_acl = callback_success_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE);

    let predicted_sent_handle = predicted_transfer_sent_handle(&fixture, amount_handle);
    let transfer_output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, transfer_output, amount_handle);
    let receiver_data = token::instruction::TestReceiverReturnCallback {
        mint: fixture.mint.pubkey(),
        from_token_account: fixture.alice_token,
        to_token_account: fixture.bob_token,
        sent_handle: predicted_sent_handle,
        sent_acl_record: transfer_output.transferred,
        callback_success_handle,
        callback_success_acl_record: callback_success_acl,
    }
    .data();
    let hook_ix = call_transfer_receiver_from_ix(
        &fixture,
        operator.pubkey(),
        operator_record,
        transfer_output.transferred,
        predicted_sent_handle,
        callback_success_acl,
        callback_success_handle,
        fixture.token_program_id,
        receiver_data,
    );

    assert!(send_many_with_signers(
        &mut fixture.svm,
        &fixture.alice.pubkey(),
        vec![transfer_ix, hook_ix],
        &[&fixture.alice, &operator],
    )
    .is_err());
}

#[test]
fn confidential_transfer_from_rejects_revoked_operator() {
    let mut fixture = token_fixture();
    let operator = fixture.svm.create_funded_account(1_000_000_000).unwrap();
    let operator_record = token::operator_record_address(fixture.alice_token, operator.pubkey()).0;
    let set_operator = set_operator_ix(&fixture, operator.pubkey(), operator_record, 0);
    send(&mut fixture.svm, &fixture.alice, set_operator);
    let amount_handle = input_handle_for_chain(9);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);

    let output = transfer_output_accounts(&fixture, 1);
    let transfer = transfer_from_ix(
        &fixture,
        operator.pubkey(),
        operator_record,
        output,
        amount_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    assert!(
        send_with_signers(&mut fixture.svm, &operator.pubkey(), transfer, &[&operator]).is_err()
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_acl_record,
        fixture.alice_current_compute_acl
    );
    assert_eq!(created_acl_count(&fixture.svm, output), 0);
}

#[test]
fn set_operator_rejects_noncanonical_token_account() {
    let mut fixture = token_fixture();
    let fake_token_account = seed_noncanonical_confidential_token_account(&mut fixture);
    let operator = Pubkey::new_unique();
    let operator_record = token::operator_record_address(fake_token_account, operator).0;
    let set_operator = set_operator_ix_with_token_account(
        &fixture,
        fake_token_account,
        operator,
        operator_record,
        100,
    );

    assert!(try_send(&mut fixture.svm, &fixture.alice, set_operator).is_err());
    assert!(read_operator_record(&fixture.svm, operator_record).is_none());
}

#[test]
fn confidential_token_rejects_unexpected_remaining_account() {
    let mut fixture = token_fixture();
    let operator = Pubkey::new_unique();
    let operator_record = token::operator_record_address(fixture.alice_token, operator).0;
    let unexpected_account = fixture.svm.create_funded_account(1_000_000).unwrap();
    let mut ix = set_operator_ix(&fixture, operator, operator_record, 100);
    ix.accounts.push(AccountMeta::new_readonly(
        unexpected_account.pubkey(),
        false,
    ));

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    assert!(read_operator_record(&fixture.svm, operator_record).is_none());
}

#[test]
fn set_operator_rejects_dirty_operator_record_target() {
    let mut fixture = token_fixture();
    let operator = Pubkey::new_unique();
    let operator_record = token::operator_record_address(fixture.alice_token, operator).0;
    fixture
        .svm
        .set_account(
            operator_record,
            Account {
                lamports: fixture.svm.minimum_balance_for_rent_exemption(1),
                data: vec![1],
                owner: system_program::ID,
                executable: false,
                rent_epoch: 0,
            },
        )
        .unwrap();

    let ix = set_operator_ix(&fixture, operator, operator_record, 100);

    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
    let dirty_target = fixture
        .svm
        .get_account(&operator_record)
        .expect("expected dirty target");
    assert_eq!(dirty_target.owner, system_program::ID);
    assert_eq!(dirty_target.data, vec![1]);
}

#[test]
fn confidential_token_account_rejects_wrong_bump_or_length() {
    let mut fixture = token_fixture();
    let operator = Pubkey::new_unique();
    let operator_record = token::operator_record_address(fixture.alice_token, operator).0;
    let (_, bump) = token::token_account_address(fixture.mint.pubkey(), fixture.alice.pubkey());
    let mut alice_account = token_account(&fixture.svm, fixture.alice_token);
    alice_account.bump = bump.wrapping_add(1);
    seed_confidential_token_account(
        &mut fixture.svm,
        fixture.token_program_id,
        fixture.alice_token,
        &alice_account,
        0,
    );
    let set_operator = set_operator_ix(&fixture, operator, operator_record, 100);
    assert!(try_send(&mut fixture.svm, &fixture.alice, set_operator).is_err());
    assert!(read_operator_record(&fixture.svm, operator_record).is_none());

    alice_account.bump = bump;
    seed_confidential_token_account(
        &mut fixture.svm,
        fixture.token_program_id,
        fixture.alice_token,
        &alice_account,
        1,
    );
    let set_operator = set_operator_ix(&fixture, operator, operator_record, 100);
    assert!(try_send(&mut fixture.svm, &fixture.alice, set_operator).is_err());
    assert!(read_operator_record(&fixture.svm, operator_record).is_none());
    assert_eq!(
        fixture
            .svm
            .get_account(&fixture.alice_token)
            .expect("expected token account")
            .data
            .len(),
        8 + token::ConfidentialTokenAccount::SPACE + 1
    );

    let amount_handle = input_handle_for_chain(92);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer = transfer_ix(&fixture, output, amount_handle);
    assert!(try_send(&mut fixture.svm, &fixture.alice, transfer).is_err());
    assert!(read_acl_record(&fixture.svm, output.bob).is_none());
}

#[test]
fn initialize_mint_rejects_zero_kms_verifier_authority() {
    let host_program_id = host::id();
    let token_program_id = token::id();
    let mut svm = svm_with_programs(&[
        (host_program_id, host_program_so_path()),
        (token_program_id, token_program_so_path()),
    ]);
    let authority = svm.create_funded_account(2_000_000_000).unwrap();
    let verifier = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        host_program_id,
        authority.pubkey(),
        verifier.pubkey(),
        authority.pubkey(),
    );
    let mint = Keypair::new();
    let underlying_mint = svm.create_token_mint(&authority, 6).unwrap();
    let compute_signer = token::compute_signer_address(mint.pubkey()).0;
    let total_supply_authority = token::total_supply_authority_address(mint.pubkey()).0;
    let total_supply_acl_record =
        total_supply_acl_record_address(host_program_id, mint.pubkey(), total_supply_authority, 0);
    svm.set_account(
        Pubkey::default(),
        Account {
            lamports: 1,
            data: Vec::new(),
            owner: system_program::ID,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();

    let ix = anchor_ix(
        token_program_id,
        token::accounts::InitializeMint {
            authority: authority.pubkey(),
            mint: mint.pubkey(),
            underlying_mint: underlying_mint.pubkey(),
            compute_signer,
            total_supply_authority,
            kms_verifier_authority: Pubkey::default(),
            total_supply_acl_record,
            zama_event_authority: event_authority(host_program_id),
            zama_program: host_program_id,
            host_config,
            system_program: system_program::ID,
            event_authority: event_authority(token_program_id),
            program: token_program_id,
        },
        token::instruction::InitializeMint {},
    );

    assert!(send_with_signers(&mut svm, &authority.pubkey(), ix, &[&authority, &mint]).is_err());
    assert!(svm.get_account(&mint.pubkey()).is_none());
    assert!(read_acl_record(&svm, total_supply_acl_record).is_none());
}

#[test]
fn confidential_mint_rejects_wrong_compute_signer_or_length() {
    let mut fixture = token_fixture();
    let operator = Pubkey::new_unique();
    let operator_record = token::operator_record_address(fixture.alice_token, operator).0;
    let mut mint = mint_account(&fixture.svm, fixture.mint.pubkey());
    mint.compute_signer = Pubkey::new_unique();
    seed_confidential_mint(
        &mut fixture.svm,
        fixture.token_program_id,
        fixture.mint.pubkey(),
        &mint,
        0,
    );
    let set_operator = set_operator_ix(&fixture, operator, operator_record, 100);
    assert!(try_send(&mut fixture.svm, &fixture.alice, set_operator).is_err());
    assert!(read_operator_record(&fixture.svm, operator_record).is_none());

    mint.compute_signer = fixture.compute_signer;
    seed_confidential_mint(
        &mut fixture.svm,
        fixture.token_program_id,
        fixture.mint.pubkey(),
        &mint,
        1,
    );
    let set_operator = set_operator_ix(&fixture, operator, operator_record, 100);
    assert!(try_send(&mut fixture.svm, &fixture.alice, set_operator).is_err());
    assert!(read_operator_record(&fixture.svm, operator_record).is_none());
    assert_eq!(
        fixture
            .svm
            .get_account(&fixture.mint.pubkey())
            .expect("expected mint")
            .data
            .len(),
        8 + token::ConfidentialMint::SPACE + 1
    );

    let amount_handle = input_handle_for_chain(93);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer = transfer_ix(&fixture, output, amount_handle);
    assert!(try_send(&mut fixture.svm, &fixture.alice, transfer).is_err());
    assert!(read_acl_record(&fixture.svm, output.bob).is_none());
}

#[test]
fn operator_record_rejects_wrong_bump_or_length() {
    let mut fixture = token_fixture();
    let operator = fixture.svm.create_funded_account(1_000_000_000).unwrap();
    let (operator_record, bump) =
        token::operator_record_address(fixture.alice_token, operator.pubkey());
    let expiration_slot = current_slot(&fixture.svm) + 100;

    seed_operator_record(
        &mut fixture.svm,
        fixture.token_program_id,
        operator_record,
        token::ConfidentialOperator {
            token_account: fixture.alice_token,
            owner: fixture.alice.pubkey(),
            operator: operator.pubkey(),
            expiration_slot,
            bump: bump.wrapping_add(1),
        },
        0,
    );
    let set_operator = set_operator_ix(
        &fixture,
        operator.pubkey(),
        operator_record,
        expiration_slot + 1,
    );
    assert!(try_send(&mut fixture.svm, &fixture.alice, set_operator).is_err());
    let record = read_operator_record(&fixture.svm, operator_record).expect("expected operator");
    assert_ne!(record.bump, bump);

    seed_operator_record(
        &mut fixture.svm,
        fixture.token_program_id,
        operator_record,
        token::ConfidentialOperator {
            token_account: fixture.alice_token,
            owner: fixture.alice.pubkey(),
            operator: operator.pubkey(),
            expiration_slot,
            bump,
        },
        1,
    );
    let set_operator = set_operator_ix(
        &fixture,
        operator.pubkey(),
        operator_record,
        expiration_slot + 1,
    );
    assert!(try_send(&mut fixture.svm, &fixture.alice, set_operator).is_err());
    assert_eq!(
        fixture
            .svm
            .get_account(&operator_record)
            .expect("expected operator")
            .data
            .len(),
        8 + token::ConfidentialOperator::SPACE + 1
    );

    let amount_handle = input_handle_for_chain(91);
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer = transfer_from_ix(
        &fixture,
        operator.pubkey(),
        operator_record,
        output,
        amount_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    assert!(
        send_with_signers(&mut fixture.svm, &operator.pubkey(), transfer, &[&operator]).is_err()
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.alice_token).balance_handle,
        fixture.alice_initial
    );
    assert_eq!(
        token_account(&fixture.svm, fixture.bob_token).balance_handle,
        fixture.bob_initial
    );
    assert!(read_acl_record(&fixture.svm, output.bob).is_none());
}

#[test]
fn close_operator_reclaims_revoked_operator_record_permissionlessly() {
    let mut fixture = token_fixture();
    let operator = fixture.svm.create_funded_account(1_000_000_000).unwrap();
    let cleaner = fixture.svm.create_funded_account(1_000_000_000).unwrap();
    let operator_record = token::operator_record_address(fixture.alice_token, operator.pubkey()).0;
    let set_operator = set_operator_ix(&fixture, operator.pubkey(), operator_record, 0);
    send(&mut fixture.svm, &fixture.alice, set_operator);

    let record_lamports = fixture
        .svm
        .get_account(&operator_record)
        .expect("expected operator record")
        .lamports;
    let owner_lamports_before = fixture
        .svm
        .get_account(&fixture.alice.pubkey())
        .expect("expected owner account")
        .lamports;
    let close = close_operator_ix(
        &fixture,
        None,
        operator.pubkey(),
        operator_record,
        fixture.alice.pubkey(),
    );
    send_with_signers(&mut fixture.svm, &cleaner.pubkey(), close, &[&cleaner]).unwrap();

    assert!(read_operator_record(&fixture.svm, operator_record).is_none());
    let owner_lamports_after = fixture
        .svm
        .get_account(&fixture.alice.pubkey())
        .expect("expected owner account")
        .lamports;
    assert_eq!(
        owner_lamports_after,
        owner_lamports_before + record_lamports
    );
}

#[test]
fn close_operator_requires_owner_for_active_operator() {
    let mut fixture = token_fixture();
    let operator = fixture.svm.create_funded_account(1_000_000_000).unwrap();
    let cleaner = fixture.svm.create_funded_account(1_000_000_000).unwrap();
    let operator_record = token::operator_record_address(fixture.alice_token, operator.pubkey()).0;
    let expiration_slot = current_slot(&fixture.svm) + 100;
    let set_operator = set_operator_ix(
        &fixture,
        operator.pubkey(),
        operator_record,
        expiration_slot,
    );
    send(&mut fixture.svm, &fixture.alice, set_operator);

    let close_without_owner = close_operator_ix(
        &fixture,
        None,
        operator.pubkey(),
        operator_record,
        fixture.alice.pubkey(),
    );
    assert!(send_with_signers(
        &mut fixture.svm,
        &cleaner.pubkey(),
        close_without_owner,
        &[&cleaner],
    )
    .is_err());
    assert!(read_operator_record(&fixture.svm, operator_record).is_some());

    let close_with_owner = close_operator_ix(
        &fixture,
        Some(fixture.alice.pubkey()),
        operator.pubkey(),
        operator_record,
        fixture.alice.pubkey(),
    );
    send_with_signers(
        &mut fixture.svm,
        &cleaner.pubkey(),
        close_with_owner,
        &[&cleaner, &fixture.alice],
    )
    .unwrap();
    assert!(read_operator_record(&fixture.svm, operator_record).is_none());
}

struct TokenFixture {
    svm: LiteSVM,
    host_program_id: Pubkey,
    host_config: Pubkey,
    token_program_id: Pubkey,
    receiver_program_id: Pubkey,
    alice: Keypair,
    bob: Keypair,
    verifier: Keypair,
    mint: Keypair,
    underlying_mint: Keypair,
    compute_signer: Pubkey,
    total_supply_authority: Pubkey,
    alice_token: Pubkey,
    bob_token: Pubkey,
    alice_usdc: Pubkey,
    vault_usdc: Pubkey,
    alice_initial: [u8; 32],
    bob_initial: [u8; 32],
    total_supply_initial: [u8; 32],
    alice_current_compute_acl: Pubkey,
    bob_current_compute_acl: Pubkey,
    total_supply_current_acl: Pubkey,
}

#[derive(Clone, Copy)]
struct TransferOutputAccounts {
    alice: Pubkey,
    bob: Pubkey,
    success: Pubkey,
    debit_candidate: Pubkey,
    transferred: Pubkey,
}

#[derive(Clone, Copy)]
struct CallbackSettlementOutputAccounts {
    settlement: Pubkey,
    zero: Pubkey,
    requested_refund: Pubkey,
    refund_success: Pubkey,
    refund_debit_candidate: Pubkey,
    bob: Pubkey,
    refund: Pubkey,
    alice: Pubkey,
    transferred: Pubkey,
}

#[derive(Clone, Copy)]
struct WrapOutputAccounts {
    amount: Pubkey,
    balance: Pubkey,
    total_supply: Pubkey,
}

#[derive(Clone, Copy)]
struct BurnOutputAccounts {
    balance: Pubkey,
    success: Pubkey,
    debit_candidate: Pubkey,
    burned: Pubkey,
    total_supply: Pubkey,
}

#[derive(Clone)]
struct UserDecryptAuthorizationPayload {
    user: Pubkey,
    reencryption_public_key: [u8; 32],
    allowed_acl_domain_keys: Vec<Pubkey>,
    start_timestamp: i64,
    duration_seconds: u64,
    extra_data: Vec<u8>,
}

#[derive(Clone)]
struct UserDecryptRequest {
    authorization: UserDecryptAuthorizationPayload,
    signature: Signature,
    handles: Vec<UserDecryptHandleEntry>,
}

#[derive(Clone, Copy)]
struct UserDecryptHandleEntry {
    handle: [u8; 32],
    owner: Pubkey,
    acl_record: Pubkey,
}

#[derive(Clone, Copy)]
struct PublicDecryptHandleEntry {
    handle: [u8; 32],
    acl_record: Pubkey,
}

#[derive(Clone, Copy)]
struct PublicDecryptWithMaterialEntry {
    handle: [u8; 32],
    acl_record: Pubkey,
    material_commitment: Pubkey,
}

fn host_program_so_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy/zama_host.so")
}

fn token_program_so_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy/confidential_token.so")
}

fn receiver_program_so_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../target/deploy/confidential_token_receiver.so")
}

fn svm_with_program(program_id: Pubkey, program_path: PathBuf) -> LiteSVM {
    svm_with_programs(&[(program_id, program_path)])
}

fn svm_with_programs(programs: &[(Pubkey, PathBuf)]) -> LiteSVM {
    for (_, path) in programs {
        assert!(
            path.exists(),
            "missing {}; run `cd solana && NO_DNA=1 anchor build --ignore-keys` before this runtime test",
            path.display()
        );
    }

    let program_bytes = programs
        .iter()
        .map(|(program_id, path)| (*program_id, std::fs::read(path).unwrap()))
        .collect::<Vec<_>>();
    let programs = program_bytes
        .iter()
        .map(|(program_id, bytes)| (*program_id, bytes.as_slice()))
        .collect::<Vec<_>>();
    AnchorLiteSVM::build_with_programs(&programs)
        .svm
        .with_log_bytes_limit(None)
}

fn seed_host_config(
    svm: &mut LiteSVM,
    program_id: Pubkey,
    admin: Pubkey,
    input_verifier_authority: Pubkey,
    test_authority: Pubkey,
) -> Pubkey {
    seed_host_config_with_flags(
        svm,
        program_id,
        admin,
        input_verifier_authority,
        test_authority,
        true,
        true,
        false,
    )
}

fn seed_host_config_with_flags(
    svm: &mut LiteSVM,
    program_id: Pubkey,
    admin: Pubkey,
    input_verifier_authority: Pubkey,
    test_authority: Pubkey,
    mock_input_enabled: bool,
    test_shims_enabled: bool,
    grant_deny_list_enabled: bool,
) -> Pubkey {
    let (host_config, bump) = Pubkey::find_program_address(&[host::HOST_CONFIG_SEED], &program_id);
    svm.set_account(
        host_config,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(HostConfig {
                admin,
                chain_id: host::SOLANA_POC_CHAIN_ID,
                input_verifier_authority,
                material_authority: input_verifier_authority,
                test_authority,
                paused: false,
                mock_input_enabled,
                test_shims_enabled,
                grant_deny_list_enabled,
                updated_slot: current_slot(svm),
                bump,
            }),
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
    host_config
}

fn extend_host_config(svm: &mut LiteSVM, host_config: Pubkey, extra_bytes: usize) {
    let mut account = svm.get_account(&host_config).expect("expected host config");
    account.data.resize(account.data.len() + extra_bytes, 0);
    account.lamports = svm.minimum_balance_for_rent_exemption(account.data.len());
    svm.set_account(host_config, account).unwrap();
}

fn token_fixture() -> TokenFixture {
    let host_program_id = host::id();
    let token_program_id = token::id();
    let receiver_program_id = receiver::id();
    let mut svm = svm_with_programs(&[
        (host_program_id, host_program_so_path()),
        (token_program_id, token_program_so_path()),
        (receiver_program_id, receiver_program_so_path()),
    ]);

    let alice = svm.create_funded_account(2_000_000_000).unwrap();
    let bob = svm.create_funded_account(1_000_000_000).unwrap();
    let verifier = svm.create_funded_account(1_000_000_000).unwrap();
    let host_config = seed_host_config(
        &mut svm,
        host_program_id,
        alice.pubkey(),
        verifier.pubkey(),
        alice.pubkey(),
    );
    let mint = Keypair::new();
    let underlying_mint = svm.create_token_mint(&alice, 6).unwrap();

    let vault_authority = vault_authority_address(token_program_id, mint.pubkey());
    let alice_usdc = svm
        .create_token_account(&underlying_mint.pubkey(), &alice)
        .unwrap();
    let vault_usdc = create_associated_spl_token_account(
        &mut svm,
        &alice,
        underlying_mint.pubkey(),
        vault_authority,
    );
    svm.mint_to(
        &underlying_mint.pubkey(),
        &alice_usdc.pubkey(),
        &alice,
        1_000_000_000,
    )
    .unwrap();
    let compute_signer = token::compute_signer_address(mint.pubkey()).0;
    let total_supply_authority = token::total_supply_authority_address(mint.pubkey()).0;
    let total_supply_current_acl =
        total_supply_acl_record_address(host_program_id, mint.pubkey(), total_supply_authority, 0);

    send_with_signers(
        &mut svm,
        &alice.pubkey(),
        anchor_ix(
            token_program_id,
            token::accounts::InitializeMint {
                authority: alice.pubkey(),
                mint: mint.pubkey(),
                underlying_mint: underlying_mint.pubkey(),
                compute_signer,
                total_supply_authority,
                kms_verifier_authority: verifier.pubkey(),
                total_supply_acl_record: total_supply_current_acl,
                zama_event_authority: event_authority(host_program_id),
                zama_program: host_program_id,
                host_config,
                system_program: system_program::ID,
                event_authority: event_authority(token_program_id),
                program: token_program_id,
            },
            token::instruction::InitializeMint {},
        ),
        &[&alice, &mint],
    )
    .unwrap();

    let alice_token = token_account_address(token_program_id, mint.pubkey(), alice.pubkey());
    let bob_token = token_account_address(token_program_id, mint.pubkey(), bob.pubkey());
    let alice_current_compute_acl =
        balance_acl_record_address(host_program_id, mint.pubkey(), alice_token, 0);
    let bob_current_compute_acl =
        balance_acl_record_address(host_program_id, mint.pubkey(), bob_token, 0);

    initialize_confidential_token_account(
        &mut svm,
        token_program_id,
        host_program_id,
        host_config,
        &alice,
        mint.pubkey(),
        alice_token,
        compute_signer,
        alice_current_compute_acl,
        0,
    );
    initialize_confidential_token_account(
        &mut svm,
        token_program_id,
        host_program_id,
        host_config,
        &bob,
        mint.pubkey(),
        bob_token,
        compute_signer,
        bob_current_compute_acl,
        0,
    );
    let alice_initial = read_acl_record(&svm, alice_current_compute_acl)
        .expect("expected Alice initial ACL")
        .handle;
    let bob_initial = read_acl_record(&svm, bob_current_compute_acl)
        .expect("expected Bob initial ACL")
        .handle;
    let total_supply_initial = read_acl_record(&svm, total_supply_current_acl)
        .expect("expected total supply initial ACL")
        .handle;

    TokenFixture {
        svm,
        host_program_id,
        host_config,
        token_program_id,
        receiver_program_id,
        alice,
        bob,
        verifier,
        mint,
        underlying_mint,
        compute_signer,
        total_supply_authority,
        alice_token,
        bob_token,
        alice_usdc: alice_usdc.pubkey(),
        vault_usdc,
        alice_initial,
        bob_initial,
        total_supply_initial,
        alice_current_compute_acl,
        bob_current_compute_acl,
        total_supply_current_acl,
    }
}

fn initialize_confidential_token_account(
    svm: &mut LiteSVM,
    token_program_id: Pubkey,
    host_program_id: Pubkey,
    host_config: Pubkey,
    owner: &Keypair,
    mint: Pubkey,
    token_account: Pubkey,
    compute_signer: Pubkey,
    acl_record: Pubkey,
    initial_balance: u64,
) {
    send(
        svm,
        owner,
        anchor_ix(
            token_program_id,
            token::accounts::InitializeTokenAccount {
                owner: owner.pubkey(),
                mint,
                compute_signer,
                token_account,
                acl_record,
                zama_event_authority: event_authority(host_program_id),
                zama_program: host_program_id,
                host_config,
                system_program: system_program::ID,
                event_authority: event_authority(token_program_id),
                program: token_program_id,
            },
            token::instruction::InitializeTokenAccount { initial_balance },
        ),
    );
}

fn wrap_and_burn_for_redeem(
    fixture: &mut TokenFixture,
    burn_handle_seed: u8,
) -> (BurnOutputAccounts, [u8; 32]) {
    let wrap_output = wrap_output_accounts(fixture, 1);
    let wrap_ix = wrap_usdc_ix(fixture, wrap_output, 100_000_000);
    send(&mut fixture.svm, &fixture.alice, wrap_ix);

    let burn_amount_handle = input_handle_for_chain(burn_handle_seed);
    authorize_burn_input_compute_acl(fixture, burn_amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = burn_output_accounts(fixture, 2, 2);
    let burn_ix = burn_ix_with_current_acls(
        fixture,
        wrap_output.balance,
        wrap_output.total_supply,
        output,
        burn_amount_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    );
    send(&mut fixture.svm, &fixture.alice, burn_ix);
    let burned = read_acl_record(&fixture.svm, output.burned)
        .expect("expected burned amount ACL")
        .handle;
    (output, burned)
}

fn release_burned_amount_for_redeem(
    fixture: &mut TokenFixture,
    burned_acl_record: Pubkey,
    burned_handle: [u8; 32],
) {
    let release_ix = request_disclose_amount_ix(
        fixture,
        fixture.alice.pubkey(),
        burned_acl_record,
        burned_handle,
    );
    send(&mut fixture.svm, &fixture.alice, release_ix);
}

fn event_authority(program_id: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"__event_authority"], &program_id).0
}

fn token_account_address(program_id: Pubkey, mint: Pubkey, owner: Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"token-account", mint.as_ref(), owner.as_ref()],
        &program_id,
    )
    .0
}

fn vault_authority_address(program_id: Pubkey, mint: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"vault-authority", mint.as_ref()], &program_id).0
}

fn acl_record_address(program_id: Pubkey, nonce_key: [u8; 32], nonce_sequence: u64) -> Pubkey {
    Pubkey::find_program_address(
        &[
            b"acl-record",
            nonce_key.as_ref(),
            &nonce_sequence.to_le_bytes(),
        ],
        &program_id,
    )
    .0
}

fn balance_acl_record_address(
    program_id: Pubkey,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    nonce_sequence: u64,
) -> Pubkey {
    acl_record_address(
        program_id,
        token::balance_nonce_key(acl_domain_key, app_account),
        nonce_sequence,
    )
}

fn total_supply_acl_record_address(
    program_id: Pubkey,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    nonce_sequence: u64,
) -> Pubkey {
    acl_record_address(
        program_id,
        token::total_supply_nonce_key(acl_domain_key, app_account),
        nonce_sequence,
    )
}

fn input_compute_acl_address(fixture: &TokenFixture, nonce_sequence: u64) -> Pubkey {
    input_compute_acl_address_with_label(fixture, token::transfer_amount_label(), nonce_sequence)
}

fn input_compute_acl_address_for_authority(
    fixture: &TokenFixture,
    authority: Pubkey,
    nonce_sequence: u64,
) -> Pubkey {
    input_compute_acl_address_with_label_for_authority(
        fixture,
        authority,
        token::transfer_amount_label(),
        nonce_sequence,
    )
}

fn burn_input_compute_acl_address(fixture: &TokenFixture, nonce_sequence: u64) -> Pubkey {
    input_compute_acl_address_with_label(fixture, token::burn_amount_label(), nonce_sequence)
}

fn random_amount_acl_address(
    fixture: &TokenFixture,
    owner: Pubkey,
    amount_kind: ConfidentialAmountKind,
    nonce_sequence: u64,
) -> Pubkey {
    let encrypted_value_label = match amount_kind {
        ConfidentialAmountKind::Transfer => token::transfer_amount_label(),
        ConfidentialAmountKind::Burn => token::burn_amount_label(),
    };
    acl_record_address(
        fixture.host_program_id,
        token::nonce_key(fixture.mint.pubkey(), owner, encrypted_value_label),
        nonce_sequence,
    )
}

fn input_compute_acl_address_with_label(
    fixture: &TokenFixture,
    encrypted_value_label: [u8; 32],
    nonce_sequence: u64,
) -> Pubkey {
    input_compute_acl_address_with_label_for_authority(
        fixture,
        fixture.alice.pubkey(),
        encrypted_value_label,
        nonce_sequence,
    )
}

fn input_compute_acl_address_with_label_for_authority(
    fixture: &TokenFixture,
    authority: Pubkey,
    encrypted_value_label: [u8; 32],
    nonce_sequence: u64,
) -> Pubkey {
    acl_record_address(
        fixture.host_program_id,
        token::nonce_key(fixture.mint.pubkey(), authority, encrypted_value_label),
        nonce_sequence,
    )
}

fn transfer_output_accounts(fixture: &TokenFixture, nonce_sequence: u64) -> TransferOutputAccounts {
    TransferOutputAccounts {
        alice: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.alice_token,
            nonce_sequence,
        ),
        bob: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.bob_token,
            nonce_sequence,
        ),
        success: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.alice_token,
                token::transfer_success_label(),
            ),
            nonce_sequence,
        ),
        debit_candidate: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.alice_token,
                token::debit_candidate_label(),
            ),
            nonce_sequence,
        ),
        transferred: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.alice_token,
                token::transferred_amount_label(),
            ),
            nonce_sequence,
        ),
    }
}

fn callback_settlement_output_accounts(
    fixture: &TokenFixture,
    sent_handle: [u8; 32],
    from_nonce_sequence: u64,
    to_nonce_sequence: u64,
) -> CallbackSettlementOutputAccounts {
    CallbackSettlementOutputAccounts {
        settlement: token::transfer_callback_settlement_address(fixture.mint.pubkey(), sent_handle)
            .0,
        zero: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.bob_token,
                token::callback_zero_label(),
            ),
            to_nonce_sequence,
        ),
        requested_refund: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.bob_token,
                token::callback_refund_request_label(),
            ),
            to_nonce_sequence,
        ),
        refund_success: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.bob_token,
                token::callback_refund_success_label(),
            ),
            to_nonce_sequence,
        ),
        refund_debit_candidate: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.bob_token,
                token::callback_refund_debit_candidate_label(),
            ),
            to_nonce_sequence,
        ),
        bob: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.bob_token,
            to_nonce_sequence,
        ),
        refund: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.bob_token,
                token::callback_refund_amount_label(),
            ),
            to_nonce_sequence,
        ),
        alice: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.alice_token,
            from_nonce_sequence,
        ),
        transferred: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.alice_token,
                token::callback_final_transferred_label(),
            ),
            from_nonce_sequence,
        ),
    }
}

fn callback_success_acl_address(fixture: &TokenFixture, nonce_sequence: u64) -> Pubkey {
    acl_record_address(
        fixture.host_program_id,
        token::nonce_key(
            fixture.mint.pubkey(),
            fixture.bob.pubkey(),
            token::callback_success_label(),
        ),
        nonce_sequence,
    )
}

fn authorize_input_compute_acl(fixture: &mut TokenFixture, handle: [u8; 32], nonce_sequence: u64) {
    authorize_input_compute_acl_with_label(
        fixture,
        handle,
        nonce_sequence,
        token::transfer_amount_label(),
        b"confidential-token-transfer-amount",
    )
}

fn authorize_transfer_amount_compute_acl_for_signer(
    fixture: &mut TokenFixture,
    authority: &Keypair,
    handle: [u8; 32],
    nonce_sequence: u64,
) {
    let acl_domain_key = fixture.mint.pubkey();
    let app_account = authority.pubkey();
    let encrypted_value_label = token::transfer_amount_label();
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let acl_record = acl_record_address(fixture.host_program_id, nonce_key, nonce_sequence);
    let proof = host::SolanaInputProof {
        handles: vec![handle],
        handle_index: 0,
        user: authority.pubkey(),
        app_account,
        acl_domain_key,
        extra_data: b"confidential-token-transfer-amount".to_vec(),
    };
    let output_subjects = vec![AclSubjectEntry::compute(fixture.compute_signer)];
    let bind_intent = input_bind_intent(
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        output_subjects.clone(),
        false,
    );
    let ed25519_ix = ed25519_verify_ix(
        &fixture.verifier,
        &host::input_proof_message(
            &proof,
            &bind_intent,
            fixture.host_program_id,
            host::SOLANA_POC_CHAIN_ID,
        ),
    );
    let bind_ix = verify_input_and_bind_ix(
        fixture.host_program_id,
        authority.pubkey(),
        fixture.verifier.pubkey(),
        fixture.host_config,
        acl_record,
        handle,
        proof,
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        output_subjects,
        false,
    );
    send_many_with_signers(
        &mut fixture.svm,
        &authority.pubkey(),
        vec![ed25519_ix, bind_ix],
        &[authority],
    )
    .unwrap();
}

fn authorize_burn_input_compute_acl(
    fixture: &mut TokenFixture,
    handle: [u8; 32],
    nonce_sequence: u64,
) {
    authorize_input_compute_acl_with_label(
        fixture,
        handle,
        nonce_sequence,
        token::burn_amount_label(),
        b"confidential-token-burn-amount",
    )
}

fn authorize_callback_success_acl(
    fixture: &mut TokenFixture,
    handle: [u8; 32],
    nonce_sequence: u64,
) {
    let acl_domain_key = fixture.mint.pubkey();
    let app_account = fixture.bob.pubkey();
    let encrypted_value_label = token::callback_success_label();
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let acl_record = acl_record_address(fixture.host_program_id, nonce_key, nonce_sequence);
    let proof = host::SolanaInputProof {
        handles: vec![handle],
        handle_index: 0,
        user: fixture.bob.pubkey(),
        app_account,
        acl_domain_key,
        extra_data: b"confidential-token-callback-success".to_vec(),
    };
    let output_subjects = vec![AclSubjectEntry::compute(fixture.compute_signer)];
    let bind_intent = input_bind_intent(
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        output_subjects.clone(),
        false,
    );
    let ed25519_ix = ed25519_verify_ix(
        &fixture.verifier,
        &host::input_proof_message(
            &proof,
            &bind_intent,
            fixture.host_program_id,
            host::SOLANA_POC_CHAIN_ID,
        ),
    );
    let bind_ix = verify_input_and_bind_ix(
        fixture.host_program_id,
        fixture.bob.pubkey(),
        fixture.verifier.pubkey(),
        fixture.host_config,
        acl_record,
        handle,
        proof,
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        output_subjects,
        false,
    );
    send_many_with_signers(
        &mut fixture.svm,
        &fixture.bob.pubkey(),
        vec![ed25519_ix, bind_ix],
        &[&fixture.bob],
    )
    .unwrap();
}

fn authorize_input_compute_acl_with_label(
    fixture: &mut TokenFixture,
    handle: [u8; 32],
    nonce_sequence: u64,
    encrypted_value_label: [u8; 32],
    extra_data: &[u8],
) {
    let acl_domain_key = fixture.mint.pubkey();
    let app_account = fixture.alice.pubkey();
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let acl_record = acl_record_address(fixture.host_program_id, nonce_key, nonce_sequence);
    let proof = host::SolanaInputProof {
        handles: vec![handle],
        handle_index: 0,
        user: fixture.alice.pubkey(),
        app_account,
        acl_domain_key,
        extra_data: extra_data.to_vec(),
    };
    let output_subjects = vec![AclSubjectEntry::compute(fixture.compute_signer)];
    let bind_intent = input_bind_intent(
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        output_subjects.clone(),
        false,
    );
    let ed25519_ix = ed25519_verify_ix(
        &fixture.verifier,
        &host::input_proof_message(
            &proof,
            &bind_intent,
            fixture.host_program_id,
            host::SOLANA_POC_CHAIN_ID,
        ),
    );
    let bind_ix = verify_input_and_bind_ix(
        fixture.host_program_id,
        fixture.alice.pubkey(),
        fixture.verifier.pubkey(),
        fixture.host_config,
        acl_record,
        handle,
        proof,
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        output_subjects,
        false,
    );
    send_many_with_signers(
        &mut fixture.svm,
        &fixture.alice.pubkey(),
        vec![ed25519_ix, bind_ix],
        &[&fixture.alice],
    )
    .unwrap();
}

fn set_deny_subject_ix(
    program_id: Pubkey,
    payer: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    deny_subject_record: Pubkey,
    subject: Pubkey,
    denied: bool,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::SetDenySubject {
            payer,
            admin,
            host_config,
            deny_subject_record,
            system_program: system_program::ID,
        },
        host::instruction::SetDenySubject { subject, denied },
    )
}

fn set_host_pause_ix(
    program_id: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    paused: bool,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::HostAdmin { admin, host_config },
        host::instruction::SetHostPause { paused },
    )
}

fn set_mock_input_enabled_ix(
    program_id: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    enabled: bool,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::HostAdmin { admin, host_config },
        host::instruction::SetMockInputEnabled { enabled },
    )
}

fn set_test_shims_enabled_ix(
    program_id: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    enabled: bool,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::HostAdmin { admin, host_config },
        host::instruction::SetTestShimsEnabled { enabled },
    )
}

fn set_grant_deny_list_enabled_ix(
    program_id: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    enabled: bool,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::HostAdmin { admin, host_config },
        host::instruction::SetGrantDenyListEnabled { enabled },
    )
}

fn allow_acl_subjects_ix(
    program_id: Pubkey,
    authority: Pubkey,
    host_config: Pubkey,
    acl_record: Pubkey,
    handle: [u8; 32],
    subject: Pubkey,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer: authority,
            authority,
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry::user(subject)],
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn allow_acl_subjects_with_permission_ix(
    program_id: Pubkey,
    authority: Pubkey,
    host_config: Pubkey,
    acl_record: Pubkey,
    permission_record: Pubkey,
    handle: [u8; 32],
    subject: Pubkey,
    role_flags: u8,
) -> Instruction {
    let mut ix = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer: authority,
            authority,
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry {
                pubkey: subject,
                role_flags,
            }],
        },
    );
    ix.accounts.push(AccountMeta::new(permission_record, false));
    ix
}

#[allow(clippy::too_many_arguments)]
fn assert_acl_record_ix(
    program_id: Pubkey,
    acl_record: Pubkey,
    subject_permission_record: Option<Pubkey>,
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subject: Pubkey,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::AssertAclRecord {
            acl_record,
            subject_permission_record,
        },
        host::instruction::AssertAclRecord {
            nonce_key,
            nonce_sequence,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            handle,
            subject,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn trivial_encrypt_and_bind_ix(
    program_id: Pubkey,
    payer: Pubkey,
    host_config: Pubkey,
    output_acl_record: Pubkey,
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
    output_encrypted_value_label: [u8; 32],
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
    amount: u64,
) -> Instruction {
    trivial_encrypt_and_bind_ix_with_public_decrypt(
        program_id,
        payer,
        host_config,
        output_acl_record,
        output_acl_domain_key,
        output_app_account,
        output_encrypted_value_label,
        output_nonce_key,
        output_nonce_sequence,
        amount,
        false,
    )
}

#[allow(clippy::too_many_arguments)]
fn trivial_encrypt_and_bind_ix_with_public_decrypt(
    program_id: Pubkey,
    payer: Pubkey,
    host_config: Pubkey,
    output_acl_record: Pubkey,
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
    output_encrypted_value_label: [u8; 32],
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
    amount: u64,
    output_public_decrypt: bool,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::TrivialEncryptAndBind {
            payer,
            compute_subject: payer,
            app_account_authority: output_app_account,
            host_config,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::TrivialEncryptAndBind {
            plaintext: amount_plaintext(amount),
            fhe_type: 5,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(payer)],
            output_public_decrypt,
        },
    )
}

fn mock_input_verified_ix(
    program_id: Pubkey,
    payer: Pubkey,
    input_verifier_authority: Pubkey,
    host_config: Pubkey,
    nonce_sequence: u64,
) -> Instruction {
    mock_input_verified_ix_with_handle(
        program_id,
        payer,
        input_verifier_authority,
        host_config,
        nonce_sequence,
        [7; 32],
    )
}

fn mock_input_verified_ix_with_handle(
    program_id: Pubkey,
    payer: Pubkey,
    input_verifier_authority: Pubkey,
    host_config: Pubkey,
    nonce_sequence: u64,
    input_handle: [u8; 32],
) -> Instruction {
    let acl_domain_key = payer;
    let app_account = payer;
    let encrypted_value_label = label("input-gate");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let acl_record = acl_record_address(program_id, nonce_key, nonce_sequence);
    anchor_ix(
        program_id,
        host::accounts::MockInputVerifiedAndBind {
            payer,
            input_verifier_authority,
            app_account_authority: app_account,
            host_config,
            output_acl_record: acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::MockInputVerifiedAndBind {
            input_handle,
            user: payer,
            output_nonce_key: nonce_key,
            output_nonce_sequence: nonce_sequence,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::compute(payer)],
            output_public_decrypt: false,
        },
    )
}

fn verify_input_and_bind_ix(
    program_id: Pubkey,
    payer: Pubkey,
    input_verifier_authority: Pubkey,
    host_config: Pubkey,
    output_acl_record: Pubkey,
    input_handle: [u8; 32],
    proof: host::SolanaInputProof,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
    output_encrypted_value_label: [u8; 32],
    output_subjects: Vec<AclSubjectEntry>,
    output_public_decrypt: bool,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::VerifyInputAndBind {
            payer,
            input_verifier_authority,
            app_account_authority: output_app_account,
            host_config,
            output_acl_record,
            instructions_sysvar: sysvar::instructions::ID,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::VerifyInputAndBind {
            input_handle,
            proof,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            output_subjects,
            output_public_decrypt,
        },
    )
}

fn input_bind_intent(
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
    output_encrypted_value_label: [u8; 32],
    output_subjects: Vec<AclSubjectEntry>,
    output_public_decrypt: bool,
) -> host::SolanaInputBindIntent {
    host::SolanaInputBindIntent {
        output_nonce_key,
        output_nonce_sequence,
        output_acl_domain_key,
        output_app_account,
        output_encrypted_value_label,
        output_subjects,
        output_public_decrypt,
    }
}

fn wrap_output_accounts(fixture: &TokenFixture, nonce_sequence: u64) -> WrapOutputAccounts {
    WrapOutputAccounts {
        amount: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.alice_token,
                token::wrap_amount_label(),
            ),
            nonce_sequence,
        ),
        balance: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.alice_token,
            nonce_sequence,
        ),
        total_supply: total_supply_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.total_supply_authority,
            nonce_sequence,
        ),
    }
}

fn burn_output_accounts(
    fixture: &TokenFixture,
    balance_nonce_sequence: u64,
    total_supply_nonce_sequence: u64,
) -> BurnOutputAccounts {
    BurnOutputAccounts {
        balance: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.alice_token,
            balance_nonce_sequence,
        ),
        success: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.alice_token,
                token::burn_success_label(),
            ),
            balance_nonce_sequence,
        ),
        debit_candidate: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.alice_token,
                token::burn_debit_candidate_label(),
            ),
            balance_nonce_sequence,
        ),
        burned: acl_record_address(
            fixture.host_program_id,
            token::nonce_key(
                fixture.mint.pubkey(),
                fixture.alice_token,
                token::burned_amount_label(),
            ),
            balance_nonce_sequence,
        ),
        total_supply: total_supply_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.total_supply_authority,
            total_supply_nonce_sequence,
        ),
    }
}

fn transfer_ix(
    fixture: &TokenFixture,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    transfer_ix_with_amount_nonce(fixture, output, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE)
}

fn transfer_ix_with_amount_nonce(
    fixture: &TokenFixture,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
    amount_nonce_sequence: u64,
) -> Instruction {
    transfer_ix_with_current_acl_and_amount_nonce(
        fixture,
        fixture.alice_current_compute_acl,
        fixture.bob_current_compute_acl,
        output,
        amount_handle,
        amount_nonce_sequence,
    )
}

fn self_transfer_ix(
    fixture: &TokenFixture,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        fixture.token_program_id,
        token::accounts::ConfidentialTransfer {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            from_account: fixture.alice_token,
            to_account: fixture.alice_token,
            compute_signer: fixture.compute_signer,
            from_current_compute_acl: fixture.alice_current_compute_acl,
            to_current_compute_acl: fixture.alice_current_compute_acl,
            amount_compute_acl: input_compute_acl_address(fixture, DEFAULT_INPUT_NONCE_SEQUENCE),
            transfer_success_acl: output.success,
            debit_candidate_acl: output.debit_candidate,
            from_output_acl: output.alice,
            transferred_amount_acl: output.transferred,
            to_output_acl: output.bob,
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::ConfidentialTransfer { amount_handle },
    )
}

fn transfer_ix_with_current_acl(
    fixture: &TokenFixture,
    from_current_compute_acl: Pubkey,
    to_current_compute_acl: Pubkey,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    transfer_ix_with_current_acl_and_amount_nonce(
        fixture,
        from_current_compute_acl,
        to_current_compute_acl,
        output,
        amount_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    )
}

fn transfer_ix_with_current_acl_and_amount_nonce(
    fixture: &TokenFixture,
    from_current_compute_acl: Pubkey,
    to_current_compute_acl: Pubkey,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
    amount_nonce_sequence: u64,
) -> Instruction {
    transfer_ix_with_amount_acl(
        fixture,
        from_current_compute_acl,
        to_current_compute_acl,
        input_compute_acl_address(fixture, amount_nonce_sequence),
        output,
        amount_handle,
    )
}

fn transfer_ix_with_amount_acl(
    fixture: &TokenFixture,
    from_current_compute_acl: Pubkey,
    to_current_compute_acl: Pubkey,
    amount_compute_acl: Pubkey,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        fixture.token_program_id,
        token::accounts::ConfidentialTransfer {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            from_account: fixture.alice_token,
            to_account: fixture.bob_token,
            compute_signer: fixture.compute_signer,
            from_current_compute_acl,
            to_current_compute_acl,
            amount_compute_acl,
            transfer_success_acl: output.success,
            debit_candidate_acl: output.debit_candidate,
            from_output_acl: output.alice,
            transferred_amount_acl: output.transferred,
            to_output_acl: output.bob,
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::ConfidentialTransfer { amount_handle },
    )
}

fn prepare_transfer_callback_ix(
    fixture: &TokenFixture,
    to_current_compute_acl: Pubkey,
    sent_amount_acl: Pubkey,
    sent_handle: [u8; 32],
    callback_success_acl: Pubkey,
    callback_success_handle: [u8; 32],
    output: CallbackSettlementOutputAccounts,
) -> Instruction {
    prepare_transfer_callback_ix_with_payer(
        fixture,
        fixture.bob.pubkey(),
        to_current_compute_acl,
        sent_amount_acl,
        sent_handle,
        callback_success_acl,
        callback_success_handle,
        output,
    )
}

#[allow(clippy::too_many_arguments)]
fn prepare_transfer_callback_ix_with_payer(
    fixture: &TokenFixture,
    payer: Pubkey,
    to_current_compute_acl: Pubkey,
    sent_amount_acl: Pubkey,
    sent_handle: [u8; 32],
    callback_success_acl: Pubkey,
    callback_success_handle: [u8; 32],
    output: CallbackSettlementOutputAccounts,
) -> Instruction {
    let hook_record = token::transfer_receiver_hook_address(fixture.mint.pubkey(), sent_handle).0;
    anchor_ix(
        fixture.token_program_id,
        token::accounts::ConfidentialPrepareTransferCallback {
            payer,
            callback_authority: fixture.bob.pubkey(),
            mint: fixture.mint.pubkey(),
            from_account: fixture.alice_token,
            to_account: fixture.bob_token,
            compute_signer: fixture.compute_signer,
            to_current_compute_acl,
            sent_amount_acl,
            callback_success_acl,
            hook_record,
            settlement_record: output.settlement,
            callback_zero_acl: output.zero,
            requested_refund_acl: output.requested_refund,
            refund_success_acl: output.refund_success,
            refund_debit_candidate_acl: output.refund_debit_candidate,
            to_output_acl: output.bob,
            refund_amount_acl: output.refund,
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::ConfidentialPrepareTransferCallback {
            sent_handle,
            callback_success_handle,
        },
    )
}

fn accept_transfer_receiver_hook_ix(
    fixture: &TokenFixture,
    sent_amount_acl: Pubkey,
    sent_handle: [u8; 32],
    callback_success_acl: Pubkey,
    callback_success_handle: [u8; 32],
) -> Instruction {
    let receiver_data = receiver::instruction::AcceptConfidentialTransfer {
        mint: fixture.mint.pubkey(),
        from_token_account: fixture.alice_token,
        to_token_account: fixture.bob_token,
        sent_handle,
        sent_acl_record: sent_amount_acl,
        callback_success_handle,
        callback_success_acl_record: callback_success_acl,
    }
    .data();
    call_transfer_receiver_ix(
        fixture,
        sent_amount_acl,
        sent_handle,
        callback_success_acl,
        callback_success_handle,
        fixture.receiver_program_id,
        receiver_data,
    )
}

fn call_transfer_receiver_ix(
    fixture: &TokenFixture,
    sent_amount_acl: Pubkey,
    sent_handle: [u8; 32],
    callback_success_acl: Pubkey,
    callback_success_handle: [u8; 32],
    receiver_program: Pubkey,
    receiver_instruction_data: Vec<u8>,
) -> Instruction {
    let hook_record = token::transfer_receiver_hook_address(fixture.mint.pubkey(), sent_handle).0;
    anchor_ix(
        fixture.token_program_id,
        token::accounts::ConfidentialCallTransferReceiver {
            caller: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            from_account: fixture.alice_token,
            to_account: fixture.bob_token,
            compute_signer: fixture.compute_signer,
            sent_amount_acl,
            callback_success_acl,
            receiver_program,
            instructions_sysvar: sysvar::instructions::ID,
            hook_record,
            system_program: system_program::ID,
        },
        token::instruction::ConfidentialCallTransferReceiver {
            sent_handle,
            callback_success_handle,
            receiver_instruction_data,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn call_transfer_receiver_from_ix(
    fixture: &TokenFixture,
    operator: Pubkey,
    operator_record: Pubkey,
    sent_amount_acl: Pubkey,
    sent_handle: [u8; 32],
    callback_success_acl: Pubkey,
    callback_success_handle: [u8; 32],
    receiver_program: Pubkey,
    receiver_instruction_data: Vec<u8>,
) -> Instruction {
    let hook_record = token::transfer_receiver_hook_address(fixture.mint.pubkey(), sent_handle).0;
    anchor_ix(
        fixture.token_program_id,
        token::accounts::ConfidentialCallTransferReceiverFrom {
            operator,
            mint: fixture.mint.pubkey(),
            from_account: fixture.alice_token,
            to_account: fixture.bob_token,
            operator_record,
            compute_signer: fixture.compute_signer,
            sent_amount_acl,
            callback_success_acl,
            receiver_program,
            instructions_sysvar: sysvar::instructions::ID,
            hook_record,
            system_program: system_program::ID,
        },
        token::instruction::ConfidentialCallTransferReceiverFrom {
            sent_handle,
            callback_success_handle,
            receiver_instruction_data,
        },
    )
}

fn finalize_transfer_callback_ix(
    fixture: &TokenFixture,
    from_current_compute_acl: Pubkey,
    sent_amount_acl: Pubkey,
    output: CallbackSettlementOutputAccounts,
) -> Instruction {
    finalize_transfer_callback_ix_with_payer(
        fixture,
        fixture.bob.pubkey(),
        from_current_compute_acl,
        sent_amount_acl,
        output,
    )
}

fn finalize_transfer_callback_ix_with_payer(
    fixture: &TokenFixture,
    payer: Pubkey,
    from_current_compute_acl: Pubkey,
    sent_amount_acl: Pubkey,
    output: CallbackSettlementOutputAccounts,
) -> Instruction {
    anchor_ix(
        fixture.token_program_id,
        token::accounts::ConfidentialFinalizeTransferCallback {
            payer,
            mint: fixture.mint.pubkey(),
            from_account: fixture.alice_token,
            to_account: fixture.bob_token,
            compute_signer: fixture.compute_signer,
            from_current_compute_acl,
            sent_amount_acl,
            settlement_record: output.settlement,
            refund_amount_acl: output.refund,
            from_output_acl: output.alice,
            transferred_amount_acl: output.transferred,
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::ConfidentialFinalizeTransferCallback {},
    )
}

fn set_operator_ix(
    fixture: &TokenFixture,
    operator: Pubkey,
    operator_record: Pubkey,
    expiration_slot: u64,
) -> Instruction {
    set_operator_ix_with_token_account(
        fixture,
        fixture.alice_token,
        operator,
        operator_record,
        expiration_slot,
    )
}

fn set_operator_ix_with_token_account(
    fixture: &TokenFixture,
    token_account: Pubkey,
    operator: Pubkey,
    operator_record: Pubkey,
    expiration_slot: u64,
) -> Instruction {
    anchor_ix(
        fixture.token_program_id,
        token::accounts::SetOperator {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            token_account,
            operator_record,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::SetOperator {
            operator,
            expiration_slot,
        },
    )
}

fn close_operator_ix(
    fixture: &TokenFixture,
    owner: Option<Pubkey>,
    operator: Pubkey,
    operator_record: Pubkey,
    refund_recipient: Pubkey,
) -> Instruction {
    anchor_ix(
        fixture.token_program_id,
        token::accounts::CloseOperator {
            owner,
            mint: fixture.mint.pubkey(),
            token_account: fixture.alice_token,
            operator_record,
            refund_recipient,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::CloseOperator { operator },
    )
}

fn transfer_from_ix(
    fixture: &TokenFixture,
    operator: Pubkey,
    operator_record: Pubkey,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
    amount_nonce_sequence: u64,
) -> Instruction {
    transfer_from_ix_with_amount_acl(
        fixture,
        operator,
        operator_record,
        input_compute_acl_address(fixture, amount_nonce_sequence),
        output,
        amount_handle,
    )
}

fn transfer_from_ix_with_amount_acl(
    fixture: &TokenFixture,
    operator: Pubkey,
    operator_record: Pubkey,
    amount_compute_acl: Pubkey,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        fixture.token_program_id,
        token::accounts::ConfidentialTransferFrom {
            operator,
            mint: fixture.mint.pubkey(),
            from_account: fixture.alice_token,
            to_account: fixture.bob_token,
            operator_record,
            compute_signer: fixture.compute_signer,
            from_current_compute_acl: fixture.alice_current_compute_acl,
            to_current_compute_acl: fixture.bob_current_compute_acl,
            amount_compute_acl,
            transfer_success_acl: output.success,
            debit_candidate_acl: output.debit_candidate,
            from_output_acl: output.alice,
            transferred_amount_acl: output.transferred,
            to_output_acl: output.bob,
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::ConfidentialTransferFrom { amount_handle },
    )
}

fn current_slot(svm: &LiteSVM) -> u64 {
    let clock: Clock = svm.get_sysvar();
    clock.slot
}

fn transient_capability(
    subject: Pubkey,
    receiver_program: Pubkey,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    durable_output_allowed: bool,
) -> TransientCapabilityGrant {
    TransientCapabilityGrant {
        subject,
        receiver_program,
        acl_domain_key,
        app_account,
        role_flags: host::ACL_ROLE_USE,
        max_uses: 1,
        durable_output_allowed,
        public_decrypt_allowed: false,
    }
}

#[allow(clippy::too_many_arguments)]
fn create_transient_session_ix(
    program_id: Pubkey,
    payer: Pubkey,
    host_config: Pubkey,
    session: Pubkey,
    session_nonce: [u8; 32],
    authority: Pubkey,
    refund_recipient: Pubkey,
    expires_slot: u64,
    max_entries: u8,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::CreateTransientSession {
            payer,
            authority,
            session,
            host_config,
            system_program: system_program::ID,
        },
        host::instruction::CreateTransientSession {
            session_nonce,
            refund_recipient,
            compute_subject: authority,
            expires_slot,
            max_entries,
        },
    )
}

fn seal_transient_session_ix(
    program_id: Pubkey,
    authority: Pubkey,
    host_config: Pubkey,
    session: Pubkey,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::SealTransientSession {
            authority,
            session,
            host_config,
        },
        host::instruction::SealTransientSession {},
    )
}

fn close_transient_session_ix(
    program_id: Pubkey,
    authority: Option<Pubkey>,
    host_config: Pubkey,
    session: Pubkey,
    refund_recipient: Pubkey,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::CloseTransientSession {
            authority,
            session,
            refund_recipient,
            host_config,
        },
        host::instruction::CloseTransientSession {},
    )
}

#[allow(clippy::too_many_arguments)]
fn session_consume_eval_ix(
    program_id: Pubkey,
    authority: Pubkey,
    host_config: Pubkey,
    session: Pubkey,
    context_id: [u8; 32],
    lhs: [u8; 32],
    rhs: [u8; 32],
    result: [u8; 32],
    output: FheEvalOutput,
) -> Instruction {
    let mut ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: authority,
            compute_subject: authority,
            app_account_authority: authority,
            host_config,
            system_program: system_program::ID,
            instructions_sysvar: Some(sysvar::instructions::ID),
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                ops: vec![FheEvalOp {
                    op: FheBinaryOpCode::Add,
                    lhs: FheEvalOperand::TransientSession {
                        handle: lhs,
                        session_index: 0,
                        capability_index: 0,
                    },
                    rhs: FheEvalOperand::Scalar(rhs),
                    output_fhe_type: 5,
                    result,
                    output,
                }],
            },
        },
    );
    ix.accounts.push(AccountMeta::new(session, false));
    ix
}

fn current_binary_handle(
    svm: &LiteSVM,
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
) -> [u8; 32] {
    let clock: Clock = svm.get_sysvar();
    let previous_bank_hash = clock
        .slot
        .checked_sub(1)
        .and_then(|slot| {
            let slot_hashes: SlotHashes = svm.get_sysvar();
            slot_hashes.get(&slot).map(|hash| hash.to_bytes())
        })
        .unwrap_or([0; 32]);
    host::computed_binary_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        clock.unix_timestamp,
    )
}

fn current_bound_binary_handle(
    svm: &LiteSVM,
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    let clock: Clock = svm.get_sysvar();
    let previous_bank_hash = clock
        .slot
        .checked_sub(1)
        .and_then(|slot| {
            let slot_hashes: SlotHashes = svm.get_sysvar();
            slot_hashes.get(&slot).map(|hash| hash.to_bytes())
        })
        .unwrap_or([0; 32]);
    host::computed_bound_binary_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        clock.unix_timestamp,
        output_nonce_key,
        output_nonce_sequence,
    )
}

fn current_rand_seed(
    svm: &LiteSVM,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 16] {
    let clock: Clock = svm.get_sysvar();
    let previous_bank_hash = clock
        .slot
        .checked_sub(1)
        .and_then(|slot| {
            let slot_hashes: SlotHashes = svm.get_sysvar();
            slot_hashes.get(&slot).map(|hash| hash.to_bytes())
        })
        .unwrap_or([0; 32]);
    host::computed_rand_seed(
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        clock.unix_timestamp,
        output_nonce_key,
        output_nonce_sequence,
    )
}

fn current_bound_ternary_handle(
    svm: &LiteSVM,
    op: FheTernaryOpCode,
    control: [u8; 32],
    if_true: [u8; 32],
    if_false: [u8; 32],
    fhe_type: u8,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    let clock: Clock = svm.get_sysvar();
    let previous_bank_hash = clock
        .slot
        .checked_sub(1)
        .and_then(|slot| {
            let slot_hashes: SlotHashes = svm.get_sysvar();
            slot_hashes.get(&slot).map(|hash| hash.to_bytes())
        })
        .unwrap_or([0; 32]);
    host::computed_bound_ternary_handle(
        op,
        control,
        if_true,
        if_false,
        fhe_type,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        clock.unix_timestamp,
        output_nonce_key,
        output_nonce_sequence,
    )
}

fn predicted_transfer_sent_handle(fixture: &TokenFixture, amount_handle: [u8; 32]) -> [u8; 32] {
    let from = token_account(&fixture.svm, fixture.alice_token);
    let nonce_sequence = from.next_balance_nonce_sequence;
    let mint = fixture.mint.pubkey();
    let transfer_success_handle = current_bound_binary_handle(
        &fixture.svm,
        FheBinaryOpCode::Ge,
        from.balance_handle,
        amount_handle,
        false,
        0,
        token::nonce_key(mint, fixture.alice_token, token::transfer_success_label()),
        nonce_sequence,
    );
    let debit_candidate_handle = current_bound_binary_handle(
        &fixture.svm,
        FheBinaryOpCode::Sub,
        from.balance_handle,
        amount_handle,
        false,
        5,
        token::nonce_key(mint, fixture.alice_token, token::debit_candidate_label()),
        nonce_sequence,
    );
    let new_from_handle = current_bound_ternary_handle(
        &fixture.svm,
        FheTernaryOpCode::IfThenElse,
        transfer_success_handle,
        debit_candidate_handle,
        from.balance_handle,
        5,
        token::balance_nonce_key(mint, fixture.alice_token),
        nonce_sequence,
    );
    current_bound_binary_handle(
        &fixture.svm,
        FheBinaryOpCode::Sub,
        from.balance_handle,
        new_from_handle,
        false,
        5,
        token::nonce_key(mint, fixture.alice_token, token::transferred_amount_label()),
        nonce_sequence,
    )
}

fn upper_bound_be(value: u128) -> [u8; 32] {
    let mut bytes = [0_u8; 32];
    bytes[16..].copy_from_slice(&value.to_be_bytes());
    bytes
}

fn current_eval_handle(
    svm: &LiteSVM,
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    context_id: [u8; 32],
    op_index: u16,
) -> [u8; 32] {
    let clock: Clock = svm.get_sysvar();
    let previous_bank_hash = clock
        .slot
        .checked_sub(1)
        .and_then(|slot| {
            let slot_hashes: SlotHashes = svm.get_sysvar();
            slot_hashes.get(&slot).map(|hash| hash.to_bytes())
        })
        .unwrap_or([0; 32]);
    host::computed_eval_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        clock.unix_timestamp,
        context_id,
        op_index,
    )
}

#[allow(clippy::too_many_arguments)]
fn current_bound_eval_handle(
    svm: &LiteSVM,
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    context_id: [u8; 32],
    op_index: u16,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    let clock: Clock = svm.get_sysvar();
    let previous_bank_hash = clock
        .slot
        .checked_sub(1)
        .and_then(|slot| {
            let slot_hashes: SlotHashes = svm.get_sysvar();
            slot_hashes.get(&slot).map(|hash| hash.to_bytes())
        })
        .unwrap_or([0; 32]);
    host::computed_bound_eval_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        clock.unix_timestamp,
        context_id,
        op_index,
        output_nonce_key,
        output_nonce_sequence,
    )
}

fn wrap_usdc_ix(fixture: &TokenFixture, output: WrapOutputAccounts, amount: u64) -> Instruction {
    wrap_usdc_ix_with_vault(fixture, output, amount, fixture.vault_usdc)
}

fn wrap_usdc_ix_with_vault(
    fixture: &TokenFixture,
    output: WrapOutputAccounts,
    amount: u64,
    vault_usdc: Pubkey,
) -> Instruction {
    anchor_ix(
        fixture.token_program_id,
        token::accounts::WrapUsdc {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            token_account: fixture.alice_token,
            underlying_mint: fixture.underlying_mint.pubkey(),
            user_usdc: fixture.alice_usdc,
            vault_usdc,
            vault_authority: vault_authority_address(
                fixture.token_program_id,
                fixture.mint.pubkey(),
            ),
            compute_signer: fixture.compute_signer,
            total_supply_authority: fixture.total_supply_authority,
            current_compute_acl: fixture.alice_current_compute_acl,
            current_total_supply_acl: fixture.total_supply_current_acl,
            amount_compute_acl: output.amount,
            output_acl: output.balance,
            total_supply_output_acl: output.total_supply,
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            host_config: fixture.host_config,
            token_program: spl_token::id(),
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::WrapUsdc { amount },
    )
}

fn create_random_bounded_amount_ix(
    fixture: &TokenFixture,
    amount_kind: ConfidentialAmountKind,
    upper_bound: [u8; 32],
) -> Instruction {
    let nonce_sequence =
        token_account(&fixture.svm, fixture.alice_token).next_amount_nonce_sequence;
    anchor_ix(
        fixture.token_program_id,
        token::accounts::CreateRandomAmount {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            token_account: fixture.alice_token,
            compute_signer: fixture.compute_signer,
            amount_acl_record: random_amount_acl_address(
                fixture,
                fixture.alice.pubkey(),
                amount_kind,
                nonce_sequence,
            ),
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::CreateRandomBoundedAmount {
            amount_kind,
            upper_bound,
        },
    )
}

fn create_random_amount_ix(
    fixture: &TokenFixture,
    amount_kind: ConfidentialAmountKind,
) -> Instruction {
    let nonce_sequence =
        token_account(&fixture.svm, fixture.alice_token).next_amount_nonce_sequence;
    anchor_ix(
        fixture.token_program_id,
        token::accounts::CreateRandomAmount {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            token_account: fixture.alice_token,
            compute_signer: fixture.compute_signer,
            amount_acl_record: random_amount_acl_address(
                fixture,
                fixture.alice.pubkey(),
                amount_kind,
                nonce_sequence,
            ),
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::CreateRandomAmount { amount_kind },
    )
}

fn burn_ix_with_current_acls(
    fixture: &TokenFixture,
    current_compute_acl: Pubkey,
    current_total_supply_acl: Pubkey,
    output: BurnOutputAccounts,
    amount_handle: [u8; 32],
    amount_nonce_sequence: u64,
) -> Instruction {
    burn_ix_with_amount_acl(
        fixture,
        current_compute_acl,
        current_total_supply_acl,
        burn_input_compute_acl_address(fixture, amount_nonce_sequence),
        output,
        amount_handle,
    )
}

fn burn_ix_with_amount_acl(
    fixture: &TokenFixture,
    current_compute_acl: Pubkey,
    current_total_supply_acl: Pubkey,
    amount_compute_acl: Pubkey,
    output: BurnOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        fixture.token_program_id,
        token::accounts::ConfidentialBurn {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            token_account: fixture.alice_token,
            compute_signer: fixture.compute_signer,
            total_supply_authority: fixture.total_supply_authority,
            current_compute_acl,
            current_total_supply_acl,
            amount_compute_acl,
            burn_success_acl: output.success,
            debit_candidate_acl: output.debit_candidate,
            output_acl: output.balance,
            burned_amount_acl: output.burned,
            total_supply_output_acl: output.total_supply,
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::ConfidentialBurn { amount_handle },
    )
}

fn request_disclose_balance_ix(
    fixture: &TokenFixture,
    owner: Pubkey,
    token_account: Pubkey,
    balance_acl_record: Pubkey,
) -> Instruction {
    anchor_ix(
        fixture.token_program_id,
        token::accounts::RequestDiscloseBalance {
            owner,
            mint: fixture.mint.pubkey(),
            token_account,
            balance_acl_record,
            authority_permission_record: None,
            deny_subject_record: None,
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            host_config: fixture.host_config,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::RequestDiscloseBalance {},
    )
}

fn request_disclose_amount_ix(
    fixture: &TokenFixture,
    requester: Pubkey,
    amount_acl_record: Pubkey,
    amount_handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        fixture.token_program_id,
        token::accounts::RequestDiscloseAmount {
            requester,
            mint: fixture.mint.pubkey(),
            amount_acl_record,
            authority_permission_record: None,
            deny_subject_record: None,
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            host_config: fixture.host_config,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::RequestDiscloseAmount { amount_handle },
    )
}

fn disclose_balance_ix(
    fixture: &TokenFixture,
    token_account: Pubkey,
    balance_acl_record: Pubkey,
    cleartext_amount: u64,
) -> Instruction {
    anchor_ix(
        fixture.token_program_id,
        token::accounts::DiscloseBalance {
            mint: fixture.mint.pubkey(),
            token_account,
            balance_acl_record,
            balance_material_commitment: host::handle_material_address(balance_acl_record).0,
            instructions_sysvar: sysvar::instructions::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::DiscloseBalance { cleartext_amount },
    )
}

fn disclose_amount_ix(
    fixture: &TokenFixture,
    amount_acl_record: Pubkey,
    amount_handle: [u8; 32],
    cleartext_amount: u64,
) -> Instruction {
    anchor_ix(
        fixture.token_program_id,
        token::accounts::DiscloseAmount {
            mint: fixture.mint.pubkey(),
            amount_acl_record,
            amount_material_commitment: host::handle_material_address(amount_acl_record).0,
            instructions_sysvar: sysvar::instructions::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::DiscloseAmount {
            amount_handle,
            cleartext_amount,
        },
    )
}

fn redeem_burned_amount_ix(
    fixture: &TokenFixture,
    burned_amount_acl: Pubkey,
    redemption_record: Pubkey,
    burned_handle: [u8; 32],
    cleartext_amount: u64,
) -> Instruction {
    redeem_burned_amount_ix_with_vault(
        fixture,
        burned_amount_acl,
        redemption_record,
        burned_handle,
        cleartext_amount,
        fixture.vault_usdc,
    )
}

fn redeem_burned_amount_ix_with_vault(
    fixture: &TokenFixture,
    burned_amount_acl: Pubkey,
    redemption_record: Pubkey,
    burned_handle: [u8; 32],
    cleartext_amount: u64,
    vault_usdc: Pubkey,
) -> Instruction {
    anchor_ix(
        fixture.token_program_id,
        token::accounts::RedeemBurnedAmount {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            token_account: fixture.alice_token,
            underlying_mint: fixture.underlying_mint.pubkey(),
            vault_usdc,
            destination_usdc: fixture.alice_usdc,
            vault_authority: vault_authority_address(
                fixture.token_program_id,
                fixture.mint.pubkey(),
            ),
            burned_amount_acl,
            burned_material_commitment: host::handle_material_address(burned_amount_acl).0,
            redemption_record,
            instructions_sysvar: sysvar::instructions::ID,
            token_program: spl_token::id(),
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::RedeemBurnedAmount {
            burned_handle,
            cleartext_amount,
        },
    )
}

fn disclosure_ed25519_ix(
    fixture: &TokenFixture,
    handle: [u8; 32],
    cleartext_amount: u64,
) -> Instruction {
    ed25519_verify_ix(
        &fixture.verifier,
        &token::disclosure_proof_message(
            fixture.mint.pubkey(),
            handle,
            cleartext_amount,
            fixture.token_program_id,
        ),
    )
}

fn allow_for_decryption_ix(
    program_id: Pubkey,
    authority: Pubkey,
    acl_record: Pubkey,
    handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::AllowForDecryption {
            authority,
            authority_permission_record: None,
            acl_record,
            host_config: Pubkey::find_program_address(&[host::HOST_CONFIG_SEED], &program_id).0,
            deny_subject_record: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowForDecryption { handle },
    )
}

fn commit_handle_material_ix(
    program_id: Pubkey,
    payer: Pubkey,
    material_authority: Pubkey,
    host_config: Pubkey,
    acl_record: Pubkey,
    material_commitment: Pubkey,
    key_id: [u8; 32],
    ciphertext_digest: [u8; 32],
    sns_ciphertext_digest: [u8; 32],
    coprocessor_set_digest: [u8; 32],
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::CommitHandleMaterial {
            payer,
            material_authority,
            host_config,
            acl_record,
            material_commitment,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::CommitHandleMaterial {
            key_id,
            ciphertext_digest,
            sns_ciphertext_digest,
            coprocessor_set_digest,
        },
    )
}

fn commit_material_for_acl(fixture: &mut TokenFixture, acl_record: Pubkey, seed: u8) -> Pubkey {
    let material_commitment = host::handle_material_address(acl_record).0;
    if read_material_commitment(&fixture.svm, material_commitment).is_some() {
        return material_commitment;
    }
    let commit_ix = commit_handle_material_ix(
        fixture.host_program_id,
        fixture.alice.pubkey(),
        fixture.verifier.pubkey(),
        fixture.host_config,
        acl_record,
        material_commitment,
        [seed; 32],
        [seed.wrapping_add(1); 32],
        [seed.wrapping_add(2); 32],
        [seed.wrapping_add(3); 32],
    );
    send_many_with_signers(
        &mut fixture.svm,
        &fixture.alice.pubkey(),
        vec![commit_ix],
        &[&fixture.alice, &fixture.verifier],
    )
    .unwrap();
    material_commitment
}

fn seed_unsealed_material_commitment(
    svm: &mut LiteSVM,
    program_id: Pubkey,
    acl_record: Pubkey,
    handle: [u8; 32],
    seed: u8,
) -> Pubkey {
    let (material_commitment, bump) = host::handle_material_address(acl_record);
    let key_id = [seed; 32];
    let ciphertext_digest = [seed.wrapping_add(1); 32];
    let sns_ciphertext_digest = [seed.wrapping_add(2); 32];
    let coprocessor_set_digest = [seed.wrapping_add(3); 32];
    let material_commitment_hash = host::handle_material_commitment_hash(
        material_commitment,
        acl_record,
        key_id,
        ciphertext_digest,
        sns_ciphertext_digest,
        coprocessor_set_digest,
    );
    svm.set_account(
        material_commitment,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(HandleMaterialCommitment {
                acl_record,
                handle,
                key_id,
                ciphertext_digest,
                sns_ciphertext_digest,
                coprocessor_set_digest,
                material_commitment_hash,
                created_slot: 0,
                state: host::HANDLE_MATERIAL_STATE_COMMITTED,
                bump,
            }),
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
    material_commitment
}

fn extend_material_commitment(svm: &mut LiteSVM, material_commitment: Pubkey, extra_bytes: usize) {
    let mut account = svm
        .get_account(&material_commitment)
        .expect("expected material commitment");
    account.data.resize(account.data.len() + extra_bytes, 0);
    account.lamports = svm.minimum_balance_for_rent_exemption(account.data.len());
    svm.set_account(material_commitment, account).unwrap();
}

fn create_spl_token_account(
    svm: &mut LiteSVM,
    payer: &Keypair,
    token_account: &Keypair,
    mint: Pubkey,
    owner: Pubkey,
) {
    let rent = svm.minimum_balance_for_rent_exemption(spl_token::state::Account::LEN);
    send_many_with_signers(
        svm,
        &payer.pubkey(),
        vec![
            system_instruction::create_account(
                &payer.pubkey(),
                &token_account.pubkey(),
                rent,
                spl_token::state::Account::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_account3(
                &spl_token::id(),
                &token_account.pubkey(),
                &mint,
                &owner,
            )
            .unwrap(),
        ],
        &[payer, token_account],
    )
    .unwrap();
}

fn create_associated_spl_token_account(
    svm: &mut LiteSVM,
    payer: &Keypair,
    mint: Pubkey,
    owner: Pubkey,
) -> Pubkey {
    let address = get_associated_token_address_with_program_id(&owner, &mint, &spl_token::id());
    let ix = spl_associated_token_account::instruction::create_associated_token_account(
        &payer.pubkey(),
        &owner,
        &mint,
        &spl_token::id(),
    );
    send_with_signers(svm, &payer.pubkey(), ix, &[payer]).unwrap();
    address
}

fn create_noncanonical_vault_token_account(fixture: &mut TokenFixture) -> Pubkey {
    let vault = Keypair::new();
    create_spl_token_account(
        &mut fixture.svm,
        &fixture.alice,
        &vault,
        fixture.underlying_mint.pubkey(),
        vault_authority_address(fixture.token_program_id, fixture.mint.pubkey()),
    );
    vault.pubkey()
}

fn seed_noncanonical_confidential_token_account(fixture: &mut TokenFixture) -> Pubkey {
    let token_account = Pubkey::new_unique();
    let data = serialized_account(token::ConfidentialTokenAccount {
        owner: fixture.alice.pubkey(),
        mint: fixture.mint.pubkey(),
        balance_handle: fixture.alice_initial,
        balance_acl_record: fixture.alice_current_compute_acl,
        next_balance_nonce_sequence: 1,
        next_amount_nonce_sequence: 0,
        bump: 0,
    });
    fixture
        .svm
        .set_account(
            token_account,
            Account {
                lamports: fixture.svm.minimum_balance_for_rent_exemption(data.len()),
                data,
                owner: fixture.token_program_id,
                executable: false,
                rent_epoch: 0,
            },
        )
        .unwrap();
    token_account
}

fn seed_confidential_token_account(
    svm: &mut LiteSVM,
    program_id: Pubkey,
    address: Pubkey,
    account: &token::ConfidentialTokenAccount,
    extra_bytes: usize,
) {
    let mut data = Vec::new();
    account.try_serialize(&mut data).unwrap();
    data.resize(data.len() + extra_bytes, 0);
    svm.set_account(
        address,
        Account {
            lamports: svm.minimum_balance_for_rent_exemption(data.len()),
            data,
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
}

fn spl_token_amount(svm: &LiteSVM, address: Pubkey) -> u64 {
    let account = svm
        .get_account(&address)
        .expect("expected SPL token account");
    spl_token::state::Account::unpack(&account.data)
        .unwrap()
        .amount
}

fn amount_plaintext(amount: u64) -> [u8; 32] {
    let mut plaintext = [0_u8; 32];
    plaintext[24..].copy_from_slice(&amount.to_be_bytes());
    plaintext
}

fn token_account(svm: &LiteSVM, address: Pubkey) -> token::ConfidentialTokenAccount {
    let account = svm
        .get_account(&address)
        .expect("expected confidential token account");
    let mut data = account.data.as_slice();
    token::ConfidentialTokenAccount::try_deserialize(&mut data).unwrap()
}

fn mint_account(svm: &LiteSVM, address: Pubkey) -> token::ConfidentialMint {
    let account = svm
        .get_account(&address)
        .expect("expected confidential mint account");
    let mut data = account.data.as_slice();
    token::ConfidentialMint::try_deserialize(&mut data).unwrap()
}

fn seed_confidential_mint(
    svm: &mut LiteSVM,
    program_id: Pubkey,
    address: Pubkey,
    mint: &token::ConfidentialMint,
    extra_bytes: usize,
) {
    let mut data = Vec::new();
    mint.try_serialize(&mut data).unwrap();
    data.resize(data.len() + extra_bytes, 0);
    svm.set_account(
        address,
        Account {
            lamports: svm.minimum_balance_for_rent_exemption(data.len()),
            data,
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
}

fn read_burn_redemption(svm: &LiteSVM, address: Pubkey) -> Option<token::BurnRedemption> {
    let account = svm.get_account(&address)?;
    let mut data = account.data.as_slice();
    token::BurnRedemption::try_deserialize(&mut data).ok()
}

fn read_transfer_callback_settlement(
    svm: &LiteSVM,
    address: Pubkey,
) -> Option<token::TransferCallbackSettlement> {
    let account = svm.get_account(&address)?;
    let mut data = account.data.as_slice();
    token::TransferCallbackSettlement::try_deserialize(&mut data).ok()
}

fn read_transfer_receiver_hook_call(
    svm: &LiteSVM,
    address: Pubkey,
) -> Option<token::TransferReceiverHookCall> {
    let account = svm.get_account(&address)?;
    let mut data = account.data.as_slice();
    token::TransferReceiverHookCall::try_deserialize(&mut data).ok()
}

fn seed_transfer_callback_settlement(
    svm: &mut LiteSVM,
    program_id: Pubkey,
    address: Pubkey,
    settlement: &token::TransferCallbackSettlement,
    extra_bytes: usize,
) {
    let mut data = Vec::new();
    settlement.try_serialize(&mut data).unwrap();
    data.resize(data.len() + extra_bytes, 0);
    svm.set_account(
        address,
        Account {
            lamports: svm.minimum_balance_for_rent_exemption(data.len()),
            data,
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
}

fn signed_current_balance_user_decrypt_request(
    fixture: &TokenFixture,
    token_account_address: Pubkey,
    signer: &Keypair,
) -> UserDecryptRequest {
    let account = token_account(&fixture.svm, token_account_address);
    signed_user_decrypt_request(
        fixture,
        signer,
        vec![UserDecryptHandleEntry {
            handle: account.balance_handle,
            owner: account.owner,
            acl_record: account.balance_acl_record,
        }],
    )
}

fn signed_user_decrypt_request(
    fixture: &TokenFixture,
    signer: &Keypair,
    handles: Vec<UserDecryptHandleEntry>,
) -> UserDecryptRequest {
    let authorization = UserDecryptAuthorizationPayload {
        user: signer.pubkey(),
        reencryption_public_key: [7; 32],
        allowed_acl_domain_keys: vec![fixture.mint.pubkey()],
        start_timestamp: 1,
        duration_seconds: 300,
        extra_data: b"zama-solana-poc".to_vec(),
    };
    let signature = signer.sign_message(&authorization_payload_bytes(&authorization));

    UserDecryptRequest {
        authorization,
        signature,
        handles,
    }
}

fn authorization_payload_bytes(authorization: &UserDecryptAuthorizationPayload) -> Vec<u8> {
    let mut bytes = b"Zama Solana UserDecrypt v0".to_vec();
    bytes.extend_from_slice(authorization.user.as_ref());
    bytes.extend_from_slice(&authorization.reencryption_public_key);
    bytes.extend_from_slice(&(authorization.allowed_acl_domain_keys.len() as u32).to_le_bytes());
    for account in &authorization.allowed_acl_domain_keys {
        bytes.extend_from_slice(account.as_ref());
    }
    bytes.extend_from_slice(&authorization.start_timestamp.to_le_bytes());
    bytes.extend_from_slice(&authorization.duration_seconds.to_le_bytes());
    bytes.extend_from_slice(&(authorization.extra_data.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&authorization.extra_data);
    bytes
}

fn assert_balance_acl(
    svm: &LiteSVM,
    address: Pubkey,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    nonce_sequence: u64,
    handle: [u8; 32],
    subjects: &[Pubkey],
) {
    assert_acl_record(
        svm,
        address,
        acl_domain_key,
        app_account,
        token::balance_label(),
        nonce_sequence,
        handle,
        subjects,
    );
}

fn assert_acl_record(
    svm: &LiteSVM,
    address: Pubkey,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    nonce_sequence: u64,
    handle: [u8; 32],
    subjects: &[Pubkey],
) {
    let record = read_acl_record(svm, address).expect("expected ACL account");
    assert_eq!(record.handle, handle);
    assert_eq!(
        record.nonce_key,
        token::nonce_key(acl_domain_key, app_account, encrypted_value_label)
    );
    assert_eq!(record.nonce_sequence, nonce_sequence);
    assert_eq!(record.acl_domain_key, acl_domain_key);
    assert_eq!(record.app_account, app_account);
    assert_eq!(record.encrypted_value_label, encrypted_value_label);
    assert_eq!(record_subjects(&record), subjects);
}

fn kms_like_user_decrypt_check(svm: &LiteSVM, request: &UserDecryptRequest) -> bool {
    let authorization = &request.authorization;
    let signed_payload = authorization_payload_bytes(authorization);
    if !request
        .signature
        .verify(authorization.user.as_ref(), &signed_payload)
        || authorization.reencryption_public_key == [0; 32]
        || authorization.duration_seconds == 0
        || authorization.extra_data.is_empty()
        || authorization.start_timestamp < 0
        || request.handles.is_empty()
    {
        return false;
    }

    request.handles.iter().all(|entry| {
        if authorization.user != entry.owner {
            return false;
        }

        let Some(raw_account) = svm.get_account(&entry.acl_record) else {
            return false;
        };
        if raw_account.owner != host::id() {
            return false;
        }

        let mut data = raw_account.data.as_slice();
        let Ok(record) = AclRecord::try_deserialize(&mut data) else {
            return false;
        };
        let expected_nonce_key = token::nonce_key(
            record.acl_domain_key,
            record.app_account,
            record.encrypted_value_label,
        );
        let expected_acl_record =
            acl_record_address(host::id(), expected_nonce_key, record.nonce_sequence);

        authorization
            .allowed_acl_domain_keys
            .contains(&record.acl_domain_key)
            && record.handle == entry.handle
            && record.nonce_key == expected_nonce_key
            && entry.acl_record == expected_acl_record
            && record_subjects(&record).contains(&authorization.user)
    })
}

fn kms_like_public_decrypt_check(svm: &LiteSVM, entries: &[PublicDecryptHandleEntry]) -> bool {
    if entries.is_empty() {
        return false;
    }

    entries.iter().all(|entry| {
        let Some(raw_account) = svm.get_account(&entry.acl_record) else {
            return false;
        };
        if raw_account.owner != host::id() {
            return false;
        }

        let mut data = raw_account.data.as_slice();
        let Ok(record) = AclRecord::try_deserialize(&mut data) else {
            return false;
        };
        let expected_nonce_key = token::nonce_key(
            record.acl_domain_key,
            record.app_account,
            record.encrypted_value_label,
        );
        let expected_acl_record =
            acl_record_address(host::id(), expected_nonce_key, record.nonce_sequence);

        record.handle == entry.handle
            && record.nonce_key == expected_nonce_key
            && entry.acl_record == expected_acl_record
            && record.public_decrypt
    })
}

fn kms_like_public_decrypt_with_material_check(
    svm: &LiteSVM,
    entries: &[PublicDecryptWithMaterialEntry],
) -> bool {
    if entries.is_empty() {
        return false;
    }

    entries.iter().all(|entry| {
        if !kms_like_public_decrypt_check(
            svm,
            &[PublicDecryptHandleEntry {
                handle: entry.handle,
                acl_record: entry.acl_record,
            }],
        ) {
            return false;
        }

        let Some(raw_account) = svm.get_account(&entry.material_commitment) else {
            return false;
        };
        if raw_account.owner != host::id() {
            return false;
        }

        let mut data = raw_account.data.as_slice();
        let Ok(material) = HandleMaterialCommitment::try_deserialize(&mut data) else {
            return false;
        };
        let expected_material = host::handle_material_address(entry.acl_record).0;
        material.acl_record == entry.acl_record
            && material.handle == entry.handle
            && entry.material_commitment == expected_material
            && material.state == host::HANDLE_MATERIAL_STATE_COMMITTED
            && material.material_commitment_hash
                == host::handle_material_commitment_hash(
                    entry.material_commitment,
                    entry.acl_record,
                    material.key_id,
                    material.ciphertext_digest,
                    material.sns_ciphertext_digest,
                    material.coprocessor_set_digest,
                )
            && read_acl_record(svm, entry.acl_record)
                .map(|record| {
                    record.material_commitment == entry.material_commitment
                        && record.material_commitment_hash == material.material_commitment_hash
                        && record.material_key_id == material.key_id
                })
                .unwrap_or(false)
    })
}

fn read_acl_record(svm: &LiteSVM, address: Pubkey) -> Option<AclRecord> {
    let account = svm.get_account(&address)?;
    let mut data = account.data.as_slice();
    AclRecord::try_deserialize(&mut data).ok()
}

fn read_acl_permission(svm: &LiteSVM, address: Pubkey) -> Option<AclPermission> {
    let account = svm.get_account(&address)?;
    let mut data = account.data.as_slice();
    AclPermission::try_deserialize(&mut data).ok()
}

fn read_deny_subject_record(svm: &LiteSVM, address: Pubkey) -> Option<DenySubjectRecord> {
    let account = svm.get_account(&address)?;
    let mut data = account.data.as_slice();
    DenySubjectRecord::try_deserialize(&mut data).ok()
}

fn read_host_config(svm: &LiteSVM, address: Pubkey) -> HostConfig {
    let account = svm.get_account(&address).expect("expected host config");
    let mut data = account.data.as_slice();
    HostConfig::try_deserialize(&mut data).expect("valid host config")
}

fn read_transient_session(svm: &LiteSVM, address: Pubkey) -> Option<TransientSession> {
    let account = svm.get_account(&address)?;
    let mut data = account.data.as_slice();
    TransientSession::try_deserialize(&mut data).ok()
}

fn read_delegation_record(svm: &LiteSVM, address: Pubkey) -> Option<UserDecryptionDelegation> {
    let account = svm.get_account(&address)?;
    let mut data = account.data.as_slice();
    UserDecryptionDelegation::try_deserialize(&mut data).ok()
}

fn read_material_commitment(svm: &LiteSVM, address: Pubkey) -> Option<HandleMaterialCommitment> {
    let account = svm.get_account(&address)?;
    let mut data = account.data.as_slice();
    HandleMaterialCommitment::try_deserialize(&mut data).ok()
}

fn read_operator_record(svm: &LiteSVM, address: Pubkey) -> Option<token::ConfidentialOperator> {
    let account = svm.get_account(&address)?;
    let mut data = account.data.as_slice();
    token::ConfidentialOperator::try_deserialize(&mut data).ok()
}

fn seed_operator_record(
    svm: &mut LiteSVM,
    program_id: Pubkey,
    address: Pubkey,
    record: token::ConfidentialOperator,
    extra_bytes: usize,
) {
    let mut data = serialized_account(record);
    data.resize(data.len() + extra_bytes, 0);
    svm.set_account(
        address,
        Account {
            lamports: svm.minimum_balance_for_rent_exemption(data.len()),
            data,
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
}

fn serialized_acl_record(record: AclRecord) -> Vec<u8> {
    serialized_account(record)
}

fn serialized_account<T: AccountSerialize>(account: T) -> Vec<u8> {
    let mut data = Vec::new();
    account.try_serialize(&mut data).unwrap();
    data
}

fn seed_acl_permission(
    svm: &mut LiteSVM,
    program_id: Pubkey,
    address: Pubkey,
    permission: AclPermission,
    extra_bytes: usize,
) {
    let mut data = serialized_account(permission);
    data.resize(data.len() + extra_bytes, 0);
    svm.set_account(
        address,
        Account {
            lamports: svm.minimum_balance_for_rent_exemption(data.len()),
            data,
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
}

fn seed_deny_subject_record(
    svm: &mut LiteSVM,
    program_id: Pubkey,
    address: Pubkey,
    record: DenySubjectRecord,
    extra_bytes: usize,
) {
    let mut data = serialized_account(record);
    data.resize(data.len() + extra_bytes, 0);
    svm.set_account(
        address,
        Account {
            lamports: svm.minimum_balance_for_rent_exemption(data.len()),
            data,
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
}

fn seed_authorizing_acl_record(
    svm: &mut LiteSVM,
    program_id: Pubkey,
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    authority: Pubkey,
) -> Pubkey {
    seed_acl_record_with_subject_entries(
        svm,
        program_id,
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &[AclSubjectEntry::user(authority)],
    )
}

fn seed_disclosable_amount_acl(fixture: &mut TokenFixture, handle: [u8; 32]) -> Pubkey {
    let encrypted_value_label = token::transferred_amount_label();
    let nonce_key = token::nonce_key(
        fixture.mint.pubkey(),
        fixture.alice_token,
        encrypted_value_label,
    );
    seed_acl_record_with_subject_entries(
        &mut fixture.svm,
        fixture.host_program_id,
        nonce_key,
        DEFAULT_INPUT_NONCE_SEQUENCE,
        fixture.mint.pubkey(),
        fixture.alice_token,
        encrypted_value_label,
        handle,
        &[
            AclSubjectEntry::user(fixture.alice.pubkey()),
            AclSubjectEntry::compute(fixture.compute_signer),
        ],
    )
}

fn seed_acl_record_with_subject_entries(
    svm: &mut LiteSVM,
    program_id: Pubkey,
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    entries: &[AclSubjectEntry],
) -> Pubkey {
    let (address, bump) = Pubkey::find_program_address(
        &[
            b"acl-record",
            nonce_key.as_ref(),
            &nonce_sequence.to_le_bytes(),
        ],
        &program_id,
    );
    let mut subjects = [Pubkey::default(); host::MAX_ACL_SUBJECTS];
    let mut subject_roles = [0_u8; host::MAX_ACL_SUBJECTS];
    for (index, entry) in entries.iter().enumerate() {
        subjects[index] = entry.pubkey;
        subject_roles[index] = entry.role_flags;
    }
    svm.set_account(
        address,
        Account {
            lamports: 1_000_000_000,
            data: serialized_acl_record(AclRecord {
                handle,
                nonce_key,
                nonce_sequence,
                acl_domain_key,
                app_account,
                encrypted_value_label,
                subjects,
                subject_roles,
                subject_count: entries.len() as u8,
                overflow_subject_count: 0,
                public_decrypt: false,
                material_commitment: Pubkey::default(),
                material_commitment_hash: [0; 32],
                material_key_id: [0; 32],
                created_slot: 0,
                bump,
            }),
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
    address
}

fn extend_acl_record(svm: &mut LiteSVM, acl_record: Pubkey, extra_bytes: usize) {
    let mut account = svm.get_account(&acl_record).expect("expected ACL record");
    account.data.resize(account.data.len() + extra_bytes, 0);
    account.lamports = svm.minimum_balance_for_rent_exemption(account.data.len());
    svm.set_account(acl_record, account).unwrap();
}

fn mutate_acl_record(svm: &mut LiteSVM, acl_record: Pubkey, mutate: impl FnOnce(&mut AclRecord)) {
    let mut account = svm.get_account(&acl_record).expect("expected ACL record");
    let mut data = account.data.as_slice();
    let mut record = AclRecord::try_deserialize(&mut data).expect("expected ACL record data");
    mutate(&mut record);
    account.data = serialized_acl_record(record);
    account.lamports = svm.minimum_balance_for_rent_exemption(account.data.len());
    svm.set_account(acl_record, account).unwrap();
}

fn record_subjects(record: &AclRecord) -> Vec<Pubkey> {
    record.subjects[..record.subject_count as usize].to_vec()
}

fn created_acl_count(svm: &LiteSVM, output: TransferOutputAccounts) -> usize {
    [
        output.alice,
        output.bob,
        output.success,
        output.debit_candidate,
        output.transferred,
    ]
    .into_iter()
    .filter(|address| svm.get_account(address).is_some())
    .count()
}

fn created_burn_acl_count(svm: &LiteSVM, output: BurnOutputAccounts) -> usize {
    [
        output.balance,
        output.success,
        output.debit_candidate,
        output.burned,
        output.total_supply,
    ]
    .into_iter()
    .filter(|address| svm.get_account(address).is_some())
    .count()
}

fn binary_op_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<FheBinaryOpEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_binary_op_event(&ix.instruction.data))
        .collect()
}

fn ternary_op_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<FheTernaryOpEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_ternary_op_event(&ix.instruction.data))
        .collect()
}

fn trivial_encrypt_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<TrivialEncryptEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_trivial_encrypt_event(&ix.instruction.data))
        .collect()
}

fn fhe_rand_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<FheRandEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_fhe_rand_event(&ix.instruction.data))
        .collect()
}

fn fhe_rand_bounded_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<FheRandBoundedEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_fhe_rand_bounded_event(&ix.instruction.data))
        .collect()
}

fn acl_allowed_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<AclAllowedEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_acl_allowed_event(&ix.instruction.data))
        .collect()
}

fn public_decrypt_allowed_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<PublicDecryptAllowedEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_public_decrypt_allowed_event(&ix.instruction.data))
        .collect()
}

fn handle_material_committed_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<HandleMaterialCommittedEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_handle_material_committed_event(&ix.instruction.data))
        .collect()
}

fn handle_material_sealed_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<HandleMaterialSealedEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_handle_material_sealed_event(&ix.instruction.data))
        .collect()
}

fn balance_handle_updated_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<BalanceHandleUpdatedEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_balance_handle_updated_event(&ix.instruction.data))
        .collect()
}

fn total_supply_handle_updated_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<TotalSupplyHandleUpdatedEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_total_supply_handle_updated_event(&ix.instruction.data))
        .collect()
}

fn random_amount_created_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<RandomAmountCreatedEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_random_amount_created_event(&ix.instruction.data))
        .collect()
}

fn balance_disclosure_requested_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<BalanceDisclosureRequestedEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_balance_disclosure_requested_event(&ix.instruction.data))
        .collect()
}

fn amount_disclosure_requested_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<AmountDisclosureRequestedEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_amount_disclosure_requested_event(&ix.instruction.data))
        .collect()
}

fn balance_disclosed_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<BalanceDisclosedEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_balance_disclosed_event(&ix.instruction.data))
        .collect()
}

fn amount_disclosed_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<AmountDisclosedEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_amount_disclosed_event(&ix.instruction.data))
        .collect()
}

fn burn_redeemed_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<BurnRedeemedEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_burn_redeemed_event(&ix.instruction.data))
        .collect()
}

fn confidential_burn_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<ConfidentialBurnEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_confidential_burn_event(&ix.instruction.data))
        .collect()
}

fn confidential_transfer_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<ConfidentialTransferEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_confidential_transfer_event(&ix.instruction.data))
        .collect()
}

fn delegation_updated_events(
    meta: &TransactionMetadata,
) -> Vec<UserDecryptionDelegationUpdatedEvent> {
    anchor_log_events(meta)
}

fn deny_subject_updated_events(meta: &TransactionMetadata) -> Vec<DenySubjectUpdatedEvent> {
    anchor_log_events(meta)
}

fn host_config_updated_events(meta: &TransactionMetadata) -> Vec<HostConfigUpdatedEvent> {
    anchor_log_events(meta)
}

fn assert_host_config_updated_event(
    meta: &TransactionMetadata,
    host_config: Pubkey,
    admin: Pubkey,
    config: &HostConfig,
) {
    let events = host_config_updated_events(meta);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].config, host_config);
    assert_eq!(events[0].admin, admin);
    assert_eq!(events[0].paused, config.paused);
    assert_eq!(events[0].mock_input_enabled, config.mock_input_enabled);
    assert_eq!(events[0].test_shims_enabled, config.test_shims_enabled);
    assert_eq!(
        events[0].grant_deny_list_enabled,
        config.grant_deny_list_enabled
    );
    assert_eq!(events[0].updated_slot, config.updated_slot);
}

fn acl_record_bound_events(meta: &TransactionMetadata) -> Vec<AclRecordBoundEvent> {
    anchor_log_events(meta)
}

fn acl_subject_allowed_events(meta: &TransactionMetadata) -> Vec<AclSubjectAllowedEvent> {
    anchor_log_events(meta)
}

fn max_cpi_depth(meta: &TransactionMetadata) -> u64 {
    meta.logs
        .iter()
        .filter_map(|log| {
            log.strip_suffix(']')?
                .rsplit_once(" invoke [")?
                .1
                .parse::<u64>()
                .ok()
        })
        .max()
        .unwrap_or(1)
}

fn program_data_log_count(meta: &TransactionMetadata) -> usize {
    meta.logs
        .iter()
        .filter(|log| log.starts_with("Program data: "))
        .count()
}

fn anchor_log_events<T>(meta: &TransactionMetadata) -> Vec<T>
where
    T: AnchorDeserialize + Discriminator,
{
    meta.logs
        .iter()
        .filter_map(|log| {
            let encoded = log.strip_prefix("Program data: ")?;
            let data = BASE64_STANDARD.decode(encoded).ok()?;
            decode_anchor_log_event::<T>(&data)
        })
        .collect()
}

fn decode_anchor_log_event<T>(data: &[u8]) -> Option<T>
where
    T: AnchorDeserialize + Discriminator,
{
    let payload = data.strip_prefix(T::DISCRIMINATOR)?;
    T::deserialize(&mut &*payload).ok()
}

fn decode_binary_op_event(data: &[u8]) -> Option<FheBinaryOpEvent> {
    let event_prefix = anchor_event_prefix(FheBinaryOpEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    FheBinaryOpEvent::deserialize(&mut &*payload).ok()
}

fn decode_ternary_op_event(data: &[u8]) -> Option<FheTernaryOpEvent> {
    let event_prefix = anchor_event_prefix(FheTernaryOpEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    FheTernaryOpEvent::deserialize(&mut &*payload).ok()
}

fn decode_trivial_encrypt_event(data: &[u8]) -> Option<TrivialEncryptEvent> {
    let event_prefix = anchor_event_prefix(TrivialEncryptEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    TrivialEncryptEvent::deserialize(&mut &*payload).ok()
}

fn decode_fhe_rand_event(data: &[u8]) -> Option<FheRandEvent> {
    let event_prefix = anchor_event_prefix(FheRandEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    FheRandEvent::deserialize(&mut &*payload).ok()
}

fn decode_fhe_rand_bounded_event(data: &[u8]) -> Option<FheRandBoundedEvent> {
    let event_prefix = anchor_event_prefix(FheRandBoundedEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    FheRandBoundedEvent::deserialize(&mut &*payload).ok()
}

fn decode_acl_allowed_event(data: &[u8]) -> Option<AclAllowedEvent> {
    let event_prefix = anchor_event_prefix(AclAllowedEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    AclAllowedEvent::deserialize(&mut &*payload).ok()
}

fn decode_public_decrypt_allowed_event(data: &[u8]) -> Option<PublicDecryptAllowedEvent> {
    let event_prefix = anchor_event_prefix(PublicDecryptAllowedEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    PublicDecryptAllowedEvent::deserialize(&mut &*payload).ok()
}

fn decode_handle_material_committed_event(data: &[u8]) -> Option<HandleMaterialCommittedEvent> {
    let event_prefix = anchor_event_prefix(HandleMaterialCommittedEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    HandleMaterialCommittedEvent::deserialize(&mut &*payload).ok()
}

fn decode_handle_material_sealed_event(data: &[u8]) -> Option<HandleMaterialSealedEvent> {
    let event_prefix = anchor_event_prefix(HandleMaterialSealedEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    HandleMaterialSealedEvent::deserialize(&mut &*payload).ok()
}

fn decode_balance_handle_updated_event(data: &[u8]) -> Option<BalanceHandleUpdatedEvent> {
    let event_prefix = anchor_event_prefix(BalanceHandleUpdatedEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    BalanceHandleUpdatedEvent::deserialize(&mut &*payload).ok()
}

fn decode_total_supply_handle_updated_event(data: &[u8]) -> Option<TotalSupplyHandleUpdatedEvent> {
    let event_prefix = anchor_event_prefix(TotalSupplyHandleUpdatedEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    TotalSupplyHandleUpdatedEvent::deserialize(&mut &*payload).ok()
}

fn decode_random_amount_created_event(data: &[u8]) -> Option<RandomAmountCreatedEvent> {
    let event_prefix = anchor_event_prefix(RandomAmountCreatedEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    RandomAmountCreatedEvent::deserialize(&mut &*payload).ok()
}

fn decode_balance_disclosure_requested_event(
    data: &[u8],
) -> Option<BalanceDisclosureRequestedEvent> {
    let event_prefix = anchor_event_prefix(BalanceDisclosureRequestedEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    BalanceDisclosureRequestedEvent::deserialize(&mut &*payload).ok()
}

fn decode_amount_disclosure_requested_event(data: &[u8]) -> Option<AmountDisclosureRequestedEvent> {
    let event_prefix = anchor_event_prefix(AmountDisclosureRequestedEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    AmountDisclosureRequestedEvent::deserialize(&mut &*payload).ok()
}

fn decode_balance_disclosed_event(data: &[u8]) -> Option<BalanceDisclosedEvent> {
    let event_prefix = anchor_event_prefix(BalanceDisclosedEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    BalanceDisclosedEvent::deserialize(&mut &*payload).ok()
}

fn decode_amount_disclosed_event(data: &[u8]) -> Option<AmountDisclosedEvent> {
    let event_prefix = anchor_event_prefix(AmountDisclosedEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    AmountDisclosedEvent::deserialize(&mut &*payload).ok()
}

fn decode_burn_redeemed_event(data: &[u8]) -> Option<BurnRedeemedEvent> {
    let event_prefix = anchor_event_prefix(BurnRedeemedEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    BurnRedeemedEvent::deserialize(&mut &*payload).ok()
}

fn decode_confidential_burn_event(data: &[u8]) -> Option<ConfidentialBurnEvent> {
    let event_prefix = anchor_event_prefix(ConfidentialBurnEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    ConfidentialBurnEvent::deserialize(&mut &*payload).ok()
}

fn decode_confidential_transfer_event(data: &[u8]) -> Option<ConfidentialTransferEvent> {
    let event_prefix = anchor_event_prefix(ConfidentialTransferEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    ConfidentialTransferEvent::deserialize(&mut &*payload).ok()
}

fn anchor_event_prefix(discriminator: &[u8]) -> Vec<u8> {
    anchor_lang::event::EVENT_IX_TAG_LE
        .iter()
        .copied()
        .chain(discriminator.iter().copied())
        .collect()
}

fn label(name: &str) -> [u8; 32] {
    let mut out = [0_u8; 32];
    let bytes = name.as_bytes();
    assert!(bytes.len() <= out.len());
    out[..bytes.len()].copy_from_slice(bytes);
    out
}

fn input_handle_for_chain(seed: u8) -> [u8; 32] {
    input_handle_for_chain_with_index_and_type(seed, 0, 5)
}

fn input_handle_for_chain_with_index(seed: u8, handle_index: u8) -> [u8; 32] {
    input_handle_for_chain_with_index_and_type(seed, handle_index, 5)
}

fn input_handle_for_chain_with_type(seed: u8, fhe_type: u8) -> [u8; 32] {
    input_handle_for_chain_with_index_and_type(seed, 0, fhe_type)
}

fn input_handle_for_chain_with_index_and_type(
    seed: u8,
    handle_index: u8,
    fhe_type: u8,
) -> [u8; 32] {
    let mut handle = [seed; 32];
    handle[21] = handle_index;
    handle[22..30].copy_from_slice(&host::SOLANA_POC_CHAIN_ID.to_be_bytes());
    handle[30] = fhe_type;
    handle[31] = host::HANDLE_VERSION;
    handle
}

fn ed25519_verify_ix(authority: &Keypair, message: &[u8]) -> Instruction {
    const PUBKEY_SIZE: usize = 32;
    const SIGNATURE_SIZE: usize = 64;
    const OFFSETS_SIZE: usize = 14;
    const OFFSETS_START: usize = 2;
    const DATA_START: usize = OFFSETS_START + OFFSETS_SIZE;

    let public_key_offset = DATA_START;
    let signature_offset = public_key_offset + PUBKEY_SIZE;
    let message_data_offset = signature_offset + SIGNATURE_SIZE;
    let signature = authority.sign_message(message);

    let mut data = Vec::with_capacity(message_data_offset + message.len());
    data.push(1);
    data.push(0);
    data.extend_from_slice(&(signature_offset as u16).to_le_bytes());
    data.extend_from_slice(&u16::MAX.to_le_bytes());
    data.extend_from_slice(&(public_key_offset as u16).to_le_bytes());
    data.extend_from_slice(&u16::MAX.to_le_bytes());
    data.extend_from_slice(&(message_data_offset as u16).to_le_bytes());
    data.extend_from_slice(&(message.len() as u16).to_le_bytes());
    data.extend_from_slice(&u16::MAX.to_le_bytes());
    data.extend_from_slice(authority.pubkey().as_ref());
    data.extend_from_slice(signature.as_ref());
    data.extend_from_slice(message);

    Instruction {
        program_id: ed25519_program::ID,
        accounts: Vec::new(),
        data,
    }
}

fn delegate_for_user_decryption_ix(
    program_id: Pubkey,
    payer: Pubkey,
    host_config: Pubkey,
    delegation_record: Pubkey,
    delegate: Pubkey,
    app_account: Pubkey,
    expiration_slot: u64,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::DelegateForUserDecryption {
            payer,
            delegator: payer,
            host_config,
            delegation_record,
            system_program: system_program::ID,
        },
        host::instruction::DelegateForUserDecryption {
            delegate,
            app_account,
            expiration_slot,
        },
    )
}

fn anchor_ix<A, D>(program_id: Pubkey, accounts: A, args: D) -> Instruction
where
    A: ToAccountMetas,
    D: InstructionData,
{
    Program::new(program_id)
        .accounts(accounts)
        .args(args)
        .instruction()
        .unwrap()
}

fn send(svm: &mut LiteSVM, payer: &Keypair, ix: Instruction) {
    try_send(svm, payer, ix).unwrap();
}

fn send_with_meta(
    svm: &mut LiteSVM,
    payer: &Keypair,
    ix: Instruction,
) -> (TransactionMetadata, Vec<Pubkey>) {
    send_many_with_meta(svm, payer, vec![ix])
}

fn send_many_with_meta(
    svm: &mut LiteSVM,
    payer: &Keypair,
    ixs: Vec<Instruction>,
) -> (TransactionMetadata, Vec<Pubkey>) {
    let ixs = with_token_compute_budget(ixs);
    let message = Message::new_with_blockhash(&ixs, Some(&payer.pubkey()), &svm.latest_blockhash());
    let account_keys = message.account_keys.clone();
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), &[payer]).unwrap();
    (svm.send_transaction(tx).unwrap(), account_keys)
}

fn send_many_with_signers_with_meta(
    svm: &mut LiteSVM,
    payer: &Pubkey,
    ixs: Vec<Instruction>,
    signers: &[&Keypair],
) -> (TransactionMetadata, Vec<Pubkey>) {
    let ixs = with_token_compute_budget(ixs);
    let message = Message::new_with_blockhash(&ixs, Some(payer), &svm.latest_blockhash());
    let account_keys = message.account_keys.clone();
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), signers).unwrap();
    (svm.send_transaction(tx).unwrap(), account_keys)
}

fn try_send_many(svm: &mut LiteSVM, payer: &Keypair, ixs: Vec<Instruction>) -> TransactionResult {
    let ixs = with_token_compute_budget(ixs);
    let message = Message::new_with_blockhash(&ixs, Some(&payer.pubkey()), &svm.latest_blockhash());
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), &[payer]).unwrap();
    svm.send_transaction(tx)
}

fn try_send(svm: &mut LiteSVM, payer: &Keypair, ix: Instruction) -> TransactionResult {
    send_with_signers(svm, &payer.pubkey(), ix, &[payer])
}

fn send_with_signers(
    svm: &mut LiteSVM,
    payer: &Pubkey,
    ix: Instruction,
    signers: &[&Keypair],
) -> TransactionResult {
    let ixs = with_token_compute_budget(vec![ix]);
    let message = Message::new_with_blockhash(&ixs, Some(payer), &svm.latest_blockhash());
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), signers).unwrap();
    svm.send_transaction(tx)
}

fn send_many_with_signers(
    svm: &mut LiteSVM,
    payer: &Pubkey,
    ixs: Vec<Instruction>,
    signers: &[&Keypair],
) -> TransactionResult {
    let ixs = with_token_compute_budget(ixs);
    let message = Message::new_with_blockhash(&ixs, Some(payer), &svm.latest_blockhash());
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), signers).unwrap();
    svm.send_transaction(tx)
}

fn with_token_compute_budget(ixs: Vec<Instruction>) -> Vec<Instruction> {
    if !ixs.iter().any(|ix| ix.program_id == token::id()) {
        return ixs;
    }
    let mut budgeted = Vec::with_capacity(ixs.len() + 2);
    budgeted.push(ComputeBudgetInstruction::request_heap_frame(
        TOKEN_HEAP_FRAME_BYTES,
    ));
    budgeted.push(ComputeBudgetInstruction::set_compute_unit_limit(
        TOKEN_COMPUTE_UNIT_LIMIT,
    ));
    budgeted.extend(ixs);
    budgeted
}
