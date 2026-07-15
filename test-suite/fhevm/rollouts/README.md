# Stateful rollout runbooks

The `run.ts` file in each rollout folder is the runbook. It is an executable TypeScript program that changes one persistent local stack in ordered steps: boot the baseline once, preserve chain and database state, upgrade selected components in place, and test every state that is expected to work.

Start with the smallest runbook that models the upgrade. Add contract tasks or extra test profiles only when the rollout requires them.

## Success criteria

A useful runbook:

1. Pins a real, complete version bundle for every state.
2. Boots the baseline exactly once with `ctx.up(...)`.
3. Applies one intentional upgrade step at a time.
4. Runs `ctx.test("rollout-standard", { parallel: false })` after every state expected to work.
5. Stops on unexpected failures and leaves a receipt explaining the last completed step.

## Write `run.ts`

Create one folder:

```text
rollouts/my-rollout/
├── run.ts
└── versions.ts
```

Use an existing runbook or resolved lock as the source for `versions.ts`. Every phase must contain the complete `*_VERSION` map, not only the keys changed in that phase.

```ts
// rollouts/my-rollout/versions.ts
type Versions = Record<string, string>;

export const scenario = "two-of-three";

export const baseline = {
  // Copy a complete, real bundle here.
  RELAYER_VERSION: "old-relayer",
  RELAYER_MIGRATE_VERSION: "old-relayer",
  // ...all other required *_VERSION entries...
} satisfies Versions;

export const target = {
  ...baseline,
  RELAYER_VERSION: "new-relayer",
  RELAYER_MIGRATE_VERSION: "new-relayer",
} satisfies Versions;

export const versionSources = ["rollout=my-rollout", "reason=what-this-proves"];
```

The runbook writes those bundles to generated lock files, boots the baseline, upgrades one runtime group, and gates both states:

```ts
// rollouts/my-rollout/run.ts
import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { baseline, scenario, target, versionSources } from "./versions";

export default async function run(ctx: RolloutRunContext) {
  const baselineLock = await ctx.writeVersionLock("00-baseline", {
    versions: baseline,
    sources: versionSources,
  });
  const targetLock = await ctx.writeVersionLock("01-target", {
    versions: target,
    sources: versionSources,
  });

  await ctx.up({
    lockFile: baselineLock,
    scenario,
    // Keep this when the runbook relies on tests from the current checkout.
    overrides: [{ group: "test-suite" }],
  });
  await ctx.test("rollout-standard", { parallel: false });

  await ctx.upgradeRuntimeGroup("relayer", { lockFile: targetLock });
  await ctx.test("rollout-standard", { parallel: false });
}
```

For multiple runtime upgrades, create a complete intermediate bundle and lock for each step. A lock passed to `ctx.upgradeRuntimeGroup(...)` may change only version keys owned by that group.

```ts
await ctx.upgradeRuntimeGroup("relayer", { lockFile: relayerLock });
await ctx.test("rollout-standard", { parallel: false });

await ctx.upgradeRuntimeGroup("coprocessor", { lockFile: coprocessorLock });
await ctx.test("rollout-standard", { parallel: false });
```

Supported runtime groups are `coprocessor`, `kms-connector`, `kms-core`, `kms`, `listener-core`, `relayer`, and `test-suite`.

Contract upgrades do not use `ctx.upgradeRuntimeGroup(...)`. They normally require `ctx.snapshotContracts(...)`, `ctx.applyVersionLock(...)`, host or gateway contract tasks, and `ctx.refreshDiscovery()`. Copy the ordering from [`./rollouts/v0.12-to-v0.13/run.ts`](./v0.12-to-v0.13/run.ts); contract order is release-specific and should not be guessed.

The runbook context exposes these operations:

- Stack and state: `ctx.up(...)`, `ctx.readState()`, and `ctx.stateDir()`.
- Version and runtime changes: `ctx.writeVersionLock(...)`, `ctx.applyVersionLock(...)`, and `ctx.upgradeRuntimeGroup(...)`.
- Contract changes: `ctx.snapshotContracts(...)`, `ctx.runHostContractTask(...)`, `ctx.runHostContractTaskOnChain(...)`, `ctx.runGatewayContractTask(...)`, and `ctx.refreshDiscovery()`.
- Test gates: `ctx.test(...)`.

## Run it locally

From `test-suite/fhevm`:

```sh
bun install
bun run check
bun test

./fhevm-cli status
./fhevm-cli rollout run ./rollouts/my-rollout/run.ts
./fhevm-cli rollout receipt
```

The generated receipt is stored in `.fhevm/rollout/receipt.md` and `.fhevm/rollout/receipt.jsonl`.

If the existing CLI-owned stack may be discarded, start clean:

```sh
./fhevm-cli clean --keep-images
```

`clean` deletes `.fhevm`, including resumable state. Do not run it when the existing stack must be preserved.

## CI

CI support is defined by `.github/workflows/test-suite-stateful-rollout.yml`. If that file is absent on the branch being tested, the rollout runner is local-only on that branch. When it is present, follow the triggers and inputs declared in the workflow itself instead of copying them into this document.

## Current limitations

- The current API upgrades whole runtime groups; it does not upgrade individual KMS nodes.
- Do not use `kms`, `kms-core`, or `kms-connector` runtime-group upgrades in threshold-KMS scenarios. `kms` and `kms-core` reject them; `kms-connector` currently upgrades only party 1.
- A failed run stops at the thrown error and records failure diagnostics; it does not roll back automatically.
- `rollout-standard` is the per-state gate. Run `./fhevm-cli test list` for the current named profiles, and add broader profiles explicitly when the feature requires them.

## Checked-in examples

- [`./rollouts/v0.12-to-v0.13/run.ts`](./v0.12-to-v0.13/run.ts): contracts, relayer, KMS, listener-core, and coprocessor.
- [`./rollouts/v0.13.0-testnet/run.ts`](./v0.13.0-testnet/run.ts): a network-shaped release rollout.

The documentation test verifies the public runbook operations, supported runtime groups, per-node limitation, and checked-in example paths against the implementation. Update this document when the rollout surface changes.
