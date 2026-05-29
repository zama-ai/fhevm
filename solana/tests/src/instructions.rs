use anchor_lang::prelude::system_program;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Signer};
use solana_sha256_hasher::hashv;

use crate::{
    acl::{
        balance_acl_record_address, event_authority, rand_acl_record_address, rand_counter_address,
    },
    fixture::{TokenFixture, TransferOutputAccounts, WrapOutputAccounts},
    transaction::anchor_ix,
};

use confidential_token as token;

pub fn poc_demo_confidential_rand_ix(
    fixture: &TokenFixture,
    nonce_sequence: u64,
) -> (Instruction, Pubkey) {
    let output_acl = rand_acl_record_address(
        fixture.host_program_id,
        fixture.mint,
        fixture.alice_token,
        nonce_sequence,
    );
    let ix = anchor_ix(
        fixture.token_program_id,
        token::accounts::PocDemoConfidentialRand {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint,
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

pub fn transfer_output_accounts(
    fixture: &TokenFixture,
    nonce_sequence: u64,
) -> TransferOutputAccounts {
    TransferOutputAccounts {
        alice: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint,
            fixture.alice_token,
            nonce_sequence,
        ),
        bob: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint,
            fixture.bob_token,
            nonce_sequence,
        ),
    }
}

pub fn wrap_output_accounts(fixture: &TokenFixture, nonce_sequence: u64) -> WrapOutputAccounts {
    WrapOutputAccounts {
        balance: balance_acl_record_address(
            fixture.host_program_id,
            fixture.mint,
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
    transfer_ix_with_amount_proof(
        fixture,
        output,
        amount_handle,
        input_proof(fixture, amount_handle),
    )
}

pub fn external_input_handle(fixture: &TokenFixture, value: u64, nonce: u64) -> [u8; 32] {
    let mut handle = hashv(&[
        b"ZK-w_hdl",
        &value.to_be_bytes(),
        fixture.alice.pubkey().as_ref(),
        fixture.alice_token.as_ref(),
        fixture.mint.as_ref(),
        &nonce.to_be_bytes(),
    ])
    .to_bytes();
    handle[21] = 0;
    handle[22..30].copy_from_slice(&zama_host::SOLANA_POC_CHAIN_ID.to_be_bytes());
    handle[30] = 5;
    handle[31] = 0;
    handle
}

pub fn input_proof(fixture: &TokenFixture, input_handle: [u8; 32]) -> [u8; 32] {
    hashv(&[
        b"zama-solana-poc-input-v0",
        input_handle.as_ref(),
        fixture.alice.pubkey().as_ref(),
        fixture.alice_token.as_ref(),
        fixture.mint.as_ref(),
        &[5],
        &zama_host::SOLANA_POC_CHAIN_ID.to_be_bytes(),
    ])
    .to_bytes()
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
            mint: fixture.mint,
            from_account: fixture.alice_token,
            to_account: fixture.alice_token,
            compute_signer: fixture.compute_signer,
            from_current_compute_acl: fixture.alice_current_compute_acl,
            to_current_compute_acl: fixture.alice_current_compute_acl,
            from_output_acl: output.alice,
            to_output_acl: output.bob,
            zama_rand_counter: rand_counter_address(fixture.host_program_id),
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::ConfidentialTransfer {
            amount_handle,
            amount_proof: input_proof(fixture, amount_handle),
        },
    )
}

pub fn transfer_ix_with_current_acl(
    fixture: &TokenFixture,
    from_current_compute_acl: Pubkey,
    to_current_compute_acl: Pubkey,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
) -> Instruction {
    transfer_ix_with_amount_proof_and_current_acl(
        fixture,
        from_current_compute_acl,
        to_current_compute_acl,
        output,
        amount_handle,
        input_proof(fixture, amount_handle),
    )
}

pub fn transfer_ix_with_amount_proof(
    fixture: &TokenFixture,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
    amount_proof: [u8; 32],
) -> Instruction {
    transfer_ix_with_amount_proof_and_current_acl(
        fixture,
        fixture.alice_current_compute_acl,
        fixture.bob_current_compute_acl,
        output,
        amount_handle,
        amount_proof,
    )
}

pub fn transfer_ix_with_amount_proof_and_current_acl(
    fixture: &TokenFixture,
    from_current_compute_acl: Pubkey,
    to_current_compute_acl: Pubkey,
    output: TransferOutputAccounts,
    amount_handle: [u8; 32],
    amount_proof: [u8; 32],
) -> Instruction {
    anchor_ix(
        fixture.token_program_id,
        token::accounts::ConfidentialTransfer {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint,
            from_account: fixture.alice_token,
            to_account: fixture.bob_token,
            compute_signer: fixture.compute_signer,
            from_current_compute_acl,
            to_current_compute_acl,
            from_output_acl: output.alice,
            to_output_acl: output.bob,
            zama_rand_counter: rand_counter_address(fixture.host_program_id),
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::ConfidentialTransfer {
            amount_handle,
            amount_proof,
        },
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
            mint: fixture.mint,
            token_account: fixture.alice_token,
            underlying_mint: fixture.underlying_mint.pubkey(),
            user_usdc: fixture.alice_usdc,
            vault_usdc: fixture.vault_usdc,
            vault_authority: vault_authority_address(
                fixture.token_program_id,
                fixture.mint,
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

pub fn request_unwrap_usdc_ix(
    fixture: &TokenFixture,
    output_acl: Pubkey,
    amount: u64,
) -> Instruction {
    use crate::acl::vault_authority_address;

    anchor_ix(
        fixture.token_program_id,
        token::accounts::RequestUnwrapUsdc {
            owner: fixture.alice.pubkey(),
            mint: fixture.mint,
            token_account: fixture.alice_token,
            underlying_mint: fixture.underlying_mint.pubkey(),
            vault_authority: vault_authority_address(
                fixture.token_program_id,
                fixture.mint,
            ),
            compute_signer: fixture.compute_signer,
            current_compute_acl: fixture.alice_current_compute_acl,
            output_acl,
            zama_rand_counter: rand_counter_address(fixture.host_program_id),
            zama_event_authority: event_authority(fixture.host_program_id),
            zama_program: fixture.host_program_id,
            token_program: anchor_spl::token::spl_token::id(),
            system_program: system_program::ID,
            event_authority: event_authority(fixture.token_program_id),
            program: fixture.token_program_id,
        },
        token::instruction::RequestUnwrapUsdc { amount },
    )
}
