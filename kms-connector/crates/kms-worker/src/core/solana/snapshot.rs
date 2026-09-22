//! The atomic host-state snapshot: the only place in the authorization path that reads chain state.
//!
//! ## One observation point, and the discovery read
//!
//! Authorization must never be assembled from states that never coexisted on any fork. The way that
//! is guaranteed here is blunt: every rule is evaluated against one snapshot — the one produced by
//! the **last** account read. Nothing is merged, because a merged observation is not one
//! observation.
//!
//! For a request whose entries are all direct, that last read is also the only one: the encrypted
//! value accounts are named by address, the invalidation record by the signer and the config
//! singleton by the deployment, so every key is computable up front.
//!
//! A delegated entry breaks the up-front part. Its delegation record lives at a PDA seeded by
//! `(delegator, delegate, authority)`, and the authoritative
//! `authority` is a field of the encrypted store — a request cannot
//! supply it (see [`connector_utils::types::solana_request`]). So a first read is needed to learn it. That read is a
//! **discovery read**: it produces addresses, not decisions, and its account values are discarded.
//! The second read covers the first read's whole key set alongside the delegation records, and it
//! alone is what the rules see.
//!
//! Discarding the first read costs nothing and avoids a hazard. Requiring the two reads to agree —
//! same slot, identical bytes — would reject delegated requests at whatever rate the chain advances
//! between two round trips, and a slot is about 400 milliseconds, while proving nothing that a
//! single deciding snapshot does not already give. Nor can the discarded read smuggle a stale value
//! in: the delegation address it produced is re-derived from the deciding snapshot's own encrypted
//! value account inside [`super::delegation::check_delegation`], and an encrypted store that
//! resolves at a given address has exactly one `authority`, because that
//! field is one of the seeds the address is derived from. A discovery read
//! that named the wrong record therefore surfaces as a key the deciding snapshot never read,
//! reported as the key-planning defect it is.
//!
//! One thing is asked of the pair, and it is not agreement: order. The deciding read must not be
//! older than the discovery read ([`HostSnapshot::deciding_after`]). A read that goes backwards is
//! not a fresher view of the chain — behind a load balancer it is a second node that has fallen
//! behind — and taking it as the deciding observation reports as absent a grant the discovery read
//! demonstrably saw, blaming the delegation for what the read did. Absence is transient,
//! so the request would only burn attempts rather than die, but the verdict would still be the
//! wrong one: the disagreement is between two reads, and ordering names it as that —
//! [`SnapshotError::DecidingReadOlderThanDiscovery`] — instead of as a per-entry finding about a
//! record. Ordering costs one comparison and still compares no values: the chain advancing between
//! the reads remains fine, which is the case that actually happens.
//!
//! Reads number exactly one for a direct-only request, exactly two when any entry is delegated, and
//! never three: nothing after the deciding snapshot reads chain state at all. The leaf-proof read
//! that follows ([`super::proof`]) is not an observation of the chain — it fetches sibling paths
//! that are verified against this snapshot's peaks.
//!
//! ## Commitment
//!
//! `confirmed`, throughout — the recorded decision for this system. A grant observed on a
//! supermajority-confirmed fork authorizes, and the accepted trade-off is that a rollback of a
//! confirmed slot could resurrect a grant that canonically never existed.

use crate::core::solana_acl::SolanaPubkeyBytes;
use solana_account_decoder_client_types::{UiAccountData, UiAccountEncoding};
use solana_commitment_config::CommitmentConfig;
use solana_pubkey::Pubkey;
use solana_rpc_client::{api::config::RpcAccountInfoConfig, nonblocking::rpc_client::RpcClient};
use std::{
    collections::{BTreeMap, BTreeSet},
    future::Future,
    sync::Arc,
    time::Duration,
};
use url::Url;

/// The System program's id, which is the all-zero pubkey. The owner every account has before a
/// program takes it over, and the one this module reads as "nothing has been written here yet".
pub const SYSTEM_PROGRAM_ID: SolanaPubkeyBytes = [0; 32];

/// One account as the snapshot saw it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SnapshotAccount {
    /// The owning program.
    pub owner: SolanaPubkeyBytes,
    /// The account data, verbatim — including any bytes beyond the decoded body, which are
    /// legal for a realloc-grown account.
    pub data: Vec<u8>,
}

impl SnapshotAccount {
    /// Whether this is an address the host program has never written to, despite an account
    /// existing there: System-program-owned and empty.
    ///
    /// Every PDA address in this path is derivable by anyone, and a bare transfer to one creates
    /// exactly this account. So an account in this state carries no claim about host state — the
    /// only party who could have put data there is the program, and it has not.
    ///
    /// The host program applies the same rule when it reads an invalidation record it may have to
    /// create. It additionally refuses the System-owned-and-empty-but-executable combination; that
    /// is unreachable rather than unchecked here, because an executable account is owned by a
    /// loader and carries its program's bytes, and turning a PDA back into an empty System-owned
    /// account needs the PDA's own signature, which only its program can produce.
    pub fn is_uninitialized_pda(&self) -> bool {
        self.owner == SYSTEM_PROGRAM_ID && self.data.is_empty()
    }
}

/// An ordered, duplicate-free set of account keys to read.
///
/// Ordering is fixed so a response can be zipped back onto its request positionally, and
/// duplicates are collapsed so a request naming the same handle twice costs one account
/// read rather than two.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct SnapshotKeys(Vec<SolanaPubkeyBytes>);

impl SnapshotKeys {
    /// Collects keys, preserving first-seen order and dropping repeats.
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

    /// The keys, in read order.
    pub fn as_slice(&self) -> &[SolanaPubkeyBytes] {
        &self.0
    }

    /// Whether this set contains `key`.
    pub fn contains(&self, key: &SolanaPubkeyBytes) -> bool {
        self.0.contains(key)
    }

    /// How many accounts will be read.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether there is nothing to read.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Host state at one observation point.
///
/// Every authorization check takes this by reference and has no other access to state. An
/// account that does not exist is present as an explicit absence rather than missing from
/// the map, so "we never asked for this key" and "this account does not exist" cannot be
/// confused — the first is a bug in key planning, the second is rule h1.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HostSnapshot {
    observed_slot: u64,
    accounts: BTreeMap<SolanaPubkeyBytes, Option<SnapshotAccount>>,
}

impl HostSnapshot {
    /// Assembles a snapshot from a read: `accounts` positionally matches `keys`.
    pub fn new(
        observed_slot: u64,
        keys: &SnapshotKeys,
        accounts: Vec<Option<SnapshotAccount>>,
    ) -> Result<Self, SnapshotError> {
        if accounts.len() != keys.len() {
            return Err(SnapshotError::ResponseLengthMismatch {
                requested: keys.len(),
                returned: accounts.len(),
            });
        }
        Ok(Self {
            observed_slot,
            accounts: keys.as_slice().iter().copied().zip(accounts).collect(),
        })
    }

    /// The slot this state was observed at. The one clock-like value in authorization, and
    /// it comes from the response rather than from any local notion of time.
    pub fn observed_slot(&self) -> u64 {
        self.observed_slot
    }

    /// The account at `key`, or `None` if it does not exist.
    ///
    /// Fails if `key` was never read: that is a key-planning bug, and answering "absent"
    /// would turn it into a silent transient rejection.
    pub fn account(
        &self,
        key: &SolanaPubkeyBytes,
    ) -> Result<Option<&SnapshotAccount>, SnapshotError> {
        match self.accounts.get(key) {
            Some(account) => Ok(account.as_ref()),
            None => Err(SnapshotError::KeyNotInSnapshot { key: *key }),
        }
    }

    /// The keys this snapshot covers, in sorted order.
    pub fn keys(&self) -> Vec<SolanaPubkeyBytes> {
        self.accounts.keys().copied().collect()
    }

    /// Takes this read as the deciding observation, given the discovery read that preceded it.
    ///
    /// The only condition is order: a deciding read older than the discovery read is a node that
    /// has fallen behind rather than a later view of the chain, and judging a request on it would
    /// terminally reject grants the discovery read already saw. Advancing is fine and expected;
    /// no value of either read is compared.
    ///
    /// Consuming `self` is the point: the deciding snapshot is obtained by passing this gate, so
    /// the sequence cannot be assembled without stating which read decides.
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

/// The single reader abstraction of the authorization path.
///
/// Authorization is generic over it, which is what lets a test drive the whole pipeline
/// against canned state and count the reads. Production has exactly one implementation
/// ([`SolanaRpcClient`]); nothing else in this module tree performs I/O.
pub trait HostStateReader: Send + Sync {
    /// Reads every key at `confirmed` commitment, returning one observed slot.
    fn read_accounts(
        &self,
        keys: &SnapshotKeys,
    ) -> impl Future<Output = Result<HostSnapshot, SnapshotError>> + Send;
}

/// Plans the first read: the deployment's config singleton, the invalidation record of the request
/// signer, and one encrypted store per entry.
///
/// Pure, and total over any validated request: every key here is derivable from the request and the
/// deployment alone. There is no scan in the authorization path, so the plan is the complete set of
/// accounts the direct branch will ever look at.
///
/// The config singleton is one key however many entries a request names — it is the deployment's,
/// not the request's — which is what lets the pause switch be read without a round trip of its own.
pub fn plan_first_read(
    request: &connector_utils::types::solana_request::SolanaUserDecryptRequest,
    deployment: &super::deployment::DeploymentIdentity,
) -> SnapshotKeys {
    let program_id = deployment.program_id();
    let signer = *request.permit().user_pubkey().as_bytes();
    let (host_config_key, _) = crate::core::solana_acl::host_config_address(program_id);
    let (watermark_key, _) = super::watermark::permit_invalidation_address(program_id, signer);
    let encrypted_stores = request
        .handles()
        .iter()
        .map(|entry| entry.encrypted_store());
    SnapshotKeys::new(
        [host_config_key, watermark_key]
            .into_iter()
            .chain(encrypted_stores),
    )
}

/// Plans the second read: the first read's key set minus the config singleton, plus the delegation
/// records whose addresses the discovery read has just made computable.
///
/// The first set is carried over because the second read is the one every rule is evaluated
/// against, so it has to hold the encrypted stores and the invalidation record too — not in
/// order to compare the two reads, which this path deliberately does not do (see the module
/// documentation). Starting from `first` is what makes the coverage a property of this function
/// rather than a discipline of its callers.
///
/// The one key dropped is the config singleton, and it is dropped because it is already spent: the
/// pause switch is decided on the first read ([`super::pause`]), which is the only rule that does
/// not wait for the deciding observation. Carrying it anyway would cost the read an account it no
/// longer uses, and the worst-case delegated request — three accounts per entry plus the signer's
/// invalidation record — is sized to saturate the RPC's hundred-account limit exactly.
pub fn plan_second_read(
    first: &SnapshotKeys,
    deployment: &super::deployment::DeploymentIdentity,
    delegation_keys: impl IntoIterator<Item = SolanaPubkeyBytes>,
) -> SnapshotKeys {
    let (host_config_key, _) =
        crate::core::solana_acl::host_config_address(deployment.program_id());
    SnapshotKeys::new(
        first
            .as_slice()
            .iter()
            .copied()
            .filter(|key| key != &host_config_key)
            .chain(delegation_keys),
    )
}

/// Confirmed account reads through the asynchronous Solana RPC client.
#[derive(Clone)]
pub struct SolanaRpcClient {
    client: Arc<RpcClient>,
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
    pub fn new(url: Url, timeout: Duration) -> Self {
        Self {
            client: Arc::new(RpcClient::new_with_timeout_and_commitment(
                url.to_string(),
                timeout,
                CommitmentConfig::confirmed(),
            )),
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
        // The UI-account method preserves malformed data as an error below, rather than
        // collapsing a failed decode into a missing account (or panicking inside the SDK).
        let response = tokio::time::timeout(
            self.timeout,
            self.client.get_multiple_ui_accounts_with_config(
                &pubkeys,
                RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    commitment: Some(CommitmentConfig::confirmed()),
                    ..Default::default()
                },
            ),
        )
        .await
        .map_err(|error| SnapshotError::Unavailable {
            reason: format!("Solana account read timed out: {error}"),
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
            })
            .collect::<Result<Vec<_>, SnapshotError>>()?;
        HostSnapshot::new(response.context.slot, keys, accounts)
    }
}

/// Why a snapshot could not be taken.
///
/// There is no variant for "the two reads disagreed": the rules are evaluated against the last
/// read alone, so two reads are never compared and never combined. The one thing asked of the
/// pair is their order, and that is the variant below.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum SnapshotError {
    /// The RPC could not be reached, or answered with an error or an unparsable body.
    #[error("host state read failed: {reason}")]
    Unavailable {
        /// What went wrong, for the log.
        reason: String,
    },
    /// The response did not carry one entry per requested key.
    #[error("host state read returned {returned} accounts for {requested} keys")]
    ResponseLengthMismatch {
        /// Keys requested.
        requested: usize,
        /// Accounts returned.
        returned: usize,
    },
    /// A check asked for an account that was never planned. A bug in key planning, not a
    /// property of chain state.
    #[error("account {key:?} was never read")]
    KeyNotInSnapshot {
        /// The key that was asked for.
        key: SolanaPubkeyBytes,
    },
    /// The deciding read observed an earlier slot than the discovery read that preceded it, so it
    /// is a node that has fallen behind rather than a later state.
    #[error(
        "the deciding read observed slot {deciding_slot}, older than the discovery read's {discovery_slot}"
    )]
    DecidingReadOlderThanDiscovery {
        /// Where the discovery read landed.
        discovery_slot: u64,
        /// Where the deciding read landed.
        deciding_slot: u64,
    },
}
