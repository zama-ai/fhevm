/**
 * commands/clean.ts — The `clean` command handler.
 *
 * Stops containers, optionally removes CLI-owned images, and deletes .fhevm.
 */
import { Effect } from "effect";

import { PreflightError } from "../errors";
import { STATE_DIR } from "../layout";
import { ImageBuilder } from "../services/ImageBuilder";
import { CommandRunner } from "../services/CommandRunner";
import { StateManager } from "../services/StateManager";
import type { CleanOptions } from "../types";
import { exists, remove } from "../utils";
import { down } from "./down";

export const clean = (options: CleanOptions) =>
  Effect.gen(function* () {
    yield* Effect.log("[clean] start");
    const stateManager = yield* StateManager;
    const cmd = yield* CommandRunner;
    const imageBuilder = yield* ImageBuilder;
    const state = yield* stateManager.load;
    yield* down;
    if (options.images && state?.builtImages?.length) {
      yield* Effect.log(
        `[clean] removing ${state.builtImages.length} owned image${state.builtImages.length === 1 ? "" : "s"}`,
      );
      const failures: string[] = [];
      for (const image of state.builtImages) {
        const currentId = yield* imageBuilder.inspectImageId(image.ref);
        if (!currentId || currentId !== image.id) {
          continue;
        }
        yield* Effect.log(`[image] ${image.ref}`);
        const result = yield* cmd.run(["docker", "image", "rm", image.ref], {
          allowFailure: true,
        });
        if (result.code !== 0) {
          failures.push(`${image.ref}: ${result.stderr.trim() || "docker image rm failed"}`);
        }
      }
      if (failures.length) {
        return yield* Effect.fail(
          new PreflightError({
            message: `Failed to remove owned images:\n${failures.join("\n")}`,
          }),
        );
      }
    }
    const stateExists = yield* Effect.promise(() => exists(STATE_DIR));
    if (stateExists) {
      yield* Effect.log(`[clean] removing ${STATE_DIR}`);
    } else {
      yield* Effect.log("[clean] no runtime state");
    }
    yield* Effect.promise(() => remove(STATE_DIR));
    yield* Effect.log("[clean] done");
  });
