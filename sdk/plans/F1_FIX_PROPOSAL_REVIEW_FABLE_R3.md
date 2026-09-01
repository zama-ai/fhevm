# Fable review round 3 — F1_FIX_PROPOSAL.md

Third and final review round. Rounds 1 and 2 are in `F1_FIX_PROPOSAL_REVIEW_FABLE.md` and
`F1_FIX_PROPOSAL_REVIEW_FABLE_R2.md`. This round targeted the risk that remains after three rounds of
patch-in-place editing — internal consistency — since that is exactly how round 2's defect arose.

## Verdict

**Ready with 2 small textual fixes**, plus 2 cosmetic ones and one recommended cut. **Nothing
structural is wrong, and a fourth round is not needed.** The document is internally consistent after
the round-2 patches; the reviewer hunted specifically for the round-2 class of defect (a rule fixed in
one place, contradicted in another) and found no remaining instance. A competent developer could build
this from the document with only the two guesses named below.

## Consistency audit

Every sentence read as binding, each rule cross-checked against every other section touching it:

- **Sort key vs effective identity** — the round-2 contradiction site. Rule 5 ("full generator name,
  including any ordinal"), the ordering paragraph ("name-major then package-path-major") and the
  effective-identity paragraph (stripped names used _only_ for duplicate rejection) now all agree.
  **Closed correctly.**
- **Mirror exemption** — stated three times, all carving out the same object. Consistent, with one
  ambiguity (finding 1).
- **Ineligible-package prohibition** — Decision bullet, validator rule and verification criterion name
  the same set.
- **Snapshot** — "regression witness, never an execution inventory" enforced identically across the
  migration text and the criteria.
- **Exit-status propagation** — the prose, the `assert-clean-generation` define and the criterion agree.
- No term carries two meanings; `phase`, `eligible`, `effective identity`, `mirror-only` each have one
  referent throughout.

## Round-2 fixes: correct, not merely present

1. **Sort by full name** — correct. Fixed-width `^\d{2}-` makes lexicographic equal numeric, so the sort
   actually delivers what ordinals promise.
2. **Ineligible non-mirror `generate:*` fails validation** — correct and complete. _Verified against
   code_: the manifest's ineligible kinds are exactly `workspace-root`, `non-package` (`./scripts`, both
   `pkg/ts`), `standalone` (four test-consumers) and `published` (three pkgs); the criterion names every
   one. The single mirror-only package is the only exemption.
3. **Parity snapshot** — exact set equality stated twice, the no-read-path condition stated as both
   design bullet and criterion, CI never regenerates it, and the `--list`-equals-execution fixture test
   closes the last round-2 gap.
4. **Grammar replaces tokenizer** — the tokenizer is now explicitly rejected _with its reason_ (it
   accepts in the unsafe direction). _Verified against code_: all 16 future generator commands in
   v12/v13 are bare `node <entrypoint> [args]`; the only command-substitution script, `pack:tarball`,
   is outside the namespace.
5. **Smaller items** — pre-phase reorder recorded, and the recorded claim _verified against the
   generator sources_: `generateContractVersions` reads committed `pkg/src`,
   `generateComputeAddressesScript` its own template, `generatePlaceholders` computes from constants,
   `copyCleartextConfig` copies the repo config. None reads another pre-generator's output, so the
   alphabetical reorder is genuinely safe. Near-miss ordinal rejection and both residues present.

The "eight automatic generators per version package" parity claim was also _verified against code_: 10
`generate:*` scripts each, minus genesis and patch-sites going manual, with the templates aggregate
splitting into three scripts that already exist individually.

## Findings

### 1. LOW-MEDIUM — the mirror-owner eligibility sentence is ambiguous about the one package it matters for

Principle 4 / implementability. **Verified against the manifest.**

Sentence one: eligible owners are kinds `dev`, `shared-helper`, `internal-consumer`. Sentence two
excludes "mirror-only packages and their upstream-owned payloads". A mirror-only package _is_ the
payload, so read literally the phrase is redundant with the kind rule — but it can equally be read as
excluding the mirror's **owner**, `./hardhat/v2/fhevm-hardhat-template`, which is kind `dev` and
therefore eligible under sentence one.

Confirmed in the manifest:

| Package                                   | kind        | distribution |
| ----------------------------------------- | ----------- | ------------ |
| `./hardhat/v2/fhevm-hardhat-template`     | `dev`       | `["npm"]`    |
| `./hardhat/v2/fhevm-hardhat-template/pkg` | `published` | `["mirror"]` |

The owner's scripts are repo-authored, not upstream's, so it should be eligible. An implementer hits
this exact package and must guess. Failure scenario: they exclude the owner, and a generator added
there later is neither run nor flagged — the F1 hole, one directory up.

**Fix (one sentence):** replace the exclusion list with — "Every package outside those three kinds —
mirror payloads, published payloads, standalone consumer fixtures, non-package entries, the workspace
root — is ineligible; a dev package whose payload is a mirror is itself eligible, because its own
scripts are repo-authored."

### 2. LOW — rule 3's "non-empty" quietly unselects an empty generator

Principle 1. **Reasoning from the document.** "Selects all non-empty scripts" means an empty
`generate:pre:foo` is absent from selection, `--list`, execution and therefore the snapshot — everything
agrees while nothing runs. The validator's empty-forbidden rule catches it in CI, so this is not a
silent CI pass, but a local `make generate` skips it without a word, and rule 4's own philosophy is
"reject rather than silently ignore".

**Fix:** drop "non-empty" from rule 3 and add "an empty command matching the namespace is an error, not
an omission" (or fold into rule 4).

### 3. COSMETIC — duplicate-effective-identity rejection has no verification criterion

The only validator rule absent from the criteria list. Add a line — or delete it along with ordinals.

### 4. COSMETIC — the snapshot's element type is unstated

"Exact set equality" needs a defined element. Step 7 implies (package, full script name) pairs; say so,
or two implementers produce incomparable snapshots — a names-only snapshot would miss a generator moving
between packages.

## Ordinals: cut

**Recommendation: cut, keeping the `^\d+-` rejection.**

- No current generator needs one — verified, and the proposal itself records that verification in steps
  3 and 4.
- The feature produced the document's only inter-section contradiction (round 2) and now costs three
  interlocking rules (fixed width, near-miss rejection, effective-identity dedup) plus finding 3. Pure
  specification weight for a hypothetical; principle 4 exists to constrain exactly this.
- Cutting is fully recoverable and fails loudly on the way back. Keep one rule — _any generator name
  matching `^\d+-` is rejected_ — which reserves the syntax so ordinals can return compatibly the day a
  real dependency appears, and meanwhile nobody can smuggle an ordering assumption in through a
  digit-prefixed name.

Replacement text: "Generators within a phase must be independent; there is no ordering mechanism. Names
beginning with digits-dash are reserved and rejected." That deletes ~15 lines, the dedup rule and
finding 3.

If the author prefers to keep ordinals, the text as written is consistent and implementable — this is a
simplification, not a defect.

## Remaining silent-pass paths

Three, all acknowledged in the document as unclosable by any tool, and all genuinely so: a generator
avoiding the namespace entirely; a single TypeScript entrypoint importing multiple generators; an
acceptance command deliberately named into `generate:*`. Finding 2 is the only _closable_ one left, and
it is one word. The shared-predicate blind spot is covered by the permanent snapshot, the
ineligible-package hole is closed, and the dirty-tree false green is closed by the gate.

## Complexity

The design has not grown since round 2 — the lines have. The runtime model is still four sentences:
namespace plus eligible kinds run, sorted by name; a clean repo gates the checks; a golden file
witnesses the set; a grammar polices commands. The growth is _recorded rejections_ — fingerprint mode,
scoped status, the tokenizer — which are cheap insurance against future "fixes" and worth their lines.
With ordinals cut, the document would shrink for the first time in three rounds.

## The exact edits

1. The mirror-owner eligibility sentence (finding 1).
2. Delete "non-empty" from rule 3; make empty an error (finding 2).
3. One criterion line for dedup, **or** delete with ordinals (finding 3).
4. One clause defining snapshot elements as (package, script name) pairs (finding 4).
5. Recommended: cut ordinals to a reserved-syntax rejection.

None is structural. Fix and implement.
