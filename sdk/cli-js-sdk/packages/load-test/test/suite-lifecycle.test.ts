import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  executeRun: vi.fn(),
  loadScenario: vi.fn(),
  planPools: vi.fn(),
  requiredDelegationValidUntil: vi.fn(),
  createHandlePool: vi.fn(),
  refreshDelegatedHandlePool: vi.fn(),
}));

vi.mock("../src/runner/run", () => ({
  executeRun: mocks.executeRun,
  RunInterruptedError: class RunInterruptedError extends Error {},
}));
vi.mock("../src/scenario/load", () => ({ loadScenario: mocks.loadScenario }));
vi.mock("../src/suite/requirements", () => ({
  planPools: mocks.planPools,
  requiredDelegationValidUntil: mocks.requiredDelegationValidUntil,
}));
vi.mock("../src/pool/handles", () => ({
  createHandlePool: mocks.createHandlePool,
  refreshDelegatedHandlePool: mocks.refreshDelegatedHandlePool,
}));
vi.mock("../src/pool/input-proof", () => ({ generateInputProofPool: vi.fn() }));
vi.mock("../src/shared/logger", () => ({
  logger: {
    error: vi.fn(),
    info: vi.fn(),
    start: vi.fn(),
    success: vi.fn(),
    warn: vi.fn(),
  },
}));

import type { Report } from "../src/report/schema";
import { scenarioSchema } from "../src/scenario/schema";
import { runSuite } from "../src/suite/run";
import { suiteSchema } from "../src/suite/schema";

let dir: string;

const scenario = scenarioSchema.parse({
  name: "suite-entry",
  flows: [{ flow: "input-proof", weight: 1 }],
  shape: { kind: "burst", count: 1 },
});

const report = {
  version: 1,
  run: {
    scenario,
    model: "drain",
    network: "testnet",
    relayerUrl: "https://relayer.example",
    startedAt: "2026-01-01T00:00:00.000Z",
    endedAt: "2026-01-01T00:00:01.000Z",
    plannedRequests: 1,
    submitted: 1,
    completed: 1,
    abandoned: 0,
    poolExhausted: false,
    submissionDurationMs: 1,
    achievedWorkflowsPerSec: 1,
  },
  targets: [],
  thresholds: { passed: true, breaches: [] },
} as unknown as Report;

const env = (dataDir: string) => ({
  network: "testnet" as const,
  contractChainId: 11_155_111,
  relayerUrl: "https://relayer.example",
  dataDir,
});

beforeEach(async () => {
  dir = await mkdtemp(join(tmpdir(), "load-test-suite-lifecycle-"));
  mocks.executeRun.mockReset().mockResolvedValue({
    report,
    outputDir: join(dir, "run"),
    status: "completed",
  });
  mocks.loadScenario.mockReset().mockResolvedValue(scenario);
  mocks.planPools.mockReset().mockResolvedValue([]);
  mocks.requiredDelegationValidUntil.mockReset().mockReturnValue(3_000n);
  mocks.createHandlePool.mockReset();
  mocks.refreshDelegatedHandlePool.mockReset();
});

afterEach(async () => {
  await rm(dir, { recursive: true, force: true });
});

describe("runSuite interruption", () => {
  it("reports a pre-aborted suite as interrupted and non-passing", async () => {
    const controller = new AbortController();
    controller.abort();
    const suite = suiteSchema.parse({
      name: "interrupted",
      entries: [{ scenario: "open-steady" }],
      pauseSec: 0,
    });

    const result = await runSuite({
      env: env(dir),
      suite,
      outputRoot: join(dir, "suite"),
      signal: controller.signal,
    });

    expect(result.status).toBe("interrupted");
    expect(result.passed).toBe(false);
    expect(result.entries).toEqual([]);
    expect(await readFile(join(dir, "suite", "suite-summary.md"), "utf8"))
      .toContain("**Status:** ⏹ interrupted");
    expect(mocks.planPools).not.toHaveBeenCalled();
    expect(mocks.executeRun).not.toHaveBeenCalled();
  });

  it("does not start the next scenario when interrupted during the pause", async () => {
    const controller = new AbortController();
    const suite = suiteSchema.parse({
      name: "pause-interrupt",
      entries: [
        { scenario: "open-steady", label: "first" },
        { scenario: "open-steady", label: "second" },
      ],
      pauseSec: 0.05,
    });
    mocks.executeRun.mockImplementationOnce(async () => {
      setTimeout(() => controller.abort(), 10);
      return { report, outputDir: join(dir, "first"), status: "completed" };
    });

    const result = await runSuite({
      env: env(dir),
      suite,
      outputRoot: join(dir, "suite"),
      signal: controller.signal,
    });

    expect(result.status).toBe("interrupted");
    expect(result.passed).toBe(false);
    expect(result.entries).toHaveLength(1);
    expect(mocks.executeRun).toHaveBeenCalledOnce();
  });

  it("propagates an interrupted run status and stops the suite", async () => {
    const suite = suiteSchema.parse({
      name: "run-interrupt",
      entries: [
        { scenario: "open-steady", label: "first" },
        { scenario: "open-steady", label: "second" },
      ],
      pauseSec: 0,
    });
    mocks.executeRun.mockResolvedValueOnce({
      report,
      outputDir: join(dir, "first"),
      status: "interrupted",
    });

    const result = await runSuite({
      env: env(dir),
      suite,
      outputRoot: join(dir, "suite"),
    });

    expect(result.status).toBe("interrupted");
    expect(result.passed).toBe(false);
    expect(result.entries).toHaveLength(1);
    expect(mocks.executeRun).toHaveBeenCalledOnce();
  });

  it("refreshes delegated ACL state even when the handle deficit is zero", async () => {
    const suite = suiteSchema.parse({
      name: "acl-refresh",
      entries: [{ scenario: "open-steady" }],
      pauseSec: 0,
    });
    mocks.planPools.mockResolvedValueOnce([{
      pool: "delegated-user-decrypt-handles",
      flow: "delegated-user-decrypt",
      current: 4,
      available: 4,
      needed: 1,
      deficit: 0,
      refreshRequired: true,
      requiredValidUntil: "2000",
      detail: "ACL refresh required",
    }]);

    await runSuite({
      env: env(dir),
      suite,
      outputRoot: join(dir, "suite"),
      prepareOnly: true,
    });

    expect(mocks.createHandlePool).not.toHaveBeenCalled();
    expect(mocks.refreshDelegatedHandlePool).toHaveBeenCalledWith(
      expect.anything(),
      expect.objectContaining({ requiredValidUntil: 3_000n }),
    );
    expect(mocks.requiredDelegationValidUntil).toHaveBeenCalledWith(
      expect.any(Array),
      { pauseSec: 0 },
    );
    const summary = JSON.parse(
      await readFile(join(dir, "suite", "suite-summary.json"), "utf8"),
    ) as { status: string };
    expect(summary.status).toBe("completed");
    expect(await readFile(join(dir, "suite", "suite-summary.md"), "utf8"))
      .toContain("**Status:** ✅ completed");
  });

  it("writes a failed terminal summary when pool planning fails", async () => {
    const suite = suiteSchema.parse({
      name: "plan-failure", entries: [{ scenario: "open-steady" }], pauseSec: 0,
    });
    const outputRoot = join(dir, "plan-failure");
    mocks.planPools.mockRejectedValueOnce(new Error("planning exploded"));
    await expect(runSuite({ env: env(dir), suite, outputRoot }))
      .rejects.toThrow("planning exploded");
    const summary = JSON.parse(
      await readFile(join(outputRoot, "suite-summary.json"), "utf8"),
    ) as { status: string; passed: boolean };
    expect(summary).toMatchObject({ status: "failed", passed: false });
    expect(await readFile(join(outputRoot, "suite-summary.md"), "utf8"))
      .toContain("**Status:** ❌ failed");
  });

  it("writes a failed terminal summary before propagating a run failure", async () => {
    const suite = suiteSchema.parse({
      name: "failed-suite", entries: [{ scenario: "open-steady" }], pauseSec: 0,
    });
    const outputRoot = join(dir, "failed-suite");
    mocks.executeRun.mockRejectedValueOnce(new Error("run exploded"));
    await expect(runSuite({ env: env(dir), suite, outputRoot })).rejects.toThrow("run exploded");
    const summary = JSON.parse(await readFile(join(outputRoot, "suite-summary.json"), "utf8")) as {
      status: string; passed: boolean;
    };
    expect(summary).toMatchObject({ status: "failed", passed: false });
  });

  it("hard-fails on a corrupt existing baseline and persists the failure", async () => {
    const suite = suiteSchema.parse({
      name: "bad-baseline", entries: [{ scenario: "open-steady" }], pauseSec: 0,
    });
    const outputRoot = join(dir, "bad-baseline");
    const baselinesDir = join(dir, "baselines");
    await mkdir(join(baselinesDir, "testnet"), { recursive: true });
    await writeFile(join(baselinesDir, "testnet", "suite-entry.json"), "{not-json}\n");

    await expect(runSuite({ env: env(dir), suite, outputRoot, baselinesDir }))
      .rejects.toThrow();
    const summary = JSON.parse(await readFile(join(outputRoot, "suite-summary.json"), "utf8")) as {
      status: string; passed: boolean;
    };
    expect(summary).toMatchObject({ status: "failed", passed: false });
  });

  it("summarizes paired runs as logical workflows with per-target outcomes", async () => {
    const pairedReport = {
      ...report,
      run: { ...report.run, submitted: 1 },
      targets: [
        {
          target: "A",
          flows: [{ submitted: 1, succeeded: 1, aborted: 0, verifyFailed: 0 }],
        },
        {
          target: "B",
          flows: [{ submitted: 1, succeeded: 0, aborted: 0, verifyFailed: 1 }],
        },
      ],
      comparison: { flows: [{ differentTerminalOutcome: 1 }] },
      thresholds: { passed: false, breaches: [] },
    } as unknown as Report;
    mocks.executeRun.mockResolvedValueOnce({
      report: pairedReport,
      outputDir: join(dir, "paired", "suite-entry"),
      status: "completed",
    });
    const suite = suiteSchema.parse({
      name: "paired", entries: [{ scenario: "open-steady" }], pauseSec: 0,
    });

    const result = await runSuite({ env: env(dir), suite, outputRoot: join(dir, "paired") });

    expect(result.entries[0]).toMatchObject({
      submitted: 1,
      errorRate: 1,
      verifyFailed: 1,
      differentTerminalOutcomes: 1,
      targets: [
        { target: "A", errorRate: 0, verifyFailed: 0 },
        { target: "B", errorRate: 1, verifyFailed: 1 },
      ],
    });
    expect(result.passed).toBe(false);
    expect(await readFile(join(dir, "paired", "suite-summary.md"), "utf8"))
      .toContain("**Status:** ❌ completed with breaches");
  });
});
