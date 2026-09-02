//! On-chain account data for `UserDecryptionDelegation`.

use super::*;

/// PoC user-decryption delegation witness.
///
/// Gateway/KMS payloads do not yet carry these records, but the account shape is
/// present so the final witness format has a concrete Solana state target.
///
/// Freshness contract for the future KMS consumer (stated here so the integration
/// doesn't have to reverse-engineer it): `delegation_counter` is a strictly monotonic
/// version — of two snapshots of the same record, the higher counter is authoritative —
/// and `last_update_slot` lets a reader require the witness be at least as fresh as a
/// slot it has observed. Writes require `last_update_slot < current slot`, so a record
/// mutates at most once per slot and every `(delegation_counter, last_update_slot)`
/// pair is unambiguous.
#[account]
pub struct UserDecryptionDelegation {
    /// User granting delegated decrypt rights.
    pub delegator: Pubkey,
    /// Delegate allowed to request user decryption.
    pub delegate: Pubkey,
    /// The encrypted value account authority the delegation is scoped over. A delegation covers
    /// every value of that authority in every domain: the domain is not one of the PDA's seeds.
    pub encrypted_value_account_authority: Pubkey,
    /// Slot after which the delegation is invalid.
    pub expiration_slot: u64,
    /// Monotonic counter incremented on every grant, regrant, and revoke.
    pub delegation_counter: u64,
    /// Slot in which this row was last updated.
    pub last_update_slot: u64,
    /// Whether the delegation has been revoked by the delegator.
    pub revoked: bool,
    /// PDA bump for this delegation account.
    pub bump: u8,
}

impl UserDecryptionDelegation {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 8 + 8 + 8 + 1 + 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::Discriminator;
    use solana_sha256_hasher::hash;

    /// The eight bytes an off-chain reader has to look for, pinned as literals *and* as the
    /// preimage they are derived from.
    ///
    /// The KMS Connector decodes this record without the framework, matching these bytes by
    /// hand — so asserting them against the framework's own derivation alone would prove
    /// nothing: both sides would move together. The literal is what a foreign implementation
    /// is compared against, and the preimage says where it comes from, so renaming the
    /// account or changing the derivation fails here rather than in the component that reads
    /// the account bytes by hand.
    #[test]
    fn discriminator_is_the_hash_of_the_account_name() {
        assert_eq!(
            UserDecryptionDelegation::DISCRIMINATOR,
            [0x25, 0x05, 0x8b, 0x21, 0x49, 0x35, 0x01, 0xf8],
            "the discriminator an off-chain reader looks for changed"
        );
        assert_eq!(
            UserDecryptionDelegation::DISCRIMINATOR,
            &hash(b"account:UserDecryptionDelegation").to_bytes()[..8],
            "the discriminator is no longer the hash of the account name"
        );
    }

    /// The declared space is exactly what the record serializes into, so an account created
    /// at this size holds the whole record and nothing beyond it. A field added without
    /// adjusting `SPACE` would otherwise be silently truncated on write — and the off-chain
    /// decoder, which hardcodes this layout, would drift.
    #[test]
    fn declared_space_matches_the_serialized_body() {
        let record = UserDecryptionDelegation {
            delegator: Pubkey::new_unique(),
            delegate: Pubkey::new_unique(),
            encrypted_value_account_authority: Pubkey::new_unique(),
            expiration_slot: u64::MAX,
            delegation_counter: u64::MAX,
            last_update_slot: u64::MAX,
            revoked: true,
            bump: 255,
        };

        let mut serialized = Vec::new();
        record.try_serialize(&mut serialized).expect("serializes");

        assert_eq!(serialized.len(), 8 + UserDecryptionDelegation::SPACE);
    }

    /// The shared crate's decoder — the one byte-level reading the KMS connector and the
    /// relayer trust — reads back exactly what this program's serializer writes. Every field
    /// carries a distinct value, so a swap of the two neighboring pubkeys or of the three
    /// same-width `u64`s fails here instead of surviving as a silent cross-side misread.
    #[test]
    fn shared_crate_decoder_reads_what_the_program_serializes() {
        let record = UserDecryptionDelegation {
            delegator: Pubkey::new_unique(),
            delegate: Pubkey::new_unique(),
            encrypted_value_account_authority: Pubkey::new_unique(),
            expiration_slot: 11,
            delegation_counter: 22,
            last_update_slot: 33,
            revoked: false,
            bump: 254,
        };

        let mut serialized = Vec::new();
        record.try_serialize(&mut serialized).expect("serializes");

        let decoded = zama_solana_acl::decode_user_decryption_delegation(&serialized)
            .expect("the shared decoder accepts the program's bytes");

        assert_eq!(decoded.delegator, record.delegator.to_bytes());
        assert_eq!(decoded.delegate, record.delegate.to_bytes());
        assert_eq!(
            decoded.encrypted_value_account_authority,
            record.encrypted_value_account_authority.to_bytes()
        );
        assert_eq!(decoded.expiration_slot, record.expiration_slot);
        assert_eq!(decoded.delegation_counter, record.delegation_counter);
        assert_eq!(decoded.last_update_slot, record.last_update_slot);
        assert_eq!(decoded.revoked, record.revoked);
        assert_eq!(decoded.bump, record.bump);
    }
}
