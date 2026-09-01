# Fable review — F1_FIX_PROPOSAL.md

Independent review (Fable 5) of `plans/F1_FIX_PROPOSAL.md`, judged against the robustness-over-speed
philosophy. The agent read the proposal, the Makefile, `check-scripts` and its base validator, the
inventory-coverage check, both "manual" generator CLIs, the generator sources, the mirror template's
scripts and gitignore, and swept the tree for textual references to generator names.

## Verdict

**Adopt with specific changes.** The core mechanism — namespace membership discovered from the manifest
inventory, one shared parser for validator and runner, clean-worktree gating — is sound, closes F1's
actual silent-skip path, and sits inside the philosophy. But there is one unspecified decision that
would execute foreign code (mirror packages), one "validator rule" that is not mechanically checkable,
an underdefined ordinal scheme, and an unstated scope decision in the gate. None require redesign; all
require text and one filter.

## Correction to finding 1, verified after the review

The review's headline claim is that the **current** `assert-generation-changed-nothing` silently passes
when generation fails, because the recipe joins everything with `; \` and has "no `set -e`".

**The mechanism is real; the claim that it is live today is wrong.** `Makefile:38` sets
`SHELL := /usr/bin/env bash -eu -o pipefail`, so `-e` _is_ in force and a failing sub-make aborts the
compound command. Verified in an isolated git repo with a generator that exits 7 and writes nothing:

| SHELL                                         | gate exit                         |
| --------------------------------------------- | --------------------------------- |
| `/usr/bin/env bash -eu -o pipefail` (current) | **2** — fails correctly           |
| `/usr/bin/env bash`                           | **0** — the silent pass described |

So the finding downgrades from "live bug" to "one line away from a live bug", and it stays valuable for
that reason: the protection lives on a distant line, and the `.SHELLFLAGS := -c` sitting beside it makes
it _look_ as though the flags live there — they do not, `.SHELLFLAGS` is inert on the GNU Make 3.81 that
macOS ships. Anyone tidying that pair would silently reintroduce the bug. The recommendation stands: add
an explicit verification criterion that **a failing generation phase must fail the generated-output
check even when it changed nothing.**

## Findings, by severity

### 1. HIGH — a failing generation phase must fail the check

See the correction above. The proposal's `assert-clean-generation` fixes this _structurally_: the bare
`$(MAKE) --no-print-directory $(1)` is its own recipe command, so its failure fails the target
regardless of shell flags. The proposal never claims this benefit, and nothing stops a future refactor
folding it back into one shell line. Promote it to a stated criterion — it is the strongest single
argument for migration step 8.

### 2. HIGH — "manifest-listed package inventory" is the wrong eligibility set

`npm-manifest.json` lists `./hardhat/v2/fhevm-hardhat-template/pkg` (`distribution: ["mirror"]`,
`member: true`) whose scripts are upstream's and legally present — rule 2.1.2 only bans scripts on
_npm-distributed_ published packages. Two failure modes:

- if upstream ever ships a conforming `generate:pre:*`, `run-generators` executes upstream-authored code
  inside `make generate`;
- the new "bare `generate:<name>` forbidden" rule deadlocks on a file that must stay byte-for-byte
  identical to upstream.

The existing rule 5.1.4b (`hasGenerateScript` → require `clean:generated`) **already lacks a mirror
exemption** — the same latent deadlock — while every other rule in that file calls
`isMirrorOnly`/`isMirrorOnlyOwner`. Fix: one shared eligibility predicate in the shared parser —
eligible kinds `dev`/`shared-helper`/`internal-consumer`, mirror-only excluded, `standalone` fixtures
excluded — used identically by runner and validator. Asymmetry would be the worst outcome: a runner that
executes what the validator ignores.

### 3. HIGH — the gate's `git status` scope is unstated, and it is the whole monorepo

`sdk/` is a subdirectory of the fhevm repo, so `git status --porcelain` reports dirt anywhere in the
repository. The proposed precondition therefore refuses on unrelated dirt outside `sdk/`. Decide and
state: whole-repo (maximally robust, free in CI) versus `-- .` scoped to sdk (usable locally, but a
scope can only MISS — corollary 3). Recommendation: whole-repo, with the scoped variant explicitly
rejected in a comment so nobody "fixes" it later without seeing the argument.

### 4. MEDIUM — the soundness argument is correct, and there is a middle option the proposal missed

Confirmed: `git status --porcelain` prints ` M path` / `?? path` regardless of content, so a file dirty
before generation and rewritten by it yields identical lines in both captures — false green. The current
Makefile comment praising dirty-tree operation is praising the unsound part.

The missed middle: fingerprint status text **plus content hashes of every dirty and untracked path** via
`git hash-object` _without_ `-w` — pure computation, writes nothing to the object database, so it honours
the proposal's no-git-objects constraint (`git stash create` does create commit objects and ignores
untracked files, so it is rightly out).

The recommendation still lands with the proposal: adopt the clean-tree gate, because it is unbotchable,
and its precondition has an unclaimed bonus — any earlier step that dirties the tree (an install
lifecycle hook writing committed files, say) now fails loudly instead of cancelling out of a diff. On
"developers will skip it": the refusal is loud, CI runs from a clean checkout regardless, so the local
cost is feedback latency, never a lie. If latency hurts, the fingerprint belongs in a _tested_
`fhevm-npm` command, never in shell.

### 5. MEDIUM — the aggregate ban is judgment, not mechanics

`generate:templates` is `node …/generateTemplates.ts && node …/generateSigners.ts && npm run
generate:local-host-bytecode`. A rule scanning for `npm run generate:` catches only the third clause; the
direct `node …/generateSigners.ts` call is invisible to command-text analysis.

Enforceable replacement: **a generator command must be a single command — no `&&`, `;`, `|`, no
`npm run`.** Mechanically checkable, catches every chaining form, and every post-migration generator
already complies. What it still misses — one `.ts` entry importing two generator modules — is the honest,
unclosable boundary the proposal already concedes.

### 6. MEDIUM — the ordinal scheme is acceptable but underdefined where it will bite

- "Sorts deterministically by script name" is lexicographic: `10-x` sorts **before** `9-y`. The runner
  must compare `(numeric ordinal, stripped name)`, or the validator must mandate fixed-width ordinals.
- "Duplicate effective ordering identities" is undefined. Workable definition: identity =
  `(package, phase, ordinal-stripped name)`; reject `10-config` with `20-config` in one package/phase,
  and `config` alongside `10-config`. It must **not** reject the same name across v12 and v13 — that
  duplication is the intended parallel structure.
- State that ordering is name-major then package-path, which preserves the current interleave
  (config v12, config v13, versions v12, …) — verified to match today's recipes.

As a design, name-encoded ordinals beat a declared dependency graph here: visible, greppable, shared by
validator and runner. Wise enough, if defined.

### 7. MEDIUM — the "manual commands cannot use `generate:*`" rule is not validatable, and the inverted hazard is the dangerous one

No validator can know a script is "manual"; it can only ban two known names. The proposal defends one
edge (a generator escaping the namespace) but not the other: a future acceptance command named
`generate:post:accept-foo` would be auto-discovered, auto-run, and would **permanently green its own
drift test** — exactly what `generatePatchSites.ts`'s header warns about. The namespace makes this
_easier_ to do by accident than the old hand-list did. Downgrade the bullet to a stated convention and
mark the matching criterion checkable per-name only.

### 8. LOW — both renames are accurate and lose no coverage

`generate:genesis` writes `pkg/state/genesis.json`, which is **gitignored** — genuinely not a generator
of committed source, so `refresh:genesis` is right. `generate:patch-sites` writes the _committed_
`internal/placeholders/patch-sites.json`, but it is an acceptance baseline whose drift guard is
`test/templates.test.ts`, and `clean:generated` deliberately does not delete it — nothing in the
generate/check/clean loop covered it before, so `accept:patch-sites` is right. Residue worth one
sentence: a package with only `accept:*`/`refresh:*` scripts escapes the derived `clean:generated`
requirement — acceptable, since those outputs must survive `clean-generated`.

### 9. LOW — atomizing `generate:templates` reverses the current order, and that is safe

Alphabetical discovery runs `local-host-bytecode` → `signers` → `templates`; the current chain is the
reverse. Verified independent: `generateSigners.ts` reads only the committed `cleartext-config.ts` (a
pre-phase output), and `generateLocalHostBytecode.ts` builds into its own `--out`, leaving committed
templates and `out/` untouched. No ordinals needed on day one — but step 1 should record this
verification, since it is exactly the unstated dependency the ordinal mechanism exists for.

### 10. LOW — two boundaries to name rather than leave implicit

- `inventory.exclude: ["./js-sdk"]` means a generator added under js-sdk is invisible to discovery,
  `check-scripts` and `check-manifest-coverage` alike. Currently clean; worth one line.
- Makefile-recipe generators remain possible and hand-wired — `sync-vendored` stays a listed call in
  `generate-pre`, fine under corollary 4, but it means the namespace governs npm scripts only.

Holes checked and closed: an unlisted package is caught mechanically (`discoverPackageKeys` finds every
`package.json` outside excluded roots and demands a manifest entry); the hardhat template's `postcompile`
hook regenerates gitignored typechain `types/`, outside F1's committed-output scope.

## Does the namespace actually close F1?

Yes, for its honestly stated perimeter. The load-bearing mechanism is the _pair_: conforming names are
discovered and run with no second edit anywhere, and the bare-`generate:*` ban converts the old failure
mode (script added, Makefile forgotten, silence) into a loud `check-scripts` failure. The remaining
silent path — a generator avoiding the namespace entirely — is unclosable by any tool, and the proposal
says so rather than pretending otherwise. The one _new_ silent path it introduces is finding 7's inverted
case; the one it under-specifies is finding 2's eligibility set.

## F6 deferral

Correct, and the warning is correct: a documentation-only output list is corollary 2's anti-pattern.
One cheap request: design `run-generators` so each generator is invoked through a common wrapper that a
future per-generator output declaration can attach to (a `--list-outputs` contract consumed by both
cleanup and a written-⊆-declared post-check). Design the socket now, do not build the plug.

## Verification criteria that are not checkable as stated

- _"Manual refresh and acceptance commands never run…"_ — checkable only for the two known names.
- _"Generated-output checks create no Git object, stash, commit, tree or ref"_ — nothing observes this;
  it is a code-review assertion, not a gate.
- _"`make -n generate` contains no hand-maintained list"_ — `make -n generate` only prints the sub-make
  invocations, so it is trivially satisfied without proving anything. Needs
  `make -n generate-pre generate-post`.
- **Missing:** a generation phase that fails without dirtying the tree must fail the check (finding 1).

## Highest-risk migration step

**Step 7 — deleting the Makefile generator lists in favour of discovery.** Every other step fails
loudly. Step 7 is the only one whose failure mode is a generator _silently dropped_ by a selector bug or
a too-strict eligibility filter — recreating F1 at the moment of fixing it, with the old inventory
deleted so nothing diffs against it.

De-risk in two moves:

1. Land `run-generators` with a `--list` dry-run and a **parity snapshot test** pinning the exact selected
   set (8 generator names × v12/v13), in a commit _before_ the Makefile lists are removed; compare
   `--list` against the old recipes once, and keep the snapshot. It is a list, but one whose staleness
   fails CI loudly in either direction — the asymmetry corollary 3 permits.
2. After the swap, run `make generate` on a clean tree and require zero diff.

**Second-place risk**, given the demonstrated caller-hunting weakness: step 9's fan-out is **112 textual
references** to `npm run generate:` across `.ts`/`.sh`/`.md`/`.sol`. Two dangerous subclasses:

- runtime error messages in `v{12,13}/scripts/anvil-lib.sh`, `anvil-fast.sh`, `anvil-local-v1.sh` — no
  compiler catches a shell string;
- banners the generators embed into _committed_ output (`pkg/ts/versions.ts`,
  `pkg/ts/cleartext-config.ts`, every `pkg/forge/src/_internal/interfaces/*.sol`). These fix themselves
  only if the banner strings in the generator sources are updated **and** `make generate` re-runs. Miss
  the string and `check-generated` stays green while shipped files instruct users to run nonexistent
  commands.

Final migration gate: `git grep -n 'npm run generate:'` must match only `generate:pre:`/`generate:post:`
names — cheap, exhaustive, and exactly the check that would have caught the earlier missed-callers
incident.
