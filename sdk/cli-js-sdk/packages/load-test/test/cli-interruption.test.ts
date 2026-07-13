import { Command } from "@commander-js/extra-typings";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  executeRun: vi.fn(),
  loadScenario: vi.fn(),
  loadSuite: vi.fn(),
  runSuite: vi.fn(),
  prepareSuite: vi.fn(),
  resolveSuiteScenarios: vi.fn(),
  inspectPoolRequirements: vi.fn(),
  preparePoolRequirements: vi.fn(),
  formatPoolPlan: vi.fn(),
  assertRelayerReadiness: vi.fn(),
  diffReports: vi.fn(),
  readReport: vi.fn(),
  generateInputProofPool: vi.fn(),
  createHandlePool: vi.fn(),
  openPool: vi.fn(),
  logger: {
    error: vi.fn(), info: vi.fn(), start: vi.fn(), success: vi.fn(), warn: vi.fn(),
  },
}));

vi.mock("../src/cli/shared", async (importOriginal) => ({
  ...await importOriginal<typeof import("../src/cli/shared")>(),
  envFromCommand: vi.fn().mockResolvedValue({
    network: "testnet",
    contractChainId: 11_155_111,
    relayerUrl: "https://relayer.example",
    dataDir: ".load-test",
  }),
  readReport: mocks.readReport,
}));

vi.mock("../src/scenario/load", () => ({ loadScenario: mocks.loadScenario }));
vi.mock("../src/runner/run", () => ({
  executeRun: mocks.executeRun,
  RunInterruptedError: class RunInterruptedError extends Error {},
}));
vi.mock("../src/suite/load", () => ({ loadSuite: mocks.loadSuite }));
vi.mock("../src/suite/run", () => ({
  runSuite: mocks.runSuite,
  prepareSuite: mocks.prepareSuite,
  resolveSuiteScenarios: mocks.resolveSuiteScenarios,
}));
vi.mock("../src/pool/planning", () => ({
  inspectPoolRequirements: mocks.inspectPoolRequirements,
  preparePoolRequirements: mocks.preparePoolRequirements,
  formatPoolPlan: mocks.formatPoolPlan,
}));
vi.mock("../src/runner/readiness", () => ({
  assertRelayerReadiness: mocks.assertRelayerReadiness,
}));
vi.mock("../src/report/diff", () => ({ diffReports: mocks.diffReports }));
vi.mock("../src/pool/input-proof", async (importOriginal) => ({
  ...await importOriginal<typeof import("../src/pool/input-proof")>(),
  generateInputProofPool: mocks.generateInputProofPool,
}));
vi.mock("../src/pool/handles", async (importOriginal) => ({
  ...await importOriginal<typeof import("../src/pool/handles")>(),
  createHandlePool: mocks.createHandlePool,
}));
vi.mock("../src/pool/store", () => ({
  PoolStore: { openIfExists: mocks.openPool },
}));
vi.mock("../src/shared/logger", () => ({
  logger: mocks.logger,
}));

import { registerScenarioCommands } from "../src/cli/commands/scenarios";
import { registerSuiteCommands } from "../src/cli/commands/suite";
import { createProgram } from "../src/cli/program";

const originalExitCode = process.exitCode;

beforeEach(() => {
  process.exitCode = undefined;
  mocks.executeRun.mockReset().mockResolvedValue({
    report: { thresholds: { passed: true, breaches: [] } },
    outputDir: "/tmp/interrupted-run",
    status: "interrupted",
  });
  mocks.loadScenario.mockReset().mockResolvedValue({
    name: "scenario",
    flows: [{ flow: "input-proof", weight: 1, handlesPerRequest: 1 }],
    shape: { kind: "constant", rps: 1, durationSec: 1 },
  });
  mocks.loadSuite.mockReset().mockResolvedValue({ name: "suite" });
  mocks.runSuite.mockReset().mockResolvedValue({
    status: "interrupted",
    passed: false,
    outputRoot: "/tmp/interrupted-suite",
    entries: [],
  });
  mocks.prepareSuite.mockReset().mockResolvedValue({
    status: "completed",
    ready: true,
    outputRoot: "/tmp/prepared-suite",
    entries: [],
  });
  mocks.resolveSuiteScenarios.mockReset().mockResolvedValue([{ scenario: { name: "scenario" } }]);
  mocks.inspectPoolRequirements.mockReset().mockResolvedValue({
    ready: true,
    items: [],
    plannedActions: [],
  });
  mocks.preparePoolRequirements.mockReset().mockResolvedValue({
    plan: { ready: true, items: [], plannedActions: [] },
    preparation: { status: "completed" },
  });
  mocks.formatPoolPlan.mockReset().mockReturnValue([]);
  mocks.assertRelayerReadiness.mockReset().mockResolvedValue(undefined);
  mocks.diffReports.mockReset().mockReturnValue({ passed: true, notes: [], regressions: [] });
  mocks.readReport.mockReset().mockResolvedValue({});
  mocks.generateInputProofPool.mockReset().mockResolvedValue(undefined);
  mocks.createHandlePool.mockReset().mockResolvedValue(undefined);
  mocks.openPool.mockReset().mockResolvedValue(undefined);
  for (const method of Object.values(mocks.logger)) method.mockReset();
});

afterEach(() => {
  process.exitCode = originalExitCode;
});

describe("CLI interruption exit behavior", () => {
  it("exposes the intentional grouped command surface from createProgram", () => {
    const program = createProgram();
    expect(program.commands.map((command) => command.name())).toEqual([
      "pool", "scenario", "suite", "report", "baseline",
    ]);
    expect(program.commands.find((command) => command.name() === "pool")
      ?.commands.map((command) => command.name())).toEqual(["add", "inspect"]);
    expect(program.commands.find((command) => command.name() === "report")
      ?.commands.map((command) => command.name())).toEqual(["render", "diff"]);
    expect(program.commands.find((command) => command.name() === "scenario")
      ?.commands.map((command) => command.name())).toEqual([
        "list", "show", "plan", "prepare", "run",
      ]);
    expect(program.commands.find((command) => command.name() === "suite")
      ?.commands.map((command) => command.name())).toEqual(["list", "show", "plan", "prepare", "run"]);
  });

  it("scopes env flags to env-resolving commands and leaves --network unset by default", () => {
    const program = createProgram();
    // Root help stays clean: env flags live on the commands that resolve them.
    expect(program.helpInformation()).not.toContain("network to target (default:");
    expect(program.helpInformation()).not.toContain("relayer base URL override");

    const scenario = program.commands.find((command) => command.name() === "scenario")!;
    const scenarioRun = scenario.commands.find((command) => command.name() === "run")!;
    expect((scenarioRun.opts() as { network?: string }).network).toBeUndefined();
    expect(scenarioRun.helpInformation()).toContain("network to target (default:");
    expect(scenarioRun.helpInformation()).toContain("relayer base URL override");

    // Read-only commands never resolve an environment, so they omit the flags.
    const scenarioList = scenario.commands.find((command) => command.name() === "list")!;
    expect(scenarioList.helpInformation()).not.toContain("relayer base URL override");
    const report = program.commands.find((command) => command.name() === "report")!;
    const reportDiff = report.commands.find((command) => command.name() === "diff")!;
    expect(reportDiff.helpInformation()).not.toContain("relayer base URL override");
    const baseline = program.commands.find((command) => command.name() === "baseline")!;
    const baselineList = baseline.commands.find((command) => command.name() === "list")!;
    expect(baselineList.helpInformation()).not.toContain("relayer base URL override");
  });

  it("reports a semantic version via --version", async () => {
    const program = createProgram();
    let output = "";
    program.configureOutput({ writeOut: (value) => { output += value; } });
    program.exitOverride();
    await expect(program.parseAsync(["node", "load-test", "--version"]))
      .rejects.toMatchObject({ code: "commander.version" });
    expect(output.trim()).toMatch(/^\d+\.\d+\.\d+$/);
  });

  it("prints the resolved suite JSON with resolved entries via suite show", async () => {
    mocks.loadSuite.mockResolvedValueOnce({ name: "suite", pauseSec: 30, entries: [] });
    mocks.resolveSuiteScenarios.mockResolvedValueOnce([
      { label: "entry-a", scenario: { name: "scenario-a" } },
    ]);
    const consoleSpy = vi.spyOn(console, "log").mockImplementation(() => undefined);
    await createProgram().parseAsync(["node", "load-test", "suite", "show", "suite"]);
    expect(mocks.loadSuite).toHaveBeenCalledWith("suite");
    const printed = JSON.parse(consoleSpy.mock.calls.at(-1)?.[0] as string) as {
      name: string;
      entries: readonly { label: string; scenario: { name: string } }[];
    };
    expect(printed.name).toBe("suite");
    expect(printed.entries).toEqual([{ label: "entry-a", scenario: { name: "scenario-a" } }]);
    consoleSpy.mockRestore();
  });

  it("renders top-level help and routes a scenario action", async () => {
    const help = createProgram();
    let output = "";
    help.configureOutput({ writeOut: (value) => { output += value; } });
    help.exitOverride();
    await expect(help.parseAsync(["node", "load-test", "--help"]))
      .rejects.toMatchObject({ code: "commander.helpDisplayed" });
    expect(output).toContain("baseline");
    expect(output).toContain("report");
    expect(output).not.toMatch(/^\s+scenarios\s/m);

    const action = createProgram();
    const consoleSpy = vi.spyOn(console, "log").mockImplementation(() => undefined);
    await action.parseAsync(["node", "load-test", "scenario", "show", "scenario"]);
    expect(mocks.loadScenario).toHaveBeenCalledWith("scenario", {});
    expect(consoleSpy).toHaveBeenCalled();
    consoleSpy.mockRestore();
  });

  it("keeps help aligned with explicit preparation and shared overrides", () => {
    const program = createProgram();
    expect(program.helpInformation()).toContain(
      "FHEVM relayer load-test tool for legacy and v2 implementations",
    );
    const scenario = program.commands.find((command) => command.name() === "scenario")!;
    const scenarioPlan = scenario.commands.find((command) => command.name() === "plan")!;
    expect(scenarioPlan.helpInformation()).toContain(
      "explicit directory for pool-plan.json/.md",
    );
    const scenarioRun = scenario.commands.find((command) => command.name() === "run")!;
    const scenarioHelp = scenarioRun.helpInformation();
    expect(scenarioHelp).toContain("constant/burst rate; scales segmented rates");
    expect(scenarioHelp).toContain("steady or per-segment/stage duration");
    expect(scenarioHelp).toContain("explicitly create missing pools first");

    expect(program.commands.some((command) => command.name() === "run")).toBe(false);
    const suite = program.commands.find((command) => command.name() === "suite")!;
    expect(suite.description()).toBe("Plan and run suites; preparation is explicit");
    const suiteRun = suite.commands.find((command) => command.name() === "run")!;
    expect(suiteRun.helpInformation()).toContain("never prepare pools implicitly");
    expect(suiteRun.helpInformation()).toContain("authorize local CPU and funded on-chain");
  });

  it("routes pool add by flow and rejects irrelevant options", async () => {
    await createProgram().parseAsync([
      "node", "load-test", "pool", "add",
      "--flow", "input-proof", "--count", "2", "--threads", "3",
    ]);
    expect(mocks.generateInputProofPool).toHaveBeenCalledWith(
      expect.anything(),
      expect.objectContaining({ count: 2, threads: 3 }),
    );
    expect(mocks.createHandlePool).not.toHaveBeenCalled();

    await expect(createProgram().parseAsync([
      "node", "load-test", "pool", "add",
      "--flow", "input-proof", "--count", "1", "--lanes", "2",
    ])).rejects.toThrow(/not valid for input-proof/);
    await expect(createProgram().parseAsync([
      "node", "load-test", "pool", "add",
      "--flow", "user-decrypt", "--count", "1", "--threads", "2",
    ])).rejects.toThrow(/only valid for input-proof/);
    await expect(createProgram().parseAsync([
      "node", "load-test", "pool", "add",
      "--flow", "public-decrypt", "--count", "1", "--delegation-days", "2",
    ])).rejects.toThrow(/only valid for delegated-user-decrypt/);
  });

  it("rejects resource flags above their ceilings", async () => {
    await expect(createProgram().parseAsync([
      "node", "load-test", "pool", "add",
      "--flow", "input-proof", "--count", "1", "--threads", "129",
    ])).rejects.toThrow(/--threads must be at most 128/);

    await expect(createProgram().parseAsync([
      "node", "load-test", "scenario", "run", "scenario", "--connections", "1025",
    ])).rejects.toThrow(/--connections must be at most 1024/);

    await expect(createProgram().parseAsync([
      "node", "load-test", "scenario", "run", "scenario", "--prepare", "--lanes", "65",
    ])).rejects.toThrow(/--lanes must be at most 64/);
  });

  it("reports delegated ACL owners as healthy, expired, or missing", async () => {
    const now = BigInt(Math.floor(Date.now() / 1000));
    mocks.openPool.mockImplementation(async (path: string) =>
      path.endsWith("delegated-user-decrypt-handles")
        ? {
            dir: path,
            meta: {
              flow: "delegated-user-decrypt",
              count: 3,
              ownerIndices: [0, 1, 2],
              delegationExpirations: {
                "0": (now + 3_600n).toString(),
                "1": (now - 1n).toString(),
              },
            },
          }
        : undefined,
    );

    await createProgram().parseAsync(["node", "load-test", "pool", "inspect"]);
    const output = mocks.logger.info.mock.calls.flat().join("\n");
    expect(output).toContain("owner 0 ACL healthy until");
    expect(output).toContain("owner 1 ACL expired at");
    expect(output).toContain("owner 2 ACL missing");
  });

  it("sets exit code 130 and removes run signal listeners", async () => {
    const sigintListeners = process.listenerCount("SIGINT");
    const sigtermListeners = process.listenerCount("SIGTERM");
    const program = new Command();
    registerScenarioCommands(program);

    await program.parseAsync(["node", "load-test", "scenario", "run", "scenario"]);

    expect(process.exitCode).toBe(130);
    expect(process.listenerCount("SIGINT")).toBe(sigintListeners);
    expect(process.listenerCount("SIGTERM")).toBe(sigtermListeners);
  });

  it("sets exit code 130 and removes suite signal listeners", async () => {
    const sigintListeners = process.listenerCount("SIGINT");
    const sigtermListeners = process.listenerCount("SIGTERM");
    const program = new Command();
    registerSuiteCommands(program);

    await program.parseAsync(["node", "load-test", "suite", "run", "suite"]);

    expect(process.exitCode).toBe(130);
    expect(process.listenerCount("SIGINT")).toBe(sigintListeners);
    expect(process.listenerCount("SIGTERM")).toBe(sigtermListeners);
  });

  it("uses exit 2 for plan check work and a blocked suite run", async () => {
    mocks.inspectPoolRequirements.mockResolvedValueOnce({
      ready: false,
      items: [{
        pool: "input-proof",
        observation: { currentItems: 0 },
        requirement: { requests: 1 },
        decision: { deficitItems: 1, refreshRequired: false, detail: "missing" },
      }],
    });
    await createProgram().parseAsync([
      "node", "load-test", "suite", "plan", "suite", "--check",
    ]);
    expect(process.exitCode).toBe(2);
    expect(mocks.inspectPoolRequirements).toHaveBeenCalledWith(
      expect.objectContaining({ artifactDir: undefined }),
    );

    process.exitCode = undefined;
    mocks.runSuite.mockResolvedValueOnce({
      status: "blocked", passed: false, outputRoot: "/tmp/blocked", entries: [],
    });
    await createProgram().parseAsync(["node", "load-test", "suite", "run", "suite"]);
    expect(process.exitCode).toBe(2);
  });

  it("routes dedicated suite preparation with explicit mutation options", async () => {
    await createProgram().parseAsync([
      "node", "load-test", "suite", "prepare", "suite", "--lanes", "3",
    ]);
    expect(mocks.prepareSuite).toHaveBeenCalledWith(expect.objectContaining({
      lanes: 3,
      suite: { name: "suite" },
    }));
  });

  it("aborts suite preparation on SIGINT, exits 130, and removes listeners", async () => {
    let received: AbortSignal | undefined;
    mocks.prepareSuite.mockImplementationOnce(async (options: { signal: AbortSignal }) => {
      received = options.signal;
      await new Promise<void>((resolve) =>
        options.signal.addEventListener("abort", () => resolve(), { once: true }),
      );
      return { status: "interrupted", ready: false, outputRoot: "/tmp/interrupted-prepare" };
    });
    const sigintListeners = process.listenerCount("SIGINT");
    const sigtermListeners = process.listenerCount("SIGTERM");
    const program = new Command();
    registerSuiteCommands(program);
    const parsing = program.parseAsync(["node", "load-test", "suite", "prepare", "suite"]);
    await vi.waitFor(() => expect(received).toBeDefined());
    process.emit("SIGINT");
    await parsing;

    expect(received?.aborted).toBe(true);
    expect(process.exitCode).toBe(130);
    expect(process.listenerCount("SIGINT")).toBe(sigintListeners);
    expect(process.listenerCount("SIGTERM")).toBe(sigtermListeners);
  });

  it("rejects suite run lanes unless preparation is authorized", async () => {
    await expect(createProgram().parseAsync([
      "node", "load-test", "suite", "run", "suite", "--lanes", "2",
    ])).rejects.toThrow(/only valid with --prepare/);
    expect(mocks.runSuite).not.toHaveBeenCalled();
  });

  it("keeps scenario plan read-only and uses exit 2 only with --check", async () => {
    mocks.inspectPoolRequirements.mockResolvedValue({
      ready: false,
      items: [{
        pool: "input-proof",
        flow: "input-proof",
        requirement: { requests: 2 },
        observation: { currentItems: 0, availableItems: 0 },
        decision: { deficitItems: 2, refreshRequired: false, ready: false, detail: "" },
      }],
      plannedActions: [{
        kind: "generate-input-proof", pool: "input-proof", flow: "input-proof", items: 2,
      }],
    });

    await createProgram().parseAsync([
      "node", "load-test", "scenario", "plan", "scenario",
    ]);
    expect(process.exitCode).toBeUndefined();
    expect(mocks.preparePoolRequirements).not.toHaveBeenCalled();
    expect(mocks.assertRelayerReadiness).not.toHaveBeenCalled();
    expect(mocks.executeRun).not.toHaveBeenCalled();
    expect(mocks.inspectPoolRequirements).toHaveBeenLastCalledWith(
      expect.objectContaining({ artifactDir: undefined }),
    );

    await createProgram().parseAsync([
      "node", "load-test", "scenario", "plan", "scenario", "--out", "/tmp/scenario-plan",
    ]);
    expect(mocks.inspectPoolRequirements).toHaveBeenLastCalledWith(
      expect.objectContaining({ artifactDir: "/tmp/scenario-plan" }),
    );

    await createProgram().parseAsync([
      "node", "load-test", "scenario", "plan", "scenario", "--check",
    ]);
    expect(process.exitCode).toBe(2);
    expect(mocks.preparePoolRequirements).not.toHaveBeenCalled();
  });

  it("blocks a deficient scenario run after persisting its plan root", async () => {
    mocks.inspectPoolRequirements.mockResolvedValue({
      ready: false,
      items: [],
      plannedActions: [{
        kind: "create-handles",
        pool: "public-decrypt-handles",
        flow: "public-decrypt",
        items: 3,
      }],
    });

    await createProgram().parseAsync([
      "node", "load-test", "scenario", "run", "scenario",
    ]);

    expect(process.exitCode).toBe(2);
    expect(mocks.assertRelayerReadiness).not.toHaveBeenCalled();
    expect(mocks.preparePoolRequirements).not.toHaveBeenCalled();
    expect(mocks.executeRun).not.toHaveBeenCalled();
    expect(mocks.inspectPoolRequirements).toHaveBeenCalledWith(expect.objectContaining({
      artifactDir: expect.stringMatching(/\.load-test\/runs\/.*-scenario$/),
    }));
  });

  it("checks readiness before explicit preparation and reuses one run directory", async () => {
    const events: string[] = [];
    mocks.inspectPoolRequirements.mockResolvedValue({
      ready: false, items: [], plannedActions: [],
    });
    mocks.assertRelayerReadiness.mockImplementation(async () => { events.push("readiness"); });
    mocks.preparePoolRequirements.mockImplementation(async (options) => {
      await options.beforeActions?.({ ready: false });
      events.push("prepare");
      return { plan: { ready: true }, preparation: { status: "completed" } };
    });
    mocks.executeRun.mockImplementation(async () => {
      events.push("execute");
      return {
        report: { thresholds: { passed: true, breaches: [] } },
        outputDir: "/tmp/stable",
        status: "completed",
      };
    });

    await createProgram().parseAsync([
      "node", "load-test", "scenario", "run", "scenario", "--prepare", "--lanes", "2",
    ]);

    expect(events).toEqual(["readiness", "prepare", "execute"]);
    const artifactDir = mocks.preparePoolRequirements.mock.calls[0]?.[0].artifactDir;
    expect(artifactDir).toMatch(/\.load-test\/runs\/.*-scenario$/);
    expect(mocks.executeRun).toHaveBeenCalledWith(
      expect.objectContaining({ outputDir: artifactDir }),
    );
  });

  it("uses the preparations root for standalone scenario preparation", async () => {
    const events: string[] = [];
    mocks.inspectPoolRequirements.mockResolvedValue({
      ready: false, items: [], plannedActions: [],
    });
    mocks.preparePoolRequirements.mockImplementationOnce(async (options) => {
      await options.beforeActions?.({ ready: false });
      events.push("mutate");
      return { plan: { ready: true }, preparation: { status: "completed" } };
    });
    mocks.assertRelayerReadiness.mockImplementationOnce(async () => { events.push("readiness"); });
    await createProgram().parseAsync([
      "node", "load-test", "scenario", "prepare", "scenario", "--lanes", "2",
    ]);
    const options = mocks.preparePoolRequirements.mock.calls[0]?.[0];
    expect(options.artifactDir).toMatch(
      /\.load-test\/preparations\/.*-scenario-scenario$/,
    );
    expect(options.lanes).toBe(2);
    expect(events).toEqual(["readiness", "mutate"]);
    expect(mocks.executeRun).not.toHaveBeenCalled();
  });

  it("skips redundant preparation and readiness for an already-ready run", async () => {
    mocks.executeRun.mockResolvedValueOnce({
      report: { thresholds: { passed: true, breaches: [] } },
      outputDir: "/tmp/ready-run",
      status: "completed",
    });

    await createProgram().parseAsync([
      "node", "load-test", "scenario", "run", "scenario", "--prepare",
    ]);

    expect(mocks.preparePoolRequirements).not.toHaveBeenCalled();
    expect(mocks.assertRelayerReadiness).not.toHaveBeenCalled();
    expect(mocks.executeRun).toHaveBeenCalledOnce();
  });

  it("warns that --lanes has no effect when the plan is already ready", async () => {
    mocks.executeRun.mockResolvedValueOnce({
      report: { thresholds: { passed: true, breaches: [] } },
      outputDir: "/tmp/ready-run-lanes",
      status: "completed",
    });

    await createProgram().parseAsync([
      "node", "load-test", "scenario", "run", "scenario", "--prepare", "--lanes", "2",
    ]);

    expect(mocks.preparePoolRequirements).not.toHaveBeenCalled();
    expect(mocks.logger.warn.mock.calls.flat()).toContain(
      "--lanes has no effect: pools are already ready, so no preparation is needed.",
    );
  });

  it("records ready standalone preparation without a readiness gate", async () => {
    await createProgram().parseAsync([
      "node", "load-test", "scenario", "prepare", "scenario",
    ]);

    expect(mocks.preparePoolRequirements).toHaveBeenCalledOnce();
    expect(mocks.assertRelayerReadiness).not.toHaveBeenCalled();
  });

  it("rejects run-only lanes unless preparation is authorized", async () => {
    await expect(createProgram().parseAsync([
      "node", "load-test", "scenario", "run", "scenario", "--lanes", "2",
    ])).rejects.toThrow(/only valid with --prepare/);
    expect(mocks.inspectPoolRequirements).not.toHaveBeenCalled();
    expect(mocks.executeRun).not.toHaveBeenCalled();
  });

  it("preserves threshold and baseline regression exit behavior", async () => {
    mocks.executeRun.mockResolvedValueOnce({
      report: { thresholds: { passed: false, breaches: [{}] } },
      outputDir: "/tmp/threshold-run",
      status: "completed",
    });
    await createProgram().parseAsync([
      "node", "load-test", "scenario", "run", "scenario",
    ]);
    expect(process.exitCode).toBe(1);

    process.exitCode = undefined;
    mocks.executeRun.mockResolvedValueOnce({
      report: { thresholds: { passed: true, breaches: [] } },
      outputDir: "/tmp/baseline-run",
      status: "completed",
    });
    mocks.diffReports.mockReturnValueOnce({
      passed: false,
      notes: ["comparison note"],
      regressions: [{ flow: "input-proof", metric: "e2e", baseline: 1, current: 2 }],
    });
    await createProgram().parseAsync([
      "node", "load-test", "scenario", "run", "scenario", "--baseline", "/tmp/base.json",
    ]);
    expect(mocks.readReport).toHaveBeenCalledWith("/tmp/base.json");
    expect(mocks.diffReports).toHaveBeenCalled();
    expect(process.exitCode).toBe(1);
  });

  it("aborts the signal passed to an active run on SIGINT", async () => {
    let received: AbortSignal | undefined;
    mocks.executeRun.mockImplementationOnce(async (options: { signal: AbortSignal }) => {
      received = options.signal;
      await new Promise<void>((resolve) =>
        options.signal.addEventListener("abort", () => resolve(), { once: true }),
      );
      return {
        report: { thresholds: { passed: true, breaches: [] } },
        outputDir: "/tmp/interrupted-run",
        status: "interrupted",
      };
    });
    const program = new Command();
    registerScenarioCommands(program);
    const parsing = program.parseAsync(["node", "load-test", "scenario", "run", "scenario"]);
    await vi.waitFor(() => expect(received).toBeDefined());
    process.emit("SIGINT");
    await parsing;
    expect(received?.aborted).toBe(true);
    expect(process.exitCode).toBe(130);
  });
});
