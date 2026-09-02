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
| `preview-env-blue-green` | Deploy [RFC-021](https://github.com/zama-ai/tech-spec/pull/443) BCS+GCS on each party (forces `nb_coprocessor=2`). Enough on its own. Combined with `preview-env-e2e-tests`: propose after the relayer is up, hold `consensus-detector` so the first e2e stays on blue (`DryRunStarted`, assert GCS `computations > 0`), then enable the detector, wait for `versioning=v0.15`, and run e2e again on green. Incompatible with `deploy_polygon`. |

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
  `1`). `> 1` is N-party consensus (separate wallets/DBs/S3), **not**
  blue-green. `2` on a dispatch **is** RFC-021 blue-green (BCS+GCS on each
  of the two parties). `3`/`5` stay N-party only. See `README.md`.
- `deploy_polygon` — also add a second Polygon Amoy (`80002`) host chain (default
  `false`). Fresh local anvil, reuses the ETH KMS key; roughly doubles the
  host-side stack. With `automated_tests` on it also runs a Polygon e2e suite.
  See the multichain section in `README.md`. Incompatible with
  `use_blockchain_dev`.
- `use_blockchain_dev` — skip per-namespace Anvil and connect to the shared
  `blockchain-dev` Geth (host chain id `1337`) + Nitro (gateway `412346`).
  Generates a unique mnemonic, funds the derived wallets from the in-cluster
  faucets, and still deploys **this preview's own contracts**. Dispatch-only
  (PR labels stay on Anvil). After teardown the contracts remain on the shared
  chain. Do not combine with `deploy_polygon`.

**Versions** — three kinds:
- **fhevm's own images** (`coprocessor_version`, `test_suite_version`, …)
  default to **empty**, meaning "resolve from the base commit" exactly as a PR
  run does. Set one only to force a specific tag for this run.
- **In-repo charts** (`coprocessor_chart_version`, `contracts_chart_version`, …)
  default to **empty**, meaning "install `charts/<name>` straight from the
  picked branch". Set one to deploy that **published** OCI chart release
  instead.
- **External deps** (`common_chart_version`, `redis_chart_version`,
  `kms_core_version`/`kms_repo_ref`, …) also default to **empty**, meaning
  "use the pinned version in the workflow env" — they're owned by other
  repos, so there's no commit of this repo to derive them from.
  (`relayer_sdk_version` keeps a real default; emptying it skips the
  relayer-sdk suite.)

- **Namespace:** `fhevm-ci-<actor>-<run-id-base36>` (dispatch) or
  `fhevm-ci-<pr-author>-<pr-number>` (PR). Actor is truncated if needed so
  the whole name stays under 28 chars.
- **Results:** run summary (deployment plan + e2e report if `automated_tests`).
- **Teardown:** **manual** — a dispatch env is not tied to a PR, so nothing
  destroys it automatically. Run **preview-env-destroy** with the namespace (see
  [Destroy an environment](#destroy-an-environment)), or re-run to reuse it.

### Launch from the CLI (`gh api`)

Same manual run, scripted. `gh api` expands the `inputs[key]=value` brackets into
the `inputs` object the dispatch endpoint expects; `ref` is the branch the run
executes from. Every input has a default, so pass only `ref` plus what you want
to override:

```bash
gh api --method POST \
  -H "Accept: application/vnd.github+json" \
  /repos/zama-ai/fhevm/actions/workflows/preview-env-deploy.yml/dispatches \
  -f "ref=<your-branch>" \
  -f "inputs[build_images]=true" \
  -f "inputs[automated_tests]=true" \
  -f "inputs[nb_coprocessor]=1" \
  -f "inputs[nb_kms_core]=4" \
  -f "inputs[deploy_polygon]=false" \
  -f "inputs[use_blockchain_dev]=false"
```

Connect to the shared `blockchain-dev` Geth + Nitro (no Anvil). The namespace
is derived automatically from this run's id (base36) — there is no
`namespace_suffix` input:

```bash
gh api --method POST \
  -H "Accept: application/vnd.github+json" \
  /repos/zama-ai/fhevm/actions/workflows/preview-env-deploy.yml/dispatches \
  -f "ref=<your-branch>" \
  -f "inputs[use_blockchain_dev]=true" \
  -f "inputs[deploy_polygon]=false"
```

The endpoint returns `204 No Content` (fire-and-forget); find the run with:

```bash
gh run list --workflow=preview-env-deploy.yml --branch=<your-branch> --limit 5
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
  `DryRunStarted` (BCS live; CI asserts each party's `"gcs-0.15.0".computations`
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

Or script it with `gh api` (runs the destroy workflow from `main`; the
`fhevm-ci-` namespace guard still applies):

```bash
gh api --method POST \
  -H "Accept: application/vnd.github+json" \
  /repos/zama-ai/fhevm/actions/workflows/preview-env-destroy.yml/dispatches \
  -f "ref=main" \
  -f "inputs[namespace]=fhevm-ci-<actor>-<run-id digest>"
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
- **There are no version pins for fhevm's own artifacts.** Charts come from your
  checkout, and every push to `main`/`release/*` tags every image with that
  commit's short SHA, so unbuilt components resolve from your base commit. Check
  the run summary's **Images** table: each row shows the tag *and* where it came
  from (`built`, `base-sha`, `dispatch-override`).
- **Unresolvable ⇒ the run fails.** If GHCR has pruned the base commit's tags and
  nothing turns up within 50 commits, `resolve-tags` fails instead of quietly
  deploying something older. Rebase onto a newer base commit, or pass an explicit
  `*_version` input via dispatch.
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
- **`use_blockchain_dev` is dispatch-only.** PR labels always deploy Anvil.
  Faucet-funded wallets are unique per run. The namespace is
  `fhevm-ci-<actor>-<run-id-base36>` (read it from the run summary).
  Destroying the namespace does **not** remove contracts from the shared
  Geth/Nitro — they stay on `blockchain-dev` (see explorers
  `host-explorer-blockchain-dev` / `gateway-explorer-blockchain-dev`).
  Automated tests use Hardhat network `zwsDev` (live path: HCU cheat tests skip).
