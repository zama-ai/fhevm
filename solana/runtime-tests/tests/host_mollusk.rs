use anchor_lang::{
    prelude::{bpf_loader_upgradeable, system_program},
    AccountDeserialize, AccountSerialize, AnchorDeserialize, Discriminator, InstructionData,
    ToAccountMetas,
};
use mollusk_svm::{
    result::{
        types::{InstructionResult, TransactionResult},
        Check,
    },
    Mollusk,
};
use solana_sdk::{
    account::Account,
    ed25519_program,
    instruction::{AccountMeta, Instruction, InstructionError},
    native_loader,
    pubkey::Pubkey,
    sysvar,
};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};
use zama_host::{
    self as host,
    events::{
        AclAllowedEvent, FheRandBoundedEvent, FheRandEvent, HandleMaterialCommittedEvent,
        HandleMaterialSealedEvent, InputVerifiedEvent, PublicDecryptAllowedEvent,
        TrivialEncryptEvent,
    },
    AclPermission, AclRecord, AclSubjectEntry, CoprocessorInputAttestation, DenySubjectRecord,
    FheBinaryOpCode, FheEvalArgs, FheEvalOperand, FheEvalOutput, FheEvalStep, FheTernaryOpCode,
    HandleMaterialCommitment, HostConfig, UserDecryptionDelegation,
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
                created_slot: 0,
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
fn mollusk_inner_instructions_capture_anchor_event_cpi() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let event_authority = event_authority(program_id);
    let result_handle = handle_for_chain(8, 5);
    let ix = anchor_ix(
        program_id,
        host::accounts::TestEmitProtocolEvent {
            test_authority: authority,
            host_config,
            event_authority,
            program: program_id,
        },
        host::instruction::TestEmitTrivialEncrypt {
            subject: authority,
            plaintext: [7; 32],
            fhe_type: 5,
            result: result_handle,
        },
    );
    let accounts = vec![
        (authority, system_account(1_000_000_000)),
        (host_config, host_config_account),
        (event_authority, system_account(0)),
        (program_id, executable_program_account()),
    ];

    let result = mollusk().process_and_validate_instruction(&ix, &accounts, &[Check::success()]);
    let event_prefix = anchor_event_prefix(TrivialEncryptEvent::DISCRIMINATOR);
    let event_cpi = result
        .inner_instructions
        .iter()
        .find(|inner| inner.instruction.data.starts_with(&event_prefix))
        .expect("expected Anchor event CPI in Mollusk inner instructions");
    let payload = event_cpi
        .instruction
        .data
        .strip_prefix(event_prefix.as_slice())
        .expect("event prefix checked above");
    let event = TrivialEncryptEvent::deserialize(&mut &*payload).expect("event should decode");
    assert_eq!(event.subject, authority.to_bytes());
    assert_eq!(event.plaintext, [7; 32]);
    assert_eq!(event.fhe_type, 5);
    assert_eq!(event.result, result_handle);
}

#[test]
fn mollusk_rand_and_bounded_bind_create_acl_records_and_events() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let rand_label = label("rand-direct");
    let rand_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, rand_label);
    let rand_nonce_sequence = 7;
    let rand_acl = host::acl_record_address(rand_nonce_key, rand_nonce_sequence).0;
    let bounded_label = label("rand-bounded");
    let bounded_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, bounded_label);
    let bounded_nonce_sequence = 8;
    let bounded_acl = host::acl_record_address(bounded_nonce_key, bounded_nonce_sequence).0;
    let context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (rand_acl, system_account(0)),
            (bounded_acl, system_account(0)),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );

    let rand_seed = current_rand_seed(&context.mollusk, rand_nonce_key, rand_nonce_sequence);
    let rand_handle = host::computed_rand_handle(rand_seed, 3, host::SOLANA_POC_CHAIN_ID);
    let rand_result = context.process_instruction(&rand_and_bind_ix(
        program_id,
        authority,
        host_config,
        rand_acl,
        rand_nonce_key,
        rand_nonce_sequence,
        acl_domain_key,
        app_account,
        rand_label,
        3,
    ));
    assert!(rand_result.raw_result.is_ok());

    let rand_record = read_acl_record(&context, rand_acl).expect("expected random ACL record");
    assert_random_acl_record(
        &rand_record,
        rand_handle,
        rand_nonce_key,
        rand_nonce_sequence,
        acl_domain_key,
        app_account,
        rand_label,
        authority,
        context.mollusk.sysvars.clock.slot,
    );
    let rand_events: Vec<FheRandEvent> = rand_result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(rand_events.len(), 1);
    assert_eq!(rand_events[0].version, host::EVENT_VERSION);
    assert_eq!(rand_events[0].subject, authority.to_bytes());
    assert_eq!(rand_events[0].seed, rand_seed);
    assert_eq!(rand_events[0].fhe_type, 3);
    assert_eq!(rand_events[0].result, rand_handle);
    assert_single_acl_allowed_event(&rand_result, rand_handle, authority);

    let upper_bound = amount_plaintext(256);
    let bounded_seed =
        current_rand_seed(&context.mollusk, bounded_nonce_key, bounded_nonce_sequence);
    let bounded_handle =
        host::computed_rand_bounded_handle(upper_bound, bounded_seed, 3, host::SOLANA_POC_CHAIN_ID);
    let bounded_result = context.process_instruction(&rand_bounded_and_bind_ix(
        program_id,
        authority,
        host_config,
        bounded_acl,
        bounded_nonce_key,
        bounded_nonce_sequence,
        acl_domain_key,
        app_account,
        bounded_label,
        upper_bound,
        3,
    ));
    assert!(bounded_result.raw_result.is_ok());

    let bounded_record =
        read_acl_record(&context, bounded_acl).expect("expected bounded random ACL record");
    assert_random_acl_record(
        &bounded_record,
        bounded_handle,
        bounded_nonce_key,
        bounded_nonce_sequence,
        acl_domain_key,
        app_account,
        bounded_label,
        authority,
        context.mollusk.sysvars.clock.slot,
    );
    let bounded_events: Vec<FheRandBoundedEvent> = bounded_result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(bounded_events.len(), 1);
    assert_eq!(bounded_events[0].version, host::EVENT_VERSION);
    assert_eq!(bounded_events[0].subject, authority.to_bytes());
    assert_eq!(bounded_events[0].upper_bound, upper_bound);
    assert_eq!(bounded_events[0].seed, bounded_seed);
    assert_eq!(bounded_events[0].fhe_type, 3);
    assert_eq!(bounded_events[0].result, bounded_handle);
    assert_single_acl_allowed_event(&bounded_result, bounded_handle, authority);
}

#[test]
fn mollusk_rand_bind_rejects_invalid_types_and_bounds_without_acl_birth() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let bad_rand_label = label("bad-rand-type");
    let bad_rand_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, bad_rand_label);
    let bad_rand_acl = host::acl_record_address(bad_rand_nonce_key, 10).0;
    let bad_bounded_type_label = label("bad-bound-type");
    let bad_bounded_type_nonce_key =
        host::acl_nonce_key(acl_domain_key, app_account, bad_bounded_type_label);
    let bad_bounded_type_acl = host::acl_record_address(bad_bounded_type_nonce_key, 11).0;
    let bad_bound_label = label("bad-bound");
    let bad_bound_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, bad_bound_label);
    let bad_bound_acl = host::acl_record_address(bad_bound_nonce_key, 12).0;
    let context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (bad_rand_acl, system_account(0)),
            (bad_bounded_type_acl, system_account(0)),
            (bad_bound_acl, system_account(0)),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );

    let unsupported_rand = context.process_instruction(&rand_and_bind_ix(
        program_id,
        authority,
        host_config,
        bad_rand_acl,
        bad_rand_nonce_key,
        10,
        acl_domain_key,
        app_account,
        bad_rand_label,
        7,
    ));
    assert!(unsupported_rand.raw_result.is_err());
    assert!(read_acl_record(&context, bad_rand_acl).is_none());

    let unsupported_bounded_type = context.process_instruction(&rand_bounded_and_bind_ix(
        program_id,
        authority,
        host_config,
        bad_bounded_type_acl,
        bad_bounded_type_nonce_key,
        11,
        acl_domain_key,
        app_account,
        bad_bounded_type_label,
        amount_plaintext(2),
        0,
    ));
    assert!(unsupported_bounded_type.raw_result.is_err());
    assert!(read_acl_record(&context, bad_bounded_type_acl).is_none());

    let invalid_bound = context.process_instruction(&rand_bounded_and_bind_ix(
        program_id,
        authority,
        host_config,
        bad_bound_acl,
        bad_bound_nonce_key,
        12,
        acl_domain_key,
        app_account,
        bad_bound_label,
        amount_plaintext(3),
        3,
    ));
    assert!(invalid_bound.raw_result.is_err());
    assert!(read_acl_record(&context, bad_bound_acl).is_none());
}

#[test]
fn mollusk_input_trivial_encrypt_bind_nonce_separates_equal_plaintexts_and_events() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("trivial-direct");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let first_acl = host::acl_record_address(nonce_key, 0).0;
    let second_acl = host::acl_record_address(nonce_key, 1).0;
    let context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (first_acl, system_account(0)),
            (second_acl, system_account(0)),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );
    let plaintext = amount_plaintext(7);
    let first_handle = current_trivial_handle(&context.mollusk, plaintext, 5, nonce_key, 0);
    let second_handle = current_trivial_handle(&context.mollusk, plaintext, 5, nonce_key, 1);
    assert_ne!(first_handle, second_handle);

    let first_result = context.process_instruction(&trivial_encrypt_and_bind_ix(
        program_id,
        authority,
        host_config,
        first_acl,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        plaintext,
        5,
        false,
    ));
    assert!(first_result.raw_result.is_ok());
    let first_record = read_acl_record(&context, first_acl).expect("expected first trivial ACL");
    assert_bound_acl_record(
        &first_record,
        first_handle,
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        authority,
        host::ACL_ROLE_USER,
        context.mollusk.sysvars.clock.slot,
    );
    let first_events: Vec<TrivialEncryptEvent> = first_result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(first_events.len(), 1);
    assert_eq!(first_events[0].version, host::EVENT_VERSION);
    assert_eq!(first_events[0].subject, authority.to_bytes());
    assert_eq!(first_events[0].plaintext, plaintext);
    assert_eq!(first_events[0].fhe_type, 5);
    assert_eq!(first_events[0].result, first_handle);
    assert_single_acl_allowed_event(&first_result, first_handle, authority);

    let second_result = context.process_instruction(&trivial_encrypt_and_bind_ix(
        program_id,
        authority,
        host_config,
        second_acl,
        nonce_key,
        1,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        plaintext,
        5,
        false,
    ));
    assert!(second_result.raw_result.is_ok());
    let second_record = read_acl_record(&context, second_acl).expect("expected second trivial ACL");
    assert_bound_acl_record(
        &second_record,
        second_handle,
        nonce_key,
        1,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        authority,
        host::ACL_ROLE_USER,
        context.mollusk.sysvars.clock.slot,
    );
    let second_events: Vec<TrivialEncryptEvent> = second_result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(second_events.len(), 1);
    assert_eq!(second_events[0].plaintext, plaintext);
    assert_eq!(second_events[0].fhe_type, 5);
    assert_eq!(second_events[0].result, second_handle);
    assert_single_acl_allowed_event(&second_result, second_handle, authority);
}

#[test]
fn mollusk_acl_allow_subjects_extends_inline_record_and_is_idempotent() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("allow-inline");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = handle_for_chain(10, 5);
    let new_subject = Pubkey::new_unique();
    let (acl_record, acl_account) = authorizing_acl_record_account(
        nonce_key,
        40,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
    );
    let context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (acl_record, acl_account),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );
    let ix = allow_acl_subjects_ix(
        program_id,
        authority,
        authority,
        None,
        host_config,
        acl_record,
        None,
        handle,
        vec![AclSubjectEntry::user(new_subject)],
        &[],
    );

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_ok());
    assert_single_acl_allowed_event(&result, handle, new_subject);
    let record = read_acl_record(&context, acl_record).expect("expected ACL record");
    assert_eq!(record.subject_count, 2);
    assert_eq!(record.subjects[0], authority);
    assert_eq!(record.subject_roles[0], host::ACL_ROLE_ALL);
    assert_eq!(record.subjects[1], new_subject);
    assert_eq!(record.subject_roles[1], host::ACL_ROLE_USER);
    assert_eq!(record.overflow_subject_count, 0);

    let idempotent = context.process_instruction(&ix);
    assert!(idempotent.raw_result.is_ok());
    assert!(acl_allowed_events(&idempotent).is_empty());
    let record = read_acl_record(&context, acl_record).expect("expected ACL record");
    assert_eq!(record.subject_count, 2);
    assert_eq!(record.subjects[1], new_subject);
    assert_eq!(record.subject_roles[1], host::ACL_ROLE_USER);
}

#[test]
fn mollusk_acl_allow_subjects_rejects_oversized_grant_batch_without_mutation() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("allow-batch-cap");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = handle_for_chain(101, 5);
    let (acl_record, acl_account) = authorizing_acl_record_account(
        nonce_key,
        101,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
    );
    let context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (acl_record, acl_account),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );
    let subjects = (0..=host::MAX_ACL_SUBJECT_GRANTS_PER_CALL)
        .map(|_| AclSubjectEntry::use_only(Pubkey::new_unique()))
        .collect();

    let result = context.process_instruction(&allow_acl_subjects_ix(
        program_id,
        authority,
        authority,
        None,
        host_config,
        acl_record,
        None,
        handle,
        subjects,
        &[],
    ));

    assert!(result.raw_result.is_err());
    let record = read_acl_record(&context, acl_record).expect("expected ACL record");
    assert_eq!(record.subject_count, 1);
    assert_eq!(record.subjects[0], authority);
    assert_eq!(record.subject_roles[0], host::ACL_ROLE_ALL);
    assert_eq!(record.overflow_subject_count, 0);
}

#[test]
fn mollusk_acl_allow_subjects_rejects_compute_only_authority_without_mutation() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("allow-compute-only");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = handle_for_chain(11, 5);
    let new_subject = Pubkey::new_unique();
    let (acl_record, acl_account) = acl_record_account_with_subject_role(
        nonce_key,
        41,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
        host::ACL_ROLE_COMPUTE,
    );
    let context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (acl_record, acl_account),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );
    let result = context.process_instruction(&allow_acl_subjects_ix(
        program_id,
        authority,
        authority,
        None,
        host_config,
        acl_record,
        None,
        handle,
        vec![AclSubjectEntry::user(new_subject)],
        &[],
    ));

    assert!(result.raw_result.is_err());
    let record = read_acl_record(&context, acl_record).expect("expected ACL record");
    assert_eq!(record.subject_count, 1);
    assert_eq!(record.subjects[0], authority);
    assert_eq!(record.subject_roles[0], host::ACL_ROLE_COMPUTE);
    assert!(!record.inline_subject_has_role(new_subject, host::ACL_ROLE_USER));
}

#[test]
fn mollusk_acl_allow_subjects_creates_overflow_permission_and_asserts_membership() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("allow-overflow");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = handle_for_chain(12, 5);
    let mut subjects = vec![AclSubjectEntry {
        pubkey: authority,
        role_flags: host::ACL_ROLE_ALL,
    }];
    subjects.extend((0..7).map(|_| AclSubjectEntry::use_only(Pubkey::new_unique())));
    let (acl_record, acl_account) = acl_record_account_with_subjects(
        nonce_key,
        42,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &subjects,
    );
    let overflow_subject = Pubkey::new_unique();
    let (permission_record, permission_bump) =
        host::acl_permission_address(acl_record, overflow_subject);
    let context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (acl_record, acl_account),
            (permission_record, system_account(0)),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );

    let result = context.process_instruction(&allow_acl_subjects_ix(
        program_id,
        authority,
        authority,
        None,
        host_config,
        acl_record,
        None,
        handle,
        vec![AclSubjectEntry::use_only(overflow_subject)],
        &[permission_record],
    ));

    assert!(result.raw_result.is_ok());
    assert_single_acl_allowed_event(&result, handle, overflow_subject);
    let record = read_acl_record(&context, acl_record).expect("expected ACL record");
    assert_eq!(record.subject_count as usize, host::MAX_ACL_SUBJECTS);
    assert_eq!(record.overflow_subject_count, 1);
    let permission =
        read_acl_permission(&context, permission_record).expect("expected overflow permission");
    assert_eq!(permission.acl_record, acl_record);
    assert_eq!(permission.subject, overflow_subject);
    assert_eq!(permission.role_flags, host::ACL_ROLE_USE);
    assert_eq!(permission.bump, permission_bump);

    let assert_result = context.process_instruction(&assert_acl_record_ix(
        program_id,
        acl_record,
        Some(permission_record),
        nonce_key,
        42,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        overflow_subject,
    ));
    assert!(assert_result.raw_result.is_ok());
}

#[test]
fn mollusk_acl_allow_subjects_rejects_bad_overflow_witnesses_without_mutation() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let handle = handle_for_chain(13, 5);
    let mut full_subjects = vec![AclSubjectEntry {
        pubkey: authority,
        role_flags: host::ACL_ROLE_ALL,
    }];
    full_subjects.extend((0..7).map(|_| AclSubjectEntry::use_only(Pubkey::new_unique())));

    let missing_label = label("allow-missing-overflow");
    let missing_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, missing_label);
    let (missing_host_config, missing_host_config_account) = host_config_account(authority);
    let (missing_acl, missing_acl_account) = acl_record_account_with_subjects(
        missing_nonce_key,
        43,
        acl_domain_key,
        app_account,
        missing_label,
        handle,
        &full_subjects,
    );
    let missing_context = mollusk_eval_context(
        authority,
        vec![
            (missing_host_config, missing_host_config_account),
            (missing_acl, missing_acl_account),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );
    let missing_subject = Pubkey::new_unique();
    let missing = missing_context.process_instruction(&allow_acl_subjects_ix(
        program_id,
        authority,
        authority,
        None,
        missing_host_config,
        missing_acl,
        None,
        handle,
        vec![AclSubjectEntry::use_only(missing_subject)],
        &[],
    ));
    assert!(missing.raw_result.is_err());
    let record = read_acl_record(&missing_context, missing_acl).expect("expected ACL record");
    assert_eq!(record.subject_count as usize, host::MAX_ACL_SUBJECTS);
    assert_eq!(record.overflow_subject_count, 0);

    let dirty_label = label("allow-dirty-overflow");
    let dirty_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, dirty_label);
    let (dirty_host_config, dirty_host_config_account) = host_config_account(authority);
    let (dirty_acl, dirty_acl_account) = acl_record_account_with_subjects(
        dirty_nonce_key,
        44,
        acl_domain_key,
        app_account,
        dirty_label,
        handle,
        &full_subjects,
    );
    let dirty_subject = Pubkey::new_unique();
    let (dirty_permission, _) = host::acl_permission_address(dirty_acl, dirty_subject);
    let dirty_context = mollusk_eval_context(
        authority,
        vec![
            (dirty_host_config, dirty_host_config_account),
            (dirty_acl, dirty_acl_account),
            (
                dirty_permission,
                Account {
                    lamports: 1_000_000,
                    data: vec![1],
                    owner: system_program::ID,
                    executable: false,
                    rent_epoch: 0,
                },
            ),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );
    let dirty = dirty_context.process_instruction(&allow_acl_subjects_ix(
        program_id,
        authority,
        authority,
        None,
        dirty_host_config,
        dirty_acl,
        None,
        handle,
        vec![AclSubjectEntry::use_only(dirty_subject)],
        &[dirty_permission],
    ));
    assert!(dirty.raw_result.is_err());
    let record = read_acl_record(&dirty_context, dirty_acl).expect("expected ACL record");
    assert_eq!(record.overflow_subject_count, 0);
    let dirty_account = dirty_context
        .account_store
        .borrow()
        .get(&dirty_permission)
        .cloned()
        .expect("expected dirty target");
    assert_eq!(dirty_account.owner, system_program::ID);
    assert_eq!(dirty_account.data, vec![1]);

    let extra_label = label("allow-extra-inline");
    let extra_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, extra_label);
    let (extra_host_config, extra_host_config_account) = host_config_account(authority);
    let (extra_acl, extra_acl_account) = authorizing_acl_record_account(
        extra_nonce_key,
        45,
        acl_domain_key,
        app_account,
        extra_label,
        handle,
        authority,
    );
    let inline_subject = Pubkey::new_unique();
    let (extra_permission, _) = host::acl_permission_address(extra_acl, Pubkey::new_unique());
    let extra_context = mollusk_eval_context(
        authority,
        vec![
            (extra_host_config, extra_host_config_account),
            (extra_acl, extra_acl_account),
            (extra_permission, system_account(0)),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );
    let extra = extra_context.process_instruction(&allow_acl_subjects_ix(
        program_id,
        authority,
        authority,
        None,
        extra_host_config,
        extra_acl,
        None,
        handle,
        vec![AclSubjectEntry::use_only(inline_subject)],
        &[extra_permission],
    ));
    assert!(extra.raw_result.is_err());
    let record = read_acl_record(&extra_context, extra_acl).expect("expected ACL record");
    assert_eq!(record.subject_count, 1);
    assert_eq!(record.overflow_subject_count, 0);
    assert!(!record.inline_subject_has_role(inline_subject, host::ACL_ROLE_USE));
    assert!(read_acl_permission(&extra_context, extra_permission).is_none());
}

#[test]
fn mollusk_acl_grant_deny_list_blocks_denied_authority_without_mutation() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let new_subject = Pubkey::new_unique();
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("deny-authority");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = input_handle_for_chain(41, 0, 5);
    let (host_config, host_config_account) =
        host_config_account_with_grant_deny_list(authority, true);
    let (acl_record, acl_account) = authorizing_acl_record_account(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
    );
    let (deny_subject_record, deny_account) = deny_subject_record_account(authority, true);
    let context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (acl_record, acl_account),
            (deny_subject_record, deny_account),
        ],
    );

    let result = context.process_instruction(&allow_acl_subjects_ix(
        program_id,
        authority,
        authority,
        None,
        host_config,
        acl_record,
        Some(deny_subject_record),
        handle,
        vec![AclSubjectEntry::user(new_subject)],
        &[],
    ));

    assert!(result.raw_result.is_err());
    assert!(acl_allowed_events(&result).is_empty());
    let record = read_acl_record(&context, acl_record).expect("expected ACL record");
    assert_eq!(record.subject_count, 1);
    assert!(!record.inline_subject_has_role(new_subject, host::ACL_ROLE_USE));
}

#[test]
fn mollusk_acl_grant_deny_list_rejects_missing_or_noncanonical_authority_witness() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("deny-witness");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = input_handle_for_chain(42, 0, 5);
    let (host_config, host_config_account) =
        host_config_account_with_grant_deny_list(authority, true);
    let (acl_record, acl_account) = authorizing_acl_record_account(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
    );

    for deny_subject_record in [None, Some(Pubkey::new_unique())] {
        let context = mollusk_eval_context(
            authority,
            vec![
                (host_config, host_config_account.clone()),
                (acl_record, acl_account.clone()),
            ],
        );
        let new_subject = Pubkey::new_unique();
        let result = context.process_instruction(&allow_acl_subjects_ix(
            program_id,
            authority,
            authority,
            None,
            host_config,
            acl_record,
            deny_subject_record,
            handle,
            vec![AclSubjectEntry::user(new_subject)],
            &[],
        ));

        assert!(result.raw_result.is_err());
        assert!(acl_allowed_events(&result).is_empty());
        let record = read_acl_record(&context, acl_record).expect("expected ACL record");
        assert_eq!(record.subject_count, 1);
        assert!(!record.inline_subject_has_role(new_subject, host::ACL_ROLE_USE));
    }
}

#[test]
fn mollusk_acl_grant_deny_list_allows_absent_or_clear_authority_deny_record() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("deny-clear");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = input_handle_for_chain(43, 0, 5);
    let authority_deny_record = host::deny_subject_address(authority).0;

    let (host_config, host_config_account) =
        host_config_account_with_grant_deny_list(authority, true);
    let (acl_record, acl_account) = authorizing_acl_record_account(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
    );
    let absent_context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account.clone()),
            (acl_record, acl_account.clone()),
        ],
    );
    let absent_subject = Pubkey::new_unique();
    let absent = absent_context.process_instruction(&allow_acl_subjects_ix(
        program_id,
        authority,
        authority,
        None,
        host_config,
        acl_record,
        Some(authority_deny_record),
        handle,
        vec![AclSubjectEntry::user(absent_subject)],
        &[],
    ));
    assert!(absent.raw_result.is_ok());
    assert_single_acl_allowed_event(&absent, handle, absent_subject);
    let absent_record = read_acl_record(&absent_context, acl_record).expect("expected ACL record");
    assert!(absent_record.inline_subject_has_role(absent_subject, host::ACL_ROLE_USE));

    let clear_subject = Pubkey::new_unique();
    let denied_new_subject = Pubkey::new_unique();
    let (authority_deny_record, clear_authority_deny_account) =
        deny_subject_record_account(authority, false);
    let (denied_new_subject_record, denied_new_subject_account) =
        deny_subject_record_account(denied_new_subject, true);
    let clear_context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (acl_record, acl_account),
            (authority_deny_record, clear_authority_deny_account),
            (denied_new_subject_record, denied_new_subject_account),
        ],
    );
    let clear = clear_context.process_instruction(&allow_acl_subjects_ix(
        program_id,
        authority,
        authority,
        None,
        host_config,
        acl_record,
        Some(authority_deny_record),
        handle,
        vec![
            AclSubjectEntry::user(clear_subject),
            AclSubjectEntry::user(denied_new_subject),
        ],
        &[],
    ));
    assert!(clear.raw_result.is_ok());
    let clear_record = read_acl_record(&clear_context, acl_record).expect("expected ACL record");
    assert!(clear_record.inline_subject_has_role(clear_subject, host::ACL_ROLE_USE));
    assert!(clear_record.inline_subject_has_role(denied_new_subject, host::ACL_ROLE_USE));
    assert!(
        read_deny_subject_record(&clear_context, denied_new_subject_record)
            .expect("expected new-subject deny record")
            .denied
    );
}

#[test]
fn mollusk_acl_grant_deny_list_rejects_witness_when_disabled() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let new_subject = Pubkey::new_unique();
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("deny-disabled");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = input_handle_for_chain(44, 0, 5);
    let (host_config, host_config_account) =
        host_config_account_with_grant_deny_list(authority, false);
    let (acl_record, acl_account) = authorizing_acl_record_account(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
    );
    let (deny_subject_record, deny_account) = deny_subject_record_account(authority, false);
    let context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (acl_record, acl_account),
            (deny_subject_record, deny_account),
        ],
    );

    let result = context.process_instruction(&allow_acl_subjects_ix(
        program_id,
        authority,
        authority,
        None,
        host_config,
        acl_record,
        Some(deny_subject_record),
        handle,
        vec![AclSubjectEntry::user(new_subject)],
        &[],
    ));

    assert!(result.raw_result.is_err());
    assert!(acl_allowed_events(&result).is_empty());
    let record = read_acl_record(&context, acl_record).expect("expected ACL record");
    assert_eq!(record.subject_count, 1);
    assert!(!record.inline_subject_has_role(new_subject, host::ACL_ROLE_USE));
}

#[test]
fn mollusk_host_admin_deny_subject_setter_preserves_idempotent_state() {
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let subject = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(admin);
    let (deny_subject_record, expected_bump) = host::deny_subject_address(subject);
    let mut context = mollusk_eval_context(
        admin,
        vec![
            (host_config, host_config_account),
            (deny_subject_record, system_account(0)),
        ],
    );

    let default_false = context.process_instruction(&set_deny_subject_ix(
        program_id,
        admin,
        admin,
        host_config,
        deny_subject_record,
        subject,
        false,
    ));
    assert!(default_false.raw_result.is_ok());
    assert!(read_deny_subject_record(&context, deny_subject_record).is_none());
    let default_account = stored_account(&context, deny_subject_record);
    assert_eq!(default_account.owner, system_program::ID);
    assert!(default_account.data.is_empty());
    assert_eq!(default_account.lamports, 0);

    context.mollusk.sysvars.warp_to_slot(10);
    let set_true = context.process_instruction(&set_deny_subject_ix(
        program_id,
        admin,
        admin,
        host_config,
        deny_subject_record,
        subject,
        true,
    ));
    assert!(set_true.raw_result.is_ok());
    let denied_record =
        read_deny_subject_record(&context, deny_subject_record).expect("expected deny record");
    assert_eq!(denied_record.subject, subject);
    assert!(denied_record.denied);
    assert_eq!(denied_record.bump, expected_bump);
    let denied_account = stored_account(&context, deny_subject_record);
    assert_eq!(denied_account.owner, host::id());
    assert!(!denied_account.data.is_empty());

    context.mollusk.sysvars.warp_to_slot(11);
    let repeat_true = context.process_instruction(&set_deny_subject_ix(
        program_id,
        admin,
        admin,
        host_config,
        deny_subject_record,
        subject,
        true,
    ));
    assert!(repeat_true.raw_result.is_ok());
    assert_account_unchanged(
        &stored_account(&context, deny_subject_record),
        &denied_account,
    );
    let repeated_denied_record =
        read_deny_subject_record(&context, deny_subject_record).expect("expected deny record");
    assert_eq!(repeated_denied_record.subject, subject);
    assert!(repeated_denied_record.denied);
    assert_eq!(repeated_denied_record.bump, expected_bump);

    context.mollusk.sysvars.warp_to_slot(12);
    let clear = context.process_instruction(&set_deny_subject_ix(
        program_id,
        admin,
        admin,
        host_config,
        deny_subject_record,
        subject,
        false,
    ));
    assert!(clear.raw_result.is_ok());
    let cleared_record =
        read_deny_subject_record(&context, deny_subject_record).expect("expected deny record");
    assert_eq!(cleared_record.subject, subject);
    assert!(!cleared_record.denied);
    assert_eq!(cleared_record.bump, expected_bump);
    let cleared_account = stored_account(&context, deny_subject_record);
    assert_eq!(cleared_account.owner, host::id());
    assert_ne!(cleared_account.data, denied_account.data);

    context.mollusk.sysvars.warp_to_slot(13);
    let repeat_clear = context.process_instruction(&set_deny_subject_ix(
        program_id,
        admin,
        admin,
        host_config,
        deny_subject_record,
        subject,
        false,
    ));
    assert!(repeat_clear.raw_result.is_ok());
    assert_account_unchanged(
        &stored_account(&context, deny_subject_record),
        &cleared_account,
    );
    let repeated_clear_record =
        read_deny_subject_record(&context, deny_subject_record).expect("expected deny record");
    assert_eq!(repeated_clear_record.subject, subject);
    assert!(!repeated_clear_record.denied);
    assert_eq!(repeated_clear_record.bump, expected_bump);
}

#[test]
fn mollusk_host_config_initialize_creates_state_and_rejects_zero_profile_fields() {
    let program_id = host::id();
    let payer = Pubkey::new_unique();
    let admin = Pubkey::new_unique();
    let input_verifier_authority = Pubkey::new_unique();
    let material_authority = Pubkey::new_unique();
    let test_authority = Pubkey::new_unique();
    let (host_config, expected_bump) = host::host_config_address();
    let args = host::InitializeHostConfigArgs {
        chain_id: host::SOLANA_POC_CHAIN_ID,
        input_verifier_authority,
        gateway_chain_id: 0,
        input_verification_contract: [0u8; 20],
        coprocessor_signer: [0u8; 20],
        decryption_contract: [0u8; 20],
        material_authority,
        test_authority,
        mock_input_enabled: false,
        test_shims_enabled: false,
        grant_deny_list_enabled: true,
    };
    let context = mollusk_eval_context(payer, vec![(host_config, system_account(0))]);

    let result = context.process_instruction(&initialize_host_config_ix(
        program_id,
        payer,
        admin,
        host_config,
        args,
    ));

    assert!(result.raw_result.is_ok());
    let config = read_host_config(&context, host_config).expect("expected host config");
    assert_eq!(config.admin, admin);
    assert_eq!(config.chain_id, host::SOLANA_POC_CHAIN_ID);
    assert_eq!(config.input_verifier_authority, input_verifier_authority);
    assert_eq!(config.material_authority, material_authority);
    assert_eq!(config.test_authority, test_authority);
    assert!(!config.paused);
    assert!(!config.mock_input_enabled);
    assert!(!config.test_shims_enabled);
    assert!(config.grant_deny_list_enabled);
    assert_eq!(config.updated_slot, context.mollusk.sysvars.clock.slot);
    assert_eq!(config.bump, expected_bump);

    let cases: [fn(&mut host::InitializeHostConfigArgs); 4] = [
        |args| args.chain_id = 0,
        |args| args.input_verifier_authority = Pubkey::default(),
        |args| args.material_authority = Pubkey::default(),
        |args| args.test_authority = Pubkey::default(),
    ];

    for mutate in cases {
        let mut args = host::InitializeHostConfigArgs {
            chain_id: host::SOLANA_POC_CHAIN_ID,
            input_verifier_authority,
            gateway_chain_id: 0,
            input_verification_contract: [0u8; 20],
            coprocessor_signer: [0u8; 20],
            decryption_contract: [0u8; 20],
            material_authority,
            test_authority,
            mock_input_enabled: true,
            test_shims_enabled: true,
            grant_deny_list_enabled: false,
        };
        mutate(&mut args);
        let context = mollusk_eval_context(payer, vec![(host_config, system_account(0))]);

        let result = context.process_instruction(&initialize_host_config_ix(
            program_id,
            payer,
            admin,
            host_config,
            args,
        ));

        assert!(result.raw_result.is_err());
        assert!(read_host_config(&context, host_config).is_none());
    }
}

#[test]
fn mollusk_host_config_rejects_wrong_admin_and_oversized_singleton_without_mutation() {
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let wrong_admin = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(admin);
    let wrong_admin_context = mollusk_eval_context(admin, vec![(host_config, host_config_account)]);

    let wrong_admin_result = wrong_admin_context.process_instruction(&set_host_pause_ix(
        program_id,
        wrong_admin,
        host_config,
        true,
    ));

    assert!(wrong_admin_result.raw_result.is_err());
    let config = read_host_config(&wrong_admin_context, host_config).expect("expected host config");
    assert!(!config.paused);
    assert_eq!(config.updated_slot, 0);

    let acl_domain_key = Pubkey::new_unique();
    let app_account = admin;
    let encrypted_value_label = label("config-shape");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = input_handle_for_chain(47, 0, 5);
    let new_subject = Pubkey::new_unique();
    let (acl_record, acl_account) = authorizing_acl_record_account(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        admin,
    );
    let (oversized_host_config, oversized_account) = host_config_account_with_extra_bytes(admin, 1);
    let oversized_context = mollusk_eval_context(
        admin,
        vec![
            (oversized_host_config, oversized_account),
            (acl_record, acl_account),
        ],
    );

    let oversized_pause = oversized_context.process_instruction(&set_host_pause_ix(
        program_id,
        admin,
        oversized_host_config,
        true,
    ));
    assert!(oversized_pause.raw_result.is_err());
    let config =
        read_host_config(&oversized_context, oversized_host_config).expect("expected host config");
    assert!(!config.paused);
    assert_eq!(config.updated_slot, 0);

    let oversized_grant = oversized_context.process_instruction(&allow_acl_subjects_ix(
        program_id,
        admin,
        admin,
        None,
        oversized_host_config,
        acl_record,
        None,
        handle,
        vec![AclSubjectEntry::user(new_subject)],
        &[],
    ));
    assert!(oversized_grant.raw_result.is_err());
    assert!(acl_allowed_events(&oversized_grant).is_empty());
    let record = read_acl_record(&oversized_context, acl_record).expect("expected ACL record");
    assert_eq!(record.subject_count, 1);
    assert!(!record.inline_subject_has_role(new_subject, host::ACL_ROLE_USE));
}

#[test]
fn mollusk_host_admin_pause_blocks_acl_grants_but_allows_unpause() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("admin-pause");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = input_handle_for_chain(45, 0, 5);
    let new_subject = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let (acl_record, acl_account) = authorizing_acl_record_account(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
    );
    let mut context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (acl_record, acl_account),
        ],
    );

    let initial_config = read_host_config(&context, host_config).expect("expected host config");
    assert!(!initial_config.paused);
    assert_eq!(initial_config.updated_slot, 0);

    let pause =
        context.process_instruction(&set_host_pause_ix(program_id, authority, host_config, true));
    assert!(pause.raw_result.is_ok());
    let paused_config = read_host_config(&context, host_config).expect("expected host config");
    assert!(paused_config.paused);
    assert_eq!(
        paused_config.updated_slot,
        context.mollusk.sysvars.clock.slot
    );

    context
        .mollusk
        .sysvars
        .warp_to_slot(paused_config.updated_slot + 1);
    let repeated_pause =
        context.process_instruction(&set_host_pause_ix(program_id, authority, host_config, true));
    assert!(repeated_pause.raw_result.is_ok());
    let repeated_config = read_host_config(&context, host_config).expect("expected host config");
    assert!(repeated_config.paused);
    assert_eq!(repeated_config.updated_slot, paused_config.updated_slot);

    let paused_grant = context.process_instruction(&allow_acl_subjects_ix(
        program_id,
        authority,
        authority,
        None,
        host_config,
        acl_record,
        None,
        handle,
        vec![AclSubjectEntry::user(new_subject)],
        &[],
    ));
    assert!(paused_grant.raw_result.is_err());
    assert!(acl_allowed_events(&paused_grant).is_empty());
    let paused_record = read_acl_record(&context, acl_record).expect("expected ACL record");
    assert_eq!(paused_record.subject_count, 1);
    assert!(!paused_record.inline_subject_has_role(new_subject, host::ACL_ROLE_USE));

    context
        .mollusk
        .sysvars
        .warp_to_slot(paused_config.updated_slot + 2);
    let unpause = context.process_instruction(&set_host_pause_ix(
        program_id,
        authority,
        host_config,
        false,
    ));
    assert!(unpause.raw_result.is_ok());
    let unpaused_config = read_host_config(&context, host_config).expect("expected host config");
    assert!(!unpaused_config.paused);
    assert!(unpaused_config.updated_slot > paused_config.updated_slot);

    context
        .mollusk
        .sysvars
        .warp_to_slot(unpaused_config.updated_slot + 1);
    let repeated_unpause = context.process_instruction(&set_host_pause_ix(
        program_id,
        authority,
        host_config,
        false,
    ));
    assert!(repeated_unpause.raw_result.is_ok());
    let repeated_unpause_config =
        read_host_config(&context, host_config).expect("expected host config");
    assert!(!repeated_unpause_config.paused);
    assert_eq!(
        repeated_unpause_config.updated_slot,
        unpaused_config.updated_slot
    );

    let unpaused_grant = context.process_instruction(&allow_acl_subjects_ix(
        program_id,
        authority,
        authority,
        None,
        host_config,
        acl_record,
        None,
        handle,
        vec![AclSubjectEntry::user(new_subject)],
        &[],
    ));
    assert!(unpaused_grant.raw_result.is_ok());
    assert_single_acl_allowed_event(&unpaused_grant, handle, new_subject);
    let record = read_acl_record(&context, acl_record).expect("expected ACL record");
    assert!(record.inline_subject_has_role(new_subject, host::ACL_ROLE_USE));
}

#[test]
fn mollusk_host_admin_grant_deny_list_flag_is_idempotent_and_drives_gate() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("admin-deny-flag");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = input_handle_for_chain(46, 0, 5);
    let denied_by_gate_subject = Pubkey::new_unique();
    let allowed_after_disable_subject = Pubkey::new_unique();
    let (host_config, host_config_account) =
        host_config_account_with_grant_deny_list(authority, false);
    let (acl_record, acl_account) = authorizing_acl_record_account(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
    );
    let mut context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (acl_record, acl_account),
        ],
    );

    let repeat_disabled = context.process_instruction(&set_grant_deny_list_enabled_ix(
        program_id,
        authority,
        host_config,
        false,
    ));
    assert!(repeat_disabled.raw_result.is_ok());
    let disabled_config = read_host_config(&context, host_config).expect("expected host config");
    assert!(!disabled_config.grant_deny_list_enabled);
    assert_eq!(disabled_config.updated_slot, 0);

    let enable = context.process_instruction(&set_grant_deny_list_enabled_ix(
        program_id,
        authority,
        host_config,
        true,
    ));
    assert!(enable.raw_result.is_ok());
    let enabled_config = read_host_config(&context, host_config).expect("expected host config");
    assert!(enabled_config.grant_deny_list_enabled);
    assert_eq!(
        enabled_config.updated_slot,
        context.mollusk.sysvars.clock.slot
    );

    context
        .mollusk
        .sysvars
        .warp_to_slot(enabled_config.updated_slot + 1);
    let repeat_enabled = context.process_instruction(&set_grant_deny_list_enabled_ix(
        program_id,
        authority,
        host_config,
        true,
    ));
    assert!(repeat_enabled.raw_result.is_ok());
    let repeated_config = read_host_config(&context, host_config).expect("expected host config");
    assert!(repeated_config.grant_deny_list_enabled);
    assert_eq!(repeated_config.updated_slot, enabled_config.updated_slot);

    let missing_witness = context.process_instruction(&allow_acl_subjects_ix(
        program_id,
        authority,
        authority,
        None,
        host_config,
        acl_record,
        None,
        handle,
        vec![AclSubjectEntry::user(denied_by_gate_subject)],
        &[],
    ));
    assert!(missing_witness.raw_result.is_err());
    assert!(acl_allowed_events(&missing_witness).is_empty());
    let gated_record = read_acl_record(&context, acl_record).expect("expected ACL record");
    assert!(!gated_record.inline_subject_has_role(denied_by_gate_subject, host::ACL_ROLE_USE));

    context
        .mollusk
        .sysvars
        .warp_to_slot(enabled_config.updated_slot + 2);
    let disable = context.process_instruction(&set_grant_deny_list_enabled_ix(
        program_id,
        authority,
        host_config,
        false,
    ));
    assert!(disable.raw_result.is_ok());
    let disabled_config = read_host_config(&context, host_config).expect("expected host config");
    assert!(!disabled_config.grant_deny_list_enabled);
    assert!(disabled_config.updated_slot > enabled_config.updated_slot);

    let grant_after_disable = context.process_instruction(&allow_acl_subjects_ix(
        program_id,
        authority,
        authority,
        None,
        host_config,
        acl_record,
        None,
        handle,
        vec![AclSubjectEntry::user(allowed_after_disable_subject)],
        &[],
    ));
    assert!(grant_after_disable.raw_result.is_ok());
    assert_single_acl_allowed_event(&grant_after_disable, handle, allowed_after_disable_subject);
    let record = read_acl_record(&context, acl_record).expect("expected ACL record");
    assert!(record.inline_subject_has_role(allowed_after_disable_subject, host::ACL_ROLE_USE));
    assert!(!record.inline_subject_has_role(denied_by_gate_subject, host::ACL_ROLE_USE));
}

#[test]
fn mollusk_delegation_rejects_zero_wildcard_and_equal_subjects() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let delegate = Pubkey::new_unique();
    let app_account = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let context = mollusk_eval_context(authority, vec![host_config_account(authority)]);
    let expiration_slot = context.mollusk.sysvars.clock.slot + 100;

    for (bad_delegate, bad_app_account) in [
        (Pubkey::default(), app_account),
        (delegate, Pubkey::default()),
        (
            Pubkey::new_from_array(host::WILDCARD_APP_CONTEXT_BYTES),
            app_account,
        ),
        (authority, app_account),
        (delegate, authority),
        (delegate, delegate),
    ] {
        let delegation_record =
            host::user_decryption_delegation_address(authority, bad_delegate, bad_app_account).0;
        let ix = delegate_for_user_decryption_ix(
            program_id,
            authority,
            host_config,
            delegation_record,
            bad_delegate,
            bad_app_account,
            expiration_slot,
        );

        let result = context.process_instruction(&ix);
        assert!(result.raw_result.is_err());
        assert!(read_delegation_record(&context, delegation_record).is_none());
    }
}

#[test]
fn mollusk_delegation_counter_tracks_regrant_revoke_and_reactivation() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let delegate = Pubkey::new_unique();
    let app_account = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let (delegation_record, _) =
        host::user_decryption_delegation_address(authority, delegate, app_account);
    let mut context = mollusk_eval_context(
        authority,
        vec![
            host_config_account(authority),
            (delegation_record, system_account(0)),
        ],
    );
    let first_expiration = context.mollusk.sysvars.clock.slot + 100;

    let first_grant = delegate_for_user_decryption_ix(
        program_id,
        authority,
        host_config,
        delegation_record,
        delegate,
        app_account,
        first_expiration,
    );
    let result = context.process_instruction(&first_grant);
    assert!(result.raw_result.is_ok());
    let first = read_delegation_record(&context, delegation_record).expect("expected delegation");
    assert_eq!(first.delegator, authority);
    assert_eq!(first.delegate, delegate);
    assert_eq!(first.app_account, app_account);
    assert_eq!(first.expiration_slot, first_expiration);
    assert_eq!(first.delegation_counter, 1);
    assert!(!first.revoked);

    let unchanged_grant = delegate_for_user_decryption_ix(
        program_id,
        authority,
        host_config,
        delegation_record,
        delegate,
        app_account,
        first_expiration,
    );
    let result = context.process_instruction(&unchanged_grant);
    assert!(result.raw_result.is_err());
    let unchanged =
        read_delegation_record(&context, delegation_record).expect("expected delegation");
    assert_eq!(unchanged.delegation_counter, 1);
    assert_eq!(unchanged.expiration_slot, first_expiration);
    assert!(!unchanged.revoked);

    context
        .mollusk
        .sysvars
        .warp_to_slot(first.last_update_slot + 1);
    let second_expiration = first_expiration + 10;
    let second_grant = delegate_for_user_decryption_ix(
        program_id,
        authority,
        host_config,
        delegation_record,
        delegate,
        app_account,
        second_expiration,
    );
    let result = context.process_instruction(&second_grant);
    assert!(result.raw_result.is_ok());
    let second = read_delegation_record(&context, delegation_record).expect("expected delegation");
    assert_eq!(second.expiration_slot, second_expiration);
    assert_eq!(second.delegation_counter, 2);
    assert!(second.last_update_slot > first.last_update_slot);
    assert!(!second.revoked);

    context
        .mollusk
        .sysvars
        .warp_to_slot(second.last_update_slot + 1);
    let revoke = revoke_delegation_for_user_decryption_ix(
        program_id,
        authority,
        host_config,
        delegation_record,
    );
    let result = context.process_instruction(&revoke);
    assert!(result.raw_result.is_ok());
    let revoked = read_delegation_record(&context, delegation_record).expect("expected delegation");
    assert_eq!(revoked.expiration_slot, 0);
    assert_eq!(revoked.delegation_counter, 3);
    assert!(revoked.last_update_slot > second.last_update_slot);
    assert!(revoked.revoked);

    context
        .mollusk
        .sysvars
        .warp_to_slot(revoked.last_update_slot + 1);
    let third_expiration = second_expiration + 10;
    let third_grant = delegate_for_user_decryption_ix(
        program_id,
        authority,
        host_config,
        delegation_record,
        delegate,
        app_account,
        third_expiration,
    );
    let result = context.process_instruction(&third_grant);
    assert!(result.raw_result.is_ok());
    let regranted =
        read_delegation_record(&context, delegation_record).expect("expected delegation");
    assert_eq!(regranted.expiration_slot, third_expiration);
    assert_eq!(regranted.delegation_counter, 4);
    assert!(regranted.last_update_slot > revoked.last_update_slot);
    assert!(!regranted.revoked);
}

#[test]
fn mollusk_material_commitment_seals_acl_and_public_decrypt_readiness() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("material-witness");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = handle_for_chain(12, 5);
    let (acl_record, acl_account) = authorizing_acl_record_account(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
    );
    let (material_commitment, _) = host::handle_material_address(acl_record);
    let context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (acl_record, acl_account),
            (material_commitment, system_account(0)),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );
    let key_id = [21; 32];
    let ciphertext_digest = [22; 32];
    let sns_ciphertext_digest = [23; 32];
    let coprocessor_set_digest = [24; 32];
    let expected_hash = host::handle_material_commitment_hash(
        material_commitment,
        acl_record,
        key_id,
        ciphertext_digest,
        sns_ciphertext_digest,
        coprocessor_set_digest,
    );

    let commit_ix = commit_handle_material_ix(
        program_id,
        authority,
        authority,
        host_config,
        acl_record,
        material_commitment,
        key_id,
        ciphertext_digest,
        sns_ciphertext_digest,
        coprocessor_set_digest,
    );
    let result = context.process_instruction(&commit_ix);
    assert!(result.raw_result.is_ok());

    let material =
        read_material_commitment(&context, material_commitment).expect("expected material");
    assert_eq!(material.acl_record, acl_record);
    assert_eq!(material.handle, handle);
    assert_eq!(material.key_id, key_id);
    assert_eq!(material.ciphertext_digest, ciphertext_digest);
    assert_eq!(material.sns_ciphertext_digest, sns_ciphertext_digest);
    assert_eq!(material.coprocessor_set_digest, coprocessor_set_digest);
    assert_eq!(material.material_commitment_hash, expected_hash);
    assert_eq!(material.created_slot, context.mollusk.sysvars.clock.slot);
    assert_eq!(material.state, host::HANDLE_MATERIAL_STATE_COMMITTED);

    let acl = read_acl_record(&context, acl_record).expect("expected ACL record");
    assert_eq!(acl.material_commitment, material_commitment);
    assert_eq!(acl.material_commitment_hash, expected_hash);
    assert_eq!(acl.material_key_id, key_id);
    assert!(!acl.public_decrypt);

    let committed_prefix = anchor_event_prefix(HandleMaterialCommittedEvent::DISCRIMINATOR);
    let committed_events: Vec<_> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| {
            let payload = inner
                .instruction
                .data
                .strip_prefix(committed_prefix.as_slice())?;
            HandleMaterialCommittedEvent::deserialize(&mut &*payload).ok()
        })
        .collect();
    assert_eq!(committed_events.len(), 1);
    assert_eq!(committed_events[0].material_commitment, material_commitment);
    assert_eq!(committed_events[0].acl_record, acl_record);
    assert_eq!(committed_events[0].handle, handle);
    assert_eq!(committed_events[0].key_id, key_id);
    assert_eq!(committed_events[0].material_commitment_hash, expected_hash);

    let sealed_prefix = anchor_event_prefix(HandleMaterialSealedEvent::DISCRIMINATOR);
    let sealed_events: Vec<_> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| {
            let payload = inner
                .instruction
                .data
                .strip_prefix(sealed_prefix.as_slice())?;
            HandleMaterialSealedEvent::deserialize(&mut &*payload).ok()
        })
        .collect();
    assert_eq!(sealed_events.len(), 1);
    assert_eq!(sealed_events[0].material_commitment, material_commitment);
    assert_eq!(sealed_events[0].acl_record, acl_record);
    assert_eq!(sealed_events[0].handle, handle);
    assert_eq!(sealed_events[0].key_id, key_id);
    assert_eq!(sealed_events[0].material_commitment_hash, expected_hash);

    let allow_ix = allow_for_decryption_ix(program_id, authority, host_config, acl_record, handle);
    let result = context.process_instruction(&allow_ix);
    assert!(result.raw_result.is_ok());
    let acl = read_acl_record(&context, acl_record).expect("expected ACL record");
    assert!(acl.public_decrypt);
    assert_eq!(acl.material_commitment, material_commitment);
    assert_eq!(acl.material_commitment_hash, expected_hash);

    let public_prefix = anchor_event_prefix(PublicDecryptAllowedEvent::DISCRIMINATOR);
    let public_events: Vec<_> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| {
            let payload = inner
                .instruction
                .data
                .strip_prefix(public_prefix.as_slice())?;
            PublicDecryptAllowedEvent::deserialize(&mut &*payload).ok()
        })
        .collect();
    assert_eq!(public_events.len(), 1);
    assert_eq!(public_events[0].acl_record, acl_record);
    assert_eq!(public_events[0].handle, handle);
    assert_eq!(public_events[0].authority, authority.to_bytes());
}

#[test]
fn mollusk_material_commitment_rejects_zero_material_fields_without_sealing() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("bad-material");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = handle_for_chain(13, 5);
    let (acl_record, acl_account) = authorizing_acl_record_account(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        authority,
    );
    let (material_commitment, _) = host::handle_material_address(acl_record);
    let context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (acl_record, acl_account),
            (material_commitment, system_account(0)),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );
    let commit_ix = commit_handle_material_ix(
        program_id,
        authority,
        authority,
        host_config,
        acl_record,
        material_commitment,
        [0; 32],
        [22; 32],
        [23; 32],
        [24; 32],
    );

    let result = context.process_instruction(&commit_ix);
    assert!(result.raw_result.is_err());
    assert!(read_material_commitment(&context, material_commitment).is_none());
    let acl = read_acl_record(&context, acl_record).expect("expected ACL record");
    assert_eq!(acl.material_commitment, Pubkey::default());
    assert_eq!(acl.material_commitment_hash, [0; 32]);
    assert_eq!(acl.material_key_id, [0; 32]);
    assert!(!acl.public_decrypt);
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
    let handle = handle_for_chain(7, 5);
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
    let context = mollusk_eval_context(
        authority,
        vec![
            host_config_account(authority),
            (acl_record, acl_account),
            (output_acl_record, system_account(0)),
        ],
    );

    let context_id = label("public-denied");
    let rhs = amount_plaintext(5);
    let mut ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: authority,
            compute_subject: authority,
            app_account_authority: authority,
            host_config,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                steps: vec![FheEvalStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheEvalOperand::AllowedDurable {
                        handle,
                        acl_record_index: 0,
                        permission_index: None,
                    },
                    rhs: FheEvalOperand::Scalar(rhs),
                    output_fhe_type: 5,
                    output: FheEvalOutput::AllowedDurable {
                        output_acl_record_index: 1,
                        output_app_account_authority_index: None,
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
fn mollusk_fhe_eval_rejects_system_account_public_decrypt_role_grant_without_input_role() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let input_label = label("public-role-input");
    let output_label = label("public-role-output");
    let input_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, input_label);
    let output_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, output_label);
    let handle = handle_for_chain(191, 5);
    let (acl_record, acl_account) = acl_record_account_with_subject_role(
        input_nonce_key,
        0,
        acl_domain_key,
        app_account,
        input_label,
        handle,
        authority,
        host::ACL_ROLE_USE,
    );
    let output_acl_record = host::acl_record_address(output_nonce_key, 0).0;
    let context = mollusk_eval_context(
        authority,
        vec![
            host_config_account(authority),
            (acl_record, acl_account),
            (output_acl_record, system_account(0)),
        ],
    );
    let context_id = label("public-role-frame");
    let rhs = amount_plaintext(5);
    let mut ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: authority,
            compute_subject: authority,
            app_account_authority: authority,
            host_config,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                steps: vec![FheEvalStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheEvalOperand::AllowedDurable {
                        handle,
                        acl_record_index: 0,
                        permission_index: None,
                    },
                    rhs: FheEvalOperand::Scalar(rhs),
                    output_fhe_type: 5,
                    output: FheEvalOutput::AllowedDurable {
                        output_acl_record_index: 1,
                        output_app_account_authority_index: None,
                        output_nonce_key,
                        output_nonce_sequence: 0,
                        output_acl_domain_key: acl_domain_key,
                        output_app_account: app_account,
                        output_encrypted_value_label: output_label,
                        output_subjects: vec![AclSubjectEntry::user(authority)],
                        output_public_decrypt: false,
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
fn mollusk_fhe_eval_allows_app_owned_public_decrypt_role_grant_without_input_role() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let app_authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let input_label = label("app-role-input");
    let output_label = label("app-role-output");
    let input_nonce_key = host::acl_nonce_key(acl_domain_key, authority, input_label);
    let output_nonce_key = host::acl_nonce_key(acl_domain_key, app_authority, output_label);
    let handle = handle_for_chain(192, 5);
    let (acl_record, acl_account) = acl_record_account_with_subject_role(
        input_nonce_key,
        0,
        acl_domain_key,
        authority,
        input_label,
        handle,
        authority,
        host::ACL_ROLE_USE,
    );
    let output_acl_record = host::acl_record_address(output_nonce_key, 0).0;
    let context = mollusk_eval_context(
        authority,
        vec![
            host_config_account(authority),
            (acl_record, acl_account),
            (output_acl_record, system_account(0)),
            (
                app_authority,
                Account {
                    lamports: 1_000_000,
                    data: vec![1],
                    owner: Pubkey::new_unique(),
                    executable: false,
                    rent_epoch: 0,
                },
            ),
        ],
    );
    let context_id = label("app-public-role-frame");
    let rhs = amount_plaintext(5);
    let mut ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: authority,
            compute_subject: authority,
            app_account_authority: authority,
            host_config,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                steps: vec![FheEvalStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheEvalOperand::AllowedDurable {
                        handle,
                        acl_record_index: 0,
                        permission_index: None,
                    },
                    rhs: FheEvalOperand::Scalar(rhs),
                    output_fhe_type: 5,
                    output: FheEvalOutput::AllowedDurable {
                        output_acl_record_index: 1,
                        output_app_account_authority_index: Some(2),
                        output_nonce_key,
                        output_nonce_sequence: 0,
                        output_acl_domain_key: acl_domain_key,
                        output_app_account: app_authority,
                        output_encrypted_value_label: output_label,
                        output_subjects: vec![AclSubjectEntry::user(authority)],
                        output_public_decrypt: false,
                    },
                }],
            },
        },
    );
    ix.accounts
        .push(AccountMeta::new_readonly(acl_record, false));
    ix.accounts.push(AccountMeta::new(output_acl_record, false));
    ix.accounts
        .push(AccountMeta::new_readonly(app_authority, true));

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_ok());
    let output_acl = read_acl_record(&context, output_acl_record).unwrap();
    assert!(!output_acl.public_decrypt);
    assert_eq!(output_acl.subject_count, 1);
    assert_eq!(output_acl.subjects[0], authority);
    assert_eq!(output_acl.subject_roles[0], host::ACL_ROLE_USER);
}

#[test]
fn mollusk_fhe_eval_composes_transient_binary_ops_into_durable_ternary_output() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let balance_label = label("balance-v2");
    let amount_label = label("amount-v2");
    let balance_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, balance_label);
    let amount_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, amount_label);
    let balance_handle = handle_for_chain(41, 5);
    let amount_handle = handle_for_chain(42, 5);
    let (balance_acl_record, balance_acl_account) = authorizing_acl_record_account(
        balance_nonce_key,
        0,
        acl_domain_key,
        app_account,
        balance_label,
        balance_handle,
        authority,
    );
    let (amount_acl_record, amount_acl_account) = authorizing_acl_record_account(
        amount_nonce_key,
        0,
        acl_domain_key,
        app_account,
        amount_label,
        amount_handle,
        authority,
    );
    let output_acl_record = host::acl_record_address(balance_nonce_key, 1).0;
    let context = mollusk_eval_context(
        authority,
        vec![
            host_config_account(authority),
            (balance_acl_record, balance_acl_account),
            (amount_acl_record, amount_acl_account),
            (output_acl_record, system_account(0)),
        ],
    );
    let context_id = label("eval-v2-mixed");
    let success_handle = current_eval_handle(
        &context.mollusk,
        FheBinaryOpCode::Ge,
        balance_handle,
        amount_handle,
        false,
        0,
        context_id,
        0,
    );
    let debit_candidate_handle = current_eval_handle(
        &context.mollusk,
        FheBinaryOpCode::Sub,
        balance_handle,
        amount_handle,
        false,
        5,
        context_id,
        1,
    );
    let output_handle = current_bound_eval_ternary_handle(
        &context.mollusk,
        FheTernaryOpCode::IfThenElse,
        success_handle,
        debit_candidate_handle,
        balance_handle,
        5,
        context_id,
        2,
        balance_nonce_key,
        1,
    );
    let mut ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: authority,
            compute_subject: authority,
            app_account_authority: app_account,
            host_config,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                steps: vec![
                    FheEvalStep::Binary {
                        op: FheBinaryOpCode::Ge,
                        lhs: FheEvalOperand::AllowedDurable {
                            handle: balance_handle,
                            acl_record_index: 0,
                            permission_index: None,
                        },
                        rhs: FheEvalOperand::AllowedDurable {
                            handle: amount_handle,
                            acl_record_index: 1,
                            permission_index: None,
                        },
                        output_fhe_type: 0,
                        output: FheEvalOutput::AllowedLocal,
                    },
                    FheEvalStep::Binary {
                        op: FheBinaryOpCode::Sub,
                        lhs: FheEvalOperand::AllowedDurable {
                            handle: balance_handle,
                            acl_record_index: 0,
                            permission_index: None,
                        },
                        rhs: FheEvalOperand::AllowedDurable {
                            handle: amount_handle,
                            acl_record_index: 1,
                            permission_index: None,
                        },
                        output_fhe_type: 5,
                        output: FheEvalOutput::AllowedLocal,
                    },
                    FheEvalStep::Ternary {
                        op: FheTernaryOpCode::IfThenElse,
                        control: FheEvalOperand::AllowedLocal { producer_index: 0 },
                        if_true: FheEvalOperand::AllowedLocal { producer_index: 1 },
                        if_false: FheEvalOperand::AllowedDurable {
                            handle: balance_handle,
                            acl_record_index: 0,
                            permission_index: None,
                        },
                        output_fhe_type: 5,
                        output: FheEvalOutput::AllowedDurable {
                            output_acl_record_index: 2,
                            output_app_account_authority_index: None,
                            output_nonce_key: balance_nonce_key,
                            output_nonce_sequence: 1,
                            output_acl_domain_key: acl_domain_key,
                            output_app_account: app_account,
                            output_encrypted_value_label: balance_label,
                            output_subjects: vec![AclSubjectEntry::user(authority)],
                            output_public_decrypt: false,
                        },
                    },
                ],
            },
        },
    );
    ix.accounts
        .push(AccountMeta::new_readonly(balance_acl_record, false));
    ix.accounts
        .push(AccountMeta::new_readonly(amount_acl_record, false));
    ix.accounts.push(AccountMeta::new(output_acl_record, false));

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_ok());
    let output_record =
        read_acl_record(&context, output_acl_record).expect("expected eval output ACL");
    assert_bound_acl_record(
        &output_record,
        output_handle,
        balance_nonce_key,
        1,
        acl_domain_key,
        app_account,
        balance_label,
        authority,
        host::ACL_ROLE_USER,
        context.mollusk.sysvars.clock.slot,
    );
    let binary_events: Vec<host::events::FheBinaryOpEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    let ternary_events: Vec<host::events::FheTernaryOpEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(binary_events.len(), 2);
    assert_eq!(binary_events[0].op, FheBinaryOpCode::Ge);
    assert_eq!(binary_events[0].lhs, balance_handle);
    assert_eq!(binary_events[0].rhs, amount_handle);
    assert_eq!(binary_events[0].result, success_handle);
    assert_eq!(binary_events[1].op, FheBinaryOpCode::Sub);
    assert_eq!(binary_events[1].lhs, balance_handle);
    assert_eq!(binary_events[1].rhs, amount_handle);
    assert_eq!(binary_events[1].result, debit_candidate_handle);
    assert_eq!(ternary_events.len(), 1);
    assert_eq!(ternary_events[0].op, FheTernaryOpCode::IfThenElse);
    assert_eq!(ternary_events[0].control, success_handle);
    assert_eq!(ternary_events[0].if_true, debit_candidate_handle);
    assert_eq!(ternary_events[0].if_false, balance_handle);
    assert_eq!(ternary_events[0].result, output_handle);
}

#[test]
fn mollusk_fhe_eval_identical_steps_derive_unique_transient_handles() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let input_label = label("eval-repeat-input");
    let input_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, input_label);
    let input_handle = handle_for_chain(192, 5);
    let (input_acl_record, input_acl_account) = authorizing_acl_record_account(
        input_nonce_key,
        0,
        acl_domain_key,
        app_account,
        input_label,
        input_handle,
        authority,
    );
    let context = mollusk_eval_context(
        authority,
        vec![
            host_config_account(authority),
            (input_acl_record, input_acl_account),
        ],
    );
    let context_id = label("eval-repeat-frame");
    let rhs = amount_plaintext(11);
    let first = current_eval_handle(
        &context.mollusk,
        FheBinaryOpCode::Add,
        input_handle,
        rhs,
        true,
        5,
        context_id,
        0,
    );
    let second = current_eval_handle(
        &context.mollusk,
        FheBinaryOpCode::Add,
        input_handle,
        rhs,
        true,
        5,
        context_id,
        1,
    );
    let third = current_eval_handle(
        &context.mollusk,
        FheBinaryOpCode::Add,
        input_handle,
        rhs,
        true,
        5,
        context_id,
        2,
    );
    assert_ne!(first, second);
    assert_ne!(first, third);
    assert_ne!(second, third);
    let expected = [first, second, third];
    let steps = (0..3)
        .map(|_| FheEvalStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheEvalOperand::AllowedDurable {
                handle: input_handle,
                acl_record_index: 0,
                permission_index: None,
            },
            rhs: FheEvalOperand::Scalar(rhs),
            output_fhe_type: 5,
            output: FheEvalOutput::AllowedLocal,
        })
        .collect();
    let mut ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: authority,
            compute_subject: authority,
            app_account_authority: app_account,
            host_config,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs { context_id, steps },
        },
    );
    ix.accounts
        .push(AccountMeta::new_readonly(input_acl_record, false));

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_ok());
    let binary_events: Vec<host::events::FheBinaryOpEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(binary_events.len(), expected.len());
    for (index, event) in binary_events.iter().enumerate() {
        assert_eq!(event.op, FheBinaryOpCode::Add);
        assert_eq!(event.lhs, input_handle);
        assert_eq!(event.rhs, rhs);
        assert!(event.scalar);
        assert_eq!(event.result, expected[index]);
    }
}

#[test]
fn mollusk_fhe_eval_binds_multiple_durable_outputs_with_distinct_authorities() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let secondary_authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let primary_label = label("eval-primary-out");
    let secondary_label = label("eval-secondary-out");
    let input_label = label("eval-multi-input");
    let input_nonce_key = host::acl_nonce_key(acl_domain_key, authority, input_label);
    let primary_nonce_key = host::acl_nonce_key(acl_domain_key, authority, primary_label);
    let secondary_nonce_key =
        host::acl_nonce_key(acl_domain_key, secondary_authority, secondary_label);
    let input_handle = handle_for_chain(151, 5);
    let (input_acl_record, input_acl_account) = authorizing_acl_record_account(
        input_nonce_key,
        0,
        acl_domain_key,
        authority,
        input_label,
        input_handle,
        authority,
    );
    let primary_output_acl = host::acl_record_address(primary_nonce_key, 0).0;
    let secondary_output_acl = host::acl_record_address(secondary_nonce_key, 0).0;
    let context = mollusk_eval_context(
        authority,
        vec![
            host_config_account(authority),
            (input_acl_record, input_acl_account),
            (primary_output_acl, system_account(0)),
            (secondary_authority, system_account(1_000_000)),
            (secondary_output_acl, system_account(0)),
        ],
    );
    let context_id = label("eval-two-auth");
    let rhs_add = amount_plaintext(3);
    let rhs_sub = amount_plaintext(1);
    let primary_handle = current_bound_eval_handle(
        &context.mollusk,
        FheBinaryOpCode::Add,
        input_handle,
        rhs_add,
        true,
        5,
        context_id,
        0,
        primary_nonce_key,
        0,
    );
    let secondary_handle = current_bound_eval_handle(
        &context.mollusk,
        FheBinaryOpCode::Sub,
        input_handle,
        rhs_sub,
        true,
        5,
        context_id,
        1,
        secondary_nonce_key,
        0,
    );
    let mut ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: authority,
            compute_subject: authority,
            app_account_authority: authority,
            host_config,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                steps: vec![
                    FheEvalStep::Binary {
                        op: FheBinaryOpCode::Add,
                        lhs: FheEvalOperand::AllowedDurable {
                            handle: input_handle,
                            acl_record_index: 0,
                            permission_index: None,
                        },
                        rhs: FheEvalOperand::Scalar(rhs_add),
                        output_fhe_type: 5,
                        output: FheEvalOutput::AllowedDurable {
                            output_acl_record_index: 1,
                            output_app_account_authority_index: None,
                            output_nonce_key: primary_nonce_key,
                            output_nonce_sequence: 0,
                            output_acl_domain_key: acl_domain_key,
                            output_app_account: authority,
                            output_encrypted_value_label: primary_label,
                            output_subjects: vec![AclSubjectEntry::user(authority)],
                            output_public_decrypt: false,
                        },
                    },
                    FheEvalStep::Binary {
                        op: FheBinaryOpCode::Sub,
                        lhs: FheEvalOperand::AllowedDurable {
                            handle: input_handle,
                            acl_record_index: 0,
                            permission_index: None,
                        },
                        rhs: FheEvalOperand::Scalar(rhs_sub),
                        output_fhe_type: 5,
                        output: FheEvalOutput::AllowedDurable {
                            output_acl_record_index: 3,
                            output_app_account_authority_index: Some(2),
                            output_nonce_key: secondary_nonce_key,
                            output_nonce_sequence: 0,
                            output_acl_domain_key: acl_domain_key,
                            output_app_account: secondary_authority,
                            output_encrypted_value_label: secondary_label,
                            output_subjects: vec![AclSubjectEntry::user(secondary_authority)],
                            output_public_decrypt: false,
                        },
                    },
                ],
            },
        },
    );
    ix.accounts
        .push(AccountMeta::new_readonly(input_acl_record, false));
    ix.accounts
        .push(AccountMeta::new(primary_output_acl, false));
    ix.accounts
        .push(AccountMeta::new_readonly(secondary_authority, true));
    ix.accounts
        .push(AccountMeta::new(secondary_output_acl, false));

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_ok());
    let primary_record =
        read_acl_record(&context, primary_output_acl).expect("expected primary output ACL");
    assert_bound_acl_record(
        &primary_record,
        primary_handle,
        primary_nonce_key,
        0,
        acl_domain_key,
        authority,
        primary_label,
        authority,
        host::ACL_ROLE_USER,
        context.mollusk.sysvars.clock.slot,
    );
    let secondary_record =
        read_acl_record(&context, secondary_output_acl).expect("expected secondary output ACL");
    assert_bound_acl_record(
        &secondary_record,
        secondary_handle,
        secondary_nonce_key,
        0,
        acl_domain_key,
        secondary_authority,
        secondary_label,
        secondary_authority,
        host::ACL_ROLE_USER,
        context.mollusk.sysvars.clock.slot,
    );
}

#[test]
fn mollusk_fhe_eval_rolls_back_first_durable_output_when_second_output_exists() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let input_label = label("eval-rollback-input");
    let first_output_label = label("eval-rollback-one");
    let second_output_label = label("eval-rollback-two");
    let input_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, input_label);
    let first_output_nonce_key =
        host::acl_nonce_key(acl_domain_key, app_account, first_output_label);
    let second_output_nonce_key =
        host::acl_nonce_key(acl_domain_key, app_account, second_output_label);
    let input_handle = handle_for_chain(181, 5);
    let stale_second_handle = handle_for_chain(182, 5);
    let (input_acl_record, input_acl_account) = authorizing_acl_record_account(
        input_nonce_key,
        0,
        acl_domain_key,
        app_account,
        input_label,
        input_handle,
        authority,
    );
    let first_output_acl = host::acl_record_address(first_output_nonce_key, 0).0;
    let (second_output_acl, second_output_account) = authorizing_acl_record_account(
        second_output_nonce_key,
        0,
        acl_domain_key,
        app_account,
        second_output_label,
        stale_second_handle,
        authority,
    );
    let context = mollusk_eval_context(
        authority,
        vec![
            host_config_account(authority),
            (input_acl_record, input_acl_account),
            (first_output_acl, system_account(0)),
            (second_output_acl, second_output_account.clone()),
        ],
    );
    let second_output_before = stored_account(&context, second_output_acl);
    let context_id = label("eval-rollback-frame");
    let rhs_first = amount_plaintext(4);
    let rhs_second = amount_plaintext(9);
    let mut ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: authority,
            compute_subject: authority,
            app_account_authority: app_account,
            host_config,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                steps: vec![
                    FheEvalStep::Binary {
                        op: FheBinaryOpCode::Add,
                        lhs: FheEvalOperand::AllowedDurable {
                            handle: input_handle,
                            acl_record_index: 0,
                            permission_index: None,
                        },
                        rhs: FheEvalOperand::Scalar(rhs_first),
                        output_fhe_type: 5,
                        output: FheEvalOutput::AllowedDurable {
                            output_acl_record_index: 1,
                            output_app_account_authority_index: None,
                            output_nonce_key: first_output_nonce_key,
                            output_nonce_sequence: 0,
                            output_acl_domain_key: acl_domain_key,
                            output_app_account: app_account,
                            output_encrypted_value_label: first_output_label,
                            output_subjects: vec![AclSubjectEntry::user(authority)],
                            output_public_decrypt: false,
                        },
                    },
                    FheEvalStep::Binary {
                        op: FheBinaryOpCode::Sub,
                        lhs: FheEvalOperand::AllowedDurable {
                            handle: input_handle,
                            acl_record_index: 0,
                            permission_index: None,
                        },
                        rhs: FheEvalOperand::Scalar(rhs_second),
                        output_fhe_type: 5,
                        output: FheEvalOutput::AllowedDurable {
                            output_acl_record_index: 2,
                            output_app_account_authority_index: None,
                            output_nonce_key: second_output_nonce_key,
                            output_nonce_sequence: 0,
                            output_acl_domain_key: acl_domain_key,
                            output_app_account: app_account,
                            output_encrypted_value_label: second_output_label,
                            output_subjects: vec![AclSubjectEntry::user(authority)],
                            output_public_decrypt: false,
                        },
                    },
                ],
            },
        },
    );
    ix.accounts
        .push(AccountMeta::new_readonly(input_acl_record, false));
    ix.accounts.push(AccountMeta::new(first_output_acl, false));
    ix.accounts.push(AccountMeta::new(second_output_acl, false));

    let result = context.process_instruction(&ix);

    assert_instruction_custom_error(
        &result,
        host::errors::ZamaHostError::FheEvalOutputAlreadyInitialized,
    );
    assert!(read_acl_record(&context, first_output_acl).is_none());
    assert_account_unchanged(
        &stored_account(&context, second_output_acl),
        &second_output_before,
    );
}

#[test]
fn mollusk_fhe_eval_rejects_duplicate_durable_output_reference_without_partial_birth() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let input_label = label("eval-alias-input");
    let output_label = label("eval-alias-output");
    let input_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, input_label);
    let output_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, output_label);
    let input_handle = handle_for_chain(183, 5);
    let (input_acl_record, input_acl_account) = authorizing_acl_record_account(
        input_nonce_key,
        0,
        acl_domain_key,
        app_account,
        input_label,
        input_handle,
        authority,
    );
    let output_acl = host::acl_record_address(output_nonce_key, 0).0;
    let context = mollusk_eval_context(
        authority,
        vec![
            host_config_account(authority),
            (input_acl_record, input_acl_account),
            (output_acl, system_account(0)),
        ],
    );
    let context_id = label("eval-alias-frame");
    let rhs = amount_plaintext(5);
    let mut ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: authority,
            compute_subject: authority,
            app_account_authority: app_account,
            host_config,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                steps: vec![
                    FheEvalStep::Binary {
                        op: FheBinaryOpCode::Add,
                        lhs: FheEvalOperand::AllowedDurable {
                            handle: input_handle,
                            acl_record_index: 0,
                            permission_index: None,
                        },
                        rhs: FheEvalOperand::Scalar(rhs),
                        output_fhe_type: 5,
                        output: FheEvalOutput::AllowedDurable {
                            output_acl_record_index: 1,
                            output_app_account_authority_index: None,
                            output_nonce_key,
                            output_nonce_sequence: 0,
                            output_acl_domain_key: acl_domain_key,
                            output_app_account: app_account,
                            output_encrypted_value_label: output_label,
                            output_subjects: vec![AclSubjectEntry::user(authority)],
                            output_public_decrypt: false,
                        },
                    },
                    FheEvalStep::Binary {
                        op: FheBinaryOpCode::Sub,
                        lhs: FheEvalOperand::AllowedDurable {
                            handle: input_handle,
                            acl_record_index: 0,
                            permission_index: None,
                        },
                        rhs: FheEvalOperand::Scalar(rhs),
                        output_fhe_type: 5,
                        output: FheEvalOutput::AllowedDurable {
                            output_acl_record_index: 1,
                            output_app_account_authority_index: None,
                            output_nonce_key,
                            output_nonce_sequence: 0,
                            output_acl_domain_key: acl_domain_key,
                            output_app_account: app_account,
                            output_encrypted_value_label: output_label,
                            output_subjects: vec![AclSubjectEntry::user(authority)],
                            output_public_decrypt: false,
                        },
                    },
                ],
            },
        },
    );
    ix.accounts
        .push(AccountMeta::new_readonly(input_acl_record, false));
    ix.accounts.push(AccountMeta::new(output_acl, false));

    let result = context.process_instruction(&ix);

    assert_instruction_custom_error(
        &result,
        host::errors::ZamaHostError::FheEvalOutputAlreadyInitialized,
    );
    assert!(read_acl_record(&context, output_acl).is_none());
}

#[test]
fn mollusk_fhe_eval_rejects_swapped_dynamic_account_ordering() {
    let fixture = EvalFixture::new();
    let context_id = label("eval-ordering");
    let mut ix = fixture.standard_instruction(
        context_id,
        fixture.success_steps(context_id, fixture.app_account, false),
    );
    let remaining_start = ix.accounts.len() - 3;
    ix.accounts[remaining_start] = AccountMeta::new_readonly(fixture.output_acl_record, false);
    ix.accounts[remaining_start + 2] = AccountMeta::new(fixture.balance_acl_record, false);

    let result = fixture.context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    fixture.assert_no_output();
    let balance_record = read_acl_record(&fixture.context, fixture.balance_acl_record)
        .expect("expected balance ACL");
    assert_eq!(balance_record.handle, fixture.balance_handle);
}

#[test]
fn mollusk_fhe_eval_switches_event_transport_above_cpi_threshold() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let input_label = label("eval-transport-input");
    let input_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, input_label);
    let input_handle = handle_for_chain(186, 5);
    let (input_acl_record, input_acl_account) = authorizing_acl_record_account(
        input_nonce_key,
        0,
        acl_domain_key,
        app_account,
        input_label,
        input_handle,
        authority,
    );
    let context = mollusk_eval_context(
        authority,
        vec![
            host_config_account(authority),
            (input_acl_record, input_acl_account),
        ],
    );
    let build_ix = |context_id: [u8; 32], step_count: usize| {
        let steps = (0..step_count)
            .map(|index| FheEvalStep::Binary {
                op: FheBinaryOpCode::Add,
                lhs: FheEvalOperand::AllowedDurable {
                    handle: input_handle,
                    acl_record_index: 0,
                    permission_index: None,
                },
                rhs: FheEvalOperand::Scalar(amount_plaintext(index as u64 + 1)),
                output_fhe_type: 5,
                output: FheEvalOutput::AllowedLocal,
            })
            .collect();
        let mut ix = anchor_ix(
            program_id,
            host::accounts::FheEval {
                payer: authority,
                compute_subject: authority,
                app_account_authority: app_account,
                host_config,
                system_program: system_program::ID,
                event_authority: event_authority(program_id),
                program: program_id,
            },
            host::instruction::FheEval {
                args: FheEvalArgs { context_id, steps },
            },
        );
        ix.accounts
            .push(AccountMeta::new_readonly(input_acl_record, false));
        ix
    };

    let cpi_result = context.process_instruction(&build_ix(label("eval-cpi-transport"), 8));
    let log_result = context.process_instruction(&build_ix(label("eval-log-transport"), 9));

    assert!(cpi_result.raw_result.is_ok());
    let cpi_binary_events: Vec<host::events::FheBinaryOpEvent> = cpi_result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(cpi_binary_events.len(), 8);

    assert!(log_result.raw_result.is_ok());
    let log_binary_events: Vec<host::events::FheBinaryOpEvent> = log_result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert!(log_binary_events.is_empty());
    assert!(log_result.inner_instructions.is_empty());
}

#[test]
fn mollusk_fhe_eval_rejects_missing_transient_producer() {
    let fixture = EvalFixture::new();
    let context_id = label("v2-missing-transient");
    let ix = fixture.standard_instruction(
        context_id,
        vec![FheEvalStep::Ternary {
            op: FheTernaryOpCode::IfThenElse,
            control: FheEvalOperand::AllowedLocal { producer_index: 0 },
            if_true: fixture.balance_operand(0),
            if_false: fixture.amount_operand(1),
            output_fhe_type: 5,
            output: fixture.durable_output(fixture.app_account, false),
        }],
    );

    let result = fixture.context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    fixture.assert_no_output();
}

#[test]
fn mollusk_fhe_eval_rejects_bad_ternary_control_type() {
    let fixture = EvalFixture::new();
    let context_id = label("v2-bad-control");
    let ix = fixture.standard_instruction(
        context_id,
        vec![FheEvalStep::Ternary {
            op: FheTernaryOpCode::IfThenElse,
            control: fixture.balance_operand(0),
            if_true: fixture.amount_operand(1),
            if_false: fixture.balance_operand(0),
            output_fhe_type: 5,
            output: fixture.durable_output(fixture.app_account, false),
        }],
    );

    let result = fixture.context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    fixture.assert_no_output();
}

#[test]
fn mollusk_fhe_eval_rejects_scalar_lhs_before_events() {
    let fixture = EvalFixture::new();
    let context_id = label("v2-scalar-lhs");
    let ix = anchor_ix(
        fixture.program_id,
        host::accounts::FheEval {
            payer: fixture.authority,
            compute_subject: fixture.authority,
            app_account_authority: fixture.app_account,
            host_config: fixture.host_config,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.program_id),
            program: fixture.program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                steps: vec![FheEvalStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheEvalOperand::Scalar(amount_plaintext(1)),
                    rhs: FheEvalOperand::Scalar(amount_plaintext(2)),
                    output_fhe_type: 5,
                    output: FheEvalOutput::AllowedLocal,
                }],
            },
        },
    );

    let result = fixture.context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    assert!(result.inner_instructions.is_empty());
    fixture.assert_no_output();
}

#[test]
fn mollusk_fhe_eval_rejects_binary_rhs_type_mismatch_before_output() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let lhs_label = label("eval-type-lhs");
    let rhs_label = label("eval-type-rhs");
    let lhs_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, lhs_label);
    let rhs_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, rhs_label);
    let lhs_handle = handle_for_chain(171, 5);
    let rhs_bool_handle = handle_for_chain(172, 0);
    let (lhs_acl_record, lhs_acl_account) = authorizing_acl_record_account(
        lhs_nonce_key,
        0,
        acl_domain_key,
        app_account,
        lhs_label,
        lhs_handle,
        authority,
    );
    let (rhs_acl_record, rhs_acl_account) = authorizing_acl_record_account(
        rhs_nonce_key,
        0,
        acl_domain_key,
        app_account,
        rhs_label,
        rhs_bool_handle,
        authority,
    );
    let output_acl_record = host::acl_record_address(lhs_nonce_key, 1).0;
    let context = mollusk_eval_context(
        authority,
        vec![
            host_config_account(authority),
            (lhs_acl_record, lhs_acl_account),
            (rhs_acl_record, rhs_acl_account),
            (output_acl_record, system_account(0)),
        ],
    );
    let context_id = label("eval-type-mismatch");
    let mut ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: authority,
            compute_subject: authority,
            app_account_authority: app_account,
            host_config,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                steps: vec![FheEvalStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheEvalOperand::AllowedDurable {
                        handle: lhs_handle,
                        acl_record_index: 0,
                        permission_index: None,
                    },
                    rhs: FheEvalOperand::AllowedDurable {
                        handle: rhs_bool_handle,
                        acl_record_index: 1,
                        permission_index: None,
                    },
                    output_fhe_type: 5,
                    output: FheEvalOutput::AllowedDurable {
                        output_acl_record_index: 2,
                        output_app_account_authority_index: None,
                        output_nonce_key: lhs_nonce_key,
                        output_nonce_sequence: 1,
                        output_acl_domain_key: acl_domain_key,
                        output_app_account: app_account,
                        output_encrypted_value_label: lhs_label,
                        output_subjects: vec![AclSubjectEntry::user(authority)],
                        output_public_decrypt: false,
                    },
                }],
            },
        },
    );
    ix.accounts
        .push(AccountMeta::new_readonly(lhs_acl_record, false));
    ix.accounts
        .push(AccountMeta::new_readonly(rhs_acl_record, false));
    ix.accounts.push(AccountMeta::new(output_acl_record, false));

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    assert!(result.inner_instructions.is_empty());
    assert!(read_acl_record(&context, output_acl_record).is_none());
}

#[test]
fn mollusk_fhe_eval_rejects_output_app_account_mismatch() {
    let fixture = EvalFixture::new();
    let context_id = label("v2-app-mismatch");
    let wrong_app_account = Pubkey::new_unique();
    let ix = fixture.standard_instruction(
        context_id,
        fixture.success_steps(context_id, wrong_app_account, false),
    );

    let result = fixture.context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    fixture.assert_no_output();
}

#[test]
fn mollusk_fhe_eval_rejects_public_decrypt_at_birth() {
    let fixture = EvalFixture::new();
    let context_id = label("v2-public-at-birth");
    let ix = fixture.standard_instruction(
        context_id,
        fixture.success_steps(context_id, fixture.app_account, true),
    );

    let result = fixture.context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    fixture.assert_no_output();
}

struct EvalFixture {
    program_id: Pubkey,
    authority: Pubkey,
    host_config: Pubkey,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    balance_label: [u8; 32],
    balance_nonce_key: [u8; 32],
    balance_handle: [u8; 32],
    amount_handle: [u8; 32],
    balance_acl_record: Pubkey,
    amount_acl_record: Pubkey,
    output_acl_record: Pubkey,
    context: mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
}

impl EvalFixture {
    fn new() -> Self {
        Self::with_hcu_limits(0, 0)
    }

    fn with_hcu_limits(max_hcu_per_tx: u64, max_hcu_depth_per_tx: u64) -> Self {
        let program_id = host::id();
        let authority = Pubkey::new_unique();
        let host_config_pair =
            host_config_account_with_hcu_limits(authority, max_hcu_per_tx, max_hcu_depth_per_tx);
        let host_config = host_config_pair.0;
        let acl_domain_key = Pubkey::new_unique();
        let app_account = authority;
        let balance_label = label("balance-v2-fixture");
        let amount_label = label("amount-v2-fixture");
        let balance_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, balance_label);
        let amount_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, amount_label);
        let balance_handle = handle_for_chain(141, 5);
        let amount_handle = handle_for_chain(142, 5);
        let (balance_acl_record, balance_acl_account) = authorizing_acl_record_account(
            balance_nonce_key,
            0,
            acl_domain_key,
            app_account,
            balance_label,
            balance_handle,
            authority,
        );
        let (amount_acl_record, amount_acl_account) = authorizing_acl_record_account(
            amount_nonce_key,
            0,
            acl_domain_key,
            app_account,
            amount_label,
            amount_handle,
            authority,
        );
        let output_acl_record = host::acl_record_address(balance_nonce_key, 1).0;
        let context = mollusk_eval_context(
            authority,
            vec![
                host_config_pair,
                (balance_acl_record, balance_acl_account),
                (amount_acl_record, amount_acl_account),
                (output_acl_record, system_account(0)),
            ],
        );

        Self {
            program_id,
            authority,
            host_config,
            acl_domain_key,
            app_account,
            balance_label,
            balance_nonce_key,
            balance_handle,
            amount_handle,
            balance_acl_record,
            amount_acl_record,
            output_acl_record,
            context,
        }
    }

    fn balance_operand(&self, acl_record_index: u16) -> FheEvalOperand {
        FheEvalOperand::AllowedDurable {
            handle: self.balance_handle,
            acl_record_index,
            permission_index: None,
        }
    }

    fn amount_operand(&self, acl_record_index: u16) -> FheEvalOperand {
        FheEvalOperand::AllowedDurable {
            handle: self.amount_handle,
            acl_record_index,
            permission_index: None,
        }
    }

    fn durable_output(
        &self,
        output_app_account: Pubkey,
        output_public_decrypt: bool,
    ) -> FheEvalOutput {
        FheEvalOutput::AllowedDurable {
            output_acl_record_index: 2,
            output_app_account_authority_index: None,
            output_nonce_key: self.balance_nonce_key,
            output_nonce_sequence: 1,
            output_acl_domain_key: self.acl_domain_key,
            output_app_account,
            output_encrypted_value_label: self.balance_label,
            output_subjects: vec![AclSubjectEntry::user(self.authority)],
            output_public_decrypt,
        }
    }

    fn success_steps(
        &self,
        _context_id: [u8; 32],
        output_app_account: Pubkey,
        output_public_decrypt: bool,
    ) -> Vec<FheEvalStep> {
        vec![
            FheEvalStep::Binary {
                op: FheBinaryOpCode::Ge,
                lhs: self.balance_operand(0),
                rhs: self.amount_operand(1),
                output_fhe_type: 0,
                output: FheEvalOutput::AllowedLocal,
            },
            FheEvalStep::Binary {
                op: FheBinaryOpCode::Sub,
                lhs: self.balance_operand(0),
                rhs: self.amount_operand(1),
                output_fhe_type: 5,
                output: FheEvalOutput::AllowedLocal,
            },
            FheEvalStep::Ternary {
                op: FheTernaryOpCode::IfThenElse,
                control: FheEvalOperand::AllowedLocal { producer_index: 0 },
                if_true: FheEvalOperand::AllowedLocal { producer_index: 1 },
                if_false: self.balance_operand(0),
                output_fhe_type: 5,
                output: self.durable_output(output_app_account, output_public_decrypt),
            },
        ]
    }

    fn standard_instruction(&self, context_id: [u8; 32], steps: Vec<FheEvalStep>) -> Instruction {
        let mut ix = anchor_ix(
            self.program_id,
            host::accounts::FheEval {
                payer: self.authority,
                compute_subject: self.authority,
                app_account_authority: self.app_account,
                host_config: self.host_config,
                system_program: system_program::ID,
                event_authority: event_authority(self.program_id),
                program: self.program_id,
            },
            host::instruction::FheEval {
                args: FheEvalArgs { context_id, steps },
            },
        );
        ix.accounts
            .push(AccountMeta::new_readonly(self.balance_acl_record, false));
        ix.accounts
            .push(AccountMeta::new_readonly(self.amount_acl_record, false));
        ix.accounts
            .push(AccountMeta::new(self.output_acl_record, false));
        ix
    }

    fn assert_no_output(&self) {
        assert!(read_acl_record(&self.context, self.output_acl_record).is_none());
    }
}

#[test]
fn mollusk_fhe_binary_op_rejects_unexpected_remaining_account() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("extra-binary");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = input_handle_for_chain(31, 0, 5);
    let (lhs_acl_record, lhs_acl_account) = authorizing_acl_record_account(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        authority,
    );
    let dummy_rhs_account = Pubkey::new_unique();
    let unexpected_account = Pubkey::new_unique();
    let context = mollusk_eval_context(
        authority,
        vec![
            host_config_account(authority),
            (lhs_acl_record, lhs_acl_account),
            (dummy_rhs_account, system_account(1_000_000)),
            (unexpected_account, system_account(1_000_000)),
        ],
    );
    let rhs_scalar = amount_plaintext(4);
    let result_handle = current_binary_handle(
        &context.mollusk,
        FheBinaryOpCode::Add,
        lhs,
        rhs_scalar,
        true,
        5,
    );
    let mut ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOp {
            compute_subject: authority,
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record: dummy_rhs_account,
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
            result: result_handle,
        },
    );
    ix.accounts
        .push(AccountMeta::new_readonly(unexpected_account, false));

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    assert!(result.inner_instructions.is_empty());
    let acl_record = read_acl_record(&context, lhs_acl_record).expect("expected LHS ACL record");
    assert_eq!(acl_record.handle, lhs);
    assert_eq!(acl_record.subject_count, 1);
    assert!(acl_record.inline_subject_has_role(authority, host::ACL_ROLE_USE));
}

#[test]
fn mollusk_fhe_eval_rejects_unused_dynamic_accounts_without_events() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("eval-extra");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = handle_for_chain(32, 5);
    let (lhs_acl_record, lhs_acl_account) = authorizing_acl_record_account(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        authority,
    );
    let extra_account = Pubkey::new_unique();
    let output_acl_record = host::acl_record_address(nonce_key, 1).0;
    let context = mollusk_eval_context(
        authority,
        vec![
            host_config_account(authority),
            (lhs_acl_record, lhs_acl_account),
            (extra_account, system_account(1_000_000)),
            (output_acl_record, system_account(0)),
        ],
    );

    let build_eval_ix = |context_id: [u8; 32]| {
        let rhs = amount_plaintext(2);
        anchor_ix(
            program_id,
            host::accounts::FheEval {
                payer: authority,
                compute_subject: authority,
                app_account_authority: app_account,
                host_config,
                system_program: system_program::ID,
                event_authority: event_authority(program_id),
                program: program_id,
            },
            host::instruction::FheEval {
                args: FheEvalArgs {
                    context_id,
                    steps: vec![FheEvalStep::Binary {
                        op: FheBinaryOpCode::Add,
                        lhs: FheEvalOperand::AllowedDurable {
                            handle: lhs,
                            acl_record_index: 0,
                            permission_index: None,
                        },
                        rhs: FheEvalOperand::Scalar(rhs),
                        output_fhe_type: 5,
                        output: FheEvalOutput::AllowedLocal,
                    }],
                },
            },
        )
    };

    let mut extra_remaining_ix = build_eval_ix(label("mollusk-eval-extra"));
    extra_remaining_ix
        .accounts
        .push(AccountMeta::new_readonly(lhs_acl_record, false));
    extra_remaining_ix
        .accounts
        .push(AccountMeta::new_readonly(extra_account, false));
    let extra_result = context.process_instruction(&extra_remaining_ix);

    assert!(extra_result.raw_result.is_err());
    assert!(extra_result.inner_instructions.is_empty());
    let record_after_extra =
        read_acl_record(&context, lhs_acl_record).expect("expected LHS ACL record");
    assert_eq!(record_after_extra.handle, lhs);
    assert!(record_after_extra.inline_subject_has_role(authority, host::ACL_ROLE_ALL));

    let rhs = amount_plaintext(3);
    let mut durable_extra_ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: authority,
            compute_subject: authority,
            app_account_authority: app_account,
            host_config,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id: label("mollusk-eval-extra-out"),
                steps: vec![FheEvalStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheEvalOperand::AllowedDurable {
                        handle: lhs,
                        acl_record_index: 0,
                        permission_index: None,
                    },
                    rhs: FheEvalOperand::Scalar(rhs),
                    output_fhe_type: 5,
                    output: FheEvalOutput::AllowedDurable {
                        output_acl_record_index: 1,
                        output_app_account_authority_index: None,
                        output_nonce_key: nonce_key,
                        output_nonce_sequence: 1,
                        output_acl_domain_key: acl_domain_key,
                        output_app_account: app_account,
                        output_encrypted_value_label: encrypted_value_label,
                        output_subjects: vec![AclSubjectEntry::user(authority)],
                        output_public_decrypt: false,
                    },
                }],
            },
        },
    );
    durable_extra_ix
        .accounts
        .push(AccountMeta::new_readonly(lhs_acl_record, false));
    durable_extra_ix
        .accounts
        .push(AccountMeta::new(output_acl_record, false));
    durable_extra_ix
        .accounts
        .push(AccountMeta::new_readonly(extra_account, false));
    let durable_extra_result = context.process_instruction(&durable_extra_ix);

    assert!(durable_extra_result.raw_result.is_err());
    assert!(durable_extra_result.inner_instructions.is_empty());
    assert!(read_acl_record(&context, output_acl_record).is_none());
}

#[test]
fn mollusk_fhe_binary_op_scalar_rhs_rejects_unused_permission_witness() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("scalar-extra-rhs");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let lhs = handle_for_chain(33, 5);
    let (lhs_acl_record, lhs_acl_account) = authorizing_acl_record_account(
        nonce_key,
        0,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        lhs,
        authority,
    );
    let dummy_rhs_account = Pubkey::new_unique();
    let unused_rhs_permission = Pubkey::new_unique();
    let output_acl_record = host::acl_record_address(nonce_key, 1).0;
    let context = mollusk_eval_context(
        authority,
        vec![
            host_config_account(authority),
            (lhs_acl_record, lhs_acl_account),
            (dummy_rhs_account, system_account(1_000_000)),
            (unused_rhs_permission, system_account(1_000_000)),
            (output_acl_record, system_account(0)),
        ],
    );
    let rhs_scalar = amount_plaintext(7);
    let result_handle = current_binary_handle(
        &context.mollusk,
        FheBinaryOpCode::Add,
        lhs,
        rhs_scalar,
        true,
        5,
    );
    let scalar_ix = anchor_ix(
        program_id,
        host::accounts::FheBinaryOp {
            compute_subject: authority,
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record: dummy_rhs_account,
            rhs_permission_record: Some(unused_rhs_permission),
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheBinaryOp {
            op: FheBinaryOpCode::Add,
            lhs,
            rhs: rhs_scalar,
            scalar: true,
            output_fhe_type: 5,
            result: result_handle,
        },
    );

    let scalar_result = context.process_instruction(&scalar_ix);

    assert!(scalar_result.raw_result.is_err());
    assert!(scalar_result.inner_instructions.is_empty());
    let lhs_record = read_acl_record(&context, lhs_acl_record).expect("expected LHS ACL record");
    assert_eq!(lhs_record.handle, lhs);

    let bound_result = current_bound_binary_handle(
        &context.mollusk,
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
            payer: authority,
            compute_subject: authority,
            app_account_authority: app_account,
            host_config,
            lhs_acl_record,
            lhs_permission_record: None,
            rhs_acl_record: dummy_rhs_account,
            rhs_permission_record: Some(unused_rhs_permission),
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
            output_subjects: vec![AclSubjectEntry::user(authority)],
            output_public_decrypt: false,
        },
    );

    let bind_result = context.process_instruction(&bind_ix);

    assert!(bind_result.raw_result.is_err());
    assert!(read_acl_record(&context, output_acl_record).is_none());
    let output_account = stored_account(&context, output_acl_record);
    assert_eq!(output_account.owner, system_program::ID);
    assert!(output_account.data.is_empty());
}

#[test]
fn mollusk_fhe_binary_op_bind_rejects_public_decrypt_role_grant_without_input_role() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("balance");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = handle_for_chain(7, 5);
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
    let context = mollusk_eval_context(
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
            output_public_decrypt: false,
        },
    );

    let result = context.process_instruction(&ix);
    assert!(result.raw_result.is_err());
    assert!(read_acl_record(&context, output_acl_record).is_none());
}

#[test]
fn mollusk_fhe_ternary_op_bind_rejects_public_decrypt_role_grant_without_input_role() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let control_label = label("ternary-control");
    let true_label = label("ternary-true");
    let false_label = label("ternary-false");
    let output_label = label("ternary-output");
    let control_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, control_label);
    let true_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, true_label);
    let false_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, false_label);
    let output_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, output_label);
    let control = handle_for_chain(11, 0);
    let if_true = handle_for_chain(12, 5);
    let if_false = handle_for_chain(13, 5);
    let (control_acl_record, control_acl_account) = acl_record_account_with_subject_role(
        control_nonce_key,
        0,
        acl_domain_key,
        app_account,
        control_label,
        control,
        authority,
        host::ACL_ROLE_USE,
    );
    let (true_acl_record, true_acl_account) = acl_record_account_with_subject_role(
        true_nonce_key,
        0,
        acl_domain_key,
        app_account,
        true_label,
        if_true,
        authority,
        host::ACL_ROLE_USE,
    );
    let (false_acl_record, false_acl_account) = acl_record_account_with_subject_role(
        false_nonce_key,
        0,
        acl_domain_key,
        app_account,
        false_label,
        if_false,
        authority,
        host::ACL_ROLE_USE,
    );
    let output_acl_record = host::acl_record_address(output_nonce_key, 1).0;
    let context = mollusk_eval_context(
        authority,
        vec![
            host_config_account(authority),
            (control_acl_record, control_acl_account),
            (true_acl_record, true_acl_account),
            (false_acl_record, false_acl_account),
            (output_acl_record, system_account(0)),
        ],
    );
    let result_handle = host::computed_bound_ternary_handle(
        FheTernaryOpCode::IfThenElse,
        control,
        if_true,
        if_false,
        5,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash(&context.mollusk),
        context.mollusk.sysvars.clock.unix_timestamp,
        output_nonce_key,
        1,
    );
    let ix = anchor_ix(
        program_id,
        host::accounts::FheTernaryOpAndBindOutput {
            payer: authority,
            compute_subject: authority,
            app_account_authority: authority,
            host_config,
            control_acl_record,
            control_permission_record: None,
            if_true_acl_record: true_acl_record,
            if_true_permission_record: None,
            if_false_acl_record: false_acl_record,
            if_false_permission_record: None,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheTernaryOpAndBindOutput {
            op: FheTernaryOpCode::IfThenElse,
            control,
            if_true,
            if_false,
            output_fhe_type: 5,
            result: result_handle,
            output_nonce_key,
            output_nonce_sequence: 1,
            output_acl_domain_key: acl_domain_key,
            output_app_account: app_account,
            output_encrypted_value_label: output_label,
            output_subjects: vec![AclSubjectEntry::user(authority)],
            output_public_decrypt: false,
        },
    );

    let result = context.process_instruction(&ix);
    assert!(result.raw_result.is_err());
    assert!(read_acl_record(&context, output_acl_record).is_none());
}

fn serialized_account<T: AccountSerialize>(account: T) -> Vec<u8> {
    let mut data = Vec::new();
    account.try_serialize(&mut data).unwrap();
    data
}

// ---------------------------------------------------------------------------
// HCU limit setters + fhe_eval enforcement
// (INV-2/4/5/6/7/10/12/14/15/17/18 — see solana/artifacts/05-tests-hcu-limit.md).
// ---------------------------------------------------------------------------

#[test]
fn mollusk_set_max_hcu_limits_persist_and_advance_slot() {
    // INV-17 + happy path: enable both limits from the disabled (0) default; each write persists
    // and stamps updated_slot.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    // Depth first (total still 0 = unlimited, so ordering trivially holds).
    let depth = context.process_instruction(&set_max_hcu_depth_per_tx_ix(
        program_id,
        admin,
        host_config,
        5_000_000,
    ));
    assert!(depth.raw_result.is_ok());
    let config = read_host_config(&context, host_config).expect("config");
    assert_eq!(config.max_hcu_depth_per_tx, 5_000_000);
    assert_eq!(config.max_hcu_per_tx, 0);
    assert_eq!(config.updated_slot, context.mollusk.sysvars.clock.slot);

    // Then total (>= depth).
    let total = context.process_instruction(&set_max_hcu_per_tx_ix(
        program_id,
        admin,
        host_config,
        20_000_000,
    ));
    assert!(total.raw_result.is_ok());
    let config = read_host_config(&context, host_config).expect("config");
    assert_eq!(config.max_hcu_per_tx, 20_000_000);
    assert_eq!(config.max_hcu_depth_per_tx, 5_000_000);
}

#[test]
fn mollusk_set_max_hcu_per_tx_rejects_wrong_admin() {
    // INV-2 / INV-4: a valid signer that is not the stored admin is rejected; no mutation.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let wrong_admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    let result = context.process_instruction(&set_max_hcu_per_tx_ix(
        program_id,
        wrong_admin,
        host_config,
        1_000_000,
    ));
    assert_instruction_custom_error(
        &result,
        host::errors::ZamaHostError::HostConfigAdminMismatch,
    );
    let config = read_host_config(&context, host_config).expect("config");
    assert_eq!(config.max_hcu_per_tx, 0);
    assert_eq!(config.updated_slot, 0);
}

#[test]
fn mollusk_set_max_hcu_per_tx_rejects_remaining_accounts() {
    // INV-5: a trailing account meta is rejected; no mutation.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    let mut ix = set_max_hcu_per_tx_ix(program_id, admin, host_config, 1_000_000);
    ix.accounts
        .push(AccountMeta::new_readonly(Pubkey::new_unique(), false));
    let result = context.process_instruction(&ix);
    assert_instruction_custom_error(
        &result,
        host::errors::ZamaHostError::UnexpectedRemainingAccounts,
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .max_hcu_per_tx,
        0
    );
}

#[test]
fn mollusk_set_max_hcu_per_tx_rejects_below_depth() {
    // INV-6 / INV-15: with depth=5M set, a total of 4M would make the depth cap dead -> rejected.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account_with_hcu_limits(admin, 0, 5_000_000);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    let result = context.process_instruction(&set_max_hcu_per_tx_ix(
        program_id,
        admin,
        host_config,
        4_000_000,
    ));
    assert_instruction_custom_error(
        &result,
        host::errors::ZamaHostError::HcuLimitOrderingInvalid,
    );
    let config = read_host_config(&context, host_config).expect("config");
    assert_eq!(config.max_hcu_per_tx, 0);
    assert_eq!(config.max_hcu_depth_per_tx, 5_000_000);
}

#[test]
fn mollusk_set_max_hcu_depth_per_tx_rejects_above_total() {
    // INV-7 / INV-15: with total=20M set, a depth of 21M would exceed it -> rejected.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account_with_hcu_limits(admin, 20_000_000, 0);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    let result = context.process_instruction(&set_max_hcu_depth_per_tx_ix(
        program_id,
        admin,
        host_config,
        21_000_000,
    ));
    assert_instruction_custom_error(
        &result,
        host::errors::ZamaHostError::HcuLimitOrderingInvalid,
    );
    let config = read_host_config(&context, host_config).expect("config");
    assert_eq!(config.max_hcu_depth_per_tx, 0);
    assert_eq!(config.max_hcu_per_tx, 20_000_000);
}

#[test]
fn mollusk_initialize_host_config_defaults_hcu_limits_to_zero() {
    // INV-14: a freshly initialized config ships with both limits disabled.
    let program_id = host::id();
    let payer = Pubkey::new_unique();
    let admin = Pubkey::new_unique();
    let (host_config, _) = host::host_config_address();
    let args = host::InitializeHostConfigArgs {
        chain_id: host::SOLANA_POC_CHAIN_ID,
        input_verifier_authority: Pubkey::new_unique(),
        gateway_chain_id: 0,
        input_verification_contract: [0u8; 20],
        coprocessor_signer: [0u8; 20],
        decryption_contract: [0u8; 20],
        material_authority: Pubkey::new_unique(),
        test_authority: Pubkey::new_unique(),
        mock_input_enabled: false,
        test_shims_enabled: false,
        grant_deny_list_enabled: false,
    };
    let context = mollusk_eval_context(payer, vec![(host_config, system_account(0))]);

    let result = context.process_instruction(&initialize_host_config_ix(
        program_id,
        payer,
        admin,
        host_config,
        args,
    ));
    assert!(result.raw_result.is_ok());
    let config = read_host_config(&context, host_config).expect("config");
    assert_eq!(config.max_hcu_per_tx, 0);
    assert_eq!(config.max_hcu_depth_per_tx, 0);
}

#[test]
fn mollusk_fhe_eval_within_enabled_limits_succeeds() {
    // INV-10 / INV-12 happy + INV-24: limits enabled but generous -> the plan still succeeds and
    // binds its durable output.
    let fixture = EvalFixture::with_hcu_limits(1_000_000, 1_000_000);
    let context_id = label("hcu-within");
    let ix = fixture.standard_instruction(
        context_id,
        fixture.success_steps(context_id, fixture.app_account, false),
    );
    let result = fixture.context.process_instruction(&ix);
    assert!(
        result.raw_result.is_ok(),
        "expected success: {:?}",
        result.raw_result
    );
    assert!(read_acl_record(&fixture.context, fixture.output_acl_record).is_some());
}

#[test]
fn mollusk_fhe_eval_total_limit_exceeded_reverts_without_output() {
    // INV-10 + INV-18: a tiny total cap trips in admission; no durable output ACL record is created.
    let fixture = EvalFixture::with_hcu_limits(1, 0);
    let context_id = label("hcu-total-trip");
    let ix = fixture.standard_instruction(
        context_id,
        fixture.success_steps(context_id, fixture.app_account, false),
    );
    let result = fixture.context.process_instruction(&ix);
    assert_instruction_custom_error(
        &result,
        host::errors::ZamaHostError::HcuTransactionLimitExceeded,
    );
    fixture.assert_no_output();
}

#[test]
fn mollusk_fhe_eval_depth_limit_exceeded_reverts_without_output() {
    // INV-12 + INV-18: total unlimited (0), a tiny depth cap trips independently; no output created.
    let fixture = EvalFixture::with_hcu_limits(0, 1);
    let context_id = label("hcu-depth-trip");
    let ix = fixture.standard_instruction(
        context_id,
        fixture.success_steps(context_id, fixture.app_account, false),
    );
    let result = fixture.context.process_instruction(&ix);
    assert_instruction_custom_error(
        &result,
        host::errors::ZamaHostError::HcuTransactionDepthLimitExceeded,
    );
    fixture.assert_no_output();
}

fn mollusk() -> Mollusk {
    let deploy_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy");
    unsafe {
        std::env::set_var("SBF_OUT_DIR", deploy_dir);
    }
    Mollusk::new(&host::id(), "zama_host")
}

fn mollusk_eval_context(
    payer: Pubkey,
    seeded_accounts: Vec<(Pubkey, Account)>,
) -> mollusk_svm::MolluskContext<HashMap<Pubkey, Account>> {
    let mut accounts = HashMap::from([(payer, system_account(5_000_000_000))]);
    for (pubkey, account) in seeded_accounts {
        accounts.insert(pubkey, account);
    }
    mollusk().with_context(accounts)
}

fn assert_transaction_success(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    instructions: &[Instruction],
) -> TransactionResult {
    let result = process_transaction_result(context, instructions);
    assert!(
        result.raw_result.is_ok(),
        "transaction failed: {:?}",
        result.raw_result
    );
    result
}

fn assert_instruction_custom_error(result: &InstructionResult, error: host::errors::ZamaHostError) {
    let expected_code: u32 = error.into();
    assert_eq!(
        result.raw_result,
        Err(InstructionError::Custom(expected_code))
    );
}

fn process_transaction_result(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    instructions: &[Instruction],
) -> TransactionResult {
    let mut account_map: HashMap<Pubkey, Account> = context
        .account_store
        .borrow()
        .iter()
        .map(|(pubkey, account)| (*pubkey, account.clone()))
        .collect();
    account_map.insert(ed25519_program::ID, precompile_account());
    let program_ids = instructions
        .iter()
        .map(|instruction| instruction.program_id)
        .collect::<HashSet<_>>();
    for meta in instructions
        .iter()
        .flat_map(|instruction| instruction.accounts.iter())
    {
        if meta.pubkey == sysvar::instructions::ID
            || meta.pubkey == system_program::ID
            || meta.pubkey == ed25519_program::ID
            || program_ids.contains(&meta.pubkey)
        {
            continue;
        }
        account_map
            .entry(meta.pubkey)
            .or_insert_with(|| system_account(0));
    }
    let accounts: Vec<(Pubkey, Account)> = account_map.into_iter().collect();
    let result = context
        .mollusk
        .process_transaction_instructions(instructions, &accounts);
    if result.raw_result.is_ok() {
        let mut store = context.account_store.borrow_mut();
        for (pubkey, account) in result.resulting_accounts.iter() {
            store.insert(*pubkey, account.clone());
        }
    }
    result
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

fn assert_account_unchanged(actual: &Account, expected: &Account) {
    assert_eq!(actual.lamports, expected.lamports);
    assert_eq!(actual.data, expected.data);
    assert_eq!(actual.owner, expected.owner);
    assert_eq!(actual.executable, expected.executable);
    assert_eq!(actual.rent_epoch, expected.rent_epoch);
}

fn executable_program_account() -> Account {
    Account {
        lamports: 1_000_000_000,
        data: Vec::new(),
        owner: bpf_loader_upgradeable::ID,
        executable: true,
        rent_epoch: 0,
    }
}

fn precompile_account() -> Account {
    Account {
        lamports: 1,
        data: Vec::new(),
        owner: native_loader::ID,
        executable: true,
        rent_epoch: 0,
    }
}

fn host_config_account_with_extra_bytes(admin: Pubkey, extra_bytes: usize) -> (Pubkey, Account) {
    let (host_config, mut account) = host_config_account(admin);
    account.data.resize(account.data.len() + extra_bytes, 0);
    (host_config, account)
}

fn host_config_account(admin: Pubkey) -> (Pubkey, Account) {
    host_config_account_with_options(admin, admin, true, true, false)
}

fn host_config_account_with_grant_deny_list(
    admin: Pubkey,
    grant_deny_list_enabled: bool,
) -> (Pubkey, Account) {
    host_config_account_with_options(admin, admin, true, true, grant_deny_list_enabled)
}

/// Like [`host_config_account`] but with the two HCU limits pre-set (the only way to seed an
/// already-enabled config for the setter/eval enforcement tests).
fn host_config_account_with_hcu_limits(
    admin: Pubkey,
    max_hcu_per_tx: u64,
    max_hcu_depth_per_tx: u64,
) -> (Pubkey, Account) {
    let (key, mut account) = host_config_account(admin);
    let mut config = {
        let mut data = account.data.as_slice();
        HostConfig::try_deserialize(&mut data).expect("valid host config")
    };
    config.max_hcu_per_tx = max_hcu_per_tx;
    config.max_hcu_depth_per_tx = max_hcu_depth_per_tx;
    account.data = serialized_account(config);
    (key, account)
}

fn host_config_account_with_options(
    admin: Pubkey,
    input_verifier_authority: Pubkey,
    mock_input_enabled: bool,
    test_shims_enabled: bool,
    grant_deny_list_enabled: bool,
) -> (Pubkey, Account) {
    let (host_config, bump) = host::host_config_address();
    (
        host_config,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(HostConfig {
                admin,
                chain_id: host::SOLANA_POC_CHAIN_ID,
                input_verifier_authority,
                gateway_chain_id: 0,
                input_verification_contract: [0u8; 20],
                coprocessor_signer: [0u8; 20],
                decryption_contract: [0u8; 20],
                current_kms_context_id: 0,
                material_authority: admin,
                test_authority: admin,
                paused: false,
                mock_input_enabled,
                test_shims_enabled,
                grant_deny_list_enabled,
                max_hcu_per_tx: 0,
                max_hcu_depth_per_tx: 0,
                updated_slot: 0,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

fn deny_subject_record_account(subject: Pubkey, denied: bool) -> (Pubkey, Account) {
    let (deny_subject_record, bump) = host::deny_subject_address(subject);
    (
        deny_subject_record,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(DenySubjectRecord {
                subject,
                denied,
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
    acl_record_account_with_subjects(
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        &[AclSubjectEntry {
            pubkey: authority,
            role_flags,
        }],
    )
}

#[allow(clippy::too_many_arguments)]
fn acl_record_account_with_subjects(
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    entries: &[AclSubjectEntry],
) -> (Pubkey, Account) {
    assert!(entries.len() <= host::MAX_ACL_SUBJECTS);
    let (acl_record, bump) = host::acl_record_address(nonce_key, nonce_sequence);
    let mut subjects = [Pubkey::default(); host::MAX_ACL_SUBJECTS];
    let mut subject_roles = [0_u8; host::MAX_ACL_SUBJECTS];
    for (index, entry) in entries.iter().enumerate() {
        subjects[index] = entry.pubkey;
        subject_roles[index] = entry.role_flags;
    }
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
                subject_count: entries.len() as u8,
                overflow_subject_count: 0,
                public_decrypt: false,
                material_commitment: Pubkey::default(),
                material_commitment_hash: [0; 32],
                material_key_id: [0; 32],
                created_slot: 0,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn trivial_encrypt_and_bind_ix(
    program_id: Pubkey,
    authority: Pubkey,
    host_config: Pubkey,
    output_acl_record: Pubkey,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
    output_encrypted_value_label: [u8; 32],
    plaintext: [u8; 32],
    fhe_type: u8,
    output_public_decrypt: bool,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::TrivialEncryptAndBind {
            payer: authority,
            compute_subject: authority,
            app_account_authority: authority,
            host_config,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::TrivialEncryptAndBind {
            plaintext,
            fhe_type,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(authority)],
            output_public_decrypt,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn allow_acl_subjects_ix(
    program_id: Pubkey,
    payer: Pubkey,
    authority: Pubkey,
    authority_permission_record: Option<Pubkey>,
    host_config: Pubkey,
    acl_record: Pubkey,
    deny_subject_record: Option<Pubkey>,
    handle: [u8; 32],
    subjects: Vec<AclSubjectEntry>,
    overflow_permission_records: &[Pubkey],
) -> Instruction {
    let mut ix = anchor_ix(
        program_id,
        host::accounts::AllowAclSubjects {
            payer,
            authority,
            authority_permission_record,
            acl_record,
            host_config,
            deny_subject_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowAclSubjects { handle, subjects },
    );
    ix.accounts.extend(
        overflow_permission_records
            .iter()
            .copied()
            .map(|record| AccountMeta::new(record, false)),
    );
    ix
}

fn initialize_host_config_ix(
    program_id: Pubkey,
    payer: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    args: host::InitializeHostConfigArgs,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::InitializeHostConfig {
            payer,
            admin,
            host_config,
            system_program: system_program::ID,
        },
        host::instruction::InitializeHostConfig { args },
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

fn set_max_hcu_per_tx_ix(
    program_id: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    value: u64,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::HostAdmin { admin, host_config },
        host::instruction::SetMaxHcuPerTx { value },
    )
}

fn set_max_hcu_depth_per_tx_ix(
    program_id: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    value: u64,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::HostAdmin { admin, host_config },
        host::instruction::SetMaxHcuDepthPerTx { value },
    )
}

#[allow(clippy::too_many_arguments)]
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
fn rand_and_bind_ix(
    program_id: Pubkey,
    authority: Pubkey,
    host_config: Pubkey,
    output_acl_record: Pubkey,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
    output_encrypted_value_label: [u8; 32],
    fhe_type: u8,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::FheRandAndBind {
            payer: authority,
            compute_subject: authority,
            app_account_authority: authority,
            host_config,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheRandAndBind {
            fhe_type,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(authority)],
            output_public_decrypt: false,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn rand_bounded_and_bind_ix(
    program_id: Pubkey,
    authority: Pubkey,
    host_config: Pubkey,
    output_acl_record: Pubkey,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
    output_encrypted_value_label: [u8; 32],
    upper_bound: [u8; 32],
    fhe_type: u8,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::FheRandBoundedAndBind {
            payer: authority,
            compute_subject: authority,
            app_account_authority: authority,
            host_config,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheRandBoundedAndBind {
            upper_bound,
            fhe_type,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::user(authority)],
            output_public_decrypt: false,
        },
    )
}

fn delegate_for_user_decryption_ix(
    program_id: Pubkey,
    authority: Pubkey,
    host_config: Pubkey,
    delegation_record: Pubkey,
    delegate: Pubkey,
    app_account: Pubkey,
    expiration_slot: u64,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::DelegateForUserDecryption {
            payer: authority,
            delegator: authority,
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

fn revoke_delegation_for_user_decryption_ix(
    program_id: Pubkey,
    authority: Pubkey,
    host_config: Pubkey,
    delegation_record: Pubkey,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::RevokeDelegationForUserDecryption {
            delegator: authority,
            host_config,
            delegation_record,
        },
        host::instruction::RevokeDelegationForUserDecryption {},
    )
}

#[allow(clippy::too_many_arguments)]
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

fn allow_for_decryption_ix(
    program_id: Pubkey,
    authority: Pubkey,
    host_config: Pubkey,
    acl_record: Pubkey,
    handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::AllowForDecryption {
            authority,
            authority_permission_record: None,
            acl_record,
            host_config,
            deny_subject_record: None,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowForDecryption { handle },
    )
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
    let previous_bank_hash = previous_bank_hash(mollusk);
    host::computed_eval_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
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
    let previous_bank_hash = previous_bank_hash(mollusk);
    host::computed_bound_eval_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        mollusk.sysvars.clock.unix_timestamp,
        context_id,
        op_index,
        output_nonce_key,
        output_nonce_sequence,
    )
}

#[allow(clippy::too_many_arguments)]
fn current_bound_eval_ternary_handle(
    mollusk: &Mollusk,
    op: FheTernaryOpCode,
    control: [u8; 32],
    if_true: [u8; 32],
    if_false: [u8; 32],
    fhe_type: u8,
    context_id: [u8; 32],
    op_index: u16,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    let previous_bank_hash = previous_bank_hash(mollusk);
    host::computed_bound_eval_ternary_handle(
        op,
        control,
        if_true,
        if_false,
        fhe_type,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        mollusk.sysvars.clock.unix_timestamp,
        context_id,
        op_index,
        output_nonce_key,
        output_nonce_sequence,
    )
}

#[allow(clippy::too_many_arguments)]
fn current_binary_handle(
    mollusk: &Mollusk,
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
) -> [u8; 32] {
    host::computed_binary_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash(mollusk),
        mollusk.sysvars.clock.unix_timestamp,
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
    let previous_bank_hash = previous_bank_hash(mollusk);
    host::computed_bound_binary_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        mollusk.sysvars.clock.unix_timestamp,
        output_nonce_key,
        output_nonce_sequence,
    )
}

fn current_trivial_handle(
    mollusk: &Mollusk,
    plaintext: [u8; 32],
    fhe_type: u8,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    let previous_bank_hash = previous_bank_hash(mollusk);
    host::computed_trivial_handle(
        plaintext,
        fhe_type,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        mollusk.sysvars.clock.unix_timestamp,
        output_nonce_key,
        output_nonce_sequence,
    )
}

fn current_rand_seed(
    mollusk: &Mollusk,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 16] {
    let previous_bank_hash = previous_bank_hash(mollusk);
    host::computed_rand_seed(
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
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

fn read_delegation_record(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<UserDecryptionDelegation> {
    let store = context.account_store.borrow();
    let account = store.get(&address)?;
    if account.owner != host::id() {
        return None;
    }
    let mut data = account.data.as_slice();
    UserDecryptionDelegation::try_deserialize(&mut data).ok()
}

fn read_material_commitment(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<HandleMaterialCommitment> {
    let store = context.account_store.borrow();
    let account = store.get(&address)?;
    if account.owner != host::id() {
        return None;
    }
    let mut data = account.data.as_slice();
    HandleMaterialCommitment::try_deserialize(&mut data).ok()
}

fn read_acl_permission(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<AclPermission> {
    let store = context.account_store.borrow();
    let account = store.get(&address)?;
    if account.owner != host::id() {
        return None;
    }
    let mut data = account.data.as_slice();
    AclPermission::try_deserialize(&mut data).ok()
}

fn read_deny_subject_record(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<DenySubjectRecord> {
    let store = context.account_store.borrow();
    let account = store.get(&address)?;
    if account.owner != host::id() {
        return None;
    }
    let mut data = account.data.as_slice();
    DenySubjectRecord::try_deserialize(&mut data).ok()
}

fn stored_account(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Account {
    context
        .account_store
        .borrow()
        .get(&address)
        .cloned()
        .expect("expected stored account")
}

fn read_host_config(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<HostConfig> {
    let store = context.account_store.borrow();
    let account = store.get(&address)?;
    if account.owner != host::id() {
        return None;
    }
    let mut data = account.data.as_slice();
    HostConfig::try_deserialize(&mut data).ok()
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

#[allow(clippy::too_many_arguments)]
fn assert_bound_acl_record(
    record: &AclRecord,
    handle: [u8; 32],
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    subject: Pubkey,
    role_flags: u8,
    created_slot: u64,
) {
    assert_eq!(record.handle, handle);
    assert_eq!(record.nonce_key, nonce_key);
    assert_eq!(record.nonce_sequence, nonce_sequence);
    assert_eq!(record.acl_domain_key, acl_domain_key);
    assert_eq!(record.app_account, app_account);
    assert_eq!(record.encrypted_value_label, encrypted_value_label);
    assert_eq!(record.subject_count, 1);
    assert_eq!(record.overflow_subject_count, 0);
    let subject_index = record
        .inline_subject_index(subject)
        .expect("subject should be stored inline");
    assert_eq!(record.subject_roles[subject_index], role_flags);
    assert!(!record.public_decrypt);
    assert_eq!(record.material_commitment, Pubkey::default());
    assert_eq!(record.material_commitment_hash, [0; 32]);
    assert_eq!(record.material_key_id, [0; 32]);
    assert_eq!(record.created_slot, created_slot);
}

#[allow(clippy::too_many_arguments)]
fn assert_random_acl_record(
    record: &AclRecord,
    handle: [u8; 32],
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    subject: Pubkey,
    created_slot: u64,
) {
    assert_bound_acl_record(
        record,
        handle,
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        subject,
        host::ACL_ROLE_USER,
        created_slot,
    );
}

fn assert_single_acl_allowed_event(
    result: &mollusk_svm::result::InstructionResult,
    handle: [u8; 32],
    subject: Pubkey,
) {
    let events = acl_allowed_events(result);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].version, host::EVENT_VERSION);
    assert_eq!(events[0].handle, handle);
    assert_eq!(events[0].subject, subject.to_bytes());
}

fn acl_allowed_events(result: &mollusk_svm::result::InstructionResult) -> Vec<AclAllowedEvent> {
    result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect()
}

// --- Coprocessor EIP-712 input-bind (secp256k1_recover) — #1494 Phase 3 ---

const GATEWAY_CHAIN_ID: u64 = 31337;
const INPUT_VERIFICATION_CONTRACT: [u8; 20] = [0xCDu8; 20];
const DECRYPTION_CONTRACT: [u8; 20] = [0xDEu8; 20];

/// Recovers the EVM address (keccak(pubkey)[12..]) for a coprocessor signing key,
/// matching the on-chain `secp256k1_recover` derivation.
fn evm_address_of(key: &k256::ecdsa::SigningKey) -> [u8; 20] {
    let encoded = key.verifying_key().to_encoded_point(false); // 0x04 || X || Y
    let hash = solana_program::keccak::hash(&encoded.as_bytes()[1..]).to_bytes();
    let mut address = [0u8; 20];
    address.copy_from_slice(&hash[12..]);
    address
}

/// Produces a 65-byte `[r || s || v]` recoverable signature over an EIP-712 digest.
fn sign_eip712(key: &k256::ecdsa::SigningKey, digest: &[u8; 32]) -> [u8; 65] {
    let (signature, recovery_id) = key.sign_prehash_recoverable(digest).unwrap();
    let mut out = [0u8; 65];
    out[..64].copy_from_slice(&signature.to_bytes());
    out[64] = 27 + recovery_id.to_byte();
    out
}

/// Host config seeded with coprocessor + KMS EIP-712 verifiers (single signer, threshold 1).
fn host_config_account_with_verifier(
    admin: Pubkey,
    coprocessor_signer: [u8; 20],
) -> (Pubkey, Account) {
    let (host_config, bump) = host::host_config_address();
    (
        host_config,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(HostConfig {
                admin,
                chain_id: host::SOLANA_POC_CHAIN_ID,
                input_verifier_authority: admin,
                gateway_chain_id: GATEWAY_CHAIN_ID,
                input_verification_contract: INPUT_VERIFICATION_CONTRACT,
                coprocessor_signer,
                decryption_contract: DECRYPTION_CONTRACT,
                current_kms_context_id: 0,
                material_authority: admin,
                test_authority: admin,
                paused: false,
                mock_input_enabled: false,
                // Enable zero birth entropy (test_shims + local poc chain) so eval handle
                // derivation is deterministic; the real secp256k1 verify path is still exercised
                // because mock_input_enabled stays false.
                test_shims_enabled: true,
                grant_deny_list_enabled: false,
                max_hcu_per_tx: 0,
                max_hcu_depth_per_tx: 0,
                updated_slot: 0,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn verify_coprocessor_input_ix(
    program_id: Pubkey,
    host_config: Pubkey,
    input_handle: [u8; 32],
    ct_handles: Vec<[u8; 32]>,
    user_address: [u8; 32],
    contract_address: [u8; 32],
    extra_data: Vec<u8>,
    signatures: Vec<[u8; 65]>,
) -> Instruction {
    Instruction {
        program_id,
        accounts: host::accounts::VerifyCoprocessorInput {
            host_config,
            event_authority: event_authority(program_id),
            program: program_id,
        }
        .to_account_metas(None),
        data: host::instruction::VerifyCoprocessorInput {
            input_handle,
            ct_handles,
            handle_index: 0,
            user_address,
            contract_address,
            contract_chain_id: 12345,
            extra_data,
            signatures,
        }
        .data(),
    }
}

/// A real coprocessor secp256k1 EIP-712 attestation verifies and emits the input receipt.
/// EVM parity (`FHEVMExecutor.verifyInput`): verification creates NO persistent ACL — the
/// only effect is the signed `InputVerifiedEvent`.
#[test]
fn mollusk_verify_coprocessor_input_accepts_real_secp256k1_attestation() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let key = k256::ecdsa::SigningKey::from_bytes(&[0x44u8; 32].into()).unwrap();
    let (host_config, host_config_account) =
        host_config_account_with_verifier(authority, evm_address_of(&key));

    let input_handle = input_handle_for_chain(0x01, 0, 5);
    let ct_handles = vec![input_handle];
    let user = Pubkey::new_unique();
    let user_address = user.to_bytes();
    let contract = Pubkey::new_unique();
    let contract_address = contract.to_bytes();
    let extra_data = vec![0x00u8];

    let digest = host::eip712::typed_data_digest(
        &host::eip712::domain_separator(
            b"InputVerification",
            b"1",
            GATEWAY_CHAIN_ID,
            &INPUT_VERIFICATION_CONTRACT,
        ),
        &host::eip712::ciphertext_verification_struct_hash(
            &ct_handles,
            &user_address,
            &contract_address,
            12345,
            &extra_data,
        ),
    );
    let signatures = vec![sign_eip712(&key, &digest)];

    let context = mollusk_eval_context(authority, vec![(host_config, host_config_account)]);
    let ix = verify_coprocessor_input_ix(
        program_id,
        host_config,
        input_handle,
        ct_handles,
        user_address,
        contract_address,
        extra_data,
        signatures,
    );

    let result = context.process_and_validate_instruction(&ix, &[Check::success()]);

    // The sole effect is the signed verified-input receipt; no ACL account is created.
    let input_events: Vec<InputVerifiedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(input_events.len(), 1);
    assert_eq!(input_events[0].version, host::EVENT_VERSION);
    assert_eq!(input_events[0].input_handle, input_handle);
    assert_eq!(input_events[0].result_handle, input_handle);
    assert_eq!(input_events[0].user, user_address);
    // No app-chosen ACL domain: the receipt carries the attested contract identity.
    assert_eq!(input_events[0].acl_domain_key, contract_address);
}

/// A signature from a key not in the configured signer set is rejected.
#[test]
fn mollusk_verify_coprocessor_input_rejects_unauthorized_signer() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let configured = k256::ecdsa::SigningKey::from_bytes(&[0x44u8; 32].into()).unwrap();
    let attacker = k256::ecdsa::SigningKey::from_bytes(&[0x99u8; 32].into()).unwrap();
    let (host_config, host_config_account) =
        host_config_account_with_verifier(authority, evm_address_of(&configured));

    let input_handle = input_handle_for_chain(0x01, 0, 5);
    let ct_handles = vec![input_handle];
    let user_address = [0x07u8; 32];
    let contract_address = [0x08u8; 32];
    let extra_data = vec![0x00u8];

    let digest = host::eip712::typed_data_digest(
        &host::eip712::domain_separator(
            b"InputVerification",
            b"1",
            GATEWAY_CHAIN_ID,
            &INPUT_VERIFICATION_CONTRACT,
        ),
        &host::eip712::ciphertext_verification_struct_hash(
            &ct_handles,
            &user_address,
            &contract_address,
            12345,
            &extra_data,
        ),
    );
    let signatures = vec![sign_eip712(&attacker, &digest)];

    let context = mollusk_eval_context(authority, vec![(host_config, host_config_account)]);
    let ix = verify_coprocessor_input_ix(
        program_id,
        host_config,
        input_handle,
        ct_handles,
        user_address,
        contract_address,
        extra_data,
        signatures,
    );

    let result = context.process_instruction(&ix);
    assert!(result.raw_result.is_err());
}

// --- KMS context lifecycle (mirror of ProtocolConfig define/destroy) — #1494 ---

fn read_kms_context(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<host::KmsContext> {
    let store = context.account_store.borrow();
    let account = store.get(&address)?;
    if account.owner != host::id() {
        return None;
    }
    let mut data = account.data.as_slice();
    host::KmsContext::try_deserialize(&mut data).ok()
}

fn define_kms_context_ix(
    program_id: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    kms_context: Pubkey,
    context_id: u64,
    signers: Vec<[u8; 20]>,
    thresholds: host::KmsThresholds,
) -> Instruction {
    Instruction {
        program_id,
        accounts: host::accounts::DefineKmsContext {
            admin,
            host_config,
            kms_context,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: host::instruction::DefineKmsContext {
            context_id,
            signers,
            thresholds,
        }
        .data(),
    }
}

fn destroy_kms_context_ix(
    program_id: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    kms_context: Pubkey,
    context_id: u64,
) -> Instruction {
    Instruction {
        program_id,
        accounts: host::accounts::DestroyKmsContext {
            admin,
            host_config,
            kms_context,
        }
        .to_account_metas(None),
        data: host::instruction::DestroyKmsContext { context_id }.data(),
    }
}

#[test]
fn mollusk_define_kms_context_records_signers_and_advances_current() {
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, host_config_account) =
        host_config_account_with_options(admin, admin, false, false, false);
    let (kms_context, _) = host::kms_context_address(1);
    let context = mollusk_eval_context(
        admin,
        vec![
            (host_config, host_config_account),
            (kms_context, system_account(0)),
        ],
    );
    let signers = vec![[0x11u8; 20], [0x22u8; 20]];
    let thresholds = host::KmsThresholds {
        public_decryption: 2,
        user_decryption: 1,
        kms_gen: 1,
        mpc: 1,
    };

    let ix = define_kms_context_ix(
        program_id,
        admin,
        host_config,
        kms_context,
        1,
        signers.clone(),
        thresholds,
    );
    context.process_and_validate_instruction(&ix, &[Check::success()]);

    let kc = read_kms_context(&context, kms_context).expect("expected KMS context");
    assert_eq!(kc.context_id, 1);
    assert_eq!(kc.signers, signers);
    assert_eq!(kc.thresholds.public_decryption, 2);
    assert!(!kc.destroyed);
    assert_eq!(
        read_host_config(&context, host_config)
            .unwrap()
            .current_kms_context_id,
        1
    );
}

/// SECURITY (KMS context): a signer set containing a duplicate address is rejected. Threshold
/// verification counts DISTINCT recovered addresses, so a duplicate would silently raise the
/// effective quorum (a 2-of-[A, A] set can never be satisfied). EVM KMS signer sets are distinct;
/// thresholds here are individually valid, so the rejection is due solely to the duplicate.
#[test]
fn mollusk_define_kms_context_rejects_duplicate_signers() {
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, host_config_account) =
        host_config_account_with_options(admin, admin, false, false, false);
    let (kms_context, _) = host::kms_context_address(1);
    let context = mollusk_eval_context(
        admin,
        vec![
            (host_config, host_config_account),
            (kms_context, system_account(0)),
        ],
    );
    let signers = vec![[0x11u8; 20], [0x11u8; 20]];
    let thresholds = host::KmsThresholds {
        public_decryption: 1,
        user_decryption: 1,
        kms_gen: 1,
        mpc: 1,
    };

    let ix = define_kms_context_ix(
        program_id,
        admin,
        host_config,
        kms_context,
        1,
        signers,
        thresholds,
    );
    let result = context.process_instruction(&ix);
    assert!(
        result.raw_result.is_err(),
        "a KMS context with duplicate signers must be rejected"
    );
    assert!(read_kms_context(&context, kms_context).is_none());
}

#[test]
fn mollusk_destroy_kms_context_rejects_current_but_destroys_prior() {
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, host_config_account) =
        host_config_account_with_options(admin, admin, false, false, false);
    let (kc1, _) = host::kms_context_address(1);
    let (kc2, _) = host::kms_context_address(2);
    let context = mollusk_eval_context(
        admin,
        vec![
            (host_config, host_config_account),
            (kc1, system_account(0)),
            (kc2, system_account(0)),
        ],
    );
    let thresholds = host::KmsThresholds {
        public_decryption: 1,
        user_decryption: 1,
        kms_gen: 1,
        mpc: 1,
    };
    context.process_and_validate_instruction(
        &define_kms_context_ix(
            program_id,
            admin,
            host_config,
            kc1,
            1,
            vec![[0x11u8; 20]],
            thresholds,
        ),
        &[Check::success()],
    );
    context.process_and_validate_instruction(
        &define_kms_context_ix(
            program_id,
            admin,
            host_config,
            kc2,
            2,
            vec![[0x22u8; 20]],
            thresholds,
        ),
        &[Check::success()],
    );

    // Prior context can be destroyed.
    context.process_and_validate_instruction(
        &destroy_kms_context_ix(program_id, admin, host_config, kc1, 1),
        &[Check::success()],
    );
    assert!(read_kms_context(&context, kc1).unwrap().destroyed);

    // The current context (2) cannot.
    let result = context.process_instruction(&destroy_kms_context_ix(
        program_id,
        admin,
        host_config,
        kc2,
        2,
    ));
    assert!(result.raw_result.is_err());
    assert!(!read_kms_context(&context, kc2).unwrap().destroyed);
}

fn amount_plaintext(amount: u64) -> [u8; 32] {
    let mut plaintext = [0_u8; 32];
    plaintext[24..].copy_from_slice(&amount.to_be_bytes());
    plaintext
}

fn handle_for_chain(seed: u8, fhe_type: u8) -> [u8; 32] {
    let mut handle = [seed; 32];
    handle[21] = 0;
    handle[22..30].copy_from_slice(&host::SOLANA_POC_CHAIN_ID.to_be_bytes());
    handle[30] = fhe_type;
    handle[31] = host::HANDLE_VERSION;
    handle
}

fn input_handle_for_chain(seed: u8, handle_index: u8, fhe_type: u8) -> [u8; 32] {
    let mut handle = handle_for_chain(seed, fhe_type);
    handle[21] = handle_index;
    handle
}

fn event_authority(program_id: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"__event_authority"], &program_id).0
}

fn anchor_event_prefix(discriminator: &[u8]) -> Vec<u8> {
    anchor_lang::event::EVENT_IX_TAG_LE
        .iter()
        .copied()
        .chain(discriminator.iter().copied())
        .collect()
}

fn decode_anchor_event<T>(data: &[u8]) -> Option<T>
where
    T: AnchorDeserialize + Discriminator,
{
    let prefix = anchor_event_prefix(T::DISCRIMINATOR);
    let payload = data.strip_prefix(prefix.as_slice())?;
    T::deserialize(&mut &*payload).ok()
}

fn label(name: &str) -> [u8; 32] {
    let mut out = [0_u8; 32];
    let bytes = name.as_bytes();
    assert!(bytes.len() <= out.len());
    out[..bytes.len()].copy_from_slice(bytes);
    out
}

// --- fhe_eval VerifiedInput operand: consume an attested input in-frame, no scratch PDA (#1539) ---

/// Signs a coprocessor EIP-712 attestation over `[input_handle]` for (user, contract) and packs it
/// into a `VerifiedInput` operand attestation. The attested `contract_address` is the input's
/// acl_domain_key.
fn verified_input_attestation(
    key: &k256::ecdsa::SigningKey,
    input_handle: [u8; 32],
    user_address: [u8; 32],
    contract_address: [u8; 32],
) -> CoprocessorInputAttestation {
    let ct_handles = vec![input_handle];
    let contract_chain_id = 12345u64;
    let extra_data = vec![0x00u8];
    let digest = host::eip712::typed_data_digest(
        &host::eip712::domain_separator(
            b"InputVerification",
            b"1",
            GATEWAY_CHAIN_ID,
            &INPUT_VERIFICATION_CONTRACT,
        ),
        &host::eip712::ciphertext_verification_struct_hash(
            &ct_handles,
            &user_address,
            &contract_address,
            contract_chain_id,
            &extra_data,
        ),
    );
    CoprocessorInputAttestation {
        input_handle,
        ct_handles,
        handle_index: 0,
        user_address,
        contract_address,
        contract_chain_id,
        extra_data,
        signatures: vec![sign_eip712(key, &digest)],
    }
}

/// Builds a one-step `fhe_eval` that adds `scalar` to a `VerifiedInput` operand and binds a durable
/// output ACL record under `acl_domain` (the `output_acl_record` PDA goes in `remaining_accounts`).
#[allow(clippy::too_many_arguments)]
fn verified_input_add_eval_ix(
    program_id: Pubkey,
    authority: Pubkey,
    host_config: Pubkey,
    attestation: CoprocessorInputAttestation,
    scalar: [u8; 32],
    context_id: [u8; 32],
    acl_domain: Pubkey,
    nonce_key: [u8; 32],
    encrypted_value_label: [u8; 32],
    output_acl_record: Pubkey,
    output_subjects: Vec<AclSubjectEntry>,
) -> Instruction {
    let mut eval_ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: authority,
            compute_subject: authority,
            app_account_authority: authority,
            host_config,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                steps: vec![FheEvalStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheEvalOperand::VerifiedInput { attestation },
                    rhs: FheEvalOperand::Scalar(scalar),
                    output_fhe_type: 5,
                    output: FheEvalOutput::AllowedDurable {
                        output_acl_record_index: 0,
                        output_app_account_authority_index: None,
                        output_nonce_key: nonce_key,
                        output_nonce_sequence: 0,
                        output_acl_domain_key: acl_domain,
                        output_app_account: authority,
                        output_encrypted_value_label: encrypted_value_label,
                        output_subjects,
                        output_public_decrypt: false,
                    },
                }],
            },
        },
    );
    eval_ix
        .accounts
        .push(AccountMeta::new(output_acl_record, false));
    eval_ix
}

/// (a) A verified input feeds a scalar add in one `fhe_eval`; the durable output binds the expected
/// handle + AclRecord under the app-level acl_domain_key — no scratch PDA for the input.
#[test]
fn mollusk_fhe_eval_verified_input_scalar_add_binds_durable_output() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let key = k256::ecdsa::SigningKey::from_bytes(&[0x44u8; 32].into()).unwrap();
    let (host_config, host_config_account) =
        host_config_account_with_verifier(authority, evm_address_of(&key));

    // `authority` is the app's signing authority — the attested contract (a mint/token PDA signing
    // via CPI in a real app). The attested input owner is a DIFFERENT end user, recorded as a
    // subject. The output binds to the app authority: acl_domain == app_account == attested contract.
    let user = Pubkey::new_unique();
    let encrypted_value_label = label("verified-input-add");
    let nonce_key = host::acl_nonce_key(authority, authority, encrypted_value_label);
    let input_handle = input_handle_for_chain(0x01, 0, 5);
    let attestation =
        verified_input_attestation(&key, input_handle, user.to_bytes(), authority.to_bytes());

    let output_acl_record = host::acl_record_address(nonce_key, 0).0;
    let context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (output_acl_record, system_account(0)),
        ],
    );

    let scalar = amount_plaintext(2);
    let context_id = label("verified-input-frame");
    let output_handle = current_bound_eval_handle(
        &context.mollusk,
        FheBinaryOpCode::Add,
        input_handle,
        scalar,
        true,
        5,
        context_id,
        0,
        nonce_key,
        0,
    );

    let eval_ix = verified_input_add_eval_ix(
        program_id,
        authority,
        host_config,
        attestation,
        scalar,
        context_id,
        authority,
        nonce_key,
        encrypted_value_label,
        output_acl_record,
        vec![AclSubjectEntry::use_only(user)],
    );

    assert_transaction_success(&context, &[eval_ix]);

    let output_record =
        read_acl_record(&context, output_acl_record).expect("expected durable output ACL");
    assert_bound_acl_record(
        &output_record,
        output_handle,
        nonce_key,
        0,
        authority,
        authority,
        encrypted_value_label,
        user,
        host::ACL_ROLE_USE,
        context.mollusk.sysvars.clock.slot,
    );
}

/// Builds a one-step `fhe_eval` that adds `scalar` to a *durable* input operand (its ACL record
/// goes in `remaining_accounts`), producing a transient result. Used to show a verified input
/// leaves behind no durable ACL a later instruction could ride on.
fn durable_input_add_eval_ix(
    program_id: Pubkey,
    authority: Pubkey,
    host_config: Pubkey,
    input_handle: [u8; 32],
    input_acl_record: Pubkey,
    scalar: [u8; 32],
    context_id: [u8; 32],
) -> Instruction {
    let mut eval_ix = anchor_ix(
        program_id,
        host::accounts::FheEval {
            payer: authority,
            compute_subject: authority,
            app_account_authority: authority,
            host_config,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::FheEval {
            args: FheEvalArgs {
                context_id,
                steps: vec![FheEvalStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheEvalOperand::AllowedDurable {
                        handle: input_handle,
                        acl_record_index: 0,
                        permission_index: None,
                    },
                    rhs: FheEvalOperand::Scalar(scalar),
                    output_fhe_type: 5,
                    output: FheEvalOutput::AllowedLocal,
                }],
            },
        },
    );
    eval_ix
        .accounts
        .push(AccountMeta::new_readonly(input_acl_record, false));
    eval_ix
}

/// (b) An attestation signed by a key the host config does not trust as the coprocessor signer is
/// rejected: VerifiedInput resolution enforces the secp256k1 verification in-frame, it is not
/// assumed from the operand being present.
#[test]
fn mollusk_fhe_eval_verified_input_rejects_forged_attestation() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let trusted = k256::ecdsa::SigningKey::from_bytes(&[0x44u8; 32].into()).unwrap();
    let attacker = k256::ecdsa::SigningKey::from_bytes(&[0x99u8; 32].into()).unwrap();
    let (host_config, host_config_account) =
        host_config_account_with_verifier(authority, evm_address_of(&trusted));

    let user = Pubkey::new_unique();
    let value_label = label("verified-input-forged");
    let nonce_key = host::acl_nonce_key(authority, authority, value_label);
    let input_handle = input_handle_for_chain(0x01, 0, 5);
    // A well-formed (user, app) binding, but signed by the attacker — not the trusted coprocessor
    // signer. The output binding passes; the in-frame secp256k1 verify is what rejects it.
    let attestation = verified_input_attestation(
        &attacker,
        input_handle,
        user.to_bytes(),
        authority.to_bytes(),
    );

    let output_acl_record = host::acl_record_address(nonce_key, 0).0;
    let context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (output_acl_record, system_account(0)),
        ],
    );

    let eval_ix = verified_input_add_eval_ix(
        program_id,
        authority,
        host_config,
        attestation,
        amount_plaintext(2),
        label("verified-input-frame"),
        authority,
        nonce_key,
        value_label,
        output_acl_record,
        vec![AclSubjectEntry::use_only(user)],
    );

    let result = context.process_instruction(&eval_ix);
    assert!(
        result.raw_result.is_err(),
        "forged attestation must be rejected"
    );
}

/// (c) No leak: a verified input grants access only inside the instruction that carries its
/// attestation. After it is consumed, no durable or scratch state persists, so a later instruction
/// cannot reach the same input handle without re-supplying the attestation — this is what justifies
/// resolving it in-frame with no PDA.
#[test]
fn mollusk_fhe_eval_verified_input_does_not_leak_to_a_later_instruction() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let key = k256::ecdsa::SigningKey::from_bytes(&[0x44u8; 32].into()).unwrap();
    let (host_config, host_config_account) =
        host_config_account_with_verifier(authority, evm_address_of(&key));

    let user = Pubkey::new_unique();
    let value_label = label("verified-input-noleak");
    let nonce_key = host::acl_nonce_key(authority, authority, value_label);
    let input_handle = input_handle_for_chain(0x01, 0, 5);
    let attestation =
        verified_input_attestation(&key, input_handle, user.to_bytes(), authority.to_bytes());

    let output_acl_record = host::acl_record_address(nonce_key, 0).0;
    // An empty account standing in for the ACL record the input would need but was never granted.
    let absent_input_acl = Pubkey::new_unique();
    let context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (output_acl_record, system_account(0)),
            (absent_input_acl, system_account(0)),
        ],
    );
    let context_id = label("verified-input-frame");

    // 1) Consume the verified input in-frame: succeeds and binds the durable output.
    let consume_ix = verified_input_add_eval_ix(
        program_id,
        authority,
        host_config,
        attestation,
        amount_plaintext(2),
        context_id,
        authority,
        nonce_key,
        value_label,
        output_acl_record,
        vec![AclSubjectEntry::use_only(user)],
    );
    assert_transaction_success(&context, &[consume_ix]);

    // 2) A separate instruction cannot use the same input handle durably — nothing persisted it.
    let reuse_ix = durable_input_add_eval_ix(
        program_id,
        authority,
        host_config,
        input_handle,
        absent_input_acl,
        amount_plaintext(2),
        label("verified-input-reuse"),
    );
    let result = context.process_instruction(&reuse_ix);
    assert!(
        result.raw_result.is_err(),
        "verified input must not be reachable without re-verifying"
    );
}

/// (d) The output of a verified input must bind the acl_domain_key the input was attested for.
/// Binding the output under a different domain (a cross-domain move) is rejected — the only
/// violation here is attested-domain != bound-domain (the output metadata is otherwise consistent).
#[test]
fn mollusk_fhe_eval_verified_input_rejects_wrong_output_acl_domain_key() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let key = k256::ecdsa::SigningKey::from_bytes(&[0x44u8; 32].into()).unwrap();
    let (host_config, host_config_account) =
        host_config_account_with_verifier(authority, evm_address_of(&key));

    let user = Pubkey::new_unique();
    let wrong_domain = Pubkey::new_unique();
    let value_label = label("verified-input-domain");
    // The attested contract is the app authority (`authority`); the output is bound under a WRONG
    // domain. Output metadata is internally consistent with the wrong domain, isolating the failure
    // to the verified-input domain binding rather than a nonce-key / metadata mismatch.
    let nonce_key = host::acl_nonce_key(wrong_domain, authority, value_label);
    let input_handle = input_handle_for_chain(0x01, 0, 5);
    let attestation =
        verified_input_attestation(&key, input_handle, user.to_bytes(), authority.to_bytes());

    let output_acl_record = host::acl_record_address(nonce_key, 0).0;
    let context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (output_acl_record, system_account(0)),
        ],
    );

    let eval_ix = verified_input_add_eval_ix(
        program_id,
        authority,
        host_config,
        attestation,
        amount_plaintext(2),
        label("verified-input-frame"),
        wrong_domain,
        nonce_key,
        value_label,
        output_acl_record,
        vec![AclSubjectEntry::use_only(user)],
    );

    let result = context.process_instruction(&eval_ix);
    assert_instruction_custom_error(&result, host::errors::ZamaHostError::AclDomainKeyMismatch);
}

/// (e) A verified input is authorized by its provider for the attested domain, so it propagates
/// public-decrypt like a public scalar: a durable output derived from it may grant the
/// PUBLIC_DECRYPT-capable `user` role (so the app can later allow_for_decryption) even though the
/// app account authority is a plain signer. Before binding the input's domain this same shape was
/// rejected as an un-propagated public-decrypt escalation.
#[test]
fn mollusk_fhe_eval_verified_input_propagates_public_decrypt_to_durable_output() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let key = k256::ecdsa::SigningKey::from_bytes(&[0x44u8; 32].into()).unwrap();
    let (host_config, host_config_account) =
        host_config_account_with_verifier(authority, evm_address_of(&key));

    let user = Pubkey::new_unique();
    let value_label = label("verified-input-pubdec");
    let nonce_key = host::acl_nonce_key(authority, authority, value_label);
    let input_handle = input_handle_for_chain(0x01, 0, 5);
    let attestation =
        verified_input_attestation(&key, input_handle, user.to_bytes(), authority.to_bytes());

    let output_acl_record = host::acl_record_address(nonce_key, 0).0;
    let context = mollusk_eval_context(
        authority,
        vec![
            (host_config, host_config_account),
            (output_acl_record, system_account(0)),
        ],
    );

    let scalar = amount_plaintext(2);
    let context_id = label("verified-input-frame");
    let output_handle = current_bound_eval_handle(
        &context.mollusk,
        FheBinaryOpCode::Add,
        input_handle,
        scalar,
        true,
        5,
        context_id,
        0,
        nonce_key,
        0,
    );

    let eval_ix = verified_input_add_eval_ix(
        program_id,
        authority,
        host_config,
        attestation,
        scalar,
        context_id,
        authority,
        nonce_key,
        value_label,
        output_acl_record,
        vec![AclSubjectEntry::user(user)],
    );

    assert_transaction_success(&context, &[eval_ix]);

    let output_record =
        read_acl_record(&context, output_acl_record).expect("expected durable output ACL");
    assert_bound_acl_record(
        &output_record,
        output_handle,
        nonce_key,
        0,
        authority,
        authority,
        value_label,
        user,
        host::ACL_ROLE_ALL,
        context.mollusk.sysvars.clock.slot,
    );
}

/// (f) Replay guard: a copied (public) attestation cannot be consumed by a signer other than the
/// attested app authority. An attacker who lifts a victim's on-chain attestation and submits their
/// own `fhe_eval` — claiming the derived output for themselves — is rejected because the output app
/// account is forced to equal the attested `contract_address`, and the output app account must
/// itself sign (here the attacker signs as themselves, not as the attested app). This is the
/// non-replayable binding: only the attested app (signing directly or via CPI) can consume.
#[test]
fn mollusk_fhe_eval_verified_input_rejects_consumption_by_non_attested_app() {
    let program_id = host::id();
    let attacker = Pubkey::new_unique(); // signs the malicious eval (app_account_authority)
    let attested_app = Pubkey::new_unique(); // the attested contract — the attacker does not control it
    let user = Pubkey::new_unique(); // the attested input owner (the victim the proof was made for)
    let key = k256::ecdsa::SigningKey::from_bytes(&[0x44u8; 32].into()).unwrap();
    let (host_config, host_config_account) =
        host_config_account_with_verifier(attacker, evm_address_of(&key));

    let value_label = label("verified-input-replay");
    // The attacker binds the output domain to the attested app (so the domain pin passes) but owns
    // the output themselves; the app_account == attested-contract check is what trips.
    let nonce_key = host::acl_nonce_key(attested_app, attacker, value_label);
    let input_handle = input_handle_for_chain(0x01, 0, 5);
    // A VALID attestation for (user, attested_app), as if copied from the victim's instruction data.
    let attestation =
        verified_input_attestation(&key, input_handle, user.to_bytes(), attested_app.to_bytes());

    let output_acl_record = host::acl_record_address(nonce_key, 0).0;
    let context = mollusk_eval_context(
        attacker,
        vec![
            (host_config, host_config_account),
            (output_acl_record, system_account(0)),
        ],
    );

    // attacker signs as app_account_authority/output_app_account (= `attacker`, != attested_app).
    let eval_ix = verified_input_add_eval_ix(
        program_id,
        attacker,
        host_config,
        attestation,
        amount_plaintext(2),
        label("verified-input-frame"),
        attested_app,
        nonce_key,
        value_label,
        output_acl_record,
        vec![AclSubjectEntry::use_only(user)],
    );

    let result = context.process_instruction(&eval_ix);
    assert_instruction_custom_error(
        &result,
        host::errors::ZamaHostError::InputBindContractMismatch,
    );
}
