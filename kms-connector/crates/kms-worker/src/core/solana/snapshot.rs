//! Confirmed host account reads. Each read is one `getMultipleAccounts` at one slot.
//!
//! The first read covers the host config, the signer's invalidation record and the named
//! encrypted stores. A delegated request then reads the same accounts, minus the already-checked
//! host config, plus the delegation records the first read made derivable. Every rule after the
//! pause switch uses that second read, which must not be older than the first.

use super::{SolanaPubkeyBytes, host_config_address, permit_invalidation_address};
use connector_utils::types::solana_request::SolanaUserDecryptionRequestV1;
use solana_account_decoder_client_types::{UiAccountData, UiAccountEncoding};
use solana_commitment_config::CommitmentConfig;
use solana_pubkey::Pubkey;
use solana_rpc_client::{api::config::RpcAccountInfoConfig, nonblocking::rpc_client::RpcClient};
use std::num::NonZeroUsize;
use std::{
    collections::{BTreeMap, BTreeSet},
    future::Future,
    sync::Arc,
    time::Duration,
};
use tokio::sync::Semaphore;
use url::Url;

/// The System program's id: the owner of an account no program has taken over.
pub const SYSTEM_PROGRAM_ID: SolanaPubkeyBytes = [0; 32];

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SnapshotAccount {
    pub owner: SolanaPubkeyBytes,
    pub data: Vec<u8>,
}

impl SnapshotAccount {
    /// System-owned and empty: what a bare transfer to a derivable address leaves. Such an account
    /// says nothing about host state.
    pub fn is_uninitialized_pda(&self) -> bool {
        self.owner == SYSTEM_PROGRAM_ID && self.data.is_empty()
    }
}

/// Account keys in read order, without repeats.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct SnapshotKeys(Vec<SolanaPubkeyBytes>);

impl SnapshotKeys {
    pub fn new(keys: impl IntoIterator<Item = SolanaPubkeyBytes>) -> Self {
        let mut seen = BTreeSet::new();
        let mut ordered = Vec::new();
        for key in keys {
            if seen.insert(key) {
                ordered.push(key);
            }
        }
        Self(ordered)
    }

    pub fn as_slice(&self) -> &[SolanaPubkeyBytes] {
        &self.0
    }

    pub fn contains(&self, key: &SolanaPubkeyBytes) -> bool {
        self.0.contains(key)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HostSnapshot {
    observed_slot: u64,
    accounts: BTreeMap<SolanaPubkeyBytes, Option<SnapshotAccount>>,
}

impl HostSnapshot {
    /// `accounts` pairs every read key with what the node returned for it, `None` for no account.
    pub fn new(
        observed_slot: u64,
        accounts: impl IntoIterator<Item = (SolanaPubkeyBytes, Option<SnapshotAccount>)>,
    ) -> Self {
        Self {
            observed_slot,
            accounts: accounts.into_iter().collect(),
        }
    }

    pub fn observed_slot(&self) -> u64 {
        self.observed_slot
    }

    /// An absent account is `Ok(None)`; a key that was never read is an error, so a key-planning
    /// bug cannot pass as a missing account.
    pub fn account(
        &self,
        key: &SolanaPubkeyBytes,
    ) -> Result<Option<&SnapshotAccount>, UnreadAccount> {
        match self.accounts.get(key) {
            Some(account) => Ok(account.as_ref()),
            None => Err(UnreadAccount { key: *key }),
        }
    }

    /// Takes this read as the deciding one. A read older than the discovery read comes from a
    /// node that fell behind, and would reject grants the discovery read already saw.
    pub fn deciding_after(self, discovery: &HostSnapshot) -> Result<Self, SnapshotError> {
        if self.observed_slot < discovery.observed_slot() {
            return Err(SnapshotError::DecidingReadOlderThanDiscovery {
                discovery_slot: discovery.observed_slot(),
                deciding_slot: self.observed_slot,
            });
        }
        Ok(self)
    }
}

pub trait HostStateReader: Send + Sync {
    fn read_accounts(
        &self,
        keys: &SnapshotKeys,
    ) -> impl Future<Output = Result<HostSnapshot, SnapshotError>> + Send;
}

pub fn plan_first_read(
    request: &SolanaUserDecryptionRequestV1,
    program_id: SolanaPubkeyBytes,
) -> SnapshotKeys {
    let signer = *request.permit().user_address().as_bytes();
    let (host_config_key, _) = host_config_address(program_id);
    let (watermark_key, _) = permit_invalidation_address(program_id, signer);
    let encrypted_stores = request.handles().iter().map(|entry| entry.encrypted_store);
    SnapshotKeys::new(
        [host_config_key, watermark_key]
            .into_iter()
            .chain(encrypted_stores),
    )
}

pub fn plan_second_read(
    first: &SnapshotKeys,
    program_id: SolanaPubkeyBytes,
    delegation_keys: impl IntoIterator<Item = SolanaPubkeyBytes>,
) -> SnapshotKeys {
    let (host_config_key, _) = host_config_address(program_id);
    SnapshotKeys::new(
        first
            .as_slice()
            .iter()
            .copied()
            .filter(|key| key != &host_config_key)
            .chain(delegation_keys),
    )
}

#[derive(Clone)]
pub struct SolanaRpcClient {
    client: Arc<RpcClient>,
    permits: Arc<Semaphore>,
    timeout: Duration,
}

impl std::fmt::Debug for SolanaRpcClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SolanaRpcClient")
            .field(&self.client.url())
            .finish()
    }
}

impl SolanaRpcClient {
    pub fn new(url: Url, timeout: Duration, max_concurrent_calls: NonZeroUsize) -> Self {
        Self {
            client: Arc::new(RpcClient::new_with_timeout_and_commitment(
                url.to_string(),
                timeout,
                CommitmentConfig::confirmed(),
            )),
            permits: Arc::new(Semaphore::new(max_concurrent_calls.get())),
            timeout,
        }
    }
}

impl HostStateReader for SolanaRpcClient {
    async fn read_accounts(&self, keys: &SnapshotKeys) -> Result<HostSnapshot, SnapshotError> {
        let pubkeys: Vec<_> = keys
            .as_slice()
            .iter()
            .copied()
            .map(Pubkey::new_from_array)
            .collect();
        // As for EVM host RPC, waiting for a concurrency permit does not count against the call
        // timeout. The SDK retries a throttled call internally, so the deadline wraps the retries
        // rather than being the HTTP client's alone.
        let _permit = self
            .permits
            .acquire()
            .await
            .expect("the permit semaphore is never closed");
        let call = self.client.get_multiple_ui_accounts_with_config(
            &pubkeys,
            RpcAccountInfoConfig {
                encoding: Some(UiAccountEncoding::Base64),
                commitment: Some(CommitmentConfig::confirmed()),
                ..Default::default()
            },
        );
        let response = tokio::time::timeout(self.timeout, call)
            .await
            .map_err(|_| SnapshotError::Unavailable {
                reason: format!("Solana account read timed out after {:?}", self.timeout),
            })?
            .map_err(|error| SnapshotError::Unavailable {
                reason: error.to_string(),
            })?;
        if response.value.len() != keys.len() {
            return Err(SnapshotError::ResponseLengthMismatch {
                requested: keys.len(),
                returned: response.value.len(),
            });
        }
        // The UI-account method reports undecodable data as an error instead of as a missing
        // account.
        let accounts = response
            .value
            .into_iter()
            .zip(keys.as_slice())
            .map(|(account, key)| {
                account
                    .map(|account| {
                        let malformed = || SnapshotError::Unavailable {
                            reason: format!(
                                "malformed Solana account at {}",
                                Pubkey::new_from_array(*key)
                            ),
                        };
                        if !matches!(
                            account.data,
                            UiAccountData::Binary(_, UiAccountEncoding::Base64)
                        ) {
                            return Err(malformed());
                        }
                        Ok(SnapshotAccount {
                            owner: account
                                .owner
                                .parse::<Pubkey>()
                                .map_err(|_| malformed())?
                                .to_bytes(),
                            data: account.data.decode().ok_or_else(malformed)?,
                        })
                    })
                    .transpose()
                    .map(|account| (*key, account))
            })
            .collect::<Result<Vec<_>, SnapshotError>>()?;
        Ok(HostSnapshot::new(response.context.slot, accounts))
    }
}

/// A read of the host that failed. Every variant is the node's, so a later attempt may succeed.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum SnapshotError {
    #[error("host state read failed: {reason}")]
    Unavailable { reason: String },
    #[error("host state read returned {returned} accounts for {requested} keys")]
    ResponseLengthMismatch { requested: usize, returned: usize },
    #[error(
        "the deciding read observed slot {deciding_slot}, older than the discovery read's {discovery_slot}"
    )]
    DecidingReadOlderThanDiscovery {
        discovery_slot: u64,
        deciding_slot: u64,
    },
}

/// A check asked for an account its read did not plan: a Connector bug, never a missing account.
#[derive(Clone, Copy, PartialEq, Eq, Debug, thiserror::Error)]
#[error("account {key:?} was never read")]
pub struct UnreadAccount {
    pub key: SolanaPubkeyBytes,
}
