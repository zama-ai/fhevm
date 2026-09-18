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
