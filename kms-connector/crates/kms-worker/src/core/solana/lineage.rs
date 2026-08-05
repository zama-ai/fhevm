//! Lineage resolution: presence, program ownership, type, identity binding, app context.
//!
//! A handle entry names the object that authorizes it — the lineage, identified by its
//! `valueKey` — and the chain of checks here is what turns that unsigned claim into a
//! validated account. Program ownership is the sole trust anchor: no party other than the
//! host program can produce data in an account the host program owns. Everything the later
//! rules read (the app account, the ACL domain, the current handle, the subject set, the MMR
//! commitments) comes from this validated account and from nowhere else.
//!
//! Trailing account bytes are legal and ignored. The account is realloc-grown to its
//! high-water mark and never shrunk, so rejecting a tail would be a denial of service against
//! every lineage that ever held more subjects than it holds now. This is the deliberate
//! opposite of the access-proof rule, where a tail is a rejection — the asymmetry is
//! normative and should not be "fixed" into symmetry.

use super::snapshot::{HostSnapshot, SnapshotError};
use crate::core::solana_acl::SolanaPubkeyBytes;
use zama_solana_acl::{AclError, EncryptedValue, decode_on_chain_account};

/// A lineage account that passed presence, ownership, type and identity binding.
///
/// No public constructor: [`resolve_lineage`] is the only way in, which is what makes "read
/// the app account from something that was never validated" unwritable.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ResolvedLineage {
    account_key: SolanaPubkeyBytes,
    lineage: EncryptedValue,
}

impl ResolvedLineage {
    /// The lineage account's address, which the historical leaf commitment binds.
    pub fn account_key(&self) -> SolanaPubkeyBytes {
        self.account_key
    }

    /// The app account this lineage belongs to — the app context of every later rule, read
    /// from the account and never from a request field.
    pub fn encrypted_value_account_authority(&self) -> SolanaPubkeyBytes {
        self.lineage.encrypted_value_account_authority
    }

    /// The ACL domain this lineage belongs to, likewise account-sourced. This is the value
    /// the signed scope is tested against.
    pub fn domain(&self) -> SolanaPubkeyBytes {
        self.lineage.domain
    }

    /// The decoded account, for the handle-binding rules.
    pub fn lineage(&self) -> &EncryptedValue {
        &self.lineage
    }
}

/// The canonical lineage account address for a value key under this deployment.
pub fn lineage_address(
    program_id: SolanaPubkeyBytes,
    encrypted_value_id: [u8; 32],
) -> (SolanaPubkeyBytes, u8) {
    crate::core::solana_encrypted_value_acl::encrypted_value_acl_address(program_id, encrypted_value_id)
}

/// Resolves one entry's lineage against the snapshot.
///
/// In order: the account exists in the snapshot; it is owned by the deployment's program;
/// its data carries the lineage discriminator and borsh-decodes; and its own fields
/// reproduce the claimed `valueKey`. The last check is the backstop that makes a substituted
/// lineage a rejection rather than a redirection: an attacker naming a victim's `valueKey`
/// while supplying a different account fails it.
pub fn resolve_lineage(
    snapshot: &HostSnapshot,
    program_id: SolanaPubkeyBytes,
    encrypted_value_id: [u8; 32],
) -> Result<ResolvedLineage, LineageFailure> {
    // The address is derived from the claimed identity, never supplied: a request naming another
    // account does not redirect the read, it only names a value key whose own account is read.
    let (account_key, _) = lineage_address(program_id, encrypted_value_id);

    // h1: present at this observation point.
    let account = snapshot
        .account(&account_key)?
        .ok_or(LineageFailure::Absent { account_key })?;

    // h2: written by the host program. The sole trust anchor — contents of an account owned by
    // anyone else prove nothing, however well-formed they are.
    if account.owner != program_id {
        return Err(LineageFailure::ForeignOwner {
            account_key,
            owner: account.owner,
            expected: program_id,
        });
    }

    // h3: a lineage and not another account type of the same program. Decoding is the shared
    // crate's, so the discriminator and the body layout are the ones the host program writes;
    // trailing bytes past the body are accepted, which is what a realloc-grown account has.
    let lineage = decode_on_chain_account(&account.data).map_err(|error| match error {
        AclError::BadDiscriminator => LineageFailure::WrongAccountType { account_key },
        AclError::BadAccountData => LineageFailure::Malformed { account_key },
        // The remaining variants belong to the authorization rules, not to decoding; decoding
        // cannot produce them, and enumerating them keeps a new one from landing here silently.
        AclError::MmrInconsistent
        | AclError::MmrPeakCapacityExceeded
        | AclError::SubjectCapacityExceeded
        | AclError::HandleMismatch
        | AclError::SubjectMissing
        | AclError::HistoricalProofInvalid
        | AclError::PublicDecryptProofInvalid => LineageFailure::Malformed { account_key },
    })?;

    // h4: the account's own fields reproduce the identity it was named by. The backstop that
    // makes a substituted lineage a rejection rather than a redirection.
    let derived = lineage.encrypted_value_id();
    if derived != encrypted_value_id {
        return Err(LineageFailure::ValueKeyMismatch {
            account_key,
            claimed: encrypted_value_id,
            derived,
        });
    }

    Ok(ResolvedLineage {
        account_key,
        lineage,
    })
}

/// Why a lineage could not be resolved.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum LineageFailure {
    /// The account does not exist at this observation point. Transient by nature: the
    /// account may simply not have reached the observed commitment yet.
    #[error("lineage account {account_key:?} does not exist at the observed slot")]
    Absent {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The account exists but belongs to another program, so its contents prove nothing.
    #[error("lineage account {account_key:?} is owned by {owner:?}, expected {expected:?}")]
    ForeignOwner {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
        /// Who owns it.
        owner: SolanaPubkeyBytes,
        /// The deployment's program id.
        expected: SolanaPubkeyBytes,
    },
    /// The account is host-owned but is not a lineage — a different account type of the same
    /// program, caught by the discriminator.
    #[error("account {account_key:?} does not carry the lineage discriminator")]
    WrongAccountType {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The discriminator matched but the body did not decode.
    #[error("lineage account {account_key:?} body does not decode")]
    Malformed {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The account's own fields derive a different value key than the one claimed.
    #[error("lineage account {account_key:?} fields derive a different value key")]
    ValueKeyMismatch {
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
