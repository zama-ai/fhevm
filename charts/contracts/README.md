# Contracts Helm Chart

Runs FHEVM smart-contract deployment, upgrade and one-off operational tasks as
Kubernetes Jobs.

## How it works

Declare each Job as an entry in `scJobs:`:

```yaml
scJobs:
  - name: protocol-payment
    kind: deploy
    image: { name: ghcr.io/zama-ai/fhevm/gateway-contracts, tag: v0.10.0 }
    env:
      - { name: DEPLOYER_PRIVATE_KEY, value: "" }
    commands:
      - npx hardhat task:deploySingleContract --name ProtocolPayment
```

Each entry renders to a `Job` named `<release>-<kind>-<name>-<tagSlug>`. Jobs
share a single PVC (`/app/addresses`) so the OpenZeppelin `.openzeppelin/`
manifest persists across runs.

After commands finish, the runner script parses the entry's `envFile` (a path
or glob, default `addresses/.env.*`) and patches the addresses ConfigMap:

- `<contract>.address`: written once, never overwritten (proxy addresses are
  immutable across upgrades).
- `<contract>.version`: overwritten with the Job's `image.tag` for `deploy`
  and `upgrade` kinds only.

If jobs for different contract families share a PVC, set `envFile` to the file
each image family writes (`addresses/.env.host`, `addresses/.env.gateway`, …)
so a job never re-stamps another family's `.version` keys with its own tag.

## Job kinds

| kind      | When to use                                        | 
|-----------|----------------------------------------------------|
| `deploy`  | First-time contract deployment                     | 
| `upgrade` | OpenZeppelin proxy upgrade to a new implementation | 
| `task`    | One-off operation                                  | 

## Re-run safety

Driven by Job-name uniqueness + the persistent PVC:

- `deploy` / `upgrade`: idempotent in practice. OpenZeppelin reads the existing
  proxy from the persisted `.openzeppelin/` manifest and skips redeploy.
- `task`: not idempotent at the chart level. Re-running an ownership transfer
  or a keygen has on-chain side effects. Don't delete a completed task Job
  unless you've confirmed it's safe to replay; the live Job acts as its own
  marker for ArgoCD reconciliation.

ArgoCD considerations:

- Do not delete completed Jobs. With unique per-tag names, the live Job's spec
  matches the desired spec on every sync, so ArgoCD takes no action. The
  completed Job is its own sentinel against re-execution.
- All Jobs are rendered with
  `argocd.argoproj.io/sync-options: Prune=false` so removing an entry from
  `scJobs` keeps the historical Job in the cluster. Note: this only protects
  against source-driven deletion. A manual `kubectl delete job …` will still
  be re-applied by the next sync.
- Jobs are also rendered with `argocd.argoproj.io/sync-wave: <index>` derived
  from their position in `scJobs`. ArgoCD applies each wave and waits for the
  Job to Succeed before starting the next, giving sequential execution within
  a sync.

## Debugging

Set `scDebug.enabled: true` to spawn a long-running pod with the deploy PVC
mounted. The pod reuses the image, env, `oldImage` and persistence settings of
the last enabled `scJobs` entry, so at least one entry is required. Use
`scDebug.mountMode`:

- `readOnly` (default): live view of the PVC, no write access.
- `readWrite`: direct read-write mount; reserve for manually fixing PVC contents.

Both modes share the PVC's node-level `ReadWriteOnce` lock with running Jobs;
a Job scheduled to a different node will wait until the debug pod is gone.

Exec into the scDebug pod to troubleshoot the contract deployment environment:

```bash
kubectl exec -it sc-deploy-0 -- sh
 ```
