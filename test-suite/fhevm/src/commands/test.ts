/**
 * commands/test.ts — The `test` command handler.
 *
 * Runs e2e tests inside the fhevm-test-suite-e2e-debug container.
 */
import path from "node:path";
import { Effect } from "effect";

import { PreflightError } from "../errors";
import { CLI_DIR, TEST_GREP, TEST_PARALLEL } from "../layout";
import { shellEscape, runWithHeartbeat } from "../pipeline";
import { StateManager } from "../services/StateManager";
import type { TestOptions } from "../types";

export const test = (
  testName: string | undefined,
  options: TestOptions,
) =>
  Effect.gen(function* () {
    const stateManager = yield* StateManager;
    const state = yield* stateManager.load;
    if (!state?.discovery?.actualFheKeyId) {
      return yield* Effect.fail(
        new PreflightError({ message: "Stack has not completed bootstrap; run `fhevm-cli up` first" }),
      );
    }

    // ciphertext-drift is a special orchestration test that shells out to a
    // dedicated bash script (SQL trigger injection + log polling).
    // TODO: rewrite scripts/inject-coprocessor-drift.sh and
    // scripts/run-ciphertext-drift-e2e.sh in Bun/Effect-TS to eliminate the
    // bash dependency and integrate with the Effect service layer (CommandRunner,
    // ContainerProbe patterns).
    if (testName === "ciphertext-drift") {
      yield* Effect.log("[test] ciphertext-drift");
      const started = Date.now();
      const script = path.join(CLI_DIR, "scripts", "run-ciphertext-drift-e2e.sh");
      try {
        yield* runWithHeartbeat(["bash", script], "test ciphertext-drift");
        yield* Effect.log(`[pass] ciphertext-drift (${Math.round((Date.now() - started) / 1000)}s)`);
      } catch (error) {
        yield* Effect.log(`[fail] ciphertext-drift (${Math.round((Date.now() - started) / 1000)}s)`);
        return yield* Effect.fail(error instanceof Error ? error : new Error(String(error)));
      }
      return;
    }

    const filter =
      options.grep ??
      (testName ? TEST_GREP[testName] : TEST_GREP["input-proof"]);
    if (!filter) {
      return yield* Effect.fail(
        new PreflightError({ message: `Unknown test profile ${testName}` }),
      );
    }
    const shouldParallel =
      options.parallel ?? (testName ? TEST_PARALLEL[testName] : false);
    const label = testName ?? "custom";
    yield* Effect.log(`[test] ${label} (${options.network})`);
    const started = Date.now();
    const command = [
      "cd /app/test-suite/e2e",
      "&&",
      "npx hardhat test",
      "--no-compile",
      options.verbose ? "--verbose" : "",
      shouldParallel ? "--parallel" : "",
      "--grep",
      shellEscape(filter),
      "--network",
      shellEscape(options.network),
    ]
      .filter(Boolean)
      .join(" ");
    try {
      yield* runWithHeartbeat(
        [
          "docker",
          "exec",
          "fhevm-test-suite-e2e-debug",
          "sh",
          "-lc",
          command,
        ],
        `test ${label}`,
      );
      yield* Effect.log(
        `[pass] ${label} (${Math.round((Date.now() - started) / 1000)}s)`,
      );
    } catch (error) {
      yield* Effect.log(
        `[fail] ${label} (${Math.round((Date.now() - started) / 1000)}s)`,
      );
      return yield* Effect.fail(error instanceof Error ? error : new Error(String(error)));
    }
  });
