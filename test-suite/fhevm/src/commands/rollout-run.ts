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
  refreshDiscovery as refreshStackDiscovery,
  up,
  upgradeRuntimeGroup as upgradeStackRuntimeGroup,
} from "../flow/up-flow";
import { STATE_DIR } from "../layout";
import { loadState } from "../state/state";
import type { LocalOverride, State, UpOptions, VersionBundle, VersionTarget } from "../types";
import { ensureDir, writeJson } from "../utils/fs";
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
type RolloutVersionLockOptions = {
  allowedVersionKeys: string[];
  lockFile: string;
  overrides?: LocalOverride[];
};

type RolloutTestOptions = {
  network?: string;
  noHardhatCompile?: boolean;
  parallel?: boolean;
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
  readState(): Promise<State>;
  refreshDiscovery(): Promise<void>;
  runGatewayContractTask(command: string, options?: RolloutContractTaskOptions): Promise<void>;
  runHostContractTask(command: string, options?: RolloutContractTaskOptions): Promise<void>;
  snapshotContracts(surface: "host" | "gateway"): Promise<void>;
  stateDir(): string;
  test(profile?: string, options?: RolloutTestOptions): Promise<void>;
  up(options: RolloutUpOptions): Promise<void>;
  upgradeRuntimeGroup(group: string, options?: RolloutRuntimeUpgradeOptions): Promise<void>;
  writeVersionLock(name: string, options: RolloutLockOptions): Promise<string>;
};

export type RolloutRunbook = (ctx: RolloutRunContext) => Promise<void> | void;

const upOptions = (options: RolloutUpOptions): UpOptions => ({
  target: "latest-main",
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

export const createRolloutContext = (receipt: RolloutReceipt = createRolloutReceipt()): RolloutRunContext => ({
  async applyVersionLock(label, options) {
    await applyStackVersionLock(label, options.lockFile, options.allowedVersionKeys, { overrides: options.overrides });
    await receipt.record("apply-version-lock", label, {
      details: {
        allowedVersionKeys: options.allowedVersionKeys,
        overrides: (options.overrides ?? []).map((override) => override.group),
      },
      docker: true,
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
  async refreshDiscovery() {
    await refreshStackDiscovery();
    await receipt.record("refresh-discovery", "refreshed runtime addresses");
  },
  async runGatewayContractTask(command, options = {}) {
    await runContractTask("gateway-sc", "gateway-sc-deploy", command, options);
    await receipt.record("gateway-contract-task", command, {
      details: { envKeys: Object.keys(options.env ?? {}).sort() },
      docker: true,
    });
  },
  async runHostContractTask(command, options = {}) {
    await runContractTask("host-sc", "host-sc-deploy", command, options);
    await receipt.record("host-contract-task", command, {
      details: { envKeys: Object.keys(options.env ?? {}).sort() },
      docker: true,
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
    await refreshTestSuiteContainer();
    await receipt.record("refresh-test-suite", "recreated test-suite container with current env", {
      details: { profile },
      docker: true,
    });
    await runTest(profile, {
      network: options.network ?? "staging",
      verbose: false,
      noHardhatCompile: options.noHardhatCompile ?? true,
      parallel: options.parallel,
    });
    await receipt.record("test", `${profile} passed`, {
      details: {
        network: options.network ?? "staging",
        noHardhatCompile: options.noHardhatCompile ?? true,
        ...(options.parallel === undefined ? {} : { parallel: options.parallel }),
      },
    });
  },
  async up(options) {
    await up(upOptions(options));
    await receipt.record("up", "boot stack", {
      details: { overrides: (options.overrides ?? []).map((override) => override.group), scenario: options.scenario },
      docker: true,
      lockFile: options.lockFile,
    });
  },
  async upgradeRuntimeGroup(group, options = {}) {
    await upgradeStackRuntimeGroup(group, options);
    await receipt.record("upgrade-runtime", group, { docker: true, lockFile: options.lockFile });
  },
  async writeVersionLock(name, options) {
    const filename = name.endsWith(".json") ? name : `${name}.lock.json`;
    const file = path.join(STATE_DIR, "rollout", filename);
    await ensureDir(path.dirname(file));
    const bundle = {
      target: options.target ?? "latest-main",
      lockName: path.basename(file),
      env: options.versions,
      sources: options.sources ?? ["rollout-runbook"],
    } satisfies VersionBundle;
    await writeJson(file, bundle);
    return file;
  },
});

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
    await receipt?.record("complete", "runbook completed", { docker: true });
  } catch (error) {
    try {
      await receipt?.record("failed", "runbook failed", {
        details: { error: error instanceof Error ? error.message : String(error) },
        docker: true,
      });
    } catch (recordError) {
      console.error("[receipt] failed to record runbook failure", recordError);
    }
    throw error;
  }
};
