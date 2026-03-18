/**
 * commands/down.ts — The `down` command handler.
 *
 * Stops all stack containers in reverse order.
 */
import { Effect } from "effect";

import { PreflightError } from "../errors";
import { ensureRuntimeArtifacts, projectContainers } from "../pipeline";
import { COMPONENTS, PROJECT } from "../layout";
import { CommandRunner } from "../services/CommandRunner";
import { ContainerRunner } from "../services/ContainerRunner";
import { StateManager } from "../services/StateManager";

export const down = Effect.gen(function* () {
  const stateManager = yield* StateManager;
  const cmd = yield* CommandRunner;
  const containerRunner = yield* ContainerRunner;
  const state = yield* stateManager.load;
  if (state) {
    yield* ensureRuntimeArtifacts(state, "teardown");
  }
  const existing = yield* projectContainers(true);
  if (!existing.length) {
    yield* Effect.log("[down] nothing to stop");
    return;
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
  const leftovers = yield* cmd.run(
    [
      "docker",
      "ps",
      "-a",
      "--filter",
      `label=com.docker.compose.project=${PROJECT}`,
      "--format",
      "{{.ID}}",
    ],
    { allowFailure: true },
  );
  const ids = leftovers.stdout
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean);
  if (ids.length) {
    yield* Effect.log(`[down] removing ${ids.length} stale project container${ids.length === 1 ? "" : "s"}`);
    yield* cmd.run(["docker", "rm", "-fv", ...ids], { allowFailure: true });
  }
});
