#!/usr/bin/env bun
import { Effect, Fiber } from "effect";

import { parseDriftInstanceIndex, parsePositiveInteger } from "../src/ciphertext-drift";
import {
  waitForDriftWarning,
  withDriftInjector,
} from "../src/ciphertext-drift-runner";
import { CommandRunner } from "../src/services/CommandRunner";

const env = process.env;
const faultyInstanceIndex = parseDriftInstanceIndex(env.FAULTY_INSTANCE_INDEX ?? "1");
const testContainer = env.TEST_CONTAINER ?? "fhevm-test-suite-e2e-debug";
const grepPattern =
  env.GREP_PATTERN ?? "test user input uint64 \\(non-trivial\\)";
const driftInjectTimeoutSeconds = parsePositiveInteger(
  env.DRIFT_INJECT_TIMEOUT_SECONDS ?? "180",
  "DRIFT_INJECT_TIMEOUT_SECONDS",
);
const driftInjectPollIntervalSeconds = parsePositiveInteger(
  env.DRIFT_INJECT_POLL_INTERVAL_SECONDS ?? "2",
  "DRIFT_INJECT_POLL_INTERVAL_SECONDS",
);
const driftAlertTimeoutSeconds = parsePositiveInteger(
  env.DRIFT_ALERT_TIMEOUT_SECONDS ?? "180",
  "DRIFT_ALERT_TIMEOUT_SECONDS",
);
const driftAlertPollIntervalSeconds = parsePositiveInteger(
  env.DRIFT_ALERT_POLL_INTERVAL_SECONDS ?? "2",
  "DRIFT_ALERT_POLL_INTERVAL_SECONDS",
);

const program = Effect.gen(function* () {
  const cmd = yield* CommandRunner;
  const logSince = new Date().toISOString();
  const detected = yield* withDriftInjector(
    {
      instanceIndex: faultyInstanceIndex,
      timeoutSeconds: driftInjectTimeoutSeconds,
      pollIntervalSeconds: driftInjectPollIntervalSeconds,
      postgresContainer: env.POSTGRES_CONTAINER ?? "coprocessor-and-kms-db",
      postgresUser: env.POSTGRES_USER ?? "postgres",
      postgresPassword: env.POSTGRES_PASSWORD ?? "postgres",
    },
    (injector) =>
      Effect.gen(function* () {
        const testExit = yield* cmd.runLive(
          [
            "docker",
            "exec",
            "-e",
            "GATEWAY_RPC_URL=",
            testContainer,
            "./run-tests.sh",
            "-n",
            "staging",
            "-g",
            grepPattern,
          ],
          { allowFailure: true },
        );
        const handleHex = yield* Fiber.join(injector);
        if (testExit !== 0) {
          return yield* Effect.fail(
            new Error(`ciphertext drift test command failed with exit code ${testExit}`),
          );
        }
        return yield* waitForDriftWarning(handleHex, {
          since: logSince,
          timeoutSeconds: driftAlertTimeoutSeconds,
          pollIntervalSeconds: driftAlertPollIntervalSeconds,
        });
      }),
  );
  console.log(`drift detected in ${detected}`);
});

await Effect.runPromise(
  program.pipe(
    Effect.provide(CommandRunner.Live),
  ),
);
