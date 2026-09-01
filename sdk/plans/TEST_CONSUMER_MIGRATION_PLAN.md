# Test-consumer migration plan (v13)

Move the twelve specs under `host-contracts-cleartext/v13/test/ts` into the `test-consumer/`
fixtures, one at a time, and retire `test/ts` only once every one of them is green in its new home.

## Why move at all

`test/ts` and `test-consumer/` are two implementations of the same idea — run the suite the way a
consumer would. `test-consumer` is the stronger of the two:

|                    | `test/ts`                                     | `test-consumer/{cjs,esm}`                                   |
| ------------------ | --------------------------------------------- | ----------------------------------------------------------- |
| Runs from          | inside the sdk workspace                      | a copy in `os.tmpdir()`, minus `node_modules`/`dist`        |
| Package resolution | tarball installed into `test/ts/node_modules` | `npm ci --install-links` against an absolute `file://` spec |
| Symlink risk       | workspace hoisting can mask a missing dep     | `verifyPhysicalInstallation` rejects a symlinked install    |
| Lockfile           | none                                          | `npm ci` against a real, patched `package-lock.json`        |

`--install-links` makes npm pack and physically install rather than symlink, so `files`, `exports`
and pack contents are exercised exactly as a tarball would exercise them. Running out of tree is the
part `test/ts` cannot match: nothing can silently resolve through the workspace.

## The runner constraint: `node --test`, never a framework

A test-consumer fixture must stay 100% representative of a real consumer scenario. A real consumer
installs the package and writes tests with what Node ships; it does not inherit this workspace's
choice of test framework. **Adding vitest — or any framework — to a fixture makes it stop being a
consumer and start being another copy of our harness.** So the fixtures use `node:test` and
`node:assert/strict` only, and add no test dependency at all.

This is already the workspace pattern elsewhere: `hardhat/v2/plugin` runs
`node --test test/*.test.ts`, and every spec under `fhevm-npm/test` imports from `node:test`.

Two consequences that shape the phases below:

- **Every moved spec must be converted.** All twelve are written against vitest.
- **There is no "green with no specs" baseline.** `node --test` against an empty directory reports
  `fail 1`, so the runner wiring cannot be proven on its own — it lands together with the first spec.

### vitest → node:test conversion

141 `expect()` calls across the twelve specs, using eleven matchers. The mapping is mechanical;
`node:assert/strict` already makes `equal`/`deepEqual` strict, so no `strict*` spelling is needed.

| vitest                                       | node:assert/strict                                                         |
| -------------------------------------------- | -------------------------------------------------------------------------- |
| `import { expect, test } from 'vitest'`      | `import test from 'node:test'` + `import assert from 'node:assert/strict'` |
| `import { afterAll } from 'vitest'`          | `import { after } from 'node:test'`                                        |
| `expect(a).toBe(b)`                          | `assert.equal(a, b)`                                                       |
| `expect(a).toEqual(b)` / `.toStrictEqual(b)` | `assert.deepEqual(a, b)`                                                   |
| `expect(a).not.toBe(b)`                      | `assert.notEqual(a, b)`                                                    |
| `expect(a).toContain(b)`                     | `assert.ok(a.includes(b))`                                                 |
| `expect(a).toBeGreaterThan(b)`               | `assert.ok(a > b)`                                                         |
| `expect(a).toHaveLength(n)`                  | `assert.equal(a.length, n)`                                                |
| `expect(a).toBeDefined()`                    | `assert.notEqual(a, undefined)`                                            |
| `expect(a).toMatch(re)`                      | `assert.match(a, re)`                                                      |
| `expect(a).toBeInstanceOf(C)`                | `assert.ok(a instanceof C)`                                                |

Convert one spec per commit, alongside its move. Never convert in bulk: a silently weakened
assertion is exactly the kind of defect this migration must not introduce.

## Principle: never delete before the replacement is green

`test/ts` stays authoritative for the whole migration. Every step **copies** a spec into a fixture
and leaves the original running. Deletion is the last phase, after all twelve pass in their new home.
Each step is one commit, so a failure is one `git revert` away.

The consequence is a duplication window where the same spec runs twice. That is intended — it is
the parity check — but see **Ports** below.

## Destinations

| Destination         | Specs                                                    |
| ------------------- | -------------------------------------------------------- |
| `test-consumer/esm` | the 10 pure consumers, plus `utils/expectedBootstrap.ts` |
| `test-consumer/cjs` | `node10-cjs-resolution`                                  |
| stays in `test/`    | `create2-precompute`                                     |
| delete outright     | `utils/deployStack.ts`                                   |

`create2-precompute` stays because it derives `PACKAGE_ROOT` from `import.meta.dirname` and spawns a
coordinator that writes into `create2-deploy/`; the fixture is copied to a temp directory, so that
path assumption cannot survive. It becomes a harness test with relative imports — and it keeps
vitest, because `test/` is our harness and is under no consumer constraint.

`utils/deployStack.ts` is dead: its header advertises `node test/ts/deployStack.ts <rpcUrl>`, and no
test, npm script or CI job references it.

## Ports

Every spec owns a disjoint anvil port range — 8600-8602 acl-owner-upgrade, 8610-8618 deploy-v13,
8620-8621 upgrade-e2e, 8630/8631 the kms-context pair, 8634 fhe-rand, 8640 ethers-adapter, 8650
create2-precompute, 8545 tarball-consumer, 8163/8437/8827/8942 precompute-addresses.

No test-consumer parallelism is allowed. Consumer packages, CJS/ESM fixtures and test files all run
one at a time. Every Node consumer suite uses `--test-concurrency=1`, and a spec must finish all
startup, assertions and cleanup before the next spec begins. The disjoint port ranges remain useful
for diagnosis and protection against unrelated processes, but they do not authorize parallel tests.

The two copies of a spec must also never run at the same time: they would fight for the same port.
During the duplication window run `test:consumer` and `test:tarball:run` sequentially, never as
parallel jobs.

---

## Phase 0 — runner wiring, landing with the first spec

Bundled with step 1 of Phase 1, because `node --test` cannot report success on an empty directory.

1. `test-consumer/esm/package.json`: change `test` to
   `npm run typecheck && tsx src/index.ts && node --import tsx --test --test-concurrency=1 test/*.test.ts`.
   **Add no dependency** — `tsx` is already there for `tsx src/index.ts`.
2. Leave `engines.node` at `>=22`. `--import tsx` strips the types, so the runner never depends on
   which Node minor unflagged native type stripping, and one toolchain covers both halves of `test`.
3. `test-consumer/esm/tsconfig.json`: `rootDir` `"src"` → `"."`, and `include` gains `"test/**/*.ts"`.
4. `npm install --install-links` inside `test-consumer/esm` — **the flag is mandatory**, see below.
5. Run `npm run test:consumer` from `v13`.

Keep `src/index.ts` and keep running it. It imports the **root** export
(`@fhevm/host-contracts-cleartext`) while every moved spec imports the **`/ts` subpath** — two
different entries in the `exports` map, both worth covering.

### Regenerating a fixture lockfile

A fixture lock must be generated the way it will be consumed: **`npm install --install-links`**.
Without the flag npm records each local dependency as a symlink stub —
`{"resolved": "file:...", "link": true}` — and files the real package under a separate `"../../pkg"`
key. `patchLocalDependencies` then rewrites the specs to absolute `file://` URLs, those keys stop
matching, and `npm ci --install-links` cannot build a tree from the lock. It reports that as
`EUSAGE: can only install with an existing package-lock.json`, which is badly misleading — the lock is
present and valid, just unusable.

With the flag, entries carry `version`/`resolved`/`dependencies` and no `link`. Compare against
`test-consumer/cjs/package-lock.json`, which has the correct shape, if a lock ever looks wrong.

`patchLocalDependencyLock` also **throws if the lockfile does not already lock a rewritten
dependency**, so any `package.json` edit in a fixture must be followed by that install, or the next
`test:consumer` run fails before it starts.

## Phase 1 — move the ESM specs, one commit each

Ordered simplest first, so the runner is proven before the hard cases arrive.

| #   | Spec                        | New for this step                                              |
| --- | --------------------------- | -------------------------------------------------------------- |
| 1   | `adapter-nonce-diagnostics` | carries the Phase 0 wiring                                     |
| 2   | `fhe-rand`                  | —                                                              |
| 3   | `ethers-adapter`            | —                                                              |
| 4   | `define-kms-context`        | —                                                              |
| 5   | `destroy-kms-context`       | —                                                              |
| 6   | `precompute-addresses`      | multi-port spec                                                |
| 7   | `tarball-consumer`          | first to need `expectedBootstrap.ts` — move the helper with it |
| 8   | `acl-owner-upgrade`         | —                                                              |
| 9   | `deploy-v13`                | largest spec, eight ports                                      |
| 10  | `upgrade-e2e`               | the only `afterAll`; needs the v12 dependency, see below       |

### Per-step checklist

1. Copy the spec to `test-consumer/esm/test/`. **Leave the original in place.**
2. Convert its vitest imports and assertions per the table above.
3. Add any new dependency to that fixture's `package.json`, then `npm install --install-links` there.
4. `npm run test:consumer` from `v13` — the new copy must pass.
5. `npm run test:tarball:run` from `v13` — the original must still pass. Not in parallel with step 4.
6. Diff the two runs' assertion counts before committing. A conversion that drops an assertion still
   goes green.

### Step 10 — upgrade-e2e

The only step that changes a dependency graph. Add to `test-consumer/esm/package.json`:

```jsonc
"@fhevm/host-contracts-cleartext-v12-dev": "file:../../../v12"
```

This works because the v12 harness manifest declares **neither `exports` nor `files`**: npm packs the
whole directory, so `pkg/ts/index.ts` and `pkg/abi/ACL.json` come along, and with no `exports` map the
subpath reach-in still resolves on the installed copy. The `import.meta.resolve(...ACL.json)`
assertion therefore keeps testing invariant I2 exactly as it does today, and the v13 half — the
publish rehearsal, and the only coverage `updateV12ToV13` has anywhere — gets stronger, resolved out
of tree instead of through workspace `node_modules`.

Update the file header when this lands. "Deliberately NOT a tarball … nothing to build, pack or
install first" stops being true: `--install-links` packs v12, so v12 must be built first.

## Phase 2 — the CJS spec

Move `node10-cjs-resolution` to `test-consumer/cjs`, which is where a CJS-resolution test belongs.
It already imports `node:test` today, so it needs no assertion conversion — only the same runner
wiring as Phase 0, applied to the CJS fixture. It reads `TARBALL_DIR_ABS_PATH` from
`@fhevm/sdk-common-dev`, which resolves against the workspace root and will not survive the copy to a
temp directory: pass the directory in by environment variable instead.

## Phase 3 — create2-precompute stays

Move it up to `test/create2-precompute.test.ts` beside the other harness specs and switch its import
of `@fhevm/host-contracts-cleartext/ts` to the relative `../pkg/ts/index.ts`. It is a harness test
now, and saying so in the specifier is more honest than resolving through a fixture that is about to
be deleted. It keeps vitest and joins an existing `test/` vitest config.

## Phase 4 — teardown, only once Phases 1-3 are green

1. Delete `test/ts/` entirely, `utils/deployStack.ts` included.
2. Delete `eslint.config.with-tarball-consumer.js`. Gate rule 5.1.7 already rejects it — that
   violation disappears with the file.
3. Drop from `v13/package.json`: `prepare:tarball-consumer`, `clean:tarball-consumer`,
   `lint:tarball-consumer`, `test:tarball`, `test:tarball:run`, and the `clean:tarball-consumer` leg
   of `clean`. Rewrite `test` and `build` to call `test:consumer` instead.
4. Delete `internal/cli/prepareTestTarballConsumer.ts`.
5. Remove the `test/ts/**` entry from `eslint.config.js` `ignores`, and the now-dangling
   `test/ts` paths in `test/tsconfig.json`.
6. Run `npm run build` in `v13`, then `npm run check-scripts` and `check-package-json` from
   `fhevm-npm` and confirm the v13 rows are gone.

## Then v12

v12 mirrors v13's layout and carries the same `test/ts` fixture and the same forbidden
`eslint.config.with-tarball-consumer.js`. Repeat the whole plan there once v13 is settled and the
shape has proven itself — not in parallel.

## Risks

- **A framework creeping back into a fixture.** The single rule that makes `test-consumer` worth
  having is that it looks like a real consumer. If a spec is hard to express in `node:test`, that is
  a signal it is a harness test and belongs in `test/`, not a reason to add a dependency.
- **Silent assertion loss during conversion.** 141 call sites, converted by hand. Compare assertion
  counts per spec before committing.
- **Port collision between duplicate copies.** Never run `test:consumer` and `test:tarball:run`
  concurrently during Phase 1.
- **Lockfile drift.** `patchLocalDependencyLock` throws on any dependency it cannot find in the
  fixture lock, and a lock generated without `--install-links` is silently unusable. Always
  `npm install --install-links` in the fixture after touching its `package.json`.
- **Path assumptions.** The fixture is copied to a temp directory, so no moved spec may derive a path
  from its own location. `create2-precompute` and `node10-cjs-resolution` are the two known cases;
  re-grep for `import.meta.dirname`, `import.meta.url` and `__dirname` before each move.
- **Build ordering.** `test:consumer` passes `--build-linked-dependencies`, which runs `npm run build` in v13 only. From
  step 10 on, v12 must be built beforehand.
