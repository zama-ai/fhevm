import { expect, test } from "bun:test";
import { assertConsumerRoute, assertListenerRoute, consensusTopology } from "./topology";
import type { State } from "../types";
const state = (threshold = 3) => ({ scenario: { kind: "coprocessor-consensus", sourcePath: "/scenarios/three-of-three.yaml",
  topology: { count: 3, threshold }, hostChains: [{}] } }) as State;
test("topology expectations cannot relabel a different active state", () => {
  expect(consensusTopology(state(), {}).threshold).toBe(3);
  expect(() => consensusTopology(state(2), {})).toThrow();
  expect(() => consensusTopology(state(), { threshold: "2" })).toThrow();
  expect(() => consensusTopology(state(), { scenario: "three-of-three-fork" })).toThrow();
});
test("fork routing is checked against running listener arguments", () => {
  expect(() => assertListenerRoute(["--url=http://fork-anvil:8546"], true, 2, "three-of-three-fork")).not.toThrow();
  expect(() => assertListenerRoute(["--url=http://host-node:8545"], true, 2, "three-of-three-fork")).toThrow();
  expect(() => assertListenerRoute(["--url=http://fork-anvil:8546"], false, 2, "three-of-three-fork")).toThrow();
  expect(() => assertListenerRoute(["--url=http://fork-anvil:8546"], true, 0, "three-of-three")).toThrow();
});
test("the fork consumer must use the fork listener's isolated stream", () => {
  expect(() => assertConsumerRoute(["--url=redis://listener-redis:6379/9"], true, 2, "three-of-three-fork")).not.toThrow();
  expect(() => assertConsumerRoute(["--url=redis://listener-redis:6379"], true, 2, "three-of-three-fork")).toThrow();
  expect(() => assertConsumerRoute(["--url=redis://listener-redis:6379/9"], false, 2, "three-of-three-fork")).toThrow();
  expect(() => assertConsumerRoute(["--url=redis://listener-redis:6379"], true, 0, "three-of-three")).not.toThrow();
});
test("two-of-three is observed without relabeling it as unanimous", () => {
  const value = state(2);
  value.scenario.sourcePath = "/scenarios/two-of-three.yaml";
  expect(consensusTopology(value, { threshold: "2" }).scenario).toBe("two-of-three");
  expect(() => consensusTopology(value, { threshold: "3" })).toThrow();
});
test("generated DEG-06 topology must retain its declared shape and every override", () => {
  const value = state();
  if (value.scenario.kind !== "coprocessor-consensus") throw new Error("fixture");
  value.scenario.sourcePath = "/tmp/consensus-degraded-Ab1234.yaml";
  value.scenario.name = "Three Of Three";
  value.scenario.instances = [0, 1, 2].map(index => ({ index, source: { mode: "inherit" as const }, args: {}, env: { DRIFT_AUTO_REVERT_ENABLED: "false" } }));
  expect(consensusTopology(value, { scenario: "three-of-three" }).scenario).toBe("three-of-three");
  value.scenario.instances[1].env = {};
  expect(() => consensusTopology(value, {})).toThrow();
});

test("bridge topology requires both chains and their actual listener routes", () => {
  const value = state(2);
  value.scenario.sourcePath = "/scenarios/two-of-two-multi-chain.yaml";
  if (value.scenario.kind !== "coprocessor-consensus") throw new Error("fixture");
  value.scenario.topology.count = 2;
  value.scenario.hostChains = [{key: "host", chainId: "12345", rpcPort: 8545}, {key: "chain-b", chainId: "67890", rpcPort: 8547}];
  expect(consensusTopology(value, {}).count).toBe(2);
  expect(() => assertListenerRoute(["--url=http://host-node:8545"], true, 0, "two-of-two-multi-chain", "host-node-chain-b")).toThrow();
  expect(() => assertListenerRoute(["--url=http://host-node-chain-b:8545"], true, 0, "two-of-two-multi-chain", "host-node-chain-b")).not.toThrow();
  value.scenario.hostChains.pop();
  expect(() => consensusTopology(value, {})).toThrow();
});
