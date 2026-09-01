//! Single owner of every remaining-accounts safety invariant for one
//! `fhe_execute` execution.
//!
//! Construction rejects duplicate account keys. Preflight marks every account
//! the execution references and [`ExecutionAccountTable::assert_all_used`] rejects
//! dangling accounts before any pass touches state, so later passes access by
//! index without their own bookkeeping. Persistent-output claims (one write per
//! account per execution), deny-record location by canonical derived address, and
//! output-PDA derivation also live here, so the invariants of the execution's
//! account handling exist in exactly one place.

use super::*;

pub(super) struct ExecutionAccountTable<'a, 'info> {
    accounts: &'a [AccountInfo<'info>],
    used: Vec<bool>,
    /// Persistent output accounts already claimed by an earlier step. Reserved to
    /// the op cap up front: the SBF bump allocator never frees, so growth by
    /// doubling would leak, and the created-public maximum execution already runs
    /// close to the 32KB heap ceiling. (For the same reason the table caches
    /// no derived PDAs: the single walk derives each output PDA exactly once.)
    persistent_outputs_claimed: Vec<Pubkey>,
    /// Decode-once cache for canonical `EncryptedValue`s, parallel to `accounts`.
    /// Each decode allocates the full subject and peak vectors on the never-freeing
    /// bump heap, and one execution reads the same account from up to three places
    /// (anchor collection, operand admission, the output write). Boxed so an empty
    /// slot costs a pointer, not the ~200-byte struct: an unboxed
    /// `Vec<Option<EncryptedValue>>` sized to the account list measurably lowered
    /// the all-created-public heap boundary, which never decodes at all.
    decoded_encrypted_values: Vec<Option<Box<EncryptedValue>>>,
}

/// Result of deriving a persistent output's canonical address: the PDA, its bump,
/// and the encrypted value ID that seeds it (needed again as a signer seed on create).
#[derive(Clone, Copy)]
pub(super) struct OutputPda {
    pub key: Pubkey,
    pub bump: u8,
    pub encrypted_value_id: [u8; 32],
}

impl<'a, 'info> ExecutionAccountTable<'a, 'info> {
    /// Rejects duplicate keys up front so no index-referenced account can be
    /// validated as one role and used as another. The scan is quadratic; the
    /// bound is the transaction account limit (~64 keys today, `u16::MAX`
    /// under SIMD-0406-style extensions), so it stays trivial.
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
            used: vec![false; accounts.len()],
            persistent_outputs_claimed: Vec::with_capacity(MAX_FHE_EXECUTION_STEPS),
            decoded_encrypted_values: (0..accounts.len()).map(|_| None).collect(),
        })
    }

    /// Decode-once, validate-once read of a canonical `EncryptedValue`: the first read decodes
    /// through `read_canonical_encrypted_value`, later reads reuse it. Safe against staleness
    /// because within one execution every read of an account precedes its single write (the
    /// one-write claim plus the read-after-write operand rejection, `preflight.rs`), and the
    /// write path goes through [`Self::take_canonical_encrypted_value`], which empties the slot.
    /// The contract is pinned by `cached_read_survives_byte_changes_and_take_invalidates` below.
    pub(super) fn canonical_encrypted_value(&mut self, index: u16) -> Result<&EncryptedValue> {
        let account = self.account(index)?;
        let slot = self
            .decoded_encrypted_values
            .get_mut(index as usize)
            .ok_or_else(|| error!(ZamaHostError::InvalidFheExecuteAccount))?;
        if slot.is_none() {
            *slot = Some(Box::new(read_canonical_encrypted_value(account)?));
        }
        Ok(slot.as_deref().expect("slot was just filled"))
    }

    /// The write path's decode: takes the cached value (or decodes when nothing read the account
    /// earlier), transferring ownership to the caller who will mutate and rewrite it. Taking
    /// doubles as cache invalidation — after the write, a fresh read would decode the new state.
    pub(super) fn take_canonical_encrypted_value(&mut self, index: u16) -> Result<EncryptedValue> {
        let account = self.account(index)?;
        match self
            .decoded_encrypted_values
            .get_mut(index as usize)
            .and_then(Option::take)
        {
            Some(value) => Ok(*value),
            None => read_canonical_encrypted_value(account),
        }
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

    /// Locates the deny record for `subject` by its canonical derived address
    /// (never by caller-supplied index). `Ok(None)` when the deny list is
    /// disabled; missing record under an enabled list fails the execution.
    pub(super) fn deny_record(
        &self,
        deny_list_enabled: bool,
        subject: Pubkey,
    ) -> Result<Option<&'a AccountInfo<'info>>> {
        if !deny_list_enabled {
            return Ok(None);
        }
        let (expected, _) = deny_subject_address(subject);
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
        subject: Pubkey,
    ) -> Result<()> {
        if !deny_list_enabled {
            return Ok(());
        }
        let (expected, _) = deny_subject_address(subject);
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

    /// Derived canonical PDA for a persistent output's declaration inputs. The
    /// one place output-PDA derivation lives.
    pub(super) fn expected_output_pda(
        &self,
        domain: Pubkey,
        account: Pubkey,
        encrypted_value_label: [u8; 32],
    ) -> OutputPda {
        let encrypted_value_id = zama_solana_acl::derive_encrypted_value_id(
            domain.to_bytes(),
            account.to_bytes(),
            encrypted_value_label,
        );
        let (key, bump) = encrypted_value_address(encrypted_value_id);
        OutputPda {
            key,
            bump,
            encrypted_value_id,
        }
    }

    /// Claims a persistent output account for this execution; a second claim of the same account is
    /// rejected. One write per account per execution is what makes a rand seed unrepeatable (#1853 W4).
    pub(super) fn claim_persistent_output(&mut self, key: Pubkey) -> Result<()> {
        require!(
            !self.persistent_outputs_claimed.contains(&key),
            ZamaHostError::FheExecuteOutputAlreadyInitialized
        );
        self.persistent_outputs_claimed.push(key);
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

    /// A canonical encrypted value account: program-owned, PDA-addressed for its own
    /// `(domain, authority, label)` triple, with the discriminator `read_canonical_encrypted_value`
    /// checks. Returns the account and the value serialized into it.
    fn canonical_value_account(tag: u8) -> (AccountInfo<'static>, EncryptedValue) {
        let mut value = EncryptedValue {
            domain: Pubkey::new_unique(),
            encrypted_value_account_authority: Pubkey::new_unique(),
            label: [tag; 32],
            current_handle: [tag; 32],
            subjects: vec![Pubkey::new_unique()],
            leaf_count: 0,
            peaks: Vec::new(),
            bump: 0,
        };
        let (key, bump) = encrypted_value_address(value.encrypted_value_id());
        value.bump = bump;
        let mut data = Vec::new();
        value.try_serialize(&mut data).expect("serializes");
        let info = AccountInfo::new(
            Box::leak(Box::new(key)),
            false,
            true,
            Box::leak(Box::new(0)),
            Box::leak(data.into_boxed_slice()),
            Box::leak(Box::new(crate::ID)),
            false,
        );
        (info, value)
    }

    /// The cache contract the doc comments promise: repeated reads return the first decode even
    /// if the account bytes change underneath (within one execution every read of an account
    /// precedes its single write, so serving the cached decode is correct), and `take` both hands
    /// out that decode and empties the slot so the next read decodes the current bytes.
    #[test]
    fn cached_read_survives_byte_changes_and_take_invalidates() {
        let (info, original) = canonical_value_account(7);
        let accounts = vec![info];
        let mut table = ExecutionAccountTable::new(&accounts).unwrap();

        let first = table.canonical_encrypted_value(0).unwrap();
        assert_eq!(first.current_handle, original.current_handle);

        // Rewrite the account with a same-size value that differs only in its handle — still
        // canonical for the same PDA, so a fresh decode would return it and pass validation.
        // Only the cache distinguishes the two.
        let mut rewritten = original.clone();
        rewritten.current_handle = [9; 32];
        let mut fresh_bytes = Vec::new();
        rewritten
            .try_serialize(&mut fresh_bytes)
            .expect("serializes");
        accounts[0]
            .try_borrow_mut_data()
            .unwrap()
            .copy_from_slice(&fresh_bytes);

        let cached = table.canonical_encrypted_value(0).unwrap();
        assert_eq!(cached.current_handle, original.current_handle);

        let taken = table.take_canonical_encrypted_value(0).unwrap();
        assert_eq!(taken.current_handle, original.current_handle);

        let after_take = table.canonical_encrypted_value(0).unwrap();
        assert_eq!(after_take.current_handle, rewritten.current_handle);
    }

    #[test]
    fn take_without_prior_read_decodes_the_account() {
        let (info, original) = canonical_value_account(3);
        let accounts = vec![info];
        let mut table = ExecutionAccountTable::new(&accounts).unwrap();
        let taken = table.take_canonical_encrypted_value(0).unwrap();
        assert_eq!(taken.current_handle, original.current_handle);
        assert_eq!(taken.subjects, original.subjects);
    }

    #[test]
    fn second_claim_of_same_persistent_output_rejects() {
        let accounts: Vec<AccountInfo> = Vec::new();
        let mut table = ExecutionAccountTable::new(&accounts).unwrap();
        let output = Pubkey::new_unique();
        table.claim_persistent_output(output).unwrap();
        assert!(table.claim_persistent_output(output).is_err());
    }

    #[test]
    fn output_pda_derivation_is_input_bound() {
        let accounts: Vec<AccountInfo> = Vec::new();
        let table = ExecutionAccountTable::new(&accounts).unwrap();
        let domain = Pubkey::new_unique();
        let app = Pubkey::new_unique();
        let first = table.expected_output_pda(domain, app, [1; 32]);
        // Identical declaration inputs derive the identical address.
        let again = table.expected_output_pda(domain, app, [1; 32]);
        assert_eq!(first.key, again.key);
        // Any changed declaration input derives a different address.
        let other = table.expected_output_pda(domain, app, [2; 32]);
        assert_ne!(first.key, other.key);
    }
}
