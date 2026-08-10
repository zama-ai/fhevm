//! Mollusk test for the `encrypted-counter` specimen — the copy-paste source for testing a new
//! `zama-host` consumer with `zama-solana-test-kit`: a fixture of about twenty lines, real host
//! CPIs, and cleartext-ledger assertions on the encrypted state.

use encrypted_counter as counter;
use mollusk_svm::result::Check;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
use std::collections::HashMap;
use zama_host as host;
use zama_solana_test_kit as kit;
use zama_solana_test_kit::oracle::CleartextLedger;
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
    let count_value = counter::count_encrypted_value_id(counter).address();
    let (host_config, host_config_data) = host_config_account(&HostConfigParams::new(owner));
    let mut mollusk = kit::svm(&counter::id(), "encrypted_counter");
    mollusk.add_program(&host::id(), "zama_host");
    kit::set_previous_bank_hash_sysvars(&mut mollusk);
    let context = mollusk.with_context(HashMap::from([
        (owner, system_account(50_000_000_000)),
        (host_config, host_config_data),
        (event_authority(host::id()), system_account(0)),
    ]));
    ensure_system_accounts(&context, &[counter, counter_authority, count_value]);
    let mut ledger = CleartextLedger::default();

    let initialize = |count_value: Pubkey| {
        anchor_ix(
            counter::id(),
            counter::accounts::Initialize {
                owner,
                counter,
                counter_authority,
                count_value,
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
                counter,
                counter_authority,
                count_value,
                host_config,
                zama_event_authority: event_authority(host::id()),
                zama_program: host::id(),
                system_program: anchor_lang::system_program::ID,
            },
            counter::instruction::Increment { amount },
        )
    };
    // Runs one counter instruction, replays its single `fhe_execute` CPI in cleartext, and
    // asserts the count behind the persisted handle.
    let assert_count = |ledger: &mut CleartextLedger, ix: &Instruction, expected: u64| {
        let result = context.process_and_validate_instruction(ix, &[Check::success()]);
        let replay = ledger.replay_fhe_cpis(&context, &result);
        assert_eq!(replay.executions, 1);
        assert_eq!(replay.persistent_outputs, 1);
        assert_eq!(ledger.u64_at(&context, count_value), expected);
    };

    // A wrongly derived encrypted value account is rejected before any CPI runs.
    let bogus_count_value = Pubkey::new_unique();
    ensure_system_accounts(&context, &[bogus_count_value]);
    context.process_and_validate_instruction(
        &initialize(bogus_count_value),
        &[anchor_error_check(
            counter::CounterError::CountValueInvalid as u32,
        )],
    );

    assert_count(&mut ledger, &initialize(count_value), 0);
    assert_count(&mut ledger, &increment(5), 5);
    assert_count(&mut ledger, &increment(37), 42);
}
