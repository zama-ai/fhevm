# Proposal (Option C): Per-Op Re-Randomization with Block-Scoped Execution

**Status**: discussion draft — third option alongside
`proposal-dsa-rerandomization.md` (per-op handle-bound seeds, "option A") and
`proposal-dsa-rerand-from-main.md` (tx-scoped seeds, "option B").
**Premise**: upcoming ReRand performance work brings a single re-randomization
to ~1 ms, making per-op re-randomization affordable again.

## Summary

Keep **wave2's execution architecture** — DCID scheduling, block-scoped atomic
batches, boundary materialization, in-memory same-block forwarding, the
deferral gate, and the scheduler/liveness improvements — and keep **main's
re-randomization placement**: every operation re-randomizes its inputs with a
content-derived seed. Drop the one thing that couples values to forks: the
block-hash-seeded boundary re-randomization (RFC 019's `Hash(H_B, ct)`), and
with it the entire fork-managed **value** layer (branch value tables,
settlement, canonical publication/repair, per-context drift machinery).

Concretely:

- **Boundary materialization becomes decompress-only.** At block entry,
  boundary inputs are decompressed from persisted bytes once and shared, as
  wave2 does today — but *not* re-randomized there.
- **Re-randomization happens per op**, inside the scheduler, as on main
  (`re_randomise_operation_inputs`): seed context = `(domain_sep, opcode, all
  input ciphertexts)`, sequential per-input seeds. (Seed-hardening variant
  below.)
- **The RFC 020 discipline is kept verbatim**: same-block intermediates
  forward in memory and never round-trip; cross-block consumption always
  reads persisted compressed bytes. Only the seed changes.
- **Value storage reverts to handle-keyed** (legacy tables + provenance
  columns for existence GC); publication is finality-gated.

## Why this is consensus-sound

Per-op re-randomization does **not** by itself remove the
representation-mixing hazard: the seed hashes the input bytes, so a node
feeding a working value and a node feeding the round-tripped form diverge in
both seed and input (the known pre-RFC-020 fragility). The block-level
uniformity rule is therefore retained and is what wave2's atomic block
execution already enforces: every node feeding a given op uses byte-identical
inputs — boundary inputs are `decompress(same persisted bytes)` everywhere,
same-block intermediates are the deterministic in-memory values everywhere
(the count guards make partially-executed blocks unexecutable, exactly as
today).

Given identical inputs per op and content-derived seeds, outputs are
byte-identical wherever and whenever the op is computed.

## The DSA property — unconditional, one clause

Because seeds contain **no block, transaction, or fork identity**, identical
computations converge byte-identically:

- **Aliasing (nonce-bump case)**: two transactions computing the same op on
  the same operands — same block or not — produce the same handle *and the
  same bytes*. No canonical-producer rule; `ON CONFLICT DO NOTHING` absorbs
  it (with an optional byte-equality assert as a sanity check, which is valid
  again under these seeds).
- **Equal-timestamp equivocating siblings**: structurally convergent. The
  siblings share all ancestors, so any handle consumed by a computation is
  either produced in shared history — boundary input in both siblings,
  identical decompressed bytes — or produced same-block in both siblings (by
  the same tx or an alias), where per-op determinism makes the working values
  identical. A producer present in only one sibling with no alias in the
  other means the consumer cannot execute there at all (ACL), which is the
  existence case. There is **no membership configuration that assigns two
  values to one handle**.

Hence: **every handle is associated with at most one ciphertext value,
unconditionally** (up to RFC 010/014 handle-collision bounds). No
equivocation residual, no canonical-producer rule — stronger than option B,
achieved with the seed shape that is already running in production on main.

Existence-level reorg handling remains (orphaned blocks' rows are
provenance-GC'd and the canonical chain re-executes — re-execution is
byte-stable here by construction), as does finality-gated publication.

## Seed options (cryptography to choose)

1. **Keep main's seed exactly** (`Hash^{2·sec}` context over `(opcode, input
   cts)`, per-index seeds): the [SW25] Theorem-4 shape at op granularity;
   zero change to deployed re-randomization code; smallest conceivable
   cryptographic delta.
2. **Handle-hardened per-op seed** (`Hash^{2·sec}(handle_out, ct_i, aux_i)`,
   option A's construction): Version-3-aligned (binds op + input handles +
   prev-blockhash through the collision-free handle RO), still
   alias-convergent (aliases share `handle_out` and input bytes). Choose this
   if the whitepaper lineage is preferred over the Theorem-4 shape.

Both are DSA-unconditional under the block-level uniformity rule. The
input-pipeline round (`Hash(ct, π)` before expansion) is unchanged either
way.

## Performance vs the wave2 branch

The execution engine is identical; the delta is purely rerand placement:

- **wave2**: one ReRand per distinct boundary ciphertext per block.
- **Option C**: one ReRand per ciphertext input per op — boundary inputs *and*
  same-block intermediates. At ~1 ms/ReRand and ~8 logs/tx with ~2 ct inputs
  each, a 500-op block costs on the order of 1 s of rerand *serial* time,
  but rerand runs inside partitions and parallelizes with the same width as
  the FHE work itself; against PBS-dominated op costs (tens of ms) the
  overhead is a few percent for typical mixes. The unfavorable mix is
  cheap-op-dominated blocks (trivial/scalar ops at ~1 ms/op would see ~2x);
  the favorable one is PBS-heavy DeFi blocks.
- **No other deltas**: compression, decompression counts, scheduling,
  batching, deferral — all identical. This makes the benchmark an exact A/B:
  same branch, same scheduler, rerand placement toggled.

System-wise, option C keeps wave2's block-atomicity liveness surface (the
count-guard/deferral machinery and its known-and-fixed findings) — that is
its operational price relative to option B, paid for the unconditional DSA
and the smaller crypto ask.

## What is kept / dropped (the clean seam)

**Kept from wave2 (work-orchestration layer, fork-aware for existence):**
DCID construction and the `dependence_chain` FIFO; `computations_branch` /
`allowed_handles_branch` / `pbs_computations_branch` as the scheduling and
work-tracking tables with their reorg-aware orphan cleanup; block-scoped
atomic batch execution (cross-lane closure, count guards), boundary
materialization (decompress-only), the deferral gate and terminal-error
propagation; scheduler improvements (batched block-scoped execution, O(V+E)
pruning, heartbeat and error-terminalization fixes); rerand-before-expand
input path; KAT infrastructure; upgrade-controller (blue/green, per-L1
activation); consensus-detector; the e2e consensus suite and watchdog; the
S3 attestation crate and handle-keyed addressing; test harness.

**Dropped from wave2 (fork-managed value layer):** `ciphertexts_branch` /
`ciphertexts128_branch` / `ciphertext_digest_branch` as fork-keyed value
stores (values persist handle-keyed in the legacy tables, plus provenance
columns); the settlement frontier and its write guards; branch cleanup
jobs/quarantine for values; canonical S3 publication, repair queue and
reconciler; per-context digests and drift-revert wiring; the
block-hash-seeded boundary rerand.

**Wave1 compatibility** is as analyzed in option B §5b: wave1 is a pure
shadow; legacy stays authoritative; nothing here needs recovery. The
work-orchestration tables this option keeps are precisely the part of wave1's
schema that gets promoted from shadow to live.

## Relaxing block-scoped execution (horizontal scaling)

Determinism of ciphertext materialization does not require single-worker
block execution — it requires a **uniform representation rule**: every node
and every worker must feed each op byte-identical input bytes. Wave2's rule
("same-block intermediates are in-memory working values") forces block-atomic
execution because working values exist only inside one worker's pass. Under
per-op re-randomization a second rule becomes sound: **every cross-op input
is the decompressed form of the producer's canonical compressed bytes** —
which is *execution-topology-independent*: any partitioning of work across
workers, time, or restarts yields identical bytes. (Under RFC 019 block
seeds this rule was unavailable: same-block intermediates were consumed
without re-randomization, so representation leaked into outputs with no
normalization.)

The uniformity boundary is therefore a knob, `B ∈ {block, component, tx, op}`:

| Boundary | Scaling unit | DSA | Extra machinery | Per-edge cost |
|---|---|---|---|---|
| block (this doc's baseline) | one worker per block context | unconditional | count guards / whole-block closure / deferral (kept) | none (in-memory) |
| **component (recommended relaxation)** | **same-block connected component (= DCID)** | **unconditional (byte-equal to block)** | per-DCID guards only — whole-block cross-lane closure *deleted* | none (in-memory within component; boundary decompress duplicated per component, cacheable) |
| tx | per transaction | needs canonical-alias rule + equivocation GC (option B §3) | alias dedupe | decompress+rerand on cross-tx edges |
| op | any op whose inputs' bytes exist | unconditional (all instances consume identical round-tripped bytes) | none — data-availability ordering replaces execution atomicity | decompress+rerand on every edge, even when locally avoidable |

### `B = component`: horizontal scaling as a pure scheduling relaxation

The DCID construction already partitions each block into same-block
connected components via dataflow union-find, and a consumer transaction is
always unioned with its same-block producer — so **a same-block edge can
never cross a component boundary, by construction**. A worker acquiring a
full component therefore satisfies "raw working bytes are consumed only
within one worker" as a theorem, not a policy; everything a component
consumes from outside itself is a cross-*block* boundary input, decompressed
from the same persisted bytes by whichever worker holds it.

Because the representation rule per op is *identical* to whole-block
execution (in-memory iff same-block producer, decompressed iff cross-block;
decompression is deterministic), **`B=component` is byte-equivalent to
`B=block`**: component-scoped, block-scoped, and even mixed fleets produce
identical bytes. It is a scheduling relaxation, not a consensus parameter —
no coordinated activation, no KAT change, freely reversible. Multiple
workers pick up independent components of the same block concurrently;
implementation is a *deletion* (drop the whole-block cross-lane closure from
the work query, keep the per-DCID seed-lane locking, count guards and
deferral, which are already component-scoped) and it fixes the confirmed
"whole-block closure serializes a multi-component block across workers"
scheduling finding as a byproduct.

Caveats: aliased producers on boundary inputs land in separate components
and both execute — redundant compute with convergent bytes (first-write-wins
absorbs it; the byte-equality assert remains valid). Boundary decompression
duplicates across components consuming the same handle (deterministic;
cacheable intra-node keyed by compressed bytes). The component becomes the
resume/re-execution atomic unit (crash mid-component re-executes that
component in-memory — smaller blast radius than whole-block). Worst case
remains one giant component (inherent to the dependency structure; the
slow-lane machinery exists for exactly that shape). Delivery checkbox:
verify no retained wave2 machinery silently assumes whole-block batching
(whole-block closure was load-bearing for wave2's settlement semantics,
which this option drops).

### `B = op`: the fully topology-independent endpoint

`B = op` additionally allows op-level work stealing *within* a component and
removes the re-execute-on-resume unit entirely, at the price of
decompress+rerand on every edge — paid strictly, even when a worker holds
the working value, or differently-partitioned fleets diverge. Mitigations
(consensus-neutral): the canonical medium is the *bytes*, not the database —
intra-node handoff may pass compressed bytes through memory; decompressed
forms are cacheable per handle; compression is already paid everywhere. The
per-edge overhead stacks on deep same-block chains — the decisive benchmark
case. **Unlike `B=component`, `B=op` changes bytes** relative to
block/component semantics: it is consensus-critical like the seed,
KAT-pinned, coordinated-activation-only. Given `B=component` captures the
DAG-level parallelism (the in-worker scheduler already parallelizes
independent ops within a component across partitions), `B=op` is the
fallback if op-granular stealing or zero re-execution-on-resume ever proves
necessary — not the first step.

## Delivery plan (benchmark-first)

Start **from the wave2 branch** (maximum reuse; the value layer is the only
part rewritten). Phases gate on measurements, since this option exists to be
benchmarked.

**Phase 0 — seed decision + KAT.** Cryptography picks seed option 1 or 2
(note: option 1 is what production main runs today, so the ask is "confirm
the status quo seed under the wave2 execution discipline", plus dropping
`Hash(H_B, ct)`). Golden vectors for the chosen seed.

**Phase 1 — rerand-placement prototype (small).** On a wave2 work-branch:
remove `re_randomise_boundary_input` from materialization (decompress-only);
reinstate per-op rerand in the scheduler (port main's
`re_randomise_operation_inputs`, or the handle-hardened variant). Values
still land in branch tables at this stage — this is deliberately *only* the
rerand toggle, to isolate the benchmark.

**Phase 2 — benchmark gate (the decision point).** Arms on identical
hardware: wave2 HEAD (baseline); Phase-1 branch with `B=component`
(byte-equal to `B=block`, so the block-scoped arm comes for free — it is the
same binary restricted to one worker); optionally a `B=op` arm (per-op
round-trip discipline — consume round-tripped bytes even intra-worker,
readiness = inputs available; note this arm is byte-*different* and would
need its own KAT). Runs: (a) ReRand microbench confirming the ~1 ms target
per type/size, plus decompress cost per type/size (the other half of any
per-edge cost); (b) block replays: PBS-heavy mix, cheap-op-heavy mix,
DCID-heavy chained blocks (worst case for `B=op`, where per-edge round-trips
stack on the critical path); (c) horizontal-scaling runs: N workers sharing
one multi-component block under `B=component` (speedup vs single-worker),
and under `B=op` if that arm is built. Exit criterion: rerand overhead
within an agreed budget (e.g. ≤10% makespan on representative mixes);
`B=component` is the default deployment shape if it holds (unconditional
DSA, horizontal scaling, no consensus delta vs block-scoped). If the per-op
rerand budget fails, fall back to option B (tx-scoped) with this data
informing its Phase 2.

**Phase 3 — value-layer rekey.** Persist outputs handle-keyed (legacy tables
+ provenance columns, additive migrations only); publication finality-gated;
sns/S3 to plain handle-keyed publication (attestation kept, repair/reconciler
dropped); drift detection to per-handle digests; delete settlement gating.
Orphan GC: reuse the existing cleanup discipline, now pointed at
provenance-tagged legacy rows; keep the re-arming atomicity (deletion +
re-arm in one step).

**Phase 4 — deletion + tests.** Remove the fork-value subsystems (list
above); adapt the e2e consensus suite (drift injection now targets
per-handle digests); new tests: alias convergence (nonce-bump), equivocation
convergence (fork harness with equal timestamps), restart mid-block
(count-guard path unchanged, should already hold).

**Phase 5 — rollout.** Blue/green via upgrade-controller, per-L1 activation
start block, drain, KAT in readiness. Byte-breaking vs both main and wave2
(boundary rerand removed / per-op seeds reinstated over decompressed-at-block
inputs), so the same coordinated-activation class as the other options.

## Questions for cryptography review (short list)

1. Confirm removing the block-boundary re-randomization in favor of per-op
   re-randomization restores the Theorem-4 (or, with seed option 2,
   Version-3) shape — i.e. that `Hash(H_B, ·)` was load-bearing only for the
   per-block rerand *economy*, not for security, when every op re-randomizes
   its inputs.
2. Seed option 1 vs 2 (§Seed options).
3. Confirm the block-level uniformity rule (RFC 020 discipline, unchanged)
   composes with per-op seeds — in particular that same-block intermediates,
   re-randomized at each consuming op but never round-tripped, need no
   further treatment.
4. Confirm the unconditional-DSA argument (§DSA), especially the structural
   equivocation-convergence case split.

## Comparison snapshot

| | wave2 (Fig. 5) | A: per-op handle-bound | B: tx-scoped | **C: per-op + block exec** |
|---|---|---|---|---|
| ReRand count | per (block, ct) — best | per op | per (tx, boundary ct) | per op (~1 ms target) |
| DSA | no → fork machinery | yes | yes + alias rule + equivocation residual | **yes, unconditional** |
| Alias handling | byte-convergent | byte-convergent | canonical-producer rule (load-bearing) | **byte-convergent** |
| Execution machinery | block-atomic (full) | none beyond tx | none beyond tx | block-atomic (kept from wave2) |
| Crypto ask | approved (status quo target) | new proof adaptation | new granularity + proof | **smallest: status-quo seed, drop `H_B`** |
| Fork-value machinery | full | none | none | **none** |
