use anchor_lang::prelude::*;
use litesvm::LiteSVM;
use solana_sdk::{clock::Clock, slot_hashes::SlotHashes};
use zama_host as host;

pub const DEFAULT_INPUT_NONCE_SEQUENCE: u64 = 0;

pub fn amount_plaintext(amount: u64) -> [u8; 32] {
    let mut plaintext = [0_u8; 32];
    plaintext[24..].copy_from_slice(&amount.to_be_bytes());
    plaintext
}

pub fn expected_trivial_handle(svm: &LiteSVM, amount: u64, fhe_type: u8) -> [u8; 32] {
    let clock: Clock = svm.get_sysvar();
    let previous_bank_hash = clock
        .slot
        .checked_sub(1)
        .and_then(|slot| {
            let slot_hashes: SlotHashes = svm.get_sysvar();
            slot_hashes.get(&slot).map(|hash| hash.to_bytes())
        })
        .unwrap_or([0; 32]);
    host::computed_trivial_handle(
        amount_plaintext(amount),
        fhe_type,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        clock.unix_timestamp,
    )
}

pub fn execute_frame_log_count(meta: &litesvm::types::TransactionMetadata) -> usize {
    meta.logs
        .iter()
        .filter(|log| log.contains("Instruction: ExecuteFrame"))
        .count()
}
