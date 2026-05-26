//! Shared Solana PoC scenarios (LiteSVM txs) reused across semantic backends.

use litesvm::types::TransactionMetadata;
use solana_sdk::{pubkey::Pubkey, signature::Signature};

use crate::{
    acl::read_acl_record,
    events::{fhe_rand_events, trivial_encrypt_events},
    fixture::{TokenFixture, TransferOutputAccounts, WrapOutputAccounts},
    instructions::{
        external_input_handle, poc_demo_confidential_rand_ix, transfer_ix,
        transfer_output_accounts, wrap_output_accounts, wrap_usdc_ix,
    },
    transaction::{send_with_meta, send_with_meta_and_signature},
    util::DEFAULT_INPUT_NONCE_SEQUENCE,
};

/// FHE type used for confidential token balances in the PoC program.
pub const BALANCE_FHE_TYPE: u8 = 5;

#[derive(Clone, Copy, Debug)]
pub struct TransferSetup {
    pub amount: u64,
    pub output_nonce_sequence: u64,
    pub input_nonce_sequence: u64,
}

impl Default for TransferSetup {
    fn default() -> Self {
        Self {
            amount: 9,
            output_nonce_sequence: 1,
            input_nonce_sequence: DEFAULT_INPUT_NONCE_SEQUENCE,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TransferScenario {
    pub meta: TransactionMetadata,
    pub account_keys: Vec<Pubkey>,
    pub signature: Signature,
    pub host_program_id: Pubkey,
    pub amount_handle: [u8; 32],
    pub alice_before: [u8; 32],
    pub bob_before: [u8; 32],
    pub output: TransferOutputAccounts,
    pub new_alice_handle: [u8; 32],
    pub new_bob_handle: [u8; 32],
}

#[derive(Clone, Debug)]
pub struct RandDemoScenario {
    pub meta: TransactionMetadata,
    pub account_keys: Vec<Pubkey>,
    pub host_program_id: Pubkey,
    pub acl_record: Pubkey,
    pub rand_handle: [u8; 32],
    pub rand_seed: [u8; 16],
}

/// Run `poc_demo_confidential_rand` and return tx metadata + rand output handles.
pub fn run_rand_demo_scenario(fixture: &mut TokenFixture, nonce_sequence: u64) -> RandDemoScenario {
    let (ix, acl_record) = poc_demo_confidential_rand_ix(fixture, nonce_sequence);
    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, ix);
    let rand_handle = read_acl_record(&fixture.svm, acl_record)
        .expect("expected rand ACL")
        .handle;
    let rand_seed = fhe_rand_events(&meta, &account_keys, fixture.host_program_id)
        .into_iter()
        .next()
        .expect("expected FheRandEvent")
        .seed;

    RandDemoScenario {
        meta,
        account_keys,
        host_program_id: fixture.host_program_id,
        acl_record,
        rand_handle,
        rand_seed,
    }
}

/// Run `confidential_transfer` with a local external input handle.
pub fn run_transfer_scenario(fixture: &mut TokenFixture, setup: TransferSetup) -> TransferScenario {
    let alice_before = fixture.alice_initial;
    let bob_before = fixture.bob_initial;
    let amount_handle = external_input_handle(fixture, setup.amount, setup.input_nonce_sequence);
    let output = transfer_output_accounts(fixture, setup.output_nonce_sequence);
    let transfer_ix = transfer_ix(fixture, output, amount_handle);
    let (meta, account_keys, signature) =
        send_with_meta_and_signature(&mut fixture.svm, &fixture.alice, transfer_ix);
    let new_alice_handle = read_acl_record(&fixture.svm, output.alice)
        .expect("expected Alice output ACL")
        .handle;
    let new_bob_handle = read_acl_record(&fixture.svm, output.bob)
        .expect("expected Bob output ACL")
        .handle;

    TransferScenario {
        meta,
        account_keys,
        signature,
        host_program_id: fixture.host_program_id,
        amount_handle,
        alice_before,
        bob_before,
        output,
        new_alice_handle,
        new_bob_handle,
    }
}

/// Same transfer without capturing a signature (legacy tests that only need metadata).
pub fn run_transfer_scenario_meta(
    fixture: &mut TokenFixture,
    setup: TransferSetup,
) -> (TransferScenario, bool) {
    let alice_before = fixture.alice_initial;
    let bob_before = fixture.bob_initial;
    let amount_handle = external_input_handle(fixture, setup.amount, setup.input_nonce_sequence);
    let output = transfer_output_accounts(fixture, setup.output_nonce_sequence);
    let transfer_ix = transfer_ix(fixture, output, amount_handle);
    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, transfer_ix);
    let new_alice_handle = read_acl_record(&fixture.svm, output.alice)
        .expect("expected Alice output ACL")
        .handle;
    let new_bob_handle = read_acl_record(&fixture.svm, output.bob)
        .expect("expected Bob output ACL")
        .handle;

    (
        TransferScenario {
            meta,
            account_keys,
            signature: Signature::default(),
            host_program_id: fixture.host_program_id,
            amount_handle,
            alice_before,
            bob_before,
            output,
            new_alice_handle,
            new_bob_handle,
        },
        false,
    )
}

#[derive(Clone, Copy, Debug)]
pub struct WrapSetup {
    pub amount: u64,
    pub output_nonce_sequence: u64,
}

impl Default for WrapSetup {
    fn default() -> Self {
        Self {
            amount: 50_000_000,
            output_nonce_sequence: 1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WrapScenario {
    pub meta: TransactionMetadata,
    pub account_keys: Vec<Pubkey>,
    pub host_program_id: Pubkey,
    pub alice_before: [u8; 32],
    pub output: WrapOutputAccounts,
    pub amount_handle: [u8; 32],
    pub new_alice_handle: [u8; 32],
}

pub fn run_wrap_scenario(fixture: &mut TokenFixture, setup: WrapSetup) -> WrapScenario {
    let alice_before = fixture.alice_initial;
    let output = wrap_output_accounts(fixture, setup.output_nonce_sequence);
    let ix = wrap_usdc_ix(fixture, output, setup.amount);
    let (meta, account_keys) = send_with_meta(&mut fixture.svm, &fixture.alice, ix);
    let new_alice_handle = read_acl_record(&fixture.svm, output.balance)
        .expect("expected wrap output ACL")
        .handle;
    let amount_handle = trivial_encrypt_events(&meta, &account_keys, fixture.host_program_id)
        .into_iter()
        .next()
        .expect("expected wrap trivial encrypt event")
        .result;

    WrapScenario {
        meta,
        account_keys,
        host_program_id: fixture.host_program_id,
        alice_before,
        output,
        amount_handle,
        new_alice_handle,
    }
}
