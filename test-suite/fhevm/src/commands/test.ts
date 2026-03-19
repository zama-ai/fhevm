/**
 * commands/test.ts — The `test` command handler.
 *
 * Runs e2e tests inside the fhevm-test-suite-e2e-debug container.
 */
import { Effect, Either, Fiber } from "effect";

import { compatPolicyForState } from "../compat";
import { PreflightError } from "../errors";
import {
  COPROCESSOR_DB_CONTAINER,
  LIGHT_TEST_PROFILES,
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
import { topologyForState } from "../runtime-plan";
import { CommandRunner } from "../services/CommandRunner";
import { StateManager } from "../services/StateManager";
import type { TestOptions } from "../types";
import { pause } from "./pause";
import { unpause } from "./unpause";

export const test = (
  testName: string | undefined,
  options: TestOptions,
) =>
  Effect.gen(function* () {
    const testContainerArgs = (...tail: string[]) => [
      "docker",
      "exec",
      "-e",
      "npm_config_update_notifier=false",
      "-e",
      "NPM_CONFIG_UPDATE_NOTIFIER=false",
      TEST_SUITE_CONTAINER,
      ...tail,
    ];
    const timedLabel = (label: string, started: number) =>
      `${label} (${Math.round((Date.now() - started) / 1000)}s)`;
    const runLogged = <A, E, R>(
      label: string,
      started: number,
      effect: Effect.Effect<A, E, R>,
    ) =>
      Effect.gen(function* () {
        const result = yield* Effect.either(effect);
        if (Either.isLeft(result)) {
          yield* Effect.log(`[fail] ${timedLabel(label, started)}`);
          return yield* Effect.fail(result.left);
        }
        yield* Effect.log(`[pass] ${timedLabel(label, started)}`);
        return result.right;
      });

    const cmd = yield* CommandRunner;
    const stateManager = yield* StateManager;
    const state = yield* stateManager.load;
    if (!state?.discovery?.actualFheKeyId) {
      return yield* Effect.fail(
        new PreflightError({ message: "Stack has not completed bootstrap; run `fhevm-cli up` first" }),
      );
    }

    const ciphertextDriftRequirement = () => {
      const topology = topologyForState(state);
      if (topology.count < 2) {
        return "ciphertext-drift requires a multi-coprocessor topology; rerun `fhevm-cli up --scenario two-of-two` first";
      }
      const faultyInstanceIndex = parseDriftInstanceIndex(
        process.env.FAULTY_INSTANCE_INDEX ?? "1",
      );
      if (faultyInstanceIndex >= topology.count) {
        return `ciphertext-drift targets coprocessor instance ${faultyInstanceIndex}, but the current topology only has ${topology.count} instance${topology.count === 1 ? "" : "s"}`;
      }
      const compat = compatPolicyForState(state);
      if ((compat.coprocessorDropFlags["gw-listener"] ?? []).includes("--ciphertext-commits-address")) {
        return "ciphertext-drift requires a gw-listener build with drift addresses enabled; use latest-main or a newer supported bundle";
      }
      return undefined;
    };

    const runProfile = (name: string) => {
      if (name === "ciphertext-drift") {
        return Effect.gen(function* () {
          yield* Effect.log("[test] ciphertext-drift");
          const started = Date.now();
          const precondition = ciphertextDriftRequirement();
          if (precondition) {
            return yield* Effect.fail(
              new PreflightError({
                message: precondition,
              }),
            );
          }
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
          const detected = yield* runLogged(
            "ciphertext-drift",
            started,
            withDriftInjector(
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
                    testContainerArgs(
                      "-e",
                      "GATEWAY_RPC_URL=",
                      "./run-tests.sh",
                      "-n",
                      "staging",
                      "-g",
                      grepPattern,
                    ),
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
            ),
          );
          yield* Effect.log(
            `[drift] detected in ${detected.warning.container} for injected handle 0x${detected.injectedHandleHex}`,
          );
        });
      }

      return Effect.gen(function* () {
        const filter = TEST_GREP[name];
        if (!filter) {
          return yield* Effect.fail(
            new PreflightError({ message: `Unknown test profile ${name}` }),
          );
        }
        const shouldParallel = options.parallel ?? TEST_PARALLEL[name];
        yield* Effect.log(`[test] ${name} (${options.network})`);
        const started = Date.now();
        const command = [
          "./run-tests.sh",
          options.verbose ? "-v" : "",
          shouldParallel ? "--parallel" : "",
          "-n",
          shellEscape(options.network),
          "-g",
          shellEscape(filter),
        ]
          .filter(Boolean)
          .join(" ");
        return yield* runLogged(
          name,
          started,
          cmd.runWithHeartbeat(
            testContainerArgs("sh", "-lc", command),
            `test ${name}`,
          ),
        );
      });
    };

    if (testName === "light") {
      if (options.grep) {
        return yield* Effect.fail(
          new PreflightError({
            message: "`fhevm-cli test light` does not accept `--grep`; run a named profile instead",
          }),
        );
      }
      if (options.parallel === true) {
        return yield* Effect.fail(
          new PreflightError({
            message: "`fhevm-cli test light` does not accept `--parallel`; suite members choose their own mode",
          }),
        );
      }
      yield* Effect.log(`[test] light (${options.network})`);
      const started = Date.now();
      yield* runLogged(
        "light",
        started,
        Effect.gen(function* () {
          yield* pause("host");
          yield* Effect.ensuring(
            runProfile("paused-host-contracts"),
            unpause("host").pipe(Effect.orDie),
          );

          yield* pause("gateway");
          yield* Effect.ensuring(
            runProfile("paused-gateway-contracts"),
            unpause("gateway").pipe(Effect.orDie),
          );

          const driftPrecondition = ciphertextDriftRequirement();
          const profiles = driftPrecondition
            ? LIGHT_TEST_PROFILES.filter((profile) => profile !== "ciphertext-drift")
            : LIGHT_TEST_PROFILES;
          if (driftPrecondition) {
            yield* Effect.log(`[skip] ciphertext-drift: ${driftPrecondition}`);
          }

          for (const profile of profiles.slice(2)) {
            yield* runProfile(profile);
          }

          yield* cmd.runWithHeartbeat(["docker", "stop", "coprocessor-host-listener"], "stop host listener");
          yield* Effect.ensuring(
            runProfile("erc20"),
            cmd.runWithHeartbeat(
              ["docker", "start", "coprocessor-host-listener"],
              "start host listener",
              { allowFailure: true },
            ).pipe(Effect.orDie),
          );
        }),
      );
      return;
    }

    if (options.grep) {
      yield* Effect.log(`[test] custom (${options.network})`);
      const started = Date.now();
      const command = [
        "./run-tests.sh",
        options.verbose ? "-v" : "",
        options.parallel ? "--parallel" : "",
        "-n",
        shellEscape(options.network),
        "-g",
        shellEscape(options.grep),
      ]
        .filter(Boolean)
        .join(" ");
      yield* runLogged(
        "custom",
        started,
        cmd.runWithHeartbeat(
          testContainerArgs("sh", "-lc", command),
          "test custom",
        ),
      );
      return;
    }

    yield* runProfile(testName ?? "input-proof");
  });
