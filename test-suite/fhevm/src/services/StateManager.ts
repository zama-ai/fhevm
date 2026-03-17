import { Context, Effect, Layer } from "effect";
import fs from "node:fs/promises";
import type { State, StepName } from "../types";
import { readJson, writeJson, exists } from "../utils";
import { STATE_FILE } from "../layout";
type PersistedState = State;

export class StateManager extends Context.Tag("StateManager")<
  StateManager,
  {
    readonly load: Effect.Effect<State | undefined>;
    readonly save: (state: State) => Effect.Effect<void>;
    readonly markStep: (state: State, step: StepName) => Effect.Effect<void>;
    readonly clear: Effect.Effect<void>;
  }
>() {
  static makeForPath(stateFile: string) {
    return {
      load: Effect.promise(async () =>
        (await exists(stateFile))
          ? readJson<PersistedState>(stateFile)
          : undefined,
      ),
      save: (state: State) =>
        Effect.promise(() => writeJson(stateFile, state)),
      markStep: (state: State, step: StepName) =>
        Effect.promise(async () => {
          if (!state.completedSteps.includes(step)) {
            state.completedSteps.push(step);
          }
          state.updatedAt = new Date().toISOString();
          await writeJson(stateFile, state);
        }),
      clear: Effect.promise(() => fs.rm(stateFile, { force: true })),
    } satisfies Context.Tag.Service<StateManager>;
  }

  static Live = Layer.succeed(StateManager, StateManager.makeForPath(STATE_FILE));
}
