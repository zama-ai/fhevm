import { describe, expect, test } from "bun:test";
import { Effect } from "effect";
import { StateManager } from "./StateManager";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import type { State, StepName } from "../types";
import { defaultCoprocessorScenario } from "../scenario";

const makeTestState = (overrides: Partial<State> = {}): State => ({
  target: "latest-release",
  lockPath: "/tmp/test.json",
  versions: { target: "latest-release", lockName: "test.json", env: {}, sources: [] },
  overrides: [],
  scenario: defaultCoprocessorScenario(),
  completedSteps: [] as StepName[],
  updatedAt: "2026-03-16T00:00:00.000Z",
  ...overrides,
});

describe("StateManager", () => {
  test("save and load round-trips state", async () => {
    const dir = await fs.mkdtemp(path.join(os.tmpdir(), "sm-test-"));
    const file = path.join(dir, "state.json");
    const state = makeTestState();
    const mgr = StateManager.makeForPath(file);
    const program = Effect.gen(function* () {
      yield* mgr.save(state);
      return yield* mgr.load;
    });
    const result = await Effect.runPromise(program);
    expect(result).toBeDefined();
    expect(result!.target).toBe("latest-release");
    expect(result!.completedSteps).toEqual([]);
    expect(result!.scenario.topology).toEqual({ count: 1, threshold: 1 });
    await fs.rm(dir, { recursive: true });
  });

  test("save persists scenario only", async () => {
    const dir = await fs.mkdtemp(path.join(os.tmpdir(), "sm-test-"));
    const file = path.join(dir, "state.json");
    const state = makeTestState();
    const mgr = StateManager.makeForPath(file);
    await Effect.runPromise(mgr.save(state));
    const raw = JSON.parse(await fs.readFile(file, "utf8")) as Record<string, unknown>;
    expect(raw.topology).toBeUndefined();
    const loaded = await Effect.runPromise(mgr.load);
    expect(loaded?.scenario.topology).toEqual({ count: 1, threshold: 1 });
    await fs.rm(dir, { recursive: true });
  });

  test("load returns undefined when file missing", async () => {
    const dir = await fs.mkdtemp(path.join(os.tmpdir(), "sm-test-"));
    const file = path.join(dir, "nonexistent.json");
    const mgr = StateManager.makeForPath(file);
    const result = await Effect.runPromise(mgr.load);
    expect(result).toBeUndefined();
    await fs.rm(dir, { recursive: true });
  });

  test("markStep adds step and updates timestamp", async () => {
    const dir = await fs.mkdtemp(path.join(os.tmpdir(), "sm-test-"));
    const file = path.join(dir, "state.json");
    const state = makeTestState();
    const mgr = StateManager.makeForPath(file);
    await Effect.runPromise(mgr.save(state));
    await Effect.runPromise(mgr.markStep(state, "preflight"));
    expect(state.completedSteps).toContain("preflight");
    expect(state.updatedAt).not.toBe("2026-03-16T00:00:00.000Z");
    await fs.rm(dir, { recursive: true });
  });

  test("markStep does not duplicate existing step", async () => {
    const dir = await fs.mkdtemp(path.join(os.tmpdir(), "sm-test-"));
    const file = path.join(dir, "state.json");
    const state = makeTestState({ completedSteps: ["preflight"] as StepName[] });
    const mgr = StateManager.makeForPath(file);
    await Effect.runPromise(mgr.save(state));
    await Effect.runPromise(mgr.markStep(state, "preflight"));
    expect(state.completedSteps.filter((s) => s === "preflight").length).toBe(1);
    await fs.rm(dir, { recursive: true });
  });

  test("markStep persists to disk", async () => {
    const dir = await fs.mkdtemp(path.join(os.tmpdir(), "sm-test-"));
    const file = path.join(dir, "state.json");
    const state = makeTestState();
    const mgr = StateManager.makeForPath(file);
    await Effect.runPromise(mgr.save(state));
    await Effect.runPromise(mgr.markStep(state, "resolve"));
    const reloaded = await Effect.runPromise(mgr.load);
    expect(reloaded).toBeDefined();
    expect(reloaded!.completedSteps).toContain("resolve");
    await fs.rm(dir, { recursive: true });
  });

  test("clear removes persisted state", async () => {
    const dir = await fs.mkdtemp(path.join(os.tmpdir(), "sm-test-"));
    const file = path.join(dir, "state.json");
    const mgr = StateManager.makeForPath(file);
    await Effect.runPromise(mgr.save(makeTestState()));
    await Effect.runPromise(mgr.clear);
    expect(await Effect.runPromise(mgr.load)).toBeUndefined();
    await fs.rm(dir, { recursive: true });
  });
});
