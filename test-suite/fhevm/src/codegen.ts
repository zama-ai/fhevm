import { Effect } from "effect";

import { generateComposeOverrides } from "./render-compose";
import { runtimePlanForState } from "./runtime-plan";
import { EnvWriter } from "./services/EnvWriter";
import type { State } from "./types";

export const regen = (state: State) =>
  Effect.gen(function* () {
    const envWriter = yield* EnvWriter;
    const plan = runtimePlanForState(state);
    yield* envWriter.generateEnvFiles(state, plan);
    yield* generateComposeOverrides(state, plan);
  });
