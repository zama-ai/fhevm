import path from "node:path";

import { assertSupportedBundleScenario, validateBundleCompatibility } from "../compat/compat";
import { generateRuntime } from "../generate";
import {
  COPROCESSOR_DB_CONTAINER,
  DEFAULT_POSTGRES_PASSWORD,
  DEFAULT_POSTGRES_USER,
  KMS_CORE_CONTAINER,
} from "../layout";
import { loadState, markStep, saveState } from "../state/state";
import { stackSpecForState } from "../stack-spec/stack-spec";
import type { LocalOverride, OverrideGroup, State, VersionBundle, VersionTarget } from "../types";
import { OVERRIDE_GROUPS } from "../types";
import { IncompatibleVersions, PreflightError } from "../errors";
import { readJson } from "../utils/fs";
import { run } from "../utils/process";
import { ensureRuntimeArtifacts } from "./artifacts";
import { discoverContracts, ensureDiscovery, validateDiscovery } from "./discovery";
import {
  KMS_CONNECTOR_HEALTH_CONTAINERS,
  coprocessorHealthContainers,
  postBootHealthGate,
  waitForContainer,
  waitForCoprocessor,
  waitForKmsConnector,
  waitForLog,
  waitForStableChainListeners,
  waitForTestSuite,
} from "./readiness";
import { composeUp, maybeBuild, multiChainComposeUp, projectContainers } from "./runtime-compose";
import { assertSchemaCompatibility } from "./schema";
import {
  multiChainCoprocessorUpgradeTargets,
  resolveUpgradePlan,
  type UpgradeGroup,
} from "./upgrade-plan";

type UpgradeOptions = {
  lockFile?: string;
};

const NETWORK_TARGETS: ReadonlySet<string> = new Set(["devnet", "testnet", "mainnet"]);

const targetNeedsGitHub = (options: { target: VersionTarget; lockFile?: string }) =>
  !options.lockFile && options.target !== "latest-supported";

const assertSupportedTargetScenario = (target: VersionTarget, scenario: State["scenario"]) => {
  if (NETWORK_TARGETS.has(target) && scenario.hostChains.length > 1) {
    throw new PreflightError(
      `--target ${target} does not currently support multi-chain scenarios; rerun without --scenario multi-chain or use latest-main`,
    );
  }
};

const postgresExecOptions = () => ({
  user: process.env.POSTGRES_USER ?? DEFAULT_POSTGRES_USER,
  password: process.env.POSTGRES_PASSWORD ?? DEFAULT_POSTGRES_PASSWORD,
});

const postgresExec = async (dbName: string, args: string[]) => {
  const postgres = postgresExecOptions();
  return run(
    [
      "docker",
      "exec",
      "-e",
      `PGPASSWORD=${postgres.password}`,
      COPROCESSOR_DB_CONTAINER,
      "psql",
      "-U",
      postgres.user,
      "-d",
      dbName,
      ...args,
    ],
    { allowFailure: true },
  );
};

export const changedVersionKeys = (current: VersionBundle, next: VersionBundle) =>
  [...new Set([...Object.keys(current.env), ...Object.keys(next.env)])]
    .filter((key) => (current.env[key] ?? "") !== (next.env[key] ?? ""))
    .sort();

export const assertVersionLockChanges = (label: string, allowedVersionKeys: readonly string[], changedKeys: string[]) => {
  const disallowed = changedKeys.filter((key) => !allowedVersionKeys.includes(key));
  if (disallowed.length) {
    throw new PreflightError(
      `${label} lock changes unrelated version keys: ${disallowed.join(", ")}. Allowed: ${allowedVersionKeys.join(", ")}`,
    );
  }
};

const isOverrideGroup = (group: string): group is OverrideGroup =>
  OVERRIDE_GROUPS.includes(group as OverrideGroup);

export const runtimeUpgradeOverrideGroups = (group: UpgradeGroup): OverrideGroup[] => {
  if (group === "kms") {
    return ["kms-connector"];
  }
  if (group === "kms-core") {
    return [];
  }
  return isOverrideGroup(group) ? [group] : [];
};

export const removeRuntimeUpgradeOverrides = (overrides: LocalOverride[], group: UpgradeGroup) => {
  const upgraded = new Set(runtimeUpgradeOverrideGroups(group));
  return overrides.filter((override) => !upgraded.has(override.group));
};

const mergeLocalOverrides = (current: LocalOverride[], additions: LocalOverride[] = []) => {
  const seen = new Set(current.map((override) => JSON.stringify(override)));
  const next = [...current];
  for (const override of additions) {
    const key = JSON.stringify(override);
    if (!seen.has(key)) {
      seen.add(key);
      next.push(override);
    }
  }
  return next;
};

const buildLockedState = async (
  label: string,
  state: State,
  lockFile: string,
  allowedVersionKeys: readonly string[],
  nextOverrides: LocalOverride[],
) => {
  const lockPath = path.resolve(lockFile);
  const next = await readJson<VersionBundle>(lockPath);
  if (!next.env || typeof next.env !== "object") {
    throw new PreflightError(`Invalid ${label} lock ${lockFile}: missing env map`);
  }
  const changedKeys = changedVersionKeys(state.versions, next);
  assertVersionLockChanges(label, allowedVersionKeys, changedKeys);
  const nextState = {
    ...state,
    target: next.target,
    lockPath,
    requiresGitHub: targetNeedsGitHub({ target: next.target, lockFile }),
    overrides: nextOverrides,
    versions: next,
    updatedAt: new Date().toISOString(),
  } satisfies State;
  assertSupportedTargetScenario(nextState.target, nextState.scenario);
  assertSupportedBundleScenario({
    versions: nextState.versions,
    overrides: nextState.overrides,
    scenario: nextState.scenario,
  });
  const incompatibilities = validateBundleCompatibility(nextState);
  if (incompatibilities.length) {
    throw new IncompatibleVersions(incompatibilities.map((item) => item.message));
  }
  return { changedKeys, state: nextState };
};

const applyRuntimeUpgradeLock = (
  state: State,
  group: UpgradeGroup,
  allowedVersionKeys: readonly string[],
  lockFile: string,
) =>
  buildLockedState(
    `upgrade ${group}`,
    state,
    lockFile,
    allowedVersionKeys,
    removeRuntimeUpgradeOverrides(state.overrides, group),
  );

/** Applies an ordered rollout version lock without restarting runtime services. */
export const applyVersionLock = async (
  label: string,
  lockFile: string,
  allowedVersionKeys: readonly string[],
  options: { overrides?: LocalOverride[] } = {},
) => {
  const state = await loadState();
  if (!state || !(await projectContainers()).length) {
    throw new PreflightError("Stack is not running; start one with `fhevm-cli up` first");
  }
  await ensureRuntimeArtifacts(state, "rollout version lock");
  const { changedKeys, state: nextState } = await buildLockedState(
    label,
    state,
    lockFile,
    allowedVersionKeys,
    mergeLocalOverrides(state.overrides, options.overrides),
  );
  await assertSchemaCompatibility(nextState.versions, nextState.overrides, nextState.scenario, false);
  await saveState(nextState);
  await generateRuntime(nextState, stackSpecForState(nextState));
  console.log(`[rollout] ${label} versions=${changedKeys.join(", ") || "(none)"}`);
};

/** Re-reads deployed contract addresses after contract runbook tasks mutate proxies. */
export const refreshDiscovery = async () => {
  const state = await loadState();
  if (!state || !(await projectContainers()).length) {
    throw new PreflightError("Stack is not running; start one with `fhevm-cli up` first");
  }
  await ensureRuntimeArtifacts(state, "rollout discovery");
  const contracts = await discoverContracts(state);
  const discovery = await ensureDiscovery(state);
  discovery.gateway = contracts.gateway;
  discovery.hosts = { ...discovery.hosts, ...contracts.hosts };
  validateDiscovery(state);
  await saveState(state);
  await generateRuntime(state, stackSpecForState(state));
  console.log("[rollout] discovery refreshed");
};

const waitForRelayer = async () => {
  await waitForContainer("fhevm-relayer-db", "healthy");
  await waitForContainer("fhevm-relayer", "running");
  await waitForLog("fhevm-relayer", /All servers are ready and responding/);
};

const waitForUpgrade = async (state: State, group: UpgradeGroup, runtimeServices: string[]) => {
  if (group === "coprocessor") {
    const extraTargets = multiChainCoprocessorUpgradeTargets(state, runtimeServices);
    for (const target of extraTargets) {
      if (target.services.length) {
        await multiChainComposeUp(target.compose, target.services);
      }
      await waitForStableChainListeners(state, target.chainKey);
    }
    await waitForCoprocessor(state);
    await postBootHealthGate([...coprocessorHealthContainers(state), ...extraTargets.flatMap((target) => target.services)]);
    return;
  }
  if (group === "kms-connector" || group === "kms") {
    await waitForContainer(KMS_CORE_CONTAINER, "running");
    await waitForKmsConnector(state);
    await postBootHealthGate(KMS_CONNECTOR_HEALTH_CONTAINERS);
    return;
  }
  if (group === "kms-core") {
    await waitForContainer(KMS_CORE_CONTAINER, "running");
    return;
  }
  if (group === "listener-core") {
    await waitForContainer("listener-redis", "running");
    await waitForContainer("listener-publisher-for-anvil", "running");
    return;
  }
  if (group === "relayer") {
    await waitForRelayer();
    return;
  }
  await waitForTestSuite();
};

/** Upgrades one runtime group in place, including allowed migrations and optional version-lock application. */
export const upgradeRuntimeGroup = async (groupValue: string | undefined, options: UpgradeOptions = {}) => {
  const state = await loadState();
  if (!state || !(await projectContainers()).length) {
    throw new PreflightError(
      "Stack is not running; start one with `fhevm-cli up --override ...` or `fhevm-cli up --scenario ...` first",
    );
  }
  await ensureRuntimeArtifacts(state, "upgrade");
  const plan = resolveUpgradePlan(state, groupValue, { lockFile: !!options.lockFile });
  for (const step of plan.steps) {
    if (!state.completedSteps.includes(step)) {
      throw new PreflightError(`upgrade requires a stack that has completed the ${step} step`);
    }
  }
  const nextState = options.lockFile
    ? (await applyRuntimeUpgradeLock(state, plan.group, plan.versionKeys, options.lockFile)).state
    : state;
  await assertSchemaCompatibility(nextState.versions, nextState.overrides, nextState.scenario, false);
  console.log(`[upgrade] ${plan.group}`);
  await saveState(nextState);
  await generateRuntime(nextState, stackSpecForState(nextState));
  if (plan.group === "listener-core") {
    await postgresExec("", ["-c", "CREATE DATABASE listener;"]);
  }
  for (const component of plan.components) {
    await maybeBuild(component.component, nextState, { force: true });
  }
  for (const component of plan.components) {
    if (!component.migrationServices.length) {
      continue;
    }
    await composeUp(component.component, component.migrationServices, { forceRecreate: true });
    for (const service of component.migrationServices) {
      await waitForContainer(service, "complete");
    }
  }
  for (const component of plan.components) {
    await composeUp(component.component, component.runtimeServices, { noDeps: true });
  }
  await waitForUpgrade(nextState, plan.group, plan.runtimeServices);
  for (const step of plan.steps) {
    await markStep(nextState, step);
  }
};
