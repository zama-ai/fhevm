/**
 * commands/down.ts — The `down` command handler.
 *
 * Stops all stack containers in reverse order.
 */
import { Effect } from "effect";

import { PreflightError } from "../errors";
import { ensureRuntimeArtifacts } from "../pipeline";
import { COMPONENTS } from "../layout";
import { ContainerRunner } from "../services/ContainerRunner";
import { StateManager } from "../services/StateManager";

export const down = Effect.gen(function* () {
  const stateManager = yield* StateManager;
  const containerRunner = yield* ContainerRunner;
  const state = yield* stateManager.load;
  if (state) {
    yield* ensureRuntimeArtifacts(state, "teardown");
  }
  const failed: string[] = [];
  for (const component of [...COMPONENTS].reverse()) {
    yield* Effect.log(`[down] ${component}`);
    const ok = yield* containerRunner.composeDown(component);
    if (!ok) {
      failed.push(component);
    }
  }
  if (failed.length) {
    return yield* Effect.fail(
      new PreflightError({ message: `Failed to stop components: ${failed.join(", ")}` }),
    );
  }
});
