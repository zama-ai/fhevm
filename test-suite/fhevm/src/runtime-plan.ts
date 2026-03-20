/**
 * Combines resolved versions, overrides, and scenario topology into the runtime plan used for rendering and orchestration.
 */
import type {
  ResolvedCoprocessorScenario,
  State,
  Topology,
  UpOptions,
  VersionBundle,
} from "./types";
import {
  assertScenarioOverrideCompatibility,
  defaultCoprocessorScenario,
  loadCoprocessorScenario,
  resolveScenarioFile,
  resolveScenarioReference,
  synthesizeOverrideScenario,
} from "./scenario";

export type RuntimePlan = {
  requiresGitHub: boolean;
  target: State["target"];
  versions: VersionBundle;
  overrides: State["overrides"];
  topology: Topology;
  coprocessor: ResolvedCoprocessorScenario;
};

export const resolveScenarioForOptions = async (
  options: Pick<UpOptions, "overrides" | "scenarioPath">,
) => {
  if (options.scenarioPath) {
    const sourcePath = await resolveScenarioReference(options.scenarioPath);
    const input = await loadCoprocessorScenario(sourcePath);
    const resolved = resolveScenarioFile(sourcePath, input);
    assertScenarioOverrideCompatibility(resolved, options.overrides);
    return resolved;
  }
  return synthesizeOverrideScenario(options.overrides) ?? defaultCoprocessorScenario();
};

const topologyFromScenario = (scenario: ResolvedCoprocessorScenario): Topology => ({
  count: scenario.topology.count,
  threshold: scenario.topology.threshold,
});

export const topologyForState = (state: Pick<State, "scenario">): Topology =>
  topologyFromScenario(state.scenario);

const runtimePlanFromResolved = (input: {
  target: State["target"];
  versions: VersionBundle;
  overrides: State["overrides"];
  scenario: ResolvedCoprocessorScenario;
  requiresGitHub: boolean;
}): RuntimePlan => ({
  requiresGitHub: input.requiresGitHub,
  target: input.target,
  versions: input.versions,
  overrides: input.overrides,
  topology: topologyFromScenario(input.scenario),
  coprocessor: input.scenario,
});

export const runtimePlanForState = (
  state: Pick<State, "requiresGitHub" | "target" | "versions" | "overrides" | "scenario">,
): RuntimePlan =>
  runtimePlanFromResolved({
    requiresGitHub: state.requiresGitHub ?? true,
    target: state.target,
    versions: state.versions,
    overrides: state.overrides,
    scenario: state.scenario,
  });
