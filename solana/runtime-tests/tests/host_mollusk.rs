use anchor_lang::{
    prelude::system_program, AccountDeserialize, AccountSerialize, InstructionData, ToAccountMetas,
};
use mollusk_svm::{result::Check, Mollusk};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};
use std::{collections::HashMap, path::PathBuf};
use zama_host::{
    self as host, AclRecord, AclSubjectEntry, FheBinaryOpCode, FheEvalArgs, FheEvalOp,
    FheEvalOperand, FheEvalOutput, HostConfig, TransientCapabilityGrant, TransientSession,
};

#[test]
fn mollusk_assert_acl_record_accepts_canonical_record() {
    let program_id = host::id();
    let acl_domain_key = Pubkey::new_unique();
    let app_account = Pubkey::new_unique();
    let encrypted_value_label = *b"mollusk_balance_________________";
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let nonce_sequence = 7;
    let handle = [9_u8; 32];
    let subject = Pubkey::new_unique();
    let (acl_record, bump) = host::acl_record_address(nonce_key, nonce_sequence);

    let mut subjects = [Pubkey::default(); host::MAX_ACL_SUBJECTS];
    let mut subject_roles = [0_u8; host::MAX_ACL_SUBJECTS];
    subjects[0] = subject;
    subject_roles[0] = host::ACL_ROLE_ALL;

    let ix = anchor_ix(
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
            subject,
        },
    );
    let accounts = vec![(
        acl_record,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(AclRecord {
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
                bump,
            }),
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    )];

    mollusk().process_and_validate_instruction(&ix, &accounts, &[Check::success()]);
}

#[test]
fn mollusk_transient_session_consumes_capability_once() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("balance");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let (acl_record, acl_account) = authorizing_acl_record_account(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
    );
    let session_nonce = label("mollusk-session-1");
    let (session, _) = host::transient_session_address(authority, session_nonce);
    let context = transient_context(
        authority,
        vec![
            host_config_account(authority),
            (acl_record, acl_account),
            (session, system_account(0)),
        ],
    );
    let capability =
        transient_capability(authority, program_id, acl_domain_key, app_account, false);

    let create_ix = create_transient_session_ix(
        program_id,
        authority,
        host_config,
        session,
        session_nonce,
        authority,
        0,
        2,
    );
    let allow_ix = allow_transient_handle_ix(
        program_id,
        authority,
        acl_record,
        session,
        host_config,
        handle,
        capability,
    );
    let seal_ix = seal_transient_session_ix(program_id, authority, host_config, session);
    context.process_and_validate_instruction(&create_ix, &[Check::success()]);
    context.process_and_validate_instruction(&allow_ix, &[Check::success()]);
    let session_account = read_transient_session(&context, session).expect("expected session");
    assert_eq!(session_account.entries.len(), 1);
    assert_eq!(session_account.entries[0].handle, handle);
    assert_eq!(session_account.entries[0].used_count, 0);
    context.process_and_validate_instruction(&seal_ix, &[Check::success()]);

    let context_id = label("consume-once");
    let rhs = amount_plaintext(5);
    let result = current_eval_handle(
        &context.mollusk,
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
        authority,
        host_config,
        session,
        context_id,
        handle,
        rhs,
        result,
        FheEvalOutput::Transient,
    );
    context.process_and_validate_instruction(&consume_ix, &[Check::success()]);

    let session_account = read_transient_session(&context, session).expect("expected session");
    assert_eq!(session_account.state, host::TRANSIENT_SESSION_STATE_SEALED);
    assert_eq!(session_account.entries.len(), 1);
    assert_eq!(session_account.entries[0].used_count, 1);

    let second_context = label("consume-twice");
    let second_result = current_eval_handle(
        &context.mollusk,
        FheBinaryOpCode::Add,
        handle,
        rhs,
        true,
        5,
        second_context,
        0,
    );
    let second_consume_ix = session_consume_eval_ix(
        program_id,
        authority,
        host_config,
        session,
        second_context,
        handle,
        rhs,
        second_result,
        FheEvalOutput::Transient,
    );
    let result = context.process_instruction(&second_consume_ix);
    assert!(result.raw_result.is_err());

    let session_account = read_transient_session(&context, session).expect("expected session");
    assert_eq!(session_account.entries[0].used_count, 1);
}

#[test]
fn mollusk_transient_session_denies_durable_output_without_policy() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("balance");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let (acl_record, acl_account) = authorizing_acl_record_account(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
    );
    let session_nonce = label("mollusk-session-2");
    let (session, _) = host::transient_session_address(authority, session_nonce);
    let output_acl_record = host::acl_record_address(nonce_key, 1).0;
    let context = transient_context(
        authority,
        vec![
            host_config_account(authority),
            (acl_record, acl_account),
            (session, system_account(0)),
            (output_acl_record, system_account(0)),
        ],
    );

    let create_ix = create_transient_session_ix(
        program_id,
        authority,
        host_config,
        session,
        session_nonce,
        authority,
        0,
        2,
    );
    let allow_ix = allow_transient_handle_ix(
        program_id,
        authority,
        acl_record,
        session,
        host_config,
        handle,
        transient_capability(authority, program_id, acl_domain_key, app_account, false),
    );
    let seal_ix = seal_transient_session_ix(program_id, authority, host_config, session);
    context.process_and_validate_instruction(&create_ix, &[Check::success()]);
    context.process_and_validate_instruction(&allow_ix, &[Check::success()]);
    context.process_and_validate_instruction(&seal_ix, &[Check::success()]);

    let context_id = label("durable-denied");
    let rhs = amount_plaintext(5);
    let result_handle = current_bound_eval_handle(
        &context.mollusk,
        FheBinaryOpCode::Add,
        handle,
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
        authority,
        host_config,
        session,
        context_id,
        handle,
        rhs,
        result_handle,
        FheEvalOutput::Durable {
            output_acl_record_index: 1,
            output_nonce_key: nonce_key,
            output_nonce_sequence: 1,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(authority)],
            output_public_decrypt: false,
        },
    );
    durable_ix
        .accounts
        .push(AccountMeta::new(output_acl_record, false));

    let result = context.process_instruction(&durable_ix);
    assert!(result.raw_result.is_err());
    assert!(read_acl_record(&context, output_acl_record).is_none());
    let session_account = read_transient_session(&context, session).expect("expected session");
    assert_eq!(session_account.entries[0].used_count, 0);
}

#[test]
fn mollusk_transient_policy_survives_chained_transient_output() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("balance");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let (acl_record, acl_account) = authorizing_acl_record_account(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
    );
    let session_nonce = label("mollusk-session-4");
    let (session, _) = host::transient_session_address(authority, session_nonce);
    let output_acl_record = host::acl_record_address(nonce_key, 1).0;
    let context = transient_context(
        authority,
        vec![
            host_config_account(authority),
            (acl_record, acl_account),
            (session, system_account(0)),
            (output_acl_record, system_account(0)),
        ],
    );

    let create_ix = create_transient_session_ix(
        program_id,
        authority,
        host_config,
        session,
        session_nonce,
        authority,
        0,
        2,
    );
    let allow_ix = allow_transient_handle_ix(
        program_id,
        authority,
        acl_record,
        session,
        host_config,
        handle,
        transient_capability(authority, program_id, acl_domain_key, app_account, false),
    );
    let seal_ix = seal_transient_session_ix(program_id, authority, host_config, session);
    context.process_and_validate_instruction(&create_ix, &[Check::success()]);
    context.process_and_validate_instruction(&allow_ix, &[Check::success()]);
    context.process_and_validate_instruction(&seal_ix, &[Check::success()]);

    let context_id = label("chain-policy");
    let rhs_first = amount_plaintext(5);
    let transient_handle = current_eval_handle(
        &context.mollusk,
        FheBinaryOpCode::Add,
        handle,
        rhs_first,
        true,
        5,
        context_id,
        0,
    );
    let rhs_second = amount_plaintext(3);
    let final_handle = current_bound_eval_handle(
        &context.mollusk,
        FheBinaryOpCode::Add,
        transient_handle,
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
                ops: vec![
                    FheEvalOp {
                        op: FheBinaryOpCode::Add,
                        lhs: FheEvalOperand::TransientSession {
                            handle,
                            session_index: 0,
                            capability_index: 0,
                        },
                        rhs: FheEvalOperand::Scalar(rhs_first),
                        output_fhe_type: 5,
                        result: transient_handle,
                        output: FheEvalOutput::Transient,
                    },
                    FheEvalOp {
                        op: FheBinaryOpCode::Add,
                        lhs: FheEvalOperand::Transient { producer_index: 0 },
                        rhs: FheEvalOperand::Scalar(rhs_second),
                        output_fhe_type: 5,
                        result: final_handle,
                        output: FheEvalOutput::Durable {
                            output_acl_record_index: 1,
                            output_nonce_key: nonce_key,
                            output_nonce_sequence: 1,
                            output_acl_domain_key: acl_domain_key,
                            output_app_account: app_account,
                            output_encrypted_value_label: encrypted_value_label,
                            output_subjects: vec![AclSubjectEntry::user(authority)],
                            output_public_decrypt: false,
                        },
                    },
                ],
            },
        },
    );
    ix.accounts.push(AccountMeta::new(session, false));
    ix.accounts.push(AccountMeta::new(output_acl_record, false));

    let result = context.process_instruction(&ix);
    assert!(result.raw_result.is_err());
    assert!(read_acl_record(&context, output_acl_record).is_none());
    let session_account = read_transient_session(&context, session).expect("expected session");
    assert_eq!(session_account.entries[0].used_count, 0);
}

#[test]
fn mollusk_transient_policy_blocks_rewrapping_to_broader_session() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("balance");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let (acl_record, acl_account) = authorizing_acl_record_account(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
    );
    let source_nonce = label("mollusk-source");
    let target_nonce = label("mollusk-target");
    let (source_session, _) = host::transient_session_address(authority, source_nonce);
    let (target_session, _) = host::transient_session_address(authority, target_nonce);
    let context = transient_context(
        authority,
        vec![
            host_config_account(authority),
            (acl_record, acl_account),
            (source_session, system_account(0)),
            (target_session, system_account(0)),
        ],
    );

    let create_source_ix = create_transient_session_ix(
        program_id,
        authority,
        host_config,
        source_session,
        source_nonce,
        authority,
        0,
        1,
    );
    let create_target_ix = create_transient_session_ix(
        program_id,
        authority,
        host_config,
        target_session,
        target_nonce,
        authority,
        0,
        1,
    );
    let allow_ix = allow_transient_handle_ix(
        program_id,
        authority,
        acl_record,
        source_session,
        host_config,
        handle,
        transient_capability(authority, program_id, acl_domain_key, app_account, false),
    );
    let seal_source_ix =
        seal_transient_session_ix(program_id, authority, host_config, source_session);
    context.process_and_validate_instruction(&create_source_ix, &[Check::success()]);
    context.process_and_validate_instruction(&create_target_ix, &[Check::success()]);
    context.process_and_validate_instruction(&allow_ix, &[Check::success()]);
    context.process_and_validate_instruction(&seal_source_ix, &[Check::success()]);

    let context_id = label("rewrap-policy");
    let rhs = amount_plaintext(5);
    let derived_handle = current_eval_handle(
        &context.mollusk,
        FheBinaryOpCode::Add,
        handle,
        rhs,
        true,
        5,
        context_id,
        0,
    );
    let mut ix = session_consume_eval_ix(
        program_id,
        authority,
        host_config,
        source_session,
        context_id,
        handle,
        rhs,
        derived_handle,
        FheEvalOutput::TransientSession {
            session_index: 1,
            capability: transient_capability(
                authority,
                Pubkey::new_unique(),
                acl_domain_key,
                app_account,
                true,
            ),
        },
    );
    ix.accounts.push(AccountMeta::new(target_session, false));

    let result = context.process_instruction(&ix);
    assert!(result.raw_result.is_err());
    let source = read_transient_session(&context, source_session).expect("expected source session");
    let target = read_transient_session(&context, target_session).expect("expected target session");
    assert_eq!(source.entries[0].used_count, 0);
    assert!(target.entries.is_empty());
}

#[test]
fn mollusk_fhe_eval_rejects_public_decrypt_output_without_input_role() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("balance");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let (acl_record, acl_account) = acl_record_account_with_subject_role(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
        host::ACL_ROLE_USE,
    );
    let output_acl_record = host::acl_record_address(nonce_key, 1).0;
    let context = transient_context(
        authority,
        vec![
            host_config_account(authority),
            (acl_record, acl_account),
            (output_acl_record, system_account(0)),
        ],
    );

    let context_id = label("public-denied");
    let rhs = amount_plaintext(5);
    let result_handle = current_bound_eval_handle(
        &context.mollusk,
        FheBinaryOpCode::Add,
        handle,
        rhs,
        true,
        5,
        context_id,
        0,
        nonce_key,
        1,
    );
    let mut ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: authority,
            compute_subject: authority,
            app_account_authority: authority,
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
                        handle,
                        acl_record_index: 0,
                        permission_index: None,
                    },
                    rhs: FheEvalOperand::Scalar(rhs),
                    output_fhe_type: 5,
                    result: result_handle,
                    output: FheEvalOutput::Durable {
                        output_acl_record_index: 1,
                        output_nonce_key: nonce_key,
                        output_nonce_sequence: 1,
                        output_acl_domain_key: acl_domain_key,
                        output_app_account: app_account,
                        output_encrypted_value_label: encrypted_value_label,
                        output_subjects: vec![AclSubjectEntry::user(authority)],
                        output_public_decrypt: true,
                    },
                }],
            },
        },
    );
    ix.accounts
        .push(AccountMeta::new_readonly(acl_record, false));
    ix.accounts.push(AccountMeta::new(output_acl_record, false));

    let result = context.process_instruction(&ix);
    assert!(result.raw_result.is_err());
    assert!(read_acl_record(&context, output_acl_record).is_none());
}

#[test]
fn mollusk_binary_op_bind_rejects_public_decrypt_output_without_input_role() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("balance");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = [7; 32];
    let (acl_record, acl_account) = acl_record_account_with_subject_role(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
        host::ACL_ROLE_USE,
    );
    let output_acl_record = host::acl_record_address(nonce_key, 1).0;
    let context = transient_context(
        authority,
        vec![
            host_config_account(authority),
            (acl_record, acl_account),
            (output_acl_record, system_account(0)),
        ],
    );
    let rhs = amount_plaintext(5);
    let result_handle = current_bound_binary_handle(
        &context.mollusk,
        FheBinaryOpCode::Add,
        handle,
        rhs,
        true,
        5,
        nonce_key,
        1,
    );
    let ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOpAndBindOutput {
            payer: authority,
            compute_subject: authority,
            app_account_authority: authority,
            host_config,
            lhs_acl_record: acl_record,
            lhs_permission_record: None,
            rhs_acl_record: Pubkey::new_unique(),
            rhs_permission_record: None,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOpAndBindOutput {
            op: FheBinaryOpCode::Add,
            lhs: handle,
            rhs,
            scalar: true,
            output_fhe_type: 5,
            result: result_handle,
            output_nonce_key: nonce_key,
            output_nonce_sequence: 1,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(authority)],
            output_public_decrypt: true,
        },
    );

    let result = context.process_instruction(&ix);
    assert!(result.raw_result.is_err());
    assert!(read_acl_record(&context, output_acl_record).is_none());
}

#[test]
fn mollusk_fhe_eval_appends_transient_session_output_for_later_consumption() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("balance");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let durable_handle = [7; 32];
    let (acl_record, acl_account) = authorizing_acl_record_account(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        durable_handle,
        authority,
    );
    let session_nonce = label("mollusk-session-3");
    let (session, _) = host::transient_session_address(authority, session_nonce);
    let context = transient_context(
        authority,
        vec![
            host_config_account(authority),
            (acl_record, acl_account),
            (session, system_account(0)),
        ],
    );
    let create_ix = create_transient_session_ix(
        program_id,
        authority,
        host_config,
        session,
        session_nonce,
        authority,
        0,
        2,
    );
    context.process_and_validate_instruction(&create_ix, &[Check::success()]);

    let first_context = label("append-session");
    let rhs_first = amount_plaintext(5);
    let session_handle = current_eval_handle(
        &context.mollusk,
        FheBinaryOpCode::Add,
        durable_handle,
        rhs_first,
        true,
        5,
        first_context,
        0,
    );
    let mut append_ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: authority,
            compute_subject: authority,
            app_account_authority: authority,
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
                        handle: durable_handle,
                        acl_record_index: 0,
                        permission_index: None,
                    },
                    rhs: FheEvalOperand::Scalar(rhs_first),
                    output_fhe_type: 5,
                    result: session_handle,
                    output: FheEvalOutput::TransientSession {
                        session_index: 1,
                        capability: transient_capability(
                            authority,
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
        .push(AccountMeta::new_readonly(acl_record, false));
    append_ix.accounts.push(AccountMeta::new(session, false));
    context.process_and_validate_instruction(&append_ix, &[Check::success()]);

    let session_account = read_transient_session(&context, session).expect("expected session");
    assert_eq!(session_account.entries.len(), 1);
    assert_eq!(session_account.entries[0].handle, session_handle);
    assert_eq!(session_account.entries[0].used_count, 0);

    let seal_ix = seal_transient_session_ix(program_id, authority, host_config, session);
    context.process_and_validate_instruction(&seal_ix, &[Check::success()]);

    let second_context = label("consume-appended");
    let rhs_second = amount_plaintext(3);
    let final_result = current_eval_handle(
        &context.mollusk,
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
        authority,
        host_config,
        session,
        second_context,
        session_handle,
        rhs_second,
        final_result,
        FheEvalOutput::Transient,
    );
    context.process_and_validate_instruction(&consume_ix, &[Check::success()]);

    let session_account = read_transient_session(&context, session).expect("expected session");
    assert_eq!(session_account.entries[0].used_count, 1);
}

fn serialized_account<T: AccountSerialize>(account: T) -> Vec<u8> {
    let mut data = Vec::new();
    account.try_serialize(&mut data).unwrap();
    data
}

fn mollusk() -> Mollusk {
    let deploy_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy");
    unsafe {
        std::env::set_var("SBF_OUT_DIR", deploy_dir);
    }
    Mollusk::new(&host::id(), "zama_host")
}

fn transient_context(
    payer: Pubkey,
    seeded_accounts: Vec<(Pubkey, Account)>,
) -> mollusk_svm::MolluskContext<HashMap<Pubkey, Account>> {
    let mut accounts = HashMap::from([(payer, system_account(5_000_000_000))]);
    for (pubkey, account) in seeded_accounts {
        accounts.insert(pubkey, account);
    }
    mollusk().with_context(accounts)
}

fn anchor_ix<A, D>(program_id: Pubkey, accounts: A, args: D) -> Instruction
where
    A: ToAccountMetas,
    D: InstructionData,
{
    Instruction {
        program_id,
        accounts: accounts.to_account_metas(None),
        data: args.data(),
    }
}

fn system_account(lamports: u64) -> Account {
    Account {
        lamports,
        data: Vec::new(),
        owner: system_program::ID,
        executable: false,
        rent_epoch: 0,
    }
}

fn host_config_account(admin: Pubkey) -> (Pubkey, Account) {
    let (host_config, bump) = host::host_config_address();
    (
        host_config,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(HostConfig {
                admin,
                chain_id: host::SOLANA_POC_CHAIN_ID,
                input_verifier_authority: admin,
                material_authority: admin,
                test_authority: admin,
                paused: false,
                mock_input_enabled: true,
                test_shims_enabled: true,
                grant_deny_list_enabled: false,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn authorizing_acl_record_account(
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    authority: Pubkey,
) -> (Pubkey, Account) {
    acl_record_account_with_subject_role(
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
        host::ACL_ROLE_ALL,
    )
}

#[allow(clippy::too_many_arguments)]
fn acl_record_account_with_subject_role(
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    authority: Pubkey,
    role_flags: u8,
) -> (Pubkey, Account) {
    let (acl_record, bump) = host::acl_record_address(nonce_key, nonce_sequence);
    let mut subjects = [Pubkey::default(); host::MAX_ACL_SUBJECTS];
    let mut subject_roles = [0_u8; host::MAX_ACL_SUBJECTS];
    subjects[0] = authority;
    subject_roles[0] = role_flags;
    (
        acl_record,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(AclRecord {
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
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn create_transient_session_ix(
    program_id: Pubkey,
    authority: Pubkey,
    host_config: Pubkey,
    session: Pubkey,
    session_nonce: [u8; 32],
    refund_recipient: Pubkey,
    expires_slot: u64,
    max_entries: u8,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::CreateTransientSession {
            payer: authority,
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

fn allow_transient_handle_ix(
    program_id: Pubkey,
    authority: Pubkey,
    acl_record: Pubkey,
    session: Pubkey,
    host_config: Pubkey,
    handle: [u8; 32],
    capability: TransientCapabilityGrant,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::AllowTransientHandle {
            authority,
            authority_permission_record: None,
            acl_record,
            session,
            host_config,
            deny_subject_record: None,
        },
        host::instruction::AllowTransientHandle { handle, capability },
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
fn current_eval_handle(
    mollusk: &Mollusk,
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    context_id: [u8; 32],
    op_index: u16,
) -> [u8; 32] {
    host::computed_eval_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash(mollusk),
        mollusk.sysvars.clock.unix_timestamp,
        context_id,
        op_index,
    )
}

#[allow(clippy::too_many_arguments)]
fn current_bound_eval_handle(
    mollusk: &Mollusk,
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
    host::computed_bound_eval_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash(mollusk),
        mollusk.sysvars.clock.unix_timestamp,
        context_id,
        op_index,
        output_nonce_key,
        output_nonce_sequence,
    )
}

#[allow(clippy::too_many_arguments)]
fn current_bound_binary_handle(
    mollusk: &Mollusk,
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    host::computed_bound_binary_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash(mollusk),
        mollusk.sysvars.clock.unix_timestamp,
        output_nonce_key,
        output_nonce_sequence,
    )
}

fn previous_bank_hash(mollusk: &Mollusk) -> [u8; 32] {
    mollusk
        .sysvars
        .clock
        .slot
        .checked_sub(1)
        .and_then(|slot| {
            mollusk
                .sysvars
                .slot_hashes
                .get(&slot)
                .map(|hash| hash.to_bytes())
        })
        .unwrap_or([0; 32])
}

fn read_transient_session(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<TransientSession> {
    let store = context.account_store.borrow();
    let account = store.get(&address)?;
    let mut data = account.data.as_slice();
    TransientSession::try_deserialize(&mut data).ok()
}

fn read_acl_record(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<AclRecord> {
    let store = context.account_store.borrow();
    let account = store.get(&address)?;
    if account.owner != host::id() {
        return None;
    }
    let mut data = account.data.as_slice();
    AclRecord::try_deserialize(&mut data).ok()
}

fn amount_plaintext(amount: u64) -> [u8; 32] {
    let mut plaintext = [0_u8; 32];
    plaintext[24..].copy_from_slice(&amount.to_be_bytes());
    plaintext
}

fn event_authority(program_id: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"__event_authority"], &program_id).0
}

fn label(name: &str) -> [u8; 32] {
    let mut out = [0_u8; 32];
    let bytes = name.as_bytes();
    assert!(bytes.len() <= out.len());
    out[..bytes.len()].copy_from_slice(bytes);
    out
}
