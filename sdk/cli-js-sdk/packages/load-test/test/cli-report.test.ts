import { Command } from "@commander-js/extra-typings";
import { beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  diffReports: vi.fn(),
  readReport: vi.fn(),
}));

vi.mock("../src/report/diff", () => ({ diffReports: mocks.diffReports }));
vi.mock("../src/cli/shared", async (importOriginal) => ({
  ...await importOriginal<typeof import("../src/cli/shared")>(),
  readReport: mocks.readReport,
}));
vi.mock("../src/shared/logger", () => ({
  logger: { error: vi.fn(), info: vi.fn(), success: vi.fn(), warn: vi.fn() },
}));

import { registerReportCommands } from "../src/cli/commands/report";

const program = (): Command => {
  const command = new Command();
  registerReportCommands(command as unknown as Parameters<typeof registerReportCommands>[0]);
  return command;
};

beforeEach(() => {
  mocks.diffReports.mockReset().mockReturnValue({ passed: true, notes: [], regressions: [] });
  mocks.readReport.mockReset().mockImplementation(async (path: string) => ({ path }));
});

describe("report diff tolerance flags", () => {
  it("documents the relative/absolute semantics and defaults in help text", () => {
    const report = program().commands.find((command) => command.name() === "report")!;
    const diff = report.commands.find((command) => command.name() === "diff")!;
    const help = diff.helpInformation();
    expect(help).toContain("--max-latency-increase");
    expect(help).toContain("relative");
    expect(help).toContain("default 0.20");
    expect(help).toContain("--max-error-rate-increase");
    expect(help).toContain("absolute");
    expect(help).toContain("default 0.01");
  });

  it("maps --max-latency-increase and --max-error-rate-increase onto diffReports options", async () => {
    await program().parseAsync([
      "node", "load-test", "report", "diff", "baseline.json", "current.json",
      "--max-latency-increase", "0.3", "--max-error-rate-increase", "0.05",
    ]);
    expect(mocks.diffReports).toHaveBeenCalledWith(
      { path: "baseline.json" },
      { path: "current.json" },
      { latencyTolerance: 0.3, errorRateTolerance: 0.05 },
    );
  });

  it("accepts 0 for both flags", async () => {
    await program().parseAsync([
      "node", "load-test", "report", "diff", "baseline.json", "current.json",
      "--max-latency-increase", "0", "--max-error-rate-increase", "0",
    ]);
    expect(mocks.diffReports).toHaveBeenCalledWith(
      { path: "baseline.json" },
      { path: "current.json" },
      { latencyTolerance: 0, errorRateTolerance: 0 },
    );
  });

  it("rejects a --max-error-rate-increase above 1", async () => {
    await expect(program().parseAsync([
      "node", "load-test", "report", "diff", "baseline.json", "current.json",
      "--max-error-rate-increase", "1.5",
    ])).rejects.toThrow(/between 0 and 1/);
  });

  it("rejects a negative --max-latency-increase", async () => {
    await expect(program().parseAsync([
      "node", "load-test", "report", "diff", "baseline.json", "current.json",
      "--max-latency-increase", "-0.1",
    ])).rejects.toThrow(/non-negative number/);
  });
});
