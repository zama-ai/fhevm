use anchor_lang::{
    prelude::{system_instruction, system_program},
    AccountDeserialize,
};
use anchor_litesvm::TestHelpers;
use litesvm::LiteSVM;
use solana_sdk::program_pack::Pack;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

use crate::{
    acl::{
        balance_acl_record_address, confidential_mint_address, event_authority,
        rand_counter_address, read_acl_record, token_account_address, vault_authority_address,
    },
    programs::{host_program_so_path, svm_with_programs, token_program_so_path},
    transaction::{anchor_ix, send, send_many_with_signers, send_with_signers},
};

use confidential_token as token;
use zama_host as host;

pub struct TokenFixture {
    pub svm: LiteSVM,
    pub host_program_id: Pubkey,
    pub token_program_id: Pubkey,
    pub alice: Keypair,
    pub bob: Keypair,
    pub mint: Pubkey,
    pub underlying_mint: Keypair,
    pub compute_signer: Pubkey,
    pub alice_token: Pubkey,
    pub bob_token: Pubkey,
    pub alice_usdc: Pubkey,
    pub vault_usdc: Pubkey,
    pub alice_initial: [u8; 32],
    pub bob_initial: [u8; 32],
    pub alice_current_compute_acl: Pubkey,
    pub bob_current_compute_acl: Pubkey,
}

#[derive(Clone, Copy, Debug)]
pub struct TransferOutputAccounts {
    pub alice: Pubkey,
    pub bob: Pubkey,
}

#[derive(Clone, Copy, Debug)]
pub struct WrapOutputAccounts {
    pub balance: Pubkey,
}

pub fn token_fixture() -> TokenFixture {
    let host_program_id = host::id();
    let token_program_id = token::id();
    let mut svm = svm_with_programs(&[
        (host_program_id, host_program_so_path()),
        (token_program_id, token_program_so_path()),
    ]);

    let alice = svm.create_funded_account(2_000_000_000).unwrap();
    let bob = svm.create_funded_account(1_000_000_000).unwrap();
    let underlying_mint = svm.create_token_mint(&alice, 6).unwrap();
    // The confidential mint is a PDA derived from the underlying SPL mint.
    let mint = confidential_mint_address(token_program_id, underlying_mint.pubkey());

    let vault_authority = vault_authority_address(token_program_id, mint);
    let alice_usdc = svm
        .create_token_account(&underlying_mint.pubkey(), &alice)
        .unwrap();
    let vault_usdc = Keypair::new();
    create_spl_token_account(
        &mut svm,
        &alice,
        &vault_usdc,
        underlying_mint.pubkey(),
        vault_authority,
    );
    svm.mint_to(
        &underlying_mint.pubkey(),
        &alice_usdc.pubkey(),
        &alice,
        1_000_000_000,
    )
    .unwrap();

    send_with_signers(
        &mut svm,
        &alice.pubkey(),
        anchor_ix(
            token_program_id,
            token::accounts::InitializeMint {
                authority: alice.pubkey(),
                mint,
                underlying_mint: underlying_mint.pubkey(),
                system_program: system_program::ID,
            },
            token::instruction::InitializeMint {},
        ),
        &[&alice],
    )
    .unwrap();

    let compute_signer = token::compute_signer_address(mint).0;
    let alice_token = token_account_address(token_program_id, mint, alice.pubkey());
    let bob_token = token_account_address(token_program_id, mint, bob.pubkey());
    let alice_current_compute_acl =
        balance_acl_record_address(host_program_id, mint, alice_token, 0);
    let bob_current_compute_acl =
        balance_acl_record_address(host_program_id, mint, bob_token, 0);

    initialize_confidential_token_account(
        &mut svm,
        token_program_id,
        host_program_id,
        &alice,
        mint,
        underlying_mint.pubkey(),
        alice_token,
        compute_signer,
        alice_current_compute_acl,
        125,
    );
    initialize_confidential_token_account(
        &mut svm,
        token_program_id,
        host_program_id,
        &bob,
        mint,
        underlying_mint.pubkey(),
        bob_token,
        compute_signer,
        bob_current_compute_acl,
        20,
    );
    let alice_initial = read_acl_record(&svm, alice_current_compute_acl)
        .expect("expected Alice initial ACL")
        .handle;
    let bob_initial = read_acl_record(&svm, bob_current_compute_acl)
        .expect("expected Bob initial ACL")
        .handle;

    TokenFixture {
        svm,
        host_program_id,
        token_program_id,
        alice,
        bob,
        mint,
        underlying_mint,
        compute_signer,
        alice_token,
        bob_token,
        alice_usdc: alice_usdc.pubkey(),
        vault_usdc: vault_usdc.pubkey(),
        alice_initial,
        bob_initial,
        alice_current_compute_acl,
        bob_current_compute_acl,
    }
}

fn initialize_confidential_token_account(
    svm: &mut LiteSVM,
    token_program_id: Pubkey,
    host_program_id: Pubkey,
    owner: &Keypair,
    mint: Pubkey,
    underlying_mint: Pubkey,
    token_account: Pubkey,
    compute_signer: Pubkey,
    acl_record: Pubkey,
    initial_balance: u64,
) {
    send(
        svm,
        owner,
        anchor_ix(
            token_program_id,
            token::accounts::InitializeTokenAccount {
                owner: owner.pubkey(),
                mint,
                underlying_mint,
                compute_signer,
                token_account,
                acl_record,
                zama_rand_counter: rand_counter_address(host_program_id),
                zama_event_authority: event_authority(host_program_id),
                zama_program: host_program_id,
                system_program: system_program::ID,
                event_authority: event_authority(token_program_id),
                program: token_program_id,
            },
            token::instruction::InitializeTokenAccount { initial_balance },
        ),
    );
}

pub fn create_spl_token_account(
    svm: &mut LiteSVM,
    payer: &Keypair,
    token_account: &Keypair,
    mint: Pubkey,
    owner: Pubkey,
) {
    let rent =
        svm.minimum_balance_for_rent_exemption(anchor_spl::token::spl_token::state::Account::LEN);
    send_many_with_signers(
        svm,
        &payer.pubkey(),
        vec![
            system_instruction::create_account(
                &payer.pubkey(),
                &token_account.pubkey(),
                rent,
                anchor_spl::token::spl_token::state::Account::LEN as u64,
                &anchor_spl::token::spl_token::id(),
            ),
            anchor_spl::token::spl_token::instruction::initialize_account3(
                &anchor_spl::token::spl_token::id(),
                &token_account.pubkey(),
                &mint,
                &owner,
            )
            .unwrap(),
        ],
        &[payer, token_account],
    )
    .unwrap();
}

pub fn spl_token_amount(svm: &LiteSVM, address: Pubkey) -> u64 {
    let account = svm
        .get_account(&address)
        .expect("expected SPL token account");
    anchor_spl::token::spl_token::state::Account::unpack(&account.data)
        .unwrap()
        .amount
}

pub fn token_account(svm: &LiteSVM, address: Pubkey) -> token::ConfidentialTokenAccount {
    let account = svm
        .get_account(&address)
        .expect("expected confidential token account");
    let mut data = account.data.as_slice();
    token::ConfidentialTokenAccount::try_deserialize(&mut data).unwrap()
}
