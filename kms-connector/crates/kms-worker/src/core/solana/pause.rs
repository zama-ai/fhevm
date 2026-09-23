//! The host pause switch. A user decryption never touches the host program, so without this rule
//! a paused host would keep releasing plaintext. It is decided on the first read: pause is about
//! the deployment, not about a handle, and the second read then fits the account budget.

use super::snapshot::{HostSnapshot, UnreadAccount};
use super::{SolanaPubkeyBytes, host_config_address};
use zama_solana_acl::decode_host_config;

/// Refuses the request unless the deployment's own `HostConfig` says the host is running.
pub fn check_not_paused(
    snapshot: &HostSnapshot,
    program_id: SolanaPubkeyBytes,
) -> Result<(), PauseFailure> {
    let (account_key, canonical_bump) = host_config_address(program_id);
    let account = snapshot
        .account(&account_key)?
        .filter(|account| !account.is_uninitialized_pda())
        .ok_or(PauseFailure::Absent { account_key })?;
    if account.owner != program_id {
        return Err(PauseFailure::ForeignOwner {
            account_key,
            owner: account.owner,
        });
    }
    let config = decode_host_config(&account.data)
        .ok()
        .filter(|config| config.bump == canonical_bump)
        .ok_or(PauseFailure::NotAHostConfig { account_key })?;
    if config.paused {
        return Err(PauseFailure::Paused);
    }
    Ok(())
}

#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum PauseFailure {
    #[error("the host is paused")]
    Paused,
    #[error("no host config at {account_key:?} at the observed slot")]
    Absent { account_key: SolanaPubkeyBytes },
    #[error("host config {account_key:?} is owned by {owner:?}")]
    ForeignOwner {
        account_key: SolanaPubkeyBytes,
        owner: SolanaPubkeyBytes,
    },
    #[error("account {account_key:?} is not a canonical host config")]
    NotAHostConfig { account_key: SolanaPubkeyBytes },
    #[error(transparent)]
    UnreadAccount(#[from] UnreadAccount),
}
