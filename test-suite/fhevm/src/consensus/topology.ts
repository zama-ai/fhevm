import path from "node:path";
import type { State } from "../types";

export function consensusTopology(state: State, expected: { scenario?: string; operators?: string; threshold?: string }) {
  const source = state.scenarioSourcePath ?? state.scenario.sourcePath;
  const scenario = source ? path.basename(source, path.extname(source)) : "";
  if (state.scenario.kind !== "coprocessor-consensus" ||
      !["three-of-three", "three-of-three-fork", "three-of-three-heterogeneous-scheduling"].includes(scenario)) {
    throw new Error("active state is not a supported consensus topology");
  }
  const { count, threshold } = state.scenario.topology;
  if (count !== 3 || threshold !== 3 || state.scenario.hostChains.length !== 1) throw new Error("consensus suites require one host chain and a 3-of-3 fleet");
  for (const [name, claimed, actual] of [["scenario", expected.scenario, scenario], ["operators", expected.operators, String(count)], ["threshold", expected.threshold, String(threshold)]]) {
    if (claimed !== undefined && claimed !== actual) throw new Error(`requested ${name} disagrees with active stack state`);
  }
  return { scenario, count, threshold };
}

export function assertListenerRoute(command: string[], running: boolean, operator: number, scenario: string): void {
  if (!running) throw new Error(`operator ${operator} listener is not running`);
  const urls = command.flatMap((arg, i) => arg.startsWith("--url=") ? [arg.slice(6)] : arg === "--url" ? [command[i + 1]] : []);
  const expected = scenario === "three-of-three-fork" && operator === 2 ? "fork-anvil" : "host-node";
  if (urls.length !== 1 || !urls[0] || new URL(urls[0]).hostname !== expected) throw new Error(`operator ${operator} listener does not follow ${expected}`);
}

export function assertConsumerRoute(command: string[], running: boolean, operator: number, scenario: string): void {
  if (!running) throw new Error(`operator ${operator} consumer is not running`);
  const urls = command.flatMap((arg, i) => arg.startsWith("--url=") ? [arg.slice(6)] : arg === "--url" ? [command[i + 1]] : []);
  const expected = scenario === "three-of-three-fork" && operator === 2 ? "/9" : "/0";
  if (urls.length !== 1 || !urls[0]) throw new Error("consumer URL missing");
  const url = new URL(urls[0]);
  if (url.hostname !== "listener-redis" || (url.pathname || "/0") !== expected) throw new Error(`operator ${operator} consumer is not on its expected ingestion stream`);
}
