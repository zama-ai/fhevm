# TypeScript project references: the `pkg/ts` emit decision

Status: **decided — Option A (merge)**, confirmed by independent review and measurement. One open
question remains in §8 step 7 (types vs runtime). Scope: `v12/pkg/ts/tsconfig.json`,
`v13/pkg/ts/tsconfig.json`, a new `sdk/tsconfig.json`, and the publish surface of `pkg/`.

Blocks: the `sdk/` project-reference tree (`tsc -b` at `sdk/` compiling v12 → v13 → js-sdk →
hardhat-plugin), which in turn is what removes the v12 tarball fixture from the v13 upgrade test.

## 1. Why a decision is needed

`v13/test` must import v12's TypeScript to deploy a v12 stack before testing the v12→v13 upgrade.
Today that goes through a packed tarball (`internal/runUpgradeE2e.ts`, which shells out to
`cd ../v12 && npm ci`). The npm workspace exists to delete that path. Replacing it with a real
project reference runs into one compiler rule, and resolving that rule forces a choice about which
tsconfig owns declaration emit.

## 2. Measured facts

Everything below was executed, not inferred.

| # | Claim | Evidence |
| --- | --- | --- |
| 1 | A referenced project may not disable emit | `error TS6310: Referenced project '…/v12/pkg/ts/tsconfig.json' may not disable emit.` |
| 2 | A **solution root** (`files: []`) referencing a `noEmit` project is fine | builds clean, no TS6310 |
| 3 | Build order comes from the edges, not the root list | root referenced only v13; `tsc -b` built v12 first |
| 4 | References work across the workspace symlink | `@fhevm/host-contracts-cleartext-v12-dev/pkg/ts/index.js` resolved through a project reference |
| 5 | Incremental works | `up to date because newest input is older than output` |
| 6 | Cross-package breakage is caught | changed v12's signature → `v13/pkg/ts/index.ts(2,51): error TS2554` |
| 7 | `pkg/ts/tsconfig.json` and `tsconfig.build.esm.json` cover an identical file set | no `*.test.ts` / `*.bench.ts` under `pkg/ts` |
| 8 | `tsc -b --noEmit` re-triggers TS6310 once a cross-project reference exists | CLI `--noEmit` propagates into the referenced project and makes it an illegal target again |
| 9 | Option B does not work | same tree, only `declarationDir` differs: `_tsbuild` → `TS2307`, `_types` → clean |

Fact 2 is what keeps this small: `internal/`, `test/` and `create2-deploy/` keep `noEmit: true`
untouched. **Only `pkg/ts` has to start emitting**, because it is the only thing another package
consumes.

Facts 8 and 9 came out of the independent review (§5) and both overturned an earlier assumption
here. They are what decides the question.

## 3. The two options

### Option A — merge

`pkg/ts/tsconfig.json` becomes the composite declaration build: drop `noEmit`, add `declaration`,
`declarationMap`, `emitDeclarationOnly`, `declarationDir: "./_types"`. `build:types` becomes
`tsc -b pkg/ts/tsconfig.json`. One declaration build instead of two.

Cost: `lint` drops `--noEmit`, so a lint run now writes `pkg/ts/_types`.

### Option B — keep both — **disqualified, does not work**

`tsconfig.build.esm.json` stays the declaration emitter. `pkg/ts/tsconfig.json` gets a throwaway
`declarationDir` (`./_tsbuild`) purely to satisfy TS6310. Build scripts stay byte-identical.

This fails outright. `tsc -b` dutifully writes `_tsbuild/index.d.ts`, but the consumer's import
resolves through v12's published `exports["./ts"].types` → `./ts/_types/index.d.ts`, which the
build graph never produces:

```
v13/test/upgrade.test.ts(1,27): error TS2307:
  Cannot find module '@fhevm/host-contracts-cleartext-v12/ts' or its corresponding type declarations.
```

TypeScript's output-to-source redirect only recognises files the referenced tsconfig *declares* as
outputs. `_types` is not one of them, so it is treated as an ordinary file that must already exist.
B therefore only appears to work on a tree where `build:types` has already been run out of band —
which is precisely the orchestration problem the reference tree exists to remove. Worse, `tsc -b`
would then report v12 "up to date" from `_tsbuild` while v13 typechecks against a possibly **stale**
`_types`: silent wrong types, with nothing to detect the divergence.

## 4. Decision

**Option A**, and the choice is not close — B is not a more conservative option, it is a broken one
(fact 9).

The cleanest statement of the evidence is that a single tree, with `declarationDir` as the only
difference, gives `TS2307` at `_tsbuild` and a clean exit at `_types`. The referenced project must
emit to the exact path the published `exports.types` names. That constraint leaves one design.

Fact 7 independently supports it: the two configs already compile the same files with the same
options, so they are one build wearing two hats.

**The "lint writes files" cost was a false objection and is withdrawn.** Fact 8 shows
`tsc -b --noEmit` re-triggers TS6310 the moment `v13/test` references `v12/pkg/ts` — the CLI flag
overrides the referenced project's config and turns it back into an illegal target. So `lint` must
drop `--noEmit` under *either* option; it is a cost of having a reference tree at all, not of A.
Given that, A's single writer to `_types` is strictly better than B's two.

That is also why the standing "never run `tsc` without `--noEmit`" rule does not apply: it exists
because a *solution* file silently exits 0, whereas here `noEmitOnError: true` in `tsconfig.base.json`
means a failing lint cannot publish garbage declarations.

### 4.1 Preserving `lint` as a fail-early type-check

The purpose of `lint`'s `tsc` step is to surface type errors *before* the real build — tsc used as a
linter. That purpose survives; only the command changes.

No flag combination gives a check-only pass across a package boundary:

| command | clean tree | `_types` already on disk |
| --- | --- | --- |
| `tsc -b --noEmit` | TS6310 | TS6310 |
| `tsc -p --noEmit` | **TS2307** | works, catches real errors |
| `tsc -b` | works | works, ~9× faster |

`-p --noEmit` is not a way out: the import resolves through v12's `exports["./ts"].types` →
`_types/index.d.ts`, and a check-only pass will not create the file it needs to read.

`tsc -b` supplies the same guarantee by a different mechanism. With a deliberate error planted in
v12 it reported the error and wrote **no `.d.ts` at all** — `noEmitOnError: true` makes "report, emit
nothing" the behaviour on failure, which is what `--noEmit` was being used for.

It is also cheaper than the status quo. Today `tsc -b --noEmit` discards its work and `build` repeats
it; under A the lint's output *is* `build:types` (measured 829 ms cold → 94 ms warm), so
`clean → … → lint → … → build:types` does the same total work moved earlier.

What is genuinely lost: `lint` is no longer side-effect-free — on a clean tree it writes
`pkg/ts/_types` and a `.tsbuildinfo`. Both are gitignored, removed by `clean`, excluded from the
tarball, and required by the build anyway.

The resulting `lint` script is given in §8, which settles which config it names.

## 5. Independent review

A Fable 5 agent was asked for a fresh, independent pick with no access to this document's
recommendation. It picked **A**, and produced two findings that overturned assumptions here — both
since re-verified independently:

- **Option B does not work** (fact 9). This document had B as merely wasteful; it is broken.
- **`--noEmit` dies under both options** (fact 8), so A's only stated cost was not a cost.

It also rejected two third options worth recording so they are not revisited:

- A separate `tsconfig.refs.json` is not a third option: if it emits anywhere but `_types` it has
  B's fatal flaw, and if it emits to `_types` it is A with an extra file and a duplicate writer.
- "Reference at the package level" does not exist — reference targets must be composite tsconfigs.
  Referencing v12's *solution root* buys build ordering but not module resolution; the import still
  needs `_types` inside the `-b` graph, which is A again.

And it caught a real gap, recorded as §8 step 7 below: `tsc -b` with `emitDeclarationOnly` produces
**types, not runtime**.

## 6. Should the tarball ship a `tsconfig.json`?

**No.** Keep `"!**/tsconfig*.json"` in `pkg/package.json`'s `files`. Verified absent from a real
`npm pack --dry-run` (328 files): no `tsconfig*.json`, no `*.tsbuildinfo`.

A tsconfig is a **build input**. Nothing a consumer resolves ever reads it — `exports` is the entry
contract, and `exports.types` already points at `./ts/_types/index.d.ts`. For a shipped config to be
usable at all it would have to be (a) listed in `exports`, (b) self-contained with no `extends`
reaching outside the package, and (c) free of `include` / `rootDir` / `composite`, all of which
describe our source layout and mean nothing in `node_modules`. Our `pkg/ts/tsconfig.json` fails all
three, and under Option A it would also carry `composite: true` — which actively invites a consumer
to point a project reference into `node_modules` and couple their build to our directory layout.

Note this is **not** an argument against shipping the sources themselves. The tarball ships 34
`.ts` files beside 34 `.d.ts` and 34 `.d.ts.map`; the maps point at those sources, which is what
gives consumers go-to-definition into real code. That is deliberate and stays.

If a shared base is ever wanted, publish it as its own dedicated package (the `@tsconfig/*` pattern),
deliberately exported — never as a by-product of the build config.

## 7. What viem does

viem **does** ship `tsconfig.json` (v2.55.19), and it is an accident, not a pattern to copy.

```json
{
  "extends": "../tsconfig.base.json",
  "include": ["./**/*.ts"],
  "compilerOptions": { "composite": true, "noEmit": true, "types": ["node"] }
}
```

Its `files` field is `["*", …negations]`, and the negation list remembers `!tsconfig.build.json` but
not `tsconfig.json` — the leak is visibly a slip. The published file is unusable twice over:

- not in viem's `exports` map, so it is unreachable by package name (`TS6053`);
- reached by relative path it fails on `TS5083: Cannot read file '…/node_modules/tsconfig.base.json'`
  — the `extends` target is outside the tarball — and then drags viem's own 1438 sources into the
  consumer's compile, producing type errors from inside `node_modules`.

Survey of the installed tree: **2 of 219 packages ship a root `tsconfig.json`**, and both (`viem`,
`@humanfs/types`) leak it via a permissive `files` glob rather than exporting it. Zero publish one
deliberately.

Two things worth taking from viem, though:

- Its internal config is `composite: true` + `noEmit: true` — the exact shape we have, confirming
  the pattern is idiomatic, and that viem has never needed a cross-package reference into it.
- It ships sources + `.d.ts` + `.d.ts.map` in equal number (1438 each), same as we do. Our publish
  approach matches the reference implementation in this space.

## 8. Config layout: `tsconfig.json` is IDE-only

Standing rule: **`tsconfig.json` is the IDE's entry point and nothing else.** Every task — lint,
types, esm, cjs — names its own `tsconfig.<task>.json`, and no npm script may name `tsconfig.json`.

Today `lint` violates this (`tsc -b ./tsconfig.json --noEmit`), and it is the only script that does.
`tsconfig.base.json` is unaffected: it is a shared options fragment, not a task entry point.

Measured: `tsc -b` accepts **multiple project paths**, so the task entry points can be listed
directly in the script and no lint-only solution file is needed. `.tsbuildinfo` is named after its
config (`tsconfig.types.tsbuildinfo`), still covered by `.gitignore`'s `*.tsbuildinfo` and by `clean`.
Also confirmed: eslint binds to no tsconfig (no `project` / `projectService`), so these renames are
safe.

Per package:

| file | role | change |
| --- | --- | --- |
| `tsconfig.base.json` | shared compilerOptions | unchanged |
| `tsconfig.json` | **IDE only** — solution, `files: []`, references every task leaf | named by no script |
| `pkg/ts/tsconfig.types.json` | composite, `emitDeclarationOnly` → `_types` — **the reference target** | renamed from `pkg/ts/tsconfig.json`, `noEmit` dropped |
| `tsconfig.build.esm.json` | JS → `_esm` | unchanged |
| `tsconfig.build.cjs.json` | JS → `_cjs` | unchanged |
| `internal/tsconfig.lint.json` | composite, `noEmit` | renamed from `internal/tsconfig.json` |
| `test/tsconfig.lint.json` | composite, `noEmit`; holds the v12 edge | renamed from `test/tsconfig.json` |
| `create2-deploy/tsconfig.lint.json` | `noEmit`, standalone, no references | renamed |

Scripts:

```
build:types = tsc -b ./pkg/ts/tsconfig.types.json
lint        = eslint
              && tsc -b ./pkg/ts/tsconfig.types.json ./internal/tsconfig.lint.json ./test/tsconfig.lint.json
              && tsc -p ./create2-deploy/tsconfig.lint.json --noEmit
```

`lint` is a superset of `build:types`; running `build` after `lint` finds the types project already
up to date (§4.1). `create2-deploy` keeps `--noEmit` because it has no references and so never
trips TS6310.

At the workspace level the same split applies: `sdk/tsconfig.json` is the IDE solution referencing
the two package IDE solutions, and `sdk/tsconfig.build.json` is the `tsc -b` entry point.

Two consequences to accept deliberately:

- A `.ts` file covered by no referenced project loses IDE support. `test/tsconfig.json`'s existing
  16-entry `exclude` list already has this effect today; the rename does not worsen it, but the
  IDE-only config is the right place to fix it later.
- Adding `create2-deploy` to the IDE solution would require it to be `composite`. Optional, and
  independent of this decision.

## 9. Execution order

Steps 1–3 are the rename pass required by §8 and are worth landing on their own commit, before any
behaviour changes, so the reference work does not arrive mixed with 6 file renames.

1. `v12`: rename `pkg/ts/tsconfig.json` → `pkg/ts/tsconfig.types.json`, `internal/tsconfig.json` →
   `internal/tsconfig.lint.json`, `test/tsconfig.json` → `test/tsconfig.lint.json`,
   `create2-deploy/tsconfig.json` → `create2-deploy/tsconfig.lint.json`. Update the `references` in
   `tsconfig.json` to the new names, and repoint `lint`. No option changes yet; `npm run lint` and
   `npm run build` must pass unchanged.
2. Mirror 1 into `v13`.
3. Confirm `tsconfig.json` is named by no script in either package.
4. `v12/pkg/ts/tsconfig.types.json` — drop `noEmit`; add `declaration`, `declarationMap`,
   `emitDeclarationOnly`, `declarationDir: "./_types"`. **v12 first**: it is the referenced project,
   and the one that must emit. Keep `declarationMap` — `build:types` passes it today and losing it
   costs go-to-definition into sources.
5. `v12` `build:types` → `tsc -b ./pkg/ts/tsconfig.types.json`, and `lint` to the §8 form. Leave
   `tsconfig.build.esm.json` / `.cjs.json` alone: pure JS emitters, non-composite, outside the
   reference graph.
6. Mirror 4 and 5 into `v13` (the two files are byte-identical today).
7. `v13/test/tsconfig.lint.json` — add
   `references: [{ "path": "../../v12/pkg/ts/tsconfig.types.json" }]`. The edge belongs on `test`,
   never on `pkg/ts`: the published payload must not depend on the previous generation. The import
   it serves is `test/ts/upgrade-e2e.test.ts:9`, `@fhevm/host-contracts-cleartext-v12/ts`.
8. `v13/package.json` — add `@fhevm/host-contracts-cleartext-v12-dev` to `devDependencies`.
9. New `sdk/tsconfig.json` (IDE) and `sdk/tsconfig.build.json` (the `tsc -b` entry point). Verify
   the build entry orders v12 before v13.
10. **Decide what `tsc -b` at `sdk/` means.** This is the open question, not a mechanical step: with
   `emitDeclarationOnly`, `tsc -b` gives typecheck ordering and editor navigation but **no runnable
   JavaScript**. The upgrade test *executes* v12, so at runtime vitest resolves
   `exports["./ts"].import` → `./ts/_esm/index.js`, which the composite project never produces.
   Either npm-script orchestration keeps producing `_esm`, or `outDir: "./_esm"` + `sourceMap` fold
   into the composite project so `tsc -b` emits runnable ESM too.

Steps 9–10 additionally depend on the workspace install, which is blocked on the `foundry.toml`
`libs` fix (forge does not walk up to a hoisted `node_modules`; measured).

## 10. The tarball is not replaced — it changes job

An earlier draft of this plan said the workspace link would let us "retire the tarball fixture".
That was wrong. **A workspace link is strictly less faithful than a tarball**, and the difference is
the `files` field. Same package, same `exports`, two install shapes:

```
symlink:  @fhevm/hc/ts/internal-only.js  ->  RESOLVES  ("NEVER PUBLISHED — excluded by files")
tarball:  @fhevm/hc/ts/internal-only.js  ->  ERR_MODULE_NOT_FOUND
```

A symlink exposes the whole working directory, so a consumer can import something that will not
exist once published, and nothing fails until a user hits it. `exports` *is* enforced under both —
verified here and against viem, whose published `tsconfig.json` is unreachable by package name — so
the gap is specifically `files`, plus two more: hoisting can satisfy an **undeclared dependency**
that a real install would not, and a symlink happily serves **stale build output** where `npm pack`
forces a fresh one.

Our reach-in path (`…-dev-v12/pkg/ts/…`) is the least faithful of the three, because the internal
package has no `exports` at all — by design (§ the naming decision), and the reason that must stay a
deliberate, documented choice.

So the two mechanisms have different jobs and both stay:

| | job | catches |
| --- | --- | --- |
| workspace link + project reference | dev loop, typecheck ordering, IDE navigation | cross-version type breakage, as you type |
| tarball (`test:tarball:run`) | publish-contract rehearsal | `files` omissions, undeclared deps, stale or missing build output |

Two consequences:

- The upgrade e2e keeps a publish-shaped path. The project reference makes the **typecheck** fast and
  ordered; it does not license deleting the artifact test.
- Highest fidelity for the v12 side is not a locally packed tarball at all — it is installing the
  real `@fhevm/host-contracts-cleartext@0.12.0` from the registry, which is literally what a user
  upgrading from v12 will have. A local pack tests today's v12 source, not what shipped. Worth
  adopting once v12 is published and frozen.
- Independent defect to fix while here: `internal/runUpgradeE2e.ts` **exits 0 when v12 is
  unavailable**. A publish-contract test that can silently not run is worse than either mechanism;
  it should skip loudly, and fail in CI.
