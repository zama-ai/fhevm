# Deriving `v12/` from `v13/`

## Goal

Stand up `sdk/host-contracts-cleartext/v12` as a **standalone, self-contained package** — its own
`node_modules`, its own `package-lock.json`, its own soldeer `dependencies/`, buildable and deployable
from scratch with nothing outside its directory — by **duplicating `v13/` and applying the smallest
possible set of edits**, every one of them attributable to the 0.13 → 0.12 protocol delta.

That is RULES.md rule 20: `v13/` is the reference implementation, `v12/` is derived from it, and
`diff -r v13 v12` must be short enough to read in full. This document is the edit list that makes the
diff auditable — one entry per hunk, each naming its cause.

"Deployable from scratch" means all three deploy paths work for v12, not just the TS one:

| Path                   | Entry point                                          | Target                    |
| ---------------------- | ---------------------------------------------------- | ------------------------- |
| TypeScript             | `pkg/ts/deploy.ts`                                   | any chain, via an adapter |
| Foundry, nonce-based   | `pkg/forge/src/FhevmDeploy.sol`, `pkg/forge/script/` | local / anvil             |
| Foundry, CREATE2-based | `create2-deploy/`                                    | testnet                   |

## Decisions taken

1. **v12 ships no upgrade path.** RULES.md rule 21's floor treatment applies to it for now: no
   `updateV11ToV12`, no upgrade e2e, no previous-generation fixture. `v11/` does not exist, so there is
   nothing to migrate from or test against. When v11 lands, the floor moves to it and v12 gains
   `updateV11ToV12` — a later phase, out of scope here.
2. **`create2-deploy/` is in scope.** v12 must be deployable from scratch by every path v13 has,
   CREATE2 testnet deploys included.
3. **v13 is frozen first.** Deriving from a moving tree turns every subsequent v13 edit into invisible
   v12 drift, so the baseline is committed and its SHA recorded before anything is copied. **Already
   satisfied** — see Phase 0.1.

## Hard constraint — nothing leaves this machine without approval

**No outward-facing action in this plan may be performed without the repo owner's explicit approval for
that specific action.** Local work is unrestricted; anything that publishes is not.

| Allowed freely, locally                                 | Requires explicit approval, every time                                                                    |
| ------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| `git commit` (Phase 0.1), local branches, local tags    | `git push`, pushing tags, deleting or force-pushing a remote ref                                          |
| `npm pack` (Phase 1, Phase 5 fixture)                   | `npm publish` to any registry — rules 2–4 name npm as a release channel, and this plan never exercises it |
| `./scripts/anvil.sh`, `deploy-testnet.sh --chain anvil` | any deploy to a live chain — testnet included; a CREATE2 deploy burns salts and is not undoable           |
| creating a local git repo for the rule 3 mirror         | creating or writing `github.com/zama-ai/host-contracts-cleartext` (rule 3)                                |

This applies to steps that look like cleanup too: reverting a bad push and deleting a stray tag are both
remote writes. Ask rather than tidying up unilaterally.

The last line of Appendix B — "once v12 passes Phase 4, retire
`sdk/host-contracts-cleartext-v12`" — is a local deletion in a working tree, and still a change the owner
should approve before it happens rather than after.

## Phase 0 — freeze the v13 baseline

### 0.1 Commit v13 — DONE

**Baseline: `6cb31b45a` — "fix(sdk): latest host-contracts-cleartext v13".** This supersedes the earlier
`f7d27c3fa`: it also carries the Phase 0.3 refactor. Verified at copy time: v13's working tree clean, 237
files tracked, nothing untracked-but-unignored, and `create2-deploy/` tracked — which is what makes
Phase 1's single-rsync copy correct.

Why the baseline had to move before Phase 1: with the refactor uncommitted, `git ls-files` would have
silently skipped three hand-written sources (`internal/contractVersions.ts`,
`internal/generateContractVersions.ts`, `internal/cli/generateContractVersions.ts`) and produced a v12
that could not run its own generators. An uncommitted baseline is not a bookkeeping preference.

That commit is the declared baseline: from then on, a hunk in `diff -r v13 v12` is either in the register
below (Appendix A) or it is a v13 change that was never propagated down.

**Line numbers in this document are stale, deliberately not chased.** They were accurate against
`6cb31b45a`; v13 has moved several hundred lines since — the Phase 0.3 refactor, the Phase 5 fixes, and
the create2 work. Re-deriving them would make them wrong again on the next commit, so treat every `file:NN`
here as "this file, near here" and locate the symbol by name. The file paths and symbol names are the part
that is maintained; the numbers were only ever a convenience.

Local commit only — no push, no tag (see the hard constraint above).

### 0.2 Two v13 fixes the new layout forces

Rule 20's corollary — a bug found while working on v12 is fixed in v13 first, then copied down. Both of
these are broken _by_ v12 gaining the `pkg/` payload split, so they must land in v13 before the copy:

- **`internal/prepareTestV12Consumer.ts:34`** runs `npm pack` with `cwd: V12_PACKAGE_ROOT`, i.e. the
  harness root. Under the split layout the harness manifest is private and the payload manifest lives in
  `pkg/`, so this packs the wrong thing. It must pack `join(V12_PACKAGE_ROOT, 'pkg')`, matching
  `internal/createPackageTarball.ts:99`, which already uses `PKG_DIR_ABS_PATH`.
- **`README.md` step 6** still documents the previous generation as the flat sibling
  `../host-contracts-cleartext-v12` (in the `list:upgrade-ops` invocation and the `grep` output).
  `internal/constants.ts` already resolves it as `../v12`. Re-point the prose.

### 0.3 v13 changes that shrink this diff — DONE

**All five landed.** 43 files changed, +572/−482, on top of baseline `f7d27c3fa`. Verified green:
`prettier:check`, `lint`, `check:zama-config`, `check:vendored` (21 files identical), `forge test` 35/35,
template tests 11/11, `test:tarball` 19/19 across 9 files, and `VerifyFhevmDeploy` against a live anvil
stack at **58 passed / 0 failed / 3 skipped**.

The strongest single signal: `internal/placeholders/patch-sites.json` regenerated **byte-identical** —
360 patch sites across 14 contracts unchanged — so none of this altered bytecode or address semantics.

| Item | Landed as                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A    | `FhevmAddressesV13`→`FhevmAddresses`, `BootstrapConfigV13`→`BootstrapConfig`, `DeployedV13`→`Deployed`, `DEFAUT_BOOTSTRAP_CONFIG_V13`→`DEFAULT_BOOTSTRAP_CONFIG` (typo fixed too), plus the internal `buildHostAddressReplacements` / `buildBootstrapPlan` / `UpgradeConfig` / `kmsVerifierInitArgs`. `FhevmAddressesV12` and `updateV12ToV13` keep their suffix — they name a _pair_. The two-layer `deployEmptyProxiesV12`/`V13` split and the `precomputeFhevmAddressesV12`/`V13` split are collapsed into one function each.                  |
| B    | `PROXY_COUNT` and `ADDRESSED_NONCE_COUNT` emitted into the generated `LocalHostAddresses.sol`; all 15 executable arity literals now derived (`PROXY_COUNT`, `_allProxyRoles().length`, `_sharedProxyRoles().length + 1`, `2 * n + 4`). `generateGenesis.ts`'s `!== 9` and `anvil-local-v2.sh`'s `START_NONCE + 12` derive too — the shell reads the constant via the existing `read_scalar`. `_sharedProxyRoles()` keeps its literal size (Solidity has no array-literal length) but a mismatch now reverts instead of producing a wrong address. |
| C    | 78 count-phrases rewritten count-free across 16 files. Zero `nine`/`eight`/`22 creates` prose left in the deploy layers.                                                                                                                                                                                                                                                                                                                                                                                                                          |
| D    | New `internal/contractVersions.ts` scans `pkg/src` for `CONTRACT_NAME` + the three version constants — **zero configuration**, so a generation needs no edit. `generateContractVersions.ts` emits `pkg/forge/src/_internal/LocalHostVersions.sol` and `pkg/ts/versions.ts`; `VerifyFhevmDeploy.s.sol`, `FhevmDeploy.t.sol` and `deploy-v13.test.ts` all read them. All 9 generated strings matched the hand-written literals exactly. `CONTRACT_VERSIONS` is exported from `pkg/ts`.                                                              |
| E    | `test/templates.test.ts`'s `addressConfigSource()` and `ALTERNATE_ADDRESSES` derive from `ADDRESS_NAMES`; `NEXT_START_NONCE_OFFSET` derives from `LAYOUT`. **`LAYOUT` and `LOCALHOST_ADDRESSES` deliberately stay hand-written** — see the correction below.                                                                                                                                                                                                                                                                                      |

Two corrections to what this section originally proposed, both found by doing the work:

- **`precompute-addresses.test.ts`'s `LAYOUT` must NOT be derived.** Its own doc comment says it is "the
  only one that is an assertion rather than an input, which is what makes it a check instead of a fifth
  thing to keep in step." Deriving it from the code under test would delete the oracle. Same for
  `LOCALHOST_ADDRESSES`. Only `NEXT_START_NONCE_OFFSET` was derived, _from_ `LAYOUT`, which keeps the
  oracle independent while removing one literal. For v12 these two tables are legitimate generation work.
- **`ALTERNATE_ADDRESSES` must use decimal digits only.** They are rendered into Solidity as address
  literals, and solc rejects a hex literal containing letters unless it carries a valid EIP-55 checksum
  (error 9429) — which is why the hand-written table looked like `0x7011121314…`. A base-16 counter fails
  the build. Now recorded in the code.

Two pre-existing defects fixed in passing, both unrelated to v12:

- **`npm run build` was already failing at baseline.** `prettier:check` is its first step, and
  `create2-deploy/script/*.sol` was added without prettier being able to parse Solidity — 11 files
  erroring with "No parser could be inferred". Fixed by adding `prettier-plugin-solidity` (`^2.0.0`,
  matching the repo root) with a `*.sol` override setting `singleQuote: false`, mirroring
  `host-contracts/.prettierrc.json`. Verified `pkg/src` stays untouched — it is prettier-ignored, which is
  what rule 6 requires.
- **`pkg/ts/artifacts/` is wiped by `generateTemplates.ts`** (`rmSync(tsArtifactDir, {recursive: true})`),
  so a second generator's output placed there is deleted on the next build. Hit this with `versions.ts`;
  it now lives at `pkg/ts/versions.ts`, beside `cleartext-config.ts`, the existing precedent.

The original rationale follows, for the record.

### 0.3.1 Why these five — the original analysis

**v13 has never been shipped.** No consumer holds its API, no tarball is on npm, rules 2–4's channels
are unused. That makes every change below free _right now_ and progressively expensive afterwards — a
renamed exported type costs nothing today and is a breaking change the day after the first publish. This
is the window, so these are prerequisites of Phase 1 rather than a wishlist.

The edit list in Phase 2 is larger than the protocol delta justifies. Almost all of the excess traces to
places where **v13 writes down a value it could derive**, and each one is paid again per generation:
once for v12, again for v11, again for v14.

Rule 20 says a non-generation-specific fix goes into v13 first and is then copied down. These qualify —
and each also closes a real drift gap in v13 today, independent of v12. Measured against the baseline:

| #   | v13 strategy that costs v12 edits                                                                                                                                                                                                                            | Hand-written sites | Fix in v13                                                                                                                                                                                                                                                                          | v12 diff after |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------- |
| A   | Generation number baked into type and function _names_ — `FhevmAddressesV13`, `BootstrapConfigV13`, `DEFAUT_BOOTSTRAP_CONFIG_V13`, `buildHostAddressReplacementsV13`, `deployEmptyProxiesV13`, `precomputeFhevmAddressesV13`, `UpgradeConfigV13`             | **74**             | Drop the suffix. A package is already version-pinned by its own `version`, so the suffix carries no information _inside_ it. Keep it only in `upgrade.ts`, which genuinely names a _pair_ of generations                                                                            | 0 lines        |
| B   | Proxy/create counts as literals — `new ACLOwner.Op[](9)`, `new Create[](22)`, `materialized == 9`, `live == 9`, `implementationCount !== 9`, `NEXT_START_NONCE_OFFSET = 12n`, `START_NONCE + 12`                                                             | **~20**            | Derive from one source. Solidity call sites mostly have `_allProxyRoles()` in scope already; the rest can read a `PROXY_COUNT` emitted into the generated `LocalHostAddresses.sol`, which already knows `ADDRESS_NAMES`. TS reads `ADDRESSED_NONCE_COUNT`, which is already derived | 0 lines        |
| C   | Prose that spells the counts out — "all nine proxies", "the eight share ONE init-code hash", "22 CREATE2s through the factory"                                                                                                                               | **~75**            | Write count-free prose: "every proxy", "the shared-implementation proxies", "one CREATE2 per `_creates()` entry"                                                                                                                                                                    | 0 lines        |
| D   | Expected `getVersion()` strings hand-copied per contract — `"ACL v0.4.0"`, `"KMSVerifier v0.3.0"`, … in three files                                                                                                                                          | **~15**            | Emit them. `generateTemplates.ts` already reads every compiled artifact, and `MAJOR/MINOR/PATCH_VERSION` are compile-time constants — add `version` beside `abi`/`template` in `pkg/ts/artifacts/*.ts` and as a generated Solidity table                                            | 0 lines        |
| E   | Address tables retyped instead of derived — `SCRIPT_VARIABLE` in `generateComputeAddressesScript.ts`, `addressConfigSource()` in `test/templates.test.ts`, `LAYOUT` + `LOCALHOST_ADDRESSES` in `test/ts/precompute-addresses.test.ts`, `ALTERNATE_ADDRESSES` | **~30**            | Loop over `ADDRESS_NAMES` / import `NONCE_OFFSET`. `LAYOUT` _is_ `NONCE_OFFSET` retyped; `LOCALHOST_ADDRESSES` is the generated `LocalHostAddresses.sol` retyped                                                                                                                    | ~2 lines       |

Order to do them in — cheapest and least risky first, each independently landable and reviewable:

1. **C — count-free prose.** ~75 sites, zero logic, no test can fail. Do it first: it makes the B sweep
   readable by leaving only real literals behind.
2. **A — drop the generation suffix from names.** 74 sites, mechanical rename. Keep the suffix _only_ in
   `upgrade.ts` and `UpdateV12ToV13MigrationConfig`, which genuinely name a pair of generations. Because
   v13 is unshipped there are no deprecated aliases to leave behind — rename outright.
3. **D — generate the expected `getVersion()` strings.** Removes group D′ entirely and closes a live
   drift gap: today `"ACL v0.4.0"` is hand-copied in three files and nothing checks it against the
   contract it describes.
4. **B — derive the counts.** Removes group H.3 entirely. Emit `PROXY_COUNT` / `CREATE_COUNT` into the
   already-generated `LocalHostAddresses.sol`; several Solidity call sites can use
   `_allProxyRoles().length`, already in scope. TS reads `ADDRESSED_NONCE_COUNT`, already derived.
5. **E — derive the address tables.** `LAYOUT` in `precompute-addresses.test.ts` _is_ `NONCE_OFFSET`
   retyped; `LOCALHOST_ADDRESSES` is the generated `LocalHostAddresses.sol` retyped. Import instead.

Together these delete groups D′ and H.3 outright and shrink D, F and G to the protocol delta alone.

#### The one worth doing now most of all — PREREQUISITE DONE, UNIFICATION STILL OPEN

**Status: the equivalence is now proved and guarded; the six copies still exist.**

`test/stack-order.test.ts` (in both generations) reads all six sources as text, extracts the order each
one materializes, and asserts they match — with `pkg/ts/deploy.ts` as the reference. It passes today,
which is the first time anything has established that the six agree. Verified by injection: swapping
`HCU_LIMIT` and `PROTOCOL_CONFIG` in `scripts/anvil-lib.sh` fails it with "one of them is deploying the
wrong implementation behind a proxy". It is byte-identical between generations, because it derives the
expected order from each package's own `deploy.ts` rather than hardcoding one.

Why this before the refactor, rather than instead of it: **a refactor that merges six copies has to start
by proving they are equivalent**, or it silently adopts whichever copy the author happened to read. That
proof did not exist. It does now, and it also keeps them equivalent while the merge is pending — which
matters because v11 is next.

It passing in v12 is the stronger result: v12's six layers were each hand-edited during the port, and the
test confirms independently that the six-way edit came out consistent.

Two limits, stated because a green test invites over-reading:

- **It verifies order, not behaviour.** Two layers can agree on the sequence and still pass different
  initializer arguments. Those arguments are deliberately not unified — they are the documented override
  point (`FhevmDeploy._fhevmProtocolConfig`) — and stay covered by the deploy tests and
  `VerifyFhevmDeploy`, which read them back off a live chain.
- **It is a text scan**, so it is coupled to the shape of the six files. Each extraction is anchored
  between markers and asserts it found something, so a restructure fails loudly rather than silently
  matching nothing — but the scan is a stepping stone, not the destination. When one generated table
  replaces the six, delete this file.

The unification itself is unchanged from the analysis below and still worth doing.

#### The original case for unifying

The stack's shape is described **six** times over — `pkg/ts/deploy.ts`, `pkg/forge/src/FhevmDeploy.sol`,
`pkg/forge/script/FhevmDeployScript.s.sol`, `pkg/forge/script/DeployLocalStack.s.sol`,
`create2-deploy/script/MaterializeInitData.sol` and `scripts/anvil-lib.sh` each carry their own ordered
proxy list and their own `initializeFromEmptyProxy` argument encoding.

**That sixfold duplication is the reason groups F, H.1, H.2 and H.3 exist as separate work: every group
in this plan is the same edit applied six times.** It is also why H.2's ordinal renumber is the port's
riskiest step — four index-aligned lists must agree, and nothing checks that they do until a deploy runs.

One generated stack description — an ordered `{role, artifact, initArgs}` table emitted beside
`LocalHostBytecode.sol`, which already holds every input it needs — collapses all six into one, makes the
ordinals unrepresentable-if-wrong, and makes each future generation's deploy layer a near-zero-line diff.

This is a real refactor of four deploy layers and deserves its own review pass, so it is listed
separately rather than folded into the five above. But it is the largest lever here, its cost is lowest
while v13 is unshipped and while v12 does not yet exist to keep in step, and **every generation added
before it lands pays for it again.** Recommendation: do it before Phase 1. If it slips, do it before v11.

Nothing else in v13 changes for v12's sake.

## Phase 1 — duplicate — DONE

**230 files copied**, and `diff -r v13 v12` reports exactly one line: `Only in v13: plans`. The clone
then passed `install.sh --reset --lockfile=keep`, `lint`, `forge test` 35/35 and the template tests 11/11
as a standalone package, with `package-lock.json` and `soldeer.lock` byte-identical to v13's.

That green run is the point of the phase: Phase 2 starts from a base known to work, so any later failure
is attributable to a Phase 2 edit rather than to the copy.

Two expected leftovers of the clone, both removed by Phase 2 group G — worth naming so they are not
mistaken for problems:

- `internal/constants.ts` still resolves `PREVIOUS_GENERATION_DIR_ABS_PATH` to `../v12`, i.e. **itself**.
  Running `npm run test:upgrade-e2e` in v12 right now would try to build v12 as its own fixture.
- `test/ts/upgrade-e2e.test.ts` and `test/ts/vitest.e2e.config.ts` still reference
  `@fhevm/host-contracts-cleartext-v12`.

Copy the **tracked** v13 tree plus `create2-deploy/`'s sources, and nothing else:

```sh
cd sdk/host-contracts-cleartext
git ls-files v13 | sed 's|^v13/||' \
  | grep -vE '^plans/|^create2-deploy/\.out' > /tmp/v12-files.txt
rsync -a --files-from=/tmp/v12-files.txt v13/ v12/
```

One rsync, not two: `create2-deploy/` **is** tracked (20 files), so `git ls-files` already carries it.
The `.out` filter matters because v13's own `.gitignore` records that the seals under `.out-*/` are
committed deliberately, with `git add -f` — so a future seal would otherwise ride along into v12 against
the exclusion table below.

Deliberately excluded:

| Excluded                                                       | Why                                                                                                               |
| -------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `node_modules/`, `dependencies/`, `out/`, `cache/`, `tarball/` | Install/build outputs. Phase 1.1 regenerates them.                                                                |
| `create2-deploy/.out-*/`                                       | Per-deployment seals and broadcast logs from v13's own anvil run. They record _that_ deployment, not the tooling. |
| `internal/.deploy-config/`, `internal/.tmp-localhost/`         | Generator scratch.                                                                                                |
| `plans/`                                                       | v13 design history. Two copies of one narrative would drift; it stays in `v13/`.                                  |

`plans/` is **not** copied. Those documents are v13 design history, they describe work done in v13, and
duplicating them would mean two copies of the same narrative drifting apart. They stay in `v13/` only,
and `v12/` has no `plans/` directory for now.

One consequence to handle in group G: v13's `README.md` closes step 7 with
"see `plans/FORGE_DEPLOY_SCRIPT_PLAN.md`". In v12 that path does not resolve — re-point it at
`../v13/plans/FORGE_DEPLOY_SCRIPT_PLAN.md` or drop the parenthetical. Nothing else references `plans/`:
it is outside the payload (rule 16) and outside every tsconfig and eslint scope.

### 1.1 Make it standalone

```sh
cd v12
./scripts/install.sh --reset --lockfile=keep    # node_modules + soldeer dependencies/
```

`package-lock.json` and `soldeer.lock` are copied verbatim and stay that way — v12's devDependency set
is identical to v13's, so a re-resolve would produce diff noise with no cause behind it. The two
harnesses being named the same (`@fhevm/host-contracts-cleartext-dev`) is fine: both are `private: true`
and there is no workspace above them to collide in.

## Phase 2 — the edit list — DONE

All nine groups applied. Verified in v12: `check:vendored` (16 files identical to `v0.12.5`),
`check:zama-config`, `prettier:check`, `lint`, `forge test` 11/11, template tests 11/11,
`test:tarball` 17/17 across 7 files, `forge build --sizes` (largest is `CleartextFHEVMExecutor` at
19,764 B — 4,812 B of margin, comfortably more than v13's), and the full bash deploy path via
`./scripts/anvil.sh` at **40 passed / 0 failed / 1 skipped** plus the rules 15/17 address check.

Phase 3 regenerated to exactly the expected shape with **zero hand-editing**: 7 contract versions (was
9), 10 nonces / 8 addresses (was 12/10), 12 interfaces (was 14), `PROXY_COUNT = 7`,
`ADDRESSED_NONCE_COUNT = 10`. Groups D′ and H.3 required no work at all, as Phase 0.3 predicted.

`diff -r v13 v12` reports 127 differing entries, every one attributable to an Appendix A cause.

**One arity literal the Phase 0.3 item-B sweep missed**, found only by running the v12 deploy:
`scripts/deploy.sh` derived the ACLOwner address from a hardcoded `START_NONCE + 12`. The sweep's path
list covered `pkg/forge`, `create2-deploy/script` and `scripts/anvil-lib.sh` — not `deploy.sh` — and its
regex would not have matched `+ 12` anyway. Fixed in v13 first (rule 20) by reading
`ADDRESSED_NONCE_COUNT` out of the generated `LocalHostAddresses.sol`, then copied down. It presented as
`FAIL ACL.owner == ACL_OWNER_ADDRESS` with the _deploy_ correct and the _expectation_ stale — worth
remembering as the signature of this class of bug.

### The original edit list

Nine groups, each one a cause. Everything not listed here stays byte-identical (Appendix B).

**If Phase 0.3 lands first, two of these groups disappear** — D′ (expected versions) and H.3 (arity
literals) become generated values, and D, F and G shrink to the protocol delta. The groups are written
for the current baseline, so treat a vanished group as confirmation the refactor worked rather than as a
step to skip.

### A. Identity and provenance — rules 5, 7, 18

| File               | Edit                                                                                                                            |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------- |
| `pkg/package.json` | `version`: `0.13.0` → `0.12.0`                                                                                                  |
| `pkg/package.json` | `fhevm.vendoredFrom`: `tag` → `v0.12.5`, `commit` → `ac18e49ea85dd3c26788fc66f9ac0ea7cfe48519` — verbatim from RULES.md rule 18 |
| `package.json`     | drop `test:upgrade-e2e` from the `test` script, and drop the `test:upgrade-e2e` script itself (group G)                         |

The published name stays `@fhevm/host-contracts-cleartext`. Every generation publishes under it and they
differ only by version — `internal/constants.ts` says so, and v13's e2e fixture relies on it, installing
the package into an _aliased directory_ rather than renaming it.

### B. The config remapping prefix — `fhevm-config-0.13.0/` → `fhevm-config-0.12.0/`

Rule 11 pins the prefix to the protocol minor. **Four** executable sites, plus prose:

- `remappings.txt:1`
- `internal/constants.ts` → `FHEVM_CONFIG_REMAPPING_PREFIX`
- `pkg/src/addresses/FHEVMHostAddresses.sol` → the import path (also edited by group D)
- `scripts/deploy.sh:85` → `CONFIG_PREFIX="fhevm-config-0.13.0/"`
- prose: `README.md`, and the `FOUNDRY_REMAPPINGS=fhevm-config-0.13.0/=…` usage instructions in
  `pkg/forge/script/FhevmDeployScript.s.sol:44,50` and `pkg/forge/script/VerifyFhevmDeploy.s.sol:61,65`

The v13 README's step 5 grep (`grep -rn 'fhevm-config-' remappings.txt pkg/src`) is **too narrow** — it
misses `scripts/` and `pkg/forge/`. Sweep the whole tree instead:

```sh
grep -rn --exclude-dir=node_modules --exclude-dir=_cjs --exclude-dir=_esm --exclude-dir=_types \
  'fhevm-config-' .
```

`create2-deploy/GUIDE.md` and `create2-deploy/README.md` carry **zero** occurrences — verified — so they
are not sites despite being the obvious place to look. A mismatch in the four executable sites is a
compile error, which is the good failure mode; `deploy.sh:85` is the one that fails at runtime instead.

### C. The vendored sources — rules 6, 18

`pkg/src/contracts/` must be byte-for-byte `host-contracts/contracts` at `v0.12.5`. Replace the tree
wholesale rather than editing files:

```sh
cd sdk/host-contracts-cleartext/v12
TMP=$(mktemp -d)
git -C ../../.. archive ac18e49ea85dd3c26788fc66f9ac0ea7cfe48519 host-contracts/contracts | tar -x -C "$TMP"
rm -rf pkg/src/contracts
mv "$TMP/host-contracts/contracts" pkg/src/contracts
rm -rf "$TMP"
```

Extract-then-move rather than a one-liner: `--strip-components` plus `--one-top-level` is GNU tar, and
this is a macOS host running BSD tar, where the second flag does not exist.

**Verified**: `v0.12.5` carries 16 files there, v13 vendors 21, and the 16 are exactly v13's 21 minus
these five — which is also the complete upstream set, so v12 vendors no subset and there is no adoption
decision to make:

- `ProtocolConfig.sol`
- `KMSGeneration.sol`
- `interfaces/IProtocolConfig.sol`
- `interfaces/IKMSGeneration.sol`
- `shared/Structs.sol`

**Verified**: this is byte-identical to what the pre-split `sdk/host-contracts-cleartext-v12` already
carries, so the extraction is checkable against a second source before the old directory is retired.
`scripts/check-vendored-sources.sh` reads tag and commit out of `pkg/package.json`, so group A makes the
gate self-configuring — no edit needed here.

### D′. Expected contract versions — ~~the surviving five all move~~ NO LONGER NEEDED

> **Delivered by Phase 0.3 item D.** The expected strings are now generated from the contracts, so v12
> inherits its own correct table with no edit. The table below is kept only as the record of what the
> 0.13→0.12 version delta actually is — useful when reading `list:upgrade-ops` output.

0.13 bumped four vendored contracts and group E bumps a fifth, so **every surviving `getVersion()`
expectation changes**, not just the two that disappear. Verified against `v0.12.5` vs v13's vendored
sources:

| Contract              | v13      | v12                                                            |
| --------------------- | -------- | -------------------------------------------------------------- |
| `ACL`                 | `v0.4.0` | `v0.3.0`                                                       |
| `FHEVMExecutor`       | `v0.4.0` | `v0.3.0`                                                       |
| `KMSVerifier`         | `v0.3.0` | `v0.2.0`                                                       |
| `HCULimit`            | `v0.3.0` | `v0.2.0`                                                       |
| `CleartextArithmetic` | `v0.4.0` | `v0.3.0`                                                       |
| `InputVerifier`       | `v0.2.0` | `v0.2.0` — unchanged, which is _why_ `updateV12ToV13` omits it |

Cross-check: v13's own `test/ts/upgrade-e2e.test.ts:178-183` already records exactly these v12 strings,
so they are not a guess.

Three files hand-copy them and all three need the table above:

- `pkg/forge/script/VerifyFhevmDeploy.s.sol:167-178`
- `test/FhevmDeploy.t.sol:69-77`
- `test/ts/deploy-v13.test.ts:112-120`

Miss this and `forge test`, the vitest suite and `./scripts/anvil.sh` all fail. **Phase 0.3 item D
deletes this group entirely** — generated versions cannot be stale.

### D. The address set — 10 names → 8

0.12 has no `ProtocolConfig` and no `KMSGeneration`, so `PROTOCOL_CONFIG_ADDRESS` and
`KMS_GENERATION_ADDRESS` disappear everywhere.

| File                                         | Edit                                                                                                                                                                                                            |
| -------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `pkg/src/addresses/FHEVMHostAddresses.sol`   | drop 2 imported symbols and their 2 `*Add` aliases                                                                                                                                                              |
| `internal/constants.ts`                      | drop 2 entries from each of `ADDRESS_NAMES`, `HOST_NONCE_OFFSET`, `NONCE_LABEL`, `CONSTANT_NAMES`                                                                                                               |
| `internal/generateTemplates.ts`              | drop the 2 `// v0.13.0` rows from `TARGET_CONTRACTS`                                                                                                                                                            |
| `internal/generateLocalHostBytecode.ts`      | drop 2 rows from the creation/runtime kind map                                                                                                                                                                  |
| `internal/generateComputeAddressesScript.ts` | drop 2 rows from `SCRIPT_VARIABLE` (`:68-69`) — it is typed `Record<AddressName, string>`, so the excess keys are a type error once `ADDRESS_NAMES` shrinks                                                     |
| `internal/generateGenesis.ts`                | `implementationCount !== 9` (`:254-255`) → 7 — an arity literal outside H.3's sweep paths                                                                                                                       |
| `test/templates.test.ts`                     | drop 2 entries from `ALTERNATE_ADDRESSES` (`:68-69`) **and** 2 lines from `addressConfigSource()` (`:142-143`), which spells each name out — left alone it renders `address(undefined)` into generated Solidity |

**The nonce layout needs no edit.** v13 derives it: `HOST_NONCE_COUNT` is computed from
`HOST_NONCE_OFFSET`, and the trailing block is positioned as `HOST_NONCE_COUNT + k`. Deleting the two
host entries shifts the layout from v13's `1,3,4,5,6,7,8 | 9,10,11` to v12's `1,3,4,5,6 | 7,8,9`
automatically, and `ADDRESSED_NONCE_COUNT` falls from 12 to 10 on its own. **Verified** against the
pre-split v12's `precomputeAddresses`, which hardcodes exactly that layout.

**Verified**: rules 15 and 17 are unaffected. ACL, FHEVMExecutor and KMSVerifier sit at nonce offsets 1,
3 and 4 in both generations, so the three `ZamaConfig.sol` addresses are identical for v12 and v13, and
`npm run check:zama-config` passes unchanged.

**Verified**: the 8 surviving names are exactly what 0.12's contracts reference — the vendored sources
import `aclAdd`, `fhevmExecutorAdd`, `hcuLimitAdd`, `inputVerifierAdd` and `pauserSetAdd`, and the
cleartext sources add `kmsVerifierAdd`, `cleartextArithmeticAdd` and `cleartextDbAdd`.

### E. The cleartext contracts — no nary operators

0.13 added `fheSum` and `fheIsIn`. Their cleartext `record*` hooks are new selectors that 0.12's executor
never calls, and their _absence_ from v12 is what will force the `CleartextArithmetic` re-point in the
future v12→v13 upgrade — so this is a real divergence, not a copy shortcut.

| File                                           | Edit                                                                                                              |
| ---------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `pkg/src/cleartext/ICleartextArithmetic.sol`   | drop `recordNaryOp`                                                                                               |
| `pkg/src/cleartext/CleartextArithmetic.sol`    | drop the `recordNaryOp` implementation and `reinitializeV2`; `MINOR_VERSION` 4 → 3, `REINITIALIZER_VERSION` 3 → 2 |
| `pkg/src/cleartext/CleartextFHEVMExecutor.sol` | drop both `_naryOp` overrides                                                                                     |
| `test/CleartextNaryOps.t.sol`                  | delete                                                                                                            |

Everything else under `pkg/src/cleartext/` and `pkg/src/` stays verbatim. In particular:

- **`pkg/src/upgrade/ACLOwner.sol` is copied verbatim.** v13 grew `execute`, `pause`/`unpause`,
  `IACLPausable`, `Executed` and `TargetHasNoCode` since the pre-split v12. None of that is
  generation-specific, so rule 20 requires v12 to carry it. **Do not** restore the older shape.
- `CleartextKMSVerifier.sol` and `CleartextInputVerifier.sol` are **verified byte-identical** between the
  pre-split v12 and v13 already.
- `CleartextDB.sol`'s `getACLAddress()` and `CleartextFHEVMExecutor.sol`'s
  `getCleartextArithmeticAddress()` are v13 additions that are **not** generation-specific — keep them.

### F. The TypeScript layer

The v13→v12 shape change is one thing: **0.13 moved the KMS signer set out of `KMSVerifier` into
`ProtocolConfig`.** In 0.12, `KMSVerifier.initializeFromEmptyProxy` takes
`(verifyingContractSource, chainIDSource, initialSigners, initialThreshold)` — the same 4-arg signature
as `InputVerifier` — and there is no `ProtocolConfig` to configure.

| File                     | Edit                                                                                                                                                                                                                                                                                                  |
| ------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `pkg/ts/types/public.ts` | drop `FhevmAddressesV13`, `ProtocolConfigInitConfig`, `KmsNode`, `KmsThresholds`, `UpdateV12ToV13MigrationConfig`; `KMSVerifierInitConfig` regains `initialSigners` + `initialThreshold`; `BootstrapConfigV13` → `BootstrapConfigV12` minus its `protocolConfig` field; `DeployedV13` → `DeployedV12` |
| `pkg/ts/constants.ts`    | `DEFAUT_BOOTSTRAP_CONFIG_V13` → `..._V12`: `kmsVerifier` gains signers + threshold, `protocolConfig` goes; drop `DEFAULT_KMS_THRESHOLDS`, `generateDefaultKmsNodes`, `generateFromExistingDefaultKmsNodes`, `nextDefaultKmsSignerWindow` and the `KMS_SIGNER_INDEX` map                               |
| `pkg/ts/deploy.ts`       | delete `deployEmptyProxiesV13` (`deployEmptyProxiesV12` already exists in v13 and becomes the only one); drop 2 rows from `targets` and 2 fields from `UpgradeConfigV13`; `kmsVerifierInitArgsV13` → the 4-arg v12 form; rename the `V13` identifiers                                                 |
| `pkg/ts/utils.ts`        | `buildHostAddressReplacementsV13` → `...V12`, minus 2 entries                                                                                                                                                                                                                                         |
| `pkg/ts/addresses.ts`    | drop the 2 addresses from `precomputeFhevmAddressesV13`; the nonce arithmetic follows group D                                                                                                                                                                                                         |
| `pkg/ts/kmsContext.ts`   | delete — `defineNewKmsContext` / `destroyKmsContext` are `ProtocolConfig` operations                                                                                                                                                                                                                  |
| `pkg/ts/upgrade.ts`      | delete — rule 21 floor                                                                                                                                                                                                                                                                                |
| `pkg/ts/index.ts`        | drop `updateV12ToV13`, `defineNewKmsContext`, `destroyKmsContext` and the deleted types; `V13` type names → `V12`                                                                                                                                                                                     |

v13's `deploy.ts` already factors 0.12's seven-proxy sequence out as `deployEmptyProxiesV12`, so this
group is mostly deletion rather than rewriting.

`pkg/ts/proxies.ts`, `pauserSet.ts`, `aclOwner.ts`, `types/private.ts` and `artifacts/types.ts` are
copied verbatim. `internal/cleartext-config.ts` (and its generated copy `pkg/ts/cleartext-config.ts`)
stays **byte-identical**: v12 simply stops importing the KMS node-metadata constants. Editing that file
would fork the one source of truth the harness and the payload share, for no gain.

**Public API v12 must expose** — this is what v13's future `updateV12ToV13` e2e consumes from it:
`deploy`, `precomputeAddresses`, `setupACLOwner`, `pauseACL`, `unpauseACL`, and the types
`FhevmAddressesV12`, `CleartextAddresses`, `BootstrapConfigV12`, `DeployedV12`.

### G. The harness — tests, scripts, docs

**Delete** (rule 21 floor — v12 has no previous generation):

- `pkg/ts/upgrade.ts` (group F), `internal/prepareTestV12Consumer.ts`, `internal/cli/prepareTestV12Consumer.ts`,
  `internal/runUpgradeE2e.ts`, `internal/cli/runUpgradeE2e.ts`
- `test/ts/upgrade-e2e.test.ts`, `test/ts/vitest.e2e.config.ts`, `test/ts/tsconfig.e2e.json`
- `test/ts/define-kms-context.test.ts`, `test/ts/destroy-kms-context.test.ts` — `ProtocolConfig` operations
- `internal/constants.ts` → `PREVIOUS_GENERATION_DIR_ABS_PATH`, `PREVIOUS_GENERATION_FIXTURE_ALIAS`

**Keep verbatim**: `internal/listUpgradeOps.ts` and `internal/cli/listUpgradeOps.ts`. They take the
previous generation as an argument and hardcode no version, so in v12 they simply have nothing to point
at rather than being broken — the same call rule 21 makes for v11.

**Edit**:

| File                                                                 | Edit                                                                                                                                                                                                                                                                                                                                               |
| -------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `test/ts/deploy-v13.test.ts`                                         | rename to `deploy-v12.test.ts`; drop the 2 contracts; apply group D′; **re-point** the KMS-signer/threshold reads at `:559-570` from `protocolConfigAddress` to `KMSVerifier.getKmsSigners()`/`getThreshold()` (both exist at 0.12.5) or drop that block wholesale; drop the `protocolConfig` bootstrap block at `:223`                            |
| `eslint.config.js`, `test/ts/vitest.config.ts`, `test/tsconfig.json` | the `deploy-v13.test.ts` path in each include/ignore list, **and** the now-dangling `define-kms-context.test.ts` / `destroy-kms-context.test.ts` / `upgrade-e2e.test.ts` entries                                                                                                                                                                   |
| `test/ts/precompute-addresses.test.ts`                               | more than a deletion: drop the 2 addresses from `LAYOUT` (`:39-49`) **and renumber** cleartextArithmetic 9n→7n, cleartextDb 10n→8n, pauserSet 11n→9n; `NEXT_START_NONCE_OFFSET` 12n→10n (`:52`); **recompute** the three shifted `LOCALHOST_ADDRESSES` literals (`:155-157`) — only the five core addresses at offsets 1/3/4/5/6 survive unchanged |
| `test/ts/utils/deployStack.ts`                                       | reads `.protocolConfigAddress` / `.kmsGenerationAddress` at `:54-55` — fields group F deletes — plus `v13` strings at `:1,41,63`. **Not verbatim**; `npm run lint` fails otherwise                                                                                                                                                                 |
| `test/ts/tarball-consumer.test.ts`                                   | the CREATE-layout comment and assertions → the 10-nonce v12 layout                                                                                                                                                                                                                                                                                 |
| `test/ts/acl-owner-upgrade.test.ts`                                  | drop `ProtocolConfig` / `KMSGeneration` from the upgrade batch                                                                                                                                                                                                                                                                                     |
| `test/ts/node10-cjs-resolution.test.ts`                              | one "v13 tarball" message string                                                                                                                                                                                                                                                                                                                   |
| `test/signers.test.ts`                                               | two `.../v13` strings in the test name and assertion message                                                                                                                                                                                                                                                                                       |
| `test/FhevmDeploy.t.sol`                                             | drop the 2 contracts, and apply group D′ to the surviving five                                                                                                                                                                                                                                                                                     |
| `scripts/anvil-lib.sh`                                               | 22 lines / 44 name occurrences (not "25 hits", and it carries no `0.13.x` version string). Beyond deletions: **re-point** the bootstrap smoke checks at `:718-721` from `ProtocolConfig.getKmsSigners()` / `getPublicDecryptionThreshold()` to the `KMSVerifier` equivalents                                                                       |
| `scripts/anvil-local-v1.sh`                                          | "cleartext v13 stack" (`:3`), "~22 at once" (`:108`) — omitted from the first draft of this list                                                                                                                                                                                                                                                   |
| `scripts/anvil-local-v2.sh`                                          | the 2 `create_proxy` calls (`:137-138`), **`anvil_setNonce … $((START_NONCE + 12))` at `:152` → +10** (an arity literal H.3's sweep does not reach), "nonces 0-10" (`:119`), the "✅ 9 …" echoes (`:141,174`)                                                                                                                                      |
| `scripts/anvil-local-v3.sh`                                          | same shape (`:168-169`, "9 proxies placed", "2 empty + 9 real", "0..11")                                                                                                                                                                                                                                                                           |
| `scripts/anvil.sh`, `anvil-fast.sh`, `deploy.sh`                     | version strings and the two contracts' checks; `deploy.sh:85` also hardcodes `CONFIG_PREFIX="fhevm-config-0.13.0/"` (group B)                                                                                                                                                                                                                      |
| `README.md`                                                          | version strings, "21 files" → 16, step 6 rewritten to say v12 is the floor, and the step 7 `plans/FORGE_DEPLOY_SCRIPT_PLAN.md` reference re-pointed or dropped (Phase 1)                                                                                                                                                                           |

**Verbatim**: `test/ts/ethers-adapter.test.ts`, `test/ts/adapter-nonce-diagnostics.test.ts`,
`test/ts/utils/anvil.ts`, `ethUtils.ts`, `ethersEthereumLib.ts`, `viemEthereumLib.ts` (one `v13` comment
string, cosmetic), `scripts/install.sh`, `scripts/pack.sh`, `scripts/derive-keys.sh`,
`scripts/check-vendored-sources.sh` (2 hits, both explanatory comments), `eslint.config.base.js`,
`eslint.config.with-tarball-consumer.js`, `.prettierrc`, `.prettierignore`, `.gitignore`, `foundry.toml`,
`internal/tsconfig.json`, `pkg/ts/tsconfig.json`, `tsconfig*.json` at the root.

Note `test/templates.test.ts` and `test/ts/utils/deployStack.ts` were on an earlier draft of this list
and are **not** verbatim — see the edit table above. `test/ts/adapter-nonce-diagnostics.test.ts:17` uses
the v13 nonce-11 PauserSet address as a stub constant; it touches no chain, so verbatim is fine, but it
is a v13-derived literal worth a comment.

### H. The Foundry deploy layers

Three overlapping deployers, each needing the same shape of edit. Worth noting as v13 duplication rather
than something v12 introduces — a v13 consolidation would shrink this group threefold, but that is a v13
change, not a v12 one.

#### H.1 `pkg/forge/`

| File                             | Edit                                                                                                                                                                                             |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `src/FhevmDeploy.sol`            | drop the 2 imports, 2 `_createProxy` calls, 2 `_create` implementations, 2 `ops` entries, and `_fhevmProtocolConfig` / `_protocolConfigInitData`; `KMSVerifier` init regains signers + threshold |
| `script/FhevmDeployScript.s.sol` | same shape; "Nine proxies, not five" → seven                                                                                                                                                     |
| `script/DeployLocalStack.s.sol`  | same shape                                                                                                                                                                                       |
| `script/VerifyFhevmDeploy.s.sol` | drop the `ProtocolConfig` / `KMSGeneration` `getVersion` and KMS-context checks; verify the signer set on `KMSVerifier` instead                                                                  |

Each of these three also carries an `ACLOwner.Op[]` arity literal — see group H.3, which must be applied
in the same pass. `src/_internal/` is **entirely generated** — no hand edits (Phase 3).

#### H.2 `create2-deploy/`

The riskiest edit in the port, because four **index-aligned ordinal lists** must renumber together. v13
has 9 proxies; v12 has 7, and dropping `ProtocolConfig` (index 5) and `KMSGeneration` (index 6) shifts
`CleartextArithmetic` from 7 to 5 and `CleartextDB` from 8 to 6:

| File / list                                             | v13 | v12 |
| ------------------------------------------------------- | --- | --- |
| `script/FhevmCreate2Base.s.sol` → `_sharedProxyRoles()` | 8   | 6   |
| `script/FhevmCreate2Base.s.sol` → `_allProxyRoles()`    | 9   | 7   |
| `script/FhevmCreate2Base.s.sol` → `_implArtifact(i)`    | 9   | 7   |
| `script/MaterializeInitData.sol` → `initData(i, ...)`   | 9   | 7   |

A partial renumber compiles cleanly and deploys the wrong implementation behind the wrong proxy, so it
must be checked as one unit — `FhevmStatus.s.sol` and `FhevmVerify.s.sol` are what catch it.

The rest:

| File                                             | Edit                                                                                                                                                                                                                                                                                                                                                                                           |
| ------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script/FhevmCreate2Base.s.sol`                  | drop `R_PROTOCOL_CONFIG`, `R_KMS_GENERATION`; `FHEVM_VERSION` doc `"0.13"` → `"0.12"`                                                                                                                                                                                                                                                                                                          |
| `script/MaterializeInitData.sol`                 | drop **4** imports (`ProtocolConfig`, `KMSGeneration`, `IProtocolConfig`, and `KmsNode` from `shared/Structs.sol` — `:11,12,15,16`), the `i == 5` and `i == 6` branches, the index doc-comment (`:63-65`), `_protocolConfig()`, `_kmsNodes()`, `_kmsThresholds()`; `_kmsVerifier()` becomes the 4-arg v12 call using `LocalHostBootstrap.kmsSigners()` and `LocalHostBootstrap.KMS_NODE_COUNT` |
| `script/Interfaces.sol`                          | drop `IWiredProtocolConfig`                                                                                                                                                                                                                                                                                                                                                                    |
| `script/FhevmVerify.s.sol`                       | drop the `ProtocolConfig` threshold/signer block; check `KMSVerifier.getKmsSigners()` and `getThreshold()` instead                                                                                                                                                                                                                                                                             |
| `deploy-testnet.ts:215`, `deploy-testnet.sh:107` | `FHEVM_VERSION` → `'0.12'`                                                                                                                                                                                                                                                                                                                                                                     |
| `deploy-testnet.ts:309-310`                      | drop the 2 address names                                                                                                                                                                                                                                                                                                                                                                       |
| `GUIDE.md`, `README.md`                          | version strings and the 22-creates / 9-proxies counts                                                                                                                                                                                                                                                                                                                                          |

**No code changes** (prose only, per H.3): `FhevmDeployCreates.s.sol`,
`FhevmOfferACLOwnership.s.sol`, `FhevmAcceptACLOwnership.s.sol`, `FhevmOfferACLOwnerToAdmin.s.sol`,
`FhevmAcceptOwnershipAsAdmin.s.sol`, `FhevmRegisterPausers.s.sol`. Each names neither dropped contract
and holds no arity literal, but most carry a "all nine proxies"-style comment that H.3's prose sweep
picks up — so "verbatim" would overstate it.

**Genuinely verbatim**: `utils.ts`, `anvil-config.json`, `tsconfig.json`.

`FhevmComputeCreate2Addresses.s.sol`, `FhevmMaterializeStack.s.sol` and `FhevmStatus.s.sol` are in
neither list: they name neither contract, but they hardcode the proxy count. See H.3.

`LocalHostBootstrap.kmsSigners()` and `KMS_NODE_COUNT` already exist in the generated output, so v12's
`_kmsVerifier()` needs no new generator support; `kmsIpAddresses()`, `kmsStorageUrls()` and
`kmsTxSenders()` simply go unused.

The salt derivation bakes `FHEVM_VERSION` (`"0.12"` vs `"0.13"`), so **v12 and v13 CREATE2 addresses
differ by construction** and the two generations can coexist on one testnet without colliding. That is
the mechanism working as designed, not a divergence to reconcile.

#### H.3 The arity sweep — ~~9 proxies → 7, 22 creates → 18~~ NO LONGER NEEDED

> **Delivered by Phase 0.3 item B.** All 15 executable sites now derive their arity, and the ~75 prose
> sites are count-free (item C). v12 inherits correct counts from its own tables with no edit. Kept as
> the record of what the literals were, and as the explanation for why H.2's ordinal renumber is still
> the port's riskiest step — that one is _not_ fixed by item B.

Dropping two proxies changes two counts that v13 spells out as **literals in 15 executable places**.
These are invisible to a name-based grep — no file here mentions `ProtocolConfig` — and getting one wrong
does not fail to compile:

> `new ACLOwner.Op[](9)` with only 7 ops filled leaves two `Op(address(0), address(0), "")` entries.
> `ACLOwner.upgrade` then calls `upgradeToAndCall` on the zero address. Best case it reverts and the whole
> atomic materialization fails; worst case an ops array built by a loop silently shifts and materializes
> the wrong implementation behind the wrong proxy.

So this group is not cosmetic and not optional. The full executable set:

| File                                                       | Line | v13                            | v12           |
| ---------------------------------------------------------- | ---- | ------------------------------ | ------------- |
| `pkg/forge/src/FhevmDeploy.sol`                            | 321  | `new address[](9)`             | 7             |
| `pkg/forge/src/FhevmDeploy.sol`                            | 347  | `new ACLOwner.Op[](9)`         | 7             |
| `pkg/forge/script/FhevmDeployScript.s.sol`                 | 275  | `new ACLOwner.Op[](9)`         | 7             |
| `pkg/forge/script/DeployLocalStack.s.sol`                  | 212  | `new ACLOwner.Op[](9)`         | 7             |
| `create2-deploy/script/FhevmCreate2Base.s.sol`             | 96   | `new string[](8)`              | 6             |
| `create2-deploy/script/FhevmCreate2Base.s.sol`             | 110  | `new string[](9)`              | 7             |
| `create2-deploy/script/FhevmCreate2Base.s.sol`             | 296  | `new Create[](22)`             | 18            |
| `create2-deploy/script/FhevmComputeCreate2Addresses.s.sol` | 126  | `new address[](8)`             | 6             |
| `create2-deploy/script/FhevmComputeCreate2Addresses.s.sol` | 146  | `new address[](9)`             | 7             |
| `create2-deploy/script/FhevmComputeCreate2Addresses.s.sol` | 230  | `new address[](9)`             | 7             |
| `create2-deploy/script/FhevmComputeCreate2Addresses.s.sol` | 266  | `require(rest.length == 9, …)` | 7 (+ message) |
| `create2-deploy/script/FhevmComputeCreate2Addresses.s.sol` | 302  | `new address[](9)`             | 7             |
| `create2-deploy/script/FhevmMaterializeStack.s.sol`        | 79   | `materialized == 9`            | 7             |
| `create2-deploy/script/FhevmMaterializeStack.s.sol`        | 118  | `new IACLOwner.Op[](9)`        | 7             |
| `create2-deploy/script/FhevmStatus.s.sol`                  | 230  | `live == 9`                    | 7             |

Line numbers are against the Phase 0.1 baseline commit; re-derive them rather than trusting them if v13
moved. The sweep that finds all of it, prose included:

```sh
grep -rnE "\[\]\((8|9|22)\)|== ?(8|9|22)\b|\b(nine|eight)\b|\b22 (creates|CREATE2s)" \
  pkg/forge create2-deploy/script create2-deploy/deploy-testnet.{ts,sh} scripts/anvil-lib.sh
```

Beyond the 15 above it reports roughly **75 prose sites** — "all nine proxies", "the eight share ONE
init-code hash", "22 CREATE2s through the factory", "all ten markers" — across `FhevmCreate2Base.s.sol`,
`FhevmComputeCreate2Addresses.s.sol`, `FhevmMaterializeStack.s.sol`, `FhevmStatus.s.sol`,
`FhevmVerify.s.sol`, `FhevmOfferACLOwnership.s.sol`, `FhevmAcceptACLOwnership.s.sol`,
`FhevmOfferACLOwnerToAdmin.s.sol`, `MaterializeInitData.sol`, `deploy-testnet.ts`, `deploy-testnet.sh`,
`anvil-lib.sh` and the `pkg/forge` scripts. Those change no behaviour, but leaving them is what makes the
next reader trust a wrong number, so they are part of the group rather than a follow-up.

**The durable fix belongs in v13, not here.** One number spelled out 15 times is the defect; v12 is
merely what exposes it. Extracting a `PROXY_COUNT` / `CREATE_COUNT` constant per layer — or deriving the
arity from `_allProxyRoles().length`, which several of these call sites already have in scope — would
make the whole group a zero-line diff for every future generation. Doing that in v13 first (rule 20) and
then copying down is strictly better than a 90-site hand sweep repeated per generation. It is scoped out
of Phase 0.2 only because it touches four deploy layers and wants its own review; if it lands first,
delete this subsection.

## Phase 3 — regenerate

Nothing below is hand-edited. Every file here is written by a generator that reads the tables edited in
Phase 2, which is why groups A–H are as small as they are — roughly 60 files regenerate themselves.

```sh
cd sdk/host-contracts-cleartext/v12
npm run build:templates          # generate:cleartext-config → generate:compute-addresses →
                                 # generate:placeholders → forge build → templates → signers →
                                 # generate:local-host-bytecode
npm run generate:patch-sites     # must run AFTER build:templates — it reads the templates
npm run generate:genesis         # pkg/state/genesis.json — input to scripts/anvil-fast.sh
```

`generate:genesis` is easy to miss: its output is untracked and absent from v13's tree today, so nothing
in `npm run test` notices. But `scripts/anvil-fast.sh` — which group G edits — consumes it, and
`internal/generateGenesis.ts` carries the `implementationCount !== 9` arity check that group D fixes. If
the fast-anvil path is deferred for v12, say so explicitly rather than leaving a script with no input.

| Generated output                                                      | Count | Generator                           |
| --------------------------------------------------------------------- | ----- | ----------------------------------- |
| `pkg/abi/*.json`                                                      | 14→12 | `generateTemplates.ts`              |
| `pkg/templates/*.json`                                                | 14→12 | `generateTemplates.ts`              |
| `pkg/ts/artifacts/*.ts`                                               | 15→13 | `generateTemplates.ts`              |
| `pkg/ts/signers/*.ts`                                                 | 3     | `generateSigners.ts`                |
| `pkg/ts/cleartext-config.ts`                                          | 1     | `copyCleartextConfig.ts`            |
| `pkg/forge/script/ComputeAddresses.s.sol`                             | 1     | `generateComputeAddressesScript.ts` |
| `pkg/forge/src/_internal/LocalHost{Bytecode,Addresses,Bootstrap}.sol` | 3     | `generateLocalHostBytecode.ts`      |
| `pkg/forge/src/_internal/interfaces/I*.sol`                           | 14→12 | `generateLocalHostBytecode.ts`      |
| `internal/placeholders/addresses.sol`                                 | 1     | `generatePlaceholders.ts`           |
| `internal/placeholders/patch-sites.json`                              | 1     | `generatePatchSites.ts`             |

**Review the `patch-sites.json` diff rather than accepting it.** A count of 0 for an address the
contracts still use means the deploy would bake in a placeholder — a stack that deploys and then calls
nothing.

**No orphan cleanup is needed** — an earlier draft of this plan said otherwise and was wrong.
`generateTemplates.ts` starts by `rmSync`-ing `pkg/abi/`, `pkg/templates/` and `pkg/ts/artifacts/`
wholesale and rebuilding them from `TARGET_CONTRACTS`, so a contract dropped from that list has its
artifacts removed automatically. The same is true of `pkg/forge/src/_internal/interfaces/`.

The corollary is worth knowing before adding any generator: **anything else written into those four
directories is deleted on the next build.** `pkg/ts/versions.ts` sits at `pkg/ts/` rather than
`pkg/ts/artifacts/` for exactly this reason.

## Phase 4 — verify — DONE

Both generations green after the port:

| Gate                               | v13                       | v12                       |
| ---------------------------------- | ------------------------- | ------------------------- |
| `lint` (eslint + tsc)              | ✅                        | ✅                        |
| `forge test`                       | 42/42                     | 18/18                     |
| template + signer tests            | 11/11                     | 11/11                     |
| `check:vendored` (rule 6)          | ✅ 21 files @ `v0.13.2`   | ✅ 16 files @ `v0.12.5`   |
| `check:zama-config` (rules 15/17)  | ✅                        | ✅                        |
| `prettier:check`                   | ✅                        | ✅                        |
| `test:tarball`                     | 19/19 (9 files)           | 17/17 (7 files)           |
| `forge build --sizes` (rule 12)    | 22,994 B / 1,582 B margin | 19,764 B / 4,812 B margin |
| `./scripts/anvil.sh` (bash deploy) | 58 passed / 0 failed      | 40 passed / 0 failed      |

### The CREATE2 path: verified by test AND by a real run

Both, in the end. The alignment test came first; then `create2-deploy` was changed so a local anvil
rehearsal needs no keystore, and the full path was actually run.

**Keystore-free on anvil only.** `--account` is now optional, and omitting it uses accounts 0 and 1 of
anvil's public mnemonic (0 deploys, 1 is the admin, which `--admin` also defaults to). The gate is
`anvil_nodeInfo` — a method anvil answers and every other node rejects with -32601. Chain id is
deliberately _not_ the test: the documented rehearsal runs on `--chain-id 11155111` (31337 is excluded
from the allow-list because it is the nonce path's chain) and a fork inherits the upstream id, so a
31337 check would have missed exactly the case this is for. Plan §12's keystore-only rule still binds
every other chain, with an explanatory refusal.

Result, in both generations: `--stage all` completes and `verify` reports **"OK — every terminal
condition met."**

**The run found a bug the test could not.** v12's `compute` died with `panic: array out-of-bounds
access (0x32)`. The Phase 0.3 item-B sweep had derived every array _size_ but left **12 hardcoded loop
bounds and one index literal** — `for (uint256 i = 0; i < 8; i++)`, `rest[8] = pauserSetAdd`,
`proxyRoles[8]` — none of which match a `[](N)` or `== N` pattern. In v13 they are all correct **by
coincidence**, because `_sharedProxyRoles().length` happens to equal 8 and `_allProxyRoles().length` 9.
In v12 every one of them was wrong.

All 13 now derive from the arrays they walk (`roles.length`, `proxies.length`, `impls.length`,
`proxyRoles.length`, `m.length`, `shared.length`), fixed in v13 first and ported. This is the third
distinct sub-class of the same defect — array sizes, then `deploy.sh`'s nonce arithmetic, now loop
bounds — which is the argument for Phase 0.3's Tier 3: while the stack's shape is written down six
times, each sweep finds only the spellings it thought to look for.

### Also verified by test

The keystore accounts the testnet driver needs are password-protected, so `deploy-testnet.sh` could not
be run here. That turned out to be the better outcome: the property actually at risk in H.2 is the
**four-list index alignment**, and a one-off anvil run would have verified it once and left it unguarded
afterwards. It is now a test instead — `test/Create2Ordinals.t.sol`, added to v13 first and copied down:

- the two role lists agree, and the longer is the shorter plus ACL
- no role is left unset (the array sizes are literals, so an assignment can be forgotten)
- roles and implementation artifacts match a hand-written oracle, position by position
- every artifact path resolves through `vm.getCode`
- `initData(i)` covers every position, and out-of-range reverts on both lists
- the create count tracks the proxy count

7 tests, passing in both generations — 9 positions in v13, 7 in v12. Before this, the only thing that
caught a partial renumber was an actual deploy reaching `FhevmStatus`/`FhevmVerify`, which needs a funded
key and a node. A full testnet rehearsal is still worth doing before any real deploy; it is now a
confirmation rather than the only line of defence.

### The original gate list

In order. Each gate is cheap and each one fails differently, so running them out of order wastes the
diagnosis.

```sh
cd sdk/host-contracts-cleartext/v12

npm run check:vendored      # rule 6/18: pkg/src/contracts byte-identical to v0.12.5 (ac18e49e)
                            #   expect: ✅ 16 vendored files identical to upstream
npm run check:zama-config   # rules 15/17: ZAMA_LOCAL_CONFIG matches ZamaConfig.sol _getLocalConfig()
forge build --sizes         # rule 12: every runtime under 24,576 B — v12's executor is smaller than
                            #   v13's, so this should pass with more headroom, not less
npm run lint
npm run test                # templates + forge + tarball. No upgrade e2e (rule 21 floor).
./scripts/anvil.sh          # rule 17: default deploy lands on the three ZamaConfig addresses
```

Then each deploy path end to end, since "deployable from scratch" is the actual goal:

```sh
./scripts/anvil.sh                          # TS path + nonce-based forge path
cd create2-deploy && ./deploy-testnet.sh --chain anvil   # CREATE2 path against a local anvil
```

The CREATE2 run is what exercises the H.2 renumber and the H.3 arities — the two edits that compile
cleanly when wrong. `FhevmStatus.s.sol` and `FhevmVerify.s.sol` are the gate: an ordinal misalignment or
a leftover zero-address op surfaces there as a wrong implementation behind a proxy, and nowhere earlier.
`./scripts/anvil.sh` covers the same for the nonce-based path, since `VerifyFhevmDeploy.s.sol` checks
every proxy's implementation and wiring.

**Verification stops at local anvil.** `--chain anvil` is the whole of Phase 4 — it exercises the same
scripts, salts and ordinals as a real run, so nothing is left unchecked by staying local. A deploy to an
actual testnet is a separate step, outside this plan, and needs approval first (see the hard constraint
above): CREATE2 consumes salts at fixed addresses, so a wrong run cannot be re-done at the same
addresses.

Finally, the rule 20 gate — read the diff in full:

```sh
diff -r ../v13 . \
  --exclude=node_modules --exclude=dependencies --exclude=out --exclude=cache \
  --exclude=tarball --exclude='.out*' --exclude=broadcast --exclude=state \
  --exclude=_cjs --exclude=_esm --exclude=_types --exclude='*.tsbuildinfo' \
  --exclude='.deploy-config' --exclude='.tmp-localhost' --exclude='.vitest-cache'
```

The build-output excludes are not optional — without them the gate drowns in `_cjs`/`_esm`/`_types`,
`broadcast/` and the tarball-consumer fixture under `test/ts/node_modules`, and "read the diff in full"
stops being something anyone does.

Every hunk must map to an Appendix A row. One that does not is either a missed edit or v13 drift.

## Phase 5 — make `updateV12ToV13` fully testable — DONE

**The upgrade e2e runs and passes.** `npm run test:upgrade-e2e` → 2/2: a v12 stack is deployed at the
canonical addresses (ACL at `0x50157CFf…`, so v12's own default deploy is landing correctly), then
upgraded to v13, with the cleartext round-trip surviving the migration. The second test is the one that
matters most for the port: it resolves the migration config with no operator input, by reading
`getCurrentKmsContextId` / `getKmsSigners` / `getThreshold` off the **live v12 KMSVerifier** — the three
reads Phase C predicted would work at `v0.12.5`.

`npm run list:upgrade-ops -- ../v12` now reads the real v12 artifacts and reports **2 materializations,
5 reinitializations**, in exact agreement with `pkg/ts/upgrade.ts`'s 7 targets and every reinitializer
version. Three rows confirm Phase 2 decisions independently:

| Row                                                                     | Confirms                                                                                     |
| ----------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| `CleartextArithmetic  CHANGED  - -> reinitializeV2`                     | group E: v12 has no reinitializer, so v13's `reinitializeV2` op has something to step _from_ |
| `CleartextInputVerifier  same  reinitializeV2 -> reinitializeV2  no op` | the README's worked example — deliberately absent from the op list                           |
| `CleartextDB  same  no op`                                              | correctly omitted from the upgrade                                                           |

### Three things had to be fixed first, and two were mine

1. **The Phase 0.2 `npm pack` fix had never been applied.** `internal/prepareTestV12Consumer.ts` packed
   `V12_PACKAGE_ROOT` — the harness root, whose manifest is `private: true`. It now packs
   `<v12>/pkg`, matching `createPackageTarball.ts`. Identified in this plan and left undone; harmless
   while v12 was flat, wrong the moment it gained the rule 9 split.
2. **The Phase 0.3 item-A rename broke the e2e's import.** It asked the v12 fixture for
   `BootstrapConfigV12`; v12 exports `BootstrapConfig`. Now imported unsuffixed and **aliased at the
   import site** — which is the right shape for this file, the one place both generations are in scope:
   the package specifier says which generation it is, and the alias is local.
3. **`node10-cjs-resolution.test.ts` contradicted `createPackageTarball.ts`.** The latter documents
   `tarball/` as deliberately shared with the previous generation; the former required exactly one file
   in it. Its comment already said "pick by name rather than taking the only entry" — but the filter was
   the name _prefix_, and both generations publish under the same npm name. It now pins the exact
   `fhevm-host-contracts-cleartext-<version>.tgz` read from the payload manifest, which is a stronger
   guard than the count ever was: a stale tarball is now a miss instead of a coin flip.

Worth noting that (3) was invisible until this phase: the e2e is the only thing that puts a second
tarball in that directory, and before v12 existed the e2e always self-skipped. A test that cannot run is
a test that cannot disagree with its neighbours.

### Verification, both generations

|                          | v13             | v12                 |
| ------------------------ | --------------- | ------------------- |
| `forge test`             | 42/42           | 18/18               |
| template + signer tests  | 11/11           | 11/11               |
| vitest suite             | 19/19 (9 files) | 17/17 (7 files)     |
| `test:upgrade-e2e`       | **2/2**         | n/a (rule 21 floor) |
| `prettier:check`, `lint` | ✅              | ✅                  |

### The original readiness notes

Out of scope here, unblocked by it. Once v12 builds and packs, v13's upgrade path stops self-skipping:

- `internal/constants.ts` already resolves the previous generation as `../v12`, and
  `PREVIOUS_GENERATION_FIXTURE_ALIAS` as `@fhevm/host-contracts-cleartext-v12`. Both are correct as they
  stand — the directory was simply empty.
- `internal/runUpgradeE2e.ts` gates on `../v12/node_modules` existing, which Phase 1.1 satisfies.
- Phase 0.2's `npm pack` fix is what makes `prepareTestV12Consumer.ts` produce a usable fixture.
- **Verified**: v0.12.5's `KMSVerifier` exposes `getCurrentKmsContextId()`, `getKmsSigners()` and
  `getThreshold()` — exactly the three reads `updateV12ToV13`'s `resolveDefaultMigration` performs. The
  v13 side needs no change to consume the new v12.
- v12 exports `deploy`, `precomputeAddresses` and `BootstrapConfigV12`, which is what
  `test/ts/upgrade-e2e.test.ts` imports from the fixture.

So Phase 5 is: run `npm run test:upgrade-e2e` in v13, and fix what it reports. The e2e **skips rather
than fails** when the fixture is missing, so confirm it actually ran — a green v13 suite today says
nothing about the upgrade path.

Two things Phase 5 should check rather than assume, both of which v12's Phase 2 choices bear on:

1. `updateV12ToV13` re-points `CleartextArithmetic` via `reinitializeV2` because v12's arithmetic lacks
   the nary selectors. Group E keeps that true — the versions (`MINOR_VERSION` 3, `REINITIALIZER_VERSION` 2) are what the upgrade steps _from_.
2. `updateV12ToV13` deliberately omits `InputVerifier`, on the grounds that its v13 bytecode is
   identical. `npm run list:upgrade-ops -- ../v12` re-checks that against the real v12 artifacts instead
   of the pre-split directory's.

## Appendix A — divergence register

The complete list of legitimate v13 ↔ v12 differences. Anything in `diff -r` not attributable to a row
here is drift.

| Cause                                               | Group | Rule   |
| --------------------------------------------------- | ----- | ------ |
| Package version `0.12.0`                            | A     | 5      |
| Vendored tag/commit `v0.12.5` / `ac18e49e`          | A     | 7, 18  |
| Config prefix `fhevm-config-0.12.0/`                | B     | 11, 20 |
| 16 vendored files, not 21                           | C     | 6, 20  |
| 8 host addresses, not 10                            | D     | 20     |
| Five contracts one minor version lower              | D′    | 6, 20  |
| 10-nonce deploy layout, not 12                      | D     | 17, 20 |
| No nary operators in the cleartext layer            | E     | 20     |
| `KMSVerifier` holds its own signer set              | F, H  | 20     |
| No `ProtocolConfig` / `KMSGeneration`               | D–H   | 20     |
| No upgrade path (floor)                             | F, G  | 21     |
| `FHEVM_VERSION = "0.12"` in the CREATE2 salt        | H.2   | 20     |
| 7 proxies not 9, 18 creates not 22                  | H.3   | 20     |
| No `plans/` directory                               | 1     | —      |
| No `create2-deploy/upgrade-testnet.ts`              | —     | 21     |
| No `create2-deploy/script/FhevmVerifyUpgrade.s.sol` | —     | 21     |

**`test/e2e/create2-upgrade.test.ts` is v13-only for the same reason `test/ts/upgrade-e2e.test.ts` is.**
It drives a fresh anvil, a v12 stack deployed by v12's own CREATE2 coordinator, and then v13's coordinator
upgrading it — a _cross-generation_ test, which by rule 21 belongs to the newer generation. v12 has nothing
to upgrade from, so a copy there would have no second stack to point at. Note the asymmetry it creates and
accept it deliberately: the test lives in v13 but exercises v12's `create2-deploy/deploy-testnet.ts` end to
end, so v12's deploy coordinator is covered from a directory v12 does not contain. That is the same shape
as the TypeScript e2e and is why rule 20 makes v13 the place the work happens.

**`pkg/ts/create2Addresses.ts` is ported, and differs only by the two missing roles.**
`precomputeCreate2Addresses` predicts the CREATE2 address set — the deterministic-deployment counterpart to
`precomputeAddresses` — and is not upgrade-path work, so rule 22 requires it here. `CREATE2_ROLES` loses
`protocolConfig` and `kmsGeneration`, so its seal carries eleven roles rather than thirteen; the derivation
itself is byte-identical, because the salt and init-code rules do not vary by generation.

`AbstractEthereumUtils` gains the same three members (`keccak256`, `encodeAbiParameters`,
`getCreate2Address`) in both payloads. They are REQUIRED, not optional, which is a breaking change for any
consumer implementing that interface — recorded here because it is the only place in this port where the
published surface got stricter rather than larger.

**`pkg/ts/verify.ts` is ported, and differs only by this generation's shape.** The public `verify` /
`snapshotStack` pair is not upgrade-path work — it checks a stack of whatever generation it ships in — so
rule 22 requires it here, and it is present with the same exports, the same report shape and the same
ABI-enumerated survey. The 51 differing lines are all one cause: v12 has no `ProtocolConfig` and no
`KMSGeneration`, so those two drop out of the target table and the KMS signer set, context id and threshold
are read off `KMSVerifier` instead. That also makes `VerifyExpectations.kmsThreshold` a scalar here against
v13's four-field `kmsThresholds`, which is the same F-group divergence the register already records.

`test/templates.test.ts`'s check that `DEFAULT_MAY_CHANGE` agrees with the proxies `updateV12ToV13`
re-points is **v13-only**, and unavoidably so: it parses `pkg/ts/upgrade.ts`, which does not exist here
(rule 21). The list it guards is still carried, so if this generation ever gains an upgrade path the check
should be ported with it.

`DEFAULT_MAY_CHANGE` is carried unchanged even though this generation is the floor and has nothing to
upgrade _from_. `mode: 'upgrade'` is still reachable — verifying a stack against a snapshot of itself, which
the ported test does — and keeping the list identical is what makes the two files diffable line by line.

**The two upgrade verifies are v13-only** — `create2-deploy/upgrade-testnet.ts` and
`create2-deploy/script/FhevmVerifyUpgrade.s.sol` — and that is rule 21 rather than an omission: v12 is
the floor, so it has nothing to upgrade _from_ and ships no upgrade path in any form — no
`pkg/ts/upgrade.ts`, no upgrade e2e, and no CREATE2 upgrade coordinator either. Rule 22 requires every
v13 change to reach v12 in the same change _unless the generation makes it impossible_, and this is that
case, recorded here as the rule demands.

What IS ported from that same work is the _shared_ half, and the split was drawn with this port in mind.
`create2-deploy/script/FhevmVerifyBase.s.sol` — the reporting primitives, the comparison helpers that
print both sides on failure, and the two checks that are the same question for any deployment (is the
canonical factory there, does every role hold code, does every proxy point at its sealed implementation,
does every baked-in address match the manifest) — is byte-identical in both generations, and v12's
`FhevmVerify.s.sol` consumes it. Only the concrete upgrade verify is absent. Extracting the base _before_
the second consumer existed is what made that possible: with one caller there was nothing to reconcile,
so the line could be drawn at "what a check is" versus "which checks to run" rather than wherever two
copies had already diverged.

Note what else IS ported: `create2-deploy/common.ts` carries the upgrade's option plumbing —
`ExistingAddresses`, `EXISTING_ROLES`, `--handle`, `--migration` — even though nothing in v12 reads it.
Keeping the shared file byte-identical apart from the two generation deltas is what keeps the diff
auditable, and when v11 lands v12 gains an upgrade path and will need exactly that plumbing. `EXISTING_ROLES`
is marked unused-in-this-generation, with a warning that its contents are v13-shaped and must be
re-derived for whatever stack a future v12 upgrade reads _from_.

**One further structural divergence, deliberately taken:** `v12/` has no `plans/`. It is documentation only —
outside the payload (rule 16), outside every tsconfig and eslint scope, and referenced from nothing but
one README parenthetical — so it changes no behaviour. Everything that _does_ affect behaviour is
identical: the harness layout, the `pkg/` payload split, the tsconfigs, eslint and prettier config,
`scripts/`, the `internal/` generators and the test structure, as rule 20 requires.

## Appendix B — what must stay byte-identical

The trap in this port is _restoring_ an older shape from the pre-split
`sdk/host-contracts-cleartext-v12` because it happens to be v12-flavoured. That directory predates
several v13 improvements that are not generation-specific, and rule 20 requires v12 to carry them. Take
these from **v13**, never from the old directory:

- `pkg/src/upgrade/ACLOwner.sol` — v13's `execute`, `pause`/`unpause`, `IACLPausable`
- `pkg/src/cleartext/CleartextDB.sol` — `getACLAddress()`
- `pkg/src/cleartext/CleartextFHEVMExecutor.sol` — `getCleartextArithmeticAddress()`
- `pkg/ts/aclOwner.ts` — `pauseACL` / `unpauseACL`
- `pkg/ts/types/public.ts` — the `AbstractEthereumSigner` nonce/inclusion contract, and
  `AbstractEthereumProvider`'s `readContract` / `getTransactionCount`
- `pkg/ts/utils.ts` — `assertDeployedAddress`, `assertNoCodeAtTargets`, `sendStep`
- the entire `pkg/forge/` layer and `create2-deploy/` — neither exists in the old directory at all
- `internal/cleartext-config.ts`, `internal/checkZamaLocalConfig.ts`, `internal/generateGenesis.ts`,
  `internal/listUpgradeOps.ts`, `scripts/check-vendored-sources.sh` — all newer than the old directory

"Take from v13" here means _provenance_, not "do not edit": `internal/generateGenesis.ts` still needs its
group D arity fix, and `internal/createPackageTarball.ts:35` carries a comment referencing the deleted
`prepareTestV12Consumer.ts`. Similarly `pkg/src/upgrade/ACLOwner.sol:107` cites
`ProtocolConfig.defineNewKmsContext` as its `execute` example — a comment, so cosmetic, but it names a
contract v12 does not have.

The pre-split `sdk/host-contracts-cleartext-v12` is useful for exactly two things, both already
exploited above: cross-checking the Phase C extraction (its `src/contracts` is **verified** byte-identical
to `v0.12.5`), and confirming Phase D's nonce layout against its `precomputeAddresses`. Once v12 passes
Phase 4, retire it.
