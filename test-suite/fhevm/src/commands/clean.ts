/**
 * commands/clean.ts — The `clean` command handler.
 *
 * Stops containers, optionally removes CLI-owned images, and deletes .fhevm.
 */
import { Effect } from "effect";

import { STATE_DIR } from "../layout";
import { ImageBuilder } from "../services/ImageBuilder";
import { StateManager } from "../services/StateManager";
import type { CleanOptions } from "../types";
import { exists, remove } from "../utils";
import { down } from "./down";

export const clean = (options: CleanOptions) =>
  Effect.gen(function* () {
    yield* Effect.log("[clean] start");
    const stateManager = yield* StateManager;
    const imageBuilder = yield* ImageBuilder;
    const state = yield* stateManager.load;
    if (options.images && state?.builtImages?.length) {
      yield* Effect.log(
        `[clean] removing ${state.builtImages.length} owned image${state.builtImages.length === 1 ? "" : "s"}`,
      );
      for (const image of state.builtImages) {
        const currentId = yield* imageBuilder.inspectImageId(image.ref);
        if (!currentId || currentId !== image.id) {
          continue;
        }
        yield* Effect.log(`[image] ${image.ref}`);
        yield* imageBuilder.removeImage(image.ref);
      }
    }
    yield* down;
    const stateExists = yield* Effect.promise(() => exists(STATE_DIR));
    if (stateExists) {
      yield* Effect.log(`[clean] removing ${STATE_DIR}`);
    } else {
      yield* Effect.log("[clean] no runtime state");
    }
    yield* Effect.promise(() => remove(STATE_DIR));
    yield* Effect.log("[clean] done");
  });
