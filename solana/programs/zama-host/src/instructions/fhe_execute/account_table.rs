//! Validates and caches remaining accounts for one `fhe_execute` execution.
//!
//! Construction rejects duplicate keys. Preflight marks referenced accounts, and
//! `assert_all_used` rejects unused accounts. Canonical State/scratch validation,
//! authority-signer lookup, deny-record lookup and flushing dirty accounts live here.
//! Preflight separately enforces one write per State slot.

use super::*;

pub(super) struct ExecutionAccountTable<'a, 'info> {
    accounts: &'a [AccountInfo<'info>],
    states: Vec<Option<Box<EncryptedState>>>,
    dirty_states: Vec<u16>,
    scratches: Vec<Option<Box<TransientState>>>,
    dirty_scratches: Vec<u16>,
    used: Vec<bool>,
}

impl<'a, 'info> ExecutionAccountTable<'a, 'info> {
    /// Rejects duplicate keys up front so no index-referenced account can be
    /// validated as one role and used as another.
    pub(super) fn new(accounts: &'a [AccountInfo<'info>]) -> Result<Self> {
        for (index, account) in accounts.iter().enumerate() {
            require!(
                !accounts[index + 1..]
                    .iter()
                    .any(|later| later.key() == account.key()),
                ZamaHostError::InvalidFheExecuteAccount
            );
        }
        Ok(Self {
            accounts,
            states: (0..accounts.len()).map(|_| None).collect(),
            dirty_states: Vec::with_capacity(MAX_FHE_EXECUTION_STEPS),
            scratches: (0..accounts.len()).map(|_| None).collect(),
            dirty_scratches: Vec::with_capacity(MAX_FHE_EXECUTION_STEPS),
            used: vec![false; accounts.len()],
        })
    }

    pub(super) fn state(&mut self, index: u16) -> Result<&EncryptedState> {
        let account = self.account(index)?;
        let cached = self
            .states
            .get_mut(index as usize)
            .ok_or(ZamaHostError::InvalidFheExecuteAccount)?;
        if cached.is_none() {
            require_keys_eq!(
                *account.owner,
                crate::ID,
                ZamaHostError::EncryptedStatePdaMismatch
            );
            let state = EncryptedState::try_deserialize(&mut &account.try_borrow_data()?[..])?;
            state.validate(account.key())?;
            *cached = Some(Box::new(state));
        }
        cached
            .as_deref()
            .ok_or_else(|| error!(ZamaHostError::InvalidFheExecuteAccount))
    }

    pub(super) fn state_mut(&mut self, index: u16) -> Result<&mut EncryptedState> {
        self.state(index)?;
        require!(
            self.account(index)?.is_writable,
            ZamaHostError::InvalidFheExecuteAccount
        );
        if !self.dirty_states.contains(&index) {
            self.dirty_states.push(index);
        }
        self.states[index as usize]
            .as_deref_mut()
            .ok_or_else(|| error!(ZamaHostError::InvalidFheExecuteAccount))
    }

    pub(super) fn flush_states(
        &self,
        payer: &AccountInfo<'info>,
        system: &AccountInfo<'info>,
    ) -> Result<()> {
        for &index in &self.dirty_states {
            let info = self.account(index)?;
            let state = self.states[index as usize]
                .as_deref()
                .ok_or(ZamaHostError::InvalidFheExecuteAccount)?;
            grow_account_if_needed(
                payer,
                info,
                system,
                zama_solana_acl::EncryptedState::account_size(state.slots.len(), state.peaks.len()),
            )?;
            write_account(info, state)?;
        }
        for &index in &self.dirty_scratches {
            write_account(
                self.account(index)?,
                self.scratches[index as usize]
                    .as_deref()
                    .ok_or(ZamaHostError::TransientAccountInvalid)?,
            )?;
        }
        Ok(())
    }

    pub(super) fn scratch(&mut self, index: u16) -> Result<&TransientState> {
        if self
            .scratches
            .get(index as usize)
            .ok_or(ZamaHostError::TransientAccountInvalid)?
            .is_some()
        {
            return Ok(self.scratches[index as usize].as_deref().unwrap());
        }
        let info = self.account(index)?;
        require_keys_eq!(
            *info.owner,
            crate::ID,
            ZamaHostError::TransientAccountInvalid
        );
        require!(
            info.data_len() == TransientState::SPACE,
            ZamaHostError::TransientAccountInvalid
        );
        let mut scratch = TransientState::try_deserialize(&mut &info.try_borrow_data()?[..])
            .map_err(|_| error!(ZamaHostError::TransientAccountInvalid))?;
        let (address, bump) = transient_address(scratch.initiating_state);
        require_keys_eq!(info.key(), address, ZamaHostError::TransientAccountInvalid);
        require!(scratch.bump == bump, ZamaHostError::TransientAccountInvalid);
        require!(
            scratch.grants.len() <= MAX_TRANSIENT_GRANTS,
            ZamaHostError::TransientAccountInvalid
        );
        scratch
            .grants
            .reserve_exact(MAX_TRANSIENT_GRANTS - scratch.grants.len());
        self.scratches[index as usize] = Some(Box::new(scratch));
        Ok(self.scratches[index as usize].as_deref().unwrap())
    }

    pub(super) fn scratch_mut(&mut self, index: u16) -> Result<&mut TransientState> {
        self.scratch(index)?;
        require!(
            self.account(index)?.is_writable,
            ZamaHostError::TransientAccountInvalid
        );
        if !self.dirty_scratches.contains(&index) {
            self.dirty_scratches.push(index);
        }
        Ok(self.scratches[index as usize].as_deref_mut().unwrap())
    }

    pub(super) fn account(&self, index: u16) -> Result<&'a AccountInfo<'info>> {
        self.accounts
            .get(index as usize)
            .ok_or_else(|| error!(ZamaHostError::InvalidFheExecuteAccount))
    }

    pub(super) fn mark(&mut self, index: u16) -> Result<()> {
        let used = self
            .used
            .get_mut(index as usize)
            .ok_or_else(|| error!(ZamaHostError::InvalidFheExecuteAccount))?;
        *used = true;
        Ok(())
    }

    /// Requires `authority` to have signed the execution: as the default context signer, or as a
    /// signing remaining account (which is then marked used). Anything else — the authority
    /// absent, or present without signing — is a refusal.
    pub(super) fn mark_signer(&mut self, authority: Pubkey, default_signer: Pubkey) -> Result<()> {
        if authority == default_signer {
            return Ok(());
        }
        let index = self
            .accounts
            .iter()
            .position(|account| account.key() == authority && account.is_signer)
            .ok_or_else(|| error!(ZamaHostError::EncryptedStateAccountAuthorityMismatch))?;
        self.used[index] = true;
        Ok(())
    }

    /// Locates the deny record for `app` by its canonical derived address
    /// (never by caller-supplied index). `Ok(None)` when the deny list is
    /// disabled; missing record under an enabled list fails the execution.
    pub(super) fn deny_record(
        &self,
        deny_list_enabled: bool,
        app: AppScope,
    ) -> Result<Option<&'a AccountInfo<'info>>> {
        if !deny_list_enabled {
            return Ok(None);
        }
        let (expected, _) = deny_scope_address(app);
        self.accounts
            .iter()
            .find(|account| account.key() == expected)
            .map(Some)
            .ok_or_else(|| error!(ZamaHostError::DenyRecordMissing))
    }

    /// Preflight marking twin of [`Self::deny_record`].
    pub(super) fn mark_deny_record(
        &mut self,
        deny_list_enabled: bool,
        app: AppScope,
    ) -> Result<()> {
        if !deny_list_enabled {
            return Ok(());
        }
        let (expected, _) = deny_scope_address(app);
        let Some(index) = self
            .accounts
            .iter()
            .position(|account| account.key() == expected)
        else {
            return Err(error!(ZamaHostError::DenyRecordMissing));
        };
        self.used[index] = true;
        Ok(())
    }

    /// Whole-execution hygiene: every passed account must have been referenced by
    /// the execution (as operand, output, authority, or deny record).
    pub(super) fn assert_all_used(&self) -> Result<()> {
        require!(
            self.used.iter().all(|used| *used),
            ZamaHostError::InvalidFheExecuteAccount
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_account(key: Pubkey) -> AccountInfo<'static> {
        let key = Box::leak(Box::new(key));
        let lamports = Box::leak(Box::new(0));
        let data = Box::leak(Vec::new().into_boxed_slice());
        let owner = Box::leak(Box::new(System::id()));
        AccountInfo::new(key, false, false, lamports, data, owner, false)
    }

    #[test]
    fn construction_rejects_duplicate_keys() {
        let duplicate = Pubkey::new_unique();
        let accounts = vec![test_account(duplicate), test_account(duplicate)];
        assert!(ExecutionAccountTable::new(&accounts).is_err());
    }

    #[test]
    fn unmarked_account_fails_all_used() {
        let accounts = vec![test_account(Pubkey::new_unique())];
        let table = ExecutionAccountTable::new(&accounts).unwrap();
        assert!(table.assert_all_used().is_err());
    }

    #[test]
    fn marked_accounts_pass_all_used_and_out_of_range_rejects() {
        let accounts = vec![test_account(Pubkey::new_unique())];
        let mut table = ExecutionAccountTable::new(&accounts).unwrap();
        assert!(table.mark(1).is_err());
        assert!(table.account(1).is_err());
        table.mark(0).unwrap();
        table.assert_all_used().unwrap();
    }

    fn canonical_state_account(tag: u8) -> (AccountInfo<'static>, EncryptedState) {
        let mut state = EncryptedState {
            program: Pubkey::new_unique(),
            authority: Pubkey::new_unique(),
            scope: [tag; 32],
            slots: vec![EncryptedSlot {
                key: [tag; 32],
                handle: [tag; 32],
            }],
            leaf_count: 0,
            peaks: Vec::new(),
            bump: 0,
        };
        let (key, bump) = state.canonical_address();
        state.bump = bump;
        let mut data = Vec::new();
        state.try_serialize(&mut data).expect("serializes");
        let info = AccountInfo::new(
            Box::leak(Box::new(key)),
            false,
            true,
            Box::leak(Box::new(0)),
            Box::leak(data.into_boxed_slice()),
            Box::leak(Box::new(crate::ID)),
            false,
        );
        (info, state)
    }

    #[test]
    fn state_read_is_decode_once_within_an_execution() {
        let (info, original) = canonical_state_account(7);
        let accounts = vec![info];
        let mut table = ExecutionAccountTable::new(&accounts).unwrap();

        assert_eq!(table.state(0).unwrap().slots, original.slots);

        let mut rewritten = original.clone();
        rewritten.slots[0].handle = [9; 32];
        let mut fresh_bytes = Vec::new();
        rewritten
            .try_serialize(&mut fresh_bytes)
            .expect("serializes");
        accounts[0]
            .try_borrow_mut_data()
            .unwrap()
            .copy_from_slice(&fresh_bytes);

        assert_eq!(table.state(0).unwrap().slots, original.slots);
    }

    #[test]
    fn state_mut_requires_a_writable_canonical_state() {
        let (info, original) = canonical_state_account(3);
        let accounts = vec![info];
        let mut table = ExecutionAccountTable::new(&accounts).unwrap();
        table.state_mut(0).unwrap().slots[0].handle = [8; 32];
        assert_eq!(table.state(0).unwrap().slots[0].handle, [8; 32]);
        assert_ne!(table.state(0).unwrap().slots, original.slots);
    }
}
