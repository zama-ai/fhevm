# fhevm-cli

`fhevm-cli` is the local orchestration entrypoint for the fhEVM test stack.

It exists for three workflows:

- run a known stack target locally
- swap in local changes for one repo-owned group
- run consensus/matrix coprocessor scenarios with deterministic generated state

Main flow:

- read flags and reject invalid combinations
- decide which versions to use
- decide which stack shape to use
- load or create saved local state
- generate env/config/compose files
- run startup steps in order
- wait for each part to be actually ready
- discover addresses and bootstrap outputs
- save progress after each completed step
- let later commands reuse that saved state

## Why This CLI Exists

Launching this stack is harder than "run docker compose up".

The CLI has to assemble one runnable stack from components that move at different speeds:

- repo-owned services can be built from `main`, from a specific SHA, from local workspace code, or from a tracked supported profile
- non-repo companions such as kms-core do not automatically track repo-owned `main`
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
- PR e2e: `latest-main --build` + the checked-in `two-of-two` scenario + `test standard`
- merge queue: `latest-main` baseline + repo-owned image overrides for components that were actually rebuilt

Use `latest-supported`, network targets, or `sha` when you are reproducing a known supported or deployed bundle rather than validating current mainline behavior.

Live target resolution uses GitHub metadata. For `latest-main`, `sha`, and network targets, install `gh` and authenticate it with package-read access, for example `gh auth refresh -s read:packages`, or provide a `GH_TOKEN` with that scope.

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
./fhevm-cli clean
```

## Mental Model

- `up` resolves a target bundle, runs preflight, generates `.fhevm`, and boots the stack
- `up --dry-run` runs the same resolve and preflight path without mutating runtime state
- `up --scenario <name-or-file>` applies an explicit coprocessor consensus scenario on top of the resolved bundle
- `up --override coprocessor` is the fast local-dev shorthand for a one-instance local coprocessor scenario
- `scenario list` prints the bundled scenario presets with their intent
- `test` runs against the current stack and may recompile contracts through Hardhat by default. Pass `--no-hardhat-compile` to skip that step. `--parallel` runs tests in parallel (auto for `operators`). `test light` is the tiny smoke lane (`input-proof` + `erc20`), `test standard` runs the default CI lane including db revert and drift, `test multi-chain-isolation` is the dedicated multi-chain coverage lane, and `test heavy` is the operators lane
- `logs` follows container output; `--no-follow` prints the tail and exits
- `pause` / `unpause` pauses or unpauses host or gateway contracts
- `down` stops the stack, prunes `.fhevm/runtime`, and keeps resumable `.fhevm/state`
- `clean` removes CLI-owned runtime state and local override images by default
- `clean` removes CLI-owned local override images by default
- `clean --keep-images` preserves them

## Ownership Model

There are four kinds of inputs/runtime artifacts:

- tracked compose templates: `docker-compose/*.yml`
- tracked env templates: `templates/env/.env.*`
- tracked config:
  - relayer template input: `templates/config/relayer.yaml`
  - template/static config: `templates/config/kms-core-*.toml`, `static/config/prometheus/prometheus.yml`
- checked-in scenario inputs under `scenarios/` (`two-of-two.yaml`, `two-of-two-multi-chain.yaml`, `multi-chain.yaml`)

Generated runtime artifacts always live under `.fhevm/`:

- `.fhevm/runtime/env/*.env`
- `.fhevm/runtime/compose/*.yml` for generated runtime overrides only
- `.fhevm/runtime/config/relayer.yaml`
- `.fhevm/runtime/config/kms-core.toml`
- `.fhevm/runtime/addresses/*`
- `.fhevm/state/locks/*`
- `.fhevm/state/state.json`

Tracked compose files are the default runtime truth. `.fhevm/runtime/compose` only holds generated overrides when runtime structure or local-image policy actually changes, with coprocessor topology as the only structural expansion.

The code follows the same split:

- `src/stack-spec/stack-spec.ts`: resolve one stack spec from bundle + env overrides + scenario/shorthand
- `src/generate/env.ts`: generate runtime env maps
- `src/generate/config.ts`: generate generated config files
- `src/generate/compose.ts`: generate compose overlays, with coprocessor topology as the only structural exception

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
`sha` requires `--sha <git-sha>` and resolves every repo-owned image to that 7-character SHA tag. The CLI does not query GitHub or prove branch ancestry for this target; Docker pull or boot-time validation reports missing images or incompatible stacks.

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
    "LISTENER_CORE_VERSION": "v0.10.7",
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

## Stateful Rollout Runbooks

Release rollouts are executable TypeScript runbooks under `rollouts/`. A runbook boots one baseline stack, performs each upgrade step in order, preserves chain/database/container state, and runs rollout-safe e2e coverage after each step:

```sh
./fhevm-cli rollout run ./rollouts/v0.12-to-v0.13-protocol-upgrade/run.ts
```

Use `./fhevm-cli rollout receipt` to print the markdown receipt of the most recent rollout run.

Runbooks use the same primitives an operator needs during a release:

- `ctx.up(...)` starts the old baseline once.
- `ctx.writeVersionLock(...)` writes explicit version locks from the runbook: every version key comes from the runbook.
- `ctx.resolveVersionLock(...)` writes a lock over the resolved current stack, changing only the keys the runbook names. The surrounding stack is resolved once per run, so every lock derived this way shares one snapshot.
- `ctx.applyVersionLock(...)` applies version changes that do not restart runtime services, then regenerates env/compose.
- `ctx.runHostContractTask(...)` and `ctx.runGatewayContractTask(...)` run contract migration/upgrade tasks from the selected deploy images.
- `ctx.upgradeRuntimeGroup(...)` restarts selected runtime components in place and runs their DB migrations when present.
- `ctx.test(...)` runs the rollout-safe e2e profile after each state.

`rollout-standard` is intentionally narrow: it covers encrypted input, FHE compute/write paths, user decrypt, delegated user decrypt, public decrypt, and ERC20 transfer coverage. Broader profiles such as `multi-chain-isolation`, HCU, pause/unpause, DB revert, and drift recovery stay available as explicit tests but are not part of the per-step rollout gate.

The `test-suite-stateful-rollout` workflow executes a checked-in runbook through manual dispatch with a path relative to `test-suite/fhevm`.

## Version Override via Environment Variables

After resolving a target bundle, the CLI applies **environment variable overrides**: any
`*_VERSION` env var that matches a key in the resolved bundle replaces that version.

This is how CI works. The merge queue workflow:

1. Resolves a frozen baseline lock from `github.event.pull_request.base.sha`
2. Uploads that lock as a workflow artifact
3. Builds repo-owned Docker images for touched components
4. Sets `*_VERSION=<head-sha-short>` only for repo-owned components whose build succeeded
5. Leaves skipped component outputs empty so the reusable e2e workflow keeps the frozen lock value
6. Runs `./fhevm-cli up --lock-file <baseline-lock>` with `two-of-two-multi-chain` for non-release orchestrate and `two-of-two` for `release/*`
7. Passes `build=false` explicitly because merge queue is validating selected registry images, while direct PR e2e uses `build=true`

Orchestrate resolves the baseline once from the PR base SHA, then passes that lock artifact into the reusable e2e workflow. Head-image overrides are applied only for components rebuilt by the PR.
The reusable workflow now runs on `pull_request` directly and treats PR e2e as source validation with `build=true`.
Orchestrate passes `build=false` explicitly because it is validating selected registry images rather than rebuilding from source.

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
LISTENER_CORE_VERSION
CONNECTOR_DB_MIGRATION_VERSION
CONNECTOR_GW_LISTENER_VERSION
CONNECTOR_KMS_WORKER_VERSION
CONNECTOR_TX_SENDER_VERSION
CONNECTOR_ENDPOINT_VERSION   # optional: omitted when the resolved images predate the connector HTTP endpoint
CONNECTOR_PROXY_VERSION      # optional: omitted when the resolved images predate the connector proxy
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

This resolves every repo-owned image to `9587546` and keeps only external companions like `core` on the maintained non-network companion set used by `latest-main`.

## Compatibility Rules

All version compatibility rules live in a single source of truth: `src/compat/compat.ts` → `COMPAT_MATRIX`.

The rules have three sections:

| Section             | Purpose                              | Example                                  |
| ------------------- | ------------------------------------ | ---------------------------------------- |
| `incompatibilities` | Version pairs that break at runtime  | relayer v1 + test-suite v2               |
| `legacyShims`       | Old versions needing extra flags/env | coprocessor < 0.12.0 needs API key flags |
| `anchors`           | Git history reference points         | simple-ACL cutover commit                |

Merge-queue e2e explicitly keeps `build=false`.
For non-release PRs it boots `two-of-two-multi-chain` from the frozen base lock plus any successful head-image overrides.
For `release/*` PRs it boots `two-of-two` from the same frozen-lock model.

### How to update

**Bump the mainline core pin:**
Edit `MAINLINE_COMPANIONS` in `src/resolve/presets.ts`. `latest-main` and `sha` pick it up automatically.

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

- `src/resolve/presets.ts`: maintained non-repo companion pins for `latest-main` and `sha`
- `src/resolve/target.ts`: support floors and target-resolution policy
- `src/compat/compat.ts`: legacy shims and explicit incompatibility rules
- `src/generate/env.ts`: runtime env projection from templates, discovery, topology, and compat
- `src/generate/compose.ts`: service command shaping, local-build rewrites, and scenario instance compose overrides

When changing runtime flags, env contracts, target semantics, or external companion versions, assume you may need to touch more than one of those files. The expected checks are:

1. update the resolution or compat rule
2. run `bun test`
3. run `bun run compat-smoke` if the change affects legacy runtime contracts

## Main Commands

```sh
./fhevm-cli up --target latest-supported
./fhevm-cli deploy --target latest-supported
./fhevm-cli up --target sha --sha 9587546
./fhevm-cli up --resume --from-step relayer
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
./fhevm-cli test erc1271-user-decryption
./fhevm-cli test unified-user-decryption
./fhevm-cli test decryption-signature-invalidation
./fhevm-cli test connector-http-public-decrypt
./fhevm-cli test connector-http
./fhevm-cli test light
./fhevm-cli test standard
./fhevm-cli test heavy
./fhevm-cli test operators
./fhevm-cli test --grep "oversized shift and rotate" --verbose
./fhevm-cli test operators --grep "edge cases" --verbose
./fhevm-cli pause host
./fhevm-cli unpause host

./fhevm-cli down
./fhevm-cli clean
```

## Local Overrides

Use `--override` to run local code for one repo-owned group on top of an otherwise versioned stack.

Important:

- by default, the stack uses the published `test-suite` image
- local e2e test changes are not picked up unless you use `--override test-suite` or `--build`
- if you are validating newly added or edited tests in this branch, prefer `--override test-suite` for a surgical local test-suite rebuild

Use `--build` when you want the whole local workspace on the active baseline. On topology-only scenario runs, `--build` also applies local coprocessor images to inherited scenario instances. If a scenario explicitly pins coprocessor source, overlapping explicit coprocessor overrides fail fast instead.
`--build` cannot be combined with `--override`.

Supported groups:

- `coprocessor`
- `kms-connector`
- `relayer`
- `gateway-contracts`
- `host-contracts`
- `test-suite`

### Override an entire group

```sh
./fhevm-cli up --target latest-supported --override coprocessor
./fhevm-cli up --target latest-main --override relayer
./fhevm-cli up --target latest-main --override test-suite
```

For `coprocessor`, this is also the shorthand local-dev scenario: one coprocessor instance, threshold `1`, source mode `local`.
For `test-suite`, this is the explicit path that makes local e2e test edits take effect at runtime.

### Build the local workspace

```sh
./fhevm-cli up --target latest-main --build
./fhevm-cli up --target latest-main --scenario two-of-two --build
```

### Public runtime bases for a local E2E run

The production Dockerfile defaults to the certified `cgr.dev/zama.ai` runtime
images. If that private registry is unavailable on a development host, an
explicit local-only switch rebuilds the coprocessor targets on public
`debian:trixie-slim` (and the migration target on `postgres:17-trixie`). The
Trixie base is required because the bundled migration `sqlx` requires
`GLIBC_2.39`; the E2E-only Dockerfile stages also install `ca-certificates` so
Rust HTTPS clients can load system roots:

```sh
./fhevm-cli up --lock-file ../../.fhevm/state/locks/sha-72af12a.json \
  --scenario scenarios/three-of-three.yaml \
  --override coprocessor --override test-suite --e2e-public-runtime
```

This option affects only locally built coprocessor and KMS connector runtime
images (not the connector DB migration) and is persisted in the generated E2E
state, so a later `./fhevm-cli up --resume` uses the same bases. It is not a
production or certification build mode.

Add `--override kms-connector` when the E2E stack also needs the connector from
the current checkout. Its three runtime services then use the same public
Debian fallback; the connector DB migration keeps its normal Dockerfile and
base image.

### Adopt the public KMS connector runtime on an existing E2E stack

If a running local E2E stack was already booted with local `coprocessor` and
`test-suite` overrides, and only the connector needs to move from its published
image to this checkout, use the following explicit recovery command from
`test-suite/fhevm`:

```sh
./fhevm-cli upgrade kms-connector --adopt-local-override --e2e-public-runtime
```

This is intentionally narrower than `up --resume`: it fails unless the
persisted stack has completed the KMS connector step and has active local
coprocessor and test-suite overrides. It persists a runtime-only
`kms-connector` override, regenerates compose with the public Debian E2E base,
builds and force-recreates only `gw-listener`, `kms-worker`, and `tx-sender`
(for every threshold party when applicable), and waits for connector readiness.
It never starts the connector DB migration and uses Compose `--no-deps`, so it
does not reset or recreate Postgres, MinIO, KMS core, generated keys, contract
discovery, or cached proofs. It is local E2E recovery only, not a production or
certification operation. The persisted `--e2e-public-runtime` policy also
applies to a later explicit local coprocessor rebuild, but this command does
not build or recreate any coprocessor service. If Docker or the host fails
mid-replacement, the persisted state records a pending adoption; retry the same
command (or run `./fhevm-cli up --resume` with no `--from-step`) and the CLI
will repeat only this runtime-only replacement rather than invoking the normal
KMS connector migration step.

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

| Group           | Suffixes                                                                                                                                    |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| `coprocessor`   | `db-migration`, `host-listener`, `host-listener-poller`, `gw-listener`, `tfhe-worker`, `zkproof-worker`, `sns-worker`, `transaction-sender` |
| `kms-connector` | `db-migration`, `gw-listener`, `kms-worker`, `tx-sender`, `endpoint`, `proxy`                |
| `test-suite`    | `e2e-debug`                                                                                                                                 |

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

`upgrade` restarts the selected runtime group in place. With `--lock-file`, it moves that group to the versions from the lock, regenerates env/compose, runs DB migration services when present, and restarts only the affected runtime services.

## Dropped Convenience Commands

- `smoke`: use explicit `up ...` plus `test ...`
- `test debug`: use `docker exec -it fhevm-test-suite-e2e-debug sh`

## Coprocessor Scenarios

Use `--scenario <name-or-file>` for consensus and stateful rollout runs. Bundled presets resolve by filename stem, and explicit file paths still work. The scenario file is the source of truth for:

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

Blue-green scenarios pin a previous-release Blue whose tfhe-rs cannot read key material from a newer KMS core. `bootstrap.tag` boots the contracts, the KMS core and connector and listener-core at that release (Green stays deferred), waits for the operators to ingest the generated keys, then upgrades them to the resolved bundle in place in production order: the changed contracts through their `task:upgrade*` tasks, the KMS core over the existing keys, the connector, listener-core, and finally the Green fleet. Local overrides for the bootstrapped components apply from the upgrade on; the relayer and test-suite start on the bundle. Centralized KMS only:

```yaml
bootstrap:
  tag: v0.14.2-0
```

`--scenario` can be combined with `--override coprocessor` as long as the scenario only defines topology/env/args and leaves coprocessor source inherited. If the scenario explicitly pins coprocessor source (for example with `source.mode=local` or `source.mode=registry`), overlapping `--override coprocessor...` inputs fail fast.

### GPU consensus workers

GPU builds use the production `gpu` feature set by default. Crash-boundary tests
require an explicit `GPU_CONSENSUS_TEST_FAILPOINTS=1` for `build`, `start` and
`test-env`; ordinary GPU CI coverage leaves it unset. The build manifest records
the feature set, and each invocation records its executable hash. Restoring a
unit refuses a different executable, so rebuilding workers during a session
requires a complete stop/build/start handover.


`scripts/gpu-consensus-workers.sh` runs the TFHE, ZK-proof and SNS workers on the host with GPU features, for the three-operator consensus topology. `start` stops the container workers it displaces and records them; `stop` puts them back and clears that record.

Only one worker may serve an operator's work queue. Each role claims rows with `FOR UPDATE SKIP LOCKED`, which is correct for one worker and silently wrong for two: each row is served by whichever process won it, so a host worker and a container split the queue between two different builds. For the SNS worker that leaves one operator holding a mix of CPU-squashed and GPU-squashed `ct128` for handles whose `ct64` is identical — it reads as a consensus defect, and telling it apart from one cost a full investigation. For the TFHE worker it would diverge `ct64` itself.

The units are transient with `Restart=on-failure`, so they survive a stack teardown and restart themselves. Always end a session with `stop` rather than stopping the units by hand — otherwise the next `up` recreates the containers underneath them. Three guards enforce this:

- `up` refuses to start while the restore record exists, and names the fix.
- readiness refuses to call the stack ready if any worker process outside the stack's containers is running, or if operators disagree on `gpu_enabled`.
- `gpu-consensus-workers.sh conflicts` reports any queue served by both a unit and a container, and `status` exits non-zero when it finds one.

```sh
./scripts/gpu-consensus-workers.sh start
./scripts/gpu-consensus-workers.sh conflicts   # exits 0 when clean
./scripts/gpu-consensus-workers.sh stop
```

A stopped unit is restored through the launcher, never through `systemctl
start`: the units are transient (`systemd-run --collect`), so stopping one
garbage-collects it and the name stops resolving. `stop-unit` keeps the unit's
recorded invocation configuration and `restart-unit` restores from that record
rather than re-resolving the tuning from the calling shell — a caller without
the `GPU_CONSENSUS_*` overrides exported would otherwise bring a deliberately
heterogeneous operator back on fleet defaults, or a second-GPU operator back on
card 0. `verify-restore <kind> <index>` is the acceptance case for that path:
it stops and restores one unit from a shell with no overrides set and requires
the restored invocation to carry the identical recorded configuration. Unit
replacement waits until the old process is gone and the transient unit is
inactive or removed; `deactivating` is not a completed stop.

The `coprocessor-db-state-revert` test profile uses the selected Hardhat network's
canonical RPC head immediately before its seed workload as the revert boundary.
The RPC chain ID must match `CHAIN_ID` (or the scenario's first host-chain ID).
The profile requires completed computations above that boundary before changing
SQL state. `DB_REVERT_ANCHOR_TIMEOUT_SECONDS` defaults to 60 seconds, including
cold TypeScript/provider startup, with a further 15-second host cancellation
allowance. A failed or mismatched probe aborts before the revert.
The target must be an operator database in the active managed DB container;
external `POSTGRES_CONTAINER`, `POSTGRES_HOST`, or `POSTGRES_DB` overrides are
refused before seeding. All managed blue and green writers are stopped and
restored according to their original state. Unmanaged writers or an existing
paused owner abort the operation before any service is stopped.

## Consensus coverage

`consensus/inventory.yaml` is the inventory of what the consensus suite covers.
One row per case, and the row is the contract: the property a green
establishes, the topology and services it needs, the fault applied and the
stage it is applied at, how that fault is independently observed, the expected
quorum outcome, and the runner that executes it. Everything that selects, runs
or reports on a case derives from it.

```sh
bun scripts/consensus-inventory.ts validate          # the inventory is well-formed
bun scripts/consensus-inventory.ts list              # what exists
bun scripts/consensus-inventory.ts show CR-01-INTERRUPT-BEFORE-COMMIT
bun scripts/consensus-inventory.ts plan standard     # what a selection would run
bun scripts/consensus-inventory.ts aggregate --run <id> --select full
```

Selections accept `leg:<name>` and `family:<name>` to distinguish dispatch legs
from case families. For example, `leg:failure-matrix` excludes the crash-retry
and Rust cases delegated by `family:failure-matrix`; unqualified names retain
the union for compatibility.

CI records immutable container image IDs per verdict. A cold-build receipt binds
locally built image groups to the clean pre-boot checkout revision; the full
branch gate rejects missing receipts, stale revisions and mismatching running
image IDs. Published companion images are identified but are not attributed to
the checkout. `build=false` still builds this checkout's test-suite image while
using published runtime images, and remains partial coverage. Local runs without
the CI receipt report only the identities their runner actually collected.

Runners emit one structured result per case into
`.fhevm/runtime/consensus-results/<run-id>.jsonl`, carrying the case contract
plus the checkout revision, workload ids, process identities
before and after the fault, observed fault/recovery timestamps, per-assertion
outcomes and the cleanup outcome. `aggregate` is the verdict: it rejects a run
that reported no result for a case it selected, a duplicate with a conflicting
state, a result produced against a different revision or topology, and a PASS
carrying a failed cleanup, an unobserved fault, or missing any declared assertion
kind. Detailed runner outcomes accompany explicit summaries of the safety,
liveness, bytes, quorum and other contracts actually checked. A run whose selection is a
subset of the required set is labeled PARTIAL and cannot satisfy the full
gate. Every required case needs a PASS; NOT_APPLICABLE cannot satisfy it. CI's
final aggregate combines all job artifacts at the checked-out revision and
requires every selected job to succeed, including its final validity gates. A
missing, skipped or failed required job fails the full selection. Image evidence
records each running container's immutable image ID rather than resolving its
possibly reassigned tag. GPU coverage runs
three operators on one H100. `--device-split` is deferred and exits before running: configured placement alone does not prove which GPU executed the selected work. It is excluded from CI.

`topology.backends` lists compatible local execution backends. `ci.backend`
identifies the backend covered by the planned CI job; `aggregate --ci` checks that
backend. A CPU result never claims additional GPU coverage. `standard` selects all
CPU and stackless cases, including Rust regressions, fork and isolated database
jobs. `full` adds the GPU cases.

With workflow `build=false`, published-stack results carry `build_mode=published`
and a separate software label; the checkout revision identifies the harness.
Cases that require local test features still build locally and say so. Such a
workflow is partial and cannot satisfy the branch delivery gate. Branch validation
uses `build=true` and aggregation with `--require-build-mode checkout`. Different
selections and build modes have separate concurrency groups, so a harness dispatch
does not cancel an active full run.

Build manifests and result writers share one dirty-source definition: only the
exact generated `E2ECoprocessorConfigLocal.sol` path is exempt. The running E2E
suite's source hash still includes that generated file.

For the stackless harness, install CLI dependencies with `bun install --frozen-lockfile` in `test-suite/fhevm`, and comparator dependencies with `npm ci --ignore-scripts --workspace @fhevm/e2e-suite` at the repository root. Then run `scripts/run-stackless-cases.sh --leg harness` from this directory; it needs no running stack.

The runners:

| Runner | Cases |
|---|---|
| `run-materialization-consensus.sh --suite materialization` | the byte oracle: raw bytes, digest bindings, provenance, transaction completion, quorum, plaintext |
| `run-materialization-consensus.sh --suite reorg` | a block replaced by a distinct sibling |
| `run-stackless-cases.sh --leg comparator` | the shared oracle's mismatch classes, against synthetic rows |
| `run-degraded-consensus.sh --case core` | quorum with an operator removed and backlog convergence, with default auto-revert |
| `run-degraded-consensus.sh --case gw` | the in-flight gateway event, with auto-revert disabled |
| `run-fork-consensus.sh --case all` | competing branches, orphaned content, a stranded child, replay |
| `run-crash-retry-consensus.sh --boundary ...` | interruption of identified in-flight work at three boundaries |
| `run-failure-matrix.sh --column ...` | per-service faults with service-appropriate workloads |
| `run-mixed-backend-guard.sh` | the guard against two builds on one queue |
| `test-fault-contracts.sh` | the fault-control layer's fail-closed rules, with no stack |

DEG06 (`--case gw`) requires drift auto-revert disabled on every
gateway listener so its deliberately conflicting event cannot revert the fleet.
Prepare the scenario before booting; CI gives DEG06 its own isolated job:

```sh
bun scripts/prepare-degraded-scenario.ts three-of-three > /tmp/consensus-degraded.yaml
./fhevm-cli up --target latest-main --scenario /tmp/consensus-degraded.yaml --build
```

Use `two-of-three` in the first command for the majority-quorum topology. The
helper preserves the topology, sources and runtime arguments and sets the existing
per-instance `DRIFT_AUTO_REVERT_ENABLED` env override to `false`. Ordinary scenarios
retain automatic drift recovery. Core degraded cases require that default, and
results record the setting observed on the live listeners. Run the core and
gateway cases on their respective configurations.

The runners enforce these rules:

- A suite's verdict is its process EXIT STATUS plus its own completion marker.
  `1 passing` in mocha output survives an after-hook failure, and a suite that
  skipped every test exits 0 too.
- Every fault injection and every heal checks both the command's status and an
  independent postcondition. A stopped process is confirmed stopped by reading
  `/proc`, and a killed one is confirmed replaced by a changed process identity
  — a successful signal proves only that the signal was sent.
- Each case finishes cancellation, durable-journal recovery, and safe service
  restoration before admitting a later case. A failed case can be followed by a
  real next case; an unresolved cleanup prevents another fault. Matrix children
  stage results until the parent has completed cleanup, so parent failure produces
  one final failed verdict instead of conflicting PASS and FAIL records.
- Failure-matrix phases share one inventory deadline. A supervisor inside the
  E2E container owns each phase's process group; cleanup proves cancellation
  before healing faults. If shutdown cannot be verified, runners refuse the
  stack and retain the phase/restore records at
  `.fhevm/runtime/failure-matrix/uncancelled-phase`. Recover those recorded
  processes and faults before clearing that marker.

The proof recovery case consumes the original interrupted request's encrypted
input and requires durable verifier outcomes. The stranded-child case proves
repair sensitivity with a scoped lost-decrement control. Replay requires
committed insert attempts for the selected graph from the restarted poller.
The gateway restart case identifies the exact event in each replacement's
warning log against safely poisoned, already-published digests, then restores
those digests. Watermark progress alone does not satisfy these recovery claims.

## Troubleshooting

**A coprocessor service exits and comes back on its own**

That is intended. The coprocessor runtime services carry `restart: "on-failure:10"`. They implement exit-for-restart — on a fatal error they log, flush telemetry and exit non-zero, expecting a supervisor — and without a policy the stack degraded permanently on any such path. `docker inspect -f '{{.RestartCount}}' <service>` tells you whether one has been recovering.

`on-failure` and not `unless-stopped`: a clean exit 0 is a service that meant to finish (a retired stack after cutover, a `--stack-version` probe) and restarting it would loop. Docker also treats `docker kill` and `docker stop` as manual and suppresses the policy for both, so fault injection still holds a service down until you start it again. The bound of 10 limits automatic retry attempts; the harness resets the budget with an explicit stop/start before arming the next fault.

**Services exit silently shortly after startup (e.g. `coprocessor-zkproof-worker`)**

This is usually a Docker memory limit. The stack requires at least 16 GB allocated to Docker. Scenarios with both multiple chains and multiple coprocessors need 32 GB.

Check your current allocation in Docker Desktop → Settings → Resources → Memory, then restart the stack.

## Runtime State

The CLI owns:

- `.fhevm/state/state.json`
- `.fhevm/state/locks/`
- `.fhevm/runtime/env/`
- `.fhevm/runtime/compose/`
- `.fhevm/runtime/addresses/`

`status` shows the active stack state, the active scenario origin when present, and any CLI-owned local build images.


### Manifest publication and containment

Build a fresh local three-coprocessor stack and run the opt-in profile:

```sh
./fhevm-cli up --target latest-main --scenario manifest-lifecycle --build
./fhevm-cli test manifest-lifecycle
```

The manifest scenario starts consensus detectors 15 seconds after all host listeners
are running, giving them time to catch up. This delay is not a catch-up completion guarantee.
Verification records `quorum_from_block` and `quorum_through_block`, plus any older
unverified prefix. The profiles require quorum coverage of the fixture blocks;
they do not require consensus from the start of history. Known historical drift
still reports `differs_from_quorum`.

For a healthy baseline on a fresh stack, run `./fhevm-cli test manifest-lifecycle-no-drift`.
It keeps all detectors running, creates a root and descendant, then submits dependent
and independent consumers. For every fixture handle on all three nodes it checks
completed computation, verified upload, matching ct64 digests, actual S3 manifest
bytes with a computed descriptor, and authenticated consensus with no drift.
Drift tables must remain empty. Final tables and checkpoints are saved in
`manifest-drift/2/no-drift-report.json`, including diagnostics on failure.
In CI select scenario `manifest-lifecycle`, test-profile `manifest-lifecycle-no-drift`,
and enable the build. This profile does not exercise healing or decryption.

The scenario gives each coprocessor its own read-only directory mount and
`--dangerous-drift-injection=/manifest-drift/injection.json`. The directory starts
empty. The containment profile uses only node 2. It waits for publication readiness, stops that detector, submits
a fresh `root -> child` fixture, writes the selected root handle and `pause_healing: true` to the file,
and starts the same container. No ciphertext/digest/finding database rows are
modified by the test.

The profile checks actual S3 manifest bytes, the injected descriptor versus the
two matching publishers, authenticated quorum verification, direct/inferred
containment, unchanged local ciphertext material, and continued independent work
in a transaction whose dependent computation remains frozen on coprocessor 2.
It does not assume that healthy observers cannot record non-quorum differences:
current drift inventory conservatively records observed disagreements.

On normal success or failure, cleanup stops the detector, removes the injection
file, and starts it normally. Reports and failure logs are written beneath
`$FHEVM_STATE_DIR/runtime/config/manifest-drift/2/` (default `.fhevm` at the repo
root). Cleanup preserves signed manifests and drift findings; use a disposable
stack for this exercise. If the runner is forcibly killed, stop the target,
remove `injection.json` from that directory, then restart it before other tests.

The containment profile requires healing to remain paused until its frozen-state
assertions finish; it fails explicitly if healing wins that race.

### Manifest healing, case matrix, and computation recovery

Rebuild a fresh stack with scenario `manifest-lifecycle`, then run
`./fhevm-cli test manifest-healing`. CI uses scenario `manifest-lifecycle`,
test-profile `manifest-healing`, with the build enabled.

The profile creates seven roots. Only node 2 publishes faults, so it is the
only coprocessor out of quorum. Two ct64 roots feed separate computed children,
a shared join, and a further descendant. Each other root has a computed child
that must remain outside inferred drift. `unknown_on_peer`, `error_on_peer`,
and `uncomputed_on_peer` are not in this profile: those reasons require the
other two publishers to agree on the anomaly.

| Reason on node 2 | Injection location | Expected ct64 recovery |
| --- | --- | --- |
| `ct64_mismatch` (two roots) | Node 2 ct64 digest | Healed; descendants inferred and contained |
| `missing_here` | Node 2 omits descriptor | Healed; no inferred descendants |
| `error_here` | Node 2 error descriptor | Healed; no inferred descendants |
| `uncomputed_here` | Node 2 uncomputed descriptor | Healed; no inferred descendants |
| `ct128_mismatch` | Node 2 ct128 digest | Not healed; ct64 consumers continue |
| `metadata_mismatch` | Node 2 keyset ID | Not healed; ct64 consumers continue |

The test stops all detectors before creating the fixture. It waits for completed
computations and verified uploads, then flips one bit in node 2's local ct64
bytes. All three injection files start with `pause_healing: true`. Nodes 0 and 1
have an empty fault list. Node 2 carries every fault. Publication and
verification stay active while healing is suppressed. Findings are created only
by normal verification and propagation, never by the test.

The runner checks actual signed/S3 descriptors, authenticated quorum verification,
and the exact direct/inferred inventory. It submits more work while healing is
paused: consumers of the ct64 branches must remain pending, while independent
work and the other branches complete. It then restarts the detectors with
`pause_healing: false`. Node 2 keeps the same faults, so its persisted seals
stay reproducible. Nodes 0 and 1 keep an empty fault list.

Recovery requires `healed_at` on every healable root and inferred descendant,
the correct target digests, exact restored bytes, completed queued computations,
and correct decrypted results. A final transaction reuses the repaired chain and
an original drift root, then decrypts again. Non-healable rows must remain
unhealed throughout these checkpoints; they do not block valid ct64 work.

`report.json` records the fixture, case matrix, manifests, containment/healing
snapshots, corruption offset, and recovery/decryption checkpoints. Logs from all
three detectors are collected on success and failure. Cleanup removes all three
injection files and restarts all detectors. It does not undo failed repairs or
remove durable findings: use a disposable stack. This scenario does not cover
peer outages, quorum loss, cross-epoch recovery, or publication-history repair.

### Manifest healing under mixing traffic

On a fresh `manifest-lifecycle` deployment, run:

```sh
./fhevm-cli test manifest-healing-stress
```

CI: scenario `manifest-lifecycle`, test-profile `manifest-healing-stress`, build enabled.
Four interleaved uint64 chains mix adjacent heads and repeatedly reuse pinned earlier
outputs. Each round also produces independent work. This is four dependency chains
within each transaction, not four concurrent Hardhat processes.

After a healthy baseline, a test-owned `AFTER INSERT` trigger on **node 2 only**
flips bit 7 of the middle byte of each eligible ciphertext with probability **1/4**.
Input ciphertexts, non-uint64 ciphertexts, nonzero versions, and blobs of at most
64 bytes are excluded. The realized ratio is random, not exactly one quarter of a
finite batch. Every eligible insertion and selected fault is recorded.

Eight rounds run with injection enabled. After multiple faults and verified drift
are observed, the trigger is removed while mixing traffic continues. Recovery has
a 15-minute deadline and requires at least three stable rounds spanning 30 seconds:
all submitted handles must complete on all peers and their stored ct64 SHA-256
fingerprints must match, no recoverable/contained finding may
remain unhealed (including unpinned findings), and finding/healing inventories must
stop changing. Historical digest metadata is not required to change: the healer
repairs local bytes separately from publication-history reconciliation. Current heads and independent work must decrypt to expected uint64
values. This checks convergence after faults stop, not bounded latency under an
unlimited permanent fault stream.

`manifest-drift/2/stress-report.json` contains per-round inventories, final drift
rows, and every sampled/injected handle. Cleanup removes the trigger and function
on success or failure and saves the audit before dropping its table. No detector
restart or manifest-only injection is used. Run this profile only on the isolated
scenario; it intentionally corrupts stored ciphertexts.
