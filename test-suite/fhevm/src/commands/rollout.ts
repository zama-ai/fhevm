import path from "node:path";

import { PreflightError } from "../errors";
import { PACKAGE_TO_REPOSITORY } from "../resolve/target";
import type { VersionBundle } from "../types";
import { ensureDir, readJson, writeJson } from "../utils/fs";

type RolloutStep = string[];
type RolloutMatrix = {
  include: Array<{ step: string; stepIndex: number; name: string }>;
};
export type CompatTestDefinition = {
  name: string;
  description?: string;
  from: Record<string, string>;
  to: Record<string, string>;
  steps: RolloutStep[];
  units: Record<string, string[]>;
  execution?: {
    scenario?: string;
    testProfile?: string;
  };
};

const REQUIRED_VERSION_KEYS = Object.keys(PACKAGE_TO_REPOSITORY).sort();
const lockStem = (index: number, label: string) => `${String(index).padStart(2, "0")}-${label}`;
const slug = (value: string) => value.toLowerCase().replaceAll(/[^a-z0-9]+/g, "-").replaceAll(/^-+|-+$/g, "");
const stepLabel = (step: readonly string[]) => step.map((unit) => slug(unit)).join("_");
const rolloutSources = (test: CompatTestDefinition, step: string) => [`compat-test=${test.name}`, `rollout-step=${step}`];

/** Validates that a compat-test env map contains every required version key. */
const validateEnvMap = (label: string, value: Record<string, string>) => {
  const missing = REQUIRED_VERSION_KEYS.filter((key) => typeof value[key] !== "string" || !value[key]?.length);
  if (missing.length) {
    throw new PreflightError(`compat-test ${label} is missing required version keys: ${missing.join(", ")}`);
  }
  return Object.fromEntries(REQUIRED_VERSION_KEYS.map((key) => [key, value[key]]));
};

/** Validates unit definitions declared by one compat-test. */
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
  const missing = Object.keys(env).filter((key) => !referenced.has(key));
  if (missing.length) {
    throw new PreflightError(`compat-test units do not cover required version keys: ${missing.join(", ")}`);
  }
};

/** Validates ordered rollout steps, unit names, and duplicate coverage. */
export const validateCompatSteps = (steps: RolloutStep[], units: Record<string, string[]>) => {
  const unitNames = Object.keys(units);
  if (!Array.isArray(steps) || !steps.length) {
    throw new PreflightError(`compat-test steps must include ${unitNames.join(", ")}`);
  }
  const flattened = steps.flatMap((step) => {
    if (!Array.isArray(step) || !step.length) {
      throw new PreflightError("compat-test steps must be non-empty unit arrays");
    }
    return step;
  });
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
  const validated = {
    ...test,
    from: validateEnvMap("from", test.from),
    to: validateEnvMap("to", test.to),
  } satisfies CompatTestDefinition;
  validateCompatUnits(validated.units, validated.from);
  validateCompatUnits(validated.units, validated.to);
  validateCompatSteps(validated.steps, validated.units);
  return validated;
};

const bundleFromEnv = (test: CompatTestDefinition, kind: "from" | "to"): VersionBundle => ({
  target: "latest-supported",
  lockName: `${slug(test.name)}-${kind}.json`,
  env: { ...test[kind] },
  sources: [`compat-test=${test.name}`, kind],
});

const rolloutEntries = (test: CompatTestDefinition) => [
  { step: "baseline", stepIndex: 0, name: lockStem(0, "baseline").replace(/\.lock\.json$/, "") },
  ...test.steps.map((step, index) => {
    const label = stepLabel(step);
    return { step: label, stepIndex: index + 1, name: lockStem(index + 1, label).replace(/\.lock\.json$/, "") };
  }),
] satisfies RolloutMatrix["include"];

/** Generates the baseline and cumulative mixed-version rollout locks for one compat-test. */
export const generateRolloutLocks = (test: CompatTestDefinition) => {
  const from = bundleFromEnv(test, "from");
  const to = bundleFromEnv(test, "to");
  const current = { ...from.env };
  const baseline: VersionBundle = {
    ...from,
    lockName: `${lockStem(0, "baseline")}.lock.json`,
    sources: [...from.sources, ...rolloutSources(test, "baseline")],
  };
  return [
    baseline,
    ...test.steps.map((step, index) => {
      for (const unit of step) {
        for (const key of test.units[unit]) {
          current[key] = to.env[key];
        }
      }
      const label = stepLabel(step);
      return {
        ...to,
        env: { ...current },
        lockName: `${lockStem(index + 1, label)}.lock.json`,
        sources: [...to.sources, ...rolloutSources(test, label)],
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
  if (!Number.isInteger(stepIndex) || stepIndex < 0 || stepIndex > test.steps.length) {
    throw new PreflightError(`rollout step must be an integer between 0 and ${test.steps.length}`);
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
