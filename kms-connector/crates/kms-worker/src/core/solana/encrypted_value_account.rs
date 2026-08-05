//! Encrypted value account resolution: presence, program ownership, type, identity binding, app
//! context.
//!
//! A handle entry names the object that authorizes it — the encrypted value account, identified by
//! its encrypted value ID — and the chain of checks here is what turns that unsigned claim into a
//! validated account. Program ownership is the sole trust anchor: no party other than the host
//! program can produce data in an account the host program owns. Everything the later rules read
//! (its authority, the ACL domain, the current handle, the subject set, the MMR commitments)
//! comes from this validated account and from nowhere else.
//!
//! Trailing account bytes are legal and ignored. The account is realloc-grown to its high-water
//! mark and never shrunk, so rejecting a tail would be a denial of service against every encrypted
//! value account that ever held more subjects than it holds now. This is the deliberate opposite of
//! the access-proof rule, where a tail is a rejection — the asymmetry is normative and should not
//! be "fixed" into symmetry.

use super::snapshot::{HostSnapshot, SnapshotError};
use crate::core::solana_acl::SolanaPubkeyBytes;
use zama_solana_acl::{AclError, EncryptedValue, decode_on_chain_account};

/// An encrypted value account that passed presence, ownership, type and identity binding.
///
/// No public constructor: [`resolve_encrypted_value_account`] is the only way in, which is what
/// makes "read the encrypted value account authority from something that was never validated"
/// unwritable.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ResolvedEncryptedValueAccount {
    account_key: SolanaPubkeyBytes,
    encrypted_value: EncryptedValue,
}

impl ResolvedEncryptedValueAccount {
    /// The encrypted value account's address, which the historical leaf commitment binds.
    pub fn account_key(&self) -> SolanaPubkeyBytes {
        self.account_key
    }

    /// The authority this encrypted value account belongs to — the app
    /// context of every later rule, read from the account and never from a request field.
    pub fn encrypted_value_account_authority(&self) -> SolanaPubkeyBytes {
        self.encrypted_value.encrypted_value_account_authority
    }

    /// The ACL domain this encrypted value account belongs to, likewise account-sourced. This is
    /// the value the signed scope is tested against.
    pub fn domain(&self) -> SolanaPubkeyBytes {
        self.encrypted_value.domain
    }

    /// The decoded account, for the handle-binding rules.
    pub fn encrypted_value(&self) -> &EncryptedValue {
        &self.encrypted_value
    }
}

/// The canonical encrypted value account address for an encrypted value ID under this deployment.
pub fn encrypted_value_account_address(
    program_id: SolanaPubkeyBytes,
    encrypted_value_id: [u8; 32],
) -> (SolanaPubkeyBytes, u8) {
    crate::core::solana_encrypted_value_acl::encrypted_value_acl_address(
        program_id,
        encrypted_value_id,
    )
}

/// Resolves one entry's encrypted value account against the snapshot.
///
/// In order: the account exists in the snapshot; it is owned by the deployment's program; its data
/// carries the encrypted value account discriminator and borsh-decodes; and its own fields
/// reproduce the claimed encrypted value ID. The last check is the backstop that makes a
/// substituted encrypted value account a rejection rather than a redirection: an attacker naming a
/// victim's encrypted value ID while supplying a different account fails it.
pub fn resolve_encrypted_value_account(
    snapshot: &HostSnapshot,
    program_id: SolanaPubkeyBytes,
    encrypted_value_id: [u8; 32],
) -> Result<ResolvedEncryptedValueAccount, EncryptedValueAccountFailure> {
    // The address is derived from the claimed identity, never supplied: a request naming another
    // account does not redirect the read, it only names an encrypted value ID whose own account is
    // read.
    let (account_key, _) = encrypted_value_account_address(program_id, encrypted_value_id);

    // h1: present at this observation point.
    let account = snapshot
        .account(&account_key)?
        .ok_or(EncryptedValueAccountFailure::Absent { account_key })?;

    // h2: written by the host program. The sole trust anchor — contents of an account owned by
    // anyone else prove nothing, however well-formed they are.
    if account.owner != program_id {
        return Err(EncryptedValueAccountFailure::ForeignOwner {
            account_key,
            owner: account.owner,
            expected: program_id,
        });
    }

    // h3: an encrypted value account and not another account type of the same program. Decoding is
    // the shared crate's, so the discriminator and the body layout are the ones the host program
    // writes; trailing bytes past the body are accepted, which is what a realloc-grown account has.
    let encrypted_value = decode_on_chain_account(&account.data).map_err(|error| match error {
        AclError::BadDiscriminator => {
            EncryptedValueAccountFailure::WrongAccountType { account_key }
        }
        AclError::BadAccountData => EncryptedValueAccountFailure::Malformed { account_key },
        // The remaining variants belong to the authorization rules, not to decoding; decoding
        // cannot produce them, and enumerating them keeps a new one from landing here silently.
        AclError::MmrInconsistent
        | AclError::MmrPeakCapacityExceeded
        | AclError::SubjectCapacityExceeded
        | AclError::HandleMismatch
        | AclError::SubjectMissing
        | AclError::HistoricalProofInvalid
        | AclError::PublicDecryptProofInvalid => {
            EncryptedValueAccountFailure::Malformed { account_key }
        }
    })?;

    // h4: the account's own fields reproduce the identity it was named by. The backstop that
    // makes a substituted encrypted value account a rejection rather than a redirection.
    let derived = encrypted_value.encrypted_value_id();
    if derived != encrypted_value_id {
        return Err(EncryptedValueAccountFailure::EncryptedValueIdMismatch {
            account_key,
            claimed: encrypted_value_id,
            derived,
        });
    }

    Ok(ResolvedEncryptedValueAccount {
        account_key,
        encrypted_value,
    })
}

/// Why an encrypted value account could not be resolved.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum EncryptedValueAccountFailure {
    /// The account does not exist at this observation point. Transient by nature: the
    /// account may simply not have reached the observed commitment yet.
    #[error("encrypted value account {account_key:?} does not exist at the observed slot")]
    Absent {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The account exists but belongs to another program, so its contents prove nothing.
    #[error("encrypted value account {account_key:?} is owned by {owner:?}, expected {expected:?}")]
    ForeignOwner {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
        /// Who owns it.
        owner: SolanaPubkeyBytes,
        /// The deployment's program id.
        expected: SolanaPubkeyBytes,
    },
    /// The account is host-owned but is not an encrypted value account — a different account type
    /// of the same program, caught by the discriminator.
    #[error("account {account_key:?} does not carry the encrypted value account discriminator")]
    WrongAccountType {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The discriminator matched but the body did not decode.
    #[error("encrypted value account {account_key:?} body does not decode")]
    Malformed {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The account's own fields derive a different encrypted value ID than the one claimed.
    #[error("encrypted value account {account_key:?} fields derive a different encrypted value ID")]
    EncryptedValueIdMismatch {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
        /// What the request claimed.
        claimed: [u8; 32],
        /// What the account's fields derive.
        derived: [u8; 32],
    },
    /// The snapshot was asked for an account it never read.
    #[error(transparent)]
    Snapshot(#[from] SnapshotError),
}
