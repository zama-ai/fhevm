/**
 * Combines resolved versions, overrides, and scenario topology into the stack spec used for generation and orchestration.
 */
import type {
  HostChainScenario,
  ResolvedBlueGreenScenario,
  ResolvedCoprocessorScenario,
  ResolvedKmsTopology,
  ResolvedScenario,
  State,
  Topology,
  UpOptions,
  VersionBundle,
} from "../types";
import { PreflightError } from "../errors";
import { versionBeforeReleaseFamily } from "../compat/compat";
import { SHA_REF, shortSha } from "../resolve/target";
import {
  assertScenarioOverrideCompatibility,
  defaultCoprocessorScenario,
  loadBlueGreenScenario,
  loadCoprocessorScenario,
  peekScenarioKind,
  resolveScenarioFile,
  resolveScenarioReference,
  synthesizeOverrideScenario,
} from "../scenario/resolve";

/** Passes release tags through; short-forms a full 40-hex SHA to the 7-char tag CI publishes. */
const normalizeBcsTag = (raw: string): string => {
  const trimmed = raw.trim();
  if (SHA_REF.test(trimmed)) {
    return shortSha(trimmed);
  }
  return trimmed;
};

export type StackSpec = {
  requiresGitHub: boolean;
  target: State["target"];
  versions: VersionBundle;
  overrides: State["overrides"];
  topology: Topology;
  hostChains: HostChainScenario[];
  coprocessor?: ResolvedCoprocessorScenario;
  blueGreen?: ResolvedBlueGreenScenario;
  kms: ResolvedKmsTopology;
};

/** Resolves the effective scenario from explicit input, overrides, or defaults. */
export const resolveScenarioForOptions = async (
  options: Pick<UpOptions, "overrides" | "scenarioPath" | "bcsTag" | "build">,
): Promise<ResolvedScenario> => {
  if (options.scenarioPath) {
    const sourcePath = await resolveScenarioReference(options.scenarioPath);
    const kind = await peekScenarioKind(sourcePath);
    if (kind === "blue-green") {
      // --build's implicit all-group overrides are fine (GCS builds from HEAD); only explicit --override conflicts.
      if (!options.build && options.overrides.some((override) => override.group === "coprocessor")) {
        throw new PreflightError("--override coprocessor is not supported with blue-green scenarios (BCS/GCS sources are scenario-defined)");
      }
      const loaded = await loadBlueGreenScenario(sourcePath);
      const resolved = options.bcsTag
        ? { ...loaded, bcs: { ...loaded.bcs, source: { mode: "registry" as const, tag: normalizeBcsTag(options.bcsTag) } } }
        : loaded;
      const source = resolved.bcs.source;
      // Pre-0.14 images lack the upgrade protocol (versioning table, FSM, retirement fence).
      if (source.mode === "registry" && versionBeforeReleaseFamily(source.tag, [0, 14, 0], { unparsed: "modern" })) {
        throw new PreflightError(`BCS tag "${source.tag}" predates v0.14 — blue-green requires a v0.14+ base`);
      }
      return resolved;
    }
    if (options.bcsTag) {
      throw new PreflightError("--bcs-tag only applies to blue-green scenarios");
    }
    const input = await loadCoprocessorScenario(sourcePath);
    const resolved = resolveScenarioFile(sourcePath, input);
    assertScenarioOverrideCompatibility(resolved, options.overrides);
    return resolved;
  }
  if (options.bcsTag) {
    throw new PreflightError("--bcs-tag requires --scenario");
  }
  return synthesizeOverrideScenario(options.overrides) ?? defaultCoprocessorScenario();
};

/** Derives runtime topology fields from a resolved scenario. */
const topologyFromScenario = (scenario: ResolvedCoprocessorScenario): Topology => ({
  count: scenario.topology.count,
  threshold: scenario.topology.threshold,
});

export const topologyForState = (state: Pick<State, "scenario">): Topology => ({
  count: state.scenario.topology.count,
  threshold: state.scenario.topology.threshold,
});

/** Packages resolved versions, overrides, and scenario state into one stack spec. */
const stackSpecFromResolved = (input: {
  target: State["target"];
  versions: VersionBundle;
  overrides: State["overrides"];
  scenario: ResolvedScenario;
  requiresGitHub: boolean;
}): StackSpec => {
  if (input.scenario.kind === "blue-green") {
    // Synthesize N BCS instances so the multi-op compose gen applies; GCS is layered on in buildCoprocessorOverride.
    const bg = input.scenario;
    const bcsAsCoprocessor: ResolvedCoprocessorScenario = {
      version: 1,
      kind: "coprocessor-consensus",
      origin: bg.origin === "file" ? "file" : "default",
      name: bg.name,
      description: bg.description,
      hostChains: bg.hostChains,
      sourcePath: bg.sourcePath,
      topology: bg.topology,
      instances: Array.from({ length: bg.topology.count }, (_, index) => ({
        index,
        source: bg.bcs.source,
        env: bg.bcs.env,
        args: bg.bcs.args,
      })),
      kms: bg.kms,
    };
    return {
      requiresGitHub: input.requiresGitHub,
      target: input.target,
      versions: input.versions,
      overrides: input.overrides,
      topology: bg.topology,
      hostChains: bg.hostChains,
      coprocessor: bcsAsCoprocessor,
      blueGreen: bg,
      kms: bg.kms,
    };
  }
  return {
    requiresGitHub: input.requiresGitHub,
    target: input.target,
    versions: input.versions,
    overrides: input.overrides,
    topology: topologyFromScenario(input.scenario),
    hostChains: input.scenario.hostChains,
    coprocessor: input.scenario,
    kms: input.scenario.kms,
  };
};

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
