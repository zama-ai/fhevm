/**
 * commands/test.ts — The `test` command handler.
 *
 * Runs e2e tests inside the fhevm-test-suite-e2e-debug container.
 */
import { Effect, Fiber } from "effect";

import { PreflightError } from "../errors";
import {
  COPROCESSOR_DB_CONTAINER,
  TEST_GREP,
  TEST_PARALLEL,
  TEST_SUITE_CONTAINER,
} from "../layout";
import {
  parseDriftInstanceIndex,
  parsePositiveInteger,
} from "../ciphertext-drift";
import {
  waitForDriftWarning,
  withDriftInjector,
} from "../ciphertext-drift-runner";
import { shellEscape } from "../pipeline";
import { CommandRunner } from "../services/CommandRunner";
import { StateManager } from "../services/StateManager";
import type { TestOptions } from "../types";

export const test = (
  testName: string | undefined,
  options: TestOptions,
) =>
  Effect.gen(function* () {
    const cmd = yield* CommandRunner;
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
              process.env.POSTGRES_CONTAINER ?? COPROCESSOR_DB_CONTAINER,
            postgresUser: process.env.POSTGRES_USER ?? "postgres",
            postgresPassword: process.env.POSTGRES_PASSWORD ?? "postgres",
          },
          (injector) =>
            Effect.gen(function* () {
              yield* cmd.runWithHeartbeat(
                [
                  "docker",
                  "exec",
                  "-e",
                  "GATEWAY_RPC_URL=",
                  process.env.TEST_CONTAINER ?? TEST_SUITE_CONTAINER,
                  "./run-tests.sh",
                  "-n",
                  "staging",
                  "-g",
                  grepPattern,
                ],
                "test ciphertext-drift",
              );
              const injectedHandleHex = yield* Fiber.join(injector);
              const warning = yield* waitForDriftWarning(injectedHandleHex, {
                since: logSince,
                timeoutSeconds: driftAlertTimeoutSeconds,
                pollIntervalSeconds: driftAlertPollIntervalSeconds,
                });
              return { injectedHandleHex, warning };
            }),
        );
        yield* Effect.log(
          detected.warning.exact
            ? `[drift] detected in ${detected.warning.container} for injected handle 0x${detected.injectedHandleHex}`
            : `[drift] detected in ${detected.warning.container} for handle 0x${detected.warning.handleHex ?? "unknown"} after injecting 0x${detected.injectedHandleHex}`,
        );
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
      yield* cmd.runWithHeartbeat(
        [
          "docker",
          "exec",
          TEST_SUITE_CONTAINER,
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
