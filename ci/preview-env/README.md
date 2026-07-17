# fhevm ephemeral PR-preview e2e config

Config for the per-PR e2e preview environment, additive to the default docker-compose path
driven by `test-suite/fhevm` (`fhevm-cli`). Mirrors the layout and conventions of
[`zama-ai/kms/ci/kube-testing`](https://github.com/zama-ai/kms/tree/main/ci/kube-testing) +
[`zama-ai/kms/ci/scripts`](https://github.com/zama-ai/kms/tree/main/ci/scripts).

Every PR that carries the `pr-preview-e2e` label gets a per-PR namespace
(`fhevm-ci-<actor>-<pr>`) on the real `zws-dev` Tailscale cluster — real published OCI charts,
a Crossplane-provisioned S3 bucket (`coprocessor-infra/`), in-cluster Postgres (official
`postgres:17-alpine` image via the generic `common` chart, one dedicated instance each for
coprocessor, kms-connector, and relayer/relayer-migrate — see `coprocessor-infra/values-postgres-
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
│   └── values-coprocessor-e2e.yaml      # coprocessor overlay
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

## TODO / remaining work

- Add multi-coprocessor support (3 or 5 coprocessor instances, each with its own dedicated
  Postgres, mirroring the per-party kms-connector/Postgres pattern above). This is intended for
  the coprocessor *workers* (`snsWorker`/`zkProofWorker`/`tfheWorker`/`txSender`), not the
  `*Listener` components, which stay singletons — to confirm against how devnet/testnet actually
  scale this before implementing.
- Add support for changing the dedicated KMS's instance type (currently whatever
  `zama-ai/kms`'s own `ci/scripts/deploy.sh` defaults to).
- Add support for changing the coprocessor's tfhe-worker instance type (e.g. GPU vs CPU nodepool
  selection).
