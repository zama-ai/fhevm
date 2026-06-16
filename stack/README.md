# stack/ — fhevm-cli redesign exemplar

This directory is a **skeleton / exemplar** for the fhevm-cli redesign. It is NOT a
working implementation. Nothing here boots a cluster. The purpose is to record the
intended architecture precisely enough that a developer can build the real thing from
it, and to anchor every design decision to real code or real chart facts from this repo.

---

## What problem this solves

The current orchestrator (`test-suite/fhevm/src/`) is roughly 10,400 lines of TypeScript
that drives a Docker Compose–based stack, resolves images, renders env files, and runs
rollout runbooks. The redesign replaces the Docker Compose layer with a thin driver over
the real Helm charts already in `charts/*` at the repo root, targeting kind as the local
engine. The business logic surface (the runbooks) stays the same; only what sits under
it changes.

---

## Two layers

```
┌────────────────────────────────────────────┐
│  Front-ends: CLI  +  runbooks              │  (thin callers of the Stack API)
├────────────────────────────────────────────┤
│  Stack API  (lib/stack.ts)                 │  (typed interface, ~200 LoC)
├────────────────────────────────────────────┤
│  DATA layer                                │
│    charts/*       — the real Helm charts   │
│    values/*.yaml  — per-scenario overrides │
└────────────────────────────────────────────┘
```

### Layer 1 — DATA

The data layer is the real charts already present in this repo and values files that
overlay them. No business logic lives here. The charts are:

| Chart | Purpose |
|---|---|
| `charts/anvil-node` | local EVM node (Foundry Anvil) |
| `charts/contracts` | sc-deploy Job + sc-addresses ConfigMap wiring |
| `charts/coprocessor` | all coprocessor components (dbMigration, hostListener, tfheWorker, …) |
| `charts/kms-connector` | KMS connector bridge to external kms-core |
| `charts/listener` | multi-chain listener core |

Values files under `stack/values/` are STUBS that illustrate which knobs a scenario
overrides. All key names in those files are taken verbatim from the charts above.

### Layer 2 — Stack API

A small typed interface over the engine. Extracted from and compatible with the existing
`RolloutRunContext` defined in
`test-suite/fhevm/src/commands/rollout-run.ts`. The full real interface is:

```typescript
// from test-suite/fhevm/src/commands/rollout-run.ts (REAL, lines 53-65)
export type RolloutRunContext = {
  applyVersionLock(label: string, options: RolloutVersionLockOptions): Promise<void>;
  readState(): Promise<State>;
  refreshDiscovery(): Promise<void>;
  runGatewayContractTask(command: string, options?: RolloutContractTaskOptions): Promise<void>;
  runHostContractTask(command: string, options?: RolloutContractTaskOptions): Promise<void>;
  snapshotContracts(surface: "host" | "gateway"): Promise<void>;
  stateDir(): string;
  test(profile?: string, options?: RolloutTestOptions): Promise<void>;
  up(options: RolloutUpOptions): Promise<void>;
  upgradeRuntimeGroup(group: string, options?: RolloutRuntimeUpgradeOptions): Promise<void>;
  writeVersionLock(name: string, options: RolloutLockOptions): Promise<string>;
};
```

`lib/stack.ts` extends this with lower-level chaos/read-state primitives required by the
new engine:

```typescript
// STUB additions — not yet implemented
exec(pod: string, cmd: string[]): Promise<string>;
sql(query: string): Promise<unknown[]>;
stop(pod: string): Promise<void>;
start(pod: string): Promise<void>;
restart(pod: string): Promise<void>;
logs(pod: string, opts?: { since?: string; tail?: number }): AsyncIterable<string>;
waitForLog(pod: string, pattern: RegExp, timeoutMs?: number): Promise<void>;
chain<T>(steps: Array<() => Promise<T>>): Promise<T[]>;
until(condition: () => Promise<boolean>, pollMs?: number, timeoutMs?: number): Promise<void>;
state(): Promise<StackState>;
discovery(): Promise<Discovery>;
pin(group: string, tag: string): Promise<void>;
```

---

## Target engine

**Primary: kind + the real Helm charts.**

A kind cluster with a local registry is the intended local engine. Each chart in
`charts/*` already has `readinessProbe` defined and is suitable for Helm install into a
kind namespace. The CLI calls `helm upgrade --install` per chart, in dependency order.

**Fallback: Docker Compose.**

Docker Compose remains supported only as a fallback while the chart-on-kind spike is in
progress. It is not the long-term target. `lib/stack.ts` abstracts the engine so that
the runbooks and CLI do not import Compose types directly.

kms-core is always EXTERNAL — it is not a chart in this repo and is not managed by
this CLI. The kms-connector chart points at it via `kmsCoreEndpoints`
(see "Endpoint indirection" below).

---

## Three data contracts

These are the explicit compatibility contracts between the data layer (charts/values) and
the callers (CLI, runbooks).

### Contract 1 — MANIFEST (image tags)

A Helm values file maps every component image to a specific tag. This is the single
source of truth for "what version is running". There are no `--target`, `--override`, or
live-resolve flags. Example (REAL keys from the charts):

```yaml
# stack/values/manifest.yaml — STUB illustrating the contract
scDeploy:
  image:
    tag: v0.13.0          # charts/contracts/values.yaml: scDeploy.image.tag

coprocessor:
  dbMigration:
    image:
      tag: v0.13.0        # charts/coprocessor/values.yaml: dbMigration.image.tag
  tfheWorker:
    image:
      tag: v0.13.0        # charts/coprocessor/values.yaml: tfheWorker.image.tag
  hostListenerShared:
    image:
      tag: v0.13.0        # charts/coprocessor/values.yaml: hostListenerShared.image.tag
  gwListener:
    image:
      tag: v0.13.0        # charts/coprocessor/values.yaml: gwListener.image.tag

kmsConnector:
  kmsWorker:
    image:
      tag: v0.13.0        # charts/kms-connector/values.yaml: kmsWorker.image.tag
```

The CLI reads this file via `applyVersionLock` / `writeVersionLock`. It never computes
tags at runtime from registry queries.

### Contract 2 — ENDPOINT indirection

Services that need to talk to a process running outside the cluster (or in a different
deployment) are pointed there via named variables in the values layer, not by hard-coded
in-cluster DNS names. The two canonical examples from the real charts:

**kmsCoreEndpoints** (REAL — `charts/kms-connector/values.yaml` line 201):
```yaml
kmsWorker:
  config:
    kmsCoreEndpoints: "http://kms-core:50051"
```
Overriding this in a scenario values file redirects the kms-connector to a local kms-core
process instead of the in-cluster one, without changing any chart template.

**hostChainWsUrl / gatewayUrl** (REAL — `charts/coprocessor/values.yaml` lines 62-71):
```yaml
commonConfig:
  hostChainWsUrl: "ws://ethereum-node:8546"
  hostChainHttpUrl: "http://ethereum-node:8545"
  gatewayUrl: {}
```
These are the analogous indirection points for the coprocessor chart.

The general pattern: any `*Url`, `*Endpoint`, or `*_ENDPOINT` key in a values file is an
indirection point. Runbooks and the CLI only override these keys; they never patch
template-rendered DNS names.

### Contract 3 — RUNBOOKS

A runbook is a TypeScript module that exports a `default` (or named `run`) function with
the signature `(ctx: RolloutRunContext) => Promise<void>`. The CLI loads and executes it.
The real example is `test-suite/fhevm/rollouts/v0.12-to-v0.13/run.ts`.

Runbooks express stateful multi-step procedures (upgrade contracts, rotate a runtime
group, run tests, checkpoint). They are the only place that sequences operations. They
must not import engine internals — only the `RolloutRunContext` / `Stack` interface.

Three built-in runbooks are planned: `rollout`, `drift`, `db-revert`. These are STUBS in
`stack/runbooks/`.

---

## sc-deploy → sc-addresses address wiring (REAL)

The `contracts` chart runs an `sc-deploy` Kubernetes Job. After deploying all contracts,
the Job writes every deployed address into the `sc-addresses` ConfigMap via
`kubectl patch configmap` (implemented in
`charts/contracts/templates/sc-deploy-config.yaml`).

Downstream charts (`coprocessor`, `kms-connector`) then consume those addresses via
`configMapKeyRef`. For example in
`charts/coprocessor/templates/coprocessor-host-listener-deployment.yaml` (lines 172–190):
```yaml
- name: ACL_CONTRACT_ADDRESS
  valueFrom:
    configMapKeyRef:
      name: eth-sc-addresses
      key: acl.address
- name: KMS_GENERATION_ADDRESS
  valueFrom:
    configMapKeyRef:
      name: eth-sc-addresses
      key: kms_generation.address
```

The Stack API's `snapshotContracts` method reads these ConfigMaps to record the deployed
state. The CLI does not need to parse contract output — it reads from the ConfigMap.

---

## Multi-coprocessor indexing (REAL)

When deploying more than one coprocessor, the contracts chart reads
`NUM_COPROCESSORS` and a set of indexed env vars (REAL keys from
`charts/contracts/values.yaml` lines 127–134):

```yaml
- name: NUM_COPROCESSORS
  value: 5
- name: COPROCESSOR_THRESHOLD
  value: 3
# Coprocessor signer addresses, indexed from 0 up to NUM_COPROCESSORS - 1
- name: COPROCESSOR_SIGNER_ADDRESS_x
  value: ""
```

A scenario values file overrides `NUM_COPROCESSORS` and each
`COPROCESSOR_SIGNER_ADDRESS_0` … `COPROCESSOR_SIGNER_ADDRESS_{N-1}`. The Stack API does
not need to know about this indexing; it passes the values file through to Helm.

---

## File map

Each file in this directory demonstrates one specific idea. Files marked STUB contain
skeleton code only and do not run.

| File | What it demonstrates | Status |
|---|---|---|
| `lib/stack.ts` | The full Stack API type (RolloutRunContext extension + chaos primitives) | STUB |
| `cli/up.ts` | CLI `up` command: load manifest, `helm upgrade --install` each chart in order | STUB |
| `cli/rollout.ts` | CLI `rollout run <script>` command: load runbook, create context, execute | STUB |
| `cli/state.ts` | CLI `state` command: read cluster state, pretty-print | STUB |
| `values/manifest.yaml` | Image tag manifest (Contract 1) — keys taken from real charts | STUB |
| `values/scenario-two-of-three.yaml` | Scenario overlay: endpoint overrides, NUM_COPROCESSORS=3 | STUB |
| `runbooks/rollout.ts` | Built-in rollout runbook skeleton | STUB |
| `runbooks/drift.ts` | Built-in drift-check runbook skeleton | STUB |
| `runbooks/db-revert.ts` | Built-in db-revert runbook skeleton | STUB |
| `../.github/workflows/acceptance.yml` | Acceptance CI: matrix (case x level), record/replay golden masters | STUB |

Reference files that are REAL (read-only, do not modify):

| File | What to learn from it |
|---|---|
| `test-suite/fhevm/src/commands/rollout-run.ts` | The real RolloutRunContext interface and runbook loader |
| `test-suite/fhevm/rollouts/v0.12-to-v0.13/run.ts` | A real runbook: multi-phase upgrade with version locks |
| `test-suite/fhevm/rollouts/v0.12-to-v0.13/versions.ts` | A real manifest: image tag record per component |
| `charts/contracts/values.yaml` | REAL sc-deploy keys, NUM_COPROCESSORS, indexed env vars |
| `charts/coprocessor/values.yaml` | REAL commonConfig, hostListenerShared, dbMigration, gwListener |
| `charts/kms-connector/values.yaml` | REAL kmsCoreEndpoints, gatewayContractAddresses, commonConfig |
| `charts/anvil-node/values.yaml` | REAL anvil image and network keys |
| `charts/listener/values.yaml` | REAL listener-core image and commonConfig |
| `charts/contracts/templates/sc-deploy-config.yaml` | REAL ConfigMap creation/patching logic |
| `charts/coprocessor/templates/coprocessor-host-listener-deployment.yaml` | REAL configMapKeyRef wiring for ACL_CONTRACT_ADDRESS, KMS_GENERATION_ADDRESS |

---

## What is REAL vs STUB

### REAL and grounded

- The `RolloutRunContext` interface — defined in `rollout-run.ts`, used by the v0.12-to-v0.13 runbook.
- All values keys cited in this document — verified against the actual chart values.yaml files.
- The sc-deploy Job → sc-addresses ConfigMap → configMapKeyRef address wiring — verified in templates.
- `NUM_COPROCESSORS` + `COPROCESSOR_SIGNER_ADDRESS_x` indexing — in `charts/contracts/values.yaml`.
- `kmsCoreEndpoints` — in `charts/kms-connector/values.yaml` line 201.
- `hostChainWsUrl`, `gatewayUrl`, `commonConfig` — in `charts/coprocessor/values.yaml`.
- kms-core is external (not a chart) — confirmed: no `charts/kms-core` directory exists.
- `dbMigration` as a Job — confirmed in `charts/coprocessor/values.yaml`.
- `scUpgrade` path — confirmed in `charts/contracts/values.yaml` (`scUpgrade.enabled`, `upgradeCommands`).
- `readinessProbe` on components — confirmed in coprocessor, kms-connector, listener chart templates.

### STUB (not yet implemented)

- `lib/stack.ts` — interface exists here as a sketch; no implementation.
- `cli/*.ts` — command handlers are empty stubs.
- `values/manifest.yaml`, `values/scenario-*.yaml` — illustrative, not wired to any runner.
- `runbooks/*.ts` — empty skeletons; no engine calls.
- `../.github/workflows/acceptance.yml` — CI harness outline only; no jobs run.
- The kind cluster bootstrap (cluster creation, local registry, image loading) — not written.
- The Helm install ordering logic (which chart depends on which) — not written.
- Record/replay golden-master harness (L0–L3 acceptance levels) — not written.

---

## Acceptance levels (target, not yet implemented)

| Level | What runs | Requirement |
|---|---|---|
| L0 | `helm template` — normalized diff of rendered manifests | No cluster; runs locally and in CI |
| L1 | Boot one stack in kind; smoke readiness probes | One isolated CI runner |
| L2 | Boot one stack; run the standard test profile | One isolated CI runner |
| L3 | Boot one stack; execute a full rollout runbook | One isolated CI runner |

L0 can be run offline today using the real charts:

```sh
# Lint
helm lint charts/contracts
helm lint charts/coprocessor
helm lint charts/kms-connector
helm lint charts/anvil-node
helm lint charts/listener

# Render and inspect
helm template my-stack charts/contracts  -f stack/values/manifest.yaml
helm template my-stack charts/coprocessor -f stack/values/manifest.yaml \
  -f stack/values/scenario-two-of-three.yaml
```

No cluster is required for these commands. They validate that value overrides are
syntactically correct and that the templates render without errors.
