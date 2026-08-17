# S3-based coprocessor consensus and drift detection

Status: implemented observation-only protocol

This document describes the manifest publisher and verifier implemented by
`consensus-detector`. Current code publishes, downloads, authenticates, compares,
and archives manifests. It does not change execution, Gateway voting, decryption
readiness, ciphertext retrieval, or local state after drift.

Deferred cross-height localization, healing, replay, signed summaries, and cleanup
are specified separately in
[Manifest healing and replay](manifest-healing-and-replay.md). Statements in that
document are not part of the current operational protocol.

## Purpose

Each coprocessor periodically publishes a signed commitment to the ciphertext
material it produced. Every coprocessor downloads the other publishers' manifests
and compares authenticated commitments. This detects divergence earlier than a
per-request consumer and leaves enough signed evidence to identify the affected
blocks and handles.

Manifests currently provide observation and durable evidence only. They do not yet
replace Gateway consensus or authorize repair.

## Current implementation at a glance

Manifest publication is a normal responsibility of `consensus-detector` and has
no feature toggle. `--my-bucket` is mandatory so publication cannot be disabled
by accidentally omitting an argument. The explicit value `--my-bucket=none`
starts neither manifest worker and logs once for publication and verification;
any real bucket value requires a manifest signer and always starts publication
and peer verification.

| Area | Implemented now | Not implemented yet |
| --- | --- | --- |
| Manifest format | Version 1 canonical encoding, Keccak-256 commitments, signatures, independent revisions, and deterministic object keys | Later format versions |
| Local publication | Branch-aware block discovery, completeness checks, empty-block commitments, detailed ranges, persisted dyadic roots, missing-predecessor reconstruction, immutable S3 upload, local archive insertion, and independent immutable revisions | Automatic higher-revision publication and replay initiation |
| Scheduling | A fixed Rust table selects a block cadence per chain; each lineage progresses independently | Runtime cadence configuration |
| Registry | `gw-listener` persists a complete `GatewayConfig` snapshot at startup, on relevant events, and periodically | A registry independent of Gateway |
| Peer download | Delayed durable tasks, pinned registry data, per-peer rows, bounded retries, claims, bounded bodies, highest-authenticated-revision selection, and range-directed manifest retrieval for historical drift localization | Cross-height discovery when peers use different cadences; periodic reopening after exhaustion |
| Comparison | Exact detailed and historical block ranges are grouped by digest; any visible difference is drift; quorum separately identifies a remediation reference; every attempt and every divergent range's digest groups are persisted | Persisted uncovered intervals and cross-height evidence |
| Drift inventory | Handle-level local/observed differences, explanations, quorum attribution, task-order guards, and remission on a later concordant task | Automatic repair or revert |
| External output | Signed manifests | Signed consensus summaries, public drift reports, and status tags |
| Operations | Structured logs, durable audit state, publication/download/verification counters, queue and drift gauges, and registry-refresh health metrics | Alert definitions, cleanup, and production replay controls |

## Runtime flow

### 1. Synchronize the publisher registry

`consensus-detector` does not query `GatewayConfig` directly. `gw-listener` reads one
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
block number than the stored snapshot. A verification task copies the selected
snapshot into its durable task and peer rows; later registry updates do not rewrite
that in-flight decision. Refresh failures increment health metrics and are retried
without stopping Gateway event processing.

### 2. Discover and seal local blocks

`block_manifest_state` stores local publication state. Its identity is:

```text
(generation, host_chain_id, block_hash)
```

Competing hashes at the same height coexist. A block row is discovered from
`handle_producer_block`, which host-listener writes atomically when an allowed
TFHE computation creates a handle. Its content can be sealed only after:

1. host-listener has committed the complete handle association set for that block;
2. every associated handle whose computation is not `is_error` has a durable ct64 digest;
3. every associated handle whose computation is not `is_error` has a durable ct128 digest and format; and
4. every successful handle's Gateway key maps to a manifest keyset ID.

If the block is still unsealed because ciphertext never arrived, remaining
incomplete handles are sealed as `is_uncomputed` only when both hold:

1. `--incomplete-manifest-max-lag`: the host chain has advanced by more than
   that many due manifests (`lag * publication_cadence` blocks past N);
2. `--incomplete-block-timeout`: no handle in that block has been computed
   (digest write or `computations.completed_at`) for that duration. A newly
   computed handle resets this stall.

Worker `is_error` still takes precedence. This does not write
`computations.is_error`.

The authoritative inventory is the handles in `handle_producer_block`, keyed by
`(host_chain_id, handle, producer_block_hash)`. Host-listener writes a row only
when the producing TFHE log is persist-allowed **in the same host block**:
ingest collects `Allowed` / `AllowedForDecryption` handles from that block's
ACL logs, then `insert_computation` records the producer. That matches ACL
semantics. `ACL.allow` and `makePubliclyDecryptable` revert unless
`msg.sender` is already allowed; `allowTransient` lasts one transaction and
does not emit `Allowed`. A handle that must survive later blocks is therefore
persist-allowed in its producing transaction, which is the same ingest block.
Later `Allowed` events grant extra accounts (`allowed_handles`, PBS) but do
not add or move a producer row and do not flip `computations.is_allowed`.
Unallowed computations are inserted `is_completed` and are not durable
ciphertext, so backfilling the producer table on a later allow would invent
manifest inventory the worker never saved.

A handle whose matching `computations` row has `is_error` is still inventoried:
it is sealed as an `is_error` descriptor with no ciphertext digests, so peers
can agree the computation failed. `state_hash` continues to omit those rows.
A handle with no computation row waits for digests until the uncomputed seal
lag. Competing producer forks stay distinct; later allowance events are not
mixed into producer ownership. The table intentionally has no historical
backfill; manifest discovery begins at the deployment or generation boundary.
An empty
block is still sealed with a block-specific digest. A missing in-generation
parent is empty-sealed only when it has no `handle_producer_block` rows. If
producer rows exist, the parent is inserted unsealed and publication waits:
`lock_next` seals it as ordinary work (including the uncomputed lag). That wait
does not discard the child's unpublished seal and does not consume the child's
publication retry budget.

The publisher scans for new host blocks every `--manifest-discovery-interval`
(default 10s). That same tick also seals and publishes already-tracked rows.
Sealed cadence query errors are classified:

- **Transient** (deadlock, serialization failure, lock or statement timeout,
  pool acquire timeout): retry later, do not charge the candidate budget.
- **Integrity** (unique / check / FK / not-null): may be a race or a stuck
  invariant. Charge the publication retry budget, like an S3 failure.
- **Definitive** (schema, decode, data exception): exhaust immediately
  (`error_count >= max_attempts + 1`) so a later peel cannot reopen it.

Discovery and work-selection query errors are retried on the next interval
without consuming a candidate's budget. None of these cancel the process. A
dead database pool still fails the publisher task so the process can restart.
After bootstrap, new heights arrive via parent→child discovery; established
discovery only looks back 5 blocks for late producer rows. A longer interval
delays those steps by up to one period. If the interval exceeds host-chain
block retention (10,000 finalized blocks), undiscovered host rows may be
pruned before they can be copied into `block_manifest_state`.

Sealing writes `block_handle_count` and the block content digest once. If a stored
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
| Hoodi | 560048 | 5 |
| Polygon mainnet | 137 | 30 |
| Polygon Amoy | 80002 | 30 |
| Unknown chain | any other ID | 30 |

This targets roughly one manifest per minute for the listed chains. Ethereum L1
(~12s) uses `K = 5`; Polygon L2 (~2s) uses `K = 30`. Repeatable
`--manifest-publication-cadence CHAIN_ID:CADENCE` overlays replace K for that
chain only (helm `manifestPublicationCadence`). The override is insert-time:
already stored rows keep their cadence, and every coprocessor in the fleet must
use the same mapping. Anvil or tests that want a tight cadence set e.g.
`--manifest-publication-cadence 31337:1` (classic Anvil) and
`--manifest-publication-cadence 12345:1` (docker-compose / preview-env host)
without changing Ethereum or Polygon defaults.

Publication is ordered within each lineage. A pending block prevents only its own
descendants from overtaking it. It does not block a ready competing lineage at the
same or a later height. A failed candidate remains pending for a later scan while
the current scan tries another lineage.

The publisher:

1. loads every sealed block after the last successful local publication through the publication
   block. An unsealed in-generation predecessor aborts this prepare with a wait;
2. rechecks each stored block handle count and digest. If an unpublished seal no
   longer matches live descriptors, that seal is discarded and the block is
   resealed later; this does not consume the publication retry budget;
3. reuses or reconstructs the local historical frontier as an optimization;
4. snapshots that frontier as the manifest's historical ranges;
5. folds the new detailed blocks into the next frontier and persists newly created
   dyadic roots;
6. validates and signs the payload;
7. uploads it with `If-None-Match: *`;
8. archives the exact signed bytes locally;
9. marks the local block row published; and
10. optionally inserts a delayed peer-verification task in the same database
    transaction.

If S3 already contains the immutable key, the publisher downloads the existing
object and accepts it only when its signed payload equals the intended payload.
This makes an S3 success followed by a database rollback safe to retry. The whole
S3 put-or-recover operation has a 15-second timeout, and both outgoing and recovery
bodies are limited to 16 MiB. That bound is not configurable. The put runs in the
same transaction that holds `FOR UPDATE` on the work row, so a slow endpoint
keeps that cadence identity locked for up to 15s. `SKIP LOCKED` still lets other
lineages proceed; descendants of this row wait until the put finishes or times
out. Every non-database publication failure, including a
transient S3 failure, uses the configured finite retry budget
(`--manifest-publication-retry-count`) and delay. The default permits 30 additional
attempts after the initial attempt. Retries keep the same immutable key and
revision, so an object created after a timeout is recovered rather than overwritten.
After the budget is exhausted that publication identity is skipped: the row remains
unpublished, has no next retry time, and is no longer a descendant blocker. The
cadence does not move. If heights 5 and 10 are skipped, height 15 still publishes
and retries the nearest skip (10) once (`retry_count + 1` total). If 10 then
succeeds, its publication peels 5 the same way. A skip that already used the extra
attempt is not reopened. The `publication_retry_exhausted` gauge pages a
still-skipped identity.

### 4. Download peer revisions

If a `GatewayConfig` snapshot already exists, publication binds the verification
task to that snapshot in the same transaction as the S3 put. The verification
delay then only waits for peers to upload. If no snapshot exists yet, a worker
binds the task when it becomes due. It creates one durable row per peer, then
claims the task with an expiring claim. Before each bounded peer list or GET,
including historical fetches, the owner renews its still-valid lease in a short
database statement. Network I/O holds no task-row lock. An expired or reassigned
lease stops the old worker; already archived manifests remain reusable.
Recovered claims retain every pinned peer.
Per-peer completion skips only current-publication download work for that attempt;
historical fetching and localization still use the full peer set and reuse archived
manifests.

Network I/O occurs outside long database transactions. For the exact local
publication block identity, the worker lists numbered keys and skips bodies already
in the archive. Keys are sorted by their numeric revision in descending order. The
worker wants the highest valid revision: it GETs at most five unknown bodies and
archives the first that authenticates. Invalid heads (`NoSuchKey`, failed
authentication, wrong scope) are remembered and never fetched again, so a later
download pass can walk below a junk prefix. Once a valid revision is archived,
later passes only consider strictly newer listed keys; older keys are obsolete. A
transient get failure (timeout or S3 error) does not fall back or remember the
key; the peer is Incomplete and the next attempt retries that same higher
revision. Each object is accepted only if:

- its key has the canonical version, context, chain, block, hash, and revision;
- the signed body has the same identity;
- the signature recovers the registered publisher; and
- the payload validates canonically.

Revisions are independent signed observations from the publisher; the highest
authenticated revision for an exact publication block is selected and stored in
the common manifest archive. Every selected
manifest compares every block in its detailed range directly, including blocks not
represented by one of its historical ranges. When a historical range differs, its
newest block number and hash select the nearest local manifest whose detailed range
covers that exact block. The worker first retrieves each peer manifest at that
publication block.
If it is missing, the worker follows the signed local lineage backward through at
most five predecessor manifests. At each predecessor it can canonically combine
adjacent smaller dyadic ranges, using the same frontier construction as publication,
to reconstruct the larger exact scope being checked. Comparison happens only after
both sides have produced that same scope. An equal reconstructed historical digest
stops that branch immediately; missing component ranges leave the scope unknown.
A covering GET that finds no usable body treats that peer as missing for this
comparison. Covering S3 fetches run on the first attempt of a verification task;
the same task's later retries do not re-list that prefix. Peers already in the
archive are reused. Still-missing covering bodies are fetched again on the first
attempt of a later publication's verification task.

A listed object that cannot be used is recorded as `corrupted:` (failed
authentication, identity, or canonical-payload validation, a body exceeding
16 MiB, or `NoSuchKey` on an immutable key). Missing and oversized objects are
rejected by exact key and the downloader tries an older revision within the
same five-candidate bound. A peer is recorded as `incomplete:` when listing or a transient
GET cannot provide a required body (current prefix or covering prefix). Both
cases complete the current attempt using the available manifests and follow its
normal bounded retry policy. Rejected unusable heads are stored in
`block_manifest_peer_download.rejected_object_keys` on the task/peer row. Each
entry is the complete canonical object key, including publication identity and
revision, so rejecting a predecessor cannot hide a current manifest with the
same revision number.

A database error on a claimed verification task uses the same classes as
publication:

- **Transient** (deadlock, serialization, lock/statement timeout): release the
  claim without incrementing `attempt_count`.
- **Integrity** (unique / check / FK / not-null): charge the attempt budget.
- **Definitive** (schema, decode, data exception): `retry_exhausted` immediately.

A dead pool still fails the verifier task. Pre-claim query errors are retried
on the next poll without cancelling the process.

If a worker crashes, another worker reclaims the expired claim. Completed peers in
the same attempt are skipped, archived revisions are not downloaded again, and an
expired claim cannot finalize the task. Retry count and delay are bounded by:

- `--manifest-verification-retry-count`; and
- `--manifest-verification-retry-delay`.

### 5. Compare commitments

The verifier selects each publisher's highest authenticated archived revision for
the task block identity. Revisions need not be contiguous and do not reference or
supersede one another. It compares only exact block ranges:

- a detailed range is identified by its first block, last block, and ending block
  hash;
- a historical range is identified by its first block, last block, scale, and
  ending block hash.

Different lineages are not comparable. Extra older history present for only some
publishers is uncovered evidence, not agreement and not drift.

For every compared block range, publishers are grouped by digest. The aggregate rules are:

| Visible result | Outcome | Quorum meaning |
| --- | --- | --- |
| All comparable values agree and the group reaches threshold | `consensus` | The value is a remediation reference |
| All comparable values agree but the group is below threshold | `unknown_but_equal` | No visible drift, but insufficient quorum |
| Any comparable values differ | `drift` | Drift exists whether or not a group reaches threshold |
| Only historical ranges reach quorum, without a decisive local detailed result | `partial_consensus` | Coverage is incomplete |
| No useful comparable result | `unknown` | No conclusion |

Each attempt also records the local result against the available quorum groups:
`matches_quorum` when every local block range has a matching quorum digest,
`differs_from_quorum` when any local digest conflicts with its quorum digest, and
`inconclusive` when a local range has no quorum digest.

A matching detailed range never hides a different historical range. Conversely,
failure to localize an already different historical range does not erase the drift;
it only limits the precision of the handle inventory.

Completed historical localization is cached in the existing verification-attempt
and drift-evidence tables. An attempt sets `localization_cacheable` only after
complete localization and a unique quorum commitment for every local range.
Missing quorum or incomplete localization leaves evidence reportable but never
reusable as completed work.

Historical reconstruction must reproduce each publisher's advertised digest.
Contradictory signed history remains unresolved even when the downloaded handles
agree. Handle comparison still records any concrete differences, but the attempt
is not localization-complete or cacheable. The normal retry budget applies; its
exhaustion retains the evidence without requiring a peer to publish a revision.

A cache hit requires the same generation, chain, context, local publisher, format,
exact historical bounds and scale, ending block hash, local commitment, publisher
commitment groups, and quorum threshold. Different forks or compared commitments
cannot share a cache hit. Unchanged comparisons can reuse durable evidence after a
restart or in a later task. Detailed blocks are always compared directly. This
cache does not fetch new revisions, reopen exhausted tasks, or authorize healing.

Quorum and drift answer different questions:

- a difference answers “does divergence exist?”; and
- quorum answers “is one observed value supported strongly enough to be a possible
  remediation reference?”

If every coprocessor publishes a different value, every coprocessor is drifted and
there is no remediation reference.

### 6. Persist handle findings and remission

For a different detailed range, the verifier compares blocks, then merges the two
canonical handle lists. A `drifted_handle` row represents:

- `missing_handle` when the observed side contains a handle absent locally;
- `unexpected_handle` when the local side contains a handle absent from the
  observed side; and
- `descriptor_mismatch` when both sides contain the handle with different
  descriptor fields.

Descriptor findings retain local and observed keyset IDs, ct64 digests, ct128
digests, and formats. `target_ct64_digest` is populated only from a computed
descriptor in the unique threshold group; NULL means no ct64 target is established.
A nullable `local_gateway_key_id` is retained only as legacy diagnostic provenance.

Historical localization descends authenticated historical commitments only after
a digest differs. Every block in an available selected detailed range is compared
handle by handle, and only unequal historical ranges lead to another jump. If an
exact covering manifest is missing, closest predecessors may still reconstruct its
historical scopes by combining their signed detailed and historical ranges. This
derived evidence applies only to history; it does not invent a detailed range for
the missing publication. Findings from successful branches are retained, while
uncovered ranges remain unknown and make the attempt incomplete. No handle is
invented for an unknown range, and an incomplete attempt cannot populate the
completed-localization cache.

This is complete same-height localization when the selected manifests expose the
same publication identity and every differing scope can be reduced to signed
detailed blocks and handles. Missing covering material leaves explicit unknown
ranges and makes localization incomplete. Discovering equivalent evidence when a
peer publishes only at different block heights or cadence is deferred; see
[Manifest healing and replay](manifest-healing-and-replay.md#cross-height-localization).

When a later local task is concordant, findings for block hashes covered by its
detailed range are marked resolved. Monotonic verification-task IDs prevent a
stale worker from reopening or resolving findings. Manifest revision remains useful
through the referenced task, but is not an ordering key across publication blocks.

## Canonical manifest model

### Immutable identity and object key

The object key is:

```text
<s3BucketUrl>/manifests/v_1/context_<coprocessor_context_id>/chain_<host_chain_id>/block_<block_number>/hash_<block_hash>/consensus_epoch/<generation>/revision/<revision>
```

The immutable identity is:

```text
(generation, publisher, version, coprocessor_context_id, host_chain_id,
 publication_block_number, publication_block_hash, revision)
```

Revision `0` is the first publication for that publication block in an operator's
bucket. Higher revisions are independent immutable objects. Their signed publisher,
publication identity, and revision number are sufficient to group and order them;
no revision or predecessor manifest is referenced from the payload.

### Payload fields

Version 1 signs:

- format version and publisher;
- coprocessor context and host chain;
- publication block number, hash, and parent hash;
- revision;
- one detailed range containing complete block entries and descriptor lists;
- newest-to-oldest dyadic historical ranges;
- an optional full-consensus checkpoint.

JSON is the S3 representation. It is not hashed directly. Canonical encoding uses
fixed field order, fixed-width integers, array lengths, and explicit presence bytes
for optional fixed-width fields.

### Ciphertext descriptor

Each newly generated allowed handle has one descriptor:

```text
handle:  bytes32
status:  computed | error | uncomputed
```

`computed` then carries:

```text
keyset_id:        uint256
gateway_key_id?:  uint256
ct64_digest:      bytes32
ct128_digest:     bytes32
ct128_format:     uint8
```

`error` may carry `error_message?`. `uncomputed` has no further fields.

Descriptors are strictly ordered by raw handle. Duplicates and unsorted lists are
invalid.

`status: error` is set when the coprocessor marked the computation as a terminal
failure (`computations.is_error`). Those descriptors carry no ciphertext
material.

`status: uncomputed` is set when sealing proceeds after the incomplete-block timeout
and incomplete-manifest lag with no ciphertext for that handle.

`error_message` is the exact `computations.error_message` for a failed handle.
It is signed with the manifest body and omitted from JSON when absent. It is
not hashed into the block content digest, so GPU vs CPU `Display` differences
cannot create ciphertext drift.

`keyset_id` identifies the compatible FHE key generation. It participates in
consensus because different key generations can explain otherwise valid but
incompatible material. `gateway_key_id` is optional signed legacy provenance. It is
excluded from the block content digest and quorum grouping, so its presence alone
cannot create drift.

### Block content digest

The block digest, called `A`, commits to the consensus fields of every descriptor:

```text
A = keccak256(
      "FHEVM manifest block content v1"
      || uint8(version)
      || uint256(coprocessor_context_id)
      || uint256(host_chain_id)
      || uint256(block_number)
      || bytes32(block_hash)
      || uint256(block_handle_count)
      || descriptor[0]
      || ...
      || descriptor[n - 1]
    )
```

The descriptor contribution contains `handle` and two exclusive flags derived
from `status` (`error`, `uncomputed`). Successful descriptors then contribute
`keyset_id`, both ciphertext digests, and `ct128_format`. Error and uncomputed
descriptors contribute only the handle and those flags. The digest excludes
`error_message`, `gateway_key_id`, publisher identity,
transactions, timestamps, object keys, and transport checksums.

An empty block uses `block_handle_count = 0` and hashes the complete header. It never
uses a zero sentinel.

### Detailed-range digest

The detailed range contains every block after the previous manifest through the
publication block. Entries are contiguous, ordered, and end at the signed
publication block.

```text
detailed_digest = keccak256(
  "FHEVM manifest detailed range v1"
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
  "FHEVM manifest dyadic range v1"
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

Every root is branch-specific and immutable. Different material creates new roots
with new digests while old roots remain evidence. There is no range revision
number. Manifest revisions are independent signed observations at one publication
identity; they carry no supersession or contiguity semantics.

### Signing

The shared implementation is the `shared/block-manifest` crate. All publishers and
verifiers must use it rather than reconstructing canonical bytes independently.

Version 1 uses these UTF-8 domain tags, hashed as raw string bytes (not
length-prefixed, not padded). The version is part of the name so a later
encoding can use a different string without a width constraint:

| Commitment | Tag |
| --- | --- |
| Block content | `FHEVM manifest block content v1` |
| Detailed range | `FHEVM manifest detailed range v1` |
| Dyadic range | `FHEVM manifest dyadic range v1` |
| Manifest payload | `FHEVM manifest v1` |

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
| Keep every signed revision and select the highest authenticated revision | Improvement: later observations remain available without deleting evidence or requiring revision contiguity |
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

### `block_manifest_state`

One local row per `(generation, host_chain_id, block_hash)`. It stores block
identity, parent, the inherited publication cadence, bounded child-discovery state,
sealed block digest and count, the latest local revision and publisher, publication
digest, current-revision publication errors and retry time, and publication
timestamps.

Every non-database publication failure retains a retry time while attempts remain.
The configurable finite limit covers transient S3 failures as well as deterministic
failures; exhaustion clears the next retry time and leaves the row visible for
intervention. Indexed generated state selects only rows that still need sealing or
publication and have not exhausted their retry budget.

### `block_range_commitment`

Immutable local dyadic roots. The row stores aligned bounds, boundary hashes,
ending hash, and digest. Rust derives the dyadic scale from the bounds; the current
version and context are fixed by the version 1 publisher and therefore are not
schema columns.

### `block_manifest`

The shared immutable archive for local and peer manifests. It stores the complete
manifest identity, digest, canonical object key, exact signed bytes,
`manifest_source` (`local` or `peer`), and `archived_at`. A locally published
manifest promotes a matching previously downloaded row to `local`. All revisions
and competing lineages coexist.

### Verification tasks and peer rows

A task points to one exact archived local manifest revision. It stores:

- scheduling and retry state;
- the pinned registry identity, publisher count, and threshold;
- claim ownership and expiry;
- the latest outcome.

Its states are `pending`, `claimed`, `consensus`, and `retry_exhausted`.
`pending` is also used while registry data has not yet been pinned.

Per-peer rows store the bound signer and bucket, attempt progress, errors, and the
highest observed revision. Manifest bodies are never duplicated into these rows.

### Verification attempt and drift evidence

`block_manifest_verification_attempt` retains every completed attempt's outcome,
local quorum status, drifted block/handle counts when applicable and localization
completed (otherwise both counts are `NULL`), timestamp, and
localization-completeness flag and quorum-gated cache eligibility. On differences only,
`block_manifest_verification_attempt_drift` retains the compared range, local and
quorum digest, and the publisher/digest groups as JSON. The mutable task can
therefore advance without erasing a split-brain decision.

### `drifted_handle`

This table is the current local drift inventory. It stores local and observed
descriptor values, the observed commitment, a nullable quorum-backed ct64 target,
the latest task reference, and resolution state. It belongs to one local operator
and uses one `generation`; peer manifests remain in the archive. `detected_at`
records the first detection; `healed_at` is reserved for successful local ct64
installation. Verification agreement never sets it.
Mismatch kind and per-field differences are derived from the two stored descriptors.

The same row carries healing state: `detection_kind`, `demand_count`,
`target_evidence`, `peer_sources`, `next_retry_at`, `claimed_by`,
`claim_expires_at`, and `healed_at`. Evidence is a JSON object containing the pinned
registry/quorum and authenticated statements; sources are a JSON array of publisher
identities and download locations. Their population and the healing worker remain
planned. Demand counts require deduplication by blocked computation in scheduler
integration; retries must not blindly increment the counter.

`detection_kind` is never NULL and is either `verified` (direct peer comparison)
or `inferred` (contaminated input). Verified does not itself mean quorum-backed.
`reason` distinguishes ct64/ct128 mismatch, missing/error/uncomputed on either
side, and metadata mismatch. A target digest separately establishes quorum-backed
replacement material. `can_be_healed` requires a local ct64-repair reason, a target,
and NULL `healed_at`; it does not promise source availability. Inferred outputs
without a target remain frozen. For multiple differences the primary reason uses
absence/status first, then ct64, metadata, and ct128; stored descriptors retain
additional material differences.

Claims require both an owner and expiry and cannot remain attached to a healed
row. Inferred rows need no peer commitment, verification task, or completed SNS
metadata. Their exact local identity has a separate unique index. Observation
status remains for non-healing differences; it is not the healing completion bit.

It is evidence, not permission to replay.

## Consensus epochs

A **consensus epoch** isolates publication and peer verification across a
breaking upgrade. It is neither the manifest format version nor its revision:
the format version changes the wire protocol, a revision distinguishes
independent immutable observations at one publication block, and the epoch
identifies the compatible-verification universe that produced the manifest
data. Rolling upgrades stay in the same epoch. Blue/Green is only a deploy
mode; a non-breaking Blue/Green run does not mint a new epoch.

During a breaking upgrade, Blue continues to publish its current epoch and
Green publishes the newly minted one. They use separate immutable S3
namespaces. On a failed upgrade the Green objects may remain in S3, but no
active lineage references that epoch. Selectors always follow the epoch signed
in the local manifest; they never search S3 for the “latest” epoch.

The signed V1 payload and its canonical bytes include:

- `ManifestPayload.consensus_epoch`: relative path identifying this published
  manifest's verification universe. The baseline is `legacy`. Later
  identifiers are `{version}/block_{n}`. Detailed-range blocks and compact
  historical ranges belong to this same epoch; they do not carry a separate
  field. Dual-stack coexistence stores the epoch on DB rows
  (`generation` / `*_generation` columns), not on the wire entry.

The first manifest of an epoch contains only blocks assigned to that epoch. A
dyadic historical range may be conceptually wider than the available
epoch-local prefix, but its digest covers only the blocks present in that
epoch.

The epoch is signed routing metadata, not ciphertext-content evidence. It must
not be added to `block_content_digest`, detailed-range digests, or dyadic-range
digests: identical material produced by Blue and Green must not look like
drift. It is instead used to choose the exact manifest namespace. A V1 object
key must be epoch-qualified, for example:

```text
manifests/v_1/context_<context>/chain_<chain>/block_<publication-block>/hash_<block-hash>/consensus_epoch/<consensus_epoch>/revision/<revision>
```

`<consensus_epoch>` is the signed epoch encoded by the shared
`block_manifest::escape_consensus_epoch` helper. Slashes remain separators;
letters, digits, `-`, `_`, and ordinary dots remain readable. Other UTF-8 bytes
are escaped as `~HH` with uppercase hex, including literal `~`. Segments exactly
`.` or `..` have every dot escaped, so `v0.15/../rc1` becomes
`v0.15/~2E~2E/rc1`. The signed epoch and version are never rewritten.
Publishers and peer listing use the same `manifest_object_prefix` helper.
This encoding is applied once to the logical key; the S3 SDK handles HTTP
encoding separately. Ordinary existing epoch keys are unchanged. Empty path
segments and oversized epochs are rejected and logged during upgrade ingest;
manifest validation also enforces the final 1024-byte S3 key limit.

Peer listing, direct fetches, archive uniqueness, and covering-manifest lookup
must all include this epoch. A peer fetches the epoch signed in the local
manifest; it must never infer an epoch by listing the latest S3 object.

The pre-upgrade baseline is `legacy` (`outcome = initial`). A later
upgrade is minted in the host-listener when it accepts a finalized
`CoprocessorUpgradeProposed` log:

```text
{version}/block_{block_number}
```

`version` is currently the software version and will become the consensus
protocol version. It is expected to be unique across breaking upgrades;
`block_number` is only a fail-safe if the same version is reused.
`block_number` is the block that contains `CoprocessorUpgradeProposed`, not
`gwStartBlock`. The log is only ingested on
`CANONICAL_PROTOCOL_CONFIG_CHAIN_ID`, so the chain id is omitted. Replay of
the same proposal returns the stored identifier. There is no shared counter:
two coprocessors that saw the same log derive the same string.

`generation_history` is the one shared history for Blue and Green. When the
host listener accepts a genuinely new upgrade proposal, it appends a `pending`
row for that string and one `generation_block_window` per host-chain window.
Cutover marks that row `succeeded`; rollback marks it `failed`. A failed log
coordinate stays recorded so it cannot be reused; a later proposal is a
different block/log and therefore a new string. The GCS selector is initially
seeded from the public selector, so both stacks use `legacy` before
the first upgrade. During an upgrade Green selects the new generation in its
GCS singleton while Blue continues selecting the public generation.

### Generation-scoped ciphertext discovery

Manifest discovery does not infer ciphertext generation from block numbers.
Generation ownership comes from the B/G schema boundary:

- Blue reads `handle_producer_block`, `ciphertext_digest`, and `computations`
  from `public`;
- Green reads its GCS copies through the connection pool's GCS-first
  `search_path`; and
- Green manifest work stays parked until `DryRunStarted`, after pre-`start_block`
  computation rows have been pruned from the GCS schema.

On release, the detector validates and pins the stack-local
`blue_green_generation` value in memory. Discovery labels completed rows from
that stack-routed schema with the pinned generation, and publication and
verification continue selecting work with that pinned value. Thus the
generation field is routing metadata for already-isolated computation results;
it is not evidence derived from the ciphertext digest or block height.

`block_manifest_state` provides a durable per-generation, per-chain discovery
frontier. Generation zero bootstraps at the latest non-orphaned block already
known to the stack; that arbitrary boundary is retained across restarts. Later
generations bootstrap at `generation_block_window.start_block`. Subsequent
passes revisit the frontier and its five preceding blocks, capped at that
generation boundary. The block-number bound limits the scan; stack-local schema
routing still determines which generation owns the discovered ciphertexts.

This history stays in `public`, alongside `versioning` and `upgrade_state`; it
is deliberately not copied into `gcs`. Each row records the generation, the
proposal identity and block, the candidate stack version, and its `pending`,
`succeeded`, or `failed` outcome. It is an allocation ledger, not a manifest
selector: manifests still carry the exact generation to fetch.

All manifest-derived tables are generation-keyed:

| Table | Generation columns and keys |
| --- | --- |
| `block_manifest_state` | `generation` is part of the primary key so Blue and Green can retain state for the same block hash. |
| `block_range_commitment` | `generation` is part of the identity because the same range and digest can exist in two generations. |
| `block_manifest` | `generation` is part of the immutable object identity and every highest-revision or covering lookup. |
| `block_manifest_verification_task` | `generation` is copied from its local manifest; a composite foreign key prevents disagreement with that manifest. |
| `block_manifest_peer_download` | The task generation is retained in its key and indexes because it selects a generation-qualified peer prefix. |
| `block_manifest_verification_attempt` | The task generation is part of its task foreign key. |
| `block_manifest_verification_attempt_drift` | The task generation is recorded for every range so an audit can reproduce its covering-manifest lookup. |
| `drifted_handle` | `generation` scopes the local inventory and its verification-task references. |

The task descendants intentionally copy the generation even though it is
derivable through joins. Generation-scoped queues, audit retention, and
rollback inspection then cannot depend on an accidental cross-generation join.

All manifest-derived tables live only in `public`. Blue and Green therefore
share one durable archive and work-state store, while generation-qualified keys,
foreign keys, selectors, and claims keep their mutable work independent. Green
can publish immediately under its new generation without copying or linking to
Blue's manifest lineage. Failed Green generations remain available for audit,
but are never selected as accepted lineage.

`blue_green_generation` remains per-stack during overlap: Blue resolves the
public singleton at `N`, while Green resolves its GCS singleton at `N + 1`.
At cutover only that singleton is promoted. Manifest tables require no copy or
merge and survive both successful and failed upgrade attempts unchanged.

The Green `consensus-detector` is deployed before activation but its manifest
publisher and verifier remain parked. `UpgradeActivated` alone does not release
them: they start only once the durable GCS state reaches `DryRunStarted`, pause
again on rollback, and continue after Green becomes live at cutover. The same
stack-version transition fences the retired Blue detector from further work.

## Deferred protocol work

Cross-height localization, consensus summaries, public drift evidence, healing,
replay, and checkpoint-driven cleanup are not part of the current protocol. Their
design and safety requirements live in
[Manifest healing and replay](manifest-healing-and-replay.md).

## Current retention

No manifest deletion worker is currently enabled. All archive rows and S3 objects
are retained. Cleanup requirements are deferred to
[Manifest healing and replay](manifest-healing-and-replay.md#full-consensus-checkpoint-and-retention).

## Metrics and alerts

Metrics use only low-cardinality labels. Publication and verification series
are labeled by `generation`. Handles, hashes, digests, URLs, and error strings
are deliberately excluded.

| Signal | Intended alert |
| --- | --- |
| `coprocessor_manifest_publication_failure_total` rate | Investigate a non-zero rate while publication is enabled. The failed manifest row retains its latest error and count. |
| `coprocessor_manifest_publication_retry_exhausted` | Page immediately when greater than zero: a required manifest has exhausted its automatic retries. |
| `coprocessor_manifest_publication_pending_work` with `coprocessor_manifest_publication_success_total` | Alert when work remains pending and the success counter is not increasing. |
| `coprocessor_manifest_publication_work_selection_timeout_total` rate | Investigate database load or an unexpectedly large blocked lineage. The selector is safely cancelled after five seconds. |
| `coprocessor_manifest_verification_tasks{state=...}` and `coprocessor_manifest_verification_oldest_due_age_seconds` | Alert on retry exhaustion, pending tasks that remain due beyond the verification service objective, or claims that have expired. Healthy in-progress claims are excluded. |
| `coprocessor_manifest_verification_drift_handles_unresolved` | Page or create an incident according to the drift-response policy. |

The remaining metrics cover peer archive/download failures, verification outcomes,
incomplete localization, and Gateway registry refresh health.

The principal metric names are:

- `coprocessor_manifest_publication_{success,failure}_total`;
- `coprocessor_manifest_verification_peer_{archived,download_failure}_total`;
- `coprocessor_manifest_verification_total{outcome=...}`;
- `coprocessor_manifest_verification_failure_total`;
- `coprocessor_manifest_verification_drift_localization_incomplete_total`;
- `coprocessor_manifest_publication_pending_work`;
- `coprocessor_manifest_publication_work_selection_timeout_total`;
- `coprocessor_manifest_publication_retry_exhausted`;
- `coprocessor_manifest_verification_tasks{state=...}`;
- `coprocessor_manifest_verification_oldest_due_age_seconds`;
- `coprocessor_manifest_verification_drift_handles_unresolved`; and
- `coprocessor_gw_listener_registry_refresh_{success,failure}_total`,
  `coprocessor_gw_listener_registry_last_success_unixtime`, and
  `coprocessor_gw_listener_registry_snapshot_block_number`.

## Required properties

- **Deterministic:** identical block material and lineage produce identical
  commitments.
- **Authenticated:** every accepted object is bound to an authorized publisher.
- **Immutable and revisioned:** each revision is an independent signed observation;
  publication never rewrites an object.
- **Lineage-aware:** competing forks progress independently and are never compared
  as one block.
- **Complete:** sealing proves the local descriptor set cannot still grow.
- **Coverage-explicit:** missing evidence is uncovered, never agreement or absence.
- **Quorum-independent detection:** any visible content difference is drift.
- **Quorum-gated reference:** only a unique sufficiently supported value is marked
  as a possible remediation reference; the current service does not act on it.
- **Crash-resilient:** publication, download, and comparison resume from durable
  state.
- **Auditable:** signed manifests and decisions remain sufficient to reconstruct an
  incident.

## Initial rollout boundary

The current observation-only path runs beside Gateway consensus. It may publish
manifests and persist comparison evidence, but it must not affect readiness or
mutate computation state. Deferred milestones are listed in
[Manifest healing and replay](manifest-healing-and-replay.md#possible-later-milestones).

### Verification logs during E2E

At the default INFO level, `Starting peer manifest verification attempt` identifies
its task, attempt, epoch, chain, block hash/number, revision, and threshold.
`Completed peer manifest verification attempt` is emitted only after the outcome
transaction commits. It also reports the local publisher, archived publishers,
quorum status, outcome, and localization completeness. Optional drift counts are
`None` when not established, rather than implying zero findings. Archived
publishers identify available current manifests; they do not imply that each peer
provided every historical scope. Aborted attempts retain their failure logs and
have no completion message.

For local metrics scraping, configure `--metrics-addr=0.0.0.0:9100` and
`--gauge-update-interval-secs=5`, and expose that port from the container. The E2E
compose template does not currently set these flags. Counters record events;
the explicit refresh interval is needed for database-backed gauges.
