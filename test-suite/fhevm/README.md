# fhevm-cli

`fhevm-cli` is the local orchestration entrypoint for the fhEVM test stack.

It exists for three workflows:

- run a known stack target locally
- swap in local changes for one repo-owned group
- run consensus/matrix coprocessor scenarios with deterministic generated state

## Why This CLI Exists

Launching this stack is harder than "run docker compose up".

The CLI has to assemble one runnable stack from components that move at different speeds:

- repo-owned services can be built from `main`, from a specific SHA, from local workspace code, or from a tracked supported profile
- non-repo companions such as relayer and kms-core do not automatically track repo-owned `main`
- some targets are meant to reproduce a known baseline (`latest-supported`, network targets, lock files)
- some targets are meant for active integration work (`latest-main`, `--build`, local overrides)
- coprocessor topology can also change independently through explicit scenarios

That means the hard part is not just booting containers. It is deciding:

1. what base stack you want
2. where repo-owned components should come from
3. what coprocessor topology should run

Examples:

- "I changed host-listener, does my branch still work?" -> `latest-main` + local repo-owned code
- "Does the merge candidate artifact bundle work?" -> `latest-main` + merge-candidate image overrides
- "Do 2 local coprocessors reach consensus?" -> baseline target + explicit scenario

The CLI exists to make those decisions explicit, reproducible, and testable.

The CLI owns all mutable runtime state under `.fhevm/`. Tracked compose and env files stay as templates.

For the boot flow diagram and invariants, see `ARCHITECTURE.md`.

## Default Paths

Most users should start with `latest-main`.

- fastest local iteration: `./fhevm-cli up --target latest-main --override <group>`
- full branch validation: `./fhevm-cli up --target latest-main --build`
- PR e2e: `latest-main --build` + the checked-in `two-of-two` scenario + `test light`
- merge queue: `latest-main` baseline + repo-owned image overrides for components that were actually rebuilt

Use `latest-supported`, network targets, or `sha` when you are reproducing a known supported or deployed bundle rather than validating current mainline behavior.

Compat is mainly there to protect those reproduction and cross-era paths. For the common `latest-main` path, the mental model should stay simple: mainline baseline, optional surgical local or CI repo-owned overrides, explicit topology when needed. For the shim/incompatibility decision tree, see `COMPAT.md`.

## Quick Start

Run from `test-suite/fhevm`:

```sh
bun install
bun run check
bun test
./fhevm-cli up --target latest-supported --dry-run
./fhevm-cli up --target latest-supported
./fhevm-cli up --target latest-main --build --dry-run
./fhevm-cli test erc20
./fhevm-cli clean --images
```

## Mental Model

- `up` resolves a target bundle, runs preflight, generates `.fhevm`, and boots the stack
- `up --dry-run` runs the same resolve and preflight path without mutating runtime state
- `up --scenario <name-or-file>` applies an explicit coprocessor consensus scenario on top of the resolved bundle
- `up --override coprocessor` is the fast local-dev shorthand for a one-instance local coprocessor scenario
- `scenario list` prints the bundled scenario presets with their intent
- `test` runs against the current stack; it does not recompile contracts. `--parallel` runs tests in parallel (auto for `operators`). `test light` runs the non-heavy PR e2e suite in one command and skips `ciphertext-drift` unless the current topology supports it
- `logs` follows container output; `--no-follow` prints the tail and exits
- `pause` / `unpause` pauses or unpauses host or gateway contracts
- `down` stops the stack, prunes `.fhevm/runtime`, and keeps resumable `.fhevm/state`
- `clean` removes CLI-owned runtime state
- `clean --images` also removes CLI-owned local override images

## Ownership Model

There are four kinds of inputs/runtime artifacts:

- tracked compose templates: `docker-compose/*.yml`
- tracked env templates: `templates/env/.env.*`
- tracked config:
  - relayer template input: `templates/config/relayer.yaml`
  - static mounted config: `static/config/kms-core/config.toml`, `static/config/prometheus/prometheus.yml`
- checked-in scenario inputs under `scenarios/` (currently `two-of-two.yaml`)

Generated runtime artifacts always live under `.fhevm/`:

- `.fhevm/runtime/env/*.env`
- `.fhevm/runtime/compose/*.yml` for generated runtime overrides only
- `.fhevm/runtime/config/relayer.yaml`
- `.fhevm/runtime/addresses/*`
- `.fhevm/state/locks/*`
- `.fhevm/state/state.json`

Tracked compose files are the default runtime truth. `.fhevm/runtime/compose` only holds generated overrides when runtime structure or local-image policy actually changes, with coprocessor topology as the only structural expansion.

The code follows the same split:

- `src/runtime-plan.ts`: resolve one runtime plan from bundle + env overrides + scenario/shorthand
- `src/render-env.ts`: render runtime env maps
- `src/render-config.ts`: render generated config files
- `src/render-compose.ts`: render compose overlays, with coprocessor topology as the only structural exception

## Resolution Order

Runtime resolution is intentionally fixed:

1. Resolve the base bundle from `--target`, `--sha`, or `--lock-file`
2. Apply matching `*_VERSION` environment overrides
3. Apply either `--scenario <name-or-file>` or the `--override coprocessor` shorthand
4. Materialize generated env/config/compose state under `.fhevm/`

## Targets

- `latest-supported`: tracked maintained bundle profile (`profiles/latest-supported.json`)
- `latest-main`: newest complete repo-owned main SHA bundle at or after the simple-ACL floor (`803f104`)
- `sha`: exact repo-owned SHA bundle plus `latest-supported` companions
- `devnet`
- `testnet`
- `mainnet`

Only `devnet`, `testnet`, and `mainnet` resolve from GitOps today. Non-network targets do not.
`latest-main` is intentionally modern-only; if the resolver cannot find a complete image set after the floor, it fails instead of walking into older protocol behavior.
`sha` requires `--sha <git-sha>` and fails fast unless every repo-owned package is available at that 7-character SHA tag, the SHA is on `main`, and it is at or after the simple-ACL floor.

## Pinning an Exact Version Bundle

If you need to run a specific set of versions (e.g., `v0.10.7` across the board), use `--lock-file`
to skip all target resolution, avoid GitHub lookups, and supply the full bundle yourself:

```sh
./fhevm-cli up --lock-file ./my-bundle.json
```

The lock file must contain every version key. Example:

```json
{
  "target": "latest-supported",
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

If you also pass `--target`, it must match the lock file. Otherwise the CLI infers the target from the lock file itself.
The lock file replaces only the version resolution step — preflight, boot pipeline, and everything else run normally.

## Version Override via Environment Variables

After resolving a target bundle, the CLI applies **environment variable overrides**: any
`*_VERSION` env var that matches a key in the resolved bundle replaces that version.

This is how CI works. The merge queue workflow:

1. Builds repo-owned Docker images for touched components
2. Sets `*_VERSION=<head-sha-short>` only for repo-owned components whose build succeeded
3. Leaves skipped components unset so they naturally stay on the `latest-main` baseline
4. Fails if a required touched-component build reports a non-skipped failure
5. Runs `./fhevm-cli up --target latest-main --scenario two-of-two`
6. Passes `build=false` explicitly because merge queue is validating selected registry images, while direct PR e2e uses `build=true`

The CLI resolves `latest-main` as the current mainline bundle, then overlays the
merge-candidate SHA-tagged env vars for every repo-owned component.
For non-workspace companions, `latest-main` uses the maintained mainline baseline from `src/presets.ts`.
The reusable workflow now runs on `pull_request` directly and treats PR e2e as source validation with `build=true`.
Orchestrate passes `build=false` explicitly because it is validating selected registry images on top of the `latest-main` baseline rather than rebuilding from source.

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
  ./fhevm-cli up --target latest-main
```

The resolved lock file records which keys were overridden in its `sources` field.

If you already know the exact repo SHA you want and all fhevm images were published with that tag:

```sh
./fhevm-cli up --target sha --sha 9587546
./fhevm-cli up --target sha --sha 9587546 --dry-run
```

This resolves every repo-owned image to `9587546` and keeps companion services (`core`, `relayer`, `relayer-migrate`) on the maintained non-network companion set used by `latest-main`.

## Compatibility Matrix

All version compatibility rules live in a single source of truth: `src/compat.ts` → `COMPAT_MATRIX`.

The matrix has three sections:

| Section | Purpose | Example |
|---------|---------|---------|
| `incompatibilities` | Version pairs that break at runtime | relayer v1 + test-suite v2 |
| `legacyShims` | Old versions needing extra flags/env | coprocessor < 0.12.0 needs API key flags |
| `anchors` | Git history reference points | simple-ACL cutover commit |

Merge-queue e2e always boots `latest-main` with the fixed `two-of-two` scenario, injects the PR head tag only for repo-owned images whose build succeeded, leaves skipped components on the `latest-main` baseline, and explicitly keeps `build=false`.

### How to update

**Bump the mainline relayer pin:**
Edit `MAINLINE_COMPANIONS` in `src/presets.ts`. `latest-main` and `sha` pick it up automatically.

**Add a new incompatibility:**
Add an entry to `COMPAT_MATRIX.incompatibilities` with a unique `code`. The CLI validates all entries at boot.

**Add a legacy shim for a breaking change:**
1. Add a profile to `SHIM_PROFILES` describing the legacy flags/env
2. Add an entry to `COMPAT_MATRIX.legacyShims` specifying which version key and threshold
3. Run `bun test` to verify

**Remove a legacy shim:**
When the minimum supported version passes the threshold, delete the `legacyShims` entry and its `SHIM_PROFILES` profile. Run `bun test`.

### Maintenance caveats

The CLI is leaner than the old bash path, but a few files still carry most of the maintenance burden:

- `src/presets.ts`: maintained companion pins for `latest-main` and `sha`
- `src/resolve.ts`: support floors and target-resolution policy
- `src/compat.ts`: legacy shims and explicit incompatibility rules
- `src/render-env.ts`: runtime env projection from templates, discovery, topology, and compat
- `src/render-compose.ts`: service command shaping, local-build rewrites, and scenario instance compose overrides

When changing runtime flags, env contracts, target semantics, or external companion versions, assume you may need to touch more than one of those files. The expected checks are:

1. update the resolution or compat rule
2. run `bun test`
3. run `bun run compat-smoke` if the change affects legacy runtime contracts

## Main Commands

```sh
./fhevm-cli up --target latest-supported
./fhevm-cli deploy --target latest-supported
./fhevm-cli up --target sha --sha 9587546
./fhevm-cli up --target latest-supported --resume --from-step relayer
./fhevm-cli up --target latest-main --build
./fhevm-cli up --target latest-main --scenario two-of-two --build
./fhevm-cli up --target latest-supported --override coprocessor
./fhevm-cli up --target latest-supported --scenario two-of-two
./fhevm-cli scenario list
./fhevm-cli upgrade coprocessor

./fhevm-cli status
./fhevm-cli logs relayer
./fhevm-cli logs --no-follow relayer
./fhevm-cli test input-proof
./fhevm-cli test erc20
./fhevm-cli test light
./fhevm-cli test operators
./fhevm-cli test --parallel --grep "test operator" --verbose
./fhevm-cli pause host
./fhevm-cli unpause host

./fhevm-cli down
./fhevm-cli clean --images
```

## Local Overrides

Use `--override` to run local code for one repo-owned group on top of an otherwise versioned stack.

Use `--build` when you want the whole local workspace on the active baseline. On topology-only scenario runs, `--build` also applies local coprocessor images to inherited scenario instances. If a scenario explicitly pins coprocessor source, overlapping explicit coprocessor overrides fail fast instead.
`--build` cannot be combined with `--override`.

Supported groups:

- `coprocessor`
- `kms-connector`
- `gateway-contracts`
- `host-contracts`
- `test-suite`

### Override an entire group

```sh
./fhevm-cli up --target latest-supported --override coprocessor
```

For `coprocessor`, this is also the shorthand local-dev scenario: one coprocessor instance, threshold `1`, source mode `local`.

### Build the local workspace

```sh
./fhevm-cli up --target latest-main --build
./fhevm-cli up --target latest-main --scenario two-of-two --build
```

### Override specific runtime services

Runtime override groups also support per-service filtering:

Per-service override syntax is supported only for `coprocessor`, `kms-connector`, and `test-suite`.
Use the short service suffix after the group prefix. Multiple services are comma-separated. Services that share the same image are auto-selected together, so `coprocessor:host-listener` also builds `host-listener-poller` locally.
Local overrides always build workspace images while non-overridden services stay on the resolved bundle.

`coprocessor` and `kms-connector` still share a database, so the CLI warns when you do a per-service override there. If your change includes schema or migration changes, use the full-group override instead.
On `latest-supported`, the CLI now compares the local migration directory against the tracked baseline profile and rejects a per-service override by default when they diverge. If you know your service remains compatible anyway, pass `--allow-schema-mismatch`.

Example on a mainline baseline:

```sh
./fhevm-cli up --target latest-main --override coprocessor:host-listener,tfhe-worker
```

Available runtime suffixes:

| Group | Suffixes |
|-------|----------|
| `coprocessor` | `db-migration`, `host-listener`, `host-listener-poller`, `gw-listener`, `tfhe-worker`, `zkproof-worker`, `sns-worker`, `transaction-sender` |
| `kms-connector` | `db-migration`, `gw-listener`, `kms-worker`, `tx-sender` |
| `test-suite` | `e2e-debug` |

### Multiple overrides

Repeat `--override` to override several groups at once:

```sh
# Two full groups
./fhevm-cli up --target latest-supported --override coprocessor --override gateway-contracts

# Per-service across runtime groups
./fhevm-cli up --target latest-supported --override coprocessor:host-listener --override kms-connector:gw-listener

# Mixed: per-service + full group
./fhevm-cli up --target latest-supported --override coprocessor:host-listener --override gateway-contracts
```

### Combining with env var overrides

You can mix per-service local builds with registry tag overrides:

```sh
COPROCESSOR_GW_LISTENER_VERSION=abc1234 \
  ./fhevm-cli up --target latest-supported --override coprocessor:host-listener
```

This builds `host-listener` (and `host-listener-poller`) locally, pulls `gw-listener` at tag
`abc1234`, and pulls all other coprocessor services at the resolved target version.

If you intentionally want to bypass the latest-supported migration guard:

```sh
./fhevm-cli up --target latest-supported --override coprocessor:host-listener --allow-schema-mismatch
```

If a runtime override is already active and you only want to rebuild and restart that local code path, use:

```sh
./fhevm-cli upgrade coprocessor
```

`upgrade` only supports active runtime override groups: `coprocessor`, `kms-connector`, and `test-suite`. It is a runtime rebuild/restart command, not a live schema migration command. For schema-coupled groups (`coprocessor`, `kms-connector`), if local DB migrations changed, `upgrade` fails fast and asks you to do a fresh `fhevm-cli up` instead of rerunning the initializer on a live database.

## Dropped Convenience Commands

- `smoke`: use explicit `up ...` plus `test ...`
- `test debug`: use `docker exec -it fhevm-test-suite-e2e-debug sh`

## Coprocessor Scenarios

Use `--scenario <name-or-file>` for consensus and rollout matrices. Bundled presets resolve by filename stem, and explicit file paths still work. The scenario file is the source of truth for:

- coprocessor count and threshold
- per-instance source mode: `inherit`, `registry`, or `local`
- per-instance env overrides
- per-instance runtime args
- optional `localServices` for local instances when only part of one coprocessor instance should be built from the workspace

Examples:

```sh
./fhevm-cli scenario list
./fhevm-cli up --target latest-supported --scenario two-of-two
```

Selective local instance example:

```yaml
version: 1
kind: coprocessor-consensus
topology:
  count: 2
  threshold: 2
instances:
  - index: 1
    source:
      mode: local
    localServices:
      - host-listener
```

That keeps the scenario explicit while limiting the local build to `host-listener` and its required sibling services for that one instance.

`--scenario` can be combined with `--override coprocessor` as long as the scenario only defines topology/env/args and leaves coprocessor source inherited. If the scenario explicitly pins coprocessor source (for example with `source.mode=local` or `source.mode=registry`), overlapping `--override coprocessor...` inputs fail fast.

## Runtime State

The CLI owns:

- `.fhevm/state/state.json`
- `.fhevm/state/locks/`
- `.fhevm/runtime/env/`
- `.fhevm/runtime/compose/`
- `.fhevm/runtime/addresses/`

`status` shows the active stack state, the active scenario origin when present, and any CLI-owned local build images.
