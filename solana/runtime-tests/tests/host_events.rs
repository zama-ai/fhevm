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
use anchor_spl::token::spl_token;
use confidential_token as token;
use litesvm::{
    types::{TransactionMetadata, TransactionResult},
    LiteSVM,
};
use solana_sdk::{
    account::Account,
    instruction::Instruction,
    message::{Message, VersionedMessage},
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    transaction::VersionedTransaction,
};
use zama_host as host;
use zama_host::{
    AclRecord, AclSubjectEntry, FheBinaryOpCode, FheBinaryOpEvent, TrivialEncryptEvent,
};

use support::fhe_runtime::{CleartextBackend, FheBackend, TypedClearValue};

const DEFAULT_INPUT_NONCE_SEQUENCE: u64 = 0;

#[test]
fn test_emit_trivial_encrypt_emits_anchor_cpi_event() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();

    let ix = anchor_ix(
        program_id,
        host::accounts::TestEmitProtocolEvent {
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
fn bind_acl_record_persists_keyed_nonce_record_without_handle_derived_address() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let app_account_authority = Keypair::new();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = app_account_authority.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let nonce_sequence = 42;
    let subject = payer.pubkey();
    let handle = [9; 32];
    let acl_record = acl_record_address(program_id, nonce_key, nonce_sequence);
    let authorizing_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        nonce_sequence - 1,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        payer.pubkey(),
    );

    let ix = anchor_ix(
        program_id,
        host::accounts::BindAclRecord {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            app_account_authority: app_account,
            authorizing_acl_record,
            acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::BindAclRecord {
            nonce_key,
            nonce_sequence,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            handle,
            subjects: vec![AclSubjectEntry { pubkey: subject }],
            public_decrypt: false,
        },
    );

    send_with_signers(
        &mut svm,
        &payer.pubkey(),
        ix,
        &[&payer, &app_account_authority],
    )
    .unwrap();

    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record.handle, handle);
    assert_eq!(record.nonce_key, nonce_key);
    assert_eq!(record.nonce_sequence, nonce_sequence);
    assert_eq!(record.acl_domain_key, acl_domain_key);
    assert_eq!(record.app_account, app_account);
    assert_eq!(record.encrypted_value_label, encrypted_value_label);
    assert_eq!(record_subjects(&record), vec![subject]);
    assert!(!record.public_decrypt);

    let assert_ix = anchor_ix(
        program_id,
        host::accounts::AssertAclRecord { acl_record },
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
    send(&mut svm, &payer, assert_ix);
}

#[test]
fn bind_acl_record_rejects_nonce_key_not_derived_from_acl_fields() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let app_account_authority = Keypair::new();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = app_account_authority.pubkey();
    let encrypted_value_label = label("balance");
    let wrong_nonce_key = [42; 32];
    let nonce_sequence = 42;
    let acl_record = acl_record_address(program_id, wrong_nonce_key, nonce_sequence);
    let handle = [9; 32];
    let authorizing_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        token::nonce_key(acl_domain_key, app_account, encrypted_value_label),
        nonce_sequence - 1,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        payer.pubkey(),
    );

    let ix = anchor_ix(
        program_id,
        host::accounts::BindAclRecord {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            app_account_authority: app_account,
            authorizing_acl_record,
            acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::BindAclRecord {
            nonce_key: wrong_nonce_key,
            nonce_sequence,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            handle,
            subjects: vec![AclSubjectEntry {
                pubkey: payer.pubkey(),
            }],
            public_decrypt: false,
        },
    );

    assert!(send_with_signers(
        &mut svm,
        &payer.pubkey(),
        ix,
        &[&payer, &app_account_authority],
    )
    .is_err());
}

#[test]
fn assert_acl_record_rejects_noncanonical_acl_record_address() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = Pubkey::new_unique();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let nonce_sequence = 42;
    let subject = payer.pubkey();
    let handle = [9; 32];
    let noncanonical_acl_record = Pubkey::new_unique();
    let mut subjects = [Pubkey::default(); host::MAX_ACL_SUBJECTS];
    subjects[0] = subject;

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
                subject_count: 1,
                public_decrypt: false,
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
fn fhe_binary_op_scalar_rhs_skips_rhs_acl_but_encrypted_rhs_requires_it() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let dummy_rhs_account = svm.create_funded_account(1_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = [7; 32];
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

    let scalar_ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOp {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            lhs_acl_record,
            rhs_acl_record: dummy_rhs_account.pubkey(),
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOp {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs: rhs_scalar,
            scalar: true,
            output_fhe_type: 5,
            output_nonce_key: nonce_key,
            output_nonce_sequence: 1,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry {
                pubkey: payer.pubkey(),
            }],
            output_public_decrypt: false,
        },
    );

    let mut cleartext = CleartextBackend::default();
    cleartext.seed_cleartext(lhs, TypedClearValue::uint64(10));
    let (meta, account_keys) = send_with_meta(&mut svm, &payer, scalar_ix);
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
    let encrypted_rhs_ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOp {
            payer: payer.pubkey(),
            compute_subject: payer.pubkey(),
            app_account_authority: app_account,
            lhs_acl_record,
            rhs_acl_record: dummy_rhs_account.pubkey(),
            output_acl_record: encrypted_rhs_output,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOp {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs: [9; 32],
            scalar: false,
            output_fhe_type: 5,
            output_nonce_key: nonce_key,
            output_nonce_sequence: 2,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry {
                pubkey: payer.pubkey(),
            }],
            output_public_decrypt: false,
        },
    );
    assert!(try_send(&mut svm, &payer, encrypted_rhs_ix).is_err());
    assert!(read_acl_record(&svm, encrypted_rhs_output).is_none());
}

#[test]
fn bind_acl_record_cannot_rebind_existing_acl_record() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    let app_account_authority = Keypair::new();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = app_account_authority.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let nonce_sequence = 42;
    let subject = payer.pubkey();
    let original_handle = [9; 32];
    let replacement_handle = [7; 32];
    let acl_record = acl_record_address(program_id, nonce_key, nonce_sequence);
    let authorizing_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        nonce_sequence - 1,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        original_handle,
        payer.pubkey(),
    );

    let build_ix = |handle| {
        anchor_ix(
            program_id,
            host::accounts::BindAclRecord {
                payer: payer.pubkey(),
                authority: payer.pubkey(),
                app_account_authority: app_account,
                authorizing_acl_record,
                acl_record,
                system_program: system_program::ID,
                event_authority: event_authority(program_id),
                program: program_id,
            },
            host::instruction::BindAclRecord {
                nonce_key,
                nonce_sequence,
                acl_domain_key,
                app_account,
                encrypted_value_label,
                handle,
                subjects: vec![AclSubjectEntry { pubkey: subject }],
                public_decrypt: false,
            },
        )
    };

    send_with_signers(
        &mut svm,
        &payer.pubkey(),
        build_ix(original_handle),
        &[&payer, &app_account_authority],
    )
    .unwrap();

    assert!(
        send_with_signers(
            &mut svm,
            &payer.pubkey(),
            build_ix(replacement_handle),
            &[&payer, &app_account_authority],
        )
        .is_err(),
        "Anchor init must reject rebinding an existing ACL record"
    );

    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record.handle, original_handle);
    assert_eq!(record.nonce_key, nonce_key);
    assert_eq!(record.nonce_sequence, nonce_sequence);
    assert_eq!(record.acl_domain_key, acl_domain_key);
    assert_eq!(record.app_account, app_account);
    assert_eq!(record.encrypted_value_label, encrypted_value_label);
    assert_eq!(record_subjects(&record), vec![subject]);
    assert!(!record.public_decrypt);
}

#[test]
fn bind_acl_record_rejects_app_account_without_matching_authority() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = Pubkey::new_unique();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let acl_record = acl_record_address(program_id, nonce_key, 0);
    let handle = [9; 32];
    let authorizing_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        nonce_key,
        1,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        payer.pubkey(),
    );
    let ix = anchor_ix(
        program_id,
        host::accounts::BindAclRecord {
            payer: payer.pubkey(),
            authority: payer.pubkey(),
            app_account_authority: payer.pubkey(),
            authorizing_acl_record,
            acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::BindAclRecord {
            nonce_key,
            nonce_sequence: 0,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            handle,
            subjects: vec![AclSubjectEntry {
                pubkey: payer.pubkey(),
            }],
            public_decrypt: false,
        },
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
}

#[test]
fn bind_acl_record_rejects_handle_laundering_by_unallowed_authority() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let alice = Keypair::new();
    let mallory = svm.create_funded_account(1_000_000_000).unwrap();
    let mallory_app_account = Keypair::new();

    let acl_domain_key = Pubkey::new_unique();
    let encrypted_value_label = label("balance");
    let handle = [7; 32];
    let source_app_account = Pubkey::new_unique();
    let source_nonce_key =
        token::nonce_key(acl_domain_key, source_app_account, encrypted_value_label);
    let authorizing_acl_record = seed_authorizing_acl_record(
        &mut svm,
        program_id,
        source_nonce_key,
        0,
        acl_domain_key,
        source_app_account,
        encrypted_value_label,
        handle,
        alice.pubkey(),
    );

    let target_nonce_key = token::nonce_key(
        acl_domain_key,
        mallory_app_account.pubkey(),
        encrypted_value_label,
    );
    let target_acl_record = acl_record_address(program_id, target_nonce_key, 0);
    let ix = anchor_ix(
        program_id,
        host::accounts::BindAclRecord {
            payer: mallory.pubkey(),
            authority: mallory.pubkey(),
            app_account_authority: mallory_app_account.pubkey(),
            authorizing_acl_record,
            acl_record: target_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::BindAclRecord {
            nonce_key: target_nonce_key,
            nonce_sequence: 0,
            acl_domain_key,
            app_account: mallory_app_account.pubkey(),
            encrypted_value_label,
            handle,
            subjects: vec![AclSubjectEntry {
                pubkey: mallory.pubkey(),
            }],
            public_decrypt: false,
        },
    );

    assert!(send_with_signers(
        &mut svm,
        &mallory.pubkey(),
        ix,
        &[&mallory, &mallory_app_account],
    )
    .is_err());
}

#[test]
fn confidential_transfer_rotates_balance_handles_and_binds_output_acl() {
    let mut fixture = token_fixture();
    let amount_handle = [9; 32];
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
    let new_alice = alice_record.handle;
    let new_bob = bob_record.handle;
    let events = binary_op_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].version, 0);
    assert_eq!(events[0].op, FheBinaryOpCode::Sub);
    assert_eq!(events[0].subject, fixture.compute_signer);
    assert_eq!(events[0].lhs, fixture.alice_initial);
    assert_eq!(events[0].rhs, amount_handle);
    assert!(!events[0].scalar);
    assert_eq!(events[0].result, new_alice);
    assert_eq!(events[1].version, 0);
    assert_eq!(events[1].op, FheBinaryOpCode::Add);
    assert_eq!(events[1].subject, fixture.compute_signer);
    assert_eq!(events[1].lhs, fixture.bob_initial);
    assert_eq!(events[1].rhs, amount_handle);
    assert!(!events[1].scalar);
    assert_eq!(events[1].result, new_bob);
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
fn confidential_self_transfer_is_no_op() {
    let mut fixture = token_fixture();
    let amount_handle = [9; 32];
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = transfer_output_accounts(&fixture, 2);
    let ix = self_transfer_ix(&fixture, output, amount_handle);

    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, ix);

    assert!(binary_op_events(&meta, &account_keys, fixture.host_program_id).is_empty());
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
}

#[test]
fn user_decrypt_model_uses_acl_domain_key_and_acl_record_authentication() {
    let mut fixture = token_fixture();
    let amount_handle = [9; 32];
    authorize_input_compute_acl(&mut fixture, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE);
    let first_output = transfer_output_accounts(&fixture, 1);
    let first_ix = transfer_ix(&fixture, first_output, amount_handle);
    send(&mut fixture.svm, &fixture.alice, first_ix);
    let first_alice = read_acl_record(&fixture.svm, first_output.alice)
        .expect("expected first Alice ACL")
        .handle;

    authorize_input_compute_acl(&mut fixture, [8; 32], 1);
    let second_output = transfer_output_accounts(&fixture, 2);
    let second_ix = transfer_ix_with_current_acl_and_amount_nonce(
        &fixture,
        first_output.alice,
        first_output.bob,
        second_output,
        [8; 32],
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
    let amount_handle = [9; 32];
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
fn allow_for_decryption_rejects_unallowed_signer() {
    let mut fixture = token_fixture();
    let amount_handle = [9; 32];
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
fn wrap_usdc_escrows_spl_tokens_and_rotates_confidential_balance() {
    let mut fixture = token_fixture();
    let amount = 100_000_000;
    let mut cleartext = CleartextBackend::default();
    cleartext.seed_cleartext(fixture.alice_initial, TypedClearValue::uint64(125));

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
    assert_eq!(trivial_events[0].subject, fixture.compute_signer);
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
    let new_alice = output_record.handle;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].op, FheBinaryOpCode::Add);
    assert_eq!(events[0].subject, fixture.compute_signer);
    assert_eq!(events[0].lhs, fixture.alice_initial);
    assert_eq!(events[0].rhs, amount_handle);
    assert_eq!(events[0].result, new_alice);
    assert_eq!(
        cleartext.decrypt_cleartext(new_alice),
        Some(TypedClearValue::uint64(100_000_125))
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

    assert_balance_acl(
        &fixture.svm,
        output.balance,
        fixture.mint.pubkey(),
        fixture.alice_token,
        1,
        new_alice,
        &[fixture.alice.pubkey(), fixture.compute_signer],
    );
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
    let transfer_amount_handle = [8; 32];
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
    authorize_input_compute_acl(&mut fixture, [9; 32], DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = transfer_output_accounts(&fixture, 1);
    let transfer_ix = transfer_ix(&fixture, output, [9; 32]);
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
    let host_events = binary_op_events(&meta, &account_keys, fixture.host_program_id).len();
    let max_cpi_depth = max_cpi_depth(&meta);

    assert_eq!(top_level_metas, 13);
    assert_eq!(writable_metas, 5);
    assert_eq!(signer_metas, 1);
    assert_eq!(host_events, 2);
    assert_eq!(created_acl_count(&fixture.svm, output), 2);
    assert!(
        inner_instructions <= 12,
        "inner instructions: {inner_instructions}"
    );
    assert!(
        meta.compute_units_consumed <= 150_000,
        "compute units: {}",
        meta.compute_units_consumed
    );
    assert_eq!(max_cpi_depth, 3);
}

#[test]
fn confidential_transfer_rejects_stale_current_acl() {
    let mut fixture = token_fixture();
    authorize_input_compute_acl(&mut fixture, [9; 32], DEFAULT_INPUT_NONCE_SEQUENCE);
    let first_output = transfer_output_accounts(&fixture, 1);
    let first_ix = transfer_ix(&fixture, first_output, [9; 32]);
    send(&mut fixture.svm, &fixture.alice, first_ix);

    authorize_input_compute_acl(&mut fixture, [8; 32], 1);
    let stale_ix =
        transfer_ix_with_amount_nonce(&fixture, transfer_output_accounts(&fixture, 2), [8; 32], 1);
    assert!(try_send(&mut fixture.svm, &fixture.alice, stale_ix).is_err());
}

#[test]
fn confidential_transfer_rejects_wrong_current_acl_record() {
    let mut fixture = token_fixture();
    authorize_input_compute_acl(&mut fixture, [9; 32], DEFAULT_INPUT_NONCE_SEQUENCE);
    let ix = transfer_ix_with_current_acl(
        &fixture,
        fixture.bob_current_compute_acl,
        fixture.bob_current_compute_acl,
        transfer_output_accounts(&fixture, 1),
        [9; 32],
    );
    assert!(try_send(&mut fixture.svm, &fixture.alice, ix).is_err());
}

#[test]
fn confidential_transfer_rejects_wrong_amount_acl() {
    let mut fixture = token_fixture();
    let amount_handle = [9; 32];
    let wrong_amount_handle = [8; 32];
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
fn confidential_transfer_rejects_output_acl_for_wrong_token_account() {
    let mut fixture = token_fixture();
    authorize_input_compute_acl(&mut fixture, [9; 32], DEFAULT_INPUT_NONCE_SEQUENCE);
    let correct_output = transfer_output_accounts(&fixture, 1);
    let swapped_output = TransferOutputAccounts {
        alice: correct_output.bob,
        bob: correct_output.alice,
    };

    let ix = transfer_ix(&fixture, swapped_output, [9; 32]);

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
    authorize_input_compute_acl(&mut fixture, [9; 32], DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = TransferOutputAccounts {
        alice: fixture.alice_current_compute_acl,
        bob: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.bob_token,
            1,
        ),
    };

    let ix = transfer_ix(&fixture, output, [9; 32]);

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

struct TokenFixture {
    svm: LiteSVM,
    host_program_id: Pubkey,
    token_program_id: Pubkey,
    alice: Keypair,
    bob: Keypair,
    mint: Keypair,
    underlying_mint: Keypair,
    compute_signer: Pubkey,
    alice_token: Pubkey,
    bob_token: Pubkey,
    alice_usdc: Pubkey,
    vault_usdc: Pubkey,
    alice_initial: [u8; 32],
    bob_initial: [u8; 32],
    alice_current_compute_acl: Pubkey,
    bob_current_compute_acl: Pubkey,
}

#[derive(Clone, Copy)]
struct TransferOutputAccounts {
    alice: Pubkey,
    bob: Pubkey,
}

#[derive(Clone, Copy)]
struct WrapOutputAccounts {
    amount: Pubkey,
    balance: Pubkey,
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

fn host_program_so_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy/zama_host.so")
}

fn token_program_so_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy/confidential_token.so")
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
    AnchorLiteSVM::build_with_programs(&programs).svm
}

fn token_fixture() -> TokenFixture {
    let host_program_id = host::id();
    let token_program_id = token::id();
    let mut svm = svm_with_programs(&[
        (host_program_id, host_program_so_path()),
        (token_program_id, token_program_so_path()),
    ]);

    let alice = svm.create_funded_account(2_000_000_000).unwrap();
    let bob = svm.create_funded_account(1_000_000_000).unwrap();
    let mint = Keypair::new();
    let underlying_mint = svm.create_token_mint(&alice, 6).unwrap();

    let vault_authority = vault_authority_address(token_program_id, mint.pubkey());
    let alice_usdc = svm
        .create_token_account(&underlying_mint.pubkey(), &alice)
        .unwrap();
    let vault_usdc = Keypair::new();
    create_spl_token_account(
        &mut svm,
        &alice,
        &vault_usdc,
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

    send_with_signers(
        &mut svm,
        &alice.pubkey(),
        anchor_ix(
            token_program_id,
            token::accounts::InitializeMint {
                authority: alice.pubkey(),
                mint: mint.pubkey(),
                underlying_mint: underlying_mint.pubkey(),
                system_program: system_program::ID,
            },
            token::instruction::InitializeMint {},
        ),
        &[&alice, &mint],
    )
    .unwrap();

    let compute_signer = token::compute_signer_address(mint.pubkey()).0;
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
        &alice,
        mint.pubkey(),
        alice_token,
        compute_signer,
        alice_current_compute_acl,
        125,
    );
    initialize_confidential_token_account(
        &mut svm,
        token_program_id,
        host_program_id,
        &bob,
        mint.pubkey(),
        bob_token,
        compute_signer,
        bob_current_compute_acl,
        20,
    );
    let alice_initial = read_acl_record(&svm, alice_current_compute_acl)
        .expect("expected Alice initial ACL")
        .handle;
    let bob_initial = read_acl_record(&svm, bob_current_compute_acl)
        .expect("expected Bob initial ACL")
        .handle;

    TokenFixture {
        svm,
        host_program_id,
        token_program_id,
        alice,
        bob,
        mint,
        underlying_mint,
        compute_signer,
        alice_token,
        bob_token,
        alice_usdc: alice_usdc.pubkey(),
        vault_usdc: vault_usdc.pubkey(),
        alice_initial,
        bob_initial,
        alice_current_compute_acl,
        bob_current_compute_acl,
    }
}

fn initialize_confidential_token_account(
    svm: &mut LiteSVM,
    token_program_id: Pubkey,
    host_program_id: Pubkey,
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
                system_program: system_program::ID,
            },
            token::instruction::InitializeTokenAccount { initial_balance },
        ),
    );
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

fn input_compute_acl_address(fixture: &TokenFixture, nonce_sequence: u64) -> Pubkey {
    acl_record_address(
        fixture.host_program_id,
        token::nonce_key(
            fixture.alice.pubkey(),
            fixture.alice.pubkey(),
            label("input"),
        ),
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
    }
}

fn authorize_input_compute_acl(fixture: &mut TokenFixture, handle: [u8; 32], nonce_sequence: u64) {
    // Temporary mock short-circuit for the future Solana input verifier /
    // transciphering boundary. This deliberately trusts the caller-supplied
    // handle so tests can exercise ACL + compute semantics before the real
    // input proof path exists.
    let acl_domain_key = fixture.alice.pubkey();
    let app_account = fixture.alice.pubkey();
    let encrypted_value_label = label("input");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let acl_record = acl_record_address(fixture.host_program_id, nonce_key, nonce_sequence);
    let ix = anchor_ix(
        fixture.host_program_id,
        host::accounts::MockInputVerifiedAndBind {
            payer: fixture.alice.pubkey(),
            app_account_authority: app_account,
            output_acl_record: acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.host_program_id),
            program: fixture.host_program_id,
        },
        host::instruction::MockInputVerifiedAndBind {
            input_handle: handle,
            user: fixture.alice.pubkey(),
            output_nonce_key: nonce_key,
            output_nonce_sequence: nonce_sequence,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry {
                pubkey: fixture.compute_signer,
            }],
            output_public_decrypt: false,
        },
    );
    send(&mut fixture.svm, &fixture.alice, ix);
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
            from_output_acl: output.alice,
            to_output_acl: output.bob,
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            system_program: system_program::ID,
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
            from_output_acl: output.alice,
            to_output_acl: output.bob,
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            system_program: system_program::ID,
        },
        token::instruction::ConfidentialTransfer { amount_handle },
    )
}

fn wrap_usdc_ix(fixture: &TokenFixture, output: WrapOutputAccounts, amount: u64) -> Instruction {
    anchor_ix(
        fixture.token_program_id,
        token::accounts::WrapUsdc {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            token_account: fixture.alice_token,
            underlying_mint: fixture.underlying_mint.pubkey(),
            user_usdc: fixture.alice_usdc,
            vault_usdc: fixture.vault_usdc,
            vault_authority: vault_authority_address(
                fixture.token_program_id,
                fixture.mint.pubkey(),
            ),
            compute_signer: fixture.compute_signer,
            current_compute_acl: fixture.alice_current_compute_acl,
            amount_compute_acl: output.amount,
            output_acl: output.balance,
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            token_program: spl_token::id(),
            system_program: system_program::ID,
        },
        token::instruction::WrapUsdc { amount },
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
            acl_record,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowForDecryption { handle },
    )
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

fn read_acl_record(svm: &LiteSVM, address: Pubkey) -> Option<AclRecord> {
    let account = svm.get_account(&address)?;
    let mut data = account.data.as_slice();
    AclRecord::try_deserialize(&mut data).ok()
}

fn serialized_acl_record(record: AclRecord) -> Vec<u8> {
    let mut data = Vec::new();
    record.try_serialize(&mut data).unwrap();
    data
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
    let (address, bump) = Pubkey::find_program_address(
        &[
            b"acl-record",
            nonce_key.as_ref(),
            &nonce_sequence.to_le_bytes(),
        ],
        &program_id,
    );
    let mut subjects = [Pubkey::default(); host::MAX_ACL_SUBJECTS];
    subjects[0] = authority;
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
                subject_count: 1,
                public_decrypt: false,
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

fn record_subjects(record: &AclRecord) -> Vec<Pubkey> {
    record.subjects[..record.subject_count as usize].to_vec()
}

fn created_acl_count(svm: &LiteSVM, output: TransferOutputAccounts) -> usize {
    [output.alice, output.bob]
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

fn decode_binary_op_event(data: &[u8]) -> Option<FheBinaryOpEvent> {
    let event_prefix = anchor_event_prefix(FheBinaryOpEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    FheBinaryOpEvent::deserialize(&mut &*payload).ok()
}

fn decode_trivial_encrypt_event(data: &[u8]) -> Option<TrivialEncryptEvent> {
    let event_prefix = anchor_event_prefix(TrivialEncryptEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    TrivialEncryptEvent::deserialize(&mut &*payload).ok()
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
    let message =
        Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &svm.latest_blockhash());
    let account_keys = message.account_keys.clone();
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), &[payer]).unwrap();
    (svm.send_transaction(tx).unwrap(), account_keys)
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
    let message = Message::new_with_blockhash(&[ix], Some(payer), &svm.latest_blockhash());
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), signers).unwrap();
    svm.send_transaction(tx)
}

fn send_many_with_signers(
    svm: &mut LiteSVM,
    payer: &Pubkey,
    ixs: Vec<Instruction>,
    signers: &[&Keypair],
) -> TransactionResult {
    let message = Message::new_with_blockhash(&ixs, Some(payer), &svm.latest_blockhash());
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), signers).unwrap();
    svm.send_transaction(tx)
}
