//! One signed decision record per operator per upgrade attempt.
//!
//! An operator records `quorum` after every track reached unanimity, or
//! `abort` after the deadline. It then acts only on the records it reads: it
//! cuts over after `quorum` from [`cutover_threshold`] members and rolls back
//! after `abort` from [`abort_threshold`] members. The two thresholds are set
//! so that both sets cannot exist while at most [`max_faulty`] members are
//! malicious or down (see `fleet_outcome`). Its own record is one vote among
//! the others: an operator that recorded `abort` still cuts over with the
//! fleet. Below both thresholds it waits.
//!
//! Every operator also republishes the records it has read (see [`Seen`]). A
//! record is signed, so it stays valid when a peer passes it on, and a member
//! that serves its record to one peer only cannot keep it from the others.

use alloy::primitives::{keccak256, Address, Signature};
use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use fhevm_engine_common::types::CoproSigner;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::manifest_consensus::{peer_http, ExecutionError};

/// Separates this signature from every other hash this signer produces.
const DOMAIN: &[u8] = b"fhevm/upgrade-decision/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Decision {
    Quorum,
    Abort,
}

/// One member of the attempt: the signer and the bucket that holds its record.
/// The bucket travels with the address, so a member that `GatewayConfig` drops
/// mid-attempt stays readable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Member {
    pub address: String,
    pub url: String,
}

/// The signed body. `members` is the set this operator used, and `anchors` the
/// blocks it agreed on, so a restart emits the cutover notifications from the
/// record itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Payload {
    pub proposal_id: String,
    pub proposal_block: i64,
    /// The release of the attempt. An operator on another image signs different
    /// values, so a reader names that operator.
    pub target_stack_version: String,
    pub target_consensus_version: i64,
    pub decision: Decision,
    pub signer: String,
    pub members: Vec<Member>,
    pub anchors: Vec<(i64, i64)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Record {
    #[serde(flatten)]
    pub payload: Payload,
    pub signature: String,
}

impl Payload {
    /// Hash the serialized body under the domain tag. JSON delimits every field,
    /// so no two payloads share a digest, and a field added later signs itself.
    fn digest(&self) -> [u8; 32] {
        let mut bytes = Vec::from(DOMAIN);
        bytes.extend_from_slice(&serde_json::to_vec(self).expect("payload serializes"));
        *keccak256(bytes)
    }

    /// Identity of the set. Two operators can hold different buckets for one
    /// member, so only the addresses decide whether records describe one attempt.
    pub(crate) fn addresses(&self) -> Vec<&str> {
        self.members.iter().map(|m| m.address.as_str()).collect()
    }
}

impl Record {
    /// True when `expected` signed the record, the record names this attempt,
    /// and its member set holds `expected`.
    fn is_valid_from(&self, expected: Address, proposal_id: &[u8], proposal_block: i64) -> bool {
        let name = name_of(expected);
        if self.payload.signer != name
            || self.payload.proposal_id != hex::encode(proposal_id)
            || self.payload.proposal_block != proposal_block
            || !self.payload.addresses().contains(&name.as_str())
        {
            return false;
        }
        let Ok(raw) = hex::decode(&self.signature) else {
            return false;
        };
        let Ok(signature) = Signature::try_from(raw.as_slice()) else {
            return false;
        };
        signature
            .recover_address_from_prehash(&self.payload.digest().into())
            .is_ok_and(|recovered| recovered == expected)
    }

    /// True when a member of `members` signed the record for this attempt.
    /// Used for a record that a peer passed on: the bucket it came from says
    /// nothing, so the signer is looked up in the member set.
    pub(crate) fn is_valid_from_member(
        &self,
        members: &[Member],
        proposal_id: &[u8],
        proposal_block: i64,
    ) -> bool {
        members
            .iter()
            .find(|member| member.address == self.payload.signer)
            .and_then(Member::parsed_address)
            .is_some_and(|signer| self.is_valid_from(signer, proposal_id, proposal_block))
    }
}

/// Every verified record this operator has read for one attempt, from the
/// signer's bucket or passed on by a peer. It holds at most one record per
/// member and decision. A faulty member can sign both decisions: it then
/// counts in both tallies, which the thresholds already allow for (see
/// [`fleet_outcome`]).
#[derive(Debug, Default)]
pub(crate) struct Seen(Vec<Record>);

impl Seen {
    /// Keeps `record` when it is the first one for its signer and decision.
    /// The caller has verified it. Returns whether it was kept.
    pub(crate) fn add(&mut self, record: Record) -> bool {
        let known = self.0.iter().any(|seen| {
            seen.payload.signer == record.payload.signer
                && seen.payload.decision == record.payload.decision
        });
        if !known {
            self.0.push(record);
        }
        !known
    }

    /// How many members signed `decision`.
    pub(crate) fn count(&self, decision: Decision) -> usize {
        self.with(decision).count()
    }

    pub(crate) fn with(&self, decision: Decision) -> impl Iterator<Item = &Record> {
        self.0
            .iter()
            .filter(move |record| record.payload.decision == decision)
    }

    pub(crate) fn has_signer(&self, address: &str) -> bool {
        self.0.iter().any(|record| record.payload.signer == address)
    }

    pub(crate) fn records(&self) -> &[Record] {
        &self.0
    }
}

impl Member {
    pub(crate) fn parsed_address(&self) -> Option<Address> {
        hex::decode(&self.address)
            .ok()
            .and_then(|raw| <[u8; 20]>::try_from(raw.as_slice()).ok())
            .map(Address::from)
    }
}

pub(crate) fn name_of(address: Address) -> String {
    hex::encode(address.as_slice())
}

/// Sorted by address, so every operator renders one set in one order.
pub(crate) fn members_of(members: &[(Address, String)]) -> Vec<Member> {
    let mut members: Vec<Member> = members
        .iter()
        .map(|(address, url)| Member {
            address: name_of(*address),
            url: url.clone(),
        })
        .collect();
    members.sort_by(|a, b| a.address.cmp(&b.address));
    members
}

pub(crate) fn key(proposal_id: &[u8], proposal_block: i64) -> String {
    keyed(&hex::encode(proposal_id), proposal_block, OUTCOME)
}

/// Where an operator republishes the records it has read (see [`Seen`]).
pub(crate) fn relay_key(proposal_id: &[u8], proposal_block: i64) -> String {
    keyed(&hex::encode(proposal_id), proposal_block, RELAY)
}

/// The key as shown in logs: the proposal id abbreviated like the state-hash
/// log key (first 2 + last 4 bytes). Display only - never a real object key.
fn log_key(proposal_id: &[u8], proposal_block: i64) -> String {
    keyed(
        &crate::state_hash::display_proposal_id(proposal_id),
        proposal_block,
        OUTCOME,
    )
}

/// This operator's own record.
const OUTCOME: &str = "outcome.json";
/// The records this operator has read, its own included.
const RELAY: &str = "seen.json";

fn keyed(proposal_hex: &str, proposal_block: i64, object: &str) -> String {
    format!("decision/proposal={proposal_hex}/attempt={proposal_block}/{object}")
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn sign(
    signer: &CoproSigner,
    proposal_id: &[u8],
    proposal_block: i64,
    target_stack_version: String,
    target_consensus_version: i64,
    decision: Decision,
    members: &[Member],
    anchors: Vec<(i64, i64)>,
) -> anyhow::Result<Record> {
    let payload = Payload {
        proposal_id: hex::encode(proposal_id),
        proposal_block,
        target_stack_version,
        target_consensus_version,
        decision,
        signer: name_of(signer.address()),
        members: members.to_vec(),
        anchors,
    };
    let signature = signer.sign_hash(&payload.digest().into()).await?;
    Ok(Record {
        payload,
        signature: hex::encode(signature.as_bytes()),
    })
}

/// Create this operator's record. A conditional create keeps the first value:
/// a restart must not replace a record a peer already read.
///
/// Every outcome is logged here with the key and the serialized record, like
/// the state-hash uploads: `info!` on a write (and on a kept first write),
/// `warn!` on a failure.
pub(crate) async fn put(
    client: &Client,
    bucket: &str,
    proposal_id: &[u8],
    record: &Record,
) -> anyhow::Result<()> {
    let key = key(proposal_id, record.payload.proposal_block);
    // Logged keys abbreviate the proposal id; the record JSON keeps it full.
    let log_key = log_key(proposal_id, record.payload.proposal_block);
    // The exact JSON uploaded as the object body, reused for the logs.
    let body = serde_json::to_string(record)?;
    let result = client
        .put_object()
        .bucket(bucket)
        .key(&key)
        .content_type("application/json")
        .if_none_match("*")
        .body(ByteStream::from(body.clone().into_bytes()))
        .send()
        .await;
    match result {
        Ok(_) => {
            info!(bucket, key = %log_key, decision = ?record.payload.decision, record = %body, "decision record written");
            Ok(())
        }
        Err(err)
            if err.as_service_error().and_then(ProvideErrorMetadata::code)
                == Some("PreconditionFailed") =>
        {
            info!(
                bucket,
                key = %log_key,
                "decision record already exists; keeping the first write"
            );
            Ok(())
        }
        Err(err) => {
            warn!(bucket, key = %log_key, record = %body, error = %err, "decision record write failed; retrying next pass");
            Err(anyhow::anyhow!("decision record write failed: {err}"))
        }
    }
}

/// Read this operator's own record from its own bucket. The record names the
/// members, so a restart recovers the attempt without the chain.
pub(crate) async fn get_own(
    client: &Client,
    bucket: &str,
    signer: Address,
    proposal_id: &[u8],
    proposal_block: i64,
) -> Option<Record> {
    let key = key(proposal_id, proposal_block);
    let object = client
        .get_object()
        .bucket(bucket)
        .key(&key)
        .send()
        .await
        .ok()?;
    let body = object.body.collect().await.ok()?.into_bytes();
    let record: Record = serde_json::from_slice(&body).ok()?;
    if !record.is_valid_from(signer, proposal_id, proposal_block) {
        warn!(
            bucket,
            key, "own decision record failed verification; ignoring"
        );
        return None;
    }
    Some(record)
}

/// A decision object of five members is a few KB. A peer serves these bodies,
/// so a larger one is refused before it is buffered.
const MAX_OBJECT_BYTES: usize = 256 * 1024;

/// The body of a peer's object, or `None` when it is absent, unreadable or
/// larger than [`MAX_OBJECT_BYTES`].
async fn read_body(http: &reqwest::Client, url: &str) -> Option<Vec<u8>> {
    let response = http.get(url).send().await.ok()?;
    if !response.status().is_success() {
        return None;
    }
    peer_http::read_capped(response, MAX_OBJECT_BYTES, url, || {
        ExecutionError::InternalError(format!("{url} exceeds {MAX_OBJECT_BYTES} bytes"))
    })
    .await
    .map_err(|err| warn!(url, error = %err, "decision object not read; ignoring"))
    .ok()
}

/// Read one member's record. An absent record, an unreadable one and one that
/// fails verification are the same answer: none of them lets this operator act.
pub(crate) async fn get(
    http: &reqwest::Client,
    member: &Member,
    proposal_id: &[u8],
    proposal_block: i64,
) -> Option<Record> {
    let address = member.parsed_address()?;
    let url = format!(
        "{}/{}",
        member.url.trim_end_matches('/'),
        key(proposal_id, proposal_block)
    );
    let record: Record = serde_json::from_slice(&read_body(http, &url).await?).ok()?;
    if !record.is_valid_from(address, proposal_id, proposal_block) {
        warn!(url, "decision record failed verification; ignoring");
        return None;
    }
    Some(record)
}

/// Republish the records this operator has read. The object is replaced as the
/// set grows. It carries no signature of its own: each record inside is signed
/// by its member, and a reader verifies each one.
pub(crate) async fn put_relay(
    client: &Client,
    bucket: &str,
    proposal_id: &[u8],
    proposal_block: i64,
    records: &[Record],
) -> anyhow::Result<()> {
    let key = relay_key(proposal_id, proposal_block);
    let body = serde_json::to_vec(records)?;
    client
        .put_object()
        .bucket(bucket)
        .key(&key)
        .content_type("application/json")
        .body(ByteStream::from(body))
        .send()
        .await
        .map_err(|err| {
            warn!(bucket, key, error = %err, "decision record relay write failed; retrying next pass");
            anyhow::anyhow!("decision record relay write failed: {err}")
        })?;
    info!(
        bucket,
        key,
        records = records.len(),
        "decision records relayed"
    );
    Ok(())
}

/// Read the records this operator republished before a restart, from its own
/// bucket. `Some` with no records when the object does not exist yet. `None`
/// when the read failed (timeout, S3 error): the caller must not overwrite the object then, or it
/// could drop records that no bucket serves any more.
pub(crate) async fn get_own_relay(
    client: &Client,
    bucket: &str,
    proposal_id: &[u8],
    proposal_block: i64,
) -> Option<Vec<Record>> {
    let key = relay_key(proposal_id, proposal_block);
    let object = match client.get_object().bucket(bucket).key(&key).send().await {
        Ok(object) => object,
        // S3 answers `AccessDenied` for an absent key when the role cannot
        // list the bucket, so both codes mean "not written yet".
        Err(err)
            if matches!(
                err.as_service_error().and_then(ProvideErrorMetadata::code),
                Some("NoSuchKey" | "AccessDenied")
            ) =>
        {
            return Some(Vec::new());
        }
        Err(err) => {
            warn!(bucket, key, error = %err, "own decision record relay not read; retrying next pass");
            return None;
        }
    };
    let body = object.body.collect().await.ok()?.into_bytes();
    Some(serde_json::from_slice(&body).unwrap_or_default())
}

/// True when this operator's relay holds every record it has counted: its
/// earlier relay was loaded and all `seen` records are written. An operator
/// acts only then. Once it cuts over it stops running the pass, so a record
/// it has not passed on by that point would never reach the slower peers.
pub(crate) fn relay_is_complete(own_relay_loaded: bool, relayed: usize, seen: usize) -> bool {
    own_relay_loaded && relayed == seen
}

/// Read the records one member republished. They are NOT verified here: the
/// caller checks each one against the member set. An absent or unreadable
/// object, or one with more than two records per member, yields nothing.
pub(crate) async fn get_relay(
    http: &reqwest::Client,
    member: &Member,
    proposal_id: &[u8],
    proposal_block: i64,
    members: usize,
) -> Vec<Record> {
    let url = format!(
        "{}/{}",
        member.url.trim_end_matches('/'),
        relay_key(proposal_id, proposal_block)
    );
    let Some(body) = read_body(http, &url).await else {
        return Vec::new();
    };
    let records: Vec<Record> = serde_json::from_slice(&body).unwrap_or_default();
    if records.len() > 2 * members {
        warn!(
            url,
            records = records.len(),
            "decision record relay is oversized; ignoring"
        );
        return Vec::new();
    }
    records
}

/// The fleet decision over the records read, assuming at most
/// [`max_faulty`] members are malicious or down.
///
/// A record lives in its signer's own bucket, so a malicious member can serve
/// `quorum` to one peer and `abort` (or nothing) to another. Both thresholds
/// therefore count signatures, and they are sized so that a cutover set and a
/// rollback set always share an honest member, who signs only one record:
/// `cutover_threshold + abort_threshold > members + max_faulty`. A cutover
/// seen by one honest operator then rules out a rollback seen by another,
/// whatever the faulty members serve to whom (fhevm-internal#2284). When all
/// honest members signed the same decision, they reach its threshold alone,
/// so `max_faulty` silent or flipped records cannot hold them. When the honest
/// members split below both thresholds, nothing is decided and the fleet
/// waits on blue.
///
/// `quorums` and `aborts` are the numbers of members that signed each
/// decision (see [`Seen::count`]). `None` means wait: an absent or unreadable
/// record is not a decision.
pub(crate) fn fleet_outcome(
    quorums: usize,
    aborts: usize,
    cutover_threshold: usize,
    abort_threshold: usize,
) -> Option<Decision> {
    // No members, no decision.
    if cutover_threshold == 0 || abort_threshold == 0 {
        return None;
    }
    // Both cannot hold under the assumption. If it is broken, stay on blue:
    // a rollback can be retried, a cutover cannot be undone.
    if aborts >= abort_threshold {
        return Some(Decision::Abort);
    }
    if quorums >= cutover_threshold {
        return Some(Decision::Quorum);
    }
    None
}

/// How many of `members` may be malicious or down. Records can be served
/// differently to different peers, so the bound is the classic
/// `members > 3 * max_faulty`: 0 of 3, 1 of 4, 1 of 5, 2 of 7.
pub(crate) fn max_faulty(members: usize) -> usize {
    members.saturating_sub(1) / 3
}

/// `members - max_faulty`: every honest member can reach it without the
/// faulty ones.
/// 3 of 3, 3 of 4, 4 of 5, 5 of 7.
pub(crate) fn cutover_threshold(members: usize) -> usize {
    members - max_faulty(members)
}

/// `2 * max_faulty + 1`: the smallest count that keeps
/// `cutover_threshold + abort_threshold > members + max_faulty`. 1 of 3,
/// 3 of 4, 3 of 5, 5 of 7.
pub(crate) fn abort_threshold(members: usize) -> usize {
    2 * max_faulty(members) + 1
}

/// The `(chain_id, block)` anchors to emit on a fleet cutover.
///
/// `own` holds the anchors this operator reached itself and wins per chain. An
/// operator that recorded `abort`, or has no record, may miss some: it takes
/// them from the `quorum` records of its peers. A peer anchor is used only
/// when the upgrade-controller would accept it (a host block inside that
/// chain's window, a Gateway block at or after `gw_start_block`), so a
/// malicious record cannot feed this operator an anchor that is then dropped.
pub(crate) fn fleet_anchors(
    own: &[(i64, i64)],
    peers: &[&[(i64, i64)]],
    windows: &[(i64, i64, i64)],
    gateway_chain_id: i64,
    gw_start_block: Option<i64>,
) -> Vec<(i64, i64)> {
    let pick = |chain_id: i64, accepts: &dyn Fn(i64) -> bool| {
        std::iter::once(own)
            .chain(peers.iter().copied())
            .flatten()
            .find(|(chain, block)| *chain == chain_id && accepts(*block))
            .copied()
    };
    let mut anchors: Vec<(i64, i64)> = windows
        .iter()
        .filter_map(|&(chain_id, start, end)| pick(chain_id, &|b| (start..=end).contains(&b)))
        .collect();
    anchors.extend(pick(gateway_chain_id, &|b| {
        gw_start_block.is_some_and(|start| b >= start)
    }));
    anchors
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::signers::local::PrivateKeySigner;

    fn addr(byte: u8) -> Address {
        Address::from([byte; 20])
    }

    fn member_list(addresses: &[Address]) -> Vec<Member> {
        members_of(
            &addresses
                .iter()
                .map(|a| (*a, format!("http://{}", name_of(*a))))
                .collect::<Vec<_>>(),
        )
    }

    async fn signed(
        key: &PrivateKeySigner,
        decision: Decision,
        members: &[Member],
    ) -> (Address, Record) {
        let signer: CoproSigner = std::sync::Arc::new(key.clone());
        let record = sign(
            &signer,
            &[0x01],
            5,
            "v0.15".to_owned(),
            2,
            decision,
            members,
            vec![(1, 100)],
        )
        .await
        .expect("sign");
        (key.address(), record)
    }

    #[tokio::test]
    async fn a_signed_record_verifies_only_for_its_own_signer_and_attempt() {
        let key = PrivateKeySigner::random();
        let members = member_list(&[key.address(), addr(9)]);
        let (address, record) = signed(&key, Decision::Quorum, &members).await;

        assert!(record.is_valid_from(address, &[0x01], 5));
        assert!(!record.is_valid_from(addr(9), &[0x01], 5));
        assert!(!record.is_valid_from(address, &[0x02], 5));
        assert!(!record.is_valid_from(address, &[0x01], 6));
    }

    #[tokio::test]
    async fn a_tampered_record_fails_verification() {
        let key = PrivateKeySigner::random();
        let members = member_list(&[key.address()]);
        let (address, mut record) = signed(&key, Decision::Quorum, &members).await;

        record.payload.decision = Decision::Abort;
        assert!(!record.is_valid_from(address, &[0x01], 5));
    }

    #[tokio::test]
    async fn a_changed_release_breaks_the_signature() {
        let key = PrivateKeySigner::random();
        let members = member_list(&[key.address()]);
        let (address, mut record) = signed(&key, Decision::Quorum, &members).await;

        record.payload.target_consensus_version = 3;
        assert!(!record.is_valid_from(address, &[0x01], 5));

        record.payload.target_consensus_version = 2;
        assert!(record.is_valid_from(address, &[0x01], 5));

        record.payload.target_stack_version = "v0.16".to_owned();
        assert!(!record.is_valid_from(address, &[0x01], 5));
    }

    #[tokio::test]
    async fn a_signer_outside_the_member_set_is_rejected() {
        let key = PrivateKeySigner::random();
        let members = member_list(&[addr(9)]);
        let (address, record) = signed(&key, Decision::Quorum, &members).await;

        assert!(!record.is_valid_from(address, &[0x01], 5));
    }

    /// A member list that changes must break the signature. An encoding without
    /// delimiters lets a long URL absorb the member that follows it.
    #[tokio::test]
    async fn folding_a_member_into_a_url_breaks_the_signature() {
        let key = PrivateKeySigner::from_slice(&[0x11; 32]).expect("key");
        let peer = addr(0xff);
        let members = member_list(&[key.address(), peer]);
        let (address, mut record) = signed(&key, Decision::Quorum, &members).await;
        let signed_digest = record.payload.digest();

        // The signer comes first, so its URL can absorb the peer behind it.
        assert_eq!(members[0].address, name_of(address));
        record.payload.members = vec![Member {
            address: name_of(address),
            url: format!("{}{}{}", members[0].url, members[1].address, members[1].url),
        }];

        assert_ne!(record.payload.digest(), signed_digest);
        assert!(!record.is_valid_from(address, &[0x01], 5));
    }

    #[test]
    fn member_lists_are_order_independent() {
        assert_eq!(
            member_list(&[addr(2), addr(1)]),
            member_list(&[addr(1), addr(2)])
        );
    }

    /// `fleet_outcome` over one optional decision per member.
    fn outcome(records: &[Option<Decision>], cutover: usize, abort: usize) -> Option<Decision> {
        let count = |wanted: Decision| records.iter().filter(|r| **r == Some(wanted)).count();
        fleet_outcome(
            count(Decision::Quorum),
            count(Decision::Abort),
            cutover,
            abort,
        )
    }

    #[test]
    fn no_members_decide_nothing() {
        assert_eq!(
            fleet_outcome(0, 0, cutover_threshold(0), abort_threshold(0)),
            None
        );
        assert_eq!(fleet_outcome(0, 0, 0, 0), None);
    }

    #[test]
    fn relay_key_sits_next_to_the_record() {
        assert_eq!(
            relay_key(&[0xab, 0xcd], 7),
            "decision/proposal=abcd/attempt=7/seen.json"
        );
    }

    #[test]
    fn an_operator_acts_only_once_its_relay_is_complete() {
        // Nothing read yet, or everything written: complete.
        assert!(relay_is_complete(true, 0, 0));
        assert!(relay_is_complete(true, 4, 4));
        // A failed write leaves a counted record unwritten.
        assert!(!relay_is_complete(true, 3, 4));
        // The earlier relay was not read after a restart.
        assert!(!relay_is_complete(false, 0, 0));
        assert!(!relay_is_complete(false, 4, 4));
    }

    /// A peer origin that serves `body` for every path.
    async fn origin(body: Vec<u8>) -> String {
        let app = axum::Router::new().fallback(move || {
            let body = body.clone();
            async move { body }
        });
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        url
    }

    #[tokio::test]
    async fn an_oversized_relay_is_refused() {
        let key = PrivateKeySigner::random();
        let mut members = member_list(&[key.address(), addr(9)]);
        let (_, record) = signed(&key, Decision::Quorum, &members).await;
        let http = reqwest::Client::new();

        let body = serde_json::to_vec(&[&record]).unwrap();
        members[0].url = origin(body.clone()).await;
        assert_eq!(
            get_relay(&http, &members[0], &[0x01], 5, 2).await,
            vec![record]
        );

        // Valid JSON, padded past the cap.
        let mut padded = body;
        padded.resize(MAX_OBJECT_BYTES + 1, b' ');
        members[0].url = origin(padded).await;
        assert!(get_relay(&http, &members[0], &[0x01], 5, 2)
            .await
            .is_empty());
    }

    /// A record passed on by a peer verifies against the member set, not
    /// against the bucket it was read from.
    #[tokio::test]
    async fn a_relayed_record_verifies_against_the_member_set() {
        let (member, outsider) = (PrivateKeySigner::random(), PrivateKeySigner::random());
        let members = member_list(&[member.address(), addr(9)]);
        let (_, record) = signed(&member, Decision::Quorum, &members).await;
        assert!(record.is_valid_from_member(&members, &[0x01], 5));
        assert!(!record.is_valid_from_member(&members, &[0x02], 5));
        assert!(!record.is_valid_from_member(&members, &[0x01], 6));

        // A signer outside the set, even one that names itself a member.
        let with_outsider = member_list(&[member.address(), outsider.address()]);
        let (_, record) = signed(&outsider, Decision::Quorum, &with_outsider).await;
        assert!(!record.is_valid_from_member(&members, &[0x01], 5));

        // A record that names a member but carries another key's signature.
        let (_, mut forged) = signed(&outsider, Decision::Quorum, &with_outsider).await;
        forged.payload.signer = name_of(member.address());
        assert!(!forged.is_valid_from_member(&with_outsider, &[0x01], 5));
    }

    /// One record per member and decision. A member that signs both decisions
    /// counts once in each tally.
    #[tokio::test]
    async fn seen_keeps_one_record_per_member_and_decision() {
        let (a, b) = (PrivateKeySigner::random(), PrivateKeySigner::random());
        let members = member_list(&[a.address(), b.address()]);
        let mut seen = Seen::default();

        assert!(seen.add(signed(&a, Decision::Quorum, &members).await.1));
        assert!(!seen.add(signed(&a, Decision::Quorum, &members).await.1));
        assert!(seen.add(signed(&a, Decision::Abort, &members).await.1));
        assert!(seen.add(signed(&b, Decision::Quorum, &members).await.1));

        assert_eq!(seen.count(Decision::Quorum), 2);
        assert_eq!(seen.count(Decision::Abort), 1);
        assert_eq!(seen.records().len(), 3);
        assert!(seen.has_signer(&name_of(b.address())));
        assert!(!seen.has_signer(&name_of(addr(9))));
    }

    /// Four members, one faulty. Two honest members signed `quorum`, the third
    /// timed out to `abort`, and the faulty member serves its `quorum` to the
    /// first honest member only. That member cuts over. Its relay hands the
    /// faulty member's record to the others, so they cut over too instead of
    /// waiting below both thresholds.
    #[tokio::test]
    async fn a_relayed_record_lets_the_slower_peers_follow_a_cutover() {
        let keys: Vec<PrivateKeySigner> = (0..4).map(|_| PrivateKeySigner::random()).collect();
        let members = member_list(&keys.iter().map(|k| k.address()).collect::<Vec<_>>());
        let (q, a) = (cutover_threshold(4), abort_threshold(4));
        let h1 = signed(&keys[0], Decision::Quorum, &members).await.1;
        let h2 = signed(&keys[1], Decision::Quorum, &members).await.1;
        let h3 = signed(&keys[2], Decision::Abort, &members).await.1;
        let faulty = signed(&keys[3], Decision::Quorum, &members).await.1;
        let decide = |seen: &Seen| {
            fleet_outcome(
                seen.count(Decision::Quorum),
                seen.count(Decision::Abort),
                q,
                a,
            )
        };

        // The first honest member reads the faulty record and cuts over.
        let mut first = Seen::default();
        for record in [&h1, &h2, &h3, &faulty] {
            first.add(record.clone());
        }
        assert_eq!(decide(&first), Some(Decision::Quorum));

        // Another honest member reads no faulty record: undecided.
        let mut other = Seen::default();
        for record in [&h1, &h2, &h3] {
            other.add(record.clone());
        }
        assert_eq!(decide(&other), None);

        // It then reads the first member's relay and verifies each record.
        for record in first.records() {
            assert!(record.is_valid_from_member(&members, &[0x01], 5));
            other.add(record.clone());
        }
        assert_eq!(decide(&other), Some(Decision::Quorum));
    }

    #[test]
    fn key_names_the_attempt() {
        assert_eq!(
            key(&[0xab, 0xcd], 7),
            "decision/proposal=abcd/attempt=7/outcome.json"
        );
        assert_ne!(key(&[0xab, 0xcd], 7), key(&[0xab, 0xcd], 8));
    }

    #[test]
    fn log_key_abbreviates_the_proposal_id() {
        let mut id = vec![0u8; 32];
        id[28..].copy_from_slice(&[0x00, 0x11, 0x11, 0x22]);
        assert_eq!(
            log_key(&id, 7),
            "decision/proposal=0000...00111122/attempt=7/outcome.json"
        );
        // Short ids print in full - nothing to elide.
        assert_eq!(log_key(&[0xab, 0xcd], 7), key(&[0xab, 0xcd], 7));
    }

    #[test]
    fn fleet_outcome_counts_records_against_both_thresholds() {
        use Decision::{Abort, Quorum};
        let (q, a) = (cutover_threshold(5), abort_threshold(5));
        assert_eq!((q, a), (4, 3));

        // Every member quorum, and one silent member: both cut over.
        assert_eq!(outcome(&[Some(Quorum); 5], q, a), Some(Quorum));
        assert_eq!(
            outcome(
                &[Some(Quorum), Some(Quorum), None, Some(Quorum), Some(Quorum)],
                q,
                a
            ),
            Some(Quorum)
        );
        // A member that flips to abort after a peer cut over does not hold the
        // slower peers in progress, and does not roll them back.
        assert_eq!(
            outcome(
                &[
                    Some(Quorum),
                    Some(Quorum),
                    Some(Abort),
                    Some(Quorum),
                    Some(Quorum)
                ],
                q,
                a
            ),
            Some(Quorum)
        );
        // Below both thresholds nothing is decided.
        assert_eq!(
            outcome(
                &[Some(Quorum), Some(Quorum), Some(Quorum), Some(Abort), None],
                q,
                a
            ),
            None
        );
        assert_eq!(
            outcome(
                &[Some(Abort), Some(Abort), Some(Quorum), Some(Quorum), None],
                q,
                a
            ),
            None
        );
        assert_eq!(outcome(&[None; 5], q, a), None);
        // The abort threshold rolls back.
        assert_eq!(
            outcome(
                &[Some(Abort), Some(Abort), Some(Abort), Some(Quorum), None],
                q,
                a
            ),
            Some(Abort)
        );
        assert_eq!(outcome(&[], q, a), None);
    }

    /// With three members no fault is tolerated: every record is needed to
    /// cut over and one abort rolls back.
    #[test]
    fn three_members_need_every_quorum_and_roll_back_on_one_abort() {
        use Decision::{Abort, Quorum};
        let (q, a) = (cutover_threshold(3), abort_threshold(3));
        assert_eq!(outcome(&[Some(Quorum); 3], q, a), Some(Quorum));
        assert_eq!(outcome(&[Some(Quorum), None, Some(Quorum)], q, a), None);
        assert_eq!(
            outcome(&[Some(Quorum), Some(Abort), Some(Quorum)], q, a),
            Some(Abort)
        );
    }

    #[test]
    fn thresholds_follow_members_greater_than_3_max_faulty() {
        // (members, max_faulty, cutover_threshold, abort_threshold)
        for (members, faulty, cutover, abort) in [
            (1, 0, 1, 1),
            (2, 0, 2, 1),
            (3, 0, 3, 1),
            (4, 1, 3, 3),
            (5, 1, 4, 3),
            (6, 1, 5, 3),
            (7, 2, 5, 5),
            (10, 3, 7, 7),
        ] {
            assert_eq!(max_faulty(members), faulty, "max_faulty for {members}");
            assert_eq!(cutover_threshold(members), cutover, "cutover for {members}");
            assert_eq!(abort_threshold(members), abort, "abort for {members}");
        }
        assert_eq!(max_faulty(0), 0);
        // A cutover set and a rollback set always share an honest member, and
        // the honest members alone reach either threshold.
        for members in 1..=50 {
            let faulty = max_faulty(members);
            assert!(cutover_threshold(members) + abort_threshold(members) > members + faulty);
            assert!(cutover_threshold(members) <= members - faulty);
            assert!(abort_threshold(members) <= members - faulty);
        }
    }

    #[test]
    fn fleet_anchors_fill_missing_chains_from_peer_quorum_records() {
        let windows = [(1, 100, 200), (2, 300, 400)];
        let (gw, gw_start) = (9, Some(50));
        let honest: &[(i64, i64)] = &[(1, 150), (2, 310), (9, 60)];

        // Own anchors win; a chain this operator never anchored comes from a peer.
        assert_eq!(
            fleet_anchors(&[(1, 120)], &[honest], &windows, gw, gw_start),
            vec![(1, 120), (2, 310), (9, 60)]
        );
        // An operator with no anchors of its own follows the peer record.
        assert_eq!(
            fleet_anchors(&[], &[honest], &windows, gw, gw_start),
            honest.to_vec()
        );
        // Anchors the controller would drop are skipped: a host block outside
        // its window, a Gateway block below gw_start_block, an unknown chain.
        let bad: &[(i64, i64)] = &[(1, 99), (2, 401), (9, 49), (7, 1)];
        assert_eq!(
            fleet_anchors(&[], &[bad, honest], &windows, gw, gw_start),
            honest.to_vec()
        );
        assert!(fleet_anchors(&[], &[bad], &windows, gw, gw_start).is_empty());
        // No gw_start_block: the Gateway anchor cannot be checked, so none.
        assert_eq!(
            fleet_anchors(&[], &[honest], &windows, gw, None),
            vec![(1, 150), (2, 310)]
        );
    }
}
