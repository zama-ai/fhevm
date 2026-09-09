//! Current encrypted slots and the authority's persistent decrypt history.

use borsh::{BorshDeserialize, BorshSerialize};

use crate::{sha256, AclError, MAX_MMR_PEAKS};

pub const ENCRYPTED_STATE_SEED: &[u8] = b"encrypted-state";
pub const MAX_STATE_SLOTS: usize = 32;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct EncryptedSlot {
    pub key: [u8; 32],
    pub handle: [u8; 32],
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct EncryptedState {
    pub program: [u8; 32],
    pub authority: [u8; 32],
    pub scope: [u8; 32],
    pub slots: Vec<EncryptedSlot>,
    pub leaf_count: u64,
    pub peaks: Vec<[u8; 32]>,
    pub bump: u8,
}

impl EncryptedState {
    pub fn seeds(&self) -> [&[u8]; 4] {
        [
            ENCRYPTED_STATE_SEED,
            &self.program,
            &self.authority,
            &self.scope,
        ]
    }

    pub const fn account_size(slots: usize, peaks: usize) -> usize {
        8 + 32 * 3 + 4 + 64 * slots + 8 + 4 + 32 * peaks + 1
    }

    pub fn get(&self, key: &[u8; 32]) -> Option<[u8; 32]> {
        self.slots
            .iter()
            .find(|slot| &slot.key == key)
            .map(|slot| slot.handle)
    }

    pub fn validate(&self) -> Result<(), AclError> {
        validate_state_shape(
            self.slots.iter().map(|slot| slot.key),
            self.leaf_count,
            self.peaks.len(),
        )
    }
}

pub fn validate_state_shape(
    keys: impl ExactSizeIterator<Item = [u8; 32]> + Clone,
    leaf_count: u64,
    peak_count: usize,
) -> Result<(), AclError> {
    if keys.len() > MAX_STATE_SLOTS
        || keys
            .clone()
            .enumerate()
            .any(|(i, key)| keys.clone().skip(i + 1).any(|other| other == key))
    {
        return Err(AclError::BadAccountData);
    }
    if peak_count > MAX_MMR_PEAKS || peak_count != leaf_count.count_ones() as usize {
        return Err(AclError::MmrInconsistent);
    }
    Ok(())
}

pub fn encrypted_state_discriminator() -> [u8; 8] {
    let digest = sha256(&[b"account:EncryptedState"]);
    let mut discriminator = [0; 8];
    discriminator.copy_from_slice(&digest[..8]);
    discriminator
}

pub fn decode_encrypted_state(data: &[u8]) -> Result<EncryptedState, AclError> {
    if data.len() < 8 || data[..8] != encrypted_state_discriminator() {
        return Err(AclError::BadDiscriminator);
    }
    let state =
        EncryptedState::deserialize(&mut &data[8..]).map_err(|_| AclError::BadAccountData)?;
    state.validate()?;
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decoder_rejects_duplicate_keys_and_inconsistent_history() {
        let mut state = EncryptedState::default();
        state.slots.push(EncryptedSlot {
            key: [1; 32],
            handle: [2; 32],
        });
        let encode = |state: &EncryptedState| {
            let mut bytes = encrypted_state_discriminator().to_vec();
            state.serialize(&mut bytes).unwrap();
            bytes
        };
        assert_eq!(decode_encrypted_state(&encode(&state)), Ok(state.clone()));
        state.slots.push(state.slots[0].clone());
        assert_eq!(
            decode_encrypted_state(&encode(&state)),
            Err(AclError::BadAccountData)
        );
        state.slots.pop();
        state.leaf_count = 1;
        assert_eq!(
            decode_encrypted_state(&encode(&state)),
            Err(AclError::MmrInconsistent)
        );
    }
}
