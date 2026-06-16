// EXEMPLAR — thin CLI subcommand; no orchestration logic of its own.
/**
 * `fhevm up` — boot or reconcile the stack.
 *
 * Parses the user-facing flags, translates them into a Stack.UpOptions bag,
 * and delegates entirely to stack.up().  The CLI layer owns nothing except
 * the flag→type mapping shown here.
 */

import type { Stack, UpOptions } from "../lib/stack";

export type UpArgs = {
  scenario?: string;
  /** Raw KMS topology string: "centralized" | "threshold:N" (e.g. "threshold:3"). */
  kms?: string;
  chains?: number;
  build?: boolean;
  /** Path to an extra Helm values overlay file. */
  values?: string;
  dryRun?: boolean;
};

/**
 * Parse the raw "--kms" flag value into a typed KmsTopology.
 *
 * Accepted forms:
 *   "centralized"   → { mode: "centralized" }
 *   "threshold:N"   → { mode: "threshold", parties: N, threshold: Math.ceil(N * 2 / 3) }
 *   "threshold:N/T" → { mode: "threshold", parties: N, threshold: T }
 */
export const parseKmsFlag = (raw: string): UpOptions["kms"] => {
  if (raw === "centralized") {
    return { mode: "centralized" };
  }
  const thresholdMatch = raw.match(/^threshold:(\d+)(?:\/(\d+))?$/);
  if (!thresholdMatch) {
    throw new Error(
      `Invalid --kms value "${raw}". Expected "centralized", "threshold:N", or "threshold:N/T".`,
    );
  }
  const parties = parseInt(thresholdMatch[1], 10);
  const threshold = thresholdMatch[2]
    ? parseInt(thresholdMatch[2], 10)
    : Math.ceil((parties * 2) / 3);
  return { mode: "threshold", parties, threshold };
};

/** Map parsed CLI args → Stack.UpOptions and call stack.up(). */
export const runUp = async (stack: Stack, args: UpArgs): Promise<void> => {
  const options: UpOptions = {
    scenario: args.scenario,
    kms: args.kms !== undefined ? parseKmsFlag(args.kms) : undefined,
    chains: args.chains,
    build: args.build,
    valuesFile: args.values,
    dryRun: args.dryRun,
  };
  await stack.up(options);
};
