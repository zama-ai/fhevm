//! Encrypted store resolution. A handle entry names its store by address; this turns that
//! unsigned claim into a validated account. Program ownership is the trust anchor: only the host
//! program can write data into an account it owns.

use super::failure::InvalidHostRecord;
use super::snapshot::SnapshotAccount;
use solana_pubkey::Pubkey;
use zama_solana_acl::{EncryptedStore, StoreRejection, validate_store};
use zama_solana_permit::{AllowedScopes, Identity};

/// An encrypted store that passed [`resolve_encrypted_store`], its only constructor.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ResolvedEncryptedStore {
    account_key: Pubkey,
    encrypted_store: EncryptedStore,
}

impl ResolvedEncryptedStore {
    pub fn account_key(&self) -> Pubkey {
        self.account_key
    }

    pub fn program(&self) -> Pubkey {
        Pubkey::new_from_array(self.encrypted_store.program)
    }

    pub fn scope(&self) -> Pubkey {
        Pubkey::new_from_array(self.encrypted_store.scope)
    }

    pub fn encrypted_store(&self) -> &EncryptedStore {
        &self.encrypted_store
    }

    /// Whether a permit signed for `scopes` covers this store's application. The pair comes from
    /// the validated store, never from the request; an empty list admits every application, as
    /// on EVM.
    pub fn is_in(&self, scopes: &AllowedScopes) -> bool {
        scopes.admits(
            &Identity::new(self.encrypted_store.program),
            &Identity::new(self.encrypted_store.scope),
        )
    }
}

/// Validates the account an entry names, by the rule the relayer's pre-check also applies
/// ([`validate_store`]). An empty System-owned account is absent: anyone can fund the derivable
/// address before the store is created there.
pub fn resolve_encrypted_store(
    account: Option<&SnapshotAccount>,
    program_id: Pubkey,
    account_key: Pubkey,
) -> Result<ResolvedEncryptedStore, EncryptedStoreFailure> {
    let store_address = |store: &EncryptedStore| {
        let bump = [store.bump];
        let mut seeds: Vec<&[u8]> = store.seeds().to_vec();
        seeds.push(&bump);
        Pubkey::create_program_address(&seeds, &program_id)
            .ok()
            .map(|address| address.to_bytes())
    };
    let encrypted_store = validate_store(
        program_id.as_array(),
        account_key.as_array(),
        account.map(SnapshotAccount::view),
        store_address,
    )
    .map_err(|rejection| match rejection {
        StoreRejection::Absent => EncryptedStoreFailure::Absent { account_key },
        StoreRejection::ForeignOwner(owner) => EncryptedStoreFailure::ForeignOwner {
            account_key,
            owner: Pubkey::new_from_array(owner),
        },
        StoreRejection::NotAnEncryptedStore => {
            EncryptedStoreFailure::NotAnEncryptedStore { account_key }
        }
        StoreRejection::AddressMismatch(derived) => EncryptedStoreFailure::AddressMismatch {
            account_key,
            derived: derived.map(Pubkey::new_from_array),
        },
        StoreRejection::InvalidHostRecord => InvalidHostRecord { account_key }.into(),
    })?;
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
    #[error("encrypted store {account_key} does not exist at the observed slot")]
    Absent { account_key: Pubkey },
    #[error("encrypted store {account_key} is owned by {owner}")]
    ForeignOwner { account_key: Pubkey, owner: Pubkey },
    #[error("account {account_key} is not an encrypted store")]
    NotAnEncryptedStore { account_key: Pubkey },
    #[error("encrypted store {account_key} does not live at the address its fields derive")]
    AddressMismatch {
        account_key: Pubkey,
        derived: Option<Pubkey>,
    },
    #[error(transparent)]
    InvalidHostRecord(#[from] InvalidHostRecord),
}
