use anchor_lang::prelude::system_program;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Signer};

use crate::{
    acl::{
        balance_acl_record_address, event_authority, rand_acl_record_address,
        rand_counter_address, read_acl_record, transfer_amount_acl_address,
    },
    fixture::{TokenFixture, TransferOutputAccounts, WrapOutputAccounts},
    transaction::{anchor_ix, send},
    util::DEFAULT_INPUT_NONCE_SEQUENCE,
};

use confidential_token as token;

pub fn authorize_transfer_amount(
    fixture: &mut TokenFixture,
    amount: u64,
    nonce_sequence: u64,
) -> [u8; 32] {
    let output_acl = transfer_amount_acl_address(fixture, nonce_sequence);
    let ix = anchor_ix(
        fixture.token_program_id,
        token::accounts::PocAuthorizeTransferAmount {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            token_account: fixture.alice_token,
            compute_signer: fixture.compute_signer,
            output_acl,
            zama_rand_counter: rand_counter_address(fixture.host_program_id),
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::PocAuthorizeTransferAmount {
            amount,
            nonce_sequence,
        },
    );
    send(&mut fixture.svm, &fixture.alice, ix);
    read_acl_record(&fixture.svm, output_acl)
        .expect("expected transfer amount ACL")
        .handle
}

pub fn poc_demo_confidential_rand_ix(
    fixture: &TokenFixture,
    nonce_sequence: u64,
) -> (Instruction, Pubkey) {
    let output_acl = rand_acl_record_address(
        fixture.host_program_id,
        fixture.mint.pubkey(),
        fixture.alice_token,
        nonce_sequence,
    );
    let ix = anchor_ix(
        fixture.token_program_id,
        token::accounts::PocDemoConfidentialRand {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            token_account: fixture.alice_token,
            compute_signer: fixture.compute_signer,
            output_acl,
            zama_rand_counter: rand_counter_address(fixture.host_program_id),
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::PocDemoConfidentialRand { nonce_sequence },
    );
    (ix, output_acl)
}

pub fn poc_demo_confidential_rand(
    fixture: &mut TokenFixture,
    nonce_sequence: u64,
) -> Pubkey {
    let (ix, output_acl) = poc_demo_confidential_rand_ix(fixture, nonce_sequence);
    send(&mut fixture.svm, &fixture.alice, ix);
    output_acl
}

pub fn transfer_output_accounts(
    fixture: &TokenFixture,
    nonce_sequence: u64,
) -> TransferOutputAccounts {
    TransferOutputAccounts {
        alice: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.alice_token,
            nonce_sequence,
        ),
        bob: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.bob_token,
            nonce_sequence,
        ),
    }
}

pub fn wrap_output_accounts(fixture: &TokenFixture, nonce_sequence: u64) -> WrapOutputAccounts {
    WrapOutputAccounts {
        balance: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint.pubkey(),
            fixture.alice_token,
            nonce_sequence,
        ),
    }
}

pub fn transfer_ix(
    fixture: &TokenFixture,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    transfer_ix_with_amount_nonce(fixture, output, amount_handle, DEFAULT_INPUT_NONCE_SEQUENCE)
}

pub fn transfer_ix_with_amount_nonce(
    fixture: &TokenFixture,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
    amount_nonce_sequence: u64,
) -> Instruction {
    transfer_ix_with_current_acl_and_amount_nonce(
        fixture,
        fixture.alice_current_compute_acl,
        fixture.bob_current_compute_acl,
        output,
        amount_handle,
        amount_nonce_sequence,
    )
}

pub fn self_transfer_ix(
    fixture: &TokenFixture,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        fixture.token_program_id,
        token::accounts::ConfidentialTransfer {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            from_account: fixture.alice_token,
            to_account: fixture.alice_token,
            compute_signer: fixture.compute_signer,
            from_current_compute_acl: fixture.alice_current_compute_acl,
            to_current_compute_acl: fixture.alice_current_compute_acl,
            amount_compute_acl: transfer_amount_acl_address(fixture, DEFAULT_INPUT_NONCE_SEQUENCE),
            from_output_acl: output.alice,
            to_output_acl: output.bob,
            zama_rand_counter: rand_counter_address(fixture.host_program_id),
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::ConfidentialTransfer { amount_handle },
    )
}

pub fn transfer_ix_with_current_acl(
    fixture: &TokenFixture,
    from_current_compute_acl: Pubkey,
    to_current_compute_acl: Pubkey,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    transfer_ix_with_current_acl_and_amount_nonce(
        fixture,
        from_current_compute_acl,
        to_current_compute_acl,
        output,
        amount_handle,
        DEFAULT_INPUT_NONCE_SEQUENCE,
    )
}

pub fn transfer_ix_with_current_acl_and_amount_nonce(
    fixture: &TokenFixture,
    from_current_compute_acl: Pubkey,
    to_current_compute_acl: Pubkey,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
    amount_nonce_sequence: u64,
) -> Instruction {
    transfer_ix_with_amount_acl(
        fixture,
        from_current_compute_acl,
        to_current_compute_acl,
        transfer_amount_acl_address(fixture, amount_nonce_sequence),
        output,
        amount_handle,
    )
}

pub fn transfer_ix_with_amount_acl(
    fixture: &TokenFixture,
    from_current_compute_acl: Pubkey,
    to_current_compute_acl: Pubkey,
    amount_compute_acl: Pubkey,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        fixture.token_program_id,
        token::accounts::ConfidentialTransfer {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            from_account: fixture.alice_token,
            to_account: fixture.bob_token,
            compute_signer: fixture.compute_signer,
            from_current_compute_acl,
            to_current_compute_acl,
            amount_compute_acl,
            from_output_acl: output.alice,
            to_output_acl: output.bob,
            zama_rand_counter: rand_counter_address(fixture.host_program_id),
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::ConfidentialTransfer { amount_handle },
    )
}

pub fn wrap_usdc_ix(
    fixture: &TokenFixture,
    output: WrapOutputAccounts,
    amount: u64,
) -> Instruction {
    use crate::acl::vault_authority_address;

    anchor_ix(
        fixture.token_program_id,
        token::accounts::WrapUsdc {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint.pubkey(),
            token_account: fixture.alice_token,
            underlying_mint: fixture.underlying_mint.pubkey(),
            user_usdc: fixture.alice_usdc,
            vault_usdc: fixture.vault_usdc,
            vault_authority: vault_authority_address(
                fixture.token_program_id,
                fixture.mint.pubkey(),
            ),
            compute_signer: fixture.compute_signer,
            current_compute_acl: fixture.alice_current_compute_acl,
            output_acl: output.balance,
            zama_rand_counter: rand_counter_address(fixture.host_program_id),
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            token_program: anchor_spl::token::spl_token::id(),
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::WrapUsdc { amount },
    )
}
