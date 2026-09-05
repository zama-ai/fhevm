//! Host admin surface: `initialize_host_config` and the `HostAdmin` setters (pause,
//! grant-deny, admin transfer, EIP-712 domain).
//!
//! Split out of `host_mollusk.rs` so that file does not keep growing. These tests only
//! touch `HostConfig` through the admin account contexts; they do not share the
//! `FheExecutionFixture` harness.

use anchor_lang::{prelude::system_program, InstructionData};
use mollusk_svm::result::Check;
use solana_sdk::{account::Account, instruction::Instruction, pubkey::Pubkey};
use zama_host::{self as host, errors::ZamaHostError};
use zama_solana_test_kit::{
    anchor_error_check, anchor_framework_error_check, anchor_ix, event_authority,
    program_data_account, system_account,
};

mod host_fixtures;
use host_fixtures::{host_config_account, mollusk_execute_context, read_host_config};

fn custom_error(error: ZamaHostError) -> Check<'static> {
    anchor_error_check(error as u32)
}

fn program_owned_account() -> Account {
    Account {
        lamports: 1_000_000,
        data: vec![],
        owner: host::id(),
        executable: false,
        rent_epoch: 0,
    }
}

// ---- initialize_host_config ----

/// Args that initialize a config with the given coprocessor signer set + threshold, valid in
/// every other respect. Callers vary only the set/threshold to exercise the registration
/// invariants.
fn init_args(signers: Vec<[u8; 20]>, threshold: u8) -> host::InitializeHostConfigArgs {
    host::InitializeHostConfigArgs {
        chain_id: host::SOLANA_POC_CHAIN_ID,
        gateway_chain_id: 0,
        input_verification_contract: [0xCDu8; 20],
        coprocessor_signers: signers,
        coprocessor_threshold: threshold,
        decryption_contract: [0u8; 20],
        grant_deny_list_enabled: false,
    }
}

struct InitializeHostConfig {
    payer: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    program_data: Pubkey,
    context: mollusk_svm::MolluskContext<std::collections::HashMap<Pubkey, Account>>,
}

impl InitializeHostConfig {
    /// The runtime an operator sees: the admin is the program's upgrade authority.
    fn new() -> Self {
        let admin = Pubkey::new_unique();
        Self::build(admin, Some(admin))
    }

    /// A runtime whose `ProgramData` names `upgrade_authority`, which is not the admin.
    fn with_upgrade_authority(upgrade_authority: Option<Pubkey>) -> Self {
        Self::build(Pubkey::new_unique(), upgrade_authority)
    }

    fn build(admin: Pubkey, upgrade_authority: Option<Pubkey>) -> Self {
        let payer = Pubkey::new_unique();
        let (host_config, _) = host::host_config_address();
        let (program_data, program_data_acct) = program_data_account(upgrade_authority);
        let context = mollusk_execute_context(
            payer,
            vec![
                (host_config, system_account(0)),
                (program_data, program_data_acct),
            ],
        );
        Self {
            payer,
            admin,
            host_config,
            program_data,
            context,
        }
    }

    fn ix(&self, args: host::InitializeHostConfigArgs) -> Instruction {
        anchor_ix(
            host::id(),
            host::accounts::InitializeHostConfig {
                payer: self.payer,
                admin: self.admin,
                program_data: self.program_data,
                host_config: self.host_config,
                system_program: system_program::ID,
                event_authority: event_authority(host::id()),
                program: host::id(),
            },
            host::instruction::InitializeHostConfig { args },
        )
    }

    fn run(&self, ix: &Instruction, check: Check) {
        self.context.process_and_validate_instruction(ix, &[check]);
    }

    fn config(&self) -> host::HostConfig {
        read_host_config(&self.context, self.host_config).expect("config")
    }
}

#[test]
fn mollusk_initialize_host_config_defaults_block_cap_to_unrestricted() {
    // A freshly initialized config ships unrestricted (u64::MAX), not banned (0).
    let init = InitializeHostConfig::new();
    init.run(&init.ix(init_args(vec![[0x11u8; 20]], 1)), Check::success());
    assert_eq!(init.config().hcu_block_cap_per_app, u64::MAX);
}

#[test]
fn mollusk_initialize_host_config_stores_registered_signer_set_and_threshold() {
    // A valid n-of-m set round-trips into the stored config: distinct signers packed, count and
    // threshold recorded.
    let init = InitializeHostConfig::new();
    let signers = vec![[0x11u8; 20], [0x22u8; 20], [0x33u8; 20]];
    init.run(&init.ix(init_args(signers.clone(), 2)), Check::success());
    let config = init.config();
    assert_eq!(config.coprocessor_signer_count, 3);
    assert_eq!(config.coprocessor_threshold, 2);
    assert_eq!(config.active_coprocessor_signers(), signers.as_slice());
}

#[test]
fn mollusk_initialize_host_config_rejects_invalid_coprocessor_sets() {
    let cases: [(&str, Vec<[u8; 20]>, u8, ZamaHostError); 6] = [
        // A duplicate would silently raise the effective quorum (distinct-signer counting).
        (
            "duplicate signer",
            vec![[0x11u8; 20], [0x11u8; 20]],
            1,
            ZamaHostError::DuplicateCoprocessorSigner,
        ),
        // threshold 3 > 2 signers: unsatisfiable.
        (
            "threshold above count",
            vec![[0x11u8; 20], [0x22u8; 20]],
            3,
            ZamaHostError::InvalidCoprocessorThreshold,
        ),
        (
            "zero threshold",
            vec![[0x11u8; 20]],
            0,
            ZamaHostError::InvalidCoprocessorThreshold,
        ),
        (
            "empty set",
            vec![],
            1,
            ZamaHostError::EmptyCoprocessorSignerSet,
        ),
        (
            "zero signer",
            vec![[0u8; 20]],
            1,
            ZamaHostError::ZeroCoprocessorSigner,
        ),
        (
            "too many signers",
            (1..=9).map(|i| [i; 20]).collect(),
            1,
            ZamaHostError::TooManyCoprocessorSigners,
        ),
    ];
    for (name, signers, threshold, error) in cases {
        let init = InitializeHostConfig::new();
        init.run(&init.ix(init_args(signers, threshold)), custom_error(error));
        assert!(
            read_host_config(&init.context, init.host_config).is_none(),
            "{name}: config was created"
        );
    }
}

#[test]
fn mollusk_initialize_host_config_rejects_admin_who_is_not_the_upgrade_authority() {
    let init = InitializeHostConfig::with_upgrade_authority(Some(Pubkey::new_unique()));
    init.run(
        &init.ix(init_args(vec![[0x11u8; 20]], 1)),
        custom_error(ZamaHostError::HostConfigAdminMismatch),
    );
}

#[test]
fn mollusk_initialize_host_config_rejects_finalized_program() {
    let init = InitializeHostConfig::with_upgrade_authority(None);
    init.run(
        &init.ix(init_args(vec![[0x11u8; 20]], 1)),
        custom_error(ZamaHostError::HostConfigAdminMismatch),
    );
}

#[test]
fn mollusk_initialize_host_config_rejects_unsigned_admin() {
    let init = InitializeHostConfig::new();
    let mut ix = init.ix(init_args(vec![[0x11u8; 20]], 1));
    for meta in ix.accounts.iter_mut() {
        if meta.pubkey == init.admin {
            meta.is_signer = false;
        }
    }
    init.run(
        &ix,
        anchor_framework_error_check(anchor_lang::error::ErrorCode::AccountNotSigner),
    );
}

#[test]
fn mollusk_initialize_host_config_rejects_wrong_program_data_address() {
    let init = InitializeHostConfig::new();
    let fake = Pubkey::new_unique();
    let program_data_acct = init
        .context
        .account_store
        .borrow()
        .get(&init.program_data)
        .cloned()
        .expect("program data");
    init.context
        .account_store
        .borrow_mut()
        .insert(fake, program_data_acct);
    let mut ix = init.ix(init_args(vec![[0x11u8; 20]], 1));
    for meta in ix.accounts.iter_mut() {
        if meta.pubkey == init.program_data {
            meta.pubkey = fake;
        }
    }
    init.run(
        &ix,
        anchor_framework_error_check(anchor_lang::error::ErrorCode::ConstraintAddress),
    );
}

// ---- HostAdmin setters ----

/// A `HostAdmin`-context instruction; the payload names the setter at the call site.
fn host_admin_ix(admin: Pubkey, host_config: Pubkey, data: impl InstructionData) -> Instruction {
    anchor_ix(
        host::id(),
        host::accounts::HostAdmin {
            admin,
            host_config,
            event_authority: event_authority(host::id()),
            program: host::id(),
        },
        data,
    )
}

fn set_admin_ix(admin: Pubkey, host_config: Pubkey, new_admin: Pubkey) -> Instruction {
    anchor_ix(
        host::id(),
        host::accounts::SetAdmin {
            admin,
            host_config,
            new_admin,
            event_authority: event_authority(host::id()),
            program: host::id(),
        },
        host::instruction::SetAdmin { new_admin },
    )
}

#[test]
fn mollusk_set_host_pause_persists() {
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &host_admin_ix(
            admin,
            host_config,
            host::instruction::SetHostPause { paused: true },
        ),
        &[Check::success()],
    );
    assert!(
        read_host_config(&context, host_config)
            .expect("config")
            .paused
    );
}

#[test]
fn mollusk_set_grant_deny_list_enabled_persists() {
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &host_admin_ix(
            admin,
            host_config,
            host::instruction::SetGrantDenyListEnabled { enabled: true },
        ),
        &[Check::success()],
    );
    assert!(
        read_host_config(&context, host_config)
            .expect("config")
            .grant_deny_list_enabled
    );
}

#[test]
fn mollusk_set_admin_transfers_to_cosigning_keypair() {
    // The everyday rotation: a wallet key that co-signs the transfer.
    let admin = Pubkey::new_unique();
    let new_admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);
    let mut ix = set_admin_ix(admin, host_config, new_admin);
    for meta in ix.accounts.iter_mut() {
        if meta.pubkey == new_admin {
            meta.is_signer = true;
        }
    }

    context.process_and_validate_instruction(&ix, &[Check::success()]);
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .admin,
        new_admin
    );
}

#[test]
fn mollusk_set_admin_rejects_keypair_that_does_not_cosign() {
    // An on-curve key can only become admin by signing; the current admin cannot hand the role
    // to a wallet that never agreed to take it.
    let admin = Pubkey::new_unique();
    let new_admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_admin_ix(admin, host_config, new_admin),
        &[custom_error(ZamaHostError::HostConfigAdminMismatch)],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .admin,
        admin
    );
}

#[test]
fn mollusk_set_admin_transfers_to_pda_without_cosign() {
    let admin = Pubkey::new_unique();
    let (new_admin, _) = Pubkey::find_program_address(&[b"new-admin"], &host::id());
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(
        admin,
        vec![(host_config, account), (new_admin, program_owned_account())],
    );

    context.process_and_validate_instruction(
        &set_admin_ix(admin, host_config, new_admin),
        &[Check::success()],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .admin,
        new_admin
    );
}

#[test]
fn mollusk_set_admin_rejects_unallocated_off_curve_key() {
    // An off-curve key that is still System-owned (never allocated) must not skip co-sign.
    let admin = Pubkey::new_unique();
    let (new_admin, _) = Pubkey::find_program_address(&[b"typo"], &host::id());
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_admin_ix(admin, host_config, new_admin),
        &[custom_error(ZamaHostError::HostConfigAdminMismatch)],
    );
    assert_eq!(
        read_host_config(&context, host_config)
            .expect("config")
            .admin,
        admin
    );
}

#[test]
fn mollusk_set_eip712_domain_persists_zeros() {
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &host_admin_ix(
            admin,
            host_config,
            host::instruction::SetEip712Domain {
                gateway_chain_id: 0,
                input_verification_contract: [0u8; 20],
                decryption_contract: [0u8; 20],
            },
        ),
        &[Check::success()],
    );
    let config = read_host_config(&context, host_config).expect("config");
    assert_eq!(config.gateway_chain_id, 0);
    assert_eq!(config.input_verification_contract, [0u8; 20]);
    assert_eq!(config.decryption_contract, [0u8; 20]);
}
