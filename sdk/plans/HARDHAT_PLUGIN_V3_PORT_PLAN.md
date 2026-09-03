# Porting the FHEVM hardhat plugin from hardhat v2 to hardhat v3

Version: **1.46** (2026-09-03). Bump the minor for a content change to the plan, the major for a
change of charter or stage order; record each bump below.

- 1.0 — initial plan, from the hardhat 3 docs.
- 1.1 — §2, §2b, §5, Stage A revised from the hardhat 3.15 SOURCE; A4 collapsed, A7 added.
- 1.2 — E2E-0 added (e2e member born early, ledger after Stage F); E2E-0b deferred to on demand.
- 1.3 — working rules preamble (incl. stop-after-each-step and plan-committed-alone); A3 and A4
  recorded as landed.
- 1.4 — A5 recorded as landed (two commits: @fhevm/sdk peer, then detection); ledger A5 row done.
- 1.5 — A6 recorded as landed; `cacheDir`/`dotEnvFile`/`solidityCoverageDir` confirmed delete-bucket.
- 1.6 — A7 recorded as landed; Stage A complete.
- 1.7 — B1a recorded as landed (two commits: cleartext package + ethers peer, then the ABI repository).
- 1.8 — B1b1 recorded as landed; the plugin switched from ethers to VIEM (open question 2 settled);
  genesis-injection measured and declined; B1a repository re-based on viem.
- 1.9 — B1b2 recorded as landed: the stack deploys once per development chain inside `newConnection`.
- 1.10 — B1c recorded as landed (verification, not setup); ledger: B1c smoke test → D0, `TestFHENotInitialized` → D5b.
- 1.11 — B2 recorded as landed (anvil operator scripts; no plugin change needed); ledger B2 row done.
- 1.12 — B3 recorded as landed; Stage B complete; ledger B3 row done.
- 1.13 — C1a landed together with the command half of C1b; network GROUPS (incl. devnet) and the
  relayer source of truth `fhevm-network-groups.config.json` introduced; face shape settled.
- 1.14 — C1b complete: generator wired into `generate`/`clean:generated`, first face committed.
- 1.15 — C2 landed (v3 detection over the face; v2 public configs from the face); Stage C complete.
- 1.16 — D0 landed (public surface isolated in `types.ts`, stubs by name); lint tsconfig split into
  source and test projects; ledger: the ZamaConfig-trio smoke test moves from D0 to D5b.
- 1.17 — D1 landed (encrypt group over the cleartext SDK client; `fhevm.client` live on development
  connections); ledger D1 row: e2e files contracts landed at E2E-0b; test port pending.
- 1.18 — D2 landed (publicDecrypt group; the v2 counter e2e test is now ported in full).
- 1.19 — D3a resolved as a REMOVAL: `createEIP712`/`createDelegatedUserDecryptEIP712` (deprecated and
  throwing in v2) leave the v3 surface; D3c is the delegated flow only.
- 1.20 — D3b landed (userDecrypt typed variants; `FhevmUser` = viem wallet client or local account;
  `timestampNow` module export); the v2 user-decrypt counter e2e test ported.
- 1.21 — D3c landed (`options.delegatorAddress` on `userDecryptE*`); delegated e2e runs on the counter
  through `SmartWalletWithDelegation`; the ConfidentialERC20 corpus stays E2E-0b.
- 1.22 — D4a1 landed (error table as typed data with a keyed lookup; template engine on plugin errors).
- 1.23 — D4a2 landed (viem decoder + formatter + in-place decoration of hardhat 3's THROWN revert
  errors, wired through `onRequest` with a per-connection repository); A7 note corrected.
- 1.24 — D4a3 landed (`revertedWithCustomErrorArgs` with a viem-backed ethers-shaped interface,
  `tryParseFhevmError`); TestErrors/TestTrivialPermissions/TestACL e2e ported. Stage D4a complete.
- 1.25 — D4b landed (`parseCoprocessorEvents` over viem `decodeEventLog`; accepts viem and ethers logs).
- 1.26 — D5a1 landed as VENDORING, not generation: the upstream HCU price table lives in
  common-vendored and both plugins receive it through `sync-vendored`.
- 1.27 — D5a2 landed (HCU engine over the D4b events: price bridge, type-name map, handle parser, walk).
- 1.28 — D5a3 landed (`computeTransactionHCU`, `typeof`, module `getHCU`); HCU e2e on the counter.
- 1.29 — D5b landed (`getCoprocessorConfig`, `assertCoprocessorInitialized`); the B1c-deferred smoke test
  and `TestFHENotInitialized` e2e landed. Stage D5 complete; only `debugger` (D6) is still a stub.
- 1.30 — D6 landed: open question 3 decided as PORT (over viem, CleartextDB read); dead
  `createDecryptionSignatures`/`createHandleCoder` dropped. The public surface is complete: Stage D done.
- 1.31 — E1 landed (`fhevm public-decrypt` and `fhevm user-decrypt`, positional required inputs); the
  skeleton's `hello` task retired.
- 1.32 — E2 landed (`fhevm check-fhevm-compatibility <address>` over the D5b methods).
- 1.33 — E3 landed: the one surviving builtin override is `node`, for the fhevm stack banner; `clean` confirmed dead.
- 1.34 — F1 landed (consumer fixture runs an encrypt/decrypt round-trip; the leg was silently not running).
- 1.35 — the v3 plugin consumer leg joined the Makefile `test-consumer` targets (approved Makefile edit).
- 1.36 — F2 landed (publint + attw esm-only + pack green, build dir cleaned before compile, package
  README); open question 5 reframed: the version must encode protocol line, hardhat generation, patch.
- 1.37 — F3 closed with evidence for three parity rows (in-process, `hardhat node`, anvil: 27/27 each);
  the public-chain row is detection-only until the network-group decision. Manifest note fixed.
- 1.38 — E2E-0b landed: the whole v2 Solidity corpus copied verbatim (47 files, byte-identical),
  `@openzeppelin/contracts` 5.1.0 + forge remapping added; every ledger row now has its contracts.
- 1.39 — payload layout aligned on js-sdk: declarations in `pkg/_types`, JavaScript in `pkg/_esm`; the
  plugin tests reach the build through a `#esm/*` imports map (types → `_types`, default → `_esm`).
- 1.40 — open question 5 DECIDED: the hardhat 3 package is `@fhevm/hardhat-plugin-v3` at `0.13.0` (the
  FHEVM protocol line + patch). Working rule added: ledger rows are ported from the smallest to the largest.
- 1.41 — ledger row 1 ported: `internal/AplusB.ts` (smallest first).
- 1.42 — ledger row 2 ported: `hcu/fhevmHCU1.ts` over `FHEVMTestSuite1`; `test/utils/receipts.ts` added.
- 1.43 — ledger row 3 ported: `doc-examples/DecryptSingleValue.ts`; `test/utils/expect.ts` (shared rejection helper).
- 1.44 — ledger rows 4–10 ported: the remaining `doc-examples/*` (DecryptMultipleValues, EncryptMultipleValues,
  EncryptSingleValue, HighestDieRoll, HeadsOrTails). `KMSInvalidSigner` checks use `revertedWithCustomErrorArgs`.
- 1.45 — `finance/*` ported (both vesting wallets, fixtures) with the shared `confidentialERC20` fixture;
  `signers.ts` gained `accountFor(signer)`.
- 1.46 — `internal/*` ported: `Rand.ts` (+ fixture; the upstream-skipped snapshot test dropped) and the
  ConfidentialERC20 delegated-decryption file as `delegatedUserDecryptionERC20.ts`.

Status: **port complete for development networks — Stages A–F and E2E-0b landed. Open: the public-chain client (network-group decision) and the remaining ledger test ports (contracts now present; smallest first).** The landing zone exists: the
`hardhat/v3` cluster (own installation root, hardhat 3.15 pinned), the plugin registered via
`definePlugin` with its per-connection `fhevm` object and the pre-serve chain preparation proven,
and the `hardhat/v3/e2e` member with the first counter test.

## Working rules (binding for whoever executes this plan, human or agent)

- No git command without confirmation — every commit is asked for, one title line, no body.
- Read access is limited to the `fhevm/sdk` folder.
- Write access is limited to `/Users/alex/src/me/zama-ai/fhevm/sdk/hardhat/v3`.
- Any change outside that folder (Makefile, npm-manifest.json, plans, fhevm-npm, common-vendored, …)
  requires explicit approval first.
- Only tests written in the `fhevm/sdk` folder may be executed.
- Running `fhevm-npm` (the check battery and its commands) is allowed.
- Stop after each step: report, then wait for the go-ahead before starting the next one.
- Ledger e2e test rows are ported from the SMALLEST to the LARGEST (rule added 2026-09-03).
- The plan is committed ALONE: every commit touching this file bumps the version below and contains
  nothing else — never bundled with a code step.

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

**F3 status (2026-09-03):** rows 1–3 CLOSED by evidence — the same 27-test e2e suite passes in-process
(`make test-hh-v3-e2e`), through `test/test-hardhat-node.sh` (node process deploys before `listen()`,
the test process reuses over `localhost`) and through `test/test-anvil.sh` (fresh anvil, deploy from the
first connection). Row 4 is detection-only: `fhevm.network.publicChains` resolves from the generated
constants, but `fhevm.client` and the contracts repository refuse public networks until the
network-group decision (which gateway serves a chain listed under two); the v2 `sepolia/*` operator
tests wait on that AND on a live RPC + funded key, so they never run in `make test`.

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
   the e2e member is born at E2E-0 (Stage A, right after A2) with the v2 Solidity corpus and ONE
   test, then grows one v2 test file per plugin step per the ledger after Stage F; the parity
   matrix of §5 is closed there by F3. Template is born workspace-native under option E.
6. Consumer fixtures + `check:publint`/`pack:tarball` already exist from the skeleton; extend the
   fixture to exercise one encrypt/decrypt round-trip.

Each phase ends green: `make build`, the affected test tiers, and the fhevm-npm battery.

## 7. Open questions

1. **Connection scoping — SETTLED at A3: connection-only.** No `hre.fhevm` in any shape. Hardhat 3
   has no default-connection object, its docs only ever extend `NetworkConnection`, and the official
   ethers plugin does the same. Consumers write `const { fhevm } = await network.connect()`; every
   ported v2 test swaps its `import { fhevm } from 'hardhat'` for that line.
2. **The web3 library — SETTLED at B1b1: VIEM, provider-direct.** Hardhat 3 recommends viem (its
   first `--init` template and toolbox), hardhat core depends on neither library, and both are
   first-class official plugin families. The plugin builds viem clients over `custom(connection.provider)`
   exactly as `@nomicfoundation/hardhat-viem` does (dev defaults: pollingInterval 50, cacheTime 0,
   retryCount 0; chain = hardhat preset with the live id). No `hardhat-viem` peer yet — nothing needs
   its types; revisit at Stage D if the API takes `connection.viem` clients. `utils/ethers.ts` does not
   survive. The e2e stays on mocha + ethers for now (closest to the v2 suites); its move to the viem
   toolbox is a separate decision before Stage D.
3. **`FhevmDebugger` (⚑ gate D6).** Port as-is, or fold into `@fhevm/sdk` mock tooling?
4. **Where the shared extraction lands.** Stage D delegates to the PUBLISHED `@fhevm/sdk` for what
   it already exports — but the D4a2 error engine and D5a2 HCU engine are new shared-layer material,
   and pushing code INTO `@fhevm/sdk` needs js-sdk repo work + interim publishes until js-sdk joins
   the workspace. Options: (a) port the engines into the v3 plugin now with an extraction marker;
   (b) accelerate the js-sdk workspace landing so extraction comes first; (c) interim sdk releases.
5. **The npm name and version of the hardhat 3 plugin — DECIDED (2026-09-03).** One package per
   hardhat generation: v2 stays `@fhevm/hardhat-plugin` (`0.4.x`), the hardhat 3 plugin is
   **`@fhevm/hardhat-plugin-v3`**, versioned on the FHEVM protocol line it targets: **`0.13.0`** (major.minor
   = protocol `0.13`, patch = plugin patch). The generation lives in the NAME, so semver keeps its three
   slots. Applied to the payload, the plugin id's `npmPackage`, the e2e and consumer fixture
   dependencies, both lockfiles, the README and `npm-manifest.json` (the "same name on purpose" note is
   gone). publint, attw, pack (`fhevm-hardhat-plugin-v3-0.13.0.tgz`), consumer leg, tests: green.
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
output second (`chore(...): regenerate …`); the same shape applies to VERBATIM copies (the e2e
Solidity corpus, E2E-0b…): one directory per `chore(...)` commit, compile-gated, no edits inside;
(2) the D-stage port blocks are the likeliest offenders —
if porting one method group drags more than ~500 lines of shared helpers with it, that is the signal
the helpers belong in the SHARED layer (§3), not in the plugin.

### Size estimates (rough, from the v2 sources — port ≈ 70% of v2 lines + tests)

Statistical only; reality overrides. Any block that trends past ~400 est. total is split at the
first review, per the cap rule.

| block                                | v2 source basis (lines)                  | est. feature | est. incl. tests |
| ------------------------------------ | ---------------------------------------- | ------------ | ---------------- |
| A1 config hooks                      | ConfigExtender ≈40                       | 80           | ~160             |
| A2 connection decoration             | new                                      | 60           | ~140             |
| E2E-0a e2e skeleton + first test     | v2 e2e config/scripts + counter test     | 200          | ~330 (incl. 2 .sol) |
| E2E-0b contract corpus (on demand)   | v2 e2e contracts 9,805 (verbatim copy)   | ≤500 each    | with ledger rows |
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
- **E2E-0. The e2e member is born NOW — next commit, before A3.** The v3 twin of
  `hardhat/v2/e2e`, as a cluster member (`hardhat/v3/e2e`, added to the cluster `workspaces`,
  fhevm-npm manifest entries, `compile/lint/test-hh-v3-e2e` make targets, `make install`). It
  needs ZERO plugin functionality to start: the first v2 test —
  `internal/FHECounterPublicDecrypt.ts` "encrypted count should be uninitialized after deployment"
  — only compiles, deploys and reads `getCount()`, and `ZamaEthereumConfig`'s constructor is a
  pure storage write (`FHE.setCoprocessor`), so no live stack is required. Every later test file
  ports when its API lands (see the ledger after Stage F) — the e2e grows ONE test file per commit,
  never ahead of the plugin. Split for the cap rule:
  - **E2E-0a. Skeleton + first test — IMPLEMENTED, awaiting commit.** What landed differs from
    the sketch below in four places, all forced by hardhat 3: `compile` is `hardhat build`
    (typechain runs inside it, output `types/ethers-contracts`); `tsconfig` sets
    `skipLibCheck: true` because typechain's generated `hardhat.d.ts` augments the hardhat-ethers
    helpers interface by extending itself (hardhat's three shipped templates set the same flag;
    hardhat.org does not document it); `ethers` is NOT declared until a test imports it (rule
    4.2.1 — the hoisted copy comes from the plugins' peer ranges); the test opens its connection
    with `network.getOrCreate()` (one chain per run, `--network` honoured). The cluster lockfile
    grew by ~1,600 lines for the five companion plugins — the one part of this step the 500-line
    cap cannot split.
    `package.json` (ESM, private,
    `@fhevm/hardhat-plugin-v3-e2e-dev`), `hardhat.config.ts` via `defineConfig` with
    `plugins: [fhevm, hardhatMocha, hardhatEthers, hardhatEthersChaiMatchers, hardhatTypechain]`
    (the hh3 flavours of v2's stack — exact pins, chosen at implementation time), networks in v3
    shape (`default`/`node`: `edr-simulated` with the v2 mnemonic; `anvil`: `http` on 8545;
    `sepolia`: `http` behind `configVariable`), solidity `0.8.27` profile matching v2's settings,
    forge fmt/lint wiring copied (`foundry.toml`, `remappings.txt`, `soldeer.lock`). Contracts:
    ONLY `contracts/test/internal/FHECounter*.sol` (verbatim). Tests: `test/utils/signers.ts`
    ported to `connection.ethers.getSigners()` + the single counter test above, its `isCleartext`
    guard marked `// enabled at A5`. Scripts: `test` only (`hardhat test`); `test:node`,
    `test:anvil*`, `test:sepolia:*` arrive with B3, B2, C+D2 respectively.
    Test: `make test-hh-v3-e2e` green; `hardhat test --network node`-style runs are NOT yet
    claimed.
    Commit: `feat(hh-v3-e2e): add the e2e member with the first counter test`
  - **E2E-0b — LANDED on demand (2026-09-03).** The whole v2 corpus (47 `.sol`, 9,805 lines) copied
    verbatim — `diff -rq` against v2 is empty; nothing edited on the way in. Hardhat 3 compiles all
    40 sources it had not seen (solc 0.8.27, cancun), forge lint/fmt are clean, the 27 e2e tests
    still pass. `@openzeppelin/contracts` 5.1.0 joined the e2e devDependencies (exact, as v2) with
    the `@openzeppelin/` forge remapping. One `chore` commit per top-level directory in import order:
    deps → utils → token → finance → governance → test → operators → operators-public-decrypt
    (verbatim copies are cap-exempt). The ledger rows that waited on contracts can now port their
    tests, one row per commit, as the plan always said.
    Was: deferred, copied per first ledger row that needs it.
    Commit: `chore(hh-v3-e2e): copy the <dir> contracts verbatim from the v2 e2e`
- **A3. ⚑ `hre.fhevm` alias — DECIDED: connection-only, no alias.** Hardhat 3's docs show only
  `NetworkConnection` extensions, never the HRE; `hre.created` is undocumented; the official ethers
  plugin attaches `connection.ethers` and nothing on the HRE. `index.ts` switches to `definePlugin`
  with `npmPackage` set (hardhat resolves the plugin's package.json through it for dependency
  diagnostics; the fallback is the id, which is wrong). Test: the HRE has no `fhevm` property
  before or after a connection is created.
  Commit: `feat(hh-v3-plugin): register via definePlugin; fhevm lives on the connection only`
- **A4. SPIKE — `hardhat node` ordering — PROVEN.** `internal/prepare.ts` adds the
  `prepareDevelopmentChain` step to `newConnection`, gated on `networkConfig.type ===
"edr-simulated"` (the wiring B1b2 fills); its body today is a marker — `hardhat_mine` one block,
  chosen over a marker transaction so no account nonce moves and later CREATE addresses stay
  deterministic. Test: `hre.network.createServer('default', '127.0.0.1', 0)` → `listen()` → the
  FIRST raw-HTTP `eth_blockNumber` is `0x1`; a second HRE with an `http` network on that port sees
  `0x1` too and does not mine again. Passed first run. No task override, no `internal/spike/`.
  Commit: `feat(hh-v3-plugin): prove newConnection runs before the node server listens`
- **A5. Network detection — LANDED** (two commits: `@fhevm/sdk` as the payload's peer `^0.13.3` +
  dev owner pin `0.13.3` with the consumer-fixture lock regenerated, then the detection).
  `internal/network.ts`: `edr-simulated` → `hardhat`; `http` on the live `eth_chainId` 31337 →
  `localhost` (hardhat node or anvil — v2 already dropped the `web3_clientVersion` probe as
  needless, so no `anvil` kind); public ids from `@fhevm/sdk/chains`; else `unknown`, which is
  NOT an error (the API throws when used, as v2 did at init); a configured chain id disagreeing with
  the node throws `HardhatPluginError`. `connection.fhevm` carries real `isCleartext`/`isDevelopment`,
  the deprecated `isMock` alias and the `network` info. Never gates on `networkName`.
  Test: six unit tests against fake connections + the live connection/node tests.
  Side effect: the plugin's `lint` script now runs `compile` first (its tests import the built
  payload; make's `lint-hh-v3-plugin` has no compile prerequisite) — same shape as the e2e.
  Commit: `feat(hh-v3-plugin): port network detection for edr, localhost, anvil, public chains`

- **A6. Consumer path resolution — LANDED.** `internal/paths.ts`: `resolveFromConsumer` via Node's
  own `createRequire` anchored at the project root (v2's `resolve` dependency dropped), and the
  `FhevmPaths` lazy getters — `root`, `nodeModulesDir`, `fhevmSolidityDir`, `fhevmSolidityConfigFile`
  (`ZamaConfig.sol`), `fhevmSdkDir` (pnpm: the real nested-store path). Missing sibling →
  `HardhatPluginError` naming specifier and root. Wired to `context.config.paths.root` at B1a.
  NOT ported, confirmed from v2's call sites: `cacheDir` (`fhevmTemp`, only the deleted remapping
  cache and its `clean` hook), `dotEnvFile` (the `.env` devnet flow → `configVariable`),
  `solidityCoverageDir`. Test: temp consumer trees — npm flat, pnpm symlink, missing sibling (also
  proves the plugin's own tree cannot leak into consumer resolution).
  Commit: `feat(hh-v3-plugin): port consumer path resolution for sibling npm modules`
- **A7. `onRequest` port — LANDED.** `internal/requests.ts`: `handleRequest` routes only
  `eth_estimateGas` and `eth_sendTransaction`; `inflateGasEstimate` ×1.2 in BigInt (no ethers where
  v2 needed it); `decorateSendError` is the hook shape, pass-through until D4a. Test: fake
  forwarder (exact arithmetic, pass-through, routing) + live: a plugin-less connection's estimate
  ×1.2 equals the plugin connection's. Finding: EDR estimates a plain transfer at 21001, not
  21000 — never hard-code estimates in tests.
  Commit: `feat(hh-v3-plugin): port gas inflation and send-error decoration to onRequest`

### Stage B — the essential job: pre-deploy per target

- **B1. In-process deploy** (three blocks):
  - **B1a — LANDED** (two commits: payload `dependencies` gets `@fhevm/host-contracts-cleartext`
    as the same cross-root `file:` link v2 uses + `ethers` peer `^6.17.0`, consumer-fixture lock
    regenerated; then the repository). `internal/contracts.ts` ports v2's `contractsRepository.ts`:
    ABIs from the package's `abi/*.json` export via `createRequire` (the plugin's OWN dependency,
    not a consumer sibling — A6's paths are not involved), ethers `Interface` + read-only
    `Contract` per host contract, lookups by name and case-insensitive address, optional contracts
    unregistered without an address, cleartext subclass + guard. Addresses are caller input: no
    local address literal. Dropped v2's duplicate wrapper `properties` (one consumer, the error
    decoder — D4a reads `name`/`address`/`readonlyContract`). Test: live provider, 3 tests.
    SUPERSEDED at B1b1: the repository now holds a viem `Abi` + read-only `getContract` (`contract`),
    and the payload peers `viem`, not `ethers` — see B1b1 and open question 2.
    Commit: `feat(hh-v3-plugin): load cleartext artifacts from the sibling contracts package`
  - **B1b1 — LANDED** (three commits: `switch the plugin to viem` — `viem` peer replaces `ethers`,
    `contracts.ts` re-based on viem `Abi` + read-only `getContract`, new `clients.ts`; `vendor the
    viem adapter` — `viemEthereumLib.ts` via sync-vendored after a dedicated common-vendored commit
    added `createViemEthereumAdaptersFromClients` (existing API untouched); then the deploy).
    `internal/deploy.ts` ports v2's `setup.ts`: one `deploy()` call through the vendored adapters,
    deployer from `mnemonicToAccount`, funded via `hardhat_setBalance` with the `anvil_` fallback,
    refused by name off its start nonce, idempotent when the ACL holds code. Addresses come from the
    package's `precomputeAddresses` (no literal); the four deployer values are plugin-local constants
    until C2. Finding: `deploy()` returns `aclOwnerAddress` in the node's casing (lower-case with the
    viem adapter, checksummed with ethers) while every other address is computed and checksummed —
    the plugin checksums it; the package or the viem adapter should, upstream.
    MEASURED AND DECLINED: injecting the package's `state/genesis.json` through the state cheats
    (175 calls, ~10 ms per fresh chain, versions verified) vs ~26 mined transactions; the user chose
    the `deploy()` TS API for parity with v2 — genesis stays a gitignored generator output.
    Test: addresses hold code + shipped `getVersion` strings, second run sends nothing, wrong-nonce
    deployer refused.
    Commit: `feat(hh-v3-plugin): port the nonce-ordered cleartext deploy sequence`
  - **B1b2 — LANDED.** `prepare.ts` runs `deployCleartextStack` inside `newConnection` when A5 says
    development (the A4 marker is gone); public networks get no request at all. The hook's WeakMap
    keeps each development connection's `Deployed` addresses for B1c and Stage D. Tests: two
    `create()` → two stacks, `getOrCreate()` twice → one, fake public connection → zero requests; the
    node test asserts ACL code on the first HTTP request and no redeploy from the remote connection;
    deploy and repository tests run plugin-less to keep a fresh chain. Cost: the plugin suite went from
    seconds to ~41 s (a dozen connections, one deploy each) — the per-chain price predicted below.
    Original spec, kept for the record: wire it to `newConnection`, development-class gating (A5),
    exactly once per CHAIN.
    Every `create()` on an `edr-simulated` network is a fresh chain and needs its own deploy; an
    `http` dev connection whose ZamaConfig addresses already hold code skips it. Test: `create()`
    twice → two deploys; `getOrCreate()` twice → one deploy; `http` with code present → zero;
    public chains untouched. Template/docs steer users to `getOrCreate` (per-file `connect()` in a
    mocha suite redeploys per file — a cost v2 never paid). Before committing the CREATE sequence,
    measure `hardhat_setCode`-style injection as the faster alternative.
    Commit: `feat(hh-v3-plugin): deploy the cleartext stack once per dev chain`
  - **B1c — LANDED, as VERIFICATION.** Setup needed no port: the package's `deploy()` registers the
    KMS/coprocessor signer sets and the HCU caps from its default bootstrap config. `internal/verify.ts`
    runs the package's own `verify({ mode: 'deploy', expected: { admin: deployer } })` after EVERY
    preparation (fresh deploy and http reuse alike — a half-deployed or foreign stack fails by name);
    event-scan and signer-expectation checks skip (no history adapter in process; defaults not
    stated). Findings for upstream: on a bare chain `verify()` throws from `owner()` instead of
    reporting, so the plugin treats a throw as a failed verification. Test: signer sets non-empty
    with thresholds inside, HCU caps positive (via the repository ABIs); verify passes on a prepared
    chain, refused by name on a bare one. 47 lines of code where 150 were estimated.
    Commit: `feat(hh-v3-plugin): run post-deploy signer and HCU setup on the fresh stack`
- **B2. Anvil deploy — LANDED, e2e only.** No plugin change: A5 classes anvil as `localhost`, the
  deployer is funded through `hardhat_setBalance` (anvil aliases it), the viem wallet signs locally.
  `hardhat/v3/e2e` gains `test:anvil`, `test:anvil:simple` and the two operator scripts ported from
  v2 (plain bash). Proven on anvil 1.5.1: first `--network anvil` run takes the ACL from no code to
  the proxy and the deployer from nonce 0 to 26 (block 27); a second run leaves the nonce alone and
  adds only the test's own block — the http skip-if-present path, verification included.
  Open: the `test-hh-v3-e2e-anvil` make target (v2 has one, excluded from `ci`) awaits approval.
  Commit: `feat(hh-v3-e2e): run the suite against an external anvil behind an operator script`
- **B3. `hardhat node` skip-if-present — LANDED** (no plugin source change: the same http reuse
  path anvil took at B2). Plugin test `node-process.test.ts` spawns hardhat's CLI (`node --port 0`,
  a fixture config carrying the plugin, CLI path via `hardhat/package.json` since the exports map
  hides `dist/src/cli.js`), reads the announced URL, asserts ACL code at that moment, then connects
  a second HRE over `http` and asserts deployer nonce and block number unchanged. e2e: `test:node` +
  `test/test-hardhat-node.sh` ported from v2, passing end to end. All three local targets now reach
  the stack through one mechanism.
  Commit: `feat(hh-v3-plugin): reuse the cleartext stack a hardhat node already deployed`

### Stage C — centralized generated constants (goal 7)

- **C1. fhevm-npm generator** (two blocks; the cleartext-config generator is the template):
  - **C1a — LANDED** (with the command half of C1b, one fhevm-npm commit + one for the re-synced
    JSON). `base/generate-chain-constants.ts`: `parseChainsConfig` (path-named rejections) and
    `renderChainConstants`, pure. The FACE: import-free TS, `FHEVM_CHAINS[group].hosts[name]` with
    hosts in the js-sdk shape `{ name, id, fhevm: { contracts } }`, contracts as `{ address }`
    objects, gateway + relayer on the GROUP (one gateway serves several host chains, and a host chain
    recurs across groups — Sepolia is under testnet AND devnet with different addresses), plus
    `FHEVM_CHAINS_SOURCE_COMMIT`. Decided on the way: `NETWORKS` → `NETWORK_GROUPS` = mainnet,
    testnet, DEVNET (registry `dist/devnet.json`, gateway 10900, hosts Sepolia/Amoy/BNB/Hoodi);
    relayer URLs and registry files move to `sdk/fhevm-network-groups.config.json`, the hand-written
    source of truth the sync renderer reads; gateway `kmsGeneration` and `multichainAcl` are optional
    (devnet carries their `_LEGACY` predecessors). Tests: exact table, prettier-clean, real config,
    quoting, 11 parser + 4 loader rejections, write/check/missing/drift lifecycle.
  - **C1b — LANDED.** Command (`generate-chain-constants` + `--check`, CLI, completion, README) in
    the C1a commit; then `common-vendored/package.json` gains the `generate:chain-constants` /
    `clean:generate:chain-constants` pair under its `generate` / `clean:generated` aggregates — no
    Makefile change, `make generate` and the `check-generated` gate already delegate to those verbs —
    and the first face `common-vendored/src/fhevm-chains.ts` (233 lines) is committed. Round trip
    proven: clean, generate, spotless tree; `--check` answers identical; vendored lint and the
    battery's scripts check pass. `sync-vendored` destinations for the copies are C2.

- **C2. Consumers — LANDED, two commits.** v3: the face is a `sync-vendored` destination of the
  plugin (manifest `vendored` entry, origin-checked); `network.ts` drops `@fhevm/sdk/chains` and
  classifies a remote chain as `public` when any group's host carries its id — `FhevmNetworkKind`
  is now `hardhat | localhost | public | unknown` and `FhevmNetworkInfo.publicChains` lists every
  (group, host) pair, since Sepolia and Amoy sit under testnet AND devnet; which group a public
  connection uses is a Stage D decision (v2 chose by the `devnet` network name + `.env`). v2: the
  face is vendored too; `EthereumConfig`/`SepoliaConfig` read `FHEVM_CHAINS`, the two dead Sepolia
  gateway constants (stale anyway) are deleted, and the face's literal types made two `as
  0x-string` assertions unnecessary. v2 plugin suite + e2e (2673 tests) green; the only 0x literals
  left in v2's constants are the cleartext stack's (a cleartext-config face, later). TODO.md notes
  that `network.ts` should import `@fhevm/sdk/chains` once the SDK generates it from the same JSON.
  Commits: `feat(hh-v3-plugin): resolve public chains from the generated chain constants`,
  `refactor(hh-plugin): replace hand-copied public addresses with the generated chain constants`

### Stage D — the public API, method-group by method-group

Each step ports one group of `HardhatFhevmRuntimeEnvironment` onto the connection-scoped object,
delegating to `@fhevm/sdk` (the published package — the workspace-member extraction can upgrade
this later without API change). One commit per group, tests run against the B1 stack:

- **D0. Public type surface — LANDED** (three commits: the lint tsconfig split, `types.ts` alone, then
  the wiring). `pkg/src/types.ts` is THE public API — every nameable type in one module, so the surface
  changes in one place; `index.ts` re-exports it and nothing else public. It mirrors v2's
  `HardhatFhevmRuntimeEnvironment` in viem terms (`Address`/`Hex`/`Log`/`TransactionReceipt`/`Abi`,
  `FhevmUser = WalletClient`), drops the engine-era members (`initializeCLIApi`, `getRelayerMetadata`,
  `generateKeypair`, positional `userDecrypt`/`delegatedUserDecrypt`, `publicDecrypt` stays), adds
  `network: FhevmNetworkInfo`, and carries `FhevmType` as a runtime enum. `FhevmConnection.ts` is a
  class implementing it: network facts live, every unported member throws
  `HardhatPluginError` "fhevm.<member> is not implemented" (getters `debugger`/`client` included) so a
  v2 test fails by member name. `test/api.test.ts` walks every member. The lint project had to split
  (`tsconfig.json` solution → `pkg/tsconfig.json` + `test/tsconfig.json`, `tsc -b`): tests import the
  built payload, and with a nominal enum the built and source copies of the `NetworkConnection.fhevm`
  augmentation no longer type-merge in one program.
  Was: `types.ts` → `HardhatFhevmRuntimeEnvironment` and friends, methods stubbed `not implemented`.
- **D1. Encrypt group — LANDED.** `createEncryptedInput` + `encryptUint/Bool/Address`, backed by the
  SDK client's `encryptValues`/`encryptValue` (`internal/encrypt.ts`, `internal/fheType.ts` for the
  enum→SDK-name bridge; euint4 refused by name). The client itself landed here too: `internal/client.ts`
  builds `createFhevmCleartextClient` over the connection's viem public client and the chain
  `internal/chains.ts` derives from the DEPLOYED stack (addresses from `Deployed`, gateway from the
  cleartext package's exported constants), awaits `ready`, and `newConnection` hands it to the
  connection object — the `client` getter is sync, so creation is eager, once per connection. The SDK's
  process-wide `setFhevmRuntimeConfig` is set to `{}` on first use; relayer auth joins when public
  networks do. On a non-development connection `fhevm.client` throws a named "not available" error:
  public clients wait on the network-group decision (Stage D open question). Consumer note: `handles`
  is `Hex[]`, so a `noUncheckedIndexedAccess` consumer must narrow `handles[0]` (the e2e test does).
  Was: `createEncryptedInput` + `encryptUint/Bool/Address`.
  Commit: `feat(hh-v3-plugin): port encrypted-input creation and encrypt helpers`
- **D2. publicDecrypt group — LANDED.** `publicDecrypt` keeps v2's handle-keyed result and carries the
  KMS proof (`decryptPublicValuesWithSignatures`) so `contract.verify(handles, abiEncodedClearValues,
  decryptionProof)` works on-chain; the typed variants coerce one `decryptPublicValue` and fail by
  handle on a type mismatch (`internal/decrypt.ts`). `internal/fhevmHandle.ts` holds the zero-handle
  guard ("Handle is not initialized" before the SDK's structural "chainId 0" message) and the
  bytes→hex key. Plugin tests cover the guards (zero handle in every variant and as bytes; an input
  handle nobody allowed is rejected by the stack); the success path is the e2e counter test, now the
  full v2 file (123 + on-chain verify, multiple, decrement, not-decryptable). `typeof` (handle
  parsing) is not assigned to a D step; it lands with D4b/D5 where the parser is needed anyway.
  Was: `publicDecrypt` + typed variants.
  Commit: `feat(hh-v3-plugin): port publicDecrypt and its typed variants`
- **D3.** `userDecrypt*` + EIP-712 builders (three blocks):
  - **D3a — LANDED as a REMOVAL.** In v2 both `createEIP712` and `createDelegatedUserDecryptEIP712`
    are `@deprecated` and THROW: they expose the relayer-sdk handshake (generateKeypair + EIP-712 +
    signTypedData) that `@fhevm/sdk` replaced with transport key pairs and signed permits, and no v2
    e2e test calls them. That is the D0 rule for engine-era members, so they and the `Kms*EIP712Type`
    types leave the v3 surface instead of being stubbed. Was: `createEIP712`.
    Commit: `refactor(hh-v3-plugin): drop the deprecated EIP-712 builders from the surface`
  - **D3b — LANDED.** `userDecryptEbool/Euint/Eaddress` (`internal/userDecrypt.ts`): fresh transport
    key pair, `signLegacyDecryptionPermit` (days→seconds, default 365 days from `timestampNow`, or
    `options.validity`), one `decryptValues`. The SDK's viem adapter signs through anything with a
    viem-style `signTypedData`, so **`FhevmUser = WalletClient | LocalAccount`**: a hardhat-viem wallet
    client (must carry its account, refused by name otherwise) or a viem local account. An ethers
    signer is NOT accepted — the plugin is viem-only; the e2e derives viem accounts from the suite
    mnemonic (`test/utils/signers.ts` `getAccounts`) and keeps sending transactions with ethers.
    `timestampNow` is a module export (index.ts), as in v2. Relayer-sdk leftovers `FhevmKeypair`,
    `options.keypair`, `HandleContractPair`, `UserDecryptResults` left the surface. e2e gained `viem`
    as an exact devDependency (root pin). Was: `userDecryptEbool/Euint/Eaddress`.
    Commit: `feat(hh-v3-plugin): port the userDecrypt typed variants`
  - **D3c — LANDED.** v2 promised "`userDecryptE*` with delegation options" when it deprecated
    `delegatedUserDecrypt`; that is the port: `FhevmUserDecryptOptions.delegatorAddress` names the
    account (typically a contract) whose handle the user decrypts on its behalf, and
    `signLegacyDecryptionPermit` receives it as `delegatorAddress`. Guarded by name. The v2 e2e file
    decrypts a ConfidentialERC20 balance — a 609-line contract corpus plus `@openzeppelin/contracts`,
    which is E2E-0b — so the v3 test plays the scenario on the counter: `SmartWalletWithDelegation`
    (copied verbatim) increments through its own `proposeTx`/`executeTx`, so the WALLET owns the
    handle; its delegate decrypts (own EOA, third EOA, after a second increment), an undelegated EOA
    and a revoked one are refused by the stack's "Delegate … is not delegated" error. `test/utils/blocks.ts`
    ports `waitNBlocks`. Was: the delegated user-decrypt flow.
    Commit: `feat(hh-v3-plugin): port the delegated user-decrypt permit flow`
- **D4.** Errors and events (two blocks):
  - **D4a.** The error layer is 827 v2 lines (`FhevmContractError` 703 + list 124) — the single
    biggest port item, and prime shared-layer material (§3: nothing in it touches `hre`). Three
    blocks, with a standing note that a shared `@fhevm/sdk` home beats porting a2 at all:
    - **D4a1 — LANDED.** `internal/errors/errorTable.ts`: the four v2 entries (InputVerifier
      `InvalidSigner`, ACL `SenderNotAllowed`, KMSVerifier `KMSInvalidSigner`, FHEVMExecutor
      `ACLNotAllowed`) plus the default custom-error line, as a frozen typed table; the engine will
      read it through `lookupErrorTemplates(contract, error)` (own-key checked) instead of v2's
      runtime `unknown` walk. `applyErrorTemplate` ported on `HardhatPluginError`, dead commented
      block dropped; message examples say `fhevm.createEncryptedInput` (no `hre.fhevm` in v3).
      Table-integrity test walks every tag. Was: the error LIST (data table).
      Commit: `feat(hh-v3-plugin): port the fhevm contract error table`
    - **D4a2 — LANDED** (~230 lines for v2's 703). Three modules under `internal/errors/`:
      `decode.ts` (viem `decodeErrorResult` against every repository ABI; exactly ONE owner, so the
      OpenZeppelin errors every proxy declares stay ambiguous and untouched), `messages.ts` (table
      templates filled from the error's ABI-named arguments — no per-contract switch any more;
      `InvalidSigner` takes the tx from/to; a template the values do not cover falls back to the
      generic line), `decorate.ts` (in-place rewrite of message + stack). Correction to A7: hardhat 3
      THROWS a failed request (EDR `SolidityError` with `data`/`transactionHash`/`stackTrace`, http
      `ProviderError` with `data` hex or `{data, transactionHash}`), so `requests.ts` catches, decorates
      and rethrows the SAME object (chai/ethers keep their `data`); every method is covered, not only
      sends. EDR blames the proxy's IMPLEMENTATION address, which the repository does not know, so the
      address hint falls back to the all-ABI decode. `internal/repository.ts` builds the contracts
      repository per development connection (WeakMap in the hook factory). Table fix: ACL
      `SenderNotAllowed` tag is `%sender%` (the ABI name). Dropped from v2: the `--verbose` error box
      (D4a3 decides logging), the exact-message precondition, the ethers `Interface` plumbing. Live
      test: `ACL.allow` from an unauthorised account surfaces as the FHEVM message.
      Was: the parsing engine, trimmed to what v3 consumers reach.
      Commit: `feat(hh-v3-plugin): port the fhevm contract error parsing engine`
    - **D4a3 — LANDED.** `revertedWithCustomErrorArgs(contract, error)` returns
      `[{ abi, interface }, error]`: the viem `abi` for viem-side assertions and an ethers-SHAPED
      `interface` (`internal/errors/interface.ts`: `getError(nameOrSelector)`, `decodeErrorResult` with
      `toArray()`) — exactly the slice `@nomicfoundation/hardhat-ethers-chai-matchers` reads, built on
      viem, no ethers in the plugin. Unknown contract or error name fails by name (v2 let chai fail).
      `tryParseFhevmError(e, { out })` (`internal/errors/parse.ts`) reads the revert from the error,
      from `e.error` or `e.info.error` (ethers' wrappers), structures InputVerifier `InvalidSigner`
      (tx parties via `getTransaction` when the hash is known) and prints a plain framed box
      (`internal/log.ts`, no color dependency) when `out` is given. The connection object now holds the
      contracts repository (`FhevmContractsRepository.client` made public). e2e: `TestErrors`,
      `TestTrivialPermissions`, `TestACL` contracts copied verbatim and their suites ported (18 e2e
      tests). Finding: v13's `InvalidSigner` carries an `address` argument.
      Was: the two public helpers.
      Commit: `feat(hh-v3-plugin): expose the fhevm error parsing helpers`
  - **D4b — LANDED.** `internal/events.ts`: the 30-name operator-event vocabulary (asserted equal to
    the executor ABI's events minus `Initialized`/`Upgraded`) and `parseCoprocessorEvents` over viem's
    `decodeEventLog`, restricted to logs FROM the executor address (v2 decoded any log against the
    executor interface). Input type `FhevmLog` is structural so a receipt from viem (`logIndex`) or
    ethers (`index`) both fit — the e2e keeps ethers receipts. `CoprocessorEvent.eventName` is the
    `CoprocessorEventName` union. Live test: `trivialEncrypt` sent straight to the executor yields
    one `TrivialEncrypt` event. e2e `TestAsyncDecrypt.ts` waits for D6 (`debugger`), as the ledger says.
    Was: `parseCoprocessorEvents`.
    Commit: `feat(hh-v3-plugin): port coprocessor event parsing`
- **D5.** HCU and coprocessor config (two blocks):
  - **D5a.** HCU is 1,162 v2 lines, over half of it a DATA table (`operatorsPrices` 523). Three
    blocks:
    - **D5a1 — LANDED as VENDORING.** v2's `operatorsPrices.ts` was already a VERBATIM copy of the
      fhevm repository's `library-solidity/codegen/src/operatorsPrices.ts` (header + one import line
      changed), with a small `priceTypes.ts` beside it. Upstream's source of truth is a TypeScript
      file, so a JSON price config would be a hand-converted duplicate — the "generation is
      premature" clause. Instead both files moved to `common-vendored/src` (master imports
      `./priceTypes.js`, which v3's ESM needs and v2's TypeScript resolves too) and became
      `sync-vendored` destinations of BOTH plugins (`common-vendored/manifest.json`,
      `npm-manifest.json`); v2's copies moved from `internal/hcu/` to `internal/vendored/`, its
      three imports and vendored README updated. `common-vendored/.prettierignore` keeps the upstream
      formatting. One copy in the workspace, byte-gated in both generations.
      Was: generated module first, verbatim port if generation is premature.
      Commit: `chore(sdk): vendor the upstream HCU price table in common-vendored`
    - **D5a2 — LANDED** in two commits (~330 source lines for v2's 650). `internal/hcu/prices.ts`
      bridges upstream operator names to executor event names over the vendored table
      (`hcuPriceOf`, `getHCU`, `getBucketedHCU`); `internal/hcu/fheTypeName.ts` maps `FhevmType`
      to the table's `Uint32` spelling; `internal/fhevmHandle.ts` gained `parseFhevmHandle`
      (protocol layout: index byte, chain id, type byte, version). `internal/hcu/hcu.ts` is the walk:
      it consumes D4b's `parseCoprocessorEvents`, so viem hands each family NAMED arguments and v2's
      77-line positional `eventArgs.ts` asserts collapse into four field guards; totals and
      depth-by-handle as v2. Input `FhevmTransactionReceipt` is structural (viem `status: 'success'`
      + `transactionHash`, or ethers `status: 1` + `hash`). Tests: every operator event priced except
      `VerifyInput`; a synthetic TrivialEncrypt→FheAdd receipt walks depth; a live `trivialEncrypt`
      costs exactly the table price. Public wiring (`computeTransactionHCU`, `typeof`) is D5a3.
      Was: the HCU engine, split by operator family if past the cap.
      Commits: `feat(hh-v3-plugin): port the HCU price bridge and handle parser`,
      `feat(hh-v3-plugin): port the HCU computation walk`
    - **D5a3 — LANDED.** `connection.fhevm.computeTransactionHCU(receipt)` over the D5a2 walk and the
      connection's executor; `fhevm.typeof(handle)` (the handle parser's type name — the member the
      plan had left unassigned); module exports `getHCU` (index.ts) and the `FheTypeName` type it
      takes. No sync stub is left on the surface. e2e: `internal/FHECounterHCU.ts` — the first
      `increment` on an uninitialized count costs TrivialEncrypt + FheAdd (`FHE.add` encrypts a zero
      first), the second a single FheAdd; the 752-line `FHEVMTestSuite1` corpus stays E2E-0b.
      That e2e forced `FhevmLog`/`FhevmTransactionReceipt` fields to plain `string` (ethers types them
      so); the decoders check the hex themselves.
      Was: `computeTransactionHCU` + module-level `getHCU` wiring.
      Commit: `feat(hh-v3-plugin): expose transaction HCU computation`
  - **D5b — LANDED.** `internal/coprocessorConfig.ts`: the ERC-7201 location of
    `confidential.storage.config` recomputed with viem and asserted against the constant, three
    `getStorageAt` reads, and the assertion against the connection's repository (ACL, executor, KMS
    verifier). Input is `FhevmAddressLike`: an address, a viem contract (`address`) or an ethers one
    (`getAddress()`), since v2 took `AddressLike`. The message names `ZamaEthereumConfig` (v2 said the
    stale `EthereumConfig`). Plugin test writes the three slots with `hardhat_setStorageAt` to cover
    empty, matching and foreign configs. e2e: `TestFHENotInitialized` (contract copied verbatim) and
    `internal/CoprocessorConfig.ts`, the ZamaConfig-trio smoke test deferred since B1c.
    Was: `getCoprocessorConfig` + `assertCoprocessorInitialized`.
    Commit: `feat(hh-v3-plugin): port coprocessor config read and init assertion`
- **D6 — LANDED as a PORT** (open question 3: `@fhevm/sdk` has no ACL-free cleartext read, and adding
  one is js-sdk work — open question 4's blocker — so the debugger stays plugin code with the extraction
  marker). `internal/debugger.ts`: `decryptEbool/Euint/Eaddress` read `CleartextDB.get(handle)`
  through the repository's viem client, cleartext networks only, type-checked by the handle parser,
  v2 messages kept. Dropped from the surface: `createDecryptionSignatures` (threw in v2, its only
  callers sit in the fully commented-out `TestAsyncDecrypt.ts`) and `createHandleCoder` (threw, no
  caller) — the D3a rule. With it the LAST stub left `FhevmConnection.ts`; `api.test.ts` now asserts
  every member live. e2e: `internal/FHECounterDebugger.ts` — after `incrementNotPubliclyDecryptable`
  the ACL refuses `publicDecrypt` while the debugger reads the count.
  Was: port vs fold into `@fhevm/sdk` mock tooling.
  Commit: `feat(hh-v3-plugin): port the fhevm debugger surface`

### Stage E — tasks

- **E1.** The decrypt tasks, lazy actions under an `emptyTask(["fhevm"], ...)` scope root; each
  action opens its connection with `hre.network.getOrCreate()` (no arguments → honours
  `--network`). Two blocks; each test: `hre.tasks.getTask([...])` resolves and a run round-trips
  against the B1 stack:
  - **E1a — LANDED.** `emptyTask(['fhevm'])` scope root + `task(['fhevm', 'public-decrypt'])` with
    POSITIONAL `type` and `handle` (hardhat 3 options always carry a default, so required inputs are
    positional — open question 7's "v3-idiomatic flags"). Action `tasks/publicDecrypt.ts`, lazy;
    opens `hre.network.getOrCreate()`; prints AND returns the value, so `task.run()` is testable. The
    skeleton's `hello` task retired with it. Commit: `feat(hh-v3-plugin): add the fhevm public-decrypt task`
  - **E1b — LANDED.** `task(['fhevm', 'user-decrypt'])`: positional `type`, `handle`, `contract`;
    option `--user <index>` (INT, default 0). The account is the network's `eth_accounts[index]`,
    wrapped as a viem wallet client over the connection's provider, so the permit is signed by the
    node (`eth_signTypedData_v4`) — no private key ever reaches the plugin. e2e
    `internal/FHECounterTasks.ts` runs both tasks through `tasks.getTask([...]).run()` against the
    counters. Commit: `feat(hh-v3-plugin): add the fhevm user-decrypt task`
- **E2 — LANDED.** `task(['fhevm', 'check-fhevm-compatibility'])`, positional `address`; action
  `tasks/checkFhevmCompatibility.ts` over the D5b surface: `getCoprocessorConfig` (all-zero + no code
  → "not a deployed contract"), then `assertCoprocessorInitialized` (uninitialized / mismatch, with the
  found configuration printed on stderr), success prints and returns the configuration. Plain text —
  v2's picocolors would be a new dependency for two colours. Plugin test drives all four outcomes with
  `hardhat_setCode` + `hardhat_setStorageAt`; e2e runs it on the counter and on an empty address.
  Was: `["fhevm", "check-fhevm-compatibility"]`, test likewise.
  Commit: `feat(hh-v3-plugin): add fhevm check-fhevm-compatibility task`
- **E3 — LANDED, one survivor.** `clean` is dead too: the plugin owns no cache directory (A6 put
  `cacheDir` in the delete bucket). `node` survives for the banner and nothing else:
  `overrideTask('node')` (`tasks/node.ts`) flags a process-wide request and hands over to `runSuper`;
  the node task creates its connection through the network hooks, whose `newConnection` deploys the
  stack and, when the flag is set, prints `internal/nodeBanner.ts` BEFORE hardhat's "Started HTTP and
  WebSocket JSON-RPC server" line: ONE line always (network, chain id, deployed-by-this-node or reused,
  verified), the ten addresses + deployer only when `hre.globalOptions.verbosity` exceeds hardhat's
  default 2. NOTE hardhat's verbosity is the COUNT of v's (`-v` = 1, `-vv` = 2 = default), so the
  table needs `-vvv` — the level hardhat's own call traces start at. `prepare.ts` now reports `reused`
  (`isCleartextStackDeployed`). A test run prints no banner (one per in-process connection would be
  noise). The child-process test asserts the banner, the ACL address and the ordering.
  Was: candidates `clean` and `node`, one commit per survivor.
  Commit: `feat(hh-v3-plugin): keep the node builtin override for the fhevm stack banner`

### Stage F — packaging and fidelity

- **F1 — LANDED.** Finding first: the fixture still asserted the retired `hello` task and the plugin's
  `test:consumer` script lacked `--run`, so the leg installed and never executed — green by omission
  since E1. Fixed (`--run --build-linked-dependencies`, as the cleartext packages spell it) and the
  fixture upgraded: plain-JS consumer declaring the peers (`@fhevm/sdk`, `viem`) as a user would;
  tasks + module exports registered; a connection deploys the stack; `encryptUint` → handle + proof;
  `trivialEncrypt` sent to the executor (its localhost address is deterministic and hard-coded with a
  comment — the public surface exposes no stack addresses, a gap to weigh in F2/§5); event parsed,
  `typeof`, `debugger.decryptEuint` = 42, HCU = table price, `publicDecrypt` refused. Lockfile
  regenerated with `test-consumer-regenerate-package-lock`; both `--run` and `--run --ci` pass.
  With approval the Makefile's `test-consumer`/`test-consumer-ci` targets gained the v3 plugin leg
  (prerequisite `compile-hh-v3-plugin`); `make test-consumer-ci` runs six legs green.
  Was: consumer fixture upgraded to a real encrypt/decrypt round-trip.
  Commit: `test(hh-v3-plugin): consumer fixture runs an encrypt/decrypt round-trip`
- **F2 — LANDED.** publint was clean; attw's only finding is `CJSResolvesToESM`, inherent to an
  ESM-only hardhat 3 plugin, so `check:publint` runs attw with `--profile esm-only` and `npm run check`
  is green. Finding: `pkg/_esm` was never cleaned before a compile, so the tarball still shipped the
  retired `hello` action — `compile:esm` now removes the directory first. `pkg/README.md` written
  (install, configure, `connection.fhevm` members, tasks, the node banner, supported networks); the
  payload description no longer says "skeleton". NOT done (outside the write scope): the
  `npm-manifest.json` note for the v3 owner still reads "hello-world skeleton until the migration
  lands". Version: see open question 5 (decided later the same day: `@fhevm/hardhat-plugin-v3` `0.13.0`).
  Was: publint/attw/pack green; pkg README.
  Commit: `chore(hh-v3-plugin): publint, attw and pack:tarball green; package README`
- **F3 — CLOSED for development networks** (no new e2e commit needed: every row the suite can reach
  was already green — see §5 "F3 status"). Left open, recorded in §5 and §7: the public-chain client
  and the Sepolia operator flows (network-group decision + live RPC). Suggested but not done (Makefile
  is outside the write scope): `test-hh-v3-e2e-node` / `-anvil-managed` targets wrapping the two
  operator scripts, so the flows are one `make` away.
  Was: close the §5 matrix, one row per commit.

### The e2e ledger — which v2 test file ports at which plugin step

The rule: a v2 test file ports in the SAME commit as the plugin step that makes its last missing
API call work (or the very next commit if the cap forces a split), never earlier, never batched
with unrelated files. The contracts a test deploys travel with it (E2E-0b), verbatim. The API-per-file map below is grepped from `hardhat/v2/e2e/test`; the
`hardhat-mock-engine/` prefix becomes `test/` in v3 (there is no other engine). Each ported file
swaps `import { ethers, fhevm } from 'hardhat'` for `const { ethers, fhevm } = await
network.connect()` (or the A3 alias, if kept) and keeps everything else byte-for-byte where hh3
allows.

| unlocked at            | v2 test files (API they need)                                                                                                                                                                                                                      |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| E2E-0a                 | DONE — `internal/FHECounterPublicDecrypt.ts` (the full v2 file since D2)                                                                                                                              |
| A5                     | DONE — the `isCleartext` guard is live in `internal/FHECounterPublicDecrypt.ts`; `utils.ts` (`isDevelopment`) ports with its first user                                                                                                                                          |
| B1c                    | DONE at D5b — `internal/CoprocessorConfig.ts` (the ZamaConfig-trio smoke test) and `internal/TestFHENotInitialized.test.ts`                                                                                       |
| B2                     | DONE — `test:anvil` + `test:anvil:simple` scripts and `test/test-anvil*.sh` (run the counter file)                                                                                                                                                       |
| B3                     | DONE — `test:node` script + `test/test-hardhat-node.sh`                                                                                                                                                                                                  |
| D1                     | DONE for the counter: `internal/FHECounterPublicDecrypt.ts` gained "increment the counter by 1" (encrypt + tx; the public-decrypt half joins at D2). `internal/delegatedUserDecryption.ts` and `finance/ConfidentialVestingWallet*.test.ts` wait on their contracts (contracts landed at E2E-0b; test port pending) |
| D2                     | DONE for the counter: `internal/FHECounterPublicDecrypt.ts` is the full v2 file. `internal/Rand.ts` + fixture DONE (2026-09-03), `doc-examples/HeadsOrTails.ts` DONE, `doc-examples/HighestDieRoll.ts` DONE (2026-09-03), `operators-public-decrypt/fhevmOperations54.ts`, `operators-manual/manualV13.ts` contracts landed at E2E-0b; test port pending |
| D3b                    | DONE for `internal/FHECounterUserDecrypt.ts` (viem accounts as users). `internal/AplusB.ts` DONE (2026-09-03). Contracts landed at E2E-0b, test port pending: `doc-examples/{DecryptSingleValue,DecryptMultipleValues,EncryptSingleValue,EncryptMultipleValues}.ts` DONE (2026-09-03), `confidentialERC20/*`, `finance/*.fixture.ts` DONE (2026-09-03), `governance/*`, `utils/EncryptedErrors.*`, `operators-manual/manualWithAllowSender.ts` |
| D3c                    | DONE on the counter (`internal/delegatedUserDecryption.ts`, `SmartWalletWithDelegation.sol`, `utils/blocks.ts`); the ConfidentialERC20 version DONE (2026-09-03) as `internal/delegatedUserDecryptionERC20.ts`                                                                                                                                                                                     |
| D4a3                   | DONE — `internal/TestErrors.test.ts`, `internal/TestTrivialPermissions.test.ts`, `internal/TestACL.ts` (contracts copied verbatim)                                                                                                                          |
| D4b                    | MOOT — `internal/TestAsyncDecrypt.ts` is 100% commented out in v2 (mock-engine era); nothing to port                                                                                                                                                                          |
| D5a3                   | DONE — `internal/FHECounterHCU.ts` and `hcu/fhevmHCU1.ts` (v2 file, over `FHEVMTestSuite1`)                                                                                                                                                                                                      |
| D5b + C2               | `sepolia/*` (`getCoprocessorConfig` + generated addresses) and the `test:sepolia:*` scripts — operator-run, never in `make test`                                                                                                                    |
| D6                     | DONE on the counter (`internal/FHECounterDebugger.ts`); `operators/fhevmOperations1…13.ts` and `operators-manual/manual.ts` corpus landed at E2E-0b; test port pending                                                                                                              |

Running total is the progress meter for goal 3: when the last row is green, the public API is
proven equivalent on hardhat 3 by the same tests that prove it on hardhat 2.

Stage order is deliberate: A4 (formerly the riskiest unknown, now a one-commit ordering proof)
sits as early as scaffolding allows, the essential job (§1) is proven before any API surface is
ported onto it, and constants land before the API port so no step ever introduces a hand-copied
address even temporarily.
