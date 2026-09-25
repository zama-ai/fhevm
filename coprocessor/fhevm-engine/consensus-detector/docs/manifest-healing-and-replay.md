# Manifest healing and replay

Status: containment propagation, TFHE scheduling/result checks, and healing-state
fields are implemented. The healing worker downloads known-source ct64 and
installs it locally. Inferred rows start with no target and no sources; a live
attestation quorum pins both, then GETs. A digest mismatch HEADs the same way.
A new quorum digest or an unavailable pinned target is counted and retried.
SNS/publication repair remain planned.

Healing is local emergency recovery. It prevents further ct64 contamination,
replaces erroneous local ciphertexts with quorum-identical material, and lets
unaffected work continue while the underlying defect or attack is investigated.
Correctness means exact ct64 byte equality, not just equivalent plaintexts.
Healing does not retract effects already submitted to Gateway or replace KMS
consensus. No-quorum cases require investigation and, after a fix, selective replay.

## Implementation boundary

TFHE batches now hold the shared containment barrier from before scheduling
through transaction commit, after acquiring the Blue/Green cutover guard.
Both detector passes take the same cutover guard, then validate and lock the
execution epoch before scanning computations; a stale task cannot mark the
new schema contents as contained.
Scheduling leaves the three work-window queries unchanged. After expansion,
`containment_filter` drops selected rows whose dependencies overlap unhealed
`ct64_mismatch` handles (any epoch) and in-batch consumers of those outputs.
Healthy ops in a partly frozen transaction are kept. A transaction that loses
every selected row is delayed in the same pick transaction: pending
`schedule_order` becomes `min(NOW(), window_max + 1s)` so the next pick is not
stuck on that head. If the window is empty after the drop, the worker still
reports work available so the chain is not retired. The counters
`coprocessor_containment_dropped_scheduled_total`,
`coprocessor_containment_dropped_transactions_total`, and
`coprocessor_containment_dropped_batches_total` record those drops
(with `coprocessor_containment_transactions_found_total` and
`coprocessor_containment_batches_found_total` as complements).
Immediately before persistence, the worker reloads the same handle set for
the result transactions and discards frozen successes and errors, including
in-batch descendants, without changing their computation rows or writing
ciphertexts. Independent results still commit. Error propagation also
excludes frozen descendants. The counter
`coprocessor_containment_dropped_computed_total` records these discarded outputs.
A detection after the final read remains covered by the detector's guaranteed
pass, which waits for the batch transaction to end.

The drift inventory now includes detection kind, demand priority, a pinned ct64
target, evidence and source storage, retry timing, claim ownership, and `healed_at`.
The inventory also has `is_contained BOOLEAN NOT NULL DEFAULT FALSE`.
`containment::propagate_drift(transaction, is_lock_protected)` implements the
idempotent block scan; `containment::enforce_guaranteed_containment(pool)`
first runs and commits the optimistic scan, then acquires the exclusive
`DRIFT_CONTAINMENT_BARRIER` in a fresh transaction, repeats the scan, and commits.
It returns inferred inserts and contained findings. If the
protected pass fails, optimistic findings remain committed with containment
still pending.
After each committed verification attempt, the verifier spawns containment
when uncontained ct64 findings remain, without awaiting it. No startup scan or periodic containment worker runs.
The detached task logs errors; they do not change the committed verification
result or consume another verification attempt. TFHE batches participate in the shared barrier and check containment before
scheduling and result persistence. Freeze records `tx_unlock_potential`, an EMA
of the per-batch unlock share of each drifted handle. Verification stores the
quorum group's pinned registry S3 URLs on `peer_sources` and records the
registry pin plus quorum ct64 statements on `target_evidence`. A matching
download installs the ct64 and sets `healed_at` in one transaction.

An interrupted or failed call can leave additional contaminated computations.
This is an accepted containment delay: all outputs remain subject to manifest
verification. At the next verification in the same epoch, containment processes
the pending findings together with any new ones. Until then, further contaminated
outputs can be computed and remain subject to verification.

Inferred findings are recorded in the consuming execution epoch. Unhealed ct64
from the other stack contaminates this one only when this stack has no stored
ciphertext for the handle. A healthy independently recalculated copy of the
same handle is not frozen by the other stack's finding. Each pass always
reads both available execution stacks (`public` and any `gcs-*`) block by block.
A newly inferred output on one stack is in the in-memory drifted set for later
events on either stack, regardless of which stack initiated containment.
Findings keep their owning epoch; imported roots are not duplicated merely to
record an import.

Verification does not close `drifted_handle` rows. Healing decides per handle;
only successful local installation sets `healed_at`. Unresolved verified
findings stay until healing, manual clearing, or delayed cleanup.
`detection_kind` is never NULL and records origin only: `verified` means a
direct authenticated peer comparison, not necessarily quorum-backed; `inferred`
means propagation from contaminated input. `reason` records what differs: `ct64_mismatch`,
`ct128_mismatch`, `missing_here`, `unknown_on_peer`, `error_here`, `error_on_peer`,
`uncomputed_here`, `uncomputed_on_peer`, or `metadata_mismatch`.

A nullable target digest separately records whether a quorum target exists.
The generated `can_be_healed` flag requires an unhealed row, a target, and a local
ct64-repair reason (ct64 mismatch, missing, error, or uncomputed here). It does not
promise a downloadable source. Ct128-only, peer-side, and metadata-only reasons
cannot enable ct64 replacement. Inferred outputs use `ct64_mismatch`; obtaining a
target does not change their origin. A later verified observation of the same
handle COALESCE-pins the quorum descriptor (`quorum_ct64_digest`,
`quorum_keyset_id`, and `quorum_ct128_*`), sources, and evidence onto that
inferred row (`ON CONFLICT` on handle identity) and does not insert a second
finding. Repeated observations do not overwrite a pinned healing candidate's
target, claim, priority, or origin.

An operation that failed while depending on drifted material is also recorded as
`inferred` with reason `ct64_mismatch`, and `local_present = false` when no local
ct64 exists. This does not claim that a peer succeeded; `error_here` remains a
direct observation. `local_present` describes the recorded result and does not
gate containment. Ordinary pending work still has no inferred row.

## Runtime organization

Keep four modules under `consensus-detector/src/manifest_consensus/`:

| Directory | Responsibility |
| --- | --- |
| `publication/` | Build, archive, and publish manifests. |
| `verification/` | Authenticate and compare peer evidence; record direct findings. |
| `containment/` | Propagate inferred drift, own scheduling/result-check query semantics, and coordinate the global advisory barrier. |
| `healing/` | Prioritize demand, establish fixed targets, download ct64, install it locally, and wake eligible work. |

Containment and healing run inside the existing consensus-detector service. No new
service, deployment, configuration infrastructure, or CI service setup is planned.
Do not move containment into a separate service or into `fhevm-engine-common`.
TFHE scheduling and result writing participate in the same query and lock
contract through the shared barrier key and their frozen inventory.
The worker-side integration uses the shared barrier and the frozen inventory. The containment
directory holds the propagation functions. The healing worker starts with publication and
verification: it LISTENs on `event_healing_work` and polls on
`--manifest-healing-poll-interval` (default **30s**), then picks up to
`--manifest-healing-batch-size` (default **8**) due `can_be_healed` rows and
downloads matching ct64 from peer buckets concurrently. A `ct64_mismatch` row is
due only once `is_contained` is set: installing it removes the root from the
containment scan, so its already-computed descendants must be marked first.
`missing_here`, `error_here`, and `uncomputed_here` rows have no wrong local ct64
for consumers to have read and are due without containment. Marking a row
contained wakes the worker. A matching GET writes `ciphertexts` and `healed_at`
in the same transaction, and completes the handle's pending or errored
`computations` row with its error cleared, as the TFHE upload path does when it
stores bytes. Consumers that failed on that input report their own `error_here`
difference and are healed the same way. A pass that
installs at least one handle NOTIFYs `work_available` once so idle TFHE
picks unfrozen dependents without waiting for its poll.

## Containment

| State | Meaning |
| --- | --- |
| Verified drift | Computed ct64 differs in a direct authenticated peer comparison for the same identity; a quorum target may be absent. |
| Inferred drift | An output, or failed operation, depends on drifted material. Its own quorum digest may not yet be known. |
| Frozen | Pending work that cannot execute or publish a result until its dependencies are healthy. The set is implicit. |

**Inferred drift applies to contaminated outputs already committed before
detection or that escaped the optimistic checks. Propagation marks those for
healing.** Frozen pending computations have no result to replace; they resume
ordinary execution after their dependencies are healed.

### Block scan

A pass loads every unhealed `ct64_mismatch` row into memory. Healing is allowed
only after `is_contained`, so already-contained unhealed findings stay in that
set. For each host chain the scan starts at the oldest of those findings'
`block_number` and reads every later `computations` row, always from both
execution stacks. There is no reverse index on `dependencies` and no extra
watermark table: `idx_computations_block_number (host_chain_id, block_number)`
bounds the suffix.

Operand filtering is in Rust because the drifted-handle set grows during the
scan. For each event, if an encrypted operand is already drifted:

| Event | Action |
| --- | --- |
| Already computed (stored ct64, or failed and allowed) | Insert `inferred` in the consumer stack's epoch and add the output handle to the in-memory set. |
| Completed intermediate without a stored result | Add the handle to the in-memory set so later stored descendants are still found; insert no row. |
| Pending | Leave implicit (frozen). Do not insert and do not add the output. |

Scalar slots are ignored. The same block is re-read while the in-memory set
grows, so a consumer in the same block as its drifted producer is not missed.
Duplicate rows across Blue and Green are processed twice; the second pass is a
no-op. After the exclusive pass, `is_contained` is set on the unhealed findings
loaded at the start of the pass and on inferred rows this pass inserted or
already covered.

### Implementation sequence: two-pass drift propagation

`is_contained` records completion of the guaranteed pass, not healing. Every new
finding starts false. The optimistic pass leaves the flag unchanged; the protected
pass marks the unhealed findings loaded at the start and the inferred rows it
inserted, in the same transaction after both stacks have been read. Concurrent
verified findings that arrived during the scan remain uncontained for a later
pass.
Only ct64 mismatches require propagation; a false flag on a ct128-only observation
does not request containment. Scheduling must forbid unhealed ct64 drift regardless
of this flag. `enforce_guaranteed_containment` commits the optimistic pass,
then acquires the cutover guard and exclusive computation barrier in a fresh
transaction before calling propagation with `is_lock_protected = true`.
Propagation itself takes no barrier. Callers log if the transaction is not
READ COMMITTED so the scan cannot use a pre-barrier snapshot; they do not
abort the pass.

After verification commits new ct64 drift findings to `drifted_handle`,
containment runs the same idempotent **drift propagation** twice:

1. **Optimistic pass, without the global barrier.** Mark already-computed
   descendants as inferred drift while TFHE workers continue their batches.
   Ordinary database row locks still apply. Scheduler and result-acceptance
   checks use the committed findings immediately to reduce further contamination.
   Commit this pass before requesting the barrier.
2. **Guaranteed pass, under the exclusive global barrier.** Wait for all
   participating TFHE workers to finish and commit or roll back their current
   batches. The barrier prevents workers from starting a new batch while held.
   Scan computations afresh from the oldest unhealed finding and repeat
   propagation to include any contaminated outputs committed during the
   optimistic pass or the wait. Commit these findings before releasing the
   barrier.

After the guaranteed pass, scheduling checks prevent any computation that depends
on the covered drifted inputs from executing until those dependencies are healed.
For example, a new row in `computations` consuming a drifted handle remains
implicitly frozen; it needs neither an inferred-drift row nor another propagation
pass. Pending dependency chains remain frozen through readiness checks. This
guarantee requires every worker to participate in both the barrier and the
scheduling/result-acceptance checks described below; the propagation lock alone
is insufficient.

### Immediate checks and optimistic propagation

1. When verification establishes new ct64 drift, persist and commit
   it immediately in `drifted_handle`. Peer downloads and comparisons precede this
   transaction. Do not wait for the exclusive barrier or a descendant scan before
   making the finding visible.
2. The scheduler checks required inputs against that inventory. Every ciphertext
   input must be available and healthy. A drifted completed dependency does not
   satisfy readiness; neither does an unavailable pending output. Check batch-local
   dependency paths as well as persisted boundary inputs.
3. Before accepting results, check inputs again using fresh reads. If an input has
   become drifted, discard the result and leave the computation pending and
   implicitly frozen. Do not write a completed output, create an inferred finding
   for discarded bytes, release dependents, or enqueue SNS work for that result.
4. Run optimistic propagation from every unhealed ct64 row, scanning retained
   `computations` events from the oldest of those findings through both stacks.
   Insert inferred findings for already-computed descendants. Existing inferred
   rows do not stop the scan: they may have newly completed descendants.
   Inserts are idempotent.

Scheduling and result acceptance share the predicate
`reason = 'ct64_mismatch' AND healed_at IS NULL`, for both verified and inferred
findings. Verified rows are stored only when the other group holds the quorum,
so a healthy publisher does not freeze the handle. An inferred row can still
lack a target until healing pins one. No separate forbidden-dependency
field is needed. The partial index `idx_drifted_handle_forbidden_dependency`
supports lookups by consensus epoch (`consensus_epoch`), context, chain, handle, and producer block hash.
Ct128-only findings do not forbid computation; ordinary readiness checks still
exclude missing or uncomputed inputs.

The computation rows remain for months. Containment scans those retained events
in block order without requiring additional manifest ancestry, a reverse
dependency index, or producer-inventory proof. Preserve execution scope, exact
input identity, and scalar positions. Propagation treats raw transaction-local
and canonical forms conservatively as one affected handle per epoch when
discovering contamination. This does not merge their healing state: canonical
repair must not clear raw contamination. Their bytes are not interchangeable.
Non-allowed intermediate rows can be born `is_completed=true` before execution;
that flag alone is not evidence of a stored computed result. Walk those
intermediates in memory to reach their stored outputs.

### Raw contamination remains independent of canonical healing

Containment must not treat a handle's raw transaction-local form as healable by
replacing its canonical ct64. Healing the canonical copy must leave raw-dependent
consumers frozen while that raw form remains a contamination source.

Unstored intermediates (`is_allowed = false`) have no `ciphertexts` row. They
exist only for the duration of an execute partition and are not a heal target
or an overwrite risk. They do not gate canonical installation.

The remaining constraint is a stored handle that same-transaction consumers
still consume as a local edge. Those consumers do not read `ciphertexts`: a
later batch reloads every row of the transaction and re-executes the producer
as a raw forward. That recompute must not replace installed canonical bytes
(see installation below). The consumers themselves stay frozen while the raw
path remains a contamination source.

The chosen healing gate is transaction completion: carry the producing
`transaction_id` in `drifted_handle` and check the remaining work in `computations`
for that transaction before repairing its canonical ct64. This avoids a separate
raw finding. "Transaction finished" means no internal consumer remains to execute,
not merely that a worker batch has committed. This gate is planned, not implemented.

If no producing transaction can be identified (`transaction_id` is absent), treat
the transaction as finished for this gate and allow healing to proceed. This is
an explicit fallback accepting imperfect containment, not proof that no raw
consumer remains.

Containment aims to minimize further propagation of known drift. A bug or corner
case, including an unidentified transaction, may let additional contaminated
outputs escape preventive marking. Those outputs remain subject to manifest
verification: a mismatch against available quorum evidence is detected later as
verified drift instead of being contained or marked inferred immediately. The
accepted consequence is later detection and additional repair work; containment
is not a prerequisite for verification to detect the disagreement.

These checks reduce contamination immediately but are optimistic: a finding may
commit after a worker's final check. The exclusive pass closes that race.

### Guaranteed pass under the global barrier

Use one database-wide PostgreSQL advisory barrier shared by all participating
workers and stacks. Each batch transaction acquires it in shared mode before
selecting work or taking relevant row locks and holds it through FHE execution,
result writes, and commit/rollback. Shared holders run concurrently; existing
work-claim locks keep their role.

After optimistic propagation commits, request the exclusive barrier in a fresh
transaction, without retaining drift or computation row locks. Wait for existing
batches to commit or roll back. Once exclusive, read the inventory and re-scan computations afresh, repeat
propagation, and commit before releasing the barrier.

A batch either commits before exclusive acquisition, making its escaped outputs
visible to propagation, or enters after propagation commits and observes the new
findings. No participating batch executes during the exclusive scan. This keeps
the current batch transaction structure, at the cost of waiting for in-flight
batches and briefly pausing all stacks during propagation. The lock is global;
findings and traversal remain scoped to their actual execution identities.

Several new findings can share a pass. Findings committed during a scan that were
not included require another pass; coalescing requests must not lose arrivals.
Further passes are triggered by drift verification outcomes. Each call checks
for unfinished ct64 containment in its epoch before scanning computations.

### Interrupted containment

The protected pass commits inferred findings and containment flags atomically.
A crash before that commit leaves `is_contained = false`. There is no startup
scan or periodic retry. The next verification detecting drift in the same epoch
triggers a pass over pending roots together with the new findings. Until that
verification, the accepted consequence is additional
contaminated outputs to detect and heal, not a completed containment guarantee.
Overlapping calls remain idempotent; the exclusive barrier serializes their
protected passes.

### Integration requirements

- Use fresh READ COMMITTED reads for optimistic result checks and read after
  exclusive acquisition. Cached readiness or an old transaction snapshot is not
  sufficient.
- Commit optimistic writes before requesting exclusivity. Use consistent lock
  ordering with cutover/rollback guards. Otherwise a row-lock holder waiting for
  exclusivity could block a worker that already holds the shared barrier.
- Dependency registration, bridge imports, caches, replay, and every path exposing
  computed material must enforce the same rules. Advisory locks do not fence
  non-participating writers. Cover every execution schema still serving inputs.
- Do not prune needed computation rows from the scanned suffix during
  propagation. The pass starts at the oldest unhealed finding's block; recovery
  after the retention window is a separate boundary and cannot reconstruct
  deleted events.
- Repair must preserve unhealed descendants while preventing later propagation from marking
  already-repaired material again. Define input/material versions and resolution
  ordering before installation is implemented.
- Monitor barrier wait and scan duration. Failed, cancelled, or skipped passes
  must not be reported as completed containment.

## Operational healing

### Demand determines priority

Verified and inferred drift are equally important. Work actually considered for
execution signals unlock potential for the drifted handles that freeze it,
including in-batch descendants. The result-write check uses the same rule on
discarded results. Late drift revises the same batch observation (in-batch
transitivity on the original selection, `k` updated) rather than mixing a
second sample. `tx_unlock_potential` is a `DOUBLE PRECISION` EMA on
`drifted_handle_demand` (one row per handle):
`(previous + n) / 2`. For each stalled transaction, `k` is the number of drifted
handles that transitively block it in the batch; each of those handles receives
`1/k`. Only handles seen as blockers are updated, on a separate connection and
table (`drifted_handle_demand`), so healing does not wait for the FHE
transaction to commit and does not share a row lock with the finding. Unseen handles stay at
the 0 prior. Frequently sampled handles converge faster. Under a stationary
window this estimates the next batch's unlock share. Several findings for the
same ciphertext share one repair job and one running mean.

Do not recursively promote ancestors. For `drifted A -> pending B -> pending C`,
considering B gives demand to A; C adds none while B is unavailable. For
`drifted A -> inferred B -> pending C`, considering C gives demand to B. Downloading
correct B can unblock C without repairing A. Two direct drifted inputs both receive
demand.

Prioritize eligible repairs by `tx_unlock_potential` descending, then older
block number within a chain; use waiting age across chains. Unused drifted ciphertexts are background
work. Remove obsolete demand when consumers complete or cease to be eligible.
Pace frozen work rather than continually retrying it, and wake waiters after repair.
Source availability and evidence readiness determine which stage can run, not the
importance of the handle. Back off unavailable repairs so they do not monopolize
healing capacity.

### Establish a fixed target and download

| Starting state | First action | Fallback |
| --- | --- | --- |
| Verified ct64 drift with a quorum target | Download from a known quorum peer and validate against the established digest. No new vote. | Ask all peers for signed ct metadata to find sources matching that same target. |
| Inferred drift without a target | Reuse already-available manifests if they establish per-handle quorum. | Request signed ct metadata directly for the prioritized handle and establish its quorum target. |

Inferred outputs may be too recent for manifests. Do not wait for manifest
publication or download historical manifests when direct signed metadata suffices.
Batch requests where supported, retaining per-handle priority and quorum decisions.

Both evidence paths use the same quorum rules and target storage. Authenticate
responses, bind them to the required identity and ct64 representation/compatibility
metadata, and count each registered publisher once across both paths. Quorum is
per handle, not per whole manifest. Ct128 differences must not split ct64 votes.
Confirm what the signed ct metadata actually binds before implementation; unsigned
fields cannot establish identity or compatibility. Contradictory historical
commitments do not establish a target.

Persist the registry snapshot, threshold, repair identity, winning ct64 digest,
compatibility metadata, and evidence. Retain matching publishers as download
sources. Detection origin stays inferred even when a repair target is established.
Download one matching ct64, validate its bytes against the target, and check
compatibility. If a source fails, try others; fallback metadata discovery searches
for that same target, not a new digest.

A pinned target never changes because its bytes are unavailable. If no peer can
supply it, keep the handle drifted and dependents frozen, retain evidence, alert,
and retry with backoff. Data loss, publication defects, or malicious behavior may
explain this; healing cannot reinterpret verification. Any target change requires
a separate reconciliation decision. No-quorum inferred drift remains blocked
until investigation and a fix permit repair or selective replay.

### Install locally and resume

Local ct64 repair is the first recovery deliverable. Atomically install validated
bytes, update local state, resolve the applicable drift, and wake eligible work.
Do not wait for a whole block, incident, S3 upload, or another network quorum round.
Other already-computed descendants need their own repair; pending work resumes
only when all its inputs are healthy.

Prevent stale computations, cached inputs, bridge copies, SNS tasks, and overlapping
repairs from overwriting or exposing obsolete state. After canonical installation,
TFHE persist must not replace the installed bytes. A later batch may still
recompute the handle in memory for remaining same-transaction consumers; it
must not write that result back. Persist already inserts with
`ON CONFLICT (handle, ciphertext_version) DO NOTHING` and does not `UPDATE`
ciphertext payloads. Healing relies on that contract: the conflict target is
the installed row, so a recompute is dropped. An input may be healthy again
but have changed since execution began: result acceptance needs material-version
checks as well as drift status. Exact identity includes consensus epoch (`consensus_epoch`) and
lineage; orphaned material may be repaired without confusing it with a sibling.
B/G remains the isolation mechanism for upgrades that change computation consensus.

Canonical installation must not clear containment of the raw transaction-local
form. Raw contamination is tracked independently as described under containment
above; downloaded canonical bytes do not heal it. Recompute of a healed handle
may still produce a different raw working value for local consumers; that value
must not land in `ciphertexts`.

## Deferred publication

The priority order is containment, demand-driven local ct64 repair, repaired
manifest revisions, then corrected ct64 upload. The last two are deferred development
and runtime work; neither gates local recovery.

Ct128 remains unchanged. Ct128-only differences are reported separately and do not
freeze computation or trigger ct64 healing. SNS regeneration is not part of healing.
KMS continues its own quorum decision.

For the initial implementation, preserve original S3 ct64 objects and signed ct
metadata. Repair local database copies only. Delay corrected ct64 upload even when
bytes are ready. Versioned ct64 publication may require changes to keys, signed
metadata, uploaders, and readers; that storage migration is outside this milestone.

Future manifest revisions should describe operational local ciphertexts and rebuild
affected downstream history, retaining all old signed revisions. They improve
observability and may supply evidence to peers lacking a target, but they do not
change already-pinned targets. Publishing local state before S3 repair requires
an explicit contract: the manifest does not guarantee that this operator's bucket
already serves the new bytes. Peers may need another matching download source.

Before enabling either publication path, distinguish original computation evidence
from repair-derived evidence linked to the pinned target. A copied ciphertext is
not a new independent computation vote. Delaying uploads alone only postpones this
issue; repaired manifests need the same evidence policy. Ct128 differences may
remain observable even after ct64 repair. No public healing/revision protocol or
ct64 object migration is implemented by this design.

## Remaining implementation decisions and validation

The next decisions are safe local installation and material versioning; raw/canonical
restoration; signed metadata identity fields and inferred-target registry policy;
durable demand/job claims, retries, and wakeups; and repair-aware propagation.
Background fairness must prevent unavailable high-demand work from starving all
other repairs. Publication evidence rules and versioned S3 storage come later.

The containment matrix currently seeds 100 combinations: 76 unhealed cases and
24 healed controls. It varies observation status, verified/inferred origin,
reason, containment flag, target presence, and stored/failed inferred outputs.
It checks both propagation modes, untouched evidence and targets, and idempotency.
Four additional database simulations cover successful and failed in-flight batches,
each either committed after an earlier check or discarded after a fresh check.
Uncommitted batch writes defeat the Fast pass while a shared barrier blocks the
Guaranteed pass. Committed outcomes become inferred drift; discarded outcomes
restore the complete computation row and leave no ciphertext or inferred finding.
TFHE database tests additionally exercise the production selection and persistence
functions: all three batch windows, frozen-chain rotation, healthy siblings,
late successes/errors, and transitive batch rejection. Persistence fixtures supply
compressed result placeholders; they do not test FHE execution or serialization.

Integration tests must exercise detection through containment and local installation:

- A batch commits after its final optimistic check while exclusivity is pending;
  the final pass marks it, and subsequent work cannot consume it.
- The write check sees drift and discards output without marking it completed or
  inferred, while preserving independent results.
- Failed propagation preserves committed verification and leaves containment
  pending; a subsequent explicit call can complete it.
- Parallel batch holders, cutover lock ordering, batch-local raw paths, imports,
  caches, and computation edges lacking auxiliary manifest ancestry remain safe.
- Demand is deduplicated across retries and writers; only direct drifted inputs
  receive priority, with verified and inferred treated equally.
- Manifest reuse and metadata fallback count each publisher once; alternative
  sources preserve the pinned target, including when a different quorum is available.
- Installation survives crashes and overlapping repairs, rejects stale writes,
  wakes eligible work, and does not silently heal computed descendants. A later
  TFHE persist of the same handle must leave the installed bytes unchanged.
- No-quorum, missing/corrupt downloads, repaired orphaned lineages, and B/G isolation
  are exercised. Ct128 and original S3 evidence stay unchanged.
