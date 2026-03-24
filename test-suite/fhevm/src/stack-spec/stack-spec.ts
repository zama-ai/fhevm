/**
 * Combines resolved versions, overrides, and scenario topology into the stack spec used for generation and orchestration.
 */
import type {
  HostChainScenario,
  ResolvedCoprocessorScenario,
  State,
  Topology,
  UpOptions,
  VersionBundle,
} from "../types";
import {
  assertScenarioOverrideCompatibility,
  defaultCoprocessorScenario,
  loadCoprocessorScenario,
  resolveScenarioFile,
  resolveScenarioReference,
  synthesizeOverrideScenario,
} from "../scenario/resolve";

export type StackSpec = {
  requiresGitHub: boolean;
  target: State["target"];
  versions: VersionBundle;
  overrides: State["overrides"];
  topology: Topology;
  hostChains: HostChainScenario[];
  coprocessor: ResolvedCoprocessorScenario;
};

/** Resolves the effective scenario from explicit input, overrides, or defaults. */
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

/** Derives runtime topology fields from a resolved scenario. */
const topologyFromScenario = (scenario: ResolvedCoprocessorScenario): Topology => ({
  count: scenario.topology.count,
  threshold: scenario.topology.threshold,
});

export const topologyForState = (state: Pick<State, "scenario">): Topology =>
  topologyFromScenario(state.scenario);

/** Packages resolved versions, overrides, and scenario state into one stack spec. */
const stackSpecFromResolved = (input: {
  target: State["target"];
  versions: VersionBundle;
  overrides: State["overrides"];
  scenario: ResolvedCoprocessorScenario;
  requiresGitHub: boolean;
}): StackSpec => ({
  requiresGitHub: input.requiresGitHub,
  target: input.target,
  versions: input.versions,
  overrides: input.overrides,
  topology: topologyFromScenario(input.scenario),
  hostChains: input.scenario.hostChains,
  coprocessor: input.scenario,
});

/** Rebuilds a stack spec from persisted state. */
export const stackSpecForState = (
  state: Pick<State, "requiresGitHub" | "target" | "versions" | "overrides" | "scenario">,
): StackSpec =>
  stackSpecFromResolved({
    requiresGitHub: state.requiresGitHub ?? true,
    target: state.target,
    versions: state.versions,
    overrides: state.overrides,
    scenario: state.scenario,
  });
