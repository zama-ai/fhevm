//! The atomic host-state snapshot: the only place in the authorization path that reads
//! chain state.
//!
//! ## One observation point, and the discovery read
//!
//! Authorization must never be assembled from states that never coexisted on any fork. The
//! way that is guaranteed here is blunt: every rule is evaluated against one snapshot — the
//! one produced by the **last** account read. Nothing is merged, because a merged observation
//! is not one observation.
//!
//! For a request whose entries are all direct, that last read is also the only one: the
//! lineage accounts are named by their `valueKey`s and the invalidation record by the signer,
//! so every key is computable up front.
//!
//! A delegated entry breaks the up-front part. Its delegation record lives at a PDA seeded by
//! `(delegator, delegate, encrypted_value_account_authority)`, and the authoritative `encrypted_value_account_authority` is a field of
//! the lineage account — a request cannot supply it (see [`super::request`]). So a first read
//! is needed to learn it. That read is a **discovery read**: it produces addresses, not
//! decisions, and its account values are discarded. The second read covers the first read's
//! whole key set alongside the delegation records, and it alone is what the rules see.
//!
//! Discarding the first read costs nothing and avoids a hazard. Requiring the two reads to
//! agree — same slot, identical bytes — would reject delegated requests at whatever rate the
//! chain advances between two round trips, and a slot is about 400 milliseconds, while proving
//! nothing that a single deciding snapshot does not already give. Nor can the discarded read
//! smuggle a stale value in: the delegation address it produced is re-derived from the
//! deciding snapshot's own lineage inside [`super::delegation::check_delegation`], and a
//! lineage that resolves at a given address has exactly one `encrypted_value_account_authority`, because that field
//! is part of the `valueKey` preimage the address is derived from. A discovery read that named
//! the wrong record therefore surfaces as a key the deciding snapshot never read, reported as
//! the key-planning defect it is.
//!
//! One thing is asked of the pair, and it is not agreement: order. The deciding read must not be
//! older than the discovery read ([`HostSnapshot::deciding_after`]). A read that goes backwards is
//! not a fresher view of the chain — behind a load balancer it is a second node that has fallen
//! behind — and taking it as the deciding observation turns a grant that demonstrably exists into a
//! terminal rejection: a delegation record written between the two reads reads as absent from the
//! older one, and a record the discovery read already saw reads as newer than the observation.
//! Both of those are terminal, so one lagging node would kill a valid request for good. Ordering
//! costs one comparison and still compares no values: the chain advancing between the reads
//! remains fine, which is the case that actually happens.
//!
//! Reads number exactly one for a direct-only request, exactly two when any entry is
//! delegated, and never three: nothing after the deciding snapshot reads state at all.
//!
//! ## Commitment
//!
//! `confirmed`, throughout — the recorded decision for this system. A grant observed on a
//! supermajority-confirmed fork authorizes, and the accepted trade-off is that a rollback of
//! a confirmed slot could resurrect a grant that canonically never existed.

use crate::core::solana_acl::SolanaPubkeyBytes;
use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use serde_json::Value;
use solana_pubkey::Pubkey;
use std::collections::{BTreeMap, BTreeSet};
use std::future::Future;
use url::Url;

/// The commitment level of every authorization read.
pub use crate::core::solana_v2_fetcher::SOLANA_COMMITMENT_CONFIRMED;

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
/// ([`RpcHostStateReader`]); nothing else in this module tree performs I/O.
pub trait HostStateReader: Send + Sync {
    /// Reads every key at `confirmed` commitment, returning one observed slot.
    fn read_accounts(
        &self,
        keys: &SnapshotKeys,
    ) -> impl Future<Output = Result<HostSnapshot, SnapshotError>> + Send;
}

/// Plans the first read: the invalidation record of the request signer, plus one lineage
/// account per entry.
///
/// Pure, and total over any validated request: every key here is derivable from the request
/// and the deployment alone. There is no scan in the authorization path, so the plan is the
/// complete set of accounts the direct branch will ever look at.
pub fn plan_first_read(
    request: &super::request::SolanaUserDecryptRequest,
    deployment: &super::deployment::DeploymentIdentity,
) -> SnapshotKeys {
    let program_id = deployment.program_id();
    let signer = *request.permit().user_pubkey().as_bytes();
    let (watermark_key, _) = super::watermark::permit_invalidation_address(program_id, signer);
    let lineages = request.handles().iter().map(|entry| {
        let (account_key, _) = super::lineage::lineage_address(program_id, entry.encrypted_value_id());
        account_key
    });
    SnapshotKeys::new(std::iter::once(watermark_key).chain(lineages))
}

/// Plans the second read: the first read's key set, unchanged, plus the delegation records
/// whose addresses the discovery read has just made computable.
///
/// The first set is carried over because the second read is the one every rule is evaluated
/// against, so it has to hold the lineages and the invalidation record too — not in order to
/// compare the two reads, which this path deliberately does not do (see the module
/// documentation). Starting from `first` is what makes the coverage a property of this function
/// rather than a discipline of its callers.
pub fn plan_second_read(
    first: &SnapshotKeys,
    delegation_keys: impl IntoIterator<Item = SolanaPubkeyBytes>,
) -> SnapshotKeys {
    SnapshotKeys::new(first.as_slice().iter().copied().chain(delegation_keys))
}

/// Builds the JSON-RPC `getMultipleAccounts` body, pinned to `confirmed` commitment and
/// base64 encoding. Split out so the pinning is assertable without a live RPC.
pub fn multiple_accounts_request_body(keys: &SnapshotKeys) -> serde_json::Value {
    let addresses: Vec<String> = keys
        .as_slice()
        .iter()
        .map(|key| Pubkey::new_from_array(*key).to_string())
        .collect();
    serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getMultipleAccounts",
        "params": [
            addresses,
            {
                "encoding": "base64",
                "commitment": SOLANA_COMMITMENT_CONFIRMED,
            }
        ],
    })
}

/// Parses a `getMultipleAccounts` response into a snapshot.
///
/// The observed slot is the response's `context.slot` — the state's own account of when it
/// was observed, never a locally chosen slot.
pub fn parse_multiple_accounts_response(
    body: &str,
    keys: &SnapshotKeys,
) -> Result<HostSnapshot, SnapshotError> {
    let json: Value = serde_json::from_str(body)
        .map_err(|error| unavailable(format!("response is not valid JSON: {error}")))?;
    if let Some(error) = json.get("error") {
        return Err(unavailable(format!("RPC returned an error: {error}")));
    }
    let result = json
        .get("result")
        .ok_or_else(|| unavailable("response carries no result".to_string()))?;
    let observed_slot = result
        .get("context")
        .and_then(|context| context.get("slot"))
        .and_then(Value::as_u64)
        .ok_or_else(|| unavailable("response carries no context slot".to_string()))?;
    let values = result
        .get("value")
        .and_then(Value::as_array)
        .ok_or_else(|| unavailable("response carries no account list".to_string()))?;
    if values.len() != keys.len() {
        return Err(SnapshotError::ResponseLengthMismatch {
            requested: keys.len(),
            returned: values.len(),
        });
    }
    let accounts = values
        .iter()
        .map(parse_account)
        .collect::<Result<Vec<_>, _>>()?;
    HostSnapshot::new(observed_slot, keys, accounts)
}

/// Parses one element of the `value` array: `null` is an account that does not exist, which is
/// an observation and not a failure.
///
/// Temporary duplication: the single-account fetcher of the proof-of-concept path
/// ([`crate::core::solana_v2_fetcher`]) carries the same owner and base64 decoding behind private
/// helpers. Reusing them would mean editing that module while it is still live, and this series
/// leaves the old path untouched. Both readers collapse into one when the old path is removed.
fn parse_account(value: &Value) -> Result<Option<SnapshotAccount>, SnapshotError> {
    if value.is_null() {
        return Ok(None);
    }
    let owner = value
        .get("owner")
        .and_then(Value::as_str)
        .ok_or_else(|| unavailable("account carries no owner".to_string()))?;
    let owner = Pubkey::try_from(owner)
        .map_err(|error| unavailable(format!("account owner '{owner}' is not a pubkey: {error}")))?
        .to_bytes();
    let encoded = value
        .get("data")
        .and_then(Value::as_array)
        .ok_or_else(|| unavailable("account data is not [payload, encoding]".to_string()))?;
    let encoding = encoded.get(1).and_then(Value::as_str).unwrap_or_default();
    if encoding != "base64" {
        return Err(unavailable(format!(
            "account data arrived as '{encoding}', the request asked for base64"
        )));
    }
    let payload = encoded
        .first()
        .and_then(Value::as_str)
        .ok_or_else(|| unavailable("account data carries no payload".to_string()))?;
    let data = BASE64_STANDARD
        .decode(payload)
        .map_err(|error| unavailable(format!("account data does not base64-decode: {error}")))?;
    Ok(Some(SnapshotAccount { owner, data }))
}

/// A read that produced no observation at all. Every parse failure lands here: a response the
/// Connector cannot read says nothing about any account, which is not the same as an account
/// being absent.
fn unavailable(reason: String) -> SnapshotError {
    SnapshotError::Unavailable { reason }
}

/// The production reader: one `getMultipleAccounts` per call against the configured host
/// RPC.
#[derive(Clone, Debug)]
pub struct RpcHostStateReader {
    url: Url,
    client: reqwest::Client,
}

impl RpcHostStateReader {
    /// Binds the reader to an RPC endpoint.
    pub fn new(url: Url, client: reqwest::Client) -> Self {
        Self { url, client }
    }
}

impl HostStateReader for RpcHostStateReader {
    async fn read_accounts(&self, keys: &SnapshotKeys) -> Result<HostSnapshot, SnapshotError> {
        let body = serde_json::to_vec(&multiple_accounts_request_body(keys))
            .map_err(|error| unavailable(format!("request body does not serialize: {error}")))?;
        let response = self
            .client
            .post(self.url.clone())
            .header("content-type", "application/json")
            .body(body)
            .send()
            .await
            .map_err(|error| unavailable(format!("request failed: {error}")))?
            .error_for_status()
            .map_err(|error| unavailable(format!("RPC answered with an HTTP error: {error}")))?
            .text()
            .await
            .map_err(|error| unavailable(format!("response body could not be read: {error}")))?;
        parse_multiple_accounts_response(&response, keys)
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
