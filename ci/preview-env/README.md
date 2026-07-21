# fhevm ephemeral PR-preview e2e config

> **New here?** Read [`101-preview-env.md`](./101-preview-env.md) first — a short,
> practical guide to launching a preview from a PR label or a manual dispatch.
> This README covers the internals (what is deployed and why).

Config for the per-PR e2e preview environment, additive to the default docker-compose path
driven by `test-suite/fhevm` (`fhevm-cli`). Mirrors the layout and conventions of
[`zama-ai/kms/ci/kube-testing`](https://github.com/zama-ai/kms/tree/main/ci/kube-testing) +
[`zama-ai/kms/ci/scripts`](https://github.com/zama-ai/kms/tree/main/ci/scripts).

Every PR that carries the `pr-preview-e2e` label gets a per-PR namespace
(`fhevm-ci-<actor>-<pr>`) on the real `zws-dev` Tailscale cluster — real published OCI charts,
a Crossplane-provisioned S3 bucket (`coprocessor-infra/`), in-cluster Postgres (official
`postgres:17-alpine` image via the generic `common` chart, one dedicated instance each for
coprocessor, listener, kms-connector, and relayer/relayer-migrate — see `coprocessor-infra/values-postgres-
coprocessor-e2e.yaml`'s header for why this isn't RDS, or Bitnami's postgresql chart), and a
dedicated real 4-party threshold+enclave KMS reused directly from `zama-ai/kms`'s own
`ci/scripts/deploy.sh` (no vendored `kms-core` chart usage here at all). See
[`../../.github/workflows/pr-preview-deploy.yml`](../../.github/workflows/pr-preview-deploy.yml).
Torn down automatically when the PR closes
([`pr-preview-destroy.yml`](../../.github/workflows/pr-preview-destroy.yml)).

There is no local Kind/laptop-based variant of this path anymore — a full stack (dedicated
4-party enclave KMS + coprocessor + kms-connector + relayer + test-suite) doesn't fit in a
reasonable local resource budget, so `pr-preview-deploy.yml`'s remote `zws-dev` cluster is the
only supported way to run this. The default docker-compose path (`test-suite/fhevm`) remains
the right choice for local iteration.

## Layout

```
ci/preview-env/
├── coprocessor-infra/
│   ├── values-coprocessor-infra-e2e.yaml    # crossplane/coprocessor-infra overlay: S3 only
│   └── values-postgres-coprocessor-e2e.yaml # `common` chart overlay: in-cluster Postgres, dedicated to coprocessor
├── host-chain/
│   ├── values-anvil-host-e2e.yaml       # anvil-node overlay, host chain
│   ├── values-host-contracts-e2e.yaml   # contracts overlay, host-contracts
│   └── values-host-trigger-keygen-e2e.yaml # contracts overlay, real FHE key/CRS gen ceremony
├── gateway-chain/
│   ├── values-anvil-gateway-e2e.yaml         # anvil-node overlay, gateway chain
│   ├── values-gateway-contracts-e2e.yaml     # contracts overlay, gateway-contracts
│   └── values-gateway-add-host-chains-e2e.yaml # contracts overlay, deferred addHostChains step
├── coprocessor/
│   ├── values-coprocessor-e2e.yaml        # coprocessor overlay (one release per party: coprocessor-<i>)
│   ├── values-coprocessor-poller-e2e.yaml # coprocessor overlay, poller-only release: S3 key/CRS download -> keys/crs tables (coprocessor-poller-<i>)
│   └── values-coprocessor-redis-e2e.yaml  # iamguarded Redis overlay: per-party host-listener-consumer broker (coprocessor-redis-<i>)
├── listener/
│   ├── values-listener-e2e.yaml          # listener chart overlay: per-party host-chain event producer (listener-<i>)
│   └── values-postgres-listener-e2e.yaml # `common` chart overlay: in-cluster Postgres, dedicated to the listener's cursor DB
├── kms-connector/
│   ├── values-kms-connector-e2e.yaml       # kms-connector overlay
│   └── values-postgres-connector-e2e.yaml  # `common` chart overlay: in-cluster Postgres, dedicated to kms-connector
├── relayer/
│   ├── values-relayer-e2e.yaml          # `common` chart overlay, relayer server
│   ├── values-relayer-migrate-e2e.yaml  # `common` chart overlay, relayer DB migration Job
│   └── values-postgres-relayer-e2e.yaml # `common` chart overlay: in-cluster Postgres, dedicated to relayer + relayer-migrate
└── test-suite/
    └── values-test-suite-e2e.yaml       # `common` chart overlay, e2e test-suite Job
```

Every file here is a **values overlay for a chart**. `anvil-node`/`contracts`/`coprocessor`/
`kms-connector` target the real, published production charts directly via
`oci://hub.zama.org/ghcr/zama-ai/fhevm/charts/*` refs (see `pr-preview-deploy.yml`).
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
CI path reuses `zama-ai/kms`'s own deploy pipeline as-is (see `pr-preview-deploy.yml`).

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
export const isLiveNetwork = () => LIVE_NETWORKS.has(network.name);
```

This preview runs the host chain as **`staging`** (chainId `12345`), which is **not**
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

## Multi-coprocessor (`nb_coprocessor`) and shared Redis

`pr-preview-deploy.yml` takes an `nb_coprocessor` input (default `1`, mirroring
`NB_KMS_CORE`) that deploys **N independent coprocessor stacks**. For party `i` (1..N):

- its own in-cluster Postgres `postgres-coprocessor-<i>`,
- its own `coprocessor-infra-<i>` release → dedicated S3 bucket, IRSA ServiceAccount
  `coprocessor-<i>`, and bucket ConfigMap `coprocessor-<i>` (holding `S3_BUCKET_NAME`),
- its own tx-sender/signer identity (derived from the shared Foundry/Hardhat mnemonic
  at HD indices `20..20+N-1`, chosen to avoid the test signers `#0-4`, host owner `#9`,
  and KMS tx-senders `#10..`), reused for both the on-chain registration and the
  `txSender` wallet,
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

> This producer path is newer to the preview than the old DB-only `hostListener`. Three
> things to validate on the first real multi-run (adjust the preview-env values, not the
> charts): the `listener-core` image tag (`LISTENER_VERSION`) must be broker-payload
> compatible with `COPROCESSOR_VERSION`'s `host_listener_consumer`; anvil must serve the
> `block_receipts` RPC (`eth_getBlockReceipts`); and the broker topic keying (chain id
> `12345`) must match the consumer's `--chain-id`.

> Resource caveat: each coprocessor party's `tfhe`/`sns` workers request substantial
> CPU/memory on the `coprocessor` nodepool, so `nb_coprocessor` > 1 multiplies the
> cluster capacity needed. Default stays `1`.

## TODO / remaining work

- ~~Add multi-coprocessor support~~ — done, see "Multi-coprocessor (`nb_coprocessor`) and
  shared Redis" above. Note the current implementation deploys **full independent stacks**
  per party (workers *and* `*Listener` components, each with its own Postgres/S3/identity),
  rather than scaling only the workers behind shared listeners. Revisit against how
  devnet/testnet actually scale this (shared vs per-party listeners) if that topology is
  preferred.
- Add support for changing the dedicated KMS's instance type (currently whatever
  `zama-ai/kms`'s own `ci/scripts/deploy.sh` defaults to).
- Add support for changing the coprocessor's tfhe-worker instance type (e.g. GPU vs CPU nodepool
  selection).
- Add multichain support
