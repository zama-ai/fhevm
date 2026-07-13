import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  buildReport: vi.fn(),
  clients: [] as Array<{
    apiKey?: string;
    close: ReturnType<typeof vi.fn>;
    isReady: ReturnType<typeof vi.fn>;
  }>,
  collectorInstances: [] as Array<{ stop: ReturnType<typeof vi.fn> }>,
  createFlowExecutor: vi.fn(),
  events: [] as string[],
  injectorInstances: [] as Array<{ stop: ReturnType<typeof vi.fn> }>,
  recorder: {
    records: [] as unknown[],
    close: vi.fn(),
  },
  runScheduler: vi.fn(),
}));

vi.mock("../src/relayer/client", () => ({
  RelayerClient: class {
    readonly apiKey?: string;
    readonly close = vi.fn(async () => {
      mocks.events.push("client.stop");
    });
    readonly isReady = vi.fn().mockResolvedValue(true);

    constructor(options: { apiKey?: string }) {
      this.apiKey = options.apiKey;
      mocks.clients.push(this);
    }
  },
}));

vi.mock("../src/flows", () => ({
  createFlowExecutor: mocks.createFlowExecutor,
}));

vi.mock("../src/collectors/prometheus", () => ({
  createPrometheusHttpSource: vi.fn(() => vi.fn()),
  PrometheusCollector: class {
    readonly capabilities = undefined;
    readonly samples: unknown[] = [];
    readonly snapshots: unknown[] = [];
    readonly start = vi.fn().mockResolvedValue(undefined);
    readonly stop = vi.fn(async () => {
      mocks.events.push("prometheus.stop");
    });

    constructor() {
      mocks.collectorInstances.push(this);
    }
  },
}));

vi.mock("../src/collectors/injector-runtime", () => ({
  InjectorRuntimeCollector: class {
    readonly start = vi.fn().mockResolvedValue(undefined);
    readonly stop = vi.fn(async () => {
      mocks.events.push("injector.stop");
    });
    readonly recordAbandoned = vi.fn();
    readonly recordDispatch = vi.fn();
    readonly recordDropped = vi.fn();
    readonly summary = vi.fn(() => ({ sampleCount: 0, dropped: 0, abandoned: 0 }));

    constructor() {
      mocks.injectorInstances.push(this);
    }
  },
}));

vi.mock("../src/collectors/relayer-config", () => ({
  snapshotRelayerConfig: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("../src/runner/recorder", () => ({
  Recorder: {
    open: vi.fn().mockImplementation(async () => mocks.recorder),
  },
}));

vi.mock("../src/runner/scheduler", () => ({
  runScheduler: mocks.runScheduler,
}));

vi.mock("../src/report/build", () => ({
  buildReport: (...args: unknown[]) => {
    mocks.events.push("report.build");
    return mocks.buildReport(...args);
  },
}));

vi.mock("../src/report/render-md", () => ({
  renderMarkdownReport: vi.fn(() => "report\n"),
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

import type { FlowExecutor } from "../src/flows/types";
import { executeRun } from "../src/runner/run";
import { scenarioSchema } from "../src/scenario/schema";

let dir: string;

const scenario = scenarioSchema.parse({
  name: "lifecycle",
  flows: [{ flow: "input-proof", weight: 1 }],
  shape: { kind: "burst", count: 1 },
});

const env = (dataDir: string) => ({
  network: "testnet" as const,
  contractChainId: 11_155_111,
  relayerUrl: "https://relayer.example",
  dataDir,
});

const executor = (): FlowExecutor & {
  close: ReturnType<typeof vi.fn>;
  prepare: ReturnType<typeof vi.fn>;
} => ({
  flow: "input-proof",
  prepare: vi.fn().mockResolvedValue(undefined),
  execute: vi.fn(),
  close: vi.fn(async () => {
    mocks.events.push("executor.stop");
  }),
});

beforeEach(async () => {
  dir = await mkdtemp(join(tmpdir(), "load-test-run-lifecycle-"));
  mocks.clients.length = 0;
  mocks.collectorInstances.length = 0;
  mocks.injectorInstances.length = 0;
  mocks.events.length = 0;
  mocks.createFlowExecutor.mockReset();
  mocks.runScheduler.mockReset();
  mocks.buildReport.mockReset();
  mocks.recorder.records.length = 0;
  mocks.recorder.close.mockReset().mockImplementation(async () => {
    mocks.events.push("recorder.stop");
  });
  mocks.buildReport.mockReturnValue({ thresholds: { passed: true, breaches: [] } });
  mocks.runScheduler.mockResolvedValue({
    submitted: 1,
    completed: 1,
    abandoned: 0,
    poolExhausted: false,
    submissionDurationMs: 1,
    interrupted: false,
  });
});

afterEach(async () => {
  await rm(dir, { recursive: true, force: true });
});

describe("executeRun lifecycle", () => {
  it("closes a registered executor and clients when prepare fails", async () => {
    const instance = executor();
    const failure = new Error("prepare failed");
    instance.prepare.mockRejectedValue(failure);
    mocks.createFlowExecutor.mockResolvedValue(instance);

    let received: unknown;
    try {
      await executeRun({ scenario, env: env(dir), skipReadiness: true });
    } catch (error) {
      received = error;
    }

    expect(received).toBe(failure);
    expect(instance.close).toHaveBeenCalledOnce();
    expect(mocks.clients).toHaveLength(1);
    expect(mocks.clients[0]?.close).toHaveBeenCalledOnce();
  });

  it("uses ZAMA_FHEVM_API_KEY_B for the candidate relayer client, falling back to the shared key", async () => {
    const originalApiKey = process.env.ZAMA_FHEVM_API_KEY;
    const originalApiKeyB = process.env.ZAMA_FHEVM_API_KEY_B;
    process.env.ZAMA_FHEVM_API_KEY = "shared-key";
    process.env.ZAMA_FHEVM_API_KEY_B = "candidate-key";
    try {
      const instance = executor();
      const failure = new Error("prepare failed");
      instance.prepare.mockRejectedValue(failure);
      mocks.createFlowExecutor.mockResolvedValue(instance);

      await executeRun({
        scenario,
        env: { ...env(dir), relayerBUrl: "https://candidate.example" },
        skipReadiness: true,
      }).catch(() => undefined);

      expect(mocks.clients.map((client) => client.apiKey)).toEqual([
        "shared-key",
        "candidate-key",
      ]);
    } finally {
      if (originalApiKey === undefined) delete process.env.ZAMA_FHEVM_API_KEY;
      else process.env.ZAMA_FHEVM_API_KEY = originalApiKey;
      if (originalApiKeyB === undefined) delete process.env.ZAMA_FHEVM_API_KEY_B;
      else process.env.ZAMA_FHEVM_API_KEY_B = originalApiKeyB;
    }
  });

  it("settles every owned resource when the scheduler fails", async () => {
    const instance = executor();
    mocks.createFlowExecutor.mockResolvedValue(instance);
    const failure = new Error("scheduler failed");
    mocks.runScheduler.mockRejectedValue(failure);

    let received: unknown;
    try {
      await executeRun({ scenario, env: env(dir), skipReadiness: true });
    } catch (error) {
      received = error;
    }

    expect(received).toBe(failure);
    expect(mocks.collectorInstances[0]?.stop).toHaveBeenCalledOnce();
    expect(mocks.injectorInstances[0]?.stop).toHaveBeenCalledOnce();
    expect(mocks.recorder.close).toHaveBeenCalledOnce();
    expect(instance.close).toHaveBeenCalledOnce();
    expect(mocks.clients[0]?.close).toHaveBeenCalledOnce();
  });

  it("preserves the primary failure when cleanup also fails", async () => {
    const instance = executor();
    const cleanupFailure = new Error("executor close failed");
    instance.close.mockRejectedValue(cleanupFailure);
    mocks.createFlowExecutor.mockResolvedValue(instance);
    const primaryFailure = new Error("scheduler failed");
    mocks.runScheduler.mockRejectedValue(primaryFailure);

    const received = await executeRun({
      scenario,
      env: env(dir),
      skipReadiness: true,
    }).catch((error: unknown) => error);

    expect(received).toBeInstanceOf(AggregateError);
    expect((received as AggregateError).errors).toEqual([
      primaryFailure,
      cleanupFailure,
    ]);
    expect((received as Error & { cause?: unknown }).cause).toBe(primaryFailure);
    expect(mocks.recorder.close).toHaveBeenCalledOnce();
    expect(mocks.clients[0]?.close).toHaveBeenCalledOnce();
  });

  it("stops resources before building an interrupted partial report", async () => {
    const instance = executor();
    mocks.createFlowExecutor.mockResolvedValue(instance);
    mocks.runScheduler.mockResolvedValue({
      submitted: 1,
      completed: 1,
      abandoned: 1,
      poolExhausted: false,
      submissionDurationMs: 1,
      interrupted: true,
    });

    const result = await executeRun({
      scenario,
      env: env(dir),
      skipReadiness: true,
    });

    expect(result.status).toBe("interrupted");
    const reportIndex = mocks.events.indexOf("report.build");
    expect(reportIndex).toBeGreaterThan(-1);
    for (const event of [
      "prometheus.stop",
      "injector.stop",
      "recorder.stop",
      "executor.stop",
      "client.stop",
    ]) {
      expect(mocks.events.indexOf(event)).toBeLessThan(reportIndex);
    }
    expect(mocks.events.indexOf("prometheus.stop")).toBeLessThan(
      mocks.events.indexOf("recorder.stop"),
    );
    expect(mocks.events.indexOf("recorder.stop")).toBeLessThan(
      mocks.events.indexOf("executor.stop"),
    );
    expect(mocks.events.indexOf("executor.stop")).toBeLessThan(
      mocks.events.indexOf("client.stop"),
    );
  });

  it("latches an interrupt that arrives during cleanup into the report", async () => {
    const instance = executor();
    mocks.createFlowExecutor.mockResolvedValue(instance);
    const controller = new AbortController();
    mocks.recorder.close.mockImplementationOnce(async () => controller.abort());
    mocks.runScheduler.mockResolvedValue({
      submitted: 1, completed: 1, abandoned: 0, poolExhausted: false,
      submissionDurationMs: 1, interrupted: false,
    });
    const result = await executeRun({
      scenario, env: env(dir), skipReadiness: true, signal: controller.signal,
    });
    expect(result.status).toBe("interrupted");
    expect(mocks.buildReport).toHaveBeenCalledWith(
      expect.objectContaining({ interrupted: true }),
    );
  });

  it("freezes report and returned status together before artifact writes", async () => {
    const instance = executor();
    mocks.createFlowExecutor.mockResolvedValue(instance);
    const controller = new AbortController();
    mocks.buildReport.mockImplementationOnce((input: { interrupted: boolean }) => {
      controller.abort();
      return {
        run: { status: input.interrupted ? "interrupted" : "completed" },
        thresholds: { passed: true, breaches: [] },
      };
    });
    const result = await executeRun({
      scenario, env: env(dir), skipReadiness: true, signal: controller.signal,
    });
    expect(result.status).toBe("completed");
    expect((result.report as unknown as { run: { status: string } }).run.status).toBe(result.status);
  });

  it("bounds a hung close and continues later cleanup phases", async () => {
    const instance = executor();
    instance.close.mockImplementationOnce(() => new Promise(() => undefined));
    mocks.createFlowExecutor.mockResolvedValue(instance);
    mocks.runScheduler.mockRejectedValueOnce(new Error("scheduler failed"));
    const error = await executeRun({
      scenario, env: env(dir), skipReadiness: true, cleanupTimeoutMs: 20,
    }).catch((caught: unknown) => caught);
    expect(error).toBeInstanceOf(AggregateError);
    expect((error as AggregateError).errors.some((entry) =>
      entry instanceof Error && entry.message.includes("Timed out cleaning up executor input-proof")
    )).toBe(true);
    expect(mocks.clients[0]?.close).toHaveBeenCalledOnce();
  });
});
