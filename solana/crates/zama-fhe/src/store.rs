use anchor_lang::prelude::Pubkey;

use crate::operand::{Operand, OperandKind};
use crate::{AppScope, FheExecutionBuildError, FheHandle, FheTyped, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StoreId {
    pub(crate) address: Pubkey,
    pub(crate) authority: Pubkey,
    pub(crate) app: AppScope,
}

impl StoreId {
    pub fn new(program: Pubkey, authority: Pubkey, scope: [u8; 32]) -> Self {
        Self {
            address: zama_host::encrypted_store_address(program, authority, scope).0,
            authority,
            app: AppScope { program, scope },
        }
    }

    pub fn address(self) -> Pubkey {
        self.address
    }
    pub fn authority(self) -> Pubkey {
        self.authority
    }
    pub fn app(self) -> AppScope {
        self.app
    }
}

/// A borrowed snapshot of a host-owned encrypted dictionary.
pub struct Store<'a> {
    id: StoreId,
    account: &'a zama_host::EncryptedStore,
}

impl<'a> Store<'a> {
    pub fn new(account: &'a zama_host::EncryptedStore) -> Self {
        Self {
            id: StoreId::new(account.program, account.authority, account.scope),
            account,
        }
    }

    pub fn id(&self) -> StoreId {
        self.id
    }

    pub fn get<T: FheTyped>(&self, key: [u8; 32]) -> Result<FheHandle<T>> {
        let handle = self
            .account
            .get(&key)
            .ok_or(FheExecutionBuildError::MissingStoreSlot)?;
        FheHandle::from_handle_operand(
            handle,
            Operand(OperandKind::StoreSlot {
                store: self.id,
                key,
                handle,
            }),
        )
    }

    pub fn set(&self, key: [u8; 32]) -> StoreOutput {
        StoreOutput {
            slot: Some((key, self.account.get(&key))),
            ..self.result()
        }
    }

    /// Seal or share a result without allocating a dictionary slot.
    pub fn result(&self) -> StoreOutput {
        StoreOutput {
            store: self.id,
            previous_leaf_count: self.account.leaf_count,
            slot: None,
            allows: vec![],
            make_public: false,
            grants: vec![],
        }
    }

    pub fn granted<T: FheTyped>(&self, handle: [u8; 32]) -> Result<FheHandle<T>> {
        FheHandle::from_handle_operand(
            handle,
            Operand(OperandKind::Granted {
                consumer: self.id,
                handle,
            }),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoreOutput {
    pub(crate) store: StoreId,
    pub(crate) previous_leaf_count: u64,
    pub(crate) slot: Option<([u8; 32], Option<[u8; 32]>)>,
    pub(crate) allows: Vec<Pubkey>,
    pub(crate) make_public: bool,
    pub(crate) grants: Vec<StoreId>,
}

impl StoreOutput {
    pub fn allow(mut self, subject: Pubkey) -> Self {
        self.allows.push(subject);
        self
    }
    pub fn make_public(mut self) -> Self {
        self.make_public = true;
        self
    }
    pub fn allow_transient(mut self, consumer: StoreId) -> Self {
        self.grants.push(consumer);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FheExecution, Scalar, Uint};

    #[test]
    fn slots_share_history_and_a_failed_rewrite_does_not_poison_the_builder() {
        let account = zama_host::EncryptedStore {
            program: Pubkey::new_unique(),
            authority: Pubkey::new_unique(),
            scope: [3; 32],
            slots: vec![],
            leaf_count: 0,
            peaks: vec![],
            bump: 0,
        };
        let store = Store::new(&account);
        let execution = FheExecution::build(store.id(), |fhe| {
            let first = fhe.trivial_encrypt_u64(1)?;
            fhe.output(first, store.set([1; 32]).allow(account.authority))?;
            assert_eq!(
                fhe.output(first, store.set([1; 32])).unwrap_err(),
                FheExecutionBuildError::DuplicateSlotWrite
            );
            let second = fhe.add(first, Scalar::<Uint<64>>::u64(1))?;
            fhe.output(second, store.set([2; 32]).make_public())?;
            Ok(())
        })
        .unwrap();
        let counts: Vec<_> = execution
            .args
            .effects
            .iter()
            .map(|effect| effect.previous_leaf_count)
            .collect();
        assert_eq!(counts, [0, 1]);
    }
}
