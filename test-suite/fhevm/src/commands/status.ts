/**
 * commands/status.ts — The `status` command handler.
 *
 * Prints state info and running containers.
 */
import { Effect } from "effect";

import { PreflightError } from "../errors";
import { describeOverride, overrideWarnings } from "../pipeline";
import { PROJECT } from "../layout";
import { CommandRunner } from "../services/CommandRunner";
import { StateManager } from "../services/StateManager";

export const status = Effect.gen(function* () {
  const stateManager = yield* StateManager;
  const cmd = yield* CommandRunner;
  const state = yield* stateManager.load;
  if (state) {
    yield* Effect.log(`[target] ${state.target}`);
    if (state.overrides.length) {
      yield* Effect.log(
        `[overrides] ${state.overrides.map(describeOverride).join(", ")}`,
      );
      for (const warning of overrideWarnings(
        state.overrides,
        state.target,
      )) {
        yield* Effect.log(`[warn] ${warning}`);
      }
    }
    yield* Effect.log(
      `[topology] n=${state.topology.count} t=${state.topology.threshold}`,
    );
    if (state.scenario.origin !== "default") {
      yield* Effect.log(
        `[scenario] ${state.scenario.origin}${state.scenario.sourcePath ? ` ${state.scenario.sourcePath}` : ""}`,
      );
      for (const instance of state.scenario.instances) {
        const source =
          instance.source.mode === "registry"
            ? `registry:${instance.source.tag}`
            : instance.source.mode;
        yield* Effect.log(`[coprocessor-${instance.index}] ${source}`);
      }
    }
    yield* Effect.log(
      `[steps] ${state.completedSteps.join(", ") || "none"}`,
    );
    if (state.builtImages?.length) {
      yield* Effect.log(`[owned-images] ${state.builtImages.length}`);
      for (const image of state.builtImages) {
        yield* Effect.log(`  ${image.ref} (${image.group})`);
      }
    }
  }
  const ps = yield* cmd
    .run(
      [
        "docker",
        "ps",
        "--filter",
        `label=com.docker.compose.project=${PROJECT}`,
        "--format",
        "{{.Names}}\t{{.Status}}",
      ],
      { allowFailure: true },
    )
    .pipe(Effect.mapError((error) => new PreflightError({ message: error.stderr })));
  if (ps.code !== 0) {
    return yield* Effect.fail(
      new PreflightError({
        message: ps.stderr.trim() || "docker ps failed",
      }),
    );
  }
  if (!ps.stdout.trim()) {
    if (state) {
      yield* Effect.log(
        "[warn] persisted state exists but no fhevm containers are running",
      );
    }
    yield* Effect.log("No fhevm containers");
    return;
  }
  yield* Effect.log(ps.stdout.trim());
});
