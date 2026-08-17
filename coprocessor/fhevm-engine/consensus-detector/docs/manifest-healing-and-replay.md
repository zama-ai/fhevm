# Manifest healing and replay

Status: agreed design summary; healing is not implemented

Healing is local emergency containment and recovery. It prevents ct64 drift from
spreading, replaces erroneous ciphertexts with quorum-identical material, and lets
unaffected computations continue while the underlying defect or attack is
investigated. It reduces the contaminated backlog and the need to catch up work
that would otherwise have consumed erroneous ciphertexts.

It succeeds the broad drift-revert pause with selective dependency blocking. It
does not retract external effects, replace Gateway or KMS consensus, or establish
that the underlying cause has been resolved. The implemented manifest protocol
remains [observation-only](s3-coprocessor-consensus.md).

## Agreed healing flow

### 1. Classify affected computations

| State | Meaning |
| --- | --- |
| Verified drift | Computed ct64 differs from authenticated quorum evidence for the same identity. |
| Inferred drift | Computed output consumed drifted material, directly or through an already-computed descendant. Its own winning digest may not yet be known. |
| Frozen | Pending or in-flight work whose result must not become usable because its inputs are not healthy. |

Correctness means exact ciphertext-byte equality with quorum, even when plaintext
values are equivalent. The design treats differing ct64 inputs as propagating
drift through supported FHE operations. A local ciphertext already matching the
unique quorum group needs no repair merely because another peer differs.

### 2. Contain propagation under one marking lock

For a batch of newly detected ct64 drift, temporarily block scheduling, dependency
registration, and result acceptance while marking the already-computed descendant
graph. Ongoing FHE execution need not stop, but its results cannot become usable
during this marking phase. Release the lock once marking commits.

Pending descendants need not all be explicitly marked if scheduling requires every
ciphertext input to be computed and healthy. Pending outputs are unavailable, so
that readiness rule also blocks their descendants. In-flight results must recheck
input validity before acceptance, including whether the inputs changed during
execution. A dependency counter alone must not make drifted material available.

This contains contamination once marking takes effect; it cannot prevent work
already performed during the delay before detection. Inferred drift accounts for
that already-computed contamination.

### 3. Establish the target and download matching ct64

Verified and inferred drift use the same repair flow: collect authenticated peer
metadata for the exact ciphertext identity, establish a unique ct64 quorum, and
retain multiple peers in that group as candidate download sources. Verified drift
may already have the required evidence; inferred drift usually needs it collected.
ct128 differences must not split otherwise matching ct64 votes.

Pin the registry snapshot, threshold, lineage, epoch, winning ct64 digest, and
required computation metadata. Historical evidence contradicting its advertised
commitment does not establish that repair target. Validate downloaded bytes
against the winning digest and check their compatibility before installation.
On download failure or a digest mismatch, try another peer in the winning group.

The ct64 representation is part of the repair contract. The contract's boundary
semantics distinguish raw intermediates from canonical ct64, depending on how an
operand handle is obtained. Healing must preserve the representation expected by
the consuming operation; canonical ct64 must not silently substitute for a raw
intermediate, even when both represent the same plaintext. Before implementation,
confirm which representation peer commitments and downloaded objects describe,
and how raw-dependent work is restored when only canonical ct64 is available.

### 4. Resume work progressively

A repaired ct64 becomes usable without waiting for the whole block or incident to
finish. Already-computed descendants still require their own healing; repairing
an ancestor does not make their stored outputs correct. Pending work resumes only
when all its inputs are healthy.

Installation must prevent stale computations, SNS tasks, and overlapping repairs
from overwriting corrected state. Pin the expected local state and repair target;
if another writer changes that state, reconcile instead of blindly replacing it.
Local material, metadata, caches, and readiness must agree before release.

Repair identity includes full lineage and B/G generation. Orphaned ciphertexts may
also be repaired; a reorg does not invalidate correctly scoped evidence. B/G is
used when upgrades risk breaking computation consensus. Storage and queued work
must preserve that isolation, not just the detection records.

### 5. Treat ct128 separately

ct128-only divergence raises a decryption-material alert but does not freeze FHE
computation, propagate inferred drift, or trigger automatic ct128 replacement.
KMS performs its own quorum decision. The alert reports possible decryption impact
and the observed ct128 digest/format groups without declaring decryption failure.

After ct64 healing, regenerate ct128 through SNS and compare it with authenticated
quorum evidence. If it matches, persist the regenerated material and digest and
resolve that difference. If it differs, or no ct128 quorum exists, retain the alert
without blocking use of the repaired ct64. Repeating a faulty SNS implementation
is not proof of healing.

### 6. Republish accurate evidence

Publish higher manifest revisions for repaired material and rebuild downstream
historical commitments in dependency order. Include manifests containing healed
inferred-drift outputs. Preserve all previous signed revisions as evidence.

Manifests describe actual local material, including any remaining ct128
disagreement; never substitute an expected quorum digest for bytes not produced or
installed locally. No additional peer-consensus round is required merely to
confirm installation of already-authenticated winning bytes. Local completion
checks must still prove that installation and the regenerated commitments agree.

### 7. Keep unresolved cases contained

Without quorum, affected work remains blocked while humans investigate. After a
fix is deployed, replay the affected computations, similarly to drift revert but
without pausing independent work. Failed downloads try other matching sources and
retry later. Healing needs durable retry/reconciliation progress independent of
the original verification task's exhausted retry budget.

Fully localized ct128-only differences can remain cached under the existing exact
range, lineage, commitments, and quorum conditions. An unchanged disagreement must
not force repeated full-history comparisons. Changed evidence is compared again.
Completed localization means the differences are understood, not that they are
healed; the ct128 alert remains open independently. Uncovered or contradictory
history remains incomplete and uncached, while known findings are retained.

## Open implementation questions

- Exact marking-lock scope and participating scheduler, dependency-registration,
  result-acceptance, bridge, and cache paths.
- Whether frozen readiness is entirely derived or needs explicit state, and how
  to check input changes when accepting an in-flight result.
- Peer metadata discovery for inferred drift, including missing exact publication
  heights, and the precise metadata included in ct64 quorum grouping.
- Atomic installation and protection against stale computation, SNS, upload, and
  overlapping repair writes.
- Durable repair jobs, retries, no-quorum replay, block completion barriers, and
  coalescing downstream manifest republication.
- Handling missing/unexpected handles and non-computed peer statuses, which are
  not ordinary ct64 replacements.

The sections below retain related deferred protocol topics; they are not additional
prerequisites for every individual ct64 repair.

## Current boundary

The implemented verifier can localize drift at the same publication height when
authenticated manifests expose comparable exact ranges. It compares signed detailed
blocks handle by handle, descends differing historical commitments, and can combine
adjacent predecessor ranges to reconstruct a missing historical scope. It persists
successful findings while marking uncovered ranges unknown. A missing detailed
manifest is never interpreted as an absent handle.

The following capabilities remain deferred:

- discovery and comparison across different publication heights or cadences;
- durable uncovered-interval evidence;
- automatic higher-revision publication after corrected material;
- signed consensus summaries and public drift reports;
- replay authorization, execution, and remission; and
- retention cleanup based on an agreed checkpoint.

## Cross-height localization

Initial peer discovery currently searches the exact local publication height and
hash. Historical localization can fall back through five same-lineage predecessors,
but it cannot deliberately select a peer publication at another height merely
because that peer used a different cadence.

A future protocol must:

1. find the nearest authenticated manifest on the same lineage within bounded scan
   and freshness limits;
2. reconstruct and compare only identical exact ranges from the two histories;
3. persist a cadence-mismatch warning when an observed height violates its expected
   cadence; and
4. persist uncovered intervals instead of interpreting missing evidence as an
   absent handle.

Equal child commitments can be pruned. Different children are followed until exact
blocks are found or an unresolved interval is persisted. If no range group reaches
quorum, diagnosis may degrade range by range, then block by block, then handle by
handle. This never lowers the configured threshold.

Complete same-height localization in the current implementation must not be confused
with this deferred cross-height capability.

## Full-consensus checkpoint and retention

A full-consensus checkpoint and its wire representation remain a separate future
design; this summary does not introduce one.

A future checkpoint would identify the newest block through which all configured
coprocessors have gap-free agreement on one lineage. Threshold agreement may guide a
repair proposal, but only all-coprocessor agreement could advance a cleanup boundary.

No manifest deletion worker is currently enabled. Any future cleanup must preserve:

- manifests and ranges required above the checkpoint;
- unresolved or below-quorum incidents;
- localization inputs and public evidence references;
- retained reorg lineages; and
- immutable manifests referenced by audit records.

Bucket-wide lifecycle administration is unsafe because the ct128 bucket contains
other objects. Cleanup must be exact-prefix and dependency-aware.

## Signed consensus summaries

A future summary would bind:

- the reporter and pinned registry snapshot;
- the local manifest reference;
- every observed digest group and publisher;
- exact compared and uncovered block ranges;
- the configured threshold and unique winning group, if one exists;
- the outcome; and
- immutable manifest references supporting the decision.

The summary must preserve local, observed, and quorum values separately. A
below-quorum difference remains explained drift with
`observed_has_quorum = false`; it is not flattened into an unexplained unknown.
Downstream consumers must authenticate both the summary and its referenced evidence.
Trusting the reporter alone would recreate a single-coprocessor authority.

## Public drift evidence

If public drift reports are introduced, each reporter could append signed immutable
revisions under a separately versioned prefix such as:

```text
<s3BucketUrl>/drift/v1/<context>/<chain>/<origin_block>/<origin_hash>/<revision>
```

A report would contain commitment groups, quorum attribution, exact coverage,
localization references, and manifest digests, but not ciphertext bodies.
Publication would require a durable queue; a best-effort object tag is not evidence.

## Required integration coverage

Exercise the actual path across five coprocessors: detect ct64 drift, mark computed
descendants under the barrier, collect quorum evidence, download and install valid
peer material, resume eligible work, and publish corrected revisions and history.
Verify that independent computations continue outside the marking phase.

Cover scheduling and result-acceptance races, multiple blocked inputs, stale
in-flight results after repair, bridge/cached copies, overlapping repair jobs, and
crashes between durable phases. Test failed or corrupt sources with successful
peer fallback, no quorum followed by investigation and replay, and correctly
isolated orphaned lineages and B/G generations.

Test ct128-only divergence without freezing, regeneration after ct64 repair both
matching and differing from quorum, and persistent ct128 alerts with reuse of
completed exact localization. Assert that installed bytes and republished
commitments match the repair evidence without requiring a second network vote to
release each healed ct64.

## Possible later milestones

Implement healing in stages: containment and scheduler integration, peer-backed
ct64 replacement, durable recovery, then SNS reconciliation and downstream
manifest republication. Cross-height discovery, signed public summaries, and
checkpoint-driven retention remain separate follow-ups. Changing Gateway or KMS
readiness is outside this healing design.
