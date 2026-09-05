//! The host pause switch.
//!
//! `HostConfig.paused` is the operator's kill switch for the host program's production-shaped
//! instructions. A user decryption never touches the host program — it is served entirely by this
//! Connector — so nothing on chain can stop it, and without this rule a paused host would keep
//! releasing plaintext while refusing every write.
//!
//! The switch is read from the deciding observation, like every other state rule, and it is
//! evaluated before any per-entry work: a paused host is a fact about the deployment, not about a
//! handle, and reporting it per entry would say one request failed thirty-three times for one
//! reason.
//!
//! Refusing while paused is transient. The permit, the delegation records and the handles are all
//! untouched by a pause, so the same request authorizes unchanged once the operator lifts it.
//!
//! What pause deliberately does not stop is the user's own levers: permit revocation and
//! delegation revocation stay open on the host program while paused, because a lever the operator
//! can switch off is not the user's lever.

use super::snapshot::{HostSnapshot, SnapshotError};
use crate::core::solana_acl::{
    SolanaPubkeyBytes, anchor_account_discriminator, host_config_address,
};

/// On-chain account name the singleton's discriminator is the hash of.
const HOST_CONFIG_ACCOUNT: &str = "HostConfig";
/// Length of the leading account discriminator.
const DISCRIMINATOR_LEN: usize = 8;
/// Upper bound on registered coprocessor signers, which fixes the width of the signer array.
const MAX_COPROCESSOR_SIGNERS: usize = 8;
/// Everything the singleton stores ahead of `paused`: the admin, the two chain ids, the input
/// verification contract, the fixed-capacity coprocessor signer set with its count and threshold,
/// the decryption contract, and the current KMS context id.
const PAUSED_INDEX: usize =
    DISCRIMINATOR_LEN + 32 + 8 + 8 + 20 + (MAX_COPROCESSOR_SIGNERS * 20) + 1 + 1 + 20 + 8;
/// Everything the singleton stores after `paused` and before its bump: the deny-list flag, the
/// three HCU knobs, and the update slot.
const BUMP_INDEX: usize = PAUSED_INDEX + 1 + 1 + 8 + 8 + 8 + 8;
/// The singleton's exact serialized length. `HostConfig` is fixed-size and is never realloc-grown,
/// so a longer or shorter account is a different layout rather than a grown one.
const HOST_CONFIG_LEN: usize = BUMP_INDEX + 1;

/// Refuses the request when the deployment's `HostConfig` says the host is paused.
///
/// The singleton's contents are checked against the address they were read from — discriminator,
/// exact length, canonical bump — rather than trusted. Reading a pause flag out of a foreign
/// layout would silently disarm the switch, which is the one failure this rule exists to prevent,
/// so anything that is not the singleton is a refusal and never a `false`.
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

    let data = &account.data;
    let not_a_config = || PauseFailure::NotAHostConfig { account_key };
    if data.len() != HOST_CONFIG_LEN
        || data.get(..DISCRIMINATOR_LEN)
            != Some(&anchor_account_discriminator(HOST_CONFIG_ACCOUNT)[..])
        || data.get(BUMP_INDEX) != Some(&canonical_bump)
    {
        return Err(not_a_config());
    }

    match data.get(PAUSED_INDEX) {
        Some(0) => Ok(()),
        Some(1) => Err(PauseFailure::Paused),
        // Borsh writes a bool as exactly 0 or 1; anything else is not the layout being read here.
        _ => Err(not_a_config()),
    }
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
    /// The account is host-owned but is not the config singleton: wrong length, wrong
    /// discriminator, a bump other than the canonical one, or a `paused` byte that is not a bool.
    #[error("account {account_key:?} is not a decodable host config")]
    NotAHostConfig {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The snapshot was asked for an account it never read.
    #[error(transparent)]
    Snapshot(#[from] SnapshotError),
}
