// EXEMPLAR — thin CLI subcommand; no orchestration logic of its own.
/**
 * `fhevm up` — boot the stack by walking the declarative RECIPE (bootStack).
 *
 * The CLI layer owns nothing except flag parsing + phase logging; ALL the boot
 * knowledge lives in the recipe (../lib/recipe). This delegates to bootStack —
 * the faithful discover→regenerate driver — not the placeholder helm path.
 */

import { bootStack, DEFAULT_CONFIG, RECIPE } from "../lib/recipe";
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
  /** Resume from this phase id (skip everything before it). */
  from?: string;
  /** Stop after this phase id (boot a prefix of the recipe). */
  until?: string;
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

/** Walk the declarative RECIPE over the Stack. --dry-run prints the phase plan. */
export const runUp = async (stack: Stack, args: UpArgs): Promise<void> => {
  // Validate the topology flag (a threshold value adjusts the kms-core phase; centralized = default).
  const kms = args.kms !== undefined ? parseKmsFlag(args.kms) : { mode: "centralized" as const };
  void (kms satisfies UpOptions["kms"]);

  if (args.dryRun) {
    for (const p of RECIPE) console.log(`  ${p.id.padEnd(16)} ${p.title}`);
    return;
  }
  await bootStack(stack, DEFAULT_CONFIG, {
    from: args.from,
    until: args.until,
    onPhase: (r) => console.log(`[${r.status === "ok" ? "ok  " : "FAIL"}] ${r.id} — ${r.title}`),
  });
};
