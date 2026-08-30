# tsconfig hygiene + hardhat/v2 tarball — remediation plan

Two audits run on 2026-08-28 over `sdk/`, node_modules excluded. Reviewed and corrected 2026-08-28.

**Audit A — tsconfig paths.** 42 `tsconfig*.json` files. Eight name paths that no longer exist; six more
name a path from a superseded layout; three more are stale in `js-sdk`. The `test/tsconfig.json` excludes
are a hand-maintained enumeration of a rule, which is why they rot — and `eslint.config.js` carries a
second copy of the same enumeration, already out of sync with the first.

**Audit B — packing.** `sdk/hardhat/v2` is a workspace member with a publishable payload that never packs:
the root `pack:tarballs` uses `--if-present`, and the package defines no `pack:tarball`.

Every step names its verification. Steps are independent **except A4+A5, which must land together**.

---

## Part A — tsconfig hygiene

### A1. ✅ DONE — Drop v12's phantom `test/ts` exclude

`sdk/host-contracts-cleartext/v12/test/ts/tsconfig.json:9` excludes `upgrade-e2e.test.ts`, which does not
exist in v12. Copy-paste leftover from v13.

- **Change:** remove the `exclude` key from the v12 file. Leave v13's alone — it only avoids checking
  `upgrade-e2e.test.ts` twice (that file would typecheck fine there; `tsconfig.e2e.json` owns it, and
  vitest does not read tsconfig `include` at all). Keeping it is right; it is not load-bearing.
- **Verify:** `cd v12 && npm run build && npx tsc -p test/ts/tsconfig.json --noEmit`. The full `build` is
  required, not just `prepare:tarball-consumer`: packing a cleaned tree yields a tarball with no
  `_cjs`/`_esm`/`_types`, and the typecheck then fails on unresolved types.

### A2. ✅ DONE — Fix `tsconfig.e2e.json`'s stale includes

`sdk/host-contracts-cleartext/v13/test/ts/tsconfig.e2e.json` includes five files; three no longer exist.
`utils/anvil.ts` and `utils/ethUtils.ts` moved into `@fhevm/sdk-common-dev`; `utils/viemEthereumLib.ts`
moved into a different package, `@fhevm/sdk-vendored-dev` (`sdk/common-vendored`).

TypeScript ignores a missing `include` entry as long as one entry matches, so `runUpgradeE2e`'s
`tsc --project tsconfig.e2e.json --noEmit` exits 0 today while checking less than intended.

- **Change:** `"include": ["upgrade-e2e.test.ts"]` — nothing more. `tsc` follows imports, so
  `utils/expectedBootstrap.ts` (that file's only relative import, line 30) is checked anyway, and a
  single entry cannot go stale while the project has any inputs at all.
- **Rejected:** `utils/**/*.ts`. It would pull in `utils/deployStack.ts`, a standalone anvil script with
  no e2e role. It typechecks, but the glob buys nothing over following imports.
- **Verify:** `npx tsc -p test/ts/tsconfig.e2e.json --noEmit` passes; then put a deliberate type error in
  `utils/expectedBootstrap.ts` and confirm it **fails** (this is the assertion that the import is
  followed). Restore.

### A3. ✅ DONE — Drop the dead `ts/vitest.config.ts` build excludes

Six files — `tsconfig.build.json`, `tsconfig.build.cjs.json`, `tsconfig.build.esm.json` in **both**
generations — exclude `ts/vitest.config.ts`. That path predates the `pkg/` layout: `include` is
`pkg/ts`, `rootDir` is `./pkg/ts`, and neither `ts/vitest.config.ts` nor `pkg/ts/vitest.config.ts` exists.

- **Change:** remove that one entry from all six. Leave the `**/*.test.ts` / `**/*.bench.ts` globs.
- **Verify:** the emit must stay byte-identical. `git status` **cannot** show this — `pkg/ts/_cjs`,
  `_esm`, `_types` are gitignored (`.gitignore:10-12` in both generations). Instead:
  ```sh
  cp -R pkg/ts/_cjs pkg/ts/_esm pkg/ts/_types /tmp/before-a3/
  npm run build:cjs && npm run build:esm && npm run build:types
  diff -r /tmp/before-a3 <(…)   # must be empty; compare each of the three directories
  ```

### A4 + A5. ✅ DONE — Collapse both enumerations, and wire `test/ts` into `build`

**These are one change. Landing A4 alone opens a coverage hole** — see the warning below.

#### A4. The rule, not the list

Both generations' `test/tsconfig.json` `include` sweeps `test/ts/**`. Those files import
`@fhevm/host-contracts-cleartext/ts`; that specifier always _resolves_ (there is a workspace symlink at
`sdk/node_modules/@fhevm/host-contracts-cleartext` → `host-contracts-cleartext/v13/pkg`), but its
`exports["./ts"].types` points at `ts/_types/index.d.ts`, which `build`'s first step (`clean`) deletes.
So without the excludes, `lint` — and therefore `build` — fails.

A second reason the plan's first draft missed: in **v12**, that bare specifier resolves to **v13's**
payload whenever the local fixture is absent. The excludes prevent v12 typechecking against v13's types.

Current state: v12 lists 10 paths, v13 lists 15. Four name files that no longer exist
(`ts/utils/ethersEthereumLib.ts` in both, plus `viemEthereumLib.ts` and `ethUtils.ts` in v13).

- **Change (both):** `"exclude": ["./ts/**"]`.
- **Safe because:** `test/ts` is its own project (`test/ts/tsconfig.json`, `rootDir: "."`), and its
  `include: ["./*.ts", "./utils/*.ts"]` covers every file the parent covers today — the only
  subdirectories of `test/ts` are `utils/` and `node_modules/`. **No file ends up covered by neither
  project**, provided A5 lands too.
- **Also change — the second enumeration:** `v12/eslint.config.js:7-15` and `v13/eslint.config.js:7-17`
  hand-list the same fixture-dependent files, and are **already out of sync** with the tsconfig list
  (v13's eslint list omits `upgrade-e2e.test.ts` and `node10-cjs-resolution.test.ts`). Collapse it the
  same way, so one rule is stated once. `eslint.config.with-tarball-consumer.js` (present and identical
  in both) is what checks `test/ts` when the fixture exists.

#### A5. `build` must typecheck `test/ts`

`build` → `lint` → `tsc -b ./tsconfig.json`, whose root solution references `pkg/ts`, `test` and
`internal`. `build`'s **last** step, `prepare:tarball-consumer`, creates the fixture — so the check can
simply follow it.

- **Change (both generations' package.json):** append to `build`:
  `… && npm run prepare:tarball-consumer && npm run lint:tarball-consumer`
  then simplify `test:tarball:run` to `npm run build && vitest run --config test/ts/vitest.config.ts`.
- **⚠ Coverage hole if A4 lands alone:** today `lint` (fixture absent) still checks, through the parent
  project, v12 → `ts/node10-cjs-resolution.test.ts`, `ts/utils/expectedBootstrap.ts`,
  `ts/vitest.config.ts`; v13 → those three **plus `ts/vitest.e2e.config.ts`** (four files, not three).
  After A4 and before A5, nothing in `build` checks them, and the "full gate" would pass over the hole.
- **Still not covered, even after A5:** `v13/test/ts/upgrade-e2e.test.ts`, excluded by both projects. Only
  `tsconfig.e2e.json` covers it, run by `test:upgrade-e2e` (`internal/runUpgradeE2e.ts:44`) — not by
  `build`. Either accept that and say so, or add the e2e project to the build chain.
- **Verify:** `npm run build` passes. Then, per generation, confirm the previously-double-checked files
  are still checked exactly once: `npx tsc -p test/ts/tsconfig.json --noEmit --listFiles` must list
  `node10-cjs-resolution.test.ts`, `utils/expectedBootstrap.ts`, `vitest.config.ts`, **and in v13
  `vitest.e2e.config.ts`**. Introduce a type error in one of them and confirm `build` fails; restore.
  Confirm `test:tarball:run` still passes and no longer runs the check twice.

### A6. ✅ DONE (in scope) — Fix the remaining stale entries

The first draft proposed allowlisting these as "defensive build-output names". Two are rot and one is a
typo:

| file                                | entry                        | finding                                          |
| ----------------------------------- | ---------------------------- | ------------------------------------------------ |
| `js-sdk/test/tsconfig.json`         | exclude `standalone`         | directory deleted in `a961e2e3c`                 |
| `js-sdk/src/wasm/tsconfig.esm.json` | exclude `./vitest.config.ts` | no such file                                     |
| `sdk/tsconfig.base.json`            | exclude `tarball`            | the directory is `tarballs` — singular is a typo |

- **Change:** delete the first two; fix the third to `tarballs`.
- **Genuinely defensive, keep:** `sdk/tsconfig.base.json`'s `artifacts`, `cache`, `typechain`,
  `typechain-types`, `out`, `dependencies` — build-output names that may legitimately not exist.
- **Verify:** `npm run lint` in `js-sdk` and at the root still passes.

### A7. ✅ DONE — Guard against re-rot

Everything above is one failure repeated: a path written by hand, in a place nothing checks.

- **Change:** add `sdk/scripts/check-tsconfig-paths.mjs` — parse every `tsconfig*.json` under `sdk/`
  (skipping `node_modules`, `_cjs`, `_esm`, `_types`, `.next`) and fail on any non-glob entry in
  `include` / `exclude` / `files` / `references` / a relative `extends` that does not exist. Needs a
  JSONC-tolerant parser: `sdk/tsconfig.base.json` and both `eslint.config.js` files carry `//` comments.
- **Allowlist:** the six defensive names kept in A6, plus a bare `node_modules` (A4 added one to each
  generation's `test/tsconfig.json`; specifying `exclude` replaces tsc's default, so it has to be
  named explicitly and `test/node_modules` legitimately may not exist). Declare each with a comment.
- **Wire into:** the root `check` script, which `build` already runs.
- **Verify:** zero findings after A1–A6; a finding appears when a path is deliberately renamed.

---

## Part B — hardhat/v2 never packs

`sdk/hardhat/v2` is a workspace member (`workspaces` lists `hardhat/v2` and `hardhat/v2/pkg`) whose
`pkg/package.json` is `@fhevm/hardhat-plugin@0.4.2`, non-private, with
`files: ["src/*","_cjs/*","_types/*","!**/tsconfig*.json","!**/*.tsbuildinfo"]`. It runs
`publint --strict ./pkg && attw --pack ./pkg` — so the payload is validated as publishable, then never
packed.

Proven: `npm run pack:tarballs` at the sdk root produces only the two cleartext tarballs.

Cause: the root script is `npm run pack:tarball --workspaces --if-present -- --out-dir …`. `--if-present`
skips any workspace that does not define `pack:tarball`, and `hardhat/v2` does not. It has no
`internal/cli/` directory at all.

### B1 + B2. ✅ DONE — one shared script, not a copy per package

Implemented 2026-08-28, and **not** as this plan first specified. The plan said to copy the v12/v13 CLI
into `hardhat/v2`; that would have made a third copy of a file already duplicated twice. Instead the CLI
moved out of the subpackages entirely.

- **New:** `sdk/scripts/pack-tarball.ts` — executable, `#!/usr/bin/env node`, matching the convention the
  other shared scripts already use (`check-lint-policy.sh`, `sync-vendored-ts.ts`, …). It calls
  `createPackageTarball` from `@fhevm/sdk-common-dev`, which an `.mjs` can import directly (node 22.20
  type-strips the TS source — verified).
- **Which member is calling** comes from `npm_package_json` — npm sets it to the manifest it is running
  the script for — falling back to `process.cwd()` when run outside npm. `packageDir` defaults to
  `<member>/pkg`, overridable with `--package-dir`; `--out-dir` and `--clean` behave as before, including
  the strict rejection of a bare path.
- **Deleted:** `v12/internal/cli/createPackageTarball.ts`, `v13/internal/cli/createPackageTarball.ts`.
- **Wired (all three members):** `"pack:tarball": "\"$(npm prefix)/scripts/pack-tarball.ts\""`.
  `npm prefix` resolves to the sdk workspace root from inside any member — verified.
- **Also added:** `"build:tarball": "npm run build && npm run pack:tarball"` in `hardhat/v2`. Without it,
  packing a cleaned tree yields a payload whose `_cjs/` and `_types/` do not exist yet (gitignored build
  output, `hardhat/v2/.gitignore:5-6`, deleted by its own `clean`) — a tarball of only `src/*`.
- **Verified:** each member packs on its own; `npm run pack:tarballs` at the root produces **three**
  tarballs including `fhevm-hardhat-plugin-0.4.2.tgz`, whose contents are `src/`, `_cjs/`, `_types/`,
  `package.json` and no harness. `--out-dir` with no value, a bare path, and a missing `--package-dir`
  target each fail with a named error.

### B3. ✅ DONE — Guard: every publishable member must pack

- **Change:** extend A7's script (or add a sibling): for every workspace, if `pkg/package.json` exists and
  is not `private`, require a `pack:tarball` script. Turns `--if-present`'s silence into a named error.
- **Scope confirmed:** the rule catches exactly `v12/pkg`, `v13/pkg` and `hardhat/v2/pkg`. `common/`
  (`@fhevm/sdk-common-dev`) and `common-vendored/` (`@fhevm/sdk-vendored-dev`) are both `private: true` with no
  `pkg/`, so they are correctly excluded.
- **Verify:** passes after B2; fails if `pack:tarball` is removed from any generation.

### B4. (Optional) tarball-consumer fixture for the plugin

The cleartext generations extract their own tarball into `test/ts/node_modules` and typecheck a consumer
against it. That is what catches `files` / `exports` mistakes; publint and attw install nothing. Worth
mirroring, but larger than B1–B3 and not required to make the tarball exist.

---

## Order

1. ~~**A1, A2, A3**~~ — ✅ done 2026-08-28. 10 stale entries removed; 13 remain (4 for A4, 3 for A6, 6 intentional).
2. ~~**A4 + A5 together**~~ — ✅ done 2026-08-28, in one change as required.
3. ~~**A6**~~ — ✅ done 2026-08-28, minus the two js-sdk entries (out of scope; recorded as KNOWN_STALE).
4. ~~**B1 + B2**~~ — ✅ done 2026-08-28.
5. ~~**A7, B3**~~ — ✅ done 2026-08-28: scripts/check-tsconfig-paths.ts and
   scripts/check-pack-scripts.ts, both wired into the root `check` (and so into `build`).

**Full gate after each part:** `cd sdk && npm run build && npm run test`.

## Out of scope

- The `hardhat/v2` consumer fixture (B4) unless explicitly asked for.
- Anything under `pkg/src/contracts`, which is vendored and read-only.
