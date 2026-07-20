# S3-based coprocessor consensus and drift detection

Status: future-design draft

## Goal

Replace the current Gateway-chain mechanism used to establish agreement on new
ciphertext material, together with its `gw-listener` drift detector, with
coprocessor-internal S3 consensus. Each `sns-worker` derives consensus from signed
periodic block-range manifests and publishes a signed consensus summary of the
result.

Each coprocessor would:

1. continue to upload its ciphertext material to its own S3 buckets;
2. publish immutable block-range commitment manifests at configured block intervals;
3. read and archive every revision published by the other coprocessors;
4. derive quorum and drift from comparable authenticated manifest commitments; and
5. publish a signed consensus summary for downstream consumers.

The manifests and summaries are the long-term consensus authority. A downstream
consumer still verifies any downloaded ciphertext body against the digest selected
by consensus, but it does not independently recreate consensus from Gateway state.

## Current boundary

Today, consensus and drift detection are split across three components:

- `transaction-sender` submits, for each handle, the handle, Gateway key ID,
  ciphertext digest (ct64), and SnS ciphertext digest (ct128) to the Gateway chain.
- `CiphertextCommits` authenticates the submitting coprocessor and marks the
  ciphertext material as added when the priority coprocessor finalizes it or the
  configured matching-submission threshold is reached.
- `gw-listener` observes individual and consensus events, compares them with the
  local digests stored in `ciphertext_digest`, and reports drift or missing
  submissions.

The Gateway contract is therefore not only a transport for drift information. Its
`isCiphertextMaterialAdded` state is currently a readiness gate used by decryption.
In the future design, an authenticated coprocessor consensus summary becomes the
readiness decision and supplies the agreed material metadata.

## Proposed model

### Separation of responsibilities

| Responsibility | Future authority | Unit |
| --- | --- | --- |
| Ciphertext consensus and readiness | Each `sns-worker` | Authenticated manifest quorum and signed consensus summary |
| Drift detection and localization | Each `sns-worker` | One periodic detailed range plus dyadic history |
| Ciphertext and attestation publication | Each `sns-worker` | One ciphertext object |
| Ciphertext retrieval | Downstream consumer | One body verified against the consensus digest |

Every coprocessor independently authenticates the same publisher set, groups
comparable manifest commitments under one pinned registry snapshot, and derives the
same winning commitment when quorum exists. A summary is not trusted merely because
one worker published it: consumers authenticate its signer and validate it against
the manifest evidence and summary-acceptance rule defined for the request path.

### Manifest consensus and consensus summaries

For each eligible local publication, `sns-worker`:

1. resolves the expected coprocessors, their signing identities, their S3 bucket
   locations, and the required agreement threshold;
2. waits for the configured post-publication delay, then downloads and archives all
   available peer manifest revisions within the bounded retry budget;
3. verifies each signature and discards manifests from unknown or duplicate
   publishers;
4. groups comparable latest authenticated revisions by their exact commitment;
5. classifies any visible difference as `drift`, independently of quorum;
6. separately selects a remediation reference only when one commitment has the
   required number of distinct authorized publishers; and
7. persists and publishes a signed summary referencing every observed group and the
   winning group when one exists.

A local publisher persists its local-versus-observed handle findings before
publishing a drift summary. If a unique winning group exists, each finding records
whether its observed side belongs to that group. Only a local value outside the
winning group can use such a finding for remediation.

If no commitment reaches the threshold, equal visible values produce
`unknown_but_equal`; differing visible values produce `drift`. No publisher may
promote the largest sub-threshold group to consensus or remediation authority. The
bounded retry process may later establish quorum when newer or previously missing
peer revisions arrive.

Manifest consensus establishes agreed metadata, but it does not prove that usable
ciphertext bytes remain available. A downstream request retrieves a body from a
publisher in the winning group and verifies it against the agreed digest, trying
another winning publisher if the first body is missing or invalid.

### Commitment manifest and publication cadence

A commitment manifest is the sealed statement of one coprocessor at a specific
host-chain block. In each coprocessor bucket, each sealed revision is stored under a
deterministic, immutable key:

```text
<s3BucketUrl>/manifests/v1/<coprocessor_context_id>/<host_chain_id>/<block_height>/<block_hash>/<revision>
```

`revision` is a non-negative integer that starts at `0` for the first seal of that
block identity and increases by one for each later superseding publication by the
same publisher. The path revision and the signed body revision must match; a mismatch
makes the object invalid. Lower revisions remain as historical evidence and are never
overwritten. An optional unsigned tip pointer may exist only as a discovery
optimization; authority is always the highest authenticated revision under the block
prefix.

The `revision` is the manifest number and is always the final component of its key.
Across all registered publisher buckets, the immutable manifest identity is:

```text
(publisher, version, coprocessor_context_id, host_chain_id,
 publication_block_number, publication_block_hash, revision)
```

Every identity component represented in the key must match the signed body. The
registered bucket selects the expected publisher, and the signature independently
authenticates that publisher; neither the bucket nor the key is trusted on its own.

Revision `0` has no supersession reference. Revision `n > 0` must reference revision
`n - 1` for the same immutable block identity and must bind its manifest digest. A
downloaded higher number may be archived while its predecessor is unavailable, but
it is not tip-eligible until the complete supersession link has been authenticated.
Throughout this document, the highest authenticated revision means the highest
tip-eligible numbered revision, not merely the largest key returned by S3.

`s3BucketUrl` is the ct128/SnS bucket registered for that coprocessor in
`GatewayConfig`. The reserved `manifests` prefix prevents collisions with ciphertext
object keys. `coprocessor_context_id` distinguishes ciphertext contexts within one
registered bucket.

For a fast chain, publishing one S3 object for every block is unnecessary. Each
chain has a configured positive integer `K`, initially expected to be `1`, `2`, or
`3`, representing a publication rate of `1/K`. A manifest is due exactly when:

```text
block_number mod K == 0
```

Every command-line configuration option introduced for this mechanism uses the
`--consensus-` prefix. This includes publication cadence, verification timing, peer
scanning and retry limits, and any later automatic-response safety controls. The
initial verification controls include `--consensus-verification-delay`,
`--consensus-verification-retry-delay`, and
`--consensus-verification-retry-count`. The same retry delay and count budget apply
to insufficient-quorum and post-drift reconcile attempts (see
[Verification retry and tip reconcile](#verification-retry-and-tip-reconcile)).
The reserved control for future manifest cleanup is
`--consensus-manifest-retention-days`, with a default of `90` days.

Do not confuse manifest `revision` (publisher supersession of a block commitment)
with verification retries (how many times the worker re-polls peer tips).

A worker computes commitments for every processed block in strict
parent-before-child order within each observed lineage, but writes manifests only at
those deterministic publication heights. The rule applies independently to every
lineage, so different block hashes at the same scheduled height may both have
manifests.

The comparison protocol tolerates a coprocessor running with a different `K` by
mistake: it resolves the newest valid peer manifest on the same lineage within the
discovery and freshness bounds, compares the exact ranges shared with local state,
and reports the cadence mismatch. Cadence is derived only from absolute block height
and `K`, never from a worker's startup time or its most recent successful publication.

A version 1 manifest contains:

- a manifest format version;
- the host chain ID;
- the coprocessor context ID;
- the publication block number and block hash;
- the parent block hash;
- the coprocessor identity;
- the publication `revision` for this block identity;
- when `revision > 0`, a `supersedes` reference to revision `n - 1` containing its
  block number, block hash, revision, and manifest digest;
- one detailed range covering every block after the preceding manifest through the
  publication block; each block remains a separate entry containing its identity,
  its block digest `A`, and a canonical ordered association list binding every
  ciphertext handle first attributed to that block to its digest descriptor;
- one sequential digest over the ordered block digests of that detailed range;
- a newest-to-oldest sequence of aligned dyadic historical ranges preceding the
  detailed range, each containing its inclusive block bounds, ending block hash,
  scale, and range digest;
- the last full-consensus checkpoint known when the manifest was sealed, when one
  exists;
- a reference to the preceding manifest containing its ending block number, ending
  block hash, ending revision, and manifest digest; and
- the publisher's signature over the complete manifest envelope.

Dyadic ranges are aligned by absolute block number, so independently started
coprocessors produce the same range identity whenever they know the same lineage
interval. A coprocessor with a longer history may expose additional older ranges;
those unmatched ranges are ignored when comparing it with a shorter history and are
not treated as drift.

### Manifest signing

`sns-worker` signs the manifest using the same coprocessor signing identity used for
the signed attestation metadata on ciphertext objects. The signature is produced as
part of publication and is verified against the coprocessor's registered
`GatewayConfig.signerAddress`.

The canonical commitment and signing implementation lives in the `manifest` module
under `shared/ciphertext-attestation`. This shared module is the single source of
truth for canonical encoding, Keccak-256 hashing, signing, and verification.
`sns-worker`, peer verifiers, KMS, and external evidence tools must use it rather
than independently reconstructing bytes.

The encoding follows the existing ciphertext-attestation convention: fixed-width
fields are concatenated in their declared order, unsigned integers use big-endian
encoding, arrays are preceded by a `uint256` element count, and optional fixed-width
fields use an explicit `uint8` presence discriminator followed by their fixed-width
storage slot. JSON is only the S3 wire representation and is never hashed directly.
Every canonical-encoding function has pinned byte and digest test vectors.

Version 1 reserves these eight-byte ASCII domain tags in the shared crate:

| Purpose | Domain tag |
| --- | --- |
| Existing ciphertext attestation | `FHEVMCTA` |
| Block ciphertext-content digest `A` | `FHEVMBLK` |
| Detailed-range digest | `FHEVMDET` |
| Dyadic historical-range digest | `FHEVMRNG` |
| Signed commitment manifest | `FHEVMMNF` |

The manifest prehash is Keccak-256 of its complete canonical payload prefixed by the
eight-byte domain tag `FHEVMMNF`. It is signed as a raw prehash with the same signing
method as `CiphertextAttestationPayload::canonical_digest`. The payload covers the
format version, publisher identity, coprocessor context ID, chain ID, block number,
block hash, parent block hash, `revision`, supersession
reference when present, the detailed-range bounds and digest, every covered block
and handle-to-descriptor association, every dyadic
history entry, the last full-consensus checkpoint when present, and the
preceding-manifest reference. Moving a manifest to another path or context, changing
its revision or lineage, or editing its ciphertext associations or history must
invalidate verification.

S3 write access alone does not authorize a manifest. An object without a valid
registered-coprocessor signature is not inserted into the authenticated manifest
archive and never participates in comparison or local revert decisions. Its key,
fetch result, and validation failure may be retained separately as invalid-observation
evidence.

The historical section remains small because it contains fixed-size dyadic range
roots rather than ciphertext material or past handle lists. It is deterministic for
the locally known lineage and full-consensus boundary; it is not selected by an
approximate time or manifest-count window. The detailed block associations are also
metadata only: their size grows with the number of new handles since the preceding
manifest, not with the ciphertext byte size.

### Block content digest `A`

`A` commits to the published ciphertext material generated by one block. A
ciphertext belongs to the consensus set exactly when executing that block generates
a new ct64 for its handle and that handle is allowed in the producing block.
Input handles that are only read or reused, copies that do not generate a new ct64,
upload retries, and later upload completion do not create another association. A
bridged or otherwise special operation is included when it generates a new ct64 in
that block.

The operation table provides the authoritative inventory from which this set is
built. For branch-aware blocks, the publisher queries `computations_branch` by
`(host_chain_id, producer_block_hash)` and orders the resulting `output_handle`
values having `is_allowed = TRUE` by their raw bytes. `block_number` is checked as
an additional consistency field. Host-listener fixes `is_allowed` only after it has
scanned all TFHE and ACL events in the block. An unallowed intermediate operation
may be evaluated in memory to supply another operation, but its output is not
persisted as a new ct64 and does not belong to the manifest. `pbs_computations_branch`
is not the inventory: it contains downstream SnS work for the allowed handles.

The legacy `computations` table is outside this design. It cannot distinguish
competing block hashes, and no manifest or consensus history is backfilled from it.
Historical coverage begins at the oldest block for which the publisher has complete
branch-aware operation, ciphertext, and lineage state. Earlier blocks are not
reconstructed from the legacy table.

Both the ct64 and ct128 digests must be available before the block can be sealed. For
an operation marked `is_allowed = TRUE` in its producing block, the same block creates
its SnS publication work. Publication waits until the matching
`pbs_computations_branch` row is complete and its branch-scoped ct128 digest and
format are final.

The uploader's zero `NO_SNS_CIPHERTEXT_DIGEST` support does not define another
terminal outcome for a new branch-aware allowed handle. It supports legacy or
recovery states; in the branch-aware path, failed SnS work is not enqueued for
publication. An operation not allowed in that block contributes no persisted ct64
descriptor and creates no publication dependency for that block manifest. A later
ACL event cannot mutate the already sealed producing-block manifest.

Each newly generated, allowed handle must appear exactly once. A duplicate handle
makes the block invalid for publication.

The descriptor is the following fixed-width tuple:

```text
handle:                 bytes32
keyset_id:               uint256
gateway_key_id?:         uint256     # optional legacy provenance
ct64_digest:            bytes32
ct128_digest:           bytes32
ct128_format:           uint8       # CiphertextFormat
```

Both ciphertext digests are Keccak-256 of the exact ciphertext bytes, matching the
existing `sns-worker` ciphertext-digest function. The ct128 format uses the existing
`CiphertextFormat` discriminant defined by ciphertext attestation. `keyset_id`
identifies the compatible FHE key generation used by the computation: locally the
worker evaluates with its server key, while the resulting ciphertext remains
decryptable by the matching secret key. The neutral keyset name therefore avoids
describing the value as only an encryption key or only a server key.

`keyset_id` and ct128 format participate in the commitment because disagreement on
either can make otherwise identified material unusable or interpreted differently.
A differing keyset ID is also a direct, actionable explanation for drift around key
activation or rotation; equal keyset IDs with differing digests instead point toward
computation, serialization, software-version, or data-integrity causes.

The legacy Gateway key ID is optional signed descriptor provenance. Its presence is
encoded with an explicit presence flag in the manifest signature payload so it cannot
be altered after publication. It is excluded from block digest `A`, quorum grouping,
and the quorum side of a consensus summary, so a present or absent legacy value alone
cannot create drift. A drift finding may retain it only as
`local_gateway_key_id`; there is no `quorum_gateway_key_id`.

Publisher identity, transaction ID, timestamps, S3 keys, and S3 transport checksums
are deliberately excluded from `A`. They are not ciphertext computation results and
may legitimately differ between operators. Coprocessor context, host chain, and
block identity are encoded once in the block-digest header rather than repeated in
every descriptor.

Descriptors are sorted by ascending raw 32-byte handle. For manifest format version
1, `A` uses the consensus fields of each descriptor (`handle`, `keyset_id`, both
ciphertext digests, and `ct128_format`). The optional `gateway_key_id` remains in the
signed manifest body but is not part of `descriptor[i]` below:

```text
A = keccak256(
      bytes8("FHEVMBLK")
      || uint8(version)
      || uint256(coprocessor_context_id)
      || uint256(host_chain_id)
      || uint256(block_number)
      || bytes32(block_hash)
      || uint256(descriptor_count)
      || descriptor[0]
      || ...
      || descriptor[n - 1]
    )
```

The manifest includes the full association list, not independent handle and digest
lists and not only `A`; a verifier canonicalizes the associations and recomputes `A`.
An object with unsorted or duplicate associations is invalid even if sorting it would
produce the claimed digest.

Every allowed new handle attributed to every covered block must appear exactly once
as an association key. This is necessary for diagnosis: `A` proves that a block differs,
while the signed associations identify handles that are missing, unexpected, or
associated with different digests. At cadence `1/K`, the association lists cover all
blocks after the preceding manifest through the current publication block. Listing
only the publication block's associations would make a divergence in one of the
intervening `K - 1` blocks visible in the detailed-range digest but not diagnosable
from that manifest.

An empty block has `descriptor_count = 0` and therefore has a well-defined,
block-specific `A`: Keccak-256 of the header above with no descriptors, rather than a
special zero value. A database insertion or completion index never participates in
ordering.

### Detailed-range digest

The detailed range covers the blocks after the preceding manifest through the
current publication block. It keeps handles grouped by block. This allows a verifier
to locate a different block first and then merge the two sorted descriptor lists for
that block without scanning handles from unrelated blocks.

The range is expected to be small because its size follows the publication cadence.
Its digest is therefore sequential rather than a Merkle tree:

```text
detailed_range_digest = keccak256(
  bytes8("FHEVMDET")
  || uint8(version)
  || uint256(coprocessor_context_id)
  || uint256(host_chain_id)
  || uint256(first_block_number)
  || uint256(last_block_number)
  || uint256(block_count)
  || A(first_block)
  || ...
  || A(last_block)
)
```

The entries are ordered by ascending block number and must form one contiguous
lineage ending at the manifest block. Every block is present, including an empty
block with its well-defined empty `A`. If the detailed-range digests differ, the
verifier compares the ordered block entries linearly. A coprocessor classified as
drifted then compares descriptors only for its differing blocks while building its
local drift inventory.

### Dyadic historical ranges

History is represented by aligned ranges of the form:

```text
[q * 2^k, (q + 1) * 2^k - 1]
```

where `k` is the scale and `q` is a non-negative integer. The history immediately
precedes the detailed range, stays on that manifest's lineage, and is listed from
newest to oldest. Recent history is represented by small ranges; progressively older
history is represented by larger ranges. At each scale the retained decomposition
contains at most two ranges.

The history uses one canonical right-anchored decomposition. Let `U` initially be
the first block number of the detailed range, and let the preceding selected range
size initially be `2^0`. Walking backward, if `U` is divisible by `2^(k+1)`, the
next range has size `2^(k+1)`; otherwise it retains size `2^k`:

```text
next_scale = if U mod 2^(k+1) == 0 { k + 1 } else { k }
next_range = [U - 2^next_scale, U - 1]
```

After selecting a range, `U` becomes its start and `k` becomes `next_scale`. This
leaves no choice of decomposition for a given detailed-range boundary and implies at
most two ranges per scale. If the next required canonical range contains a block
absent from `block_consensus`, history stops. The publisher does not substitute
smaller locally known fragments because that would make the decomposition depend on
its start time.

Each sealed block first creates a size-one range with
`range_digest([h, h]) = A(h)`. A larger required range is materialized by combining
its two adjacent, aligned, equal-sized child range digests. Parent digests are binary
combinations:

```text
range_digest(parent) = keccak256(
  bytes8("FHEVMRNG")
  || uint8(version)
  || uint256(coprocessor_context_id)
  || uint256(host_chain_id)
  || uint256(parent_start)
  || uint256(parent_end)
  || uint256(parent_scale)
  || bytes32(parent_end_block_hash)
  || bytes32(left_child_digest)
  || bytes32(right_child_digest)
)
```

A parent is created only when its two children are complete, adjacent, equal-sized,
correctly aligned, and on the same lineage. Empty blocks participate as ordinary
size-one ranges, so a missing block cannot be confused with an empty block. A sealed
range digest is immutable and can be persisted once, then reused by later manifests.
If a repair supersedes an already sealed block digest, affected range nodes are
rebuilt with distinct digests; old range digests remain evidence and are never
overwritten. Range roots do not introduce a second revision counter: manifest
`revision` is the only numbered supersession mechanism. A new block causes at most
`O(log block_number)` range combinations.

Two historical entries are directly comparable only when their exact identity
`(version, coprocessor_context_id, host_chain_id, range_start, range_end,
range_end_block_hash)` matches; the scale is derived from the aligned bounds.
Verifiers compare the intersection of these exact range identities:

- equal roots skip the complete range;
- different roots prove a disagreement somewhere inside the range; and
- a range present in only one manifest is `not_covered`, not disagreement.

If an older coprocessor exposes additional history that a later-started coprocessor
does not know, those extra oldest ranges are ignored. The verifier does not split an
extra large range merely to align it with absent smaller ranges. Child roots are
needed only after the same exact range root differs, for localization. The verifier
may then fetch older manifests in which that interval was exposed at a finer scale,
recursing until it reaches blocks and their detailed handle lists. Each refinement
must be authenticated and its child digests must recompute the coarser parent digest
being refined. Equal child ranges are pruned; every differing child range is added
to the refinement worklist. The traversal finishes only after every differing block
in the covered interval has been identified or the remaining intervals have been
recorded explicitly as unresolved because finer authenticated history is unavailable.

This is Merkle-style navigation without publishing one large tree. New manifests
carry detailed recent blocks and coarse roots for older history, while older
manifests expose those same intervals at progressively finer resolutions. A verifier
fetches only the historical manifests needed to descend through differing ranges.
The resulting block and handle findings refer to the block that generated each new
ct64; manifests do not index later blocks that merely reuse that handle.

This gives precision proportional to drift age. A recent mismatch is localized by
small ranges. An older mismatch is first localized to a larger range and needs more
historical manifest reads to reach the exact block. Manifests stay small because they
never repeat historical handles.

### Full-consensus boundary

The last full-consensus checkpoint is the newest `(block_number, block_hash)` through
which all configured coprocessors have established agreement on the covered block
commitments of that lineage. It bounds retained history and cleanup. A checkpoint
from another lineage is never used to truncate this manifest's history.

Let `C` be its block number. A normal manifest builds its historical ranges backward
from the block immediately preceding its detailed range. It adds ranges from newest
to oldest until it reaches the first range `R` satisfying:

```text
R.start <= C <= R.end
```

and stops. No older range is emitted. Blocks older than `C` that happen to fall in
this terminal range are only dyadic-alignment overlap. When full consensus advances,
future manifests stop at the range containing the newer checkpoint and older range
nodes become eligible for cleanup, subject to unresolved drift evidence and reorg
retention.

If full consensus has never been established, the manifest includes the largest
history possible from the block hashes and complete block state known locally. It
stops only where the publisher can no longer prove the preceding block on that
lineage. Publishers may therefore expose different initial history lengths without
creating false drift; every exact shared range remains comparable and unmatched
older ranges are ignored.

Full consensus can advance only from authenticated latest manifests of all configured
coprocessors. Their detailed and historical ranges must provide gap-free comparable
coverage from the preceding full-consensus boundary through the new checkpoint, and
all roots and detailed entries in that common coverage must agree. A threshold result
is sufficient for remediation attribution under the configured safety policy, but
not for advancing this all-coprocessor cleanup boundary. Drift detection itself does
not require threshold support.

For the first full-consensus checkpoint, there is no preceding boundary. Its
certificate therefore starts at the oldest block in the gap-free coverage shared by
all latest manifests. Additional older ranges exposed by only some coprocessors are
ignored and are not included in that certificate.

### Lineage and reorgs

Manifest publication does not wait for host-chain finality. As soon as a block and
its ciphertext set are locally complete, it can participate in the lineage and be
published at a configured publication height.

Block and range digests are maintained independently per lineage. A child can be
processed only after its parent block identity is known. A dyadic parent range can
be formed only from children on that lineage. On a reorg, the worker resumes from
the common ancestor, computes independent block and range commitments for the new
lineage, and publishes new immutable manifests under the new block hashes. The old
lineage objects remain available as evidence and are not overwritten or revoked.

A single coprocessor can therefore publish multiple manifests with the same block
number and different block hashes. They coexist under different S3 keys and archive
identities. The authoritative tip for comparison is the highest authenticated
revision for that publisher and exact block identity.

Publication scheduling is ordered only within a lineage, not across every hash on a
chain. A pending block prevents its own descendants from overtaking it, but it does
not prevent an independent fork frontier from progressing. During one chain-locked
scan, an incomplete block or a manifest creation/upload failure is skipped and the
worker tries the next eligible lineage frontier. The failed candidate remains pending
and is retried on later scans; immutable S3 writes make an upload whose database
transaction failed safe to retry.

### Manifest format and layout versioning

The manifest carries `version: 1` in its signed envelope. Version `1` uses
the stable
`/manifests/v1/<coprocessor_context_id>/<host_chain_id>/<block_height>/<block_hash>/<revision>`
layout. The path version and signed body version must match, and the path revision
and signed body revision must match; any mismatch makes the object invalid.

The version covers the S3 discovery layout, required fields, canonical encoding,
hash algorithms and domain separators, and signature payload. It is included in the
signed payload and the hash domains for `A`, detailed-range digests, dyadic range
digests, and the manifest digest.

A reader must never parse an unknown version as the current format; it records the
object as unsupported until that version is implemented. A future version uses a
new path such as `/manifests/v2/...` and carries the matching signed body version.
Readers may support multiple layouts during a rolling upgrade, but comparison or
bridging of range digests across versions must be explicitly defined. Version 1
readers never infer compatibility from equal JSON fields alone.

Manifests with different block hashes at the same height belong to different lineages
and are not comparable for ciphertext drift. They establish neither agreement nor
drift with each other.

Commitments are compared only over the same chain, exact block or dyadic range
identity, and lineage. That comparison does not wait for finality. Different block,
detailed-range, or exact historical-range digests establish `drift` immediately,
whether or not any digest group reaches quorum. Quorum is recorded separately and
controls whether one observed value is safe to use as an automatic-remediation
reference. Once observed, drift remains a correctness finding even if the block is
later orphaned. Whether that lineage eventually wins the reorg is a separate
annotation and does not retroactively erase the finding.

A divergence on an orphaned lineage does not contaminate a later canonical lineage:
each branch has independent block and range roots. No digest is carried from one
branch into another merely because their block numbers match.

### Finality and transient forks

Different RPC providers may temporarily expose different branches during the
host-chain finality window. This is expected lineage disagreement, not ciphertext
computation drift. Quorum grouping never mixes block hashes, even when their block
heights are equal.

| Observed manifests | Verification outcome | Local revert |
| --- | --- | --- |
| Different block hashes at the same height | `different_lineage` | Never |
| Same block hash, fewer than quorum, all results equal | `unknown_but_equal` | Never |
| Same block hash, fewer than quorum, results differ | `drift` with `observed_has_quorum = false` | Never |
| Same block hash, one identical result reaches quorum and another differs | `drift` with the winning group recorded | Only a local value outside the winning group may become eligible |

A branch observed by too few coprocessors can establish drift when comparable
content differs, but it cannot authorize drift-revert. If that branch later
disappears, its observations remain fork evidence. If enough coprocessors
independently agree on that branch, it may also establish a remediation reference
for that lineage; a later reorg still does not turn branch agreement into
computation drift.

Restricting publication and verification to finalized blocks would remove most fork
bookkeeping, unknown outcomes on short-lived branches, and orphan-lineage retention.
It would also delay detection and deliberately stop detecting computation drift on
branches that are later orphaned. The current pre-finality design is safe against
fork-induced revert because of exact-hash comparison and quorum attribution, but it
is more operationally complex. A post-finality-only design would therefore be a
scope change, not merely an implementation optimization.

### Historical localization and drift cursor

The latest manifests first compare their detailed ranges and every exact dyadic
range they share. A differing range root brackets the drift without downloading all
intermediate manifests. To refine it, the verifier keeps a worklist of every
differing range and reads older authenticated manifests that expose each interval at
a smaller scale. Equal child ranges are discarded and every differing child is
refined again. The verifier does not stop after the first differing block: every
local reporter continues until it has found every differing block in the covered
interval, then compares the sorted handle descriptors for those blocks while
building its mutable drift inventory. Membership in a quorum group changes
remediation eligibility, not localization completeness.

Downloaded historical manifests are inserted into the same durable manifest archive
as newly observed tips. Recursive localization first consults that archive and only
fetches an older S3 object when its signed body is not already stored. This makes the
redundant resolutions carried over successive manifests a reusable local range index
without introducing a separate normalized peer-manifest representation.

Once one parent commitment has reached quorum, refinement does not require the same
manifest from every member of that quorum. Any available publisher in the winning
commitment group may provide the finer historical ranges or detailed block list. The
verifier accepts that refinement only when its authenticated child digests recompute
the quorum parent digest. It may use a different winning publisher at a later level
under the same rule. The preferred S3 candidate for a differing range `[L, R]` is the
earliest manifest on that lineage whose detailed range contains `R`, followed when
needed by its immediate successor where the post-`R` frontier is exposed; a much
later manifest may already contain only a coarser root.

The local side of the comparison uses its current persisted block and range digests,
so it does not need to redownload its own manifests. On the quorum side, the worker
tries a stored manifest first, then downloads it from the preferred winning
publisher, then tries another publisher carrying the same quorum commitment. A
missing publisher is not needed when the remaining publishers already establish
quorum. At block resolution, one complete signed descriptor list whose `A` recomputes
the quorum block digest is sufficient as the consensus reference; duplicate copies
of that handle list from every quorum member are unnecessary.

The ending block hash may be used in deterministic S3 lookup, but the block hash
alone is not a secure manifest reference: two manifests can describe the same
host-chain block while committing to different ciphertext results. A predecessor or
localization reference therefore includes the referenced manifest digest, and the
fetched object must match it.

Once a drift has been attributed but not yet repaired, the drifted coprocessor keeps
a lineage-specific durable scan cursor containing at least the last completely
inspected block number and hash. It also keeps the accumulated unresolved handle
findings and the range roots or manifest revisions that justified the cursor. New
manifests are compared only after that cursor; already inspected handles are retained
and newly discovered differing handles are appended.

The current handle inventory is stored in `block_consensus_drift_handle`, with one
row per local publisher, context, chain, exact block hash, handle, and distinct
observed commitment. A row keeps both the local and observed descriptors, the
individual keyset, ct64, ct128, and format mismatch flags, optional local
`gateway_key_id` provenance, and immutable references to the first and latest
verification evidence. Every localized difference creates an `unresolved` row.
`observed_has_quorum` states whether that observed descriptor belongs to the unique
threshold group and is therefore eligible to become a remediation reference; a
below-quorum row is persistent diagnosis only.

Replay publishes a superseding local manifest revision; it does not edit the
manifest that established the drift. When a later detailed-range verification puts
that local revision in a threshold-sized concordant visible group, all unresolved
rows for the exact covered block hashes are updated to `resolved` with the resolving
target, manifest digest, revision, and timestamp. The original drift descriptor
values and first-detection evidence remain available for diagnosis. Revision guards
prevent an older worker from overwriting a newer result.

For each later interval having complete quorum reference manifests, the drifted
coprocessor continues the normal linear comparison and adds only handles that
actually differ. If manifest availability drops below quorum for a particular
interval while that incident remains active, it fails conservatively only for that
unverifiable interval: every locally expected allowed handle in those blocks is
added to the repair inventory and recomputed. The cursor advances only after the
comparison findings or conservative inventory for the complete interval are durable.
Conservative rows are marked `unverified_during_drift`, rather than asserted as a
quorum-confirmed digest mismatch. Later quorum evidence verifies the repaired result;
it does not remove conservative work from the queue before recomputation.

The cursor advances only through completely inspected contiguous blocks. It is
rolled back or invalidated after a reorg, a superseding historical commitment that
changes evidence below the cursor, or discovery of an earlier uncovered interval.
Other coprocessors need not maintain this mutable repair view, but they retain the
immutable signed manifests and compact drift evidence needed to verify the incident.

### Publication

A manifest is published only after the coprocessor has a reliable signal that the
ciphertext set for the publication block and its preceding unpublished ancestors is
complete. Prefer sealing only once the full allowed set and digests are final.
Publication should then:

1. resume from the durably processed lineage state and last published tip;
2. process a block only after its parent lineage state is available;
3. compute `A` for every block, including empty blocks, and persist any newly
   completed dyadic parent roots;
4. at each configured publication height, attach the ordered block identities,
   values of `A`, and complete handle-to-descriptor association lists accumulated
   since the preceding manifest;
5. compute the sequential detailed-range digest, attach the dyadic history through
   the range containing the last full-consensus checkpoint (or all locally known
   history when none exists), and attach the preceding-manifest reference;
6. assign the next `revision` for this publisher and block identity (`0` on first
   seal, otherwise one more than the last successfully published revision);
7. compute the manifest digest over the complete envelope including `revision`;
8. sign the complete manifest with the coprocessor identity;
9. write it under the immutable key ending in `/<revision>`;
10. optionally update an unsigned tip pointer only after that object exists; and
11. durably schedule peer verification for the end of
    `--consensus-verification-delay`, including for a superseding repair revision.

The first manifest has no historical section: with no preceding manifest, every
available contiguous `block_consensus` row belongs to its detailed range. For each
later manifest, the new canonical history is built from the preceding manifest's
history plus its detailed blocks. The current cadence's newly discovered blocks stay
in the new detailed range and enter history only when the following manifest is
prepared.

A missing local archive row is not the same as having no preceding manifest. When a
durable preceding-manifest reference exists but its signed body is missing locally,
the publisher reconstructs the same canonical frontier from the retained block and
range lineage, preserves that preceding reference, and publishes the successor. It
must not silently reinterpret the successor as the first manifest.

After an outage, the worker reconstructs missed scheduled checkpoints in lineage
order rather than skipping directly to the current chain head. Retrying the same
publication block hash at the same `revision` must be idempotent (identical body
digest). A different body is permitted only through the repair or correction flow
below and must use a new `revision`.

#### Superseding revisions after repair or correction

A publisher may append a higher `revision` for the same block identity only after an
explicit repair or correction changes an already sealed commitment, including:

- local drift-revert or re-execution that changes the association list or digests;
- correction of a detected implementation or data-integrity fault in an earlier
  seal; or
- roll-forward republish of later manifests whose detailed or historical range roots
  depended on a repaired ancestor `A`.

Normal late completion is not a revision mechanism: the initial manifest must wait
for the completeness barriers and must never seal an incomplete ciphertext set.

Each revision object is immutable. Quorum and drift classification use only the
latest authenticated revision per publisher. Older revisions are retained as a stack
for audit and anti-equivocation analysis; they are not additional votes. Different
authenticated manifest digests at the same immutable identity are equivocation. A
tip that decreases is an invalid discovery result.

Without supersession, a residual honest published quorum can still let a repaired
operator self-align by comparing local commitments to peer tips, but peers keep
seeing the operator's old tip as drift, and a mass-drift case where no correct tip
was ever published cannot discover a new agreement on S3. Supersession is therefore
required for collective recovery.

A partially written or unsealed manifest must never participate in comparison.

Publication and verification are separate asynchronous phases. Successful local
publication does not immediately fetch or compare peer manifests. Every successfully
published local revision, including a superseding repair revision, creates its own
durable verification target using the configured `--consensus-verification-delay`.
The delay gives slower coprocessors time to process the same lineage and publish their
tips. The target and its eligibility time must be durable so a worker restart does not
skip the comparison.

Before the delay expires, absent peer manifests are expected lag: the observation
remains `waiting_for_delay` and does not produce a missing-manifest failure or drift
finding. Once eligible, the worker runs peer discovery and comparison as described
below.

### Verification retry and tip reconcile

Every verification attempt re-resolves each expected publisher's **latest**
authenticated revision for the target block identity (list the block prefix, probe
revisions, or follow a tip pointer and then verify the body). It never trusts the
`consensus-status` tag to include or exclude a peer. Cached verified bodies may be
reused only when the tip revision and content digest are unchanged.

Quorum groups **tips only**: one signed manifest per distinct authorized publisher,
taken from that publisher's highest valid revision. Grouping is performed for each
exact comparison scope: the detailed range, a shared dyadic range, or an exact block
and descriptor set reached during localization. An unmatched historical range is
outside the comparison scope rather than a different vote.

`--consensus-verification-retry-count` is the number of additional attempts allowed
after the initial verification attempt. The same delay
(`--consensus-verification-retry-delay`) and count budget apply to:

- `unknown_but_equal` (visible tips are concordant but insufficient for quorum);
- `drift` while any visible comparable tip still differs, including when no digest
  group has quorum; and
- `consensus` only when later tip revisions can extend the recorded coverage.

Repair-in-progress and still-drifted are indistinguishable until a tip changes. The
worker does not need a separate "wait for new revision" signal: an unchanged bad tip
classifies the same way on the next poll; a new matching tip converges; a new
non-matching tip is continued or renewed drift. Mass drift therefore converges more
slowly (each operator repairs and publishes a higher revision; later polls form
quorum) but uses the same loop as single-peer drift.

The number of retries already performed and the next eligible attempt time are
persisted in the database and updated atomically, so restarts neither reset the
budget nor lose a scheduled retry. Every attempt polls for higher peer revisions, so
a changed peer tip is consumed while that bounded budget remains. Publishing a local
superseding revision creates a fresh verification target for that exact revision even
when the preceding target was exhausted. An exhausted target is not polled forever;
reopening one after only a remote tip changes would require a separate durable trigger
or periodic reconcile policy. Peer absence may become a missing or stale-peer finding,
but never ciphertext drift by itself.

### Multi-worker verification locking

A coprocessor may run several `sns-worker` replicas against the same database. A
replica claims one due verification target in a short transaction by selecting the
dedicated verification-target row with `FOR UPDATE SKIP LOCKED`, recording a unique
worker ID and an expiring lease, and committing. A row with a live lease is skipped,
preventing two replicas from verifying the same target concurrently.

An eligible verification row already has a sealed detailed-range digest and manifest
revision. Verification does not create a manifest or compute block, detailed-range,
or dyadic-history digests. Publication and range construction remain a separate
database-coordinated phase.

S3 list and GET operations run after the claim transaction commits, so no database
transaction or row lock is held across network I/O. Every authenticated numbered
revision is inserted into the signed-manifest archive in a short transaction. The
per-peer row then records that the peer completed the current attempt. If a process
dies after storing only part of an attempt, another replica reclaims the expired
lease, skips peers that completed that same attempt, and omits already archived
revision bodies from subsequent GETs.

After all remaining peers complete, the lease owner opens one final short
transaction, selects tip-eligible revisions from the archive, computes the result,
and atomically updates the outcome, retry count, next-attempt time, and exhaustion.
An expired or replaced lease cannot finalize or mutate peer progress. Each S3
operation still has a bounded SDK timeout, and each claim processes only one target.

A later external S3 tag updater may run after this transaction commits. It remains
best-effort because an S3 call cannot be made atomic with the PostgreSQL transaction;
failure to update the tag must not change the persisted verification outcome.

### Verification outcomes and quorum

For one block identity and manifest version, the verifier pins one registry snapshot
for the complete decision, then resolves valid **tips** (latest authenticated
revision per publisher) for the same lineage checkpoint. It groups publishers by
their detailed-range digest and by each exact shared dyadic-range digest. Historical
ranges that are not exposed by all compared publishers remain outside that range's
vote. Only distinct publishers authorized by the pinned snapshot count toward its
quorum. Lower revisions are evidence only.

The local operator's side of the comparison uses its **current durable commitment**
for the block identity (which must match its latest published tip once publication
has succeeded). After local repair, the operator publishes a higher revision before
peers can observe the repair; self-status may already match a residual honest quorum
from local state, but peer views of this operator update only when the new tip is
visible.

The verification outcome is:

- `consensus`: all visible comparable commitments agree and that value reaches the
  configured quorum;
- `drift`: at least one exact detailed or historical scope contains two different
  visible commitments, whether or not either value reaches quorum; or
- `unknown_but_equal` ("insufficient quorum, but equal"): all visible comparable
  commitments agree, but that value does not reach quorum.

Outcomes are first evaluated independently for every exact detailed or historical
scope. Any visible difference makes the aggregate result `drift`; a matching recent
detailed range never overrides a historical mismatch. Historical localization
controls the precision of the block and handle inventory, not drift existence.

Every outcome records its exact comparison scope: detailed-range bounds, exact
dyadic range identities, uncovered intervals, and every localized drift block or
remaining unresolved range when applicable. A bare `consensus` result must never be
interpreted as agreement outside that recorded coverage. Separately, the
full-consensus checkpoint advances only when all configured coprocessors agree over
the required gap-free coverage.

Insufficient-quorum and non-final drift/reconcile situations schedule another attempt
after `--consensus-verification-retry-delay` while the extended retry policy allows
it (see [Verification retry and tip reconcile](#verification-retry-and-tip-reconcile)).
If the unknown budget is exhausted without a matching tip appearing during the
bounded attempts, the
worker retains `unknown_but_equal` and marks retry exhaustion rather than guessing
consensus. A pairwise content difference is never an unknown outcome: it is `drift`
and its local-versus-observed descriptor explanations are persisted. Quorum remains
separate metadata and is required before the observed side can authorize automatic
remediation.

#### Comparison with the current Gateway contract

The current Gateway `CiphertextCommits.addCiphertextMaterial` groups votes by
`keccak256(handle, keyId, ciphertextDigest, snsCiphertextDigest)`. Each registered
transaction sender can vote once per handle. A material tuple is finalized when its
counter reaches `GatewayConfig.getCoprocessorMajorityThreshold()`, or immediately
when the configured priority sender submits it. Different tuples occupy different
counter buckets; the contract emits each submission but does not expose a durable
`drift` classification for non-winning buckets.

| Manifest behavior relative to Gateway | Classification | Consequence |
| --- | --- | --- |
| Exact descriptor values are grouped before quorum, as Gateway groups the complete material tuple | Equivalent | Quorum never combines votes for different content |
| Any two comparable manifest values establish persistent `drift`, even when neither reaches threshold | Improvement | All-different and split-vote failures are diagnosable instead of remaining only separate on-chain vote buckets/events |
| Quorum is still required before one value becomes an automatic-remediation reference | Equivalent safety property | Detection is quorum-independent; correction authority is not |
| A publisher may issue a higher signed manifest revision while every revision remains archived | Improvement with an explicit semantic change | Recovery can be observed without deleting evidence; unlike the Gateway vote, the publisher's current tip can change |
| Manifest verification has no priority-sender shortcut | Intentional improvement for decentralized consensus | One publisher cannot make its own material the remediation reference; this differs from current Gateway priority mode |
| Manifest observations are downloaded asynchronously rather than written atomically on chain | Regression/risk | Missing or delayed tips can yield `unknown_but_equal`; bounded retries and durable queues are required |
| Manifests cover ct64, ct128, format, keyset and history, not only the Gateway material tuple | Improvement | Drift can be localized across historical blocks and explained field by field |

These differences are intentional for the planned removal of Gateway consensus.
They must not be interpreted as reproducing the Gateway's first threshold-winning
state machine off chain.

#### Degenerate comparison without manifest-level quorum

When no commitment reaches quorum for the top-level manifest or detailed-range
scope, the verifier does not choose the largest group. If visible commitments
differ, the result is already `drift`; it enters a slower localization path over all
authenticated, comparable data on the same lineage.

The threshold at every scope is the same global coprocessor threshold loaded with
`getCoprocessorMajorityThreshold()` from the pinned `GatewayConfig` snapshot. It is
separate from the individual coprocessor records that contain signer identities and
`s3BucketUrl`, and it is never recomputed from the number of available manifests.

The path is:

1. expand the affected coverage range by range, fetching missing signed manifests
   when useful, until exact block entries are available or an interval is explicitly
   `unresolved`;
2. for every covered block, group the available authorized publishers by the exact
   block digest `A` using the unchanged configured threshold;
3. when one `A` reaches threshold, record block-level consensus and classify
   different available block digests as block-scoped drift; and
4. when no `A` reaches threshold and complete signed block association lists are
   available, compare the union of their handles one handle at a time.

For handle comparison, each publisher with a complete authenticated block list casts
one vote for either the full descriptor tuple or explicit `ABSENT`. A handle omitted
from such a complete list is an `ABSENT` vote. A missing, unreadable, incomplete, or
unauthenticated manifest casts no vote and is never interpreted as absence. A
descriptor or `ABSENT` value that reaches the configured threshold is a handle-scoped
consensus result; other available values for that handle are handle-scoped drift. If
no value reaches threshold, that handle remains unknown.

The degenerate path therefore provides block-by-block and, when necessary,
handle-by-handle results even when no complete manifest variant has quorum. It never
lowers the threshold to the number of manifests that happened to be available,
never assembles those scoped results into a synthetic manifest vote, and never
advances the full-consensus checkpoint. The overall outcome remains `drift`, while
every persisted comparison row has `observed_has_quorum = false` and is ineligible
for automatic revert.

### Manifest consensus-status tag

After each completed verification attempt, `sns-worker` adds or replaces this S3
object tag on its own **current tip** manifest object (the highest local revision for
that block identity):

```text
consensus-status = consensus
                 | drift
                 | unknown_but_equal
```

The tag is absent only before the first verification attempt completes for that tip.
An insufficient-quorum or drift outcome is written immediately and may be
overwritten by a later retry or reconcile attempt. `Drift` means visible comparable
content diverged; it does not by itself claim that the local publisher is outside a
quorum group. After retry exhaustion, the last outcome remains visible on that tip
until a superseding local revision creates a fresh target or a separately scheduled
reconcile attempt updates it.

Object tagging does not rewrite the manifest body or its S3 user metadata. The tag
is externally observable operational evidence for manual inspection, integration
tests, and other monitoring that does not have database access. Such observers read
it with S3 object-tagging APIs and may use it to assert what conclusion the operator
published.

The tag is nevertheless a self-reported, unsigned conclusion. It must never be
counted as a vote, used to skip peer tip resolution, or trusted by itself for KMS
consensus, drift attribution, or automatic revert. Peers that are or were tagged
`drift` remain in the comparison set until their latest authenticated revision is
evaluated. An observer requiring authoritative proof recomputes the outcome from the
signed tip manifests. The database stores the authoritative local outcome; a missing
or stale tag is inconclusive and does not indicate verification failure. The worker
requires `s3:PutObjectTagging` only for manifests in its own writable bucket or
isolated prefix. Updating `consensus-status` must preserve unrelated object tags.

### Peer comparison

Each `sns-worker` reads the manifest prefix of every expected coprocessor using
read-only credentials, verifies the manifest identity and signature, and persists
what it observed locally. This is a monitoring path and is not consulted by KMS for
per-ciphertext consensus.

Local and peer manifests use one durable storage format. Every structurally valid,
signature-verified downloaded manifest is stored as the same exact signed bytes as a
locally published manifest, keyed by the immutable manifest identity defined above.
The archive's `publisher` is copied from the signed payload after signature
verification.

Inserting the same object bytes again is idempotent. A different authenticated
manifest digest for the same immutable identity is equivocation evidence. Different
wire bytes that decode to the same canonical payload and valid signature are an S3
mutation or encoding anomaly, not an additional vote; the anomaly is retained for
audit. Every observed revision is retained without overwriting earlier numbers, and
after peer tip discovery consensus selects the highest authenticated stored revision
for each publisher and block identity. Whether a manifest is local or peer-authored
is derived by comparing `publisher` with the configured local publisher; no separate
`is_local` or `is_peer` field is needed. Registry authorization is evaluated under
the pinned verification snapshot and is not part of the immutable stored-manifest
identity.

The archive records `first_seen_at`, meaning when this worker first stored the
manifest. It does not claim to know when a peer actually published it. Historical
range and descriptor data are read directly from the stored signed body and may be
decoded into an in-memory cache when useful; the initial design does not normalize
peer ranges or descriptors into additional tables.

The storage topology may use either independently operated S3 buckets with mutual
read access or one shared S3 namespace. In a shared namespace, each coprocessor must
have an isolated write prefix so it cannot overwrite another publisher's manifests.
In both models, every reader needs all expected publisher locations and must verify
the signed publisher identity rather than trusting the bucket or object path alone.

The current discovery baseline is `GatewayConfig`: every registered coprocessor has
a transaction-sender address, signer address, and `s3BucketUrl`. Workers and KMS use
that registry snapshot to resolve the expected publishers, signatures, bucket
locations, and majority threshold. In the current integration, `s3BucketUrl` points
to the public ct128/SnS ciphertext bucket; commitment-manifest keys are relative to
that registered base URL unless a separate manifest location is introduced later.

Current `sns-worker` does not query `GatewayConfig`; it only knows its locally
configured ct64 and ct128 bucket names. The manifest implementation must add a
Gateway RPC endpoint and `GatewayConfig` address, then build a cached registry
snapshot by:

1. calling `getCoprocessorTxSenders()` and `getCoprocessorSigners()`;
2. calling `getCoprocessor(tx_sender)` for each registered transaction sender;
3. retaining the record's signer identity and `s3BucketUrl`; and
4. loading `getCoprocessorMajorityThreshold()` for comparison decisions.

`getCoprocessor` must be keyed by transaction-sender address, not signer address.
The snapshot is refreshed on registry updates and is pinned for each comparison so
one decision cannot mix two registry snapshots.

### Peer manifest discovery and cadence mismatch

When delayed verification runs for a locally published manifest, a peer may still
have no manifest at the identical height because it is late or accidentally
configured with another cadence. The reader:

1. first looks for the peer manifest at the local publication block;
2. if it is absent, resolves the peer's newest authenticated tip on the same lineage
   using bounded prefix listing or a discovery pointer followed by manifest
   authentication;
3. selects the newest valid peer manifest found within the scan and freshness
   limits; and
4. compares its detailed range and the intersection of exact dyadic historical range
   identities with local durable commitments under one pinned registry snapshot.

This comparison does not require the local coprocessor to have published a manifest
at the peer's height. Every block advances local lineage and range state, so the
verifier can compare a peer's exact ranges against locally persisted roots. The
verifier attributes `drift` to a specific operator only after an identical result
reaches quorum.

Discovering a different block hash at the same height requires either listing that
height's S3 prefix or reading a per-peer discovery pointer and authenticating the
referenced manifest. It is useful for
explaining why the matching path is absent, but the different-hash manifest does not
participate in drift comparison.

If an exact historical root differs, the reader follows authenticated manifest
references to obtain finer roots until it identifies the block, reaches unavailable
history, or reaches a retained full-consensus boundary. Missing finer history leaves
the drift bracketed to the last differing range; it does not erase the finding.

For a range whose parent digest already has quorum, the reader searches the durable
archive for a refinement from any publisher in that winning digest group. On a cache
miss, it tries the closest manifest whose detailed range covers the range end, then
the next manifest exposing the resulting frontier. If that object is unavailable, it
tries another winning publisher before treating the range as unresolved. Every
accepted refinement must recompute the already authenticated quorum parent. Missing
manifests from other expected publishers do not block localization while the winning
group still supplies a valid refinement.

Tip discovery and any finer-range localization must be bounded and resume from
durable per-peer state; they must not issue an unbounded number of S3 requests on
every poll. A peer manifest older than the configured freshness limit is reported as
stale even if it is valid.

Every observed peer publication height `H` is checked against `H mod K == 0` for the
expected chain cadence. Finding an authenticated peer manifest for which this is
false is direct evidence of cadence misconfiguration. It emits and persists a
cadence-mismatch warning, but it does not invalidate the manifest and is not itself
ciphertext drift; drift comparison still runs at height `H` on the same lineage.
The warning includes the peer, expected `K`, incompatible observed height, and block
identity. An irregular sequence of otherwise compatible publication heights may
also be reported, but missing expected heights alone remain ambiguous with lag or
S3 availability.

For each verification target and discovered peer observation, the worker must
distinguish at least these states:

- local manifest not sealed;
- waiting for the configured verification delay;
- waiting for peer manifests;
- consensus quorum reached;
- different peer lineage observed but not comparable;
- `unknown_but_equal` while retries remain or exhausted;
- `drift`, with or without a quorum-backed observed value;
- invalid or unauthenticated manifest.

Absence at one exact publication height is not yet an availability failure because a
peer may use another cadence. Failure to find a sufficiently recent valid manifest
within the bounded scan is an availability or lag failure, not proof of drift. A
difference requires comparable authenticated commitments for the same block identity
under one pinned registry snapshot. Automatic remediation additionally requires a
unique quorum of an identical competing commitment.

For the production-shaped five-coprocessor case with quorum `3`, verification must
cover every possible local publisher origin for at least these populations:

| Intended population | Observable commitment groups | Result for each local origin |
| --- | --- | --- |
| One drifter | `4 + 1` | Five `drift` summaries; four local values belong to the quorum group and one does not |
| Two matching drifters | `3 + 2` | Five `drift` summaries; three local values belong to the quorum group and two do not |
| Three drifters, two matching and one distinct | `2 + 2 + 1` | Five `drift` summaries without a quorum-backed remediation reference |
| All five drifted as two matching pairs and one singleton | `2 + 2 + 1` | Five `drift` summaries without a quorum-backed remediation reference |
| Every coprocessor differs | `1 + 1 + 1 + 1 + 1` | Five `drift` summaries without a quorum-backed remediation reference |

The two `2 + 2 + 1` populations are intentionally indistinguishable from manifests
alone. Neither has a quorum-backed reference, so the implementation must not infer
which pair, singleton, or intended ground truth is correct. Tests evaluate the same
manifest population once with each publisher as the local origin. No-quorum prevents
automatic remediation, not drift detection or persistence: every summary and handle
row records the distinct commitment groups and explanations derivable from the
signed descriptors, such as `keyset_mismatch`, `ct64_digest_mismatch`,
`ct128_digest_mismatch`, `format_mismatch`, or handle-set differences.

Revision tests use the production publisher and object-store path: revision `0` and
its repaired revision `1` are created from database descriptors, signed, uploaded to
their numbered immutable S3 keys, archived, and linked by `supersedes`. A dependent
later manifest is also republished so its `previous_manifest` points to the repaired
revision. Separate multi-worker downloader tests retain every downloaded revision,
select only the highest contiguous authenticated tip, and resolve persisted drift
when a matching peer revision appears during the bounded retry window.

Comparison has two useful cases:

- different `A` values at a compared block mean the coprocessors disagree on the
  ciphertext content attributed to that block;
- different detailed-range digests mean at least one block in the current
  publication interval differs; and
- different roots for the same dyadic range mean at least one block in that exact
  historical interval differs.

When a range root differs, workers fetch progressively finer historical manifests.
Every reporter traverses each differing child range until it has enumerated all
differing blocks in the covered interval. It uses authenticated manifests matching
each observed digest as its references, compares the signed handle-to-descriptor
lists for every differing block, and records whether each finding is a missing
handle, an unexpected handle, or a descriptor mismatch. A reference belonging to a
unique quorum group is marked separately. Fetching ciphertext bodies is not required
for this comparison; a later investigation may compare S3 object metadata or bodies
against the manifest evidence.

Manifests at the same height but under different block hashes are recorded as
different lineages, not drift. Their `A` values are not two claims about the same
block content because the blocks themselves differ.

### Consensus summaries

Every coprocessor persists a signed summary from its own local viewpoint for each
completed verification decision. The summary identifies the reporter, pinned
registry snapshot, exact comparison scope, local manifest reference, observed
commitment groups and publishers, quorum threshold, outcome, and, when one exists,
the winning commitment and its manifest references.

A reporter whose local commitment belongs to the winning group can publish the
summary as soon as the quorum decision is durable. A reporter outside that group
first durably records its localized local-versus-observed findings and scan cursor,
so the summary and remediation inventory cannot disagree after a crash. An exhausted
no-quorum evaluation publishes `unknown_but_equal` when every visible value agrees,
or `drift` when visible values differ. Neither is remediation authority without a
quorum-backed observed descriptor.

The summary preserves the local, observed, and quorum metadata instead of flattening
them into one tuple. Every local-versus-observed difference is drift evidence. When
no group reaches quorum, the summary still contains each non-local commitment group
and its descriptor explanations, but marks every comparison
`observed_has_quorum = false`; it must not collapse diagnosable drift into an
unexplained `unknown`.

### Public drift-evidence files

When comparable results differ, every coprocessor that detects the drift publishes a
compact signed summary in its own registered bucket. The summary identifies every
observed group and whether a unique group has quorum. Each reporter maintains its
own local-versus-observed inventory; only comparisons against a quorum-backed group
are eligible for automatic remediation.

The compact evidence flow is:

1. use the differing detailed or dyadic range roots to find the first differing
   block on the same lineage;
2. identify every commitment group and, when one exists, the quorum commitment;
3. record immutable references and digests for the manifests establishing that
   attribution; and
4. publish the signed summary without handle associations.

Evidence summaries use this immutable, listable path in each detector's registered
bucket:

```text
<s3BucketUrl>/drift/v1/<coprocessor_context_id>/<host_chain_id>/<origin_block_height>/<origin_block_hash>/<revision>
```

The detector's registered `s3BucketUrl` identifies the reporting coprocessor. In a
shared physical bucket, each registered base URL must resolve to an operator-isolated
prefix so detectors cannot overwrite one another's summaries. Summary revision `0`
is the first complete report for that origin block. Later complete reports increment
the final key component, retain the preceding summary digest in the signed body, and
never overwrite an earlier revision. An optional unsigned tip pointer may accelerate
discovery but is not evidence.

An exact evidence file contains at least:

- format version, detector identity, detector signature, and detection timestamp;
- coprocessor context ID, host chain ID, the verifier's pinned registry reference,
  and configured quorum;
- the comparison checkpoint and exact origin block identities;
- the comparison scope, including detailed-range bounds, exact differing dyadic
  ranges, uncovered intervals, and the localization path;
- the winning commitment values and the distinct authorized publishers establishing
  quorum;
- each drifted publisher and its differing commitment values;
- the immutable S3 keys, revisions, and digests of the quorum and drifted manifests;
  and
- the range or predecessor references used for block localization.

The compact summary contains commitment metadata and manifest references, not handle
associations or ciphertext bodies.
The detector signs the complete canonical evidence envelope with its registered
coprocessor signing identity. Its signature proves which detector published the
report; external observers fetch and verify the referenced manifest signatures,
distinct quorum membership, lineage, and digest computations.

If missing retained history prevents exact block localization, the differing range
root and its bounds are still persisted locally, but the detector cannot truthfully
use the origin-block summary path. Localization remains retryable until the exact
origin is available.

Evidence publication is durable and retried, unlike the best-effort
`consensus-status` tag. A database evidence row keyed by the detector, context, host
chain, origin block, and revision tracks the immutable summary key, content digest,
publication state, and attempts. Multiple `sns-worker` replicas claim evidence rows
with `FOR UPDATE SKIP LOCKED`. If later verification discovers more drifted results
or stronger evidence, the detector appends a higher signed revision. The numbered
keys and signed predecessor-digest chain make the complete history independently
auditable without relying on S3 bucket versioning.

To make drift publicly discoverable, every registered bucket must allow
`s3:ListBucket` for the `/drift/` prefix and `s3:GetObject` for its summary
objects. Public ciphertext reads alone do not imply permission to list this prefix.
An external observer discovers all reports by resolving the registered coprocessor
buckets and listing `/drift/` in each one.

### Retention and deferred cleanup

Manifest revisions are append-only: cleanup may eventually remove an eligible old
object, but it never replaces one revision with another. A downloaded or locally
published revision remains in the durable archive and is reused for comparison until
the future retention policy explicitly makes it eligible for removal.

Cleanup eligibility must never be inferred from the unsigned, best-effort
`consensus-status` S3 tag. It is derived from authenticated manifests and durable
database state. A future cleanup policy may remove a manifest only after the
configured recovery window when all of the following hold:

- the relevant lineage coverage is below a durable full-consensus checkpoint;
- the manifest is not a current publisher tip or required predecessor;
- no unresolved drift, unknown outcome, localization cursor, public evidence, or
  retained reorg depends on it; and
- removing it cannot turn previously available fine-grained history into an
  uncovered interval for an active incident.

All numbered public drift-evidence revisions and every manifest referenced by them
remain retained. Status tags remain operational hints only and play no role in
retention authority.

The cleanup mechanism is intentionally deferred. The ct128/SnS bucket contains
other information, so an incorrectly scoped bucket Lifecycle rule could delete
unrelated objects or interfere with existing bucket rules. No automatic manifest
deletion is enabled in the first implementation; all objects are retained until
bucket ownership, exact prefix scoping, versioning behavior, and configuration
merging have been designed and validated.

When cleanup is enabled, `--consensus-manifest-retention-days` is an operational
recovery window, not a protocol constant. It defaults to `90` and must be longer than
the finality window, verification delay and complete retry schedule, maximum cadence
scan, and the supported coprocessor outage and catch-up interval. Eligibility begins
from the durable consensus and dependency state, not merely from S3 object age.

The control is inactive until exact-key deletion, dependency checks, and failure
recovery are defined. `sns-worker` must not receive bucket-wide
Lifecycle-administration permission as part of the initial implementation.

The full-consensus checkpoint is also the database garbage-collection watermark for
dyadic range nodes. Nodes wholly older than the terminal range containing that
checkpoint may be removed when they are not referenced by unresolved drift evidence,
the retained reorg window, or a still-retained parent node. The checkpoint record and
its authenticated evidence remain durable.

Deleting an old S3 manifest can remove the finer roots or association list needed to
localize an already detected historical mismatch. Drift summaries therefore record
immutable quorum and dissenting manifest references, and every referenced manifest
remains retained. Cleanup never converts an uncovered interval into agreement.

## Required properties

- **Deterministic:** identical block identity, ciphertext material, and lineage
  ranges always produce identical block, detailed-range, and dyadic-range digests.
- **Authenticated:** a reader can bind a manifest to a configured coprocessor.
- **Immutable and revisioned:** each published manifest object is immutable; a
  publisher may only supersede by appending a higher `revision` for the same block
  identity, and verifiers always use the latest authenticated tip.
- **Reconcile-tolerant:** verification polling re-resolves tips so single-peer and
  mass repair can converge after higher revisions appear, without trusting status
  tags.
- **Complete:** sealing proves that no more entries for that block can arrive
  locally.
- **Sequential:** every block advances durable state after its parent within the same
  lineage, even when no manifest is published or the block is empty.
- **Historically linked:** dyadic inline history can be refined by authenticated
  older manifests until the exact block is found or retained history ends.
- **Manifest-loss-tolerant:** a later dyadic root can reveal an earlier disagreement
  and a quorum can attribute drift even when an intermediate manifest could not be
  read; unavailable finer manifests reduce localization precision, not detection.
- **Quorum-source-redundant:** any available publisher in a winning commitment group
  may supply finer ranges or one complete handle list when that data recomputes the
  quorum digest; every quorum member need not provide the same historical manifest.
- **Hierarchically degradable:** when no manifest or range commitment reaches
  threshold, comparison continues block by block and then handle by handle without
  lowering the configured threshold or treating unavailable data as absence.
- **Lineage-aware:** each fork has independent block and range roots, and only exact
  range identities on the same lineage are compared, without waiting for finality.
- **Coverage-explicit:** unmatched older history is recorded as `not_covered` and is
  never interpreted as agreement or drift.
- **Durable:** observations and decisions survive worker restarts; late drift must
  not be lost because an in-memory timeout expired.
- **Threshold-tolerant:** an unavailable coprocessor does not unnecessarily stop
  healthy coprocessors once the required agreement threshold is reached.
- **Auditable:** manifests and comparison results are retained long enough to
  reconstruct an incident.

## Drift response

Drift detection and the action taken after detection are separate decisions. Once
the delayed verification phase establishes a quorum result, it can classify a
different operator result as drift regardless of block finality. Cancellation or
revert can follow one of several policies:

- observation and alerting only;
- revert only once the affected block is finalized; or
- cancel or revert as soon as an early-action delay and quorum policy is satisfied.

Drift-revert is cooperative and local. Each coprocessor evaluates the tip manifests
it can read and can cancel or revert only its own state. It does not issue a revert
command to another operator. A malicious operator can refuse to apply its local
revert, but it cannot directly prevent honest operators from repairing themselves or
directly revert their state.

After a successful local repair, the operator publishes a higher manifest `revision`
and rolls forward dependent later checkpoints. Extended verification retries re-poll
peer tips so residual-majority and mass-drift recoveries can converge once enough new
tips exist. Until those tips appear, observing an old dissenting revision is expected
and is handled by the same retry loop rather than by ignoring `drift`-tagged objects.

The local early-action policy remains to be parameterized. A missing manifest can
mean that a peer is merely slower, while one conflicting manifest does not establish
which side is wrong. Quorum and delay identify whether the local commitment belongs
to the winning or losing variant; they are inputs to each operator's own decision,
not authorization for a centralized revert. The policy must define:

- how long to wait for slower peers;
- how many distinct authorized coprocessors establish a winning commitment;
- whether local revert is eligible only when another variant reaches that threshold
  and the local commitment does not;
- exactly which pending and persisted state is cancelled; and
- how cancellation is scoped to one block hash without deleting material still used
  by another lineage.

The current safety model must be preserved as a hard precondition for automatic
local revert. For one pinned verification registry snapshot, the configured
coprocessor set and threshold must make it safe to distinguish a drifted minority
from the honest majority. A plurality or one conflicting peer is never sufficient.

The drift and automatic-revert rules are deliberately separate:

- all visible comparable commitments are equal and reach threshold: classify the
  observation as `consensus` and do not revert locally;
- all visible comparable commitments are equal but below threshold: classify it as
  `unknown_but_equal`, retry while permitted, and do not revert;
- any visible comparable commitment differs: classify the observation as `drift`;
- if the local commitment belongs to the unique threshold group, do not revert it;
- if another commitment reaches threshold and the local commitment does not, its
  `observed_has_quorum = true` findings may become eligible for local revert; and
- if no commitment reaches threshold, persist the drift findings with
  `observed_has_quorum = false`, retry while permitted, and do not revert.

The worker also fails closed and does not auto-revert when the registry snapshot is
unknown or changes during evaluation, the configured threshold is invalid for the
active set, more than one winning group appears possible, evidence mixes block hashes
or format versions, or the winning evidence cannot be authenticated. These cases
emit a safety-configuration or ambiguous-consensus alert for operator review.

An operator that intentionally ignores an eligible local revert remains visible as a
dissenting publisher. Its local summary cannot make minority material authoritative:
an accepted summary must reference an authenticated threshold group, and consumers
verify that manifest evidence rather than trusting the reporter's local material.

The persisted consensus status records the observation. It must not implicitly
trigger destructive action in the first implementation.

## Initial persistence model

Local lineage construction, immutable manifest storage, and verification outcomes
have different identities and lifecycles. They are persisted separately rather than
combining local blocks and per-peer observations in one `block_consensus` row.

### Local block lineage

`block_consensus` is local publisher state. Its logical identity is:

```text
(coprocessor_context_id, host_chain_id, block_hash)
```

`block_number` and `parent_block_hash` are immutable checked fields. The row stores
the local block digest `A`, descriptor count, completion state, publication schedule,
detailed-range bounds and digest when applicable, and the latest local manifest
revision and digest. Competing block hashes at the same height coexist.

Publication state distinguishes at least `not_scheduled`, `pending`, `published`,
and `failed`; a boolean cannot distinguish these cases. Peer observations,
authorization snapshots, and comparison outcomes do not change the identity of a
local block and are not stored as alternate publisher versions of this row.

### Signed manifest archive

Every structurally valid, signature-verified local or downloaded manifest is stored
under the complete immutable manifest identity defined in
[Commitment manifest and publication cadence](#commitment-manifest-and-publication-cadence).
The row stores the manifest digest, exact downloaded or published bytes,
deterministic object key, and `first_seen_at`. Local and peer rows have the same
shape; local ownership is derived by comparing the signed `publisher` with the
configured local publisher.

All observed revision numbers and competing block hashes coexist. Peer ranges and
descriptors remain encoded in the signed body and are decoded when comparison or
per-handle diagnosis needs them. The initial design does not normalize peer ranges
or descriptors into additional tables. Exact repair findings may be persisted
separately for deduplication and audit.

### Verification targets and results

A durable verification target is keyed by the exact local manifest revision being
checked and the pinned registry snapshot. It stores eligibility, attempt timestamps,
retry count and exhaustion, next-attempt time, and the latest outcome. Outcomes
distinguish at least `pending`, `consensus`, `drift`, `unknown_but_equal`,
`different_lineage`, and `invalid_evidence`.

The target is inserted atomically with local publication. If GatewayConfig has not
yet supplied a complete snapshot, it remains in `waiting_registry`; binding later
copies the snapshot identity, threshold, signer set, and bucket URLs into the target
and its per-peer download rows. Those rows record attempt completion and the highest
observed revision. Signed bodies themselves live only in the shared manifest archive,
so a retry, restart, or later target reuses authenticated revisions instead of
downloading them again.

The result records exact comparison coverage, selected tip identity for every
publisher, uncovered intervals, localized drift bounds or blocks, configured
publisher count and threshold, the winning commitment and signer count, whether the
local commitment matches it, and whether local revert is eligible. Keeping the
registry snapshot and selected per-publisher evidence with the result prevents a
later registry refresh or tip revision from rewriting the basis of an earlier
decision.

### Local dyadic ranges and frontier reconstruction

Completed local dyadic `BlockRange` digests are persisted separately from manifest
bodies. Their logical identity contains the version, context, chain, aligned range
bounds, scale, ending block hash, and range digest. Each row stores enough boundary
identity to validate or reuse the merge. A repair appends distinct immutable roots
rather than mutating old roots. The latest full-consensus checkpoint is
persisted per chain and lineage so restarts select the same terminal history boundary
and cleanup cannot move it backward.

To prepare the next local manifest on a lineage, the publisher reconstructs an
in-memory range frontier from the preceding stored local manifest. Its historical
entries select retained `BlockRange` rows, then its detailed block entries are folded
into that frontier using the canonical boundary rule. Every newly materialized parent
range is inserted immutably, so a cold start reuses the stored binary-tree work. With
no preceding published manifest, the frontier starts empty. No separate
per-manifest frontier table is persisted.

## Metrics

The first implementation exposes the following Prometheus metrics. Names are
tentative but their semantics are part of the design:

| Metric | Type | Meaning |
| --- | --- | --- |
| `coprocessor_sns_worker_manifest_publication_total{result}` | Counter | Manifest publication attempts ending in `success` or `failure` |
| `coprocessor_sns_worker_peer_manifest_fetch_total{peer,result}` | Counter | Peer fetches ending in `success`, `not_found`, `timeout`, `http_error`, `invalid`, or `unsupported_version` |
| `coprocessor_sns_worker_missing_peer_manifests{peer}` | Gauge | Whether a peer still lacks a sufficiently recent comparable manifest after cadence-aware scanning and verification retry exhaustion |
| `coprocessor_sns_worker_peer_manifest_cadence_mismatch{peer}` | Gauge | Whether an authenticated peer manifest was observed at a height incompatible with the expected `K` |
| `coprocessor_sns_worker_localized_drift_block_number{peer,bound}` | Gauge | Exact drift block or lower/upper block bound for one observed differing peer result |

All metrics may also carry a bounded `host_chain_id` label. The `peer`, `result`,
and `bound` values come from fixed or configured finite sets. Block hashes, manifest
digests, ciphertext handles, object keys, URLs, and error strings are never metric
labels; exact evidence belongs in structured logs and persisted manifest evidence.

Counters increment only when a new finding or state transition is durably persisted,
not every time a polling loop observes the same condition. Missing-peer and localized
block gauges are derived from current persisted state.

Per-ciphertext drift findings come from comparing one signed local association list
with each distinct observed result after an exact drift block has been localized.
They distinguish at least `missing_handle`, `unexpected_handle`, and
`digest_mismatch`, with more specific `keyset_mismatch` and `format_mismatch`
classification when the descriptors explain the difference. A finding retains the
local and observed descriptor values separately, including `local_keyset_id` and
`observed_keyset_id`, plus `observed_has_quorum`. `local_gateway_key_id` may be
retained as nullable legacy diagnostic state, but no observed Gateway ID is stored
or inferred.

Below-quorum differences are persisted as per-ciphertext drift with
`observed_has_quorum = false`. The Prometheus metric remains unspecified until its
deduplication semantics are defined. In particular, a handle must not be a metric
label; exact handles and conflicting descriptor values belong in persisted evidence
and structured logs.

## Unresolved blockers

### Decryption request boundary

Today, the Gateway decryption contracts check
`CiphertextCommits.isCiphertextMaterialAdded`, and the KMS path receives on-chain
`SnsCiphertextMaterial` containing the expected material and consensus senders. The
future request path must no longer require that on-chain readiness state. It needs to
provide the downstream consumer with the requested handle and context, an
authenticated coprocessor consensus summary, the agreed keyset ID, digests and
format, and the eligible retrieval buckets. The exact summary-acceptance rule and
request protocol remain to be specified; they must not recreate Gateway consensus
under another transport name.

### Block completeness

Manifest sealing uses two separate barriers:

1. **Operation-set closure.** A matching row in `host_chain_blocks_valid` for
   `(chain_id, block_hash)` proves that host-listener finished decoding that block.
   It inserts that row after processing the block's events, in the same transaction
   as the corresponding `computations_branch` rows. Once that transaction commits,
   the publisher can query the complete operation inventory with:

   ```sql
   SELECT output_handle
   FROM computations_branch
   WHERE host_chain_id = $1
     AND producer_block_hash = $2
     AND block_number = $3
     AND is_allowed = TRUE
   ORDER BY output_handle;
   ```

2. **Material completion.** Every allowed operation from that closed inventory must
   be complete and have its branch-scoped ct64 in `ciphertexts_branch`. Its matching
   same-block SnS publication work must also be complete, with branch-scoped ct128
   material and both ciphertext digests and the ct128 format final. Only then is the
   descriptor set immutable and safe to hash.

An empty `pbs_computations` queue proves neither condition and is never a sealing
signal. `pbs_computations_branch` may be used to track ct128 work for the selected
handles, but it does not define which handles belong to the block.

This design requires `sns-worker` to consume branch-aware operation and ciphertext
state with complete branch provenance through execution and upload. It does not add
a legacy fallback. Consensus publication must remain disabled unless that
branch-aware read path and complete provenance are active.

### Keyset identity provenance

Before the manifest format is frozen, the publisher must persist or resolve the
stable `keyset_id` for every generated descriptor. It must not relabel the current
Gateway event ID as a keyset ID merely because current key activation happens to make
the two values equal. The legacy `key_id_gw` maps to the optional
`gateway_key_id` descriptor provenance until the old lookup path is removed; it is
not a quorum input.

### Verification registry snapshots and authentication

`GatewayConfig` is the current authoritative mapping from coprocessor identity to
signing key and `s3BucketUrl`. That URL currently identifies the ct128/SnS bucket and
can also be the base location for commitment manifests.

The manifest does not carry a configuration epoch. It is a publisher's signed
ciphertext statement and may be evaluated under more than one authority
configuration. Each verifier instead pins the exact `GatewayConfig` snapshot used to
authorize signers, resolve S3 locations, and calculate quorum, and persists that
reference with its decision and public evidence.

If authority later moves away from `GatewayConfig`, the replacement registry and
update procedure must be specified. A future shared S3 namespace may likewise need a
separate location field, but neither change mutates an already signed manifest.

The same registry snapshot must be used consistently for the threshold decision and
signature authorization of one verification. A registry update during an in-flight
verification must not mix two snapshots.

### Equivocation and S3 mutability

A signed manifest proves who produced its contents, but not that the publisher did
not show different signed manifests to different peers. Immutable object naming,
bucket versioning or object lock, retention, and evidence exchange must be defined
to make equivocation detectable and auditable.

Multiple manifests from one publisher at the same height are not equivocation when
their block hashes differ: they describe different observed lineages. Multiple
revisions for the same block hash are not equivocation when they form a monotonic
supersession stack with distinct immutable keys; the tip is the highest
authenticated revision. Different authenticated manifest digests for the same
immutable manifest identity are equivocation.

## Initial implementation boundary

The first safe milestone is observation-only S3 drift detection running in parallel
with the existing Gateway consensus:

1. create and populate `block_consensus` without changing execution or readiness;
2. compute lineage-specific block digests, detailed-range digests, and persisted
   dyadic range roots in parent-before-child order;
3. publish authenticated manifests at configured publication heights under
   immutable `/<revision>` keys, with supersession only after repair or correction;
4. discover each peer's latest authenticated revision, validate it, store its exact
   signed body in the same manifest archive as local revisions with its signed
   `publisher`, compare tips, and retain all observed revisions;
5. recursively localize every range difference, preferring a member of the winning
   group when one exists, and persist below-quorum observations as non-actionable;
6. extend verification retries across insufficient-quorum outcomes and drift
   reconcile so later superseding tips can converge without status-tag filtering;
7. best-effort tag the local tip with every completed verification outcome and
   overwrite outcomes when later retries produce a newer result;
8. emit drift, cadence, invalid-evidence, and stale-peer metrics and alerts; and
9. collect evidence for delay and quorum tuning without automatic cancel or revert.

Later milestones can publish authenticated consensus summaries, enable a defined
drift-response policy, make downstream requests require an accepted summary,
retrieve ciphertext only from the winning manifest group, and finally remove the
on-chain ciphertext-material readiness dependency.
