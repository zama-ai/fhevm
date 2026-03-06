import { describe, expect, test } from "bun:test";
import { mkdir, readFile, rm } from "fs/promises";

import {
  createInitialState,
  loadState,
  markPipelineCompleted,
  markPipelineFailed,
  markStepCompleted,
  markStepFailed,
  markStepRunning,
  saveState,
} from "./state";

function makeTempDir(): string {
  return `.fhevm/test-state/${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

const STEPS = [
  { number: 1, name: "minio" },
  { number: 2, name: "kms-core" },
] as const;

describe("pipeline state", () => {
  test("creates initial state", () => {
    const state = createInitialState(STEPS);
    expect(state.version).toBe(1);
    expect(state.status).toBe("running");
    expect(state.lastStep).toBe(0);
    expect(state.steps).toHaveLength(2);
    expect(state.steps[0]).toMatchObject({ number: 1, name: "minio", status: "pending" });
  });

  test("saves and loads state round-trip", async () => {
    const dir = makeTempDir();
    const path = `${dir}/state.json`;
    await mkdir(dir, { recursive: true });

    const state = createInitialState(STEPS);
    markStepRunning(state, 1);
    markStepCompleted(state, 1, 1234);
    state.runtime.minioIp = "172.18.0.2";

    await saveState(path, state);

    const loaded = await loadState(path);
    expect(loaded).not.toBeNull();
    expect(loaded?.lastStep).toBe(1);
    expect(loaded?.runtime.minioIp).toBe("172.18.0.2");

    const raw = await readFile(path, "utf8");
    expect(raw.endsWith("\n")).toBe(true);

    await rm(dir, { recursive: true, force: true });
  });

  test("returns null for missing state file", async () => {
    const loaded = await loadState(`.fhevm/test-state/missing-${Date.now()}.json`);
    expect(loaded).toBeNull();
  });

  test("rejects unsupported state version", async () => {
    const dir = makeTempDir();
    const path = `${dir}/state.json`;
    await mkdir(dir, { recursive: true });
    await Bun.write(path, JSON.stringify({ version: 999 }, null, 2));

    await expect(loadState(path)).rejects.toThrow("unsupported boot state version");

    await rm(dir, { recursive: true, force: true });
  });

  test("rejects malformed state without steps array", async () => {
    const dir = makeTempDir();
    const path = `${dir}/state.json`;
    await mkdir(dir, { recursive: true });
    await Bun.write(path, JSON.stringify({ version: 1, startedAt: new Date().toISOString() }, null, 2));

    await expect(loadState(path)).rejects.toThrow("steps must be an array");

    await rm(dir, { recursive: true, force: true });
  });

  test("rejects malformed state with invalid startedAt", async () => {
    const dir = makeTempDir();
    const path = `${dir}/state.json`;
    await mkdir(dir, { recursive: true });
    await Bun.write(
      path,
      JSON.stringify(
        {
          version: 1,
          startedAt: 123,
          steps: [{ number: 1, name: "minio", status: "pending" }],
        },
        null,
        2,
      ),
    );

    await expect(loadState(path)).rejects.toThrow("startedAt must be a string");

    await rm(dir, { recursive: true, force: true });
  });

  test("tracks step and pipeline transitions", () => {
    const state = createInitialState(STEPS);

    markStepRunning(state, 1);
    expect(state.steps[0]?.status).toBe("running");

    markStepCompleted(state, 1, 2500);
    expect(state.steps[0]).toMatchObject({ status: "completed", durationMs: 2500 });
    expect(state.lastStep).toBe(1);
    expect(state.lastStepName).toBe("minio");

    markStepRunning(state, 2);
    markStepFailed(state, 2, "boom");
    markPipelineFailed(state, 2, "boom");
    expect(state.steps[1]).toMatchObject({ status: "failed", error: "boom" });
    expect(state.status).toBe("failed");
    expect(state.failedStep).toBe(2);
    expect(state.failedStepName).toBe("kms-core");

    markPipelineCompleted(state);
    expect(state.status).toBe("completed");
    expect(state.failedStep).toBeUndefined();
  });
});
