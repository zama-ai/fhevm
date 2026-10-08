//! Mollusk test for the `encrypted-counter` specimen — the copy-paste source for testing a new
//! `zama-host` consumer with `zama-solana-test-kit`: a fixture of about twenty lines, real host
//! CPIs on the cleartext host build, and assertions on the plaintexts it records in the store.

use encrypted_counter as counter;
use kit::transaction::process_fhe_instruction;
use mollusk_svm::result::Check;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
use std::collections::HashMap;
use zama_host as host;
use zama_solana_test_kit as kit;
use zama_solana_test_kit::{
    anchor_error_check, anchor_ix, ensure_system_accounts, event_authority, host_config_account,
    system_account, HostConfigParams,
};

#[test]
fn counter_initializes_to_zero_and_adds_increments() {
    // The fixture: what it costs to stand this consumer up against a real zama-host.
    let owner = Pubkey::new_unique();
    let counter = counter::counter_address(owner).0;
    let counter_authority = counter::counter_authority_address(counter).0;
    let encrypted_store = counter::counter_state_id(counter).address();
    let (host_config, host_config_data) = host_config_account(&HostConfigParams::new(owner));
    let mut mollusk = kit::svm(&counter::id(), "encrypted_counter");
    mollusk.add_program(&host::id(), kit::cleartext::HOST_PROGRAM);
    kit::set_previous_bank_hash_sysvars(&mut mollusk);
    let context = mollusk.with_context(HashMap::from([
        (owner, system_account(50_000_000_000)),
        (host_config, host_config_data),
        (event_authority(host::id()), system_account(0)),
    ]));
    ensure_system_accounts(&context, &[counter, counter_authority, encrypted_store]);

    let initialize = |encrypted_store: Pubkey| {
        anchor_ix(
            counter::id(),
            counter::accounts::Initialize {
                owner,
                transient_store: host::transient_store_address(owner).0,
                instructions: solana_sdk::sysvar::instructions::ID,
                counter,
                counter_authority,
                encrypted_store,
                host_config,
                zama_event_authority: event_authority(host::id()),
                zama_program: host::id(),
                system_program: anchor_lang::system_program::ID,
            },
            counter::instruction::Initialize {},
        )
    };
    let increment = |amount: u64| {
        anchor_ix(
            counter::id(),
            counter::accounts::Increment {
                owner,
                transient_store: host::transient_store_address(owner).0,
                instructions: solana_sdk::sysvar::instructions::ID,
                counter,
                counter_authority,
                encrypted_store,
                host_config,
                zama_event_authority: event_authority(host::id()),
                zama_program: host::id(),
                system_program: anchor_lang::system_program::ID,
            },
            counter::instruction::Increment { amount },
        )
    };
    // Runs one counter instruction and asserts the count the host recorded behind the handle.
    let assert_count = |ix: &Instruction, expected: u64| {
        process_fhe_instruction(&context, owner, ix, &[Check::success()]);
        assert_eq!(
            kit::cleartext::store_u64(&context, encrypted_store, counter::COUNT_KEY),
            expected
        );
    };

    // A wrongly derived encrypted store is rejected before any CPI runs.
    let bogus_encrypted_store = Pubkey::new_unique();
    ensure_system_accounts(&context, &[bogus_encrypted_store]);
    process_fhe_instruction(
        &context,
        owner,
        &initialize(bogus_encrypted_store),
        &[anchor_error_check(
            counter::CounterError::CountValueInvalid as u32,
        )],
    );

    assert_count(&initialize(encrypted_store), 0);
    assert_count(&increment(5), 5);
    assert_count(&increment(37), 42);
}
