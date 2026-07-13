//! Mollusk-based runtime tests for the RFC-024 `EncryptedValue` ACL model.
//!
//! Migrated from the old keyed-nonce `AclRecord`/`AclPermission` model (deleted
//! along with `assert_acl_record`, `allow_acl_subjects`, `commit_handle_material`,
//! and the single-op `fhe_*` instructions) to the new stateless-indexing
//! `EncryptedValue` lineage: durable outputs bound through `fhe_eval`,
//! `allow_subjects`, and `make_handle_public`; raw create/update are covered as
//! fail-closed ABI stubs. See `zama-host/src/state/encrypted_value.rs` and
//! `zama_solana_acl` for the model this exercises.
//!
//! Scope note: this migration focuses the suite on the ACL/MMR surface that
//! actually changed (`EncryptedValue` lifecycle + `fhe_eval` durable outputs +
//! lineage reconstruction/proofs). The old suite's coverage of instructions
//! untouched by the ACL rewrite (KMS context define/destroy, HCU limit
//! setters, delegation-for-user-decryption, host-admin pause/config,
//! grant-deny-list plumbing, material-commitment sealing, overflow-permission
//! PDAs) was dropped rather than ported 1:1 for this pass — see the final
//! migration report for the itemized list and reasons. Every dropped test's
//! instruction either still compiles unchanged (its own unit/doc tests in the
//! program crate keep covering it) or referenced a concept deleted by the
//! rewrite (`AclPermission` overflow, `AclRecord` material sealing) with no
//! surviving equivalent to port.

use anchor_lang::{
    prelude::system_program, AccountDeserialize, AccountSerialize, AnchorDeserialize,
    Discriminator, InstructionData, ToAccountMetas,
};
use mollusk_svm::{result::Check, Mollusk};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use std::{collections::HashMap, path::PathBuf};
use zama_host::{
    self as host, instructions::EncryptedValueSubjectGrant, DenySubjectRecord, EncryptedValue,
    FheBinaryOpCode, FheEvalArgs, FheEvalOperand, FheEvalOutput, FheEvalStep, FheTernaryOpCode,
    HostConfig,
};

// ---------------------------------------------------------------------------
// Harness
// ---------------------------------------------------------------------------

fn mollusk() -> Mollusk {
    let deploy_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/deploy");
    unsafe {
        std::env::set_var("SBF_OUT_DIR", deploy_dir);
    }
    let mut mollusk = Mollusk::new(&host::id(), "zama_host");
    // fhe_eval derives handle entropy from the previous bank hash: run at a
    // nonzero slot with a SlotHashes entry below it, like a real validator.
    mollusk.sysvars.clock.slot = 100;
    mollusk.sysvars.slot_hashes =
        solana_sdk::slot_hashes::SlotHashes::new(&[(99, solana_sdk::hash::Hash::new_unique())]);
    mollusk
}

fn mollusk_without_previous_bank_hash() -> Mollusk {
    let mut mollusk = mollusk();
    mollusk.sysvars.slot_hashes = solana_sdk::slot_hashes::SlotHashes::default();
    mollusk
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

fn serialized_account<T: AccountSerialize>(account: T) -> Vec<u8> {
    let mut data = Vec::new();
    account.try_serialize(&mut data).unwrap();
    data
}

fn label(name: &str) -> [u8; 32] {
    let mut out = [0_u8; 32];
    let bytes = name.as_bytes();
    assert!(bytes.len() <= out.len());
    out[..bytes.len()].copy_from_slice(bytes);
    out
}

fn handle_for_chain(seed: u8, fhe_type: u8) -> [u8; 32] {
    let mut handle = [seed; 32];
    handle[21] = 0;
    handle[22..30].copy_from_slice(&host::SOLANA_POC_CHAIN_ID.to_be_bytes());
    handle[30] = fhe_type;
    handle[31] = host::HANDLE_VERSION;
    handle
}

fn event_authority(program_id: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"__event_authority"], &program_id).0
}

const GATEWAY_CHAIN_ID: u64 = 31337;
const INPUT_VERIFICATION_CONTRACT: [u8; 20] = [0xCDu8; 20];
const DECRYPTION_CONTRACT: [u8; 20] = [0xDEu8; 20];

fn funded_system_account() -> Account {
    Account {
        lamports: 10_000_000_000,
        data: vec![],
        owner: system_program::ID,
        executable: false,
        rent_epoch: 0,
    }
}

fn system_program_account() -> Account {
    Account {
        lamports: 1,
        data: b"system_program".to_vec(),
        owner: solana_sdk::native_loader::ID,
        executable: true,
        rent_epoch: 0,
    }
}

fn empty_system_account() -> Account {
    Account {
        lamports: 0,
        data: vec![],
        owner: system_program::ID,
        executable: false,
        rent_epoch: 0,
    }
}

fn host_config_account_with_flags(
    admin: Pubkey,
    paused: bool,
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
                input_verifier_authority: admin,
                gateway_chain_id: GATEWAY_CHAIN_ID,
                input_verification_contract: INPUT_VERIFICATION_CONTRACT,
                coprocessor_signer: [0u8; 20],
                decryption_contract: DECRYPTION_CONTRACT,
                current_kms_context_id: 0,
                material_authority: admin,
                paused,
                grant_deny_list_enabled,
                max_hcu_per_tx: 0,
                max_hcu_depth_per_tx: 0,
                hcu_block_cap_per_app: u64::MAX,
                updated_slot: 0,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

fn host_config_account(admin: Pubkey) -> (Pubkey, Account) {
    host_config_account_with_flags(admin, false, false)
}

fn paused_host_config_account(admin: Pubkey) -> (Pubkey, Account) {
    host_config_account_with_flags(admin, true, false)
}

fn deny_enabled_host_config_account(admin: Pubkey) -> (Pubkey, Account) {
    host_config_account_with_flags(admin, false, true)
}

fn deny_subject_record_account(subject: Pubkey, denied: bool) -> (Pubkey, Account) {
    let (record, bump) = host::deny_subject_address(subject);
    (
        record,
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

/// Builds an `EncryptedValue` account (discriminator + body) for direct account-map seeding.
fn encrypted_value_account(value: &EncryptedValue) -> Account {
    Account {
        lamports: 10_000_000_000,
        data: serialized_account(value.clone()),
        owner: host::id(),
        executable: false,
        rent_epoch: 0,
    }
}

fn new_lineage(
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subjects: &[Pubkey],
) -> (Pubkey, EncryptedValue) {
    let value_key = zama_solana_acl::derive_value_key(
        acl_domain_key.to_bytes(),
        app_account.to_bytes(),
        encrypted_value_label,
    );
    let (address, bump) = host::encrypted_value_address(value_key);
    let value = EncryptedValue {
        acl_domain_key,
        app_account,
        encrypted_value_label,
        current_handle: handle,
        subjects: subjects.to_vec(),
        leaf_count: 0,
        peaks: Vec::new(),
        bump,
    };
    (address, value)
}

fn read_encrypted_value(
    result: &mollusk_svm::result::InstructionResult,
    address: Pubkey,
) -> EncryptedValue {
    let account = result
        .resulting_accounts
        .iter()
        .find(|(key, _)| *key == address)
        .map(|(_, account)| account)
        .expect("encrypted value account present in result");
    let mut data: &[u8] = &account.data;
    EncryptedValue::try_deserialize(&mut data).expect("valid EncryptedValue account")
}

fn read_encrypted_value_from_context(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> EncryptedValue {
    let account = context
        .account_store
        .borrow()
        .get(&address)
        .expect("encrypted value account present")
        .clone();
    let mut data: &[u8] = &account.data;
    EncryptedValue::try_deserialize(&mut data).expect("valid EncryptedValue account")
}

fn supersede_with_fhe_eval(
    payer: Pubkey,
    compute_subject: Pubkey,
    host_config: Pubkey,
    host_config_account: Account,
    address: Pubkey,
    value: &EncryptedValue,
    context_id_tag: u8,
) -> EncryptedValue {
    let args = FheEvalArgs {
        context_id: [context_id_tag; 32],
        steps: vec![FheEvalStep::TrivialEncrypt {
            plaintext: [context_id_tag; 32],
            fhe_type: 5,
            output: FheEvalOutput::AllowedDurable {
                output_encrypted_value_index: 0,
                output_app_account_authority_index: None,
                output_acl_domain_key: value.acl_domain_key,
                output_app_account: value.app_account,
                output_encrypted_value_label: value.encrypted_value_label,
                output_subjects: value
                    .subjects
                    .iter()
                    .copied()
                    .map(|pubkey| host::AclSubjectEntry { pubkey })
                    .collect(),
                previous_handle: Some(value.current_handle),
                previous_subjects: Some(value.subjects.clone()),
                make_public: false,
            },
        }],
    };
    let ix = fhe_eval_ix(
        payer,
        compute_subject,
        value.app_account,
        host_config,
        args,
        vec![writable(address)],
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (payer, funded_system_account()),
        (compute_subject, funded_system_account()),
        (value.app_account, funded_system_account()),
        (address, encrypted_value_account(value)),
        (host_config, host_config_account),
        (event_authority(host::id()), Account::default()),
    ];
    let result = mollusk().process_and_validate_instruction(&ix, &accounts, &[Check::success()]);
    read_encrypted_value(&result, address)
}

#[allow(clippy::too_many_arguments)]
fn expect_fhe_eval_supersede_error(
    payer: Pubkey,
    compute_subject: Pubkey,
    host_config: Pubkey,
    host_config_account: Account,
    address: Pubkey,
    value: &EncryptedValue,
    previous_handle: [u8; 32],
    previous_subjects: Vec<Pubkey>,
    output_subjects: Vec<host::AclSubjectEntry>,
    context_id_tag: u8,
    expected: Check<'static>,
) {
    let args = FheEvalArgs {
        context_id: [context_id_tag; 32],
        steps: vec![FheEvalStep::TrivialEncrypt {
            plaintext: [context_id_tag; 32],
            fhe_type: 5,
            output: FheEvalOutput::AllowedDurable {
                output_encrypted_value_index: 0,
                output_app_account_authority_index: None,
                output_acl_domain_key: value.acl_domain_key,
                output_app_account: value.app_account,
                output_encrypted_value_label: value.encrypted_value_label,
                output_subjects,
                previous_handle: Some(previous_handle),
                previous_subjects: Some(previous_subjects),
                make_public: false,
            },
        }],
    };
    let ix = fhe_eval_ix(
        payer,
        compute_subject,
        value.app_account,
        host_config,
        args,
        vec![writable(address)],
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (payer, funded_system_account()),
        (compute_subject, funded_system_account()),
        (value.app_account, funded_system_account()),
        (address, encrypted_value_account(value)),
        (host_config, host_config_account),
        (event_authority(host::id()), Account::default()),
    ];
    mollusk().process_and_validate_instruction(&ix, &accounts, &[expected]);
}

fn custom_error(error: host::errors::ZamaHostError) -> Check<'static> {
    Check::err(solana_sdk::program_error::ProgramError::Custom(
        anchor_lang::error::ERROR_CODE_OFFSET + error as u32,
    ))
}

// ---------------------------------------------------------------------------
// Instruction builders
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn create_encrypted_value_ix(
    payer: Pubkey,
    app_account_authority: Pubkey,
    encrypted_value: Pubkey,
    host_config: Pubkey,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subjects: Vec<EncryptedValueSubjectGrant>,
) -> Instruction {
    create_encrypted_value_ix_with_deny(
        payer,
        app_account_authority,
        encrypted_value,
        host_config,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        subjects,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
fn create_encrypted_value_ix_with_deny(
    payer: Pubkey,
    app_account_authority: Pubkey,
    encrypted_value: Pubkey,
    host_config: Pubkey,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subjects: Vec<EncryptedValueSubjectGrant>,
    deny_subject_record: Option<Pubkey>,
) -> Instruction {
    anchor_ix(
        host::id(),
        host::accounts::CreateEncryptedValue {
            payer,
            app_account_authority,
            encrypted_value,
            host_config,
            deny_subject_record,
            system_program: system_program::ID,
        },
        host::instruction::CreateEncryptedValue {
            acl_domain_key,
            app_account,
            encrypted_value_label,
            handle,
            subjects,
        },
    )
}

fn allow_subjects_ix(
    payer: Pubkey,
    authority: Pubkey,
    encrypted_value: Pubkey,
    host_config: Pubkey,
    subjects: Vec<EncryptedValueSubjectGrant>,
) -> Instruction {
    allow_subjects_ix_with_deny(
        payer,
        authority,
        encrypted_value,
        host_config,
        subjects,
        None,
    )
}

fn allow_subjects_ix_with_deny(
    payer: Pubkey,
    authority: Pubkey,
    encrypted_value: Pubkey,
    host_config: Pubkey,
    subjects: Vec<EncryptedValueSubjectGrant>,
    deny_subject_record: Option<Pubkey>,
) -> Instruction {
    anchor_ix(
        host::id(),
        host::accounts::AllowEncryptedValueSubjects {
            payer,
            authority,
            encrypted_value,
            host_config,
            deny_subject_record,
            system_program: system_program::ID,
        },
        host::instruction::AllowSubjects { subjects },
    )
}

fn remove_subject_ix(
    authority: Pubkey,
    encrypted_value: Pubkey,
    host_config: Pubkey,
    subject: Pubkey,
) -> Instruction {
    remove_subject_ix_with_deny(authority, encrypted_value, host_config, subject, None)
}

fn remove_subject_ix_with_deny(
    authority: Pubkey,
    encrypted_value: Pubkey,
    host_config: Pubkey,
    subject: Pubkey,
    deny_subject_record: Option<Pubkey>,
) -> Instruction {
    anchor_ix(
        host::id(),
        host::accounts::RemoveEncryptedValueSubject {
            authority,
            encrypted_value,
            host_config,
            deny_subject_record,
        },
        host::instruction::RemoveSubject { subject },
    )
}

fn update_encrypted_value_ix(
    payer: Pubkey,
    app_account_authority: Pubkey,
    encrypted_value: Pubkey,
    host_config: Pubkey,
    new_handle: [u8; 32],
    previous_handle: [u8; 32],
    previous_subjects: Vec<Pubkey>,
) -> Instruction {
    update_encrypted_value_ix_with_deny(
        payer,
        app_account_authority,
        encrypted_value,
        host_config,
        new_handle,
        previous_handle,
        previous_subjects,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
fn update_encrypted_value_ix_with_deny(
    payer: Pubkey,
    app_account_authority: Pubkey,
    encrypted_value: Pubkey,
    host_config: Pubkey,
    new_handle: [u8; 32],
    previous_handle: [u8; 32],
    previous_subjects: Vec<Pubkey>,
    deny_subject_record: Option<Pubkey>,
) -> Instruction {
    anchor_ix(
        host::id(),
        host::accounts::UpdateEncryptedValue {
            payer,
            app_account_authority,
            encrypted_value,
            host_config,
            deny_subject_record,
            system_program: system_program::ID,
        },
        host::instruction::UpdateEncryptedValue {
            new_handle,
            previous_handle,
            previous_subjects,
        },
    )
}

fn make_handle_public_ix(
    payer: Pubkey,
    authority: Pubkey,
    encrypted_value: Pubkey,
    host_config: Pubkey,
    handle: [u8; 32],
) -> Instruction {
    make_handle_public_ix_with_deny(payer, authority, encrypted_value, host_config, handle, None)
}

fn make_handle_public_ix_with_deny(
    payer: Pubkey,
    authority: Pubkey,
    encrypted_value: Pubkey,
    host_config: Pubkey,
    handle: [u8; 32],
    deny_subject_record: Option<Pubkey>,
) -> Instruction {
    anchor_ix(
        host::id(),
        host::accounts::MakeEncryptedValueHandlePublic {
            payer,
            authority,
            encrypted_value,
            host_config,
            deny_subject_record,
            system_program: system_program::ID,
        },
        host::instruction::MakeHandlePublic { handle },
    )
}

/// Builds an `fhe_eval` instruction. `remaining` accounts are appended in
/// order and referenced by index from `args`.
fn fhe_eval_ix(
    payer: Pubkey,
    compute_subject: Pubkey,
    app_account_authority: Pubkey,
    host_config: Pubkey,
    args: FheEvalArgs,
    remaining: Vec<AccountMeta>,
) -> Instruction {
    fhe_eval_ix_with_deny_records(
        payer,
        compute_subject,
        app_account_authority,
        host_config,
        args,
        remaining,
        Vec::new(),
    )
}

fn fhe_eval_ix_with_deny(
    payer: Pubkey,
    compute_subject: Pubkey,
    app_account_authority: Pubkey,
    host_config: Pubkey,
    args: FheEvalArgs,
    remaining: Vec<AccountMeta>,
    deny_subject_record: Option<Pubkey>,
) -> Instruction {
    fhe_eval_ix_with_deny_records(
        payer,
        compute_subject,
        app_account_authority,
        host_config,
        args,
        remaining,
        deny_subject_record.into_iter().collect(),
    )
}

fn fhe_eval_ix_with_deny_records(
    payer: Pubkey,
    compute_subject: Pubkey,
    app_account_authority: Pubkey,
    host_config: Pubkey,
    args: FheEvalArgs,
    remaining: Vec<AccountMeta>,
    deny_subject_records: Vec<Pubkey>,
) -> Instruction {
    let mut ix = anchor_ix(
        host::id(),
        host::accounts::FheEval {
            payer,
            compute_subject,
            app_account_authority,
            host_config,
            system_program: system_program::ID,
            // Unrestricted block cap (u64::MAX) in every existing fixture: block_cap
            // short-circuits before touching the optional accounts, so any signer works
            // and the two HCU witnesses stay absent.
            hcu_authority: app_account_authority,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(host::id()),
            program: host::id(),
        },
        host::instruction::FheEval { args },
    );
    ix.accounts.extend(remaining);
    ix.accounts
        .extend(deny_subject_records.into_iter().map(readonly));
    ix
}

fn writable(pubkey: Pubkey) -> AccountMeta {
    AccountMeta::new(pubkey, false)
}

fn readonly(pubkey: Pubkey) -> AccountMeta {
    AccountMeta::new_readonly(pubkey, false)
}

fn readonly_signer(pubkey: Pubkey) -> AccountMeta {
    AccountMeta::new_readonly(pubkey, true)
}

// ---------------------------------------------------------------------------
// disabled raw create_encrypted_value
// ---------------------------------------------------------------------------

#[test]
fn mollusk_create_encrypted_value_rejects_raw_handle_without_provenance() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let acl_domain_key = Pubkey::new_unique();
    let lbl = label("balance");
    let handle = handle_for_chain(1, 5);
    let subject = Pubkey::new_unique();

    let value_key =
        zama_solana_acl::derive_value_key(acl_domain_key.to_bytes(), authority.to_bytes(), lbl);
    let (encrypted_value, _bump) = host::encrypted_value_address(value_key);

    let ix = create_encrypted_value_ix(
        authority,
        authority,
        encrypted_value,
        host_config,
        acl_domain_key,
        authority,
        lbl,
        handle,
        vec![EncryptedValueSubjectGrant { subject }],
    );

    let accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (encrypted_value, empty_system_account()),
        (host_config, host_config_account),
    ];

    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::RawEncryptedValueLifecycleDisabled,
        )],
    );
}

#[test]
fn mollusk_fhe_eval_fails_closed_without_previous_bank_hash() {
    let authority = Pubkey::new_unique();
    let subject = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let (encrypted_value, _value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle_for_chain(1, 5),
        &[subject],
    );
    let args = FheEvalArgs {
        context_id: [1; 32],
        steps: vec![FheEvalStep::TrivialEncrypt {
            plaintext: [1; 32],
            fhe_type: 5,
            output: FheEvalOutput::AllowedDurable {
                output_encrypted_value_index: 0,
                output_app_account_authority_index: None,
                output_acl_domain_key: Pubkey::new_unique(),
                output_app_account: authority,
                output_encrypted_value_label: label("balance"),
                output_subjects: vec![host::AclSubjectEntry { pubkey: subject }],
                previous_handle: None,
                previous_subjects: None,
                make_public: false,
            },
        }],
    };
    let ix = fhe_eval_ix(
        authority,
        subject,
        authority,
        host_config,
        args,
        vec![writable(encrypted_value)],
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (subject, funded_system_account()),
        (encrypted_value, empty_system_account()),
        (host_config, host_config_account),
        (event_authority(host::id()), Account::default()),
    ];

    mollusk_without_previous_bank_hash().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::PreviousBankHashUnavailable,
        )],
    );
}

// ---------------------------------------------------------------------------
// allow_subjects
// ---------------------------------------------------------------------------

#[test]
fn mollusk_allow_subjects_adds_new_subject_and_is_idempotent() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let owner = Pubkey::new_unique();
    let new_subject = Pubkey::new_unique();
    let (address, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle_for_chain(2, 5),
        &[owner],
    );

    let ix = allow_subjects_ix(
        authority,
        owner,
        address,
        host_config,
        vec![
            EncryptedValueSubjectGrant {
                subject: new_subject,
            },
            EncryptedValueSubjectGrant { subject: owner },
        ],
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (owner, funded_system_account()),
        (address, encrypted_value_account(&value)),
        (host_config, host_config_account),
    ];
    let result = mollusk().process_and_validate_instruction(&ix, &accounts, &[Check::success()]);
    let updated = read_encrypted_value(&result, address);
    assert_eq!(updated.subjects, vec![owner, new_subject]);
    assert!(updated.has_subject(owner));
    assert!(updated.has_subject(new_subject));
    assert_eq!(updated.leaf_count, 0); // allow_subjects never appends leaves
}

#[test]
fn mollusk_allow_subjects_rejects_unallowed_authority() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let outsider = Pubkey::new_unique();
    let allowed = Pubkey::new_unique();
    let (address, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle_for_chain(2, 5),
        &[allowed],
    );
    let ix = allow_subjects_ix(
        authority,
        outsider,
        address,
        host_config,
        vec![EncryptedValueSubjectGrant {
            subject: Pubkey::new_unique(),
        }],
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (outsider, funded_system_account()),
        (address, encrypted_value_account(&value)),
        (host_config, host_config_account),
    ];
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::SubjectNotAllowed)],
    );
}

#[test]
fn mollusk_allow_subjects_rejects_ninth_distinct_subject() {
    let payer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let handle = handle_for_chain(2, 5);
    let (encrypted_value, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle,
        &[authority],
    );
    let context = mollusk().with_context(HashMap::from([
        (system_program::ID, system_program_account()),
        (payer, funded_system_account()),
        (authority, funded_system_account()),
        (encrypted_value, encrypted_value_account(&value)),
        (host_config, host_config_account),
    ]));

    let new_subjects = (0..zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS)
        .map(|_| Pubkey::new_unique())
        .collect::<Vec<_>>();
    for subject in new_subjects
        .iter()
        .take(zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS - 1)
    {
        let allow_ix = allow_subjects_ix(
            payer,
            authority,
            encrypted_value,
            host_config,
            vec![EncryptedValueSubjectGrant { subject: *subject }],
        );
        context.process_and_validate_instruction(&allow_ix, &[Check::success()]);
    }

    let capped = read_encrypted_value_from_context(&context, encrypted_value);
    assert_eq!(
        capped.subjects.len(),
        zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS
    );
    assert_eq!(capped.subjects[0], authority);
    assert_eq!(
        &capped.subjects[1..],
        &new_subjects[..zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS - 1]
    );
    assert_eq!(capped.current_handle, handle);
    assert_eq!(capped.leaf_count, 0);
    assert!(capped.peaks.is_empty());

    let rejected = allow_subjects_ix(
        payer,
        authority,
        encrypted_value,
        host_config,
        vec![EncryptedValueSubjectGrant {
            subject: new_subjects[zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS - 1],
        }],
    );
    context.process_and_validate_instruction(
        &rejected,
        &[custom_error(
            host::errors::ZamaHostError::EncryptedValueSubjectCapacityExceeded,
        )],
    );

    let after_reject = read_encrypted_value_from_context(&context, encrypted_value);
    assert_eq!(after_reject.subjects, capped.subjects);
    assert_eq!(after_reject.current_handle, capped.current_handle);
    assert_eq!(after_reject.leaf_count, capped.leaf_count);
    assert_eq!(after_reject.peaks, capped.peaks);
}

// ---------------------------------------------------------------------------
// remove_subject
// ---------------------------------------------------------------------------

#[test]
fn mollusk_remove_subject_removes_current_member_and_blocks_future_authority() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let owner = Pubkey::new_unique();
    let removed = Pubkey::new_unique();
    let (address, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle_for_chain(6, 5),
        &[owner, removed],
    );

    let ix = remove_subject_ix(owner, address, host_config, removed);
    let accounts = vec![
        (owner, funded_system_account()),
        (address, encrypted_value_account(&value)),
        (host_config, host_config_account.clone()),
    ];
    let result = mollusk().process_and_validate_instruction(&ix, &accounts, &[Check::success()]);
    let updated = read_encrypted_value(&result, address);
    assert_eq!(updated.subjects, vec![owner]);
    assert!(updated.has_subject(owner));
    assert!(!updated.has_subject(removed));
    assert_eq!(updated.leaf_count, 0);
    assert!(updated.peaks.is_empty());

    let rejected = allow_subjects_ix(
        authority,
        removed,
        address,
        host_config,
        vec![EncryptedValueSubjectGrant {
            subject: Pubkey::new_unique(),
        }],
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (removed, funded_system_account()),
        (address, encrypted_value_account(&updated)),
        (host_config, host_config_account),
    ];
    mollusk().process_and_validate_instruction(
        &rejected,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::SubjectNotAllowed)],
    );
}

#[test]
fn mollusk_remove_subject_rejects_absent_subject() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let owner = Pubkey::new_unique();
    let other = Pubkey::new_unique();
    let (address, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle_for_chain(6, 5),
        &[owner],
    );
    let ix = remove_subject_ix(owner, address, host_config, other);
    let accounts = vec![
        (owner, funded_system_account()),
        (address, encrypted_value_account(&value)),
        (host_config, host_config_account),
    ];
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::SubjectNotFound)],
    );
}

#[test]
fn mollusk_remove_subject_rejects_last_subject() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let owner = Pubkey::new_unique();
    let (address, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle_for_chain(6, 5),
        &[owner],
    );
    let ix = remove_subject_ix(owner, address, host_config, owner);
    let accounts = vec![
        (owner, funded_system_account()),
        (address, encrypted_value_account(&value)),
        (host_config, host_config_account),
    ];
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::EncryptedValueLastSubject,
        )],
    );
}

#[test]
fn mollusk_removed_subject_gets_no_historical_leaf_when_later_superseded() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let owner = Pubkey::new_unique();
    let removed = Pubkey::new_unique();
    let handle0 = handle_for_chain(7, 5);
    let (address, value0) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle0,
        &[owner, removed],
    );

    let remove_ix = remove_subject_ix(owner, address, host_config, removed);
    let accounts0 = vec![
        (owner, funded_system_account()),
        (address, encrypted_value_account(&value0)),
        (host_config, host_config_account.clone()),
    ];
    let result0 =
        mollusk().process_and_validate_instruction(&remove_ix, &accounts0, &[Check::success()]);
    let value_after_remove = read_encrypted_value(&result0, address);
    assert_eq!(value_after_remove.subjects, vec![owner]);

    let updated = supersede_with_fhe_eval(
        authority,
        owner,
        host_config,
        host_config_account,
        address,
        &value_after_remove,
        8,
    );
    assert_eq!(updated.leaf_count, 1);

    let expected_leaf = zama_solana_acl::historical_access_leaf_commitment(
        address.to_bytes(),
        0,
        handle0,
        owner.to_bytes(),
    );
    let mut expected_peaks = Vec::new();
    let mut expected_count = 0u64;
    zama_solana_acl::mmr_append(&mut expected_peaks, &mut expected_count, expected_leaf).unwrap();
    assert_eq!(updated.peaks, expected_peaks);

    let events = [zama_solana_acl::lineage::LineageEvent::handle_superseded(
        handle0,
        &[owner.to_bytes()],
    )];
    let proof = zama_solana_acl::lineage::build_verified_proof_from_events(
        address.to_bytes(),
        &events,
        &updated.peaks,
        updated.leaf_count,
        0,
    )
    .unwrap();
    let shared = updated.to_shared();
    assert!(zama_solana_acl::authorize_historical(
        address.to_bytes(),
        &shared,
        handle0,
        owner.to_bytes(),
        &proof,
    )
    .is_ok());
    assert!(zama_solana_acl::authorize_historical(
        address.to_bytes(),
        &shared,
        handle0,
        removed.to_bytes(),
        &proof,
    )
    .is_err());
}

#[test]
fn mollusk_subject_retains_historical_access_sealed_before_removal() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let owner = Pubkey::new_unique();
    let removed = Pubkey::new_unique();
    let handle0 = handle_for_chain(9, 5);
    let (address, value0) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle0,
        &[owner, removed],
    );

    let value1 = supersede_with_fhe_eval(
        authority,
        owner,
        host_config,
        host_config_account.clone(),
        address,
        &value0,
        10,
    );
    assert_eq!(value1.leaf_count, 2);

    let remove_ix = remove_subject_ix(owner, address, host_config, removed);
    let accounts1 = vec![
        (owner, funded_system_account()),
        (address, encrypted_value_account(&value1)),
        (host_config, host_config_account),
    ];
    let result1 =
        mollusk().process_and_validate_instruction(&remove_ix, &accounts1, &[Check::success()]);
    let final_value = read_encrypted_value(&result1, address);
    assert_eq!(final_value.subjects, vec![owner]);
    assert_eq!(final_value.leaf_count, 2);

    let events = [zama_solana_acl::lineage::LineageEvent::handle_superseded(
        handle0,
        &[owner.to_bytes(), removed.to_bytes()],
    )];
    let proof = zama_solana_acl::lineage::build_verified_proof_from_events(
        address.to_bytes(),
        &events,
        &final_value.peaks,
        final_value.leaf_count,
        1,
    )
    .unwrap();
    assert!(zama_solana_acl::authorize_historical(
        address.to_bytes(),
        &final_value.to_shared(),
        handle0,
        removed.to_bytes(),
        &proof,
    )
    .is_ok());
}

// ---------------------------------------------------------------------------
// Durable supersession + disabled raw update_encrypted_value (item 2c/2d)
// ---------------------------------------------------------------------------

#[test]
fn mollusk_fhe_eval_supersedes_and_appends_allowed_subject_leaves() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let subject_a = Pubkey::new_unique();
    let subject_b = Pubkey::new_unique();
    let old_handle = handle_for_chain(3, 5);
    let (address, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        old_handle,
        &[subject_a, subject_b],
    );

    let updated = supersede_with_fhe_eval(
        authority,
        subject_a,
        host_config,
        host_config_account,
        address,
        &value,
        4,
    );
    assert_ne!(updated.current_handle, old_handle);
    assert_eq!(updated.leaf_count, 2);

    let mut expected_peaks = Vec::new();
    let mut expected_count = 0u64;
    for (index, subject) in [subject_a, subject_b].iter().enumerate() {
        let expected_leaf = zama_solana_acl::historical_access_leaf_commitment(
            address.to_bytes(),
            index as u64,
            old_handle,
            subject.to_bytes(),
        );
        zama_solana_acl::mmr_append(&mut expected_peaks, &mut expected_count, expected_leaf)
            .unwrap();
    }
    assert_eq!(updated.peaks, expected_peaks);
}

#[test]
fn mollusk_update_encrypted_value_rejects_raw_handle_without_provenance() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let subject = Pubkey::new_unique();
    let old_handle = handle_for_chain(3, 5);
    let (address, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        old_handle,
        &[subject],
    );

    let ix = update_encrypted_value_ix(
        authority,
        authority,
        address,
        host_config,
        handle_for_chain(4, 5),
        old_handle,
        value.subjects.clone(),
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (address, encrypted_value_account(&value)),
        (host_config, host_config_account),
    ];
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::RawEncryptedValueLifecycleDisabled,
        )],
    );
}

#[test]
fn mollusk_fhe_eval_rejects_stale_previous_subjects() {
    // Item 2c: submitting stale previous_subjects through the real durable-output path.
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let subject = Pubkey::new_unique();
    let old_handle = handle_for_chain(3, 5);
    let (address, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        old_handle,
        &[subject],
    );

    expect_fhe_eval_supersede_error(
        authority,
        subject,
        host_config,
        host_config_account,
        address,
        &value,
        old_handle,
        vec![Pubkey::new_unique()], // stale/wrong previous_subjects
        value
            .subjects
            .iter()
            .copied()
            .map(|pubkey| host::AclSubjectEntry { pubkey })
            .collect(),
        5,
        custom_error(host::errors::ZamaHostError::PreviousStateMismatch),
    );
}

#[test]
fn mollusk_fhe_eval_rejects_stale_previous_handle() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let subject = Pubkey::new_unique();
    let old_handle = handle_for_chain(3, 5);
    let (address, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        old_handle,
        &[subject],
    );
    expect_fhe_eval_supersede_error(
        authority,
        subject,
        host_config,
        host_config_account,
        address,
        &value,
        handle_for_chain(99, 5), // wrong previous_handle
        value.subjects.clone(),
        value
            .subjects
            .iter()
            .copied()
            .map(|pubkey| host::AclSubjectEntry { pubkey })
            .collect(),
        6,
        custom_error(host::errors::ZamaHostError::PreviousStateMismatch),
    );
}

// ---------------------------------------------------------------------------
// make_handle_public
// ---------------------------------------------------------------------------

#[test]
fn mollusk_make_handle_public_appends_public_decrypt_leaf() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let subject = Pubkey::new_unique();
    let handle = handle_for_chain(5, 5);
    let (address, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle,
        &[subject],
    );
    let ix = make_handle_public_ix(authority, subject, address, host_config, handle);
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (subject, funded_system_account()),
        (address, encrypted_value_account(&value)),
        (host_config, host_config_account),
    ];
    let result = mollusk().process_and_validate_instruction(&ix, &accounts, &[Check::success()]);
    let updated = read_encrypted_value(&result, address);
    assert_eq!(updated.leaf_count, 1);
    let expected = zama_solana_acl::public_decrypt_leaf_commitment(address.to_bytes(), 0, handle);
    let mut expected_peaks = Vec::new();
    let mut expected_count = 0u64;
    zama_solana_acl::mmr_append(&mut expected_peaks, &mut expected_count, expected).unwrap();
    assert_eq!(updated.peaks, expected_peaks);
}

#[test]
fn mollusk_make_handle_public_rejects_wrong_expected_handle() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let subject = Pubkey::new_unique();
    let handle = handle_for_chain(5, 5);
    let wrong_handle = handle_for_chain(6, 5);
    let (address, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle,
        &[subject],
    );
    let ix = make_handle_public_ix(authority, subject, address, host_config, wrong_handle);
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (subject, funded_system_account()),
        (address, encrypted_value_account(&value)),
        (host_config, host_config_account),
    ];
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::EncryptedValuePublicHandleMismatch,
        )],
    );
}

#[test]
fn mollusk_make_handle_public_rejects_unallowed_subject() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let allowed = Pubkey::new_unique();
    let subject = Pubkey::new_unique();
    let handle = handle_for_chain(5, 5);
    let (address, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle,
        &[allowed],
    );
    let ix = make_handle_public_ix(authority, subject, address, host_config, handle);
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (subject, funded_system_account()),
        (address, encrypted_value_account(&value)),
        (host_config, host_config_account),
    ];
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::SubjectNotAllowed)],
    );
}

// ---------------------------------------------------------------------------
// Deny-list and pause gates
// ---------------------------------------------------------------------------

#[test]
fn mollusk_denied_caller_cannot_mutate_acl_update_or_eval_output() {
    let caller = Pubkey::new_unique();
    let (host_config, host_config_account) = deny_enabled_host_config_account(caller);
    let (deny_record, deny_record_account) = deny_subject_record_account(caller, true);
    let other = Pubkey::new_unique();

    let acl_domain_key = Pubkey::new_unique();
    let create_label = label("deny-create");
    let create_value_key = zama_solana_acl::derive_value_key(
        acl_domain_key.to_bytes(),
        caller.to_bytes(),
        create_label,
    );
    let (create_address, _bump) = host::encrypted_value_address(create_value_key);
    let create_ix = create_encrypted_value_ix_with_deny(
        caller,
        caller,
        create_address,
        host_config,
        acl_domain_key,
        caller,
        create_label,
        handle_for_chain(50, 5),
        vec![EncryptedValueSubjectGrant { subject: caller }],
        Some(deny_record),
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (caller, funded_system_account()),
        (create_address, empty_system_account()),
        (host_config, host_config_account.clone()),
        (deny_record, deny_record_account.clone()),
    ];
    mollusk().process_and_validate_instruction(
        &create_ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::RawEncryptedValueLifecycleDisabled,
        )],
    );

    let (allow_address, allow_value) = new_lineage(
        Pubkey::new_unique(),
        caller,
        label("deny-allow"),
        handle_for_chain(51, 5),
        &[caller],
    );
    let allow_ix = allow_subjects_ix_with_deny(
        caller,
        caller,
        allow_address,
        host_config,
        vec![EncryptedValueSubjectGrant { subject: other }],
        Some(deny_record),
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (caller, funded_system_account()),
        (allow_address, encrypted_value_account(&allow_value)),
        (host_config, host_config_account.clone()),
        (deny_record, deny_record_account.clone()),
    ];
    mollusk().process_and_validate_instruction(
        &allow_ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::AclSubjectDenied)],
    );

    let make_public_ix = make_handle_public_ix_with_deny(
        caller,
        caller,
        allow_address,
        host_config,
        allow_value.current_handle,
        Some(deny_record),
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (caller, funded_system_account()),
        (allow_address, encrypted_value_account(&allow_value)),
        (host_config, host_config_account.clone()),
        (deny_record, deny_record_account.clone()),
    ];
    mollusk().process_and_validate_instruction(
        &make_public_ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::AclSubjectDenied)],
    );

    let (remove_address, remove_value) = new_lineage(
        Pubkey::new_unique(),
        caller,
        label("deny-remove"),
        handle_for_chain(52, 5),
        &[caller, other],
    );
    let remove_ix = remove_subject_ix_with_deny(
        caller,
        remove_address,
        host_config,
        other,
        Some(deny_record),
    );
    let accounts = vec![
        (caller, funded_system_account()),
        (remove_address, encrypted_value_account(&remove_value)),
        (host_config, host_config_account.clone()),
        (deny_record, deny_record_account.clone()),
    ];
    mollusk().process_and_validate_instruction(
        &remove_ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::AclSubjectDenied)],
    );

    let old_handle = handle_for_chain(53, 5);
    let (update_address, update_value) = new_lineage(
        Pubkey::new_unique(),
        caller,
        label("deny-update"),
        old_handle,
        &[caller],
    );
    let update_ix = update_encrypted_value_ix_with_deny(
        caller,
        caller,
        update_address,
        host_config,
        handle_for_chain(54, 5),
        old_handle,
        update_value.subjects.clone(),
        Some(deny_record),
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (caller, funded_system_account()),
        (update_address, encrypted_value_account(&update_value)),
        (host_config, host_config_account.clone()),
        (deny_record, deny_record_account.clone()),
    ];
    mollusk().process_and_validate_instruction(
        &update_ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::RawEncryptedValueLifecycleDisabled,
        )],
    );

    let output_label = label("deny-eval");
    let output_value_key =
        zama_solana_acl::derive_value_key(caller.to_bytes(), caller.to_bytes(), output_label);
    let (output_address, _bump) = host::encrypted_value_address(output_value_key);
    let args = FheEvalArgs {
        context_id: [9; 32],
        steps: vec![FheEvalStep::TrivialEncrypt {
            plaintext: [1; 32],
            fhe_type: 5,
            output: FheEvalOutput::AllowedDurable {
                output_encrypted_value_index: 0,
                output_app_account_authority_index: None,
                output_acl_domain_key: caller,
                output_app_account: caller,
                output_encrypted_value_label: output_label,
                output_subjects: vec![host::AclSubjectEntry { pubkey: caller }],
                previous_handle: None,
                previous_subjects: None,
                make_public: false,
            },
        }],
    };
    let eval_ix = fhe_eval_ix_with_deny(
        caller,
        caller,
        caller,
        host_config,
        args,
        vec![writable(output_address)],
        Some(deny_record),
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (caller, funded_system_account()),
        (host_config, host_config_account),
        (deny_record, deny_record_account),
        (event_authority(host::id()), Account::default()),
        (output_address, empty_system_account()),
    ];
    mollusk().process_and_validate_instruction(
        &eval_ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::AclSubjectDenied)],
    );
}

#[test]
fn mollusk_fhe_eval_rejects_denied_second_output_authority_in_multi_output_frame() {
    let authority_a = Pubkey::new_unique();
    let authority_b = Pubkey::new_unique();
    let (host_config, host_config_account) = deny_enabled_host_config_account(authority_a);
    let (deny_a, deny_a_account) = (
        host::deny_subject_address(authority_a).0,
        empty_system_account(),
    );
    let (deny_b, deny_b_account) = deny_subject_record_account(authority_b, true);
    let output_a_label = label("multi-deny-a");
    let output_b_label = label("multi-deny-b");
    let output_a = host::encrypted_value_address(zama_solana_acl::derive_value_key(
        authority_a.to_bytes(),
        authority_a.to_bytes(),
        output_a_label,
    ))
    .0;
    let output_b = host::encrypted_value_address(zama_solana_acl::derive_value_key(
        authority_b.to_bytes(),
        authority_b.to_bytes(),
        output_b_label,
    ))
    .0;
    let args = FheEvalArgs {
        context_id: [11; 32],
        steps: vec![
            FheEvalStep::TrivialEncrypt {
                plaintext: [3; 32],
                fhe_type: 5,
                output: FheEvalOutput::AllowedDurable {
                    output_encrypted_value_index: 0,
                    output_app_account_authority_index: None,
                    output_acl_domain_key: authority_a,
                    output_app_account: authority_a,
                    output_encrypted_value_label: output_a_label,
                    output_subjects: vec![host::AclSubjectEntry {
                        pubkey: authority_a,
                    }],
                    previous_handle: None,
                    previous_subjects: None,
                    make_public: false,
                },
            },
            FheEvalStep::TrivialEncrypt {
                plaintext: [4; 32],
                fhe_type: 5,
                output: FheEvalOutput::AllowedDurable {
                    output_encrypted_value_index: 1,
                    output_app_account_authority_index: Some(2),
                    output_acl_domain_key: authority_b,
                    output_app_account: authority_b,
                    output_encrypted_value_label: output_b_label,
                    output_subjects: vec![host::AclSubjectEntry {
                        pubkey: authority_b,
                    }],
                    previous_handle: None,
                    previous_subjects: None,
                    make_public: false,
                },
            },
        ],
    };
    let ix = fhe_eval_ix_with_deny_records(
        authority_a,
        authority_a,
        authority_a,
        host_config,
        args,
        vec![
            writable(output_a),
            writable(output_b),
            readonly_signer(authority_b),
        ],
        vec![deny_a, deny_b],
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (authority_a, funded_system_account()),
        (authority_b, funded_system_account()),
        (host_config, host_config_account),
        (event_authority(host::id()), Account::default()),
        (output_a, empty_system_account()),
        (output_b, empty_system_account()),
        (deny_a, deny_a_account),
        (deny_b, deny_b_account),
    ];

    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::AclSubjectDenied)],
    );
}

#[test]
fn mollusk_paused_state_blocks_acl_update_and_eval_output() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = paused_host_config_account(authority);
    let owner = Pubkey::new_unique();
    let other = Pubkey::new_unique();

    let (allow_address, allow_value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("pause-allow"),
        handle_for_chain(55, 5),
        &[owner],
    );
    let allow_ix = allow_subjects_ix(
        authority,
        owner,
        allow_address,
        host_config,
        vec![EncryptedValueSubjectGrant { subject: other }],
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (owner, funded_system_account()),
        (allow_address, encrypted_value_account(&allow_value)),
        (host_config, host_config_account.clone()),
    ];
    mollusk().process_and_validate_instruction(
        &allow_ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::HostConfigPaused)],
    );

    let remove_ix = remove_subject_ix(owner, allow_address, host_config, other);
    let accounts = vec![
        (owner, funded_system_account()),
        (allow_address, encrypted_value_account(&allow_value)),
        (host_config, host_config_account.clone()),
    ];
    mollusk().process_and_validate_instruction(
        &remove_ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::HostConfigPaused)],
    );

    let update_ix = update_encrypted_value_ix(
        authority,
        authority,
        allow_address,
        host_config,
        handle_for_chain(56, 5),
        allow_value.current_handle,
        allow_value.subjects.clone(),
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (allow_address, encrypted_value_account(&allow_value)),
        (host_config, host_config_account.clone()),
    ];
    mollusk().process_and_validate_instruction(
        &update_ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::RawEncryptedValueLifecycleDisabled,
        )],
    );

    let output_label = label("pause-eval");
    let output_value_key =
        zama_solana_acl::derive_value_key(authority.to_bytes(), authority.to_bytes(), output_label);
    let (output_address, _bump) = host::encrypted_value_address(output_value_key);
    let args = FheEvalArgs {
        context_id: [10; 32],
        steps: vec![FheEvalStep::TrivialEncrypt {
            plaintext: [2; 32],
            fhe_type: 5,
            output: FheEvalOutput::AllowedDurable {
                output_encrypted_value_index: 0,
                output_app_account_authority_index: None,
                output_acl_domain_key: authority,
                output_app_account: authority,
                output_encrypted_value_label: output_label,
                output_subjects: vec![host::AclSubjectEntry { pubkey: owner }],
                previous_handle: None,
                previous_subjects: None,
                make_public: false,
            },
        }],
    };
    let eval_ix = fhe_eval_ix(
        authority,
        owner,
        authority,
        host_config,
        args,
        vec![writable(output_address)],
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (owner, funded_system_account()),
        (host_config, host_config_account),
        (event_authority(host::id()), Account::default()),
        (output_address, empty_system_account()),
    ];
    mollusk().process_and_validate_instruction(
        &eval_ix,
        &accounts,
        &[custom_error(host::errors::ZamaHostError::HostConfigPaused)],
    );
}

// ---------------------------------------------------------------------------
// Item 2a: supersession lineage end-to-end against zama_solana_acl::lineage::reconstruct
// ---------------------------------------------------------------------------

#[test]
fn mollusk_supersession_lineage_matches_offchain_reconstruction() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let subject_a = Pubkey::new_unique();
    let subject_b = Pubkey::new_unique();
    let handle0 = handle_for_chain(10, 5);
    let (address, value0) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle0,
        &[subject_a, subject_b],
    );

    let value1 = supersede_with_fhe_eval(
        authority,
        subject_a,
        host_config,
        host_config_account.clone(),
        address,
        &value0,
        11,
    );

    let value2 = supersede_with_fhe_eval(
        authority,
        subject_a,
        host_config,
        host_config_account,
        address,
        &value1,
        12,
    );

    // Rebuild the HandleSuperseded events purely from the two instructions' own
    // previous_handle/previous_subjects args, exactly as an off-chain indexer would.
    let events = [
        zama_solana_acl::lineage::LineageEvent::handle_superseded(
            handle0,
            &value0
                .subjects
                .iter()
                .map(|p| p.to_bytes())
                .collect::<Vec<_>>(),
        ),
        zama_solana_acl::lineage::LineageEvent::handle_superseded(
            value1.current_handle,
            &value1
                .subjects
                .iter()
                .map(|p| p.to_bytes())
                .collect::<Vec<_>>(),
        ),
    ];
    let reconstructed = zama_solana_acl::lineage::reconstruct(address.to_bytes(), &events).unwrap();
    assert!(reconstructed.peaks_match(&value2.peaks, value2.leaf_count));
    assert_eq!(reconstructed.leaf_count, 4); // 2 subjects x 2 supersessions
}

// ---------------------------------------------------------------------------
// Item 2b: historical + public-decrypt proof round-trip, incl. no-roll-forward
// ---------------------------------------------------------------------------

#[test]
fn mollusk_historical_proof_round_trip_after_two_supersessions() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let subject = Pubkey::new_unique();
    let other_subject = Pubkey::new_unique();
    let handle0 = handle_for_chain(20, 5);
    let (address, value0) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle0,
        &[subject],
    );

    let value1 = supersede_with_fhe_eval(
        authority,
        subject,
        host_config,
        host_config_account.clone(),
        address,
        &value0,
        21,
    );

    let value2 = supersede_with_fhe_eval(
        authority,
        subject,
        host_config,
        host_config_account,
        address,
        &value1,
        22,
    );

    let events = [
        zama_solana_acl::lineage::LineageEvent::handle_superseded(
            handle0,
            &value0
                .subjects
                .iter()
                .map(|p| p.to_bytes())
                .collect::<Vec<_>>(),
        ),
        zama_solana_acl::lineage::LineageEvent::handle_superseded(
            value1.current_handle,
            &value1
                .subjects
                .iter()
                .map(|p| p.to_bytes())
                .collect::<Vec<_>>(),
        ),
    ];

    // Leaf 0 authorizes (handle0, subject) historically against the live peaks.
    let proof0 = zama_solana_acl::lineage::build_verified_proof_from_events(
        address.to_bytes(),
        &events,
        &value2.peaks,
        value2.leaf_count,
        0,
    )
    .unwrap();
    let shared_value2 = value2.to_shared();
    assert!(zama_solana_acl::authorize_historical(
        address.to_bytes(),
        &shared_value2,
        handle0,
        subject.to_bytes(),
        &proof0,
    )
    .is_ok());
    // Wrong subject rejected.
    assert!(zama_solana_acl::authorize_historical(
        address.to_bytes(),
        &shared_value2,
        handle0,
        other_subject.to_bytes(),
        &proof0,
    )
    .is_err());
    // Wrong handle rejected.
    assert!(zama_solana_acl::authorize_historical(
        address.to_bytes(),
        &shared_value2,
        value1.current_handle,
        subject.to_bytes(),
        &proof0,
    )
    .is_err());
}

#[test]
fn mollusk_public_decrypt_proof_has_no_roll_forward() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let subject = Pubkey::new_unique();
    let handle0 = handle_for_chain(30, 5);
    let (address, value0) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle0,
        &[subject],
    );

    let make_public_ix = make_handle_public_ix(authority, subject, address, host_config, handle0);
    let accounts0 = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (subject, funded_system_account()),
        (address, encrypted_value_account(&value0)),
        (host_config, host_config_account.clone()),
    ];
    let result0 = mollusk().process_and_validate_instruction(
        &make_public_ix,
        &accounts0,
        &[Check::success()],
    );
    let value_public = read_encrypted_value(&result0, address);

    let final_value = supersede_with_fhe_eval(
        authority,
        subject,
        host_config,
        host_config_account,
        address,
        &value_public,
        31,
    );

    let events = [
        zama_solana_acl::lineage::LineageEvent::MarkedPublic { handle: handle0 },
        zama_solana_acl::lineage::LineageEvent::handle_superseded(
            handle0,
            &value_public
                .subjects
                .iter()
                .map(|p| p.to_bytes())
                .collect::<Vec<_>>(),
        ),
    ];
    let proof = zama_solana_acl::lineage::build_verified_proof_from_events(
        address.to_bytes(),
        &events,
        &final_value.peaks,
        final_value.leaf_count,
        0,
    )
    .unwrap();
    let shared_final = final_value.to_shared();
    assert!(
        zama_solana_acl::authorize_public(address.to_bytes(), &shared_final, handle0, &proof)
            .is_ok()
    );
    // A proof built for the old handle never authorizes the newer handle: no roll-forward.
    assert!(zama_solana_acl::authorize_public(
        address.to_bytes(),
        &shared_final,
        final_value.current_handle,
        &proof
    )
    .is_err());
}

// ---------------------------------------------------------------------------
// fhe_eval: durable output create + supersede through the real CPI-free path
// ---------------------------------------------------------------------------

#[test]
fn mollusk_fhe_eval_creates_durable_output_from_local_binary_add() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let lhs = handle_for_chain(40, 5);
    let rhs = handle_for_chain(41, 5);
    let (lhs_address, lhs_value) =
        new_lineage(authority, authority, label("lhs"), lhs, &[authority]);
    let (rhs_address, rhs_value) =
        new_lineage(authority, authority, label("rhs"), rhs, &[authority]);
    let output_acl_domain_key = authority;
    let output_app_account = authority;
    let output_label = label("sum");
    let output_value_key = zama_solana_acl::derive_value_key(
        output_acl_domain_key.to_bytes(),
        output_app_account.to_bytes(),
        output_label,
    );
    let (output_address, _bump) = host::encrypted_value_address(output_value_key);

    let args = FheEvalArgs {
        context_id: [7; 32],
        steps: vec![FheEvalStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheEvalOperand::AllowedDurable {
                handle: lhs,
                encrypted_value_index: 0,
            },
            rhs: FheEvalOperand::AllowedDurable {
                handle: rhs,
                encrypted_value_index: 1,
            },
            output_fhe_type: 5,
            output: FheEvalOutput::AllowedDurable {
                output_encrypted_value_index: 2,
                output_app_account_authority_index: None,
                output_acl_domain_key,
                output_app_account,
                output_encrypted_value_label: output_label,
                output_subjects: vec![host::AclSubjectEntry { pubkey: authority }],
                previous_handle: None,
                previous_subjects: None,
                make_public: false,
            },
        }],
    };

    let ix = fhe_eval_ix(
        authority,
        authority,
        authority,
        host_config,
        args,
        vec![
            writable(lhs_address),
            writable(rhs_address),
            writable(output_address),
        ],
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (host_config, host_config_account),
        (event_authority(host::id()), Account::default()),
        (lhs_address, encrypted_value_account(&lhs_value)),
        (rhs_address, encrypted_value_account(&rhs_value)),
        (output_address, empty_system_account()),
    ];
    let result = mollusk().process_and_validate_instruction(&ix, &accounts, &[Check::success()]);
    let output = read_encrypted_value(&result, output_address);
    assert_eq!(output.subjects, vec![authority]);
    assert_eq!(output.leaf_count, 0);
}

#[test]
fn mollusk_fhe_eval_supersedes_durable_output_with_previous_state() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let input_handle = handle_for_chain(42, 5);
    let (input_address, input_value) = new_lineage(
        authority,
        authority,
        label("in"),
        input_handle,
        &[authority],
    );
    let output_handle = handle_for_chain(43, 5);
    let (output_address, output_value) = new_lineage(
        authority,
        authority,
        label("out"),
        output_handle,
        &[authority],
    );
    let previous_subjects = output_value.subjects.clone();

    let args = FheEvalArgs {
        context_id: [8; 32],
        steps: vec![FheEvalStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheEvalOperand::AllowedDurable {
                handle: input_handle,
                encrypted_value_index: 0,
            },
            rhs: FheEvalOperand::Scalar([0; 32]),
            output_fhe_type: 5,
            output: FheEvalOutput::AllowedDurable {
                output_encrypted_value_index: 1,
                output_app_account_authority_index: None,
                output_acl_domain_key: authority,
                output_app_account: authority,
                output_encrypted_value_label: label("out"),
                output_subjects: vec![host::AclSubjectEntry { pubkey: authority }],
                previous_handle: Some(output_handle),
                previous_subjects: Some(previous_subjects),
                make_public: false,
            },
        }],
    };

    let ix = fhe_eval_ix(
        authority,
        authority,
        authority,
        host_config,
        args,
        vec![writable(input_address), writable(output_address)],
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (host_config, host_config_account),
        (event_authority(host::id()), Account::default()),
        (input_address, encrypted_value_account(&input_value)),
        (output_address, encrypted_value_account(&output_value)),
    ];
    let result = mollusk().process_and_validate_instruction(&ix, &accounts, &[Check::success()]);
    let updated_output = read_encrypted_value(&result, output_address);
    assert_ne!(updated_output.current_handle, output_handle);
    // Supersession appends one historical leaf for the sole USE subject.
    assert_eq!(updated_output.leaf_count, 1);
}

// ---------------------------------------------------------------------------
// fhe_eval: narrow produced-public lifecycle batch
// ---------------------------------------------------------------------------

struct BornPublicFrame {
    instruction: Instruction,
    accounts: Vec<(Pubkey, Account)>,
    outputs: Vec<(u16, Pubkey)>,
}

fn born_public_frame(step_count: usize, born_public_steps: &[usize]) -> BornPublicFrame {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let mut output_metas = Vec::new();
    let mut output_accounts = Vec::new();
    let mut outputs = Vec::new();
    let mut steps = Vec::with_capacity(step_count);

    for step_index in 0..step_count {
        let output = if born_public_steps.contains(&step_index) {
            let output_label = label(&format!("born-public-{step_index}"));
            let value_key = zama_solana_acl::derive_value_key(
                authority.to_bytes(),
                authority.to_bytes(),
                output_label,
            );
            let output_address = host::encrypted_value_address(value_key).0;
            let output_index = output_metas.len() as u16;
            output_metas.push(writable(output_address));
            output_accounts.push((output_address, empty_system_account()));
            outputs.push((step_index as u16, output_address));
            FheEvalOutput::AllowedDurable {
                output_encrypted_value_index: output_index,
                output_app_account_authority_index: None,
                output_acl_domain_key: authority,
                output_app_account: authority,
                output_encrypted_value_label: output_label,
                output_subjects: vec![host::AclSubjectEntry { pubkey: authority }],
                previous_handle: None,
                previous_subjects: None,
                make_public: true,
            }
        } else {
            FheEvalOutput::AllowedLocal
        };
        steps.push(FheEvalStep::TrivialEncrypt {
            plaintext: [(step_index + 1) as u8; 32],
            fhe_type: 5,
            output,
        });
    }

    let instruction = fhe_eval_ix(
        authority,
        authority,
        authority,
        host_config,
        FheEvalArgs {
            context_id: label("born-public-frame"),
            steps,
        },
        output_metas,
    );
    let mut accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (host_config, host_config_account),
        (event_authority(host::id()), Account::default()),
    ];
    accounts.extend(output_accounts);
    BornPublicFrame {
        instruction,
        accounts,
        outputs,
    }
}

fn born_public_events(
    result: &mollusk_svm::result::InstructionResult,
) -> Vec<host::PublicOutputsProducedEvent> {
    let message = result.message.as_ref().expect("compiled Mollusk message");
    let account_keys = message.account_keys();
    let prefix = anchor_lang::event::EVENT_IX_TAG_LE
        .iter()
        .copied()
        .chain(
            host::PublicOutputsProducedEvent::DISCRIMINATOR
                .iter()
                .copied(),
        )
        .collect::<Vec<_>>();
    result
        .inner_instructions
        .iter()
        .filter_map(|inner| {
            if account_keys.get(inner.instruction.program_id_index as usize) != Some(&host::id()) {
                return None;
            }
            let payload = inner.instruction.data.strip_prefix(prefix.as_slice())?;
            host::PublicOutputsProducedEvent::deserialize(&mut &*payload).ok()
        })
        .collect()
}

fn assert_born_public_batch(
    result: &mollusk_svm::result::InstructionResult,
    expected_outputs: &[(u16, Pubkey)],
) {
    let events = born_public_events(result);
    assert_eq!(events.len(), 1, "expected exactly one lifecycle batch");
    let prefix = anchor_lang::event::EVENT_IX_TAG_LE
        .iter()
        .copied()
        .chain(
            host::PublicOutputsProducedEvent::DISCRIMINATOR
                .iter()
                .copied(),
        )
        .collect::<Vec<_>>();
    let message = result.message.as_ref().expect("compiled Mollusk message");
    let account_keys = message.account_keys();
    let inner = result
        .inner_instructions
        .iter()
        .find(|inner| {
            account_keys.get(inner.instruction.program_id_index as usize) == Some(&host::id())
                && inner.instruction.data.starts_with(&prefix)
        })
        .expect("produced-public lifecycle inner instruction");
    assert_eq!(
        account_keys.get(inner.instruction.program_id_index as usize),
        Some(&host::id())
    );
    assert_eq!(inner.instruction.accounts.len(), 1);
    assert_eq!(
        account_keys.get(inner.instruction.accounts[0] as usize),
        Some(&event_authority(host::id()))
    );
    let event = &events[0];
    assert_eq!(event.version, host::PUBLIC_OUTPUTS_PRODUCED_EVENT_VERSION);
    assert_eq!(event.outputs.len(), expected_outputs.len());
    for (record, (step_index, encrypted_value)) in event.outputs.iter().zip(expected_outputs.iter())
    {
        assert_eq!(record.step_index, *step_index);
        assert_eq!(record.encrypted_value, *encrypted_value);
        assert_eq!(
            record.output_handle,
            read_encrypted_value(result, *encrypted_value).current_handle
        );
    }
}

#[test]
fn mollusk_fhe_eval_without_born_public_output_emits_no_lifecycle_batch() {
    let frame = born_public_frame(1, &[]);
    let result = mollusk().process_and_validate_instruction(
        &frame.instruction,
        &frame.accounts,
        &[Check::success()],
    );
    assert!(born_public_events(&result).is_empty());
}

#[test]
fn mollusk_fhe_eval_emits_one_born_public_lifecycle_batch() {
    let frame = born_public_frame(1, &[0]);
    let result = mollusk().process_and_validate_instruction(
        &frame.instruction,
        &frame.accounts,
        &[Check::success()],
    );
    assert_born_public_batch(&result, &frame.outputs);
}

#[test]
fn mollusk_fhe_eval_batches_multiple_born_public_outputs_in_step_order() {
    let frame = born_public_frame(3, &[0, 2]);
    let result = mollusk().process_and_validate_instruction(
        &frame.instruction,
        &frame.accounts,
        &[Check::success()],
    );
    assert_born_public_batch(&result, &frame.outputs);
}

#[test]
fn mollusk_fhe_eval_maximum_born_public_frame_fits_one_cpi() {
    let born_public_steps = (0..host::MAX_FHE_EVAL_OPS).collect::<Vec<_>>();
    let frame = born_public_frame(host::MAX_FHE_EVAL_OPS, &born_public_steps);
    let result = mollusk().process_and_validate_instruction(
        &frame.instruction,
        &frame.accounts,
        &[Check::success()],
    );
    assert_born_public_batch(&result, &frame.outputs);
}

#[test]
fn mollusk_fhe_eval_wrong_event_authority_fails_without_output() {
    let mut frame = born_public_frame(1, &[0]);
    let wrong_event_authority = Pubkey::new_unique();
    let event_authority_meta = frame
        .instruction
        .accounts
        .iter_mut()
        .find(|meta| meta.pubkey == event_authority(host::id()))
        .expect("event authority account meta");
    event_authority_meta.pubkey = wrong_event_authority;
    frame
        .accounts
        .push((wrong_event_authority, Account::default()));

    let result = mollusk().process_instruction(&frame.instruction, &frame.accounts);
    assert!(result.program_result.is_err());
    assert!(born_public_events(&result).is_empty());
    let output = result.get_account(&frame.outputs[0].1).unwrap();
    assert_eq!(output.owner, system_program::ID);
    assert!(output.data.is_empty());
}

#[test]
fn mollusk_transaction_later_failure_rolls_back_born_public_output() {
    let frame = born_public_frame(1, &[0]);
    let transaction = mollusk().process_transaction_instructions(
        &[frame.instruction.clone(), frame.instruction],
        &frame.accounts,
    );
    assert!(transaction.program_result.is_err());
    let output = transaction.get_account(&frame.outputs[0].1).unwrap();
    assert_eq!(output.owner, system_program::ID);
    assert!(output.data.is_empty());
}

// ===========================================================================
// HCU per-app block cap: trust registry, per-slot meter, two-pass enforcement.
//
// Ported from PR #2991 ("per-app HCU limit per block"). The admin-setter and
// trust-registry tests below carry over almost verbatim: they only touch
// `HostConfig` and the two new HCU state accounts, none of which changed shape
// in the ACL/MMR rewrite. The `fhe_eval` enforcement tests are rebuilt on a
// fresh `EvalFixture` using durable `EncryptedValue` inputs/outputs instead of
// the old keyed-nonce `AclRecord` the original PR tested against.
// ===========================================================================

/// Exact HCU cost of `EvalFixture::success_steps`: `Ge` at ebool (21_000) + `Sub` at
/// euint64 (38_000) + `IfThenElse` at euint64 (45_000). See `zama-host/src/instructions/fhe_eval/hcu.rs`.
const FIXTURE_FRAME_HCU: u64 = 21_000 + 38_000 + 45_000; // 104_000

/// Exact HCU cost of the fixture's transient-only frame: a single `Ge` at ebool.
const TRANSIENT_FRAME_HCU: u64 = 21_000;

fn system_account(lamports: u64) -> Account {
    Account {
        lamports,
        data: vec![],
        owner: system_program::ID,
        executable: false,
        rent_epoch: 0,
    }
}

fn anchor_error(error: anchor_lang::error::ErrorCode) -> Check<'static> {
    Check::err(solana_sdk::program_error::ProgramError::Custom(
        error as u32,
    ))
}

/// Like [`host_config_account`] but with the two per-frame HCU limits pre-set.
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

/// Like [`host_config_account`] but with the per-app block cap overridden to `cap`. Seeded
/// directly, bypassing the setter ordering guard.
fn host_config_account_with_block_cap(admin: Pubkey, cap: u64) -> (Pubkey, Account) {
    let (key, mut account) = host_config_account(admin);
    let mut config = {
        let mut data = account.data.as_slice();
        HostConfig::try_deserialize(&mut data).expect("valid host config")
    };
    config.hcu_block_cap_per_app = cap;
    account.data = serialized_account(config);
    (key, account)
}

fn mollusk_eval_context(
    payer: Pubkey,
    seeded_accounts: Vec<(Pubkey, Account)>,
) -> mollusk_svm::MolluskContext<HashMap<Pubkey, Account>> {
    let mut accounts = HashMap::from([(payer, funded_system_account())]);
    for (pubkey, account) in seeded_accounts {
        accounts.insert(pubkey, account);
    }
    mollusk().with_context(accounts)
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

fn read_hcu_block_meter(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<host::HcuBlockMeter> {
    let store = context.account_store.borrow();
    let account = store.get(&address)?;
    if account.owner != host::id() {
        return None;
    }
    let mut data = account.data.as_slice();
    host::HcuBlockMeter::try_deserialize(&mut data).ok()
}

fn read_hcu_trusted_app_record(
    context: &mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
    address: Pubkey,
) -> Option<host::HcuTrustedAppRecord> {
    let store = context.account_store.borrow();
    let account = store.get(&address)?;
    if account.owner != host::id() {
        return None;
    }
    let mut data = account.data.as_slice();
    host::HcuTrustedAppRecord::try_deserialize(&mut data).ok()
}

// ---- admin-setter / trust-registry instruction builders ----

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

fn set_hcu_block_cap_per_app_ix(
    program_id: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    value: u64,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::HostAdmin { admin, host_config },
        host::instruction::SetHcuBlockCapPerApp { value },
    )
}

fn set_hcu_app_trusted_ix(
    program_id: Pubkey,
    payer: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    app: Pubkey,
    trusted: bool,
) -> Instruction {
    let record = host::hcu_trusted_app_address(app).0;
    set_hcu_app_trusted_ix_with_record(program_id, payer, admin, host_config, record, app, trusted)
}

#[allow(clippy::too_many_arguments)]
fn set_hcu_app_trusted_ix_with_record(
    program_id: Pubkey,
    payer: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    record: Pubkey,
    app: Pubkey,
    trusted: bool,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::SetHcuAppTrusted {
            payer,
            admin,
            host_config,
            hcu_trusted_app_record: record,
            system_program: system_program::ID,
        },
        host::instruction::SetHcuAppTrusted { app, trusted },
    )
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

// ---- HCU state account fixtures ----

/// A program-owned trust record at the canonical `("hcu-trusted", app)` PDA.
fn hcu_trusted_app_record_account(app: Pubkey, trusted: bool) -> (Pubkey, Account) {
    let (key, bump) = host::hcu_trusted_app_address(app);
    (
        key,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(host::HcuTrustedAppRecord { app, trusted, bump }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

/// A program-owned meter at the canonical `("hcu-block-meter", app)` PDA, pre-loaded with
/// `used_hcu` as of `last_seen_slot`.
fn hcu_block_meter_account(app: Pubkey, last_seen_slot: u64, used_hcu: u64) -> (Pubkey, Account) {
    let (key, bump) = host::hcu_block_meter_address(app);
    (
        key,
        Account {
            lamports: 1_000_000_000,
            data: serialized_account(host::HcuBlockMeter {
                app,
                last_seen_slot,
                used_hcu,
                bump,
            }),
            owner: host::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
}

// ---- set_max_hcu_per_tx: block-cap ordering enforced from the other side ----

#[test]
fn mollusk_set_max_hcu_per_tx_rejects_above_block_cap_band() {
    // The block-cap ordering guard from the other side: with the cap in the metering band
    // (500k), raising max_hcu_per_tx above it would make a single legal max-per-tx frame
    // structurally unable to pass the block cap -> rejected, no mutation.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account_with_block_cap(admin, 500_000);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_max_hcu_per_tx_ix(program_id, admin, host_config, 600_000),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockCapBelowMaxPerTx,
        )],
    );
    let config = read_host_config(&context, host_config).expect("config");
    assert_eq!(config.max_hcu_per_tx, 0);
    assert_eq!(config.hcu_block_cap_per_app, 500_000);

    // At the boundary (== cap) the guard is silent: a total equal to the band cap is accepted.
    context.process_and_validate_instruction(
        &set_max_hcu_per_tx_ix(program_id, admin, host_config, 500_000),
        &[Check::success()],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .max_hcu_per_tx,
        500_000
    );
}

#[test]
fn mollusk_set_max_hcu_per_tx_unrestricted_block_cap_accepts_any_total() {
    // With the cap at the unrestricted sentinel (the ship default), the block-cap ordering
    // guard is vacuous and any total is accepted.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_max_hcu_per_tx_ix(program_id, admin, host_config, 20_000_000),
        &[Check::success()],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .max_hcu_per_tx,
        20_000_000
    );
}

// ---- set_hcu_block_cap_per_app (admin cap setter) ----

#[test]
fn mollusk_set_hcu_block_cap_metering_band_persists_and_advances_slot() {
    // With the per-frame cap disabled, any positive band value is accepted, persisted, and
    // stamps updated_slot.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_hcu_block_cap_per_app_ix(program_id, admin, host_config, 500_000),
        &[Check::success()],
    );
    let config = read_host_config(&context, host_config).expect("config");
    assert_eq!(config.hcu_block_cap_per_app, 500_000);
    assert_eq!(config.updated_slot, context.mollusk.sysvars.clock.slot);
}

#[test]
fn mollusk_set_hcu_block_cap_at_max_per_tx_boundary_is_accepted() {
    // A band value exactly equal to max_hcu_per_tx is the tightest legal cap: it must be
    // accepted so a single max-cost frame stays possible on a fresh meter.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account_with_hcu_limits(admin, 20_000_000, 0);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_hcu_block_cap_per_app_ix(program_id, admin, host_config, 20_000_000),
        &[Check::success()],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .hcu_block_cap_per_app,
        20_000_000
    );
}

#[test]
fn mollusk_set_hcu_block_cap_below_max_per_tx_is_rejected() {
    // A band value under max_hcu_per_tx would make a single legal max-per-tx frame
    // structurally impossible (other than the deliberate ban); reject without mutation.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account_with_hcu_limits(admin, 20_000_000, 0);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_hcu_block_cap_per_app_ix(program_id, admin, host_config, 19_000_000),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockCapBelowMaxPerTx,
        )],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .hcu_block_cap_per_app,
        u64::MAX
    );
}

#[test]
fn mollusk_set_hcu_block_cap_with_max_per_tx_unlimited_accepts_any_band_value() {
    // max_hcu_per_tx == 0 means the per-frame cap is unlimited, so the ordering guard is
    // vacuous and even a tiny band value is accepted.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_hcu_block_cap_per_app_ix(program_id, admin, host_config, 1),
        &[Check::success()],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .hcu_block_cap_per_app,
        1
    );
}

#[test]
fn mollusk_set_hcu_block_cap_ban_and_unrestricted_sentinels_bypass_ordering() {
    // The two sentinels — 0 (ban untrusted apps) and u64::MAX (unrestricted) — are always
    // accepted, even below max_hcu_per_tx, because neither is a metering-band value.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account_with_hcu_limits(admin, 20_000_000, 0);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_hcu_block_cap_per_app_ix(program_id, admin, host_config, 0),
        &[Check::success()],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .hcu_block_cap_per_app,
        0
    );

    context.process_and_validate_instruction(
        &set_hcu_block_cap_per_app_ix(program_id, admin, host_config, u64::MAX),
        &[Check::success()],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .hcu_block_cap_per_app,
        u64::MAX
    );
}

#[test]
fn mollusk_set_hcu_block_cap_is_idempotent() {
    // Setting the current value is a no-op: it does not advance updated_slot (mirrors the
    // other admin setters).
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account_with_block_cap(admin, 750_000);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_hcu_block_cap_per_app_ix(program_id, admin, host_config, 750_000),
        &[Check::success()],
    );
    let config = read_host_config(&context, host_config).expect("config");
    assert_eq!(config.hcu_block_cap_per_app, 750_000);
    assert_eq!(config.updated_slot, 0);
}

#[test]
fn mollusk_set_hcu_block_cap_rejects_wrong_admin() {
    // A valid signer that is not the stored admin cannot change the cap.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let wrong_admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_eval_context(
        admin,
        vec![
            (host_config, account),
            (wrong_admin, funded_system_account()),
        ],
    );

    context.process_and_validate_instruction(
        &set_hcu_block_cap_per_app_ix(program_id, wrong_admin, host_config, 500_000),
        &[custom_error(
            host::errors::ZamaHostError::HostConfigAdminMismatch,
        )],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .hcu_block_cap_per_app,
        u64::MAX
    );
}

#[test]
fn mollusk_set_hcu_block_cap_rejects_remaining_accounts() {
    // A trailing account meta is rejected before any mutation.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    let mut ix = set_hcu_block_cap_per_app_ix(program_id, admin, host_config, 500_000);
    ix.accounts
        .push(AccountMeta::new_readonly(Pubkey::new_unique(), false));
    context.process_and_validate_instruction(
        &ix,
        &[custom_error(
            host::errors::ZamaHostError::UnexpectedRemainingAccounts,
        )],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .hcu_block_cap_per_app,
        u64::MAX
    );
}

// ---- set_hcu_app_trusted (admin trust registry) ----

#[test]
fn mollusk_set_hcu_app_trusted_creates_trusted_record() {
    // A first trust-set lazy-creates the canonical record with trusted = true.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let app = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_hcu_app_trusted_ix(program_id, admin, admin, host_config, app, true),
        &[Check::success()],
    );
    let record = read_hcu_trusted_app_record(&context, host::hcu_trusted_app_address(app).0)
        .expect("record");
    assert_eq!(record.app, app);
    assert!(record.trusted);
}

#[test]
fn mollusk_set_hcu_app_trusted_writes_untrusted_false_record() {
    // A well-formed record may carry trusted = false; that is an explicit "metered", not an error.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let app = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    // Register trusted, then clear it back to false.
    context.process_and_validate_instruction(
        &set_hcu_app_trusted_ix(program_id, admin, admin, host_config, app, true),
        &[Check::success()],
    );
    context.process_and_validate_instruction(
        &set_hcu_app_trusted_ix(program_id, admin, admin, host_config, app, false),
        &[Check::success()],
    );
    let record = read_hcu_trusted_app_record(&context, host::hcu_trusted_app_address(app).0)
        .expect("record");
    assert!(!record.trusted);
}

#[test]
fn mollusk_set_hcu_app_trusted_is_idempotent() {
    // Re-setting the current trust value is a no-op and leaves the record intact.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let app = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_hcu_app_trusted_ix(program_id, admin, admin, host_config, app, true),
        &[Check::success()],
    );
    context.process_and_validate_instruction(
        &set_hcu_app_trusted_ix(program_id, admin, admin, host_config, app, true),
        &[Check::success()],
    );
    assert!(
        read_hcu_trusted_app_record(&context, host::hcu_trusted_app_address(app).0)
            .expect("record")
            .trusted
    );
}

#[test]
fn mollusk_set_hcu_app_trusted_rejects_wrong_record_pda() {
    // A record account that is not the canonical ("hcu-trusted", app) PDA is rejected.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let app = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    // A record derived for a *different* app is the wrong PDA for `app`.
    let wrong_record = host::hcu_trusted_app_address(Pubkey::new_unique()).0;
    context.process_and_validate_instruction(
        &set_hcu_app_trusted_ix_with_record(
            program_id,
            admin,
            admin,
            host_config,
            wrong_record,
            app,
            true,
        ),
        &[custom_error(
            host::errors::ZamaHostError::HcuTrustedAppRecordMismatch,
        )],
    );
    assert!(read_hcu_trusted_app_record(&context, host::hcu_trusted_app_address(app).0).is_none());
}

#[test]
fn mollusk_set_hcu_app_trusted_rejects_wrong_admin() {
    // Only the stored admin may register trust — an app cannot self-trust.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let wrong_admin = Pubkey::new_unique();
    let app = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_eval_context(
        admin,
        vec![
            (host_config, account),
            (wrong_admin, funded_system_account()),
        ],
    );

    context.process_and_validate_instruction(
        &set_hcu_app_trusted_ix(program_id, wrong_admin, wrong_admin, host_config, app, true),
        &[custom_error(
            host::errors::ZamaHostError::HostConfigAdminMismatch,
        )],
    );
    assert!(read_hcu_trusted_app_record(&context, host::hcu_trusted_app_address(app).0).is_none());
}

#[test]
fn mollusk_set_hcu_app_trusted_rejects_remaining_accounts() {
    // A trailing account meta is rejected before any write.
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let app = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_eval_context(admin, vec![(host_config, account)]);

    let mut ix = set_hcu_app_trusted_ix(program_id, admin, admin, host_config, app, true);
    ix.accounts
        .push(AccountMeta::new_readonly(Pubkey::new_unique(), false));
    context.process_and_validate_instruction(
        &ix,
        &[custom_error(
            host::errors::ZamaHostError::UnexpectedRemainingAccounts,
        )],
    );
    assert!(read_hcu_trusted_app_record(&context, host::hcu_trusted_app_address(app).0).is_none());
}

#[test]
fn mollusk_initialize_host_config_defaults_block_cap_to_unrestricted() {
    // A freshly initialized config ships unrestricted (u64::MAX), not banned (0).
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
        grant_deny_list_enabled: false,
    };
    let context = mollusk_eval_context(payer, vec![(host_config, system_account(0))]);

    context.process_and_validate_instruction(
        &initialize_host_config_ix(program_id, payer, admin, host_config, args),
        &[Check::success()],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .hcu_block_cap_per_app,
        u64::MAX
    );
}

// ---- EvalFixture: a durable-output frame for block-cap enforcement ----

struct EvalFixture {
    program_id: Pubkey,
    authority: Pubkey,
    app_account: Pubkey,
    host_config: Pubkey,
    balance_handle: [u8; 32],
    amount_handle: [u8; 32],
    balance_value: Pubkey,
    amount_value: Pubkey,
    output_value: Pubkey,
    output_label: [u8; 32],
    /// Dedicated HCU metering identity — deliberately distinct from `app_account` /
    /// `app_account_authority` so block-cap tests prove the meter never keys on those.
    hcu_authority: Pubkey,
    context: mollusk_svm::MolluskContext<HashMap<Pubkey, Account>>,
}

impl EvalFixture {
    /// A fixture whose config carries a per-app block cap; per-frame HCU limits stay off.
    fn with_block_cap(cap: u64) -> Self {
        let program_id = host::id();
        let authority = Pubkey::new_unique();
        let hcu_authority = Pubkey::new_unique();
        let app_account = authority;
        let (host_config, host_config_account) = host_config_account_with_block_cap(authority, cap);
        let balance_label = label("balance-hcu-fixture");
        let amount_label = label("amount-hcu-fixture");
        let output_label = label("output-hcu-fixture");
        let balance_handle = handle_for_chain(151, 5);
        let amount_handle = handle_for_chain(152, 5);
        let (balance_value, balance_ev) = new_lineage(
            authority,
            app_account,
            balance_label,
            balance_handle,
            &[authority],
        );
        let (amount_value, amount_ev) = new_lineage(
            authority,
            app_account,
            amount_label,
            amount_handle,
            &[authority],
        );
        let output_value_key = zama_solana_acl::derive_value_key(
            authority.to_bytes(),
            app_account.to_bytes(),
            output_label,
        );
        let (output_value, _bump) = host::encrypted_value_address(output_value_key);
        let context = mollusk_eval_context(
            authority,
            vec![
                (host_config, host_config_account),
                (balance_value, encrypted_value_account(&balance_ev)),
                (amount_value, encrypted_value_account(&amount_ev)),
            ],
        );
        Self {
            program_id,
            authority,
            app_account,
            host_config,
            balance_handle,
            amount_handle,
            balance_value,
            amount_value,
            output_value,
            output_label,
            hcu_authority,
            context,
        }
    }

    /// The app identity both new PDAs are keyed on (the frame's `hcu_authority` signer) —
    /// deliberately NOT `app_account_authority`.
    fn block_cap_app(&self) -> Pubkey {
        self.hcu_authority
    }

    fn meter_pda(&self) -> Pubkey {
        host::hcu_block_meter_address(self.hcu_authority).0
    }

    fn trust_pda(&self) -> Pubkey {
        host::hcu_trusted_app_address(self.hcu_authority).0
    }

    fn seed_account(&self, key: Pubkey, account: Account) {
        self.context.account_store.borrow_mut().insert(key, account);
    }

    fn balance_operand(&self) -> FheEvalOperand {
        FheEvalOperand::AllowedDurable {
            handle: self.balance_handle,
            encrypted_value_index: 0,
        }
    }

    fn amount_operand(&self) -> FheEvalOperand {
        FheEvalOperand::AllowedDurable {
            handle: self.amount_handle,
            encrypted_value_index: 1,
        }
    }

    /// `Ge` (ebool) + `Sub` (euint64) + `IfThenElse` (euint64, durable output) — costs exactly
    /// `FIXTURE_FRAME_HCU`.
    fn success_steps(&self) -> Vec<FheEvalStep> {
        vec![
            FheEvalStep::Binary {
                op: FheBinaryOpCode::Ge,
                lhs: self.balance_operand(),
                rhs: self.amount_operand(),
                output_fhe_type: 0,
                output: FheEvalOutput::AllowedLocal,
            },
            FheEvalStep::Binary {
                op: FheBinaryOpCode::Sub,
                lhs: self.balance_operand(),
                rhs: self.amount_operand(),
                output_fhe_type: 5,
                output: FheEvalOutput::AllowedLocal,
            },
            FheEvalStep::Ternary {
                op: FheTernaryOpCode::IfThenElse,
                control: FheEvalOperand::AllowedLocal { producer_index: 0 },
                if_true: FheEvalOperand::AllowedLocal { producer_index: 1 },
                if_false: self.balance_operand(),
                output_fhe_type: 5,
                output: FheEvalOutput::AllowedDurable {
                    output_encrypted_value_index: 2,
                    output_app_account_authority_index: None,
                    output_acl_domain_key: self.authority,
                    output_app_account: self.app_account,
                    output_encrypted_value_label: self.output_label,
                    output_subjects: vec![host::AclSubjectEntry {
                        pubkey: self.authority,
                    }],
                    previous_handle: None,
                    previous_subjects: None,
                    make_public: false,
                },
            },
        ]
    }

    /// The standard durable-output frame with the fixture's `hcu_authority` signed in,
    /// threading the two optional block-cap accounts.
    fn block_cap_instruction(&self, meter: Option<Pubkey>, trust: Option<Pubkey>) -> Instruction {
        let mut ix = anchor_ix(
            self.program_id,
            host::accounts::FheEval {
                payer: self.authority,
                compute_subject: self.authority,
                app_account_authority: self.app_account,
                host_config: self.host_config,
                system_program: system_program::ID,
                hcu_authority: self.hcu_authority,
                hcu_block_meter: meter,
                hcu_trusted_app_record: trust,
                event_authority: event_authority(self.program_id),
                program: self.program_id,
            },
            host::instruction::FheEval {
                args: FheEvalArgs {
                    context_id: label("block-cap-frame"),
                    steps: self.success_steps(),
                },
            },
        );
        ix.accounts.push(writable(self.balance_value));
        ix.accounts.push(writable(self.amount_value));
        ix.accounts.push(writable(self.output_value));
        ix
    }

    /// A transient-only frame (single step, `AllowedLocal` output) — produces no durable
    /// output; the block-cap identity comes solely from the `hcu_authority` signer.
    fn transient_only_instruction(
        &self,
        meter: Option<Pubkey>,
        trust: Option<Pubkey>,
    ) -> Instruction {
        let steps = vec![FheEvalStep::Binary {
            op: FheBinaryOpCode::Ge,
            lhs: self.balance_operand(),
            rhs: self.amount_operand(),
            output_fhe_type: 0,
            output: FheEvalOutput::AllowedLocal,
        }];
        let mut ix = anchor_ix(
            self.program_id,
            host::accounts::FheEval {
                payer: self.authority,
                compute_subject: self.authority,
                app_account_authority: self.app_account,
                host_config: self.host_config,
                system_program: system_program::ID,
                hcu_authority: self.hcu_authority,
                hcu_block_meter: meter,
                hcu_trusted_app_record: trust,
                event_authority: event_authority(self.program_id),
                program: self.program_id,
            },
            host::instruction::FheEval {
                args: FheEvalArgs {
                    context_id: label("transient-only"),
                    steps,
                },
            },
        );
        ix.accounts.push(writable(self.balance_value));
        ix.accounts.push(writable(self.amount_value));
        ix
    }

    /// Asserts the durable output was never created, from a returned `InstructionResult`
    /// (works whether or not the output account was ever persisted into `self.context`).
    fn assert_no_output(&self, result: &mollusk_svm::result::InstructionResult) {
        let owner = result
            .resulting_accounts
            .iter()
            .find(|(key, _)| *key == self.output_value)
            .map(|(_, account)| account.owner);
        assert_ne!(
            owner,
            Some(host::id()),
            "output EncryptedValue should not have been created"
        );
    }
}

// ---- fhe_eval block-cap enforcement ----

#[test]
fn mollusk_fhe_eval_unrestricted_cap_none_none_succeeds() {
    // The default (u64::MAX) short-circuits: with the mandatory hcu_authority signed in but
    // neither optional account supplied, the frame binds its durable output and no meter is
    // ever created or touched.
    let fixture = EvalFixture::with_block_cap(u64::MAX);
    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, None),
        &[Check::success()],
    );
    read_encrypted_value_from_context(&fixture.context, fixture.output_value);
    assert!(read_hcu_block_meter(&fixture.context, fixture.meter_pda()).is_none());
}

#[test]
fn mollusk_fhe_eval_missing_hcu_authority_account_fails_structurally() {
    // The hcu_authority is a mandatory account, not program logic: a frame missing it never
    // reaches the handler — the account layer rejects the shape outright, even under the
    // unrestricted default. There is no account shape that evals without an HCU identity.
    let fixture = EvalFixture::with_block_cap(u64::MAX);
    let mut ix = fixture.block_cap_instruction(None, None);
    let authority = fixture.block_cap_app();
    ix.accounts.retain(|meta| meta.pubkey != authority);
    let result = fixture.context.process_instruction(&ix);
    assert!(result.raw_result.is_err());
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_eval_unsigned_hcu_authority_is_rejected() {
    // The hcu_authority must SIGN. A supplied-but-unsigned authority is rejected by the
    // account layer — otherwise any caller could name a trusted app's authority to steal its
    // bypass, or a victim's authority to drain its in-slot budget.
    let fixture = EvalFixture::with_block_cap(500_000);
    let mut ix = fixture.block_cap_instruction(Some(fixture.meter_pda()), None);
    let authority = fixture.block_cap_app();
    for meta in ix.accounts.iter_mut() {
        if meta.pubkey == authority {
            meta.is_signer = false;
        }
    }
    let result = fixture.context.process_and_validate_instruction(
        &ix,
        &[anchor_error(
            anchor_lang::error::ErrorCode::AccountNotSigner,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_eval_unrestricted_cap_ignores_supplied_accounts() {
    // Even when both optional accounts are supplied, the unrestricted short-circuit touches
    // neither: a pre-loaded meter is left byte-for-byte unchanged.
    let fixture = EvalFixture::with_block_cap(u64::MAX);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    let (meter_pda, meter_account) = hcu_block_meter_account(fixture.block_cap_app(), slot, 999);
    fixture.seed_account(meter_pda, meter_account);
    let (trust_pda, trust_account) = hcu_trusted_app_record_account(fixture.block_cap_app(), true);
    fixture.seed_account(trust_pda, trust_account);

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), Some(trust_pda)),
        &[Check::success()],
    );
    assert_eq!(
        read_hcu_block_meter(&fixture.context, meter_pda)
            .expect("meter")
            .used_hcu,
        999
    );
}

#[test]
fn mollusk_fhe_eval_ban_cap_zero_untrusted_no_meter_is_rejected() {
    // cap == 0 bans untrusted apps outright — rejected even with no meter supplied, and no
    // durable output is created.
    let fixture = EvalFixture::with_block_cap(0);
    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, None),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockLimitExceeded,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_eval_ban_cap_zero_untrusted_with_meter_is_rejected_unchanged() {
    // The ban trips before the meter is consulted: a supplied meter is left unchanged.
    let fixture = EvalFixture::with_block_cap(0);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    let (meter_pda, meter_account) = hcu_block_meter_account(fixture.block_cap_app(), slot, 0);
    fixture.seed_account(meter_pda, meter_account);

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockLimitExceeded,
        )],
    );
    assert_eq!(
        read_hcu_block_meter(&fixture.context, meter_pda)
            .expect("meter")
            .used_hcu,
        0
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_eval_ban_cap_zero_trusted_witness_bypasses() {
    // Trusted apps are never banned: with a valid trust witness the frame succeeds even at
    // cap == 0, without any meter.
    let fixture = EvalFixture::with_block_cap(0);
    let (trust_pda, trust_account) = hcu_trusted_app_record_account(fixture.block_cap_app(), true);
    fixture.seed_account(trust_pda, trust_account);

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, Some(trust_pda)),
        &[Check::success()],
    );
    read_encrypted_value_from_context(&fixture.context, fixture.output_value);
}

#[test]
fn mollusk_fhe_eval_untrusted_missing_meter_fails_closed() {
    // In the metering band, an untrusted app that forwards neither a meter nor a trust
    // witness is rejected — never silently un-metered. (This is also the CPI rollout hazard:
    // a caller that forwards neither account breaks, rather than bypassing the cap.)
    let fixture = EvalFixture::with_block_cap(500_000);
    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, None),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockMeterMissing,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_eval_trusted_witness_bypasses_and_creates_no_meter() {
    // A valid trust witness bypasses metering entirely: the frame succeeds with no meter and
    // none is lazily created (contention-free trusted path).
    let fixture = EvalFixture::with_block_cap(500_000);
    let (trust_pda, trust_account) = hcu_trusted_app_record_account(fixture.block_cap_app(), true);
    fixture.seed_account(trust_pda, trust_account);

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, Some(trust_pda)),
        &[Check::success()],
    );
    read_encrypted_value_from_context(&fixture.context, fixture.output_value);
    assert!(read_hcu_block_meter(&fixture.context, fixture.meter_pda()).is_none());
}

#[test]
fn mollusk_fhe_eval_untrusted_false_witness_requires_meter() {
    // A well-formed record with trusted == false is not a bypass — it falls through to the
    // metering path, so a missing meter still fails closed.
    let fixture = EvalFixture::with_block_cap(500_000);
    let (trust_pda, trust_account) = hcu_trusted_app_record_account(fixture.block_cap_app(), false);
    fixture.seed_account(trust_pda, trust_account);

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, Some(trust_pda)),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockMeterMissing,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_eval_wrong_pda_trust_witness_is_rejected() {
    // A witness for a different app (wrong PDA) cannot bypass this app's cap.
    let fixture = EvalFixture::with_block_cap(500_000);
    let (other_trust_pda, other_trust_account) =
        hcu_trusted_app_record_account(Pubkey::new_unique(), true);
    fixture.seed_account(other_trust_pda, other_trust_account);

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, Some(other_trust_pda)),
        &[custom_error(
            host::errors::ZamaHostError::HcuTrustedAppRecordMismatch,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_eval_malformed_trust_witness_is_rejected() {
    // A witness at the canonical PDA but not program-owned (self-made) is rejected — an app
    // cannot forge its own trust. Only an *absent* witness is benign.
    let fixture = EvalFixture::with_block_cap(500_000);
    let trust_pda = fixture.trust_pda();
    fixture.seed_account(
        trust_pda,
        Account {
            lamports: 1_000_000,
            data: vec![1u8; 8],
            owner: system_program::ID,
            executable: false,
            rent_epoch: 0,
        },
    );

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, Some(trust_pda)),
        &[custom_error(
            host::errors::ZamaHostError::HcuTrustedAppRecordMismatch,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_eval_wrong_app_meter_is_rejected() {
    // A meter that belongs to a different app (wrong PDA / record.app) cannot be charged for
    // this app.
    let fixture = EvalFixture::with_block_cap(500_000);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    let (other_meter_pda, other_meter_account) =
        hcu_block_meter_account(Pubkey::new_unique(), slot, 0);
    fixture.seed_account(other_meter_pda, other_meter_account);

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(other_meter_pda), None),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockMeterMismatch,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_eval_squatted_meter_with_data_is_rejected() {
    // A pre-squatted (system-owned, non-empty DATA) account at the meter PDA fails
    // lazy-creation rather than being adopted as a counter. An attacker cannot actually put
    // data on the PDA (allocate needs the PDA's signature), so this guards against a genuinely
    // malformed account.
    let fixture = EvalFixture::with_block_cap(500_000);
    let meter_pda = fixture.meter_pda();
    fixture.seed_account(
        meter_pda,
        Account {
            lamports: 1_000_000,
            data: vec![7u8; 16],
            owner: system_program::ID,
            executable: false,
            rent_epoch: 0,
        },
    );

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[custom_error(
            host::errors::ZamaHostError::PdaCreationMismatch,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_eval_prefunded_empty_meter_is_created_not_griefed() {
    // Anti-griefing: the meter PDA address is predictable, so a third party can pre-fund it
    // with a bare lamport transfer (system-owned, EMPTY data) before the app's first metered
    // frame. The fused `create_account` would abort on any pre-funded target
    // (AccountAlreadyInUse) and wedge every metered frame forever; the
    // fund-shortfall+allocate+assign path absorbs the donation and creates the meter normally.
    let fixture = EvalFixture::with_block_cap(500_000);
    let meter_pda = fixture.meter_pda();
    fixture.seed_account(meter_pda, system_account(1));

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[Check::success()],
    );
    let meter = read_hcu_block_meter(&fixture.context, meter_pda).expect("meter created");
    assert_eq!(meter.app, fixture.block_cap_app());
    assert_eq!(meter.used_hcu, FIXTURE_FRAME_HCU);
    // The donated lamport was topped up to at least rent-exempt.
    let lamports = fixture
        .context
        .account_store
        .borrow()
        .get(&meter_pda)
        .expect("meter account")
        .lamports;
    assert!(
        lamports
            >= anchor_lang::prelude::Rent::default()
                .minimum_balance(8 + host::HcuBlockMeter::SPACE)
    );
    read_encrypted_value_from_context(&fixture.context, fixture.output_value);
}

#[test]
fn mollusk_fhe_eval_overfunded_empty_meter_is_created_preserving_surplus() {
    // A donation far above rent is equally harmless: no top-up transfer occurs, the meter is
    // created, and the surplus lamports are preserved (the account is simply
    // more-than-rent-exempt).
    let fixture = EvalFixture::with_block_cap(500_000);
    let meter_pda = fixture.meter_pda();
    let donated = 5_000_000_000u64;
    fixture.seed_account(meter_pda, system_account(donated));

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[Check::success()],
    );
    let meter = read_hcu_block_meter(&fixture.context, meter_pda).expect("meter created");
    assert_eq!(meter.used_hcu, FIXTURE_FRAME_HCU);
    let lamports = fixture
        .context
        .account_store
        .borrow()
        .get(&meter_pda)
        .expect("meter account")
        .lamports;
    assert_eq!(lamports, donated);
}

#[test]
fn mollusk_fhe_eval_prefunded_output_acl_is_created_not_griefed() {
    // The same anti-griefing property for the durable output-ACL path
    // (`create_pda_strict`): its address is predictable too, so a pre-funded (system-owned,
    // empty) donation at the output PDA must not block the frame. Asserted under the
    // unrestricted cap so the meter path is inert and only the output-ACL creation is
    // exercised.
    let fixture = EvalFixture::with_block_cap(u64::MAX);
    fixture.seed_account(fixture.output_value, system_account(1));

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(None, None),
        &[Check::success()],
    );
    read_encrypted_value_from_context(&fixture.context, fixture.output_value);
}

#[test]
fn mollusk_fhe_eval_trust_pda_supplied_as_meter_is_rejected() {
    // Role confusion: the trust record's PDA is not the meter PDA, so passing it in the meter
    // slot fails the meter's PDA check.
    let fixture = EvalFixture::with_block_cap(500_000);
    let trust_pda = fixture.trust_pda();
    let (_, trust_account) = hcu_trusted_app_record_account(fixture.block_cap_app(), true);
    fixture.seed_account(trust_pda, trust_account);

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(trust_pda), None),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockMeterMismatch,
        )],
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_eval_over_cap_trips_in_admission_without_output_or_mutation() {
    // A frame whose cost exceeds the cap trips in the read-only admission pass: no durable
    // output is created and the meter is left unchanged (breach before any write).
    let fixture = EvalFixture::with_block_cap(FIXTURE_FRAME_HCU - 1);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    let (meter_pda, meter_account) = hcu_block_meter_account(fixture.block_cap_app(), slot, 0);
    fixture.seed_account(meter_pda, meter_account);

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockLimitExceeded,
        )],
    );
    assert_eq!(
        read_hcu_block_meter(&fixture.context, meter_pda)
            .expect("meter")
            .used_hcu,
        0
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_eval_charge_accumulates_onto_prior_slot_usage() {
    // Within a slot, a successful charge adds the frame cost onto the meter's existing usage
    // (monotonic; the meter is reused, not reset).
    let fixture = EvalFixture::with_block_cap(500_000);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    let (meter_pda, meter_account) = hcu_block_meter_account(fixture.block_cap_app(), slot, 50_000);
    fixture.seed_account(meter_pda, meter_account);

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[Check::success()],
    );
    let meter = read_hcu_block_meter(&fixture.context, meter_pda).expect("meter");
    assert_eq!(meter.used_hcu, 50_000 + FIXTURE_FRAME_HCU);
    assert_eq!(meter.last_seen_slot, slot);
    read_encrypted_value_from_context(&fixture.context, fixture.output_value);
}

#[test]
fn mollusk_fhe_eval_over_cap_with_prior_usage_is_rejected_unchanged() {
    // Prior in-slot usage plus this frame exceeds the cap -> rejected, meter unchanged.
    let fixture = EvalFixture::with_block_cap(150_000);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    let (meter_pda, meter_account) =
        hcu_block_meter_account(fixture.block_cap_app(), slot, 100_000);
    fixture.seed_account(meter_pda, meter_account);

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockLimitExceeded,
        )],
    );
    assert_eq!(
        read_hcu_block_meter(&fixture.context, meter_pda)
            .expect("meter")
            .used_hcu,
        100_000
    );
    fixture.assert_no_output(&result);
}

#[test]
fn mollusk_fhe_eval_lazy_reset_zeroes_prior_slot_usage() {
    // A meter last written in a different slot is treated as used = 0 for this slot's frame:
    // even a value that would exceed the cap in-slot no longer blocks, and the meter is
    // rewritten at the current slot with just this frame's cost.
    let fixture = EvalFixture::with_block_cap(150_000);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    // Seed as-of a different slot with usage that would exceed the cap if it carried over.
    let (meter_pda, meter_account) =
        hcu_block_meter_account(fixture.block_cap_app(), slot.wrapping_add(1), 140_000);
    fixture.seed_account(meter_pda, meter_account);

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[Check::success()],
    );
    let meter = read_hcu_block_meter(&fixture.context, meter_pda).expect("meter");
    assert_eq!(meter.used_hcu, FIXTURE_FRAME_HCU);
    assert_eq!(meter.last_seen_slot, slot);
}

#[test]
fn mollusk_fhe_eval_clean_first_call_lazy_creates_meter_at_frame_cost() {
    // A first metered frame lazy-creates a program-owned meter initialized to exactly the
    // frame's cost, stamped at the current slot and keyed on this app.
    let fixture = EvalFixture::with_block_cap(500_000);
    let meter_pda = fixture.meter_pda();

    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[Check::success()],
    );
    let meter = read_hcu_block_meter(&fixture.context, meter_pda).expect("meter created");
    assert_eq!(meter.app, fixture.block_cap_app());
    assert_eq!(meter.used_hcu, FIXTURE_FRAME_HCU);
    assert_eq!(
        meter.last_seen_slot,
        fixture.context.mollusk.sysvars.clock.slot
    );
    read_encrypted_value_from_context(&fixture.context, fixture.output_value);
    // Metering keys on the dedicated hcu_authority, never on app_account_authority: the two
    // identities differ in this fixture and nothing accrued under the latter's key.
    assert_ne!(fixture.block_cap_app(), fixture.app_account);
    assert!(read_hcu_block_meter(
        &fixture.context,
        host::hcu_block_meter_address(fixture.app_account).0
    )
    .is_none());
}

#[test]
fn mollusk_fhe_eval_per_app_meters_are_isolated_under_uniform_cap() {
    // The cap is uniform, but each app has its own meter: one app being maxed out this slot
    // does not throttle a different app, and does not draw down its budget.
    let fixture = EvalFixture::with_block_cap(150_000);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    // A different app is maxed out for the slot.
    let (other_meter_pda, other_meter_account) =
        hcu_block_meter_account(Pubkey::new_unique(), slot, 150_000);
    fixture.seed_account(other_meter_pda, other_meter_account);

    // The fixture app's own frame still succeeds against its own fresh meter.
    let meter_pda = fixture.meter_pda();
    fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[Check::success()],
    );
    assert_eq!(
        read_hcu_block_meter(&fixture.context, other_meter_pda)
            .expect("other meter")
            .used_hcu,
        150_000
    );
    assert_eq!(
        read_hcu_block_meter(&fixture.context, meter_pda)
            .expect("meter")
            .used_hcu,
        FIXTURE_FRAME_HCU
    );
}

#[test]
fn mollusk_fhe_eval_extra_remaining_account_still_rejected_with_block_cap() {
    // The two block-cap accounts are named context accounts, not remaining_accounts, so the
    // "every remaining account is used" invariant is preserved: a trailing extra account is
    // still rejected.
    let fixture = EvalFixture::with_block_cap(u64::MAX);
    let mut ix = fixture.block_cap_instruction(None, None);
    ix.accounts
        .push(AccountMeta::new_readonly(Pubkey::new_unique(), false));
    fixture.context.process_and_validate_instruction(
        &ix,
        &[custom_error(
            host::errors::ZamaHostError::InvalidFheEvalAccount,
        )],
    );
}

#[test]
fn mollusk_fhe_eval_transient_only_frame_is_metered_via_hcu_authority() {
    // A transient-only frame (all AllowedLocal outputs) creates no durable ACL record, so
    // nothing welds `app_account_authority` on-chain — but the metering identity is the
    // dedicated `hcu_authority` signer, independent of the frame's output shape, so the
    // frame is still charged in full. (Under a signer-less design this work would escape the
    // cap entirely; this is the regression guard for that gap.)
    let fixture = EvalFixture::with_block_cap(500_000);
    let meter_pda = fixture.meter_pda();
    let result = fixture.context.process_and_validate_instruction(
        &fixture.transient_only_instruction(Some(meter_pda), None),
        &[Check::success()],
    );
    // No durable output ACL record was produced...
    fixture.assert_no_output(&result);
    // ...yet the frame accrued onto the authority's meter.
    let meter = read_hcu_block_meter(&fixture.context, meter_pda).expect("meter created");
    assert_eq!(meter.app, fixture.block_cap_app());
    assert_eq!(meter.used_hcu, TRANSIENT_FRAME_HCU);
}

#[test]
fn mollusk_fhe_eval_meter_accumulation_overflow_fails_closed() {
    // Accumulating this frame onto a near-max in-slot usage would overflow u64. The checked
    // add must fail closed (reject, never wrap), and the meter is left unchanged. The cap is a
    // huge band value so it is the overflow — not the cap comparison — that trips.
    let fixture = EvalFixture::with_block_cap(u64::MAX - 1);
    let slot = fixture.context.mollusk.sysvars.clock.slot;
    let (meter_pda, meter_account) =
        hcu_block_meter_account(fixture.block_cap_app(), slot, u64::MAX - 1_000);
    fixture.seed_account(meter_pda, meter_account);

    let result = fixture.context.process_and_validate_instruction(
        &fixture.block_cap_instruction(Some(meter_pda), None),
        &[custom_error(
            host::errors::ZamaHostError::HcuBlockLimitExceeded,
        )],
    );
    assert_eq!(
        read_hcu_block_meter(&fixture.context, meter_pda)
            .expect("meter")
            .used_hcu,
        u64::MAX - 1_000
    );
    fixture.assert_no_output(&result);
}
