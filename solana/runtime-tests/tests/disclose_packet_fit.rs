//! Packet-size envelope for `disclose_secp` under high KMS threshold × deep MMR proofs.
//!
//! Kept out of `token_mollusk.rs` so wire-size measurements do not grow the behavioral suite.
//! Validity is irrelevant here: only bincode-serialized legacy transaction length vs
//! `PACKET_DATA_SIZE` is asserted.

use anchor_lang::{prelude::Pubkey, InstructionData, ToAccountMetas};
use confidential_token as token;
use solana_sdk::{instruction::Instruction, message::Message, transaction::Transaction};
use zama_host as host;

fn anchor_ix<A, D>(program_id: Pubkey, accounts: A, args: D) -> Instruction
where
    A: ToAccountMetas,
    D: InstructionData,
{
    Instruction {
        program_id,
        accounts: accounts.to_account_metas(None),
        data: args.data(),
    }
}

fn event_authority(program_id: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"__event_authority"], &program_id).0
}

/// Builds a `disclose_secp` legacy transaction carrying `sig_count` signatures and an MMR proof of
/// `sibling_count` siblings over the real account layout, and returns its bincode-serialized wire
/// size.
fn disclose_secp_tx_size(sig_count: usize, sibling_count: usize) -> usize {
    let owner = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let encrypted_value = Pubkey::new_unique();
    let host_config = host::host_config_address().0;
    let kms_context = host::kms_context_address(1).0;
    let proof = host::instructions::MmrInclusionProof {
        leaf_index: 0,
        siblings: vec![[0u8; 32]; sibling_count],
    };
    let ix = anchor_ix(
        token::id(),
        token::accounts::DiscloseSecp {
            mint,
            encrypted_value,
            host_config,
            kms_context,
            zama_program: host::id(),
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::DiscloseSecp {
            handle: [0u8; 32],
            cleartext: [0u8; 32],
            signatures: vec![[0u8; 65]; sig_count],
            extra_data: vec![0x00u8],
            proof,
        },
    );
    let message = Message::new(&[ix], Some(&owner));
    let tx = Transaction::new_unsigned(message);
    bincode::serialize(&tx)
        .expect("serialize transaction")
        .len()
}

/// Builds a `redeem_burned_amount` legacy transaction carrying `sig_count` signatures and an MMR
/// proof of `sibling_count` siblings over the real account layout, and returns its
/// bincode-serialized wire size. Redeem carries the same cert + MMR proof payload as `disclose_secp`
/// but over the larger burn-redemption account list (vault, destination, replay marker, ...), so its
/// single-packet envelope is the binding one for the shared verifier path.
fn redeem_burned_amount_tx_size(sig_count: usize, sibling_count: usize) -> usize {
    let owner = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let host_config = host::host_config_address().0;
    let kms_context = host::kms_context_address(1).0;
    let proof = host::instructions::MmrInclusionProof {
        leaf_index: 0,
        siblings: vec![[0u8; 32]; sibling_count],
    };
    let ix = anchor_ix(
        token::id(),
        token::accounts::RedeemBurnedAmount {
            owner,
            mint,
            token_account: Pubkey::new_unique(),
            underlying_mint: Pubkey::new_unique(),
            vault_usdc: Pubkey::new_unique(),
            destination_usdc: Pubkey::new_unique(),
            vault_authority: Pubkey::new_unique(),
            burned_amount_value: Pubkey::new_unique(),
            redemption_record: Pubkey::new_unique(),
            host_config,
            kms_context,
            deny_subject_record: None,
            zama_program: host::id(),
            token_program: Pubkey::new_unique(),
            system_program: Pubkey::new_unique(),
            event_authority: event_authority(token::id()),
            program: token::id(),
        },
        token::instruction::RedeemBurnedAmount {
            burned_handle: [0u8; 32],
            cleartext_amount: 0,
            signatures: vec![[0u8; 65]; sig_count],
            extra_data: vec![0x00u8],
            proof,
        },
    );
    let message = Message::new(&[ix], Some(&owner));
    let tx = Transaction::new_unsigned(message);
    bincode::serialize(&tx)
        .expect("serialize transaction")
        .len()
}

#[test]
fn redeem_burned_amount_threshold_fit_table() {
    // Companion to `disclose_secp_threshold_fit_table` for the burn-redemption path introduced by
    // #3243, which reuses the same stateless-verifier cert + MMR proof payload. Redeem carries a
    // larger fixed account list than disclose (vault, destination, replay marker, deny-record slot),
    // so at the same (t, depth) it is ~242B larger and its single-packet envelope is strictly
    // tighter. Measured fitting corner: t=7 at depth 0 (1159B); t=7/depth-10 already overflows
    // (1479B, over by 247), and t>=9 overflows before the proof. Same qualitative boundary as
    // disclose, so the deep-encrypted value account × high-threshold redeem is the binding corner for the shared
    // verifier path and needs the #1704 two-tx fallback when it overflows.
    let cases = [
        (7usize, 0usize, true),
        (7, 10, false),
        (7, 20, false),
        (9, 10, false),
        (13, 10, false),
    ];
    let limit = solana_packet::PACKET_DATA_SIZE;
    eprintln!("redeem_burned_amount threshold fit table (packet limit = {limit} bytes):");
    for (t, depth, expected_fits) in cases {
        let size = redeem_burned_amount_tx_size(t, depth);
        let fits = size <= limit;
        eprintln!(
            "  t={t:>2} sigs, depth={depth:>2} siblings -> {size:>4} bytes  ({}{})",
            if fits { "FITS" } else { "OVER" },
            if fits {
                String::new()
            } else {
                format!(" by {}", size - limit)
            },
        );
        assert_eq!(
            fits, expected_fits,
            "t={t}, depth={depth} measured {size} bytes (fits={fits}); table expected fits={expected_fits}. \
             The single-packet envelope moved — update the table and revisit the #1704 two-tx fallback."
        );
    }
}

#[test]
fn disclose_secp_threshold_fit_table() {
    // Corner table: (threshold t, MMR proof sibling depth, expected-to-fit). Measured wire sizes for
    // the full `disclose_secp` legacy transaction against the 1232-byte packet limit.
    //
    // Re-measured against the thin consume path (fhevm-internal#1704). Dropping the DisclosureRequest
    // witness did NOT shrink the tx enough to keep t=7/depth-10 inside the packet: disclose_secp is
    // ~24B larger than the old disclose_amount_secp at the same (t, depth). Fitting corner today:
    // t=7 at depth 0 only. Deep-encrypted value account × high-threshold consumes need the #1704 two-tx fallback.
    let cases = [
        (7usize, 0usize, true),
        (7, 10, false),
        (7, 20, false),
        (9, 10, false),
        (13, 10, false),
    ];
    let limit = solana_packet::PACKET_DATA_SIZE;
    eprintln!("disclose_secp threshold fit table (packet limit = {limit} bytes):");
    for (t, depth, expected_fits) in cases {
        let size = disclose_secp_tx_size(t, depth);
        let fits = size <= limit;
        eprintln!(
            "  t={t:>2} sigs, depth={depth:>2} siblings -> {size:>4} bytes  ({}{})",
            if fits { "FITS" } else { "OVER" },
            if fits {
                String::new()
            } else {
                format!(" by {}", size - limit)
            },
        );
        assert_eq!(
            fits, expected_fits,
            "t={t}, depth={depth} measured {size} bytes (fits={fits}); table expected fits={expected_fits}. \
             The single-packet envelope moved — update the table and revisit the #1704 two-tx fallback."
        );
    }
}
