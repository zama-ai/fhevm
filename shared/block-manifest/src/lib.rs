//! Canonical block-manifest commitments shared by publishers and verifiers.
//!
//! JSON is the envelope. What is signed is `keccak256(canonical_bytes)`, a raw
//! prehash, without the EIP-191/personal-sign prefix. Signatures are emitted as
//! 65 bytes (`r || s || v`, with `v` equal to 27 or 28), encoded as a `0x`-prefixed
//! hex string in JSON. Domain tags are UTF-8 strings
//! (`FHEVM manifest block content v1`, etc.) hashed as raw bytes, not length-prefixed
//! and not padded to eight bytes. They separate block content, detailed range,
//! compact history, and the full payload.
//!
//! `consensus_epoch` is a relative path (slashes allowed; see
//! `LEGACY_CONSENSUS_EPOCH`), escaped with [`escape_consensus_epoch`] only in S3
//! keys. The signed identity is unchanged. It isolates verification
//! across breaking upgrades. `revision` is an independent immutable observation
//! at that publication identity; higher is newer. `HistoricalRange.digest` is
//! a signed opaque commitment — `validate()` does not recompute it from
//! children, which are out of band.

//!
//! Producers build ordered descriptors and ranges, then call
//! [`ManifestPayload::sign`] and serialize the returned [`SignedManifest`].
//! Consumers enforce [`MAX_MANIFEST_BYTES`] while reading the JSON body, then
//! deserialize and call [`SignedManifest::verify`]. Serde deserialization alone
//! does not validate commitments or signatures. JSON whitespace and field order
//! do not affect the canonical digest; consume the typed fields, not unknown
//! JSON fields that Serde discards.
//!
//! Verification recomputes detailed block/range digests and checks range geometry
//! and the signature against the payload's publisher. Callers must additionally
//! check that publisher, epoch, chain, context, block identity, and object key
//! match the requested publication. Historical roots require separate evidence:
//! their children are not included here. A computed descriptor commits to
//! ciphertext digests; it does not prove ciphertext availability in S3.

use alloy_primitives::{Address, B256, Signature, U256};
use alloy_signer::Signer;
pub use ciphertext_attestation::CiphertextFormat;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

/// UTF-8 domain tags hashed as raw string bytes: not length-prefixed, not
/// padded. The protocol version is part of the name so a later encoding can
/// use a different string without a width constraint.
pub const BLOCK_CONTENT_DOMAIN_TAG: &str = "FHEVM manifest block content v1";
pub const DETAILED_RANGE_DOMAIN_TAG: &str = "FHEVM manifest detailed range v1";
pub const DYADIC_RANGE_DOMAIN_TAG: &str = "FHEVM manifest dyadic range v1";
pub const MANIFEST_DOMAIN_TAG: &str = "FHEVM manifest v1";
/// Maximum permitted serialized JSON body size for one signed manifest.
///
/// Producers must check serialized output; consumers must bound reads before
/// deserialization, including streamed bodies with no trusted Content-Length.
/// This crate exposes no bounded JSON parser. Serde, [`ManifestPayload::validate`],
/// and [`SignedManifest::verify`] do not enforce this byte limit.
pub const MAX_MANIFEST_BYTES: usize = 16 * 1024 * 1024;
/// Genesis consensus epoch before the first `CoprocessorUpgradeProposed`.
/// Same value for every starting stack version (`legacy`), not the crate
/// version. Frozen: never rename it. Subsequent epochs identify breaking
/// upgrades using `{version}/block_{n}`, where `n` is the finalized proposal's
/// block number. Selecting the active epoch is the caller's responsibility.
pub const LEGACY_CONSENSUS_EPOCH: &str = "legacy";
const MAX_CONSENSUS_EPOCH_BYTES: usize = 256;

mod hex_bytes {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{}", hex::encode(bytes)))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let value = value.strip_prefix("0x").unwrap_or(&value);
        hex::decode(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
#[repr(u8)]
pub enum ManifestVersion {
    V1 = 1,
}

impl TryFrom<u8> for ManifestVersion {
    type Error = ManifestError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::V1),
            other => Err(ManifestError::UnsupportedVersion(other)),
        }
    }
}

impl From<ManifestVersion> for u8 {
    fn from(value: ManifestVersion) -> Self {
        value as u8
    }
}

/// Consensus outcome for one allowed handle.
///
/// JSON is internally tagged as `"status": "computed" | "error" | "uncomputed"`.
/// Error uses the field name `error_message` (omitted when absent).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum CiphertextStatus {
    /// Ciphertext material was computed. Digests commit to its bytes without
    /// asserting that an external storage service currently serves them.
    Computed {
        keyset_id: U256,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        gateway_key_id: Option<U256>,
        ct64_digest: B256,
        ct128_digest: B256,
        ct128_format: CiphertextFormat,
    },
    /// Computation ended in a terminal error; no ciphertext digests are carried.
    Error {
        /// Publisher-provided diagnostic text, signed with the manifest but
        /// excluded from the block content digest. Different wording does not
        /// create ciphertext-consensus drift.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        error_message: Option<String>,
    },
    /// No ciphertext result was available when this observation was sealed.
    /// This is distinct from a terminal error and does not predict later
    /// observations. The decision to publish this status is outside the wire contract.
    Uncomputed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockCiphertextDescriptor {
    pub handle: B256,
    #[serde(flatten)]
    pub status: CiphertextStatus,
}

impl BlockCiphertextDescriptor {
    pub fn computed(
        handle: B256,
        keyset_id: U256,
        gateway_key_id: Option<U256>,
        ct64_digest: B256,
        ct128_digest: B256,
        ct128_format: CiphertextFormat,
    ) -> Self {
        Self {
            handle,
            status: CiphertextStatus::Computed {
                keyset_id,
                gateway_key_id,
                ct64_digest,
                ct128_digest,
                ct128_format,
            },
        }
    }

    /// Descriptor for an allowed handle whose computation reached a terminal
    /// error and therefore has no ciphertext material.
    pub fn from_computation_error(handle: B256, error_message: Option<String>) -> Self {
        Self {
            handle,
            status: CiphertextStatus::Error { error_message },
        }
    }

    /// Descriptor for an allowed handle with no ciphertext result available
    /// at the time of this observation.
    pub fn from_uncomputed(handle: B256) -> Self {
        Self {
            handle,
            status: CiphertextStatus::Uncomputed,
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self.status, CiphertextStatus::Error { .. })
    }

    pub fn is_uncomputed(&self) -> bool {
        matches!(self.status, CiphertextStatus::Uncomputed)
    }

    pub fn error_message(&self) -> Option<&str> {
        match &self.status {
            CiphertextStatus::Error { error_message } => error_message.as_deref(),
            _ => None,
        }
    }

    pub fn keyset_id(&self) -> Option<U256> {
        match &self.status {
            CiphertextStatus::Computed { keyset_id, .. } => Some(*keyset_id),
            _ => None,
        }
    }

    pub fn gateway_key_id(&self) -> Option<U256> {
        match &self.status {
            CiphertextStatus::Computed { gateway_key_id, .. } => *gateway_key_id,
            _ => None,
        }
    }

    pub fn ct64_digest(&self) -> Option<B256> {
        match &self.status {
            CiphertextStatus::Computed { ct64_digest, .. } => Some(*ct64_digest),
            _ => None,
        }
    }

    pub fn ct128_digest(&self) -> Option<B256> {
        match &self.status {
            CiphertextStatus::Computed { ct128_digest, .. } => Some(*ct128_digest),
            _ => None,
        }
    }

    pub fn ct128_format(&self) -> Option<CiphertextFormat> {
        match &self.status {
            CiphertextStatus::Computed { ct128_format, .. } => Some(*ct128_format),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestBlockEntry {
    pub block_number: U256,
    pub block_hash: B256,
    pub parent_block_hash: B256,
    pub block_content_digest: B256,
    pub ciphertexts: Vec<BlockCiphertextDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetailedRange {
    pub first_block_number: U256,
    pub last_block_number: U256,
    pub digest: B256,
    pub blocks: Vec<ManifestBlockEntry>,
}

/// Compact commitment to an inclusive interval preceding the detailed range.
/// Validation checks canonical ordering, boundaries, and scale. The digest is
/// signed but cannot be recomputed without historical evidence outside this body.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalRange {
    pub start_block_number: U256,
    pub end_block_number: U256,
    /// Nominal width is `2^scale` blocks. Only the oldest interval may be
    /// shortened at the beginning of the epoch.
    pub scale: u32,
    pub end_block_hash: B256,
    pub digest: B256,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestPayload {
    pub version: ManifestVersion,
    /// Original UTF-8 epoch identity, signed exactly as supplied. Only its S3
    /// representation is escaped; see [`escape_consensus_epoch`].
    pub consensus_epoch: String,
    pub publisher: Address,
    pub coprocessor_context_id: U256,
    pub host_chain_id: U256,
    pub publication_block_number: U256,
    pub publication_block_hash: B256,
    pub publication_parent_block_hash: B256,
    /// Independent immutable observation at this publication identity.
    /// Higher values are later observations of the same block.
    pub revision: u64,
    pub detailed_range: DetailedRange,
    pub historical_ranges: Vec<HistoricalRange>,
}

/// JSON envelope with payload fields and `signature` in the same object.
/// Deserializing this type does not authenticate it; call [`Self::verify`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedManifest {
    #[serde(flatten)]
    pub payload: ManifestPayload,
    #[serde(with = "hex_bytes")]
    pub signature: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum ManifestError {
    #[error("unsupported manifest version: {0}")]
    UnsupportedVersion(u8),
    #[error("block ciphertext descriptors are not strictly ordered by handle")]
    DescriptorsNotStrictlyOrdered,
    #[error("invalid manifest: {0}")]
    InvalidManifest(String),
    #[error("malformed manifest signature: {0}")]
    MalformedSignature(String),
    #[error("manifest signature recovery failed: {0}")]
    SignatureRecovery(String),
    #[error("manifest signer mismatch: recovered {recovered}, expected {expected}")]
    SignerMismatch {
        recovered: Address,
        expected: Address,
    },
    #[error("manifest signing failed: {0}")]
    Signing(#[from] alloy_signer::Error),
}

impl PartialEq for ManifestError {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

/// Computes the canonical digest of the ciphertext material generated by one
/// block. Descriptors must be strictly ordered by raw handle bytes.
pub fn block_content_digest(
    version: ManifestVersion,
    coprocessor_context_id: U256,
    host_chain_id: U256,
    block_number: U256,
    block_hash: B256,
    descriptors: &[BlockCiphertextDescriptor],
) -> Result<B256, ManifestError> {
    validate_descriptors(descriptors)?;

    let mut hasher = Keccak256::new();
    hasher.update(BLOCK_CONTENT_DOMAIN_TAG.as_bytes());
    hasher.update([version as u8]);
    update_u256(&mut hasher, coprocessor_context_id);
    update_u256(&mut hasher, host_chain_id);
    update_u256(&mut hasher, block_number);
    hasher.update(block_hash.as_slice());
    update_u256(&mut hasher, U256::from(descriptors.len()));
    update_descriptors(&mut hasher, descriptors);
    Ok(finalize(hasher))
}

/// Computes the sequential digest of the ordered block digests in one
/// publication's detailed range.
pub fn detailed_range_digest(
    version: ManifestVersion,
    coprocessor_context_id: U256,
    host_chain_id: U256,
    first_block_number: U256,
    last_block_number: U256,
    block_digests: &[B256],
) -> B256 {
    let mut hasher = Keccak256::new();
    hasher.update(DETAILED_RANGE_DOMAIN_TAG.as_bytes());
    hasher.update([version as u8]);
    update_u256(&mut hasher, coprocessor_context_id);
    update_u256(&mut hasher, host_chain_id);
    update_u256(&mut hasher, first_block_number);
    update_u256(&mut hasher, last_block_number);
    update_u256(&mut hasher, U256::from(block_digests.len()));
    for digest in block_digests {
        hasher.update(digest.as_slice());
    }
    finalize(hasher)
}

/// Combines two adjacent equal-sized dyadic ranges into their aligned parent.
#[allow(clippy::too_many_arguments)]
pub fn dyadic_range_digest(
    version: ManifestVersion,
    coprocessor_context_id: U256,
    host_chain_id: U256,
    parent_start: U256,
    parent_end: U256,
    parent_scale: u32,
    parent_end_block_hash: B256,
    left_child_digest: B256,
    right_child_digest: B256,
) -> B256 {
    let mut hasher = Keccak256::new();
    hasher.update(DYADIC_RANGE_DOMAIN_TAG.as_bytes());
    hasher.update([version as u8]);
    update_u256(&mut hasher, coprocessor_context_id);
    update_u256(&mut hasher, host_chain_id);
    update_u256(&mut hasher, parent_start);
    update_u256(&mut hasher, parent_end);
    update_u256(&mut hasher, U256::from(parent_scale));
    hasher.update(parent_end_block_hash.as_slice());
    hasher.update(left_child_digest.as_slice());
    hasher.update(right_child_digest.as_slice());
    finalize(hasher)
}

impl ManifestPayload {
    /// Validate epoch/key bounds, detailed commitments and lineage, and compact
    /// history geometry. Historical digests remain opaque signed commitments.
    /// This does not check a signature, external chain state, or JSON byte size.
    pub fn validate(&self) -> Result<(), ManifestError> {
        validate_consensus_epoch(&self.consensus_epoch)?;
        let prefix = manifest_object_prefix(
            self.version,
            self.coprocessor_context_id,
            self.host_chain_id,
            self.publication_block_number,
            self.publication_block_hash,
            &self.consensus_epoch,
        );
        if prefix.len() + self.revision.to_string().len() > 1024 {
            return Err(invalid(
                "manifest object key exceeds the S3 1024-byte limit",
            ));
        }
        validate_detailed_range(self)?;
        validate_history(self)
    }

    /// Canonical, fixed-order bytes committed by the manifest signature.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, ManifestError> {
        self.validate()?;
        let mut out = Vec::new();
        out.extend_from_slice(MANIFEST_DOMAIN_TAG.as_bytes());
        out.push(self.version as u8);
        push_str(&mut out, &self.consensus_epoch);
        out.extend_from_slice(self.publisher.as_slice());
        push_u256(&mut out, self.coprocessor_context_id);
        push_u256(&mut out, self.host_chain_id);
        push_u256(&mut out, self.publication_block_number);
        out.extend_from_slice(self.publication_block_hash.as_slice());
        out.extend_from_slice(self.publication_parent_block_hash.as_slice());
        push_u256(&mut out, U256::from(self.revision));

        push_u256(&mut out, self.detailed_range.first_block_number);
        push_u256(&mut out, self.detailed_range.last_block_number);
        out.extend_from_slice(self.detailed_range.digest.as_slice());
        push_u256(&mut out, U256::from(self.detailed_range.blocks.len()));
        for block in &self.detailed_range.blocks {
            push_u256(&mut out, block.block_number);
            out.extend_from_slice(block.block_hash.as_slice());
            out.extend_from_slice(block.parent_block_hash.as_slice());
            out.extend_from_slice(block.block_content_digest.as_slice());
            push_u256(&mut out, U256::from(block.ciphertexts.len()));
            push_descriptors(&mut out, &block.ciphertexts);
        }

        push_u256(&mut out, U256::from(self.historical_ranges.len()));
        for range in &self.historical_ranges {
            push_u256(&mut out, range.start_block_number);
            push_u256(&mut out, range.end_block_number);
            push_u256(&mut out, U256::from(range.scale));
            out.extend_from_slice(range.end_block_hash.as_slice());
            out.extend_from_slice(range.digest.as_slice());
        }

        Ok(out)
    }

    pub fn canonical_digest(&self) -> Result<B256, ManifestError> {
        Ok(B256::from_slice(
            Keccak256::digest(self.canonical_bytes()?).as_slice(),
        ))
    }

    /// Validate the payload and sign its canonical digest as a raw prehash.
    /// The signing address must equal [`Self::publisher`]. The caller serializes
    /// the result and enforces [`MAX_MANIFEST_BYTES`] before publication.
    pub async fn sign<S: Signer + Sync + ?Sized>(
        self,
        signer: &S,
    ) -> Result<SignedManifest, ManifestError> {
        if self.publisher != signer.address() {
            return Err(invalid("payload publisher does not match the signing key"));
        }
        let signature = signer.sign_hash(&self.canonical_digest()?).await?;
        Ok(SignedManifest {
            payload: self,
            signature: signature.as_bytes().to_vec(),
        })
    }
}

/// Encode an epoch for an S3 key without changing its signed identity.
/// Preserve path separators and readable ASCII; escape other UTF-8 bytes as
/// `~HH` (uppercase hex), including `~` itself. Dot-only segments are escaped
/// so URL parsers cannot interpret them as relative paths. This is object-key
/// encoding; HTTP transport encoding remains the S3 client's responsibility.
pub fn escape_consensus_epoch(epoch: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut escaped = String::with_capacity(epoch.len());
    for (index, segment) in epoch.split('/').enumerate() {
        if index != 0 {
            escaped.push('/');
        }
        let dot_segment = matches!(segment, "." | "..");
        for byte in segment.bytes() {
            if byte.is_ascii_alphanumeric()
                || matches!(byte, b'-' | b'_')
                || (byte == b'.' && !dot_segment)
            {
                escaped.push(char::from(byte));
            } else {
                escaped.push('~');
                escaped.push(char::from(HEX[usize::from(byte >> 4)]));
                escaped.push(char::from(HEX[usize::from(byte & 15)]));
            }
        }
    }
    escaped
}

/// Canonical S3 prefix for the numbered revisions of one publication.
pub fn manifest_object_prefix(
    version: ManifestVersion,
    context: U256,
    chain: U256,
    block: U256,
    block_hash: B256,
    epoch: &str,
) -> String {
    format!(
        "manifests/v_{}/context_{context}/chain_{chain}/block_{block}/hash_{}/consensus_epoch/{}/revision/",
        u8::from(version),
        hex::encode(block_hash),
        escape_consensus_epoch(epoch),
    )
}

fn validate_consensus_epoch(consensus_epoch: &str) -> Result<(), ManifestError> {
    if consensus_epoch.is_empty() || consensus_epoch.len() > MAX_CONSENSUS_EPOCH_BYTES {
        return Err(invalid("manifest consensus_epoch is empty or too long"));
    }
    if consensus_epoch.contains('\0') {
        return Err(invalid("manifest consensus_epoch contains NUL"));
    }
    if consensus_epoch.split('/').any(str::is_empty) {
        return Err(invalid(
            "manifest consensus_epoch contains an empty path segment",
        ));
    }
    Ok(())
}

fn validate_detailed_range(payload: &ManifestPayload) -> Result<(), ManifestError> {
    let detailed_range = &payload.detailed_range;
    let blocks = &detailed_range.blocks;
    let Some(first) = blocks.first() else {
        return Err(invalid("detailed range must contain at least one block"));
    };
    let last = blocks.last().expect("checked non-empty");

    if first.block_number != detailed_range.first_block_number
        || last.block_number != detailed_range.last_block_number
    {
        return Err(invalid(
            "detailed range bounds do not match its block entries",
        ));
    }
    if last.block_number != payload.publication_block_number
        || last.block_hash != payload.publication_block_hash
        || last.parent_block_hash != payload.publication_parent_block_hash
    {
        return Err(invalid(
            "detailed range does not end at the publication block",
        ));
    }

    for (index, block) in blocks.iter().enumerate() {
        validate_descriptors(&block.ciphertexts)?;
        let expected = block_content_digest(
            payload.version,
            payload.coprocessor_context_id,
            payload.host_chain_id,
            block.block_number,
            block.block_hash,
            &block.ciphertexts,
        )?;
        if expected != block.block_content_digest {
            return Err(invalid(format!(
                "block {} content digest does not match its descriptors",
                block.block_number
            )));
        }

        if let Some(previous) = index.checked_sub(1).map(|i| &blocks[i])
            && (block.block_number != previous.block_number + U256::ONE
                || block.parent_block_hash != previous.block_hash)
        {
            return Err(invalid("detailed range is not one contiguous lineage"));
        }
    }

    let block_digests: Vec<_> = blocks
        .iter()
        .map(|block| block.block_content_digest)
        .collect();
    let expected = detailed_range_digest(
        payload.version,
        payload.coprocessor_context_id,
        payload.host_chain_id,
        detailed_range.first_block_number,
        detailed_range.last_block_number,
        &block_digests,
    );
    if expected != detailed_range.digest {
        return Err(invalid("detailed range digest does not match its blocks"));
    }

    Ok(())
}

impl SignedManifest {
    /// Validate the payload and recover its publisher from the raw-prehash
    /// signature. Callers must check the expected publisher and publication
    /// scope separately; success does not prove historical roots, external
    /// ciphertext availability, or compliance with [`MAX_MANIFEST_BYTES`].
    pub fn verify(&self) -> Result<(), ManifestError> {
        let digest = self.payload.canonical_digest()?;
        let signature = Signature::try_from(self.signature.as_slice())
            .map_err(|err| ManifestError::MalformedSignature(err.to_string()))?;
        let recovered = signature
            .recover_address_from_prehash(&digest)
            .map_err(|err| ManifestError::SignatureRecovery(err.to_string()))?;
        if recovered != self.payload.publisher {
            return Err(ManifestError::SignerMismatch {
                recovered,
                expected: self.payload.publisher,
            });
        }
        Ok(())
    }

    pub fn digest(&self) -> Result<B256, ManifestError> {
        self.payload.canonical_digest()
    }
}

fn validate_descriptors(descriptors: &[BlockCiphertextDescriptor]) -> Result<(), ManifestError> {
    if descriptors
        .windows(2)
        .any(|pair| pair[0].handle >= pair[1].handle)
    {
        return Err(ManifestError::DescriptorsNotStrictlyOrdered);
    }
    Ok(())
}

fn validate_history(payload: &ManifestPayload) -> Result<(), ManifestError> {
    let detailed = &payload.detailed_range;
    let history = &payload.historical_ranges;
    let mut expected_end = detailed.first_block_number.checked_sub(U256::ONE);
    let mut expected_end_hash = detailed.blocks.first().map(|block| block.parent_block_hash);
    let mut upper = detailed.first_block_number;
    let mut previous_scale = 0;

    for (index, range) in history.iter().enumerate() {
        let Some(end) = expected_end else {
            return Err(invalid("history exists before block zero"));
        };
        if range.end_block_number != end
            || expected_end_hash.is_some_and(|hash| range.end_block_hash != hash)
        {
            return Err(invalid(
                "historical ranges are not contiguous with the detailed range",
            ));
        }

        let expected_scale = canonical_history_scale(upper, previous_scale)?;
        if range.scale != expected_scale {
            return Err(invalid(format!(
                "historical range scale {} is not canonical; expected {} below block {}",
                range.scale, expected_scale, upper,
            )));
        }
        let size = U256::ONE
            .checked_shl(range.scale as usize)
            .ok_or_else(|| invalid("historical range scale is too large"))?;
        let Some(range_width) = range
            .end_block_number
            .checked_sub(range.start_block_number)
            .and_then(|width| width.checked_add(U256::ONE))
        else {
            return Err(invalid("historical range starts after it ends"));
        };
        let virtual_start = range
            .end_block_number
            .checked_add(U256::ONE)
            .and_then(|upper| upper.checked_sub(size))
            .ok_or_else(|| invalid("historical range virtual boundary underflow"))?;
        let is_oldest = index + 1 == history.len();
        let is_full = range_width == size && range.start_block_number == virtual_start;
        let is_generation_truncated = is_oldest
            && range.start_block_number >= virtual_start
            && range.start_block_number <= range.end_block_number;
        if !is_full && !is_generation_truncated {
            return Err(invalid(
                "historical range is neither full nor an oldest generation-truncated range",
            ));
        }
        expected_end = range.start_block_number.checked_sub(U256::ONE);
        upper = virtual_start;
        previous_scale = range.scale;
        // The next older range's end hash is not present in this compact wire
        // entry. Its root still commits to every block hash; lineage validation
        // against the host chain supplies the older boundary hash.
        expected_end_hash = None;
    }
    Ok(())
}

/// Scale of the next compact history range below `upper` (exclusive).
///
/// `upper` is the first block *not* covered by this range (the detailed-range
/// start, then each older range's virtual start). Start with `previous_scale = 0`;
/// on each subsequent call, pass the scale of the immediately newer range.
/// The scale increases by one when `upper` is aligned to that doubled width,
/// otherwise it stays unchanged.
pub fn canonical_history_scale(upper: U256, previous_scale: u32) -> Result<u32, ManifestError> {
    let larger_scale = previous_scale
        .checked_add(1)
        .ok_or_else(|| invalid("historical range scale overflow"))?;
    let larger_size = U256::ONE
        .checked_shl(larger_scale as usize)
        .ok_or_else(|| invalid("historical range scale is too large"))?;
    Ok(if upper % larger_size == U256::ZERO {
        larger_scale
    } else {
        previous_scale
    })
}

fn update_descriptors(hasher: &mut Keccak256, descriptors: &[BlockCiphertextDescriptor]) {
    for descriptor in descriptors {
        hasher.update(descriptor.handle.as_slice());
        match &descriptor.status {
            CiphertextStatus::Computed {
                keyset_id,
                ct64_digest,
                ct128_digest,
                ct128_format,
                ..
            } => {
                hasher.update([0, 0]);
                update_u256(hasher, *keyset_id);
                hasher.update(ct64_digest.as_slice());
                hasher.update(ct128_digest.as_slice());
                hasher.update([*ct128_format as u8]);
            }
            CiphertextStatus::Error { .. } => {
                hasher.update([1, 0]);
            }
            CiphertextStatus::Uncomputed => {
                hasher.update([0, 1]);
            }
        }
    }
}

fn push_descriptors(out: &mut Vec<u8>, descriptors: &[BlockCiphertextDescriptor]) {
    for descriptor in descriptors {
        out.extend_from_slice(descriptor.handle.as_slice());
        match &descriptor.status {
            CiphertextStatus::Computed {
                keyset_id,
                gateway_key_id,
                ct64_digest,
                ct128_digest,
                ct128_format,
            } => {
                out.push(0);
                out.push(0);
                push_u256(out, *keyset_id);
                push_optional_u256(out, *gateway_key_id);
                out.extend_from_slice(ct64_digest.as_slice());
                out.extend_from_slice(ct128_digest.as_slice());
                out.push(*ct128_format as u8);
            }
            CiphertextStatus::Error { error_message } => {
                out.push(1);
                out.push(0);
                push_optional_str(out, error_message.as_deref());
            }
            CiphertextStatus::Uncomputed => {
                out.push(0);
                out.push(1);
            }
        }
    }
}

fn push_optional_str(out: &mut Vec<u8>, value: Option<&str>) {
    match value {
        Some(value) => {
            out.push(1);
            push_str(out, value);
        }
        None => out.push(0),
    }
}

fn push_optional_u256(out: &mut Vec<u8>, value: Option<U256>) {
    match value {
        Some(value) => {
            out.push(1);
            push_u256(out, value);
        }
        None => {
            out.push(0);
            out.extend_from_slice(&[0; 32]);
        }
    }
}

fn push_str(out: &mut Vec<u8>, value: &str) {
    push_u256(out, U256::from(value.len() as u64));
    out.extend_from_slice(value.as_bytes());
}

fn push_u256(out: &mut Vec<u8>, value: U256) {
    out.extend_from_slice(&value.to_be_bytes::<32>());
}

fn update_u256(hasher: &mut Keccak256, value: U256) {
    hasher.update(value.to_be_bytes::<32>());
}

fn finalize(hasher: Keccak256) -> B256 {
    B256::from_slice(hasher.finalize().as_slice())
}

fn invalid(message: impl Into<String>) -> ManifestError {
    ManifestError::InvalidManifest(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{address, b256};
    use alloy_signer_local::PrivateKeySigner;

    #[test]
    fn epoch_escaping_preserves_identity_and_readability() {
        let cases = [
            ("v0.15/rc1", "v0.15/rc1"),
            ("v0.15/../rc1", "v0.15/~2E~2E/rc1"),
            ("./..", "~2E/~2E~2E"),
            ("~2E", "~7E2E"),
            ("release candidate", "release~20candidate"),
            ("%2E%2E?#\\", "~252E~252E~3F~23~5C"),
            ("révision", "r~C3~A9vision"),
            (LEGACY_CONSENSUS_EPOCH, LEGACY_CONSENSUS_EPOCH),
        ];
        let mut seen = std::collections::HashSet::new();
        for (original, expected) in cases {
            assert!(validate_consensus_epoch(original).is_ok());
            let encoded = escape_consensus_epoch(original);
            assert_eq!(encoded, expected);
            assert!(
                seen.insert(encoded.clone()),
                "distinct identities must not collide"
            );
            let mut decoded = Vec::new();
            let mut bytes = encoded.bytes();
            while let Some(byte) = bytes.next() {
                if byte == b'~' {
                    let hex = [bytes.next().unwrap(), bytes.next().unwrap()];
                    decoded.extend(hex::decode(hex).unwrap());
                } else {
                    decoded.push(byte);
                }
            }
            assert_eq!(String::from_utf8(decoded).unwrap(), original);
        }
        for invalid in ["", "/v1", "v1/", "v1//rc1", "v1\0rc1"] {
            assert!(validate_consensus_epoch(invalid).is_err());
        }
    }

    #[test]
    fn escaped_epoch_respects_the_s3_key_size_limit() {
        let mut manifest = payload(Address::ZERO);
        manifest.consensus_epoch = "#".repeat(MAX_CONSENSUS_EPOCH_BYTES);
        manifest.coprocessor_context_id = U256::MAX;
        manifest.host_chain_id = U256::MAX;
        manifest.publication_block_number = U256::MAX;
        let error = manifest.validate().unwrap_err();
        assert!(error.to_string().contains("1024-byte"));
    }

    fn descriptor(handle: u8) -> BlockCiphertextDescriptor {
        BlockCiphertextDescriptor::computed(
            B256::repeat_byte(handle),
            U256::from(handle),
            Some(U256::from(handle)),
            B256::repeat_byte(handle.wrapping_add(1)),
            B256::repeat_byte(handle.wrapping_add(2)),
            CiphertextFormat::CompressedOnCpu,
        )
    }

    pub(super) fn payload(publisher: Address) -> ManifestPayload {
        let context = U256::ONE;
        let chain = U256::from(7);
        let descriptors = vec![descriptor(1), descriptor(2)];
        let content = block_content_digest(
            ManifestVersion::V1,
            context,
            chain,
            U256::from(42),
            B256::repeat_byte(0xAA),
            &descriptors,
        )
        .unwrap();
        let detailed_digest = detailed_range_digest(
            ManifestVersion::V1,
            context,
            chain,
            U256::from(42),
            U256::from(42),
            &[content],
        );

        ManifestPayload {
            version: ManifestVersion::V1,
            consensus_epoch: LEGACY_CONSENSUS_EPOCH.to_owned(),
            publisher,
            coprocessor_context_id: context,
            host_chain_id: chain,
            publication_block_number: U256::from(42),
            publication_block_hash: B256::repeat_byte(0xAA),
            publication_parent_block_hash: B256::repeat_byte(0xA9),
            revision: 0,
            detailed_range: DetailedRange {
                first_block_number: U256::from(42),
                last_block_number: U256::from(42),
                digest: detailed_digest,
                blocks: vec![ManifestBlockEntry {
                    block_number: U256::from(42),
                    block_hash: B256::repeat_byte(0xAA),
                    parent_block_hash: B256::repeat_byte(0xA9),
                    block_content_digest: content,
                    ciphertexts: descriptors,
                }],
            },
            historical_ranges: vec![],
        }
    }

    #[test]
    fn block_content_rejects_unsorted_or_duplicate_handles() {
        for descriptors in [
            vec![descriptor(2), descriptor(1)],
            vec![descriptor(1), descriptor(1)],
        ] {
            assert_eq!(
                block_content_digest(
                    ManifestVersion::V1,
                    U256::ONE,
                    U256::from(7),
                    U256::from(42),
                    B256::repeat_byte(0xAA),
                    &descriptors,
                ),
                Err(ManifestError::DescriptorsNotStrictlyOrdered),
            );
        }
    }

    #[test]
    fn empty_and_non_empty_blocks_have_distinct_content_digests() {
        let header = (
            ManifestVersion::V1,
            U256::ONE,
            U256::from(7),
            U256::from(42),
            B256::repeat_byte(0xAA),
        );
        let empty =
            block_content_digest(header.0, header.1, header.2, header.3, header.4, &[]).unwrap();
        let non_empty = block_content_digest(
            header.0,
            header.1,
            header.2,
            header.3,
            header.4,
            &[descriptor(1)],
        )
        .unwrap();
        assert_ne!(empty, non_empty);
    }

    #[test]
    fn computation_error_is_consensus_material_without_ciphertext_fields() {
        let header = (
            ManifestVersion::V1,
            U256::ONE,
            U256::from(7),
            U256::from(42),
            B256::repeat_byte(0xAA),
        );
        let digest = |descriptors: &[BlockCiphertextDescriptor]| {
            block_content_digest(
                header.0,
                header.1,
                header.2,
                header.3,
                header.4,
                descriptors,
            )
            .unwrap()
        };
        let success = descriptor(1);
        let error = BlockCiphertextDescriptor::from_computation_error(
            success.handle,
            Some("UnsupportedFheTypes".to_owned()),
        );
        let error_other_message = BlockCiphertextDescriptor::from_computation_error(
            success.handle,
            Some("different Display".to_owned()),
        );

        assert_ne!(digest(std::slice::from_ref(&success)), digest(&[]));
        assert_ne!(digest(std::slice::from_ref(&error)), digest(&[]));
        assert_ne!(
            digest(std::slice::from_ref(&success)),
            digest(std::slice::from_ref(&error))
        );
        assert_eq!(
            digest(std::slice::from_ref(&error)),
            digest(std::slice::from_ref(&error_other_message))
        );

        let uncomputed = BlockCiphertextDescriptor::from_uncomputed(success.handle);
        assert_ne!(digest(std::slice::from_ref(&uncomputed)), digest(&[]));
        assert_ne!(
            digest(std::slice::from_ref(&uncomputed)),
            digest(std::slice::from_ref(&error))
        );
        assert_ne!(
            digest(std::slice::from_ref(&success)),
            digest(std::slice::from_ref(&uncomputed))
        );
    }

    #[test]
    fn computation_error_message_is_signed_provenance_but_not_consensus_material() {
        let handle = B256::repeat_byte(1);
        let with_message = BlockCiphertextDescriptor::from_computation_error(
            handle,
            Some("Unknown fhe operation".to_owned()),
        );
        let without_message = BlockCiphertextDescriptor::from_computation_error(handle, None);
        let block_digest = |descriptor: BlockCiphertextDescriptor| {
            block_content_digest(
                ManifestVersion::V1,
                U256::ONE,
                U256::from(7),
                U256::from(42),
                B256::repeat_byte(0xAA),
                &[descriptor],
            )
            .unwrap()
        };
        assert_eq!(
            block_digest(with_message.clone()),
            block_digest(without_message.clone()),
        );

        let mut with_message_payload = payload(Address::ZERO);
        with_message_payload.detailed_range.blocks[0].ciphertexts = vec![with_message];
        with_message_payload.detailed_range.blocks[0].block_content_digest =
            block_digest(with_message_payload.detailed_range.blocks[0].ciphertexts[0].clone());
        with_message_payload.detailed_range.digest = detailed_range_digest(
            ManifestVersion::V1,
            U256::ONE,
            U256::from(7),
            U256::from(42),
            U256::from(42),
            &[with_message_payload.detailed_range.blocks[0].block_content_digest],
        );
        let mut without_message_payload = with_message_payload.clone();
        without_message_payload.detailed_range.blocks[0].ciphertexts = vec![without_message];

        assert_ne!(
            with_message_payload.canonical_digest().unwrap(),
            without_message_payload.canonical_digest().unwrap(),
        );
    }

    #[test]
    fn optional_gateway_key_id_is_signed_provenance_but_not_consensus_material() {
        let with_gateway = descriptor(1);
        let CiphertextStatus::Computed {
            keyset_id,
            ct64_digest,
            ct128_digest,
            ct128_format,
            ..
        } = with_gateway.status.clone()
        else {
            panic!("test descriptor is computed");
        };
        let without_gateway = BlockCiphertextDescriptor::computed(
            with_gateway.handle,
            keyset_id,
            None,
            ct64_digest,
            ct128_digest,
            ct128_format,
        );
        let block_digest = |descriptor| {
            block_content_digest(
                ManifestVersion::V1,
                U256::ONE,
                U256::from(7),
                U256::from(42),
                B256::repeat_byte(0xAA),
                &[descriptor],
            )
            .unwrap()
        };
        assert_eq!(
            block_digest(with_gateway.clone()),
            block_digest(without_gateway.clone()),
        );

        let mut with_gateway_payload = payload(Address::ZERO);
        with_gateway_payload.detailed_range.blocks[0].ciphertexts = vec![with_gateway];
        with_gateway_payload.detailed_range.blocks[0].block_content_digest =
            block_digest(with_gateway_payload.detailed_range.blocks[0].ciphertexts[0].clone());
        with_gateway_payload.detailed_range.digest = detailed_range_digest(
            ManifestVersion::V1,
            U256::ONE,
            U256::from(7),
            U256::from(42),
            U256::from(42),
            &[with_gateway_payload.detailed_range.blocks[0].block_content_digest],
        );
        let mut without_gateway_payload = with_gateway_payload.clone();
        without_gateway_payload.detailed_range.blocks[0].ciphertexts = vec![without_gateway];

        assert_ne!(
            with_gateway_payload.canonical_digest().unwrap(),
            without_gateway_payload.canonical_digest().unwrap(),
        );
    }

    #[test]
    fn digest_vectors_are_pinned() {
        let content = block_content_digest(
            ManifestVersion::V1,
            U256::ONE,
            U256::from(7),
            U256::from(42),
            B256::repeat_byte(0xAA),
            &[descriptor(1), descriptor(2)],
        )
        .unwrap();
        let detailed = detailed_range_digest(
            ManifestVersion::V1,
            U256::ONE,
            U256::from(7),
            U256::from(42),
            U256::from(42),
            &[content],
        );
        let range = dyadic_range_digest(
            ManifestVersion::V1,
            U256::ONE,
            U256::from(7),
            U256::from(40),
            U256::from(41),
            1,
            B256::repeat_byte(0xA9),
            B256::repeat_byte(0x11),
            B256::repeat_byte(0x22),
        );

        assert_eq!(
            content,
            b256!("d3717f13a0cf04dda33de1d2e25feebef70a0039dafc278be16c9a6e6dd1a5f3")
        );
        assert_eq!(
            detailed,
            b256!("8e056c9e1604ab43b966b8a45f162b4b76eb5fc7c1576317cbf9b97a892fa6f9")
        );
        assert_eq!(
            range,
            b256!("c43c4863aa888fd3f7f87ea903dc30a9a4e9c91c3f5767c167c8df36034e879d")
        );
    }

    #[test]
    fn v1_generation_is_signed_but_not_ciphertext_consensus_material() {
        let mut first = payload(Address::ZERO);
        first.version = ManifestVersion::V1;
        first.consensus_epoch = "7".to_owned();
        let content = block_content_digest(
            ManifestVersion::V1,
            first.coprocessor_context_id,
            first.host_chain_id,
            first.publication_block_number,
            first.publication_block_hash,
            &first.detailed_range.blocks[0].ciphertexts,
        )
        .unwrap();
        first.detailed_range.blocks[0].block_content_digest = content;
        first.detailed_range.digest = detailed_range_digest(
            ManifestVersion::V1,
            first.coprocessor_context_id,
            first.host_chain_id,
            first.detailed_range.first_block_number,
            first.detailed_range.last_block_number,
            &[content],
        );
        let mut second = first.clone();
        second.consensus_epoch = "8".to_owned();

        assert_ne!(
            first.canonical_digest().unwrap(),
            second.canonical_digest().unwrap(),
        );
        assert_eq!(
            first.detailed_range.blocks[0].block_content_digest,
            second.detailed_range.blocks[0].block_content_digest,
        );
    }

    #[test]
    fn manifest_rejects_a_non_canonical_dyadic_history() {
        let mut payload = payload(Address::ZERO);
        payload.historical_ranges = vec![HistoricalRange {
            start_block_number: U256::from(41),
            end_block_number: U256::from(41),
            scale: 0,
            end_block_hash: B256::repeat_byte(0xA9),
            digest: B256::repeat_byte(1),
        }];

        assert_eq!(
            payload.validate(),
            Err(invalid(
                "historical range scale 0 is not canonical; expected 1 below block 42",
            )),
        );
    }

    #[tokio::test]
    async fn signed_manifest_round_trips_and_verifies() {
        let signer = PrivateKeySigner::random();
        let signed = payload(signer.address()).sign(&signer).await.unwrap();
        signed.verify().unwrap();

        let json = serde_json::to_string(&signed).unwrap();
        let decoded: SignedManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, signed);
        decoded.verify().unwrap();
    }

    #[test]
    fn manifest_digest_is_pinned() {
        let payload = payload(address!("00112233445566778899aabbccddeeff00112233"));
        assert_eq!(
            payload.canonical_digest().unwrap(),
            b256!("09750ce73606cedee5546207620198b671a9bc8c72554063c1e260b7ddca3334")
        );
    }

    #[test]
    fn descriptor_status_json_uses_error_message() {
        let computed = descriptor(1);
        let computed_json = serde_json::to_value(&computed).unwrap();
        assert_eq!(computed_json["status"], "computed");
        assert!(computed_json.get("is_error").is_none());
        assert!(computed_json.get("is_uncomputed").is_none());
        assert_eq!(
            serde_json::from_value::<BlockCiphertextDescriptor>(computed_json).unwrap(),
            computed
        );

        let error = BlockCiphertextDescriptor::from_computation_error(
            B256::repeat_byte(1),
            Some("Unknown fhe operation".to_owned()),
        );
        let error_json = serde_json::to_value(&error).unwrap();
        assert_eq!(error_json["status"], "error");
        assert_eq!(error_json["error_message"], "Unknown fhe operation");
        assert!(error_json.get("message").is_none());
        assert!(error_json.get("ct64_digest").is_none());
        assert_eq!(
            serde_json::from_value::<BlockCiphertextDescriptor>(error_json).unwrap(),
            error
        );

        let uncomputed = BlockCiphertextDescriptor::from_uncomputed(B256::repeat_byte(2));
        let uncomputed_json = serde_json::to_value(&uncomputed).unwrap();
        assert_eq!(uncomputed_json["status"], "uncomputed");
        assert!(uncomputed_json.get("error_message").is_none());
        assert_eq!(
            serde_json::from_value::<BlockCiphertextDescriptor>(uncomputed_json).unwrap(),
            uncomputed
        );
    }
}

#[cfg(test)]
mod wire_contract_tests;
