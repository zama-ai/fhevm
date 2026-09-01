//! Fixtures shared by the host test binaries: `host_mollusk.rs` (behavior) and
//! `fhe_execute_boundary.rs` (the capacity instrument). Each binary compiles this module into
//! itself, so a helper used by only one of them is expected.
#![allow(dead_code)]

use anchor_lang::prelude::system_program;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use zama_host::encode::ExecutionDictionary;
use zama_host::{self as host, FheExecuteArgs, FheExecuteOutput, FheExecuteStep};
use zama_solana_test_kit::{
    anchor_ix, empty_system_account, event_authority, funded_system_account, label, readonly,
    system_program_account, writable, HostConfigParams,
};

pub fn host_config_account_with_flags(
    admin: Pubkey,
    paused: bool,
    grant_deny_list_enabled: bool,
) -> (Pubkey, Account) {
    zama_solana_test_kit::host_config_account(&HostConfigParams {
        paused,
        grant_deny_list_enabled,
        ..HostConfigParams::new(admin)
    })
}

pub fn host_config_account(admin: Pubkey) -> (Pubkey, Account) {
    host_config_account_with_flags(admin, false, false)
}

/// Builds an `fhe_execute` instruction. `remaining` accounts are appended in
/// order and referenced by index from `args`.
pub fn fhe_execute_ix(
    payer: Pubkey,
    compute_subject: Pubkey,
    encrypted_value_account_authority: Pubkey,
    host_config: Pubkey,
    args: FheExecuteArgs,
    remaining: Vec<AccountMeta>,
) -> Instruction {
    fhe_execute_ix_with_deny_records(
        payer,
        compute_subject,
        encrypted_value_account_authority,
        host_config,
        args,
        remaining,
        Vec::new(),
    )
}

pub fn fhe_execute_ix_with_deny_records(
    payer: Pubkey,
    compute_subject: Pubkey,
    encrypted_value_account_authority: Pubkey,
    host_config: Pubkey,
    mut args: FheExecuteArgs,
    remaining: Vec<AccountMeta>,
    deny_subject_records: Vec<Pubkey>,
) -> Instruction {
    // The execution self-describes its `remaining_accounts` length (DD-033); fixtures
    // build the account list here, so the declared count is stamped here too.
    args.account_count = u8::try_from(remaining.len() + deny_subject_records.len())
        .expect("fixture remaining accounts fit u8");
    let mut ix = anchor_ix(
        host::id(),
        host::accounts::FheExecute {
            payer,
            compute_subject,
            encrypted_value_account_authority,
            host_config,
            system_program: system_program::ID,
            // Unrestricted block cap (u64::MAX) in every existing fixture: block_cap
            // short-circuits before touching the optional accounts, so the two HCU
            // witnesses stay absent.
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: event_authority(host::id()),
            program: host::id(),
        },
        host::instruction::FheExecute { args },
    );
    ix.accounts.extend(remaining);
    ix.accounts
        .extend(deny_subject_records.into_iter().map(readonly));
    ix
}

pub struct CreatedPublicBatch {
    pub instruction: Instruction,
    pub accounts: Vec<(Pubkey, Account)>,
    pub outputs: Vec<(u16, Pubkey)>,
}

pub fn created_public_batch(
    step_count: usize,
    created_public_steps: &[usize],
) -> CreatedPublicBatch {
    let authority = Pubkey::new_unique();
    persistent_creates_batch(
        step_count,
        created_public_steps,
        authority,
        true,
        std::slice::from_ref(&authority),
    )
}

/// [`created_public_batch`] with a caller-fixed authority (for boundary sweeps recorded in the
/// cost snapshot: PDA bump searches are part of measured compute, so recorded profiles need
/// stable keys) and a caller-chosen `make_public` — `false` gives the plain persistent create,
/// the shape `zama-fhe`'s `heap_budget/` measures on the app side.
pub fn persistent_creates_batch(
    step_count: usize,
    created_public_steps: &[usize],
    authority: Pubkey,
    make_public: bool,
    subjects: &[Pubkey],
) -> CreatedPublicBatch {
    let (host_config, host_config_account) = host_config_account(authority);
    let mut output_metas = Vec::new();
    let mut output_accounts = Vec::new();
    let mut outputs = Vec::new();
    let mut steps = Vec::with_capacity(step_count);
    let mut dictionary = ExecutionDictionary::default();

    for step_index in 0..step_count {
        let output = if created_public_steps.contains(&step_index) {
            let output_label = label(&format!("created-public-{step_index}"));
            let encrypted_value_id = zama_solana_acl::derive_encrypted_value_id(
                authority.to_bytes(),
                authority.to_bytes(),
                output_label,
            );
            let output_address = host::encrypted_value_address(encrypted_value_id).0;
            let output_index = u8::try_from(output_metas.len()).unwrap();
            output_metas.push(writable(output_address));
            output_accounts.push((output_address, empty_system_account()));
            outputs.push((step_index as u16, output_address));
            FheExecuteOutput::StoredValue {
                output_encrypted_value_index: output_index,
                output_authority_index: None,
                output_domain_index: dictionary.intern_key(authority),
                output_account_index: dictionary.intern_key(authority),
                output_label_index: dictionary.intern(output_label),
                output_subject_indexes: dictionary.intern_subjects(subjects.iter().copied()),
                previous_state: None,
                make_public,
            }
        } else {
            FheExecuteOutput::Transient
        };
        steps.push(FheExecuteStep::TrivialEncrypt {
            plaintext: [(step_index + 1) as u8; 32],
            fhe_type: 5,
            output,
        });
    }

    let instruction = fhe_execute_ix(
        authority,
        authority,
        authority,
        host_config,
        FheExecuteArgs {
            account_count: 0,
            dictionary: dictionary.into_entries(),
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
    CreatedPublicBatch {
        instruction,
        accounts,
        outputs,
    }
}
