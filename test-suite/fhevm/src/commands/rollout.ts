import path from "node:path";

import { PreflightError } from "../errors";
import { PACKAGE_TO_REPOSITORY } from "../resolve/target";
import type { VersionBundle } from "../types";
import { ensureDir, readJson, writeJson } from "../utils/fs";

type RolloutMatrixEntry = {
  step: string;
  stepIndex: number;
  name: string;
};
type RolloutMatrix = {
  include: RolloutMatrixEntry[];
};
type CompatStepDefinition = {
  name: string;
  units?: string[];
  substeps?: Array<{
    name: string;
    units: string[];
  }>;
};
type ExpandedCompatStep = {
  label: string;
  units: string[];
};
type CompatHarnessDefinition = {
  testSuiteVersion: string;
  relayerSdkVersion: string;
};

export type CompatTestDefinition = {
  name: string;
  description?: string;
  from: Record<string, string>;
  to: Record<string, string>;
  harness?: CompatHarnessDefinition;
  steps: CompatStepDefinition[];
  units: Record<string, string[]>;
  execution?: {
    scenario?: string;
  };
};

const REQUIRED_VERSION_KEYS = Object.keys(PACKAGE_TO_REPOSITORY).sort();
const lockStem = (index: number, label: string) => `${String(index).padStart(2, "0")}-${label}`;
const slug = (value: string) => value.toLowerCase().replaceAll(/[^a-z0-9]+/g, "-").replaceAll(/^-+|-+$/g, "");
const rolloutSources = (test: CompatTestDefinition, step: string) => [`compat-test=${test.name}`, `rollout-step=${step}`];
const compatContractsFromSources = (test: CompatTestDefinition) =>
  ["GATEWAY_VERSION", "HOST_VERSION"].map((key) => `compat-from:${key}=${test.from[key]}`);
const parseCompatVersion = (version: string) => /^v?\d+\.\d+\.\d+(?:[-+].*)?$/.test(version);
const compatClientEnv = (test: CompatTestDefinition): Record<string, string> =>
  ({
    TEST_SUITE_VERSION: test.harness!.testSuiteVersion,
    RELAYER_SDK_VERSION: test.harness!.relayerSdkVersion,
  });

const validateStepName = (kind: string, value: unknown) => {
  if (typeof value !== "string" || !value.trim()) {
    throw new PreflightError(`compat-test ${kind} must include a non-empty name`);
  }
  return value.trim();
};

const validateStepUnits = (kind: string, units: unknown) => {
  if (!Array.isArray(units) || !units.length || units.some((unit) => typeof unit !== "string" || !unit.length)) {
    throw new PreflightError(`compat-test ${kind} must list one or more rollout units`);
  }
  return units;
};

const expandCompatSteps = (steps: CompatStepDefinition[]) => {
  if (!Array.isArray(steps) || !steps.length) {
    throw new PreflightError("compat-test steps must include at least one rollout step");
  }
  const labels = new Set<string>();
  const expanded: ExpandedCompatStep[] = [];
  for (const [index, step] of steps.entries()) {
    const stepName = validateStepName(`step[${index}]`, step?.name);
    const hasUnits = Array.isArray(step.units) && step.units.length > 0;
    const hasSubsteps = Array.isArray(step.substeps) && step.substeps.length > 0;
    if (hasUnits === hasSubsteps) {
      throw new PreflightError(`compat-test step ${stepName} must define exactly one of units or substeps`);
    }
    if (hasUnits) {
      const label = slug(stepName);
      if (labels.has(label)) {
        throw new PreflightError(`Duplicate rollout step labels: ${label}`);
      }
      labels.add(label);
      expanded.push({
        label,
        units: validateStepUnits(`step ${stepName}`, step.units),
      });
      continue;
    }
    for (const [subIndex, substep] of step.substeps!.entries()) {
      const substepName = validateStepName(`step ${stepName} substep[${subIndex}]`, substep?.name);
      const label = `${slug(stepName)}-${slug(substepName)}`;
      if (labels.has(label)) {
        throw new PreflightError(`Duplicate rollout step labels: ${label}`);
      }
      labels.add(label);
      expanded.push({
        label,
        units: validateStepUnits(`step ${stepName} substep ${substepName}`, substep.units),
      });
    }
  }
  return expanded;
};

/** Validates that a compat-test env map contains every required version key. */
const validateEnvMap = (
  label: string,
  value: Record<string, string>,
  clientEnv: Record<string, string>,
) => {
  const withClientEnv = { ...value, ...clientEnv };
  const missing = REQUIRED_VERSION_KEYS.filter((key) => typeof withClientEnv[key] !== "string" || !withClientEnv[key]?.length);
  if (missing.length) {
    throw new PreflightError(`compat-test ${label} is missing required version keys: ${missing.join(", ")}`);
  }
  return Object.fromEntries(REQUIRED_VERSION_KEYS.map((key) => [key, withClientEnv[key]]));
};

/** Validates unit definitions declared by one compat-test and returns the covered keys. */
const validateCompatUnits = (units: Record<string, string[]>, env: Record<string, string>) => {
  const names = Object.keys(units);
  if (!names.length) {
    throw new PreflightError("compat-test must define at least one rollout unit");
  }
  const referenced = new Set<string>();
  for (const name of names) {
    const keys = units[name];
    if (!Array.isArray(keys) || !keys.length || keys.some((key) => typeof key !== "string" || !key.length)) {
      throw new PreflightError(`compat-test unit ${name} must list one or more version keys`);
    }
    for (const key of keys) {
      if (!(key in env)) {
        throw new PreflightError(`compat-test unit ${name} references unknown version key ${key}`);
      }
      if (referenced.has(key)) {
        throw new PreflightError(`compat-test version key ${key} is assigned to multiple units`);
      }
      referenced.add(key);
    }
  }
  return referenced;
};

/** Validates ordered rollout steps, unit names, and duplicate coverage. */
export const validateCompatSteps = (steps: CompatStepDefinition[], units: Record<string, string[]>) => {
  const unitNames = Object.keys(units);
  const flattened = expandCompatSteps(steps).flatMap((step) => step.units);
  const unknown = flattened.filter((unit) => !unitNames.includes(unit));
  if (unknown.length) {
    throw new PreflightError(`Unknown rollout units: ${[...new Set(unknown)].join(", ")}. Valid: ${unitNames.join(", ")}`);
  }
  const duplicates = flattened.filter((unit, index) => flattened.indexOf(unit) !== index);
  if (duplicates.length) {
    throw new PreflightError(`Duplicate rollout units: ${[...new Set(duplicates)].join(", ")}`);
  }
  const missing = unitNames.filter((unit) => !flattened.includes(unit));
  if (missing.length || flattened.length !== unitNames.length) {
    throw new PreflightError(`compat-test steps must include each unit exactly once: ${unitNames.join(", ")}`);
  }
  return steps;
};

/** Loads and validates one checked-in compat-test definition. */
export const readCompatTest = async (file: string) => {
  const test = await readJson<CompatTestDefinition>(path.resolve(file));
  if (!test?.name) {
    throw new PreflightError("compat-test must include a non-empty name");
  }
  if (!test.harness?.testSuiteVersion || !parseCompatVersion(test.harness.testSuiteVersion)) {
    throw new PreflightError("compat-test harness.testSuiteVersion must be set explicitly to a semver tag");
  }
  if (!test.harness.relayerSdkVersion || typeof test.harness.relayerSdkVersion !== "string" || !test.harness.relayerSdkVersion.length) {
    throw new PreflightError("compat-test harness.relayerSdkVersion must be set explicitly");
  }
  const harness = { ...test.harness };
  const clientEnv = compatClientEnv({ ...test, harness });
  const validated = {
    ...test,
    harness,
    from: validateEnvMap("from", test.from, clientEnv),
    to: validateEnvMap("to", test.to, clientEnv),
  } satisfies CompatTestDefinition;
  const fromCovered = validateCompatUnits(validated.units, validated.from);
  const toCovered = validateCompatUnits(validated.units, validated.to);
  const uncovered = REQUIRED_VERSION_KEYS.filter((key) => !fromCovered.has(key) || !toCovered.has(key));
  const changingUncovered = uncovered.filter((key) => validated.from[key] !== validated.to[key]);
  if (changingUncovered.length) {
    throw new PreflightError(`compat-test units do not cover changing version keys: ${changingUncovered.join(", ")}`);
  }
  validateCompatSteps(validated.steps, validated.units);
  return validated;
};

const bundleFromEnv = (test: CompatTestDefinition, kind: "from" | "to"): VersionBundle => ({
  target: "latest-supported",
  lockName: `${slug(test.name)}-${kind}.json`,
  env: { ...test[kind], ...compatClientEnv(test) },
  sources: [`compat-test=${test.name}`, kind],
});

const rolloutEntries = (test: CompatTestDefinition) => {
  const expanded = expandCompatSteps(test.steps);
  return [
    {
      step: "baseline",
      stepIndex: 0,
      name: lockStem(0, "baseline").replace(/\.lock\.json$/, ""),
    },
    ...expanded.map((step, index) => ({
      step: step.label,
      stepIndex: index + 1,
      name: lockStem(index + 1, step.label).replace(/\.lock\.json$/, ""),
    })),
  ] satisfies RolloutMatrix["include"];
};

/** Generates the baseline and cumulative mixed-version rollout locks for one compat-test. */
export const generateRolloutLocks = (test: CompatTestDefinition) => {
  const from = bundleFromEnv(test, "from");
  const to = bundleFromEnv(test, "to");
  const current = { ...from.env };
  const baseline: VersionBundle = {
    ...from,
    lockName: `${lockStem(0, "baseline")}.lock.json`,
    sources: [...from.sources, ...compatContractsFromSources(test), ...rolloutSources(test, "baseline")],
  };
  return [
    baseline,
    ...expandCompatSteps(test.steps).map((step, index) => {
      for (const unit of step.units) {
        for (const key of test.units[unit]) {
          current[key] = to.env[key];
        }
      }
      return {
        ...to,
        env: { ...current },
        lockName: `${lockStem(index + 1, step.label)}.lock.json`,
        sources: [...to.sources, ...compatContractsFromSources(test), ...rolloutSources(test, step.label)],
      } satisfies VersionBundle;
    }),
  ];
};

/** Returns the GitHub Actions matrix descriptor for one compat-test. */
export const rolloutMatrix = (test: CompatTestDefinition): RolloutMatrix => ({
  include: rolloutEntries(test),
});

/** Returns one rendered rollout lock for a specific matrix step index. */
export const renderRolloutStep = (test: CompatTestDefinition, stepIndex: number): VersionBundle => {
  const expanded = expandCompatSteps(test.steps);
  if (!Number.isInteger(stepIndex) || stepIndex < 0 || stepIndex > expanded.length) {
    throw new PreflightError(`rollout step must be an integer between 0 and ${expanded.length}`);
  }
  return generateRolloutLocks(test)[stepIndex];
};

/** Writes one rollout lock file for a specific compat-test step. */
export const rolloutStep = async (options: { compatTest: string; out: string; step: number }) => {
  if (!options.compatTest) throw new PreflightError("rollout requires --compat-test <file>");
  if (!options.out) throw new PreflightError("rollout requires --out <file>");
  const test = await readCompatTest(options.compatTest);
  const bundle = renderRolloutStep(test, options.step);
  await ensureDir(path.dirname(options.out));
  await writeJson(options.out, { ...bundle, lockName: path.basename(options.out) });
  console.log(options.out);
};

/** Prints the rollout matrix JSON for one compat-test. */
export const printRolloutMatrix = async (options: { compatTest: string }) => {
  if (!options.compatTest) throw new PreflightError("rollout requires --compat-test <file>");
  console.log(JSON.stringify(rolloutMatrix(await readCompatTest(options.compatTest))));
};

/** Writes rollout lock files and the GitHub Actions matrix descriptor into one output directory. */
export const rollout = async (options: { compatTest: string; out: string }) => {
  if (!options.compatTest) throw new PreflightError("rollout requires --compat-test <file>");
  if (!options.out) throw new PreflightError("rollout requires --out <directory>");
  const test = await readCompatTest(options.compatTest);
  const locks = generateRolloutLocks(test);
  const matrix = rolloutMatrix(test);
  await ensureDir(options.out);
  await Promise.all([
    ...locks.map((bundle) => writeJson(path.join(options.out, bundle.lockName), bundle)),
    writeJson(path.join(options.out, "matrix.json"), matrix),
  ]);
  for (const file of [...locks.map((bundle) => bundle.lockName), "matrix.json"]) {
    console.log(path.join(options.out, file));
  }
};
