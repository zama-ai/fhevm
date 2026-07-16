# S3-based coprocessor consensus and drift detection

Status: future-design draft

## Goal

Replace the current Gateway-chain mechanism used to establish agreement on new
ciphertext material, together with its `gw-listener` drift detector, with a
pair of S3-based mechanisms:

- KMS establishes authoritative consensus for a specific ciphertext when it is
  requested; and
- `sns-worker` compares periodic cumulative commitment manifests to detect drift
  proactively.

Each coprocessor would:

1. continue to upload its ciphertext material to its own S3 buckets;
2. publish immutable cumulative commitment manifests at configured block intervals;
   and
3. read the manifests published by the other coprocessors to detect missing or
   divergent results.

Block-manifest agreement is not used as the consensus authority. KMS independently
validates the requested ciphertext and only sends agreed material to the decryption
service.

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
In the future design, KMS's successful per-ciphertext S3 consensus becomes the
readiness decision for that request.

## Proposed model

### Separation of responsibilities

| Responsibility | Future authority | Unit |
| --- | --- | --- |
| Ciphertext consensus and readiness | KMS | Metadata consensus plus one verified ciphertext body |
| Proactive drift detection | Each `sns-worker` | One periodic cumulative block checkpoint |
| Ciphertext and attestation publication | Each `sns-worker` | One ciphertext object |

KMS does not trust a block-manifest verdict from an `sns-worker`. It independently
evaluates the signed S3 attestations for every ciphertext involved in a decryption
request. Conversely, block-manifest comparison may alert before a ciphertext is
requested, but it cannot authorize decryption.

### KMS per-ciphertext consensus

For each requested ciphertext handle, KMS:

1. resolves the expected coprocessors, their signing identities, their S3 bucket
   locations, and the required agreement threshold;
2. sends metadata-only `HEAD` requests for the handle and coprocessor context to all
   reachable coprocessor buckets, without downloading the ciphertext bodies;
3. verifies each signature and discards attestations from unknown or duplicate
   signers;
4. groups valid attestations by their complete material tuple: Gateway key ID,
   ciphertext digest, SnS ciphertext digest, and ciphertext format;
5. accepts a tuple only when it has the required number of distinct authorized
   signers;
6. downloads the ciphertext from a bucket belonging to that winning group; and
7. verifies the downloaded bytes against the agreed SnS ciphertext digest before
   sending them to the decryption service.

The consensus phase therefore uses only signed S3 object metadata. KMS does not need
to download one ciphertext copy per coprocessor. After metadata consensus is reached,
it downloads one body from the winning group. If that body is missing or fails digest
verification, KMS tries another bucket from the same winning group.

Metadata consensus proves agreement on the expected ciphertext, but it does not by
itself prove that usable bytes remain available. A ciphertext is ready only after both
the attestation threshold and one successful, digest-verified body retrieval.

If no tuple reaches the threshold, the ciphertext is not ready. The KMS request must
fail or remain retryable; it must not fall back to a local manifest, the largest
sub-threshold group, or an unverified ciphertext.

This flow is partially present today as a shadow check: KMS already fetches S3
attestations and evaluates their threshold, but it still compares the winning tuple
with the on-chain ciphertext material and uses the on-chain consensus sender list for
retrieval. The future path must instead use the S3 consensus result itself as the
material and retrieval authority.

### Commitment manifest and publication cadence

A commitment manifest is the sealed statement of one coprocessor at a specific
host-chain block. In each coprocessor bucket, it is stored under the deterministic
key:

```text
<s3BucketUrl>/manifests/v1/<coprocessor_context_id>/<host_chain_id>/<block_height>/<block_hash>
```

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
`--consensus-verification-retry-count`. The reserved control for future manifest
cleanup is `--consensus-manifest-retention-days`, with a default of `90` days.

A worker computes commitments for every processed block in strict
parent-before-child order within each observed lineage, but writes manifests only at
those deterministic publication heights. The rule applies independently to every
lineage, so different block hashes at the same scheduled height may both have
manifests.

The comparison protocol tolerates a coprocessor running with a different `K` by
mistake: it scans backward linearly over block ancestors of the same lineage,
compares the newest peer manifest it finds at that manifest's exact block height,
and reports the cadence mismatch. Cadence is derived only from absolute block
height and `K`, never from a worker's startup time or its most recent successful
publication.

A manifest should initially contain:

- a manifest format version;
- the host chain ID;
- the coprocessor context ID;
- the activation block number and activation parent block hash;
- the publication block number and block hash;
- the parent block hash;
- the coprocessor identity;
- the active coprocessor-set or configuration epoch;
- the current block ciphertext-content digest `A`;
- the current cumulative ciphertext-content digest `B`;
- an ordered sequence of every block covered since the preceding manifest,
  including the publication block; each block entry contains its identity, its
  `A`, and a canonical ordered association list binding every ciphertext handle
  first attributed to that block to its digest descriptor;
- a bounded, ordered history of recent cumulative `B` values, each associated with
  its block number, block hash, and parent block hash;
- a reference to the preceding manifest containing its ending block number, ending
  block hash, and manifest digest; and
- the publisher's signature over the complete manifest envelope.

### Manifest signing

`sns-worker` signs the manifest using the same coprocessor signing identity used for
the signed attestation metadata on ciphertext objects. The signature is produced as
part of publication and is verified against the coprocessor's registered
`GatewayConfig.signerAddress`.

The canonical commitment and signing implementation is shared with ciphertext
attestation under `shared/ciphertext-attestation`, initially in a new `manifest`
module. This shared module is the single source of truth for canonical encoding,
Keccak-256 hashing, signing, and verification. `sns-worker`, peer verifiers, KMS, and
external evidence tools must use it rather than independently reconstructing bytes.

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
| Initial cumulative digest | `FHEVMBI0` |
| Cumulative digest update `B` | `FHEVMCHN` |
| Signed commitment manifest | `FHEVMMNF` |

The manifest prehash is Keccak-256 of its complete canonical payload prefixed by the
eight-byte domain tag `FHEVMMNF`. It is signed as a raw prehash with the same signing
method as `CiphertextAttestationPayload::canonical_digest`. The payload covers the
format version, configuration epoch, publisher identity, coprocessor context ID,
chain ID, block number, block hash, parent block hash, activation block number,
activation parent block hash, `A`, `B`, every covered block and
handle-to-descriptor association, history entries, and the preceding-manifest
reference. Moving a manifest to another path or context, changing its lineage, or
editing its ciphertext associations or history must invalidate verification.

S3 write access alone does not authorize a manifest: an object without a valid
registered-coprocessor signature is stored as invalid evidence and never participates
in comparison or local revert decisions.

The history selection can be approximate: for example, approximately the last `X`
hours or `Y` blocks, and it may omit empty blocks. Every history value that is
included must nevertheless be exact. Approximation applies to window coverage, not
to digest correctness.

This history remains small because each item is a block identity and fixed-size
digest rather than ciphertext material. The covered-block associations are also
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
Consensus activation starts only once complete branch-aware operation and
ciphertext state is available. Blocks before that activation point are outside the
commitment lineage and are never evaluated retrospectively.

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
gateway_key_id:         uint256
ct64_digest:            bytes32
ct128_digest:           bytes32
ct128_format:           uint8       # CiphertextFormat
```

Both ciphertext digests are Keccak-256 of the exact ciphertext bytes, matching the
existing `sns-worker` ciphertext-digest function. The ct128 format uses the existing
`CiphertextFormat` discriminant defined by ciphertext attestation. Gateway key ID and
ct128 format participate because KMS consensus also groups attestations by those
fields; omitting either could hide material that is unusable or interpreted
differently.

Publisher identity, transaction ID, timestamps, S3 keys, and S3 transport checksums
are deliberately excluded from `A`. They are not ciphertext computation results and
may legitimately differ between operators. Coprocessor context, host chain, and
block identity are encoded once in the block-digest header rather than repeated in
every descriptor.

Descriptors are sorted by ascending raw 32-byte handle. For manifest format version
1, `A` is:

```text
A = keccak256(
      bytes8("FHEVMBLK")
      || uint8(format_version)
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
intervening `K - 1` blocks detectable through `B` but not diagnosable from the
manifest chain.

An empty block has `descriptor_count = 0` and therefore has a well-defined,
block-specific `A`: Keccak-256 of the header above with no descriptors, rather than a
special zero value. A database insertion or completion index never participates in
ordering.

### Cumulative digest `B`

`B` chains the non-empty block digests `A` in parent-before-child order for one
host-chain lineage. It uses Keccak-256 like ciphertext digests and `A`. The initial
accumulator for each `(format version, coprocessor context, host chain, activation
lineage)` is:

```text
B_initial = keccak256(
              bytes8("FHEVMBI0")
              || uint8(format_version)
              || uint256(coprocessor_context_id)
              || uint256(host_chain_id)
              || uint256(activation_block_number)
              || bytes32(activation_parent_block_hash)
            )
```

`--consensus-activation-block` configures the activation height independently for
each host chain and must have the same value at every coprocessor. For an activation
block, `B(parent)` in the update rule below means `B_initial`. Including the
activation parent hash keeps initialization lineage-specific if the activation
height itself is reorganized.

For each non-empty block, the exact update is:

```text
B(block) = keccak256(
             bytes8("FHEVMCHN")
             || uint8(format_version)
             || uint256(coprocessor_context_id)
             || uint256(host_chain_id)
             || bytes32(B(parent))
             || uint256(block_number)
             || bytes32(parent_block_hash)
             || bytes32(block_hash)
             || bytes32(A(block))
           )
```

If a block has no ciphertext descriptors, it does not advance `B`. The worker still
records `B(block) = B(parent)` and processes the block to preserve lineage progress.
It still publishes a manifest when an empty block is a configured publication
height. This keeps publication liveness distinguishable from missing output.

The configuration epoch does not enter `A`, `B_initial`, or the `B` update. A
membership or threshold rotation changes who may sign and which signatures may form
a quorum; it does not change the ciphertext material or reset cumulative history.
The first manifest under a new epoch therefore continues the same lineage-specific
`B` and references the preceding manifest even when that predecessor belongs to the
previous epoch. Votes from different epochs are never combined. A verifier may
validate digest continuity across the boundary, but it evaluates quorum using only
the registry snapshot named by the current manifest.

Changing the manifest format or any commitment encoding is different from a
membership rotation. A future format version must define a new domain-separated
initialization or an explicit bridge from the preceding version; version 1 readers
never infer such a bridge.

Because `B` is cumulative, a missed historical drift remains visible: later `B`
values continue to differ even if the coprocessors agree on later `A` values. This is
intentional. Re-convergence requires an explicit reconciliation or reset rule rather
than silently forgetting the earlier divergence.

The cumulative chain also makes detection tolerant to missing intermediate manifest
objects or failed reads. Peers do not need every scheduled manifest to remain
available: at any later common checkpoint, `B` still commits to all preceding
non-empty block digests. A later retry or a successfully compared later manifest can
therefore detect an earlier divergence that could not be classified when it first
occurred, even after the earlier verification target exhausted its retry count.

This eventual-detection property requires a later comparable manifest on the same
lineage. If that lineage stops permanently, for example after becoming orphaned,
finite retry exhaustion leaves the verification outcome unknown; `B` cannot create
another comparison opportunity by itself.

This property preserves detection, not necessarily exact diagnosis. If both the
intermediate manifest and the relevant history checkpoints are unavailable, a later
`B` mismatch proves that prior content diverged but may only bracket its origin.

### Lineage and reorgs

Manifest publication does not wait for host-chain finality. As soon as a block and
its ciphertext set are locally complete, it can participate in the lineage and be
published at a configured publication height.

`B` is maintained independently per block lineage rather than as one global cursor.
A child can be processed only after the cumulative state of its parent is known. On
a reorg, the worker resumes from the common ancestor, computes commitments for the
new lineage, and publishes new immutable manifests under the new block hashes. The
old-lineage objects remain available as evidence and are not overwritten or revoked.

A single coprocessor can therefore publish multiple manifests with the same block
number and different block hashes. They coexist under different S3 keys. The manifest
identity is `(format version, chain ID, block number, block hash, configuration
epoch, publisher)`, not the block number alone.

### Manifest format and layout versioning

The manifest carries `format_version: 1` in its signed envelope. Version `1` uses
the stable
`/manifests/v1/<coprocessor_context_id>/<host_chain_id>/<block_height>/<block_hash>`
layout. The path version and signed body version must match; a mismatch makes the
object invalid.

The version covers the S3 discovery layout, required fields, canonical encoding,
hash algorithms and domain separators, and signature payload. It is included in the
signed payload and the hash domains for `A`, `B`, and the manifest digest.

A reader must never parse an unknown version as the current format; it records the
object as unsupported until that version is implemented. A future version uses a
new path such as `/manifests/v2/...` and carries the matching signed body version.
Readers may support multiple layouts during a rolling upgrade, but the activation
block and the rule for carrying or resetting cumulative `B` across the version
transition must be explicitly defined.

Manifests with different block hashes at the same height belong to different lineages
and are not comparable for ciphertext drift. They establish neither agreement nor
drift with each other.

Commitments are compared only for the same chain ID, block number, and block hash.
That comparison does not wait for finality. Different `A` or `B` values establish a
disagreement, but a specific operator is classified as drifted only when another
identical result reaches quorum. Once attributed, drift remains a correctness
finding even if the block is later orphaned. Whether that lineage eventually wins
the reorg is a separate annotation and does not retroactively erase the finding.

A divergence on an orphaned lineage does not contaminate a later canonical lineage:
each branch derives `B` from its own parent chain. A ciphertext-content divergence on
the same lineage remains cumulative until an explicit reconciliation rule applies.

### Finality and transient forks

Different RPC providers may temporarily expose different branches during the
host-chain finality window. This is expected lineage disagreement, not ciphertext
computation drift. Quorum grouping never mixes block hashes, even when their block
heights are equal.

| Observed manifests | Verification outcome | Local revert |
| --- | --- | --- |
| Different block hashes at the same height | `different_lineage` | Never |
| Same block hash, fewer than quorum, all results equal | `unknown_but_equal` | Never |
| Same block hash, fewer than quorum, results differ | `unknown_but_unequal` | Never |
| Same block hash, one identical result reaches quorum | Members are `consensus`; different results are `drift` | Only a local `drift` result may become eligible |

A branch observed by too few coprocessors therefore cannot cause drift attribution,
let alone drift-revert. If that branch later disappears, its unknown observations
remain fork evidence only. If enough coprocessors independently agree on that branch,
it may reach consensus for that lineage; a later reorg still does not turn branch
agreement into computation drift.

Restricting publication and verification to finalized blocks would remove most fork
bookkeeping, unknown outcomes on short-lived branches, and orphan-lineage retention.
It would also delay detection and deliberately stop detecting computation drift on
branches that are later orphaned. The current pre-finality design is safe against
fork-induced revert because of exact-hash comparison and quorum attribution, but it
is more operationally complex. A post-finality-only design would therefore be a
scope change, not merely an implementation optimization.

### Rolling history and predecessor lookup

Each manifest repeats recent `(block number, block hash, parent block hash, B)`
checkpoints. If current `B` values differ on the same lineage, peers compare their
overlapping history to find the last matching checkpoint and the first differing
checkpoint. Depending on the history sampling, this either identifies the first
divergent block or quickly brackets the interval in which disagreement began. Once
a result reaches quorum, the same localization attributes that disagreement as
drift for operators outside the quorum group.

If the divergence began before the embedded history window, the worker follows the
preceding-manifest reference and repeats the comparison. This creates a backward
chain that can be traversed until the first divergent block or a common manifest is
found.

The ending block hash may be used in the deterministic S3 lookup path, but the block
hash alone is not a secure manifest reference: two manifests can describe the same
host-chain block while committing to different ciphertext results. The predecessor
reference therefore includes the previous manifest digest, and the fetched object
must match it.

### Publication

A manifest is published only after the coprocessor has a reliable signal that the
ciphertext set for the publication block and its preceding unpublished ancestors is
complete. Publication should then:

1. resume from the durably processed lineage state and last published manifest;
2. process a block only after its parent lineage state is available;
3. compute `A` and advance `B` for each non-empty block;
4. at each configured publication height, attach the ordered block identities,
   values of `A`, and complete handle-to-descriptor association lists accumulated
   since the preceding manifest;
5. attach recent `B` history and the preceding-manifest reference;
6. compute the manifest digest;
7. sign the complete manifest with the coprocessor identity;
8. write it under an immutable, versioned S3 key;
9. optionally update a discovery pointer only after the immutable object exists;
   and
10. durably schedule peer verification for the end of
    `--consensus-verification-delay`.

After an outage, the worker reconstructs missed scheduled checkpoints in lineage
order rather than skipping directly to the current chain head. Retrying the same
publication block hash must be idempotent.

A partially written or unsealed manifest must never participate in comparison.

Publication and verification are separate asynchronous phases. Successful local
publication does not immediately fetch or compare peer manifests. The configurable
`--consensus-verification-delay` gives slower coprocessors time to process the same
lineage and publish their manifests. The delay starts when local manifest publication
succeeds. The verification target and its eligibility time must be durable so a
worker restart does not skip the comparison.

Before the delay expires, absent peer manifests are expected lag: the observation
remains `waiting_for_delay` and does not produce a missing-manifest failure or drift
finding. Once eligible, the worker runs peer discovery and comparison. If the
available authenticated manifests do not yet establish the required quorum, the
target remains pending and the worker schedules another attempt after
`--consensus-verification-retry-delay`.

`--consensus-verification-retry-count` is the number of additional attempts allowed
after the initial verification attempt. The number of retries already performed and
the next eligible attempt time are persisted in the database and updated atomically,
so restarts neither reset the budget nor lose a scheduled retry. Peer absence may
also become a missing or stale-peer finding, but never ciphertext drift by itself.

### Multi-worker verification locking

A coprocessor may run several `sns-worker` replicas against the same database. A
replica claims one due verification target by starting a transaction and selecting
its `block_consensus` row with `FOR UPDATE SKIP LOCKED`. A row already locked by
another replica is skipped, preventing two replicas from verifying the same target
concurrently.

The claiming replica keeps the row lock while it fetches and validates peer
manifests and computes the result. In the same transaction, it then updates the
comparison outcome, observed evidence, retry count, next-attempt time, retry
exhaustion, and latest verification outcome before committing. If the worker fails,
PostgreSQL rolls back the update and releases the lock, leaving the target eligible
for another replica.

Because this deliberately holds a database transaction across S3 reads, every S3
operation must have a bounded timeout and one transaction should process only one
verification target. If related peer-observation rows must also be locked, all
replicas acquire them in the same deterministic order to avoid deadlocks.

The external S3 tag update is attempted once after this transaction commits. It is
best-effort because an S3 call cannot be made atomic with the PostgreSQL transaction.
Failure to update the tag does not change the persisted verification outcome and is
not retried.

### Verification outcomes and quorum

For one block identity, manifest version, and configuration epoch, the verifier
groups valid results by the identical commitment `(A, B)`. Only distinct authorized
publishers count toward the configured quorum.

The verification outcome is:

- `consensus`: one result reaches quorum and the operator's result belongs to that
  group;
- `drift`: one result reaches quorum and the operator published a different result;
- `unknown_but_equal` ("unknown - but equal"): too few valid results reach quorum,
  but every available result is identical; or
- `unknown_but_unequal` ("unknown - but unequal"): no result reaches quorum and the
  available valid results contain at least two different commitments.

Both unknown outcomes are verification errors suitable for retry. While retries
remain, the worker schedules another attempt after
`--consensus-verification-retry-delay`. If the retry budget is exhausted, it retains
the exact unknown subtype and marks retry exhaustion rather than guessing consensus
or drift. A pairwise difference therefore supports `unknown_but_unequal`, not an
attributed drift finding.

### Manifest consensus-status tag

After each completed verification attempt, `sns-worker` adds or replaces this S3
object tag on its own immutable manifest object:

```text
consensus-status = consensus
                 | drift
                 | unknown_but_equal
                 | unknown_but_unequal
```

The tag is absent only before the first verification attempt completes. An unknown
outcome is written immediately and may be overwritten by a later retry. The worker
sets `consensus` when its manifest belongs to the quorum group and `drift` when
another identical result reaches quorum and its manifest differs. After retry
exhaustion, the last unknown outcome remains visible.

Object tagging does not rewrite the manifest body or its S3 user metadata. The tag
is externally observable operational evidence for manual inspection, integration
tests, and other monitoring that does not have database access. Such observers read
it with S3 object-tagging APIs and may use it to assert what conclusion the operator
published.

The tag is nevertheless a self-reported, unsigned conclusion. It must never be
counted as a vote or trusted by itself for KMS consensus, drift attribution, or
automatic revert. An observer requiring authoritative proof recomputes the outcome
from the signed manifests. The database stores the authoritative local outcome; a
missing or stale tag is inconclusive and does not indicate verification failure. The
worker requires `s3:PutObjectTagging` only for manifests in its own writable bucket
or isolated prefix. Updating `consensus-status` must preserve unrelated object tags.

### Peer comparison

Each `sns-worker` reads the manifest prefix of every expected coprocessor using
read-only credentials, verifies the manifest identity and signature, and persists
what it observed locally. This is a monitoring path and is not consulted by KMS for
per-ciphertext consensus.

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
one decision cannot mix two membership epochs.

### Peer manifest discovery and cadence mismatch

When delayed verification runs for a locally published manifest, a peer may still
have no manifest at the identical height because it is late or accidentally
configured with another cadence. The reader:

1. first looks for the peer manifest at the local publication block;
2. if it is absent, walks backward linearly over that block's ancestors and queries
   the peer path for each exact `(block number, block hash)`;
3. selects the newest valid peer manifest found within the scan and freshness
   limits; and
4. compares the peer's `A` and `B` at that manifest's block height with the local
   durable `A` and `B` for the same block identity and configuration epoch.

This comparison does not require the local coprocessor to have published a manifest
at the peer's height. Every block advances local lineage state, so the verifier can
compare against its own computed commitment at any ancestor height. Equality of `B`
is the result being tested; it is not part of the definition of a common checkpoint.
If the values differ, disagreement is detected at the peer manifest's height and the
history and predecessor-link procedure localizes the earlier origin when necessary.
The verifier attributes `drift` to a specific operator only after an identical
result reaches quorum.

Discovering a different block hash at the same height requires either listing that
height's S3 prefix or reading a signed per-peer discovery pointer. It is useful for
explaining why the matching path is absent, but the different-hash manifest does not
participate in drift comparison.

If the inline histories cannot localize a `B` mismatch, the reader follows the
authenticated predecessor references until it finds the first differing block,
brackets it, or reaches the retention boundary.

The backward scan must be bounded and resume from durable per-peer discovery state;
it must not issue an unbounded number of S3 requests on every poll. A peer manifest
older than the configured freshness limit is reported as stale even if it is valid.

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
- `unknown_but_unequal` while retries remain or exhausted;
- invalid or unauthenticated manifest.

Absence at one exact publication height is not yet an availability failure because a
peer may use another cadence. Failure to find a sufficiently recent valid manifest
within the bounded scan is an availability or lag failure, not proof of drift. A
difference requires comparable authenticated commitments for the same block identity
and configuration epoch. Attributed drift additionally requires a quorum of an
identical competing commitment.

Comparison has two useful cases:

- different `A` values mean the coprocessors disagree on ciphertext content
  attributed to the peer manifest's terminal block; and
- equal `A` values with different `B` values mean the peer manifest's terminal block
  agrees but an earlier non-empty block diverged.

When `B` differs, workers use the overlapping history to locate or bracket the first
divergent block. If the origin is older than the embedded history, they traverse the
predecessor links. Once the exact block is localized and the quorum and dissenting
manifests covering it are available, the detector compares their signed
handle-to-descriptor association lists. For each drifted operator, it records whether
each finding is a missing handle, an unexpected handle, or a digest mismatch.
Fetching ciphertext bodies is not required for this comparison; a later
investigation may compare S3 object metadata or bodies against the manifest evidence.

Manifests at the same height but under different block hashes are recorded as
different lineages, not drift. Their `A` values are not two claims about the same
block content because the blocks themselves differ.

### Public drift-evidence files

When a quorum result differs from one or more operator results, every coprocessor
that detects the drift publishes a signed summary in its own registered bucket.
Publication is not limited to the drifted operator: a detector whose own manifest is
in the quorum group publishes the same incident from its point of view. A cooperative
drifted operator also publishes after classifying itself as drifted, but detection
does not depend on it doing so.

The normal evidence flow is:

1. use the differing cumulative `B` values and manifest histories to find the first
   differing block on the same lineage;
2. obtain the quorum and drifted manifests covering that block;
3. compare their canonical handle-to-descriptor association lists;
4. classify each out-of-consensus handle as `missing_handle`,
   `unexpected_handle`, or `digest_mismatch`; and
5. publish the signed evidence file.

Evidence summaries use this stable, listable path in each detector's registered
bucket:

```text
<s3BucketUrl>/drift/<origin_block_height>/<origin_block_hash>/summary
```

The detector's registered `s3BucketUrl` identifies the reporting coprocessor. In a
shared physical bucket, each registered base URL must resolve to an operator-isolated
prefix so detectors cannot overwrite one another's summaries. Because summaries are
only for external observation, their version is carried only as signed
`format_version: 1` in the body; it is not part of the S3 path.

An exact evidence file contains at least:

- format version, detector identity, detector signature, and detection timestamp;
- coprocessor context ID, host chain ID, configuration epoch, registry reference,
  and configured quorum;
- the comparison checkpoint and exact origin block identities;
- the winning `(A, B)` values and the distinct authorized publishers establishing
  quorum;
- each drifted publisher and its differing `(A, B)` values;
- the signed quorum and drifted manifest envelopes, their immutable S3 keys and
  digests, and the history or predecessor chain used for localization;
- the ordered per-handle findings, including the quorum descriptor and each
  differing or absent operator descriptor.

The evidence contains digest metadata and signed manifests, not ciphertext bodies.
The detector signs the complete canonical evidence envelope with its registered
coprocessor signing identity. Its signature proves which detector published the
report; external observers still verify the embedded manifest signatures, distinct
quorum membership, lineage, and digest computations.

If missing retained history prevents exact localization, the cumulative mismatch is
still persisted locally, but the detector cannot truthfully use the origin-block
summary path or claim per-handle attribution. Localization remains retryable until
the exact origin and association-list differences are available.

Evidence publication is durable and retried, unlike the best-effort
`consensus-status` tag. A database evidence row keyed by the detector, context, host
chain, and origin block tracks the summary key, content digest, revision,
publication state, and attempts. Multiple `sns-worker` replicas claim evidence rows
with `FOR UPDATE SKIP LOCKED`.

The stable path contains the detector's latest complete summary for that origin
block. If later verification discovers more drifted results or more evidence, the
detector replaces it with a fully signed revision containing the previous summary
digest. S3 bucket versioning should retain older revisions; the signed digest chain
makes replacement history independently auditable.

To make drift publicly discoverable, every registered bucket must allow
`s3:ListBucket` for the `/drift/` prefix and `s3:GetObject` for its summary
objects. Public ciphertext reads alone do not imply permission to list this prefix.
An external observer discovers all reports by resolving the registered coprocessor
buckets and listing `/drift/` in each one.

### Retention and deferred cleanup

Once a manifest has reached `consensus`, its main online-safety purpose is complete
after a recovery window. Manifests that did not reach consensus and public drift
summaries remain useful as incident and audit evidence.

The intended future retention period for successfully verified manifests is
configured by `--consensus-manifest-retention-days`, which defaults to `90`. The
policy is:

- expire a manifest after the configured number of days only when its
  `consensus-status` S3 tag is `consensus`;
- retain manifests tagged `drift`, `unknown_but_equal`, or
  `unknown_but_unequal`;
- retain untagged manifests, including when the best-effort tag update failed; and
- retain every `/drift/` summary and, when S3 versioning is enabled, all its
  revisions.

The cleanup mechanism is intentionally deferred. The ct128/SnS bucket contains
other information, so an incorrectly scoped bucket Lifecycle rule could delete
unrelated objects or interfere with existing bucket rules. No automatic manifest
deletion is enabled in the first implementation; all objects are retained until
bucket ownership, exact prefix scoping, versioning behavior, and configuration
merging have been designed and validated.

When cleanup is enabled, the configured value is an operational recovery window,
not a protocol constant. It must be longer than the finality window, verification
delay and complete retry schedule, maximum cadence scan, and the supported
coprocessor outage and catch-up interval. S3 Lifecycle age is based on object
creation rather than verification completion, so the configured window must include
the worst-case time needed to reach consensus.

`--consensus-manifest-retention-days` remains the intended future retention control,
with a default of 90 days. It is inactive until the cleanup mechanism is defined.
`sns-worker` must not receive bucket-wide Lifecycle-administration permission as part
of the initial implementation.

Deleting an old consensus manifest does not erase cumulative detection: a later `B`
mismatch can still reveal earlier disagreement. It does bound exact localization and
offline catch-up, because the deleted manifest's association list and predecessor
link are no longer available. Drift summaries embed their signed quorum and
dissenting evidence before those consensus manifests become eligible for cleanup.

## Required properties

- **Deterministic:** identical block identity and ciphertext material always produce
  identical `A` and `B` commitments.
- **Authenticated:** a reader can bind a manifest to a configured coprocessor.
- **Immutable and replay-safe:** a published manifest cannot be silently replaced
  by an older or different statement for the same block and epoch.
- **Complete:** sealing proves that no more entries for that block can arrive
  locally.
- **Sequential:** every block advances durable state after its parent within the same
  lineage, even when no manifest is published or the block is empty.
- **Historically linked:** bounded inline history can be extended by authenticated
  predecessor traversal.
- **Manifest-loss-tolerant:** a later common cumulative checkpoint can reveal an
  earlier disagreement, and a quorum can attribute drift, even when an intermediate
  manifest could not be read or was lost.
- **Lineage-aware:** each fork has an independent cumulative chain, and only
  commitments for the same block hash are compared, without waiting for that block
  to finalize.
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

Drift-revert is cooperative and local. Each coprocessor evaluates the manifests it
can read and can cancel or revert only its own state. It does not issue a revert
command to another operator. A malicious operator can refuse to apply its local
revert, but it cannot directly prevent honest operators from repairing themselves or
directly revert their state.

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
local revert. For one pinned configuration epoch, the configured coprocessor set and
threshold must make it safe to distinguish a drifted minority from the honest
majority. A plurality or one conflicting peer is never sufficient.

The consensus and automatic-revert rule is:

- local commitment belongs to a threshold group: classify it as `consensus` and do
  not revert locally;
- another commitment reaches threshold and the local commitment does not: local
  result is `drift` and is eligible for revert; and
- no commitment reaches threshold: classify the available evidence as
  `unknown_but_equal` or `unknown_but_unequal`, retry while permitted, and do not
  revert.

The worker also fails closed and does not auto-revert when the registry epoch is
unknown or changes during evaluation, the configured threshold is invalid for the
active set, more than one winning group appears possible, evidence mixes block hashes
or format versions, or the winning evidence cannot be authenticated. These cases
emit a safety-configuration or ambiguous-consensus alert for operator review.

An operator that intentionally ignores an eligible local revert remains visible as a
dissenting publisher. KMS's independent per-ciphertext S3 consensus prevents that
operator's minority material from becoming authoritative for decryption.

The persisted consensus status records the observation. It must not implicitly
trigger destructive action in the first implementation.

## Initial `block_consensus` persistence

Every coprocessor initially publishes manifests and maintains a local
`block_consensus` table. The logical row identity is:

```text
(host_chain_id, block_number, block_hash, configuration_epoch, coprocessor_id)
```

This preserves one observation per publisher and allows multiple block hashes at the
same height. The table records at least:

- `parent_block_hash`;
- manifest format version;
- block content digest `A`, nullable when a peer history exposes only `B`;
- cumulative digest `B`;
- expected cadence `K` and whether the observed manifest height is compatible with
  it;
- verification eligibility, first-attempt, latest-attempt, and next-attempt
  timestamps;
- the persisted verification retry count and whether the configured retry budget is
  exhausted;
- publication state, distinguishing at least `not_scheduled`, `pending`,
  `published`, `failed`, `observed`, and `invalid`;
- the block number, block hash, object key, and digest of the manifest covering the
  observation;
- comparison status, distinguishing at least `pending`, `consensus`, `drift`,
  `unknown_but_equal`, `unknown_but_unequal`, `different_lineage`, and
  `invalid_evidence`;
- the pinned registry epoch, configured coprocessor count and threshold, winning
  commitment and signer count, whether the local commitment matches the winner, and
  whether local revert is eligible; and
- publication, observation, comparison, and update timestamps.

A publication-state enum is preferable to a single `manifest_published` boolean:
`false` cannot distinguish a non-publication block, a pending publication, a failed
upload, a missing peer object, or an invalid manifest.

The status is relative to a specific block hash and epoch. An aggregate status alone
is insufficient for future quorum-based revert because it loses which authorized
coprocessor supplied each commitment; the per-publisher evidence must be retained.

`block_consensus` remains the block-level index and status table. Exact
per-ciphertext diagnosis additionally requires retaining the verified manifest body
or normalizing its association entries and findings into child rows keyed by the
block observation and handle. The implementation choice is still open, but the
evidence must survive polling retries so findings can be deduplicated and audited.

## Metrics

The first implementation exposes the following Prometheus metrics. Names are
tentative but their semantics are part of the design:

| Metric | Type | Meaning |
| --- | --- | --- |
| `coprocessor_sns_worker_manifest_publication_total{result}` | Counter | Manifest publication attempts ending in `success` or `failure` |
| `coprocessor_sns_worker_peer_manifest_fetch_total{peer,result}` | Counter | Peer fetches ending in `success`, `not_found`, `timeout`, `http_error`, `invalid`, or `unsupported_version` |
| `coprocessor_sns_worker_missing_peer_manifests{peer}` | Gauge | Whether a peer still lacks a sufficiently recent comparable manifest after cadence-aware scanning and verification retry exhaustion |
| `coprocessor_sns_worker_peer_manifest_cadence_mismatch{peer}` | Gauge | Whether an authenticated peer manifest was observed at a height incompatible with the expected `K` |
| `coprocessor_sns_worker_localized_drift_block_number{peer,bound}` | Gauge | Exact drift block or lower/upper block bound for a peer classified outside the quorum result |

All metrics may also carry a bounded `host_chain_id` label. The `peer`, `result`,
and `bound` values come from fixed or configured finite sets. Block hashes, manifest
digests, ciphertext handles, object keys, URLs, and error strings are never metric
labels; exact evidence belongs in structured logs and persisted manifest evidence.

Counters increment only when a new finding or state transition is durably persisted,
not every time a polling loop observes the same condition. Missing-peer and localized
block gauges are derived from current persisted state.

Per-ciphertext drift findings come from comparing a drifted operator's signed
association list with the quorum result after an exact drift block has been
localized. They distinguish at least `missing_handle`, `unexpected_handle`, and
`digest_mismatch`. Differences observed under `unknown_but_unequal` are retained as
disagreement evidence but are not attributed as per-ciphertext drift. The Prometheus
metric remains unspecified until its deduplication semantics are defined. In
particular, a handle must not be a metric label; exact handles and the conflicting
descriptor values belong in persisted evidence and structured logs.

## Unresolved blockers

### Decryption request boundary

Today, the Gateway decryption contracts check
`CiphertextCommits.isCiphertextMaterialAdded`, and the KMS path receives on-chain
`SnsCiphertextMaterial` containing the expected material and consensus senders. The
future request path must no longer require that on-chain readiness state. It needs to
provide KMS with the requested handle and context while allowing the KMS S3 verdict
to supply the agreed key ID, digests, format, and eligible retrieval buckets.

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

### Membership epochs and authentication

`GatewayConfig` is the current authoritative mapping from coprocessor identity to
signing key and `s3BucketUrl`. That URL currently identifies the ct128/SnS bucket and
can also be the base location for commitment manifests.

The remaining design question is how configuration rotation maps to manifest epochs,
including whether a future shared S3 namespace is represented by per-coprocessor URLs
or by another configuration field. If authority later moves away from
`GatewayConfig`, the replacement registry and update procedure must be specified.

The same registry snapshot must be used consistently for the threshold decision and
signature authorization of one request. A registry update during an in-flight
request must not mix two coprocessor epochs.

### Equivocation and S3 mutability

A signed manifest proves who produced its contents, but not that the publisher did
not show different signed manifests to different peers. Immutable object naming,
bucket versioning or object lock, retention, and evidence exchange must be defined
to make equivocation detectable and auditable.

Multiple manifests from one publisher at the same height are not equivocation when
their block hashes differ: they describe different observed lineages. Conflicting
manifests for the same format version, chain ID, block number, block hash, epoch, and
publisher are equivocation.

## Initial implementation boundary

The first safe milestone is observation-only S3 drift detection running in parallel
with the existing Gateway consensus:

1. create and populate `block_consensus` without changing execution or readiness;
2. compute lineage-specific `A` and `B` values in parent-before-child order;
3. publish authenticated manifests at configured publication heights;
4. discover, validate, compare, and persist peer manifests and histories;
5. best-effort tag the local manifest with every completed verification outcome and
   overwrite unknown outcomes when later retries produce a newer result;
6. emit drift, cadence, invalid-evidence, and stale-peer metrics and alerts; and
7. collect evidence for delay and quorum tuning without automatic cancel or revert.

Later milestones can enable a defined drift-response policy, make KMS reject requests
that do not reach S3 metadata consensus, retrieve ciphertext only from the winning S3
group, and finally remove the on-chain ciphertext-material readiness dependency.
