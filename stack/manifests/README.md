# stack/manifests — the recipe's data layer (raw v0.13 manifests)

These are the **live-proven** Kubernetes manifests the §0 boot recipe applies via
`kubectlApply(${dataDir}/<file>.yaml)` (see `stack/lib/recipe.ts`). They were authored and
**verified running on cluster `fhevm-p2` (2026-06-17)** — the coprocessor reached a registered
FHE key and the relayer served `/v2/keyurl`. They are raw manifests (not Helm) because
`charts/{coprocessor,kms-connector}` are still v0.11 while these run **v0.13.0-6** (the
engine-fork the goal must resolve — bump the charts, or keep this raw-manifest path).

## Files (apply order per recipe)

All files below were extracted from the live `fhevm-p2` cluster (`kubectl get -o yaml`,
volatile metadata/status stripped) — the complete proven workload set.

| file | recipe phase | contents |
|---|---|---|
| `anvil-host.yaml` / `anvil-gateway.yaml` | `chains` | anvil-node chart values per chain (chainId, mnemonic, port). |
| `data-plane.yaml` | `data-plane` | postgres `db` + `minio` Deployments + `minio-init` Job. |
| `services.yaml` | (wiring) | Services: `db`, `minio`, `kms-core`, and the `host-node`/`gateway-node` **alias** Services consumers expect (the chart names svc `<release>-anvil-node`). |
| `kms-core.yaml` | `kms-core` | kms-core Deployment + config ConfigMap (the centralized v0.13.20-0 companion). |
| `kms-connector.yaml` | `kms-connector` | `connector-env` ConfigMap + 3 connector Deployments (gw-listener/kms-worker/tx-sender) + migration Job. |
| `coprocessor.yaml` | `coprocessor` | `coprocessor-env` ConfigMap + db-migration Job + 7 service Deployments (host-listener[+poller], gw-listener, tfhe/zkproof/sns-worker, transaction-sender). **omits** host-listener-consumer (needs redis). |
| `host-trigger.yaml` | `trigger-keygen` | host-side `task:triggerKeygen` + `task:triggerCrsgen` Job (mounts the `host-addr` PVC). |
| `relayer.yaml` | `relayer` | `relayer-env` ConfigMap + relayer-migrate Deployment + relayer Deployment + Services. |
| `relayer-config.yaml` | `relayer` | the relayer app config; create as ConfigMap `relayer-config` (`kubectl create configmap relayer-config --from-file=local.yaml=relayer-config.yaml`) before `relayer.yaml`. |

`setup.yaml` covers the remaining setup objects: the `host-addr` (deploy `/app/addresses`)
and `kms-keys` PVCs, and a `db-init` Job that idempotently creates the
`coprocessor`/`kms-connector`/`relayer` databases. minio buckets are created by
`data-plane.yaml`'s `minio-init` Job.

Created **out-of-band** (never committed): the `registry-credentials` Secret (a ghcr
`read:packages` token) and `minio-secrets` (dev minio creds). Create these before applying:
`kubectl create secret docker-registry registry-credentials …` and the minio access/secret
key Secret the kms-core + minio-init read.

## Runtime values to TEMPLATIZE (do not ship these as-is)

These manifests carry the **live snapshot** values; the recipe's discovery phases must inject
the realized ones at runtime (the §0 seed-overwrite invariant). Before this is a clean,
reusable data layer:

1. **FHE_KEY_ID / KMS_*_KEY URLs** in `coprocessor.yaml` + `relayer*.yaml` are pinned to the
   realized `0400…0002` / `0500…0002` from this cluster's re-keygen. They must be the
   **seed** (`…0001`) at apply time, then patched by the `await-keygen` phase's discovered id.
2. **host-listener `minio-dnat` initContainer** DNATs `172.17.0.1:9000 → 10.244.0.19:9000`
   (the minio **pod IP** — ephemeral). It must resolve the minio **Service** at runtime
   (the DNAT works around the v0.13.0-6 host-listener's hardcoded `minio:9000→172.17.0.1`
   rewrite — see EXEMPLAR §0 finding 3; the cleaner fix is advertising a non-matching host).
3. **relayer keyurl** (`APP_KEYURL__*` + `relayer-config.yaml`) uses `localhost:9000` — the
   **host-run port-forward** hack. In-cluster it must be `minio:9000`. Also: the relayer
   advertises keyurl from **on-chain (gateway)**, so host+gateway must agree on the active
   key (the key-consistency wall, EXEMPLAR §0 finding 5).

Contract addresses (ACL, executor, gateway-config, …) are **deterministic** and match staging
exactly, so they are correct as-is. The kms signer is discovered live (never hardcoded).
