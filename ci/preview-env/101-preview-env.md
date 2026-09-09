# Preview env 101 — how to spin up an e2e preview

A **preview env** is a full, throwaway fhevm stack (anvil host+gateway chains, a
real threshold+enclave KMS, contracts, coprocessor, kms-connector, relayer and
the e2e test-suite) deployed to its own namespace on the `zws-dev` cluster. Use
it to exercise a PR end-to-end against real charts and a real KMS.

This page is the **usage** guide. For what's inside and why, see
[`README.md`](./README.md); the workflow itself is
[`preview-env-deploy.yml`](../../.github/workflows/preview-env-deploy.yml).

There are two ways to launch: **from a PR** (add a label) or **manually**
(`workflow_dispatch`, for full control over versions/topology).

## Prerequisites

- Membership of the `coprocessor-dev-access` or `kms-dev-access` group (either
  grants namespace admin) and Tailscale access to the `zws-dev` cluster —
  needed to connect afterwards.
- Write access to the PR (to add labels) or to run workflows (for a manual run).

---

## Option A — from a PR (labels)

Add one (or more) of these labels to the PR. The env deploys automatically; each
new push re-deploys it fresh (an in-flight run is cancelled).

| Label | What it does |
| --- | --- |
| `preview-env-e2e` | Deploy the stack, **building fresh images from the PR branch** first (only changed components; the rest resolve to the base commit's images). In-repo charts (`charts/*`) install straight from the checkout. |
| `preview-env-e2e-tests` | Same, **and** auto-run the e2e test DAG, posting a pass/fail report back to the PR. Deploys the env on its own. |
| `preview-env-blue-green` | Deploy [RFC-021](https://github.com/zama-ai/tech-spec/pull/443) BCS+GCS on each party (forces `nb_coprocessor=2`) **on shared `blockchain-dev`** (not Anvil). Enough on its own. Combined with `preview-env-e2e-tests`: propose after the relayer is up, hold `consensus-detector` so the first e2e stays on blue (`DryRunStarted`, assert GCS `computations > 0`), then enable the detector, wait for `versioning=v0.15`, and run e2e again on green. Incompatible with `deploy_polygon`. |

On PRs, images are **always** built fresh from the branch - there is no
pinned-only PR path (use a `workflow_dispatch` run with `build_images=false`
for that).

- **Namespace:** `fhevm-ci-<pr-author>-<pr-number>`.
- **Results:** a `:rocket:` comment on success; with `preview-env-e2e-tests`, a
  per-test SDK-matrix report comment (see [See test results](#see-test-results)).
- **Teardown:** automatic when the PR is **closed**, or when you **remove** the
  preview label(s) (handled by
  [`preview-env-destroy.yml`](../../.github/workflows/preview-env-destroy.yml)).

---

## Option B — manual run (`workflow_dispatch`)

GitHub → **Actions** → **preview-env-deploy** → **Run workflow**, pick the branch
(via "Use workflow from"), set inputs, run. Use this to change versions or
topology, or to deploy without a PR.

Key inputs (all have sensible defaults — you rarely set more than a couple):

**Control**
- `build_images` — build fresh images from the picked branch (`true`) or build
  nothing (`false`).
- `build_test_suite_only` — when building, build **only** the e2e test-suite
  image (fast test-suite iteration); every other image resolves to the base
  commit's.
- `automated_tests` — auto-run the e2e DAG and write the report to the run
  summary.
- `observability` — also deploy an in-namespace Prometheus + Grafana + Jaeger
  stack and switch on OTLP tracing in components supporting it (off by
  default; see [Observe your environment](#observe-your-environment)).
**Topology**
- `nb_kms_core` — number of KMS parties (default `4`).
- `nb_coprocessor` — number of independent coprocessor **identities** (default
  `1`). `2` is two-party consensus (one fleet each), **not** blue-green.
  `3`/`5` stay N-party only. See `README.md`.
- `enable_blue_green` — RFC-021 BCS+GCS on each identity (default `false`).
  Forces `nb_coprocessor=2` when N=1. The `preview-env-blue-green` PR label
  is the other gate. Incompatible with `deploy_polygon` (so also with
  `chain_mode=testnets`).
- `deploy_polygon` — also add a second Polygon Amoy (`80002`) host chain (default
  `false`). Fresh local anvil, reuses the ETH KMS key; roughly doubles the
  host-side stack. With `automated_tests` on it also runs a Polygon e2e suite.
  See the multichain section in `README.md`. Incompatible with
  `chain_mode=blockchain-dev`; implied by `chain_mode=testnets` (real Amoy).
- `chain_mode` — `anvil` (default), `blockchain-dev`, or `testnets`; see the
  chain-modes section in `README.md`.
  - `blockchain-dev`: skip per-namespace Anvil and connect to the shared
    `blockchain-dev` Geth (host chain id `1337`) + Nitro (gateway `412346`).
    Unique mnemonic per run, wallets funded from the in-cluster faucets. The
    `preview-env-blue-green` PR label forces this on (plain `preview-env-e2e`
    labels stay on Anvil). Not with `deploy_polygon`.
  - `testnets`: public **Sepolia** (`11155111`) + **Polygon Amoy** (`80002`) as the
    two host chains, the `blockchain-dev` Nitro as gateway. RPC URLs (Secret `rpc`) and
    faucet keys (Secrets `eth-faucet`, `polygon-faucet`) come from AWS Secrets Manager via
    the gitops `sync-secrets` chart — no GitHub secrets. Keys are `zws-dev/ethereum-faucet`
    and `zws-dev/polygon-faucet` (`private-key`), the same AWS secrets gitops uses for the
    zws-dev faucets. Slow (12 s blocks) and it spends real testnet gas.
  Both external modes still deploy **this preview's own contracts**, which remain
  on the shared chains after teardown.

**Versions** — one optional `overrides` JSON object (empty / `{}` = resolve as
today). Allowed keys are listed in
[`scripts/parse-overrides.cjs`](./scripts/parse-overrides.cjs). Unknown keys
fail the run.

| Kind | Override keys | Default on PR / empty dispatch |
| --- | --- | --- |
| **fhevm images** | `coprocessor_version`, `kms_connector_version`, `test_suite_version`, … | Resolve from the change-detection base commit (built components use the PR/dispatch SHA). |
| **In-repo charts** | `coprocessor_chart_version`, `contracts_chart_version`, … | Install `charts/<name>` from the workflow checkout. |
| **External pins** | `common_chart_version`, `kms_core_version`, `kms_repo_ref`, `redis_chart_version`, … | Always the pins in `parse-overrides.cjs` (`ALWAYS_DEFAULTS`). **Not** resolved from the fhevm base commit. |
| **Relayer SDK** | `relayer_sdk_version` | PR: empty → `@fhevm/sdk` only. Dispatch: `0.4.4` when omitted; set `""` in `overrides` to skip the relayer-sdk suite. |

**Dedicated KMS (`zama-ai/kms`)** — never built by this workflow. Party count
is **`nb_kms_core`** (`4` or `13`), not an override. Version is two override
keys (keep them aligned to the same kms release):

| Key | Becomes | Role |
| --- | --- | --- |
| `kms_core_version` | `KMS_CORE_TAG` | GHCR tag for `core-service-enclave` → `deploy.sh --tag`. CI reads PCR labels from this image before install. |
| `kms_repo_ref` | `KMS_REPO_REF` | Git ref sparse-checked out of `zama-ai/kms` (`deploy.sh`, charts, threshold wiring). |

Current defaults (also in `parse-overrides.cjs`): `kms_core_version=d27c3b5`,
`kms_repo_ref=35edfa2f0656ee266e3299a004a83ac7d4fe2418`.

**kms-connector is not kms-core.** `kms_connector_version` /
`kms_connector_chart_version` are fhevm-owned (same resolve/build rules as
coprocessor). Changing KMS version does not change kms-connector unless you
override those too.

**PR labels cannot override KMS.** `overrides` is empty on `pull_request`, so
every labeled preview uses the `ALWAYS_DEFAULTS` pair above. To test a kms
build: `workflow_dispatch` or the CLI with `--set`, or bump the defaults in
`parse-overrides.cjs` for everyone.

Dispatch examples:

```bash
# Test a kms enclave + matching deploy scripts
ci/preview-env/preview-env launch --ref <branch> --tests \
  --set kms_core_version=<tag> \
  --set kms_repo_ref=<kms-commit-sha>

# 13-party threshold KMS
ci/preview-env/preview-env launch --ref <branch> --kms-parties 13 --tests
```

Or `overrides` in the Actions form:

```json
{
  "kms_core_version": "abc1234",
  "kms_repo_ref": "35edfa2f0656ee266e3299a004a83ac7d4fe2418"
}
```

The enclave tag must exist on GHCR with `zama.kms.eif_pcr{0,1,2}` labels.
An old `kms_repo_ref` may lack features the workflow expects (e.g.
`--tracing-endpoint` when `observability=true`).

- **Namespace:** `fhevm-ci-<actor>-<run-id-base36>` (dispatch) or
  `fhevm-ci-<pr-author>-<pr-number>` (PR). Actor is truncated if needed so
  the whole name stays under 28 chars.
- **Results:** run summary (deployment plan + e2e report if `automated_tests`).
- **Teardown:** **manual** — a dispatch env is not tied to a PR, so nothing
  destroys it automatically. Run **preview-env-destroy** with the namespace (see
  [Destroy an environment](#destroy-an-environment)), or re-run to reuse it.

### Launch from the CLI

[`preview-env`](./preview-env) wraps `gh workflow run`. It does **not**
helm-install; Actions stays the write path. `--ref` must already be on origin.

```bash
ci/preview-env/preview-env launch --ref <your-branch> --tests
ci/preview-env/preview-env launch --ref <your-branch> --blockchain-dev
ci/preview-env/preview-env launch --ref <your-branch> --testnets --tests
ci/preview-env/preview-env launch --ref <your-branch> --blue-green --blockchain-dev --tests
ci/preview-env/preview-env launch --ref <your-branch> --set coprocessor_version=abc1234
ci/preview-env/preview-env launch --ref <your-branch> --tests \
  --set kms_core_version=d27c3b5 --set kms_repo_ref=35edfa2f0656ee266e3299a004a83ac7d4fe2418
```

`--blue-green` sends `enable_blue_green=true` and `nb_coprocessor=2`.
`--parties 2` without `--blue-green` is two-party consensus only.

`launch` polls for the new Actions run and prints its id. Stream progress with
`--watch`, or inspect later:

```bash
ci/preview-env/preview-env launch --ref <your-branch> --tests --watch
ci/preview-env/preview-env status --ref <your-branch>     # latest deploy on branch
ci/preview-env/preview-env status <run-id>                # or a …/actions/runs/<id> URL
ci/preview-env/preview-env watch <run-id>
ci/preview-env/preview-env namespace --run-id <run-id>
```

---

## Connect to your environment

```bash
tailscale configure kubeconfig tailscale-operator-zws-dev.diplodocus-boa.ts.net
kubectl get pods -n <namespace>          # e.g. fhevm-ci-alice-1234
```

## Observe your environment

Deploy with the `observability` dispatch input set to `true` (off by default,
manual runs only for now) to get an in-namespace **Prometheus + Grafana +
Jaeger** stack: Prometheus auto-scrapes every instrumented service in the
namespace, Jaeger collects OTLP traces, and Grafana is the UI over both.
Details and design notes: [`README.md`](./README.md#observability-opt-in-observability-dispatch-input).

With Tailscale up (same prerequisites as connecting):

```bash
kubectl port-forward -n <namespace> svc/grafana 3000:3000     # http://localhost:3000
kubectl port-forward -n <namespace> svc/prometheus 9090:9090  # http://localhost:9090 (raw PromQL)
kubectl port-forward -n <namespace> svc/jaeger 16686:16686    # http://localhost:16686 (Jaeger UI)
```

## See test results

- **With auto-tests** (`preview-env-e2e-tests` label or `automated_tests=true`):
  the workflow runs the e2e DAG for both `@fhevm/sdk` and `@zama-fhe/relayer-sdk`
  and posts a per-test pass/fail table to the PR comment / run summary.
  Combined with `preview-env-blue-green`, that DAG runs **twice**: once during
  `DryRunStarted` (BCS live; CI asserts each party's `"gcs-<release>".computations`
  is non-empty) and once after cutover (`versioning=v0.15`, GCS live).
- **Without:** the stack is deployed with an idle test-suite Job — run tests
  yourself against the namespace, or re-label with `preview-env-e2e-tests`.

## Destroy an environment

Teardown means: `helm uninstall` every release in the namespace (so Crossplane
claims — coprocessor S3 buckets, KMS S3 vaults/enclave nodegroups — are released
and their AWS resources deprovisioned instead of leaking) then delete the
namespace. All handled by
[`preview-env-destroy.yml`](../../.github/workflows/preview-env-destroy.yml).

**PR env (automatic).** Nothing to do — the env is torn down when you:
- **close/merge** the PR, or
- **remove** the `preview-env-e2e` label (removing only `-tests` while
  `preview-env-e2e` stays keeps the env alive).

**Manual (dispatch) env.** A dispatch env has no PR to key off, so tear it down
by hand: GitHub → **Actions** → **preview-env-destroy** → **Run workflow**, and
set the `namespace` input to the **exact** namespace from your deploy run's
summary (e.g. `fhevm-ci-alice-987654`). It must start with `fhevm-ci-` (a guard
refuses anything else, so it can't nuke an unrelated namespace).

Or:

```bash
ci/preview-env/preview-env destroy fhevm-ci-<exact-name>
```

**Fallback.** If a run can't reach the cluster, do it yourself:

```bash
helm list -n <namespace> --short | xargs -r -L1 helm uninstall -n <namespace>
kubectl delete namespace <namespace>
```

> A namespace can sit in `Terminating` for a few minutes while Crossplane
> finalizers release the AWS resources — that's expected, not a stuck delete.

## Gotchas

- **PR labels always build your branch.** Both `preview-env-e2e` and
  `preview-env-e2e-tests` build fresh images from the PR HEAD (only changed
  components; the rest resolve from your base commit). To deploy your base
  commit's images only, use a `workflow_dispatch` run with `build_images=false`.
- **Chart changes deploy directly.** In-repo charts (`charts/*`) install straight
  from your branch's checkout — no publish, no version bump needed.
- **There are no auto-pins for fhevm's own images** (coprocessor, kms-connector,
  contracts, …). Charts come from your checkout; images resolve from your base
  commit unless built this run. Check the run summary **Images** table for
  `built`, `base-sha`, or `dispatch-override`. **Exception:** dedicated **kms-core** is always an
  external pin (`kms_core_version` + `kms_repo_ref` in `parse-overrides.cjs`) —
  PR labels cannot change it without editing that file or using dispatch.
- **Unresolvable ⇒ the run fails.** If GHCR has pruned the base commit's tags and
  nothing turns up within 50 commits, `resolve-tags` fails instead of quietly
  deploying something older. Rebase onto a newer base commit, or pass an explicit
  `*_version` key in the `overrides` JSON.
- **Stacked PRs resolve images from `main`.** Only `main`/`release/*` commits
  publish images, so a PR based on another feature branch resolves them from its
  merge-base with `main` and **excludes the parent PR's code changes** (with a
  warning). Charts are unaffected — the checkout includes the parent branch.
  Retarget at `main` once the parent merges.
- **Each push re-deploys** the PR env from scratch and cancels any in-flight run.
- **Namespaces key off the PR author**, not whoever pushed/labeled — so deploy
  and teardown always agree.
- **`nb_coprocessor > 1` is expensive** (each party is a full stack with its own
  workers/Postgres/S3). Keep it `1` unless you're specifically testing multi-party.
- **Manual (dispatch) envs never auto-destroy** — run **preview-env-destroy** with
  the namespace to clean up (see [Destroy an environment](#destroy-an-environment)).
- **`chain_mode=blockchain-dev` on dispatch, or via `preview-env-blue-green`.** Plain
  `preview-env-e2e` / `-tests` labels stay on Anvil. Faucet-funded wallets are
  unique per run. Destroying the namespace does **not** remove contracts from
  the shared Geth/Nitro — they stay on `blockchain-dev` (see explorers
  `host-explorer-blockchain-dev` / `gateway-explorer-blockchain-dev`).
  Automated tests use Hardhat network `zwsDev` (live path: HCU cheat tests skip).
- **`chain_mode=testnets` costs real gas and leaves contracts on Sepolia + Amoy.**
  Check the treasury balance before dispatching (the fund step fails fast if it
  cannot cover the top-ups). Expect a much longer deploy: 12 s Sepolia blocks
  stretch every hardhat step and the keygen ceremony. Etherscan / Polygonscan show
  the deployed addresses (`host-sc-addresses` / `polygon-sc-addresses` ConfigMaps).
