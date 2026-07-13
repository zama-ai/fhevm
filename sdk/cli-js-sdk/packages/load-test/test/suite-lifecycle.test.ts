import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  executeRun: vi.fn(),
  loadScenario: vi.fn(),
  inspectPoolRequirements: vi.fn(),
  preparePoolRequirements: vi.fn(),
  assertRelayerReadiness: vi.fn(),
}));

vi.mock("../src/runner/run", () => ({
  executeRun: mocks.executeRun,
  RunInterruptedError: class RunInterruptedError extends Error {},
}));
vi.mock("../src/scenario/load", () => ({ loadScenario: mocks.loadScenario }));
vi.mock("../src/pool/planning", () => ({
  inspectPoolRequirements: mocks.inspectPoolRequirements,
  preparePoolRequirements: mocks.preparePoolRequirements,
  formatPoolPlan: () => ["pool plan"],
}));
vi.mock("../src/runner/readiness", () => ({
  assertRelayerReadiness: mocks.assertRelayerReadiness,
}));
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
import type { PoolPlanArtifact } from "../src/pool/planning";
import { scenarioSchema } from "../src/scenario/schema";
import { prepareSuite, runSuite } from "../src/suite/run";
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

const readyPlan = {
  version: 2,
  kind: "load-test-pool-plan",
  observedAt: "2026-01-01T00:00:00.000Z",
  scenarioDigest: "digest",
  scenarios: ["suite-entry"],
  pauseSec: 0,
  environment: {
    network: "testnet", contractChainId: 11_155_111,
    contractAddress: "0x0000000000000000000000000000000000000001",
    relayer: "https://relayer.example",
  },
  requiredDelegationValidUntil: "3000",
  ready: true,
  items: [],
  plannedActions: [],
} as const satisfies PoolPlanArtifact;

const blockedPlan = {
  ...readyPlan,
  ready: false,
  items: [{
    pool: "delegated-user-decrypt-handles",
    flow: "delegated-user-decrypt",
    requirement: {
      workload: { mode: "finite", requestBudget: 1 },
      requiredValidUntil: "3000",
    },
    observation: { currentItems: 4, availableItems: 4 },
    decision: {
      deficitItems: 0,
      refreshRequired: true,
      ready: false,
      detail: "ACL refresh required",
    },
  }],
  plannedActions: [{
    kind: "refresh-delegation-acl",
    pool: "delegated-user-decrypt-handles",
    flow: "delegated-user-decrypt",
    requiredValidUntil: "3000",
  }],
} as const;

beforeEach(async () => {
  dir = await mkdtemp(join(tmpdir(), "load-test-suite-lifecycle-"));
  mocks.executeRun.mockReset().mockResolvedValue({
    report,
    outputDir: join(dir, "run"),
    status: "completed",
  });
  mocks.loadScenario.mockReset().mockResolvedValue(scenario);
  mocks.inspectPoolRequirements.mockReset().mockResolvedValue(readyPlan);
  mocks.preparePoolRequirements.mockReset().mockResolvedValue({
    plan: readyPlan,
    preparation: { status: "completed", finalReady: true },
  });
  mocks.assertRelayerReadiness.mockReset().mockResolvedValue(undefined);
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
    expect(mocks.inspectPoolRequirements).not.toHaveBeenCalled();
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

  it("gates explicit preparation on readiness and persists preparation", async () => {
    const suite = suiteSchema.parse({
      name: "acl-refresh",
      entries: [{ scenario: "open-steady" }],
      pauseSec: 0,
    });
    mocks.inspectPoolRequirements.mockResolvedValueOnce(blockedPlan);
    mocks.preparePoolRequirements.mockImplementationOnce(async (options) => {
      await options.beforeActions?.(blockedPlan);
      return { plan: readyPlan, preparation: { status: "completed", finalReady: true } };
    });

    const result = await prepareSuite({
      env: env(dir),
      suite,
      outputRoot: join(dir, "suite"),
    });

    expect(mocks.assertRelayerReadiness).toHaveBeenCalledOnce();
    expect(mocks.preparePoolRequirements).toHaveBeenCalledWith(
      expect.objectContaining({ artifactDir: join(dir, "suite"), pauseSec: 0 }),
    );
    expect(mocks.executeRun).not.toHaveBeenCalled();
    expect(mocks.preparePoolRequirements.mock.invocationCallOrder[0])
      .toBeLessThan(mocks.assertRelayerReadiness.mock.invocationCallOrder[0]!);
    expect(result).toMatchObject({ status: "completed", ready: true });
    const summary = JSON.parse(
      await readFile(join(dir, "suite", "suite-preparation.json"), "utf8"),
    ) as { status: string; ready: boolean };
    expect(summary).toMatchObject({ status: "completed", ready: true });
    await expect(readFile(join(dir, "suite", "suite-summary.json"), "utf8"))
      .rejects.toThrow();
  });

  it("records ready no-op preparation under the preparations root", async () => {
    const suite = suiteSchema.parse({
      name: "already-ready", entries: [{ scenario: "open-steady" }], pauseSec: 0,
    });
    const result = await prepareSuite({ env: env(dir), suite });

    expect(result.outputRoot).toContain(join(dir, "preparations"));
    expect(mocks.assertRelayerReadiness).not.toHaveBeenCalled();
    expect(mocks.preparePoolRequirements).toHaveBeenCalledOnce();
    expect(await readFile(join(result.outputRoot, "suite-preparation.md"), "utf8"))
      .toContain("**Final readiness:** ready");
    await expect(readFile(join(result.outputRoot, "suite-summary.md"), "utf8"))
      .rejects.toThrow();
  });

  it("returns durable interrupted preparation instead of rethrowing abort", async () => {
    const controller = new AbortController();
    controller.abort();
    const suite = suiteSchema.parse({
      name: "pre-aborted", entries: [{ scenario: "open-steady" }], pauseSec: 0,
    });
    const outputRoot = join(dir, "pre-aborted");

    const result = await prepareSuite({ env: env(dir), suite, outputRoot, signal: controller.signal });

    expect(result).toMatchObject({ status: "interrupted", ready: false, outputRoot });
    expect(JSON.parse(await readFile(join(outputRoot, "suite-preparation.json"), "utf8")))
      .toMatchObject({ status: "interrupted", ready: false });
    expect(mocks.inspectPoolRequirements).not.toHaveBeenCalled();
    expect(mocks.preparePoolRequirements).not.toHaveBeenCalled();
  });

  it("does not disguise a non-abort failure when the signal is also aborted", async () => {
    const controller = new AbortController();
    mocks.inspectPoolRequirements.mockImplementationOnce(async () => {
      controller.abort();
      throw new Error("planning exploded");
    });
    const suite = suiteSchema.parse({
      name: "real-failure", entries: [{ scenario: "open-steady" }], pauseSec: 0,
    });
    const outputRoot = join(dir, "real-failure");

    await expect(prepareSuite({
      env: env(dir), suite, outputRoot, signal: controller.signal,
    })).rejects.toThrow("planning exploded");
    expect(JSON.parse(await readFile(join(outputRoot, "suite-preparation.json"), "utf8")))
      .toMatchObject({ status: "failed", ready: false, error: "planning exploded" });
  });

  it("fails inside the authoritative preparation gate and persists a failed suite summary", async () => {
    mocks.inspectPoolRequirements.mockResolvedValueOnce(blockedPlan);
    mocks.assertRelayerReadiness.mockRejectedValueOnce(new Error("candidate unavailable"));
    mocks.preparePoolRequirements.mockImplementationOnce(async (options) => {
      await options.beforeActions?.(blockedPlan);
      return { plan: readyPlan, preparation: { status: "completed", finalReady: true } };
    });
    const suite = suiteSchema.parse({
      name: "not-ready", entries: [{ scenario: "open-steady" }], pauseSec: 0,
    });
    const outputRoot = join(dir, "not-ready");

    await expect(runSuite({
      env: env(dir), suite, outputRoot, prepare: true,
    })).rejects.toThrow("candidate unavailable");

    expect(mocks.preparePoolRequirements).toHaveBeenCalledOnce();
    expect(mocks.executeRun).not.toHaveBeenCalled();
    expect(JSON.parse(
      await readFile(join(outputRoot, "suite-summary.json"), "utf8"),
    )).toMatchObject({ status: "failed", passed: false });
  });

  it("refuses execution when preparation reports residual work", async () => {
    mocks.inspectPoolRequirements.mockResolvedValueOnce(blockedPlan);
    mocks.preparePoolRequirements.mockImplementationOnce(async (options) => {
      await options.beforeActions?.(blockedPlan);
      return {
        plan: blockedPlan,
        preparation: { status: "failed", finalReady: false },
      };
    });
    const suite = suiteSchema.parse({
      name: "residual", entries: [{ scenario: "open-steady" }], pauseSec: 0,
    });

    await expect(runSuite({
      env: env(dir), suite, outputRoot: join(dir, "residual"), prepare: true,
    })).rejects.toThrow(/residual work/);
    expect(mocks.executeRun).not.toHaveBeenCalled();
  });

  it("blocks an ordinary run without mutating when preparation is required", async () => {
    mocks.inspectPoolRequirements.mockResolvedValueOnce(blockedPlan);
    const suite = suiteSchema.parse({
      name: "blocked", entries: [{ scenario: "open-steady" }], pauseSec: 0,
    });
    const result = await runSuite({
      env: env(dir), suite, outputRoot: join(dir, "blocked"),
    });

    expect(result.status).toBe("blocked");
    expect(result.passed).toBe(false);
    expect(mocks.assertRelayerReadiness).not.toHaveBeenCalled();
    expect(mocks.preparePoolRequirements).not.toHaveBeenCalled();
    expect(mocks.executeRun).not.toHaveBeenCalled();
    expect(await readFile(join(dir, "blocked", "suite-summary.md"), "utf8"))
      .toContain("**Status:** ⛔ blocked");
  });

  it("writes a failed terminal summary when pool planning fails", async () => {
    const suite = suiteSchema.parse({
      name: "plan-failure", entries: [{ scenario: "open-steady" }], pauseSec: 0,
    });
    const outputRoot = join(dir, "plan-failure");
    mocks.inspectPoolRequirements.mockRejectedValueOnce(new Error("planning exploded"));
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
