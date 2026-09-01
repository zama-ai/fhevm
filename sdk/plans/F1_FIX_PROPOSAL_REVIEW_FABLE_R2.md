# Fable review round 2 — revised F1_FIX_PROPOSAL.md

Independent review (Fable 5) of the revision, checking both whether round 1's findings were genuinely
closed and whether the rewrite introduced new problems. Round 1 is in
`F1_FIX_PROPOSAL_REVIEW_FABLE.md`.

## Verdict

**Adopt with specific changes.** 8 of the 10 prior findings are closed in mechanism, not just prose.
What remains is text-level: one internal contradiction that makes a feature inert, one new
silent-ignore path, and two unstated conditions the parity snapshot's defence depends on. Nothing
requires redesign.

## Closure of round 1

| #   | Finding                  | Status                                                                                                 |
| --- | ------------------------ | ------------------------------------------------------------------------------------------------------ |
| 1   | Exit-status propagation  | **Closed** — the sub-make is its own recipe line, so its status reaches Make regardless of shell flags |
| 2   | Eligibility predicate    | **Closed**, one new residue (see finding 2 below)                                                      |
| 3   | Gate scope               | **Closed** — repo-wide, with `-- .` rejected and the reasoning recorded                                |
| 4   | Dirty-tree middle option | **Closed as an explicit decision** — fingerprint mode named and rejected                               |
| 5   | Single-command rule      | **Closed as design**, implementation-choice nuance below                                               |
| 6   | Ordinal semantics        | **Closed on all three sub-points, but the fix introduced a contradiction**                             |
| 7   | Manual-command rule      | **Closed** — downgraded to a documented convention, criteria now per-known-name                        |
| 8   | Renames                  | **Closed**                                                                                             |
| 9   | Templates-split order    | **Partially** — post-phase recorded, pre-phase reorder not                                             |
| 10  | Boundaries               | **Closed** — js-sdk and `sync-common-vendored` both named                                              |

The revision also correctly avoids leaning on round 1's withdrawn "live silent-pass bug" claim.

## Findings

### 1. MEDIUM-HIGH — the sort key makes ordinals inert

**Verified by reading the document.** Runner rule 5 sorts "by **effective** generator name and then
package path". The effective name is defined as "the generator name **after removing its optional
ordinal**". Sorting on the ordinal-stripped name therefore discards the ordering the ordinal exists to
express: `generate:pre:05-seed` and `generate:pre:10-config` sort as `seed` vs `config`, so `config`
runs first — inverting a declared dependency silently, deterministically, forever.

The two sentences cannot both hold. The ordinal paragraph clearly intends full-name lexicographic order
— that is why fixed width is mandated. **Fix:** sort by the full script name; use effective identity
_only_ for duplicate rejection. A one-sentence edit, but as written an implementer following rule 5
literally builds a runner whose ordering feature does not work.

### 2. MEDIUM — ineligible packages become invisible, and most of them should be forbidden zones

**Verified against the code.** The validator rules apply "only [to] packages accepted by the shared
eligibility predicate". So a `generate:pre:foo` added to the **workspace root**, a **standalone**
test-consumer fixture, or a **non-package** tree (`./scripts`, `pkg/ts`) is neither executed nor
flagged — the exact "added but never runs" failure F1 exists to kill, recreated one directory up.

Nothing else catches it: rule 2.1.2 fires only for `kind === 'published' && isNpmDistributed`, so it
covers v12/pkg and v13/pkg but leaves root, standalone and non-package scripts unpoliced.

Only mirror-only packages have a legitimate claim to invisibility, their bytes being upstream's.
**Fix:** the validator must reject any `generate:*`-matching script on any manifest-listed package
_outside_ the eligible set, mirror-only exempted. Turns "ignored" into "loud" for one rule's cost.

### 3. MEDIUM — the snapshot's "fails loudly" defence rests on two unstated conditions

The defence holds only if:

- **(a)** the comparison is **exact set equality** — a subset check lets additions slide, and a
  snapshot test that regenerates its own golden file in CI checks nothing;
- **(b)** **no execution path ever reads the snapshot.**

(b) is the dangerous one. The foreseeable bad refactor is a developer facing a red snapshot check who
"fixes" it by filtering the runner's selection to the snapshot — at which point it becomes precisely the
hand-maintained execution list F1 removes, and staleness becomes a silent skip. Both conditions are one
sentence each, plus a criterion: _`run-generators` contains no code path that reads the parity
snapshot._

### 4. LOW-MEDIUM — the tokenizer fails in the wrong direction

The single-command rule offers "a small shell-command tokenizer **or** a strict allowed-command
grammar". A tokenizer's bugs _accept_ what they mis-tokenize — a MISS, the forbidden direction. A strict
grammar can only over-reject, which is loud and trivially fixed by rephrasing. Every post-migration
command is a bare `node <path> [args]` with no quoted operators, so the fail-closed variant costs
nothing today. **Commit to the grammar and drop the tokenizer option** — less code, safer failure
direction, and it discharges some complexity budget.

### 5. LOW — the pre-phase reorder is real and unrecorded

Discovery moves `exports` from first to fourth. Verified safe — pre-phase generators read committed
templates, manifests and contract sources, not each other's outputs — but step 3 records only the
post-phase verification. Add the symmetric sentence.

### 6. LOW — near-miss ordinals parse silently as plain names

"Exactly two decimal digits followed by `-`" does not say what happens to `100-foo` or `1-foo`. If they
parse as ordinal-less names containing a dash, an author who intended ordering gets no error and no
guarantee. Reject any name matching `^\d+-` that is not exactly `^\d{2}-`. One regex.

### 7. LOW — two residues worth a sentence each

- A package whose only namespace scripts are `accept:*`/`refresh:*` escapes the derived
  `clean:generated` requirement. Correct behaviour — those outputs must survive `clean-generated` — but
  state it.
- "Same phase selection implementation" is a code-review assertion. A fixture test that `--list` output
  equals the executed set would make divergence mechanically checked rather than structurally assumed.

## The permanent parity snapshot — the decision under question

**Keep it permanent**, conditional on finding 3's two hardening sentences.

**Can a stale snapshot only fail loudly?** Yes, under exact set equality — both an unapproved addition
and an unapproved disappearance diff against the pin. But be clear what it does _not_ catch: the
original F1 failure itself. A generator discovery fails to select is absent from selection _and_ from
the snapshot the developer writes by looking at selection; the two agree in their shared blindness. The
snapshot is not a coverage gate. It is a **regression witness**.

**And that is why permanence is justified — by an argument the proposal does not make.** The proposal
mandates that validator and runner share one parser and one eligibility predicate. Right for
consistency, but it eliminates independent cross-checking: a bug or careless refactor in the shared
predicate blinds both at once. Verified plausible against the manifest: someone flips a version
package's kind, or narrows the predicate while adding a sixth kind — eight generators vanish from
selection, the validator (same predicate) sees nothing, and `check-generated` goes green because the
committed output simply stops being re-verified. The permanent snapshot is the **only** mechanism in the
design that fails loudly at that moment. A migration-only snapshot protects against exactly one selector
bug — the one present during migration week — and is deleted before the refactor that needs it.

**The rubber-stamping developer** is strictly better off than under original F1: execution is wired by
discovery, not by the snapshot, so the generator runs regardless of what they paste in. Mindless
updating degrades the snapshot from approval gate to ceremony — but a ceremonial regression witness
still witnesses regressions. Worst case is wasted ritual, never a skipped step; F1's worst case is
silence. Add one mitigation: the mismatch error must say what the file is and what approving means.

**Principle 3:** the permitted asymmetric case, not a violation — a declared list whose staleness fails
loudly in both directions and which no execution path consults. Finding 3's conditions are what keep it
in the permitted category.

**Principle 4:** a golden-file set-equality test is among the cheapest mechanisms here to understand.
The namespace answers "what runs"; the snapshot answers "did what-runs change without a human
noticing". Different questions, one sentence each.

## Complexity budget

Still holdable in one head. The runtime model is four sentences: scripts named `generate:pre/post:*` in
dev/helper/consumer packages run, sorted by name; everything is gated on a clean repo; a golden file
pins the set. Most of the growth is _specification of edge behaviour_, not mechanism.

Earns its place: the namespace and two phases; the eligibility predicate (one line over kinds that
already exist); the repo-wide clean gate (simpler than what it replaces); the parity snapshot; the
13-step migration (long, but every step fails loudly and parity-before-deletion is the right order).

Does not, as specified: **the tokenizer** (finding 4). **Ordinals** are marginal — no generator needs one
today, and the feature has already cost the proposal its one internal contradiction. Keep only with the
sort key fixed; cutting to "all generators in a phase must be independent; there is no ordering
mechanism yet" would also be defensible.

## New silent-pass paths

- Ineligible-package invisibility — the one real new instance (finding 2).
- Ordinal de-duplication — not silent; the rule rejects loudly. The silent problem is the sort key.
- `--list` vs execution divergence — structurally prevented by the shared-selector mandate; make it
  tested (finding 7).
- Snapshot × eligibility — the interaction _is_ the snapshot's value, provided finding 3's conditions
  are stated.

## Verification criteria

Materially improved: `make -n` now names the sub-targets, the failing-generator criterion exists, and
the no-git-objects claim is honestly labelled "code review confirms" rather than masquerading as a gate.
Still not mechanical: "use the same phase selection implementation", and the manual-command criteria are
per-name only — which the proposal now acknowledges. Nothing is falsely claimed as verifiable.
