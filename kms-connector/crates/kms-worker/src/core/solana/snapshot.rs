//! Confirmed host account reads. Each read is one `getMultipleAccounts` at one slot, answered
//! in key order.
//!
//! The first read covers the signer's invalidation record and the named encrypted stores. A
//! delegated request then reads them again with the Clock and the delegation rows the first read
//! made derivable, at a slot no older than the first: the read passes `minContextSlot`, so a node
//! behind the first read refuses it. The first read only locates the rows; every rule that
//! authorizes uses the last read.
//!
//! A refusal from a node behind is recoverable. The worker loop retries a Gateway request; an HTTP
//! request is not retried internally, so its caller receives `upstream_transient` and resubmits,
//! as for any transient failure of an EVM request.
//!
//! Reads are at confirmed commitment, not finalized. A grant observed on a supermajority-confirmed
//! fork is sufficient authorization; if that fork is rolled back, a share may already have been
//! released against state the canonical chain no longer holds. That risk is accepted (INVARIANTS
//! #46).

use super::SolanaPubkeyBytes;
use solana_account_decoder_client_types::{UiAccountData, UiAccountEncoding};
use solana_commitment_config::CommitmentConfig;
use solana_pubkey::Pubkey;
use solana_rpc_client::api::{
    client_error::ErrorKind, custom_error::JSON_RPC_SERVER_ERROR_MIN_CONTEXT_SLOT_NOT_REACHED,
    request::RpcError,
};
use solana_rpc_client::{api::config::RpcAccountInfoConfig, nonblocking::rpc_client::RpcClient};
use std::num::NonZeroUsize;
use std::{future::Future, sync::Arc, time::Duration};
use tokio::sync::Semaphore;
use url::Url;
use zama_solana_acl::{CLOCK_SYSVAR_ID, decode_clock_unix_timestamp};

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

/// What one read returned: its slot, and one account per requested key, in key order.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AccountsRead {
    pub slot: u64,
    pub accounts: Vec<Option<SnapshotAccount>>,
}

pub trait HostStateReader: Send + Sync {
    /// Reads `keys` at one slot no older than `min_context_slot`.
    fn read_accounts(
        &self,
        keys: &[SolanaPubkeyBytes],
        min_context_slot: Option<u64>,
    ) -> impl Future<Output = Result<AccountsRead, SnapshotError>> + Send;
}

/// Reads `keys` and holds the answer to one account per key, whichever reader served it.
pub async fn read_positional(
    reader: &impl HostStateReader,
    keys: &[SolanaPubkeyBytes],
    min_context_slot: Option<u64>,
) -> Result<AccountsRead, SnapshotError> {
    let read = reader.read_accounts(keys, min_context_slot).await?;
    if read.accounts.len() != keys.len() {
        return Err(SnapshotError::ResponseLengthMismatch {
            requested: keys.len(),
            returned: read.accounts.len(),
        });
    }
    Ok(read)
}

/// An address the Connector derived, with its canonical bump.
pub type DerivedAddress = (SolanaPubkeyBytes, u8);

/// An account read at an address the Connector derived.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObservedRow {
    pub key: SolanaPubkeyBytes,
    pub bump: u8,
    pub account: Option<SnapshotAccount>,
}

/// The two delegation rows of one delegated entry, and the Clock's Unix time they are judged at.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObservedRows {
    pub exact: ObservedRow,
    pub wildcard: ObservedRow,
    pub now: u64,
}

/// The delegation rows to read for the entry at `entry`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DelegationRowKeys {
    pub entry: usize,
    pub exact: DerivedAddress,
    pub wildcard: DerivedAddress,
}

/// One request entry as read: its named store and, for a delegated entry, its delegation rows.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObservedEntry {
    pub store: Option<SnapshotAccount>,
    pub delegation: Option<ObservedRows>,
}

/// Everything one read observed, by role.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HostObservation {
    pub slot: u64,
    pub watermark: ObservedRow,
    pub entries: Vec<ObservedEntry>,
}

/// Reads the signer's invalidation record, the named stores and, when `delegated` is not empty,
/// the Clock and those delegation rows. The keys are sent positionally, repeats included, so no
/// account can be taken for another.
pub async fn observe(
    reader: &impl HostStateReader,
    watermark: DerivedAddress,
    stores: &[SolanaPubkeyBytes],
    delegated: &[DelegationRowKeys],
    min_context_slot: Option<u64>,
) -> Result<HostObservation, SnapshotError> {
    let clock = (!delegated.is_empty()).then_some(CLOCK_SYSVAR_ID);
    let keys: Vec<_> = std::iter::once(watermark.0)
        .chain(clock)
        .chain(stores.iter().copied())
        .chain(
            delegated
                .iter()
                .flat_map(|rows| [rows.exact.0, rows.wildcard.0]),
        )
        .collect();
    let AccountsRead { slot, accounts } = read_positional(reader, &keys, min_context_slot).await?;
    let mut accounts = accounts.into_iter();
    let mut next = || accounts.next().expect("one account per key");
    let row = |(key, bump): DerivedAddress, account| ObservedRow { key, bump, account };

    let watermark = row(watermark, next());
    let now = if clock.is_some() {
        Some(unix_timestamp(next())?)
    } else {
        None
    };
    let mut entries: Vec<_> = stores
        .iter()
        .map(|_| ObservedEntry {
            store: next(),
            delegation: None,
        })
        .collect();
    if let Some(now) = now {
        for rows in delegated {
            entries[rows.entry].delegation = Some(ObservedRows {
                exact: row(rows.exact, next()),
                wildcard: row(rows.wildcard, next()),
                now,
            });
        }
    }
    Ok(HostObservation {
        slot,
        watermark,
        entries,
    })
}

/// The Clock's Unix time, which delegation expiry is checked against. The Clock always exists, so
/// a read without a decodable one comes from a bad node.
fn unix_timestamp(clock: Option<SnapshotAccount>) -> Result<u64, SnapshotError> {
    let clock = clock.ok_or(SnapshotError::MalformedClock)?;
    decode_clock_unix_timestamp(&clock.owner, &clock.data)
        .map_err(|_| SnapshotError::MalformedClock)
}

/// The host reader: Anza's `solana-rpc-client`, the official client, as alloy is for EVM hosts.
/// It is bounded as the EVM host provider is: a semaphore caps concurrent calls, and one deadline
/// covers the client's internal retries.
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
    async fn read_accounts(
        &self,
        keys: &[SolanaPubkeyBytes],
        min_context_slot: Option<u64>,
    ) -> Result<AccountsRead, SnapshotError> {
        let pubkeys: Vec<_> = keys.iter().copied().map(Pubkey::new_from_array).collect();
        // As for EVM host RPC, waiting for a concurrency permit does not count against the call
        // timeout.
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
                min_context_slot,
                ..Default::default()
            },
        );
        let response = tokio::time::timeout(self.timeout, call)
            .await
            .map_err(|_| SnapshotError::Unavailable {
                reason: format!("Solana account read timed out after {:?}", self.timeout),
            })?
            .map_err(|error| match error.kind() {
                ErrorKind::RpcError(RpcError::RpcResponseError { code, .. })
                    if *code == JSON_RPC_SERVER_ERROR_MIN_CONTEXT_SLOT_NOT_REACHED =>
                {
                    SnapshotError::NodeBehind
                }
                _ => SnapshotError::Unavailable {
                    reason: error.to_string(),
                },
            })?;
        // The UI-account method reports undecodable data as an error instead of as a missing
        // account. Every returned account is kept, so `read_positional` sees the node's count.
        let accounts = response
            .value
            .into_iter()
            .enumerate()
            .map(|(position, account)| {
                account
                    .map(|account| {
                        let malformed = || SnapshotError::Unavailable {
                            reason: format!("malformed Solana account at position {position}"),
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
            })
            .collect::<Result<Vec<_>, SnapshotError>>()?;
        Ok(AccountsRead {
            slot: response.context.slot,
            accounts,
        })
    }
}

/// A read of the host that failed. Every variant is the node's, so a later attempt may succeed.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum SnapshotError {
    #[error("host state read failed: {reason}")]
    Unavailable { reason: String },
    #[error("host state read returned {returned} accounts for {requested} keys")]
    ResponseLengthMismatch { requested: usize, returned: usize },
    #[error("host state read returned no decodable Clock sysvar")]
    MalformedClock,
    #[error("the node has not reached the slot of the request's first read")]
    NodeBehind,
}
