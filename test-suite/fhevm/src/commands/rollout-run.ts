/**
 * Executes release-specific rollout runbooks against the local fhevm stack.
 */
import path from "node:path";
import { pathToFileURL } from "node:url";

import { PreflightError } from "../errors";
import { runContractTask, snapshotContractSources } from "../flow/contracts";
import { waitForTestSuite } from "../flow/readiness";
import { composeUp } from "../flow/runtime-compose";
import {
  applyVersionLock as applyStackVersionLock,
  up,
  startDeferredGreen as startStackDeferredGreen,
  upgradeThresholdKmsOperator,
  upgradeRuntimeGroup as upgradeStackRuntimeGroup,
} from "../flow/up-flow";
import { STATE_DIR } from "../layout";
import { previewBundle } from "../resolve/bundle-store";
import { loadState } from "../state/state";
import type { LocalOverride, State, UpOptions, VersionBundle, VersionTarget } from "../types";
import { ensureDir, writeJson } from "../utils/fs";
import { type RolloutReceipt, createRolloutReceipt } from "./rollout-receipt";
import { prepareRolloutSolidity } from "./rollout-solidity";
import { test as runTest } from "./test";

type RolloutUpOptions = {
  lockFile: string;
  overrides?: LocalOverride[];
  scenario?: string;
};

type RolloutRuntimeUpgradeOptions = {
  lockFile?: string;
  bcsTag?: string;
  bcsCompatTag?: string;
};
type RolloutKmsOperatorUpgradeOptions = {
  lockFile: string;
  migration?: NonNullable<State["kmsMigrationByNodeId"]>[string];
  overrides?: LocalOverride[];
};
type RolloutVersionLockOptions = {
  allowedVersionKeys: string[];
  lockFile: string;
  overrides?: LocalOverride[];
};

type RolloutTestOptions = {
  /** Resolved baseline commit for fixtures that must also run on an older host ABI. */
  solidityLibraryRevision?: string;
  grep?: string;
  network?: string;
  noHardhatCompile?: boolean;
  parallel?: boolean;
};
type RolloutContractTaskOptions = {
  env?: Record<string, string>;
};
type RolloutLockOptions = {
  sha?: string;
  versions: Record<string, string>;
  sources?: string[];
  target?: VersionTarget;
};

export type RolloutRunContext = {
  applyVersionLock(label: string, options: RolloutVersionLockOptions): Promise<void>;
  readState(): Promise<State>;
  runGatewayContractTask(command: string, options?: RolloutContractTaskOptions): Promise<void>;
  runHostContractTask(command: string, options?: RolloutContractTaskOptions): Promise<void>;
  snapshotContracts(surface: "host" | "gateway"): Promise<void>;
  stateDir(): string;
  test(profile?: string, options?: RolloutTestOptions): Promise<void>;
  up(options: RolloutUpOptions): Promise<void>;
  /** Sequentially upgrades each listed serving KMS Core and its matching Connector as one operation. */
  upgradeKmsOperators(operatorIds: readonly number[], options: RolloutKmsOperatorUpgradeOptions): Promise<void>;
  upgradeRuntimeGroup(group: string, options?: RolloutRuntimeUpgradeOptions): Promise<void>;
  /** Starts a Green fleet after prerequisite material has converged on Blue. */
  startDeferredGreen(): Promise<void>;
  resolveVersionLock(name: string, options: RolloutLockOptions): Promise<string>;
  writeVersionLock(name: string, options: RolloutLockOptions): Promise<string>;
};

export type RolloutRunbook = (ctx: RolloutRunContext) => Promise<void> | void;

type RolloutContextOperations = {
  previewBundle: typeof previewBundle;
  upgradeThresholdKmsOperator: typeof upgradeThresholdKmsOperator;
};

/** Rollouts resolve their surrounding stack from current main unless a runbook pins another target. */
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
  if (options.solidityLibraryRevision) {
    const library = await prepareRolloutSolidity(options.solidityLibraryRevision);
    await receipt.record("fixture-solidity", "compiled fixtures against pinned baseline Solidity", {
      details: { profile, ...library },
    });
  }
  await runTest(profile, {
    network: options.network ?? "staging",
    verbose: false,
    noHardhatCompile: options.noHardhatCompile ?? true,
    parallel: options.parallel,
    grep: options.grep,
  });
};

const writeRolloutVersionLock = async (name: string, options: RolloutLockOptions) => {
  const filename = name.endsWith(".json") ? name : `${name}.lock.json`;
  const file = path.join(STATE_DIR, "rollout", filename);
  await ensureDir(path.dirname(file));
  const bundle = {
    target: options.target ?? ROLLOUT_TARGET,
    lockName: path.basename(file),
    env: options.versions,
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
  const resolvedBundles = new Map<string, VersionBundle>();

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
    async readState() {
      const state = await loadState();
      if (!state) {
        throw new PreflightError("Stack is not running; run ctx.up(...) first");
      }
      return state;
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
    async upgradeKmsOperators(operatorIds, options) {
      const state = await loadState();
      if (!state) {
        throw new PreflightError("upgradeKmsOperators requires a running stack");
      }
      if (operatorIds.length === 0) {
        throw new PreflightError("upgradeKmsOperators requires at least one operator id");
      }
      const selected = new Set<number>();
      for (const operatorId of operatorIds) {
        if (!Number.isInteger(operatorId) || operatorId < 1 || operatorId > state.scenario.kms.committeeSize) {
          throw new PreflightError(
            `upgradeKmsOperators expects serving operator ids between 1 and ${state.scenario.kms.committeeSize}; received ${operatorId}`,
          );
        }
        if (selected.has(operatorId)) {
          throw new PreflightError(`upgradeKmsOperators received duplicate operator id ${operatorId}`);
        }
        selected.add(operatorId);
      }
      for (const operatorId of operatorIds) {
        try {
          await (operationOverrides.upgradeThresholdKmsOperator ?? upgradeThresholdKmsOperator)(operatorId, options);
        } catch (error) {
          try {
            await receipt.record("upgrade-kms-operator-failed", `KMS operator ${operatorId}`, {
              details: { error: error instanceof Error ? error.message : String(error), operatorId },
              docker: true,
              lockFile: options.lockFile,
            });
          } catch (receiptError) {
            throw new AggregateError(
              [error, receiptError],
              `KMS operator ${operatorId} upgrade failed and its required receipt snapshot also failed`,
            );
          }
          throw error;
        }
        await receipt.record("upgrade-kms-operator", `KMS operator ${operatorId}`, {
          details: { operatorId },
          docker: true,
          lockFile: options.lockFile,
        });
      }
    },
    async upgradeRuntimeGroup(group, options = {}) {
      await upgradeStackRuntimeGroup(group, options);
      await receipt.record("upgrade-runtime", group, { lockFile: options.lockFile });
    },
    async startDeferredGreen() {
      await startStackDeferredGreen();
      await receipt.record("start-green", "started deferred Green fleet", { docker: true });
    },
    async resolveVersionLock(name, options) {
      const target = options.target ?? ROLLOUT_TARGET;
      const cacheKey = `${target}:${options.sha ?? ""}`;
      let base = resolvedBundles.get(cacheKey);
      if (!base) {
        base = await (operationOverrides.previewBundle ?? previewBundle)(
          { target, requestedTarget: target, sha: options.sha, reset: false },
          {},
        );
        resolvedBundles.set(cacheKey, base);
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
