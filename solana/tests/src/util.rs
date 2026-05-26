use anchor_lang::prelude::*;
use litesvm::LiteSVM;
use solana_program::sysvar::slot_hashes;
use solana_sdk::{account::Account, clock::Clock, hash::Hash, slot_hashes::SlotHashes};
use zama_host as host;

/// On-chain `PodSlotHashes::fetch()` reads the full sysvar buffer (see `solana-sysvar`).
const SLOT_HASHES_SYSVAR_LEN: usize = 20_488;

pub const DEFAULT_INPUT_NONCE_SEQUENCE: u64 = 0;

/// SHA-256(b"zama-solana-test-bank-hash-v1") — non-zero previous bank hash for all LiteSVM tests.
pub const DEFAULT_TEST_PREVIOUS_BANK_HASH: [u8; 32] = [
    0x5f, 0x2a, 0x1f, 0x93, 0x75, 0x45, 0x32, 0x21, 0xc6, 0x88, 0x8d, 0x6d, 0x9b, 0x3b, 0x15, 0xfc,
    0xeb, 0x69, 0xee, 0x0b, 0xfb, 0x09, 0x98, 0xba, 0x81, 0x59, 0x85, 0x2b, 0x9a, 0x15, 0x9e, 0x2b,
];

pub fn amount_plaintext(amount: u64) -> [u8; 32] {
    let mut plaintext = [0_u8; 32];
    plaintext[24..].copy_from_slice(&amount.to_be_bytes());
    plaintext
}

pub fn expected_trivial_handle(svm: &LiteSVM, amount: u64, fhe_type: u8) -> [u8; 32] {
    let clock: Clock = svm.get_sysvar();
    let previous_bank_hash = previous_bank_hash_from_sysvar(svm, clock.slot);
    host::computed_trivial_handle(
        amount_plaintext(amount),
        fhe_type,
        host::SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        clock.unix_timestamp,
    )
}

pub fn previous_bank_hash_from_sysvar(svm: &LiteSVM, current_slot: u64) -> [u8; 32] {
    current_slot
        .checked_sub(1)
        .and_then(|slot| {
            let slot_hashes: SlotHashes = svm.get_sysvar();
            slot_hashes.get(&slot).map(|hash| hash.to_bytes())
        })
        .unwrap_or(DEFAULT_TEST_PREVIOUS_BANK_HASH)
}

/// Seed slot N-1 with `hash` so host handle derivation uses the real sysvar path.
pub fn set_previous_slot_hash(svm: &mut LiteSVM, hash: [u8; 32]) {
    let mut clock: Clock = svm.get_sysvar();
    if clock.slot == 0 {
        clock.slot = 2;
        svm.set_sysvar(&clock);
        clock = svm.get_sysvar();
    }
    let previous_slot = clock.slot.saturating_sub(1);
    let slot_hashes = SlotHashes::new(&[(previous_slot, Hash::new_from_array(hash))]);
    let mut data = vec![0u8; SLOT_HASHES_SYSVAR_LEN];
    bincode::serialize_into(&mut data[..], &slot_hashes).unwrap();
    svm.set_account(
        slot_hashes::id(),
        Account {
            lamports: 1,
            data,
            owner: solana_sdk::sysvar::id(),
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
}

pub fn execute_frame_log_count(meta: &litesvm::types::TransactionMetadata) -> usize {
    meta.logs
        .iter()
        .filter(|log| log.contains("Instruction: ExecuteFrame"))
        .count()
}
