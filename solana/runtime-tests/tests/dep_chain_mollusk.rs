//! Mollusk test for the `dep-chain` specimen — the load-smoke shape: one `fhe_execute` carrying
//! the on-chain builder's full 16-step ceiling (`zama_fhe::MAX_ON_CHAIN_EXECUTION_STEPS` — the
//! deepest chain a CPI-composing program can build on the fixed 32 KB program heap) as a strictly
//! DEPENDENT add chain (each step's operand is the previous step's transient result), proving the
//! kit's cleartext oracle replays transient intermediates and that a full-depth chain fits one
//! instruction's compute budget. The host's own 32-step ceiling is exercised by `host_mollusk`'s
//! MAX_FHE_EXECUTION_STEPS executions and, live, by the load-smoke scenario's raw typed client.

use dep_chain as chain_program;
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
fn full_depth_dependent_chain_computes_in_one_execution() {
    let owner = Pubkey::new_unique();
    let chain = chain_program::chain_address(owner).0;
    let chain_authority = chain_program::chain_authority_address(chain).0;
    let tail_value = chain_program::tail_encrypted_value_id(chain).address();
    let (host_config, host_config_data) = host_config_account(&HostConfigParams::new(owner));
    let mut mollusk = kit::svm(&chain_program::id(), "dep_chain");
    mollusk.add_program(&host::id(), "zama_host");
    kit::set_previous_bank_hash_sysvars(&mut mollusk);
    let context = mollusk.with_context(HashMap::from([
        (owner, system_account(50_000_000_000)),
        (host_config, host_config_data),
        (event_authority(host::id()), system_account(0)),
    ]));
    ensure_system_accounts(&context, &[chain, chain_authority, tail_value]);
    let mut ledger = CleartextLedger::default();

    let initialize = || {
        anchor_ix(
            chain_program::id(),
            chain_program::accounts::Initialize {
                owner,
                chain,
                chain_authority,
                tail_value,
                host_config,
                zama_event_authority: event_authority(host::id()),
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
                tail_value,
                host_config,
                zama_event_authority: event_authority(host::id()),
                zama_program: host::id(),
                system_program: anchor_lang::system_program::ID,
            },
            chain_program::instruction::Extend { links, amount },
        )
    };
    // Runs one chain instruction, replays its single `fhe_execute` CPI in cleartext — every
    // transient link included — and asserts the tail behind the one persisted handle.
    let assert_tail = |ledger: &mut CleartextLedger, ix: &Instruction, expected: u64| {
        let result = context.process_and_validate_instruction(ix, &[Check::success()]);
        let replay = ledger.replay_fhe_cpis(&context, &result);
        assert_eq!(replay.executions, 1);
        assert_eq!(replay.persistent_outputs, 1);
        assert_eq!(ledger.u64_at(&context, tail_value), expected);
    };

    assert_tail(&mut ledger, &initialize(), 0);
    // The full-depth chain: 16 dependent adds in one execution — the on-chain builder's ceiling.
    assert_tail(&mut ledger, &extend(chain_program::MAX_CHAIN_LINKS, 1), 16);
    // A short chain over the persisted tail; and the single-link degenerate form persists directly.
    assert_tail(&mut ledger, &extend(4, 2), 24);
    assert_tail(&mut ledger, &extend(1, 2), 26);

    // Chain-length bounds fail closed before any CPI runs.
    context.process_and_validate_instruction(
        &extend(0, 1),
        &[anchor_error_check(
            chain_program::DepChainError::InvalidChainLength as u32,
        )],
    );
    context.process_and_validate_instruction(
        &extend(chain_program::MAX_CHAIN_LINKS + 1, 1),
        &[anchor_error_check(
            chain_program::DepChainError::InvalidChainLength as u32,
        )],
    );
}
