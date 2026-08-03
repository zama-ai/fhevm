# Proposal: Transaction-Scoped Derandomized Re-Randomization (DSA, from main)

**Status**: discussion draft — companion to `proposal-dsa-rerandomization.md`
(kept as-is); this version is written as a delta **from `main`**, not from the
wave2 branch, and includes a performance model against wave2 and a delivery
plan.
**Relates to**: [SW25] (eprint 2025/2005), Handle-Collisions whitepaper
(Dahl–Smart–Walter, Mar 2026), RFC 010/014 (handle entropy/uniqueness),
RFC 011/019/020/023, Solana roadmap.

## 1. Framing: what main already is, and why it is both right and too slow

`main` already ships derandomized re-randomization in the [SW25] Theorem-4
shape, at **operation granularity**: `re_randomise_operation_inputs`
(scheduler.rs) seeds a `ReRandomizationContext` with `(domain_sep, opcode,
all input ciphertexts)` and draws per-input seeds — i.e. every input of every
FHE op is re-randomized. Because that seed contains no executing-block data,
main's handle→ciphertext association is already effectively single-valued
(given RFC 010/014 handle entropy), which is why main needs no
value-canonicalization machinery.

Main's problem is **cost, not soundness**: re-randomizing every input of every
op — including every intermediate flowing op-to-op inside a transaction — is
the severe overhead we accepted only as the trivial route to consensus
capability.

The wave2 branch fixes the cost by adopting the whitepaper's Fig. 5 economy
(re-randomize each ciphertext once per **block**, seeded by the current block
hash). But Fig. 5's binder — the current block hash — is the single element of
the whole construction not pinned by handle formation. It makes
handle→ciphertext multi-valued across forks, and managing that multi-valuedness
is what the ~13–14k lines of wave2's fork-keyed storage, settlement, canonical
publication/repair, drift-revert and atomic block-context execution exist for.

**This proposal picks the intermediate granularity: the transaction.** In the
[SW25] framework, re-randomization is *pre-processing of the inputs to an
Eval call*; intermediates inside the evaluated circuit are never
re-randomized. The choice of what constitutes one Eval call is ours:

| Eval call = | Seed binder | ReRand cost | Handle→ct single-valued? |
|---|---|---|---|
| one op (main) | opcode + input cts | every input of every op — **severe** | yes |
| one block (Fig. 5 / wave2) | current block hash | once per ct per block — best | **no** → full fork machinery |
| **one transaction (this proposal)** | **tx hash** | once per ct per consuming tx | **yes** (§3) |

The transaction is the **coarsest Eval granularity whose full description is
pinned by content available at handle formation**: a transaction's circuit is
its calldata (committed by `tx_hash`), whereas a block's or component's
description includes cross-transaction context that no single handle pins.

## 2. The construction

For each transaction `T` (hash `tx`), define its FHE circuit `F_T` = the
sequence of symbolic ops in `T`'s logs, and its **boundary inputs** = the
ciphertext handles referenced by `F_T` but not produced inside `T`. Then:

```
for each boundary input ct_i of T:
    seed_i ← Hash^{2·sec}(tx, ct_i, aux_i)          # aux = ⊥ for evaluated cts
    rt_i   ← TFHE.ReRand(ct_i, seed_i, pk)
evaluate F_T on (rt_0, ..) with in-memory forwarding of intra-T intermediates
compress and persist every output (keyed by handle alone)
```

Discipline rules (the RFC 020 analogue, one level down). The invariant:
**the canonical inter-transaction representation of a value is its persisted
compressed form; a working value never crosses a transaction boundary.**

- **Within a transaction**: intermediates are forwarded in memory, never
  compress→decompress round-tripped, never re-randomized. (This is main's
  existing per-transaction component execution.)
- **Across transactions — including same-block**: consumers always read the
  producer's *persisted compressed bytes* (decompress-per-consumer), and the
  `ct` hashed into the seed is those compressed bytes. Uniform on every node,
  hence byte-deterministic: identical persisted bytes, identical seeds,
  identical results.
- **Tx outputs are persisted atomically** (all of a transaction's outputs in
  one DB transaction), so no node can consume a half-persisted producer; a
  crashed node re-executes the whole transaction to identical bytes.

The compress/decompress boundary is deliberately **per transaction, not per
block**, and this is forced, not stylistic: the Eval granularity fixes the
seed scope, the round-trip boundary, *and* the unit of execution that must be
atomic in memory, and the three must coincide. If a same-block consumer were
allowed to take the producer's in-memory working value (wave2's block-level
rule), a node that executed the producer in an earlier batch — restart,
timeslice, eviction — would hold only the compressed form, and both its
round-tripped bytes and its seed (since `ct` is hashed into it) would differ
from a node that ran both in one pass; `decompress(compress(w)) ≠ w` bitwise
per RFC 020. Wave2 closes that gap by making single-pass block execution
mandatory (atomic block contexts, cross-lane closure, count guards, deferral)
— the machinery this proposal deletes. Transaction-level boundaries need only
transaction-level atomicity, which execution provides naturally.

Decompression *caching* remains legal as a pure optimization: decompressing
identical persisted bytes is deterministic, so a hot boundary value may be
decompressed once and each consuming transaction given a copy (each applying
its own seed). The cache must be keyed by the persisted compressed bytes and
must never substitute a producer's pre-compression working value.

Performance note (feeds §4): for *chained same-block transactions* the
compress→persist→decompress→ReRand step sits on the chain's critical path
between links, where wave2 forwarded in memory and compressed in parallel —
this is the sharpest corner of the comparison and a primary target of the
Phase-2 workload replay.

Input ciphertexts keep both existing rounds: the pre-expansion ReRand seeded
`Hash(ct, π)` (unchanged), and the boundary ReRand above when a transaction
consumes them.

### Why the seed is in the proven perimeter

The whitepaper's requirement (explicit in its Fig. 5 justification and
Theorem 2 proof mechanics) is that the seed input **binds the evaluation
context** — the function and its inputs — so the reduction can invert an early
random-oracle query into the evaluation it determines. Secrecy and
unpredictability of seeds are *not* required (the ROM already grants the
adversary unrestricted hash queries; the reduction eagerly evaluates on early
seed queries). `tx` binds the transaction's calldata and therefore `F_T` and
its input handle references, exactly as `blockhash_current` binds the block's
function in Fig. 5 — one granularity down. The proof adaptation is the same
shape as the whitepaper's own footnote-1 bookkeeping; it needs the authors'
confirmation (questions in §6).

## 3. The DSA property, stated honestly

Claim: every persisted ciphertext value is a function of its handle.

Induction over the dependency DAG: an output of `T` is determined by `F_T`
(bound by `tx`), `T`'s boundary input bytes (pinned by their handles,
inductively), and the seeds `Hash(tx, ct_i)`. A handle formed in block `B`
pins `(op, operand handles, chainid, blockhash(B-1), B.timestamp)` (RFC
010+014); on any fork where that handle exists, the producing transaction
re-executes with the same calldata and the same boundary bytes, giving the
same value.

Two qualifications, both inherited from the existing protocol rather than new:

1. **Same-block aliasing.** Two *different* transactions in one block (or in
   sibling blocks with equal timestamps) computing the identical op on
   identical operands form the **same handle** with different `tx` — different
   seeds, different bytes. This is routine, not adversarial: a user
   re-submitting identical calldata under a bumped nonce, or two callers
   triggering the same op on shared-state handles in one block. (In
   *different* blocks there is no collision — the parent hash/timestamp in
   the handle preimage separates everything.) Note this byte-divergence is
   **specific to tx-scoped seeds**: under both main's seed (opcode + input
   cts) and wave2's (block hash + ct), same-block aliases converge
   byte-identically. The rule is therefore load-bearing: the **canonical
   producer** of an aliased handle is the lexicographically smallest
   producing `tx_hash` in the containing block; only its bytes are
   persisted. This machinery exists and is tested on the wave2 branch
   (`aliased_tids` routing + lex-smallest dedupe at persist) and is directly
   reusable — with one adjustment: its byte-equality assertion (valid under
   block seeds) inverts here; aliased producers differing byte-wise is the
   expected state, not a consensus alarm. All nodes see the same block
   content, hence the same producer set, hence the same winner.

   A losing producer's own transaction stays internally consistent: it
   consumes *its own* in-memory value of the aliased handle downstream (an
   ordinary Eval-internal wire). Every node computes it identically
   (consensus holds), re-randomization does not change plaintexts (semantics
   hold), fully-identical transactions collide on *all* downstream handles
   with the same winner (consistent selection), a partially-overlapping
   transaction's divergent downstream handles are unique to it (DSA holds),
   and the discarded bytes are never persisted, published, or decrypted.
2. **Ethereum equivocation residual.** With RFC 014, competing fork blocks
   share a handle only under Ethereum validator equivocation (same slot, same
   timestamp — slashable; L2 re-derived blocks get different timestamps).
   Even then, the common case self-resolves: a transaction present in *both*
   siblings (the norm — both draw from one mempool) yields identical seeds
   and byte-identical values on both forks. The residual needs the
   intersection of equivocation **and** cross-sibling aliasing with a
   *differing producer set* (the lex-min canonical producer of a shared
   handle differs between siblings), giving the handle fork-dependent bytes.

   Handling is at the *existence* level, three layers:
   - **Provenance tagging**: every row carries `(producer_block_hash,
     producer_tx)` — the logical key stays the handle; provenance exists so
     GC can target exactly the orphaned sibling's rows.
   - **Orphan GC + re-arm + re-execute**: the listener follows one head, so
     equivocation manifests as a reorg — mark the losing sibling orphaned,
     delete its rows by provenance, and atomically re-arm the canonical
     sibling's computations so re-execution re-persists the canonical value
     (first-write-wins does not apply across a GC boundary; the deletion and
     re-arming are one step — a known bug class with existing regression
     tests on the wave2 branch).
   - **Finality gating on every external effect**: on-chain
     `addCiphertextMaterial` (immutable — this gate is non-negotiable), the
     handle-keyed S3 object, and anything feeding decryption require the
     producing block to be finalized. At most one equivocating sibling can
     finalize, so the transient two-value window never leaves a
     coprocessor's private DB; post-publication drift detection sees only
     post-finality state.

   The resulting precise DSA statement: **at most one value per handle is
   observable at or after finality, unconditionally**; pre-finality, a
   private store may transiently hold an orphaned sibling's value, bounded
   by the reorg window and erased by provenance GC. This is the residual
   class RFC 014 already accepts for handle uniqueness — the proposal adds
   no new assumption, and both escape hatches (per-op seeds, handle
   uniquifier) eliminate it entirely.

Optional hardening (removes qualification 1 entirely, shrinks 2): add a
per-transaction uniquifier to the on-chain handle preimage (e.g. an executor
storage counter — `tx.hash` itself is not EVM-accessible), making every op
instance's handle globally unique. Host-contract change; worth costing as a
follow-up RFC, **not** a dependency of this proposal.

### One tx hash in several blocks (reorg re-inclusion) is idempotent, not reuse

A `tx_hash` never executes twice in one canonical history (nonce), so
multi-block occurrences are fork transients: re-inclusion after a reorg, or
presence in competing siblings. Under this seed those are byte-identical
replays: the calldata (hence `F_T` and the boundary-handle references) is
fixed, the boundary bytes are pinned (a canonical tx cannot reference
orphaned-fork handles, per the ACL causality argument), so the seeds and
outputs are identical — landing under **new** handles, since RFC 010/014 put
the parent hash and timestamp in the handle preimage. Two handles, one value;
the orphaned one is existence-GC'd. There is no randomness-reuse surface: the
seed's components split the space — `tx_hash` separates different
computations, `ct` separates different input bytes — so the same seed can only
ever meet the same data (idempotent replay, which is what derandomized
evaluation is for). The only path to differing boundary bytes under one
`tx_hash` is the §3.2 equivocation-alias residual, where the `ct` component
makes the seeds differ automatically; the consequence stays confined to the
existence-GC story above. (Cross-chain: EIP-155 makes tx hashes
chain-distinct, and handles embed the chainID, so legacy-replay collisions
cannot consume another chain's ciphertexts.)

## 4. Performance against the wave2 branch

Wave2 is the correct baseline (main's per-op cost is the thing both designs
exist to eliminate). Symbolically, per block:

- **ReRand count**: wave2 = |distinct cts entering the block| (once per block).
  This proposal = Σ over txs of |boundary inputs per tx|. The delta is
  (a) a ct consumed by k transactions in one block costs k ReRands instead
  of 1, and (b) **same-block cross-transaction edges** — wave2 forwards these
  in memory (0 ReRand, 0 decompress); here each edge costs 1 decompress +
  1 ReRand. Intra-transaction intermediates cost zero in both (and are the
  bulk of ops: the ingest heuristic assumes ~8 logs/tx, so per-op rerand's
  ~8–16 ReRands/tx collapse to the tx's boundary-input count, typically 1–4).
- **Compression**: identical — both persist every output compressed.
- **Decompress**: wave2 once per boundary ct per block; here once per
  (tx, boundary ct). Block-level *decompression caching* remains legal as a
  pure optimization (decompressing identical bytes is deterministic; each
  consumer applies its own seed to a copy), which removes most of the
  decompress delta; the per-(tx, ct) ReRand is the irreducible extra.
- **System-level wins over wave2** (opposite sign, and significant): no
  atomic block-context execution — which removes the entire class of
  liveness/throughput hazards the scheduling reviews kept confirming
  (all-or-nothing count-guard coupling, deferral/no-progress re-execution
  spins, whole-block lock serialization across workers, lease-TTL churn on
  long blocks); per-transaction execution units restore fine-grained
  parallelism and lower latency; **publication is finality-gated instead of
  settlement-gated**, removing the settlement frontier from the decryption
  latency path entirely; handle-keyed storage shrinks the hot tables and
  indexes.

Expected net: ReRand is XOF expansion + compact-list encryption + add (no
PBS), while op evaluation is PBS-dominated, so the extra per-edge ReRands are
small relative to the FHE work already being done per block; the worst case
is deep same-block cross-transaction chains (exactly the DCID-heavy
workloads), the best case is everything else plus the scheduling wins. Two
measurements should gate the decision, both runnable with existing harnesses:

1. Microbench: `ReRand` cost per type/radix size vs mean op cost (extend the
   existing scheduler benches).
2. Workload replay: a DCID-heavy block (chained same-block txs) and a mixed
   block through both pipelines (wave2 branch vs prototype), comparing
   block makespan and worker CPU.

## 5. What is reused (from both trees)

**From main (the base):** per-transaction component scheduler and execution
model; legacy handle-keyed tables and queries; transaction-sender
finality-gated publication; existing reorg/block-validity handling.

**From the wave2 branch (cherry-picked, largely as-is):**
- rerand-before-expand input path (#3001) and the one-seed-per-list input
  construction;
- aliased-producer machinery: `aliased_tids` routing, lex-smallest dedupe,
  byte-equality verification at persist;
- golden-vector KAT infrastructure (`consensus_drift_repro`, byte-KATs) —
  re-pinned to the new seed;
- blue/green rollout: upgrade-controller, per-L1 activation start block,
  unanimity gate, drain-before-cutover;
- consensus-detector and the e2e multi-coprocessor consensus suite +
  watchdog (value-model agnostic);
- RFC 023 step-1 S3 attestation crate and handle-keyed S3 addressing
  (simplify: no canonical repair queue — one value per handle, publish on
  finality);
- provenance-tagging + orphan GC patterns from wave1 (existence-level only);
- test-harness improvements, panic/liveness fixes in the scheduler that are
  granularity-independent (heartbeat ticking, error terminalization).

**Deliberately not carried over:** branch/fork-keyed tables and migrations,
the settlement frontier and its write guards, branch cleanup jobs/quarantine,
canonical S3 publication + repair queue + reconciler, per-context drift
digests and auto-revert wiring, cross-lane closure/count guards/boundary
materialization/deferral, `dependence_chain` dormant `dependency_count`
coupling to blocks.

## 5b. Wave1 compatibility (already merged, deployment imminent)

Wave1 on main is a **pure shadow**: the tfhe-worker, sns-worker and
transaction-sender contain no branch-table references — execution, digests and
publication are legacy-only. Branch tables are populated only by the
host-listener's event materialization (gated on
`FHEVM_BRANCH_ACTIVATION_BLOCK`; legacy-only below it), the zkproof-worker's
input dual-write, and the legacy→branch digest mirror trigger
(`20260610130300` + striped locks `20260704120000`). Consequences:

- **Legacy stays complete and authoritative under wave1** — the authority
  flip (legacy writes stopping) happens only at the wave2 cutover, which this
  proposal replaces. Deploying wave1 burns no bridge: no backfill or data
  recovery is ever needed to adopt this path.
- **Branch tables become inert cargo**: keep them (applied migrations stay
  byte-frozen; all changes are new migration files), retire later with a
  dedicated DROP migration (invert the wave2 legacy-retirement plan). If
  wave1 runs armed for long, plan the eventual drop like any large-object
  drop.
- **Drop the digest mirror trigger early**: it fires on every legacy digest
  write regardless of env (write amplification + advisory-lock traffic on a
  hot table) and feeds a mirror nothing consumer-facing reads on main. A
  one-line migration, safe by construction.
- **Wave1's reorg/finality hardening is retained**: verified parent-linkage
  finalization and orphan-cleanup discipline are exactly the existence-level
  machinery §3's provenance GC builds on.
- **Deployment knob**: if this path is chosen, deploy wave1 with
  `FHEVM_BRANCH_ACTIVATION_BLOCK` unarmed (branch shadow stays near-empty);
  if armed meanwhile, nothing breaks — un-arming stops shadow growth at any
  point.
- **Pre-cutover checklist** on a wave1-exposed DB: legacy completeness
  invariant (every allowed handle has legacy ciphertext + digest rows), no
  tooling grew a branch-table read dependency, DSA migrations are new files
  only.

## 6. Questions for cryptography review

1. Confirm Eval-granularity = transaction with `seed = Hash^{2·sec}(tx_hash,
   ct, aux)` sits in the Theorem 2 perimeter: `tx_hash` (keccak of the signed
   transaction) as an RO commitment to `F_T` and its input references, with
   the reduction inverting early seed queries per the existing footnote-1
   technique.
2. Confirm intra-transaction intermediates need no re-randomization (they are
   internal wires of one Eval call, as intra-block wires are under Fig. 5).
3. Confirm the canonical-producer rule for aliased handles (lex-smallest
   producing tx in the containing block) introduces no issue beyond Fact 1
   duplicate handling — aliased producers are distinct Eval calls whose
   outputs are deliberately not both persisted.
4. Confirm the input-path composition: pre-expansion `Hash(ct, π)` +
   transaction-boundary `Hash(tx, ct)` on consumption.
5. Confirm the Ethereum-equivocation residual (§3.2) is acceptable given it
   is handled by existence GC and matches RFC 014's accepted residual.
6. If per-op seeds are preferred for alias-safety (`Hash(handle_out, ct)`,
   see the companion proposal), state so — that variant is provably closest
   to Version 3 but reinstates main's per-op ReRand cost; we would then need
   the per-block decompression cache plus measurements before committing.

## 7. Delivery plan

**Phase 0 — sign-off & KAT (gates everything).**
Circulate this + companion doc to the cryptographers; resolve §6. Produce
golden vectors for the new seed (extend `consensus_drift_repro`): fixed keys,
fixed tx fixtures → expected rerandomized bytes and digests. Exit: approved
seed + committed KAT vectors.

**Phase 1 — coprocessor prototype on main (no rollout machinery).**
Branch from main. Replace `re_randomise_operation_inputs` with
transaction-boundary preprocessing: compute each component's boundary-input
set (the per-tx DFG already knows produced-vs-consumed handles), seed
`Hash(tx, ct)`, ReRand once, forward intermediates as today. Port the wave2
aliased-producer dedupe onto main's persist path. Add provenance columns
`(producer_block_hash, producer_tx)` to `ciphertexts` (additive migration)
+ orphan GC keyed on block validity (wave1-lite; no settlement). Port the
rerand-before-expand input path. Exit: KAT green; two-node docker consensus
(e2e suite from the branch) green under reorg tests.

**Phase 2 — performance gate.**
Run §4's microbench + workload replay vs the wave2 branch. Decision point:
proceed / adopt per-block decompression cache / revisit granularity with the
cryptographers. Exit: measured report attached to this doc.

**Phase 3 — consensus tooling & S3.**
Re-point the drift detector at per-handle digests (legacy `ciphertext_digest`
shape) + consensus-detector unchanged; S3: handle-keyed publication on
finality with the attestation crate; delete the repair/reconciler dependency.
Exit: 3-node fork e2e (three-of-three-fork scenario) green, including the
equivocation-alias GC test (new).

**Phase 4 — rollout.**
Blue/green via upgrade-controller: per-L1 activation start block, drain
pre-activation work, unanimity gate, KAT check in the readiness probe.
Byte-breaking, so same coordination class as the wave2 cutover; the same
runbook structure applies with the settlement-drain step replaced by a
plain in-flight-work drain. Exit: staging cutover rehearsal green.

**Phase 5 — cleanup & docs.**
RFC 019 amendment (tx-scoped seeds), RFC 011/020/023 revisions, retire the
unused wave2 subsystems list (§5), Solana integration note: requirements
reduce to (a) handle formation with per-formation entropy on the host
program, (b) a tx-identity commitment available in events — both natural on
Solana; no fork-tracking/rerand framework port.

Rough sizing: Phases 1–2 are the substantive engineering (a focused subset of
one team-iteration); Phases 3–5 are mostly reuse and deletion. The largest
single risk is Phase 0 iteration with the cryptographers on granularity
(question 6), which the companion doc's per-op fallback bounds.
