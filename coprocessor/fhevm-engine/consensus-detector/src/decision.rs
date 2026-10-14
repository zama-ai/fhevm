//! One signed decision record per operator per upgrade attempt.
//!
//! An operator records `prepared` after every track reached unanimity, or
//! `abort` after the deadline. It cuts over only after it reads `prepared` from
//! every member, and it rolls back after it reads `abort` from one member. Both
//! actions need a record, so a peer's silence makes this operator wait.

use alloy::primitives::{keccak256, Address, Signature};
use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use fhevm_engine_common::types::CoproSigner;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

/// Separates this signature from every other hash this signer produces.
const DOMAIN: &[u8] = b"fhevm/upgrade-decision/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Decision {
    Prepared,
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
    format!(
        "decision/proposal={}/attempt={proposal_block}/outcome.json",
        hex::encode(proposal_id)
    )
}

pub(crate) async fn sign(
    signer: &CoproSigner,
    proposal_id: &[u8],
    proposal_block: i64,
    decision: Decision,
    members: &[Member],
    anchors: Vec<(i64, i64)>,
) -> anyhow::Result<Record> {
    let payload = Payload {
        proposal_id: hex::encode(proposal_id),
        proposal_block,
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
pub(crate) async fn put(
    client: &Client,
    bucket: &str,
    proposal_id: &[u8],
    record: &Record,
) -> anyhow::Result<()> {
    let key = key(proposal_id, record.payload.proposal_block);
    let result = client
        .put_object()
        .bucket(bucket)
        .key(&key)
        .content_type("application/json")
        .if_none_match("*")
        .body(ByteStream::from(serde_json::to_vec(record)?))
        .send()
        .await;
    match result {
        Ok(_) => {
            info!(bucket, key, decision = ?record.payload.decision, "decision record written");
            Ok(())
        }
        Err(err)
            if err.as_service_error().and_then(ProvideErrorMetadata::code)
                == Some("PreconditionFailed") =>
        {
            Ok(())
        }
        Err(err) => Err(anyhow::anyhow!("decision record write failed: {err}")),
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
    let response = http.get(&url).send().await.ok()?;
    if !response.status().is_success() {
        return None;
    }
    let record: Record = serde_json::from_slice(&response.bytes().await.ok()?).ok()?;
    if !record.is_valid_from(address, proposal_id, proposal_block) {
        warn!(url, "decision record failed verification; ignoring");
        return None;
    }
    Some(record)
}

/// Rules 2 and 3 over one record per member. `Abort` wins, so an operator that
/// wrote `prepared` still rolls back. `None` means wait: an absent or unreadable
/// record is not a decision.
pub(crate) fn fleet_outcome(records: &[Option<Decision>]) -> Option<Decision> {
    if records.contains(&Some(Decision::Abort)) {
        return Some(Decision::Abort);
    }
    if !records.is_empty() && records.iter().all(|r| *r == Some(Decision::Prepared)) {
        return Some(Decision::Prepared);
    }
    None
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
        let record = sign(&signer, &[0x01], 5, decision, members, vec![(1, 100)])
            .await
            .expect("sign");
        (key.address(), record)
    }

    #[tokio::test]
    async fn a_signed_record_verifies_only_for_its_own_signer_and_attempt() {
        let key = PrivateKeySigner::random();
        let members = member_list(&[key.address(), addr(9)]);
        let (address, record) = signed(&key, Decision::Prepared, &members).await;

        assert!(record.is_valid_from(address, &[0x01], 5));
        assert!(!record.is_valid_from(addr(9), &[0x01], 5));
        assert!(!record.is_valid_from(address, &[0x02], 5));
        assert!(!record.is_valid_from(address, &[0x01], 6));
    }

    #[tokio::test]
    async fn a_tampered_record_fails_verification() {
        let key = PrivateKeySigner::random();
        let members = member_list(&[key.address()]);
        let (address, mut record) = signed(&key, Decision::Prepared, &members).await;

        record.payload.decision = Decision::Abort;
        assert!(!record.is_valid_from(address, &[0x01], 5));
    }

    #[tokio::test]
    async fn a_signer_outside_the_member_set_is_rejected() {
        let key = PrivateKeySigner::random();
        let members = member_list(&[addr(9)]);
        let (address, record) = signed(&key, Decision::Prepared, &members).await;

        assert!(!record.is_valid_from(address, &[0x01], 5));
    }

    /// A member list that changes must break the signature. An encoding without
    /// delimiters lets a long URL absorb the member that follows it.
    #[tokio::test]
    async fn folding_a_member_into_a_url_breaks_the_signature() {
        let key = PrivateKeySigner::from_slice(&[0x11; 32]).expect("key");
        let peer = addr(0xff);
        let members = member_list(&[key.address(), peer]);
        let (address, mut record) = signed(&key, Decision::Prepared, &members).await;
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

    #[test]
    fn key_names_the_attempt() {
        assert_eq!(
            key(&[0xab, 0xcd], 7),
            "decision/proposal=abcd/attempt=7/outcome.json"
        );
        assert_ne!(key(&[0xab, 0xcd], 7), key(&[0xab, 0xcd], 8));
    }

    #[test]
    fn abort_beats_prepared_and_a_gap_decides_nothing() {
        use Decision::{Abort, Prepared};
        assert_eq!(fleet_outcome(&[Some(Prepared); 3]), Some(Prepared));
        assert_eq!(
            fleet_outcome(&[Some(Prepared), Some(Abort), Some(Prepared)]),
            Some(Abort)
        );
        assert_eq!(fleet_outcome(&[Some(Abort), None]), Some(Abort));
        assert_eq!(fleet_outcome(&[Some(Prepared), None]), None);
        assert_eq!(fleet_outcome(&[None, None]), None);
        assert_eq!(fleet_outcome(&[]), None);
    }
}
