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
use std::path::PathBuf;
use zama_host::{
    self as host, instructions::EncryptedValueSubjectGrant, EncryptedValue, FheBinaryOpCode,
    FheEvalArgs, FheEvalOperand, FheEvalOutput, FheEvalStep, HostConfig,
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
                gateway_chain_id: GATEWAY_CHAIN_ID,
                input_verification_contract: INPUT_VERIFICATION_CONTRACT,
                coprocessor_signer: [0u8; 20],
                decryption_contract: DECRYPTION_CONTRACT,
                current_kms_context_id: 0,
                material_authority: admin,
                test_authority: admin,
                paused: false,
                mock_input_enabled: false,
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
    subjects: &[(Pubkey, u8)],
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
        subjects: subjects.iter().map(|(p, _)| *p).collect(),
        subject_roles: subjects.iter().map(|(_, r)| *r).collect(),
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
    anchor_ix(
        host::id(),
        host::accounts::CreateEncryptedValue {
            payer,
            app_account_authority,
            encrypted_value,
            host_config,
            deny_subject_record: None,
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
    anchor_ix(
        host::id(),
        host::accounts::AllowEncryptedValueSubjects {
            payer,
            authority,
            encrypted_value,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
        },
        host::instruction::AllowSubjects { subjects },
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
    anchor_ix(
        host::id(),
        host::accounts::UpdateEncryptedValue {
            payer,
            app_account_authority,
            encrypted_value,
            host_config,
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
) -> Instruction {
    anchor_ix(
        host::id(),
        host::accounts::MakeEncryptedValueHandlePublic {
            payer,
            authority,
            encrypted_value,
            host_config,
            deny_subject_record: None,
            system_program: system_program::ID,
        },
        host::instruction::MakeHandlePublic {},
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
    ix
}

fn writable(pubkey: Pubkey) -> AccountMeta {
    AccountMeta::new(pubkey, false)
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
        vec![EncryptedValueSubjectGrant {
            subject,
            role_flags: host::ACL_ROLE_USE | host::ACL_ROLE_GRANT,
        }],
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
fn mollusk_create_encrypted_value_rejects_public_decrypt_at_birth() {
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
        vec![EncryptedValueSubjectGrant {
            subject: Pubkey::new_unique(),
            role_flags: host::ACL_ROLE_USE | host::ACL_ROLE_PUBLIC_DECRYPT,
        }],
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
            host::errors::ZamaHostError::PublicDecryptAtBirthUnsupported,
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
            EncryptedValueSubjectGrant {
                subject: dup,
                role_flags: host::ACL_ROLE_USE,
            },
            EncryptedValueSubjectGrant {
                subject: dup,
                role_flags: host::ACL_ROLE_GRANT,
            },
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
        &[custom_error(
            host::errors::ZamaHostError::SubjectMissingRole,
        )],
    );
}

// ---------------------------------------------------------------------------
// allow_subjects
// ---------------------------------------------------------------------------

#[test]
fn mollusk_allow_subjects_adds_new_subject_and_extends_roles() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let owner = Pubkey::new_unique();
    let new_subject = Pubkey::new_unique();
    let (address, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle_for_chain(2, 5),
        &[(owner, host::ACL_ROLE_USE | host::ACL_ROLE_GRANT)],
    );

    let ix = allow_subjects_ix(
        authority,
        owner,
        address,
        host_config,
        vec![
            EncryptedValueSubjectGrant {
                subject: new_subject,
                role_flags: host::ACL_ROLE_USE,
            },
            // Idempotent role extension for an existing subject.
            EncryptedValueSubjectGrant {
                subject: owner,
                role_flags: host::ACL_ROLE_PUBLIC_DECRYPT,
            },
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
    assert!(updated.subject_has_role(owner, host::ACL_ROLE_PUBLIC_DECRYPT));
    assert!(updated.subject_has_role(new_subject, host::ACL_ROLE_USE));
    assert_eq!(updated.leaf_count, 0); // allow_subjects never appends leaves
}

#[test]
fn mollusk_allow_subjects_rejects_authority_without_grant_role() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let use_only = Pubkey::new_unique();
    let (address, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle_for_chain(2, 5),
        &[(use_only, host::ACL_ROLE_USE)],
    );
    let ix = allow_subjects_ix(
        authority,
        use_only,
        address,
        host_config,
        vec![EncryptedValueSubjectGrant {
            subject: Pubkey::new_unique(),
            role_flags: host::ACL_ROLE_USE,
        }],
    );
    let accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (use_only, funded_system_account()),
        (address, encrypted_value_account(&value)),
        (host_config, host_config_account),
    ];
    mollusk().process_and_validate_instruction(
        &ix,
        &accounts,
        &[custom_error(
            host::errors::ZamaHostError::SubjectMissingRole,
        )],
    );
}

// ---------------------------------------------------------------------------
// update_encrypted_value: supersession + previous-state mismatch (item 2c/2d)
// ---------------------------------------------------------------------------

#[test]
fn mollusk_update_encrypted_value_supersedes_and_appends_use_role_leaves_only() {
    // Item 2d: a GRANT-only subject (no USE) must get no historical leaf.
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let use_subject = Pubkey::new_unique();
    let grant_only_subject = Pubkey::new_unique();
    let old_handle = handle_for_chain(3, 5);
    let new_handle = handle_for_chain(4, 5);
    let (address, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        old_handle,
        &[
            (use_subject, host::ACL_ROLE_USE),
            (grant_only_subject, host::ACL_ROLE_GRANT),
        ],
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
    assert_eq!(updated.leaf_count, 1); // only use_subject gets a leaf

    let expected_leaf = zama_solana_acl::historical_access_leaf_commitment(
        address.to_bytes(),
        0,
        old_handle,
        use_subject.to_bytes(),
    );
    let mut expected_peaks = Vec::new();
    let mut expected_count = 0u64;
    zama_solana_acl::mmr_append(&mut expected_peaks, &mut expected_count, expected_leaf).unwrap();
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
        &[(subject, host::ACL_ROLE_USE)],
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
        &[(subject, host::ACL_ROLE_USE)],
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
        &[(subject, host::ACL_ROLE_USE | host::ACL_ROLE_PUBLIC_DECRYPT)],
    );
    let ix = make_handle_public_ix(authority, subject, address, host_config);
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
fn mollusk_make_handle_public_rejects_subject_without_role() {
    let authority = Pubkey::new_unique();
    let (host_config, host_config_account) = host_config_account(authority);
    let subject = Pubkey::new_unique();
    let (address, value) = new_lineage(
        Pubkey::new_unique(),
        authority,
        label("balance"),
        handle_for_chain(5, 5),
        &[(subject, host::ACL_ROLE_USE)], // no PUBLIC_DECRYPT
    );
    let ix = make_handle_public_ix(authority, subject, address, host_config);
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
            host::errors::ZamaHostError::SubjectMissingRole,
        )],
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
        &[
            (subject_a, host::ACL_ROLE_USE),
            (subject_b, host::ACL_ROLE_USE),
        ],
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
        &[(subject, host::ACL_ROLE_USE)],
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
        &[(subject, host::ACL_ROLE_USE | host::ACL_ROLE_PUBLIC_DECRYPT)],
    );

    let make_public_ix = make_handle_public_ix(authority, subject, address, host_config);
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
    let (lhs_address, lhs_value) = new_lineage(
        authority,
        authority,
        label("lhs"),
        lhs,
        &[(authority, host::ACL_ROLE_USE)],
    );
    let (rhs_address, rhs_value) = new_lineage(
        authority,
        authority,
        label("rhs"),
        rhs,
        &[(authority, host::ACL_ROLE_USE)],
    );
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
                output_subjects: vec![host::AclSubjectEntry {
                    pubkey: authority,
                    role_flags: host::ACL_ROLE_USE,
                }],
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
        &[(authority, host::ACL_ROLE_USE)],
    );
    let output_handle = handle_for_chain(43, 5);
    let (output_address, output_value) = new_lineage(
        authority,
        authority,
        label("out"),
        output_handle,
        &[(authority, host::ACL_ROLE_USE)],
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
                output_subjects: vec![host::AclSubjectEntry {
                    pubkey: authority,
                    role_flags: host::ACL_ROLE_USE,
                }],
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
