# Proposal: Handle-Bound Re-Randomization (Dynamic Single Assignment)

**Status**: discussion draft v2 — input wanted from cryptography (Dahl/Smart/Walter), coprocessor, and host-contracts stakeholders
**Relates to**: [SW25] (eprint 2025/2005, sIND-CPA-D / derandomized evaluation for TFHE), the Handle-Collisions whitepaper (Dahl–Smart–Walter, Mar 2026), RFC 010 (unpredictable compute handles), RFC 011 (reorg handling), RFC 019 (block-context re-randomization), RFC 020 (compress/decompress consensus), RFC 023, Solana integration roadmap

## Summary

The approved de-randomized re-randomization (whitepaper Fig. 5) seeds every
ciphertext use with

```
seed ← Hash^{2·sec}(blockhash_current, ct, aux)        # current block's hash
```

Because `blockhash_current` is not pinned by handle formation, the same handle
can be recomputed under different block hashes across forks — handle→ciphertext
is multi-valued, and most of the wave2 coprocessor machinery (fork-keyed
storage, settlement, canonical S3 publication/repair, per-context drift
detection, atomic block-batch execution) exists to manage that multi-valuedness.

This proposal replaces the seed with

```
seed ← Hash^{2·sec}(handle_out, ct, aux)               # handle of the CONSUMING op's output
```

where `handle_out` is the RFC-010 output handle of the operation consuming
`ct`:

```
handle_out = Hash^168(handle_a, handle_b, Operand, blockhash_prev) ‖ index ‖ chainID ‖ type ‖ version
```

`handle_out` is itself a random-oracle commitment to **the operation, all of
its input handles, and the previous block hash** — i.e. the same data the
Fig. 5 argument extracts from `blockhash_current`, at per-operation granularity
instead of per-block granularity. This yields **dynamic single assignment
(DSA)**: a handle can only ever be associated with one ciphertext value, across
all forks — eliminating value-level reorg handling (~13–14k of the ~16.5k
non-test lines the block-context feature adds) while preserving the structure
of the whitepaper's security argument.

A first draft of this proposal used `Hash(handle_in, prev_block_hash, ct)`.
**That variant is rejected below on cryptographic grounds** (it breaks the
function-binding property the proof requires); the analysis of why leads
directly to the corrected construction.

## Why the seed exists: the whitepaper's requirements

[SW25] shows IND-CPA-D security of TFHE requires pseudo-random evaluation:
re-randomization whose coins are derived by a random-oracle hash of the
evaluation context. The seed lineage in the whitepaper is:

- Theorem 4 [SW25]: `seed_i = Hash(ct_0..ct_{m-1}, aux_0..aux_{m-1}, F, i)` —
  the seed binds **all inputs, the function F, and the argument position**.
- Version 3 (Fig. 4, Thm 2): `seed_i = Hash(handle_0..handle_{m-1}, F,
  blockhash, ct_i, aux_i)` — ciphertext lists replaced by handles; sound iff
  `Hash^168` is a random oracle and **handles are collision-free** (the rest of
  the whitepaper exists to enforce exactly that).
- Version 4 (Fig. 5, product): `seed = Hash(blockhash_current, ct, aux)` —
  justified as equivalent to Fig. 4 **because the current block hash contains
  (commits to) the description of F and all input handles**. The whitepaper is
  explicit: *"It is vital it is the current blockhash as we require this hash
  to contain the description of the function."*

Two structural observations from the Theorem 2 proof matter here:

1. **What the proof needs from the seed input is *binding*, not secrecy or
   unpredictability.** The game is in the ROM: the adversary may query the
   hash on any input at any time, so seeds are "predictable" by construction
   whenever their preimages are known. The reduction explicitly handles an
   adversary that queries a seed *before* the corresponding evaluation: it
   inverts the query into the evaluation it determines (looking up ciphertexts
   by their unique handles) and eagerly forwards that evaluation to its own
   oracle ("it proceeds analogously to queries of type 2"). This works
   precisely because the seed input **uniquely determines the evaluation
   context** — which function, which inputs, which argument.
2. **Nested random-oracle commitments are already used.** Footnote 1 resolves
   the previous-vs-current blockhash mismatch for output handles "because the
   current blockhash is derived from it" — i.e. the proof is comfortable
   extracting bound data through a chain of RO applications, not only from raw
   seed inputs.

## Verdict on `Hash(handle_in, prev_block_hash, ct)`: not acceptable

The parent block hash precedes the block that contains the evaluation, so it
**cannot commit to F**; and `handle_in` binds only the input's provenance, not
the operation consuming it. Concretely, the reduction's eager-evaluation
technique breaks: an adversary can query the random oracle on
`(handle_in, prev_block_hash, ct)` and *afterwards* choose which function to
evaluate on `ct` in the upcoming block — the seed query no longer determines
the evaluation, so the reduction cannot program the oracle consistently. This
is precisely the property the whitepaper's "it is vital" sentence protects.
Whether a concrete attack exists is beside the point: the construction leaves
the proven perimeter, and the cryptographers rejected weaker deviations (the
index→ciphertext substitution needed Fact 1 and a new theorem).

Note what the failure is **not**: it is not about the parent hash being
revealed one block earlier. Pre-computability of seeds is handled by the proof
(observation 1 above). The failure is the loss of **function binding**.

## The corrected construction: seed from the consuming op's output handle

```
seed ← Hash^{2·sec}(handle_out, ct, aux)
```

- **Function binding, restored at finer granularity.** `handle_out`'s
  `Hash^168` preimage contains the operation (the local F), every input handle
  of that operation, and `blockhash_prev`. A seed query determines the exact
  evaluation it belongs to — the reduction's type-3 handling carries over: on
  an early seed query, extract `(op, input handles)` from the adversary's
  `Hash^168` query that formed `handle_out` (or answer randomly if it never
  made one — second-preimage resistance, as in the paper), then eagerly
  evaluate. This mirrors the footnote-1 technique; Theorem 2's assumptions
  (RO for both hashes, handle collision-freeness) are unchanged.
- **DSA.** By induction over the dependency DAG: an output's bytes are a
  function of its own handle (the seed), its inputs' bytes (pinned by their
  handles), and the op (pinned by the handle's preimage). Base cases: ZK
  inputs (below) and trivial encrypts. Hence handle→value is a *function*,
  modulo the 2^84 handle-collision bound the whitepaper's protocol enforces.
  Fork behavior: different-parent forks mint different handles (RFC 010
  entropy), so orphaned handles never exist canonically; same-parent siblings
  that form identical handles produce byte-identical values — convergence.
- **Freshness/domain separation** is per consuming operation — strictly finer
  than Fig. 5's per-block-first-use. A ciphertext used by two ops (or in two
  blocks) re-randomizes independently; the same ct appearing twice in *one*
  op's argument list receives the same seed, which is exactly the duplicate
  case Fact 1 / Theorem 1 already covers.

### Costs relative to Fig. 5

- **Per-op instead of per-block re-randomization.** Fig. 5 re-randomizes each
  ciphertext once per block (first use); this construction re-randomizes once
  per consuming operation, so a ct feeding k ops costs k ReRands. Fan-out
  within a block is typically small, and ReRand is XOF-expansion + addition
  (no PBS), but this needs measurement. (Under the DSA execution model —
  always read persisted bytes — re-randomization is per-consumer-load anyway;
  the delta is only losing the *option* of a per-block cache.)
- **Proof refresh.** Theorem 2's proof sketch must be re-adapted (the type-2/
  type-3 query bookkeeping changes shape). We believe the adaptation is
  mechanical for the reasons above; it needs the authors' confirmation.

### Input ciphertexts

Input ciphertexts receive **two** re-randomization rounds in the deployed
scheme (RFC 019's two seed constructions):

1. **Pre-expansion** (§3.4's pipeline position, ReRand → UnPack → Compress):
   seeded from the proven input itself, `Hash^{2·sec}(ct, π)`. Here the
   function is the fixed public input pipeline, so function binding is
   trivial; the seed is formation-fixed and therefore already DSA-compatible.
   **Unchanged by this proposal.**
2. **On entry to a consuming block**: the ordinary boundary re-randomization —
   today `Hash(blockhash_current, ct)` — which ZK-input ciphertexts undergo
   like any other boundary input. This is exactly the seed this proposal
   replaces: under DSA it becomes `Hash^{2·sec}(handle_out, ct, aux)` for the
   consuming operation, uniformly with computed ciphertexts. The block-level
   binding §3.4 associates with input processing is preserved at consumption
   time through `handle_out` (which commits to the op, its input handles, and
   the previous block hash).

So inputs need no special treatment: round 1 is untouched, round 2 is the
general construction above.

## Alternatives considered

| Construction | F-binding | DSA | Notes |
|---|---|---|---|
| `Hash(blockhash_current, ct, aux)` — Fig. 5, status quo | ✓ (block granularity) | ✗ | Requires the full wave2 value-canonicalization stack |
| `Hash(handle_in, prev_block_hash, ct)` — draft v1 | ✗ | ✓ | **Rejected**: seed does not determine the evaluation; proof breaks |
| **`Hash(handle_out, ct, aux)` — this proposal** | ✓ (op granularity) | ✓ | Proof adaptation mirrors existing techniques; per-op ReRand cost |
| `Hash(tx_hash, ct, aux)` | ✓ (tx granularity: calldata determines F) | ✓ (same sibling-convergence argument) | Works in principle; introduces tx serialization as a third RO and coarser-than-op seeds (duplicate-use dedup needed per tx); `handle_out` is the protocol's existing commitment object |
| `PRF_k(handle_out, ct)` with a fleet-shared secret k | ✓ | ✓ | Removes even ROM-modeled pre-computability; costs KMS-managed key + rotation coupling + loss of public re-derivability; fallback if public seeds are deemed uncomfortable despite the ROM argument |

## Questions for cryptography review

1. Confirm the Theorem 2 adaptation: seed queries of the form
   `(handle_out, ct_i, aux_i)`, with `(op, input handles, blockhash_prev)`
   extracted from the adversary's `Hash^168` query forming `handle_out`
   (eager evaluation as in the current type-3 handling; random answers for
   never-queried handles via second-preimage resistance).
2. Confirm that per-op seeds (vs Fig. 5's per-block-first-use) reintroduce no
   duplicate-randomization issue beyond what Fact 1 already covers.
3. Confirm the two-round input composition remains sound when round 2 moves
   from `Hash(blockhash_current, ct)` to `Hash(handle_out, ct, aux)`: the
   pre-expansion round keeps its formation-fixed seed `Hash(ct, π)` (RFC 019's
   input construction), and the block-level binding shifts from the consuming
   block's own hash to the consuming op's handle.
4. Confirm that seed pre-computability one block early (via `blockhash_prev`
   inside `handle_out`) is immaterial in the ROM game, per the reduction's
   handling of early oracle queries — i.e. that binding, uniqueness, and
   handle collision-freeness remain the only load-bearing seed properties.
5. Sibling-block byte-convergence (identical handles ⇒ identical values on
   same-parent forks) is the intended DSA behavior; confirm no
   randomness-reuse concern, given identical seeds imply identical evaluation
   contexts.

## What DSA buys (unchanged from v1; summary)

Measured on the wave2 branch vs main, non-test coprocessor source: of ~16.5k
lines the block-context feature adds, ~13–14k exist only to manage
multi-valued handles — fork-keyed branch tables, the settlement frontier and
its write guards, orphan-value cleanup/quarantine, canonical S3 publication +
repair queue + re-verification, per-context digests/drift detection, and the
atomic block-context execution model (cross-lane closure, count guards,
boundary materialization, deferral) whose liveness surface three review rounds
kept finding P1-class defects in. Under DSA the safe execution discipline is
"always read inputs from persisted bytes" (every node decompresses identical
bytes and applies identical seeds), i.e. the legacy per-computation model.
What remains: the ReRand crypto itself (~0.3–0.5k, with the new seed),
existence-level reorg handling and finality-gated publication (pre-wave2 code
already on main), and deterministic compression.

**Solana**: the host-chain contract reduces to (a) handle formation mixes
per-formation entropy — already the RFC 010 scheme — and (b) the coprocessor
reads the consuming op's output handle from the event, which it does on every
chain by construction. No per-chain fork-tracking/re-randomization framework;
the value layer is chain-model-agnostic.

## Migration sketch

Byte-incompatible: coordinated activation per L1 start block via the existing
blue/green machinery (unanimity gate, pre-activation drain, golden-vector KAT
pinning the new seed derivation). Host contracts are unchanged — `handle_out`
is already emitted with every compute event; the change is coprocessor-side
seed derivation only. RFC 019 would be amended; RFCs 011/020/023 simplify in
their next revisions.
