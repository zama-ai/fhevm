/**
 * Executes release-specific rollout runbooks against the local fhevm stack.
 */
import path from "node:path";
import { pathToFileURL } from "node:url";

import { CommandError, PreflightError } from "../errors";
import { runContractTask, snapshotContractSources } from "../flow/contracts";
import { waitForTestSuite } from "../flow/readiness";
import { composeUp } from "../flow/runtime-compose";
import {
  applyVersionLock as applyStackVersionLock,
  refreshDiscovery as refreshStackDiscovery,
  up,
  upgradeCoprocessorInstance,
  upgradeThresholdKmsNode,
  upgradeRuntimeGroup as upgradeStackRuntimeGroup,
} from "../flow/up-flow";
import { STATE_DIR, composePath, hostChainRuntimes } from "../layout";
import { reconstructionThreshold } from "../kms-party";
import { previewBundle } from "../resolve/bundle-store";
import { OPTIONAL_REPO_KEYS } from "../resolve/target";
import { loadState } from "../state/state";
import type { LocalOverride, State, UpOptions, VersionBundle, VersionTarget } from "../types";
import { ensureDir, writeJson } from "../utils/fs";
import { partyContainers, setRunning, waitForPartiesRunning, waitForPartiesStopped } from "./kms-generation";
import { type RolloutReceipt, createRolloutReceipt } from "./rollout-receipt";
import { test as runTest } from "./test";

type RolloutUpOptions = {
  lockFile: string;
  overrides?: LocalOverride[];
  scenario?: string;
};

type RolloutRuntimeUpgradeOptions = {
  lockFile?: string;
};
type RolloutKmsNodeUpgradeOptions = {
  lockFile: string;
};
type RolloutCoprocessorInstanceUpgradeOptions = {
  lockFile: string;
};
type RolloutVersionLockOptions = {
  allowedVersionKeys: string[];
  lockFile: string;
  overrides?: LocalOverride[];
};

type RolloutTestOptions = {
  grep?: string;
  network?: string;
  noHardhatCompile?: boolean;
  parallel?: boolean;
};
type RolloutExpectedTestFailureOptions = RolloutTestOptions & {
  errorIncludes: string;
};
type RolloutContractTaskOptions = {
  env?: Record<string, string>;
};
type RolloutLockOptions = {
  versions: Record<string, string>;
  sources?: string[];
  target?: VersionTarget;
};

export type RolloutRunContext = {
  applyVersionLock(label: string, options: RolloutVersionLockOptions): Promise<void>;
  expectTestFailure(profile: string, options: RolloutExpectedTestFailureOptions): Promise<void>;
  readState(): Promise<State>;
  refreshDiscovery(): Promise<void>;
  runGatewayContractTask(command: string, options?: RolloutContractTaskOptions): Promise<void>;
  runHostContractTask(command: string, options?: RolloutContractTaskOptions): Promise<void>;
  // Runs a host contract task against a specific host chain's deploy container
  // (e.g. a Polygon stand-in second chain). Enables multi-chain contract deploys.
  runHostContractTaskOnChain(chainKey: string, command: string, options?: RolloutContractTaskOptions): Promise<void>;
  snapshotContracts(surface: "host" | "gateway"): Promise<void>;
  stateDir(): string;
  test(profile?: string, options?: RolloutTestOptions): Promise<void>;
  up(options: RolloutUpOptions): Promise<void>;
  /**
   * Sequentially applies one KMS lock to exactly the listed serving nodes. Each node's
   * connector moves with its core, since the pair cannot straddle a release boundary.
   */
  upgradeKmsNodes(nodeIds: readonly number[], options: RolloutKmsNodeUpgradeOptions): Promise<void>;
  /**
   * Sequentially moves exactly the listed coprocessor operators to one lock, leaving
   * their peers behind. The bundle lock only lands once the last one crosses.
   */
  upgradeCoprocessorInstances(
    indexes: readonly number[],
    options: RolloutCoprocessorInstanceUpgradeOptions,
  ): Promise<void>;
  /** Runs a check with an exact reconstruction quorum that must include this KMS node. */
  withRequiredKmsNode(nodeId: number, task: () => Promise<void>): Promise<void>;
  upgradeRuntimeGroup(group: string, options?: RolloutRuntimeUpgradeOptions): Promise<void>;
  resolveVersionLock(name: string, options: RolloutLockOptions): Promise<string>;
  writeVersionLock(name: string, options: RolloutLockOptions): Promise<string>;
};

export type RolloutRunbook = (ctx: RolloutRunContext) => Promise<void> | void;

type RolloutContextOperations = {
  previewBundle: typeof previewBundle;
  setRunning: typeof setRunning;
  upgradeThresholdKmsNode: typeof upgradeThresholdKmsNode;
  upgradeCoprocessorInstance: typeof upgradeCoprocessorInstance;
  waitForPartiesRunning: typeof waitForPartiesRunning;
  waitForPartiesStopped: typeof waitForPartiesStopped;
};

/** Rollouts always resolve their surrounding stack from current main. */
const ROLLOUT_TARGET: VersionTarget = "latest-main";

const upOptions = (options: RolloutUpOptions): UpOptions => ({
  target: ROLLOUT_TARGET,
  overrides: options.overrides ?? [],
  scenarioPath: options.scenario,
  lockFile: options.lockFile,
  allowSchemaMismatch: false,
  resume: false,
  dryRun: false,
  reset: false,
});

const refreshTestSuiteContainer = async () => {
  console.log("[test-suite] recreate container to load current generated env");
  await composeUp("test-suite", ["test-suite-e2e-debug"], { noDeps: true, forceRecreate: true });
  await waitForTestSuite();
};

const runRolloutTest = async (receipt: RolloutReceipt, profile: string, options: RolloutTestOptions) => {
  await refreshTestSuiteContainer();
  await receipt.record("refresh-test-suite", "recreated test-suite container with current env", {
    details: { profile },
  });
  await runTest(profile, {
    network: options.network ?? "staging",
    verbose: false,
    noHardhatCompile: options.noHardhatCompile ?? true,
    parallel: options.parallel,
    grep: options.grep,
  });
};

export const matchesExpectedTestFailure = (error: unknown, errorIncludes: string): error is CommandError =>
  error instanceof CommandError && error.stderr.includes(errorIncludes);

/**
 * A rollout that starts before a component existed has to say so, and the only way a lock
 * bundle expresses "this release does not run that component" is by leaving its key out —
 * an optional key that is present must carry a real tag. Runbooks write the empty string
 * instead, because a version map reads far better with every key listed at every phase than
 * with keys appearing and disappearing between phases, so the empty string is dropped here.
 */
const withoutUnsetOptionalKeys = (versions: Record<string, string>) =>
  Object.fromEntries(Object.entries(versions).filter(([key, value]) => value.length || !OPTIONAL_REPO_KEYS.has(key)));

const writeRolloutVersionLock = async (name: string, options: RolloutLockOptions) => {
  const filename = name.endsWith(".json") ? name : `${name}.lock.json`;
  const file = path.join(STATE_DIR, "rollout", filename);
  await ensureDir(path.dirname(file));
  const bundle = {
    target: options.target ?? ROLLOUT_TARGET,
    lockName: path.basename(file),
    env: withoutUnsetOptionalKeys(options.versions),
    sources: options.sources ?? ["rollout-runbook"],
  } satisfies VersionBundle;
  await writeJson(file, bundle);
  return file;
};

export const createRolloutContext = (
  receipt: RolloutReceipt = createRolloutReceipt(),
  operationOverrides: Partial<RolloutContextOperations> = {},
): RolloutRunContext => {
  // One resolve per target for the whole run, and not just to save API calls:
  // `latest-main` is never cached on disk (see targetUsesCache), so resolving twice
  // could return two different main shas and silently make a runbook's baseline and
  // target locks differ in components the runbook never meant to change.
  const resolvedBundles = new Map<VersionTarget, VersionBundle>();

  return {
    async applyVersionLock(label, options) {
      await applyStackVersionLock(label, options.lockFile, options.allowedVersionKeys, { overrides: options.overrides });
      await receipt.record("apply-version-lock", label, {
        details: {
          allowedVersionKeys: options.allowedVersionKeys,
          overrides: (options.overrides ?? []).map((override) => override.group),
        },
        lockFile: options.lockFile,
      });
    },
    async expectTestFailure(profile, options) {
      try {
        await runRolloutTest(receipt, profile, options);
      } catch (error) {
        if (!matchesExpectedTestFailure(error, options.errorIncludes)) {
          throw error;
        }
        await receipt.record("test", `${profile} failed as expected`, {
          details: {
            errorIncludes: options.errorIncludes,
            ...(options.grep === undefined ? {} : { grep: options.grep }),
            observedError: error.stderr.slice(-2_000),
          },
        });
        return;
      }
      throw new PreflightError(`${profile} unexpectedly passed; expected an error containing ${JSON.stringify(options.errorIncludes)}`);
    },
    async readState() {
      const state = await loadState();
      if (!state) {
        throw new PreflightError("Stack is not running; run ctx.up(...) first");
      }
      return state;
    },
    async refreshDiscovery() {
      await refreshStackDiscovery();
      await receipt.record("refresh-discovery", "refreshed runtime addresses");
    },
    async runGatewayContractTask(command, options = {}) {
      await runContractTask("gateway-sc", "gateway-sc-deploy", command, options);
      await receipt.record("gateway-contract-task", command, {
        details: { envKeys: Object.keys(options.env ?? {}).sort() },
      });
    },
    async runHostContractTask(command, options = {}) {
      await runContractTask("host-sc", "host-sc-deploy", command, options);
      await receipt.record("host-contract-task", command, {
        details: { envKeys: Object.keys(options.env ?? {}).sort() },
      });
    },
    async runHostContractTaskOnChain(chainKey, command, options = {}) {
      const state = await loadState();
      const runtime = hostChainRuntimes(state?.scenario.hostChains ?? []).find((chain) => chain.key === chainKey);
      if (!runtime) {
        throw new PreflightError(`unknown host chain "${chainKey}"; check the scenario hostChains`);
      }
      // The non-default chain's deploy container is `${runtime.sc}-deploy` with its
      // env at envPath(runtime.sc) (RPC/CHAIN_ID/HOST_ADDRESS_DIR for that chain).
      await runContractTask("host-sc", `${runtime.sc}-deploy`, command, {
        ...options,
        envComponent: runtime.sc,
        composeFile: composePath(runtime.sc),
      });
      await receipt.record("host-contract-task", `[chain ${chainKey}] ${command}`, {
        details: { chain: chainKey, envKeys: Object.keys(options.env ?? {}).sort() },
      });
    },
    async snapshotContracts(surface) {
      await snapshotContractSources(surface);
      await receipt.record("snapshot-contracts", surface);
    },
    stateDir() {
      return STATE_DIR;
    },
    async test(profile = "rollout-standard", options = {}) {
      await runRolloutTest(receipt, profile, options);
      await receipt.record("test", `${profile} passed`, {
        details: {
          network: options.network ?? "staging",
          noHardhatCompile: options.noHardhatCompile ?? true,
          ...(options.grep === undefined ? {} : { grep: options.grep }),
          ...(options.parallel === undefined ? {} : { parallel: options.parallel }),
        },
      });
    },
    async up(options) {
      await up(upOptions(options));
      await receipt.record("up", "boot stack", {
        details: { overrides: (options.overrides ?? []).map((override) => override.group), scenario: options.scenario },
        lockFile: options.lockFile,
      });
    },
    async upgradeKmsNodes(nodeIds, options) {
      const state = await loadState();
      if (!state || state.scenario.kms.mode !== "threshold") {
        throw new PreflightError("upgradeKmsNodes requires a running threshold KMS cluster");
      }
      if (nodeIds.length === 0) {
        throw new PreflightError("upgradeKmsNodes requires at least one node id");
      }
      const selected = new Set<number>();
      for (const nodeId of nodeIds) {
        if (!Number.isInteger(nodeId) || nodeId < 1 || nodeId > state.scenario.kms.committeeSize) {
          throw new PreflightError(
            `upgradeKmsNodes expects serving node ids between 1 and ${state.scenario.kms.committeeSize}; received ${nodeId}`,
          );
        }
        if (selected.has(nodeId)) {
          throw new PreflightError(`upgradeKmsNodes received duplicate node id ${nodeId}`);
        }
        selected.add(nodeId);
      }
      for (const nodeId of nodeIds) {
        try {
          await (operationOverrides.upgradeThresholdKmsNode ?? upgradeThresholdKmsNode)(nodeId, options);
        } catch (error) {
          try {
            await receipt.record("upgrade-kms-node-failed", `KMS node ${nodeId}`, {
              details: { error: error instanceof Error ? error.message : String(error), nodeId },
              docker: true,
              lockFile: options.lockFile,
            });
          } catch (receiptError) {
            throw new AggregateError(
              [error, receiptError],
              `KMS node ${nodeId} upgrade failed and its required receipt snapshot also failed`,
            );
          }
          throw error;
        }
        await receipt.record("upgrade-kms-node", `KMS node ${nodeId}`, {
          details: { nodeId },
          docker: true,
          lockFile: options.lockFile,
        });
      }
    },
    async upgradeCoprocessorInstances(indexes, options) {
      const state = await loadState();
      if (!state || state.scenario.kind !== "coprocessor-consensus") {
        throw new PreflightError("upgradeCoprocessorInstances requires a running coprocessor-consensus scenario");
      }
      if (indexes.length === 0) {
        throw new PreflightError("upgradeCoprocessorInstances requires at least one instance index");
      }
      const count = state.scenario.instances.length;
      const selected = new Set<number>();
      for (const index of indexes) {
        if (!Number.isInteger(index) || index < 0 || index >= count) {
          throw new PreflightError(
            `upgradeCoprocessorInstances expects instance indexes between 0 and ${count - 1}; received ${index}`,
          );
        }
        if (selected.has(index)) {
          throw new PreflightError(`upgradeCoprocessorInstances received duplicate instance index ${index}`);
        }
        selected.add(index);
      }
      for (const index of indexes) {
        try {
          await (operationOverrides.upgradeCoprocessorInstance ?? upgradeCoprocessorInstance)(index, options);
        } catch (error) {
          try {
            await receipt.record("upgrade-coprocessor-instance-failed", `coprocessor instance ${index}`, {
              details: { error: error instanceof Error ? error.message : String(error), index },
              docker: true,
              lockFile: options.lockFile,
            });
          } catch (receiptError) {
            throw new AggregateError(
              [error, receiptError],
              `coprocessor instance ${index} upgrade failed and its required receipt snapshot also failed`,
            );
          }
          throw error;
        }
        await receipt.record("upgrade-coprocessor-instance", `coprocessor instance ${index}`, {
          details: { index },
          docker: true,
          lockFile: options.lockFile,
        });
      }
    },
    async withRequiredKmsNode(nodeId, task) {
      const state = await loadState();
      if (!state || state.scenario.kms.mode !== "threshold") {
        throw new PreflightError("withRequiredKmsNode requires a running threshold KMS cluster");
      }
      const { committeeSize, threshold } = state.scenario.kms;
      if (!Number.isInteger(nodeId) || nodeId < 1 || nodeId > committeeSize) {
        throw new PreflightError(
          `withRequiredKmsNode expects a serving node id between 1 and ${committeeSize}; received ${nodeId}`,
        );
      }
      const requiredCount = reconstructionThreshold(threshold);
      const running = [nodeId];
      for (let candidate = 1; running.length < requiredCount && candidate <= committeeSize; candidate += 1) {
        if (candidate !== nodeId) {
          running.push(candidate);
        }
      }
      const stopped = Array.from({ length: committeeSize }, (_, index) => index + 1).filter(
        (candidate) => !running.includes(candidate),
      );
      if (stopped.length === 0) {
        throw new PreflightError(
          `withRequiredKmsNode cannot require one node when all ${committeeSize} serving nodes are needed for reconstruction`,
        );
      }

      let taskError: unknown;
      let taskFailed = false;
      try {
        await (operationOverrides.setRunning ?? setRunning)(stopped.flatMap(partyContainers), "stop");
        await (operationOverrides.waitForPartiesStopped ?? waitForPartiesStopped)(stopped);
        await receipt.record(
          "require-kms-node",
          `KMS node ${nodeId} required in ${requiredCount}/${committeeSize} quorum`,
          {
            details: { nodeId, running, stopped },
            docker: true,
          },
        );
        await task();
      } catch (error) {
        taskError = error;
        taskFailed = true;
      }

      try {
        await (operationOverrides.setRunning ?? setRunning)(stopped.flatMap(partyContainers), "start");
        await (operationOverrides.waitForPartiesRunning ?? waitForPartiesRunning)(stopped);
        await receipt.record("restore-kms-nodes", `restored KMS nodes ${stopped.join(", ")}`, {
          details: { stopped },
          docker: true,
        });
      } catch (restoreError) {
        if (taskFailed) {
          throw new AggregateError(
            [taskError, restoreError],
            "KMS quorum check failed and stopped nodes could not be restored",
          );
        }
        throw restoreError;
      }
      if (taskFailed) {
        throw taskError;
      }
    },
    async upgradeRuntimeGroup(group, options = {}) {
      await upgradeStackRuntimeGroup(group, options);
      await receipt.record("upgrade-runtime", group, { lockFile: options.lockFile });
    },
    async resolveVersionLock(name, options) {
      const target = options.target ?? ROLLOUT_TARGET;
      let base = resolvedBundles.get(target);
      if (!base) {
        base = await (operationOverrides.previewBundle ?? previewBundle)(
          { target, requestedTarget: target, reset: false },
          {},
        );
        resolvedBundles.set(target, base);
      }
      return writeRolloutVersionLock(name, {
        target,
        versions: { ...base.env, ...options.versions },
        sources: [...base.sources, ...(options.sources ?? ["rollout-runbook"])],
      });
    },
    async writeVersionLock(name, options) {
      return writeRolloutVersionLock(name, options);
    },
  };
};

export const loadRolloutRunbook = async (script: string): Promise<RolloutRunbook> => {
  const file = path.resolve(script);
  const loaded = await import(`${pathToFileURL(file).href}?t=${Date.now()}`);
  const runbook = loaded.default ?? loaded.run;
  if (typeof runbook !== "function") {
    throw new PreflightError(`Rollout runbook ${script} must export a default function or named run(ctx) function`);
  }
  return runbook as RolloutRunbook;
};

export const runRolloutRunbook = async (script: string, ctx?: RolloutRunContext) => {
  if (!script) {
    throw new PreflightError("rollout run expects a runbook path");
  }
  const receipt = ctx ? undefined : createRolloutReceipt();
  const context = ctx ?? createRolloutContext(receipt);
  await receipt?.start(path.resolve(script));
  try {
    await (
      await loadRolloutRunbook(script)
    )(context);
    await receipt?.record("complete", "runbook completed");
  } catch (error) {
    try {
      await receipt?.record("failed", "runbook failed", {
        details: { error: error instanceof Error ? error.message : String(error) },
        diagnostics: true,
      });
    } catch (recordError) {
      console.error("[receipt] failed to record runbook failure", recordError);
    }
    throw error;
  }
};
