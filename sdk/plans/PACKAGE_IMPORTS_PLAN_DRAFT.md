# Hardhat plugin build-import methodology

Status: **draft for discussion — no implementation has been approved or started**.

## Decision and stopping point

Implement only two steps:

1. Improve the Hardhat v3 plugin by renaming its existing private build alias from `#esm/*` to
   `#build/*` and proving the complete plugin test topology still works.
2. Once v3 is accepted, port the same build-import intent to the Hardhat v2 plugin, adapted to its
   CJS-only output, and prove both its private build mapping and its existing public Node10 consumer
   contract.

Then stop. Do not change `fhevm-npm`, `npm-manifest.json`, the cleartext packages, js-sdk, CLI packages
or workspace-wide policy. Their possible migrations will be reconsidered later using evidence from
the two Hardhat implementations.

## Context and problem

The Hardhat v3 plugin's tests already execute compiled JavaScript while TypeScript reads emitted
declarations:

```json
{
  "imports": {
    "#esm/*": {
      "types": "./pkg/_types/*",
      "default": "./pkg/_esm/*"
    }
  }
}
```

This is more robust than using `tsconfig.paths` as the runtime explanation. TypeScript paths can make
typechecking pass without creating a Node runtime mapping; `package.json#imports` is understood by
Node and keeps the type and runtime targets together.

The current alias name encodes the output format rather than the test's intent. The stable intent is
“load the compiled build,” so the proposed name is `#build/*`. This remains meaningful if a package's
output layout or supported format changes.

Hardhat v2 produces a different build: CommonJS JavaScript in `pkg/_cjs` and declarations in
`pkg/_types`. Its one existing test validates the published payload's `main` and `types` contract as a
Node10 TypeScript consumer. That public-package test must remain; a private `#build/*` test complements
it but cannot replace it.

## Overarching goal: robustness with flexibility

- **Runtime truth:** private build tests resolve through native package imports, not a typechecker-only
  alias.
- **Fresh builds:** compilation removes complete output trees first, so deleted source modules cannot
  survive as stale JavaScript and pass tests or enter a tarball.
- **Type/runtime agreement:** tests typecheck against emitted declarations and execute emitted
  JavaScript.
- **Format flexibility:** v3 maps to ESM and v2 maps to CJS behind the same semantic `#build/*` name.
- **Consumer honesty:** installed/public behavior continues to be tested through the package name and
  published metadata rather than through the private alias.
- **Limited scope:** establish the mechanism in v3, port it to v2, then stop before attempting global
  policy.

## Benefits

1. Node understands `#build/*` at runtime.
2. Tests do not embed `_esm`, `_cjs` or long relative build paths.
3. TypeScript can select declarations while Node selects JavaScript.
4. A build-directory change is localized to one `package.json` mapping.
5. The private `#` namespace cannot be mistaken for a public package export.
6. The same semantic alias works for packages with different module formats.
7. Native resolution fails immediately when a build target is missing.
8. Clean-before-build prevents removed modules from surviving in output.

## Important boundary

`#build/*` proves a private compiled module tree. It does not prove:

- the published package's `exports`, `main`, `types` or packed file list;
- resolution from an external consumer;
- browser or bundler behavior;
- source-unit behavior before compilation.

The existing consumer and packaging checks remain necessary.

## Build freshness decision

A missing build fails at import resolution, but a stale build normally loads successfully. The plan
therefore chooses **clean-before-build**, not source/output modification-time comparison.

For each plugin, the compile operation used before build-contract tests must remove every emitted
JavaScript and declaration tree before invoking TypeScript. Cleaning cannot rely only on a separate
manual `npm run clean`, because callers and CI may invoke `compile` directly.

Expected behavior:

```text
compile
  -> remove all declared output trees
  -> emit JavaScript
  -> emit declarations
  -> run build-contract tests
```

This makes the output set a fresh function of the current source set. It is cheaper and more reliable
than comparing mtimes.

## Step 1 — improve Hardhat v3

### Current facts

- The `kind: dev` owner is `sdk/hardhat/v3/plugin/package.json`.
- The private mapping belongs there because the importing tests are inside that package scope; it
  does not belong in the published `pkg/package.json`.
- Twenty-three test files import `#esm/*`.
- `test/fixtures/node.config.ts` is the twenty-fourth importer. It is loaded by
  `hardhat node --config` in the child-process tests and must not be missed.
- The build produces ESM JavaScript in `pkg/_esm` and declarations in `pkg/_types`.
- The current v3 compile scripts already remove both output trees before emitting them.

### Change

Replace the mapping with:

```json
{
  "imports": {
    "#build/*": {
      "types": "./pkg/_types/*",
      "default": "./pkg/_esm/*"
    }
  }
}
```

Replace `#esm/` with `#build/` in all 24 importing files. Do not change source-relative imports that
serve another purpose, public package exports, or consumer imports.

### Verification

1. Search the complete v3 plugin tree and prove that no `#esm/` reference remains.
2. Run the v3 compile from clean output and prove that `_esm` and `_types` are recreated.
3. Run TypeScript/lint validation so the `types` condition is exercised.
4. Run all 23 plugin test files so Node resolves `#build/*` to `_esm`.
5. Run the child-process tests that launch `hardhat node --config test/fixtures/node.config.ts`.
6. Run the checked-in ESM consumer test so public-package resolution remains intact.
7. Run the package checks/packing validation already required by the plugin.
8. Inspect the diff and confirm that the change is limited to the alias rename and any necessary test
   expectation updates.

### Acceptance criteria

- Exactly 24 former `#esm/*` importers use `#build/*`.
- TypeScript reads `pkg/_types`; runtime execution reads `pkg/_esm`.
- A missing build fails clearly.
- A removed source module cannot remain in output because compilation cleans first.
- Normal, child-process, consumer and packaging checks pass.
- No public API or published package mapping changes.

Stop after Step 1 and review the result before starting Step 2.

## Step 2 — port the mechanism to Hardhat v2

Start only after Step 1 has been accepted.

### Current facts

- The `kind: dev` owner is `sdk/hardhat/v2/plugin/package.json`.
- It is an ESM test harness around a CJS-only published payload.
- The build produces CommonJS JavaScript in `pkg/_cjs` and declarations in `pkg/_types`.
- `pkg/_cjs/package.json` declares `{"type":"commonjs"}`, so Node interprets the mapped `.js` files
  as CommonJS even though the owner package is `type: module`.
- The existing `node10-cjs-resolution.test.ts` checks the public package's shipped declarations and
  CJS entry. It must not be rewritten to use `#build/*`.
- The v2 compile scripts currently need the same explicit clean-before-emit guarantee as v3.

### Change

Add this mapping to the v2 `kind: dev` owner:

```json
{
  "imports": {
    "#build/*": {
      "types": "./pkg/_types/*",
      "default": "./pkg/_cjs/*"
    }
  }
}
```

Then:

1. Make the v2 compile path remove `pkg/_cjs` and `pkg/_types` before emitting.
2. Preserve creation of `pkg/_cjs/package.json` after the CJS output is emitted.
3. Add one small build-contract test that imports a side-effect-free compiled module through
   `#build/*`. A suitable initial target is `internal/utils/time.js`, tested through a namespace
   import so the ESM harness consumes the mapped CommonJS module explicitly.
4. Keep `node10-cjs-resolution.test.ts` as the independent public-consumer contract test.
5. Do not add `#build/*` to `pkg/package.json`; the alias is private test infrastructure owned by the
   surrounding `kind: dev` package.

The initial test should prove both faces with one import:

```ts
import * as time from '#build/internal/utils/time.js';
```

TypeScript must resolve its declarations from `_types`, while Node must execute `_cjs` and expose the
expected `timestampNow` function to the ESM test harness.

### Verification

1. Run the v2 compile from clean output.
2. Confirm `_cjs`, `_types` and the `_cjs/package.json` format marker are freshly recreated.
3. Run TypeScript/lint validation so the mapping's declaration branch is exercised.
4. Run the new private `#build/*` smoke test.
5. Run the unchanged Node10 consumer-resolution test.
6. Run the v2 consumer and package/packing checks.
7. Demonstrate the freshness guarantee once: create a temporary source module, compile it, remove the
   source, compile again, and verify that its old `_cjs` and `_types` outputs do not remain. Do this in
   a disposable test fixture or with a test-controlled temporary tree, never by leaving repository
   files behind.
8. Inspect the diff and confirm that no v2 public API or published resolution contract changed.

### Acceptance criteria

- The v2 owner exposes one used `#build/*` mapping.
- TypeScript selects `_types` and runtime selects `_cjs` through the same specifier.
- The ESM test harness successfully consumes the mapped CJS module.
- Compile cleans both output trees before emission.
- The existing Node10 public-consumer guarantee still passes independently.
- Consumer and packaging checks pass.
- No public API changes.

## Final stop

After Step 2 is accepted, stop the initiative. Record observations from v3 ESM and v2 CJS, but do not
yet:

- add manifest test profiles;
- add or change an `fhevm-npm` consistency check;
- migrate cleartext v12 or v13;
- migrate js-sdk, CLI or browser tests;
- require `imports` in other packages;
- define a workspace-wide mandatory rollout.

Those decisions require a separate follow-up after both Hardhat mechanisms have been used and
reviewed.
