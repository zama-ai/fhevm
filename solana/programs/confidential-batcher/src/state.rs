//! Account layouts, PDA helpers, encrypted-value labels, and the rate math.

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::BatcherError;

/// Batcher config wiring one deposit confidential mint, one confidential
/// shares mint, and one public vault together.
#[account]
#[derive(InitSpace)]
pub struct Batcher {
    /// Confidential mint users deposit through the batcher; wraps the vault's
    /// underlying mint.
    pub deposit_confidential_mint: Pubkey,
    /// Confidential mint wrapping the vault's share mint.
    pub shares_confidential_mint: Pubkey,
    /// Public `demo_vault::Vault` the batcher fronts.
    pub vault: Pubkey,
    /// Minimum slots a batch must stay open before dispatch.
    pub min_batch_age_slots: u64,
    /// Index the next `open_batch` creates.
    pub next_batch_index: u64,
}

impl Batcher {
    /// Serialized size of the account body, excluding the Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 8 + 8;
}

/// Lifecycle of a batch.
#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone, Copy, PartialEq, Eq, Debug)]
pub enum BatchStatus {
    /// Open for joins and quits.
    Pending,
    /// Total burned; awaiting the KMS certificate.
    Dispatched,
    /// Rate frozen; claims open.
    Settled,
    /// Certified total was zero; nothing to claim.
    Canceled,
}

/// One batch: its own confidential token account (so the revealed total is
/// exactly this batch's sum), lifecycle status, and the public settle results.
#[account]
#[derive(InitSpace)]
pub struct Batch {
    /// Batcher config this batch belongs to.
    pub batcher: Pubkey,
    /// Zero-based index within the batcher.
    pub index: u64,
    /// Lifecycle status.
    pub status: BatchStatus,
    /// Slot the batch opened at; dispatch requires `min_batch_age_slots` past.
    pub opened_slot: u64,
    /// Number of join calls routed into this batch.
    pub join_count: u64,
    /// Bump of the per-batch authority PDA.
    pub authority_bump: u8,
    /// PDA bump for `(batcher, index)`.
    pub bump: u8,
    /// Born-public handle of the burned batch total (set at dispatch).
    pub burned_total_handle: [u8; 32],
    /// KMS-certified batch total (set at settle; public by design).
    pub total_deposited: u64,
    /// Vault shares received for the batch total (set at settle).
    pub shares_received: u64,
    /// Frozen rate `shares_received * RATE_SCALE / total_deposited` (settle).
    pub share_rate: u64,
}

impl Batch {
    /// Serialized size of the account body, excluding the Anchor discriminator.
    pub const SPACE: usize = 32 + 8 + 1 + 8 + 8 + 1 + 1 + 32 + 8 + 8 + 8;
}

/// Per-(batch, user) deposit record. The encrypted amount itself lives in the
/// batcher-owned `EncryptedValue` lineage at `deposit_encrypted_value`
/// (audience: the user, who can decrypt their pending deposit, and the batch
/// authority, which computes refunds and claims from it).
#[account]
#[derive(InitSpace)]
pub struct DepositRecord {
    /// Batch this record belongs to.
    pub batch: Pubkey,
    /// Depositing user.
    pub user: Pubkey,
    /// `EncryptedValue` lineage holding the user's accumulated batch deposit.
    pub deposit_encrypted_value: Pubkey,
    /// Whether the user's shares were claimed.
    pub claimed: bool,
    /// PDA bump for `(batch, user)`.
    pub bump: u8,
}

impl DepositRecord {
    /// Serialized size of the account body, excluding the Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 1 + 1;
}

/// Returns the batch PDA for a batcher and index.
pub fn batch_address(batcher: Pubkey, index: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[BATCH_SEED, batcher.as_ref(), &index.to_le_bytes()],
        &crate::ID,
    )
}

/// Returns the per-batch authority PDA that owns the batch's token accounts.
pub fn batch_authority_address(batch: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[BATCH_AUTHORITY_SEED, batch.as_ref()], &crate::ID)
}

/// Returns the deposit record PDA for a batch and user.
pub fn deposit_record_address(batch: Pubkey, user: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[DEPOSIT_RECORD_SEED, batch.as_ref(), user.as_ref()],
        &crate::ID,
    )
}

/// Returns the batch's plain SPL account for redeemed underlying tokens.
pub fn batch_underlying_address(batch: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[BATCH_UNDERLYING_SEED, batch.as_ref()], &crate::ID)
}

/// Returns the batch's plain SPL account for received vault shares.
pub fn batch_share_tokens_address(batch: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[BATCH_SHARE_TOKENS_SEED, batch.as_ref()], &crate::ID)
}

/// Encrypted-value label for a user's accumulated batch deposit.
pub fn pending_deposit_label(user: Pubkey) -> [u8; 32] {
    solana_sha256_hasher::hashv(&[b"batcher-pending-deposit", user.as_ref()]).to_bytes()
}

/// Encrypted-value label for a user's claimed share amount.
pub fn claim_amount_label(user: Pubkey) -> [u8; 32] {
    solana_sha256_hasher::hashv(&[b"batcher-claim-amount", user.as_ref()]).to_bytes()
}

/// Returns the canonical `EncryptedValue` PDA for a batcher lineage. Batcher
/// lineages live in the batch's own ACL domain: `acl_domain_key = batch`,
/// `app_account = batch_authority`, per-user label.
pub fn batcher_encrypted_value_address(
    batch: Pubkey,
    batch_authority: Pubkey,
    label: [u8; 32],
) -> (Pubkey, u8) {
    zama_host::encrypted_value_address(zama_solana_acl_value_key(batch, batch_authority, label))
}

fn zama_solana_acl_value_key(
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    label: [u8; 32],
) -> [u8; 32] {
    // Delegate to the shared derivation through zama-fhe's DurableSlot so the
    // batcher and host agree exactly.
    zama_fhe::DurableSlot::new(
        acl_domain_key,
        app_account,
        zama_fhe::DurableLabel::new(label),
    )
    .value_key()
}

/// Freezes a settled batch's public share rate, rounded DOWN so the sum of all
/// claims can never exceed the wrapped shares:
/// `rate = shares_received * RATE_SCALE / total_deposited`.
///
/// The one plaintext division of the whole flow. `total_deposited` must be
/// non-zero (zero-total batches cancel instead). The u64 fit check fails
/// closed if the vault's share price ever left the supported domain.
pub fn freeze_share_rate(shares_received: u64, total_deposited: u64) -> Result<u64> {
    require!(total_deposited > 0, BatcherError::InvalidFheEvalPlan);
    let rate = (shares_received as u128) * (RATE_SCALE as u128) / (total_deposited as u128);
    u64::try_from(rate).map_err(|_| BatcherError::ShareRateOverflow.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The cleartext mirror of the claim MulDiv: `deposit * rate / RATE_SCALE`.
    fn claim_shares(deposit: u64, rate: u64) -> u64 {
        ((deposit as u128) * (rate as u128) / (RATE_SCALE as u128)) as u64
    }

    #[test]
    fn manual_space_matches_derived_init_space() {
        assert_eq!(Batcher::SPACE, Batcher::INIT_SPACE);
        assert_eq!(Batch::SPACE, Batch::INIT_SPACE);
        assert_eq!(DepositRecord::SPACE, DepositRecord::INIT_SPACE);
    }

    #[test]
    fn one_to_one_price_rate_is_the_scale() {
        assert_eq!(freeze_share_rate(1_000, 1_000).unwrap(), RATE_SCALE);
    }

    #[test]
    fn rate_rounds_down() {
        // 2 shares for 3 deposited: rate = 2 * 1e9 / 3 = 666_666_666 (floor).
        assert_eq!(freeze_share_rate(2, 3).unwrap(), 666_666_666);
    }

    #[test]
    fn claims_never_exceed_received_shares() {
        // Adversarially rounded batch: floor(rate) then floor(claims) must
        // never over-distribute what was wrapped.
        let cases: &[(&[u64], u64)] = &[
            (&[300, 500], 799),
            (&[1, 1, 1], 2),
            (&[7, 11, 13], 17),
            (&[u64::MAX / 2, u64::MAX / 2 - 100], u64::MAX / 3),
        ];
        for (deposits, shares) in cases {
            let total: u64 = deposits
                .iter()
                .copied()
                .fold(0, |acc, d| acc.checked_add(d).unwrap());
            let rate = freeze_share_rate(*shares, total).unwrap();
            let distributed: u128 = deposits
                .iter()
                .map(|deposit| claim_shares(*deposit, rate) as u128)
                .sum();
            assert!(
                distributed <= *shares as u128,
                "distributed {distributed} > shares {shares} for deposits {deposits:?}"
            );
        }
    }

    #[test]
    fn zero_total_rate_is_rejected() {
        assert!(freeze_share_rate(10, 0).is_err());
    }

    #[test]
    fn labels_are_distinct_per_user_and_per_purpose() {
        let alice = Pubkey::new_unique();
        let bob = Pubkey::new_unique();
        assert_ne!(pending_deposit_label(alice), pending_deposit_label(bob));
        assert_ne!(pending_deposit_label(alice), claim_amount_label(alice));
    }
}
