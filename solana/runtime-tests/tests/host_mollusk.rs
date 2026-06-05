use anchor_lang::{
    prelude::{bpf_loader_upgradeable, system_program},
    AccountDeserialize, AccountSerialize, AnchorDeserialize, Discriminator, InstructionData,
    ToAccountMetas,
};
use mollusk_svm::{
    result::{types::TransactionResult, Check},
    Mollusk,
};
use solana_sdk::{
    account::Account,
    ed25519_program,
    instruction::{AccountMeta, Instruction},
    native_loader,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
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
    AclPermission, AclRecord, AclSubjectEntry, DenySubjectRecord, FheBinaryOpCode, FheEvalArgs,
    FheEvalOp, FheEvalOperand, FheEvalOutput, HandleMaterialCommitment, HostConfig,
    TransientCapabilityGrant, TransientSession, UserDecryptionDelegation,
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
    let context = transient_context(
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
    let context = transient_context(
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
    let context = transient_context(
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
fn mollusk_input_mock_verified_bind_creates_acl_record_and_event() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let verifier = Pubkey::new_unique();
    let (host_config, host_config_account) =
        host_config_account_with_flags(authority, verifier, true, true);
    let acl_domain_key = authority;
    let app_account = authority;
    let encrypted_value_label = label("input-gate");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_acl = host::acl_record_address(nonce_key, 13).0;
    let input_handle = input_handle_for_chain(7, 0, 5);
    let context = transient_context(
        authority,
        vec![
            (verifier, system_account(1_000_000_000)),
            (host_config, host_config_account),
            (output_acl, system_account(0)),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );

    let result = context.process_instruction(&mock_input_verified_and_bind_ix(
        program_id,
        authority,
        verifier,
        host_config,
        output_acl,
        nonce_key,
        13,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        input_handle,
        false,
    ));
    assert!(result.raw_result.is_ok());
    let record = read_acl_record(&context, output_acl).expect("expected mock input ACL");
    assert_bound_acl_record(
        &record,
        input_handle,
        nonce_key,
        13,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        authority,
        host::ACL_ROLE_COMPUTE_SUBJECT,
        context.mollusk.sysvars.clock.slot,
    );
    let input_events: Vec<InputVerifiedEvent> = result
        .inner_instructions
        .iter()
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(input_events.len(), 1);
    assert_eq!(input_events[0].version, host::EVENT_VERSION);
    assert_eq!(input_events[0].input_handle, input_handle);
    assert_eq!(input_events[0].result_handle, input_handle);
    assert_eq!(input_events[0].user, authority.to_bytes());
    assert_eq!(input_events[0].acl_domain_key, acl_domain_key.to_bytes());
    assert_single_acl_allowed_event(&result, input_handle, authority);
}

#[test]
fn mollusk_input_birth_rejects_invalid_trivial_and_mock_without_acl_birth() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let verifier = Pubkey::new_unique();
    let (host_config, host_config_account) =
        host_config_account_with_flags(authority, verifier, true, true);
    let acl_domain_key = authority;
    let app_account = authority;
    let bad_type_label = label("bad-trivial-type");
    let bad_type_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, bad_type_label);
    let bad_type_acl = host::acl_record_address(bad_type_nonce_key, 20).0;
    let public_birth_label = label("trivial-public-birth");
    let public_birth_nonce_key =
        host::acl_nonce_key(acl_domain_key, app_account, public_birth_label);
    let public_birth_acl = host::acl_record_address(public_birth_nonce_key, 21).0;
    let bad_input_label = label("bad-input-handle");
    let bad_input_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, bad_input_label);
    let bad_input_acl = host::acl_record_address(bad_input_nonce_key, 22).0;
    let context = transient_context(
        authority,
        vec![
            (verifier, system_account(1_000_000_000)),
            (host_config, host_config_account),
            (bad_type_acl, system_account(0)),
            (public_birth_acl, system_account(0)),
            (bad_input_acl, system_account(0)),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );

    let unsupported_type = context.process_instruction(&trivial_encrypt_and_bind_ix(
        program_id,
        authority,
        host_config,
        bad_type_acl,
        bad_type_nonce_key,
        20,
        acl_domain_key,
        app_account,
        bad_type_label,
        amount_plaintext(7),
        1,
        false,
    ));
    assert!(unsupported_type.raw_result.is_err());
    assert!(read_acl_record(&context, bad_type_acl).is_none());

    let public_birth = context.process_instruction(&trivial_encrypt_and_bind_ix(
        program_id,
        authority,
        host_config,
        public_birth_acl,
        public_birth_nonce_key,
        21,
        acl_domain_key,
        app_account,
        public_birth_label,
        amount_plaintext(7),
        5,
        true,
    ));
    assert!(public_birth.raw_result.is_err());
    assert!(read_acl_record(&context, public_birth_acl).is_none());

    let computed_input = context.process_instruction(&mock_input_verified_and_bind_ix(
        program_id,
        authority,
        verifier,
        host_config,
        bad_input_acl,
        bad_input_nonce_key,
        22,
        acl_domain_key,
        app_account,
        bad_input_label,
        computed_like_handle_for_chain(7, 5),
        false,
    ));
    assert!(computed_input.raw_result.is_err());
    assert!(read_acl_record(&context, bad_input_acl).is_none());

    let disabled_label = label("mock-disabled");
    let disabled_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, disabled_label);
    let disabled_acl = host::acl_record_address(disabled_nonce_key, 23).0;
    let (disabled_host_config, disabled_host_config_account) =
        host_config_account_with_flags(authority, verifier, false, true);
    let disabled_context = transient_context(
        authority,
        vec![
            (verifier, system_account(1_000_000_000)),
            (disabled_host_config, disabled_host_config_account),
            (disabled_acl, system_account(0)),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );
    let disabled = disabled_context.process_instruction(&mock_input_verified_and_bind_ix(
        program_id,
        authority,
        verifier,
        disabled_host_config,
        disabled_acl,
        disabled_nonce_key,
        23,
        acl_domain_key,
        app_account,
        disabled_label,
        input_handle_for_chain(8, 0, 5),
        false,
    ));
    assert!(disabled.raw_result.is_err());
    assert!(read_acl_record(&disabled_context, disabled_acl).is_none());

    let wrong_verifier = Pubkey::new_unique();
    let wrong_verifier_label = label("mock-wrong-verifier");
    let wrong_verifier_nonce_key =
        host::acl_nonce_key(acl_domain_key, app_account, wrong_verifier_label);
    let wrong_verifier_acl = host::acl_record_address(wrong_verifier_nonce_key, 24).0;
    let (wrong_host_config, wrong_host_config_account) =
        host_config_account_with_flags(authority, verifier, true, true);
    let wrong_context = transient_context(
        authority,
        vec![
            (verifier, system_account(1_000_000_000)),
            (wrong_verifier, system_account(1_000_000_000)),
            (wrong_host_config, wrong_host_config_account),
            (wrong_verifier_acl, system_account(0)),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );
    let wrong = wrong_context.process_instruction(&mock_input_verified_and_bind_ix(
        program_id,
        authority,
        wrong_verifier,
        wrong_host_config,
        wrong_verifier_acl,
        wrong_verifier_nonce_key,
        24,
        acl_domain_key,
        app_account,
        wrong_verifier_label,
        input_handle_for_chain(9, 0, 5),
        false,
    ));
    assert!(wrong.raw_result.is_err());
    assert!(read_acl_record(&wrong_context, wrong_verifier_acl).is_none());
}

#[test]
fn mollusk_signed_input_bind_accepts_ed25519_preinstruction_and_events() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let verifier = Keypair::new();
    let (host_config, host_config_account) =
        host_config_account_with_flags(authority, verifier.pubkey(), true, true);
    let acl_domain_key = authority;
    let app_account = authority;
    let encrypted_value_label = label("signed-input");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let output_acl = host::acl_record_address(nonce_key, 30).0;
    let input_handle = input_handle_for_chain(7, 1, 5);
    let proof = host::SolanaInputProof {
        handles: vec![input_handle_for_chain(8, 0, 5), input_handle],
        handle_index: 1,
        user: authority,
        app_account,
        acl_domain_key,
        extra_data: b"mollusk-proof-context".to_vec(),
    };
    let output_subjects = vec![AclSubjectEntry::compute(authority)];
    let instructions = signed_verify_input_and_bind_instructions(
        program_id,
        authority,
        verifier.pubkey(),
        &verifier,
        host_config,
        output_acl,
        input_handle,
        proof,
        nonce_key,
        30,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        output_subjects,
        false,
    );
    let context = transient_context(
        authority,
        vec![
            (verifier.pubkey(), system_account(1_000_000_000)),
            (host_config, host_config_account),
            (output_acl, system_account(0)),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );

    let result = process_transaction_result(&context, &instructions);

    assert!(result.raw_result.is_ok());
    let record = read_acl_record(&context, output_acl).expect("expected signed input ACL");
    assert_bound_acl_record(
        &record,
        input_handle,
        nonce_key,
        30,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        authority,
        host::ACL_ROLE_COMPUTE_SUBJECT,
        context.mollusk.sysvars.clock.slot,
    );
    let input_events: Vec<InputVerifiedEvent> = result
        .inner_instructions
        .iter()
        .flat_map(|group| group.iter())
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(input_events.len(), 1);
    assert_eq!(input_events[0].version, host::EVENT_VERSION);
    assert_eq!(input_events[0].input_handle, input_handle);
    assert_eq!(input_events[0].result_handle, input_handle);
    assert_eq!(input_events[0].user, authority.to_bytes());
    assert_eq!(input_events[0].acl_domain_key, acl_domain_key.to_bytes());
    assert_acl_allowed_transaction_event(&result, input_handle, authority);
}

#[test]
fn mollusk_signed_input_bind_rejects_missing_wrong_and_replayed_signature_without_acl_birth() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let verifier = Keypair::new();
    let wrong_verifier = Keypair::new();
    let (host_config, host_config_account) =
        host_config_account_with_flags(authority, verifier.pubkey(), true, true);
    let acl_domain_key = authority;
    let app_account = authority;
    let input_handle = input_handle_for_chain(7, 0, 5);
    let proof = host::SolanaInputProof {
        handles: vec![input_handle],
        handle_index: 0,
        user: authority,
        app_account,
        acl_domain_key,
        extra_data: b"mollusk-signature-context".to_vec(),
    };
    let subjects = vec![AclSubjectEntry::compute(authority)];

    let missing_label = label("signed-no-sig");
    let missing_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, missing_label);
    let missing_acl = host::acl_record_address(missing_nonce_key, 31).0;
    let wrong_label = label("signed-wrong-signer");
    let wrong_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, wrong_label);
    let wrong_acl = host::acl_record_address(wrong_nonce_key, 32).0;
    let signed_label = label("signed-policy-base");
    let signed_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, signed_label);
    let replay_label = label("signed-policy-replay");
    let replay_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, replay_label);
    let replay_acl = host::acl_record_address(replay_nonce_key, 33).0;
    let context = transient_context(
        authority,
        vec![
            (verifier.pubkey(), system_account(1_000_000_000)),
            (host_config, host_config_account),
            (missing_acl, system_account(0)),
            (wrong_acl, system_account(0)),
            (replay_acl, system_account(0)),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );

    let missing = process_transaction_result(
        &context,
        &[verify_input_and_bind_ix(
            program_id,
            authority,
            verifier.pubkey(),
            host_config,
            missing_acl,
            input_handle,
            proof.clone(),
            missing_nonce_key,
            31,
            acl_domain_key,
            app_account,
            missing_label,
            subjects.clone(),
            false,
        )],
    );
    assert!(missing.raw_result.is_err());
    assert!(read_acl_record(&context, missing_acl).is_none());

    let wrong_instructions = signed_verify_input_and_bind_instructions(
        program_id,
        authority,
        verifier.pubkey(),
        &wrong_verifier,
        host_config,
        wrong_acl,
        input_handle,
        proof.clone(),
        wrong_nonce_key,
        32,
        acl_domain_key,
        app_account,
        wrong_label,
        subjects.clone(),
        false,
    );
    let wrong = process_transaction_result(&context, &wrong_instructions);
    assert!(wrong.raw_result.is_err());
    assert!(read_acl_record(&context, wrong_acl).is_none());

    let signed_intent = input_bind_intent(
        signed_nonce_key,
        33,
        acl_domain_key,
        app_account,
        signed_label,
        subjects.clone(),
        false,
    );
    let replay_ed25519_ix = ed25519_verify_ix(
        &verifier,
        &host::input_proof_message(
            &proof,
            &signed_intent,
            program_id,
            host::SOLANA_POC_CHAIN_ID,
        ),
    );
    let replay_bind_ix = verify_input_and_bind_ix(
        program_id,
        authority,
        verifier.pubkey(),
        host_config,
        replay_acl,
        input_handle,
        proof,
        replay_nonce_key,
        33,
        acl_domain_key,
        app_account,
        replay_label,
        subjects,
        false,
    );
    let replay = process_transaction_result(&context, &[replay_ed25519_ix, replay_bind_ix]);
    assert!(replay.raw_result.is_err());
    assert!(read_acl_record(&context, replay_acl).is_none());
}

#[test]
fn mollusk_signed_input_bind_rejects_bad_proof_handles_without_acl_birth() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let verifier = Keypair::new();
    let (host_config, host_config_account) =
        host_config_account_with_flags(authority, verifier.pubkey(), true, true);
    let acl_domain_key = authority;
    let app_account = authority;
    let bad_index_label = label("signed-bad-index");
    let bad_index_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, bad_index_label);
    let bad_index_acl = host::acl_record_address(bad_index_nonce_key, 34).0;
    let computed_label = label("signed-computed");
    let computed_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, computed_label);
    let computed_acl = host::acl_record_address(computed_nonce_key, 35).0;
    let bad_type_label = label("signed-bad-type");
    let bad_type_nonce_key = host::acl_nonce_key(acl_domain_key, app_account, bad_type_label);
    let bad_type_acl = host::acl_record_address(bad_type_nonce_key, 36).0;
    let subjects = vec![AclSubjectEntry::compute(authority)];
    let context = transient_context(
        authority,
        vec![
            (verifier.pubkey(), system_account(1_000_000_000)),
            (host_config, host_config_account),
            (bad_index_acl, system_account(0)),
            (computed_acl, system_account(0)),
            (bad_type_acl, system_account(0)),
            (event_authority(program_id), system_account(0)),
            (program_id, executable_program_account()),
        ],
    );

    let bad_index_handle = input_handle_for_chain(7, 2, 5);
    let bad_index_proof = host::SolanaInputProof {
        handles: vec![bad_index_handle],
        handle_index: 0,
        user: authority,
        app_account,
        acl_domain_key,
        extra_data: Vec::new(),
    };
    let bad_index_instructions = signed_verify_input_and_bind_instructions(
        program_id,
        authority,
        verifier.pubkey(),
        &verifier,
        host_config,
        bad_index_acl,
        bad_index_handle,
        bad_index_proof,
        bad_index_nonce_key,
        34,
        acl_domain_key,
        app_account,
        bad_index_label,
        subjects.clone(),
        false,
    );
    let bad_index = process_transaction_result(&context, &bad_index_instructions);
    assert!(bad_index.raw_result.is_err());
    assert!(read_acl_record(&context, bad_index_acl).is_none());

    let computed_handle = computed_like_handle_for_chain(8, 5);
    let computed_proof = host::SolanaInputProof {
        handles: vec![computed_handle],
        handle_index: 0,
        user: authority,
        app_account,
        acl_domain_key,
        extra_data: Vec::new(),
    };
    let computed_instructions = signed_verify_input_and_bind_instructions(
        program_id,
        authority,
        verifier.pubkey(),
        &verifier,
        host_config,
        computed_acl,
        computed_handle,
        computed_proof,
        computed_nonce_key,
        35,
        acl_domain_key,
        app_account,
        computed_label,
        subjects.clone(),
        false,
    );
    let computed = process_transaction_result(&context, &computed_instructions);
    assert!(computed.raw_result.is_err());
    assert!(read_acl_record(&context, computed_acl).is_none());

    let bad_type_handle = input_handle_for_chain(9, 0, 1);
    let bad_type_proof = host::SolanaInputProof {
        handles: vec![bad_type_handle],
        handle_index: 0,
        user: authority,
        app_account,
        acl_domain_key,
        extra_data: Vec::new(),
    };
    let bad_type_instructions = signed_verify_input_and_bind_instructions(
        program_id,
        authority,
        verifier.pubkey(),
        &verifier,
        host_config,
        bad_type_acl,
        bad_type_handle,
        bad_type_proof,
        bad_type_nonce_key,
        36,
        acl_domain_key,
        app_account,
        bad_type_label,
        subjects,
        false,
    );
    let bad_type = process_transaction_result(&context, &bad_type_instructions);
    assert!(bad_type.raw_result.is_err());
    assert!(read_acl_record(&context, bad_type_acl).is_none());
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
    let context = transient_context(
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
    let context = transient_context(
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
    let context = transient_context(
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
    let context = transient_context(
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
    let missing_context = transient_context(
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
    let dirty_context = transient_context(
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
    let extra_context = transient_context(
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
    let context = transient_context(
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
        let context = transient_context(
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
    let absent_context = transient_context(
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
    let clear_context = transient_context(
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
    let context = transient_context(
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
    let mut context = transient_context(
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
    let context = transient_context(payer, vec![(host_config, system_account(0))]);

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
        let context = transient_context(payer, vec![(host_config, system_account(0))]);

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
    let wrong_admin_context = transient_context(admin, vec![(host_config, host_config_account)]);

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
    let oversized_context = transient_context(
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
fn mollusk_host_config_flag_setters_preserve_idempotent_state_and_slot() {
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(admin);
    let mut context = transient_context(admin, vec![(host_config, host_config_account)]);

    let initial_config = read_host_config(&context, host_config).expect("expected host config");
    assert!(initial_config.mock_input_enabled);
    assert!(initial_config.test_shims_enabled);
    assert_eq!(initial_config.updated_slot, 0);

    let repeat_mock_true = context.process_instruction(&set_mock_input_enabled_ix(
        program_id,
        admin,
        host_config,
        true,
    ));
    assert!(repeat_mock_true.raw_result.is_ok());
    let repeated_config = read_host_config(&context, host_config).expect("expected host config");
    assert!(repeated_config.mock_input_enabled);
    assert!(repeated_config.test_shims_enabled);
    assert_eq!(repeated_config.updated_slot, 0);

    context.mollusk.sysvars.warp_to_slot(10);
    let disable_mock = context.process_instruction(&set_mock_input_enabled_ix(
        program_id,
        admin,
        host_config,
        false,
    ));
    assert!(disable_mock.raw_result.is_ok());
    let disabled_mock_config =
        read_host_config(&context, host_config).expect("expected host config");
    assert!(!disabled_mock_config.mock_input_enabled);
    assert!(disabled_mock_config.test_shims_enabled);
    assert_eq!(disabled_mock_config.updated_slot, 10);

    context.mollusk.sysvars.warp_to_slot(11);
    let repeat_mock_false = context.process_instruction(&set_mock_input_enabled_ix(
        program_id,
        admin,
        host_config,
        false,
    ));
    assert!(repeat_mock_false.raw_result.is_ok());
    let repeated_mock_false_config =
        read_host_config(&context, host_config).expect("expected host config");
    assert!(!repeated_mock_false_config.mock_input_enabled);
    assert!(repeated_mock_false_config.test_shims_enabled);
    assert_eq!(
        repeated_mock_false_config.updated_slot,
        disabled_mock_config.updated_slot
    );

    context.mollusk.sysvars.warp_to_slot(12);
    let repeat_shims_true = context.process_instruction(&set_test_shims_enabled_ix(
        program_id,
        admin,
        host_config,
        true,
    ));
    assert!(repeat_shims_true.raw_result.is_ok());
    let repeated_shims_true_config =
        read_host_config(&context, host_config).expect("expected host config");
    assert!(!repeated_shims_true_config.mock_input_enabled);
    assert!(repeated_shims_true_config.test_shims_enabled);
    assert_eq!(
        repeated_shims_true_config.updated_slot,
        disabled_mock_config.updated_slot
    );

    context.mollusk.sysvars.warp_to_slot(13);
    let disable_shims = context.process_instruction(&set_test_shims_enabled_ix(
        program_id,
        admin,
        host_config,
        false,
    ));
    assert!(disable_shims.raw_result.is_ok());
    let disabled_shims_config =
        read_host_config(&context, host_config).expect("expected host config");
    assert!(!disabled_shims_config.mock_input_enabled);
    assert!(!disabled_shims_config.test_shims_enabled);
    assert_eq!(disabled_shims_config.updated_slot, 13);

    context.mollusk.sysvars.warp_to_slot(14);
    let repeat_shims_false = context.process_instruction(&set_test_shims_enabled_ix(
        program_id,
        admin,
        host_config,
        false,
    ));
    assert!(repeat_shims_false.raw_result.is_ok());
    let repeated_shims_false_config =
        read_host_config(&context, host_config).expect("expected host config");
    assert!(!repeated_shims_false_config.mock_input_enabled);
    assert!(!repeated_shims_false_config.test_shims_enabled);
    assert_eq!(
        repeated_shims_false_config.updated_slot,
        disabled_shims_config.updated_slot
    );
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
    let mut context = transient_context(
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
    let mut context = transient_context(
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
fn mollusk_host_admin_pause_allows_transient_session_close_cleanup() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let authority_close_nonce = label("admin-close-auth");
    let expired_close_nonce = label("admin-close-exp");
    let (authority_close_session, _) =
        host::transient_session_address(authority, authority_close_nonce);
    let (expired_close_session, _) =
        host::transient_session_address(authority, expired_close_nonce);
    let mut context = transient_context(
        authority,
        vec![
            (host_config, host_config_account),
            (authority_close_session, system_account(0)),
            (expired_close_session, system_account(0)),
        ],
    );
    let current_slot = context.mollusk.sysvars.clock.slot;

    assert_transaction_success(
        &context,
        &[
            create_transient_session_ix(
                program_id,
                authority,
                host_config,
                authority_close_session,
                authority_close_nonce,
                authority,
                current_slot,
                1,
            ),
            create_transient_session_ix(
                program_id,
                authority,
                host_config,
                expired_close_session,
                expired_close_nonce,
                authority,
                current_slot,
                1,
            ),
        ],
    );
    assert!(read_transient_session(&context, authority_close_session).is_some());
    assert!(read_transient_session(&context, expired_close_session).is_some());

    let pause =
        context.process_instruction(&set_host_pause_ix(program_id, authority, host_config, true));
    assert!(pause.raw_result.is_ok());
    assert!(
        read_host_config(&context, host_config)
            .expect("expected host config")
            .paused
    );

    let authority_close = context.process_instruction(&close_transient_session_ix(
        program_id,
        Some(authority),
        host_config,
        authority_close_session,
        authority,
    ));
    assert!(authority_close.raw_result.is_ok());
    assert!(read_transient_session(&context, authority_close_session).is_none());

    context.mollusk.sysvars.warp_to_slot(current_slot + 1);
    let expired_close = context.process_instruction(&close_transient_session_ix(
        program_id,
        None,
        host_config,
        expired_close_session,
        authority,
    ));
    assert!(expired_close.raw_result.is_ok());
    assert!(read_transient_session(&context, expired_close_session).is_none());
}

#[test]
fn mollusk_delegation_rejects_zero_wildcard_and_equal_subjects() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let delegate = Pubkey::new_unique();
    let app_account = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let context = transient_context(authority, vec![host_config_account(authority)]);
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
    let mut context = transient_context(
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
    let context = transient_context(
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
    let context = transient_context(
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
fn mollusk_transient_session_consumes_capability_once() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let encrypted_value_label = label("balance");
    let nonce_key = host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label);
    let handle = handle_for_chain(7, 5);
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
        1,
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
    assert_transaction_success(&context, &[create_ix, allow_ix, seal_ix, consume_ix]);

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
fn mollusk_transient_session_rejects_consume_without_same_transaction_create() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let host_config = host_config_account(authority).0;
    let acl_domain_key = Pubkey::new_unique();
    let app_account = authority;
    let session_nonce = label("late-consume");
    let (session, bump) = host::transient_session_address(authority, session_nonce);
    let handle = handle_for_chain(7, 5);
    let capability =
        transient_capability(authority, program_id, acl_domain_key, app_account, false);
    let context = transient_context(
        authority,
        vec![
            host_config_account(authority),
            (
                session,
                Account {
                    lamports: 1_000_000_000,
                    data: serialized_account(TransientSession {
                        session_nonce,
                        authority,
                        refund_recipient: authority,
                        compute_subject: authority,
                        created_slot: 0,
                        expires_slot: u64::MAX,
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
            ),
        ],
    );

    let context_id = label("late-consume-eval");
    let rhs = amount_plaintext(3);
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

    let result = context.process_instruction(&consume_ix);
    assert!(result.raw_result.is_err());
    let session_account = read_transient_session(&context, session).expect("expected session");
    assert_eq!(session_account.entries[0].used_count, 0);
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
    let handle = handle_for_chain(7, 5);
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
        1,
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
    let handle = handle_for_chain(7, 5);
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
        1,
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
    let handle = handle_for_chain(7, 5);
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
    let context = transient_context(
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
    let unused_sysvar_slot = Pubkey::new_unique();
    let context = transient_context(
        authority,
        vec![
            host_config_account(authority),
            (lhs_acl_record, lhs_acl_account),
            (extra_account, system_account(1_000_000)),
            (unused_sysvar_slot, system_account(1_000_000)),
        ],
    );

    let build_eval_ix = |context_id: [u8; 32], instructions_sysvar: Option<Pubkey>| {
        let rhs = amount_plaintext(2);
        let result_handle = current_eval_handle(
            &context.mollusk,
            FheBinaryOpCode::Add,
            lhs,
            rhs,
            true,
            5,
            context_id,
            0,
        );
        anchor_ix(
            program_id,
            host::accounts::FheEval {
                payer: authority,
                compute_subject: authority,
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
                        result: result_handle,
                        output: FheEvalOutput::Transient,
                    }],
                },
            },
        )
    };

    let mut extra_remaining_ix = build_eval_ix(label("mollusk-eval-extra"), None);
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

    let mut unused_sysvar_ix = build_eval_ix(
        label("mollusk-eval-unused-sysvar"),
        Some(unused_sysvar_slot),
    );
    unused_sysvar_ix
        .accounts
        .push(AccountMeta::new_readonly(lhs_acl_record, false));
    let sysvar_result = context.process_instruction(&unused_sysvar_ix);

    assert!(sysvar_result.raw_result.is_err());
    assert!(sysvar_result.inner_instructions.is_empty());
    let record_after_sysvar =
        read_acl_record(&context, lhs_acl_record).expect("expected LHS ACL record");
    assert_eq!(record_after_sysvar.handle, lhs);
    assert!(record_after_sysvar.inline_subject_has_role(authority, host::ACL_ROLE_ALL));
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
    let context = transient_context(
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
fn mollusk_fhe_binary_op_bind_rejects_public_decrypt_output_without_input_role() {
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
    let durable_handle = handle_for_chain(7, 5);
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
        1,
    );
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

    let seal_ix = seal_transient_session_ix(program_id, authority, host_config, session);

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
    assert_transaction_success(&context, &[create_ix, append_ix, seal_ix, consume_ix]);

    let session_account = read_transient_session(&context, session).expect("expected session");
    assert_eq!(session_account.entries.len(), 1);
    assert_eq!(session_account.entries[0].handle, session_handle);
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
    host_config_account_with_flags(admin, admin, true, true)
}

fn host_config_account_with_flags(
    admin: Pubkey,
    input_verifier_authority: Pubkey,
    mock_input_enabled: bool,
    test_shims_enabled: bool,
) -> (Pubkey, Account) {
    host_config_account_with_options(
        admin,
        input_verifier_authority,
        mock_input_enabled,
        test_shims_enabled,
        false,
    )
}

fn host_config_account_with_grant_deny_list(
    admin: Pubkey,
    grant_deny_list_enabled: bool,
) -> (Pubkey, Account) {
    host_config_account_with_options(admin, admin, true, true, grant_deny_list_enabled)
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
fn mock_input_verified_and_bind_ix(
    program_id: Pubkey,
    authority: Pubkey,
    input_verifier_authority: Pubkey,
    host_config: Pubkey,
    output_acl_record: Pubkey,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
    output_encrypted_value_label: [u8; 32],
    input_handle: [u8; 32],
    output_public_decrypt: bool,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::MockInputVerifiedAndBind {
            payer: authority,
            input_verifier_authority,
            app_account_authority: authority,
            host_config,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::MockInputVerifiedAndBind {
            input_handle,
            user: authority,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            output_subjects: vec![AclSubjectEntry::compute(authority)],
            output_public_decrypt,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn signed_verify_input_and_bind_instructions(
    program_id: Pubkey,
    payer: Pubkey,
    input_verifier_authority: Pubkey,
    ed25519_authority: &Keypair,
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
) -> [Instruction; 2] {
    let bind_intent = input_bind_intent(
        output_nonce_key,
        output_nonce_sequence,
        output_acl_domain_key,
        output_app_account,
        output_encrypted_value_label,
        output_subjects.clone(),
        output_public_decrypt,
    );
    [
        ed25519_verify_ix(
            ed25519_authority,
            &host::input_proof_message(&proof, &bind_intent, program_id, host::SOLANA_POC_CHAIN_ID),
        ),
        verify_input_and_bind_ix(
            program_id,
            payer,
            input_verifier_authority,
            host_config,
            output_acl_record,
            input_handle,
            proof,
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            output_subjects,
            output_public_decrypt,
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
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

fn current_trivial_handle(
    mollusk: &Mollusk,
    plaintext: [u8; 32],
    fhe_type: u8,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    host::computed_trivial_handle(
        plaintext,
        fhe_type,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash(mollusk),
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
    host::computed_rand_seed(
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
    if account.owner != host::id() {
        return None;
    }
    let mut data = account.data.as_slice();
    TransientSession::try_deserialize(&mut data).ok()
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

fn assert_acl_allowed_transaction_event(
    result: &TransactionResult,
    handle: [u8; 32],
    subject: Pubkey,
) {
    let events: Vec<AclAllowedEvent> = result
        .inner_instructions
        .iter()
        .flat_map(|group| group.iter())
        .filter_map(|inner| decode_anchor_event(&inner.instruction.data))
        .collect();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].version, host::EVENT_VERSION);
    assert_eq!(events[0].handle, handle);
    assert_eq!(events[0].subject, subject.to_bytes());
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
                test_shims_enabled: false,
                grant_deny_list_enabled: false,
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
fn verify_coprocessor_input_and_bind_ix(
    program_id: Pubkey,
    payer: Pubkey,
    host_config: Pubkey,
    output_acl_record: Pubkey,
    input_handle: [u8; 32],
    ct_handles: Vec<[u8; 32]>,
    user_address: [u8; 32],
    contract_address: [u8; 32],
    extra_data: Vec<u8>,
    signatures: Vec<[u8; 65]>,
    output_nonce_key: [u8; 32],
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
    output_encrypted_value_label: [u8; 32],
    output_subjects: Vec<AclSubjectEntry>,
) -> Instruction {
    Instruction {
        program_id,
        accounts: host::accounts::VerifyCoprocessorInputAndBind {
            payer,
            app_account_authority: output_app_account,
            host_config,
            output_acl_record,
            system_program: system_program::ID,
            event_authority: event_authority(program_id),
            program: program_id,
        }
        .to_account_metas(None),
        data: host::instruction::VerifyCoprocessorInputAndBind {
            input_handle,
            ct_handles,
            handle_index: 0,
            user_address,
            contract_address,
            contract_chain_id: 12345,
            extra_data,
            signatures,
            output_nonce_key,
            output_nonce_sequence: 0,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            output_subjects,
            output_public_decrypt: false,
        }
        .data(),
    }
}

/// End-to-end: a real coprocessor secp256k1 EIP-712 attestation binds the handle.
#[test]
fn mollusk_verify_coprocessor_input_and_bind_accepts_real_secp256k1_attestation() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let key = k256::ecdsa::SigningKey::from_bytes(&[0x44u8; 32].into()).unwrap();
    let (host_config, host_config_account) =
        host_config_account_with_verifier(authority, evm_address_of(&key));

    let acl_domain_key = Pubkey::new_unique();
    let label = label("balance");
    let nonce_key = host::acl_nonce_key(acl_domain_key, authority, label);
    let output_acl_record = host::acl_record_address(nonce_key, 0).0;

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
    let signatures = vec![sign_eip712(&key, &digest)];

    let context = transient_context(
        authority,
        vec![
            (host_config, host_config_account),
            (output_acl_record, system_account(0)),
        ],
    );
    let ix = verify_coprocessor_input_and_bind_ix(
        program_id,
        authority,
        host_config,
        output_acl_record,
        input_handle,
        ct_handles,
        user_address,
        contract_address,
        extra_data,
        signatures,
        nonce_key,
        acl_domain_key,
        authority,
        label,
        vec![AclSubjectEntry::user(authority)],
    );

    context.process_and_validate_instruction(&ix, &[Check::success()]);

    let record = read_acl_record(&context, output_acl_record).expect("expected bound ACL record");
    assert_eq!(record.handle, input_handle);
    assert_eq!(record.app_account, authority);
    assert_eq!(record.nonce_key, nonce_key);
    assert!(!record.public_decrypt);
}

/// A signature from a key not in the configured signer set is rejected; no ACL is bound.
#[test]
fn mollusk_verify_coprocessor_input_and_bind_rejects_unauthorized_signer() {
    let program_id = host::id();
    let authority = Pubkey::new_unique();
    let configured = k256::ecdsa::SigningKey::from_bytes(&[0x44u8; 32].into()).unwrap();
    let attacker = k256::ecdsa::SigningKey::from_bytes(&[0x99u8; 32].into()).unwrap();
    let (host_config, host_config_account) =
        host_config_account_with_verifier(authority, evm_address_of(&configured));

    let acl_domain_key = Pubkey::new_unique();
    let label = label("balance");
    let nonce_key = host::acl_nonce_key(acl_domain_key, authority, label);
    let output_acl_record = host::acl_record_address(nonce_key, 0).0;

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

    let context = transient_context(
        authority,
        vec![
            (host_config, host_config_account),
            (output_acl_record, system_account(0)),
        ],
    );
    let ix = verify_coprocessor_input_and_bind_ix(
        program_id,
        authority,
        host_config,
        output_acl_record,
        input_handle,
        ct_handles,
        user_address,
        contract_address,
        extra_data,
        signatures,
        nonce_key,
        acl_domain_key,
        authority,
        label,
        vec![AclSubjectEntry::user(authority)],
    );

    let result = context.process_instruction(&ix);

    assert!(result.raw_result.is_err());
    assert!(read_acl_record(&context, output_acl_record).is_none());
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
    let context = transient_context(
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

#[test]
fn mollusk_destroy_kms_context_rejects_current_but_destroys_prior() {
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, host_config_account) =
        host_config_account_with_options(admin, admin, false, false, false);
    let (kc1, _) = host::kms_context_address(1);
    let (kc2, _) = host::kms_context_address(2);
    let context = transient_context(
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

fn computed_like_handle_for_chain(seed: u8, fhe_type: u8) -> [u8; 32] {
    let mut handle = handle_for_chain(seed, fhe_type);
    handle[21] = 0xff;
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
