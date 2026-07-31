//! Single owner of every remaining-accounts safety invariant for one
//! `fhe_eval` frame.
//!
//! Construction rejects duplicate account keys. Preflight marks every account
//! the plan references and [`EvalAccountTable::assert_all_used`] rejects
//! dangling accounts before any pass touches state, so later passes access by
//! index without their own bookkeeping. Persistent-output claims (one write per
//! account per frame), deny-record location by canonical derived address, and
//! output-PDA derivation also live here, so the invariants of the frame's
//! account handling exist in exactly one place.

use super::*;

pub(super) struct EvalAccountTable<'a, 'info> {
    accounts: &'a [AccountInfo<'info>],
    used: Vec<bool>,
    /// Persistent output accounts already claimed by an earlier step. Reserved to
    /// the op cap up front: the SBF bump allocator never frees, so growth by
    /// doubling would leak, and the created-public maximum frame already runs
    /// close to the 32KB heap ceiling. (For the same reason the table caches
    /// no derived PDAs: the single walk derives each output PDA exactly once.)
    persistent_outputs_claimed: Vec<Pubkey>,
}

/// Result of deriving a persistent output's canonical address: the PDA, its bump,
/// and the value key that seeds it (needed again as a signer seed on create).
#[derive(Clone, Copy)]
pub(super) struct OutputPda {
    pub key: Pubkey,
    pub bump: u8,
    pub value_key: [u8; 32],
}

impl<'a, 'info> EvalAccountTable<'a, 'info> {
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
                ZamaHostError::InvalidFheEvalAccount
            );
        }
        Ok(Self {
            accounts,
            used: vec![false; accounts.len()],
            persistent_outputs_claimed: Vec::with_capacity(MAX_FHE_EVAL_OPS),
        })
    }

    pub(super) fn account(&self, index: u16) -> Result<&'a AccountInfo<'info>> {
        self.accounts
            .get(index as usize)
            .ok_or_else(|| error!(ZamaHostError::InvalidFheEvalAccount))
    }

    pub(super) fn mark(&mut self, index: u16) -> Result<()> {
        let used = self
            .used
            .get_mut(index as usize)
            .ok_or_else(|| error!(ZamaHostError::InvalidFheEvalAccount))?;
        *used = true;
        Ok(())
    }

    /// Locates the deny record for `subject` by its canonical derived address
    /// (never by caller-supplied index). `Ok(None)` when the deny list is
    /// disabled; missing record under an enabled list fails the frame.
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
            .ok_or_else(|| error!(ZamaHostError::AclDenyRecordMissing))
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
            return Err(error!(ZamaHostError::AclDenyRecordMissing));
        };
        self.used[index] = true;
        Ok(())
    }

    /// Derived canonical PDA for a persistent output's declaration inputs. The
    /// one place output-PDA derivation lives.
    pub(super) fn expected_output_pda(
        &self,
        acl_domain_key: Pubkey,
        app_account: Pubkey,
        label: [u8; 32],
    ) -> OutputPda {
        let value_key = zama_solana_acl::derive_value_key(
            acl_domain_key.to_bytes(),
            app_account.to_bytes(),
            label,
        );
        let (key, bump) = encrypted_value_address(value_key);
        OutputPda {
            key,
            bump,
            value_key,
        }
    }

    /// Claims a persistent output account for this frame; a second claim of the
    /// same account is rejected (one write per account per frame — load-bearing
    /// for the rand seed anchor, see #1853 W4).
    pub(super) fn claim_persistent_output(&mut self, key: Pubkey) -> Result<()> {
        require!(
            !self.persistent_outputs_claimed.contains(&key),
            ZamaHostError::FheEvalOutputAlreadyInitialized
        );
        self.persistent_outputs_claimed.push(key);
        Ok(())
    }

    /// Whole-frame hygiene: every passed account must have been referenced by
    /// the plan (as operand, output, authority, or deny record).
    pub(super) fn assert_all_used(&self) -> Result<()> {
        require!(
            self.used.iter().all(|used| *used),
            ZamaHostError::InvalidFheEvalAccount
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
        assert!(EvalAccountTable::new(&accounts).is_err());
    }

    #[test]
    fn unmarked_account_fails_all_used() {
        let accounts = vec![test_account(Pubkey::new_unique())];
        let table = EvalAccountTable::new(&accounts).unwrap();
        assert!(table.assert_all_used().is_err());
    }

    #[test]
    fn marked_accounts_pass_all_used_and_out_of_range_rejects() {
        let accounts = vec![test_account(Pubkey::new_unique())];
        let mut table = EvalAccountTable::new(&accounts).unwrap();
        assert!(table.mark(1).is_err());
        assert!(table.account(1).is_err());
        table.mark(0).unwrap();
        table.assert_all_used().unwrap();
    }

    #[test]
    fn second_claim_of_same_persistent_output_rejects() {
        let accounts: Vec<AccountInfo> = Vec::new();
        let mut table = EvalAccountTable::new(&accounts).unwrap();
        let output = Pubkey::new_unique();
        table.claim_persistent_output(output).unwrap();
        assert!(table.claim_persistent_output(output).is_err());
    }

    #[test]
    fn output_pda_derivation_is_input_bound() {
        let accounts: Vec<AccountInfo> = Vec::new();
        let table = EvalAccountTable::new(&accounts).unwrap();
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
