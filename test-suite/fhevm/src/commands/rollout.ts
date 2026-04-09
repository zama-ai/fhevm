import path from "node:path";

import { PreflightError } from "../errors";
import { PACKAGE_TO_REPOSITORY } from "../resolve/target";
import type { VersionBundle } from "../types";
import { ensureDir, readJson, writeJson } from "../utils/fs";

export const ROLLOUT_UNITS = {
  RELAYER: ["RELAYER_VERSION", "RELAYER_MIGRATE_VERSION"],
  GATEWAY_CONTRACTS: ["GATEWAY_VERSION"],
  HOST_CONTRACTS: ["HOST_VERSION"],
  KMS_CORE: ["CORE_VERSION"],
  KMS_CONNECTOR: [
    "CONNECTOR_DB_MIGRATION_VERSION",
    "CONNECTOR_GW_LISTENER_VERSION",
    "CONNECTOR_KMS_WORKER_VERSION",
    "CONNECTOR_TX_SENDER_VERSION",
  ],
  COPROCESSOR: [
    "COPROCESSOR_DB_MIGRATION_VERSION",
    "COPROCESSOR_HOST_LISTENER_VERSION",
    "COPROCESSOR_GW_LISTENER_VERSION",
    "COPROCESSOR_TX_SENDER_VERSION",
    "COPROCESSOR_TFHE_WORKER_VERSION",
    "COPROCESSOR_ZKPROOF_WORKER_VERSION",
    "COPROCESSOR_SNS_WORKER_VERSION",
  ],
  TEST_SUITE: ["TEST_SUITE_VERSION"],
} as const;

type RolloutUnit = keyof typeof ROLLOUT_UNITS;
type RolloutStep = RolloutUnit[];
type RolloutMatrix = {
  include: Array<{ step: string; lockFile: string; name: string }>;
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
const UNIT_NAMES = Object.keys(ROLLOUT_UNITS) as RolloutUnit[];
const normalizeUnitMap = (value: Record<string, string[]>) =>
  Object.fromEntries(Object.keys(value).sort().map((key) => [key, [...value[key]].sort()]));
const lockStem = (index: number, label: string) => `${String(index).padStart(2, "0")}-${label}`;
const slug = (value: string) => value.toLowerCase().replaceAll(/[^a-z0-9]+/g, "-").replaceAll(/^-+|-+$/g, "");
const stepLabel = (step: readonly RolloutUnit[]) => step.map((unit) => slug(unit)).join("_");
const rolloutSources = (test: CompatTestDefinition, step: string) => [`compat-test=${test.name}`, `rollout-step=${step}`];

/** Validates that a compat-test env map contains every required version key. */
const validateEnvMap = (label: string, value: Record<string, string>) => {
  const missing = REQUIRED_VERSION_KEYS.filter((key) => typeof value[key] !== "string" || !value[key]?.length);
  if (missing.length) {
    throw new PreflightError(`compat-test ${label} is missing required version keys: ${missing.join(", ")}`);
  }
  return Object.fromEntries(REQUIRED_VERSION_KEYS.map((key) => [key, value[key]]));
};

/** Validates that the fixed unit map embedded in the compat-test matches the CLI definition. */
const validateCompatUnits = (units: Record<string, string[]>) => {
  const expected = normalizeUnitMap(Object.fromEntries(UNIT_NAMES.map((unit) => [unit, [...ROLLOUT_UNITS[unit]]])));
  const actual = normalizeUnitMap(units);
  if (JSON.stringify(actual) !== JSON.stringify(expected)) {
    throw new PreflightError("compat-test units do not match the fixed rollout unit definitions");
  }
};

/** Validates ordered rollout steps, unit names, and duplicate coverage. */
export const validateCompatSteps = (steps: RolloutStep[]) => {
  if (!Array.isArray(steps) || !steps.length) {
    throw new PreflightError(`compat-test steps must include ${UNIT_NAMES.join(", ")}`);
  }
  const flattened = steps.flatMap((step) => {
    if (!Array.isArray(step) || !step.length) {
      throw new PreflightError("compat-test steps must be non-empty unit arrays");
    }
    return step;
  });
  const unknown = flattened.filter((unit) => !UNIT_NAMES.includes(unit));
  if (unknown.length) {
    throw new PreflightError(`Unknown rollout units: ${[...new Set(unknown)].join(", ")}. Valid: ${UNIT_NAMES.join(", ")}`);
  }
  const duplicates = flattened.filter((unit, index) => flattened.indexOf(unit) !== index);
  if (duplicates.length) {
    throw new PreflightError(`Duplicate rollout units: ${[...new Set(duplicates)].join(", ")}`);
  }
  const missing = UNIT_NAMES.filter((unit) => !flattened.includes(unit));
  if (missing.length || flattened.length !== UNIT_NAMES.length) {
    throw new PreflightError(`compat-test steps must include each unit exactly once: ${UNIT_NAMES.join(", ")}`);
  }
  return steps;
};

/** Loads and validates one checked-in compat-test definition. */
export const readCompatTest = async (file: string) => {
  const test = await readJson<CompatTestDefinition>(path.resolve(file));
  if (!test?.name) {
    throw new PreflightError("compat-test must include a non-empty name");
  }
  validateCompatUnits(test.units);
  validateCompatSteps(test.steps);
  return {
    ...test,
    from: validateEnvMap("from", test.from),
    to: validateEnvMap("to", test.to),
  } satisfies CompatTestDefinition;
};

const bundleFromEnv = (test: CompatTestDefinition, kind: "from" | "to"): VersionBundle => ({
  target: "latest-supported",
  lockName: `${slug(test.name)}-${kind}.json`,
  env: { ...test[kind] },
  sources: [`compat-test=${test.name}`, kind],
});

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
        for (const key of ROLLOUT_UNITS[unit]) {
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

/** Writes rollout lock files and the GitHub Actions matrix descriptor into one output directory. */
export const rollout = async (options: { compatTest: string; out: string }) => {
  if (!options.compatTest) throw new PreflightError("rollout requires --compat-test <file>");
  if (!options.out) throw new PreflightError("rollout requires --out <directory>");
  const test = await readCompatTest(options.compatTest);
  const locks = generateRolloutLocks(test);
  const matrix: RolloutMatrix = {
    include: locks.map((bundle, index) => ({
      step: index === 0 ? "baseline" : stepLabel(test.steps[index - 1]),
      lockFile: bundle.lockName,
      name: bundle.lockName.replace(/\.lock\.json$/, ""),
    })),
  };
  await ensureDir(options.out);
  await Promise.all([
    ...locks.map((bundle) => writeJson(path.join(options.out, bundle.lockName), bundle)),
    writeJson(path.join(options.out, "matrix.json"), matrix),
  ]);
  for (const file of [...locks.map((bundle) => bundle.lockName), "matrix.json"]) {
    console.log(path.join(options.out, file));
  }
};
