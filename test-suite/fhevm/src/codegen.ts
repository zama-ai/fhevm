import { Effect } from "effect";

import { generateComposeOverrides } from "./render-compose";
import { runtimePlanForState } from "./runtime-plan";
import { EnvWriter } from "./services/EnvWriter";
import type { State } from "./types";

export type { ComposeDoc } from "./render-compose";
export {
  LOCAL_BUILD_TAG,
  appendVolume,
  applyBuildPolicy,
  applyInstanceAdjustments,
  buildComposeOverride,
  generatedComposeComponents,
  generateComposeOverrides,
  interpolateComposeValue,
  interpolateString,
  localInstanceTag,
  loadComposeDoc,
  overriddenServicesForComponent,
  resolvedComposeEnv,
  resolveComposePath,
  retagLocal,
  rewriteComposePaths,
  rewriteCoprocessorDependsOn,
  rewriteImageTag,
  rewriteVolume,
  serviceNameList,
} from "./render-compose";

export const regen = (state: State) =>
  Effect.gen(function* () {
    const envWriter = yield* EnvWriter;
    const plan = runtimePlanForState(state);
    yield* envWriter.generateEnvFiles(state, plan);
    yield* generateComposeOverrides(state, plan);
  });
