# `sdk/` workspace architecture

Status: **the decided shape**, as of 2026-08-27. This document states _what_ the architecture is and
which invariants hold it together. It does not re-derive anything — the reasoning, the measurements
and the rejected alternatives live in the plans listed in §10.

Three questions remain open; they are in §9 and none of them changes the shape below.

---

## 0. Golden rule

> **NEVER EDIT `host-contracts-cleartext/v*/pkg/src/contracts/` BY HAND.**
> Not v12, not v13, not any future generation. No hand edits, no lint autofix, no import reordering
> — not even when a tool reports those files as non-compliant.
>
> **`forge fmt` is the single exception**, and only because the files are _stored_ forge-formatted,
> which makes it a no-op on them. Every other tool is still forbidden.

They are vendored from `host-contracts` at the commit named in
`pkg/package.json → fhevm.vendoredFrom`, and must stay identical to **`forge fmt` of** it (RULES.md
rule 6, gated by `sdk/scripts/check-vendored-sources.sh`). **Their content is upstream's decision,
not this repo's** — only their formatting is normalised, and only by forge.

Three things to know about the gate:

- Until 2026-08-27 it was **wired into nothing** — defined as a script, run manually, absent from
  `build`, `test` and CI. The golden rule had no automatic enforcement. It now runs in `build` in
  both packages, immediately after `clean`, alongside `check:forge-version`.
- The comparison normalises **only the upstream side**. The vendored file is compared exactly as
  stored, so reformatting one by any other means fails the gate rather than being absorbed.
- `.foundry-version` is load-bearing: the gate's expectation is a function of `forge fmt` output, so
  a forge upgrade changes what rule 6 means. `check-forge-version.sh` runs first so that shows up as
  a version mismatch rather than as N mysterious drifts.

Why the rule is stated this way rather than as a byte compare: upstream formats with
`prettier-plugin-solidity`, this workspace formats with `forge`, and the two cannot be reconciled by
configuration (measured, `FORGE_FMT_MIGRATION_PLAN.md` §1.1). A byte compare forced the directory to
be excluded from `forge fmt` — which left two styles in one tree _and_ left the files exposed, since
the VS Code formatter pipes buffer text with no path and so cannot honour `[fmt] ignore`. Storing
them forge-formatted makes an editor save a **no-op**: a stronger guarantee than the exclusion it
replaced.

The operational form of the rule: **if a tool other than `forge fmt` wants to change a file in there,
exclude the tool — never touch the file.** Adopting an upstream change is a deliberate re-vendor,
never an edit.

Note `pkg/src/contracts` is deliberately **absent** from `[fmt] ignore`; only
`test/ts/node_modules` is listed, because that is an installed tarball fixture. Excluding the
vendored directory would re-create the very problem this design removed — and if it is ever
re-added, use the **bare directory name**: `"pkg/src/contracts/**"` looks right and silently fails,
since `**` requires an intervening directory and these files sit directly in that folder.

`files.readonlyInclude` on the vendored tree is no longer load-bearing — an editor save is a no-op
now — but it remains worth setting as defence in depth, since it still blocks stray typing and
quick-fixes, which the formatter change does not address.

---

## 1. The workspace

`sdk/` is an autonomous npm workspace sitting **outside** the root `fhevm` workspace.

```
fhevm/package.json          workspaces: host-contracts, library-solidity, …, sdk/js-sdk, sdk/js-sdk/src
fhevm/sdk/package.json      workspaces: host-contracts-cleartext/v12, /v13, /v13/pkg
```

The root list is explicit paths, never a glob over `sdk/*`. A member belonging to two workspaces
would hoist out of this one's toolchain and silently pick up different tool versions.

**npm, not pnpm.** pnpm accepts duplicate workspace names and silently links the wrong version
(measured: both `workspace:…@0.12.0` and `@0.13.0` resolved to `../v13/pkg`), where npm refuses
loudly with `EDUPLICATEWORKSPACE`. Once names are unique both work, so there is no reason to add a
second package manager. pnpm's strict `node_modules` is a real argument, but it belongs to its own
decision, not to this one.

`js-sdk` and `hardhat-plugin/v2|v3` are **not** members yet: `js-sdk` is still a root-workspace
member and cannot belong to two, and the plugins depend on it. They join at Step 2 of
`PLAN_NPM_SKELETON.md`.

## 2. Version identity

Each generation is a pair: a private harness and the payload published to npm.

| directory  | name                                      | version  | published? | workspace member? |
| ---------- | ----------------------------------------- | -------- | ---------- | ----------------- |
| `v12/`     | `@fhevm/host-contracts-cleartext-dev-v12` | `0.12.0` | no         | **yes**           |
| `v12/pkg/` | `@fhevm/host-contracts-cleartext`         | `0.12.0` | yes        | **no**            |
| `v13/`     | `@fhevm/host-contracts-cleartext-dev-v13` | `0.13.0` | no         | **yes**           |
| `v13/pkg/` | `@fhevm/host-contracts-cleartext`         | `0.13.0` | yes        | **yes**           |

A harness carries a version even though it is never published, and each harness version equals its
own payload's. v13's edge onto v12 is then an **exact pin**, `"0.12.0"` — not `*`, not a caret:

```
harness=<no version>  "*"        LINKS      ← rejected: matches anything
harness=0.12.0        "^0.12.0"  LINKS      ← rejected: also accepts 0.12.9 silently
harness=0.12.0        "0.12.0"   LINKS      ← chosen
harness=0.12.1        "0.12.0"   FAILS      ← the point: a bump must be acknowledged
```

The cost is real and accepted: bumping v12 breaks `npm install` until v13's pin is updated too. That
failure is **loud** — npm stops treating the dependency as a workspace member and looks for the
unmatched version on the registry, where a private harness has never been published, so it 404s. A
loud failure on an un-acknowledged generation bump is the behaviour being bought.

**Only the current generation's `pkg/` is a member.** Every `pkg/` shares one published name, so
listing two of them is what npm rejects. Older generations are still reachable, through their
harness:

```
@fhevm/host-contracts-cleartext-dev-v12/pkg/ts/…      TypeScript
@fhevm/host-contracts-cleartext-dev-v12/pkg/src/…     Solidity
```

This works because package `exports` are opt-in and **the harness manifests declare none**. That is
not an accident to be tidied up later; it is the mechanism. See invariant I2.

Rolling forward to v14 means: drop `v13/pkg` from the member list, add `v14` and `v14/pkg`, and
repoint v14's test edge at v13.

## 3. TypeScript project structure

**`tsconfig.json` is the IDE's entry point and nothing else.** Every task names its own
`tsconfig.<task>.json`, and no npm script may name `tsconfig.json`.

Per package:

| file                                                | role                                                                                        |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| `tsconfig.base.json`                                | shared compilerOptions; not an entry point                                                  |
| `tsconfig.json`                                     | **IDE only** — solution, `files: []`, references every task leaf                            |
| `pkg/ts/tsconfig.types.json`                        | composite, `emitDeclarationOnly` → `_types`. **The single cross-package reference target.** |
| `tsconfig.build.esm.json` / `.cjs.json`             | JS emitters → `_esm` / `_cjs`; non-composite, outside the reference graph                   |
| `{internal,test,create2-deploy}/tsconfig.lint.json` | composite, `noEmit`                                                                         |

Two rules govern the graph:

- **Build order comes from the reference edges, never from a list.** `tsc -b` topologically sorts;
  a root that names only v13 still builds v12 first.
- **A referenced project must emit, and to the exact path its published `exports.types` names.**
  Emitting anywhere else fails at resolution, not at build — the consumer resolves through
  `exports.types`, which the graph then never produces.

`lint` remains a fail-early type-check, but `--noEmit` is not how it gets there: that flag
propagates into the referenced project and re-triggers `TS6310`. The guarantee comes from
`noEmitOnError: true` in `tsconfig.base.json` instead — on error, nothing is written. As a
side-effect `lint` now memoises its work for `build` rather than discarding it.

## 4. Dependency graph

```
v12/pkg/ts ──► v13/test          (upgrade test only)
                  │
   future:        └──► js-sdk ──► hardhat-plugin/v2
                          └─────► hardhat-plugin/v3
```

The cross-generation edge lives on **`test`**, never on `pkg/ts`. The published payload must not
depend on the previous generation — only the test that exercises upgrading does. See invariant I5.

Solidity is the primary artifact and TypeScript is secondary (rule 26 of the skeleton plan); the
Solidity side reaches across generations by path through the same harness subpath, and needs no
project references.

## 5. Publish surface

- **No tsconfig is ever published.** `"!**/tsconfig*.json"` and `"!**/*.tsbuildinfo"` in
  `pkg/package.json`'s `files`; verified absent from a real `npm pack` of 328 files. A tsconfig is a
  build input — nothing a consumer resolves reads one, and a `composite` config in `node_modules`
  invites consumers to couple their build to our directory layout.
- **Sources ship alongside declarations.** 34 `.ts` + 34 `.d.ts` + 34 `.d.ts.map`; the maps point at
  the sources, which is what gives consumers go-to-definition into real code. viem does the same
  (1438 of each).
- `exports` is the entry contract. It is enforced identically for a tarball and a symlink.

## 6. Verification: two mechanisms, two jobs

|                                    | job                                          | catches                                                        |
| ---------------------------------- | -------------------------------------------- | -------------------------------------------------------------- |
| workspace link + project reference | dev loop, typecheck ordering, IDE navigation | cross-version type breakage, as you type                       |
| tarball (`test:tarball:run`)       | publish rehearsal                            | `files` omissions, undeclared deps, stale/missing build output |

**A workspace link is strictly less faithful than a tarball.** A symlink exposes the whole working
directory, so a consumer can import a file that `files` excludes and nothing fails until a user hits
it:

```
symlink:  …/ts/internal-only.js  ->  RESOLVES
tarball:  …/ts/internal-only.js  ->  ERR_MODULE_NOT_FOUND
```

So the link does not replace the tarball — but neither does it need replacing. **Each answers a
different question, and the choice follows from which question is being asked**: use a tarball when
the packed artifact is what is on trial, and a workspace link for everything else. A tarball costs a
build, a pack and an install, and buys publish fidelity; paying that to obtain a sibling's source
code buys nothing.

That settles "where do we pick v(N-1)?" (`PLAN_NPM_SKELETON.md` line 23): **through the workspace
link.** The upgrade e2e needs v12 as a _library_, to stand up the "before" stack — v12's own publish
contract is proven by v12's own `test:tarball:run`. In the same test v13 stays a tarball, because
there the published artifact is exactly what is being rehearsed. Our reach-in path is the least
faithful of the three, because the harness has no `exports` at all — deliberate, and the reason I2
must stay documented rather than merely true.

### 6.1 Every publishable package owns a tarball rehearsal

**Rule: any package published to npm must carry the machinery to test itself _as if already
published_, and that machinery must be part of the normal test run — not a manual step.**

The workspace cannot provide this. `files` is applied by `npm pack`, never by a symlink, so a
workspace link will happily resolve a module that the published artifact does not contain. Nothing
in the type system, the reference graph, `publint` or `attw` catches that: only packing and
installing does. The same pass is also what catches an undeclared dependency that hoisting was
silently satisfying, and build output that is stale or missing at pack time.

Four parts, all of which already exist in v12 and v13 at parity:

| part                                                            | v12 / v13 implementation                                        |
| --------------------------------------------------------------- | --------------------------------------------------------------- |
| a real tarball, in the shared gitignored `sdk/tarballs/`        | `pack:tarball` → `sdk/scripts/pack-tarball.ts`                 |
| a consumer fixture that installs it **by its published name**   | `prepare:tarball-consumer` → `test/ts/node_modules`             |
| tests that run against the fixture, not against source          | `test:tarball:run` → `vitest --config test/ts/vitest.config.ts` |
| a typecheck + lint of the consumer, in consumer resolution mode | `lint:tarball-consumer`                                         |

Two properties matter more than the exact script names:

- **The fixture must import the published name** (`@fhevm/host-contracts-cleartext/ts`), never a
  relative path and never the harness. Importing by published name is what puts `exports` and
  `files` on trial; any other spelling tests nothing.
- **`build` regenerates the fixture** (`build` ends with `prepare:tarball-consumer`, and `clean`
  removes it), so a built tree is never left with an unresolvable `test/ts`.

This machinery covers **this** package's publish contract, and nothing else. It deliberately no longer
stretches to the cross-generation case: the upgrade test imports v12 through the workspace link, because
what it needs there is a library, not a rehearsal of v12's tarball — which v12 already runs itself.

That reframes §9.2 rather than cancelling it. Installing `@fhevm/host-contracts-cleartext@0.12.0` from
the registry would be a _different_ test — "does the artifact a real upgrading user has actually work"
— and it would deserve its own name and its own reason, not a silent upgrade of this one.

### 6.2 Tarballs collect at the workspace root

**`sdk/tarballs/` is the shared output directory**, gitignored, populated by one root script:

```
pack:tarballs  = npm run pack:tarball --workspaces --if-present -- --out-dir "$(npm prefix)/tarballs"
clean:tarballs = rm -rf ./tarballs
```

`npm prefix` is a documented command that prints the local prefix — the nearest ancestor directory
containing a `package.json` or `node_modules`, i.e. this workspace root. The path it builds is
absolute, so it is correct regardless of how deeply a member is nested —
`host-contracts-cleartext/v13` and a future `hardhat-plugin/v2` need no different path, where a
relative `../../tarballs` would be right for one and wrong for the other. It costs one subprocess
(~180 ms) per run, not per member. npm runs members serially, so nothing races on the directory.

`clean:tarballs` is deliberately relative where `pack:tarballs` is absolute. The two are not
describing the directory inconsistently:

- `pack:tarballs` hands its path to **child** scripts that run in _member_ directories, where
  `tarballs` would resolve to the wrong place. It must be absolute.
- `clean:tarballs` never leaves the root's own cwd, which npm sets to the package root under every
  invocation form tested (plain, `--prefix ..`, `--prefix <abs>`).
- And `rm -rf` is the one place command substitution makes things _worse_: an empty `$(npm prefix)`
  yields `rm -rf "/tarballs"`, whereas a relative path cannot degrade into anything dangerous.

Two rejected alternatives for the pack path, both of which look right and are not:

- **`$npm_config_local_prefix`** carries the identical value, and npm's documented rule that config
  is exposed to scripts as `npm_config_*` makes it look official. But `local-prefix` is internal:
  `npm config get local-prefix` returns `undefined` and it does not appear in `npm config ls -l`.
  It works, and it is an implementation detail. `npm prefix` is the supported way to ask.
- **`$INIT_CWD`** is genuinely documented and is set by npm, yarn and pnpm alike — but it means the
  directory npm was _invoked_ from, which is only incidentally the workspace root. Measured in the
  root script's shell:

  |              | from `sdk/` | from elsewhere, `--prefix ..` |
  | ------------ | ----------- | ----------------------------- |
  | `INIT_CWD`   | `<sdk>`     | `<sdk>/elsewhere`             |
  | `npm prefix` | `<sdk>`     | `<sdk>`                       |

  So `npm --prefix sdk run pack:tarballs` in CI would scatter tarballs into whatever directory the
  job happened to be in. The trap is sharpened by the variable _meaning different things at
  different nesting levels_: a nested `npm run --workspaces` resets `INIT_CWD` to its own cwd, so a
  member script reads the workspace root and looks correct, while the root script that performs the
  expansion does not.

This was not a new hazard, it was an existing one given a proper owner. `v13/tarball/` _was_ already
shared: `prepareTestV12Consumer.ts` packed v12 into **v13's** tarball directory, and carried a
warning not to sweep it because that would delete v13's own tarball. Every generation re-packed its
predecessor into its own tree. With one collection point, each package packs **itself**, once, and
consumers read from there — which is what stops the work multiplying as v14 and v15 arrive.

**Implemented** (§8 item 5). v13 no longer packs v12 at all: it runs v12's own `pack:tarball` with
`--out-dir` pointed at the shared directory and reads back the path the script prints.

Ownership is the rule that makes a shared mutable directory safe, and it is one line:

> Members only **add** to `sdk/tarballs/`. Only the workspace root **clears** it.

A member's own `clean` must therefore stop removing the shared directory. `createPackageTarball.ts`
already accepts `--out-dir` and defaults to the package-local `./tarball`, so a package still packs
standalone when run outside the workspace; the root script is what redirects it.

## 7. Invariants

Breaking any of these breaks the architecture, usually silently.

|        | invariant                                                                                                                                                                        | why                                                                                                                                   |
| ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| **I0** | **Never edit `v*/pkg/src/contracts/` by hand. `forge fmt` is the only tool allowed near it** (§0)                                                                                | stored forge-formatted, so forge fmt is a no-op; every other tool would introduce drift                                               |
| I1     | The root `fhevm` workspace must never glob `sdk/*`                                                                                                                               | double membership hoists members out of this toolchain                                                                                |
| I2     | Harness manifests (`v*/package.json`) must never declare `exports`                                                                                                               | it is what makes `…-dev-vN/pkg/…` reachable                                                                                           |
| I3     | Only the current generation's `pkg/` is a workspace member                                                                                                                       | every `pkg/` shares one published name                                                                                                |
| I4     | No npm script may name `tsconfig.json`                                                                                                                                           | it is the IDE's entry point and nothing else                                                                                          |
| I5     | The cross-generation reference edge lives on `test`, never `pkg/ts`                                                                                                              | the published payload must not depend on v(N-1)                                                                                       |
| I6     | A referenced project emits to exactly the path `exports.types` names                                                                                                             | anything else fails at resolution, not at build                                                                                       |
| I7     | Root tool versions match `sdk/js-sdk` exactly                                                                                                                                    | a workspace hoists one copy; a higher root pin upgrades every member                                                                  |
| I8     | No `type` field at the workspace root                                                                                                                                            | `hardhat-plugin/v2` is commonjs, `v3` is esm                                                                                          |
| I9     | If a Solidity dependency ever hoists to `sdk/node_modules`, `foundry.toml` `libs` must gain `"../../node_modules"`                                                               | forge does not walk up to a parent `node_modules` the way node does                                                                   |
| I10    | Never publish a tsconfig                                                                                                                                                         | build input, and `composite` leaks our layout                                                                                         |
| I11    | Every published package packs a tarball and tests a consumer that imports it **by published name**, as part of the normal test run                                               | `files` is applied by `npm pack`, never by a symlink; nothing else catches an omission                                                |
| I12    | Members only add to `sdk/tarballs/`; only the workspace root clears it                                                                                                           | every generation packs under the same name, so a member's sweep would delete another's tarball                                        |
| I13    | A harness version always equals its own `pkg/` version, and cross-generation edges are exact pins                                                                                | strictest available constraint; a bump then cannot pass silently, it 404s until acknowledged                                          |
| I14    | **`forge lint` is the only Solidity linter. solhint is banned workspace-wide** — no dependency, config, binary or `-disable` comment, enforced by `check:lint-policy` in `build` | two linters means two rule sets disagreeing on one file, and `[lint] exclude_lints` stops being the single place a rule is turned off |

## 8. Immediate to-do

**Done:** the workspace installs. `npm install` at `sdk/` succeeded; `lint` and `build` exit 0 in
both packages; the reach-in resolves (`…-dev-v12/pkg/ts/…` and `…/pkg/src/…`); the exact pin linked
to the local member (`"resolved": "host-contracts-cleartext/v12", "link": true`).

Two predictions in this plan were wrong and are corrected above:

- **`@openzeppelin` did not hoist.** npm placed it per-member (see the lockfile), so forge's existing
  `libs = ["dependencies", "node_modules"]` still resolves it and no `foundry.toml` change was
  needed. I9 is now stated as a contingency rather than a required edit.
- **Prettier needed no reformat.** It did hoist, 3.6.x → 3.9.6 in both members, but
  `prettier --check` passes unchanged — so the "commit the reformat alone" step below is moot.

Remaining:

1. ~~Delete `v12/package-lock.json` and `v13/package-lock.json`.~~ **DONE.** Both were tracked and
   both were stale — each declared `name: "@fhevm/host-contracts-cleartext-dev"`, the pre-split name,
   from before the generations were separated. `sdk/package-lock.json` (`fhevm-sdk-workspace`, 309
   entries) is the one lockfile now. The obsolete `cd ../v12 && npm ci` guidance at
   `prepareTestV12Consumer.ts` and `runUpgradeE2e.ts` now points at `npm install` at the workspace
   root, which is what actually installs v12.
2. Write I2 into `v12/package.json` as a comment — from inside that file the reason is invisible.
   Still open: that manifest mentions `exports` nowhere, so the mechanism that makes
   `…-dev-v12/pkg/ts/…` resolve is invisible exactly where someone would break it.
3. ~~Fix stale text that describes problems already solved.~~ **DONE.** `sdk/package.json` comments 1
   and 5 were already clean; `PLAN_NPM_SKELETON.md` line 63 still claimed "each version is a pair"
   and now records what was actually built — only the current generation lists its `pkg/`, because
   every payload shares one published name (I3).
4. ~~`internal/runUpgradeE2e.ts` **exits 0 when v12 is unavailable**.~~ **DONE**, and not by adding a
   stricter check — by removing the reason a check was needed. The e2e used to build v12, pack it and
   extract it under an alias in `test/ts/node_modules` just to obtain an importable copy, and each of
   those steps could fail into a printed line and `exit 0`. v12 is a **workspace member**, so it is now
   imported like any dependency — `@fhevm/host-contracts-cleartext-dev-v12/pkg/ts/index.ts` — and there
   is no build, pack or install left to fail. Both skip paths, the fixture sentinel and the whole of
   `internal/prepareTestV12Consumer.ts` are deleted; `PREVIOUS_GENERATION_FIXTURE_ALIAS` with them.

   The general rule this follows: **a tarball is for testing a publish contract, not for obtaining a
   package.** v13 is still consumed by its published name in the same test, because there the artifact
   itself is on trial (§6.1).

   It also closes item 2 by accident, and better than the comment that item asked for: the e2e now
   **exercises** I2 rather than documenting it. Verified — adding `"exports"` to v12's harness manifest
   fails the e2e with `"./pkg/ts/index.ts" is not exported … (see exports field in …)`, naming the
   offending file. Restored, and the two e2e tests pass again.
5. ~~Migrate to `sdk/tarballs/` (§6.2).~~ **DONE**, in the stated order:
   - `tarball` removed from both members' `clean` (I12), first, so the shared output survives;
   - `WORKSPACE_TARBALLS_DIR_ABS_PATH` added to `internal/constants.ts` in both generations. It walks
     up to the nearest package.json declaring `workspaces` rather than counting `..` segments — the
     file is copied verbatim into each new generation, and a fixed depth would be wrong the first
     time a package sits anywhere else;
   - `prepareTestTarballConsumer.ts` packs into the shared directory in both generations, and no
     longer passes `clean: true` — sweeping `*.tgz` there would delete the sibling's tarball, which
     is the bug this directory removes;
   - `prepareTestV12Consumer.ts` no longer runs `npm pack` on v12's payload. It invokes **v12's own**
     `pack:tarball --out-dir <shared>` and reads the absolute path the script prints, so each package
     packs itself and v13 only chooses where.

   Verified: clearing `sdk/tarballs/` and running `prepare:tarball-consumer` in both generations
   repopulates it with **both** `.tgz` files — v13's run does not remove v12's — a member `clean`
   leaves it intact, and the upgrade fixture installs from
   `sdk/tarballs/fhevm-host-contracts-cleartext-0.12.0.tgz`. The stale per-member `tarball/`
   directories were deleted; `createPackageTarball`'s package-local default is deliberately kept, so
   a package still packs standalone outside the workspace.

## 9. Open questions

1. **Does `tsc -b` at `sdk/` mean "typecheck the world" or "make the world runnable"?** With
   `emitDeclarationOnly` it produces types but no JS, and the upgrade test _executes_ v12 (vitest
   resolves `exports["./ts"].import` → `_esm/index.js`). Either npm scripts keep producing `_esm`,
   or `outDir` + `sourceMap` fold into the composite project. **The only real design question left.**
2. **Once v12 is published and frozen, should the upgrade test install
   `@fhevm/host-contracts-cleartext@0.12.0` from the registry** rather than packing locally? Higher
   fidelity — it is literally what an upgrading user has — but unavailable until then.
3. **Step 0-bis**: testing TypeScript 7 without disturbing the TS 6 config. Untouched so far.

## 10. Where the derivations live

| document                                  | covers                                                                                                             |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `PLAN_NPM_SKELETON.md`                    | the original constraints, rules and Step 0–8 roadmap                                                               |
| `TSCONFIG_PROJECT_REFERENCES_DECISION.md` | §3 in full: the emit decision, 9 measured facts, the rejected options, the IDE-only config layout, execution order |

Everything asserted as measured in those documents was executed, not inferred.
