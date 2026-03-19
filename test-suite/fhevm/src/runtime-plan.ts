import { Effect } from "effect";

import {
  defaultCoprocessorScenario,
  loadCoprocessorScenario,
  resolveScenarioReference,
  resolveScenarioFile,
  synthesizeOverrideScenario,
} from "./scenario";
import type {
  ResolvedCoprocessorScenario,
  State,
  Topology,
  UpOptions,
  VersionBundle,
} from "./types";

export type RuntimePlan = {
  requiresGitHub: boolean;
  target: State["target"];
  versions: VersionBundle;
  overrides: State["overrides"];
  topology: Topology;
  coprocessor: ResolvedCoprocessorScenario;
};

export const resolveScenarioForOptions = (
  options: Pick<UpOptions, "overrides" | "scenarioPath">,
) =>
  Effect.gen(function* () {
    if (options.scenarioPath) {
      const sourcePath = yield* resolveScenarioReference(options.scenarioPath);
      const input = yield* loadCoprocessorScenario(sourcePath);
      return resolveScenarioFile(sourcePath, input);
    }
    return synthesizeOverrideScenario(options.overrides) ?? defaultCoprocessorScenario();
  });

export const topologyFromScenario = (scenario: ResolvedCoprocessorScenario): Topology => ({
  count: scenario.topology.count,
  threshold: scenario.topology.threshold,
});

export const topologyForState = (state: Pick<State, "scenario">): Topology =>
  topologyFromScenario(state.scenario);

export const runtimePlanFromResolved = (input: {
  target: State["target"];
  versions: VersionBundle;
  overrides: State["overrides"];
  scenario: ResolvedCoprocessorScenario;
  requiresGitHub: boolean;
}): RuntimePlan => {
  const topology = topologyFromScenario(input.scenario);
  return {
    requiresGitHub: input.requiresGitHub,
    target: input.target,
    versions: input.versions,
    overrides: input.overrides,
    topology,
    coprocessor: input.scenario,
  };
};

export const runtimePlanForState = (
  state: Pick<
    State,
    "requiresGitHub" | "target" | "versions" | "overrides" | "scenario"
  >,
): RuntimePlan =>
  runtimePlanFromResolved({
    requiresGitHub: state.requiresGitHub ?? true,
    target: state.target,
    versions: state.versions,
    overrides: state.overrides,
    scenario: state.scenario,
  });
