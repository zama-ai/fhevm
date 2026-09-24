//! Encrypted store resolution. A handle entry names its store by address; this turns that
//! unsigned claim into a validated account. Program ownership is the trust anchor: only the host
//! program can write data into an account it owns.

use super::SolanaPubkeyBytes;
use super::failure::InvalidHostRecord;
use super::snapshot::SnapshotAccount;
use solana_pubkey::Pubkey;
use zama_solana_acl::WILDCARD_APP;
use zama_solana_acl::{AclError, EncryptedStore, decode_encrypted_store};

/// An encrypted store that passed [`resolve_encrypted_store`], its only constructor.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ResolvedEncryptedStore {
    account_key: SolanaPubkeyBytes,
    encrypted_store: EncryptedStore,
}

impl ResolvedEncryptedStore {
    pub fn account_key(&self) -> SolanaPubkeyBytes {
        self.account_key
    }

    pub fn program(&self) -> SolanaPubkeyBytes {
        self.encrypted_store.program
    }

    pub fn scope(&self) -> SolanaPubkeyBytes {
        self.encrypted_store.scope
    }

    pub fn encrypted_store(&self) -> &EncryptedStore {
        &self.encrypted_store
    }
}

/// The address a store with these fields must live at: the PDA of its own seeds and stored bump.
fn encrypted_store_address(
    program_id: SolanaPubkeyBytes,
    state: &EncryptedStore,
) -> Option<SolanaPubkeyBytes> {
    let bump = [state.bump];
    let mut seeds: Vec<&[u8]> = state.seeds().to_vec();
    seeds.push(&bump);
    Pubkey::create_program_address(&seeds, &Pubkey::new_from_array(program_id))
        .ok()
        .map(|address| address.to_bytes())
}

/// The account must exist, be owned by the host program, decode as an encrypted store, live at
/// the address its own fields derive, and name a real authority. Trailing bytes are legal: a
/// store grows by realloc and never shrinks. An empty System-owned account is absent: anyone can
/// fund the derivable address before the store is created there.
pub fn resolve_encrypted_store(
    account: Option<&SnapshotAccount>,
    program_id: SolanaPubkeyBytes,
    account_key: SolanaPubkeyBytes,
) -> Result<ResolvedEncryptedStore, EncryptedStoreFailure> {
    let account = account
        .filter(|account| !account.is_uninitialized_pda())
        .ok_or(EncryptedStoreFailure::Absent { account_key })?;
    if account.owner != program_id {
        return Err(EncryptedStoreFailure::ForeignOwner {
            account_key,
            owner: account.owner,
        });
    }
    let invalid = InvalidHostRecord { account_key };
    let encrypted_store = decode_encrypted_store(&account.data).map_err(|error| match error {
        AclError::BadDiscriminator => EncryptedStoreFailure::NotAnEncryptedStore { account_key },
        _ => invalid.into(),
    })?;
    let derived = encrypted_store_address(program_id, &encrypted_store);
    if derived != Some(account_key) {
        return Err(EncryptedStoreFailure::AddressMismatch {
            account_key,
            derived,
        });
    }
    // With the sentinel as program, the store's delegation row would be the wildcard row itself.
    // No legal store names it: its authority must sign as a PDA of the program, and no one holds
    // the key to deploy a program there.
    if encrypted_store.program == WILDCARD_APP {
        return Err(invalid.into());
    }
    Ok(ResolvedEncryptedStore {
        account_key,
        encrypted_store,
    })
}

/// Why the account an entry names is not a usable encrypted store. The address is the user's
/// claim, so a wrong account is the request's fault; only a host-owned store the host program
/// could not have written is an [`InvalidHostRecord`].
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum EncryptedStoreFailure {
    #[error("encrypted store {account_key:?} does not exist at the observed slot")]
    Absent { account_key: SolanaPubkeyBytes },
    #[error("encrypted store {account_key:?} is owned by {owner:?}")]
    ForeignOwner {
        account_key: SolanaPubkeyBytes,
        owner: SolanaPubkeyBytes,
    },
    #[error("account {account_key:?} is not an encrypted store")]
    NotAnEncryptedStore { account_key: SolanaPubkeyBytes },
    #[error("encrypted store {account_key:?} does not live at the address its fields derive")]
    AddressMismatch {
        account_key: SolanaPubkeyBytes,
        derived: Option<SolanaPubkeyBytes>,
    },
    #[error(transparent)]
    InvalidHostRecord(#[from] InvalidHostRecord),
}
