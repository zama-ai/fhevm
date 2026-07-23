# S3-based coprocessor consensus and drift detection

Status: partially implemented, observation only

This document describes two things:

1. the manifest publisher and verifier implemented in this branch; and
2. the remaining protocol needed before manifests can replace Gateway consensus or
   trigger replay.

The distinction is deliberate. Current code publishes, downloads, authenticates,
compares, and archives manifests. It does not change execution, Gateway voting,
decryption readiness, ciphertext retrieval, or local state after drift.

## Purpose

Each coprocessor periodically publishes a signed commitment to the ciphertext
material it produced. Every coprocessor downloads the other publishers' manifests
and compares authenticated commitments. This detects divergence earlier than a
per-request consumer and leaves enough signed evidence to identify the affected
blocks and handles.

Long term, coprocessors will derive consensus internally from these manifests and
publish signed consensus summaries. A downstream consumer will use an accepted
summary to choose a ciphertext source, then verify the downloaded body against the
agreed digest.

## Current implementation at a glance

The runtime is disabled by default. It is enabled by:

- `--consensus-publish-manifest`; and
- `--consensus-verify-others-party-manifests`.

Both flags default to `false`.

| Area | Implemented now | Not implemented yet |
| --- | --- | --- |
| Manifest format | Version 1 canonical encoding, Keccak-256 commitments, signatures, revision links, predecessor links, and deterministic object keys | Later format versions |
| Local publication | Branch-aware block discovery, completeness checks, empty-block commitments, detailed ranges, persisted dyadic roots, missing-predecessor reconstruction, immutable S3 upload, local archive insertion, and replay revisions | Automatic replay initiation |
| Scheduling | A fixed Rust table selects a block cadence per chain; each lineage progresses independently | Runtime cadence configuration |
| Registry | `gw-listener` persists a complete `GatewayConfig` snapshot at startup, on relevant events, and periodically | A registry independent of Gateway |
| Peer download | Delayed durable targets, pinned registry data, per-peer rows, bounded retries, leases, per-object crash recovery, bounded bodies, recursive predecessor retrieval, and archival of every observed revision | Cross-height discovery when peers use different cadences; periodic reopening after exhaustion |
| Comparison | Exact detailed and historical scopes are grouped by digest; any visible difference is drift; quorum separately identifies a remediation reference; every attempt and digest group is persisted | Persisted uncovered intervals and cross-height evidence |
| Drift inventory | Handle-level local/observed differences, explanations, quorum attribution, target-order guards, and remission on a later concordant target | Automatic repair or revert |
| External output | Signed manifests | Signed consensus summaries, public drift reports, and status tags |
| Operations | Structured logs, durable audit state, publication/download/verification counters, queue and drift gauges, and registry-refresh health metrics | Alert definitions, cleanup, and production replay controls |

## Runtime flow

### 1. Synchronize the publisher registry

`sns-worker` does not query `GatewayConfig` directly. `gw-listener` reads one
complete contract snapshot and atomically replaces
`public.gateway_config_coprocessors`.

The snapshot contains:

- the Gateway chain and contract identity;
- the snapshot block number and hash;
- the configured threshold; and
- each coprocessor's transaction sender, manifest signer, and `s3BucketUrl`.

`gw-listener` refreshes this snapshot:

- at startup;
- after `UpdateCoprocessors`;
- after `UpdateCoprocessorThreshold`; and
- periodically, every 30 minutes by default.

An event-triggered refresh first verifies that the event block number still has the
event block hash, then pins every contract call to that canonical hash. Persistence
uses a PostgreSQL advisory transaction lock and rejects a snapshot with a lower
block number than the stored snapshot. A verification target copies the selected
snapshot into durable target and peer rows; later registry updates do not rewrite
that in-flight decision. Refresh failures increment health metrics and are retried
without stopping Gateway event processing.

### 2. Discover and seal local blocks

`block_consensus` stores local publication state. Its identity is:

```text
(host_chain_id, block_hash)
```

Competing hashes at the same height coexist. A block row is discovered from
branch-aware host-chain state. Its content can be sealed only after:

1. host-listener has closed the operation set for that exact block hash;
2. every allowed computation attributed to that block is complete;
3. its branch-scoped ct64 digest is final; and
4. the matching SnS work, ct128 digest, and ct128 format are complete.

The authoritative inventory is the set of allowed outputs in
`computations_branch` for `(host_chain_id, producer_block_hash)`. Input reuse,
upload retries, and later consumption do not attribute the handle again. An empty
block is still sealed with a block-specific digest.

Sealing writes the descriptor count and block content digest once. If a stored
value already exists or no row is updated, publication fails with the stored state
in the diagnostic log; it never overwrites a conflicting commitment.

### 3. Build and publish a manifest

A manifest is due when:

```text
block_number mod K == 0
```

The initial chain table is:

| Chain | Chain ID | `K` |
| --- | ---: | ---: |
| Ethereum mainnet | 1 | 5 |
| Sepolia | 11155111 | 5 |
| Polygon mainnet | 137 | 30 |
| Polygon Amoy | 80002 | 30 |
| Base mainnet | 8453 | 30 |
| Base Sepolia | 84532 | 30 |
| Unknown chain | any other ID | 30 |

This targets roughly one manifest per minute for the listed chains. The default
assumes one-second blocks and publishes every 30 blocks. The table should move to a
configuration file or command-line configuration before operational rollout.

Publication is ordered within each lineage. A pending block prevents only its own
descendants from overtaking it. It does not block a ready competing lineage at the
same or a later height. A failed candidate remains pending for a later scan while
the current scan tries another lineage.

The publisher:

1. loads every sealed block after the previous manifest through the publication
   block;
2. rechecks each stored descriptor count and digest;
3. reconstructs the predecessor's historical frontier;
4. snapshots that frontier as the manifest's historical ranges;
5. folds the new detailed blocks into the next frontier and persists newly created
   dyadic roots;
6. validates and signs the payload;
7. uploads it with `If-None-Match: *`;
8. archives the exact signed bytes locally;
9. marks the local block row published; and
10. optionally inserts a delayed peer-verification target in the same database
    transaction.

If S3 already contains the immutable key, the publisher downloads the existing
object and accepts it only when its signed payload equals the intended payload.
This makes an S3 success followed by a database rollback safe to retry.

### 4. Download peer revisions

After the configured verification delay, a worker binds one waiting target to the
current complete registry snapshot. It creates one durable row per peer, then claims
the target with an expiring lease.

Network I/O occurs outside long database transactions. For the exact local
publication block identity, the worker lists numbered keys and skips bodies already
in the archive. It downloads, authenticates, and commits one bounded-size object at
a time, so a later timeout or process crash does not discard earlier revisions.
Each object is accepted
only if:

- its key has the canonical version, context, chain, block, hash, and revision;
- the signed body has the same identity;
- the signature recovers the registered publisher; and
- the payload validates canonically.

Every accepted revision is stored in the common manifest archive. A higher revision
is tip-eligible only when revisions `0..n` form a complete authenticated
`supersedes` chain. Merely observing the largest S3 key is not sufficient.
Signed `previous_manifest` and `supersedes` references include their publisher.
Missing referenced objects are fetched recursively from the same peer bucket, with
a bounded traversal, so signer rotation and missing historical bodies do not hide
an otherwise available comparison.

If a worker crashes, another worker reclaims the expired lease. Completed peers in
the same attempt are skipped, archived revisions are not downloaded again, and an
expired lease cannot finalize the target. Retry count and delay are bounded by:

- `--consensus-verification-retry-count`; and
- `--consensus-verification-retry-delay`.

### 5. Compare commitments

The verifier selects each publisher's highest tip-eligible revision for the target
block identity. It compares only exact scopes:

- a detailed scope is identified by its first block, last block, and ending block
  hash;
- a historical scope is identified by its first block, last block, scale, and
  ending block hash.

Different lineages are not comparable. Extra older history present for only some
publishers is uncovered evidence, not agreement and not drift.

For every scope, publishers are grouped by digest. The aggregate rules are:

| Visible result | Outcome | Quorum meaning |
| --- | --- | --- |
| All comparable values agree and the group reaches threshold | `consensus` | The value is a remediation reference |
| All comparable values agree but the group is below threshold | `unknown_but_equal` | No visible drift, but insufficient quorum |
| Any comparable values differ | `drift` | Drift exists whether or not a group reaches threshold |
| Only historical scopes reach quorum, without a decisive local detailed result | `partial_consensus` | Coverage is incomplete |
| No useful comparable result | `unknown` | No conclusion |

A matching detailed range never hides a different historical range. Conversely,
failure to localize an already different historical range does not erase the drift;
it only limits the precision of the handle inventory.

Quorum and drift answer different questions:

- a difference answers “does divergence exist?”; and
- quorum answers “is one observed value supported strongly enough to be a possible
  remediation reference?”

If every coprocessor publishes a different value, every coprocessor is drifted and
there is no remediation reference.

### 6. Persist handle findings and remission

For a different detailed scope, the verifier compares blocks, then merges the two
canonical handle lists. It persists:

- `missing_handle` when the observed side contains a handle absent locally;
- `unexpected_handle` when the local side contains a handle absent from the
  observed side; and
- `descriptor_mismatch` when both sides contain the handle with different
  descriptor fields.

Descriptor findings retain local and observed keyset IDs, ct64 digests, ct128
digests, and formats. `observed_has_quorum` records whether the observed descriptor
belongs to the unique threshold group. A nullable `local_gateway_key_id` is retained
only as legacy diagnostic provenance.

Historical localization follows authenticated predecessor manifests until it has
the detailed block bodies needed to recompute the different range. If either side's
required bodies are unavailable, localization remains incomplete and no invented
handle finding is written.

When a later local target is concordant, findings for block hashes covered by its
detailed range are marked resolved. Monotonic verification-target IDs prevent a
stale worker from reopening or resolving findings. Manifest revision remains useful
diagnostic data, but is not an ordering key across publication blocks.

## Canonical manifest model

### Immutable identity and object key

The object key is:

```text
<s3BucketUrl>/manifests/v1/<coprocessor_context_id>/<host_chain_id>/<block_number>/<block_hash>/<revision>
```

The immutable identity is:

```text
(publisher, version, coprocessor_context_id, host_chain_id,
 publication_block_number, publication_block_hash, revision)
```

Revision `0` is the first publication for that publication block in an operator's
bucket. Revision `n > 0` must reference revision `n - 1` for the same block and bind
the predecessor publisher and manifest digest. The publisher may therefore change
during signer rotation while the signed chain remains explicit. Older revisions
remain immutable evidence.

`previous_manifest` is different from `supersedes`:

- `supersedes` links revisions of the same publication block; and
- `previous_manifest` links this publication block to the preceding publication on
  the same lineage.

### Payload fields

Version 1 signs:

- format version and publisher;
- coprocessor context and host chain;
- publication block number, hash, and parent hash;
- revision and optional superseded revision;
- one detailed range containing complete block entries and descriptor lists;
- newest-to-oldest dyadic historical ranges;
- an optional full-consensus checkpoint; and
- the previous-manifest reference.

JSON is the S3 representation. It is not hashed directly. Canonical encoding uses
fixed field order, fixed-width integers, array lengths, and explicit presence bytes
for optional fixed-width fields.

### Ciphertext descriptor

Each newly generated allowed handle has one descriptor:

```text
handle:           bytes32
keyset_id:        uint256
gateway_key_id?:  uint256
ct64_digest:      bytes32
ct128_digest:     bytes32
ct128_format:     uint8
```

Descriptors are strictly ordered by raw handle. Duplicates and unsorted lists are
invalid.

`keyset_id` identifies the compatible FHE key generation. It participates in
consensus because different key generations can explain otherwise valid but
incompatible material. `gateway_key_id` is optional signed legacy provenance. It is
excluded from the block content digest and quorum grouping, so its presence alone
cannot create drift.

### Block content digest

The block digest, called `A`, commits to the consensus fields of every descriptor:

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

The descriptor contribution contains `handle`, `keyset_id`, both ciphertext
digests, and `ct128_format`. It excludes `gateway_key_id`, publisher identity,
transactions, timestamps, object keys, and transport checksums.

An empty block uses `descriptor_count = 0` and hashes the complete header. It never
uses a zero sentinel.

### Detailed-range digest

The detailed range contains every block after the previous manifest through the
publication block. Entries are contiguous, ordered, and end at the signed
publication block.

```text
detailed_digest = keccak256(
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

The full block and descriptor entries remain in the signed body so a verifier can
explain the differing digest without fetching ciphertext bodies.

### Dyadic historical ranges

History uses aligned power-of-two ranges:

```text
[q * 2^scale, (q + 1) * 2^scale - 1]
```

`scale` is the base-2 exponent of the range size. It is not another distance field:

```text
range_size = 2^scale
```

Although start and end determine the size, signing `scale` makes the tree level
explicit and domain-separates parent construction. Validation rejects any scale that
does not match the aligned bounds.

The canonical history is right-anchored immediately before the detailed range. Let
`U` be the first detailed block and let the previous scale start at zero:

```text
next_scale = if U mod 2^(previous_scale + 1) == 0
             { previous_scale + 1 }
             else
             { previous_scale }
next_range = [U - 2^next_scale, U - 1]
```

After selecting a range, `U` becomes its start. If the next canonical range is not
fully available, history stops; the publisher does not replace it with arbitrary
smaller fragments.

A size-one range has digest `A`. Two adjacent aligned siblings form a parent:

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
  || bytes32(left_digest)
  || bytes32(right_digest)
)
```

Every root is branch-specific and immutable. A repair creates new roots with new
digests while old roots remain evidence. There is no range revision number;
manifest revision is the supersession mechanism.

### Signing

The shared implementation is
`shared/ciphertext-attestation/src/manifest.rs`. All publishers and verifiers must
use it rather than reconstructing canonical bytes independently.

Version 1 uses these eight-byte domain tags:

| Commitment | Tag |
| --- | --- |
| Block content | `FHEVMBLK` |
| Detailed range | `FHEVMDET` |
| Dyadic range | `FHEVMRNG` |
| Manifest payload | `FHEVMMNF` |

The publisher signs the Keccak-256 canonical manifest digest with its registered
coprocessor signer. S3 write access alone does not authorize an object.

## Lineages, reorgs, and finality

Publication does not wait for host-chain finality. Commitments are maintained per
lineage, and only parent/child blocks on that lineage can form a range.

After a reorg:

- the old and new block hashes coexist;
- each branch keeps independent block and range commitments;
- manifests at the same height use different object-key hash components; and
- the old signed objects remain immutable evidence.

Two different block hashes at the same height are not ciphertext drift because they
do not describe the same block. Once two manifests expose the same exact block or
range identity, any content difference is drift immediately. A later orphan status
may annotate the incident but does not make the historical observation false.

## Comparison rules and Gateway compatibility

The current Gateway contract groups votes by the complete material tuple
`keccak256(handle, keyId, ciphertextDigest, snsCiphertextDigest)`. A tuple becomes
ready when it reaches `getCoprocessorMajorityThreshold()` or when the configured
priority sender submits it. Different tuples occupy different counters; the
contract does not expose a durable aggregate drift classification.

The manifest rules preserve the safety boundary and improve diagnosis:

| Rule | Relative to Gateway |
| --- | --- |
| Group the complete comparable descriptor before counting publishers | Same safety property |
| Require quorum before one group can guide remediation | Same safety property |
| Classify every visible difference as drift, even below quorum | Improvement: split and all-different failures become explicit |
| Keep every signed revision and select the highest authenticated contiguous tip | Improvement: repair is observable without deleting evidence |
| Exclude `gateway_key_id` from consensus while retaining it as provenance | Intentional change for the post-Gateway model |
| Do not give a priority publisher a manifest bypass | Intentional change; any future override must be specified separately |

The following five-coprocessor, threshold-three populations are important:

| Groups | Result |
| --- | --- |
| `5` | Consensus |
| `4 + 1` | Drift; the group of four is a remediation reference |
| `3 + 2` | Drift; the group of three is a remediation reference |
| `2 + 2 + 1` | Drift; no remediation reference |
| `1 + 1 + 1 + 1 + 1` | Every publisher is drifted; no remediation reference |

The same population must be evaluated once from every publisher as the local
origin. The manifests do not reveal an intended ground truth when no group reaches
threshold.

## Persistence model

The schema separates mutable local progress from immutable evidence.

### `block_consensus`

One local row per `(host_chain_id, block_hash)`. It stores block identity, parent,
the inherited publication cadence, bounded child-discovery state, sealed block
digest and count, the latest local revision and publisher, publication digest, and
publication timestamps. Indexed generated state selects only rows that still need
sealing or publication. A failed attempt remains retryable.

### `block_consensus_range`

Immutable local dyadic roots. The row stores aligned bounds, scale, boundary hashes,
ending hash, and digest. The current version and context are fixed by the version 1
publisher and therefore are not schema columns.

### `block_consensus_manifest`

The shared immutable archive for local and peer manifests. It stores the complete
manifest identity, digest, canonical object key, exact signed bytes, and
`first_seen_at`. All revisions and competing lineages coexist.

### Verification targets and peer rows

A target identifies one exact local manifest revision. It stores:

- scheduling and retry state;
- the pinned registry identity, publisher count, and threshold;
- lease ownership and expiry;
- latest outcome; and
- aggregate quorum and local-drift scope counts.

Per-peer rows store the bound signer and bucket, attempt progress, errors, and the
highest observed revision. Manifest bodies are never duplicated into these rows.

### Verification attempt and scope evidence

`block_consensus_verification_attempt` retains every completed attempt's outcome,
scope counts, timestamp, and localization-completeness flag. Each corresponding
`block_consensus_verification_scope` stores exact bounds, the local digest, and the
unique quorum digest when one exists. Scope-member rows preserve every observed
publisher/digest group and whether that group met the configured threshold. The
mutable target can therefore advance without erasing a no-quorum or split-brain
decision.

### `block_consensus_drift_handle`

This table is the current local drift inventory. It stores local and observed
descriptor values, mismatch flags, observed publisher and commitments, quorum
attribution, first/last target references, diagnostic local revisions, and resolution
state.

It is evidence, not permission to replay.

## Target protocol not yet implemented

### Cross-height discovery and complete localization

Current peer discovery searches the exact local publication height and hash. The
target protocol must also handle a peer accidentally using another cadence:

1. find the newest authenticated manifest on the same lineage within bounded scan
   and freshness limits;
2. compare the intersection of exact ranges;
3. persist a cadence-mismatch warning when an observed height violates `H mod K =
   0`; and
4. persist uncovered intervals instead of interpreting missing evidence as an
   absent handle.

When a historical root differs, localization should use authenticated older
manifests from any publisher in the relevant digest group. Each refinement must
recompute its parent. Equal children are pruned; different children are followed
until exact blocks are found or an unresolved interval is persisted.

If no range group reaches quorum, diagnosis may degrade range by range, then block
by block, then handle by handle. This never lowers the configured threshold and
never treats an unavailable manifest as an absent handle.

### Full-consensus checkpoint

The payload reserves an optional full-consensus checkpoint, but the current
publisher writes `None`.

The checkpoint is the newest block through which all configured coprocessors have
gap-free agreement on one lineage. Threshold agreement may guide remediation, but
only all-coprocessor agreement advances this cleanup boundary. A future publisher
will stop historical retention at the dyadic range containing the checkpoint.

### Signed consensus summaries

A future summary will bind:

- the reporter and pinned registry snapshot;
- the local manifest tip;
- every observed digest group and publisher;
- exact compared and uncovered scopes;
- the configured threshold and unique winning group, if one exists;
- the outcome; and
- immutable manifest references supporting the decision.

The summary must preserve local, observed, and quorum values separately. A
below-quorum difference remains explained drift with
`observed_has_quorum = false`; it is not flattened into an unexplained unknown.

Downstream consumers must authenticate a summary and its referenced manifest
evidence. Trusting the reporter alone would recreate a single-coprocessor authority.

### Public drift evidence

Public drift reports are also future work. If implemented, each reporter will append
signed immutable revisions under:

```text
<s3BucketUrl>/drift/v1/<context>/<chain>/<origin_block>/<origin_hash>/<revision>
```

A report will contain commitment groups, quorum attribution, exact scope and
coverage, localization references, and manifest digests. It will not contain
ciphertext bodies. Publication must use a durable queue; a best-effort object tag is
not evidence.

### Retention and observability

No manifest deletion worker is currently enabled. All archive rows and S3 objects
are retained.

Future deletion must preserve:

- current tips and required predecessors;
- unresolved or below-quorum incidents;
- localization inputs;
- public evidence references;
- retained reorg lineages; and
- every range needed above the full-consensus checkpoint.

Bucket-wide lifecycle administration is unsafe because the ct128 bucket contains
other objects. Cleanup must be exact-prefix and dependency-aware.

The current low-cardinality metric set covers publication success/failure, peer
archive and failure counts, verification outcomes, incomplete localization,
publication backlog, target states, oldest due-target age, latest publication time,
unresolved drift handles, and Gateway registry refresh health. Handles, hashes,
digests, URLs, and error strings are deliberately excluded from labels. Alerts for
stalled publication, stale registry refresh, aging targets, unresolved drift, and
retry exhaustion remain deployment work. Cadence mismatch and future replay state
need metrics when those control planes are implemented.

The principal metric names are:

- `coprocessor_sns_manifest_publication_{success,failure}_total`;
- `coprocessor_sns_peer_manifest_{archived,download_failure}_total`;
- `coprocessor_sns_manifest_verification_total{outcome=...}`;
- `coprocessor_sns_manifest_verification_failure_total`;
- `coprocessor_sns_drift_localization_incomplete_total`;
- `coprocessor_sns_manifest_pending_work`;
- `coprocessor_sns_manifest_verification_targets{state=...}`;
- `coprocessor_sns_manifest_oldest_due_verification_age_seconds`;
- `coprocessor_sns_manifest_latest_publication_unixtime`;
- `coprocessor_sns_drift_handles_unresolved`; and
- `coprocessor_gw_listener_registry_refresh_{success,failure}_total`,
  `coprocessor_gw_listener_registry_last_success_unixtime`, and
  `coprocessor_gw_listener_registry_snapshot_block_number`.

## Requirements before replay is connected

The existing revision helper and drift-handle table make remission testable, but
they are not a production replay control plane. The following are blockers.

1. **Durable incident and job.** An immutable incident must pin the verification
   target, registry snapshot, observed groups, unique winner when one exists, local
   losing commitment, lineage, coverage, policy, and expected result. A separately
   leased job must advance idempotently through replay, material completion,
   revision publication, re-verification, and resolution.
2. **Fail-closed eligibility.** Replay is eligible only when one authenticated
   group satisfies the required quorum/intersection rule, local state is outside
   that group, no competing winner is possible, and the exact block hash still
   satisfies the configured lineage and finality policy. Below-quorum drift,
   different lineages, stale authority, or ambiguous evidence never authorizes
   replay.
3. **Repair planning and dependency closure.** The plan must distinguish keyset
   metadata, ct128-only, ct64 computation, missing-handle, and unexpected-handle
   repairs. It must include every dependent computation, PBS task, ACL association,
   bridge copy, digest, S3 object, and later manifest. Until selective closure is
   proven, replay from the earliest affected block is safer than handle-only replay.
4. **Localization completeness.** Selective replay is forbidden while any
   different interval is uncovered. The system must wait, require operator review,
   or deliberately choose a conservative suffix replay from the earliest safe
   bound.
5. **Coordinated rollback and manifest roll-forward.** Mutable computation and
   manifest state must be reset together while immutable manifests, observations,
   and incidents remain. Every later manifest whose detailed range or history
   depends on the repaired block must be republished as a higher revision in
   parent-before-child order.
6. **Pinned deterministic execution context.** Replay must pin the winning keyset,
   computation and inputs, lineage, protocol compatibility, software constraints,
   and expected descriptors. It must not silently use the newest server key after a
   rotation.
7. **Asynchronous completion barrier.** Computation, SnS conversion, database
   persistence, S3 upload, and manifest publication do not complete in one
   transaction. A durable barrier must prove all corrected material ready before
   exactly one revision is queued and published.
8. **Continuous reconciliation and precise remission.** New local or peer
   revisions, registry updates, and a periodic unresolved-incident scan must reopen
   evaluation after the original bounded target is exhausted. Equal visible values
   below threshold mean no current visible drift, but still insufficient quorum to
   prove repair.
9. **Operational safety and audit.** Replay starts default-off with observation,
   dry-run, and manual-approval modes. Production requires a kill switch, scope and
   rate limits, a repeated-repair circuit breaker, readiness and alerts for stuck
   jobs, and an immutable record of evidence, policy, actor, action, and result.

The production integration test must use five coprocessors and exercise the actual
path: create drift, derive one eligible local repair, race concurrent claimers,
execute replay, regenerate and upload material, publish every affected revision,
redownload the revisions, and prove remission. It must inject a crash after each
durable phase and cover reorgs, key rotation, dependency chains, incomplete
localization, no quorum, S3 failure, and circuit-breaker activation.

## Required properties

- **Deterministic:** identical block material and lineage produce identical
  commitments.
- **Authenticated:** every accepted object is bound to an authorized publisher.
- **Immutable and revisioned:** correction appends evidence; it does not rewrite it.
- **Lineage-aware:** competing forks progress independently and are never compared
  as one block.
- **Complete:** sealing proves the local descriptor set cannot still grow.
- **Coverage-explicit:** missing evidence is uncovered, never agreement or absence.
- **Quorum-independent detection:** any visible content difference is drift.
- **Quorum-gated remediation:** only a unique sufficiently supported value can guide
  correction.
- **Crash-resilient:** publication, download, comparison, and future replay resume
  from durable state.
- **Auditable:** signed manifests and decisions remain sufficient to reconstruct an
  incident.

## Initial rollout boundary

The safe first milestone is the current observation-only path running beside
Gateway consensus. It may publish manifests and persist comparison evidence, but it
must not affect readiness or mutate computation state.

Later milestones may add complete localization, signed summaries, a reviewed replay
control plane, downstream summary acceptance, winning-group ciphertext retrieval,
and finally removal of Gateway ciphertext-material readiness.
