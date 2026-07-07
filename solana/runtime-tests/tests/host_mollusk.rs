//! Mollusk-based runtime tests for the RFC-024 `EncryptedValue` ACL model.
//!
//! Migrated from the old keyed-nonce `AclRecord`/`AclPermission` model (deleted
//! along with `assert_acl_record`, `allow_acl_subjects`, `commit_handle_material`,
//! and the single-op `fhe_*` instructions) to the new stateless-indexing
//! `EncryptedValue` lineage: `create_encrypted_value`, `allow_subjects`,
//! `update_encrypted_value`, `make_handle_public`, and durable outputs bound
//! through `fhe_eval`. See `zama-host/src/state/encrypted_value.rs` and
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
    prelude::system_program, AccountDeserialize, AccountSerialize, InstructionData, ToAccountMetas,
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
    FheBinaryOpCode, FheEvalArgs, FheEvalOperand, FheEvalOutput, FheEvalStep, HostConfig,
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
                test_authority: admin,
                paused,
                mock_input_enabled: false,
                test_shims_enabled: true,
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
// create_encrypted_value
// ---------------------------------------------------------------------------

#[test]
fn mollusk_create_encrypted_value_succeeds_and_stores_subjects() {
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

    let result = mollusk().process_and_validate_instruction(&ix, &accounts, &[Check::success()]);
    let value = read_encrypted_value(&result, encrypted_value);
    assert_eq!(value.current_handle, handle);
    assert_eq!(value.subjects, vec![subject]);
    assert_eq!(value.leaf_count, 0);
    assert!(value.peaks.is_empty());
}

#[test]
fn mollusk_create_encrypted_value_rejects_empty_subjects() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let acl_domain_key = Pubkey::new_unique();
    let lbl = label("balance");
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
        handle_for_chain(1, 5),
        vec![],
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
            host::errors::ZamaHostError::EncryptedValueEmptySubjects,
        )],
    );
}

#[test]
fn mollusk_create_encrypted_value_rejects_duplicate_subjects() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let acl_domain_key = Pubkey::new_unique();
    let lbl = label("balance");
    let value_key =
        zama_solana_acl::derive_value_key(acl_domain_key.to_bytes(), authority.to_bytes(), lbl);
    let (encrypted_value, _bump) = host::encrypted_value_address(value_key);
    let dup = Pubkey::new_unique();

    let ix = create_encrypted_value_ix(
        authority,
        authority,
        encrypted_value,
        host_config,
        acl_domain_key,
        authority,
        lbl,
        handle_for_chain(1, 5),
        vec![
            EncryptedValueSubjectGrant { subject: dup },
            EncryptedValueSubjectGrant { subject: dup },
        ],
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
        &[custom_error(host::errors::ZamaHostError::SubjectNotAllowed)],
    );
}

#[test]
fn mollusk_create_encrypted_value_rejects_over_cap_subjects_at_birth() {
    let payer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let acl_domain_key = Pubkey::new_unique();
    let lbl = label("balance");
    let value_key =
        zama_solana_acl::derive_value_key(acl_domain_key.to_bytes(), authority.to_bytes(), lbl);
    let (encrypted_value, _bump) = host::encrypted_value_address(value_key);
    let subjects = (0..=zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS)
        .map(|_| EncryptedValueSubjectGrant {
            subject: Pubkey::new_unique(),
        })
        .collect();

    let ix = create_encrypted_value_ix(
        payer,
        authority,
        encrypted_value,
        host_config,
        acl_domain_key,
        authority,
        lbl,
        handle_for_chain(1, 5),
        subjects,
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (payer, funded_system_account()),
        (authority, funded_system_account()),
        (encrypted_value, empty_system_account()),
        (host_config, host_config_account),
    ];
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::EncryptedValueSubjectCapacityExceeded,
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
    let acl_domain_key = Pubkey::new_unique();
    let lbl = label("balance");
    let handle = handle_for_chain(2, 5);
    let value_key =
        zama_solana_acl::derive_value_key(acl_domain_key.to_bytes(), authority.to_bytes(), lbl);
    let (encrypted_value, _bump) = host::encrypted_value_address(value_key);
    let context = mollusk().with_context(HashMap::from([
        (system_program::ID, system_program_account()),
        (payer, funded_system_account()),
        (authority, funded_system_account()),
        (encrypted_value, empty_system_account()),
        (host_config, host_config_account),
    ]));

    let create_ix = create_encrypted_value_ix(
        payer,
        authority,
        encrypted_value,
        host_config,
        acl_domain_key,
        authority,
        lbl,
        handle,
        vec![EncryptedValueSubjectGrant { subject: authority }],
    );
    context.process_and_validate_instruction(&create_ix, &[Check::success()]);

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
    let handle1 = handle_for_chain(8, 5);
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

    let update_ix = update_encrypted_value_ix(
        authority,
        authority,
        address,
        host_config,
        handle1,
        handle0,
        value_after_remove.subjects.clone(),
    );
    let accounts1 = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (address, encrypted_value_account(&value_after_remove)),
        (host_config, host_config_account),
    ];
    let result1 =
        mollusk().process_and_validate_instruction(&update_ix, &accounts1, &[Check::success()]);
    let updated = read_encrypted_value(&result1, address);
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
    let handle1 = handle_for_chain(10, 5);
    let (address, value0) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle0,
        &[owner, removed],
    );

    let update_ix = update_encrypted_value_ix(
        authority,
        authority,
        address,
        host_config,
        handle1,
        handle0,
        value0.subjects.clone(),
    );
    let accounts0 = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (address, encrypted_value_account(&value0)),
        (host_config, host_config_account.clone()),
    ];
    let result0 =
        mollusk().process_and_validate_instruction(&update_ix, &accounts0, &[Check::success()]);
    let value1 = read_encrypted_value(&result0, address);
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
// update_encrypted_value: supersession + previous-state mismatch (item 2c/2d)
// ---------------------------------------------------------------------------

#[test]
fn mollusk_update_encrypted_value_supersedes_and_appends_allowed_subject_leaves() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let subject_a = Pubkey::new_unique();
    let subject_b = Pubkey::new_unique();
    let old_handle = handle_for_chain(3, 5);
    let new_handle = handle_for_chain(4, 5);
    let (address, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        old_handle,
        &[subject_a, subject_b],
    );
    let previous_subjects = value.subjects.clone();

    let ix = update_encrypted_value_ix(
        authority,
        authority,
        address,
        host_config,
        new_handle,
        old_handle,
        previous_subjects,
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (address, encrypted_value_account(&value)),
        (host_config, host_config_account),
    ];
    let result = mollusk().process_and_validate_instruction(&ix, &accounts, &[Check::success()]);
    let updated = read_encrypted_value(&result, address);
    assert_eq!(updated.current_handle, new_handle);
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
fn mollusk_update_encrypted_value_rejects_stale_previous_subjects() {
    // Item 2c: submitting stale previous_subjects through the real instruction path.
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
        vec![Pubkey::new_unique()], // stale/wrong previous_subjects
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
            host::errors::ZamaHostError::PreviousStateMismatch,
        )],
    );
}

#[test]
fn mollusk_update_encrypted_value_rejects_stale_previous_handle() {
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
        handle_for_chain(99, 5), // wrong previous_handle
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
            host::errors::ZamaHostError::PreviousStateMismatch,
        )],
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
        &[custom_error(host::errors::ZamaHostError::AclSubjectDenied)],
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
        &[custom_error(host::errors::ZamaHostError::AclSubjectDenied)],
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
        &[custom_error(host::errors::ZamaHostError::HostConfigPaused)],
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
    let handle1 = handle_for_chain(11, 5);
    let handle2 = handle_for_chain(12, 5);
    let (address, value0) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle0,
        &[subject_a, subject_b],
    );

    let ix1 = update_encrypted_value_ix(
        authority,
        authority,
        address,
        host_config,
        handle1,
        handle0,
        value0.subjects.clone(),
    );
    let accounts1 = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (address, encrypted_value_account(&value0)),
        (host_config, host_config_account.clone()),
    ];
    let result1 = mollusk().process_and_validate_instruction(&ix1, &accounts1, &[Check::success()]);
    let value1 = read_encrypted_value(&result1, address);

    let ix2 = update_encrypted_value_ix(
        authority,
        authority,
        address,
        host_config,
        handle2,
        handle1,
        value1.subjects.clone(),
    );
    let accounts2 = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (address, encrypted_value_account(&value1)),
        (host_config, host_config_account),
    ];
    let result2 = mollusk().process_and_validate_instruction(&ix2, &accounts2, &[Check::success()]);
    let value2 = read_encrypted_value(&result2, address);

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
            handle1,
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
    let handle1 = handle_for_chain(21, 5);
    let handle2 = handle_for_chain(22, 5);
    let (address, value0) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle0,
        &[subject],
    );

    let ix1 = update_encrypted_value_ix(
        authority,
        authority,
        address,
        host_config,
        handle1,
        handle0,
        value0.subjects.clone(),
    );
    let accounts1 = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (address, encrypted_value_account(&value0)),
        (host_config, host_config_account.clone()),
    ];
    let result1 = mollusk().process_and_validate_instruction(&ix1, &accounts1, &[Check::success()]);
    let value1 = read_encrypted_value(&result1, address);

    let ix2 = update_encrypted_value_ix(
        authority,
        authority,
        address,
        host_config,
        handle2,
        handle1,
        value1.subjects.clone(),
    );
    let accounts2 = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (address, encrypted_value_account(&value1)),
        (host_config, host_config_account),
    ];
    let result2 = mollusk().process_and_validate_instruction(&ix2, &accounts2, &[Check::success()]);
    let value2 = read_encrypted_value(&result2, address);

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
            handle1,
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
        handle1,
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
    let handle1 = handle_for_chain(31, 5);
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

    let update_ix = update_encrypted_value_ix(
        authority,
        authority,
        address,
        host_config,
        handle1,
        handle0,
        value_public.subjects.clone(),
    );
    let accounts1 = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (address, encrypted_value_account(&value_public)),
        (host_config, host_config_account),
    ];
    let result1 =
        mollusk().process_and_validate_instruction(&update_ix, &accounts1, &[Check::success()]);
    let final_value = read_encrypted_value(&result1, address);

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
    assert!(
        zama_solana_acl::authorize_public(address.to_bytes(), &shared_final, handle1, &proof)
            .is_err()
    );
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
