import path from "node:path";
import type { State } from "../types";

export function consensusTopology(state: State, expected: { scenario?: string; operators?: string; threshold?: string }) {
  const source = state.scenarioSourcePath ?? state.scenario.sourcePath;
  let scenario = source ? path.basename(source, path.extname(source)) : "";
  const instances = state.scenario.kind === "coprocessor-consensus" ? state.scenario.instances : undefined;
  // DEG-06 uses a generated copy to disable automatic revert. Recognize only
  // that authored shape; an arbitrary temporary scenario cannot relabel itself.
  if (state.scenario.kind === "coprocessor-consensus" && /^consensus-degraded-[A-Za-z0-9]+$/.test(scenario) &&
      state.scenario.name === "Three Of Three" &&
      instances?.length === 3 &&
      [0, 1, 2].every(index => instances?.filter(instance =>
        instance.index === index && instance.env?.DRIFT_AUTO_REVERT_ENABLED === "false").length === 1)) {
    scenario = "three-of-three";
  }
  if (state.scenario.kind !== "coprocessor-consensus" ||
      !["three-of-three", "two-of-three", "three-of-three-fork", "three-of-three-heterogeneous-scheduling", "two-of-two-multi-chain", "three-of-three-backlog"].includes(scenario)) {
    throw new Error("active state is not a supported consensus topology");
  }
  const { count, threshold } = state.scenario.topology;
  const bridge = scenario === "two-of-two-multi-chain";
  if (count !== (bridge ? 2 : 3) || threshold !== (bridge || scenario === "two-of-three" ? 2 : 3) ||
      state.scenario.hostChains.length !== (bridge ? 2 : 1)) throw new Error("consensus suite topology does not match its declared fleet and host-chain count");
  for (const [name, claimed, actual] of [["scenario", expected.scenario, scenario], ["operators", expected.operators, String(count)], ["threshold", expected.threshold, String(threshold)]]) {
    if (claimed !== undefined && claimed !== actual) throw new Error(`requested ${name} disagrees with active stack state`);
  }
  return { scenario, count, threshold };
}

export function assertListenerRoute(command: string[], running: boolean, operator: number, scenario: string, hostNode = "host-node"): void {
  if (!running) throw new Error(`operator ${operator} listener is not running`);
  const urls = command.flatMap((arg, i) => arg.startsWith("--url=") ? [arg.slice(6)] : arg === "--url" ? [command[i + 1]] : []);
  const expected = scenario === "three-of-three-fork" && operator === 2 ? "fork-anvil" : hostNode;
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
