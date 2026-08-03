//! Dynamic-account bookkeeping for lowered batches.
//!
//! Public API surface: app programs. An app that assembles its own account list reads these
//! predicates to decide which accounts a lowered batch still needs, so they stay exported even
//! where this repository's own programs let the CPI helper do it.

use anchor_lang::prelude::Pubkey;

#[cfg(feature = "cpi")]
use anchor_lang::{prelude::AccountInfo, Key};

#[cfg(feature = "cpi")]
use crate::batch::Batch;

/// Why a batch needs a dynamic account.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchAccountPurpose {
    PersistentInputAcl,
    PersistentOutputAcl,
    PersistentOutputAuthority,
}

/// Public view of one dynamic account required by a batch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchAccountRequirement {
    pubkey: Pubkey,
    is_writable: bool,
    is_signer: bool,
    purposes: Vec<BatchAccountPurpose>,
}

impl BatchAccountRequirement {
    pub fn pubkey(&self) -> Pubkey {
        self.pubkey
    }

    pub fn is_writable(&self) -> bool {
        self.is_writable
    }

    pub fn is_signer(&self) -> bool {
        self.is_signer
    }

    pub fn has_purpose(&self, purpose: BatchAccountPurpose) -> bool {
        self.purposes.contains(&purpose)
    }

    pub fn purposes(&self) -> &[BatchAccountPurpose] {
        &self.purposes
    }

    pub fn requires_dynamic_account(&self) -> bool {
        self.purposes
            .iter()
            .any(|purpose| *purpose != BatchAccountPurpose::PersistentOutputAuthority)
    }

    pub fn requires_output_authority(&self) -> bool {
        self.has_purpose(BatchAccountPurpose::PersistentOutputAuthority)
    }
}

/// Dynamic account role required by a batch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BatchAccountMeta {
    pub(crate) pubkey: Pubkey,
    pub(crate) is_writable: bool,
    pub(crate) is_signer: bool,
    pub(crate) purposes: Vec<BatchAccountPurpose>,
}

impl BatchAccountMeta {
    pub(crate) fn readonly(pubkey: Pubkey, purpose: BatchAccountPurpose) -> Self {
        Self {
            pubkey,
            is_writable: false,
            is_signer: false,
            purposes: vec![purpose],
        }
    }

    pub(crate) fn writable(pubkey: Pubkey, purpose: BatchAccountPurpose) -> Self {
        Self {
            pubkey,
            is_writable: true,
            is_signer: false,
            purposes: vec![purpose],
        }
    }

    pub(crate) fn readonly_signer(pubkey: Pubkey, purpose: BatchAccountPurpose) -> Self {
        Self {
            pubkey,
            is_writable: false,
            is_signer: true,
            purposes: vec![purpose],
        }
    }

    pub(crate) fn promote(&mut self, required: Self) {
        self.is_writable |= required.is_writable;
        self.is_signer |= required.is_signer;
        for purpose in required.purposes {
            if !self.purposes.contains(&purpose) {
                self.purposes.push(purpose);
            }
        }
    }
}

impl From<&BatchAccountMeta> for BatchAccountRequirement {
    fn from(meta: &BatchAccountMeta) -> Self {
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
pub struct BatchAppAuthority(Pubkey);

impl BatchAppAuthority {
    pub fn new(pubkey: Pubkey) -> Self {
        Self(pubkey)
    }

    pub fn pubkey(self) -> Pubkey {
        self.0
    }
}

/// Output authority required by a batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatchOutputAuthorityRequirement {
    pub(crate) pubkey: Pubkey,
    pub(crate) cpi_account_authority: bool,
}

impl BatchOutputAuthorityRequirement {
    pub fn pubkey(&self) -> Pubkey {
        self.pubkey
    }
}

/// Account-list resolution failure for an [`Batch`].
#[cfg(feature = "cpi")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchAccountResolutionError {
    /// The same dynamic account pubkey was supplied more than once.
    DuplicateDynamicAccount { pubkey: Pubkey },
    /// A supplied dynamic account is not required by this batch's non-authority
    /// remaining-account slots.
    UnexpectedDynamicAccount { pubkey: Pubkey },
    /// A non-authority remaining-account slot could not be resolved.
    MissingDynamicAccount {
        requirement: BatchAccountRequirement,
    },
    /// A writable remaining-account slot was supplied as readonly.
    DynamicAccountNotWritable {
        requirement: BatchAccountRequirement,
    },
    /// The same persistent output authority witness was supplied more than once.
    DuplicateOutputAuthority { pubkey: Pubkey },
    /// A supplied output authority is not required by this batch.
    UnexpectedOutputAuthority { pubkey: Pubkey },
    /// A required persistent output authority witness could not be resolved.
    MissingOutputAuthority {
        authority: BatchOutputAuthorityRequirement,
    },
}

#[cfg(feature = "cpi")]
impl BatchAccountResolutionError {
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

/// Ordered dynamic accounts resolved from an [`Batch`].
#[cfg(feature = "cpi")]
#[derive(Debug)]
pub struct ResolvedBatchAccounts<'info> {
    accounts: Vec<AccountInfo<'info>>,
}

#[cfg(feature = "cpi")]
impl<'info> ResolvedBatchAccounts<'info> {
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
pub(crate) fn resolve_batch_accounts<'info>(
    batch: &Batch,
    dynamic_accounts: impl IntoIterator<Item = AccountInfo<'info>>,
    output_authorities: impl IntoIterator<Item = AccountInfo<'info>>,
) -> std::result::Result<ResolvedBatchAccounts<'info>, BatchAccountResolutionError> {
    let dynamic_accounts = dynamic_accounts.into_iter().collect::<Vec<_>>();
    let output_authorities = output_authorities.into_iter().collect::<Vec<_>>();

    for (index, account) in dynamic_accounts.iter().enumerate() {
        let pubkey = account.key();
        if dynamic_accounts[index + 1..]
            .iter()
            .any(|candidate| candidate.key() == pubkey)
        {
            return Err(BatchAccountResolutionError::DuplicateDynamicAccount { pubkey });
        }
        let Some(required) = batch
            .dynamic_account_requirements()
            .find(|required| required.pubkey() == pubkey)
        else {
            return Err(BatchAccountResolutionError::UnexpectedDynamicAccount { pubkey });
        };
        if !required.requires_dynamic_account() {
            return Err(BatchAccountResolutionError::UnexpectedDynamicAccount { pubkey });
        }
    }

    for (index, authority) in output_authorities.iter().enumerate() {
        let pubkey = authority.key();
        if output_authorities[index + 1..]
            .iter()
            .any(|candidate| candidate.key() == pubkey)
        {
            return Err(BatchAccountResolutionError::DuplicateOutputAuthority { pubkey });
        }
        if !batch
            .output_authorities()
            .any(|required| required == pubkey)
        {
            return Err(BatchAccountResolutionError::UnexpectedOutputAuthority { pubkey });
        }
    }

    for authority in batch.output_authority_requirements() {
        if !output_authorities
            .iter()
            .any(|candidate| candidate.key() == authority.pubkey())
        {
            return Err(BatchAccountResolutionError::MissingOutputAuthority { authority });
        }
    }

    let mut accounts = Vec::new();
    for required in batch.dynamic_account_requirements() {
        let account = if required.requires_output_authority() {
            output_authorities
                .iter()
                .find(|candidate| candidate.key() == required.pubkey())
                .cloned()
                .ok_or(BatchAccountResolutionError::MissingOutputAuthority {
                    authority: BatchOutputAuthorityRequirement {
                        pubkey: required.pubkey(),
                        cpi_account_authority: false,
                    },
                })?
        } else if required.requires_dynamic_account() {
            dynamic_accounts
                .iter()
                .find(|candidate| candidate.key() == required.pubkey())
                .cloned()
                .ok_or_else(|| BatchAccountResolutionError::MissingDynamicAccount {
                    requirement: required.clone(),
                })?
        } else {
            continue;
        };
        if required.is_writable() && !account.is_writable {
            return Err(BatchAccountResolutionError::DynamicAccountNotWritable {
                requirement: required,
            });
        }
        accounts.push(account);
    }

    Ok(ResolvedBatchAccounts { accounts })
}
