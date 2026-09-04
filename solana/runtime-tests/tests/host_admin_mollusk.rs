//! HostAdmin setter persist tests: pause, grant-deny, admin transfer, EIP-712 domain.
//!
//! Split out of `host_mollusk.rs` so that file does not keep growing. These tests only
//! touch `HostConfig` through the admin account context; they do not share the
//! `FheExecutionFixture` harness.

use mollusk_svm::result::Check;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
use zama_host::{self as host};
use zama_solana_test_kit::{anchor_ix, event_authority};

mod host_fixtures;
use host_fixtures::{host_config_account, mollusk_execute_context, read_host_config};

fn set_host_pause_ix(
    program_id: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    paused: bool,
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::HostAdmin {
            admin,
            host_config,
            event_authority: event_authority(host::id()),
            program: host::id(),
        },
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
        host::accounts::HostAdmin {
            admin,
            host_config,
            event_authority: event_authority(host::id()),
            program: host::id(),
        },
        host::instruction::SetGrantDenyListEnabled { enabled },
    )
}

fn set_admin_ix(
    program_id: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    new_admin: Pubkey,
) -> Instruction {
    anchor_ix(
        program_id,
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

fn set_eip712_domain_ix(
    program_id: Pubkey,
    admin: Pubkey,
    host_config: Pubkey,
    gateway_chain_id: u64,
    input_verification_contract: [u8; 20],
    decryption_contract: [u8; 20],
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::HostAdmin {
            admin,
            host_config,
            event_authority: event_authority(host::id()),
            program: host::id(),
        },
        host::instruction::SetEip712Domain {
            gateway_chain_id,
            input_verification_contract,
            decryption_contract,
        },
    )
}

#[test]
fn mollusk_set_host_pause_persists() {
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_host_pause_ix(program_id, admin, host_config, true),
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
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_grant_deny_list_enabled_ix(program_id, admin, host_config, true),
        &[Check::success()],
    );
    assert!(
        read_host_config(&context, host_config)
            .expect("config")
            .grant_deny_list_enabled
    );
}

#[test]
fn mollusk_set_admin_transfers_to_pda_without_cosign() {
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (new_admin, _) = Pubkey::find_program_address(&[b"new-admin"], &host::id());
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_admin_ix(program_id, admin, host_config, new_admin),
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
fn mollusk_set_eip712_domain_persists_zeros() {
    let program_id = host::id();
    let admin = Pubkey::new_unique();
    let (host_config, account) = host_config_account(admin);
    let context = mollusk_execute_context(admin, vec![(host_config, account)]);

    context.process_and_validate_instruction(
        &set_eip712_domain_ix(program_id, admin, host_config, 0, [0u8; 20], [0u8; 20]),
        &[Check::success()],
    );
    let config = read_host_config(&context, host_config).expect("config");
    assert_eq!(config.gateway_chain_id, 0);
    assert_eq!(config.input_verification_contract, [0u8; 20]);
    assert_eq!(config.decryption_contract, [0u8; 20]);
}
