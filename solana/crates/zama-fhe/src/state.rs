use anchor_lang::prelude::Pubkey;

use crate::operand::{Operand, OperandKind};
use crate::{AppScope, FheExecutionBuildError, FheTyped, Result, StoredValue};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateId {
    pub(crate) address: Pubkey,
    pub(crate) authority: Pubkey,
    pub(crate) app: AppScope,
}

impl StateId {
    pub fn new(program: Pubkey, authority: Pubkey, scope: [u8; 32]) -> Self {
        Self {
            address: zama_host::encrypted_state_address(program, authority, scope).0,
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
    pub fn scratch_address(self) -> Pubkey {
        zama_host::transient_address(self.address).0
    }
}

/// A borrowed snapshot of a host-owned encrypted dictionary.
pub struct State<'a> {
    id: StateId,
    account: &'a zama_host::EncryptedState,
}

impl<'a> State<'a> {
    pub fn new(account: &'a zama_host::EncryptedState) -> Self {
        Self {
            id: StateId::new(account.program, account.authority, account.scope),
            account,
        }
    }

    pub fn id(&self) -> StateId {
        self.id
    }

    pub fn get<T: FheTyped>(&self, key: [u8; 32]) -> Result<StoredValue<T>> {
        let handle = self
            .account
            .get(&key)
            .ok_or(FheExecutionBuildError::MissingStateSlot)?;
        StoredValue::from_handle_operand(
            handle,
            Operand(OperandKind::StateSlot {
                state: self.id,
                key,
                handle,
            }),
        )
    }

    pub fn set(&self, key: [u8; 32]) -> StateOutput {
        StateOutput {
            slot: Some((key, self.account.get(&key))),
            ..self.result()
        }
    }

    /// Seal or share a result without allocating a dictionary slot.
    pub fn result(&self) -> StateOutput {
        StateOutput {
            state: self.id,
            previous_leaf_count: self.account.leaf_count,
            slot: None,
            allows: vec![],
            make_public: false,
            grants: vec![],
        }
    }

    pub fn granted<T: FheTyped>(&self, handle: [u8; 32], initiating: StateId) -> Result<StoredValue<T>> {
        self.granted_from_scratch(handle, initiating.scratch_address())
    }

    pub fn granted_from_scratch<T: FheTyped>(&self, handle: [u8; 32], scratch: Pubkey) -> Result<StoredValue<T>> {
        StoredValue::from_handle_operand(handle, Operand(OperandKind::Granted { consumer: self.id, scratch, handle }))
    }

}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateOutput {
    pub(crate) state: StateId,
    pub(crate) previous_leaf_count: u64,
    pub(crate) slot: Option<([u8; 32], Option<[u8; 32]>)>,
    pub(crate) allows: Vec<Pubkey>,
    pub(crate) make_public: bool,
    pub(crate) grants: Vec<(StateId, StateId)>,
}

impl StateOutput {
    pub fn allow(mut self, subject: Pubkey) -> Self {
        self.allows.push(subject);
        self
    }
    pub fn make_public(mut self) -> Self {
        self.make_public = true;
        self
    }
    pub fn allow_transient(mut self, initiating: StateId, consumer: StateId) -> Self {
        self.grants.push((initiating, consumer));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ExecutionEncryptedValueAccountAuthority, FheExecution, Output, Scalar, Uint};

    #[test]
    fn slots_share_history_and_a_failed_rewrite_does_not_poison_the_builder() {
        let account = zama_host::EncryptedState {
            program: Pubkey::new_unique(),
            authority: Pubkey::new_unique(),
            scope: [3; 32],
            slots: vec![],
            leaf_count: 0,
            peaks: vec![],
            bump: 0,
        };
        let state = State::new(&account);
        let execution = FheExecution::build(
            ExecutionEncryptedValueAccountAuthority::new(account.authority),
            |fhe| {
                let first = fhe.trivial_encrypt_u64(
                    1,
                    Output::state(state.set([1; 32]).allow(account.authority)),
                )?;
                assert_eq!(
                    fhe.trivial_encrypt_u64(2, Output::state(state.set([1; 32])))
                        .unwrap_err(),
                    FheExecutionBuildError::PersistentOperandWrittenEarlier
                );
                fhe.add(
                    first,
                    Scalar::<Uint<64>>::u64(1),
                    Output::state(state.set([2; 32]).make_public()),
                )?;
                Ok(())
            },
        )
        .unwrap();
        let counts: Vec<_> = execution
            .args
            .steps
            .iter()
            .map(
                |step| match crate::execution::fhe_execute_step_output(step) {
                    zama_host::FheExecuteOutput::State {
                        previous_leaf_count,
                        ..
                    } => *previous_leaf_count,
                    _ => panic!("state output"),
                },
            )
            .collect();
        assert_eq!(counts, [0, 1]);
    }
}
