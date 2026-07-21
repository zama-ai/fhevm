# Preview env 101 — how to spin up an e2e preview

A **preview env** is a full, throwaway fhevm stack (anvil host+gateway chains, a
real threshold+enclave KMS, contracts, coprocessor, kms-connector, relayer and
the e2e test-suite) deployed to its own namespace on the `zws-dev` cluster. Use
it to exercise a PR end-to-end against real charts and a real KMS.

This page is the **usage** guide. For what's inside and why, see
[`README.md`](./README.md); the workflow itself is
[`pr-preview-deploy.yml`](../../.github/workflows/pr-preview-deploy.yml).

There are two ways to launch: **from a PR** (add a label) or **manually**
(`workflow_dispatch`, for full control over versions/topology).

## Prerequisites

- Membership of the `coprocessor-dev-access` group (grants namespace admin) and
  Tailscale access to the `zws-dev` cluster — needed to connect afterwards.
- Write access to the PR (to add labels) or to run workflows (for a manual run).

---

## Option A — from a PR (labels)

Add one (or more) of these labels to the PR. The env deploys automatically; each
new push re-deploys it fresh (an in-flight run is cancelled).

| Label | What it does |
| --- | --- |
| `pr-preview-e2e` | Deploy the stack using the **pinned** image versions (builds nothing). Fastest. |
| `pr-preview-e2e-build` | Same, but **builds fresh images from the PR branch** first (only changed components; the rest fall back to pinned). Use this to test your code. |
| `pr-preview-e2e-tests` | Deploy **and** auto-run the e2e test DAG, posting a pass/fail report back to the PR. Deploys the env on its own. |

Labels combine, e.g. `pr-preview-e2e-build` + `pr-preview-e2e-tests` = build your
branch **and** auto-run the suite.

- **Namespace:** `fhevm-ci-<pr-author>-<pr-number>`.
- **Results:** a `:rocket:` comment on success; with `pr-preview-e2e-tests`, a
  per-test SDK-matrix report comment (see [See test results](#see-test-results)).
- **Teardown:** automatic when the PR is **closed**, or when you **remove** the
  preview label(s) (handled by
  [`pr-preview-destroy.yml`](../../.github/workflows/pr-preview-destroy.yml)).

---

## Option B — manual run (`workflow_dispatch`)

GitHub → **Actions** → **pr-preview-deploy** → **Run workflow**, pick the branch
(via "Use workflow from"), set inputs, run. Use this to change versions or
topology, or to deploy without a PR.

Key inputs (all have sensible defaults — you rarely set more than a couple):

**Control**
- `build_images` — build fresh images from the picked branch (`true`) or deploy
  pinned versions only (`false`).
- `build_test_suite_only` — when building, build **only** the e2e test-suite
  image (fast test-suite iteration); everything else stays pinned.
- `automated_tests` — auto-run the e2e DAG and write the report to the run
  summary.
- `namespace_suffix` — fixed suffix for the namespace (e.g. a ticket id);
  empty ⇒ this run's id (always unique).

**Topology**
- `nb_kms_core` — number of KMS parties (default `4`).
- `nb_coprocessor` — number of independent coprocessor stacks (default `1`).
  `> 1` multiplies cluster capacity — see the resource caveat in `README.md`.

**Versions** (chart versions, image tags, `kms_core_version`/`kms_repo_ref`,
`relayer_sdk_version`, …) — override any pin for this run. Each falls back to the
same value used on PR runs when left at its default.

- **Namespace:** `fhevm-ci-<actor>-<namespace_suffix | run-id>`.
- **Results:** run summary (deployment plan + e2e report if `automated_tests`).
- **Teardown:** **manual** — a dispatch env is not tied to a PR, so nothing
  destroys it automatically. Delete the namespace (or re-run to reuse it).

---

## Connect to your environment

```bash
tailscale configure kubeconfig tailscale-operator-zws-dev.diplodocus-boa.ts.net
kubectl get pods -n <namespace>          # e.g. fhevm-ci-alice-1234
```

## See test results

- **With auto-tests** (`pr-preview-e2e-tests` label or `automated_tests=true`):
  the workflow runs the e2e DAG for both `@fhevm/sdk` and `@zama-fhe/relayer-sdk`
  and posts a per-test pass/fail table to the PR comment / run summary.
- **Without:** the stack is deployed with an idle test-suite Job — run tests
  yourself against the namespace, or re-label with `pr-preview-e2e-tests`.

## Gotchas

- **`build_images=false` ⇒ pinned versions.** A plain `pr-preview-e2e` label does
  **not** test your branch's code; use `-build` for that.
- **Each push re-deploys** the PR env from scratch and cancels any in-flight run.
- **Namespaces key off the PR author**, not whoever pushed/labeled — so deploy
  and teardown always agree.
- **`nb_coprocessor > 1` is expensive** (each party is a full stack with its own
  workers/Postgres/S3). Keep it `1` unless you're specifically testing multi-party.
- **Manual (dispatch) envs never auto-destroy** — clean up the namespace yourself.
