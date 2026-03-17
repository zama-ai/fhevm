/**
 * commands/test.ts — The `test` command handler.
 *
 * Runs e2e tests inside the fhevm-test-suite-e2e-debug container.
 */
import { Effect, Fiber } from "effect";

import { PreflightError } from "../errors";
import { TEST_GREP, TEST_PARALLEL } from "../layout";
import {
  parseDriftInstanceIndex,
  parsePositiveInteger,
} from "../ciphertext-drift";
import {
  waitForDriftWarning,
  withDriftInjector,
} from "../ciphertext-drift-runner";
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

    if (testName === "ciphertext-drift") {
      yield* Effect.log("[test] ciphertext-drift");
      const started = Date.now();
      const logSince = new Date().toISOString();
      const faultyInstanceIndex = parseDriftInstanceIndex(
        process.env.FAULTY_INSTANCE_INDEX ?? "1",
      );
      const driftInjectTimeoutSeconds = parsePositiveInteger(
        process.env.DRIFT_INJECT_TIMEOUT_SECONDS ?? "180",
        "DRIFT_INJECT_TIMEOUT_SECONDS",
      );
      const driftInjectPollIntervalSeconds = parsePositiveInteger(
        process.env.DRIFT_INJECT_POLL_INTERVAL_SECONDS ?? "2",
        "DRIFT_INJECT_POLL_INTERVAL_SECONDS",
      );
      const driftAlertTimeoutSeconds = parsePositiveInteger(
        process.env.DRIFT_ALERT_TIMEOUT_SECONDS ?? "180",
        "DRIFT_ALERT_TIMEOUT_SECONDS",
      );
      const driftAlertPollIntervalSeconds = parsePositiveInteger(
        process.env.DRIFT_ALERT_POLL_INTERVAL_SECONDS ?? "2",
        "DRIFT_ALERT_POLL_INTERVAL_SECONDS",
      );
      const grepPattern =
        process.env.GREP_PATTERN ??
        "test user input uint64 \\(non-trivial\\)";
      try {
        const detected = yield* withDriftInjector(
          {
            instanceIndex: faultyInstanceIndex,
            timeoutSeconds: driftInjectTimeoutSeconds,
            pollIntervalSeconds: driftInjectPollIntervalSeconds,
            postgresContainer:
              process.env.POSTGRES_CONTAINER ?? "coprocessor-and-kms-db",
            postgresUser: process.env.POSTGRES_USER ?? "postgres",
            postgresPassword: process.env.POSTGRES_PASSWORD ?? "postgres",
          },
          (injector) =>
            Effect.gen(function* () {
              yield* runWithHeartbeat(
                [
                  "docker",
                  "exec",
                  "-e",
                  "GATEWAY_RPC_URL=",
                  process.env.TEST_CONTAINER ?? "fhevm-test-suite-e2e-debug",
                  "./run-tests.sh",
                  "-n",
                  "staging",
                  "-g",
                  grepPattern,
                ],
                "test ciphertext-drift",
              );
              const handleHex = yield* Fiber.join(injector);
              return yield* waitForDriftWarning(handleHex, {
                since: logSince,
                timeoutSeconds: driftAlertTimeoutSeconds,
                pollIntervalSeconds: driftAlertPollIntervalSeconds,
              });
            }),
        );
        yield* Effect.log(`[drift] detected in ${detected}`);
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
