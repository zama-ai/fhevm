// EXEMPLAR — thin CLI subcommand; no orchestration logic of its own.
/**
 * `fhevm test <suite>` — run a named test profile against the running stack.
 *
 * Maps the positional <suite> argument and optional flags to stack.test().
 * The CLI layer owns only the arg→type mapping; all logic lives in the Stack.
 */

import type { Stack } from "../lib/stack";

export type TestArgs = {
  /** Named test profile (e.g. "rollout-standard", "smoke"). */
  suite: string;
  network?: string;
  parallel?: boolean;
};

/** Map parsed CLI args → stack.test() call. */
export const runTest = async (stack: Stack, args: TestArgs): Promise<void> => {
  await stack.test(args.suite, {
    network: args.network,
    parallel: args.parallel,
  });
};
