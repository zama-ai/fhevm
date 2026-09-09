//! Encrypted value account resolution: presence, program ownership, type, address binding, and
//! the authority and application the account names.
//!
//! A handle entry names the object that authorizes it — the encrypted value account, by address —
//! and the chain of checks here is what turns that unsigned claim into a validated account.
//! Program ownership is the sole trust anchor: no party other than the host program can produce
//! data in an account the host program owns. Everything the later rules read (its authority, its
//! `(program, scope)`, the MMR commitments) comes from this validated account and from nowhere
//! else.
//!
//! Trailing account bytes are legal and ignored. The account is realloc-grown by one peak at a
//! time and never shrunk, so rejecting a tail would refuse every account whose MMR once had more
//! peaks than it has now.

use super::snapshot::{HostSnapshot, SnapshotError};
use crate::core::solana_acl::{SolanaPubkeyBytes, WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY};
use solana_pubkey::Pubkey;
use zama_solana_acl::{AclError, EncryptedValue, decode_on_chain_account};

/// An encrypted value account that passed presence, ownership, type and address binding.
///
/// No public constructor: [`resolve_encrypted_value_account`] is the only way in, which is what
/// makes "read the authority from something that was never validated" unwritable.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ResolvedEncryptedValueAccount {
    account_key: SolanaPubkeyBytes,
    encrypted_value: EncryptedValue,
}

impl ResolvedEncryptedValueAccount {
    /// The encrypted value account's address, which every leaf commitment binds.
    pub fn account_key(&self) -> SolanaPubkeyBytes {
        self.account_key
    }

    /// The authority this encrypted value account belongs to, which the delegation rule is read
    /// against. Taken from the account and never from a request field.
    pub fn encrypted_value_account_authority(&self) -> SolanaPubkeyBytes {
        self.encrypted_value.encrypted_value_account_authority
    }

    /// The application program the value belongs to, verified by the host on every write.
    pub fn program(&self) -> SolanaPubkeyBytes {
        self.encrypted_value.program
    }

    /// The program-declared scope within `program`. Meaningful only as the pair
    /// `(program, scope)`, which is what the signed permit scope is tested against.
    pub fn scope(&self) -> SolanaPubkeyBytes {
        self.encrypted_value.scope
    }

    /// The decoded account, for the handle-binding rules.
    pub fn encrypted_value(&self) -> &EncryptedValue {
        &self.encrypted_value
    }
}

/// The address an encrypted value account with these fields must live at: the PDA of its own
/// seeds under the stored bump. `None` when the stored bump yields no valid address.
pub fn encrypted_value_account_address(
    program_id: SolanaPubkeyBytes,
    value: &EncryptedValue,
) -> Option<SolanaPubkeyBytes> {
    let bump = [value.bump];
    let mut seeds: Vec<&[u8]> = value.seeds().to_vec();
    seeds.push(&bump);
    Pubkey::create_program_address(&seeds, &Pubkey::new_from_array(program_id))
        .ok()
        .map(|address| address.to_bytes())
}

/// Resolves one entry's encrypted value account against the snapshot.
///
/// In order: the account exists in the snapshot; it is owned by the deployment's program; its
/// data carries the encrypted value account discriminator and borsh-decodes; its own fields
/// derive the address it was read at; and the authority it names is not the wildcard sentinel.
/// The address check is the backstop that makes a well-formed account found anywhere else a
/// rejection: an account's fields are its identity, and an account that does not live where its
/// fields say is not that identity's account.
pub fn resolve_encrypted_value_account(
    snapshot: &HostSnapshot,
    program_id: SolanaPubkeyBytes,
    account_key: SolanaPubkeyBytes,
) -> Result<ResolvedEncryptedValueAccount, EncryptedValueAccountFailure> {
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

    // h3: an encrypted value account and not another account type of the same program. Decoding
    // is the shared crate's, so the discriminator and the body layout are the ones the host
    // program writes; trailing bytes past the body are accepted, which is what a realloc-grown
    // account has.
    let encrypted_value = decode_on_chain_account(&account.data).map_err(|error| match error {
        AclError::BadDiscriminator => {
            EncryptedValueAccountFailure::WrongAccountType { account_key }
        }
        AclError::BadAccountData => EncryptedValueAccountFailure::Malformed { account_key },
        // The remaining variants belong to the authorization rules, not to decoding; decoding
        // cannot produce them, and enumerating them keeps a new one from landing here silently.
        AclError::MmrInconsistent
        | AclError::MmrPeakCapacityExceeded
        | AclError::HistoricalProofInvalid
        | AclError::PublicDecryptProofInvalid => {
            EncryptedValueAccountFailure::Malformed { account_key }
        }
    })?;

    // h4: the account's own fields derive the address it was read at.
    let derived = encrypted_value_account_address(program_id, &encrypted_value);
    if derived != Some(account_key) {
        return Err(EncryptedValueAccountFailure::AddressMismatch {
            account_key,
            derived,
        });
    }

    // h5: the authority the account names is a real authority, not the wildcard sentinel. The
    // authority-specific delegation address is derived from this value, and with the sentinel in
    // it that derivation lands on the wildcard row itself — the authority-specific check would be
    // structurally a wildcard check. No legal account carries it: the authority is a PDA of the
    // application program, and the sentinel is not one.
    if encrypted_value.encrypted_value_account_authority
        == WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY
    {
        return Err(EncryptedValueAccountFailure::SentinelAuthority { account_key });
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
    /// The account's own fields derive a different address than the one it was read at.
    #[error(
        "encrypted value account {account_key:?} does not live at the address its fields derive"
    )]
    AddressMismatch {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
        /// What the account's fields derive, if the stored bump yields an address at all.
        derived: Option<SolanaPubkeyBytes>,
    },
    /// The account names the wildcard sentinel as its authority. No legal account does, and
    /// resolving it would send the authority-specific delegation check to the wildcard row.
    #[error("encrypted value account {account_key:?} names the wildcard sentinel as its authority")]
    SentinelAuthority {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The snapshot was asked for an account it never read.
    #[error(transparent)]
    Snapshot(#[from] SnapshotError),
}
