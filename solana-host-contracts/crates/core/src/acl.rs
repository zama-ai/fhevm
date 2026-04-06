use crate::error::{HostContractError, Result};
use crate::events::HostEvent;
use crate::types::{Handle, Pubkey};
use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UserDecryptionDelegation {
    pub expiration_date: u64,
    pub last_slot_delegate_or_revoke: u64,
    pub delegation_counter: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AclSession {
    transient_allowed_pairs: BTreeSet<(Handle, Pubkey)>,
}

impl AclSession {
    pub fn allow(&mut self, handle: Handle, account: Pubkey) {
        self.transient_allowed_pairs.insert((handle, account));
    }

    pub fn clear(&mut self) {
        self.transient_allowed_pairs.clear();
    }

    fn contains(&self, handle: Handle, account: Pubkey) -> bool {
        self.transient_allowed_pairs.contains(&(handle, account))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AclState {
    owner: Pubkey,
    executor_program: Pubkey,
    paused: bool,
    pausers: BTreeSet<Pubkey>,
    persisted_allowed_pairs: BTreeSet<(Handle, Pubkey)>,
    allowed_for_decryption: BTreeSet<Handle>,
    user_decryption_delegations: BTreeMap<(Pubkey, Pubkey, Pubkey), UserDecryptionDelegation>,
    deny_list: BTreeSet<Pubkey>,
}

impl AclState {
    pub fn new(owner: Pubkey, executor_program: Pubkey) -> Self {
        Self {
            owner,
            executor_program,
            paused: false,
            pausers: BTreeSet::new(),
            persisted_allowed_pairs: BTreeSet::new(),
            allowed_for_decryption: BTreeSet::new(),
            user_decryption_delegations: BTreeMap::new(),
            deny_list: BTreeSet::new(),
        }
    }

    pub fn owner(&self) -> Pubkey {
        self.owner
    }

    pub fn reset_runtime_state(&mut self) {
        self.paused = false;
        self.pausers.clear();
        self.persisted_allowed_pairs.clear();
        self.allowed_for_decryption.clear();
        self.user_decryption_delegations.clear();
        self.deny_list.clear();
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn is_pauser(&self, account: Pubkey) -> bool {
        self.pausers.contains(&account)
    }

    pub fn add_pauser(&mut self, caller: Pubkey, account: Pubkey) -> Result<()> {
        if caller != self.owner {
            return Err(HostContractError::SenderNotAllowed);
        }
        self.pausers.insert(account);
        Ok(())
    }

    pub fn pause(&mut self, caller: Pubkey) -> Result<()> {
        if !self.pausers.contains(&caller) {
            return Err(HostContractError::NotPauser);
        }
        self.paused = true;
        Ok(())
    }

    pub fn unpause(&mut self, caller: Pubkey) -> Result<()> {
        if caller != self.owner {
            return Err(HostContractError::SenderNotAllowed);
        }
        self.paused = false;
        Ok(())
    }

    pub fn allow(
        &mut self,
        caller: Pubkey,
        handle: Handle,
        account: Pubkey,
        session: &AclSession,
    ) -> Result<HostEvent> {
        self.ensure_not_paused()?;
        self.ensure_sender_can_allow(caller, handle, session)?;
        self.persisted_allowed_pairs.insert((handle, account));
        Ok(HostEvent::Allowed {
            caller,
            account,
            handle,
        })
    }

    pub fn allow_many<H>(
        &mut self,
        caller: Pubkey,
        handles: H,
        account: Pubkey,
        session: &AclSession,
    ) -> Result<HostEvent>
    where
        H: AsRef<[Handle]>,
    {
        let handles = handles.as_ref();
        self.ensure_not_paused()?;
        if handles.is_empty() {
            return Err(HostContractError::HandlesListIsEmpty);
        }
        for handle in handles {
            self.ensure_sender_can_allow(caller, *handle, session)?;
            self.persisted_allowed_pairs.insert((*handle, account));
        }
        Ok(HostEvent::AllowedMany {
            caller,
            account,
            handles: handles.to_vec(),
        })
    }

    pub fn allow_for_decryption<H>(
        &mut self,
        caller: Pubkey,
        handles: H,
        session: &AclSession,
    ) -> Result<HostEvent>
    where
        H: AsRef<[Handle]>,
    {
        let handles = handles.as_ref();
        self.ensure_not_paused()?;
        if handles.is_empty() {
            return Err(HostContractError::HandlesListIsEmpty);
        }
        if self.is_account_denied(caller) {
            return Err(HostContractError::SenderDenied);
        }
        for handle in handles {
            if !self.is_allowed(*handle, caller, session) {
                return Err(HostContractError::SenderNotAllowed);
            }
        }
        self.allowed_for_decryption.extend(handles.iter().copied());
        Ok(HostEvent::AllowedForDecryption {
            caller,
            handles: handles.to_vec(),
        })
    }

    pub fn allow_transient(
        &self,
        caller: Pubkey,
        handle: Handle,
        account: Pubkey,
        session: &mut AclSession,
    ) -> Result<()> {
        self.ensure_not_paused()?;
        if caller != self.executor_program {
            self.ensure_sender_can_allow(caller, handle, session)?;
        }
        session.allow(handle, account);
        Ok(())
    }

    pub fn delegate_for_user_decryption(
        &mut self,
        delegator: Pubkey,
        delegate: Pubkey,
        contract_address: Pubkey,
        expiration_date: u64,
        current_timestamp: u64,
        current_slot: u64,
    ) -> Result<HostEvent> {
        self.ensure_not_paused()?;
        if self.is_account_denied(delegator) {
            return Err(HostContractError::SenderDenied);
        }
        if expiration_date <= current_timestamp {
            return Err(HostContractError::ExpirationDateInThePast);
        }
        if delegator == contract_address {
            return Err(HostContractError::SenderCannotBeContractAddress);
        }
        if delegator == delegate {
            return Err(HostContractError::SenderCannotBeDelegate);
        }
        if delegate == contract_address {
            return Err(HostContractError::DelegateCannotBeContractAddress);
        }

        let key = (delegator, delegate, contract_address);
        let delegation = self.user_decryption_delegations.entry(key).or_default();
        if delegation.last_slot_delegate_or_revoke == current_slot {
            return Err(HostContractError::AlreadyDelegatedOrRevokedInSameSlot);
        }
        if delegation.expiration_date == expiration_date {
            return Err(HostContractError::ExpirationDateAlreadySetToSameValue);
        }

        let old_expiration_date = delegation.expiration_date;
        delegation.last_slot_delegate_or_revoke = current_slot;
        delegation.delegation_counter += 1;
        delegation.expiration_date = expiration_date;

        Ok(HostEvent::DelegatedForUserDecryption {
            delegator,
            delegate,
            contract_address,
            delegation_counter: delegation.delegation_counter,
            old_expiration_date,
            new_expiration_date: expiration_date,
        })
    }

    pub fn revoke_delegation_for_user_decryption(
        &mut self,
        delegator: Pubkey,
        delegate: Pubkey,
        contract_address: Pubkey,
        current_slot: u64,
    ) -> Result<HostEvent> {
        self.ensure_not_paused()?;
        let key = (delegator, delegate, contract_address);
        let delegation = self
            .user_decryption_delegations
            .get_mut(&key)
            .ok_or(HostContractError::NotDelegatedYet)?;

        if delegation.expiration_date == 0 {
            return Err(HostContractError::NotDelegatedYet);
        }
        if delegation.last_slot_delegate_or_revoke == current_slot {
            return Err(HostContractError::AlreadyDelegatedOrRevokedInSameSlot);
        }

        let old_expiration_date = delegation.expiration_date;
        delegation.expiration_date = 0;
        delegation.last_slot_delegate_or_revoke = current_slot;
        delegation.delegation_counter += 1;

        Ok(HostEvent::RevokedDelegationForUserDecryption {
            delegator,
            delegate,
            contract_address,
            delegation_counter: delegation.delegation_counter,
            old_expiration_date,
        })
    }

    pub fn get_user_decryption_delegation_expiration_date(
        &self,
        delegator: Pubkey,
        delegate: Pubkey,
        contract_address: Pubkey,
    ) -> u64 {
        self.user_decryption_delegations
            .get(&(delegator, delegate, contract_address))
            .map(|delegation| delegation.expiration_date)
            .unwrap_or_default()
    }

    pub fn is_allowed(&self, handle: Handle, account: Pubkey, session: &AclSession) -> bool {
        self.persist_allowed(handle, account) || session.contains(handle, account)
    }

    pub fn persist_allowed(&self, handle: Handle, account: Pubkey) -> bool {
        self.persisted_allowed_pairs.contains(&(handle, account))
    }

    pub fn is_allowed_for_decryption(&self, handle: Handle) -> bool {
        self.allowed_for_decryption.contains(&handle)
    }

    pub fn is_handle_delegated_for_user_decryption(
        &self,
        delegator: Pubkey,
        delegate: Pubkey,
        contract_address: Pubkey,
        handle: Handle,
        current_timestamp: u64,
    ) -> bool {
        self.persist_allowed(handle, delegator)
            && self.persist_allowed(handle, contract_address)
            && self
                .user_decryption_delegations
                .get(&(delegator, delegate, contract_address))
                .map(|delegation| delegation.expiration_date >= current_timestamp)
                .unwrap_or(false)
    }

    pub fn is_account_denied(&self, account: Pubkey) -> bool {
        self.deny_list.contains(&account)
    }

    pub fn block_account(&mut self, caller: Pubkey, account: Pubkey) -> Result<HostEvent> {
        if caller != self.owner {
            return Err(HostContractError::SenderNotAllowed);
        }
        if !self.deny_list.insert(account) {
            return Err(HostContractError::AccountAlreadyBlocked);
        }
        Ok(HostEvent::BlockedAccount { account })
    }

    pub fn unblock_account(&mut self, caller: Pubkey, account: Pubkey) -> Result<HostEvent> {
        if caller != self.owner {
            return Err(HostContractError::SenderNotAllowed);
        }
        if !self.deny_list.remove(&account) {
            return Err(HostContractError::AccountNotBlocked);
        }
        Ok(HostEvent::UnblockedAccount { account })
    }

    fn ensure_not_paused(&self) -> Result<()> {
        if self.paused {
            return Err(HostContractError::Paused);
        }
        Ok(())
    }

    fn ensure_sender_can_allow(
        &self,
        caller: Pubkey,
        handle: Handle,
        session: &AclSession,
    ) -> Result<()> {
        if self.is_account_denied(caller) {
            return Err(HostContractError::SenderDenied);
        }
        if !self.is_allowed(handle, caller, session) {
            return Err(HostContractError::SenderNotAllowed);
        }
        Ok(())
    }
}
