import path from "node:path";

import { PreflightError } from "../errors";
import { readLockBundle } from "../resolve/bundle-store";
import type { VersionBundle } from "../types";
import { ensureDir, writeJson } from "../utils/fs";

export const ROLLOUT_GROUPS = {
  relayer: ["RELAYER_VERSION", "RELAYER_MIGRATE_VERSION"],
  contracts: ["GATEWAY_VERSION", "HOST_VERSION"],
  "kms-plane": [
    "CONNECTOR_DB_MIGRATION_VERSION",
    "CONNECTOR_GW_LISTENER_VERSION",
    "CONNECTOR_KMS_WORKER_VERSION",
    "CONNECTOR_TX_SENDER_VERSION",
    "CORE_VERSION",
  ],
  coprocessor: [
    "COPROCESSOR_DB_MIGRATION_VERSION",
    "COPROCESSOR_HOST_LISTENER_VERSION",
    "COPROCESSOR_GW_LISTENER_VERSION",
    "COPROCESSOR_TX_SENDER_VERSION",
    "COPROCESSOR_TFHE_WORKER_VERSION",
    "COPROCESSOR_ZKPROOF_WORKER_VERSION",
    "COPROCESSOR_SNS_WORKER_VERSION",
  ],
  "test-suite": ["TEST_SUITE_VERSION"],
} as const;

type RolloutGroup = keyof typeof ROLLOUT_GROUPS;
type RolloutMatrix = {
  include: Array<{ group: RolloutGroup | "baseline"; lockFile: string; name: string }>;
};

const ROLLOUT_GROUP_NAMES = Object.keys(ROLLOUT_GROUPS) as RolloutGroup[];
const lockStem = (index: number, group: string) => `${String(index).padStart(2, "0")}-${group}`;
const rolloutSources = (from: VersionBundle, to: VersionBundle, group: string) => [
  `rollout-from=${from.lockName}`,
  `rollout-to=${to.lockName}`,
  `rollout-step=${group}`,
];

/** Parses and validates a rollout order that must contain each rollout group exactly once. */
export const parseRolloutOrder = (value: string) => {
  const order = value
    .split(",")
    .map((item) => item.trim())
    .filter(Boolean);
  if (!order.length) {
    throw new PreflightError(`rollout --order must list ${ROLLOUT_GROUP_NAMES.join(", ")}`);
  }
  const unknown = order.filter((group) => !ROLLOUT_GROUP_NAMES.includes(group as RolloutGroup));
  if (unknown.length) {
    throw new PreflightError(`Unknown rollout groups: ${unknown.join(", ")}. Valid: ${ROLLOUT_GROUP_NAMES.join(", ")}`);
  }
  const duplicates = order.filter((group, index) => order.indexOf(group) !== index);
  if (duplicates.length) {
    throw new PreflightError(`Duplicate rollout groups: ${[...new Set(duplicates)].join(", ")}`);
  }
  const missing = ROLLOUT_GROUP_NAMES.filter((group) => !order.includes(group));
  if (missing.length || order.length !== ROLLOUT_GROUP_NAMES.length) {
    throw new PreflightError(`rollout --order must include each group exactly once: ${ROLLOUT_GROUP_NAMES.join(", ")}`);
  }
  return order as RolloutGroup[];
};

/** Generates the baseline and cumulative mixed-version rollout locks for one ordered release plan. */
export const generateRolloutLocks = (from: VersionBundle, to: VersionBundle, order: readonly RolloutGroup[]) => {
  const current = { ...from.env };
  const baseline: VersionBundle = {
    ...from,
    lockName: `${lockStem(0, "baseline")}.lock.json`,
    sources: [...from.sources, ...rolloutSources(from, to, "baseline")],
  };
  return [
    baseline,
    ...order.map((group, index) => {
      for (const key of ROLLOUT_GROUPS[group]) {
        current[key] = to.env[key];
      }
      return {
        ...to,
        env: { ...current },
        lockName: `${lockStem(index + 1, group)}.lock.json`,
        sources: [...to.sources, ...rolloutSources(from, to, group)],
      } satisfies VersionBundle;
    }),
  ];
};

/** Writes rollout lock files and the GitHub Actions matrix descriptor into one output directory. */
export const rollout = async (options: { from: string; to: string; order: string; out: string }) => {
  if (!options.from) throw new PreflightError("rollout requires --from <lock-file>");
  if (!options.to) throw new PreflightError("rollout requires --to <lock-file>");
  if (!options.out) throw new PreflightError("rollout requires --out <directory>");
  const order = parseRolloutOrder(options.order);
  const [from, to] = await Promise.all([readLockBundle(options.from), readLockBundle(options.to)]);
  const locks = generateRolloutLocks(from, to, order);
  const matrix: RolloutMatrix = {
    include: locks.map((bundle, index) => ({
      group: index === 0 ? "baseline" : order[index - 1],
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
