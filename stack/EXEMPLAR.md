# fhevm-cli redesign — exemplar findings

This document records the chart-on-kind work — from the offline render (§1–§2) through a
**full live boot to a finalized FHE keygen (§0 below, 2026-06-17 — the authoritative
current state)**. It maps every scaffolded file to the design idea it demonstrates, and
captures the **proven boot recipe + invariants** that `stack/lib` must encode.

---

## 0. Full stack boot through FHE keygen — PROVEN on kind (2026-06-17)

A hand-driven boot took the stack **from an empty kind cluster to a finalized FHE
keygen** — the hardest, most load-bearing part of the protocol — validating the
charts-on-kind + staging-env approach end to end. Everything here was run live and
verified, not inferred. Cluster: `fhevm-p2`.

### Live state

| Layer | Status |
|---|---|
| anvil host (chainId 12345) + gateway (54321) + `host-node`/`gateway-node` alias svcs | Running, RPC verified |
| postgres `db` (+ `coprocessor`, `kms-connector` databases) | Running |
| minio (buckets `kms-public`/`ct64`/`ct128`, user `fhevm-access-key`) | Running |
| gateway + host contracts | deployed; **deterministic addresses match staging exactly** |
| `sc-addresses` ConfigMap | published |
| kms-core (centralized, `core-service:v0.13.20-0`, signing key on a PVC) | Running on :50051 |
| kms-connector (gw-listener / kms-worker / tx-sender + migration) | Running, polling host KMSGeneration |
| **FHE keygen** | **FINALIZED** — `activeKeyId` set; `PublicKey` 32 KiB + `ServerKey` 1.6 GiB + `CRS` 4.4 MiB in minio; realized `FHE_KEY_ID = 0400…0001` |
| coprocessor (9 svcs) → relayer → `erc20` + L2 golden | **remaining** |

This supersedes the "still open: full multi-chart boot" note in §0a below — that boot is
now done, through keygen.

### Proven boot recipe (the spec for `stack/lib`)

Versions (v0.13 train): `GATEWAY_VERSION`/`HOST_VERSION` = `v0.13.0-6`, `CORE_VERSION` =
`v0.13.20-0`, `COPROCESSOR_*`/`CONNECTOR_*`/`LISTENER_CORE_VERSION` = `v0.13.0-6`,
`RELAYER_SDK_VERSION` = `0.4.2`. Private ghcr images via a `registry-credentials` secret
(`read:packages` token).

1. kind cluster + `registry-credentials` secret.
2. Chains: 2 `anvil-node` releases. **host** chainId 12345, mnemonic `adapt mosquito move
   limb mobile illegal tree voyage juice mosquito burger raise father hope layer`, port
   8545. **gateway** chainId 54321, mnemonic `coyote sketch defense hover finger envelope
   celery urge panther venue verb cheese`, port 8546. Add alias Services
   `host-node`/`gateway-node` (the chart names its service `<release>-anvil-node`; the
   stack expects `host-node:8545` / `gateway-node:8546`). Anvil **requires** a mnemonic
   (empty → `anvil --mnemonic ""` exit 1).
3. postgres `db` (databases `coprocessor` + `kms-connector`) · minio (buckets + user +
   `/minio_secrets/{access_key,secret_key}` files for kms-core).
4. kms-core up: config = `templates/config/kms-core-modern.toml` (NOT the legacy
   `config/kms-core/config.toml` — its `storage_cache_size` is rejected by v0.13.20-0).
   Persist `/app/kms/core/service/keys` on a PVC so the signing key is stable.
5. **Discover** kms-core's signer = the address in its own log
   (`stored … ethereum address 0x…`). Do **not** trust minio `VerfAddress` (goes stale).
6. Deploy gateway contracts: `gateway-contracts:v0.13.0-6`, RPC=`gateway-node:8546`,
   env=`.env.gateway-sc` (deployer derived from the gateway "coyote sketch" mnemonic).
7. Deploy host contracts: `host-contracts:v0.13.0-6`, RPC=`host-node:8545`,
   env=`.env.host-sc`, **`KMS_SIGNER_ADDRESS_0` = discovered signer (step 5)**,
   `KMS_TX_SENDER_ADDRESS_0=0x31de9c8a…`. KMSGeneration is a **host** contract in v0.13.
   Write the deploy's `/app/addresses` to a shared volume (the trigger needs it).
8. Publish `sc-addresses` (deterministic from deployer+nonce — a fresh chain reproduces
   them: ACL `0x05fD9B…`, host KMSGeneration `0x3E0fBCcE…`).
9. **Fund** the connector tx-sender (`0x31de9c8a…`) on host + gateway.
10. kms-connector (4 svcs): `KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS` = the host
    KMSGeneration `0x3E0fBCcE…`, watched over `KMS_CONNECTOR_ETHEREUM_URL=host-node:8545`
    (NOT the staging gateway value `0x3576…`). Wait for the gw-listener log
    `Started KMSGeneration polling from block N`.
11. Trigger keygen+crsgen **host-side**: `task:triggerKeygen --params-type 0
    --use-internal-proxy-address true` then `task:triggerCrsgen --params-type 0
    --max-bit-length 2048 --use-internal-proxy-address true`, with the deploy's
    `/app/addresses` mounted (else `HH404 FHEVMHostAddresses.sol`).
12. Wait `activeKeyId ≠ 0` on the host KMSGeneration **and** `PublicKey`/`ServerKey`/`CRS`
    in minio → the realized `FHE_KEY_ID` is discovered here; feed it forward.
13. coprocessor (db-migration + 9 svcs): `AWS_ENDPOINT_URL=http://minio:9000` (staging
    uses a compose IP), expand `${VAR}` in `DATABASE_URL`, `FHE_KEY_ID` = realized id.
14. relayer (`.env.relayer`).
15. `erc20` e2e from test-suite → PASS → record the L2 golden.

### Five invariants (each hit live — encode as guardrails)

1. **KMSGeneration is host-side in v0.13** (gateway KMSGeneration is read-only); trigger
   on host, connector watches host over `ETHEREUM_URL`.
2. **Discover the live signer; never hardcode.** `kms-gen-keys` won't overwrite an
   existing minio object, so `VerfAddress` goes stale across pod restarts. Staging `.env`
   constants (signer, KMSGen addr, FHE key id) are **seeds overwritten at runtime** — the
   CLI's `generateRuntime` placeholder-overwrite pattern.
3. **Order: deploy → connector → trigger.** Never reset a chain under a running
   connector — its DB `last_block_polled` desyncs from the fresh chain.
4. The keygen **tx-sender must be funded** on the host chain **and** its on-chain
   registered signer must equal kms-core's actual signer (else `KmsSignerDoesNotMatchTxSender`).
5. The connector must be **polling before** the trigger fires (no backfill); keygen is
   **two-phase** (prep → keygen) — the prep response must land on-chain to emit the real
   `KeygenRequest`.

**How the CLI does steps 5–7 cleanly** (the pattern `stack/lib` must encode): the pipeline
order is `base → kms-signer → gateway-deploy → host-deploy` (`src/types.ts`);
`discoverKmsSigners` scrapes the signing-key handle from kms-core's logs and reads its
`VerfAddress` from minio (`src/flow/readiness.ts:270`); `generateRuntime` then injects
that address into the deploy env as `KMS_SIGNER_ADDRESS_x` (`src/generate/env.ts:150`). The
hand-boot above inverts this (deploy-then-discover), which is exactly the bug to avoid.

---

## 0a. Initial mechanics spike (2026-06-16) — superseded by §0; still corrects §1–§2

The offline render (§1–§2) *inferred* a registry blocker. A live run on a local kind
cluster shows that inference was wrong for local kind. What was actually run:

- **Cluster:** kind v0.32.0, Kubernetes v1.36.1, with the default `standard`
  StorageClass (rancher.io/local-path) present.
- **anvil-node:** `helm install` → pod `Running 1/1`, Service on 8545, **PVC bound to
  the default StorageClass**. Boots as-is, no overlay. Confirmed.
- **Private image pull:** created a `registry-credentials` secret from the local `gh`
  token (account Eikix, `read:packages`), then ran a pod on
  `ghcr.io/zama-ai/fhevm/host-contracts:v0.13.0` (~982 MB) → **pulled and Running 1/1 in
  ~50 s.** Private zama ghcr images pull straight into kind with our creds.
- **Coprocessor images are on ghcr too:** `tfhe-worker`, `host-listener`, `sns-worker`,
  `db-migration` all resolve on `ghcr.io/zama-ai/fhevm/coprocessor/*` (the same path the
  compose e2e uses).

**Correction.** The "private GHCR images need creds / can't pull / use `kind load`"
blocker in §1–§2 is wrong for local kind. With a `read:packages` token + a
`registry-credentials` secret, images pull directly — no `kind load`, no `hub.zama.org`
access. The only real registry nuance: the charts *default* the coprocessor/listener
images to `hub.zama.org` (the prod registry); repoint them to ghcr in a local overlay —
see `stack/values/kind-local.yaml`. `hub.zama.org` is reachable (HTTP 401 = up, needs
auth) but **not needed locally**.

**Still open (iterative, not a blocker):** a full multi-chart boot — apply
`kind-local.yaml`, add an in-cluster Postgres + minio, the two anvil chains, and drive
the `sc-deploy` Job → `sc-addresses` ConfigMap wiring to an e2e pass.

**Gate verdict: GREEN on the load-bearing mechanics (cluster + public-chart boot +
private-image pull all proven locally) → commit to direct-kind; compose stays only as
the documented fallback.**

---

## 1. Chart-on-kind spike: per-chart verdicts

### anvil-node — BOOTS AS-IS

`helm lint` clean (0 failures). `helm template` renders cleanly with all defaults.

No local overrides are required. All services are ClusterIP. The PVC (1 Gi RWO) has no
`storageClassName` — kind's default `standard` StorageClass satisfies it. The image
(`ghcr.io/foundry-rs/foundry:stable`) is public; no imagePullSecrets needed. The
commented-out nodeSelector/tolerations/affinity blocks reference AWS Karpenter but are
disabled and do not appear in rendered output.

Optional: set `network.mnemonic` to a known phrase for reproducible accounts. Not
required for boot.

**Kind verdict: boots without any overlay.**

---

### contracts — NEEDS LOCAL OVERRIDES

`helm lint` and `helm template` both succeed with defaults. The pod schedules on kind,
but the Job exits non-zero without real values for the application-level env vars (all
default to `""`).

Hard blockers on kind:

1. **imagePullSecrets hardcoded to `registry-credentials`** in both `sc-deploy-job.yaml`
   and `sc-deploy-statefulset.yaml` — not configurable via values. The image
   (`ghcr.io/zama-ai/fhevm/host-contracts`) is private. Requires either:
   (a) `kubectl create secret docker-registry registry-credentials ...` pre-created, or
   (b) image pre-loaded with `kind load docker-image`.

2. **`storageClassName: ""`** on the PVC. An empty string may be treated as "no
   StorageClass" by some Kubernetes versions (bypassing the default) rather than "use
   cluster default". Safe fix: set `persistence.volumeClaim.storageClassName: standard`
   in a local overlay.

Runtime blockers (pod starts, Job fails without these):
- `DEPLOYER_PRIVATE_KEY`, `PAUSER_SET_ADDRESS`, `PAUSER_SET_CONTRACT_ADDRESS`
- `PAUSER_ADDRESS_0..N`, `KMS_TX_SENDER_ADDRESS_0..N`, `KMS_SIGNER_ADDRESS_0..N`
- `KMS_NODE_IP_0..N`, `KMS_NODE_STORAGE_URL_0..N`
- `CHAIN_ID_GATEWAY`, `DECRYPTION_ADDRESS`, `INPUT_VERIFICATION_ADDRESS`
- `COPROCESSOR_SIGNER_ADDRESS_0..N`

**Kind verdict: boots with two overlay fixes (storageClassName + imagePullSecrets
pre-step); contracts deploy only once application env vars are also supplied.**

---

### coprocessor — NEEDS LOCAL OVERRIDES

`helm lint` and `helm template` both succeed. Only Job + Service ClusterIP + Deployment
— no LoadBalancer, no PVC, no Ingress.

Hard blockers on kind:

1. **imagePullSecrets hardcoded to `registry-credentials`** in every pod spec (not
   behind a values flag). Image source: `hub.zama.org/zama-protocol/zama-ai/fhevm/coprocessor/*`.
   Same pre-step required as for `contracts`.

2. **`commonConfig.databaseUrl` defaults to `""`** — `DATABASE_URL` is only injected
   when non-empty. Without it every component binary panics on startup. Must set in a
   local overlay: `commonConfig.databaseUrl: "postgresql://user:pass@postgres-svc/coprocessor"`.

Optional/disabled by default (not kind blockers):
- `snsWorker.enabled: false` — requires AWS S3 and credentials. Leave disabled.
- `txSender.wallet.awsKms.enabled: false` — requires AWS KMS. Leave disabled.
- `databaseAuthMode: password` (default) — the IAM path requires RDS + IRSA. Default is
  fine on kind.

**Kind verdict: boots with one overlay key (`commonConfig.databaseUrl`) plus the
registry-credentials pre-step. All optional AWS components are off by default.**

---

### kms-connector — NEEDS LOCAL OVERRIDES

`helm lint` and `helm template` both succeed. Renders 3 Deployments + 3 ClusterIP
Services + 1 db-migration Job. No PVC, no LoadBalancer, no Ingress.

Hard blockers on kind:

1. **imagePullSecrets hardcoded to `registry-credentials`** in all 4 pod specs (not
   configurable). Images on `ghcr.io/zama-ai` (private). Same pre-step required.

2. **`commonConfig.databaseUrl` DSN wiring broken**: the Deployments resolve
   `DATABASE_URL` as `postgresql://$(DATABASE_ENDPOINT)/connector` but `DATABASE_ENDPOINT`
   is not set by the chart itself — it must be injected via `commonConfig.env`. Without
   it all three apps start with a broken connection string. Fix in local overlay:
   ```yaml
   commonConfig:
     env:
       - name: DATABASE_ENDPOINT
         value: "postgres:5432"
   ```

3. **`kmsConnectorTxSender.wallet.secret`** (name: `kms-connector-tx-sender`, key:
   `kms-wallet`) is a `secretKeyRef` that must pre-exist in the namespace. Pod will fail
   with `CreateContainerConfigError` until the Secret exists.

4. **OTEL endpoint active by default** (`kmsConnectorTxSender.tracing.enabled: true`):
   tx-sender emits `OTEL_EXPORTER_OTLP_ENDPOINT` pointing at
   `http://otel-deployment-opentelemetry-collector.observability.svc.cluster.local:4317`
   which does not exist on kind. Set `kmsConnectorTxSender.tracing.enabled: false` in
   the overlay to suppress.

**Kind verdict: boots with three overlay keys + two kubectl pre-steps (registry secret +
wallet secret).**

Minimum local overlay:
```yaml
kmsConnectorTxSender:
  tracing:
    enabled: false
commonConfig:
  env:
    - name: DATABASE_ENDPOINT
      value: "postgres:5432"
```
Pre-steps:
```sh
kubectl create secret docker-registry registry-credentials \
  --docker-server=ghcr.io --docker-username=... --docker-password=...
kubectl create secret generic kms-connector-tx-sender \
  --from-literal=kms-wallet=<private-key-hex>
```

---

### listener — NEEDS LOCAL OVERRIDES

`helm lint` clean. `helm template` with no values renders only a ServiceAccount — every
other resource (Deployment, ConfigMap, Service, Secret) is gated on
`if .Values.listeners` or `if not .Values.externalSecret.enabled`.

Hard blockers on kind:

1. **`listeners` defaults to `[]`**: no Deployment, ConfigMap, or Service is emitted
   until at least one listener entry with `name` and `blockchain.chain_id` is provided.

2. **`externalSecret.enabled: true` by default**: assumes the External Secrets Operator
   is installed and has already provisioned `listener-secrets`. A vanilla kind cluster
   has no ESO. Must set `externalSecret.enabled: false` and supply `fallbackSecret.data`.

3. **Image `hub.zama.org/ghcr/zama-ai/fhevm/listener/listener-core`** is private (same
   registry pre-step as all other charts). The eRPC sidecar (`ghcr.io/erpc/erpc`) is
   public and does not need auth.

4. **`database-credentials` Secret**: the default `env` block references
   `secretKeyRef: {name: database-credentials, key: database-url}` — this Secret is
   never created by the chart and is not overridden by `fallbackSecret`. Must be
   pre-created separately.

**Kind verdict: boots with a 3-key overlay (one `listeners[]` entry,
`externalSecret.enabled: false` + `fallbackSecret.data`, registry pre-step) plus one
additional kubectl pre-step for `database-credentials`.**

---

## 2. Overall kind verdict

The full fhevm stack can be hosted on kind. There is no chart-level blocker that would
require structural changes to the Helm templates. Every blocking item is a configuration
gap (missing values overlay, imagePullSecrets pre-step, or a missing pre-existing Secret)
that is addressed at deploy time.

Summary of required pre-steps (shared across all charts):
```sh
# One-time per kind cluster namespace
kubectl create secret docker-registry registry-credentials \
  --docker-server=ghcr.io --docker-username=<GHCR_USER> --docker-password=<GHCR_TOKEN>
kubectl create secret docker-registry registry-credentials-hub \
  --docker-server=hub.zama.org --docker-username=<HUB_USER> --docker-password=<HUB_TOKEN>
kubectl create secret generic kms-connector-tx-sender \
  --from-literal=kms-wallet=<private-key-hex>
kubectl create secret generic database-credentials \
  --from-literal=database-url=postgresql://user:pass@postgres-svc/listener
```

The coprocessor, kms-connector, and listener charts also need an in-cluster PostgreSQL
instance — not included in the current chart set but trivially provided by a
`bitnami/postgresql` Helm release or a simple Deployment.

---

## 3. Scaffolded files

All files below live under `stack/` and
`.github/workflows/acceptance.yml` in the worktree. Files marked STUB are skeleton code
only — they do not run and contain no implementation.

| File | Idea demonstrated | Status |
|---|---|---|
| `stack/lib/stack.ts` | Full Stack API interface: `RolloutRunContext` promoted to typed kind+Helm surface; chaos/read-state primitives (`exec`, `sql`, `stop`, `start`, `restart`, `logs`, `waitForLog`, `chain`, `until`) | STUB |
| `stack/lib/helm.ts` | Thin wrapper over `helm upgrade --install --wait` with values-file merging and dry-run mode | STUB |
| `stack/lib/kubectl.ts` | Thin wrapper over `kubectl` for exec, scale, rollout-status, configmap reads | STUB |
| `stack/cli/main.ts` | CLI entry point: command routing (`up`, `down`, `rollout`, `state`, `test`) | STUB |
| `stack/cli/up.ts` | `up` command: load MANIFEST, call `helm upgrade --install` per chart in dependency order | STUB |
| `stack/cli/runbook.ts` | `rollout run <script>` command: load runbook module, construct Stack context, execute | STUB |
| `stack/cli/test.ts` | `test` command: invoke Stack API `test()` with a named profile | STUB |
| `stack/values/default.yaml` | Default values overlay illustrating the MANIFEST contract (Contract 1) — image.tag keys taken verbatim from real charts | STUB |
| `stack/values/two-of-three.yaml` | Scenario overlay: `NUM_COPROCESSORS=3`, coprocessor signer addresses, `kmsCoreEndpoints` indirection (Contract 2) | STUB |
| `stack/runbooks/drift.ts` | Built-in drift-check runbook: `up → state() → diff` | STUB |
| `stack/runbooks/v0.12-to-v0.13.ts` | Built-in rollout runbook: `up → upgrade(group) → snapshotContracts → test` | STUB |
| `stack/fhevm` | Symlink / reference anchor pointing at `test-suite/fhevm/` for IDE navigation | REAL |
| `stack/README.md` | Full design document: two-layer architecture, three data contracts, real values-key references, sc-addresses wiring, multi-coprocessor indexing, acceptance levels | REAL (grounded) |
| `.github/workflows/acceptance.yml` | Acceptance CI harness: `record-goldens` job (run once on the old CLI) + `acceptance` matrix (case × level, L0–L3), golden-artifact download, per-level diff steps, always-teardown | STUB (harness shape is real; all step bodies are PLACEHOLDERs) |

---

## 4. What is REAL vs STUB

### REAL and grounded (verified against repo source)

- `RolloutRunContext` interface — defined in
  `test-suite/fhevm/src/commands/rollout-run.ts` lines 53–65; used by the
  `v0.12-to-v0.13` runbook. The `Stack` interface in `lib/stack.ts` extends it.
- All values keys cited in `README.md` and the scaffolded values files — verified
  against `charts/*/values.yaml`.
- sc-deploy Job → `sc-addresses` ConfigMap → `configMapKeyRef` address wiring —
  verified in `charts/contracts/templates/sc-deploy-config.yaml` and
  `charts/coprocessor/templates/coprocessor-host-listener-deployment.yaml` lines
  172–190.
- `NUM_COPROCESSORS` + `COPROCESSOR_SIGNER_ADDRESS_x` indexing — in
  `charts/contracts/values.yaml` lines 127–134.
- `kmsCoreEndpoints` — `charts/kms-connector/values.yaml` line 201.
- `hostChainWsUrl`, `gatewayUrl`, `commonConfig` — `charts/coprocessor/values.yaml`
  lines 62–71.
- kms-core is external (no `charts/kms-core` directory exists).
- `dbMigration` as a Kubernetes Job — confirmed in coprocessor chart templates.
- `scUpgrade.enabled` / `upgradeCommands` path — `charts/contracts/values.yaml`.
- `readinessProbe` on all deployable components — confirmed in coprocessor,
  kms-connector, listener chart templates.
- Helm lint + template results for all five charts — run offline as part of this spike.

### STUB (not yet implemented)

- `lib/stack.ts` — interface only; no engine implementation.
- `lib/helm.ts`, `lib/kubectl.ts` — empty skeletons.
- `cli/*.ts` — command handlers are empty stubs.
- `values/*.yaml` — illustrative; not wired to any runner.
- `runbooks/*.ts` — empty skeletons; no Stack API calls.
- `.github/workflows/acceptance.yml` step bodies — all are `echo "PLACEHOLDER"`.
- Golden-master record/replay harness (L0–L3) — L0 runs locally; L1–L3 not written. No
  L2 `erc20` golden recorded yet (the missing behavioral oracle).

### PROVEN live but NOT yet encoded (the gap `stack/lib` must close)

The §0 boot did all of these **by hand on `fhevm-p2`**; none is yet in `lib`:

- kind cluster bootstrap, `registry-credentials`, in-cluster Postgres + minio — done live.
- Boot dependency ordering (the 15-step recipe in §0) — proven, not codified.
- KMS-signer discovery + the `generateRuntime` placeholder-overwrite pattern — understood
  (§0 invariant 2), not implemented.
- FHE keygen DKG (kms-core + connector + host-side trigger) — finalized live, not codified.

---

## 5. Recommended next steps

### Step 1 — Finish the live boot to `erc20` (DONE through FHE keygen — see §0)

The full kind boot is no longer hypothetical: §0 records it run live, from an empty
cluster through a **finalized FHE keygen** (the recipe + invariants are in §0). What
remains on the live cluster (`fhevm-p2`):

1. **coprocessor** (db-migration + 9 svcs) against the generated keys —
   `AWS_ENDPOINT_URL=http://minio:9000`, expand `${VAR}` in `DATABASE_URL`,
   `FHE_KEY_ID` = the realized id from §0 step 12.
2. **relayer**.
3. **`erc20` e2e** → **record the first L2 golden** (the behavioral oracle the whole
   acceptance plan depends on).

### Step 2 — Extract the Stack API implementation in place

`lib/stack.ts` is currently an interface. Implement it against the real charts —
**encoding the §0 boot recipe (the 15 ordered phases) and its five invariants** (esp.
kms-signer discovery, the deploy→connector→trigger ordering, and the placeholder-overwrite
convention) — without introducing new abstractions:

- `up()`: shell out to `helm upgrade --install --wait` for each chart in order.
- `discovery()`: `kubectl get configmap sc-addresses -o json` → parse
  `ContractAddresses`.
- `refreshDiscovery()`: re-read `sc-addresses` ConfigMap and patch running Deployments.
- `exec()`, `sql()`, `stop()`, `start()`, `restart()`, `logs()`: thin wrappers over
  `kubectl exec/scale/rollout/logs`.
- `test()`: delegate to the existing test-suite runner (avoid reimplementing).

Keep the implementation in `stack/lib/` and make the existing
`test-suite/fhevm/src/commands/rollout-run.ts` entry point call through the new Stack
API rather than the Docker Compose layer directly. This gives a live integration point
before the Docker Compose layer is removed.

### Step 3 — Wire the acceptance.yml harness

Replace all `echo "PLACEHOLDER"` step bodies in
`.github/workflows/acceptance.yml` with real commands:

1. **L0 first**: `helm template` + normalize script. No cluster needed; can run in CI
   today once the CLI has a `template` subcommand that reads the MANIFEST.
2. Record goldens once from the current CLI using the `record-goldens` job
   (`workflow_dispatch` with `record_goldens: true`).
3. Add the workflow as a required check on PRs that touch `charts/*` or
   `test-suite/fhevm/src/`.
4. Promote to L1 once the kind boot is verified on a devbox. L2 and L3 follow naturally
   once the test and runbook paths are wired.

### Step 4 — Remove Docker Compose (final cutover)

Once L2 acceptance passes consistently, delete the Docker Compose orchestration layer
(`test-suite/fhevm/src/commands/rollout-run.ts` and its Docker-specific dependencies)
and the `acceptance.yml` migration harness. The Stack API backed by kind+Helm becomes
the sole engine.
