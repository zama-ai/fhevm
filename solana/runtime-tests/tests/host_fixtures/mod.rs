//! Fixtures shared by the host test binaries: `host_mollusk.rs` (behavior),
//! `host_admin_mollusk.rs` (admin setters), and `fhe_execute_boundary.rs` (the capacity
//! instrument). Each binary compiles this module into itself, so a helper used by only one of
//! them is expected.
#![allow(dead_code)]

use anchor_lang::prelude::system_program;
use anchor_lang::AccountDeserialize;
use mollusk_svm::MolluskContext;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use std::collections::HashMap;
use zama_host::encode::ExecutionDictionary;
use zama_host::{
    self as host, AppScope, FheExecuteArgs, FheExecuteOutput, FheExecuteStep, HostConfig, PdaSeed,
};
use zama_solana_test_kit::{
    anchor_ix, empty_system_account, event_authority, funded_system_account, host_svm, label,
    readonly, system_program_account, writable, HostConfigParams,
};

/// Seed tag of the fixture value authorities: `PDA("value-authority", seed_key)` of the program.
pub const VALUE_AUTHORITY_SEED: &[u8] = b"value-authority";

/// The scope every fixture application lives in unless a test picks its own.
pub fn fixture_scope() -> [u8; 32] {
    label("fixture-scope")
}

/// The authority that signs for one application's encrypted values: a PDA of the application
/// program, plus what a create needs to prove that (the seeds, bump last).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValueAuthority {
    pub program: Pubkey,
    pub key: Pubkey,
    pub seed_key: Pubkey,
    pub bump: u8,
}

/// The fixture value authority `PDA("value-authority", seed_key)` of `program`.
pub fn value_authority(program: Pubkey, seed_key: Pubkey) -> ValueAuthority {
    let (key, bump) =
        Pubkey::find_program_address(&[VALUE_AUTHORITY_SEED, seed_key.as_ref()], &program);
    ValueAuthority {
        program,
        key,
        seed_key,
        bump,
    }
}

/// The one authority of a program that needs no second one: seeded on the program key itself.
pub fn sole_value_authority(program: Pubkey) -> ValueAuthority {
    value_authority(program, program)
}

impl ValueAuthority {
    pub fn app(&self, scope: [u8; 32]) -> AppScope {
        AppScope {
            program: self.program,
            scope,
        }
    }

    /// The create-time proof that `key` is a PDA of `program`.
    pub fn seeds(&self, dictionary: &mut ExecutionDictionary) -> Vec<PdaSeed> {
        vec![
            PdaSeed::Literal {
                bytes: VALUE_AUTHORITY_SEED.to_vec(),
            },
            PdaSeed::Interned {
                index: dictionary.intern_key(self.seed_key),
            },
            PdaSeed::Literal {
                bytes: vec![self.bump],
            },
        ]
    }

    /// The canonical address of this authority's value at `value_label` in `scope`.
    pub fn value_address(&self, scope: [u8; 32], value_label: [u8; 32]) -> Pubkey {
        host::encrypted_value_address(self.program, self.key, scope, value_label).0
    }

    /// A persistent output of this authority: a create when `previous_handle` is `None` (carrying
    /// the PDA proof), an update otherwise.
    #[allow(clippy::too_many_arguments)]
    pub fn stored_output(
        &self,
        dictionary: &mut ExecutionDictionary,
        output_encrypted_value_index: u8,
        scope: [u8; 32],
        output_label: [u8; 32],
        allows: &[Pubkey],
        previous_handle: Option<[u8; 32]>,
        make_public: bool,
    ) -> FheExecuteOutput {
        FheExecuteOutput::StoredValue {
            output_encrypted_value_index,
            output_authority_index: None,
            output_program_index: dictionary.intern_key(self.program),
            output_authority_key_index: dictionary.intern_key(self.key),
            output_scope_index: dictionary.intern(scope),
            output_label_index: dictionary.intern(output_label),
            output_authority_seeds: match previous_handle {
                None => self.seeds(dictionary),
                Some(_) => Vec::new(),
            },
            output_allow_indexes: dictionary.intern_keys(allows.iter().copied()),
            previous_handle_index: previous_handle.map(|handle| dictionary.intern(handle)),
            make_public,
        }
    }
}

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

pub fn mollusk_execute_context(
    payer: Pubkey,
    seeded_accounts: Vec<(Pubkey, Account)>,
) -> MolluskContext<HashMap<Pubkey, Account>> {
    let mut accounts = HashMap::from([(payer, funded_system_account())]);
    for (pubkey, account) in seeded_accounts {
        accounts.insert(pubkey, account);
    }
    host_svm().with_context(accounts)
}

pub fn read_host_config(
    context: &MolluskContext<HashMap<Pubkey, Account>>,
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

/// The optional accounts of an `fhe_execute`: the application's deny record (appended last to
/// the remaining accounts, present iff the config enables the deny list) and the rand nonce
/// (present iff the execution has a rand step).
#[derive(Default)]
pub struct FheExecuteExtras {
    pub deny_scope_record: Option<Pubkey>,
    pub rand_nonce: Option<Pubkey>,
}

/// Builds an `fhe_execute` instruction. `remaining` accounts are appended in order and
/// referenced by index from `args`.
pub fn fhe_execute_ix(
    payer: Pubkey,
    encrypted_value_account_authority: Pubkey,
    host_config: Pubkey,
    args: FheExecuteArgs,
    remaining: Vec<AccountMeta>,
) -> Instruction {
    fhe_execute_ix_with_extras(
        payer,
        encrypted_value_account_authority,
        host_config,
        args,
        remaining,
        FheExecuteExtras::default(),
    )
}

pub fn fhe_execute_ix_with_extras(
    payer: Pubkey,
    encrypted_value_account_authority: Pubkey,
    host_config: Pubkey,
    mut args: FheExecuteArgs,
    remaining: Vec<AccountMeta>,
    extras: FheExecuteExtras,
) -> Instruction {
    // The execution self-describes its `remaining_accounts` length (DD-033); fixtures
    // build the account list here, so the declared count is stamped here too.
    args.account_count =
        u8::try_from(remaining.len() + usize::from(extras.deny_scope_record.is_some()))
            .expect("fixture remaining accounts fit u8");
    let mut ix = anchor_ix(
        host::id(),
        host::accounts::FheExecute {
            payer,
            encrypted_value_account_authority,
            host_config,
            system_program: system_program::ID,
            // Unrestricted block cap (u64::MAX) in every existing fixture: block_cap
            // short-circuits before touching the optional accounts, so the two HCU
            // witnesses stay absent.
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            rand_nonce: extras.rand_nonce,
            event_authority: event_authority(host::id()),
            program: host::id(),
        },
        host::instruction::FheExecute { args },
    );
    ix.accounts.extend(remaining);
    ix.accounts.extend(extras.deny_scope_record.map(readonly));
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
    let payer = Pubkey::new_unique();
    persistent_creates_batch(
        step_count,
        created_public_steps,
        payer,
        sole_value_authority(Pubkey::new_unique()),
        true,
        std::slice::from_ref(&payer),
    )
}

/// [`created_public_batch`] with caller-fixed keys (for boundary sweeps recorded in the cost
/// snapshot: PDA bump searches are part of measured compute, so recorded profiles need stable
/// keys), a caller-chosen `make_public` — `false` gives the plain persistent create, the shape
/// `zama-fhe`'s `heap_budget/` measures on the app side — and the keys every output allows.
pub fn persistent_creates_batch(
    step_count: usize,
    created_public_steps: &[usize],
    payer: Pubkey,
    authority: ValueAuthority,
    make_public: bool,
    allows: &[Pubkey],
) -> CreatedPublicBatch {
    let (host_config, host_config_account) = host_config_account(payer);
    let scope = fixture_scope();
    let mut output_metas = Vec::new();
    let mut output_accounts = Vec::new();
    let mut outputs = Vec::new();
    let mut steps = Vec::with_capacity(step_count);
    let mut dictionary = ExecutionDictionary::default();

    for step_index in 0..step_count {
        let output = if created_public_steps.contains(&step_index) {
            let output_label = label(&format!("created-public-{step_index}"));
            let output_address = authority.value_address(scope, output_label);
            let output_index = u8::try_from(output_metas.len()).unwrap();
            output_metas.push(writable(output_address));
            output_accounts.push((output_address, empty_system_account()));
            outputs.push((step_index as u16, output_address));
            authority.stored_output(
                &mut dictionary,
                output_index,
                scope,
                output_label,
                allows,
                None,
                make_public,
            )
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
        payer,
        authority.key,
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
        (payer, funded_system_account()),
        (authority.key, empty_system_account()),
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
