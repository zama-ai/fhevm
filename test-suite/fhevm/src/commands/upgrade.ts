/**
 * commands/upgrade.ts — The `upgrade` command handler.
 *
 * Rebuilds and restarts an active local runtime override group.
 */
import { Effect } from "effect";

import { regen } from "../codegen";
import { PreflightError } from "../errors";
import { TEST_SUITE_CONTAINER } from "../layout";
import {
  assertSchemaCompatibility,
  ensureRuntimeArtifacts,
  projectContainers,
  resolveUpgradePlan,
  waitForCoprocessor,
  waitForKmsConnector,
} from "../pipeline";
import { ContainerRunner } from "../services/ContainerRunner";
import { ContainerProbe } from "../services/ContainerProbe";
import { ImageBuilder } from "../services/ImageBuilder";
import { StateManager } from "../services/StateManager";

export const upgrade = (groupValue: string | undefined) =>
  Effect.gen(function* () {
    const stateManager = yield* StateManager;
    const containerRunner = yield* ContainerRunner;
    const probe = yield* ContainerProbe;
    const imageBuilder = yield* ImageBuilder;
    const state = yield* stateManager.load;
    if (!state) {
      return yield* Effect.fail(
        new PreflightError({
          message:
            "Stack is not running; start one with `fhevm-cli up --override ...` or `fhevm-cli up --scenario ...` first",
        }),
      );
    }
    if (!(yield* projectContainers()).length) {
      return yield* Effect.fail(
        new PreflightError({
          message:
            "Stack is not running; start one with `fhevm-cli up --override ...` or `fhevm-cli up --scenario ...` first",
        }),
      );
    }
    yield* ensureRuntimeArtifacts(state, "upgrade");
    const { component, group, services, step } = yield* Effect.try({
      try: () => resolveUpgradePlan(state, groupValue),
      catch: (error) =>
        new PreflightError({ message: (error as Error).message }),
    });
    if (!state.completedSteps.includes(step)) {
      return yield* Effect.fail(
        new PreflightError({
          message: `upgrade requires a stack that has completed the ${step} step`,
        }),
      );
    }
    yield* assertSchemaCompatibility(
      state.versions,
      state.overrides,
      state.scenario,
      false,
    );
    yield* Effect.log(`[upgrade] ${group}`);
    yield* regen(state);
    yield* imageBuilder.maybeBuild(component, state, (s) => stateManager.save(s), { force: true });
    yield* containerRunner.composeUp(component, services, {
      noDeps: true,
    });
    if (group === "coprocessor") {
      yield* waitForCoprocessor(state);
    } else if (group === "kms-connector") {
      yield* waitForKmsConnector;
    } else {
      yield* probe.waitForRunning(TEST_SUITE_CONTAINER);
    }
    yield* stateManager.markStep(state, step);
  });
