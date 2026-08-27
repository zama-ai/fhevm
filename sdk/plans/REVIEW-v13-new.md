# Review — `host-contracts-cleartext-v13-new`

Pre-duplication review (before copying this tree for v11 and v12). Reviewed against `RULES.md`
(rules cited by number). All paths are relative to `sdk/host-contracts-cleartext-v13-new/` unless
noted. Date: 2026-08-21.

**Verdict.** The core machinery is sound: vendoring gate passes (21/21 files identical to
`v0.13.2` @ `07fb05fb`), payload manifest is pure (rule 9), zero external imports in `pkg/ts`
(rule 8), zero test files in the payload (rule 14), the placeholder/patch-site pipeline is layered
and careful, and the nonce arithmetic between `addresses.ts` and `deploy.ts` checks out by hand.
But several rules hold **by construction only, with no automated gate**, one npm-consumer path is
broken (`exports` map), the shipped README is a stale design doc, and the tree carries version
markers in ~34 files with no single source — every one of these defects gets multiplied by three
the moment the tree is duplicated. Fix the blockers and the typos first; duplicate second.

---

## 1. Blockers — fix before duplicating

### B1. Rules 15/17: the three fixed local addresses are enforced nowhere automated

The literals (`0x50157…755D`, `0xe3a9…dD24`, `0x901F…b030`) appear only in `scripts/anvil.sh`
(manual-only, not in `npm run test`) and `pkg/README.md`. They are absent from `pkg/ts` entirely:
no exported constants, and `deploy()` never verifies the default local deploy landed on them.
RULES.md rule 17 already names this gap; it is still open. A deploy-order refactor would drift the
addresses and every test would still pass, because all assertions compare against *derived*
addresses (`precomputeAddresses`) — both sides of the comparison move together.

Cheap fix, no anvil needed: a pure unit test asserting
`precomputeAddresses({from: <index-5 address of the fhevm mnemonic>, startNonce: 0n})` yields
exactly the three literals, plus exporting them as constants from `pkg/ts` so `deploy()` can warn
or assert on the chainid-31337 default path.

### B2. `pkg/package.json` `exports` blocks the Solidity payload for exports-honoring resolvers

`exports` lists only `./ts`, `./abi/*.json`, `./templates/*.json` — no `./src/*`, no
`./package.json`. With an `exports` field present, Node resolution refuses every unlisted subpath,
so `import "@fhevm/host-contracts-cleartext/src/contracts/ACL.sol"` fails under any resolver that
honors `exports` (Hardhat 3's resolver does; Hardhat 2's classic resolver and Foundry's
`node_modules` auto-remap do not). For a Solidity-first package this amputates the primary product
on the newest Hardhat. Also broken: `require.resolve('…/package.json')`, which is exactly the
rule-7 tooling read path.

Fix: add `"./src/*": "./src/*"` (or `"./src/*.sol"`) and `"./package.json": "./package.json"`.

### B3. `pkg/README.md` is a stale internal design doc, shipped as the npm README

It describes a layout that doesn't exist (`host-contracts-cleartext/host-contracts/ACL.sol` vs the
real `src/contracts/`), remapping prefixes that match nothing (`@fhevm/host-contracts-addresses-0.13.0/`
— the real prefix is `fhevm-config-0.13.0/`), a "replace this file" plan already executed, and the
stale `encrypted-types/=dependencies/@encrypted-types-0.0.4/` remapping that rule 13 explicitly
says must go. This ships in the tarball (verified via `npm pack --dry-run`). Rewrite as consumer
documentation before duplication — otherwise three stale READMEs get published.

### B4. Rule 16 not satisfied and the harness config claims otherwise

`foundry.toml:2` says "The payload ships its own foundry.toml (pkg/foundry.toml)" — no such file
exists, and the two OpenZeppelin submodules rule 16 requires are also absent, so the future
standalone mirror is not `forge install`-able (10 unresolved imports, per RULES.md's own
measurement). Either add `pkg/foundry.toml` (OZ prefixes only, per rule 16) now, or fix the
comment and track the mirror work explicitly. A stale claim in the config is worse than an open
TODO. (Related good news: the phantom-path `.npmignore` RULES.md rule 16 complains about has been
removed — update RULES.md's status paragraph.)

### B5. `internal/listUpgradeOps.ts:95` — placeholder normalization collides on name length

`const canonical = name.length.toString(16).padStart(40, 'c')` derives the normalized marker from
the placeholder **name's length**, so distinct addresses normalize to identical bytes:
`FHEVM_EXECUTOR_ADDRESS` / `INPUT_VERIFIER_ADDRESS` / `KMS_GENERATION_ADDRESS` (all 22 chars) and
`KMS_VERIFIER_ADDRESS` / `CLEARTEXT_DB_ADDRESS` (both 20). A cross-generation diff that swaps
*which* address a bytecode site references reports `bytecode same` — a false "no op needed"
verdict from the exact tool meant to decide upgrade ops. Since this tool will be used to author
the v11→v12 and v12→v13 migration plans, fix before duplication: derive the canonical value from
the name itself (e.g. keccak of the name, truncated to 20 bytes).

---

## 2. Missing CI gates (rules that currently hold by luck)

None of these fail any script a CI would run (`npm run build` / `npm run test`):

| Rule | Gate that exists | Gap |
| --- | --- | --- |
| 6/7 vendoring | `check:vendored` script (correct and careful) | wired into neither `test` nor `build`; also exits 0 (skip) when the commit isn't in the local git history — fine locally, but CI should treat that as failure |
| 12 EIP-170 | none | no `forge build --sizes` step anywhere; worse, **tests actively mask it** — `test/ts/utils/anvil.ts:33` starts every anvil with `--code-size-limit 60000`, and its justifying comment ("CleartextFHEVMExecutor ~25 KB … exceeds the limit") is stale and wrong (22,994 B < 24,576 B). Drop the flag, add `--sizes` to the build |
| 14 no tests in payload | none | RULES.md prescribes the one-line `find pkg -name '*.t.sol' -o -name '*.test.ts'` gate; not implemented |
| 9 manifest purity | none | nothing asserts `pkg/package.json` has no `scripts`/`devDependencies`; a stray field would ship unnoticed |
| 15/17 fixed addresses | `scripts/anvil.sh` (manual) | see B1 |

`internal/createPackageTarball.ts` is the natural place for the rule-9/14/7 checks plus two more
it lacks: it never verifies the `exports`-referenced build outputs exist (a `pack:tarball` run
after `clean` produces a tarball whose `./ts` entry points at nothing, and `npm pack` succeeds),
and it never asserts the tarball's file list (available in the same `npm pack --json` output it
already parses) contains `src/`, `abi/`, `templates/`, `ts/_esm` and no `tsconfig*`/test files.

---

## 3. Should-fix

### Payload (`pkg/ts`) — fix before ×3 duplication

- **Rule 10 divergence, undocumented.** The adapter layer (`types/public.ts:35-73`) does not
  follow js-sdk's `XxxParameters` / `XxxReturnType` / `XxxModuleFunction` triple pattern that
  rule 10 names as the source of truth: three ad-hoc interfaces with inline anonymous parameter
  objects; only `encodeCall`/`deploy` partially conform. Rule 10 requires an explicit documented
  exception for any divergence — either add it or rework to triples. Worst instance:
  `writeContract(parameters: unknown): Promise<unknown>` (`types/public.ts:72`) erases all type
  checking on the state-changing operation, though every call site passes the same
  address/abi/functionName/args shape.
- **Public-API typos** — `DEFAUT_BOOTSTRAP_CONFIG_V13` (`constants.ts:148`, the default `deploy()`
  config) and `DEFAULT_COPROCESSOR_THESHOLD` (`constants.ts:32`). Renaming after duplication means
  breaking three published APIs instead of zero.
- **False docstring on `deployPauserSetContract`** (`deploy.ts:344-350`): claims a dedicated
  `pauserSetDeployer` at `startNonce` 0, but `deploy.ts:91` passes the main deployer and the
  address is computed at that deployer's `nextStartNonce + 2n`; there is no `startNonce` parameter.
  A v11/v12 duplicator trusting this comment breaks the nonce chain — which is rule 17's exact
  "easy to break by accident" scenario.
- **11 bare `console.log`s in the library deploy path** (`deploy.ts:269-472`): a published SDK
  helper spamming stdout with no opt-out; inject an optional logger or drop them.
- **Silent v12-via-v13-ABI coupling** (`upgrade.ts:138-140`): `resolveDefaultMigration` reads a
  live v12 KMSVerifier through the v13 `CleartextKMSVerifier` ABI; works only because the legacy
  getters survive in the v13 ABI, with no test tripwire if an ABI regeneration drops them; the
  three result casts are unchecked.

### Harness / scripts

- **v12 acquisition is fragile and silently skippable.** `prepareTestV12Consumer.ts` /
  `runUpgradeE2e.ts` hardcode the sibling path `../host-contracts-cleartext-v12`, the fixture dir,
  and the tarball prefix; `npm pack` runs at the sibling's *root*, which only works for the current
  pre-split v12 layout — a sibling regenerated from this tree's `pkg/` split would pack the private
  harness manifest and the e2e would degrade to a **silent green skip** (the catch at
  `prepareTestV12Consumer.ts:76` conflates "not installed" with "actually broken"). Add an env
  override (`CI=1` → skip becomes failure) and make the pack path payload-aware before duplicating.
- **`generateTemplates.ts` deletes before validating** (`:294-299`): `pkg/abi`, `pkg/templates`,
  `pkg/ts/artifacts` are removed before all 14 artifacts are confirmed loadable, so a mid-loop
  throw leaves the payload half-generated. Also, the `generate:templates` script (unlike
  `build:templates`) runs without `forge build` and `loadArtifact` has no freshness check — stale
  forge output gets baked in silently.
- **`runUpgradeE2e.ts:22-27` ignores `result.error`**: a missing binary (ENOENT) exits 1 with no
  diagnostic printed.
- **`scripts/anvil.sh:190-193`**: the rule-15 address verification runs even with a custom
  `--mnemonic`, guaranteeing a false failure; skip with a warning when the mnemonic is overridden.
- **`scripts/install.sh:91`**: `mapfile` requires bash ≥ 4; stock macOS `/bin/bash` is 3.2. The
  only bashism in the three scripts.
- **`test/ts/tarball-consumer.test.ts:34`**: uses port 8545 (anvil's default) — collides with any
  developer's running node and can silently target a stale external chain; every other suite uses
  dedicated ports. Note for duplication: a straight copy reuses the whole port map (8545/86xx),
  so parallel `npm test` across sibling trees on one CI machine will collide — shift the port
  ranges per generation.

### Payload manifest polish

- No `license`, `description`, `repository` fields and **no LICENSE file in the 274-file tarball**;
  npm warns at publish, and consumers get an unlicensed package.
- `"type": "module"` with a single `.d.ts` serving both `import` and `require` conditions is the
  arethetypeswrong "ESM types masquerading as CJS" case; emit `.d.cts` (or split types per format).
- No root (`.`) export and no `main`: `import '@fhevm/host-contracts-cleartext'` throws. Plausibly
  intentional for a Solidity-first package — document the decision in the (rewritten) README.

---

## 4. Nice-to-have

- Dead soldeer machinery: `forge-std` is declared (`foundry.toml:33`), locked (`soldeer.lock`) and
  remapped (`remappings.txt:2`) but imported by nothing — no `.t.sol`, no `.s.sol` in the tree.
- Dead/unreachable exports in `pkg/ts`: `checkDeployedBytecode`, `encodeACLOwnerUpgrade`,
  `getEmptyUUPSProxyACLArtifact`, `getERC1967ProxyArtifact` are exported by their modules but not
  re-exported from `index.ts` (the only consumer-reachable entry); decide dead vs missing.
- Dead constants with drift hazard: the nine `DEFAULT_*_MNEMONIC*` constants in `constants.ts` are
  referenced nowhere; `generateSigners.ts` hand-duplicates the mnemonic with a "kept in sync"
  comment and no assert (currently byte-identical, verified).
- `HexString` defined twice (`types/private.ts:4` and generated `artifacts/types.ts:3`).
- `pkg/ts/tsconfig.json:7` `"types": ["node"]` in a layer that uses no Node API — ambient-globals
  leak risk for the rule-8 "library-free" claim.
- Config dead entries: `tsconfig.base.json:65` `rootDir: "src"` (no such dir); the
  `ts/vitest.config.ts` exclude in all three build tsconfigs; the CJS compiler overrides living
  only on the CLI (`package.json:22`) so `tsc -p tsconfig.build.cjs.json` alone checks the wrong
  module mode.
- Hand-duplicated fixture file lists in `eslint.config.js:15-24` and `test/tsconfig.json:12-20` —
  a rename hazard during duplication.
- `scripts/derive-keys.sh` header/code drift (COUNT 10 vs 25, path `/2/` vs `/4/`, output always
  labeled COPROCESSOR).
- `test/ts/utils/anvil.ts`: no `'error'` listener on spawn (missing anvil → raw uncaught
  exception), SIGKILL fallback doesn't await final exit.
- `test/ts/utils/viemEthereumLib.ts:60-72`: adapter `deploy` awaits the receipt but doesn't check
  `receipt.status` (contrast `writeContract`, which does).
- `internal/createPackageTarball.ts:48`: npm cache dir is a fixed, version-free name in shared
  tmpdir — collides across users and across duplicated trees.
- `generateTemplates.ts:179`: forge out dir keyed by source basename; unique today, unasserted.
- Cosmetic comment rot: `generateTemplates.ts:137` (wrong config path), `types/public.ts:46-47`
  (`getCodeAt` described as "pure ABI encoding"), `upgrade.ts:55` (garbled "cleartextArithmeticAdd"),
  local vs absolute nonce numbering styles between `deployEmptyProxiesV13`/`V12`.
- `npm run test` recompiles forge three times and the TS build twice; wall-clock only.
- RULES.md staleness to fold back: rule 16's `.npmignore` status paragraph (file now gone), and
  rule 13's "one stale trace" (still present, see B3).

---

## 5. Duplication surface — what v11/v12 copies must patch

Version markers live in **~34 files** (excluding build output and `plans/`). Categories:

1. **Identity & vendoring** — `pkg/package.json` (`version`, `fhevm.vendoredFrom` tag+commit).
2. **Config injection prefix** — `remappings.txt:1` and
   `pkg/src/addresses/FHEVMHostAddresses.sol:15` (`fhevm-config-0.13.0/`) — must move in lockstep.
3. **Public API symbols** — everything the `pkg/ts` review enumerated: `DeployedV13`,
   `BootstrapConfigV13`, `FhevmAddressesV12/V13`, `precomputeFhevmAddressesV12/V13`,
   `updateV12ToV13`, `buildUpdateV12ToV13Plan`, `reinitializeV2/V3/V4` selectors,
   `kmsVerifierInitArgsV13`, `buildHostAddressReplacementsV13`, plus the `index.ts` re-exports.
   The nonce *layout itself* is generation-specific (v13 has 2 extra proxies; 12 deployer txs).
4. **Previous-generation coupling** — `internal/prepareTestV12Consumer.ts` (sibling path, fixture
   dir, tarball prefix), `internal/runUpgradeE2e.ts`, `internal/listUpgradeOps.ts` usage examples,
   `test/ts/upgrade-e2e.test.ts` (entirely pair-specific), `vitest.e2e.config.ts`,
   `tsconfig.e2e.json`.
5. **Tests & config lists** — `deploy-v13.test.ts` (filename referenced by name in
   `vitest.config.ts:29`, `test/tsconfig.json:14`, `eslint.config.js:18`), `EXPECTED_VERSIONS`
   tables, nonce tables in `tarball-consumer.test.ts`, `scripts/anvil.sh` labels and the
   22,994 B size figure, `test/signers.test.ts` names.
6. **Regenerated artifacts** — `pkg/abi`, `pkg/templates`, `pkg/ts/artifacts` are per-generation
   outputs of `build:templates`, not files to patch by hand.

**Recommendation before duplicating:** introduce one `internal/generation.ts` (or JSON) declaring
`{ line: '0.13', configPrefix: 'fhevm-config-0.13.0', previous: { name: 'v12', sibling: '../host-contracts-cleartext-v12', updateSymbol: 'updateV12ToV13' } }`
and have `internal/` scripts, test config globs, and (where possible) generated code read it. The
public TS symbol names can't be generated away, but everything in categories 1, 2, 4, and 5 can be
reduced to a one-file edit — which is also what RULES.md's preamble demands ("release automation,
not hand-copying"). Consider also renaming version-suffixed test filenames to version-neutral ones
(`deploy-stack.test.ts`) so three config lists stop tracking a filename.

---

## 6. Verified clean (no action)

- Rule 6: `check:vendored` passes — 21/21 vendored files byte-identical to `v0.13.2`
  (`07fb05fb75f0`); `.prettierignore` excludes `pkg/src`; the check script itself is careful
  (payload-relative `to`, vacuous-pass guards, subset semantics).
- Rule 8: zero external imports in `pkg/ts` (exhaustive grep + clean strict `tsc --noEmit`).
- Rule 9: payload manifest pure (no scripts/devDependencies/tooling); tarball has no
  tsconfig/tsbuildinfo leaks (274 files, verified dry-run).
- Rule 13: no `encrypted-types` dependency or source reference (only the B3 README line).
- Rule 14: zero test files under `pkg/`.
- Rule 10 (functions): `deploy()` and `updateV12ToV13()` exist with the required shapes and are
  exported.
- Nonce arithmetic between `addresses.ts` and `deploy.ts` is consistent (12 deployer txs, offsets
  verified by hand), with pairwise `assertNoCodeAt`/`assertDeployedAddress` guards and the
  `assertNoPlaceholdersRemain` deploy-time backstop.
- The harness `lint` is not the js-sdk solution-file no-op: `tsc -b` traverses all three project
  references (verified with `--verbose`).
- `test/templates.test.ts` is strong (identity test, alternate-address forge diff, patch-site
  baseline tripwire, high-entropy config test); anvil suites have solid try/finally teardown and
  receipt-awaited writes; the viem adapter implements the abstract interfaces with the
  write-then-read race handled per project convention.
