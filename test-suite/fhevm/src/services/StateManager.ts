import { Context, Effect, Layer } from "effect";
import fs from "node:fs/promises";
import type { State, StepName } from "../types";
import { readJson, writeJson, exists } from "../utils";
import { STATE_FILE } from "../layout";
import { topologyFromScenario } from "../runtime-plan";

type PersistedState = Omit<State, "topology"> & {
  topology?: State["topology"];
};

const hydrateState = (state: PersistedState): State => ({
  ...state,
  topology: topologyFromScenario(state.scenario),
});

const persistState = (state: State): PersistedState => {
  const { topology: _topology, ...persisted } = state;
  return persisted;
};

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
          ? hydrateState(await readJson<PersistedState>(stateFile))
          : undefined,
      ),
      save: (state: State) =>
        Effect.promise(() => writeJson(stateFile, persistState(state))),
      markStep: (state: State, step: StepName) =>
        Effect.promise(async () => {
          if (!state.completedSteps.includes(step)) {
            state.completedSteps.push(step);
          }
          state.updatedAt = new Date().toISOString();
          await writeJson(stateFile, persistState(state));
        }),
      clear: Effect.promise(() => fs.rm(stateFile, { force: true })),
    } satisfies Context.Tag.Service<StateManager>;
  }

  static Live = Layer.succeed(StateManager, StateManager.makeForPath(STATE_FILE));
}
