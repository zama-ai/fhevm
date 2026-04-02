import { defaultCoprocessorScenario } from "./scenario/resolve";
import type { State } from "./types";

/** Returns a default scenario fixture with optional overrides for tests. */
export const testDefaultScenario = (patch: Partial<State["scenario"]> = {}): State["scenario"] => {
  const base = defaultCoprocessorScenario();
  return {
    ...base,
    ...patch,
    hostChains: patch.hostChains ?? base.hostChains,
    topology: patch.topology ?? base.topology,
    instances: patch.instances ?? base.instances,
  };
};
