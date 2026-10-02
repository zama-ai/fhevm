//! Mollusk test for the `dep-chain` specimen — the load-smoke shape AND the at-cap heap proof:
//! one `fhe_execute` carrying the host's full `MAX_FHE_EXECUTION_STEPS` ceiling as a strictly
//! DEPENDENT add chain (each step's operand is the previous step's transient result). Extending at
//! full depth builds and invokes a maximum execution inside the specimen program, so this test is
//! what verifies under SBF that an at-cap on-chain build — tables, packet, account resolution,
//! Anchor deserialization — fits the fixed 32 KB program heap that `heap_budget/` counts
//! host-side. On the cleartext host build it also proves transient intermediates resolve to their
//! plaintexts on chain and that a full-depth chain fits one instruction's compute budget with the
//! simulator's evaluation added.

use dep_chain as chain_program;
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
fn full_depth_dependent_chain_computes_in_one_execution() {
    let owner = Pubkey::new_unique();
    let chain = chain_program::chain_address(owner).0;
    let chain_authority = chain_program::chain_authority_address(chain).0;
    let encrypted_store = chain_program::chain_state_id(chain).address();
    let (host_config, host_config_data) = host_config_account(&HostConfigParams::new(owner));
    let mut mollusk = kit::svm(&chain_program::id(), "dep_chain");
    mollusk.add_program(&host::id(), kit::cleartext::HOST_PROGRAM);
    kit::set_previous_bank_hash_sysvars(&mut mollusk);
    let context = mollusk.with_context(HashMap::from([
        (owner, system_account(50_000_000_000)),
        (host_config, host_config_data),
        (event_authority(host::id()), system_account(0)),
    ]));
    ensure_system_accounts(&context, &[chain, chain_authority, encrypted_store]);

    let initialize = || {
        anchor_ix(
            chain_program::id(),
            chain_program::accounts::Initialize {
                owner,
                chain,
                chain_authority,
                encrypted_store,
                host_config,
                zama_event_authority: event_authority(host::id()),
                transient_store: host::transient_store_address(owner).0,
                instructions: solana_sdk::sysvar::instructions::ID,
                zama_program: host::id(),
                system_program: anchor_lang::system_program::ID,
            },
            chain_program::instruction::Initialize {},
        )
    };
    let extend = |links: u8, amount: u64| {
        anchor_ix(
            chain_program::id(),
            chain_program::accounts::Extend {
                owner,
                chain,
                chain_authority,
                encrypted_store,
                host_config,
                zama_event_authority: event_authority(host::id()),
                transient_store: host::transient_store_address(owner).0,
                instructions: solana_sdk::sysvar::instructions::ID,
                zama_program: host::id(),
                system_program: anchor_lang::system_program::ID,
            },
            chain_program::instruction::Extend { links, amount },
        )
    };
    // Runs one chain instruction and asserts the tail the host recorded behind the one persisted
    // handle, which every transient link fed.
    let assert_tail = |ix: &Instruction, expected: u64| {
        kit::transaction::process_fhe_instruction(&context, owner, ix, &[Check::success()]);
        assert_eq!(
            kit::cleartext::store_u64(
                &context,
                encrypted_store,
                chain_program::encrypted_tail_label()
            ),
            expected
        );
    };

    assert_tail(&initialize(), 0);
    // The full-depth chain: 32 dependent adds in one execution — the host's cap, built on-chain,
    // which is the at-cap heap proof described in the module docs.
    assert_tail(&extend(chain_program::MAX_CHAIN_LINKS, 1), 32);
    // A short chain over the persisted tail; and the single-link degenerate form persists directly.
    assert_tail(&extend(4, 2), 40);
    assert_tail(&extend(1, 2), 42);

    // Chain-length bounds fail closed before any CPI runs.
    kit::transaction::process_fhe_instruction(
        &context,
        owner,
        &extend(0, 1),
        &[anchor_error_check(
            chain_program::DepChainError::InvalidChainLength as u32,
        )],
    );
    kit::transaction::process_fhe_instruction(
        &context,
        owner,
        &extend(chain_program::MAX_CHAIN_LINKS + 1, 1),
        &[anchor_error_check(
            chain_program::DepChainError::InvalidChainLength as u32,
        )],
    );
}
