import { defaultCoprocessorScenario } from "./scenario/resolve";
import type { ResolvedCoprocessorScenario } from "./types";

/** Returns a default scenario fixture with optional overrides for tests. */
export const testDefaultScenario = (
  patch: Partial<ResolvedCoprocessorScenario> = {},
): ResolvedCoprocessorScenario => {
  const base = defaultCoprocessorScenario();
  return {
    ...base,
    ...patch,
    hostChains: patch.hostChains ?? base.hostChains,
    topology: patch.topology ?? base.topology,
    instances: patch.instances ?? base.instances,
  };
};
