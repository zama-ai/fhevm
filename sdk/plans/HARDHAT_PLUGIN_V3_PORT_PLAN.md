# Porting the FHEVM hardhat plugin from hardhat v2 to hardhat v3

Status: **plan for review — Stage A2 landed, nothing else implemented.** The landing zone exists: the
`hardhat/v3` cluster (own installation root, hardhat 3.15 pinned, hello-world plugin proving the
topology, tasks, and the single-instance guarantee end to end), plus A2's per-connection `fhevm`
object attached via the `network` hooks. §2, §2b, §5 and Stage A were revised after reading the
hardhat 3.15 SOURCE (`sdk/hardhat/v3/node_modules/hardhat/src`), not only the docs.

## The goals (the charter — every decision below serves one of these)

1. Port the FHEVM hardhat plugin for hh v2 to hh v3.
2. **Minimize the amount of code in hh v3.**
3. Keep the same public-facing API — except breaking changes hardhat v3 itself forces.
4. Keep `hardhat node` support as in the v2 plugin.
5. Keep anvil node support as in the v2 plugin.
6. Keep small functions.
7. Keep centralized constants — auto-generated via fhevm-npm code generation where needed.
8. Use vendored code where needed.

## 1. What the v2 plugin is (inventory to port against)

**The plugin's essential job**: pre-deploy the FHEVM contract set before every node-related command
launch (`hardhat test`, `hardhat node`, runs against anvil, ...), so user code always finds a live
stack at the expected addresses. Everything else — the `hre.fhevm` API, tasks, provider hooks — sits
on top of that guarantee. The port stands or falls on reproducing it in hardhat 3's lifecycle: the
natural v3 home is the `network.newConnection` chain (deploy once per development-class CHAIN —
every in-process connection is a fresh chain; an `http` dev connection reuses what is already
there), which Stage B proves for all three local targets.

~7,150 lines across 44 files in `hardhat/v2/plugin/pkg/src`. Four layers:

- **Wiring** (`index.ts`, `type-extensions.ts`): `extendConfig` + `extendEnvironment` (attaches
  `hre.fhevm`) + `extendProvider` + side-effect task imports. All three extension mechanisms are
  GONE in hardhat 3 — this layer is a full rewrite, and it is small.
- **Public API** (`FhevmExternalAPI.ts`, `types.ts` → `HardhatFhevmRuntimeEnvironment`):
  `isMock/isCleartext/isDevelopment`, `debugger`, `client` (an `@fhevm/sdk` FhevmClient),
  `createEncryptedInput`/`encryptUint|Bool|Address`, `publicDecrypt*`, `userDecrypt*`,
  `createEIP712`/`createDelegatedUserDecryptEIP712`, `typeof`, error helpers
  (`tryParseFhevmError`, `revertedWithCustomErrorArgs`), `parseCoprocessorEvents`,
  `computeTransactionHCU`, `getCoprocessorConfig`/`assertCoprocessorInitialized`; plus module
  exports `getHCU`, `timestampNow`, `FhevmType`, the type surface. **This is the contract goal 3
  protects.**
- **Tasks**: the `fhevm` scope (`user-decrypt`, `public-decrypt`, `check-fhevm-compatibility`,
  `resolve-fhevm-config` stub) and five builtin overrides (`test`, `clean`, compile remappings,
  compile source paths, `node`'s provider + server-ready hooks — the `hardhat node` support).
- **Environment/adapters**: `FhevmEnvironment` (network detection: in-process `hardhat`,
  `--network localhost` against a `hardhat node`, named `anvil`, public chains via
  `@fhevm/sdk/chains`), provider wrapper, deploy/setup for the cleartext stack, HCU tables,
  handle/type helpers, **`internal/FhevmEnvironmentPaths.ts` — KEEP (explicitly protected from the
  delete triage)**: the consumer-tree path resolution (`resolveFromConsumer` + the paths class) that
  locates sibling npm modules from the USER's project — `@fhevm/solidity` and its `ZamaConfig.sol`,
  `@fhevm/sdk` (pnpm nested-store aware), `solidity-coverage`, the consumer's `node_modules` root.
  Deploy, config checks and compile integration all stand on it; in v3 it anchors on
  `context.config.paths.root` (the `HookContext` is the HRE minus `tasks`, and hardhat resolves
  `root` to the closest npm package root). Also `internal/constants.ts` (113 lines — includes HAND-COPIED addresses:
  the ZamaConfig trio, Sepolia gateway `DECRYPTION_ADDRESS`/`INPUT_VERIFICATION_ADDRESS`),
  `internal/vendored/ethersEthereumLib.ts` (from common-vendored).

## 2. Hardhat 3 plugin API — the facts (from the hardhat 3.15 source)

Confirmed against `node_modules/hardhat/src` (`types/*`, `internal/core/*`,
`internal/builtin-plugins/{network-manager,node,test,clean,solidity}`):

- A plugin is an OBJECT wrapped in `definePlugin` from `hardhat/plugins`: `{ id, npmPackage,
dependencies, conditionalDependencies, hookHandlers, globalOptions, tasks }`. Hook-category
  factories and task actions are LAZY `() => import(...)` modules; each factory runs at most once
  per HRE. `definePlugin` registers the id so the CLI can warn about a plugin that is imported but
  missing from the user's `plugins` array — a bare object loses that warning.
- Plugin order is a reverse topological sort: the 13 builtins first (`network-manager` and `node`
  among them), then the user's `plugins` in declaration order.
- Hook categories: `config` (`extendUserConfig`, `validateUserConfig`, `resolveUserConfig`,
  `validateResolvedConfig` — run BEFORE the hook context exists), `hre` (`created`), `network`
  (`newConnection`, `closeConnection`, `onRequest`, `onCoverageData`, `onGasMeasurement`), `test`
  (`registerFileForTestRunner`, `onTestRunStart/WorkerDone/RunDone`), `solidity` (incl. a
  remappings hook), `configurationVariables`, `userInterruptions`.
- Chained hooks receive `(context, ...args, next)`; order is dynamic handlers first, then plugins in
  REVERSE resolved order, then the default. Sequential hooks (`hre.created`) run in FORWARD order,
  so a user plugin's `created` sees `hre.network` already attached. `next` is called at most once.
- `HookContext` is the HRE with `tasks` removed (prototype-linked): it carries `config`,
  `userConfig`, `globalOptions`, `hooks`, `artifacts`, `network`, `solidity`.
- **Per-connection state**: `newConnection` is a DECORATOR chain — `await next(context)`, attach
  to the returned `NetworkConnection`, return the SAME object. `closeConnection` mirrors it. A
  `WeakMap` keyed by the connection is what the builtin network-manager uses for its own
  per-connection state. A2 landed on exactly this.
- `NetworkConnection` = `{ id, networkName, networkConfig, chainType, provider, close() }`.
  `NetworkConfig` is a discriminated union: `type: "edr-simulated"` (in-process EDR, default
  `chainId 31337`, every `create()` is a FRESH chain) or `type: "http"` (`url`, `timeout`,
  `httpHeaders`). `chainType` is hardhat's `generic | l1 | op` axis — irrelevant to us.
- `hre.network` = `NetworkManager`: `create` (always a new connection), `getOrCreate` (cached by
  network name + chain type, mutex-guarded), deprecated `connect` (= `create`), `createServer`
  (JSON-RPC server over a fresh connection, `edr-simulated` only). No-argument calls resolve to
  `--network` or the network literally named `default`.
- Type extensions: `declare module "hardhat/types/network"` for `NetworkConnection`,
  `hardhat/types/hre` for the HRE, `hardhat/types/config` for config keys — all in the package
  `exports` map. The plugin's `index.ts` re-exports `export type * from "./type-extensions.js"`.
- Tasks: `task(id)`, `emptyTask(id, description)` (scope roots), `overrideTask(id)` from
  `hardhat/config`; ids are `string | string[]`. Override actions receive `(args, hre, runSuper)`,
  overrides stack in plugin order, and `runSuper` walks back toward the original. Plugins must use
  lazy `setAction`; `setInlineAction` is for user configs only.
- Solidity npm imports (`@fhevm/solidity/...`) resolve natively; no remappings override exists or is
  needed.
- The cluster's `node_modules` holds `hardhat` and its `@nomicfoundation` runtime deps ONLY: no
  mocha or node test-runner plugin, and `hardhat test` runs nothing without one.

## 2b. The v2 → v3 translation (what v3 forces — the allowed breaking changes)

| v2 mechanism                    | v3 replacement                                                                 | consequence                                                                                                                                                                                                                                                                       |
| ------------------------------- | ------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| side-effect `extendEnvironment` | plugin OBJECT with `hookHandlers`                                              | declarative wiring, no import-order magic                                                                                                                                                                                                                                         |
| `hre.fhevm` global singleton    | hre created per invocation; networks are CONNECTIONS (`hre.network.connect()`) | **the one real API break**: fhevm state binds to a connection — `connection.fhevm`, implemented exactly as the docs prescribe (WeakMap in the network hook factory, `newConnection`/`closeConnection`); an `hre.fhevm` convenience alias to the default connection where possible |
| `extendProvider` wrapper        | `network.onRequest(context, connection, request, next)`                        | v2's wrapper keeps exactly two jobs — inflate `eth_estimateGas` by 120%, decorate failed `eth_sendTransaction` with a decoded FHEVM error — both port 1:1; `onRequest` RETURNS a `JsonRpcResponse` (error in `response.error`, not thrown), so the decoration mutates the response |
| `extendConfig`                  | `config` hook handlers (`extendUserConfig`/`resolveUserConfig`)                | mechanical                                                                                                                                                                                                                                                                        |
| eager task actions              | lazy action modules (`setAction(() => import('./tasks/x.js'))`)                | proven in the skeleton                                                                                                                                                                                                                                                            |
| CJS build (`node10`)            | ESM-only, `nodenext`                                                           | drops the node10 `paths` workaround for `@fhevm/sdk/chains` entirely                                                                                                                                                                                                              |

**Formerly-open items, settled from the hardhat 3.15 SOURCE** (`src/types` + `src/internal/builtin-plugins`):

- **`hardhat node` needs NO override for the pre-deploy.** The builtin node action
  (`builtin-plugins/node/task-action.ts`) runs `hre.network.create(connectionParams)` FIRST and
  only then builds `JsonRpcServerImplementation` and calls `listen()`. Our `newConnection`
  decorator therefore fires before the server accepts a single request. Two constraints: the
  decorator must return the SAME connection object (the action asserts
  `provider instanceof EdrProvider`), and the node task connects to the network literally named
  `node` (not `default`) unless `--network` is given — gate on `networkConfig.type`, never on name.
  Logging is enabled AFTER `create()`, so the deploy transactions stay silent.
- `localhost` is pre-extended by hardhat as `{ type: "http", url: "http://localhost:8545" }`; a
  second process on `--network localhost` gets an `HttpProvider`, our hook fires on it, and a
  chain-id probe + code-at-address check finds the stack the node process deployed.
- Scoped tasks are string-array ids with an `emptyTask(["fhevm"], ...)` root:
  `task(["fhevm", "user-decrypt"], ...)`; `hre.tasks.getTask(string | string[])`.
- `network.onRequest` covers both surviving jobs of v2's `extendProvider` (see the table above).
  Nothing else in v2's wrapper survives — its mock-relayer interception is already gone in v2.
- `newConnection` is a DECORATOR chain (see §2); `closeConnection` mirrors it.
- `hre.created(context, hre)` exists, runs in forward plugin order, and sees `hre.network`. There is
  no "default connection" object to alias — only `hre.network.getOrCreate()` (async, cached).
- `coverage` and `gas-analytics` are builtins with `network.onCoverageData` / `onGasMeasurement`
  hooks — confirming the v2 solidity-coverage workarounds are DELETE-bucket, and offering a native
  home for HCU-style measurement later.
- The `test` task is a coordinator over runner SUBTASKS (`test mocha`, `test node`,
  `test solidity`), each supplied by a separate plugin package; it needs no setup override from us
  because `newConnection` does the setup.

## 3. Minimization strategy (goal 2) — three buckets for every v2 file

- **DELETE — v2 workarounds hardhat 3 obsoletes.** CONFIRMED from the source: the compile
  remappings subtask override (hh3 resolves npm imports natively and has a `solidity` remappings
  hook), the source-paths override, `utils/solidityCoverage.ts` (hh3 has built-in coverage), the
  `test` task override (setup now happens in `newConnection`), ts-node/CJS glue, the
  `@fhevm/sdk/chains` tsconfig `paths` workaround, the `ProviderWrapper` subclass itself. NOT
  deleted: the wrapper's two behaviours (gas inflation, send-error decoration) — they port to
  `onRequest` (see §2b).
- **SHARE — hardhat-agnostic logic leaves the plugin.** The plugin already delegates crypto to
  `@fhevm/sdk` (`client: FhevmClient`; `chains.ts` imports `@fhevm/sdk/chains`; the in-code
  `TODO(migration step N)` markers chart this). Everything not touching `hre` belongs there:
  handle/type helpers (`fhevmHandle`, `fheType`), HCU computation and price tables, EIP-712
  builders, encrypted-input assembly, error parsing. **v2 benefits equally** — finish the shared
  extraction in `@fhevm/sdk` first, shrink v2 onto it, then v3 imports the same surface. The v3
  plugin ends up as ADAPTER GLUE ONLY: hooks, tasks, network detection, config.
- **PORT — the thin remainder**, one small function at a time (goal 6): the hook handlers
  (`newConnection`/`closeConnection`/`onRequest`), the task definitions, `networkProvider` chain
  detection, the deploy/setup path for the cleartext stack, and
  `FhevmEnvironmentPaths`/`resolveFromConsumer` (the sibling-module locators — see §1;
  hardhat-agnostic enough that they may eventually move to `@fhevm/sdk`, but they are ported, never
  deleted).

Cross-generation sharing rule: v3 never imports from `hardhat/v2/*` (separate installation roots,
different hardhat majors). Shared code has exactly two legal homes — `@fhevm/sdk` (imported;
preferred) and `common-vendored/src` (copied by `sync-vendored`, goal 8) for anything a published
payload needs but cannot depend on. `ethersEthereumLib.ts` already models the vendored path; add a
v3 destination for it (and any new shared file) in `common-vendored/manifest.json` + the manifest
`vendored` entry.

## 4. Centralized, generated constants (goal 7)

`internal/constants.ts` hand-copies addresses that the workspace already owns elsewhere. The v3
plugin gets a GENERATED constants module instead — and v2 should adopt it too:

- **Deployed addresses** (ZamaConfig trio, Sepolia/mainnet gateway contracts): a new fhevm-npm
  generator face rendered from `sdk/fhevm-chains.config.json` (which is itself pinned to the
  protocol registry and checked by `check-fhevm-chains-origin`). Same pattern as
  `generate-cleartext-config`: committed output, regeneration gate, `--check` twin.
- **Cleartext/localhost values** (mnemonics, HD paths, local addresses): already faced from
  `sdk/cleartext-config.json`; the plugin consumes them via `@fhevm/host-contracts-cleartext`
  exports or a generated face — never a fresh literal.
- Whatever remains in `constants.ts` after that must be genuinely plugin-local (task names,
  file names), and small.

## 5. Node support parity (goals 4 & 5)

The v2 behavior to reproduce, as a test matrix for the v3 e2e package (later cluster sibling):

| target                                          | v2 behavior to keep                                                                                                    |
| ----------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| in-process `edr-simulated` (`default`, `node`)  | auto-deploy cleartext stack on EVERY new connection (each `create()` is a fresh chain), mock mode, `connection.fhevm` fully functional |
| `hardhat node` (`--network localhost`)          | `newConnection` deploys in the node process before `listen()`; a second process connects over `http`, finds code at the addresses, skips the deploy; chainId 31337 preserved |
| anvil (`--network anvil`)                       | `http` network; detection by client version / network name + chain probing; same deploy or skip-if-present against an external anvil; the `test:anvil` operator flow |
| public chains (sepolia, mainnet, polygon, amoy) | resolved via `@fhevm/sdk/chains` by chainId; no stack deploy; addresses from the generated constants                   |

`hardhat node` was the highest-risk row; the source de-risked it (§2b): the pre-serve hook point
is `newConnection` itself. What remains to prove is the ordering (A4) and the skip-if-present
detection over `http` (B3).

## 6. Phases

1. **Shared extraction (shrinks v2, not just v3):** move the hardhat-agnostic bucket into
   `@fhevm/sdk` behind its existing entry points; v2 plugin consumes it; v2 suite + e2e stay green.
   This phase is measured by v2's line count going DOWN.
2. **v3 spike — the risky adapters only:** hre/connection extension exposing a minimal
   `fhevm` object (done, A2), the `newConnection`-before-`listen()` ordering proof for
   `hardhat node`, anvil detection. Goal is to pin the connection-scoping decision (the one API
   break) with the user; the hook surface itself is now pinned by the source (§2).
3. **Port the public API:** `FhevmExternalAPI` on v3, delegating to the shared layer; the
   generated constants module; vendored destinations for v3.
4. **Tasks:** the fhevm scope + whichever builtin overrides still have a job in v3.
5. **`hardhat/v3/e2e` + `hardhat/v3/fhevm-hardhat-template`** (already-reserved cluster slots):
   the parity matrix of §5 runs there; template is born workspace-native under option E.
6. Consumer fixtures + `check:publint`/`pack:tarball` already exist from the skeleton; extend the
   fixture to exercise one encrypt/decrypt round-trip.

Each phase ends green: `make build`, the affected test tiers, and the fhevm-npm battery.

## 7. Open questions

1. **Connection scoping — the one deliberate API break (⚑ gate A3).** The source narrows this: no
   default-connection object exists, so a SYNC `hre.fhevm` is impossible. The only alias shape is
   an async accessor over `hre.network.getOrCreate()`. Choice: that async accessor (marginal v2
   compatibility — v2 code was sync anyway) vs connection-only (`const { fhevm } = await
network.connect()`, the v3 idiom). Recommendation: connection-only; decide at A3.
2. **The ethers bridge.** Nothing in our path needs `connection.ethers` from the v3 ethers plugin.
   Recommendation: provider-direct — hand `connection.provider` (EIP-1193) to `@fhevm/sdk`, wrap
   in an ethers `BrowserProvider` internally only where the SDK requires ethers. Decides how much
   of `utils/ethers.ts` survives (expected: little).
3. **`FhevmDebugger` (⚑ gate D6).** Port as-is, or fold into `@fhevm/sdk` mock tooling?
4. **Where the shared extraction lands.** Stage D delegates to the PUBLISHED `@fhevm/sdk` for what
   it already exports — but the D4a2 error engine and D5a2 HCU engine are new shared-layer material,
   and pushing code INTO `@fhevm/sdk` needs js-sdk repo work + interim publishes until js-sdk joins
   the workspace. Options: (a) port the engines into the v3 plugin now with an extraction marker;
   (b) accelerate the js-sdk workspace landing so extraction comes first; (c) interim sdk releases.
5. **The npm version line for `@fhevm/hardhat-plugin` v3.** The skeleton says `0.1.0` but v2
   publishes `0.4.2` under the SAME name. Proposal: v3 starts at `1.0.0` (major = hardhat
   generation; `peerDependencies` does the real gating), v2 stays on `0.x`, dist-tags steer
   (`latest` → v3, `hardhat2` → v2). Must be settled before F2 (publint/pack).
6. **Is v2 frozen or co-developed during the port?** The plan assumes v2 SHRINKS alongside (C2,
   phase-1 extraction). Active v2 feature work in parallel means every shared change needs a
   two-generation test pass; bugfix-only keeps the port fast. Decides phase-1 aggressiveness.
7. **Task CLI compatibility.** v2 spells `hardhat fhevm user-decrypt`; v3 keeps the task names but
   its option/flag syntax differs. Is byte-for-byte CLI compatibility required (external docs or
   scripts invoking these?), or is "same task names, v3-idiomatic flags" acceptable? The plan
   assumes the latter, per goal 3's carve-out.

## 8. Step-by-step implementation — small feature → test → commit

The working rule for every step: ONE small feature, its test(s) in the same change, one commit.
A commit is legal only when `make build` is green, the step's own tests pass, and the fhevm-npm
battery + `check-commit-scope` pass. Suggested message prefix: `feat(hh-v3-plugin): …`
(or `feat(fhevm-npm): …` for generator steps). Decision gates are marked ⚑ — stop and decide with
the user before that step.

**Hard size cap: a commit never exceeds 500 changed lines** (`git diff --stat` total, tests
included). A block that approaches the cap is split further BEFORE committing — by method, by file,
or by extracting its test fixtures into a preparatory commit — never waved through. Two mechanical
consequences: (1) machine-written diffs (lockfiles, generated modules) count too, so a step whose
generation output is large lands as two commits — generator/config change first, the regenerated
output second (`chore(...): regenerate …`); (2) the D-stage port blocks are the likeliest offenders —
if porting one method group drags more than ~500 lines of shared helpers with it, that is the signal
the helpers belong in the SHARED layer (§3), not in the plugin.

### Size estimates (rough, from the v2 sources — port ≈ 70% of v2 lines + tests)

Statistical only; reality overrides. Any block that trends past ~400 est. total is split at the
first review, per the cap rule.

| block                                | v2 source basis (lines)                  | est. feature | est. incl. tests |
| ------------------------------------ | ---------------------------------------- | ------------ | ---------------- |
| A1 config hooks                      | ConfigExtender ≈40                       | 80           | ~160             |
| A2 connection decoration             | new                                      | 60           | ~140             |
| A3 hre alias                         | new                                      | 30           | ~70              |
| A4 node ordering spike               | new (no override needed)                 | 30           | ~80              |
| A5 network detection                 | networkProvider 233                      | 120          | ~220             |
| A7 onRequest port                    | FhevmProviderExtender 117                | 70           | ~150             |
| A6 path resolution                   | Paths 148 + path 10                      | 120          | ~240             |
| B1a contracts repository             | contractsRepository 202                  | 150          | ~230             |
| B1b1/B1b2 deploy (split, see below)  | FhevmEnvironment 898 (deploy slice)      | 150/150      | ~230/230         |
| B1c post-deploy setup                | deploy/setup 198                         | 150          | ~230             |
| B2 anvil deploy                      | delta on B1                              | 80           | ~180             |
| B3 node skip-if-present              | delta on A4+B1 (detection + spawn test)  | 40           | ~120             |
| C1a/C1b generator                    | new                                      | 120/80       | ~240/180         |
| C2 consumers                         | constants 113 swap                       | 100          | ~150             |
| D0 public type surface               | types 208                                | 180          | ~200             |
| D1 encrypt group                     | encryptedInput 102 + API slice           | 140          | ~290             |
| D2 publicDecrypt                     | API slice                                | 60           | ~180             |
| D3a/b/c user decrypt                 | API slice                                | 40/80/50     | ~100/200/130     |
| D4a1/a2/a3 errors (split, see below) | FhevmContractError 703 + list 124        | 120/250/60   | ~180/400/120     |
| D4b events                           | events 81 + eventArgs 77                 | 120          | ~220             |
| D5a1/a2/a3 HCU (split, see below)    | operatorsPrices 523 + hcu 379 + byOp 147 | data/280/80  | ~530\*/400/160   |
| D5b coprocessor config               | coprocessorConfig 103                    | 80           | ~160             |
| D6 debugger                          | FhevmDebugger 98                         | 80           | ~160             |
| E1a/E1b, E2 tasks                    | tasks/fhevm 262 ÷ 4                      | ~70 each     | ~130 each        |
| E3 surviving overrides               | builtin-tasks remainder                  | ≤60 each     | ~100 each        |
| F1/F2/F3                             | fixtures                                 | 60–150       | ~120–250         |

\* D5a1 is a DATA table (operator prices) — see its block for how it stays legal under the cap.

### Stage A — scaffolding and risk spikes (v3 cluster only)

- **A1. Config hook.** ~~`config.extendUserConfig`/`resolveUserConfig`~~ **COLLAPSED**: the v2
  plugin's `ConfigExtender` is empty — there is no `fhevm` user-config surface to port. Config
  hooks (and the `hardhat/types/config` augmentation) are added only when a real config key
  appears in a later step.
- **A2. Connection decoration — LANDED** (`137b1406d`). `network.newConnection` attaches a stub
  `connection.fhevm` (`isMock`/`isCleartext` placeholders), `closeConnection` cleans up;
  `NetworkConnection` augmentation. Test: two connections get DISTINCT fhevm instances; close
  releases state. Follow-up folded into A3: wrap the plugin object in `definePlugin` (§2).
- **A3. ⚑ `hre.fhevm` alias** (open question 1 decided here). Only an ASYNC accessor over
  `hre.network.getOrCreate()` is possible; recommendation is to drop the alias. Either way, this
  commit also switches `index.ts` to `definePlugin`. Test matches the decision.
  Commit: `feat(hh-v3-plugin): decide the hre.fhevm alias; register the plugin via definePlugin`
- **A4. SPIKE — `hardhat node` ordering** (ONE block; the source already shows `create()` precedes
  `listen()`, this commit proves it in our tree). In `newConnection`, on an `edr-simulated`
  connection, write a marker transaction. Test: `hre.network.createServer()` → `listen()` → a raw
  HTTP `eth_getCode`/`eth_getTransactionCount` probe from the test sees the marker on the FIRST
  request; a second `createHardhatRuntimeEnvironment` connecting over `localhost` (with the port
  overridden) sees it too. No task override, no `internal/spike/`.
  Commit: `feat(hh-v3-plugin): prove newConnection runs before the node server listens`
- **A5. Network detection.** Port the `networkProvider` minimum onto the connection:
  `networkConfig.type === "edr-simulated"` → in-process dev; `type === "http"` → probe `eth_chainId`
  (+ `web3_clientVersion` for anvil) → `localhost`-class dev / `anvil` / public chain id from
  `@fhevm/sdk/chains`. Never gate on `networkName` (the node task uses `node`; users rename freely).
  Test: unit tests against a fake `EthereumProvider` (chainId/clientVersion fixtures) and both
  config types.
  Commit: `feat(hh-v3-plugin): port network detection for edr, localhost, anvil, public chains`

- **A6. Consumer path resolution.** Port `FhevmEnvironmentPaths` + `resolveFromConsumer` — the
  sibling-npm-module locators (`@fhevm/solidity`, `ZamaConfig.sol`, `@fhevm/sdk` incl. pnpm layout,
  consumer `node_modules` root) — anchored on `context.config.paths.root`.
  Test: unit tests against a fixture consumer tree (npm and pnpm layouts); missing sibling → the
  named, actionable error.
  Commit: `feat(hh-v3-plugin): port consumer path resolution for sibling npm modules`
- **A7. `onRequest` port.** The two surviving `FhevmProviderExtender` behaviours as an `onRequest`
  handler: inflate `eth_estimateGas` results by 120%; on a failed `eth_sendTransaction`, decorate
  `response.error` with the decoded FHEVM error (the decoder itself arrives in D4a — until then a
  pass-through with the hook shape and the estimate inflation only). Test: fake `next` returning
  fixed responses; inflation exact; error response passed through untouched pre-D4a.
  Commit: `feat(hh-v3-plugin): port gas inflation and send-error decoration to onRequest`

### Stage B — the essential job: pre-deploy per target

- **B1. In-process deploy** (three blocks):
  - **B1a.** Port the contracts repository: load the cleartext artifacts/bytecode from the sibling
    `@fhevm/host-contracts-cleartext` package (stands on A6's paths).
    Test: artifacts resolve and parse from a fixture consumer tree.
    Commit: `feat(hh-v3-plugin): load cleartext artifacts from the sibling contracts package`
  - **B1b1.** The deploy transaction sequence itself (nonce-ordered CREATEs onto the ZamaConfig
    addresses), as a pure function of (provider, artifacts). Test: addresses hold code afterwards.
    Commit: `feat(hh-v3-plugin): port the nonce-ordered cleartext deploy sequence`
  - **B1b2.** Wire it to `newConnection`: development-class gating (A5), exactly once per CHAIN.
    Every `create()` on an `edr-simulated` network is a fresh chain and needs its own deploy; an
    `http` dev connection whose ZamaConfig addresses already hold code skips it. Test: `create()`
    twice → two deploys; `getOrCreate()` twice → one deploy; `http` with code present → zero;
    public chains untouched. Template/docs steer users to `getOrCreate` (per-file `connect()` in a
    mocha suite redeploys per file — a cost v2 never paid). Before committing the CREATE sequence,
    measure `hardhat_setCode`-style injection as the faster alternative.
    Commit: `feat(hh-v3-plugin): deploy the cleartext stack once per dev chain`
  - **B1c.** Post-deploy setup (signers registration, HCU caps — v2's `deploy/setup.ts`).
    Test: coprocessor/KMS signers registered; `assertCoprocessorInitialized` path green.
    Commit: `feat(hh-v3-plugin): run post-deploy signer and HCU setup on the fresh stack`
- **B2. Anvil deploy.** Same flow against an external anvil.
  Test: behind a `test:anvil` operator script, exactly like v2's.
  Commit: `feat(hh-v3-plugin): pre-deploy the cleartext stack against external anvil`
- **B3. `hardhat node` skip-if-present** (builds on A4 + B1b2; no new deploy code). Test: spawn
  `hardhat node` as a child process with the plugin in that process's config, connect
  `--network localhost` from a second HRE, assert the ZamaConfig addresses hold code and the
  second process ran ZERO deploy transactions; tear down.
  Commit: `feat(hh-v3-plugin): reuse the cleartext stack a hardhat node already deployed`

### Stage C — centralized generated constants (goal 7)

- **C1. fhevm-npm generator** (two blocks; the cleartext-config generator is the template):
  - **C1a.** The renderer: `fhevm-chains.config.json` → the constants module text, pure function.
    Test: fixture chains file → exact expected output; malformed input refused with named errors.
    Commit: `feat(fhevm-npm): render chain address constants from fhevm-chains.config.json`
  - **C1b.** The command: `generate-chain-constants` + `--check` twin, CLI registration, battery +
    `make generate` wiring. Test: write/check/missing/drift lifecycle on a temp workspace.
    Commit: `feat(fhevm-npm): add generate-chain-constants command with check mode`
- **C2. Consumers.** v3 plugin consumes the generated module from birth; v2's hand-copied
  `internal/constants.ts` addresses are replaced by the same module (v2 SHRINKS — first
  minimization dividend). Test: v2 suite + e2e stay green; grep proves no address literal remains.
  Commit: `refactor(hh-plugin): replace hand-copied addresses with generated constants`

### Stage D — the public API, method-group by method-group

Each step ports one group of `HardhatFhevmRuntimeEnvironment` onto the connection-scoped object,
delegating to `@fhevm/sdk` (the published package — the workspace-member extraction can upgrade
this later without API change). One commit per group, tests run against the B1 stack:

- **D0.** The public type surface (`types.ts` → `HardhatFhevmRuntimeEnvironment` and friends),
  types only, methods stubbed `not implemented` — the contract lands first, groups fill it in.
  Commit: `feat(hh-v3-plugin): land the public fhevm type surface with stub methods`
- **D1.** `createEncryptedInput` + `encryptUint/Bool/Address`.
  Commit: `feat(hh-v3-plugin): port encrypted-input creation and encrypt helpers`
- **D2.** `publicDecrypt` + typed variants.
  Commit: `feat(hh-v3-plugin): port publicDecrypt and its typed variants`
- **D3.** `userDecrypt*` + EIP-712 builders (three blocks):
  - **D3a.** `createEIP712`. Commit: `feat(hh-v3-plugin): port the user-decrypt EIP-712 builder`
  - **D3b.** `userDecryptEbool/Euint/Eaddress`.
    Commit: `feat(hh-v3-plugin): port the userDecrypt typed variants`
  - **D3c.** `createDelegatedUserDecryptEIP712` + the delegated flow.
    Commit: `feat(hh-v3-plugin): port the delegated user-decrypt permit flow`
- **D4.** Errors and events (two blocks):
  - **D4a.** The error layer is 827 v2 lines (`FhevmContractError` 703 + list 124) — the single
    biggest port item, and prime shared-layer material (§3: nothing in it touches `hre`). Three
    blocks, with a standing note that a shared `@fhevm/sdk` home beats porting a2 at all:
    - **D4a1.** The error LIST (data table).
      Commit: `feat(hh-v3-plugin): port the fhevm contract error table`
    - **D4a2.** The parsing engine (`FhevmContractError`), trimmed to what v3 consumers reach —
      if it still trends past the cap, split by error family.
      Commit: `feat(hh-v3-plugin): port the fhevm contract error parsing engine`
    - **D4a3.** The two public helpers (`tryParseFhevmError`, `revertedWithCustomErrorArgs`).
      Commit: `feat(hh-v3-plugin): expose the fhevm error parsing helpers`
  - **D4b.** `parseCoprocessorEvents`.
    Commit: `feat(hh-v3-plugin): port coprocessor event parsing`
- **D5.** HCU and coprocessor config (two blocks):
  - **D5a.** HCU is 1,162 v2 lines, over half of it a DATA table (`operatorsPrices` 523). Three
    blocks:
    - **D5a1.** The operator price table. It is DATA, so first choice is goal 7's answer: a
      generated module (fhevm-npm face from a committed price config) — then the generator commit
      is small and the big diff is the machine-written output commit, which is the cap rule's
      sanctioned shape. Verbatim port only if generation is premature.
      Commit: `feat(hh-v3-plugin): add the HCU operator price table (generated)`
    - **D5a2.** The HCU engine (`hcu.ts` + `HCUByOperator`). Split by operator family if it trends
      past the cap. Commit: `feat(hh-v3-plugin): port the HCU computation engine`
    - **D5a3.** `computeTransactionHCU` + module-level `getHCU` wiring.
      Commit: `feat(hh-v3-plugin): expose transaction HCU computation`
  - **D5b.** `getCoprocessorConfig` + `assertCoprocessorInitialized`.
    Commit: `feat(hh-v3-plugin): port coprocessor config read and init assertion`
- **D6. ⚑ `debugger`** (open question 3 decided here): port vs fold into `@fhevm/sdk` mock tooling.
  Commit: `feat(hh-v3-plugin): port the fhevm debugger surface`

### Stage E — tasks

- **E1.** The decrypt tasks, lazy actions under an `emptyTask(["fhevm"], ...)` scope root; each
  action opens its connection with `hre.network.getOrCreate()` (no arguments → honours
  `--network`). Two blocks; each test: `hre.tasks.getTask([...])` resolves and a run round-trips
  against the B1 stack:
  - **E1a.** `task(["fhevm", "public-decrypt"])`.
    Commit: `feat(hh-v3-plugin): add the fhevm public-decrypt task`
  - **E1b.** `task(["fhevm", "user-decrypt"])`.
    Commit: `feat(hh-v3-plugin): add the fhevm user-decrypt task`
- **E2.** `["fhevm", "check-fhevm-compatibility"]`. Test likewise.
  Commit: `feat(hh-v3-plugin): add fhevm check-fhevm-compatibility task`
- **E3.** Builtin overrides that SURVIVE the delete-bucket triage. Confirmed dead: `test`,
  compile remappings, source paths, coverage. Candidates: `clean` (only if the plugin owns a cache
  dir), `node` (only for an fhevm banner — a `runSuper` pass-through, never for the deploy). One
  commit per override, each with the reason it still exists in its commit message.
  Commit: `feat(hh-v3-plugin): keep <name> builtin override — one commit per survivor`

### Stage F — packaging and fidelity

- **F1.** Consumer fixture upgraded to a real encrypt/decrypt round-trip. Test: `test-consumer` leg.
  Commit: `test(hh-v3-plugin): consumer fixture runs an encrypt/decrypt round-trip`
- **F2.** `check:publint`/`attw`/`pack:tarball` green; pkg README. Commit.
  Commit: `chore(hh-v3-plugin): publint, attw and pack:tarball green; package README`
- **F3.** `hardhat/v3/e2e` cluster member: the §5 parity matrix becomes its test suite, row by row
  (each row = one commit). FIRST commit adds a test-runner plugin — none is installed (§2), and
  `hardhat test` is a coordinator that runs nothing without one. Pick `@nomicfoundation/hardhat-mocha`
  (closest to v2 suites) or `@nomicfoundation/hardhat-node-test-runner`; exact pin.
  Commit: `feat(hh-v3-e2e): add e2e member covering one node-parity-matrix row`

Stage order is deliberate: A4 (formerly the riskiest unknown, now a one-commit ordering proof)
sits as early as scaffolding allows, the essential job (§1) is proven before any API surface is
ported onto it, and constants land before the API port so no step ever introduces a hand-copied
address even temporarily.
