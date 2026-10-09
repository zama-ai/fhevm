//! Packet-size envelope for the KMS-certificate consumers `disclose_secp` and
//! `redeem_burned_amount` as the KMS threshold grows.
//!
//! Kept out of `token_mollusk.rs` so wire-size measurements do not grow the behavioral suite.
//! Signatures are placeholders; account and payload lengths match the current ABI.
//! Bincode-serialized legacy transaction length vs
//! `PACKET_DATA_SIZE` is asserted. Clients send version 1 transactions (4,096 bytes, 64 account
//! keys), so a row that fits this packet also fits the transaction clients send.

use anchor_lang::prelude::Pubkey;
use confidential_token as token;
use solana_sdk::{instruction::Instruction, message::Message, transaction::Transaction};
use zama_host as host;
use zama_solana_test_kit::{anchor_ix, canonical_test_context_id, event_authority};

/// Version-1 `extra_data`: the certificate names its KMS context explicitly.
fn public_extra_data() -> Vec<u8> {
    [&[1][..], &canonical_test_context_id(1)].concat()
}

fn legacy_tx_size(ix: Instruction, payer: Pubkey) -> usize {
    let tx = Transaction::new_unsigned(Message::new(&[ix], Some(&payer)));
    bincode::serialize(&tx)
        .expect("serialize transaction")
        .len()
}

fn disclose_secp_tx_size(sig_count: usize) -> usize {
    legacy_tx_size(
        anchor_ix(
            token::id(),
            token::accounts::DiscloseSecp {
                host_config: host::host_config_address().0,
                kms_context: host::kms_context_address(canonical_test_context_id(1)).0,
                zama_program: host::id(),
                event_authority: event_authority(token::id()),
                program: token::id(),
            },
            token::instruction::DiscloseSecp {
                handle: [0u8; 32],
                cleartext: [0u8; 32],
                signatures: vec![[0u8; 65]; sig_count],
                extra_data: public_extra_data(),
            },
        ),
        Pubkey::new_unique(),
    )
}

/// Redeem carries the same certificate as `disclose_secp` over the larger burn-redemption account
/// list (vault, destination, pending_burn, ...), so its envelope is the binding one. The owner
/// signs and pays.
fn redeem_burned_amount_tx_size(sig_count: usize) -> usize {
    let owner = Pubkey::new_unique();
    legacy_tx_size(
        anchor_ix(
            token::id(),
            token::accounts::RedeemBurnedAmount {
                owner,
                mint: Pubkey::new_unique(),
                token_account: Pubkey::new_unique(),
                underlying_mint: Pubkey::new_unique(),
                vault_usdc: Pubkey::new_unique(),
                destination_usdc: Pubkey::new_unique(),
                vault_authority: Pubkey::new_unique(),
                burned_amount_store: Pubkey::new_unique(),
                pending_burn: Pubkey::new_unique(),
                host_config: host::host_config_address().0,
                kms_context: host::kms_context_address(canonical_test_context_id(1)).0,
                zama_program: host::id(),
                token_program: Pubkey::new_unique(),
                event_authority: event_authority(token::id()),
                program: token::id(),
            },
            token::instruction::RedeemBurnedAmount {
                burned_handle: [0u8; 32],
                cleartext_amount: 0,
                signatures: vec![[0u8; 65]; sig_count],
                extra_data: public_extra_data(),
            },
        ),
        owner,
    )
}

/// Asserts each `(threshold, expected_fits)` row against the 1232-byte legacy packet limit.
fn assert_fit_table(name: &str, tx_size: fn(usize) -> usize, cases: &[(usize, bool)]) {
    let limit = solana_packet::PACKET_DATA_SIZE;
    eprintln!("{name} threshold fit table (packet limit = {limit} bytes):");
    for &(t, expected_fits) in cases {
        let size = tx_size(t);
        let fits = size <= limit;
        eprintln!("  t={t:>2} sigs -> {size:>4} bytes");
        assert_eq!(
            fits, expected_fits,
            "{name} t={t} measured {size} bytes (fits={fits}); table expected fits={expected_fits}. \
             The single-packet envelope moved: update the table."
        );
    }
}

#[test]
fn disclose_secp_threshold_fit_table() {
    assert_fit_table(
        "disclose_secp",
        disclose_secp_tx_size,
        &[(7, true), (12, true), (13, false)],
    );
}

#[test]
fn redeem_burned_amount_threshold_fit_table() {
    // The production KMS threshold is 7 of 13 signers.
    assert_fit_table(
        "redeem_burned_amount",
        redeem_burned_amount_tx_size,
        &[(7, true), (8, true), (9, false)],
    );
}
