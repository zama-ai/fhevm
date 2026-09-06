//! On-chain account data for `EncryptedValue` (RFC 035).
//!
//! One account per encrypted value, reused across every handle update, carrying a compact MMR
//! history of every decrypt permission ever sealed on it. Field order follows
//! `zama_solana_acl::EncryptedValue`, so the shared crate's discriminator, size formula, seeds
//! and MMR helpers apply directly.

use super::*;

/// Canonical state for one encrypted value.
///
/// PDA: `[ENCRYPTED_VALUE_SEED, program, encrypted_value_account_authority, scope, label]`.
/// The account name must stay exactly `EncryptedValue` — Anchor derives the discriminator from
/// the type name, and it must match `zama_solana_acl::encrypted_value_discriminator()`.
#[account]
pub struct EncryptedValue {
    /// The application program this value belongs to. Proven on create: the authority must be a
    /// PDA of this program (the create carries the seeds), so no other program can sign for it.
    pub program: Pubkey,
    /// The account that controls this encrypted value: a PDA of `program` that must sign to create
    /// it, read it into a computation, or update its handle. Enforced by address rather than by
    /// comparing this field — the signer must equal the authority declared in the execution and
    /// the account written must be the PDA rederived from the declared four seeds, which is what
    /// ties the signer to the stored value. For a token balance this is the token account itself.
    pub encrypted_value_account_authority: Pubkey,
    /// Program-declared scope within `program` (the mint for the token program).
    /// `(program, scope)` is the application identity for HCU metering and the deny list.
    pub scope: [u8; 32],
    /// Which of the authority's values this is.
    pub label: [u8; 32],
    /// Current encrypted value identifier (the live handle).
    pub current_handle: [u8; 32],
    /// Number of MMR leaves appended; `0` means no history.
    pub leaf_count: u64,
    /// MMR peaks, oldest mountain first (`popcount(leaf_count)` entries).
    pub peaks: Vec<[u8; 32]>,
    /// PDA bump.
    pub bump: u8,
}

impl EncryptedValue {
    /// Anchor account body size (excludes the 8-byte discriminator) with `peaks_len` peaks.
    pub fn space(peaks_len: usize) -> usize {
        zama_solana_acl::EncryptedValue::account_size(peaks_len) - 8
    }

    /// Converts to the shared crate's wire type for MMR/authorization helpers.
    pub fn to_shared(&self) -> zama_solana_acl::EncryptedValue {
        zama_solana_acl::EncryptedValue {
            program: self.program.to_bytes(),
            encrypted_value_account_authority: self.encrypted_value_account_authority.to_bytes(),
            scope: self.scope,
            label: self.label,
            current_handle: self.current_handle,
            leaf_count: self.leaf_count,
            peaks: self.peaks.clone(),
            bump: self.bump,
        }
    }

    /// The canonical address of this value, rederived from its own fields.
    pub fn canonical_address(&self) -> (Pubkey, u8) {
        encrypted_value_address(
            self.program,
            self.encrypted_value_account_authority,
            self.scope,
            self.label,
        )
    }
}

/// Returns the canonical `EncryptedValue` PDA address for its four identity seeds.
pub fn encrypted_value_address(
    program: Pubkey,
    encrypted_value_account_authority: Pubkey,
    scope: [u8; 32],
    label: [u8; 32],
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &zama_solana_acl::encrypted_value_seeds(
            &program.to_bytes(),
            &encrypted_value_account_authority.to_bytes(),
            &scope,
            &label,
        ),
        &crate::ID,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::Discriminator;

    /// The Anchor-derived discriminator for `EncryptedValue` must match the
    /// shared crate's `sha256("account:EncryptedValue")[..8]`, since the
    /// off-chain KMS/coprocessor decode account data with the shared crate alone.
    #[test]
    fn discriminator_matches_shared_crate() {
        assert_eq!(
            EncryptedValue::DISCRIMINATOR,
            zama_solana_acl::encrypted_value_discriminator()
        );
    }

    /// The shared crate's decoder reads back exactly what this program's serializer writes, and
    /// the seeds it hands out derive the same address this program derives.
    #[test]
    fn shared_crate_decoder_reads_what_the_program_serializes() {
        let mut value = EncryptedValue {
            program: Pubkey::new_unique(),
            encrypted_value_account_authority: Pubkey::new_unique(),
            scope: [3; 32],
            label: [4; 32],
            current_handle: [5; 32],
            leaf_count: 3,
            peaks: vec![[6; 32], [7; 32]],
            bump: 0,
        };
        let (key, bump) = value.canonical_address();
        value.bump = bump;
        let mut serialized = Vec::new();
        value.try_serialize(&mut serialized).expect("serializes");
        assert_eq!(serialized.len(), 8 + EncryptedValue::space(2));

        let decoded = zama_solana_acl::decode_on_chain_account(&serialized)
            .expect("the shared decoder accepts the program's bytes");
        assert_eq!(decoded, value.to_shared());
        let (derived, derived_bump) = Pubkey::find_program_address(&decoded.seeds(), &crate::ID);
        assert_eq!((derived, derived_bump), (key, bump));
    }
}
