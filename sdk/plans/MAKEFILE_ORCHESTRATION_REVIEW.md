# Makefile orchestration review

An independent review (Fable 5) of the `sdk/` Makefile against the four principles the orchestration
was built on, plus the fixes that follow from it. Reviewed at the state of branch
`devex/alexb/host-contracts-cleartext-v13`, GNU Make 3.81 on macOS.

## The four principles, and how they scored

| # | Principle                                              | Verdict               |
| - | ------------------------------------------------------ | --------------------- |
| 1 | Every package exposes **atomic** scripts                | ❌ violated           |
| 2 | The **Makefile owns orchestration**                     | ⚠️ partially satisfied |
| 3 | Three verbs: **generate / build / check**               | ❌ violated           |
| 4 | `make ci` does **everything from scratch**              | ⚠️ partially satisfied |

The one unambiguously good result is the part that was hardest to get right: **the cross-package
dependency edges are correct and complete.** Every stamp lists the sources it actually depends on,
in the right direction, and no target succeeds by accident of ordering. Incrementality works — a
second `make build` is a no-op, `make rebuild` is ~51s, `make -j4 lint` gets ~4× speedup.

What is broken is not the graph. It is that several *nodes* do more than they claim, and one node
did nothing at all.

## Findings, by severity

### S1 — `generate-exports` had an empty recipe

`Makefile:418` declared the target and its help text, but carried no recipe. The two
`$(call run,...)` lines were lost when the Makefile was restructured — the same silent collision
that ate the build recipes earlier.

Two consequences, the second worse than the first:

- `make generate` never rendered any export manifest.
- **`check-generated` was vacuous for exports.** It compares tracked files against freshly rendered
  ones; with nothing rendering, it could not fail. The gate was reporting green on an unchecked
  surface.

`make -n generate` printed only `sync-vendored`, which is how it stayed invisible.

**Status: fixed.** Both `generate:exports` calls restored; `make -n generate` now emits three lines.

### S2 — `prune` omitted output directories

`Makefile:76`. `forge lint` writes `e2e/cache-forge` (introduced to dodge the Hardhat/Foundry cache
collision), which was not pruned from the `sources` walk. So `make lint` staled the e2e stamp and
forced a rebuild on the next `make build` — incrementality quietly defeated by an unrelated verb.

**Status: fixed.** Added `cache-forge`, `broadcast`, `coverage`, `dist`, `fhevmTemp`, `types`.

### S3 — generation runs inside `make test`

v12 and v13 both define:

```
test: npm run build:templates && npm run test:templates:run && ...
build:templates: generate:cleartext-config && generate:contract-versions && generate:exports
                 && generate:compute-addresses && generate:placeholders && build:forge
                 && generateTemplates.ts && generateSigners.ts && generate:local-host-bytecode
```

`build:templates` runs every generator and `forge clean && forge build`, rewriting tracked files
that are members of `SRC_V12`/`SRC_V13`. **After `make test`, all six build stamps go stale.**

This contradicts a decision already taken: generation is not part of build. It should not be part of
test either. `make test` is supposed to consume build output, not regenerate its inputs.

**Status: fixed, and the fix was larger than the finding.** Removing `build:templates` from `test`
exposed the real gap: **none of these generators were in `make generate`** — only exports and
vendored were. The generators ran *only* as a side effect of `make test`. So "generation is a
separate flow" had never actually been completed for v12/v13.

The obstacle is that generation straddles a build. `generate:templates`, `generate:signers` and
`generate:local-host-bytecode` read forge artifacts produced by compiling the sources that
`generate:placeholders` and friends write. That two-phase shape is exactly why it had been buried in
a mega-script. The Makefile now owns it:

```
generate: generate-exports → generate-contracts → sync-common-vendored
          → build-cleartext-v12 v13 → generate-templates
```

`check-generated` splits along the same boundary: the pre-build half stays a cheap ci gate, and a
new `check-generated-post` runs inside `check-post`, where artifacts exist.

With `build:templates` no longer called by `test`, its last two callers — `build:tarball` and
`test:templates`, both unreachable aggregates — were removed along with it. `pack:tarball` stays; it
is required by rule 2.1.2. `check-scripts` reports zero violations after the removal.

### S4 — `GENERATED_PATHS` omits most tracked generator outputs

`Makefile:65` lists five paths. The generators write far more tracked, banner-marked files:

| Listed                              | Not listed, but tracked and generated              |
| ----------------------------------- | -------------------------------------------------- |
| `pkg/ts/index.ts`                   | `pkg/ts/versions.ts`                                |
| `test-consumer/`                    | `pkg/abi/` (14 files)                               |
| `pkg/ts/types`                      | `pkg/templates/` (14 files)                         |
| `plugin/pkg/src/internal/vendored`  | `pkg/ts/signers/` (3 files)                         |
|                                     | `pkg/forge/src/_internal/` (19 banner files)        |
|                                     | `pkg/forge/script/ComputeAddresses.s.sol`           |

`check-generated` only guards what this list names, so drift in everything else is invisible to ci.

**Status: fixed.** The list is now split along the same build boundary as `generate`, with
`GENERATED_PRE_PATHS` and `GENERATED_POST_PATHS` feeding `check-generated` and `check-generated-post`
respectively. Every entry was traced back to the generator that writes it rather than guessed:

| Generator                      | Phase | Writes                                                     |
| ------------------------------ | ----- | ---------------------------------------------------------- |
| `generate:exports`             | pre   | `pkg/ts/index.ts`, `test-consumer/`                         |
| `generate:cleartext-config`    | pre   | `pkg/ts/cleartext-config.ts`                                |
| `generate:contract-versions`   | pre   | `pkg/ts/versions.ts`, `_internal/LocalHostVersions.sol`     |
| `generate:compute-addresses`   | pre   | `pkg/forge/script/ComputeAddresses.s.sol`                   |
| `generate:placeholders`        | pre   | `internal/placeholders/addresses.sol`                       |
| `sync-vendored`                | pre   | `plugin/pkg/src/internal/vendored`                          |
| `generateTemplates.ts`         | post  | `pkg/templates/`, `pkg/abi/`                                |
| `generateSigners.ts`           | post  | `pkg/ts/signers/`                                           |
| `generate:local-host-bytecode` | post  | `_internal/LocalHost{Bytecode,Bootstrap}.sol`, `interfaces/` |

`pkg/forge/src/_internal` is listed as a whole directory: all 18 tracked files under it carry the
generated banner. Every path was verified to exist and be tracked in both v12 and v13. Guarded
surface goes from 7 paths to 22.

### S5 — `check-npm-cli-pre-build` is a PHONY order-only prerequisite of the build stamps

`Makefile:239`. Phony targets are always considered out of date, so the 11-command suite re-ran for
every stamp that named it — **4× in a single `make ci`**. Correct, just wasteful.

**Status: fixed.** Converted to a stamp, `$(DIR_STAMPS)/check-pre`, over the inputs that actually
decide its result: every tracked `package.json`, `package-lock.json` and `foundry.toml`, plus
`npm-manifest.json`, the `fhevm-npm` CLI sources that encode the rules, and the vendored sources that
`sync-vendored --check` reads. `check-npm-cli-pre-build` survives as a phony alias so nothing else
had to move. Measured: first `make check-pre` runs the suite, second reports "Nothing to be done",
and `make build` / `make check-post` now invoke it zero times. `clean` drops the stamps, so `ci`
still runs it exactly once, from scratch.

### S6 — `fmt` / `fmt-check` skip `fhevm-npm/` and `scripts/`

Neither directory is covered by any `prettier:*` script the Makefile calls. Five unformatted files
live there today. This is also the direct cause of the current ci failure (below): unformatted
tsconfigs went unnoticed locally because the formatter never looked at that class of file.

**Status: fixed.** The root cause was one level deeper than "no script": every other package has a
`prettier.config.js` re-exporting `prettier.base.mjs`, and **`fhevm-npm/` and `scripts/` had none**.
So even running prettier there by hand used stock defaults (double quotes, width 80) rather than the
workspace style — which is why an ad-hoc `npx prettier --check` reported 12 files while only 6 were
genuinely unformatted. Both directories now have the config.

`fhevm-npm` gained `prettier:check` / `prettier:write` matching every other package. Neither
directory is a workspace member — deliberately, and they stay that way — so `npm run -w` cannot reach
them; the Makefile uses a new `run-prefix` helper for `fhevm-npm`, and formats the nameless `scripts/`
directory with a direct prettier call.

`make fmt` then rewrote exactly 6 files (19 insertions, 27 deletions, all line-rejoining to width
120). `make fmt-check` now passes, which also clears the ci blocker described below.

### S7 — `.SHELLFLAGS` is inert

`Makefile:35` sets `.SHELLFLAGS := -eu -o pipefail -c`. That is a **GNU Make 4.0** feature; this
machine runs **3.81**, which hardcodes `-c`. Recipes are *not* running under `-e -u -o pipefail`.

Not an active bug — every recipe chains with `&&`, which carries failure on its own — but the line
is a false safety claim, and anyone adding a pipeline to a recipe would be relying on protection
that is not there.

**Status: fixed.** Confirmed by isolated experiment on 3.81, both ways: with `.SHELLFLAGS` a failing
pipe let the recipe continue; with the same flags moved onto `SHELL` the recipe aborts. So the flags
now live there —

```make
SHELL := /usr/bin/env bash -eu -o pipefail
.SHELLFLAGS := -c
```

— which is a real behaviour change, not a comment fix: `-u` and `pipefail` are now genuinely in
force. `help`, `fmt-check`, `check-pre`, `graph` and `make -n build` all still pass under it.

### S8 — targets unreachable from any verb

- `check-mirror` (wired as `test:mirror` on the template dev owner) has no Make target.
- The plugin's own `test:consumer` is never invoked.
- `clone-hardhat-template-v2.ts` / `setup-hardhat-template-v2.ts` have no target, and their paths
  likely need re-depthing after the template moved into `pkg/`.

**Status: fixed, and it uncovered a live bug.** The "likely" in that last bullet was real, and worse
than a stale script: **three** places still pointed at the dev-owner parent rather than the mirrored
template, which after the restructure contains only `package.json` and `pkg/`.

| File                                        | Was                          | Now                              |
| ------------------------------------------- | ---------------------------- | -------------------------------- |
| `scripts/clone-hardhat-template-v2.ts`      | `.../fhevm-hardhat-template` | `.../fhevm-hardhat-template/pkg` |
| `scripts/setup-hardhat-template-v2.ts`      | same                         | same                             |
| `plugin/internal/test-consumer.ts`          | same                         | same                             |

The clone script is the sharp one: it *wipes* its target before re-cloning, so pointed at the parent
it would have taken the workspace-owned `package.json` with it. It now keeps a separate
`TEMPLATE_OWNER_REL` for the one place that legitimately means the parent — the `test:mirror` hint it
prints. The plugin's `test:consumer` was not merely unreachable; it would have copied the wrong
directory.

`test:consumer` is now wired into both `test-consumer` and `test-consumer-ci`.

`check-mirror` gets a target, but a deliberately inert one: **the mirror spec is not implemented
yet**, so the real check reports 7 violations that are gaps in the spec rather than real drift. The
target prints a Not-Yet-Implemented warning and exits 0, keeping the work visible without wiring a
known-red gate into ci.

### S9 — non-atomic scripts

Principle 1's actual violations:

- v12/v13 `build:forge` chains `forge clean` **and** `check:contract-sizes` — a clean, a build and a
  check in one script.
- e2e `build` and `lint` **both** run `typechain`, so `make build lint` generates twice.

**Status: fixed.**

`build:forge` is now just `forge build --skip test`. The `forge clean` was redundant — the package's
own `clean` script already removes `cache` and `out`, and dropping it lets forge's dependency cache
do its job. `check:contract-sizes` moved to `check-payload`, where the other payload gates live.

For e2e the duplication was threefold, not twofold: a `$(DIR_STAMPS)/e2e.typechain` stamp already
existed and both `e2e.build` and `lint-hh-v2-e2e` depend on it, so the Makefile was already
orchestrating typechain correctly — the npm scripts were re-running it on top. Removed from both.

Verified: `make rebuild` green in 48s (was ~51s), `make lint` green with **zero** typechain
invocations, `make check-post` green with `check:contract-sizes` running for both generations.

### S10 — `check-generated` was blind to new generated files

Not from the review; found while explaining the gate. `git diff --exit-code` compares **tracked
content only**. A generator that starts emitting a *new* file produces an untracked file, which
`git diff` does not report at all — so the gate passes green and the file is never committed.

Proven directly: dropping an untracked file into a generated path left `git diff --exit-code`
returning 0, while `git status --porcelain` reported it.

**Status: fixed.** The gate now tests `git status --porcelain` over the generated paths and fails if
the output is non-empty, so it catches modified, deleted *and* untracked. It still prints the diff
for readability. Verified both ways: an untracked probe file now fails the gate (exit 2, `??` line),
and the two genuinely stale files fail it (exit 2, ` M` lines).

### Does using git for this gate create the risk of forgetting a commit?

No — it is what makes forgetting *detectable*. Every permutation is caught, because git is the only
thing that knows what was committed versus what the generators produce:

| What gets forgotten                        | What CI sees                                   | Result |
| ------------------------------------------ | ---------------------------------------------- | ------ |
| Manifest committed, generated output not   | new manifest + old output → regenerate → change | caught |
| `make generate` never run                  | old output → regenerate → change                | caught |
| Output committed, manifest not             | old manifest → regenerate → output *reverts*    | caught |

The generated files are never hand-edited — they carry a `DO NOT EDIT` banner. The procedure for a
legitimate change is: edit `export.manifest.json`, run `make generate`, commit both together. Then
regenerating is a no-op and the gate is silent. Something the manifest cannot express goes in a
hand-written file beside the generated one, exactly as `index.cjs` sits beside `export.ts`; only the
generated path is listed in `GENERATED_PATHS`.

### S11 — `runUpgradeE2e.ts` still called the deleted `build:templates`

My own miss while fixing S3: I grepped `package.json`, the `Makefile` and a few directories for
`build:templates`, but not the TypeScript sources. `v13/internal/runUpgradeE2e.ts:37` shelled out to
`npm run build:templates`, so `make test` failed with `Missing script: "build:templates"` — and the
failure was invisible in the log, because `npm run --silent` swallows the error and all 21 vitest
tests above it had passed.

**Status: fixed.** The line is gone; the runner keeps `npm run build` (which writes only to pruned
output directories, so it cannot stale a stamp) and goes straight to the e2e suite. Three
user-facing error messages that told people to run the deleted script now point at `make generate`.
`test:upgrade-e2e` goes from exit 1 to exit 0.

### S12 — `pkg/ts/types` was listed as generated, but is mixed

`sync-vendored -vv` reports it writes exactly **one** file there,
`pkg/ts/types/ethereumLibTypes.ts`. The other two, `public.ts` and `private.ts`, are hand-written.
Listing the whole directory in `GENERATED_PRE_PATHS` meant a legitimate edit to either would fail
`check-generated` — and would have made a naive `clean-generated` destroy hand-written source.

**Status: fixed.** Narrowed to the single vendored file, and `pkg/src/contracts` (the pinned upstream
Solidity, also a `sync-vendored` destination) added, which had been unguarded.

### `clean` does not delete generated code — deliberate, but it left a real gap

Flagged as dangerous. Checking the dependency direction first: forge's `src` is `pkg/src`, which
imports nothing generated, so there is **no** bootstrap circularity — the generators genuinely can
reproduce everything from nothing.

But `clean` still must not delete them. Those files are committed **and shipped** — `pkg/package.json`
lists `abi`, `templates` and `forge` in `files` — and since build no longer generates, a `clean` that
removed them would leave `make clean && make build` emitting an incomplete package.

The underlying instinct was right about something else, though. `check-generated` has a blind spot it
cannot close on its own: **a generator that silently stops emitting a file**. The committed copy is
left untouched, the diff stays clean, and the gate passes.

So there is now a separate `clean-generated` target, deliberately *not* part of `clean`. Deleting
first turns that blind spot into a visible deletion:

```
make clean-generated && make generate    # then expect a spotless `git status`
```

It deletes from `GENERATED_REMOVABLE`, which is deliberately narrower than `GENERATED_PATHS` — a
checking list may safely name mixed directories, a deleting list may not. Excluded for that reason:
`test-consumer/` (hand-written `index.cjs`), `pkg/src/contracts` (vendored from a pinned ref, not
generated here), and `pkg/ts/types` (two hand-written files).

## Why `make ci` currently fails

It stopped at step 2, `fmt-check`, on **uncommitted unformatted v13 `tsconfig.build{,.cjs,.esm}.json`**
— edited for the TS5055 fix and never formatted. Cleared by S6's `make fmt`.

It now stops at step 4, `check-generated`, on the last remaining item: **two regenerated
`test-consumer/cjs/src/export.ts` files (v12, v13) that are modified but uncommitted.** They are
correct generator output — the per-condition `exports` fix let a CommonJS consumer drop the
`require()` + `resolution-mode` workaround for a plain `import`, and the committed copies predate
that. The gate is working exactly as designed.

Staging them would turn the gate green locally and change nothing in real CI, which clones the
committed content and would fail identically. The fix is to commit them.

Everything before and after that gate passes: `fmt-check`, `check-pre`, `check-common-vendored`,
`rebuild` (48s), `lint`, `check-post`.

## The governing principle, stated late and applied backwards

> There is no point optimizing a Makefile to the top. It's ok to build too much stuff. It's not okay to
> miss a build or a step. Better make too much than put yourself in a position to forget to build
> something. Better rebuild than build.

This arrived after several fixes had already been made, and it reverses two of them.

**S5 is reverted.** Stamping `check-npm-cli-pre-build` stopped it re-running four times per ci — but a
stamp is only as good as its input list, and that list could not be right: `check-tsconfig-paths` reads
every `tsconfig.json`, `check-lint-policy` reads the eslint configs, and neither is package metadata. An
incomplete input list means the stamp **skips a check that should have run**. Re-running a cheap check
costs seconds; missing one costs a broken release. It is PHONY again.

**A timestamp-preserving change to `templates.test.ts` is reverted.** That test rewrites
`internal/placeholders/addresses.sol` and the templates, then restores the original bytes — git stays
clean, but the bumped mtimes stale every cleartext build stamp. Restoring the mtimes fixed that, and was
the wrong fix twice over: it optimized away a rebuild that costs little, and worse, it would make Make
believe nothing had changed even if the restore had gone wrong. Staling the stamp is the *correct*
outcome. The next build rebuilds, which is exactly the trade the principle asks for.

**S4's path lists are gone entirely** — see below.

## S4, revisited: no list at all

The hand-maintained `GENERATED_PRE_PATHS` / `GENERATED_POST_PATHS` were the wrong shape, not merely
incomplete. Any list is something a developer can forget to extend when adding a generator, and a
forgotten entry is *silently* unguarded: the gate reports green on output it never looked at. Making the
list longer does not fix that; making it computed per package only moves where the forgetting happens.

`check-generated` now keeps no list. It snapshots `git status --porcelain` before generating and after,
and treats anything newly dirty as drift — whatever it is, wherever it lives. Comparing before against
after, rather than just demanding a clean tree, is what lets it run mid-development: pre-existing changes
appear in both snapshots and are ignored.

Verified on this worktree, which currently carries **108 modified files**: the gate ignored all of them
and reported exactly the two files generation actually changed. With those two restored,
`check-generated-post` exits 0.

`GENERATED_REMOVABLE` survives, because a *delete* list genuinely cannot be inferred: you cannot ask git
which files a generator *would* recreate. That makes it the most dangerous thing in the file — the path
lists could only ever *miss* something, this one can *destroy hand-written source*.

The list has since moved out of the Makefile entirely: each package owns it as a `clean:generated`
script, beside the generators that write those files, which is the only place it stands a chance of
being kept current. `GENERATED_REMOVABLE` is deleted.

That leaves the Makefile unable to inspect the paths, so the per-path "is it tracked?" guard is replaced
by a single stronger one: `clean-generated` refuses unless **`git status --porcelain` is completely
empty**. A spotless tree means everything about to be deleted is committed, so any mistake — including
one in a list this file has never seen — is undone by `git checkout -- .`. It deliberately tests `git
status` rather than `git diff`, because `git diff` ignores untracked files, and an untracked file
deleted is gone for good.

And the obligation is now **enforced rather than remembered**. `check-scripts` gained rule 5.1.4b: any
package defining one or more `generate:*` scripts must also define a non-empty `clean:generated`. The
condition is derived from the package's own scripts, not from a list, so adding a generator to a package
that had none fails the check until it can also be cleaned — the forgetting loop is closed at the point
where the forgetting would happen.

Verified: the guard fires on this 108-file dirty worktree and deletes nothing; the new rule passes with
the script present and reports
`./host-contracts-cleartext/v13: package must define a non-empty 'clean:generated' script` with it
removed; the fhevm-npm suite goes 101 → 103 tests, all passing; `check-pre`, `fmt-check` and
`lint-npm-cli` are green. Documented as 5.1.4b in `npm-rules.md`.

## Fix order

Severity order, most severe first. Each is independently landable.

- [x] **S1** — restore the `generate-exports` recipe *(done)*
- [x] **S2** — prune every output directory from the `sources` walk *(done)*
- [x] **S3** — remove `build:templates` from v12/v13 `test` *(done — larger than scoped, see below)*
- [x] **S4** — ~~extend `GENERATED_PATHS`~~ → replaced by a listless before/after gate *(done)*
- [x] **S5** — ~~stop re-running `check-npm-cli-pre-build` per stamp~~ *(done, then REVERTED: traded safety for speed)*
- [x] **S6** — bring `fhevm-npm/` and `scripts/` under `fmt` / `fmt-check` *(done)*
- [x] **S7** — resolve `.SHELLFLAGS` honestly for Make 3.81 *(done — flags moved to `SHELL`)*
- [x] **S8** — give `check-mirror` and the plugin's `test:consumer` a Make target; re-depth the
      hardhat-template scripts or delete them *(done — re-depthing fixed a live bug)*
- [x] **S9** — split `build:forge`; stop double-running `typechain` in e2e *(done)*

Two items already agreed and still pending, unrelated to this review:

- [x] **S10** — make `check-generated` catch untracked generated files *(done; found after the review)*
- [x] delete `scripts/check-tsconfig-paths.ts` and `scripts/check-dep-versions.ts` (superseded) *(done)*
- [x] **S11** — `runUpgradeE2e.ts` called the deleted `build:templates` *(done; broke `make test`)*
- [x] **S12** — `pkg/ts/types` narrowed to its one generated file; `pkg/src/contracts` added *(done)*
- [x] add `clean-generated` to close the stopped-emitting blind spot *(done)*
- [x] move `check-generated` early in `ci` so it fails in ~5s *(done)*
- [x] document `FHEVM_NPM_ARGS` *(done)*
- [ ] commit the two regenerated `test-consumer/cjs/src/export.ts` files — **the only thing left
      between `make ci` and green**


## F2, F3, F4 — resolved by removing build stamps entirely

Per `plans/F2_FIX_PROPOSAL.md`. The instinct to fix F2 by extending the source-extension list was wrong:
any maintained input list can omit a config file, a root file, a new file type, or a deletion, and then
silently skip a required build. So the stamps are gone rather than repaired.

Every `build-*` target is now phony and owns its recipe directly:

```
build-cleartext-v12 -> build-cleartext-v13 -> build-hh-v2-plugin -> build-hh-v2-template
                                                                 -> typechain-hh-v2-e2e -> build-hh-v2-e2e
```

Make runs a phony target at most once per invocation, so `make build` still builds each package exactly
once, in dependency order. Removed: `DIR_STAMPS`, `SRC_*`, `sources`, `prune`, every `.make/*` rule, and
`clean-stamps`. The `.make` directory is deleted.

This closed three findings at once:

- **F2** — no input list exists to be incomplete. `foundry.toml`, `remappings.txt`, `soldeer.lock`,
  the root `tsconfig.base.json`/`foundry.base.toml`, the `.template` generator inputs and file deletions
  are all covered, because nothing is consulted to decide whether to build.
- **F3** — every `test-*` target now names the `build-*` it consumes, so tests can no longer run against
  stale output. `test-consumer`/`test-consumer-ci` too.
- **F4** — `check-generated-post` depends on `build-cleartext-v13` instead of being correct only by
  `ci`'s sub-make ordering.

`graph` no longer needs a throwaway stamp directory and prints real target names.

**Measured cost of the trade.** A repeat `make build` is **30s**, against ~0s for the old stamped no-op
and ~48s for a full clean rebuild. It is not a full rebuild because forge and tsc keep their own
incremental caches — Make simply always asks them. `make lint` and `make check-post` stay green.

Verification criteria from the proposal, all met: build runs every time; a second unchanged build still
runs; standalone `test-*` and artifact-dependent `check-*` build first; no stamp or extension list
remains; `ci` still begins with `clean`.
