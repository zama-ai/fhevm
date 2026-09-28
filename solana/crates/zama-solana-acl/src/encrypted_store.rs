//! Current encrypted slots and the authority's persistent decrypt history.

use borsh::{BorshDeserialize, BorshSerialize};

use crate::account::{initialized, AccountView};
use crate::{sha256, AclError, MAX_MMR_PEAKS, WILDCARD_APP};

pub const ENCRYPTED_STORE_SEED: &[u8] = b"encrypted-state";
pub const MAX_STORE_SLOTS: usize = 32;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct EncryptedSlot {
    pub key: [u8; 32],
    pub handle: [u8; 32],
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct EncryptedStore {
    pub program: [u8; 32],
    pub authority: [u8; 32],
    pub scope: [u8; 32],
    pub slots: Vec<EncryptedSlot>,
    pub leaf_count: u64,
    pub peaks: Vec<[u8; 32]>,
    pub bump: u8,
}

impl EncryptedStore {
    pub fn seeds(&self) -> [&[u8]; 4] {
        [
            ENCRYPTED_STORE_SEED,
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
        validate_store_shape(
            self.slots.iter().map(|slot| slot.key),
            self.leaf_count,
            self.peaks.len(),
        )
    }
}

pub fn validate_store_shape(
    keys: impl ExactSizeIterator<Item = [u8; 32]> + Clone,
    leaf_count: u64,
    peak_count: usize,
) -> Result<(), AclError> {
    if keys.len() > MAX_STORE_SLOTS
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

pub fn encrypted_store_discriminator() -> [u8; 8] {
    let digest = sha256(&[b"account:EncryptedStore"]);
    let mut discriminator = [0; 8];
    discriminator.copy_from_slice(&digest[..8]);
    discriminator
}

pub fn decode_encrypted_store(data: &[u8]) -> Result<EncryptedStore, AclError> {
    if data.len() < 8 || data[..8] != encrypted_store_discriminator() {
        return Err(AclError::BadDiscriminator);
    }
    let state =
        EncryptedStore::deserialize(&mut &data[8..]).map_err(|_| AclError::BadAccountData)?;
    state.validate()?;
    Ok(state)
}

/// Why the account a request names is not a usable encrypted store.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoreRejection {
    /// Nothing is stored at the address.
    Absent,
    /// The account belongs to another program.
    ForeignOwner([u8; 32]),
    /// A host account of another type.
    NotAnEncryptedStore,
    /// The store's own fields derive another address, or none.
    AddressMismatch(Option<[u8; 32]>),
    /// A host-owned store the host program could not have written.
    InvalidHostRecord,
}

/// The encrypted store at `account_key`, checked as the host program writes one: present, owned
/// by `host_program`, decodable, at the address its own seeds and stored bump derive, and naming
/// the wildcard sentinel in neither its program nor its scope. Trailing bytes are legal: a store
/// grows by realloc and never shrinks.
///
/// `store_address` derives a store's address under the host program (`create_program_address`
/// over [`EncryptedStore::seeds`] and its bump), `None` when those give no PDA. Derivation stays
/// with each caller, which owns its solana-pubkey version.
pub fn validate_store(
    host_program: &[u8; 32],
    account_key: &[u8; 32],
    account: Option<AccountView<'_>>,
    store_address: impl FnOnce(&EncryptedStore) -> Option<[u8; 32]>,
) -> Result<EncryptedStore, StoreRejection> {
    let account = initialized(account).ok_or(StoreRejection::Absent)?;
    if account.owner != host_program {
        return Err(StoreRejection::ForeignOwner(*account.owner));
    }
    let store = decode_encrypted_store(account.data).map_err(|error| match error {
        AclError::BadDiscriminator => StoreRejection::NotAnEncryptedStore,
        _ => StoreRejection::InvalidHostRecord,
    })?;
    let derived = store_address(&store);
    if derived != Some(*account_key) {
        return Err(StoreRejection::AddressMismatch(derived));
    }
    // With the sentinel as program, the store's delegation row would be the wildcard row itself.
    // No legal store names it in either position (see `WILDCARD_APP`).
    if store.program == WILDCARD_APP || store.scope == WILDCARD_APP {
        return Err(StoreRejection::InvalidHostRecord);
    }
    Ok(store)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decoder_rejects_duplicate_keys_and_inconsistent_history() {
        let mut state = EncryptedStore::default();
        state.slots.push(EncryptedSlot {
            key: [1; 32],
            handle: [2; 32],
        });
        let encode = |state: &EncryptedStore| {
            let mut bytes = encrypted_store_discriminator().to_vec();
            state.serialize(&mut bytes).unwrap();
            bytes
        };
        assert_eq!(decode_encrypted_store(&encode(&state)), Ok(state.clone()));
        state.slots.push(state.slots[0].clone());
        assert_eq!(
            decode_encrypted_store(&encode(&state)),
            Err(AclError::BadAccountData)
        );
        state.slots.pop();
        state.leaf_count = 1;
        assert_eq!(
            decode_encrypted_store(&encode(&state)),
            Err(AclError::MmrInconsistent)
        );
    }

    #[test]
    fn a_store_is_validated_as_the_host_writes_one() {
        const HOST: [u8; 32] = [7; 32];
        const KEY: [u8; 32] = [8; 32];
        let store = EncryptedStore {
            program: [1; 32],
            scope: [2; 32],
            ..EncryptedStore::default()
        };
        let encode = |state: &EncryptedStore| {
            let mut bytes = encrypted_store_discriminator().to_vec();
            state.serialize(&mut bytes).unwrap();
            bytes
        };
        let data = encode(&store);
        let at_key = |_: &EncryptedStore| Some(KEY);
        let validate =
            |owner: &[u8; 32], data: &[u8], derive: fn(&EncryptedStore) -> Option<[u8; 32]>| {
                validate_store(&HOST, &KEY, Some(AccountView { owner, data }), derive)
            };

        assert_eq!(validate(&HOST, &data, at_key), Ok(store.clone()));
        let mut grown = data.clone();
        grown.extend_from_slice(&[0; 64]);
        assert_eq!(validate(&HOST, &grown, at_key), Ok(store.clone()));

        assert_eq!(
            validate_store(&HOST, &KEY, None, at_key),
            Err(StoreRejection::Absent)
        );
        assert_eq!(validate(&[0; 32], &[], at_key), Err(StoreRejection::Absent));
        assert_eq!(
            validate(&[9; 32], &data, at_key),
            Err(StoreRejection::ForeignOwner([9; 32]))
        );
        assert_eq!(
            validate(&HOST, &[0; 16], at_key),
            Err(StoreRejection::NotAnEncryptedStore)
        );
        assert_eq!(
            validate(&HOST, &data[..data.len() - 1], at_key),
            Err(StoreRejection::InvalidHostRecord)
        );
        assert_eq!(
            validate(&HOST, &data, |_| None),
            Err(StoreRejection::AddressMismatch(None))
        );
        for sentinel in [
            EncryptedStore {
                program: WILDCARD_APP,
                ..store.clone()
            },
            EncryptedStore {
                scope: WILDCARD_APP,
                ..store.clone()
            },
        ] {
            assert_eq!(
                validate(&HOST, &encode(&sentinel), at_key),
                Err(StoreRejection::InvalidHostRecord)
            );
        }
    }
}
