/**
 * commands/status.ts — The `status` command handler.
 *
 * Prints state info and running containers.
 */
import { Effect } from "effect";

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
    .pipe(
      Effect.catchAll(() =>
        Effect.succeed({ stdout: "", stderr: "", code: 1 }),
      ),
    );
  yield* Effect.log(ps.stdout.trim() || "No fhevm containers");
});
