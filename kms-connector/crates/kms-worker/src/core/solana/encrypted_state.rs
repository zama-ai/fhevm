//! Encrypted state resolution: presence, program ownership, type, address binding, and
//! the authority and application the account names.
//!
//! A handle entry names the object that authorizes it — the encrypted state, by address —
//! and the chain of checks here is what turns that unsigned claim into a validated account.
//! Program ownership is the sole trust anchor: no party other than the host program can produce
//! data in an account the host program owns. Everything the later rules read (its authority, its
//! `(program, scope)`, the MMR commitments) comes from this validated account and from nowhere
//! else.
//!
//! Trailing account bytes are legal and ignored. State storage can grow as slots or history are
//! added without shrinking after a shorter encoding, so the allocated tail is not state data.

use super::snapshot::{HostSnapshot, SnapshotError};
use crate::core::solana_acl::{SolanaPubkeyBytes, WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY};
use solana_pubkey::Pubkey;
use zama_solana_acl::{AclError, EncryptedState, decode_encrypted_state};

/// An encrypted state that passed presence, ownership, type and address binding.
///
/// No public constructor: [`resolve_encrypted_state`] is the only way in, which is what
/// makes "read the authority from something that was never validated" unwritable.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ResolvedEncryptedState {
    account_key: SolanaPubkeyBytes,
    encrypted_state: EncryptedState,
}

impl ResolvedEncryptedState {
    /// The encrypted state's address, which every leaf commitment binds.
    pub fn account_key(&self) -> SolanaPubkeyBytes {
        self.account_key
    }

    /// The authority this encrypted state belongs to, which the delegation rule is read
    /// against. Taken from the account and never from a request field.
    pub fn authority(&self) -> SolanaPubkeyBytes {
        self.encrypted_state.authority
    }

    /// The application program the state belongs to, verified by the host on every write.
    pub fn program(&self) -> SolanaPubkeyBytes {
        self.encrypted_state.program
    }

    /// The program-declared scope within `program`. Meaningful only as the pair
    /// `(program, scope)`, which is what the signed permit scope is tested against.
    pub fn scope(&self) -> SolanaPubkeyBytes {
        self.encrypted_state.scope
    }

    /// The decoded account, for the handle-binding rules.
    pub fn encrypted_state(&self) -> &EncryptedState {
        &self.encrypted_state
    }
}

/// The address an encrypted state with these fields must live at: the PDA of its own
/// seeds under the stored bump. `None` when the stored bump yields no valid address.
pub fn encrypted_state_address(
    program_id: SolanaPubkeyBytes,
    state: &EncryptedState,
) -> Option<SolanaPubkeyBytes> {
    let bump = [state.bump];
    let mut seeds: Vec<&[u8]> = state.seeds().to_vec();
    seeds.push(&bump);
    Pubkey::create_program_address(&seeds, &Pubkey::new_from_array(program_id))
        .ok()
        .map(|address| address.to_bytes())
}

/// Resolves one entry's encrypted state against the snapshot.
///
/// In order: the account exists in the snapshot; it is owned by the deployment's program; its
/// data carries the encrypted state discriminator and borsh-decodes; its own fields
/// derive the address it was read at; and the authority it names is not the wildcard sentinel.
/// The address check is the backstop that makes a well-formed account found anywhere else a
/// rejection: an account's fields are its identity, and an account that does not live where its
/// fields say is not that identity's account.
pub fn resolve_encrypted_state(
    snapshot: &HostSnapshot,
    program_id: SolanaPubkeyBytes,
    account_key: SolanaPubkeyBytes,
) -> Result<ResolvedEncryptedState, EncryptedStateFailure> {
    // h1: present at this observation point.
    let account = snapshot
        .account(&account_key)?
        .ok_or(EncryptedStateFailure::Absent { account_key })?;

    // h2: written by the host program. The sole trust anchor — contents of an account owned by
    // anyone else prove nothing, however well-formed they are.
    if account.owner != program_id {
        return Err(EncryptedStateFailure::ForeignOwner {
            account_key,
            owner: account.owner,
            expected: program_id,
        });
    }

    // h3: an encrypted state and not another account type of the same program. Decoding
    // is the shared crate's, so the discriminator and the body layout are the ones the host
    // program writes; trailing bytes past the body are accepted, which is what a realloc-grown
    // account has.
    let encrypted_state = decode_encrypted_state(&account.data).map_err(|error| match error {
        AclError::BadDiscriminator => EncryptedStateFailure::WrongAccountType { account_key },
        AclError::BadAccountData | AclError::MmrInconsistent => {
            EncryptedStateFailure::Malformed { account_key }
        }
        // The remaining variants belong to the authorization rules, not to decoding; decoding
        // cannot produce them, and enumerating them keeps a new one from landing here silently.
        AclError::MmrPeakCapacityExceeded
        | AclError::HistoricalProofInvalid
        | AclError::PublicDecryptProofInvalid => EncryptedStateFailure::Malformed { account_key },
    })?;

    // h4: the account's own fields derive the address it was read at.
    let derived = encrypted_state_address(program_id, &encrypted_state);
    if derived != Some(account_key) {
        return Err(EncryptedStateFailure::AddressMismatch {
            account_key,
            derived,
        });
    }

    // h5: the authority the account names is a real authority, not the wildcard sentinel. The
    // authority-specific delegation address is derived from this value, and with the sentinel in
    // it that derivation lands on the wildcard row itself — the authority-specific check would be
    // structurally a wildcard check. No legal account carries it: the authority is a PDA of the
    // application program, and the sentinel is not one.
    if encrypted_state.authority == WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY {
        return Err(EncryptedStateFailure::SentinelAuthority { account_key });
    }

    Ok(ResolvedEncryptedState {
        account_key,
        encrypted_state,
    })
}

/// Why an encrypted state could not be resolved.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum EncryptedStateFailure {
    /// The account does not exist at this observation point. Transient by nature: the
    /// account may simply not have reached the observed commitment yet.
    #[error("encrypted state {account_key:?} does not exist at the observed slot")]
    Absent {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The account exists but belongs to another program, so its contents prove nothing.
    #[error("encrypted state {account_key:?} is owned by {owner:?}, expected {expected:?}")]
    ForeignOwner {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
        /// Who owns it.
        owner: SolanaPubkeyBytes,
        /// The deployment's program id.
        expected: SolanaPubkeyBytes,
    },
    /// The account is host-owned but is not an encrypted state — a different account type
    /// of the same program, caught by the discriminator.
    #[error("account {account_key:?} does not carry the encrypted state discriminator")]
    WrongAccountType {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The discriminator matched but the body did not decode.
    #[error("encrypted state {account_key:?} body does not decode")]
    Malformed {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The account's own fields derive a different address than the one it was read at.
    #[error("encrypted state {account_key:?} does not live at the address its fields derive")]
    AddressMismatch {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
        /// What the account's fields derive, if the stored bump yields an address at all.
        derived: Option<SolanaPubkeyBytes>,
    },
    /// The account names the wildcard sentinel as its authority. No legal account does, and
    /// resolving it would send the authority-specific delegation check to the wildcard row.
    #[error("encrypted state {account_key:?} names the wildcard sentinel as its authority")]
    SentinelAuthority {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The snapshot was asked for an account it never read.
    #[error(transparent)]
    Snapshot(#[from] SnapshotError),
}
