# Porting the FHEVM hardhat plugin from hardhat v2 to hardhat v3

Status: **plan for review — nothing implemented.** The landing zone exists: the `hardhat/v3` cluster
(own installation root, hardhat 3.15 pinned, hello-world plugin proving the topology, tasks, and the
single-instance guarantee end to end).

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
natural v3 home is the `network.newConnection` chain (deploy-once per connection to a
development-class network), which is exactly what the phase-2 spike must prove for all three local
targets.

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
  Deploy, config checks and compile integration all stand on it; in v3 it anchors on the resolved
  project root from the hook context instead of `hre.config.paths`. Also `internal/constants.ts` (113 lines — includes HAND-COPIED addresses:
  the ZamaConfig trio, Sepolia gateway `DECRYPTION_ADDRESS`/`INPUT_VERIFICATION_ADDRESS`),
  `internal/vendored/ethersEthereumLib.ts` (from common-vendored).

## 2. Hardhat 3 plugin API — the facts (from hardhat.org/docs/plugin-development)

Confirmed against the official docs (plugin-development, /explanations/hooks, /lifecycle,
/type-extensions):

- A plugin is an OBJECT: `{ id, dependencies, conditionalDependencies, hookHandlers, tasks,
globalOptions }`. Hook handlers and task actions are LAZY-LOADED modules — nothing runs at import.
- Hook categories: `config` (runs BEFORE the hook context exists — pure config transforms),
  `network` (`newConnection`, `onRequest`, `closeConnection`), `test`, `solidity`, `hre`.
  Chained hooks receive `(context, args, next)` — chain-of-responsibility; chain order is dynamic
  handlers first, then plugins in reverse dependency order, then the default.
- **Per-connection state is the documented pattern for what `hre.fhevm` was**: a `WeakMap` created
  in the hook-handler category factory, populated in `newConnection`, cleaned in `closeConnection`.
- Type extensions: `declare module 'hardhat/types/network'` to add to `NetworkConnection` (and
  `hardhat/types/hre` for the HRE), runtime property attached by the matching hook, and the plugin's
  `index.ts` re-exports `export type * from './type-extensions.js'`. Type-only file, `export {}`.
- Task overrides across plugins execute in reverse plugin order — the builtin-override pattern
  (`test`, `clean`, `node`) has a first-class home.

## 2b. The v2 → v3 translation (what v3 forces — the allowed breaking changes)

| v2 mechanism                    | v3 replacement                                                                 | consequence                                                                                                                                                                                                                                                                       |
| ------------------------------- | ------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| side-effect `extendEnvironment` | plugin OBJECT with `hookHandlers`                                              | declarative wiring, no import-order magic                                                                                                                                                                                                                                         |
| `hre.fhevm` global singleton    | hre created per invocation; networks are CONNECTIONS (`hre.network.connect()`) | **the one real API break**: fhevm state binds to a connection — `connection.fhevm`, implemented exactly as the docs prescribe (WeakMap in the network hook factory, `newConnection`/`closeConnection`); an `hre.fhevm` convenience alias to the default connection where possible |
| `extendProvider` wrapper        | `network` hook handlers (`onRequest` interception)                             | same interception point, hook-shaped                                                                                                                                                                                                                                              |
| `extendConfig`                  | `config` hook handlers (`extendUserConfig`/`resolveUserConfig`)                | mechanical                                                                                                                                                                                                                                                                        |
| eager task actions              | lazy action modules (`setAction(() => import('./tasks/x.js'))`)                | proven in the skeleton                                                                                                                                                                                                                                                            |
| CJS build (`node10`)            | ESM-only, `nodenext`                                                           | drops the node10 `paths` workaround for `@fhevm/sdk/chains` entirely                                                                                                                                                                                                              |

**Formerly-open items, settled from the hardhat 3.15 SOURCE** (dist/src/types + internal/builtin-plugins):

- `hardhat node` is a BUILTIN PLUGIN (`builtin-plugins/node`: `task("node", "Start a JSON-RPC server
on top of Hardhat Network")`, with `json-rpc/` and `task-action` internals) — overridable through
  the reverse-plugin-order task-override mechanism; the phase-2 spike hooks there.
- Scoped tasks are string-array ids: `task(["fhevm", "user-decrypt"], ...)`;
  `hre.tasks.getTask(string | string[])`.
- `network.onRequest(context, connection, jsonRpcRequest, next) => JsonRpcResponse` intercepts the
  full JSON-RPC stream with chaining — it covers everything v2's `extendProvider` did.
- `network.newConnection(context, next) => NetworkConnection` is a DECORATOR chain: call `next()`,
  attach `fhevm` to the returned connection, return it — even simpler than the WeakMap pattern for
  the attach itself (the WeakMap remains useful for teardown state); `closeConnection` mirrors it.
- `hre.created(context, hre)` exists for the `hre.fhevm` convenience alias.
- `coverage` and `gas-analytics` are builtins with `network.onCoverageData` / `onGasMeasurement`
  hooks — confirming the v2 solidity-coverage workarounds are DELETE-bucket, and offering a native
  home for HCU-style measurement later.

## 3. Minimization strategy (goal 2) — three buckets for every v2 file

- **DELETE — v2 workarounds hardhat 3 obsoletes.** Candidates (confirm each at port time): the
  compile remappings subtask override (hh3 resolves npm imports natively), the source-paths
  override, `utils/solidityCoverage.ts` (hh3 has built-in coverage), ts-node/CJS glue,
  the `@fhevm/sdk/chains` tsconfig `paths` workaround, most of the provider-wrapper gymnastics.
- **SHARE — hardhat-agnostic logic leaves the plugin.** The plugin already delegates crypto to
  `@fhevm/sdk` (`client: FhevmClient`; `chains.ts` imports `@fhevm/sdk/chains`; the in-code
  `TODO(migration step N)` markers chart this). Everything not touching `hre` belongs there:
  handle/type helpers (`fhevmHandle`, `fheType`), HCU computation and price tables, EIP-712
  builders, encrypted-input assembly, error parsing. **v2 benefits equally** — finish the shared
  extraction in `@fhevm/sdk` first, shrink v2 onto it, then v3 imports the same surface. The v3
  plugin ends up as ADAPTER GLUE ONLY: hooks, tasks, network detection, config.
- **PORT — the thin remainder**, one small function at a time (goal 6): the hook handlers, the
  task definitions, `networkProvider` chain detection, the deploy/setup path for the cleartext
  stack, and `FhevmEnvironmentPaths`/`resolveFromConsumer` (the sibling-module locators — see §1;
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
| in-process `hardhat` network                    | auto-deploy cleartext stack, mock mode, `hre.fhevm` fully functional                                                   |
| `hardhat node` (`--network localhost`)          | the node-side hooks deploy/serve the stack; a second process connects and finds it; chainId 31337 semantics preserved  |
| anvil (`--network anvil`)                       | detection by network name + chain probing; same stack deploy against an external anvil; the `test:anvil` operator flow |
| public chains (sepolia, mainnet, polygon, amoy) | resolved via `@fhevm/sdk/chains`; no stack deploy; addresses from the generated constants                              |

`hardhat node` is the highest-risk row: v3's node/server hook surface differs most — prototype it
FIRST (phase 2) so the design fails early if the hook points don't exist.

## 6. Phases

1. **Shared extraction (shrinks v2, not just v3):** move the hardhat-agnostic bucket into
   `@fhevm/sdk` behind its existing entry points; v2 plugin consumes it; v2 suite + e2e stay green.
   This phase is measured by v2's line count going DOWN.
2. **v3 spike — the risky adapters only:** hre/connection extension exposing a minimal
   `fhevm` object, the `hardhat node` hooks, anvil detection. Throwaway allowed; goal is to pin
   the v3 hook surface and the connection-scoping decision (the one API break) with the user.
3. **Port the public API:** `FhevmExternalAPI` on v3, delegating to the shared layer; the
   generated constants module; vendored destinations for v3.
4. **Tasks:** the fhevm scope + whichever builtin overrides still have a job in v3.
5. **`hardhat/v3/e2e` + `hardhat/v3/fhevm-hardhat-template`** (already-reserved cluster slots):
   the parity matrix of §5 runs there; template is born workspace-native under option E.
6. Consumer fixtures + `check:publint`/`pack:tarball` already exist from the skeleton; extend the
   fixture to exercise one encrypt/decrypt round-trip.

Each phase ends green: `make build`, the affected test tiers, and the fhevm-npm battery.

## 7. Open questions

1. **Connection scoping — the one deliberate API break (⚑ gate A3).** `connection.fhevm` with an
   `hre.fhevm` convenience alias, or `hre.fhevm` bound to the default connection only? Sub-question
   to answer first: maximum v2 compatibility (alias — most v2 test suites port unchanged) vs maximum
   v3 idiom (connection-only — cleaner, every consumer updates). Decide after the phase-2 spike.
2. **The ethers bridge.** Keep `@nomicfoundation/hardhat-ethers` (v3 flavor) or go provider-direct
   through `@fhevm/sdk`'s adapters? Affects how much of `utils/ethers.ts` survives.
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
| A4a/b/c node spike                   | builtin-tasks 149 (node part)            | 40/80/40     | ~100/160/160     |
| A5 network detection                 | networkProvider 233                      | 160          | ~280             |
| A6 path resolution                   | Paths 148 + path 10                      | 120          | ~240             |
| B1a contracts repository             | contractsRepository 202                  | 150          | ~230             |
| B1b1/B1b2 deploy (split, see below)  | FhevmEnvironment 898 (deploy slice)      | 150/150      | ~230/230         |
| B1c post-deploy setup                | deploy/setup 198                         | 150          | ~230             |
| B2 anvil deploy                      | delta on B1                              | 80           | ~180             |
| B3 node deploy                       | delta on A4+B1                           | 100          | ~220             |
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

- **A1. Config hook.** `config.extendUserConfig`/`resolveUserConfig` accepting the v2-compatible
  `fhevm` user-config keys, plus the `hardhat/types/config` augmentation.
  Test: programmatic HRE with an `fhevm` config block; assert the resolved shape and defaults.
  Commit: `feat(hh-v3-plugin): add fhevm config hooks and config type extensions`
- **A2. Connection decoration.** `network.newConnection` attaches a stub `connection.fhevm`
  (`isMock`/`isCleartext` placeholders), `closeConnection` cleans up; `NetworkConnection`
  augmentation. Test: two connections get DISTINCT fhevm instances; close releases state.
  Commit: `feat(hh-v3-plugin): attach per-connection fhevm object via network hooks`
- **A3. ⚑ `hre.fhevm` alias** (open question 1 decided here): `hre.created` attaches the
  default-connection alias, or the alias is dropped. Test matches the decision.
  Commit: `feat(hh-v3-plugin): expose hre.fhevm alias for the default connection`
- **A4. SPIKE — `hardhat node`** (three blocks; the port's riskiest assumption dies or survives
  here; `internal/spike/` code allowed until the shape settles):
  - **A4a.** Override the builtin `node` task as a pure pass-through (reverse-plugin-order
    override; internals at `builtin-plugins/node/{json-rpc,task-action}`).
    Test: `hardhat node` still starts and serves with the override installed.
    Commit: `feat(hh-v3-plugin): override the builtin node task as a pass-through`
  - **A4b.** Find the pre-serve hook point: prepare the chain (write a marker transaction/state)
    BEFORE the server accepts connections. Test: the marker exists on first request.
    Commit: `feat(hh-v3-plugin): prepare the node chain before the server accepts requests`
  - **A4c.** External detection: a second process connects over `--network localhost` and finds the
    marker. Test: spawn node as a child process, probe from a second HRE, tear down.
    Commit: `feat(hh-v3-plugin): detect a prepared hardhat node from a second process`
- **A5. Network detection.** Port the `networkProvider` chain-detection minimum (in-process vs
  `localhost` vs `anvil` vs public chain ids from `@fhevm/sdk/chains`), no deploys yet.
  Test: unit tests against a fake provider (chainId/clientVersion fixtures).
  Commit: `feat(hh-v3-plugin): port network detection for hardhat, localhost, anvil`

- **A6. Consumer path resolution.** Port `FhevmEnvironmentPaths` + `resolveFromConsumer` — the
  sibling-npm-module locators (`@fhevm/solidity`, `ZamaConfig.sol`, `@fhevm/sdk` incl. pnpm layout,
  consumer `node_modules` root) — anchored on the v3 resolved project root.
  Test: unit tests against a fixture consumer tree (npm and pnpm layouts); missing sibling → the
  named, actionable error.
  Commit: `feat(hh-v3-plugin): port consumer path resolution for sibling npm modules`

### Stage B — the essential job: pre-deploy per target

- **B1. In-process deploy** (three blocks):
  - **B1a.** Port the contracts repository: load the cleartext artifacts/bytecode from the sibling
    `@fhevm/host-contracts-cleartext` package (stands on A6's paths).
    Test: artifacts resolve and parse from a fixture consumer tree.
    Commit: `feat(hh-v3-plugin): load cleartext artifacts from the sibling contracts package`
  - **B1b1.** The deploy transaction sequence itself (nonce-ordered CREATEs onto the ZamaConfig
    addresses), as a pure function of (provider, artifacts). Test: addresses hold code afterwards.
    Commit: `feat(hh-v3-plugin): port the nonce-ordered cleartext deploy sequence`
  - **B1b2.** Wire it to `newConnection`: development-class gating, exactly once per connection.
    Test: two test files in one run, one deploy; non-dev networks untouched.
    Commit: `feat(hh-v3-plugin): deploy the cleartext stack once per dev connection`
  - **B1c.** Post-deploy setup (signers registration, HCU caps — v2's `deploy/setup.ts`).
    Test: coprocessor/KMS signers registered; `assertCoprocessorInitialized` path green.
    Commit: `feat(hh-v3-plugin): run post-deploy signer and HCU setup on the fresh stack`
- **B2. Anvil deploy.** Same flow against an external anvil.
  Test: behind a `test:anvil` operator script, exactly like v2's.
  Commit: `feat(hh-v3-plugin): pre-deploy the cleartext stack against external anvil`
- **B3. `hardhat node` deploy** (builds on A4). Test: spawn node, connect `--network localhost`
  from a second HRE, find the stack without deploying again.
  Commit: `feat(hh-v3-plugin): serve a pre-deployed cleartext stack from hardhat node`

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

- **E1.** The decrypt tasks, lazy actions (two blocks; each test:
  `hre.tasks.getTask([...])` resolves and a run round-trips against the B1 stack):
  - **E1a.** `task(["fhevm", "public-decrypt"])`.
    Commit: `feat(hh-v3-plugin): add the fhevm public-decrypt task`
  - **E1b.** `task(["fhevm", "user-decrypt"])`.
    Commit: `feat(hh-v3-plugin): add the fhevm user-decrypt task`
- **E2.** `["fhevm", "check-fhevm-compatibility"]`. Test likewise.
  Commit: `feat(hh-v3-plugin): add fhevm check-fhevm-compatibility task`
- **E3.** Builtin overrides that SURVIVE the delete-bucket triage (expected: little or nothing
  beyond A4's node override — `coverage` and remappings are hh3-native). One commit per override,
  each with the reason it still exists in its commit message.
  Commit: `feat(hh-v3-plugin): keep <name> builtin override — one commit per survivor`

### Stage F — packaging and fidelity

- **F1.** Consumer fixture upgraded to a real encrypt/decrypt round-trip. Test: `test-consumer` leg.
  Commit: `test(hh-v3-plugin): consumer fixture runs an encrypt/decrypt round-trip`
- **F2.** `check:publint`/`attw`/`pack:tarball` green; pkg README. Commit.
  Commit: `chore(hh-v3-plugin): publint, attw and pack:tarball green; package README`
- **F3.** `hardhat/v3/e2e` cluster member: the §5 parity matrix becomes its test suite, row by row
  (each row = one commit).
  Commit: `feat(hh-v3-e2e): add e2e member covering one node-parity-matrix row`

Stage order is deliberate: A4 (the riskiest unknown) sits as early as scaffolding allows, the
essential job (§1) is proven before any API surface is ported onto it, and constants land before
the API port so no step ever introduces a hand-copied address even temporarily.
