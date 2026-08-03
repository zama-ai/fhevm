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
    PersistentOutputAuthority,
}

/// Public view of one dynamic account required by an execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionAccountRequirement {
    pubkey: Pubkey,
    is_writable: bool,
    is_signer: bool,
    purposes: Vec<ExecutionAccountPurpose>,
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

    pub fn has_purpose(&self, purpose: ExecutionAccountPurpose) -> bool {
        self.purposes.contains(&purpose)
    }

    pub fn purposes(&self) -> &[ExecutionAccountPurpose] {
        &self.purposes
    }

    pub fn requires_dynamic_account(&self) -> bool {
        self.purposes
            .iter()
            .any(|purpose| *purpose != ExecutionAccountPurpose::PersistentOutputAuthority)
    }

    pub fn requires_output_authority(&self) -> bool {
        self.has_purpose(ExecutionAccountPurpose::PersistentOutputAuthority)
    }
}

/// Dynamic account role required by an execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExecutionAccountMeta {
    pub(crate) pubkey: Pubkey,
    pub(crate) is_writable: bool,
    pub(crate) is_signer: bool,
    pub(crate) purposes: Vec<ExecutionAccountPurpose>,
}

impl ExecutionAccountMeta {
    pub(crate) fn readonly(pubkey: Pubkey, purpose: ExecutionAccountPurpose) -> Self {
        Self {
            pubkey,
            is_writable: false,
            is_signer: false,
            purposes: vec![purpose],
        }
    }

    pub(crate) fn writable(pubkey: Pubkey, purpose: ExecutionAccountPurpose) -> Self {
        Self {
            pubkey,
            is_writable: true,
            is_signer: false,
            purposes: vec![purpose],
        }
    }

    pub(crate) fn readonly_signer(pubkey: Pubkey, purpose: ExecutionAccountPurpose) -> Self {
        Self {
            pubkey,
            is_writable: false,
            is_signer: true,
            purposes: vec![purpose],
        }
    }

    /// Widens this entry so it also satisfies `required`, returning the record that undoes it.
    /// The record lives next to the mutation on purpose: it is only complete because `promote`
    /// does exactly two things — OR the flags and append purposes — so anything added here has to
    /// be added to [`ExecutionAccountMeta::demote`] in the same edit.
    pub(crate) fn promote(&mut self, required: Self) -> MetaPromotion {
        let undo = MetaPromotion {
            was_writable: self.is_writable,
            was_signer: self.is_signer,
            purposes_len: self.purposes.len(),
        };
        self.is_writable |= required.is_writable;
        self.is_signer |= required.is_signer;
        for purpose in required.purposes {
            if !self.purposes.contains(&purpose) {
                self.purposes.push(purpose);
            }
        }
        undo
    }

    /// Restores the entry to what it was before the [`MetaPromotion`] was taken.
    pub(crate) fn demote(&mut self, undo: MetaPromotion) {
        self.is_writable = undo.was_writable;
        self.is_signer = undo.was_signer;
        self.purposes.truncate(undo.purposes_len);
    }
}

/// What one [`ExecutionAccountMeta::promote`] changed, small enough to record without allocating.
#[derive(Debug)]
pub(crate) struct MetaPromotion {
    was_writable: bool,
    was_signer: bool,
    purposes_len: usize,
}

impl From<&ExecutionAccountMeta> for ExecutionAccountRequirement {
    fn from(meta: &ExecutionAccountMeta) -> Self {
        Self {
            pubkey: meta.pubkey,
            is_writable: meta.is_writable,
            is_signer: meta.is_signer,
            purposes: meta.purposes.clone(),
        }
    }
}

/// App authority that signs the fixed ZamaHost fhe_execute CPI account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionAppAuthority(Pubkey);

impl ExecutionAppAuthority {
    pub fn new(pubkey: Pubkey) -> Self {
        Self(pubkey)
    }

    pub fn pubkey(self) -> Pubkey {
        self.0
    }
}

/// Output authority required by an execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionOutputAuthorityRequirement {
    pub(crate) pubkey: Pubkey,
    pub(crate) cpi_account_authority: bool,
}

impl ExecutionOutputAuthorityRequirement {
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
    /// The same persistent output authority witness was supplied more than once.
    DuplicateOutputAuthority { pubkey: Pubkey },
    /// A supplied output authority is not required by this execution.
    UnexpectedOutputAuthority { pubkey: Pubkey },
    /// A required persistent output authority witness could not be resolved.
    MissingOutputAuthority {
        authority: ExecutionOutputAuthorityRequirement,
    },
}

#[cfg(feature = "cpi")]
impl ExecutionAccountResolutionError {
    pub fn pubkey(&self) -> Pubkey {
        match self {
            Self::DuplicateDynamicAccount { pubkey }
            | Self::UnexpectedDynamicAccount { pubkey }
            | Self::DuplicateOutputAuthority { pubkey }
            | Self::UnexpectedOutputAuthority { pubkey } => *pubkey,
            Self::MissingDynamicAccount { requirement }
            | Self::DynamicAccountNotWritable { requirement } => requirement.pubkey(),
            Self::MissingOutputAuthority { authority } => authority.pubkey(),
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
    output_authorities: impl IntoIterator<Item = AccountInfo<'info>>,
) -> std::result::Result<ResolvedExecutionAccounts<'info>, ExecutionAccountResolutionError> {
    let dynamic_accounts = dynamic_accounts.into_iter().collect::<Vec<_>>();
    let output_authorities = output_authorities.into_iter().collect::<Vec<_>>();

    for (index, account) in dynamic_accounts.iter().enumerate() {
        let pubkey = account.key();
        if dynamic_accounts[index + 1..]
            .iter()
            .any(|candidate| candidate.key() == pubkey)
        {
            return Err(ExecutionAccountResolutionError::DuplicateDynamicAccount { pubkey });
        }
        let Some(required) = execution
            .dynamic_account_requirements()
            .find(|required| required.pubkey() == pubkey)
        else {
            return Err(ExecutionAccountResolutionError::UnexpectedDynamicAccount { pubkey });
        };
        if !required.requires_dynamic_account() {
            return Err(ExecutionAccountResolutionError::UnexpectedDynamicAccount { pubkey });
        }
    }

    for (index, authority) in output_authorities.iter().enumerate() {
        let pubkey = authority.key();
        if output_authorities[index + 1..]
            .iter()
            .any(|candidate| candidate.key() == pubkey)
        {
            return Err(ExecutionAccountResolutionError::DuplicateOutputAuthority { pubkey });
        }
        if !execution
            .output_authorities()
            .any(|required| required == pubkey)
        {
            return Err(ExecutionAccountResolutionError::UnexpectedOutputAuthority { pubkey });
        }
    }

    for authority in execution.output_authority_requirements() {
        if !output_authorities
            .iter()
            .any(|candidate| candidate.key() == authority.pubkey())
        {
            return Err(ExecutionAccountResolutionError::MissingOutputAuthority { authority });
        }
    }

    let mut accounts = Vec::new();
    for required in execution.dynamic_account_requirements() {
        let account = if required.requires_output_authority() {
            output_authorities
                .iter()
                .find(|candidate| candidate.key() == required.pubkey())
                .cloned()
                .ok_or(ExecutionAccountResolutionError::MissingOutputAuthority {
                    authority: ExecutionOutputAuthorityRequirement {
                        pubkey: required.pubkey(),
                        cpi_account_authority: false,
                    },
                })?
        } else if required.requires_dynamic_account() {
            dynamic_accounts
                .iter()
                .find(|candidate| candidate.key() == required.pubkey())
                .cloned()
                .ok_or_else(|| ExecutionAccountResolutionError::MissingDynamicAccount {
                    requirement: required.clone(),
                })?
        } else {
            continue;
        };
        if required.is_writable() && !account.is_writable {
            return Err(ExecutionAccountResolutionError::DynamicAccountNotWritable {
                requirement: required,
            });
        }
        accounts.push(account);
    }

    Ok(ResolvedExecutionAccounts { accounts })
}
