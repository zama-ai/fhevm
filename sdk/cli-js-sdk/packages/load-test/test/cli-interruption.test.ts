import { Command } from "@commander-js/extra-typings";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  executeRun: vi.fn(),
  loadScenario: vi.fn(),
  loadSuite: vi.fn(),
  runSuite: vi.fn(),
  generateInputProofPool: vi.fn(),
  createHandlePool: vi.fn(),
  openPool: vi.fn(),
  logger: {
    error: vi.fn(), info: vi.fn(), start: vi.fn(), success: vi.fn(), warn: vi.fn(),
  },
}));

vi.mock("../src/cli/shared", () => ({
  envFromCommand: vi.fn().mockResolvedValue({
    network: "testnet",
    contractChainId: 11_155_111,
    relayerUrl: "https://relayer.example",
    dataDir: ".load-test",
  }),
  parseNonNegativeInt: (value: string) => Number(value),
  parsePositiveInt: (value: string) => Number(value),
  parsePositiveNumber: (value: string) => Number(value),
  parsePositiveIntOrAuto: (value: string) => value === "auto" ? "auto" : Number(value),
  parseValueTypes: (value: string) => value.split(","),
  readReport: vi.fn(),
}));

vi.mock("../src/scenario/load", () => ({ loadScenario: mocks.loadScenario }));
vi.mock("../src/runner/run", () => ({
  executeRun: mocks.executeRun,
  RunInterruptedError: class RunInterruptedError extends Error {},
}));
vi.mock("../src/suite/load", () => ({ loadSuite: mocks.loadSuite }));
vi.mock("../src/suite/run", () => ({ runSuite: mocks.runSuite }));
vi.mock("../src/report/diff", () => ({ diffReports: vi.fn() }));
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

import { registerRunCommand } from "../src/cli/commands/run";
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
  mocks.loadScenario.mockReset().mockResolvedValue({ name: "scenario" });
  mocks.loadSuite.mockReset().mockResolvedValue({ name: "suite" });
  mocks.runSuite.mockReset().mockResolvedValue({
    status: "interrupted",
    passed: false,
    outputRoot: "/tmp/interrupted-suite",
    entries: [],
  });
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
      "pool", "scenario", "suite", "run", "report", "baseline",
    ]);
    expect(program.commands.find((command) => command.name() === "pool")
      ?.commands.map((command) => command.name())).toEqual(["add", "inspect"]);
    expect(program.commands.find((command) => command.name() === "report")
      ?.commands.map((command) => command.name())).toEqual(["render", "diff"]);
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
    expect(mocks.loadScenario).toHaveBeenCalledWith("scenario");
    expect(consoleSpy).toHaveBeenCalled();
    consoleSpy.mockRestore();
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
    registerRunCommand(program);

    await program.parseAsync(["node", "load-test", "run", "scenario"]);

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
    registerRunCommand(program);
    const parsing = program.parseAsync(["node", "load-test", "run", "scenario"]);
    await vi.waitFor(() => expect(received).toBeDefined());
    process.emit("SIGINT");
    await parsing;
    expect(received?.aborted).toBe(true);
    expect(process.exitCode).toBe(130);
  });
});
