//! On-chain account data for the per-user permit invalidation watermark.

use super::*;

/// One per user: the moment of that user's most recent permit revocation.
///
/// A permit whose validity window starts before this moment is dead, permanently.
/// There is no list to walk and nothing to enumerate, which is what keeps revocation
/// one transaction of constant work.
///
/// The moment also bounds what revocation reaches: a permit pre-signed with a start
/// still in the future opens later than this watermark and survives it. Request-time
/// checks refuse a permit before its window opens, but once it does, an earlier
/// revocation does not kill it. The EVM side accepts the same limitation deliberately,
/// so a reader of this account must not assume a stronger guarantee than EVM gives.
///
/// A missing account reads as watermark zero. There is no migration and no
/// initialization step: a user who has never revoked anything simply has no account.
#[account]
pub struct PermitInvalidation {
    /// The user whose permits this watermark governs. Stored although it is implied by
    /// the address, so a decoded account can be checked against the key it came from.
    pub user: Pubkey,
    /// Unix seconds of the last revocation. Never decreases.
    pub invalidation_watermark: u64,
    /// PDA bump for this account.
    pub bump: u8,
}

impl PermitInvalidation {
    /// Serialized size of the account body, excluding the Anchor discriminator.
    pub const SPACE: usize = 32 + 8 + 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::Discriminator;
    use solana_sha256_hasher::hash;

    /// The eight bytes an off-chain reader has to look for, pinned as literals *and* as the
    /// preimage they are derived from.
    ///
    /// This account is taken as an unchecked account by the only instruction that touches
    /// it, so the framework never registers its layout anywhere a consumer can read. The
    /// consumer that will decode it does so without the framework, computing these bytes
    /// itself — which is why asserting them against the framework's own derivation would
    /// prove nothing: both sides would move together. The literal is what a foreign
    /// implementation can be compared against, and the preimage says where it comes from,
    /// so renaming the account or changing the derivation fails here rather than in the
    /// component that reads the account by hand.
    #[test]
    fn discriminator_is_the_hash_of_the_account_name() {
        assert_eq!(
            PermitInvalidation::DISCRIMINATOR,
            [0xec, 0x8b, 0xdb, 0xa9, 0xb9, 0x22, 0xe9, 0x88],
            "the discriminator an off-chain reader looks for changed"
        );
        assert_eq!(
            PermitInvalidation::DISCRIMINATOR,
            &hash(b"account:PermitInvalidation").to_bytes()[..8],
            "the discriminator is no longer the hash of the account name"
        );
    }

    /// The declared space is exactly what the record serializes into, so an account created
    /// at this size holds the whole record and nothing beyond it. A field added without
    /// adjusting `SPACE` would otherwise be silently truncated on write.
    #[test]
    fn declared_space_matches_the_serialized_body() {
        let record = PermitInvalidation {
            user: Pubkey::new_unique(),
            invalidation_watermark: u64::MAX,
            bump: 255,
        };

        let mut serialized = Vec::new();
        record.try_serialize(&mut serialized).expect("serializes");

        assert_eq!(serialized.len(), 8 + PermitInvalidation::SPACE);
    }
}
