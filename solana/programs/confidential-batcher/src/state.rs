//! Account layouts, PDA helpers, encrypted-value labels, and the payout math.

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::BatcherError;

/// Which way through the vault a batcher instance moves value. Direction lives
/// on the `Batcher` config — one instance per direction, mirroring the EVM's
/// two batcher deployments — so a pending deposit batch never blocks a redeem
/// batch, and every batch inherits its batcher's direction.
#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone, Copy, PartialEq, Eq, Debug)]
pub enum BatchDirection {
    /// Users join with confidential underlying; claims pay confidential shares.
    Deposit,
    /// Users join with confidential shares; claims pay confidential underlying.
    Redeem,
}

/// Batcher config wiring a join confidential mint, a payout confidential mint,
/// and one public vault together for one direction.
#[account]
#[derive(InitSpace)]
pub struct Batcher {
    /// Direction of this batcher instance.
    pub direction: BatchDirection,
    /// Confidential mint users join batches with. Wraps the vault's underlying
    /// mint for deposit batchers, the vault's share mint for redeem batchers.
    pub join_confidential_mint: Pubkey,
    /// Confidential mint claims pay out in. Wraps the vault's share mint for
    /// deposit batchers, the vault's underlying mint for redeem batchers.
    pub payout_confidential_mint: Pubkey,
    /// Public `demo_vault::Vault` the batcher fronts.
    pub vault: Pubkey,
    /// Minimum slots a batch must stay open before dispatch.
    pub min_batch_age_slots: u64,
    /// Index the next `open_batch` creates.
    pub next_batch_index: u64,
}

impl Batcher {
    /// Serialized size of the account body, excluding the Anchor discriminator.
    pub const SPACE: usize = 1 + 32 + 32 + 32 + 8 + 8;
}

/// Lifecycle of a batch.
#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone, Copy, PartialEq, Eq, Debug)]
pub enum BatchStatus {
    /// Open for joins and quits.
    Pending,
    /// Total burned; awaiting the KMS certificate.
    Dispatched,
    /// Payout received; claims open.
    Settled,
    /// Certified total was zero; nothing to claim.
    Canceled,
}

/// One batch: its own confidential token account (so the revealed total is
/// exactly this batch's sum), lifecycle status, and the public settle results.
#[account]
#[derive(InitSpace)]
pub struct Batch {
    /// Batcher config this batch belongs to (carries the direction).
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
    /// KMS-certified batch join total (set at settle; public by design).
    pub total_joined: u64,
    /// Payout units the vault leg produced for the batch total (set at settle).
    pub payout_received: u64,
    /// Informational rate `payout_received * RATE_SCALE / total_joined`,
    /// saturating at u64::MAX (settle). Claims use exact proportional division
    /// instead — never this field.
    pub payout_rate: u64,
}

impl Batch {
    /// Serialized size of the account body, excluding the Anchor discriminator.
    pub const SPACE: usize = 32 + 8 + 1 + 8 + 8 + 1 + 1 + 32 + 8 + 8 + 8;
}

/// Per-(batch, user) join record. The encrypted amount itself lives in the
/// batcher-owned `EncryptedValue` lineage at `joined_encrypted_value`
/// (audience: the user, who can decrypt their pending amount, and the batch
/// authority, which computes refunds and claims from it).
#[account]
#[derive(InitSpace)]
pub struct JoinRecord {
    /// Batch this record belongs to.
    pub batch: Pubkey,
    /// Joining user.
    pub user: Pubkey,
    /// `EncryptedValue` lineage holding the user's accumulated joined amount.
    pub joined_encrypted_value: Pubkey,
    /// Whether the user's payout was claimed.
    pub claimed: bool,
    /// PDA bump for `(batch, user)`.
    pub bump: u8,
}

impl JoinRecord {
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

/// Returns the join record PDA for a batch and user.
pub fn join_record_address(batch: Pubkey, user: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[JOIN_RECORD_SEED, batch.as_ref(), user.as_ref()],
        &crate::ID,
    )
}

/// Returns the batch's plain SPL account in the join mint's underlying.
pub fn batch_join_underlying_address(batch: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[BATCH_JOIN_UNDERLYING_SEED, batch.as_ref()], &crate::ID)
}

/// Returns the batch's plain SPL account in the payout mint's underlying.
pub fn batch_payout_underlying_address(batch: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[BATCH_PAYOUT_UNDERLYING_SEED, batch.as_ref()], &crate::ID)
}

/// Encrypted-value label for a user's accumulated joined batch amount.
pub fn pending_join_label(user: Pubkey) -> [u8; 32] {
    solana_sha256_hasher::hashv(&[b"batcher-pending-join", user.as_ref()]).to_bytes()
}

/// Encrypted-value label for a user's claimed payout amount.
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

/// Computes the informational payout rate of a settled batch, rounded DOWN
/// and saturating at u64::MAX:
/// `rate = payout_received * RATE_SCALE / total_joined`.
///
/// Event-facing only — claims never multiply by it (they use exact
/// proportional division), so a redeem batch whose per-unit payout exceeds
/// the u64 rate domain (extreme share price) must not fail settle over a
/// display number. `total_joined` must be non-zero (zero-total batches cancel
/// instead).
pub fn payout_rate(payout_received: u64, total_joined: u64) -> Result<u64> {
    require!(total_joined > 0, BatcherError::InvalidFheEvalPlan);
    let rate = (payout_received as u128) * (RATE_SCALE as u128) / (total_joined as u128);
    Ok(u64::try_from(rate).unwrap_or(u64::MAX))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The cleartext mirror of the claim MulDiv: the exact proportional floor
    /// `joined * payout_received / total_joined`.
    fn claim_payout(joined: u64, payout_received: u64, total_joined: u64) -> u64 {
        ((joined as u128) * (payout_received as u128) / (total_joined as u128)) as u64
    }

    /// The superseded double-rounding claim math (`joined * rate / RATE_SCALE`
    /// on a floored rate), kept only to prove the exact division strands less.
    fn claim_payout_via_rate(joined: u64, rate: u64) -> u64 {
        ((joined as u128) * (rate as u128) / (RATE_SCALE as u128)) as u64
    }

    #[test]
    fn manual_space_matches_derived_init_space() {
        assert_eq!(Batcher::SPACE, Batcher::INIT_SPACE);
        assert_eq!(Batch::SPACE, Batch::INIT_SPACE);
        assert_eq!(JoinRecord::SPACE, JoinRecord::INIT_SPACE);
    }

    #[test]
    fn one_to_one_payout_rate_is_the_scale() {
        assert_eq!(payout_rate(1_000, 1_000).unwrap(), RATE_SCALE);
    }

    #[test]
    fn rate_rounds_down() {
        // 2 payout units for 3 joined: rate = 2 * 1e9 / 3 = 666_666_666 (floor).
        assert_eq!(payout_rate(2, 3).unwrap(), 666_666_666);
    }

    #[test]
    fn rate_saturates_instead_of_failing_settle() {
        // A redeem batch of 1 share paying out u64::MAX underlying: the true
        // rate is above u64; the informational field saturates.
        assert_eq!(payout_rate(u64::MAX, 1).unwrap(), u64::MAX);
    }

    #[test]
    fn claims_never_exceed_received_payout() {
        // Adversarially rounded batches, including the u64-scale case where
        // the old rate-based double rounding stranded ~6.1e9 raw units:
        // sum(floor(joined_i * payout / total)) <= floor(payout) always.
        let cases: &[(&[u64], u64)] = &[
            (&[300, 500], 799),
            (&[1, 1, 1], 2),
            (&[7, 11, 13], 17),
            (&[u64::MAX / 2, u64::MAX / 2 - 100], u64::MAX / 3),
            (&[u64::MAX / 3, u64::MAX / 3, u64::MAX / 3], u64::MAX - 5),
        ];
        for (joins, payout) in cases {
            let total: u64 = joins
                .iter()
                .copied()
                .fold(0, |acc, j| acc.checked_add(j).unwrap());
            let distributed: u128 = joins
                .iter()
                .map(|joined| claim_payout(*joined, *payout, total) as u128)
                .sum();
            assert!(
                distributed <= *payout as u128,
                "distributed {distributed} > payout {payout} for joins {joins:?}"
            );
        }
    }

    #[test]
    fn exact_division_strands_less_than_the_rate_would() {
        // The fhevm-internal#1774 case: at u64 scale the floored rate loses
        // precision that every claim then re-multiplies. Exact proportional
        // division recovers those units.
        let joins = [u64::MAX / 2, u64::MAX / 2 - 100];
        let payout = u64::MAX / 3;
        let total: u64 = joins.iter().sum();
        let rate = payout_rate(payout, total).unwrap();
        let via_rate: u128 = joins
            .iter()
            .map(|j| claim_payout_via_rate(*j, rate) as u128)
            .sum();
        let exact: u128 = joins
            .iter()
            .map(|j| claim_payout(*j, payout, total) as u128)
            .sum();
        let stranded_via_rate = payout as u128 - via_rate;
        let stranded_exact = payout as u128 - exact;
        // The historical measurement: 6_148_914_726 raw units stranded by the
        // double rounding; exact division strands at most one unit per claim.
        assert_eq!(stranded_via_rate, 6_148_914_726);
        assert!(stranded_exact <= joins.len() as u128);
        assert!(stranded_exact < stranded_via_rate);
    }

    #[test]
    fn zero_total_rate_is_rejected() {
        assert!(payout_rate(10, 0).is_err());
    }

    #[test]
    fn labels_are_distinct_per_user_and_per_purpose() {
        let alice = Pubkey::new_unique();
        let bob = Pubkey::new_unique();
        assert_ne!(pending_join_label(alice), pending_join_label(bob));
        assert_ne!(pending_join_label(alice), claim_amount_label(alice));
    }
}
