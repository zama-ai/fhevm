//! Dynamic-account bookkeeping for lowered executions.
//!
//! Public API surface: app programs. An app that assembles its own account list reads these
//! predicates to decide which accounts a lowered execution still needs, so they stay exported even
//! where this repository's own programs let the CPI helper do it.

use anchor_lang::prelude::Pubkey;

#[cfg(feature = "cpi")]
use anchor_lang::{prelude::AccountInfo, Key};

#[cfg(feature = "cpi")]
use crate::execution::FheExecution;

/// Why an execution needs a dynamic account.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionAccountPurpose {
    PersistentInputAcl,
    PersistentOutputAcl,
    /// A persistent value's authority that is not the execution's fixed CPI signer; it signs
    /// for every read and write of that value.
    PersistentValueAuthority,
}

/// At most one of each [`ExecutionAccountPurpose`]. Three variants, so a stack array — never a
/// heap allocation, never a tally site.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PurposeList {
    items: [ExecutionAccountPurpose; 3],
    len: u8,
}

impl PartialEq for PurposeList {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for PurposeList {}

impl PurposeList {
    pub(crate) fn one(purpose: ExecutionAccountPurpose) -> Self {
        Self {
            items: [purpose; 3],
            len: 1,
        }
    }

    pub(crate) fn as_slice(&self) -> &[ExecutionAccountPurpose] {
        &self.items[..self.len as usize]
    }

    pub(crate) fn len(&self) -> usize {
        self.len as usize
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub(crate) fn truncate(&mut self, len: usize) {
        debug_assert!(len <= self.len as usize);
        self.len = len as u8;
    }

    pub(crate) fn contains(&self, purpose: ExecutionAccountPurpose) -> bool {
        self.as_slice().contains(&purpose)
    }

    pub(crate) fn try_insert(&mut self, purpose: ExecutionAccountPurpose) -> bool {
        if self.contains(purpose) {
            return false;
        }
        debug_assert!(self.len < 3);
        self.items[self.len as usize] = purpose;
        self.len += 1;
        true
    }

    pub(crate) fn requires_dynamic_account(&self) -> bool {
        self.as_slice()
            .iter()
            .any(|purpose| *purpose != ExecutionAccountPurpose::PersistentValueAuthority)
    }

    pub(crate) fn requires_value_authority(&self) -> bool {
        self.contains(ExecutionAccountPurpose::PersistentValueAuthority)
    }
}

impl IntoIterator for PurposeList {
    type Item = ExecutionAccountPurpose;
    type IntoIter = std::iter::Take<std::array::IntoIter<ExecutionAccountPurpose, 3>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter().take(self.len as usize)
    }
}

/// Public view of one dynamic account required by an execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionAccountRequirement {
    pubkey: Pubkey,
    is_writable: bool,
    is_signer: bool,
    purposes: PurposeList,
}

impl ExecutionAccountRequirement {
    pub fn pubkey(&self) -> Pubkey {
        self.pubkey
    }

    pub fn is_writable(&self) -> bool {
        self.is_writable
    }

    pub fn is_signer(&self) -> bool {
        self.is_signer
    }

    pub fn purposes(&self) -> &[ExecutionAccountPurpose] {
        self.purposes.as_slice()
    }

    pub fn requires_dynamic_account(&self) -> bool {
        self.purposes.requires_dynamic_account()
    }

    pub fn requires_value_authority(&self) -> bool {
        self.purposes.requires_value_authority()
    }
}

/// Dynamic account role required by an execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExecutionAccountMeta {
    pub(crate) pubkey: Pubkey,
    pub(crate) is_writable: bool,
    pub(crate) is_signer: bool,
    pub(crate) purposes: PurposeList,
}

impl ExecutionAccountMeta {
    pub(crate) fn readonly(pubkey: Pubkey, purpose: ExecutionAccountPurpose) -> Self {
        Self {
            pubkey,
            is_writable: false,
            is_signer: false,
            purposes: PurposeList::one(purpose),
        }
    }

    pub(crate) fn writable(pubkey: Pubkey, purpose: ExecutionAccountPurpose) -> Self {
        Self {
            pubkey,
            is_writable: true,
            is_signer: false,
            purposes: PurposeList::one(purpose),
        }
    }

    pub(crate) fn readonly_signer(pubkey: Pubkey, purpose: ExecutionAccountPurpose) -> Self {
        Self {
            pubkey,
            is_writable: false,
            is_signer: true,
            purposes: PurposeList::one(purpose),
        }
    }

    pub(crate) fn requires_dynamic_account(&self) -> bool {
        self.purposes.requires_dynamic_account()
    }

    pub(crate) fn requires_value_authority(&self) -> bool {
        self.purposes.requires_value_authority()
    }
}

impl From<&ExecutionAccountMeta> for ExecutionAccountRequirement {
    fn from(meta: &ExecutionAccountMeta) -> Self {
        Self {
            pubkey: meta.pubkey,
            is_writable: meta.is_writable,
            is_signer: meta.is_signer,
            purposes: meta.purposes,
        }
    }
}

/// The encrypted value account authority that signs the fixed ZamaHost `fhe_execute` CPI account —
/// the execution-wide one, as opposed to the per-value authority an
/// [`ExecutionValueAuthorityRequirement`] names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionEncryptedValueAccountAuthority(Pubkey);

impl ExecutionEncryptedValueAccountAuthority {
    pub fn new(pubkey: Pubkey) -> Self {
        Self(pubkey)
    }

    pub fn pubkey(self) -> Pubkey {
        self.0
    }
}

/// A persistent value authority required to sign an execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionValueAuthorityRequirement {
    pub(crate) pubkey: Pubkey,
}

impl ExecutionValueAuthorityRequirement {
    pub fn pubkey(&self) -> Pubkey {
        self.pubkey
    }
}

/// Account-list resolution failure for an [`FheExecution`].
#[cfg(feature = "cpi")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionAccountResolutionError {
    /// The same dynamic account pubkey was supplied more than once.
    DuplicateDynamicAccount { pubkey: Pubkey },
    /// A supplied dynamic account is not required by this execution's non-authority
    /// remaining-account slots.
    UnexpectedDynamicAccount { pubkey: Pubkey },
    /// A non-authority remaining-account slot could not be resolved.
    MissingDynamicAccount {
        requirement: ExecutionAccountRequirement,
    },
    /// A writable remaining-account slot was supplied as readonly.
    DynamicAccountNotWritable {
        requirement: ExecutionAccountRequirement,
    },
    /// The same value authority witness was supplied more than once.
    DuplicateValueAuthority { pubkey: Pubkey },
    /// A supplied value authority is not required by this execution.
    UnexpectedValueAuthority { pubkey: Pubkey },
    /// A required value authority witness could not be resolved.
    MissingValueAuthority {
        authority: ExecutionValueAuthorityRequirement,
    },
}

#[cfg(feature = "cpi")]
impl ExecutionAccountResolutionError {
    pub fn pubkey(&self) -> Pubkey {
        match self {
            Self::DuplicateDynamicAccount { pubkey }
            | Self::UnexpectedDynamicAccount { pubkey }
            | Self::DuplicateValueAuthority { pubkey }
            | Self::UnexpectedValueAuthority { pubkey } => *pubkey,
            Self::MissingDynamicAccount { requirement }
            | Self::DynamicAccountNotWritable { requirement } => requirement.pubkey(),
            Self::MissingValueAuthority { authority } => authority.pubkey(),
        }
    }
}

/// Ordered dynamic accounts resolved from an [`FheExecution`].
#[cfg(feature = "cpi")]
#[derive(Debug)]
pub struct ResolvedExecutionAccounts<'info> {
    accounts: Vec<AccountInfo<'info>>,
}

#[cfg(feature = "cpi")]
impl<'info> ResolvedExecutionAccounts<'info> {
    pub fn account_infos(&self) -> &[AccountInfo<'info>] {
        &self.accounts
    }

    pub fn resolve(&self, pubkey: Pubkey) -> Option<AccountInfo<'info>> {
        self.accounts
            .iter()
            .find(|candidate| candidate.key() == pubkey)
            .cloned()
    }
}

#[cfg(feature = "cpi")]
pub(crate) fn resolve_execution_accounts<'info>(
    execution: &FheExecution,
    dynamic_accounts: impl IntoIterator<Item = AccountInfo<'info>>,
    value_authorities: impl IntoIterator<Item = AccountInfo<'info>>,
) -> std::result::Result<ResolvedExecutionAccounts<'info>, ExecutionAccountResolutionError> {
    // Collected into tables sized from the counts the execution itself requires (a successful
    // resolution supplies exactly those), never from the caller's iterator hint: on the
    // never-freeing bump heap these tables are part of what `build()` admitted against the
    // budget, so their size has to be the exact function of shape that
    // `FheExecutionCost::invoke_heap_bytes` charged.
    let mut dynamic_accounts_table = Vec::with_capacity(execution.cost.dynamic_accounts);
    dynamic_accounts_table.extend(dynamic_accounts);
    let dynamic_accounts = dynamic_accounts_table;
    let mut value_authorities_table = Vec::with_capacity(execution.cost.value_authorities);
    value_authorities_table.extend(value_authorities);
    let value_authorities = value_authorities_table;

    for (index, account) in dynamic_accounts.iter().enumerate() {
        let pubkey = account.key();
        if dynamic_accounts[index + 1..]
            .iter()
            .any(|candidate| candidate.key() == pubkey)
        {
            return Err(ExecutionAccountResolutionError::DuplicateDynamicAccount { pubkey });
        }
        // Read off the stored metas directly: the requirement view is the public API, and
        // looking it up per comparison would rebuild a PurposeList for every candidate.
        let Some(required) = execution
            .remaining_accounts
            .iter()
            .find(|meta| meta.pubkey == pubkey)
        else {
            return Err(ExecutionAccountResolutionError::UnexpectedDynamicAccount { pubkey });
        };
        if !required.requires_dynamic_account() {
            return Err(ExecutionAccountResolutionError::UnexpectedDynamicAccount { pubkey });
        }
    }

    for (index, authority) in value_authorities.iter().enumerate() {
        let pubkey = authority.key();
        if value_authorities[index + 1..]
            .iter()
            .any(|candidate| candidate.key() == pubkey)
        {
            return Err(ExecutionAccountResolutionError::DuplicateValueAuthority { pubkey });
        }
        if !execution
            .value_authorities()
            .any(|required| required == pubkey)
        {
            return Err(ExecutionAccountResolutionError::UnexpectedValueAuthority { pubkey });
        }
    }

    for authority in execution.value_authority_requirements() {
        if !value_authorities
            .iter()
            .any(|candidate| candidate.key() == authority.pubkey())
        {
            return Err(ExecutionAccountResolutionError::MissingValueAuthority { authority });
        }
    }

    // One resolved slot per remaining account (validation above rejected anything else), and
    // requirement views are built only on error paths, where their clone aborts the
    // instruction anyway.
    let mut accounts = Vec::with_capacity(execution.remaining_accounts.len());
    for required in &execution.remaining_accounts {
        let account = if required.requires_value_authority() {
            value_authorities
                .iter()
                .find(|candidate| candidate.key() == required.pubkey)
                .cloned()
                .ok_or(ExecutionAccountResolutionError::MissingValueAuthority {
                    authority: ExecutionValueAuthorityRequirement {
                        pubkey: required.pubkey,
                    },
                })?
        } else if required.requires_dynamic_account() {
            dynamic_accounts
                .iter()
                .find(|candidate| candidate.key() == required.pubkey)
                .cloned()
                .ok_or_else(|| ExecutionAccountResolutionError::MissingDynamicAccount {
                    requirement: ExecutionAccountRequirement::from(required),
                })?
        } else {
            continue;
        };
        if required.is_writable && !account.is_writable {
            return Err(ExecutionAccountResolutionError::DynamicAccountNotWritable {
                requirement: ExecutionAccountRequirement::from(required),
            });
        }
        accounts.push(account);
    }

    Ok(ResolvedExecutionAccounts { accounts })
}
