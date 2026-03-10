# fhevm-cli

`fhevm-cli` is the local orchestration entrypoint for the fhEVM test stack.

It exists for three workflows:

- run a known stack target locally
- swap in local changes for one repo-owned group
- run multicopro topologies with deterministic generated state

The CLI owns all mutable runtime state under `.fhevm/`. Tracked compose and env files stay as templates.

For the boot flow diagram and invariants, see [ARCHITECTURE.md](/Users/work/.codex/worktrees/75e6/fhevm/test-suite/fhevm/ARCHITECTURE.md).

## Quick Start

Run from [/Users/work/.codex/worktrees/75e6/fhevm/test-suite/fhevm](/Users/work/.codex/worktrees/75e6/fhevm/test-suite/fhevm):

```sh
bun install
bun run check
bun test
./fhevm-cli up --target latest-release --dry-run
./fhevm-cli up --target latest-release
./fhevm-cli test erc20
./fhevm-cli clean --images
```

## Mental Model

- `up` resolves a target bundle, runs preflight, generates `.fhevm`, and boots the stack
- `up --dry-run` runs the same resolve and preflight path without mutating runtime state
- `test` runs against the current stack; it does not recompile contracts
- `down` stops the stack
- `clean` removes CLI-owned runtime state
- `clean --images` also removes CLI-owned local override images

## Targets

- `latest-release`: latest stable fhevm release plus checked-in companion defaults
- `latest-main`: newest complete repo-owned main SHA bundle plus checked-in companion defaults
- `devnet`
- `testnet`
- `mainnet`

Only `devnet`, `testnet`, and `mainnet` resolve from GitOps. Non-network targets do not.

## Pinning an Exact Version Bundle

If you need to run a specific set of versions (e.g., `v0.10.7` across the board), use `--lock-file`
to skip all target resolution and supply the full bundle yourself:

```sh
./fhevm-cli up --target latest-release --lock-file ./my-bundle.json
```

The lock file must contain every version key. Example:

```json
{
  "target": "latest-release",
  "lockName": "pinned-v0.10.7.json",
  "sources": ["manual"],
  "env": {
    "GATEWAY_VERSION": "v0.10.7",
    "HOST_VERSION": "v0.10.7",
    "COPROCESSOR_DB_MIGRATION_VERSION": "v0.10.7",
    "COPROCESSOR_HOST_LISTENER_VERSION": "v0.10.7",
    "COPROCESSOR_GW_LISTENER_VERSION": "v0.10.7",
    "COPROCESSOR_TX_SENDER_VERSION": "v0.10.7",
    "COPROCESSOR_TFHE_WORKER_VERSION": "v0.10.7",
    "COPROCESSOR_ZKPROOF_WORKER_VERSION": "v0.10.7",
    "COPROCESSOR_SNS_WORKER_VERSION": "v0.10.7",
    "CONNECTOR_DB_MIGRATION_VERSION": "v0.10.7",
    "CONNECTOR_GW_LISTENER_VERSION": "v0.10.7",
    "CONNECTOR_KMS_WORKER_VERSION": "v0.10.7",
    "CONNECTOR_TX_SENDER_VERSION": "v0.10.7",
    "CORE_VERSION": "v0.13.0",
    "RELAYER_VERSION": "v0.9.0",
    "RELAYER_MIGRATE_VERSION": "v0.9.0",
    "TEST_SUITE_VERSION": "v0.10.7"
  }
}
```

The `--target` flag still determines which compat policy applies. The lock file replaces only
the version resolution step — preflight, boot pipeline, and everything else run normally.

## Version Override via Environment Variables

After resolving a target bundle, the CLI applies **environment variable overrides**: any
`*_VERSION` env var that matches a key in the resolved bundle replaces that version.

This is how CI works. The merge queue workflow:

1. Builds Docker images tagged with the PR's HEAD SHA (e.g., `abc1234`)
2. Sets env vars like `COPROCESSOR_HOST_LISTENER_VERSION=abc1234`
3. Runs `./fhevm-cli up --target latest-release`

The CLI resolves `latest-release` as the baseline (providing companion versions like
`CORE_VERSION` and `RELAYER_VERSION` that aren't built from this repo), then overlays the
SHA-tagged env vars for every component that was built from the PR.

Supported override keys (any subset):

```
GATEWAY_VERSION
HOST_VERSION
COPROCESSOR_DB_MIGRATION_VERSION
COPROCESSOR_HOST_LISTENER_VERSION
COPROCESSOR_GW_LISTENER_VERSION
COPROCESSOR_TX_SENDER_VERSION
COPROCESSOR_TFHE_WORKER_VERSION
COPROCESSOR_ZKPROOF_WORKER_VERSION
COPROCESSOR_SNS_WORKER_VERSION
CONNECTOR_DB_MIGRATION_VERSION
CONNECTOR_GW_LISTENER_VERSION
CONNECTOR_KMS_WORKER_VERSION
CONNECTOR_TX_SENDER_VERSION
CORE_VERSION
RELAYER_VERSION
RELAYER_MIGRATE_VERSION
TEST_SUITE_VERSION
```

Example — test a local coprocessor image without `--override`:

```sh
COPROCESSOR_HOST_LISTENER_VERSION=abc1234 \
COPROCESSOR_TFHE_WORKER_VERSION=abc1234 \
  ./fhevm-cli up --target latest-release
```

The resolved lock file records which keys were overridden in its `sources` field.

## Main Commands

```sh
./fhevm-cli up --target latest-release
./fhevm-cli up --target latest-release --resume --from-step relayer
./fhevm-cli up --target latest-release --override coprocessor --profile local
./fhevm-cli up --target latest-release --coprocessors 2 --threshold 2

./fhevm-cli status
./fhevm-cli logs relayer
./fhevm-cli test input-proof
./fhevm-cli test erc20

./fhevm-cli down
./fhevm-cli clean --images
```

## Local Overrides

Use `--override` to run local code for one repo-owned group on top of an otherwise versioned stack.

Supported groups:

- `coprocessor`
- `kms-connector`
- `gateway-contracts`
- `host-contracts`
- `test-suite`

### Override an entire group

```sh
./fhevm-cli up --target latest-release --override coprocessor --profile local
```

### Override specific services

To build only specific services locally while pulling the rest from the registry:

```sh
./fhevm-cli up --target latest-release --override coprocessor:host-listener,tfhe-worker --profile local
```

Use the short service suffix after the colon (e.g., `host-listener` not `coprocessor-host-listener`).
Multiple services are comma-separated. Services that share a Docker image are automatically
co-selected (e.g., `host-listener` includes `host-listener-poller`).

> **Note:** `coprocessor` and `kms-connector` services share a database. Per-service overrides
> work when your local changes don't include DB migrations. If your changes add or alter
> migrations, non-overridden services will fail against the mismatched schema — use
> `--override coprocessor` (full group) instead.

Available suffixes per group:

| Group | Suffixes |
|-------|----------|
| `coprocessor` | `db-migration`, `host-listener`, `host-listener-poller`, `gw-listener`, `tfhe-worker`, `zkproof-worker`, `sns-worker`, `transaction-sender` |
| `kms-connector` | `db-migration`, `gw-listener`, `kms-worker`, `tx-sender` |
| `gateway-contracts` | `deploy-mocked-zama-oft`, `set-relayer-mocked-payment`, `sc-deploy`, `sc-add-network`, `sc-add-pausers`, `sc-trigger-keygen`, `sc-trigger-crsgen` |
| `host-contracts` | `sc-deploy`, `sc-add-pausers` |
| `test-suite` | `e2e-debug` |

### Multiple overrides

Repeat `--override` to override several groups at once:

```sh
# Two full groups
./fhevm-cli up --target latest-release --override coprocessor --override gateway-contracts --profile local

# Per-service across groups
./fhevm-cli up --target latest-release --override coprocessor:host-listener --override gateway-contracts:sc-deploy --profile local

# Mixed: per-service + full group
./fhevm-cli up --target latest-release --override coprocessor:host-listener --override kms-connector --profile local
```

The `--profile` flag applies to every override that doesn't specify its own profile.

### Combining with env var overrides

You can mix per-service local builds with registry tag overrides:

```sh
COPROCESSOR_GW_LISTENER_VERSION=abc1234 \
  ./fhevm-cli up --target latest-release --override coprocessor:host-listener --profile local
```

This builds `host-listener` (and `host-listener-poller`) locally, pulls `gw-listener` at tag
`abc1234`, and pulls all other coprocessor services at the resolved target version.

## Multicopro

Example:

```sh
./fhevm-cli up \
  --target latest-release \
  --coprocessors 2 \
  --threshold 2 \
  --instance-env 1:OTEL_SERVICE_NAME=coprocessor-1-local \
  --instance-arg '1:tfhe-worker=--coprocessor-fhe-threads=4'
```

Generated env, compose overlays, addresses, locks, and state all live under `.fhevm/`.

## Runtime State

The CLI owns:

- `.fhevm/state.json`
- `.fhevm/locks/`
- `.fhevm/env/`
- `.fhevm/compose/`
- `.fhevm/addresses/`

`status` shows the active stack state and any CLI-owned local build images.
