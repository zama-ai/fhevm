//! The host pause switch.
//!
//! `HostConfig.paused` is the operator's kill switch for the host program's production-shaped
//! instructions. A user decryption never touches the host program — it is served entirely by this
//! Connector — so nothing on chain can stop it, and without this rule a paused host would keep
//! releasing plaintext while refusing every write.
//!
//! It is evaluated on the first read, ahead of every per-entry rule — delegation discovery
//! included — and before a delegated request takes its second read, so a paused host costs one
//! round trip rather than two and never reports a pause as a finding about some handle. That is one
//! observation earlier than every other state rule, and deliberately: pause is an operator action
//! on a deployment, not an authorization record about a handle, so it is not part of the coherent
//! per-entry picture the deciding snapshot exists to give. Keeping it off the second read is also
//! what keeps the worst-case delegated read inside the account budget `MAX_REQUEST_HANDLES` is
//! derived from.
//!
//! Refusing while paused is transient. The permit, the delegation records and the handles are all
//! untouched by a pause, so the same request authorizes unchanged once the operator lifts it.
//!
//! What pause deliberately does not stop is the user's own levers: permit revocation and
//! delegation revocation stay open on the host program while paused, because a lever the operator
//! can switch off is not the user's lever.
//!
//! The layout is not read here. `zama_solana_acl::decode_host_config` is the one byte-level
//! reading of the singleton, shared with the program that writes it and pinned against its
//! serializer on the host side, so this module derives an address, checks an owner, decodes, and
//! reads one flag.

use super::snapshot::{HostSnapshot, SnapshotError};
use crate::core::solana_acl::{SolanaPubkeyBytes, host_config_address};
use zama_solana_acl::decode_host_config;

/// Refuses the request when the deployment's `HostConfig` says the host is paused.
///
/// Anything that is not the deployment's own singleton is a refusal and never a `false`: reading
/// a pause flag out of an absent account or a foreign layout is the one failure this rule exists
/// to prevent.
pub fn check_not_paused(
    snapshot: &HostSnapshot,
    program_id: SolanaPubkeyBytes,
) -> Result<(), PauseFailure> {
    let (account_key, canonical_bump) = host_config_address(program_id);

    let Some(account) = snapshot.account(&account_key)? else {
        return Err(PauseFailure::Absent { account_key });
    };
    // A bare transfer to the derivable address leaves a System-owned empty account, which is the
    // same "the program has never written here" observation as no account at all.
    if account.is_uninitialized_pda() {
        return Err(PauseFailure::Absent { account_key });
    }
    if account.owner != program_id {
        return Err(PauseFailure::ForeignOwner {
            account_key,
            owner: account.owner,
            expected: program_id,
        });
    }

    let not_a_config = || PauseFailure::NotAHostConfig { account_key };
    let config = decode_host_config(&account.data).map_err(|_| not_a_config())?;
    // The stored bump has to be the canonical one for this address. Not an attacker check — only
    // the owning program can write these bytes, and the address was derived here — but a
    // singleton whose bump is not the one this derivation produces is not the singleton this
    // reader reads.
    if config.bump != canonical_bump {
        return Err(not_a_config());
    }
    if config.paused {
        return Err(PauseFailure::Paused);
    }
    Ok(())
}

/// Why the pause rule refused a request, or could not be evaluated.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum PauseFailure {
    /// The operator has paused the host.
    #[error("the host is paused")]
    Paused,
    /// The deployment's `HostConfig` was not observed at all.
    #[error("no host config at {account_key:?} at the observed slot")]
    Absent {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The singleton address holds an account owned by another program.
    #[error("host config {account_key:?} is owned by {owner:?}, expected {expected:?}")]
    ForeignOwner {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
        /// Who owns it.
        owner: SolanaPubkeyBytes,
        /// The deployment's program id.
        expected: SolanaPubkeyBytes,
    },
    /// The account is host-owned but is not the config singleton: the shared decoder refused it,
    /// or it stores a bump other than the canonical one for its address.
    #[error("account {account_key:?} is not a decodable host config")]
    NotAHostConfig {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The snapshot was asked for an account it never read.
    #[error(transparent)]
    Snapshot(#[from] SnapshotError),
}
