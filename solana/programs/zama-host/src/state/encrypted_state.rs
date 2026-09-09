use super::*;

pub use zama_solana_acl::{ENCRYPTED_STATE_SEED, MAX_STATE_SLOTS};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct EncryptedSlot {
    pub key: [u8; 32],
    pub handle: [u8; 32],
}

#[account]
pub struct EncryptedState {
    pub program: Pubkey,
    pub authority: Pubkey,
    pub scope: [u8; 32],
    pub slots: Vec<EncryptedSlot>,
    pub leaf_count: u64,
    pub peaks: Vec<[u8; 32]>,
    pub bump: u8,
}

impl EncryptedState {
    pub fn canonical_address(&self) -> (Pubkey, u8) {
        encrypted_state_address(self.program, self.authority, self.scope)
    }

    pub(crate) fn validate(&self, address: Pubkey) -> Result<()> {
        zama_solana_acl::encrypted_state::validate_state_shape(
            self.slots.iter().map(|slot| slot.key),
            self.leaf_count,
            self.peaks.len(),
        )
        .map_err(|_| error!(ZamaHostError::InvalidFheExecuteAccount))?;
        let (expected, bump) = self.canonical_address();
        require!(
            address == expected && self.bump == bump,
            ZamaHostError::EncryptedStatePdaMismatch
        );
        Ok(())
    }

    pub fn set(
        &mut self,
        key: [u8; 32],
        expected: Option<[u8; 32]>,
        handle: [u8; 32],
    ) -> Result<()> {
        require!(
            self.get(&key) == expected,
            ZamaHostError::PreviousStateMismatch
        );
        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.key == key) {
            slot.handle = handle;
        } else {
            require!(
                self.slots.len() < MAX_STATE_SLOTS,
                ZamaHostError::EncryptedStateCapacityExceeded
            );
            self.slots.push(EncryptedSlot { key, handle });
        }
        Ok(())
    }

    pub fn get(&self, key: &[u8; 32]) -> Option<[u8; 32]> {
        self.slots
            .iter()
            .find(|slot| &slot.key == key)
            .map(|slot| slot.handle)
    }
}

pub fn encrypted_state_address(
    program: Pubkey,
    authority: Pubkey,
    scope: [u8; 32],
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            ENCRYPTED_STATE_SEED,
            program.as_ref(),
            authority.as_ref(),
            &scope,
        ],
        &crate::ID,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::Discriminator;
    use zama_solana_acl::{
        historical_access_leaf_commitment, mmr_append, mmr_build_proof, mmr_verify,
    };

    fn empty_state() -> EncryptedState {
        EncryptedState {
            program: Pubkey::new_unique(),
            authority: Pubkey::new_unique(),
            scope: [0; 32],
            slots: vec![],
            leaf_count: 0,
            peaks: vec![],
            bump: 0,
        }
    }

    #[test]
    fn canonical_validation_rejects_wrong_bump_address_and_duplicate_slots() {
        let mut state = empty_state();
        let (address, bump) = state.canonical_address();
        state.bump = bump;
        state.validate(address).unwrap();
        assert!(state.validate(Pubkey::new_unique()).is_err());
        state.bump ^= 1;
        assert!(state.validate(address).is_err());
        state.bump = bump;
        state.slots = vec![
            EncryptedSlot {
                key: [1; 32],
                handle: [2; 32]
            };
            2
        ];
        assert!(state.validate(address).is_err());
    }

    #[test]
    fn shared_decoder_accepts_the_host_wire_layout_and_identity() {
        let mut state = EncryptedState {
            program: Pubkey::new_unique(),
            authority: Pubkey::new_unique(),
            scope: [3; 32],
            slots: vec![EncryptedSlot {
                key: [4; 32],
                handle: [5; 32],
            }],
            leaf_count: 0,
            peaks: vec![],
            bump: 0,
        };
        let (address, bump) = state.canonical_address();
        state.bump = bump;
        let mut bytes = Vec::new();
        state.try_serialize(&mut bytes).unwrap();
        assert_eq!(
            EncryptedState::DISCRIMINATOR,
            zama_solana_acl::encrypted_state_discriminator()
        );
        let decoded = zama_solana_acl::decode_encrypted_state(&bytes).unwrap();
        assert_eq!(
            Pubkey::find_program_address(&decoded.seeds(), &crate::ID),
            (address, bump)
        );
        assert_eq!(decoded.get(&[4; 32]), state.get(&[4; 32]));
        assert_eq!(
            bytes.len(),
            zama_solana_acl::EncryptedState::account_size(1, 0)
        );
    }
    #[test]
    fn replacing_one_slot_preserves_other_slots_and_historical_grants() {
        let mut state = empty_state();
        let balance = [1; 32];
        let pending_burn = [2; 32];
        let old_handle = [3; 32];
        state.set(balance, None, old_handle).unwrap();
        state.set(pending_burn, None, [4; 32]).unwrap();
        let leaf = historical_access_leaf_commitment([5; 32], 0, old_handle, [6; 32]);
        mmr_append(&mut state.peaks, &mut state.leaf_count, leaf).unwrap();
        state.set(balance, Some(old_handle), [7; 32]).unwrap();
        assert_eq!(state.get(&pending_burn), Some([4; 32]));
        assert!(mmr_verify(
            &state.peaks,
            state.leaf_count,
            leaf,
            &mmr_build_proof(&[leaf], 0).unwrap()
        ));
        let before = state.clone();
        assert_eq!(
            state.set(balance, Some(old_handle), [8; 32]),
            Err(error!(ZamaHostError::PreviousStateMismatch))
        );
        assert_eq!(state.slots, before.slots);
        assert_eq!(state.peaks, before.peaks);
        assert_eq!(state.leaf_count, before.leaf_count);
    }

    #[test]
    fn full_state_still_allows_replacement_but_never_silently_evicts() {
        let mut state = empty_state();
        for key in 0..MAX_STATE_SLOTS as u8 {
            state.set([key; 32], None, [1; 32]).unwrap();
        }
        let before = state.clone();
        assert_eq!(
            state.set([255; 32], None, [2; 32]),
            Err(error!(ZamaHostError::EncryptedStateCapacityExceeded))
        );
        assert_eq!(state.slots, before.slots);
        assert_eq!(state.peaks, before.peaks);
        assert_eq!(state.leaf_count, before.leaf_count);
        state.set([0; 32], Some([1; 32]), [3; 32]).unwrap();
    }
}
