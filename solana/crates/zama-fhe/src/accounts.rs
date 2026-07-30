//! Dynamic-account bookkeeping for lowered eval plans.

use anchor_lang::prelude::Pubkey;

#[cfg(feature = "cpi")]
use anchor_lang::{prelude::AccountInfo, Key};

#[cfg(feature = "cpi")]
use crate::plan::EvalPlan;

/// Why an eval plan needs a dynamic account.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalAccountPurpose {
    DurableInputAcl,
    DurableOutputAcl,
    DurableOutputAuthority,
}

/// Public view of one dynamic account required by an eval plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalAccountRequirement {
    pubkey: Pubkey,
    is_writable: bool,
    is_signer: bool,
    purposes: Vec<EvalAccountPurpose>,
}

impl EvalAccountRequirement {
    pub fn pubkey(&self) -> Pubkey {
        self.pubkey
    }

    pub fn is_writable(&self) -> bool {
        self.is_writable
    }

    pub fn is_signer(&self) -> bool {
        self.is_signer
    }

    pub fn has_purpose(&self, purpose: EvalAccountPurpose) -> bool {
        self.purposes.contains(&purpose)
    }

    pub fn purposes(&self) -> &[EvalAccountPurpose] {
        &self.purposes
    }

    pub fn requires_dynamic_account(&self) -> bool {
        self.purposes
            .iter()
            .any(|purpose| *purpose != EvalAccountPurpose::DurableOutputAuthority)
    }

    pub fn requires_output_authority(&self) -> bool {
        self.has_purpose(EvalAccountPurpose::DurableOutputAuthority)
    }
}

/// Dynamic account role required by an eval plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EvalAccountMeta {
    pub(crate) pubkey: Pubkey,
    pub(crate) is_writable: bool,
    pub(crate) is_signer: bool,
    pub(crate) purposes: Vec<EvalAccountPurpose>,
}

impl EvalAccountMeta {
    pub(crate) fn readonly(pubkey: Pubkey, purpose: EvalAccountPurpose) -> Self {
        Self {
            pubkey,
            is_writable: false,
            is_signer: false,
            purposes: vec![purpose],
        }
    }

    pub(crate) fn writable(pubkey: Pubkey, purpose: EvalAccountPurpose) -> Self {
        Self {
            pubkey,
            is_writable: true,
            is_signer: false,
            purposes: vec![purpose],
        }
    }

    pub(crate) fn readonly_signer(pubkey: Pubkey, purpose: EvalAccountPurpose) -> Self {
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

impl From<&EvalAccountMeta> for EvalAccountRequirement {
    fn from(meta: &EvalAccountMeta) -> Self {
        Self {
            pubkey: meta.pubkey,
            is_writable: meta.is_writable,
            is_signer: meta.is_signer,
            purposes: meta.purposes.clone(),
        }
    }
}

/// App authority that signs the fixed ZamaHost eval CPI account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EvalAppAuthority(Pubkey);

impl EvalAppAuthority {
    pub fn new(pubkey: Pubkey) -> Self {
        Self(pubkey)
    }

    pub fn pubkey(self) -> Pubkey {
        self.0
    }
}

/// Output authority required by an eval plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvalOutputAuthorityRequirement {
    pub(crate) pubkey: Pubkey,
    pub(crate) cpi_account_authority: bool,
}

impl EvalOutputAuthorityRequirement {
    pub fn pubkey(&self) -> Pubkey {
        self.pubkey
    }

    pub fn signs_cpi_account(&self) -> bool {
        self.cpi_account_authority
    }
}

/// Account-list resolution failure for an [`EvalPlan`].
#[cfg(feature = "cpi")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalAccountResolutionError {
    /// The same dynamic account pubkey was supplied more than once.
    DuplicateDynamicAccount { pubkey: Pubkey },
    /// A supplied dynamic account is not required by this plan's non-authority
    /// remaining-account slots.
    UnexpectedDynamicAccount { pubkey: Pubkey },
    /// A non-authority remaining-account slot could not be resolved.
    MissingDynamicAccount { requirement: EvalAccountRequirement },
    /// A writable remaining-account slot was supplied as readonly.
    DynamicAccountNotWritable { requirement: EvalAccountRequirement },
    /// The same durable output authority witness was supplied more than once.
    DuplicateOutputAuthority { pubkey: Pubkey },
    /// A supplied output authority is not required by this plan.
    UnexpectedOutputAuthority { pubkey: Pubkey },
    /// A required durable output authority witness could not be resolved.
    MissingOutputAuthority {
        authority: EvalOutputAuthorityRequirement,
    },
}

#[cfg(feature = "cpi")]
impl EvalAccountResolutionError {
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

/// Ordered dynamic accounts resolved from an [`EvalPlan`].
#[cfg(feature = "cpi")]
#[derive(Debug)]
pub struct ResolvedEvalAccounts<'info> {
    accounts: Vec<AccountInfo<'info>>,
}

#[cfg(feature = "cpi")]
impl<'info> ResolvedEvalAccounts<'info> {
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
pub(crate) fn resolve_eval_accounts<'info>(
    plan: &EvalPlan,
    dynamic_accounts: impl IntoIterator<Item = AccountInfo<'info>>,
    output_authorities: impl IntoIterator<Item = AccountInfo<'info>>,
) -> std::result::Result<ResolvedEvalAccounts<'info>, EvalAccountResolutionError> {
    let dynamic_accounts = dynamic_accounts.into_iter().collect::<Vec<_>>();
    let output_authorities = output_authorities.into_iter().collect::<Vec<_>>();

    for (index, account) in dynamic_accounts.iter().enumerate() {
        let pubkey = account.key();
        if dynamic_accounts[index + 1..]
            .iter()
            .any(|candidate| candidate.key() == pubkey)
        {
            return Err(EvalAccountResolutionError::DuplicateDynamicAccount { pubkey });
        }
        let Some(required) = plan
            .dynamic_account_requirements()
            .find(|required| required.pubkey() == pubkey)
        else {
            return Err(EvalAccountResolutionError::UnexpectedDynamicAccount { pubkey });
        };
        if !required.requires_dynamic_account() {
            return Err(EvalAccountResolutionError::UnexpectedDynamicAccount { pubkey });
        }
    }

    for (index, authority) in output_authorities.iter().enumerate() {
        let pubkey = authority.key();
        if output_authorities[index + 1..]
            .iter()
            .any(|candidate| candidate.key() == pubkey)
        {
            return Err(EvalAccountResolutionError::DuplicateOutputAuthority { pubkey });
        }
        if !plan.output_authorities().any(|required| required == pubkey) {
            return Err(EvalAccountResolutionError::UnexpectedOutputAuthority { pubkey });
        }
    }

    for authority in plan.output_authority_requirements() {
        if !output_authorities
            .iter()
            .any(|candidate| candidate.key() == authority.pubkey())
        {
            return Err(EvalAccountResolutionError::MissingOutputAuthority { authority });
        }
    }

    let mut accounts = Vec::new();
    for required in plan.dynamic_account_requirements() {
        let account = if required.requires_output_authority() {
            output_authorities
                .iter()
                .find(|candidate| candidate.key() == required.pubkey())
                .cloned()
                .ok_or(EvalAccountResolutionError::MissingOutputAuthority {
                    authority: EvalOutputAuthorityRequirement {
                        pubkey: required.pubkey(),
                        cpi_account_authority: false,
                    },
                })?
        } else if required.requires_dynamic_account() {
            dynamic_accounts
                .iter()
                .find(|candidate| candidate.key() == required.pubkey())
                .cloned()
                .ok_or_else(|| EvalAccountResolutionError::MissingDynamicAccount {
                    requirement: required.clone(),
                })?
        } else {
            continue;
        };
        if required.is_writable() && !account.is_writable {
            return Err(EvalAccountResolutionError::DynamicAccountNotWritable {
                requirement: required,
            });
        }
        accounts.push(account);
    }

    Ok(ResolvedEvalAccounts { accounts })
}
