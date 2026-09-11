# fhevm ephemeral PR-preview e2e config

> **New here?** Read [`101-preview-env.md`](./101-preview-env.md) first — a short,
> practical guide to launching a preview from a PR label or a manual dispatch.
> This README covers the internals (what is deployed and why).

Config for the per-PR e2e preview environment, additive to the default docker-compose path
driven by `test-suite/fhevm` (`fhevm-cli`). Mirrors the layout and conventions of
[`zama-ai/kms/ci/kube-testing`](https://github.com/zama-ai/kms/tree/main/ci/kube-testing) +
[`zama-ai/kms/ci/scripts`](https://github.com/zama-ai/kms/tree/main/ci/scripts).

Every PR that carries the `preview-env-e2e` label gets a per-PR namespace
(`fhevm-ci-<actor>-<pr>`) on the real `zws-dev` Tailscale cluster — this repo's real
production charts (installed straight from the branch checkout, so a PR's chart changes
deploy as-is), a Crossplane-provisioned S3 bucket (`coprocessor-infra/`), in-cluster Postgres (official
`postgres:17-alpine` image via the generic `common` chart, one dedicated instance each for
coprocessor, listener, kms-connector, and relayer/relayer-migrate — see `coprocessor-infra/values-postgres-
coprocessor-e2e.yaml`'s header for why this isn't RDS, or Bitnami's postgresql chart), and a
dedicated real 4-party threshold+enclave KMS reused directly from `zama-ai/kms`'s own
`ci/scripts/deploy.sh` (no vendored `kms-core` chart usage here at all). See
[`../../.github/workflows/preview-env-deploy.yml`](../../.github/workflows/preview-env-deploy.yml).
Torn down automatically when the PR closes
([`preview-env-destroy.yml`](../../.github/workflows/preview-env-destroy.yml)).

There is no local Kind/laptop-based variant of this path anymore — a full stack (dedicated
4-party enclave KMS + coprocessor + kms-connector + relayer + test-suite) doesn't fit in a
reasonable local resource budget, so `preview-env-deploy.yml`'s remote `zws-dev` cluster is the
only supported way to run this. The default docker-compose path (`test-suite/fhevm`) remains
the right choice for local iteration.

## Layout

```
ci/preview-env/
├── coprocessor-infra/
│   ├── values-coprocessor-infra-e2e.yaml    # crossplane/coprocessor-infra overlay: S3 only
│   └── values-postgres-coprocessor-e2e.yaml # `common` chart overlay: in-cluster Postgres, dedicated to coprocessor
├── testnets/
│   ├── values-rpc.yaml                 # sync-secrets: Sepolia/Amoy RPC URLs → Secret rpc
│   ├── values-eth-faucet.yaml        # sync-secrets: zws-dev/ethereum-faucet → Secret eth-faucet
│   └── values-polygon-faucet.yaml    # sync-secrets: zws-dev/polygon-faucet → Secret polygon-faucet
├── host-chain/
│   ├── values-anvil-host-e2e.yaml       # anvil-node overlay, host chain
│   ├── values-anvil-host-polygon-e2e.yaml   # anvil-node overlay, Polygon host chain (deploy_polygon)
│   ├── values-host-contracts-e2e.yaml   # contracts overlay, host-contracts
│   ├── values-host-contracts-polygon-e2e.yaml # contracts overlay, Polygon host-contracts (mirrors ETH ProtocolConfig)
│   └── values-host-trigger-keygen-e2e.yaml # contracts overlay, real FHE key/CRS gen ceremony
├── gateway-chain/
│   ├── values-anvil-gateway-e2e.yaml         # anvil-node overlay, gateway chain
│   ├── values-gateway-contracts-e2e.yaml     # contracts overlay, gateway-contracts
│   ├── values-gateway-add-host-chains-e2e.yaml # contracts overlay, deferred addHostChains step
│   └── values-gateway-add-host-chains-polygon-e2e.yaml # contracts overlay, register Polygon (80002) (deploy_polygon)
├── coprocessor/
│   ├── values-coprocessor-e2e.yaml        # coprocessor overlay (one release per party: coprocessor-<i>)
│   ├── values-coprocessor-bcs-e2e.yaml    # RFC-021 BCS overlay (pinned 0.14.0, extraSelectorLabels)
│   ├── values-coprocessor-gcs-e2e.yaml    # RFC-021 GCS overlay (compiled release + upgrade-controller / consensus-detector)
│   ├── values-coprocessor-polygon-e2e.yaml # additive multichain overlay: adds the Polygon chains[] consumer (deploy_polygon)
│   ├── values-coprocessor-poller-e2e.yaml # coprocessor overlay, poller-only release: S3 key/CRS download -> keys/crs tables (coprocessor-poller-<i>)
│   ├── values-coprocessor-poller-polygon-e2e.yaml # additive multichain overlay: adds the Polygon poller (deploy_polygon)
│   └── values-coprocessor-redis-e2e.yaml  # iamguarded Redis overlay: per-party host-listener-consumer broker (coprocessor-redis-<i>)
├── listener/
│   ├── values-listener-e2e.yaml          # listener chart overlay: per-party host-chain event producer (listener-<i>)
│   ├── values-listener-polygon-e2e.yaml  # listener overlay, Polygon producer (listener-polygon-<i>) (deploy_polygon)
│   └── values-postgres-listener-e2e.yaml # `common` chart overlay: in-cluster Postgres, dedicated to the listener's cursor DB
├── kms-connector/
│   ├── values-kms-connector-e2e.yaml       # kms-connector overlay
│   ├── values-kms-connector-polygon-e2e.yaml # additive overlay: adds the Polygon host chain to hostChains (deploy_polygon)
│   └── values-postgres-connector-e2e.yaml  # `common` chart overlay: in-cluster Postgres, dedicated to kms-connector
├── observability/
│   ├── values-prometheus-e2e.yaml # `common` chart overlay (raw objects): in-namespace Prometheus, endpoints-SD scraping
│   ├── values-jaeger-e2e.yaml     # `common` chart overlay: Jaeger all-in-one, OTLP trace collector
│   └── values-grafana-e2e.yaml    # `common` chart overlay: Grafana UI over both
├── relayer/
│   ├── values-relayer-e2e.yaml          # `common` chart overlay, relayer server
│   ├── values-relayer-migrate-e2e.yaml  # `common` chart overlay, relayer DB migration Job
│   └── values-postgres-relayer-e2e.yaml # `common` chart overlay: in-cluster Postgres, dedicated to relayer + relayer-migrate
├── test-suite/
│   ├── values-test-suite-e2e.yaml       # `common` chart overlay, e2e test-suite Job
│   └── values-test-suite-workflow-polygon-e2e.yaml # Argo Workflow overlay, Polygon e2e run (deploy_polygon + automated_tests)
├── preview-env                          # gh CLI: launch / watch / destroy (does not helm-install)
└── scripts/                             # deploy-time helpers called from preview-env-deploy.yml
```

Every file here is a **values overlay for a chart**. `anvil-node`/`contracts`/`coprocessor`/
`kms-connector`/`listener` target the real production charts in `../../charts/`, installed
straight from the workflow's branch checkout — a PR's chart changes deploy without waiting
for a publish, and the overlays can never drift ahead of the charts they configure. A
`*_chart_version` dispatch override swaps one in-repo chart for its published
`oci://hub.zama.org/ghcr/zama-ai/fhevm/charts/*` release instead (see
`preview-env-deploy.yml`'s "Resolve chart sources" step).
`relayer`/`test-suite`/the three `values-postgres-*-e2e.yaml` files all target the generic
`oci://hub.zama.org/ghcr/zama-zws/helm-charts/common` chart (the same one `zama-zws/gitops`'s
`fhevm-dev` environment uses for `relayer`/`test-suite`), not a fhevm-specific chart - there's
nothing fhevm-specific about a plain in-cluster Postgres instance either, running the official
`docker.io/library/postgres:17-alpine` image. Deliberately NOT Bitnami's own `postgresql` chart:
Bitnami restructured its free container catalog on 2025-08-28 and moved every pinned/versioned
image tag off `docker.io/bitnami` into the unsupported, no-longer-updated `docker.io/
bitnamilegacy` archive, breaking fresh installs of that chart (see values-postgres-coprocessor-
e2e.yaml's header for the full story). This also sidesteps Crossplane/RDS entirely for these
three throwaway databases. `kms-core` (KMS itself) is never deployed from this repo at all — the
CI path reuses `zama-ai/kms`'s own deploy pipeline as-is (see `preview-env-deploy.yml`).

### Dedicated KMS version pins

Preview-env **never builds** kms-core. It sparse-checkouts `zama-ai/kms` at
`kms_repo_ref`, pulls `core-service-enclave:<kms_core_version>` for PCR
attestation, and runs kms's `deploy.sh --tag … --num-parties "${NB_KMS_CORE}"`.

| Input | Override key | Meaning |
| --- | --- | --- |
| Party count | `nb_kms_core` (`4` \| `13`) | Topology only — not in `overrides`. |
| Enclave image | `kms_core_version` | GHCR tag → `KMS_CORE_TAG`. |
| Deploy scripts + chart | `kms_repo_ref` | Git SHA/ref on `zama-ai/kms`. |

Defaults live in [`scripts/parse-overrides.cjs`](./scripts/parse-overrides.cjs)
(`kms_core_version`, `kms_repo_ref`). **PR labels always use those defaults.**
Override only via dispatch `overrides` / CLI `--set`, or by bumping
`ALWAYS_DEFAULTS` for everyone. Keep the two keys aligned to the same kms
release.

**kms-connector** (`kms_connector_version`, `kms_connector_chart_version`) is
fhevm-owned and follows the normal image/chart resolve rules — separate from
kms-core.

Usage: [`101-preview-env.md`](./101-preview-env.md#option-b--manual-run-workflow_dispatch).

Note this path's own images (contracts/kms-connector/relayer/test-suite) are addressed via
`hub.zama.org/ghcr/zama-ai/fhevm/...` (the Harbor pull-through-cache mirror of `ghcr.io`), not
`hub.zama.org/zama-protocol/...`. A freshly-built PR image only exists at its GHCR tag/mirror;
`zama-protocol` is populated later by the real promotion pipeline, so pointing there would
404 on PR previews. `coprocessor`'s chart default already uses the mirror path for the same
reason.

## Why values overlays, not new charts

The production Helm charts in `../../charts/` already support everything the simplest e2e
scenario needs via values alone (see gap analysis in the feasibility plan). Keeping
e2e-specific config as overlays here — rather than forking or templating the charts — means
this path never drifts from what devnet/testnet/mainnet actually deploy.

## Network mode: live vs local (test-coverage trade-off)

The e2e suite (`test-suite/e2e`) keys a lot of behavior off the Hardhat **network
name** it is run with, via `isLiveNetwork()`:

```ts
// test-suite/e2e/test/network.ts
const LIVE_NETWORKS = new Set(['devnet', 'devnetNative', 'zwsDev', 'sepolia', 'mainnet', 'polygonAmoy']);
export const activeNetworkName = () => network.name;
export const isLiveNetwork = () => LIVE_NETWORKS.has(activeNetworkName());
```

This preview's **default** (PR labels + dispatch with `chain_mode=anvil`)
runs the host chain as **`staging`** (chainId `12345` Anvil), which is **not**
in that set, so `isLiveNetwork()` is `false` and the suite takes its **local /
deterministic** path. That matters because the check is on the *name only*, not on
what the underlying node can actually do — our host chain is anvil (fully
controllable), but the suite's coverage is decided purely by the label.

- **Non-live name (`staging`, what we use) → strongest coverage.** The
  "local deterministic coverage" blocks run: they call owner-only setters
  (`setHCUPerBlock` / `setMaxHCUPerTx` / `setMaxHCUDepthPerTx`), assert **exact**
  HCU numbers, use the anvil/hardhat cheats (`evm_setAutomine` /
  `evm_setIntervalMining` / `evm_mine`) to pack a block and watch the per-block cap
  get exhausted, restore state afterward, and check the negative case ("reject
  `setHCUPerBlock` from a non-owner"). These assume the test EOA owns the privileged
  contracts and that the chain is a resettable throwaway.
- **Live name (`zwsDev` / `devnet` / `sepolia` / …) → reduced coverage.** Those
  deterministic blocks are `this.skip()`'d and only read-only checks run (e.g.
  observed HCU ≤ deployed caps), because a real shared chain gives you no owner
  rights, no `evm_*` cheats, and no state reset.

So switching this preview to a live-flagged network is **not free**: it would make
`hcu-block-cap` go green, but only because the failing subtests (owner-only cap
mutations → `NotHostOwner`, and the non-owner rejection) stop running — you verify
*less*, not *more*. It hides the underlying issue (the test signer not being the ACL
owner) rather than fixing it.

Caveats if you ever do want the live path against anvil:
- **`devnet` is not usable here** — it is in `LIVE_NETWORKS` but has no entry in
  `hardhat.config.ts`'s `networks:` map, so `--network devnet` errors out. The only
  live-flagged name wired to `RPC_URL` (i.e. can point at our anvil) is **`zwsDev`**.
- **`zwsDev` implies chainId `1337`**, and Hardhat validates the configured chainId
  against the node's `eth_chainId`. So it is not a rename: the host anvil would have
  to run with `--chain-id 1337`, which cascades into re-wiring the host chain id
  (`12345` → `1337`) across the coprocessor / gateway / host values.

## Observability (opt-in: `observability` dispatch input)

`preview-env-deploy.yml` takes an `observability` input (default `false`,
dispatch-only for now) that deploys a self-contained, namespaced observability
stack alongside the env — three more `common`-chart releases (`prometheus`,
`jaeger`, `grafana`, all official public images), torn down with the namespace
like everything else:

- **Prometheus** scrapes every Service in the namespace exposing a named
  `metrics` or `monitoring` port via Kubernetes endpoints service discovery.
  No static target list, no ServiceMonitors (no dependency on cluster-wide
  prometheus-operator CRDs), and the per-party fan-out
  (`nb_coprocessor`/`nb_kms_core`) is followed automatically. Metric history
  sits on a 10Gi PVC (7d retention) so a pod reschedule mid-bench doesn't wipe
  it. Its manifests ship as raw `additionalResources` objects (the
  endpoints-SD ServiceAccount/Role/RoleBinding wiring needs that — see
  `observability/values-prometheus-e2e.yaml`'s header).
- **Jaeger all-in-one** (v2, in-memory) is the OTLP collector.
- **Grafana** (anonymous admin — throwaway namespace, Tailscale-only) is the
  UI over both, with Prometheus + Jaeger datasources provisioned. No
  dashboards are provisioned yet; UI-created ones die with the pod, so export
  what you want to keep.

Access is `kubectl port-forward` from the dev laptop (Tailscale up, namespace
admin via `coprocessor-dev-access`/`kms-dev-access`) — see
[`101-preview-env.md`](./101-preview-env.md#observe-your-environment).

## Chain modes (`chain_mode`: `anvil` | `blockchain-dev` | `testnets`)

`preview-env-deploy.yml` picks the chains via the `chain_mode` dispatch input
(`ci/preview-env/scripts/resolve-chain.sh` resolves it; PR labels stay on Anvil except
`preview-env-blue-green`, which forces `blockchain-dev`). Everything below the chain
layer is identical across modes: the same charts, the same overlays, patched at deploy
time by `apply-chain-env.sh` for the two external modes.

| Mode | Host chain(s) | Gateway | Wallets | Funding |
|------|---------------|---------|---------|---------|
| `anvil` (default) | per-namespace Anvil `12345` (+ Anvil Amoy `80002` with `deploy_polygon`) | per-namespace Anvil `54321` | Foundry junk mnemonic, 120 prefunded accounts | none needed |
| `blockchain-dev` | shared zws-dev Geth `--dev` `1337` (`http://ethereum-rpc-node.blockchain-dev:8545`, WS same port, 5 s blocks) | shared Nitro `412346` (HTTP `:8547`, WS `:8548`) | fresh mnemonic per run | in-cluster PoW faucets (host + gateway) |
| `testnets` | public **Sepolia `11155111`** + **Polygon Amoy `80002`**, RPC URLs from AWS via sync-secrets (`deploy_polygon` implied) | shared Nitro `412346` | fresh mnemonic per run | Sepolia from `zws-dev/ethereum-faucet`, Amoy from `zws-dev/polygon-faucet`; gateway from the Nitro faucet |

Both external modes still deploy **this preview's own** host + gateway contracts, derive the
same HD index map (`#0` gateway deployer, `#3` relayer, `#9` host/ACL owner, `#10+` KMS /
coprocessor tx-senders) from the generated mnemonic (stored as secret
`preview-wallets-mnemonic` in the namespace), and leave the contracts on the shared chains
after teardown.

### `blockchain-dev`

Because Geth has no `evm_*` cheats and `--slots-in-an-epoch`, automated tests use Hardhat
network **`zwsDev`** (live path: HCU deterministic blocks skip). The coprocessor poller
seeds at the **current host head**, not block 0 — the Geth dev chain already has millions
of blocks. Incompatible with `deploy_polygon` (no Amoy node in `blockchain-dev`).

### `testnets` (Sepolia + Amoy)

The two host chains are the real public testnets, so this is the only preview shape with
**two host chains on real block times** (12 s / ~2 s). What it needs and what it changes:

- **Secrets come from AWS Secrets Manager via the gitops `sync-secrets` chart**
  (`testnets/values-rpc.yaml`, `values-eth-faucet.yaml`, `values-polygon-faucet.yaml`,
  installed by `deploy-rpc-secret.sh` against `ClusterSecretStore/secret-store`):
  - Secret **`rpc`**: `ethereum-rpc-url`, `ethereum-rpc-ws-url`, `polygon-rpc-url`,
    `polygon-rpc-ws-url` from the existing `zws-dev/external-eth-rpcs` /
    `zws-dev/external-polygon-rpcs` entries (same keys gitops gives the coprocessor).
  - Secret **`eth-faucet`**: `private-key` from **`zws-dev/ethereum-faucet`** (Sepolia).
  - Secret **`polygon-faucet`**: `private-key` from **`zws-dev/polygon-faucet`** (Amoy).
    Same AWS secrets gitops uses for the zws-dev faucets. No GitHub secrets.
- **In-cluster consumers read Secret `rpc` directly** (`secretKeyRef`): contracts Jobs (`RPC_URL`,
  `CANONICAL_RPC_URL`), listeners (`APP_BLOCKCHAIN__RPC_URL`), coprocessor `chains[]`
  (`httpUrlValueFrom`/`wsUrlValueFrom`), kms-connector (`$(RPC_ETH_URL)` / `$(RPC_POLYGON_URL)`
  through `commonConfig.env`), relayer, test-suite and the e2e Workflows. Only the runner-side
  funder reads the faucet Secrets, masked, from `deploy-rpc-secret.sh`.
- **Funding.** `fund-wallets-treasury.cjs` tops up `#0-#4` to 0.2 ETH on Sepolia and
  deployer `#9` to 1.0 ETH; on Amoy the signers get 2.0 POL and the deployer 6.0
  (`FLOOR_WEI` / `DEPLOYER_FLOOR_WEI`). Amoy gas is the reason for both: one e2e fixture
  deploy costs up to 0.3 POL, and the host-contracts deploy there spends ~2.5 POL at
  69 gwei because the canonical-snapshot flow builds the empty-proxy set twice. Unspent
  POL is stranded (fresh mnemonic per run), so raise these only against measured cost.
  It fails fast if either faucet cannot cover the shortfall.
  The **KMS tx-senders also get 0.05 ETH each on Sepolia**: since RFC013 KMSGeneration
  sits on the canonical host chain, so they sign the keygen/crsgen responses there and
  the ceremony stalls at "insufficient funds" without it. Their decryption responses,
  the coprocessor tx-senders, `#0` and `#3` are gateway-side and come from the Nitro faucet.
- **Ceremony timing.** The kms-connector's Ethereum listener pins its reads to the
  *finalized* block, ~14 min behind head on Sepolia, and that is hardcoded in the
  connector. Keygen spans two such round trips and was measured at **43 min** end to end,
  so `apply-chain-env.sh` raises the in-pod waits to `KEYGEN_WAIT_TIMEOUT_MS=60m` and
  `CRSGEN_WAIT_TIMEOUT_MS=40m` (both 15 m elsewhere), with `KEYGEN_TIMEOUT=110m` giving
  helm room for both in the one pod. Budget ~70 min of wall clock here on a good run.
  The relayer inherits the same dependency - it seeds `/v2/keyurl` from `getCrsMaterials`
  at the finalized block and exits if the CRS is not visible yet - so `deploy-relayer.sh`
  waits on its rollout before the e2e Workflows start, otherwise every test fails on
  ECONNREFUSED to `relayer:3000` and reads as a product regression. That wait needs the
  `readinessProbe` on `/v2/keyurl` in `relayer/values-relayer-e2e.yaml`: with no probe a
  pod counts as Ready the moment the container starts, so the rollout returns while the
  relayer is still crash-looping. `/v2/keyurl` is probed rather than `/healthz` because it
  serves an in-memory value with a hardcoded 200, so it cannot 503 the pod out of the
  Service in the middle of a suite.
- **Second host chain reuses the `deploy_polygon` path**: the same Polygon overlays, with
  RPC/chain ids patched to Amoy and the Anvil Polygon node skipped. Amoy mirrors the ETH
  ProtocolConfig (canonical source) exactly as the Anvil Polygon does.
- **Finality**: listeners run `finality_depth` 2 (Sepolia) / 3 (Amoy) and pollers
  `--finality-lag` 2 / 3 (`HOST_FINALITY_*` / `POLYGON_FINALITY_*`), instead of the 0 / 1
  that single-node Anvil and Geth `--dev` allow.
- **Poller seed** is `--seed-start-block=-10` ("10 behind head", resolved by the poller at
  startup) rather than a head captured early in the run — 12 s blocks make any captured
  head drift far during the KMS deploy.
- **Timeouts**: contracts `helm --wait` 30 m (was 10 m) and keygen 60 m (was 45 m).
- **Tests** run on Hardhat networks `sepolia` (ETH) and `polygonAmoy` (both in
  `LIVE_NETWORKS`, so the reduced live path).
- **Cost and hygiene**: every run spends real testnet gas (~10 upgradeable contracts + the
  keygen ceremony per chain, then e2e FHE txs) and leaves its contracts on Sepolia and Amoy
  for good. Dispatch-only, no PR label.
- **Not yet**: `enable_blue_green` (the blue-green scripts are single-chain today).

To run `host-contracts` `task:prepareCoprocessorUpgrade` against this env, use
`--environment devnet` (same Sepolia + Amoy chain set) with `RPC_URL_GATEWAY_DEVNET` pointed at a
`kubectl port-forward` of the Nitro node.

## Multi-coprocessor (`nb_coprocessor`) and shared Redis

`preview-env-deploy.yml` takes an `nb_coprocessor` input (default `1`) that deploys
**N independent coprocessor stacks** — the same per-party fan-out pattern used for the
`nb_kms_core` KMS parties. For party `i` (1..N):

- its own in-cluster Postgres `postgres-coprocessor-<i>`,
- its own `coprocessor-infra-<i>` release → dedicated S3 bucket, IRSA ServiceAccount
  `coprocessor-<i>`, and bucket ConfigMap `coprocessor-<i>` (holding `S3_BUCKET_NAME`),
- its own tx-sender/signer identity (derived from the shared Foundry/Hardhat mnemonic
  at HD indices `(10+nb_kms_core)..(10+nb_kms_core)+N-1` — starting right after the KMS
  tx-sender range `#10..10+nb_kms_core-1` so the two never collide for any party count,
  and clear of the test signers `#0-4` and host owner `#9`), reused for both the
  on-chain registration and the `txSender` wallet,
- a `coprocessor-<i>` chart release wired to all of the above.

The gateway is told `NUM_COPROCESSORS=N` and `COPROCESSOR_THRESHOLD=floor(N/2)+1`, and
each party is registered with its `COPROCESSOR_TX_SENDER_ADDRESS_<idx>` /
`COPROCESSOR_SIGNER_ADDRESS_<idx>` / `COPROCESSOR_S3_BUCKET_URL_<idx>` (the static
`_0` values are no longer in `gateway-chain/values-gateway-contracts-e2e.yaml` — the
deploy step owns them now).

### Event pipeline: dedicated `listener` producer -> per-party Redis -> consumer

The preview uses the same producer/broker/consumer split as zama-zws/gitops
(`eth-blockchain/eth-listener` + `coproc` consumer), **not** the coprocessor chart's
self-contained `hostListener`. Per party `i`:

- **`listener-<i>`** (`charts/listener`, image `listener/listener-core`) is the producer:
  a generic EVM indexer that reads the host chain and **publishes** events to that
  party's Redis (`broker.ensure_publish: true`, `APP_BROKER__BROKER_URL`). It takes no
  FHEVM contract addresses — only `chain_id` / RPC / broker / its own cursor DB
  (`postgres-listener-<i>`). See `listener/values-listener-e2e.yaml`.
- **`coprocessor-redis-<i>`** (iamguarded standalone Redis) is that party's broker
  (`redis://coprocessor-redis-<i>-master:6379`). Per-party, not shared — mirrors gitops,
  where each listener has its own ElastiCache.
- **`coprocessor-<i>`'s `hostListenerConsumer`** reads that Redis broker and writes the
  decoded ACL / executor / kms-generation events (it has those addresses via its chain
  config) into the coprocessor DB. The coprocessor chart's built-in `hostListener` is
  **disabled** in `values-coprocessor-e2e.yaml` — the `listener` chart replaces it as the
  producer.
- **`coprocessor-poller-<i>`** (`charts/coprocessor` with only `hostListenerPoller`
  enabled, `values-coprocessor-poller-e2e.yaml`) is the DB-side background processor,
  deployed as its own release per party (mirrors gitops `eth-coproc-listener-poller`).
  Neither the producer nor the consumer downloads key material: they only record the
  `kms_key_activation` / `kms_crs_activation` event as `pending`. The poller polls the
  host chain, finalizes blocks, and on a pending activation whose block is finalized it
  **downloads** the ServerKey/PublicKey/CRS from the KMS public-vault S3 (needs the IRSA
  SA, hence `serviceAccountName=coprocessor-<i>`) and fills the `keys` / `crs` tables the
  tfhe/zkproof/sns workers read. Without it those tables stay empty and every worker
  fails "No keys found in database". It **must be deployed before the keygen trigger** so
  it anchors near genesis (`MAX(block)` of the empty DB = 0) and finalizes the keygen
  blocks in order as they are mined — deployed after keygen it anchors past them and has
  to grind the whole finalization backlog back down first.

> This producer path is newer to the preview than the old DB-only `hostListener`. Two
> things to validate on the first real multi-run (adjust the preview-env values, not the
> charts): anvil must serve the `block_receipts` RPC (`eth_getBlockReceipts`), and the
> broker topic keying (chain id `12345`) must match the consumer's `--chain-id`. (The
> `listener-core` image resolves from the same commit as the coprocessor consumer, so
> broker-payload compatibility holds by construction.)

> Resource caveat: each coprocessor party's `tfhe`/`sns` workers request substantial
> CPU/memory on the `coprocessor` nodepool, so `nb_coprocessor` > 1 multiplies the
> cluster capacity needed. Default stays `1`. Blue-green doubles the worker fleets
> again (BCS + GCS per party).

## N-party consensus vs RFC-021 blue-green

These are different models. `nb_coprocessor > 1` is N-party unless you opt into blue-green.

| Model | Meaning | How to enable |
| --- | --- | --- |
| **N-party consensus** | N on-chain identities (wallet, S3, Postgres). Gateway `NUM_COPROCESSORS=N`. | `nb_coprocessor` in `{1,2,3,5}` |
| **Blue-green (RFC-021)** | **Two fleets of the same identity**: BCS (live `v0.14.0-7`) + GCS (HEAD at its compiled release), shared DB/S3/wallet. Cutover is `ProtocolConfig.proposeCoprocessorUpgrade` then off-chain unanimity of all N operators. | PR label `preview-env-blue-green` (deploys, forces N=2) or dispatch `enable_blue_green=true` |

Blue-green does **not** register 2N gateway slots. Per party the preview keeps one listener, one Redis, one poller; BCS and GCS `hostListenerConsumer`s share that broker. GCS also runs `upgrade-controller` and `consensus-detector`. Incompatible with `deploy_polygon` (hence with `chain_mode=testnets`) and with `nb_coprocessor=1`.

With `automated_tests` (or the `preview-env-e2e-tests` label) the cutover is
driven by in-window e2e traffic: propose **after** the relayer is ready, hold
`consensus-detector` at 0 so unanimity cannot fire mid-suite, run the e2e DAG
while GCS is in `DryRunStarted` (CI asserts `"gcs-<version>".computations > 0`
on every party), scale the detector back up, wait for
`versioning=v0.15`, then run the same DAG again on green. A deploy without
auto-tests still proposes and waits for `DryRunStarted` only.

## Multichain: second Polygon host chain (`deploy_polygon`)

`preview-env-deploy.yml` takes a `deploy_polygon` input (default `false`) that adds a
**second host chain — Polygon Amoy (chainId `80002`)** alongside the ETH one. It is a
fresh local `anvil` (**not** a fork of live Amoy, and no `--fork`): nothing in the stack
depends on Polygon consensus, only on a standard EVM JSON-RPC/WS endpoint, so a plain
anvil with `--chain-id 80002` is indistinguishable to every fhevm component.

Polygon **reuses the ETH-activated KMS key** — there is no second keygen ceremony:

- `host-contracts-polygon` deploys with `--with-kms-generation false
  --protocol-config-source canonical`, which **mirrors** the ETH ProtocolConfig (its
  active KMS context/key) onto Polygon (`values-host-contracts-polygon-e2e.yaml`). So
  it must run *after* the ETH host-contracts (the canonical source).
- The coprocessor becomes multichain via **additive overlays**
  (`values-coprocessor-polygon-e2e.yaml` / `-poller-polygon-e2e.yaml`) that add a
  `polygon` `chains[]` entry (a `host-listener-consumer` + poller keyed to `80002`,
  `useLegacyName: false` so names don't collide with the ETH `host` entry). They set
  `canonicalProtocolConfigChainId: "12345"` so Polygon defers to ETH's key. The
  coprocessor DB (`keys`/`crs`) is shared, so the ETH keygen already filled it.
- Per party: a `listener-polygon-<i>` producer (own cursor DB
  `postgres-listener-polygon-<i>`) publishes chain-`80002` events to the **same**
  per-party Redis; the Polygon consumer filters them out by `--chain-id`.
- The relayer gets a second `host_chains` entry and the kms-connector a second
  `hostChains` entry (`values-kms-connector-polygon-e2e.yaml`) so host ACL checks cover
  Polygon ciphertexts.
- Chain `80002` is registered into the shared GatewayConfig by a second
  `addHostChainsToGatewayConfig` call (`values-gateway-add-host-chains-polygon-e2e.yaml`,
  additive, doesn't disturb the ETH registration).

These overlays live in **separate files** (not the base values) on purpose: the
coprocessor `dbMigration` Job renders `HOST_CHAIN_<i>_ACL` for every `chains[]` entry
regardless of whether its consumer is enabled, so a Polygon entry pointing at the
`polygon-sc-addresses` ConfigMap would crash the migration whenever Polygon isn't
deployed. Every Polygon step in the workflow is gated on `deploy_polygon == 'true'`.

> Coverage nuance: the Polygon e2e run uses Hardhat network `polygonAmoy`, which **is**
> in `LIVE_NETWORKS` (see the network-mode section above), so `isLiveNetwork()` is
> `true` and it takes the **reduced / read-only** path (the `hcu-block-cap` owner-only
> and `evm_*` deterministic subtests `this.skip()`). The Polygon run is a multichain
> routing smoke test; the ETH `staging` run remains the full-coverage one.

## TODO / remaining work

- ~~Add multi-coprocessor support~~ — done, see "Multi-coprocessor (`nb_coprocessor`) and
  shared Redis" above. Note the current implementation deploys **full independent stacks**
  per party (workers *and* `*Listener` components, each with its own Postgres/S3/identity),
  rather than scaling only the workers behind shared listeners. Revisit against how
  devnet/testnet actually scale this (shared vs per-party listeners) if that topology is
  preferred.
- ~~Add support for changing the dedicated KMS's instance type~~ — version/repo
  override via `kms_core_version` + `kms_repo_ref` in `overrides` (see
  "Dedicated KMS version pins" above). Enclave **instance type** is still
  whatever kms `deploy.sh` picks for `aws-ci`.
- Add support for changing the coprocessor's tfhe-worker instance type (e.g. GPU vs CPU nodepool
  selection).
- ~~Add multichain support~~ — done, see "Multichain: second Polygon host chain
  (`deploy_polygon`)" above (opt-in; ETH + Polygon Amoy sharing one KMS key).
- ~~Deploy against real public testnets~~ — done, see "Chain modes" above
  (`chain_mode=testnets`: Sepolia + Amoy with RPC URLs + funder key from AWS Secrets Manager, Nitro gateway).
- Wire RFC-021 blue-green onto `chain_mode=testnets`: `propose-coprocessor-upgrade.sh` /
  `assert-gcs-dry-run.sh` are single-chain (scalar `upgrade_state` reads, one-element
  windows array), which is what keeps `enable_blue_green` rejected there today. This is
  the multi-chain shape fhevm-internal#1884 asks for.
