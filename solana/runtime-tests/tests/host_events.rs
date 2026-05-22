// Test builders mirror Anchor instruction surfaces and LiteSVM result types.
#![allow(clippy::result_large_err, clippy::too_many_arguments)]

use anchor_lang::{prelude::system_program, AccountSerialize};
use anchor_litesvm::TestHelpers;
use confidential_token::{self as token, BalanceHandleUpdateReason};
use solana_sdk::{
    account::Account,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use zama_host as host;
use zama_host::{
    AclRecord, AclSubjectEntry, FheFrameAction, FheFrameStep, FheOpcode, FheOperand,
};
use zama_solana_litesvm_harness::{
    acl_record_address, allow_for_decryption_ix, amount_plaintext, anchor_ix,
    assert_acl_record, assert_balance_acl,
    authorization_payload_bytes, authorize_transfer_amount,
    balance_acl_record_address, binary_op_events, balance_handle_updated_events, CleartextBackend,
    cleartext_rand_value, created_acl_count, event_authority, execute_frame_ix, execute_frame_log_count,
    expected_trivial_handle, fhe_rand_events, FheBackend, FheBinaryOpCode, host_program_so_path, kms_like_public_decrypt_check,
    kms_like_user_decrypt_check, label, max_cpi_depth, rand_counter_address, read_acl_record,
    read_rand_counter, record_subjects, run_rand_demo_scenario,
    run_transfer_scenario_meta, run_wrap_scenario, seed_transfer_inputs,
    assert_transfer_cleartext, assert_transfer_output_invariants,
    assert_no_zama_host_events_on_failure, assert_wrap_output_invariants,
    previous_bank_hash_from_sysvar, set_previous_slot_hash,
    wrap_output_accounts, wrap_usdc_ix, WrapSetup,
    seed_authorizing_acl_record, self_transfer_ix, send, send_many_with_signers, send_with_meta,
    signed_confidential_rand_user_decrypt_request, signed_current_balance_user_decrypt_request,
    signed_user_decrypt_request, spl_token_amount,
    svm_with_program, token_account, token_fixture, transfer_amount_acl_address, transfer_ix,
    transfer_ix_with_amount_acl, transfer_ix_with_amount_nonce, transfer_ix_with_current_acl,
    transfer_ix_with_current_acl_and_amount_nonce, transfer_output_accounts, try_send,
    try_send_with_meta,
    trivial_encrypt_events, PublicDecryptHandleEntry,
    TransferExpect, TransferOutputAccounts, TransferSetup, TypedClearValue,
    SemanticBackend, UserDecryptHandleEntry, DEFAULT_INPUT_NONCE_SEQUENCE,
    DEFAULT_TEST_PREVIOUS_BANK_HASH,
};

#[test]
fn execute_frame_emits_trivial_encrypt_via_cpi() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_acl_record = acl_record_address(program_id, nonce_key, 1);
    let steps = vec![FheFrameStep::TrivialEncrypt {
        plaintext: amount_plaintext(7),
        fhe_type: 5,
    }];
    let actions = vec![FheFrameAction::Allow {
        source: FheOperand::PreviousResult { index: 0 },
        output_acl_record,
        nonce_key,
        nonce_sequence: 1,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        subjects: vec![AclSubjectEntry {
            pubkey: payer.pubkey(),
        }],
        public_decrypt: false,
    }];
    let ix = execute_frame_ix(
        program_id,
        payer.pubkey(),
        steps,
        actions,
        vec![app_account],
        vec![output_acl_record],
    );

    let (meta, account_keys) = send_with_meta(&mut svm, &payer, ix);
    let events = trivial_encrypt_events(&meta, &account_keys, program_id);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].plaintext, amount_plaintext(7));
    assert_eq!(events[0].fhe_type, 5);
    assert_eq!(events[0].subject, payer.pubkey().to_bytes());
}

#[test]
fn execute_frame_rejects_allow_for_unauthorized_app_account() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let other_app_account = Pubkey::new_unique();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_acl_record = acl_record_address(program_id, nonce_key, 1);
    let steps = vec![FheFrameStep::TrivialEncrypt {
        plaintext: amount_plaintext(7),
        fhe_type: 5,
    }];
    let actions = vec![FheFrameAction::Allow {
        source: FheOperand::PreviousResult { index: 0 },
        output_acl_record,
        nonce_key,
        nonce_sequence: 1,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        subjects: vec![AclSubjectEntry {
            pubkey: payer.pubkey(),
        }],
        public_decrypt: false,
    }];
    let ix = execute_frame_ix(
        program_id,
        payer.pubkey(),
        steps,
        actions,
        vec![other_app_account],
        vec![output_acl_record],
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
}

#[test]
fn allow_acl_subjects_extends_existing_canonical_record() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();

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
            authority: payer.pubkey(),
            acl_record,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry {
                pubkey: new_subject,
            }],
        },
    );

    send(&mut svm, &payer, ix);

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
        host::accounts::AssertAclRecord { acl_record },
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
            authority: payer.pubkey(),
            acl_record,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry {
                pubkey: payer.pubkey(),
            }],
        },
    );

    send(&mut svm, &payer, ix);

    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record_subjects(&record), vec![payer.pubkey()]);
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

    let mut acl_data = Vec::new();
    AclRecord {
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
    }
    .try_serialize(&mut acl_data)
    .unwrap();

    svm.set_account(
        noncanonical_acl_record,
        Account {
            lamports: 1_000_000_000,
            data: acl_data,
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
fn execute_frame_scalar_rhs_skips_rhs_acl_but_encrypted_rhs_requires_it() {
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
    let steps = vec![FheFrameStep::Operation {
        opcode: FheOpcode::Add,
        operands: vec![
            FheOperand::AclRecord {
                handle: lhs,
                acl_record: lhs_acl_record,
            },
            FheOperand::Scalar {
                value: rhs_scalar,
                fhe_type: 5,
            },
        ],
        scalar_byte: 1,
        output_fhe_type: 5,
    }];
    let actions = vec![FheFrameAction::Allow {
        source: FheOperand::PreviousResult { index: 0 },
        output_acl_record,
        nonce_key,
        nonce_sequence: 1,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        subjects: vec![AclSubjectEntry {
            pubkey: payer.pubkey(),
        }],
        public_decrypt: false,
    }];

    let mut cleartext = CleartextBackend::default();
    cleartext.seed_cleartext(lhs, TypedClearValue::uint64(10));
    let ix = execute_frame_ix(
        program_id,
        payer.pubkey(),
        steps,
        actions,
        vec![app_account],
        vec![lhs_acl_record, output_acl_record],
    );
    let (meta, account_keys) = send_with_meta(&mut svm, &payer, ix);
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

    let encrypted_rhs_steps = vec![FheFrameStep::Operation {
        opcode: FheOpcode::Add,
        operands: vec![
            FheOperand::AclRecord {
                handle: lhs,
                acl_record: lhs_acl_record,
            },
            FheOperand::AclRecord {
                handle: [9; 32],
                acl_record: dummy_rhs_account.pubkey(),
            },
        ],
        scalar_byte: 0,
        output_fhe_type: 5,
    }];
    let encrypted_rhs_ix = execute_frame_ix(
        program_id,
        payer.pubkey(),
        encrypted_rhs_steps,
        vec![],
        vec![],
        vec![lhs_acl_record, dummy_rhs_account.pubkey()],
    );
    assert!(try_send(&mut svm, &payer, encrypted_rhs_ix).is_err());
}

#[test]
fn execute_frame_does_not_create_durable_acl_without_allow_step() {
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
    let setup_output = acl_record_address(program_id, nonce_key, 1);
    let output_acl_record = acl_record_address(program_id, nonce_key, 2);
    send(
        &mut svm,
        &payer,
        execute_frame_ix(
            program_id,
            payer.pubkey(),
            vec![FheFrameStep::Operation {
                opcode: FheOpcode::Add,
                operands: vec![
                    FheOperand::AclRecord {
                        handle: lhs,
                        acl_record: lhs_acl_record,
                    },
                    FheOperand::Scalar {
                        value: amount_plaintext(5),
                        fhe_type: 5,
                    },
                ],
                scalar_byte: 1,
                output_fhe_type: 5,
            }],
            vec![FheFrameAction::Allow {
                source: FheOperand::PreviousResult { index: 0 },
                output_acl_record: setup_output,
                nonce_key,
                nonce_sequence: 1,
                acl_domain_key,
                app_account,
                encrypted_value_label,
                subjects: vec![AclSubjectEntry {
                    pubkey: payer.pubkey(),
                }],
                public_decrypt: false,
            }],
            vec![app_account],
            vec![lhs_acl_record, setup_output],
        ),
    );

    let ix = execute_frame_ix(
        program_id,
        payer.pubkey(),
        vec![FheFrameStep::Operation {
            opcode: FheOpcode::Add,
            operands: vec![
                FheOperand::AclRecord {
                    handle: lhs,
                    acl_record: lhs_acl_record,
                },
                FheOperand::AclRecord {
                    handle: [9; 32],
                    acl_record: dummy_rhs_account.pubkey(),
                },
            ],
            scalar_byte: 0,
            output_fhe_type: 5,
        }],
        vec![],
        vec![],
        vec![lhs_acl_record, dummy_rhs_account.pubkey()],
    );
    assert!(try_send(&mut svm, &payer, ix).is_err());
    assert!(read_acl_record(&svm, output_acl_record).is_none());
}

#[test]
fn execute_frame_allows_previous_result_to_feed_later_steps() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("frame-balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_acl_record = acl_record_address(program_id, nonce_key, 1);

    let amount = amount_plaintext(5);
    let delta = amount_plaintext(7);
    let steps = vec![
        FheFrameStep::TrivialEncrypt {
            plaintext: amount,
            fhe_type: 5,
        },
        FheFrameStep::Operation {
            opcode: FheOpcode::Add,
            operands: vec![
                FheOperand::PreviousResult { index: 0 },
                FheOperand::Scalar {
                    value: delta,
                    fhe_type: 5,
                },
            ],
            scalar_byte: 1,
            output_fhe_type: 5,
        },
    ];
    let actions = vec![FheFrameAction::Allow {
        source: FheOperand::PreviousResult { index: 1 },
        output_acl_record,
        nonce_key,
        nonce_sequence: 1,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        subjects: vec![AclSubjectEntry {
            pubkey: payer.pubkey(),
        }],
        public_decrypt: false,
    }];
    let ix = execute_frame_ix(
        program_id,
        payer.pubkey(),
        steps,
        actions,
        vec![app_account],
        vec![output_acl_record],
    );

    let mut cleartext = CleartextBackend::default();
    let (meta, account_keys) = send_with_meta(&mut svm, &payer, ix);
    cleartext
        .ingest_transaction(&meta, &account_keys, program_id)
        .unwrap();

    let output_record = read_acl_record(&svm, output_acl_record).expect("expected output ACL");
    assert_eq!(record_subjects(&output_record), vec![payer.pubkey()]);
    assert_eq!(
        cleartext.decrypt_cleartext(output_record.handle),
        Some(TypedClearValue::uint64(12))
    );
}

#[test]
fn execute_frame_transient_result_can_authorize_allow_for_decryption() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("frame-public");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_acl_record = acl_record_address(program_id, nonce_key, 1);

    let steps = vec![FheFrameStep::TrivialEncrypt {
        plaintext: amount_plaintext(99),
        fhe_type: 5,
    }];
    let actions = vec![
        FheFrameAction::Allow {
            source: FheOperand::PreviousResult { index: 0 },
            output_acl_record,
            nonce_key,
            nonce_sequence: 1,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            subjects: vec![AclSubjectEntry {
                pubkey: payer.pubkey(),
            }],
            public_decrypt: false,
        },
        FheFrameAction::AllowForDecryption {
            source: FheOperand::PreviousResult { index: 0 },
            acl_record: output_acl_record,
        },
    ];
    let ix = execute_frame_ix(
        program_id,
        payer.pubkey(),
        steps,
        actions,
        vec![app_account],
        vec![output_acl_record],
    );

    send(&mut svm, &payer, ix);

    let output_record = read_acl_record(&svm, output_acl_record).expect("expected output ACL");
    assert!(output_record.public_decrypt);
}

#[test]
fn execute_frame_rejects_unsupported_opcode_without_side_effects() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();

    let steps = vec![FheFrameStep::Operation {
        opcode: FheOpcode::Mul,
        operands: vec![],
        scalar_byte: 0,
        output_fhe_type: 5,
    }];
    let ix = execute_frame_ix(program_id, payer.pubkey(), steps, vec![], vec![], vec![]);

    assert!(try_send(&mut svm, &payer, ix).is_err());
}

#[test]
fn execute_frame_allow_creates_distinct_acl_records_per_nonce_sequence() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();

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
    let rhs_scalar = amount_plaintext(5);
    let first_output = acl_record_address(program_id, nonce_key, 1);
    let second_output = acl_record_address(program_id, nonce_key, 2);
    let build_ix = |output_acl_record, output_nonce_sequence| {
        let steps = vec![FheFrameStep::Operation {
            opcode: FheOpcode::Add,
            operands: vec![
                FheOperand::AclRecord {
                    handle: lhs,
                    acl_record: lhs_acl_record,
                },
                FheOperand::Scalar {
                    value: rhs_scalar,
                    fhe_type: 5,
                },
            ],
            scalar_byte: 1,
            output_fhe_type: 5,
        }];
        let actions = vec![FheFrameAction::Allow {
            source: FheOperand::PreviousResult { index: 0 },
            output_acl_record,
            nonce_key,
            nonce_sequence: output_nonce_sequence,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            subjects: vec![AclSubjectEntry {
                pubkey: payer.pubkey(),
            }],
            public_decrypt: false,
        }];
        execute_frame_ix(
            program_id,
            payer.pubkey(),
            steps,
            actions,
            vec![app_account],
            vec![lhs_acl_record, output_acl_record],
        )
    };

    let instructions = vec![build_ix(first_output, 1), build_ix(second_output, 2)];
    send_many_with_signers(&mut svm, &payer.pubkey(), instructions, &[&payer]).unwrap();

    let first = read_acl_record(&svm, first_output).expect("expected first output ACL");
    let second = read_acl_record(&svm, second_output).expect("expected second output ACL");
    assert_eq!(first.nonce_sequence, 1);
    assert_eq!(second.nonce_sequence, 2);
    assert_ne!(first_output, second_output);
}

#[test]
fn allow_acl_subjects_rejects_wrong_handle() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();

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
            authority: payer.pubkey(),
            acl_record,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle: [7; 32],
            subjects: vec![AclSubjectEntry {
                pubkey: payer.pubkey(),
            }],
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

    let acl_domain_key = Pubkey::new_unique();
    let app_account = Pubkey::new_unique();
    let encrypted_value_label = label("balance");
    let handle = [7; 32];
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
            authority: mallory.pubkey(),
            acl_record,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry {
                pubkey: mallory.pubkey(),
            }],
        },
    );

    assert!(try_send(&mut svm, &mallory, ix).is_err());
    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record_subjects(&record), vec![alice.pubkey()]);
}

#[test]
fn allow_acl_subjects_rejects_when_subject_capacity_exceeded() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();

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

    for _ in 1..host::MAX_ACL_SUBJECTS {
        let new_subject = Pubkey::new_unique();
        send(
            &mut svm,
            &payer,
            anchor_ix(
                program_id,
                host::accounts::AllowAclSubjects {
                    authority: payer.pubkey(),
                    acl_record,
                    event_authority: event_authority(program_id),
                    program: program_id,
                },
                host::instruction::AllowAclSubjects {
                    handle,
                    subjects: vec![AclSubjectEntry {
                        pubkey: new_subject,
                    }],
                },
            ),
        );
    }

    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record_subjects(&record).len(), host::MAX_ACL_SUBJECTS);

    let overflow_ix = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            authority: payer.pubkey(),
            acl_record,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects {
            handle,
            subjects: vec![AclSubjectEntry {
                pubkey: Pubkey::new_unique(),
            }],
        },
    );
    assert!(try_send(&mut svm, &payer, overflow_ix).is_err());
    let record = read_acl_record(&svm, acl_record).expect("expected ACL record");
    assert_eq!(record_subjects(&record).len(), host::MAX_ACL_SUBJECTS);
}

#[test]
fn execute_frame_rejects_missing_remaining_account() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();

    let missing_acl = Pubkey::new_unique();
    let steps = vec![FheFrameStep::Operation {
        opcode: FheOpcode::Add,
        operands: vec![
            FheOperand::AclRecord {
                handle: [7; 32],
                acl_record: missing_acl,
            },
            FheOperand::Scalar {
                value: amount_plaintext(1),
                fhe_type: 5,
            },
        ],
        scalar_byte: 1,
        output_fhe_type: 5,
    }];
    let (result, account_keys) = try_send_with_meta(
        &mut svm,
        &payer,
        execute_frame_ix(program_id, payer.pubkey(), steps, vec![], vec![], vec![]),
    );
    assert_no_zama_host_events_on_failure(result, &account_keys, program_id);
}

#[test]
fn execute_frame_emits_no_events_when_operand_acl_rejects() {
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
    let steps = vec![FheFrameStep::Operation {
        opcode: FheOpcode::Add,
        operands: vec![
            FheOperand::AclRecord {
                handle: lhs,
                acl_record: lhs_acl_record,
            },
            FheOperand::AclRecord {
                handle: [9; 32],
                acl_record: dummy_rhs_account.pubkey(),
            },
        ],
        scalar_byte: 0,
        output_fhe_type: 5,
    }];
    let (result, account_keys) = try_send_with_meta(
        &mut svm,
        &payer,
        execute_frame_ix(
            program_id,
            payer.pubkey(),
            steps,
            vec![],
            vec![],
            vec![lhs_acl_record],
        ),
    );
    assert_no_zama_host_events_on_failure(result, &account_keys, program_id);
}

#[test]
fn execute_frame_transient_result_not_available_on_subsequent_instruction() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();

    send(
        &mut svm,
        &payer,
        execute_frame_ix(
            program_id,
            payer.pubkey(),
            vec![FheFrameStep::TrivialEncrypt {
                plaintext: amount_plaintext(3),
                fhe_type: 5,
            }],
            vec![],
            vec![],
            vec![],
        ),
    );

    let follow_up = execute_frame_ix(
        program_id,
        payer.pubkey(),
        vec![FheFrameStep::Operation {
            opcode: FheOpcode::Add,
            operands: vec![
                FheOperand::PreviousResult { index: 0 },
                FheOperand::Scalar {
                    value: amount_plaintext(1),
                    fhe_type: 5,
                },
            ],
            scalar_byte: 1,
            output_fhe_type: 5,
        }],
        vec![],
        vec![],
        vec![],
    );
    let (result, account_keys) = try_send_with_meta(&mut svm, &payer, follow_up);
    assert_no_zama_host_events_on_failure(result, &account_keys, program_id);
}

#[test]
fn transfer_scenario_cleartext_backend() {
    let mut fixture = token_fixture();
    let scenario = assert_transfer_cleartext(
        &mut fixture,
        TransferSetup::default(),
        125,
        20,
        TransferExpect { alice: 116, bob: 29 },
    );
    assert_transfer_output_invariants(&fixture, &scenario);
}

#[test]
fn confidential_transfer_rotates_balance_handles_and_binds_output_acl() {
    let mut fixture = token_fixture();
    let setup = TransferSetup::default();
    let (scenario, _) = run_transfer_scenario_meta(&mut fixture, setup);
    let mut cleartext = CleartextBackend::default();
    seed_transfer_inputs(&mut cleartext, &scenario, 125, 20, setup.amount);
    cleartext
        .ingest_host_transaction(
            &scenario.meta,
            &scenario.account_keys,
            scenario.host_program_id,
        )
        .unwrap();
    let new_alice = scenario.new_alice_handle;
    let new_bob = scenario.new_bob_handle;
    let output = scenario.output;
    let events = binary_op_events(
        &scenario.meta,
        &scenario.account_keys,
        scenario.host_program_id,
    );
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].version, 0);
    assert_eq!(events[0].op, FheBinaryOpCode::Sub);
    assert_eq!(events[0].subject, fixture.compute_signer.to_bytes());
    assert_eq!(events[0].lhs, fixture.alice_initial);
    assert_eq!(events[0].rhs, scenario.amount_handle);
    assert!(!events[0].scalar);
    assert_eq!(events[0].result, new_alice);
    assert_eq!(events[1].version, 0);
    assert_eq!(events[1].op, FheBinaryOpCode::Add);
    assert_eq!(events[1].subject, fixture.compute_signer.to_bytes());
    assert_eq!(events[1].lhs, fixture.bob_initial);
    assert_eq!(events[1].rhs, scenario.amount_handle);
    assert!(!events[1].scalar);
    assert_eq!(events[1].result, new_bob);
    let balance_events =
        balance_handle_updated_events(&scenario.meta, &scenario.account_keys, fixture.token_program_id);
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
    let amount_handle = authorize_transfer_amount(&mut fixture, 9, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = transfer_output_accounts(&fixture, 2);
    let ix = self_transfer_ix(&fixture, output, amount_handle);

    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, ix);

    assert!(binary_op_events(&meta, &account_keys, fixture.host_program_id).is_empty());
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
}

#[test]
fn user_decrypt_model_uses_acl_domain_key_and_acl_record_authentication() {
    let mut fixture = token_fixture();
    let amount_handle = authorize_transfer_amount(&mut fixture, 9, DEFAULT_INPUT_NONCE_SEQUENCE);
    let first_output = transfer_output_accounts(&fixture, 1);
    let first_ix = transfer_ix(&fixture, first_output, amount_handle);
    send(&mut fixture.svm, &fixture.alice, first_ix);
    let first_alice = read_acl_record(&fixture.svm, first_output.alice)
        .expect("expected first Alice ACL")
        .handle;

    let second_amount_handle = authorize_transfer_amount(&mut fixture, 8, 1);
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
    let amount_handle = authorize_transfer_amount(&mut fixture, 9, DEFAULT_INPUT_NONCE_SEQUENCE);
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
    let amount_handle = authorize_transfer_amount(&mut fixture, 9, DEFAULT_INPUT_NONCE_SEQUENCE);
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
fn fhe_execute_wrapper_initialize_creates_balance_acl_via_execute_frame() {
    let fixture = token_fixture();
    assert_balance_acl(
        &fixture.svm,
        fixture.alice_current_compute_acl,
        fixture.mint.pubkey(),
        fixture.alice_token,
        0,
        fixture.alice_initial,
        &[fixture.alice.pubkey(), fixture.compute_signer],
    );
}

#[test]
fn fhe_execute_wrapper_wrap_uses_single_execute_frame_cpi() {
    let mut fixture = token_fixture();
    let output = wrap_output_accounts(&fixture, 1);
    let ix = wrap_usdc_ix(&fixture, output, 100);
    let (meta, _) = send_with_meta(&mut fixture.svm, &fixture.alice, ix);
    assert_eq!(execute_frame_log_count(&meta), 1);
}

#[test]
fn fhe_execute_wrapper_transfer_uses_single_execute_frame_cpi() {
    let mut fixture = token_fixture();
    let amount_handle = authorize_transfer_amount(&mut fixture, 9, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = transfer_output_accounts(&fixture, 1);
    let ix = transfer_ix(&fixture, output, amount_handle);
    let (meta, _) = send_with_meta(&mut fixture.svm, &fixture.alice, ix);
    assert_eq!(execute_frame_log_count(&meta), 1);
}

#[test]
fn poc_authorize_transfer_amount_uses_fhe_execute_wrapper() {
    let mut fixture = token_fixture();
    let output_acl = transfer_amount_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE);
    let (meta, account_keys) = send_with_meta(
        &mut fixture.svm,
        &fixture.alice,
        anchor_ix(
            fixture.token_program_id,
            token::accounts::PocAuthorizeTransferAmount {
                owner: fixture.alice.pubkey(),
                mint: fixture.mint.pubkey(),
                token_account: fixture.alice_token,
                compute_signer: fixture.compute_signer,
                output_acl,
                zama_rand_counter: rand_counter_address(fixture.host_program_id),
                zama_event_authority: event_authority(fixture.host_program_id),
                zama_program: fixture.host_program_id,
                system_program: system_program::ID,
                event_authority: event_authority(fixture.token_program_id),
                program: fixture.token_program_id,
            },
            token::instruction::PocAuthorizeTransferAmount {
                amount: 42,
                nonce_sequence: DEFAULT_INPUT_NONCE_SEQUENCE,
            },
        ),
    );
    assert_eq!(execute_frame_log_count(&meta), 1);
    let events = trivial_encrypt_events(&meta, &account_keys, fixture.host_program_id);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].plaintext, amount_plaintext(42));
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
    assert_eq!(trivial_events[0].subject, fixture.compute_signer.to_bytes());
    assert_eq!(trivial_events[0].plaintext, amount_plaintext(amount));

    let amount_handle = trivial_events[0].result;
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
    assert_eq!(events[0].subject, fixture.compute_signer.to_bytes());
    assert_eq!(events[0].lhs, fixture.alice_initial);
    assert_eq!(events[0].rhs, amount_handle);
    assert_eq!(events[0].result, new_alice);
    let balance_events =
        balance_handle_updated_events(&meta, &account_keys, fixture.token_program_id);
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
    let transfer_amount = 100_000_000_u64;
    let transfer_amount_handle =
        authorize_transfer_amount(&mut fixture, transfer_amount, DEFAULT_INPUT_NONCE_SEQUENCE);
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
    let amount_handle = authorize_transfer_amount(&mut fixture, 9, DEFAULT_INPUT_NONCE_SEQUENCE);
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
    let host_events = binary_op_events(&meta, &account_keys, fixture.host_program_id).len();
    let app_events =
        balance_handle_updated_events(&meta, &account_keys, fixture.token_program_id).len();
    let max_cpi_depth = max_cpi_depth(&meta);

    assert_eq!(top_level_metas, 16);
    assert_eq!(writable_metas, 6);
    assert_eq!(signer_metas, 1);
    assert_eq!(host_events, 2);
    assert_eq!(app_events, 2);
    assert_eq!(created_acl_count(&fixture.svm, &[output.alice, output.bob]), 2);
    assert!(
        inner_instructions <= 14,
        "inner instructions: {inner_instructions}"
    );
    assert!(
        meta.compute_units_consumed <= 160_000,
        "compute units: {}",
        meta.compute_units_consumed
    );
    assert_eq!(max_cpi_depth, 3);
}

#[test]
fn confidential_transfer_rejects_stale_current_acl() {
    let mut fixture = token_fixture();
    let first_amount = authorize_transfer_amount(&mut fixture, 9, DEFAULT_INPUT_NONCE_SEQUENCE);
    let first_output = transfer_output_accounts(&fixture, 1);
    let first_ix = transfer_ix(&fixture, first_output, first_amount);
    send(&mut fixture.svm, &fixture.alice, first_ix);

    let stale_amount = authorize_transfer_amount(&mut fixture, 8, 1);
    let stale_ix = transfer_ix_with_amount_nonce(
        &fixture,
        transfer_output_accounts(&fixture, 2),
        stale_amount,
        1,
    );
    assert!(try_send(&mut fixture.svm, &fixture.alice, stale_ix).is_err());
}

#[test]
fn confidential_transfer_rejects_wrong_current_acl_record() {
    let mut fixture = token_fixture();
    let amount_handle = authorize_transfer_amount(&mut fixture, 9, DEFAULT_INPUT_NONCE_SEQUENCE);
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
fn confidential_transfer_rejects_wrong_amount_acl() {
    let mut fixture = token_fixture();
    let _wrong_amount_handle =
        authorize_transfer_amount(&mut fixture, 8, DEFAULT_INPUT_NONCE_SEQUENCE);
    let amount_handle = expected_trivial_handle(&fixture.svm, 9, 5);

    let output = transfer_output_accounts(&fixture, 1);
    let ix = transfer_ix_with_amount_acl(
        &fixture,
        fixture.alice_current_compute_acl,
        fixture.bob_current_compute_acl,
        transfer_amount_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE),
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
    assert_eq!(created_acl_count(&fixture.svm, &[output.alice, output.bob]), 0);
}

#[test]
fn confidential_transfer_rejects_output_acl_for_wrong_token_account() {
    let mut fixture = token_fixture();
    let amount_handle = authorize_transfer_amount(&mut fixture, 9, DEFAULT_INPUT_NONCE_SEQUENCE);
    let correct_output = transfer_output_accounts(&fixture, 1);
    let swapped_output = TransferOutputAccounts {
        alice: correct_output.bob,
        bob: correct_output.alice,
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
    assert_eq!(
        created_acl_count(&fixture.svm, &[correct_output.alice, correct_output.bob]),
        0
    );
}

#[test]
fn confidential_transfer_rejects_reused_output_acl_record() {
    let mut fixture = token_fixture();
    let amount_handle = authorize_transfer_amount(&mut fixture, 9, DEFAULT_INPUT_NONCE_SEQUENCE);
    let output = TransferOutputAccounts {
        alice: fixture.alice_current_compute_acl,
        bob: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.bob_token,
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
fn wrap_output_invariants_hold_across_event_acl_and_token_account() {
    let mut fixture = token_fixture();
    let scenario = run_wrap_scenario(&mut fixture, WrapSetup::default());
    assert_wrap_output_invariants(
        &fixture,
        &scenario.meta,
        &scenario.account_keys,
        scenario.output.balance,
    );
}

#[test]
fn host_handle_derivation_matches_fixture_bank_hash_for_trivial_encrypt() {
    use solana_sdk::clock::Clock;
    use zama_host as host;

    let fixture = token_fixture();
    let clock: Clock = fixture.svm.get_sysvar();
    let bank_hash = previous_bank_hash_from_sysvar(&fixture.svm, clock.slot);
    assert_ne!(bank_hash, [0; 32], "fixture must seed non-zero bank hash");

    let amount = 42_u64;
    let expected = host::computed_trivial_handle(
        amount_plaintext(amount),
        5,
        host::SOLANA_POC_CHAIN_ID,
        bank_hash,
        clock.unix_timestamp,
    );

    let mut fixture = token_fixture();
    let amount_handle =
        authorize_transfer_amount(&mut fixture, amount, DEFAULT_INPUT_NONCE_SEQUENCE);
    assert_eq!(amount_handle, expected);
}

#[test]
fn trivial_encrypt_uses_slot_hashes_sysvar_when_present() {
    use solana_sdk::clock::Clock;
    use zama_host as host;

    let mut fixture = token_fixture();
    let bank_hash = [0xAB_u8; 32];
    set_previous_slot_hash(&mut fixture.svm, bank_hash);

    let amount_handle =
        authorize_transfer_amount(&mut fixture, 42, DEFAULT_INPUT_NONCE_SEQUENCE);
    let acl = read_acl_record(
        &fixture.svm,
        transfer_amount_acl_address(&fixture, DEFAULT_INPUT_NONCE_SEQUENCE),
    )
    .expect("amount ACL");
    assert_eq!(amount_handle, acl.handle);

    let clock: Clock = fixture.svm.get_sysvar();
    let expected = host::computed_trivial_handle(
        amount_plaintext(42),
        5,
        host::SOLANA_POC_CHAIN_ID,
        bank_hash,
        clock.unix_timestamp,
    );
    assert_eq!(amount_handle, expected);

    let fallback = host::computed_trivial_handle(
        amount_plaintext(42),
        5,
        host::SOLANA_POC_CHAIN_ID,
        [0; 32],
        clock.unix_timestamp,
    );
    assert_ne!(amount_handle, fallback);
}

#[test]
fn execute_frame_fails_when_previous_bank_hash_unavailable() {
    use solana_sdk::slot_hashes::SlotHashes;

    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    svm.set_sysvar(&SlotHashes::new(&[]));
    let payer = svm.create_funded_account(1_000_000_000).unwrap();

    let acl_domain_key = Pubkey::new_unique();
    let app_account = payer.pubkey();
    let encrypted_value_label = label("balance");
    let nonce_key = token::nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_acl_record = acl_record_address(program_id, nonce_key, 1);
    let steps = vec![FheFrameStep::TrivialEncrypt {
        plaintext: amount_plaintext(7),
        fhe_type: 5,
    }];
    let actions = vec![FheFrameAction::Allow {
        source: FheOperand::PreviousResult { index: 0 },
        output_acl_record,
        nonce_key,
        nonce_sequence: 1,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        subjects: vec![AclSubjectEntry {
            pubkey: payer.pubkey(),
        }],
        public_decrypt: false,
    }];
    let ix = execute_frame_ix(
        program_id,
        payer.pubkey(),
        steps,
        actions,
        vec![app_account],
        vec![output_acl_record],
    );

    assert!(try_send(&mut svm, &payer, ix).is_err());
}

#[test]
fn execute_frame_rand_emits_event_and_distinct_handle() {
    use solana_sdk::clock::Clock;

    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    set_previous_slot_hash(&mut svm, DEFAULT_TEST_PREVIOUS_BANK_HASH);

    let steps = vec![FheFrameStep::Rand { fhe_type: 5 }];
    let ix = execute_frame_ix(program_id, payer.pubkey(), steps, vec![], vec![], vec![]);
    let (meta, account_keys) = send_with_meta(&mut svm, &payer, ix);

    let events = fhe_rand_events(&meta, &account_keys, program_id);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].fhe_type, 5);
    assert_eq!(events[0].subject, payer.pubkey().to_bytes());

    let clock: Clock = svm.get_sysvar();
    let bank_hash = previous_bank_hash_from_sysvar(&svm, clock.slot);
    let expected_seed = host::computed_rand_seed(
        0,
        bank_hash,
        host::SOLANA_POC_CHAIN_ID,
        clock.unix_timestamp,
    );
    let expected_handle =
        host::computed_rand_handle(5, expected_seed, host::SOLANA_POC_CHAIN_ID);
    assert_eq!(events[0].seed, expected_seed);
    assert_eq!(events[0].result, expected_handle);

    let mut cleartext = CleartextBackend::default();
    cleartext
        .ingest_transaction(&meta, &account_keys, program_id)
        .unwrap();
    assert_eq!(
        cleartext.decrypt_cleartext(expected_handle),
        Some(TypedClearValue::uint64(
            u64::try_from(cleartext_rand_value(expected_seed, 64)).unwrap()
        ))
    );
    assert_eq!(read_rand_counter(&svm, program_id), Some(1));
}

#[test]
fn execute_frame_rejects_unsupported_rand_fhe_type_without_side_effects() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    set_previous_slot_hash(&mut svm, DEFAULT_TEST_PREVIOUS_BANK_HASH);

    let steps = vec![FheFrameStep::Rand { fhe_type: 99 }];
    let ix = execute_frame_ix(program_id, payer.pubkey(), steps, vec![], vec![], vec![]);

    assert!(try_send(&mut svm, &payer, ix).is_err());
    assert_eq!(read_rand_counter(&svm, program_id), None);
}

#[test]
fn execute_frame_two_rand_steps_in_one_frame_use_different_handles() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    set_previous_slot_hash(&mut svm, DEFAULT_TEST_PREVIOUS_BANK_HASH);

    let steps = vec![
        FheFrameStep::Rand { fhe_type: 5 },
        FheFrameStep::Rand { fhe_type: 5 },
    ];
    let ix = execute_frame_ix(program_id, payer.pubkey(), steps, vec![], vec![], vec![]);
    let (meta, account_keys) = send_with_meta(&mut svm, &payer, ix);

    let events = fhe_rand_events(&meta, &account_keys, program_id);
    assert_eq!(events.len(), 2);
    assert_ne!(events[0].seed, events[1].seed);
    assert_ne!(events[0].result, events[1].result);
    assert_eq!(read_rand_counter(&svm, program_id), Some(2));
}

#[test]
fn execute_frame_rand_counter_bumps_across_instructions_in_one_transaction() {
    let program_id = host::id();
    let mut svm = svm_with_program(program_id, host_program_so_path());
    let payer = svm.create_funded_account(1_000_000_000).unwrap();
    set_previous_slot_hash(&mut svm, DEFAULT_TEST_PREVIOUS_BANK_HASH);

    let build_ix = || {
        execute_frame_ix(
            program_id,
            payer.pubkey(),
            vec![FheFrameStep::Rand { fhe_type: 5 }],
            vec![],
            vec![],
            vec![],
        )
    };

    send_many_with_signers(
        &mut svm,
        &payer.pubkey(),
        vec![build_ix(), build_ix()],
        &[&payer],
    )
    .unwrap();

    assert_eq!(read_rand_counter(&svm, program_id), Some(2));
}

#[test]
fn confidential_token_e2e_rand_demo_encrypt_compute_and_user_decrypt_request() {
    let mut fixture = token_fixture();
    const RAND_NONCE_SEQUENCE: u64 = 1;
    let scenario = run_rand_demo_scenario(&mut fixture, RAND_NONCE_SEQUENCE);

    assert_eq!(
        scenario.rand_handle,
        read_acl_record(&fixture.svm, scenario.acl_record)
            .expect("expected rand ACL")
            .handle
    );
    assert_acl_record(
        &fixture.svm,
        scenario.acl_record,
        fixture.mint.pubkey(),
        fixture.alice_token,
        token::rand_label(),
        RAND_NONCE_SEQUENCE,
        scenario.rand_handle,
        &[fixture.alice.pubkey(), fixture.compute_signer],
    );

    let mut cleartext = CleartextBackend::default();
    cleartext
        .ingest_transaction(
            &scenario.meta,
            &scenario.account_keys,
            fixture.host_program_id,
        )
        .unwrap();
    let expected_plaintext =
        u64::try_from(cleartext_rand_value(scenario.rand_seed, 64)).unwrap();
    assert_eq!(
        cleartext.decrypt_cleartext(scenario.rand_handle),
        Some(TypedClearValue::uint64(expected_plaintext))
    );

    let decrypt_request = signed_confidential_rand_user_decrypt_request(
        &fixture,
        &fixture.alice,
        scenario.rand_handle,
        scenario.acl_record,
    );
    assert_eq!(decrypt_request.handles.len(), 1);
    assert_eq!(decrypt_request.handles[0].handle, scenario.rand_handle);
    assert_eq!(decrypt_request.handles[0].acl_record, scenario.acl_record);
    assert_eq!(decrypt_request.authorization.user, fixture.alice.pubkey());
    assert_eq!(
        decrypt_request.authorization.allowed_acl_domain_keys,
        vec![fixture.mint.pubkey()]
    );
    assert!(kms_like_user_decrypt_check(&fixture.svm, &decrypt_request));

    let wrong_owner = signed_confidential_rand_user_decrypt_request(
        &fixture,
        &fixture.bob,
        scenario.rand_handle,
        scenario.acl_record,
    );
    assert!(!kms_like_user_decrypt_check(&fixture.svm, &wrong_owner));

    let record = read_acl_record(&fixture.svm, scenario.acl_record).expect("expected rand ACL");
    assert_eq!(record.encrypted_value_label, token::rand_label());
}
