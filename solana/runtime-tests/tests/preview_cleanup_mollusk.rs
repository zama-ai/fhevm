//! Actual SPL/System CPIs in every preview application's administrative cleanup.
use anchor_lang::solana_program::bpf_loader_upgradeable;
use mollusk_svm::result::Check;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use std::collections::HashMap;
use zama_solana_test_kit::{
    funded_system_account, program_data_account, spl_mint_account, spl_token_account,
    system_account,
};

fn programs() -> [(Pubkey, &'static str); 3] {
    [
        (demo_vault::id(), "demo_vault_admin_sweep"),
        (confidential_token::id(), "confidential_token_admin_sweep"),
        (
            confidential_batcher::id(),
            "confidential_batcher_admin_sweep",
        ),
    ]
}
fn seeds_data(discriminator: &[u8], seeds: &[&[u8]]) -> Vec<u8> {
    let mut data = discriminator.to_vec();
    data.extend((seeds.len() as u32).to_le_bytes());
    for seed in seeds {
        data.extend((seed.len() as u32).to_le_bytes());
        data.extend(*seed);
    }
    data
}

#[test]
fn preview_cleanup_checks_authority_and_refunds_token_rent_and_pda_sol() {
    for (program, artifact) in programs() {
        let admin = Pubkey::new_unique();
        let stranger = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token = Pubkey::new_unique();
        let (authority, bump) = Pubkey::find_program_address(&[b"cleanup-test"], &program);
        let program_data = bpf_loader_upgradeable::get_program_data_address(&program);
        let token_account = spl_token_account(mint, authority, 7);
        let token_rent = token_account.lamports;
        let funded = funded_system_account();
        let before = funded.lamports;
        let mut svm = zama_solana_test_kit::svm(&program, artifact);
        mollusk_svm_programs_token::token::add_program(&mut svm);
        let context = svm.with_context(HashMap::from([
            (admin, funded),
            (stranger, funded_system_account()),
            (program_data, program_data_account(Some(admin)).1),
            (mint, spl_mint_account(Some(admin), 7)),
            (token, token_account),
            (
                authority,
                Account {
                    lamports: 12345,
                    ..system_account(0)
                },
            ),
        ]));
        let mut close = Instruction {
            program_id: program,
            accounts: vec![
                AccountMeta::new(admin, true),
                AccountMeta::new_readonly(program_data, false),
                AccountMeta::new_readonly(authority, false),
                AccountMeta::new(token, false),
                AccountMeta::new(mint, false),
                AccountMeta::new_readonly(anchor_spl::token::ID, false),
            ],
            data: seeds_data(
                &[64, 227, 93, 185, 105, 167, 223, 84],
                &[b"cleanup-test", &[bump]],
            ),
        };
        close.accounts[0] = AccountMeta::new(stranger, true);
        assert!(context.process_instruction(&close).program_result.is_err());
        assert_eq!(
            context.account_store.borrow().get(&token).unwrap().lamports,
            token_rent
        );
        close.accounts[0] = AccountMeta::new(admin, true);
        let good_data = close.data.clone();
        close.data = seeds_data(&[64, 227, 93, 185, 105, 167, 223, 84], &[b"wrong", &[bump]]);
        assert!(context.process_instruction(&close).program_result.is_err());
        close.data = good_data;
        context.process_and_validate_instruction(&close, &[Check::success()]);
        assert_eq!(
            context.account_store.borrow().get(&admin).unwrap().lamports,
            before + token_rent
        );
        assert_eq!(
            context
                .account_store
                .borrow()
                .get(&token)
                .map_or(0, |a| a.lamports),
            0
        );
        let drain = Instruction {
            program_id: program,
            accounts: vec![
                AccountMeta::new(admin, true),
                AccountMeta::new_readonly(program_data, false),
                AccountMeta::new(authority, false),
                AccountMeta::new_readonly(anchor_lang::system_program::ID, false),
            ],
            data: seeds_data(
                &[229, 176, 14, 73, 64, 247, 65, 76],
                &[b"cleanup-test", &[bump]],
            ),
        };
        context.process_and_validate_instruction(&drain, &[Check::success()]);
        assert_eq!(
            context.account_store.borrow().get(&admin).unwrap().lamports,
            before + token_rent + 12345
        );
    }
}

#[test]
fn preview_owned_account_cleanup_is_idempotent_and_skips_foreign_owners() {
    for (program, artifact) in programs() {
        let admin = Pubkey::new_unique();
        let owned = Pubkey::new_unique();
        let foreign = Pubkey::new_unique();
        let program_data = bpf_loader_upgradeable::get_program_data_address(&program);
        let context = zama_solana_test_kit::svm(&program, artifact).with_context(HashMap::from([
            (admin, funded_system_account()),
            (program_data, program_data_account(Some(admin)).1),
            (
                owned,
                Account {
                    owner: program,
                    lamports: 1000,
                    data: vec![1; 20],
                    ..system_account(0)
                },
            ),
            (
                foreign,
                Account {
                    lamports: 123,
                    ..system_account(0)
                },
            ),
        ]));
        let ix = Instruction {
            program_id: program,
            accounts: vec![
                AccountMeta::new(admin, true),
                AccountMeta::new_readonly(program_data, false),
                AccountMeta::new(owned, false),
                AccountMeta::new(foreign, false),
                AccountMeta::new(owned, false),
            ],
            data: vec![36, 68, 214, 114, 46, 227, 146, 228],
        };
        context.process_and_validate_instruction(&ix, &[Check::success()]);
        context.process_and_validate_instruction(&ix, &[Check::success()]);
        assert_eq!(
            context
                .account_store
                .borrow()
                .get(&owned)
                .map_or(0, |a| a.lamports),
            0
        );
        assert_eq!(
            context
                .account_store
                .borrow()
                .get(&foreign)
                .unwrap()
                .lamports,
            123
        );
    }
}

#[test]
fn default_builds_have_no_preview_administrative_entrypoint() {
    for (program, artifact) in programs() {
        let admin = Pubkey::new_unique();
        let program_data = bpf_loader_upgradeable::get_program_data_address(&program);
        let context =
            zama_solana_test_kit::svm(&program, artifact.trim_end_matches("_admin_sweep"))
                .with_context(HashMap::from([
                    (admin, funded_system_account()),
                    (program_data, program_data_account(Some(admin)).1),
                ]));
        for discriminator in [
            vec![36, 68, 214, 114, 46, 227, 146, 228],
            vec![64, 227, 93, 185, 105, 167, 223, 84],
            vec![229, 176, 14, 73, 64, 247, 65, 76],
        ] {
            let ix = Instruction {
                program_id: program,
                accounts: vec![
                    AccountMeta::new(admin, true),
                    AccountMeta::new_readonly(program_data, false),
                ],
                data: discriminator,
            };
            // Anchor 101: InstructionFallbackNotFound, rather than an account-validation error.
            context.process_and_validate_instruction(
                &ix,
                &[zama_solana_test_kit::anchor_framework_error_check(
                    anchor_lang::error::ErrorCode::InstructionFallbackNotFound,
                )],
            );
        }
    }
}
